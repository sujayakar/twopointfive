// Long randomized headless soak of the gameplay layer: the real Game (see headless.ts for what is real and what is stubbed) stepped at a fixed rate for tens of
// simulated minutes per seed under a seeded random schedule of stimuli — gunshot / kick / bang events, kills, the player teleported onto guards, lit in their
// cones, bumping them, doors picked / kicked / slammed, blackouts, restarts and guard resets, escalation raised / stood down, forced room clears interrupted at
// whatever phase they are in, AI and tour toggles — while a set of invariants is asserted every frame and softer smells are counted.
//
//   bun run tools/qa/soak.ts [--seeds 1,2,3] [--minutes 20] [--hz 60] [--quiet] [--json out.json] [--logdir dir] [--grab-per-min 2.4]
//
// Everything random goes through one seeded PRNG per stream (the game's own Math.random included), so a violation is reproducible from its seed + frame
// (re-running the same seed replays the same frames exactly; ~1 s of wall clock per simulated minute). Invariants checked every frame: no exception; nothing
// NaN / out of the world; guard state / awareness / speed sane; no dead man scripted, no orphan script, no clearing while the tour flag or AI-off is up;
// escalation level and clock consistent, no lockdown duty below lockdown; doors never latched-open / locked-and-broken / out of range; pick ∈ [0,1];
// followers' leaders alive; nobody stuck (ordered to move 4 s, went nowhere — reported with the doors / props / nav facts around him), overlapping a colleague
// > 2 s, popping > 0.3 m in a frame, or standing inside static geometry; no alert man rooted at a doorway 6 s with nothing he fires reaching Sam; the
// player's wall hold (Q, pressed at random with the wander keys) only ever on a free man, on its plane and inside its run, bladed, peeking only at an
// open end, and nothing of it left once he is off; mission stages only forward; every downed man carries how he went down (and no living man does),
// a witness record only ever names a man who IS down (found bodies as breathing exactly when they are), and no witness record changes in a frame stepped
// with the AI off; the interrogation engine (dialogue.ts, leaned on at random through Game.interrogate or the bare engine): no exception, no unresolved
// {token} in anything said, no line written for a knowledge level other than the man's own, one stage forward per press and never back for that man;
// the grab / hold (player.ts Hold, driven at random through the real press and the held row, holstered or drawn: grab the nearest calm man from behind,
// question him, walk him about, then choke him out / shove him off (arm) or fire past him / pistol-whip him / check the whip (gun)): Player.holding and the
// one Guard.held always agree both ways (phase and variant), the held man alive, unscripted, pathless, within a hand of his station HOLD.dist ahead of Sam
// once held, never perceiving Sam and never barking, no hold while Sam is down / mid-takedown / on a wall, the choke only in the arm variant and the whip only
// in the gun one, the pistol in the hand exactly when it is the gun variant, rounds past him never his and never closer than the cadence, nothing of a hold
// left on the animators once it is over; leaks; and everything authored restored by a restart.
// Softer smells are counted (barks/min and repeats, path re-plans, clear outcomes by phase, time per alarm level, deaths). tools/qa/probes.ts has the
// deterministic single-situation repros for what this found.
import { standUp, ROOT, Headless } from './headless.ts';
const IK = 'Space';   // the game's interact key (src/game/consts.ts INTERACT_KEY)

const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { killGuard, fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { startPicking, startKick, startTakedown, startGrab } = await import(`${ROOT}/src/game/player.ts`);
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { planClears, escalationOf, nearKnownBody, bodyPos, rollcallState, stillLooking, searchSummary, witnessSummary } = await import(`${ROOT}/src/game/guards.ts`);
const { isBreathing } = await import(`${ROOT}/src/game/game.ts`);
const Dlg = await import(`${ROOT}/src/game/dialogue.ts`);
const { Clearing } = await import(`${ROOT}/src/game/squad.ts`);
const { Doors } = await import(`${ROOT}/src/game/doors.ts`);
const { PLAYER_ID, DRIVE_SECS } = await import(`${ROOT}/src/game/consts.ts`);

type Vec3 = [number, number, number];

// ---------------------------------------------------------------- args
const argv = process.argv.slice(2);
const arg = (k: string, d: string) => { const i = argv.indexOf(`--${k}`); return i >= 0 && argv[i + 1] ? argv[i + 1] : d; };
const SEEDS = arg('seeds', '1,2,3').split(',').map(Number);
const MINUTES = Number(arg('minutes', '20'));
const HZ = Number(arg('hz', '60'));
const QUIET = argv.includes('--quiet');
const JSON_OUT = arg('json', '');
const LOG_DIR = arg('logdir', '');   // per-seed full timeline files (stimuli, clears, smells, violations in order)
const GRAB_PER_MIN = Number(arg('grab-per-min', '2.4'));   // the grab stimulus's rate (raise it to lean on the hold's paths: `--grab-per-min 8`)
const TRACE = arg('trace', '');      // 'route:t0:t1[:everyFrames]' — every quarter second (or every N frames) in [t0, t1] that guard's state / duty / fix / path + the doors go to the log file (chasing one violation)
const DT = 1 / HZ;

// ---------------------------------------------------------------- rng
function mulberry32(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

// ---------------------------------------------------------------- clearing outcome tap (class-level, once)
type ClearOutcome = 'complete' | 'complete-with-skips' | 'cap-timeout' | 'muster-giveup' | 'interrupted' | 'death' | 'dissolved' | 'reset' | 'broken';
let clearTap: ((o: ClearOutcome, c: any, phase: string) => void) | null = null;
let roomTap: ((how: string, c: any) => void) | null = null;
{
  const proto = Clearing.prototype as any; const orig = proto.tick; const origNext = proto.nextRoom;
  proto.nextRoom = function (...args: unknown[]) {   // why is this room being left: walked away from (approach ETA blown), the drill capped, or properly done
    const how = this.stage === 'approach' ? 'approach-giveup' : this.stage === 'drill' && this.drill && !this.drill.complete ? 'drill-cap' : 'room-done';
    this.__skips = (this.__skips ?? 0) + (how === 'room-done' ? 0 : 1); roomTap?.(how, this);
    return origNext.apply(this, args);
  };
  proto.tick = function (dt: number) {
    const stageBefore = this.stage, phaseBefore = this.stage === 'drill' && this.drill ? `drill:${this.drill.phase}` : this.stage;
    const alive = orig.call(this, dt);
    if (!alive && clearTap) {
      const g = this.g; let o: ClearOutcome;
      if (!g.guards.includes(this.a) || (this.b && !g.guards.includes(this.b))) o = 'reset';
      else if (stageBefore === 'muster' && this.stage === 'done' && this.roomI === 0 && !(g.quietUtility || !g.aiEnabled || escalationOf(g) < 2 || g.player.down)) o = 'muster-giveup';
      else if (this.roomI >= this.rooms.length) o = this.__skips ? 'complete-with-skips' : 'complete';
      else if (this.a.state === 'dead' || (this.b && this.b.state === 'dead')) o = 'death';
      else if (g.quietUtility || !g.aiEnabled || escalationOf(g) < 2 || g.player.down) o = 'dissolved';
      else if (this.total > this.cap) o = 'cap-timeout';
      else if (this.a.state !== 'search' || (this.b && this.b.state !== 'search')) o = 'interrupted';   // breakOff put them on patrol/suspicious with something on their minds
      else o = 'broken';
      clearTap(o, this, phaseBefore);
    }
    return alive;
  };
}

// ---------------------------------------------------------------- one seed
interface Violation { seed: number; frame: number; time: number; kind: string; key: string; msg: string; dump?: unknown; }
interface SeedReport {
  seed: number; frames: number; simSeconds: number; wallSeconds: number; exceptions: number;
  violations: Violation[]; violationCounts: Record<string, number>;
  smells: Record<string, unknown>; stimuli: Record<string, number>; timelineTail: string[];
}

async function runSeed(seed: number): Promise<SeedReport> {
  const rngGame = mulberry32(seed * 7919 + 13);      // the game's own dice (reactT jitter, spread, ragdoll twist…)
  const rng = mulberry32(seed * 104729 + 71);       // the schedule's
  Math.random = rngGame;
  const W = await standUp();
  const { game, level, input } = W;
  const col = game.col, nav = col.nav, doors = game.doors;
  const t0wall = performance.now();
  const FRAMES = Math.round(MINUTES * 60 * HZ);
  const pick = <T>(a: T[]): T => a[Math.floor(rng() * a.length) % a.length];
  const rr = (a: number, b: number) => a + rng() * (b - a);
  const routeName = (gd: any) => level.routes[gd.routeI]?.name ?? `g${gd.char.id}`;
  const living = () => game.guards.filter((g: any) => g.state !== 'dead');
  const f2 = (x: number) => (Number.isFinite(x) ? Number(x.toFixed(2)) : x);
  const p2 = (p: Vec3 | null | undefined) => p ? [f2(p[0]), f2(p[2])] : null;

  // -------- logging
  const timeline: string[] = []; const fullLog: string[] = [];
  const stimCounts: Record<string, number> = {};
  let frame = 0;
  const tl = (s: string) => { const line = `${game.time.toFixed(2)}s f${frame} ${s}`; timeline.push(line); fullLog.push(line); if (timeline.length > 4000) timeline.splice(0, 1000); if (!QUIET) console.log(`  [seed ${seed}] ${line}`); };
  const violations: Violation[] = []; const vCounts: Record<string, number> = {}; const vSeen = new Map<string, number>();
  const dump = () => ({
    t: f2(game.time), frame, esc: game.escalationSummary(), escT: f2(game.escalationT), episode: game.alarm.episode, placed: game.alarm.placed, alarmPos: p2(game.alarm.pos),
    clearing: game.clearingSummary() || null, quiet: game.quietUtility, ai: game.aiEnabled, god: game.godMode, invisible: game.playerInvisible, blackout: game.blackout.active,
    rollcall: (() => { const R = rollcallState(game); return { stage: R.stage, nextIn: f2(R.nextAt - game.time), missing: R.missing ? routeName(R.missing) : null, caller: R.caller ? routeName(R.caller) : null, checks: R.checks }; })(),
    player: { pos: p2(game.player.char.pos), down: game.player.down, crouch: game.player.crouch, sprint: game.player.sprinting, picking: game.player.picking?.def.name ?? null, kick: game.player.kick?.door.def.name ?? null, takedown: !!game.player.takedown, dragging: !!game.player.dragging, holding: game.player.holding ? `${routeName(game.player.holding.g)}:${game.player.holding.phase} ${f2(game.player.holding.t)}s` : null, vis: f2(game.player.visibility) },
    guards: game.guards.map((gd: any) => ({ name: routeName(gd), state: gd.state, pos: p2(gd.char.pos), yaw: f2(gd.char.bodyYaw), speed: f2(gd.speed), aw: f2(gd.awareness), reactT: f2(gd.reactT), searchT: f2(gd.searchT), wp: gd.wp, wait: f2(gd.wait),
      path: `${gd.pathI}/${gd.path.length}`, pathGoal: p2(gd.pathGoal), lastKnown: p2(gd.lastKnown), stuckT: f2(gd.stuckT), hold: gd.hold, pinned: gd.pinned, drawn: gd.drawn, task: gd.task ? (gd.task.kind === 'checkOn' ? `checkOn:${routeName(gd.task.who)} wp${gd.task.wp} left${gd.task.left}` : gd.task.kind) : null,
      muzzleDown: gd.muzzleDown, standoff: gd.standoff ? `${routeName(gd.standoff.pair)} side${gd.standoff.side} ${gd.standoff.orbitGoal ? 'to ' + p2(gd.standoff.orbitGoal) : gd.standoff.cornered ? 'cornered' : 'planted'} shots${gd.standoff.shots} ${f2(gd.standoff.orbitM)}m seen${f2(game.time - gd.standoff.seenT)}` : null, missedAt: f2(gd.missedAt), missingRaised: gd.missingRaised, down: gd.downKind ? `${gd.downKind}/${gd.downBy}` : null, witness: witnessSummary(game, gd) || null, talk: gd.talk?.stage ? `s${gd.talk.stage} said${gd.talk.said.size}${gd.talk.given.size ? ' gave ' + [...gd.talk.given].join('/') : ''}` : null, held: gd.held ? `${gd.held.phase} since ${f2(gd.held.since)}` : null,
      search: searchSummary(game, gd) || null, pie: gd.pieing ? `${gd.pieing.door.def.name}:${gd.pieing.phase}` : null,
      leader: gd.leader ? routeName(gd.leader) : null, post: gd.post?.name ?? null, dazzled: game.time < gd.dazzledUntil,
      bodyDuty: gd.bodyDuty ? { body: routeName(gd.bodyDuty.body), role: gd.bodyDuty.role, stage: gd.bodyDuty.stage, t: f2(gd.bodyDuty.t), age: f2(game.time - gd.bodyDuty.t0), spot: p2(gd.bodyDuty.spot), stallT: f2(gd.stallT) } : null,
      script: gd.script ? { goal: p2(gd.script.goal), arrived: !!gd.script.arrived, face: f2(gd.script.face ?? NaN), stance: gd.script.stance ?? null, upper: gd.script.upper ?? null, speed: gd.script.speed ?? null, strafe: !!gd.script.strafe } : null,
      anim: { stance: gd.char.anim.stance, upper: gd.char.anim.upper, kicking: gd.char.anim.kicking, crouch: f2(gd.char.anim.crouch) } })),
    doors: doors.list.map((d: any) => `${d.def.name}:${f2(d.angle)}${d.latched ? 'L' : ''}${d.locked ? 'k' : ''}${d.lockBroken ? 'B' : ''}${d.pick > 0 ? ` pick${f2(d.pick)}` : ''}${d.closing ? 'c' : ''}`).join(' '),
    mission: { stage: game.mission.stage, driveT: f2(game.mission.driveT), stats: game.mission.stats },
    lastStimuli: timeline.slice(-10), lastMsgs: game.messages.map((m: any) => `${f2(m.t)} ${m.text}`),
  });
  const MAX_PER_KEY = 3;
  function violate(kind: string, key: string, msg: string, withDump = true) {
    vCounts[kind] = (vCounts[kind] ?? 0) + 1;
    const k = `${kind}|${key}`; const n = (vSeen.get(k) ?? 0) + 1; vSeen.set(k, n);
    if (n > MAX_PER_KEY) return;
    const v: Violation = { seed, frame, time: f2(game.time), kind, key, msg, dump: withDump && n === 1 ? dump() : undefined };
    violations.push(v);
    console.log(`!! [seed ${seed}] ${game.time.toFixed(2)}s f${frame} ${kind} (${key}): ${msg}`);
    fullLog.push(`${game.time.toFixed(2)}s f${frame} !! VIOLATION ${kind} (${key}): ${msg}${v.dump ? '\n' + JSON.stringify(v.dump) : ''}`);
  }

  // -------- instrumentation: barks, messages, clears, replans
  const barkLog: { t: number; who: string; text: string }[] = []; let barkRepeats = 0; const barkTexts: Record<string, number> = {};
  let inInterrogation = false;   // up while the interrogate stimulus presses a man (his lines are held speech, not barks)
  const origSay = game.say.bind(game);
  game.say = (gd: any, text: string, radio = false) => {
    const who = routeName(gd);
    if (!inInterrogation) for (let i = barkLog.length - 1; i >= 0 && game.time - barkLog[i].t < 5; i--) if (barkLog[i].who === who && barkLog[i].text === text) { barkRepeats++; if (barkRepeats <= 12) tl(`SMELL bark repeat <5s ${who}: "${text}"`); break; }   // (interrogation lines are out of the repeat smell: a man leaned on past his last stage repeats his exhausted pool by design)
    barkLog.push({ t: game.time, who, text }); if (!inInterrogation) barkTexts[text] = (barkTexts[text] ?? 0) + 1;
    if (/[{}]/.test(text)) violate('unresolved-token', who, `${who} says "${text}" — a {token} left in (${inInterrogation ? 'interrogation' : 'bark'})`);
    fullLog.push(`${game.time.toFixed(2)}s f${frame}   ${radio ? 'RADIO' : 'SAY'} ${who} (${gd.script ? 'scripted' : gd.state}${gd.bodyDuty ? ' ' + gd.bodyDuty.role + ':' + gd.bodyDuty.stage : ''} @${f2(gd.char.pos[0])},${f2(gd.char.pos[2])}): ${text}`);   // (the per-seed log file gets every line said; the console timeline only the smells)
    if (gd.state === 'dead') violate('dead-speaks', text, `${who} is dead but says "${text}"`);
    const wasHeld = !!gd.held; const r = origSay(gd, text, radio);
    if (wasHeld && !inInterrogation && gd.bubble && gd.bubble.t === game.time && gd.bubble.text === text) violate('held-speaks', who, `${who} is held in Sam's arm but barked "${text}" (guards.ts say should drop everything but his answers)`);
    if (!inInterrogation && /sound off|radio check|not at his post|check on him|going round his route/.test(text)) { smells.rollcallLines = (smells.rollcallLines ?? 0) + 1; if (game.quietUtility) violate('rollcall-under-tour', who, `${who} says "${text}" while quietUtility is up (the net's radio check must not run under the tour)`); if (!game.aiEnabled) violate('rollcall-ai-off', who, `${who} says "${text}" with the AI off`); }   // (held speech may mention the radio check by design: only barks count as the net's own lines)
    if (/radioing it in$/.test(text)) { const k = /out cold/.test(text) ? 'bodyCallsOutCold' : 'bodyCallsDead'; smells[k] = (smells[k] ?? 0) + 1; }   // the finder's call, by what he found (a takedown / choke leaves a breathing man; a round a corpse)
    if (/^he's got .* hold your fire/.test(text)) { smells.hostageCalls = (smells.hostageCalls ?? 0) + 1; if (!radio) violate('hostage-call-not-radio', who, `"${text}" said off the net`); if (!game.player.holding) violate('hostage-call-no-hold', who, `${who} called "${text}" with nobody held`); }   // the standoff's one call per hold
    if (/one of ours/.test(text)) smells.hostageOrders = (smells.hostageOrders ?? 0) + 1;
    if (gd.standoff && !radio) smells.standoffBarks = (smells.standoffBarks ?? 0) + 1;
    if (/I hit him/.test(text)) smells.friendlyFireBarks = (smells.friendlyFireBarks ?? 0) + 1;
    if ((text === 'clear here — staying on it' || text === 'must have been nothing') && nearKnownBody(game, gd.char.pos, 4)) { const b = nearKnownBody(game, gd.char.pos, 4); violate('line-over-body', who, `${who} says "${text}" ${v3.distXZ(gd.char.pos, bodyPos(b)).toFixed(1)} m from ${routeName(b)}'s found body (duty ${gd.bodyDuty ? gd.bodyDuty.role + ':' + gd.bodyDuty.stage : 'none'})`); }
    return r;
  };
  let msgCount = 0; const origMsg = game.msg.bind(game); game.msg = (t: string) => { msgCount++; return origMsg(t); };
  const clearOutcomes: Record<string, number> = {}; const clearInterruptPhases: Record<string, number> = {}; let clearsStarted = 0; let lastClearingRef: any = null;
  clearTap = (o, c, phase) => { clearOutcomes[o] = (clearOutcomes[o] ?? 0) + 1; if (o !== 'complete') clearInterruptPhases[`${o}@${phase}`] = (clearInterruptPhases[`${o}@${phase}`] ?? 0) + 1; tl(`CLEAR ended: ${o} at ${phase} (room ${c.roomI + 1}/${c.rooms.length}, ${c.total.toFixed(0)} s)`); };
  const roomEnds: Record<string, number> = {};
  roomTap = (how, c) => { roomEnds[how] = (roomEnds[how] ?? 0) + 1; tl(`CLEAR room ${c.roomI + 1} (${c.cur?.room ?? '?'}) left: ${how}${how === 'drill-cap' && c.drill ? ` in phase ${c.drill.phase} — A ${routeName(c.a)} at (${c.a.char.pos[0].toFixed(2)}, ${c.a.char.pos[2].toFixed(2)}) arrived=${c.a.script?.arrived}${c.b ? `, B ${routeName(c.b)} at (${c.b.char.pos[0].toFixed(2)}, ${c.b.char.pos[2].toFixed(2)}) arrived=${c.b.script?.arrived}` : ''}` : how === 'approach-giveup' ? ` — A at (${c.a.char.pos[0].toFixed(2)}, ${c.a.char.pos[2].toFixed(2)}) → stack (${c.cur?.stack[0][0]}, ${c.cur?.stack[0][2]})${c.b ? `, B at (${c.b.char.pos[0].toFixed(2)}, ${c.b.char.pos[2].toFixed(2)})` : ''}` : ''}`); };
  const lastPathRef = new Map<any, any>(); let replans = 0; const replanWindow = new Map<any, number[]>(); let maxReplansPerSec = 0; let maxReplansWho = '';
  const escTime = [0, 0, 0]; let escChanges = 0; let lastEsc = 0;
  const stateTime: Record<string, number> = {};

  // -------- helpers
  const randomFloorPoint = (): Vec3 => { for (let i = 0; i < 200; i++) { const p: Vec3 = [rr(1, 39), 0, rr(1, 27)]; if (!nav.isBlocked(p[0], p[2])) return p; } return [20, 0, 11]; };
  const nearGuardPoint = (gd: any, dmin: number, dmax: number, inFront: boolean | null): Vec3 | null => {
    const gp = gd.char.pos;
    for (let i = 0; i < 40; i++) {
      const a = inFront === null ? rr(-Math.PI, Math.PI) : gd.char.bodyYaw + (inFront ? rr(-0.7, 0.7) : Math.PI + rr(-0.6, 0.6));
      const d = rr(dmin, dmax); const p: Vec3 = [gp[0] + Math.sin(a) * d, 0, gp[2] + Math.cos(a) * d];
      if (!nav.isBlocked(p[0], p[2]) && nav.walkable(gp, p)) return p;
    }
    return null;
  };
  const pushEvent = (kind: string, pos: Vec3, extra: Record<string, unknown> = {}) => game.events.push({ kind, pos: [pos[0], 0, pos[2]], time: game.time, loud: true, level: 1, ...extra });
  /** a point on the keyed (corridor) face of a door, just off the doorway's middle — the side the lock is against the player from */
  const keySideOf = (d: any): Vec3 => { const alongX = Math.abs(Math.sin(d.def.closedDir)) < 0.5; return alongX ? [d.frameCentre[0], 0, d.def.hinge[1] + d.keySide * 0.9] : [d.def.hinge[0] - d.keySide * 0.9, 0, d.frameCentre[2]]; };
  /** a free spot near an arbitrary floor point (a corpse's hips) with a clear knee-height line to it */
  const nearPoint = (c: Vec3, dmin: number, dmax: number): Vec3 | null => { for (let i = 0; i < 40; i++) { const a = rr(-Math.PI, Math.PI), d = rr(dmin, dmax); const p: Vec3 = [c[0] + Math.sin(a) * d, 0, c[2] + Math.cos(a) * d]; if (!nav.isBlocked(p[0], p[2]) && !col.segmentBlocked([c[0], 0.5, c[2]], [p[0], 0.5, p[2]])) return p; } return null; };

  // -------- ongoing behaviours
  let wander: { until: number; keys: string[] } = { until: 0, keys: [] };
  let holdF: { until: number; door: any } | null = null;
  let cursorDoor: { until: number; door: any } | null = null;
  let visUntil = 0; let downSince = -1; let aiOffUntil = -1; let tour: { until: number; saved: any } | null = null; let godFlipAt = rr(60, 240);
  let restartCheck: { source: string } | null = null; let missionRef = game.mission; let stageIdx = 0;
  const STAGE_ORDER: Record<string, number> = { infiltrate: 0, drive: 1, exfil: 2, done: 3, failed: 3 };

  const lastPos = new Map<any, Vec3>(); let teleportedThisFrame = false;
  let qPresses = 0, wallTime = 0, wallPeekPrev = 0;   // back to the wall: Q presses dealt, seconds spent holding a wall, last frame's peek (it may only rise at an open end)
  function releaseKeys() { for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', IK, 'AltLeft']) input.keys.delete(k); wander = { until: game.time + rr(1, 4), keys: [] }; holdF = null; }
  function teleport(p: Vec3, why: string) { releaseKeys(); game.teleportPlayer(p); game.player.fHeld = false; teleportedThisFrame = true; tl(`teleport player → (${p[0].toFixed(1)}, ${p[2].toFixed(1)}) ${why}`); }   // (teleportPlayer breaks off a kick / pick / takedown / drag itself and marks a still-held F as spent; the harness's F came up in releaseKeys, between frames, so that mark is cleared here or an F-hold begun this same frame would never register)

  // -------- restore check after a restart
  function checkRestored(source: string) {
    const bad: string[] = [];
    for (const d of doors.list) {
      const a0 = Math.max(d.minA, Math.min(d.maxA, d.def.angle ?? 0));
      if (Math.abs(d.angle - a0) > 1e-6) bad.push(`${d.def.name} angle ${d.angle.toFixed(3)}≠${a0}`);
      if (d.locked !== !!d.def.locked || d.lockBroken || d.pick !== 0 || d.noticed || d.picking) bad.push(`${d.def.name} lock state locked=${d.locked} broken=${d.lockBroken} pick=${d.pick}`);
      if (d.latched !== (Math.abs(a0) < 1e-3)) bad.push(`${d.def.name} latched=${d.latched}`);
    }
    if (game.escalation !== 0 || game.escalationT !== -1 || game.alarm.episode || game.alarm.placed || game.clearing) bad.push(`escalation ${game.escalationSummary()} T=${game.escalationT} clearing=${!!game.clearing}`);
    game.guards.forEach((gd: any, i: number) => {
      const r0 = level.routes[gd.routeI].points[0];
      if (gd.state !== 'patrol' || gd.script || gd.leader || gd.post || gd.task || gd.hold || gd.pinned || gd.held || gd.standoff || gd.awareness !== 0) bad.push(`guard ${i} state=${gd.state} script=${!!gd.script} leader=${!!gd.leader} post=${!!gd.post} held=${!!gd.held} standoff=${!!gd.standoff} aw=${gd.awareness}`);
      if (v3.distXZ(gd.char.pos, r0) > 0.01 || gd.wp !== 0) bad.push(`guard ${i} not at route start (${p2(gd.char.pos)} vs ${p2(r0)}, wp ${gd.wp})`);
    });
    for (const [t, on] of game.initialOn) if (t.on !== on) bad.push(`light ${t.name} on=${t.on}≠${on}`);
    if (game.targets.some((t: any) => t.broken || t.disabledUntil !== -1)) bad.push('a fixture still broken/disabled');
    if (game.blackout.active) bad.push('blackout still active');
    if (game.mission.stage !== 'infiltrate' || game.mission.pulled || game.mission.driveT !== -1) bad.push(`mission ${game.mission.stage}`);
    { const pl = game.player; if (pl.down || v3.distXZ(pl.char.pos, level.playerSpawn) > 0.01 || pl.picking || pl.kick || pl.dragging || pl.takedown || pl.holding || pl.guardHold !== 0 || pl.char.anim.holdPose || pl.pendingThrow || pl.throwHeld || pl.fHeld || pl.doorHold !== 0 || pl.doorCracking || pl.crouch || pl.sprinting || pl.char.anim.lockpick || pl.char.anim.kicking || pl.hitsLeft !== 1 || pl.slot !== 1 || pl.wall || pl.char.anim.stance !== 'none') bad.push(`player pos ${p2(pl.char.pos)} down=${pl.down} picking=${!!pl.picking} kick=${!!pl.kick} drag=${!!pl.dragging} takedown=${!!pl.takedown} holding=${!!pl.holding} guardHold=${pl.guardHold} throw=${!!pl.pendingThrow}/${pl.throwHeld} fHeld=${pl.fHeld} doorHold=${pl.doorHold} crouch=${pl.crouch} sprint=${pl.sprinting} lockpick=${pl.char.anim.lockpick} kicking=${pl.char.anim.kicking} hits=${pl.hitsLeft} slot=${pl.slot} wall=${!!pl.wall} stance=${pl.char.anim.stance}`); }
    if (game.events.length || game.items.items.length) bad.push(`aftermath: ${game.events.length} events, ${game.items.items.length} items`);
    if (game.props.props.some((p: any) => Math.hypot(p.x - p.home[0], p.z - p.home[1]) > 0.01)) bad.push('props not home');
    if (bad.length) violate('restart-restore', source, bad.slice(0, 6).join(' · '));
  }

  // -------- stimuli
  /** mild: may also fire during a respite (a stretch left quiet on purpose so alarm episodes can lapse, the floor can escalate by itself and clears can run deep) */
  interface Stim { name: string; perMin: number; mild?: boolean; when?: () => boolean; run: () => string | null | void; }
  const lockedDoors = () => doors.list.filter((d: any) => d.locked && !d.lockBroken);
  const stims: Stim[] = [
    { name: 'shot-event', perMin: 1.2, run: () => { const p = randomFloorPoint(); pushEvent('shot', p); return `at (${p[0].toFixed(1)}, ${p[2].toFixed(1)})`; } },
    { name: 'kick-event', perMin: 0.3, run: () => { const d = pick(doors.list) as any; pushEvent('kick', d.pos, { who: PLAYER_ID }); return d.def.name; } },
    { name: 'bang-event', perMin: 0.3, run: () => { const p = randomFloorPoint(); pushEvent('bang', p); return `at (${p[0].toFixed(1)}, ${p[2].toFixed(1)})`; } },
    { name: 'door-noise-event', perMin: 0.5, mild: true, run: () => { const d = pick(doors.list) as any; game.events.push({ kind: 'door', pos: [d.pos[0], 0, d.pos[2]], time: game.time, loud: rng() < 0.3, level: rr(0.1, 1), who: rng() < 0.5 ? PLAYER_ID : 0 }); return d.def.name; } },
    { name: 'kill-guard', perMin: 0.45, when: () => living().length > 0, run: () => {   // every call shape killGuard takes: the two old ones (a round / the takedown, defaults) and the explicit how / by of the shield and the choke-out to come
      const gd = pick(living()) as any; const was = gd.script ? 'scripted' : gd.state; const quiet = rng() < 0.35; const a = rr(-Math.PI, Math.PI); const dir: Vec3 = [Math.sin(a), 0, Math.cos(a)]; const r = rng(); stimKilled.add(gd);
      let shape: string;
      if (r < 0.5) { killGuard(game, gd, dir, quiet); shape = quiet ? 'takedown (default → struck)' : 'round (default → shot)'; }
      else if (quiet) { const how = rng() < 0.5 ? 'struck' : 'choked'; killGuard(game, gd, dir, true, how); shape = how; }
      else { const by = pick(['player', 'player', 'guard', 'world']); killGuard(game, gd, dir, false, 'shot', by); shape = `shot by ${by}`; }
      return `${routeName(gd)} (${was}, ${shape})`;
    } },
    { name: 'player-shoots-guard', perMin: 0.2, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; fireWeapon(game, game.player.char, v3.add(gd.char.pos, [0, 1.25, 0]), true); return routeName(gd); } },
    { name: 'player-shoots-wall', perMin: 0.25, when: () => !game.player.down, run: () => { const p = randomFloorPoint(); fireWeapon(game, game.player.char, [p[0], 1.2, p[2]], true); return null; } },
    { name: 'teleport-near-guard', perMin: 0.6, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 0.8, 3.5, null); if (!p) return 'no spot'; teleport(p, `near ${routeName(gd)}`); return null; } },
    { name: 'bump-guard', perMin: 0.35, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 0.35, 0.6, null); if (!p) return 'no spot'; teleport(p, `bumping ${routeName(gd)} (${gd.script ? 'scripted' : gd.state})`); return null; } },
    { name: 'lit-in-cone', perMin: 0.45, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 2, 7, true); if (!p) return 'no spot'; teleport(p, `in front of ${routeName(gd)}`); game.player.visibility = rr(0.6, 1); visUntil = game.time + rr(3, 10); return `vis ${game.player.visibility.toFixed(2)}`; } },
    { name: 'sneak-behind', perMin: 0.3, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 0.7, 0.95, false); if (!p) return 'no spot'; teleport(p, `behind ${routeName(gd)}`); if (rng() < 0.7 && gd.state !== 'alert') { if (rng() < 0.4 && !game.player.holding && !grabJob) { startGrab(game, gd); return 'grab (left to whatever ends it)'; } startTakedown(game, gd); return 'takedown'; } return null; } },
    { name: 'pick-door', perMin: 0.45, when: () => !game.player.down && !grabJob && lockedDoors().some((d: any) => d.latched), run: () => {
      const d = pick(lockedDoors().filter((x: any) => x.latched)) as any; const from = keySideOf(d);
      const spot = doors.workSpot(d, from); teleport([spot.pos[0] + rr(-0.3, 0.3), 0, spot.pos[2] + rr(-0.3, 0.3)], `to pick ${d.def.name}`);
      const full = rng() < 0.7; const secs = full ? Doors.PICK_SECS + rr(1.5, 3) : rr(0.8, 3.5);
      if (rng() < 0.5) { startPicking(game, d); input.keys.add(IK); holdF = { until: game.time + secs, door: d }; return `${d.def.name} direct ${full ? 'full' : 'partial'} ${secs.toFixed(1)}s`; }
      cursorDoor = { until: game.time + secs + 0.5, door: d }; input.keys.add(IK); holdF = { until: game.time + secs, door: d }; return `${d.def.name} via F-hold ${full ? 'full' : 'partial'} ${secs.toFixed(1)}s`;
    } },
    { name: 'kick-door', perMin: 0.35, when: () => !game.player.down && !grabJob, run: () => {
      const cands = lockedDoors().filter((x: any) => x.latched); const d = (cands.length && rng() < 0.8 ? pick(cands) : pick(doors.list)) as any;
      const spot = doors.kickSpot(d, keySideOf(d)); teleport([spot.pos[0] + rr(-0.2, 0.2), 0, spot.pos[2] + rr(-0.2, 0.2)], `to kick ${d.def.name}`);
      if (rng() < 0.5) { startKick(game, d); return `${d.def.name} direct`; }
      game.player.crouch = false; cursorDoor = { until: game.time + 1.0, door: d }; input.pressed.add(IK); return `${d.def.name} via F-tap (lockedOut=${doors.lockedOut(d, game.player.char.pos)})`;
    } },
    { name: 'door-fiddle', perMin: 0.5, mild: true, run: () => { const d = pick(doors.list) as any; const r = rng(); const from = randomFloorPoint();
      if (r < 0.25) { doors.kickIn(d, from, PLAYER_ID); return `kickIn ${d.def.name}`; } if (r < 0.6) { doors.use(d, from, rng() < 0.5 ? PLAYER_ID : 10); return `use ${d.def.name}`; }
      if (r < 0.8) { doors.pullTo(d, 10); return `pullTo ${d.def.name}`; } d.latched = false; d.vel = rr(-8, 8); d.lastWho = PLAYER_ID; return `fling ${d.def.name} ${d.vel.toFixed(1)}`; } },
    { name: 'blackout', perMin: 0.25, run: () => { if (game.blackout.active && rng() < 0.6) { game.endBlackout(); return 'end'; } const r = rng(); game.setBlackout(r < 0.2 ? Infinity : rr(5, 40), r < 0.2 ? 'shot' : r < 0.6 ? 'ocp' : 'debug'); return r < 0.2 ? 'permanent' : 'timed'; } },
    { name: 'restart', perMin: 0.18, run: () => { releaseKeys(); cursorDoor = null; game.restartEncounter(); teleportedThisFrame = true; game.godMode = rng() < 0.4; missionRef = game.mission; stageIdx = 0; checkRestored('restartEncounter()'); return `god=${game.godMode}`; } },
    { name: 'reset-guards', perMin: 0.15, run: () => { game.clearAftermath(); game.resetGuards(); return null; } },
    { name: 'escalate', perMin: 0.4, run: () => { game.escalate(); return game.escalationSummary(); } },
    { name: 'stand-down', perMin: 0.15, run: () => { game.standDown(); return null; } },
    { name: 'force-clear', perMin: 0.5, run: () => { if (rng() < 0.7) game.clearNearestRoom(); else planClears(game, true); return game.clearingSummary() || 'no clear'; } },
    { name: 'interrupt-clear', perMin: 0.4, mild: true, when: () => !!game.clearing && game.clearing.stage !== 'done', run: () => interruptClear() },
  ];
  function interruptClear(): string {
      const C = game.clearing; const phase = C.stage === 'drill' && C.drill ? `drill:${C.drill.phase}` : C.stage; const r = rng(); let what: string;
      interruptPhases[phase] = (interruptPhases[phase] ?? 0) + 1;
      if (r < 0.16) { pushEvent('shot', randomFloorPoint()); what = 'shot event'; }
      else if (r < 0.26) { const gd = rng() < 0.5 || !C.b ? C.a : C.b; stimKilled.add(gd); killGuard(game, gd, [1, 0, 0], rng() < 0.5); what = `killed ${routeName(gd)}`; }
      else if (r < 0.34) { const third = living().find((x: any) => x !== C.a && x !== C.b); if (third) { stimKilled.add(third); killGuard(game, third, [0, 0, 1], true); what = 'killed third man quietly'; } else what = 'no third'; }
      else if (r < 0.42) { game.standDown(); what = 'standDown'; }
      else if (r < 0.50) { game.aiEnabled = false; aiOffUntil = game.time + rr(0.5, 4); what = 'ai off'; }
      else if (r < 0.58) { game.setBlackout(rr(6, 20), 'ocp'); what = 'blackout'; }
      else if (r < 0.70) { const p = nearGuardPoint(C.a, 0.35, 0.55, null); if (p && !game.player.down) { teleport(p, 'bump point man'); what = 'bump #1'; } else what = 'bump: no spot'; }
      else if (r < 0.80) { const p = nearGuardPoint(C.b ?? C.a, 2, 6, true); if (p && !game.player.down) { teleport(p, 'lit before the pair'); game.player.visibility = 0.9; visUntil = game.time + 6; what = 'lit in cone'; } else what = 'lit: no spot'; }
      else if (r < 0.86) { releaseKeys(); cursorDoor = null; game.restartEncounter(); teleportedThisFrame = true; missionRef = game.mission; stageIdx = 0; checkRestored('restart mid-clear'); what = 'restart'; }
      else if (r < 0.92) { const d = C.cur ? doors.byName(C.cur.door) : null; if (d) { doors.kickIn(d, randomFloorPoint(), PLAYER_ID); what = `kicked their door ${d.def.name}`; } else what = 'no door'; }
      else { game.events.push({ kind: 'step', pos: v3.copy(C.a.char.pos), time: game.time, loud: false, level: 1, who: PLAYER_ID }); game.events.push({ kind: 'step', pos: v3.copy(C.a.char.pos), time: game.time, loud: false, level: 1, who: PLAYER_ID }); what = 'loud steps at #1'; }
      return `${what} @ ${phase}`;
  }
  const interruptPhases: Record<string, number> = {};
  let rackJob: { until: number; pressed: boolean } | null = null; const missionSeen: Record<string, number> = {};
  const moreStims: Stim[] = [
    { name: 'mission-step', perMin: 0.35, mild: true, when: () => !game.player.down && !game.quietUtility && !game.player.picking && !game.player.kick, run: () => {   // walk the mission thread a stage at a time through its real triggers
      const m = game.mission, M = level.mission;
      if (m.stage === 'infiltrate') { teleport([18.0 + rr(-1, 1), 0, 8.9], 'into the server room (mission)'); return 'infiltrate → expect drive'; }
      if (m.stage === 'drive') { teleport([M.rack.front[0] + rr(-0.2, 0.2), 0, M.rack.front[2] + 0.65], 'to the mission rack'); game.player.crouch = false; rackJob = { until: game.time + DRIVE_SECS + 3, pressed: false }; wander = { until: rackJob.until + 1, keys: [] }; return 'drive: F on the rack'; }
      if (m.stage === 'exfil') { teleport([M.exfilX + 0.8, 0, M.exfilZ + rr(-1, 1)], 'out past the fire exit'); return 'exfil → expect done'; }
      return `stage ${m.stage} (nothing to do)`;
    } },
    { name: 'ai-toggle', perMin: 0.1, run: () => { game.aiEnabled = false; aiOffUntil = game.time + rr(1, 8); return `off for ${(aiOffUntil - game.time).toFixed(1)}s`; } },
    { name: 'tour-blip', perMin: 0.1, when: () => !tour, run: () => {   // what Demo.start / stop do to the game, minus the beats
      const saved = { ai: game.aiEnabled, god: game.godMode, invisible: game.playerInvisible, pos: v3.copy(game.player.char.pos) };
      game.quietUtility = true; game.clearAftermath(); releaseKeys(); cursorDoor = null;
      if (game.player.down) { game.restartEncounter(); teleportedThisFrame = true; missionRef = game.mission; stageIdx = 0; } else game.resetGuards();
      game.aiEnabled = true; game.godMode = true; game.playerInvisible = true; if (game.blackout.active) game.endBlackout();
      tour = { until: game.time + rr(5, 40), saved }; return `for ${(tour.until - game.time).toFixed(0)}s`;
    } },
    { name: 'drop-throwable', perMin: 0.25, run: () => { const p = rng() < 0.5 && living().length ? (nearGuardPoint(pick(living()), 1, 4, null) ?? randomFloorPoint()) : randomFloorPoint(); const kind = rng() < 0.5 ? 'smoke' : 'flash'; game.dropThrowable(kind, p); return `${kind} at (${p[0].toFixed(1)}, ${p[2].toFixed(1)})`; } },
    { name: 'toggle-invisible', perMin: 0.15, run: () => { game.playerInvisible = !game.playerInvisible; return String(game.playerInvisible); } },
    { name: 'drag-body', perMin: 0.3, when: () => !game.player.down && game.guards.some((g: any) => g.state === 'dead'), run: () => { const b = pick(game.guards.filter((g: any) => g.state === 'dead')) as any; const hips = b.char.bones.hips ?? b.char.pos; const p = nearPoint([hips[0], 0, hips[2]], 0.5, 1.0); if (!p) return 'no spot'; teleport(p, 'to a body'); game.toggleDrag(b); wander = { until: game.time + rr(2, 6), keys: [pick(['KeyW', 'KeyS', 'KeyA', 'KeyD'])] }; for (const k of wander.keys) input.keys.add(k); return routeName(b); } },
    { name: 'interrogate', perMin: 1.5, mild: true, when: () => living().length > 0 && !game.player.down, run: () => {   // lean on a random living man one to three presses: mostly through Game.interrogate with Sam put behind him (the reach gate, the bubble), else the bare engine from wherever Sam is (alert / scripted / far-off men get exercised too)
      const gd = pick(living()) as any; const presses = 1 + Math.floor(rng() * 3); const out: string[] = []; let via = 'engine';
      if (rng() < 0.7) { const p = nearGuardPoint(gd, 0.7, 1.6, false) ?? nearGuardPoint(gd, 0.7, 1.8, null); if (p) { teleport(p, `to lean on ${routeName(gd)}`); via = 'game'; } }
      inInterrogation = true; if (game.player.holding?.g !== gd) inSandboxTalk.add(gd);   // (a man nobody holds: the engine stamps his heldSince itself — the sandbox stand-in, no leftover)
      try {
        for (let i = 0; i < presses; i++) {
          const kNow = Dlg.knowledgeOf(game, gd), stageBefore = gd.talk.stage;
          const r = via === 'game' ? game.interrogate(gd, rng) : Dlg.interrogate(game, gd, rng);
          if (!r) { out.push(`null (${v3.distXZ(gd.char.pos, game.player.char.pos).toFixed(1)} m)`); if (via === 'engine') violate('dialogue-null', routeName(gd), `the bare engine returned null for a living man (${gd.callsign} ${gd.state})`); break; }
          checkPick(gd, r, kNow, stageBefore);
          out.push(`s${String(r.stage)} ${r.kind ? 'INTEL ' + r.kind : r.id}${r.widened ? '~' + r.widened : ''}`);
        }
      } finally { inInterrogation = false; }
      return `${gd.callsign} (${gd.script ? 'scripted' : gd.state}, ${Dlg.knowledgeOf(game, gd)}/${Dlg.bandOf(Dlg.agitationOf(game, gd))}/${Dlg.temperamentOf(gd)}) via ${via} ×${presses}: ${out.join(' | ')}`;
    } },
  ];
  /** The grab (player.ts startGrab / updateHold): behind the nearest suitable man, the pistol first HOLSTERED or DRAWN at random (which decides the hold: the arm
   *  variant or the gun-to-his-head one — player.ts grabVariant), grabbed through the real press (cursor on him, Space held past GRAB_HOLD_SECS) or the call; once
   *  held, one to three taps on his row at random spacings (some too soon on purpose: they only cut his line), a few seconds' walk on the wander keys (whatever they
   *  press — Q, C, Shift included — the pair just walks), then an end dealt at the start for the variant it turned out to be: arm — choke him out (Space held through
   *  it), start a choke and ease off, shove him off (E), or just keep him; gun — pistol-whip him (Space held past its landing), cock it and check it, fire a few
   *  rounds past him at whoever / whatever the cursor lands on and THEN whip or shove, shove, or keep. grabTick drives it a frame at a time off Player.holding; the
   *  invariants watch the rest. */
  moreStims.push({ name: 'grab', perMin: GRAB_PER_MIN, when: () => !game.player.down && !grabJob && !holdF && !cursorDoor && !rackJob && !game.player.holding && living().some((x: any) => x.state !== 'alert' && !x.held), run: () => {
    const gd = pick(living().filter((x: any) => x.state !== 'alert' && !x.held)) as any;
    const p = nearGuardPoint(gd, 0.7, 0.9, false); if (!p) return `no spot behind ${routeName(gd)}`;
    teleport(p, `to grab ${routeName(gd)} (${gd.script ? 'scripted' : gd.state})`); game.player.crouch = rng() < 0.5;
    const drawn = rng() < 0.55; game.player.slot = 1; game.player.holstered = !drawn; if (drawn && rng() < 0.4) game.player.pistol.lightOn = !game.player.pistol.lightOn;
    const via = rng() < 0.5 ? 'press' : 'call';
    const end = drawn ? pick(['whip', 'whip', 'whip-abort', 'fire', 'fire', 'release', 'keep']) : pick(['choke', 'choke', 'release', 'release', 'abort', 'keep']);
    grabJob = { gd, via, stage: 'grabbing', t0: game.time, nextAt: 0, presses: 1 + Math.floor(rng() * 3), end, spaceUpAt: -1, asked: 0, variant: drawn ? 'gun' : 'arm', shotsLeft: 0, fired: 0, lastShotT: -1 };
    if (via === 'call') startGrab(game, gd);
    else { W.cursorAt(v3.add(gd.char.pos, [0, 1.35, 0])); input.keys.add(IK); input.pressed.add(IK); grabJob.spaceUpAt = game.time + 0.3 + rr(0.1, 0.4); }
    return `${routeName(gd)} ${drawn ? 'DRAWN (gun)' : 'holstered (arm)'} via ${via}, ×${grabJob.presses} questions, then ${end}`;
  } });
  /** The human shield (guards.ts standoffPerceive / standoffAlert): the same grab, but of a man who has a living, free colleague within a dozen metres — so somebody
   *  walks in on it, calls it, and the floor plays the standoff round the pair — and mostly KEPT (or walked about, then choked in front of them) long enough for the
   *  ring, the flank and the fire rule to get exercised; Sam turns the pair now and then (a new cursor spot). The standoff's own invariants watch the rest. */
  moreStims.push({ name: 'grab-in-view', perMin: GRAB_PER_MIN * 0.5, when: () => !game.player.down && !grabJob && !holdF && !cursorDoor && !rackJob && !game.player.holding && game.aiEnabled && !game.quietUtility && living().filter((x: any) => !x.held).length >= 2 && living().some((x: any) => x.state !== 'alert' && !x.held && !x.script), run: () => {
    const cands = living().filter((x: any) => x.state !== 'alert' && !x.held && !x.script && living().some((o: any) => o !== x && !o.held && v3.distXZ(o.char.pos, x.char.pos) < 12));
    const gd = (cands.length ? pick(cands) : pick(living().filter((x: any) => x.state !== 'alert' && !x.held))) as any; if (!gd) return 'nobody';
    const p = nearGuardPoint(gd, 0.7, 0.9, false); if (!p) return `no spot behind ${routeName(gd)}`;
    teleport(p, `to grab ${routeName(gd)} in company (${gd.script ? 'scripted' : gd.state})`); game.player.crouch = false;
    const end = pick(['keep', 'keep', 'choke', 'release', 'walk-choke']);
    grabJob = { gd, via: 'call', stage: 'grabbing', t0: game.time, nextAt: 0, presses: Math.floor(rng() * 2), end: end === 'walk-choke' ? 'choke' : end, spaceUpAt: -1, asked: 0 };
    startGrab(game, gd);
    const near = living().filter((o: any) => o !== gd).map((o: any) => `${routeName(o)} ${o.state} ${v3.distXZ(o.char.pos, gd.char.pos).toFixed(0)} m`).join(', ');
    return `${routeName(gd)} (${near}), then ${end}`;
  } });
  let grabJob: { gd: any; via: string; stage: 'grabbing' | 'talk' | 'walk' | 'firing' | 'ending' | 'keep'; t0: number; nextAt: number; presses: number; end: string; spaceUpAt: number; asked: number; variant: 'arm' | 'gun'; shotsLeft: number; fired: number; lastShotT: number } | null = null;
  const grabEnds: Record<string, number> = {};
  function grabTick() {
    const J = grabJob; if (!J) return; const t = game.time, H = game.player.holding;
    if (J.spaceUpAt >= 0 && t >= J.spaceUpAt) { input.keys.delete(IK); J.spaceUpAt = -1; }
    const over = (how: string) => {
      grabEnds[how] = (grabEnds[how] ?? 0) + 1; tl(`grab over: ${how} (${routeName(J.gd)} ${J.gd.state}${J.gd.downKind ? '/' + J.gd.downKind : ''}, ${J.variant}, asked ${J.asked}${J.fired ? `, fired ${J.fired}` : ''}, ${(t - J.t0).toFixed(1)} s)`);
      if (J.gd.state === 'dead' && J.gd.downKind === 'shot' && J.gd.downBy === 'player' && J.stage !== 'grabbing' && Math.abs(game.player.lastFireT - J.gd.diedAt) < DT) violate('held-shot-by-sam', routeName(J.gd), `${routeName(J.gd)} died of Sam's own round while in his arm`);   // (fireWeapon skips the held man: a round past him is never his — the kill-guard stimulus's player 'shot' shape never coincides with a real round to the frame)
      if (J.spaceUpAt >= 0) input.keys.delete(IK); grabJob = null;
    };
    if (!H) {   // not (or no longer) holding: the press never took (he turned, walked off, was alert), or it ended — by our end or anything else's
      if (J.stage === 'grabbing' && t - J.t0 < 0.8) return;
      return over(J.stage === 'grabbing' ? `never took (${J.via})` : J.stage === 'ending' ? `ended:${J.end}` : `ended early at ${J.stage}`);
    }
    if (H.g !== J.gd) return over('holding somebody else?!');
    if (J.stage !== 'grabbing' && H.variant !== J.variant) { violate('grab-variant', routeName(J.gd), `grabbed ${J.variant === 'gun' ? 'drawn' : 'holstered'} but the hold is '${H.variant}'`); J.variant = H.variant; }
    switch (J.stage) {
      case 'grabbing': if (H.phase === 'held') { J.stage = 'talk'; J.variant = H.variant; J.nextAt = t + rr(0.3, 2); if (rng() < 0.5) { input.mouseX = rr(200, 1400); input.mouseY = rr(150, 850); } } break;   // (a new cursor spot: the pair comes round to it)
      case 'talk':
        if (t < J.nextAt) break;
        if (J.presses > 0) { input.pressed.add(IK); J.presses--; J.asked++; J.nextAt = t + (rng() < 0.35 ? rr(0.15, 0.9) : rr(1.3, 4)); smells.holdQuestions = (smells.holdQuestions ?? 0) + 1; }
        else { J.stage = 'walk'; const secs = rr(1.5, 6); J.nextAt = t + secs; for (const k of wander.keys) input.keys.delete(k); const keys = [pick(['KeyW', 'KeyS', 'KeyA', 'KeyD'])]; if (rng() < 0.4) keys.push(pick(['KeyW', 'KeyS', 'KeyA', 'KeyD'])); if (rng() < 0.3) keys.push('ShiftLeft'); wander = { until: t + secs, keys }; for (const k of keys) input.keys.add(k); if (rng() < 0.3) input.pressed.add('KeyQ'); if (rng() < 0.3) input.pressed.add('KeyC'); if (J.variant === 'gun' && rng() < 0.3) input.pressed.add('KeyL'); }
        break;
      case 'walk':
        if (t < J.nextAt) break;
        J.stage = J.end === 'keep' ? 'keep' : 'ending'; J.nextAt = t + 12;
        if (J.end === 'choke') { input.keys.add(IK); input.pressed.add(IK); J.spaceUpAt = t + HOLD.secs.chokeHold + HOLD.secs.choke + rr(0.05, 0.6); smells.holdChokes = (smells.holdChokes ?? 0) + 1; }
        else if (J.end === 'abort') { input.keys.add(IK); input.pressed.add(IK); J.spaceUpAt = t + HOLD.secs.chokeHold + rr(0.3, HOLD.secs.choke - 0.4); J.end = rng() < 0.5 ? 'release-after-abort' : 'keep'; J.stage = 'walk'; J.nextAt = J.spaceUpAt + rr(0.5, 2); smells.holdAborts = (smells.holdAborts ?? 0) + 1; }
        else if (J.end === 'whip') { input.keys.add(IK); input.pressed.add(IK); J.spaceUpAt = t + HOLD.secs.chokeHold + HOLD.secs.whip + rr(0.05, 0.6); smells.holdWhips = (smells.holdWhips ?? 0) + 1; }
        else if (J.end === 'whip-abort') { input.keys.add(IK); input.pressed.add(IK); J.spaceUpAt = t + HOLD.secs.chokeHold + rr(0.08, HOLD.secs.whip - 0.12); J.end = pick(['release-after-abort', 'whip-after-abort', 'keep']); J.stage = 'walk'; J.nextAt = J.spaceUpAt + rr(0.5, 2); smells.holdWhipAborts = (smells.holdWhipAborts ?? 0) + 1; }
        else if (J.end === 'whip-after-abort') { input.keys.add(IK); input.pressed.add(IK); J.spaceUpAt = t + HOLD.secs.chokeHold + HOLD.secs.whip + rr(0.05, 0.3); smells.holdWhips = (smells.holdWhips ?? 0) + 1; }
        else if (J.end === 'fire') { J.stage = 'firing'; J.shotsLeft = 2 + Math.floor(rng() * 4); J.nextAt = t; for (const k of wander.keys) input.keys.delete(k); }   // rounds past him first (some clicks inside the cadence on purpose), then an end
        else if (J.end === 'release' || J.end === 'release-after-abort') { input.pressed.add('KeyE'); smells.holdReleases = (smells.holdReleases ?? 0) + 1; }
        break;
      case 'firing':   // a click now and then — mostly on a colleague's chest (the auto-aim's snap), else anywhere on the screen; some inside the cadence on purpose (they must not fire)
        if (t < J.nextAt) break;
        if (J.shotsLeft > 0) {
          const others = living().filter((x: any) => x !== J.gd);
          if (others.length && rng() < 0.7) { const o = pick(others) as any; W.cursorAt((o.char.bones.chest ?? v3.add(o.char.pos, [0, 1.2, 0])) as Vec3); } else { input.mouseX = rr(200, 1400); input.mouseY = rr(150, 850); }
          input.clicked |= 1; J.shotsLeft--; J.nextAt = t + (rng() < 0.3 ? rr(0.1, 0.5) : rr(0.75, 1.6)); smells.holdClicks = (smells.holdClicks ?? 0) + 1;
        } else { J.end = pick(['whip', 'whip', 'release', 'keep']); J.stage = 'walk'; J.nextAt = t + rr(0.3, 1.5); }
        break;
      case 'ending':
        if (t > J.nextAt) { violate('grab-stuck', `${J.end}`, `the hold on ${routeName(J.gd)} is still up ${(t - J.nextAt + 12).toFixed(0)} s after '${J.end}' went in (phase ${H.phase} t ${H.t.toFixed(2)}, variant ${H.variant}, fHeld ${game.player.fHeld}, hover ${game.hover?.kind})`); over('stuck'); }
        break;
      case 'keep': if (t > J.nextAt + 20) over('kept 30 s'); else if (rng() < DT / 3) { input.mouseX = rr(200, 1400); input.mouseY = rr(150, 850); } break;   // (Sam swings the pair round to a new facing every few seconds: whoever is circling has to keep circling)
    }
    if (H.shotT >= 0 && H.shotT !== J.lastShotT) { J.fired++; smells.holdShots = (smells.holdShots ?? 0) + 1; if (J.lastShotT >= 0 && H.shotT - J.lastShotT < 0.7 - 1e-6) violate('hold-cadence', routeName(J.gd), `two rounds past him ${(H.shotT - J.lastShotT).toFixed(3)} s apart (HOLD_FIRE_CD 0.7)`); J.lastShotT = H.shotT; }   // rounds that actually left (Hold.shotT stamps each)
  }
  /** the interrogation invariants on one pick: the cell's K is the man's knowledge as it stood, the line is his K's or floor colour (never another level's), the text
   *  is resolved, non-empty and inside the bubble limit, and the stage went exactly one forward */
  function checkPick(gd: any, r: any, kNow: string, stageBefore: number) {
    const who = `${gd.callsign}`;
    smells.interrogations = (smells.interrogations ?? 0) + 1; smells[`dialogueCell:${r.k}`] = (smells[`dialogueCell:${r.k}`] ?? 0) + 1; if (r.kind) smells[`dialogueGave:${r.kind}`] = (smells[`dialogueGave:${r.kind}`] ?? 0) + 1; if (r.widened) smells[`dialogueWidened:${r.widened}`] = (smells[`dialogueWidened:${r.widened}`] ?? 0) + 1;
    if (r.k !== kNow) violate('dialogue-k-mismatch', who, `${who}'s pick was made for K '${r.k}' but knowledgeOf said '${kNow}' the same frame`);
    if (r.lineK !== '*' && r.lineK !== r.k) violate('dialogue-k-leak', `${who}:${r.id}`, `${who} (K ${r.k}) said ${r.id}, a line written for '${r.lineK}': "${r.text}"`);
    if (/[{}]/.test(r.text) || !String(r.text).trim() || r.text === '…') violate('dialogue-text', `${who}:${r.id}`, `${who} got "${r.text}" (${r.id}, cell ${r.cell}, widened '${r.widened}')`);
    if (r.text.length > Dlg.MAX_CHARS) violate('dialogue-long', r.id, `${r.id} is ${r.text.length} chars: "${r.text}"`);
    if (r.stageN !== stageBefore + 1 || gd.talk.stage !== r.stageN) violate('dialogue-stage', who, `${who}: stage ${stageBefore} → pick stageN ${r.stageN}, talk.stage now ${gd.talk.stage}`);
    const wantKey = r.stageN <= 3 ? r.stageN : 'x'; if (r.stage !== wantKey) violate('dialogue-stage-key', who, `${who}: stageN ${r.stageN} keyed '${String(r.stage)}'`);
    if (r.kind && !gd.talk.given.has(r.kind)) violate('dialogue-given', who, `${who} gave ${r.kind} but talk.given lacks it`);
    fullLog.push(`${game.time.toFixed(2)}s f${frame}   TALK ${who} ${r.cell} s${String(r.stage)} [${r.id}${r.widened ? '~' + r.widened : ''}] ${r.text}`);
  }
  stims.push(...moreStims);
  /** respite: aggressive stimuli off, the player parked out of the way with the meter low — so an alarm episode can actually lapse (the floor escalates on its own,
   *  a natural lockdown deals its own clear) and a clear can reach its late phases before the scheduled interrupt (if any) lands */
  let respiteUntil = -1; let interruptAt = -1; let alertSince = -1;
  function startRespite(secs: number, why: string) {
    respiteUntil = game.time + secs;
    if (!game.player.down) { const far: Vec3[] = [[5.1, 0, 15.2], [4.9, 0, 23.2], [37.5, 0, 26.5], [2.0, 0, 2.0]]; let best = far[0], bd = -1; for (const p of far) { const d = Math.min(...living().map((g: any) => v3.distXZ(g.char.pos, p)), 99); if (d > bd) { bd = d; best = p; } } teleport(best, `(respite: ${why})`); }
    game.player.visibility = 0.02; visUntil = respiteUntil; tl(`RESPITE ${secs.toFixed(0)} s — ${why}`);
  }

  // -------- per-frame invariant state
  const stuck = new Map<any, { t0: number; p0: Vec3; last: Vec3; ground: number; reach: number; want: number; lastReport: number }>();
  const noprog = new Map<any, { t0: number; p0: Vec3; lastReport: number }>();
  const closeT = new Map<string, number>(); const closeReported = new Map<string, number>();
  const dutyBad = new Map<any, number>(); const lingerT = new Map<any, number>(); const lingerRep = new Map<any, number>();
  const doorPlant = new Map<any, { t: number; lastReport: number }>(); let lastPlayerHitT = -100;   // 'aim stuck in the door': an alert man rooted by a doorway with nothing he fires reaching Sam
  const checkOnStale = new Map<any, number>();   // seconds a man has carried a checkOn errand for somebody already found / alive (guards.ts checkOnTick drops it the next suspicious frame; alert/search may hold it — that is fine — so only counted while suspicious)
  const planSeen = new Map<any, any>(), planVisits = new WeakMap<any, number>(), pieSeen = new Map<any, any>(), pieCrossed = new WeakSet<any>();   // search plans / pies already counted (smells)
  const witnessSnap = new Map<any, string>(), sawActSeen = new Map<any, any>(), sawHeldSeen = new WeakSet<any>(); let steppedAiOff = false;   // each man's witness record as JSON going into an AI-off frame (the no-write invariant) / sawAct records already counted / whether the frame just stepped ran with the AI off
  const talkStage = new WeakMap<any, number>();   // each Guard object's interrogation stage last frame (it may only go forward; weak: reset men go with their objects)
  const heldCount = new WeakMap<any, number>(); let heldPrev = new Set<any>(); let holdDeadFrames = 0;   // grabs the harness has SEEN begin per Guard object (witness.wasHeld may not outrun it), who was held last frame, frames a corpse has stayed in the arm
  const standoffSeen = new WeakSet<any>(), standoffOrbit = new WeakMap<any, number>(), standoffFar = new Map<any, number>(), standoffNear = new Map<any, number>(), standoffNoHold = new Map<any, number>();   // the human shield: records counted, metres walked per record, per-man clocks for 'too far and idle' / 'on top of them' / 'nobody held'
  let heldLastFrame = new Set<any>(); let heldGoingIn: any = null; const stimKilled = new WeakSet<any>();   // who was in Sam's arm going into this frame (the set for the friendly-fire check, the one man for the round geometry); men the harness itself put down (killGuard pokes are nobody's round)
  const inSandboxTalk = new WeakSet<any>();   // men leaned on through the sandbox path (Game.interrogate / the bare engine on a man nobody holds): the engine stamps talk.heldSince itself for those, so it is no leftover on them
  let leaderBad = 0; let exceptions = 0; let consecutiveExc = 0; let overdueFrames = 0;
  const finiteChar = (c: any) => Number.isFinite(c.pos[0]) && Number.isFinite(c.pos[1]) && Number.isFinite(c.pos[2]) && Number.isFinite(c.bodyYaw) && Number.isFinite(c.aimYaw) && Number.isFinite(c.aimPitch) && Number.isFinite(c.vel[0]) && Number.isFinite(c.vel[2]);

  function invariants() {
    const t = game.time;
    if (game.player.hitFlash > 1 - 3.5 * DT) lastPlayerHitT = t;   // (set to 1 by a round that reached him, decayed by 3·dt at the end of the same update)
    { const now = new Set<any>(); for (const gd of game.guards) if (gd.held) { now.add(gd); if (!heldPrev.has(gd)) { heldCount.set(gd, (heldCount.get(gd) ?? 0) + 1); smells.holdsBegun = (smells.holdsBegun ?? 0) + 1; } } heldPrev = now; }   // a hold that began this frame (by the stimulus or the press)
    // muzzle discipline: every guard round fired this frame (a guardShot event stamped now, placed at the shooter's feet) — its muzzle → Sam's-chest segment against
    // every OTHER living guard's body axis: inside 0.3 m of one is a round through (or a hand off) a colleague, which guards.ts muzzleCheck exists to make impossible.
    // The man in Sam's arm is the exception with his own, tighter rule (the human shield, guards.ts flankShot): the segment to where the round STOPS — Sam's near
    // side, 0.28 short of his axis — must pass at least 0.45 m (less a whisker for the muzzle's offset from the shoulder line the rule is worked on) from his axis:
    // only a round from behind the pair does that
    for (const e of game.events) {
      if (e.kind !== 'guardShot' || e.time !== t) continue;
      const shooter = game.guards.find((x: any) => x.state !== 'dead' && v3.distXZ(x.char.pos, e.pos) < 0.05); if (!shooter) continue;
      smells.guardRounds = (smells.guardRounds ?? 0) + 1; if (shooter.standoff) smells.standoffShots = (smells.standoffShots ?? 0) + 1;
      const heldMan = game.player.holding?.g ?? heldGoingIn;   // (a hold this very round ended — Sam shot down from behind — still had the man on his station when it flew)
      const from = shooter.char.muzzle; const to = v3.add(game.player.char.pos, [0, game.player.crouch ? 0.8 : 1.25, 0]);
      const dx = to[0] - from[0], dy = to[1] - from[1], dz = to[2] - from[2]; const L2 = dx * dx + dz * dz; if (L2 < 1e-6) continue;
      for (const o of game.guards) {
        if (o === shooter || o.state !== 'dead' && !o.char.alive || o.state === 'dead') continue;
        const h = o.char.bones.hips ?? o.char.pos;
        if (o === heldMan) {   // the shield: clearance off the segment muzzle → Sam's near side (his hit cylinder, 0.28 about his pelvis, at the standoff's aim height)
          const sh = game.player.char.bones.hips ?? game.player.char.pos; const ax = sh[0] - from[0], az = sh[2] - from[2], ay = (game.player.char.pos[1] + 1.42) - from[1]; const L = Math.hypot(ax, ay, az);
          const k = L > 0.33 ? (L - 0.28) / L : 1; const ix = ax * k, iz = az * k; const I2 = ix * ix + iz * iz;
          const tt = I2 > 1e-6 ? Math.max(0, Math.min(1, ((h[0] - from[0]) * ix + (h[2] - from[2]) * iz) / I2)) : 0;
          const d = Math.hypot(from[0] + ix * tt - h[0], from[2] + iz * tt - h[2]);
          smells.roundsPastHostageMin = Math.min(smells.roundsPastHostageMin ?? 99, f2(d));
          if (d < 0.45 - 0.02) violate('round-near-hostage', `${routeName(shooter)}→${routeName(o)}`, `${routeName(shooter)} (${shooter.standoff ? 'standoff' : shooter.state}) fired with the held man ${routeName(o)} only ${d.toFixed(2)} m off the round's segment to Sam's near side (want ≥ 0.45); shooter at ${p2(shooter.char.pos)}, Sam at ${p2(game.player.char.pos)} facing ${f2(game.player.char.bodyYaw)}, hold ${game.player.holding?.phase}`);
          continue;
        }
        const tt = Math.max(0, Math.min(1, ((h[0] - from[0]) * dx + (h[2] - from[2]) * dz) / L2));
        const px = from[0] + dx * tt, py = from[1] + dy * tt, pz = from[2] + dz * tt; const y0 = o.char.pos[1] + 0.05, y1 = o.char.bones.headTop?.[1] ?? 1.75; const ddy = py < y0 ? y0 - py : py > y1 ? py - y1 : 0;
        const d = Math.hypot(px - h[0], pz - h[2], ddy);
        if (d < 0.3) violate('round-past-colleague', `${routeName(shooter)}→${routeName(o)}`, `${routeName(shooter)} (${shooter.pinned ? 'pinned ' : ''}${shooter.state}${shooter.standoff ? '/standoff' : ''}, muzzleDown ${shooter.muzzleDown}) fired with ${routeName(o)} (${o.script ? 'scripted' : o.state}) ${d.toFixed(2)} m off the muzzle→Sam segment at ${p2(o.char.pos)}; shooter at ${p2(shooter.char.pos)}, Sam at ${p2(game.player.char.pos)}`);
      }
    }
    // the human shield's standoff (guards.ts Guard.standoff): only ever on a living, free, ALERT man while somebody IS held (a frame's grace for a hold that ended
    // under a later man's round this frame), never under the tour flag; the record names the held man; and a man in it is at the ring (inside 7.5 m of the pair) or
    // on his way to it — three seconds' standing further off than that (not reacting, not dazzled) is the old 'planted and staring' back under a new name. A held
    // man dropped by a colleague's ROUND (downBy 'guard' out of fireWeapon, not the harness's own killGuard poke) is the shield failing: the AI's fire rule exists so
    // that never happens.
    {
      const H = game.player.holding; const heldNow = H && H.phase !== 'release' ? H.g : null;
      let anyStandoff = false;
      for (const gd of game.guards) {
        const S = gd.standoff; if (!S) { standoffFar.delete(gd); continue; }
        anyStandoff = true; const name = routeName(gd);
        if (!standoffSeen.has(S)) { standoffSeen.add(S); smells.standoffsEntered = (smells.standoffsEntered ?? 0) + 1; tl(`STANDOFF ${name} on ${routeName(S.pair)} held (side ${S.side > 0 ? '+' : '−'}, ${v3.distXZ(gd.char.pos, S.pair.char.pos).toFixed(1)} m)`); }
        const om = standoffOrbit.get(S) ?? 0; if (S.orbitM > om) { smells.standoffOrbitMetres = f2((smells.standoffOrbitMetres ?? 0) + (S.orbitM - om)); standoffOrbit.set(S, S.orbitM); }
        if (gd.state !== 'alert' || gd.held || gd.script || !gd.char.alive) violate('standoff-bad-holder', name, `${name} carries a standoff record while ${gd.state}${gd.held ? '/held' : ''}${gd.script ? '/scripted' : ''}`);
        if (game.quietUtility) violate('standoff-under-tour', name, `${name} in a standoff with quietUtility up`);
        if (!heldNow) { const acc = (standoffNoHold.get(gd) ?? 0) + 1; standoffNoHold.set(gd, acc); if (acc > 1) violate('standoff-no-hold', name, `${name} in a standoff on ${routeName(S.pair)} but nobody is held (holding ${H ? routeName(H.g) + ':' + H.phase : 'null'}) for ${acc} frames`); }
        else { standoffNoHold.set(gd, 0); if (S.pair !== heldNow) violate('standoff-wrong-pair', name, `${name}'s standoff names ${routeName(S.pair)} but ${routeName(heldNow)} is the man held`); }
        if (heldNow) {
          const d = v3.distXZ(gd.char.pos, game.player.char.pos); const going = (gd.path.length > 0 && gd.pathI < gd.path.length && gd.speed > 0.3) || gd.reactT > 0 || t < gd.dazzledUntil;
          const acc = d > 7.5 && !going ? (standoffFar.get(gd) ?? 0) + DT : 0; standoffFar.set(gd, acc);
          if (acc > 3) { standoffFar.set(gd, -30); violate('standoff-far', name, `${name} in the standoff ${d.toFixed(1)} m from the pair and going nowhere for 3 s (goal ${p2(S.orbitGoal)}, cornered ${S.cornered}, rest ${f2(S.restT)}, path ${gd.pathI}/${gd.path.length}, speed ${f2(gd.speed)}) at ${p2(gd.char.pos)}, pair at ${p2(game.player.char.pos)}`); }
          if (d < 2.6 && gd.reactT <= 0 && !(S.backing && gd.speed > 0.8)) {   // well inside the ring for two seconds running and not backing off at pace: fine if the walls (and Sam driving the pair at him) leave him nowhere a stride or two off that gains a metre — else he is just standing there
            const acc2 = (standoffNear.get(gd) ?? 0) + DT; standoffNear.set(gd, acc2);
            if (acc2 > 2) {
              standoffNear.set(gd, -30);
              let room: Vec3 | null = null; const gp = gd.char.pos;
              for (let i = 0; i < 16 && !room; i++) for (const s of [2.0, 1.2]) { const a = i * Math.PI / 8; const q: Vec3 = [gp[0] + Math.sin(a) * s, 0, gp[2] + Math.cos(a) * s]; if (!nav.isBlocked(q[0], q[2]) && nav.walkable(gp, q) && !col.segmentBlocked([gp[0], 0.5, gp[2]], [q[0], 0.5, q[2]]) && v3.distXZ(q, game.player.char.pos) > d + 0.8) { room = q; break; } }
              if (room) violate('standoff-on-top', name, `${name} in the standoff only ${d.toFixed(2)} m from Sam for 2 s with open floor to back onto at ${p2(room)} (goal ${p2(S.orbitGoal)}, backing ${S.backing}, speed ${f2(gd.speed)}, path ${gd.pathI}/${gd.path.length}) at ${p2(gp)}, Sam at ${p2(game.player.char.pos)}`);
              else { smells.standoffBoxedIn = (smells.standoffBoxedIn ?? 0) + 1; tl(`SMELL standoff boxed in: ${name} ${d.toFixed(2)} m from Sam with nowhere to back off to at ${p2(gp)}`); }
            }
          } else standoffNear.set(gd, 0);
        }
      }
      if (anyStandoff) smells.standoffSeconds2 = f2((smells.standoffSeconds2 ?? 0) + DT);
      for (const gd of game.guards) if (gd.state === 'dead' && gd.downBy === 'guard' && gd.downKind === 'shot' && heldLastFrame.has(gd) && !stimKilled.has(gd)) violate('hostage-shot', routeName(gd), `${routeName(gd)} was shot dead by a colleague's round while held in Sam's arm (the fire rule failed) at ${p2(gd.char.pos)}; shooters about: ${game.guards.filter((x: any) => x !== gd && x.state !== 'dead').map((x: any) => `${routeName(x)} ${x.standoff ? 'standoff' : x.state} @${p2(x.char.pos)}`).join(', ')}`);
      heldLastFrame = new Set(game.guards.filter((x: any) => x.held));
    }
    for (const gd of game.guards) if (gd.state !== 'dead' && gd.muzzleDown) { smells.muzzleDownSeconds = f2((smells.muzzleDownSeconds ?? 0) + DT); if (gd.char.anim.stance !== 'lowReady' && !gd.char.anim.kicking) violate('muzzle-down-not-lowered', routeName(gd), `${routeName(gd)} muzzleDown but posed ${gd.char.anim.stance}/${gd.char.anim.upper} (${gd.script ? 'scripted' : gd.state})`); }
    // the search that reads the light (guards.ts searchPlanFor / searchProbes): its queries stay inside the frame's budget, a plan only ever hangs off a searching man,
    // every spot it deals stands on walkable floor, its cursor points at a real spot in the walking / looking phases; threshold discipline (guards.ts pieDoorway):
    // a pie only ever on a free hunting man — never under the tour flag, never pinned or scripted, never in patrol / suspicion
    if (game.searchProbeN > 8) violate('search-probe-budget', String(game.searchProbeN), `${game.searchProbeN} searchers' light queries queued this frame (budget 8)`);
    smells.searchProbeMax = Math.max(smells.searchProbeMax ?? 0, game.searchProbeN);
    for (const gd of game.guards) {
      const name = routeName(gd), P = gd.searchPlan;
      if (P) {
        if (gd.state !== 'search' || gd.script) violate('search-plan-leak', name, `${name} carries a search plan in state ${gd.state}${gd.script ? ' (scripted)' : ''}`);
        if (!['sweep', 'walk', 'look', 'done'].includes(P.phase)) violate('search-plan-phase', name, `phase ${P.phase}`);
        if ((P.phase === 'walk' || P.phase === 'look') && !(P.current >= 0 && P.current < P.spots.length)) violate('search-plan-cursor', name, `${name} in phase ${P.phase} with current ${P.current} of ${P.spots.length}`);
        if (P.spots.length > 16) violate('search-plan-size', name, `${P.spots.length} spots dealt (≤ 16)`);
        for (const s of P.spots) { if (nav.isBlocked(s.pos[0], s.pos[2])) { violate('search-spot-blocked', `${name}:${s.pos[0]},${s.pos[2]}`, `${name}'s ${s.kind} spot at (${s.pos[0]}, ${s.pos[2]}) is not on walkable floor`); break; } if (!(s.dark >= 0 && s.dark <= 1 && s.cover >= 0 && s.cover <= 1 && s.plaus >= 0 && s.plaus <= 1 && Number.isFinite(s.score))) { violate('search-spot-range', name, `spot ${JSON.stringify({ ...s, samples: s.samples.length })}`); break; } }
        if (planSeen.get(gd) !== P) { planSeen.set(gd, P); smells.searchPlans = (smells.searchPlans ?? 0) + 1; smells.searchSpotsDealt = (smells.searchSpotsDealt ?? 0) + P.spots.length; }
        const vis = P.visits; const pv = planVisits.get(P) ?? 0; if (vis > pv) { smells.searchSpotsLooked = (smells.searchSpotsLooked ?? 0) + (vis - pv); planVisits.set(P, vis); }
      }
      if (gd.pieing) {
        if (game.quietUtility) violate('pie-under-tour', name, `${name} pieing the ${gd.pieing.door.def.name} door with quietUtility up`);
        if (gd.pinned || gd.script || gd.state === 'dead') violate('pie-not-free', name, `${name} pieing the ${gd.pieing.door.def.name} door while ${gd.pinned ? 'pinned' : gd.script ? 'scripted' : gd.state}`);
        if (gd.state !== 'alert' && gd.state !== 'search') violate('pie-state', name, `${name} pieing the ${gd.pieing.door.def.name} door in state ${gd.state}`);
        if (!['approach', 'slice', 'cross'].includes(gd.pieing.phase)) violate('pie-phase', name, `phase ${gd.pieing.phase}`);
        if (pieSeen.get(gd) !== gd.pieing) { pieSeen.set(gd, gd.pieing); smells.piesBegun = (smells.piesBegun ?? 0) + 1; }
        if (gd.pieing.phase === 'cross' && !pieCrossed.has(gd.pieing)) { pieCrossed.add(gd.pieing); smells.piesCrossed = (smells.piesCrossed ?? 0) + 1; smells[`pies:${gd.pieing.door.def.name}`] = (smells[`pies:${gd.pieing.door.def.name}`] ?? 0) + 1; }
      }
    }
    for (const gd of game.guards) if (gd.state !== 'dead' && gd.task?.kind === 'checkOn') { smells.checkOnSeconds = f2((smells.checkOnSeconds ?? 0) + DT); const M = gd.task.who; if (!M || !game.guards.includes(M)) violate('checkon-orphan', routeName(gd), `${routeName(gd)} walks the route of a man not in the guard list`); else if (gd.state === 'suspicious' && game.aiEnabled && (M.state !== 'dead' || M.found)) { const acc = (checkOnStale.get(gd) ?? 0) + DT; checkOnStale.set(gd, acc); if (acc > 0.1) violate('checkon-stale', routeName(gd), `${routeName(gd)} still on checkOn for ${routeName(M)} who is ${M.state}${M.found ? ' (found)' : ''} for ${acc.toFixed(2)} s`); } else checkOnStale.set(gd, 0); }
    // NaN / range
    if (!finiteChar(game.player.char)) violate('nan', 'player', `player pos/yaw not finite: ${game.player.char.pos} yaw ${game.player.char.bodyYaw} aim ${game.player.char.aimYaw}/${game.player.char.aimPitch}`);
    for (const b of ['hips', 'head', 'handR']) { const q = game.player.char.bones[b]; if (q && !(Number.isFinite(q[0]) && Number.isFinite(q[1]) && Number.isFinite(q[2]))) { violate('nan', `player-bone-${b}`, `player bone ${b} = ${q}`); break; } }
    const pp = game.player.char.pos; if (pp[0] < -0.5 || pp[0] > 40.5 || pp[2] < -0.5 || pp[2] > 28.5) violate('out-of-world', 'player', `player at ${p2(pp)}`);
    if (!game.player.down && (pp[0] < 0.42 - 1e-3 || pp[0] > 39.58 + 1e-3 || pp[2] < 0.42 - 1e-3 || pp[2] > 27.58 + 1e-3)) violate('off-the-slab', 'player', `player's circle over the world edge at (${pp[0].toFixed(3)}, ${pp[2].toFixed(3)}) (picking ${!!game.player.picking} kick ${!!game.player.kick} takedown ${!!game.player.takedown} sprint ${game.player.sprinting})`);   // player.ts collidePlayer clamps a live player's 0.42 m circle onto the 40 × 28 slab every frame
    game.guards.forEach((gd: any, i: number) => {
      const c = gd.char; const name = routeName(gd);
      if (!finiteChar(c) || !Number.isFinite(gd.speed) || !Number.isFinite(gd.awareness)) violate('nan', `guard-${name}`, `guard ${name} pos ${c.pos} yaw ${c.bodyYaw} aim ${c.aimYaw}/${c.aimPitch} speed ${gd.speed} aw ${gd.awareness}`);
      const hb = c.bones.hips; if (hb && !(Number.isFinite(hb[0]) && Number.isFinite(hb[1]) && Number.isFinite(hb[2]))) violate('nan', `guard-bones-${name}`, `guard ${name} hips bone ${hb} (state ${gd.state})`);
      if (c.pos[0] < -0.5 || c.pos[0] > 40.5 || c.pos[2] < -0.5 || c.pos[2] > 28.5) violate('out-of-world', `guard-${name}`, `guard ${name} at ${p2(c.pos)} (${gd.state})`);
      if (!['patrol', 'suspicious', 'alert', 'search', 'dead'].includes(gd.state)) violate('bad-state', name, `state ${gd.state}`);
      if (gd.awareness < -1e-6 || gd.awareness > 1 + 1e-6) violate('awareness-range', name, `awareness ${gd.awareness}`);
      if (Math.abs(gd.speed) > 6) violate('speed-range', name, `speed ${gd.speed}`);
      if (gd.script && gd.state === 'dead') violate('dead-scripted', name, `guard ${name} is dead but still carries a script (goal ${p2(gd.script.goal)}, stance ${gd.script.stance}); clearing=${game.clearingSummary() || 'none'}`);
      if ((gd.state === 'dead') !== !c.alive) violate('alive-mismatch', name, `state ${gd.state} alive ${c.alive}`);
      if (gd.state !== 'dead' && gd.leader && (gd.leader === gd || (gd.leader.state === 'dead' && (gd.leader.found || game.alarm.episode)) || !game.guards.includes(gd.leader)))   /* a leader nobody knows is dead keeps the role by design (superviseLockdown) */ { leaderBad++; if (leaderBad >= 2) violate('bad-leader', name, `follower ${name}'s leader is ${gd.leader === gd ? 'himself' : gd.leader.state === 'dead' ? 'dead' : 'not in the guard list'} for ${leaderBad} frames (esc ${game.escalation}, quiet ${game.quietUtility})`); }
      if (gd.state === 'dead' && (gd.leader || gd.post)) smells.staleDutyOnCorpse = (smells.staleDutyOnCorpse ?? 0) + DT;   // harmless (everything skips the dead) but note it
      if (gd.post && gd.leader) violate('post-and-leader', name, `both a post (${gd.post.name}) and a leader`);
      // KO vs dead (combat.ts killGuard's side record): every downed man says how and by whom, no living man does; the witness record (game.ts Witness) only ever
      // names a man who IS down — as the kind he went down by — and a found / called-in body is remembered breathing exactly when he is (isBreathing)
      if (gd.state === 'dead' && (!gd.downKind || !gd.downBy)) violate('down-kind-missing', name, `${name} is down but downKind ${gd.downKind} / downBy ${gd.downBy}`);
      if (gd.state !== 'dead' && (gd.downKind || gd.downBy)) violate('down-kind-on-living', name, `${name} is ${gd.state} but carries downKind ${gd.downKind} / downBy ${gd.downBy}`);
      {
        const Wt = gd.witness;
        if (!Wt || !Wt.heard) violate('witness-missing', name, `${name} has no witness record`);
        else {
          const byCall = (cs: string) => game.guards.find((x: any) => x.callsign === cs);
          if (Wt.sawAct) { const v = byCall(Wt.sawAct.victim); if (!v || v.state !== 'dead') violate('witness-sawact-not-down', name, `${name} remembers Sam ${Wt.sawAct.kind} ${Wt.sawAct.victim}, who is ${v ? v.state : 'not on the roster'}`); else if (v.downKind !== Wt.sawAct.kind) violate('witness-sawact-kind', name, `${name} remembers ${Wt.sawAct.victim} ${Wt.sawAct.kind} but he went down ${v.downKind}`); if (v === gd) violate('witness-sawact-self', name, `${name} remembers his own end`); if (Wt.sawAct.t > t + 1e-6) violate('witness-time', name, `sawAct in the future (${Wt.sawAct.t} > ${t})`); }
          for (const k of ['sawBody', 'calledToBody'] as const) { const r = Wt[k]; if (!r) continue; const v = byCall(r.victim); if (!v || v.state !== 'dead' || !v.found) violate(`witness-${k}-bad`, name, `${name}'s ${k} names ${r.victim}: ${v ? `${v.state} found=${v.found}` : 'not on the roster'}`); else if (r.breathing !== isBreathing(v)) violate(`witness-${k}-breathing`, name, `${name}'s ${k} says ${r.victim} breathing=${r.breathing} but isBreathing=${isBreathing(v)} (${v.downKind})`); }
          const H = Wt.heard; if (H.shots < 0 || H.kick < 0 || H.bang < 0 || H.small < 0 || Wt.dazzled < 0 || Wt.wasHeld < 0 || !(Wt.peakAwareness >= 0 && Wt.peakAwareness <= 1 + 1e-6)) violate('witness-range', name, `${name}: ${JSON.stringify({ ...Wt, heard: H })}`);
          if (Wt.alertedBy && !['sight', 'sound', 'radio', 'body', 'held'].includes(Wt.alertedBy)) violate('witness-alertedby', name, `alertedBy ${Wt.alertedBy}`);
          if (Wt.sawHeld && (!byCall(Wt.sawHeld.who) || Wt.sawHeld.t > t + 1e-6 || byCall(Wt.sawHeld.who) === gd)) violate('witness-sawheld-bad', name, `${name}: sawHeld ${JSON.stringify(Wt.sawHeld)} — not a colleague on the roster, or in the future`);
          if (Wt.sawHeld && !sawHeldSeen.has(Wt.sawHeld)) { sawHeldSeen.add(Wt.sawHeld); smells.witnessSawHeld = (smells.witnessSawHeld ?? 0) + 1; }
          if (Wt.wasHeld > (heldCount.get(gd) ?? 0)) violate('witness-washeld-uncounted', name, `${name}: wasHeld ${Wt.wasHeld} but the harness saw him grabbed ${heldCount.get(gd) ?? 0}×`);
          if (Wt.sawAct && sawActSeen.get(gd) !== Wt.sawAct) { sawActSeen.set(gd, Wt.sawAct); smells.witnessSawAct = (smells.witnessSawAct ?? 0) + 1; smells[`witnessSaw:${Wt.sawAct.kind}`] = (smells[`witnessSaw:${Wt.sawAct.kind}`] ?? 0) + 1; }
          // nothing is remembered with the AI off: the record as it stood just before a step taken with aiEnabled=false must come out of it unchanged (snapshots per Guard
          // object taken right before the step, so a stimulus that wrote legitimately while the AI was still on, or fresh men after a reset, cannot false-alarm)
          if (steppedAiOff) { const pre = witnessSnap.get(gd); const now = JSON.stringify(Wt); if (pre !== undefined && pre !== now) violate('witness-under-ai-off', name, `${name}'s witness record changed in a frame stepped with the AI off: ${pre} → ${now}`); }
        }
      }
      // the interrogation's progress (game.ts Talk): present on every man, its stage only ever forward for the same Guard object, presses ≥ stage, heldSince never in the future
      {
        const Tk = gd.talk;
        if (!Tk || !(Tk.said instanceof Set)) violate('talk-missing', name, `${name} has no talk record`);
        else {
          const prevS = talkStage.get(gd); if (prevS !== undefined && Tk.stage < prevS) violate('talk-stage-back', name, `${name}'s interrogation stage went ${prevS} → ${Tk.stage}`); talkStage.set(gd, Tk.stage);
          if (Tk.presses < Tk.stage || Tk.stage < 0 || Tk.heldSince > t + 1e-6 || Tk.lastT > t + 1e-6) violate('talk-range', name, `${name}: stage ${Tk.stage} presses ${Tk.presses} heldSince ${Tk.heldSince} lastT ${Tk.lastT} at t ${t.toFixed(2)}`);
        }
      }
      // the found-body script: a living man's duty must point at a corpse still on the floor, and only a suspicious (or about-to-be: patrol with the news, scripted about to break off) man carries one
      if (gd.state !== 'dead' && gd.bodyDuty) {
        const B = gd.bodyDuty.body;
        if (!B || B.state !== 'dead' || !game.guards.includes(B)) violate('bad-body-duty', name, `${name}'s body duty points at ${B ? routeName(B) + ' (' + B.state + (game.guards.includes(B) ? '' : ', not in the guard list') + ')' : 'nothing'}`);
        if (gd.state === 'alert' || gd.state === 'search' && !gd.script) { const acc = (dutyBad.get(gd) ?? 0) + DT; dutyBad.set(gd, acc); if (acc > 0.5) violate('body-duty-state', name, `${name} carries a body duty (${gd.bodyDuty.role}:${gd.bodyDuty.stage}) in state ${gd.state} for ${acc.toFixed(1)} s`); } else dutyBad.set(gd, 0);
        if (game.time - gd.bodyDuty.t0 > 60) violate('body-duty-stale', name, `${name}'s body duty is ${(game.time - gd.bodyDuty.t0).toFixed(0)} s old (${gd.bodyDuty.role}:${gd.bodyDuty.stage}, state ${gd.state})`);
      }
      // lingering on a found body: a calm-ish living man planted within 1.2 m of a called-in corpse for 8 s running reads as milling on it (smell, not a violation: a drill mark may sit by a
      // body, and the finder covering him from arm's length because the walls left no better spot is doing his job)
      if (gd.state !== 'dead' && gd.state !== 'alert' && !game.player.down && !(gd.bodyDuty?.role === 'finder' && gd.bodyDuty.stage === 'cover')) {
        const B = nearKnownBody(game, c.pos, 1.2); const acc = B ? (lingerT.get(gd) ?? 0) + DT : 0; lingerT.set(gd, acc);
        if (acc > 8 && t - (lingerRep.get(gd) ?? -100) > 30) { lingerRep.set(gd, t); smells.lingerOnBody = (smells.lingerOnBody ?? 0) + 1; if (smells.lingerOnBody <= 6) tl(`SMELL linger-on-body ${name}: ${acc.toFixed(0)} s within 1.2 m of ${routeName(B)}'s body (${gd.script ? 'scripted ' + (game.clearingSummary() || '') : gd.state}${gd.bodyDuty ? ' duty ' + gd.bodyDuty.role + ':' + gd.bodyDuty.stage : ''})`); }
      }
      // planted at a doorway: alert, free to move (not pinned / reacting / dazzled / scripted), covering < 0.25 m/s within 1.5 m of a doorway's middle for 6 s running,
      // and in all that time not one round of anybody's reaching Sam — the playtest's man rooted at the door with his muzzle through the frame (a jamb or the leaf in
      // his line of fire but not his line of sight, or a fix on the threshold he would neither walk through nor give up). Firing and hitting (god mode keeps Sam
      // up) is a fight, not a plant; a man whose target is down is covering a body; and one holding fire because the colleague in Sam's arm stands in his line
      // (guards.ts muzzleCheck counts the held man — the human shield's standoff, which the AI does not yet play out: docs/internal/grab-interrogate-design.md
      // §2.2, slice 3) is standing on purpose, like a pinned man.
      if (gd.state === 'alert' && !gd.script && !gd.pinned && gd.reactT <= 0 && !(t < gd.dazzledUntil) && !game.player.down && game.aiEnabled && gd.speed < 0.25) {
        const dp = doorPlant.get(gd) ?? { t: 0, lastReport: -100 };
        const nearDoor = doors.list.find((d: any) => v3.distXZ(c.pos, d.frameCentre) < 1.5);
        const Hh = game.player.holding; const shield = !!gd.standoff || (!!Hh && gd.muzzleDown && v3.distXZ(Hh.g.char.pos, game.player.char.pos) < 0.8);   // (in the standoff round the pair — its own invariants watch that — or his gun down for the man in the arm)
        if (shield) smells.standoffSeconds = f2((smells.standoffSeconds ?? 0) + DT);
        dp.t = nearDoor && !shield && t - lastPlayerHitT > 0.5 ? dp.t + DT : 0; doorPlant.set(gd, dp);
        if (dp.t > 6 && t - dp.lastReport > 30) { dp.lastReport = t; violate('alert-planted-at-door', `${name}:${nearDoor.def.name}`, `guard ${name} alert and rooted ${v3.distXZ(c.pos, nearDoor.frameCentre).toFixed(2)} m from the ${nearDoor.def.name} doorway (leaf ${nearDoor.angle.toFixed(2)}${nearDoor.latched ? 'L' : ''}${nearDoor.locked ? 'k' : ''}) for ${dp.t.toFixed(1)} s with nothing reaching Sam — Sam ${v3.distXZ(c.pos, game.player.char.pos).toFixed(1)} m off at ${p2(game.player.char.pos)} vis ${game.player.visibility.toFixed(2)}, seen this frame ${gd.sawPlayerThisFrame}, last seen ${(t - gd.lastSeenT).toFixed(1)} s ago, fix ${p2(gd.lastKnown)}, path ${gd.pathI}/${gd.path.length}, shotBlockedT ${(gd.shotBlockedT ?? 0).toFixed(1)}, stallT ${gd.alertStallT.toFixed(1)}`); }
      } else doorPlant.delete(gd);
      // stuck: wanted to move (speed > 0.3) essentially the whole 4 s window while scripted / pathing, and ended it within a hand's breadth of where it began (the
      // report carries his biggest excursion and the ground he covered in the window: ~0 / ~0 is a wedge, metres of either is a man sent out and straight back)
      if (gd.state !== 'dead') {
        const pathing = (gd.path.length > 0 && gd.pathI < gd.path.length) || (gd.script && gd.script.goal && !gd.script.arrived);
        let s = stuck.get(gd); if (!s) { s = { t0: t, p0: v3.copy(c.pos), last: v3.copy(c.pos), ground: 0, reach: 0, want: 0, lastReport: -100 }; stuck.set(gd, s); }
        if (gd.speed > 0.3 && pathing) s.want += DT;
        s.ground += v3.distXZ(c.pos, s.last); s.last = v3.copy(c.pos); s.reach = Math.max(s.reach, v3.distXZ(c.pos, s.p0));
        if (t - s.t0 >= 4) {
          const moved = v3.distXZ(c.pos, s.p0);   // net: where the window ends against where it began (a wedged man see-sawing on followPath's sidestep still ends about where he started; the excursion and the ground he covered go in the report to tell that from an out-and-back errand)
          if (s.want >= 3.8 && moved < 0.05 && s.reach >= 1.0 && t - s.lastReport > 20) { s.lastReport = t; smells.outAndBack = (smells.outAndBack ?? 0) + 1; }   // sent out ≥ 1 m and straight back inside 4 s: an errand / re-plan, not a wedge (a see-sawing wedged man never gets a metre away) — counted, not a violation
          else if (s.want >= 3.8 && moved < 0.05 && t - s.lastReport > 20) {
            s.lastReport = t;
            const nearDoors = doors.list.filter((d: any) => Math.hypot(d.def.hinge[0] - c.pos[0], d.def.hinge[1] - c.pos[2]) < d.def.width + 0.8).map((d: any) => `${d.def.name}@${d.angle.toFixed(2)}${Math.abs(d.angle - d.maxA) < 0.02 || Math.abs(d.angle - d.minA) < 0.02 ? '(AT STOP)' : ''}${d.latched ? 'L' : ''}`).join(',') || 'none';
            const q = v3.copy(c.pos); const inStatic = col.collideCircle(q, c.radius, 0.2, 1.5, 1) ? v3.distXZ(q, c.pos).toFixed(2) : '0';
            const nearProps = game.props.props.filter((p: any) => Math.hypot(p.x - c.pos[0], p.z - c.pos[2]) < Math.max(p.def.half[0], p.def.half[1]) + 0.75).map((p: any) => `${p.def.name}(${p.def.kind} ${p.def.mass}kg)@${p.x.toFixed(2)},${p.z.toFixed(2)}${Math.hypot(p.x - p.home[0], p.z - p.home[1]) > 0.05 ? ' moved ' + Math.hypot(p.x - p.home[0], p.z - p.home[1]).toFixed(2) + 'm' : ''}`).join(',') || 'none';
            violate('stuck', `${name}:${gd.script ? 'script' : gd.state}`, `guard ${name} ordered at ${gd.speed.toFixed(2)} m/s for ${s.want.toFixed(1)} of 4 s but ended it ${moved.toFixed(3)} m from where it began (excursion ${s.reach.toFixed(2)} m, ground covered ${s.ground.toFixed(2)} m) at ${p2(c.pos)} — ${gd.script ? `scripted goal ${p2(gd.script.goal)} stance ${gd.script.stance} (${game.clearingSummary() || 'no clearing'})` : `${gd.state} path ${gd.pathI}/${gd.path.length} next wp ${p2(gd.path[gd.pathI])} → ${p2(gd.pathGoal)}`} stuckT ${gd.stuckT.toFixed(2)}; doors in reach: ${nearDoors}; props in reach: ${nearProps}; static overlap ${inStatic} m; nav cell blocked: ${nav.isBlocked(c.pos[0], c.pos[2])}; path to goal exists: ${gd.pathGoal ? !!nav.findPath(c.pos, gd.pathGoal) : 'n/a'}`);
          }
          s.t0 = t; s.p0 = v3.copy(c.pos); s.want = 0; s.ground = 0; s.reach = 0;
        }
        // softer: has somewhere to be but < 0.25 m progress in 8 s (dithering / wedged at low speed)
        let q = noprog.get(gd); if (!q) { q = { t0: t, p0: v3.copy(c.pos), lastReport: -100 }; noprog.set(gd, q); }
        const alertHolding = gd.state === 'alert' && (gd.reactT > 0 || gd.pinned || game.player.down || t - gd.lastSeenT < 0.6);   // covering / shooting from where he stands is not 'going somewhere'
        const wantsToGo = pathing && !(gd.state === 'suspicious' && gd.reactT > 0) && !alertHolding && !(t < gd.dazzledUntil) && !gd.hold && !(gd.state === 'patrol' && gd.wait > 0);
        if (!wantsToGo) { q.t0 = t; q.p0 = v3.copy(c.pos); }
        else if (t - q.t0 >= 8) { const moved = v3.distXZ(c.pos, q.p0); if (moved < 0.25 && t - q.lastReport > 30) { q.lastReport = t; smells.noProgress++; if (smells.noProgress <= 10) tl(`SMELL no-progress ${name}: ${moved.toFixed(2)} m in 8 s, ${gd.script ? `scripted → ${p2(gd.script.goal)} arrived=${gd.script.arrived}` : `${gd.state} path ${gd.pathI}/${gd.path.length} → ${p2(gd.pathGoal)}`} at ${p2(c.pos)} speed ${gd.speed.toFixed(2)}`); } q.t0 = t; q.p0 = v3.copy(c.pos); }
      }
      // pair separation
      for (let j = i + 1; j < game.guards.length; j++) {
        const o = game.guards[j]; if (gd.state === 'dead' || o.state === 'dead') continue;
        const key = `${name}|${routeName(o)}`; const d = v3.distXZ(c.pos, o.char.pos);
        if (d < 0.3) { const acc = (closeT.get(key) ?? 0) + DT; closeT.set(key, acc); if (acc > 2 && t - (closeReported.get(key) ?? -100) > 30) { closeReported.set(key, t); violate('guards-overlap', key, `${key} within ${d.toFixed(2)} m for ${acc.toFixed(1)} s at ${p2(c.pos)} (${gd.script ? 'scripted' : gd.state} / ${o.script ? 'scripted' : o.state}; ${game.clearingSummary() || 'no clearing'})`); } }
        else closeT.set(key, 0);
      }
    });
    if (!game.guards.some((gd: any) => gd.state !== 'dead' && gd.leader && (gd.leader === gd || gd.leader.state === 'dead'))) leaderBad = 0;
    // per-frame displacement: nobody alive moves more than POP in a frame unless the harness teleported them (0.3 m at 60 Hz; a stride plus a leaf's capped shove
    // scale with the frame, so the bar does too at a coarser --hz); through a wall is its own kind
    {
      const POP = Math.max(0.3, 12 * DT + 0.06);
      const plc = game.player.char;
      const prev = lastPos.get('player'); const cur = v3.copy(plc.pos);
      if (prev && !teleportedThisFrame && !game.player.down && game.mission === missionRef) {
        const d = v3.distXZ(prev, cur);
        if (d > POP) { const wall = col.segmentBlocked([prev[0], 0.6, prev[2]], [cur[0], 0.6, cur[2]]) && col.segmentBlocked([prev[0], 1.2, prev[2]], [cur[0], 1.2, cur[2]]); violate(wall ? 'popped-through-wall' : 'popped', 'player', `player jumped ${d.toFixed(2)} m in one frame ${p2(prev)} → ${p2(cur)}${wall ? ' THROUGH static geometry' : ''} (wall ${game.player.wall ? `held u=${game.player.wall.u.toFixed(2)} settle=${game.player.wall.settle.toFixed(2)}` : 'free'}; picking ${!!game.player.picking} kick ${!!game.player.kick} takedown ${!!game.player.takedown}; nearest guard ${Math.min(...living().map((g: any) => v3.distXZ(g.char.pos, cur))).toFixed(2)} m)`); }
        // and however short the move: a STATIC (not a leaf) between where he stood a frame ago and where he stands now, at knee and at hip height, is a wall crossed
        else if (d > 1e-3 && col.segmentBlocked([prev[0], 0.45, prev[2]], [cur[0], 0.45, cur[2]], 0) && col.segmentBlocked([prev[0], 0.9, prev[2]], [cur[0], 0.9, cur[2]], 0) && !col.segmentBlockedDynamic([prev[0], 0.45, prev[2]], [cur[0], 0.45, cur[2]]) && !col.segmentBlockedDynamic([prev[0], 0.9, prev[2]], [cur[0], 0.9, cur[2]])) {
          const nearDoors = doors.list.filter((dd: any) => Math.hypot(dd.def.hinge[0] - cur[0], dd.def.hinge[1] - cur[2]) < dd.def.width + 0.8).map((dd: any) => `${dd.def.name}@${dd.angle.toFixed(2)}`).join(',') || 'none';
          violate('player-crossed-wall', 'player', `player crossed static geometry between frames: ${p2(prev)} → ${p2(cur)} (${d.toFixed(3)} m; crouch ${game.player.crouch} sprint ${game.player.sprinting} picking ${!!game.player.picking} kick ${!!game.player.kick} takedown ${!!game.player.takedown} drag ${!!game.player.dragging}; nearest guard ${Math.min(...living().map((g: any) => v3.distXZ(g.char.pos, cur)), 99).toFixed(2)} m; doors in reach ${nearDoors})`);
        }
      }
      lastPos.set('player', cur);
      for (const gd of game.guards) {
        const p0 = lastPos.get(gd); const c1 = v3.copy(gd.char.pos);
        if (p0 && gd.state !== 'dead') { const d = v3.distXZ(p0, c1); if (d > POP) { const wall = col.segmentBlocked([p0[0], 0.6, p0[2]], [c1[0], 0.6, c1[2]]); violate(wall ? 'popped-through-wall' : 'popped', routeName(gd), `guard ${routeName(gd)} jumped ${d.toFixed(2)} m in one frame ${p2(p0)} → ${p2(c1)}${wall ? ' THROUGH static geometry' : ''} (${gd.script ? 'scripted ' + (game.clearingSummary() || '') : gd.state})`); } }
        lastPos.set(gd, c1);
      }
      teleportedThisFrame = false;
    }
    // escalation
    const esc = game.escalation;
    if (![0, 1, 2].includes(esc)) violate('esc-range', String(esc), `escalation ${esc}`);
    if (esc === 0 && game.escalationT !== -1) violate('esc-clock', 'level0-clock', `level 0 but escalationT=${game.escalationT} (t=${t.toFixed(1)})`);
    if (esc > 0 && !(game.escalationT > 0)) violate('esc-clock', 'levelN-noclock', `level ${esc} but escalationT=${game.escalationT}`);
    const hunting = game.guards.some((gd: any) => gd.state === 'alert' || gd.state === 'search');
    if (!game.quietUtility && esc > 0 && t >= game.escalationT && !game.alarm.episode && !hunting && !stillLooking(game)) { if (++overdueFrames >= 2) violate('esc-clock', 'overdue-stepdown', `level ${esc} clock lapsed ${(t - game.escalationT).toFixed(1)} s ago, no episode, nobody hunting or out looking for a silent man, yet no step-down for ${overdueFrames} frames`); } else overdueFrames = 0;   // (one frame is legit: a clear finishing inside updateEscalation puts the pair back on patrol after this frame's step-down check; a route walk for a silent colleague / the net asking after one holds the level by design — guards.ts stillLooking)
    if (!game.quietUtility && game.aiEnabled) { const R = rollcallState(game); if (R.stage !== 'idle' && t - R.t0 > 60) violate('rollcall-stuck', R.stage, `radio check stuck at '${R.stage}' for ${(t - R.t0).toFixed(0)} s (missing ${R.missing ? routeName(R.missing) + ' ' + R.missing.state + (R.missing.found ? ' found' : '') : 'none'}, caller ${R.caller ? routeName(R.caller) + ' ' + R.caller.state : 'none'})`); }
    for (const gd of game.guards) if (gd.state !== 'dead' && gd.task?.kind === 'checkOn' && t - gd.task.t0 > 200) violate('checkon-overlong', routeName(gd), `${routeName(gd)} has walked ${routeName(gd.task.who)}'s route for ${(t - gd.task.t0).toFixed(0)} s (cap 150 + slack; state ${gd.state})`);
    if (game.mission.debrief) { game.mission.debrief = false; smells.debriefs = (smells.debriefs ?? 0) + 1; tl(`DEBRIEF: ${game.objectiveText()}`); }
    if (esc < 2 && !game.quietUtility && game.guards.some((gd: any) => gd.state !== 'dead' && (gd.leader || gd.post))) violate('lockdown-duty-leak', `level${esc}`, `level ${esc} but somebody still has a leader/post: ${game.escalationSummary()}`);
    // clearing
    if (game.clearing && (game.quietUtility || !game.aiEnabled)) violate('clearing-while-off', game.quietUtility ? 'quietUtility' : 'aiOff', `clearing active (${game.clearingSummary() || game.clearing.stage}) while ${game.quietUtility ? 'quietUtility' : 'aiEnabled=false'}`);
    if (game.clearing && game.clearing.stage !== 'done' && game.clearing.stage !== 'muster') for (const gd of [game.clearing.a, game.clearing.b]) if (gd && gd.state !== 'dead' && !gd.script && game.guards.includes(gd)) violate('clearing-unscripted', routeName(gd), `clearing at ${game.clearing.stage} but ${routeName(gd)} has no script (state ${gd.state})`);
    for (const gd of game.guards) if (gd.script && !game.clearing && gd.state !== 'dead') violate('orphan-script', routeName(gd), `${routeName(gd)} scripted (goal ${p2(gd.script.goal)}, stance ${gd.script.stance}) but no clearing is running (state ${gd.state})`);
    // doors
    for (const d of doors.list) {
      const n = d.def.name;
      if (!Number.isFinite(d.angle) || !Number.isFinite(d.vel) || !Number.isFinite(d.pick)) violate('nan', `door-${n}`, `door ${n} angle ${d.angle} vel ${d.vel} pick ${d.pick}`);
      if (d.latched && Math.abs(d.angle) > 1e-3) violate('door-latched-open', n, `door ${n} latched at angle ${d.angle.toFixed(4)}`);
      if (d.locked && d.lockBroken) violate('door-locked-and-broken', n, `door ${n} locked && lockBroken`);
      if (d.pick < 0 || d.pick > 1) violate('door-pick-range', n, `door ${n} pick ${d.pick}`);
      if (d.angle < d.minA - 1e-6 || d.angle > d.maxA + 1e-6) violate('door-angle-range', n, `door ${n} angle ${d.angle.toFixed(3)} outside [${d.minA}, ${d.maxA}]`);
      if (Math.abs(d.vel) > 40) violate('door-vel', n, `door ${n} vel ${d.vel.toFixed(1)}`);
    }
    // player
    const pl = game.player;
    if (pl.down !== !pl.char.alive) violate('alive-mismatch', 'player', `down ${pl.down} alive ${pl.char.alive}`);
    if (pl.picking && pl.kick) violate('player-pick-and-kick', 'player', 'picking and kicking at once');
    // back to the wall: holding ⇒ a live, free man (nothing else planted), the animator in its wall hold, and his body where the hold says — on the plane within the shove
    // allowance and inside the traced run (else moveOnWall lets go); free ⇒ nothing of the hold left on the animator or the camera
    if (pl.wall) {
      const w = pl.wall; const p = pl.char.pos; const off = p[0] * w.n[0] + p[2] * w.n[2] - (w.d + 0.45), u = p[0] * w.along[0] + p[2] * w.along[2];
      if (pl.down || pl.picking || pl.kick || pl.takedown || pl.dragging || pl.sprinting) violate('wall-while-busy', 'player', `holding a wall while down ${pl.down} picking ${!!pl.picking} kick ${!!pl.kick} takedown ${!!pl.takedown} drag ${!!pl.dragging} sprint ${pl.sprinting}`);
      if (![w.n[0], w.n[2], w.d, w.lo, w.hi, w.u, w.v, w.off, w.peek].every(Number.isFinite) || Math.abs(Math.hypot(w.n[0], w.n[2]) - 1) > 1e-6 || w.hi - w.lo < 0.84) violate('wall-bad-hold', 'player', `hold ${JSON.stringify(w)}`);
      if (w.settle >= 1 && (off < -0.07 || off > 0.27 ||   /* −0.07: furniture resting on him may press him a few cm toward the wall inside the static push-out's tolerance — the wall itself is asserted by 'in-geometry' at 3 cm */ u < w.lo + 0.42 - 0.27 || u > w.hi - 0.42 + 0.27)) violate('wall-off-plane', 'player', `holding n (${w.n[0].toFixed(2)}, ${w.n[2].toFixed(2)}) face ${w.d.toFixed(2)} run ${w.lo.toFixed(2)}‥${w.hi.toFixed(2)} but standing ${off.toFixed(3)} m off the plane / at station ${u.toFixed(2)} at ${p2(p)} (settle ${w.settle.toFixed(2)}, off ${w.off.toFixed(2)}; nearest guard ${Math.min(...living().map((g: any) => v3.distXZ(g.char.pos, p)), 99).toFixed(2)} m)`);
      if (pl.char.anim.stance !== 'wall') violate('wall-stance', 'player', `holding a wall in stance '${pl.char.anim.stance}'`);
      if (w.peek > 0 && !w.atEdge && w.peek > wallPeekPrev + 1e-6) violate('wall-peek', 'player', `peek rising (${w.peek.toFixed(2)}) away from an open end (atEdge ${w.atEdge})`);
      wallPeekPrev = w.peek;
    } else { wallPeekPrev = 0; if (!pl.down && !game.quietUtility && (Math.abs(pl.char.anim.lookYawExtra) > 1e-6 || W.cam.peekOffset)) violate('wall-leftover', 'player', `free but neck craned ${pl.char.anim.lookYawExtra.toFixed(2)} / camera peekOffset ${JSON.stringify(W.cam.peekOffset)}`); }
    // a man in his arm (player.ts Hold): Player.holding ⇔ exactly one Guard.held, both naming each other; the man alive (a corpse in the arm is dropped the next
    // frame), unscripted, pathless, blind to Sam; on his station HOLD.dist dead ahead on Sam's facing once held (a hand's slack for a chair shoving him); no hold
    // while down or with the hands / feet in anything else; and nothing of it left on either animator once it is over
    {
      const H = pl.holding; const heldMen = game.guards.filter((x: any) => x.held);
      if (heldMen.length > 1) violate('held-multiple', 'guards', `${heldMen.length} men carry .held: ${heldMen.map(routeName).join(', ')}`);
      for (const x of heldMen) if (!H || H.g !== x) violate('held-orphan', routeName(x), `${routeName(x)} carries .held (${x.held.phase}) but Player.holding is ${H ? routeName(H.g) : 'null'}`);
      if (H) {
        const gd = H.g, name = routeName(gd);
        smells.holdSeconds = f2((smells.holdSeconds ?? 0) + DT);
        if (!game.guards.includes(gd)) violate('holding-orphan', name, `Player.holding names ${name}, who is not on the roster`);
        else if (gd.state === 'dead') { holdDeadFrames++; if (holdDeadFrames > 1) violate('holding-corpse', name, `holding ${name} who is dead (${gd.downKind}) for ${holdDeadFrames} frames`); }
        else {
          holdDeadFrames = 0;
          if (gd.held?.by !== 'player') violate('holding-mismatch', name, `Player.holding names ${name} but his .held is ${JSON.stringify(gd.held)}`);
          else { if (gd.held.phase !== H.phase) violate('hold-phase-mismatch', name, `holding phase ${H.phase} vs his ${gd.held.phase}`); if (gd.held.variant !== H.variant) violate('hold-variant-mismatch', name, `holding variant ${H.variant} vs his ${gd.held.variant}`); }
          if (!['grab', 'held', 'choke', 'whip', 'release'].includes(H.phase) || !(H.t >= 0) || H.t > (H.phase === 'grab' ? HOLD.secs.grab : H.phase === 'choke' ? HOLD.secs.choke : H.phase === 'whip' ? HOLD.secs.whip : H.phase === 'release' ? HOLD.secs.release : Infinity) + 2 * DT + 1e-6) violate('hold-phase', name, `phase ${H.phase} t ${f2(H.t)}`);
          if ((H.phase === 'choke' && H.variant !== 'arm') || (H.phase === 'whip' && H.variant !== 'gun')) violate('hold-phase-variant', name, `phase ${H.phase} in the ${H.variant} variant`);
          if (H.variant !== 'arm' && H.variant !== 'gun') violate('hold-variant', name, `variant ${H.variant}`);
          if (H.phase === 'held' && H.t > 0.6 && (H.variant === 'gun') !== !pl.char.anim.hideHeldItem) violate('hold-prop', name, `${H.variant} hold but hideHeldItem ${pl.char.anim.hideHeldItem} (the pistol ${pl.char.anim.hideHeldItem ? 'on the thigh' : 'in the hand'})`);   // the gun variant keeps the pistol drawn up by his head; the arm variant's hands are both on him
          if (H.variant === 'gun' && (pl.slot !== 1 || pl.holstered)) violate('gunhold-unarmed', name, `gun hold with slot ${pl.slot} holstered ${pl.holstered}`);
          if (!(H.gunOut >= 0 && H.gunOut <= 1 + 1e-6) || (H.variant === 'arm' && H.gunOut > 0.02)) violate('hold-gunout', name, `gunOut ${H.gunOut} in the ${H.variant} variant`);
          if (gd.script) violate('held-scripted', name, `${name} held but scripted (goal ${p2(gd.script.goal)})`);
          if (gd.path.length) violate('held-pathing', name, `${name} held but carries a path ${gd.pathI}/${gd.path.length}`);
          if (gd.sawPlayerThisFrame) violate('held-perceives', name, `${name} held but sawPlayerThisFrame`);
          if (gd.bodyDuty || gd.pieing || gd.searchPlan) violate('held-duty', name, `${name} held but carries ${gd.bodyDuty ? 'a body duty' : gd.pieing ? 'a pie' : 'a search plan'}`);
          const d = v3.distXZ(pl.char.pos, gd.char.pos); const dyaw = Math.abs(Math.atan2(Math.sin(pl.char.bodyYaw - gd.char.bodyYaw), Math.cos(pl.char.bodyYaw - gd.char.bodyYaw)));
          if ((H.phase === 'held' || H.phase === 'choke' || H.phase === 'whip') && (Math.abs(d - HOLD.dist) > 0.12 || dyaw > 0.05)) violate('held-off-station', name, `${name} ${d.toFixed(3)} m from Sam (station ${HOLD.dist}), yaws ${dyaw.toFixed(3)} apart, in phase ${H.phase} at ${p2(gd.char.pos)}; nearest other guard ${Math.min(...living().filter((o: any) => o !== gd).map((o: any) => v3.distXZ(o.char.pos, gd.char.pos)), 99).toFixed(2)} m; doors near ${doors.list.filter((dd: any) => v3.distXZ(dd.frameCentre, gd.char.pos) < 1.5).map((dd: any) => `${dd.def.name}@${dd.angle.toFixed(2)}`).join(',') || 'none'}`);
          if (H.phase === 'grab' && d > 1.4) violate('grab-far', name, `grabbing ${name} from ${d.toFixed(2)} m`);
          if (!gd.char.anim.heldPose && H.phase !== 'grab') violate('held-anim-missing', name, `${name} held (${H.phase}) but his animator has no heldPose`);
        }
        if (pl.down) violate('holding-while-down', 'player', `holding ${name} while down`);
        if (pl.takedown || pl.kick || pl.picking || pl.wall || pl.dragging || pl.sprinting || pl.crouch) violate('holding-while-busy', 'player', `holding ${name} while takedown ${!!pl.takedown} kick ${!!pl.kick} picking ${!!pl.picking} wall ${!!pl.wall} drag ${!!pl.dragging} sprint ${pl.sprinting} crouch ${pl.crouch}`);
        if (!pl.char.anim.holdPose && H.t > 0) violate('hold-anim-missing', 'player', `holding (${H.phase}) but Sam's animator has no holdPose`);   // (t 0: grabbed inside this step's door pass, posed from the next)
        if (game.hover?.kind !== 'held' && !pl.down && H.t > 0) violate('hold-hover', 'player', `holding but the highlighted row is ${game.hover?.kind ?? 'none'}`);   // (t 0: grabbed inside this very step, after the cursor pass — the row comes next frame)
      } else {
        holdDeadFrames = 0;
        if (pl.char.anim.holdPose) violate('hold-anim-leftover', 'player', `free but Sam's animator still has holdPose (${pl.char.anim.holdPose.phase})`);
      }
      for (const x of game.guards) if (x.state !== 'dead' && !x.held && x.char.anim.heldPose) violate('held-anim-leftover', routeName(x), `${routeName(x)} free but his animator still has heldPose (${x.char.anim.heldPose.phase})`);
      for (const x of game.guards) if (x.state !== 'dead' && !x.held && x.talk.heldSince !== -1 && !inSandboxTalk.has(x)) violate('heldsince-leftover', routeName(x), `${routeName(x)} free but talk.heldSince ${f2(x.talk.heldSince)}`);
    }
    if (pl.picking && !pl.crouch) violate('player-pick-standing', 'player', 'picking but not crouched');
    if (pl.hitsLeft < 0 || pl.canisters < 0 || pl.flashbangs < 0) violate('player-neg-inventory', 'player', `hits ${pl.hitsLeft} cans ${pl.canisters} flash ${pl.flashbangs}`);
    // mission monotonic
    if (game.mission !== missionRef) { missionRef = game.mission; stageIdx = 0; }
    const si = STAGE_ORDER[game.mission.stage]; if (si === undefined) violate('mission-stage', game.mission.stage, `unknown stage ${game.mission.stage}`); else { if (si < stageIdx) violate('mission-backwards', `${stageIdx}->${si}`, `mission stage went back to ${game.mission.stage} without a restart`); stageIdx = Math.max(stageIdx, si); }
    if (game.mission.driveT > 0 && game.mission.stage !== 'drive') violate('mission-drive', game.mission.stage, `driveT ${game.mission.driveT} in stage ${game.mission.stage}`);
    // leaks / bounds
    if (game.items.items.length > 150) violate('leak', 'items', `${game.items.items.length} live items`);
    if (game.engine.lights.lights.length > 150) violate('leak', 'lights', `${game.engine.lights.lights.length} lights`);
    if (game.events.length > 400) violate('leak', 'events', `${game.events.length} events queued`);
    if (game.fx.length > 200) violate('leak', 'fx', `${game.fx.length} fx boxes`);
    if (game.messages.length > 5) violate('leak', 'messages', `${game.messages.length} messages`);
    // geometry overlap: a living guard noticeably inside a static (every 10th frame); the player's own 0.42 m movement circle more than 3 cm into one at the END of
    // any frame (what gets drawn — his collision is a sweep settled after the leaves and the furniture have moved him, so nothing should be left over)
    if (frame % 10 === 0) {
      for (const gd of game.guards) { if (gd.state === 'dead') continue; const q = v3.copy(gd.char.pos); if (col.collideCircle(q, 0.22, 0.3, 1.4, 1) && v3.distXZ(q, gd.char.pos) > 0.06) violate('in-geometry', routeName(gd), `guard ${routeName(gd)} ${v3.distXZ(q, gd.char.pos).toFixed(2)} m inside static geometry at ${p2(gd.char.pos)} (${gd.script ? 'scripted ' + game.clearingSummary() : gd.state})`); }
    }
    if (!pl.down) { const q = v3.copy(pl.char.pos); if (col.collideCircle(q, 0.42, 0.2, pl.crouch ? 1.0 : 1.7, 1) && v3.distXZ(q, pl.char.pos) > 0.03) violate('in-geometry', 'player', `player's circle ${v3.distXZ(q, pl.char.pos).toFixed(3)} m into static geometry at frame end at ${p2(pl.char.pos)} (crouch ${pl.crouch} sprint ${pl.sprinting} picking ${!!pl.picking} kick ${!!pl.kick} takedown ${!!pl.takedown} drag ${!!pl.dragging}; nearest guard ${Math.min(...living().map((g: any) => v3.distXZ(g.char.pos, pl.char.pos)), 99).toFixed(2)} m)`); }
  }

  // -------- smells
  const smells: any = { noProgress: 0, playerDeaths: 0, guardDeaths: 0 };
  let wasDown = false; let lastDead = 0;

  // -------- main loop
  tl(`--- seed ${seed}: ${MINUTES} min @ ${HZ} Hz`);
  game.godMode = false; game.player.visibility = 0.05;
  for (frame = 0; frame < FRAMES; frame++) {
    const t = game.time;
    // scheduled stimuli (Poisson per stimulus; only the mild ones during a respite)
    const inRespite = t < respiteUntil;
    for (const s of stims) {
      if (rng() < s.perMin / 60 * DT && (!inRespite || s.mild) && (!s.when || s.when())) {
        stimCounts[s.name] = (stimCounts[s.name] ?? 0) + 1;
        let detail: string | null | void = null;
        try { detail = s.run(); } catch (e: any) { exceptions++; violate('exception', `stim:${s.name}`, `${e?.stack ?? e}`); }
        tl(`STIM ${s.name}${detail ? ': ' + detail : ''}`);
      }
    }
    // ongoing behaviours
    if (interruptAt > 0 && t >= interruptAt) { interruptAt = -1; if (game.clearing && game.clearing.stage !== 'done') { stimCounts['interrupt-clear(scheduled)'] = (stimCounts['interrupt-clear(scheduled)'] ?? 0) + 1; let d = ''; try { d = interruptClear(); } catch (e: any) { exceptions++; violate('exception', 'stim:interrupt-clear', `${e?.stack ?? e}`); } tl(`STIM interrupt-clear(scheduled): ${d}`); } }
    if (game.guards.some((g: any) => g.state === 'alert')) { if (alertSince < 0) alertSince = t; else if (t - alertSince > 30 && !inRespite) { alertSince = -1; startRespite(rr(45, 110), 'alert for 30 s — the player slips away'); } } else alertSince = -1;
    { const q = game.player.char.pos; if (!game.player.down && (q[0] < -0.3 || q[0] > 40.3 || q[2] < -0.3 || q[2] > 28.3)) { smells.playerLeftWorld = (smells.playerLeftWorld ?? 0) + 1; teleport([5.1, 0, 15.2], 'back from beyond the world edge'); } }
    if (aiOffUntil > 0 && t >= aiOffUntil) { game.aiEnabled = true; aiOffUntil = -1; tl('ai back on'); }
    if (tour && t >= tour.until) { const sv = tour.saved; game.quietUtility = false; game.aiEnabled = sv.ai; game.godMode = sv.god; game.playerInvisible = sv.invisible; game.clearAftermath(); game.resetGuards(); if (!game.player.down) { game.teleportPlayer(sv.pos); teleportedThisFrame = true; game.player.crouch = false; } tour = null; tl('tour-blip over (Demo.stop equivalent)'); }
    if (t >= godFlipAt) { game.godMode = !game.godMode; godFlipAt = t + rr(60, 300); tl(`godMode → ${game.godMode}`); }
    if (t >= visUntil) { game.player.visibility = inRespite || rng() < 0.65 ? rr(0, 0.12) : rr(0.3, 1); visUntil = t + rr(3, 12); }
    if (holdF && t >= holdF.until) { input.keys.delete(IK); tl(`released F (${holdF.door.def.name}: locked=${holdF.door.locked} pick=${holdF.door.pick.toFixed(2)})`); holdF = null; }
    if (cursorDoor) { if (t >= cursorDoor.until) cursorDoor = null; else W.cursorAt(cursorDoor.door.pos); }
    else if (rackJob) {   // hands on the rack: cursor on its marker, one F press once it hovers in reach, then hold still until the drive is out (or time is up)
      if (t >= rackJob.until || game.mission.stage !== 'drive' || game.player.down) { tl(`rack job over: stage ${game.mission.stage} pulled=${game.mission.pulled} driveT=${game.mission.driveT.toFixed(2)}`); rackJob = null; }
      else { W.cursorAt(level.mission.rack.front); if (!rackJob.pressed && game.hover?.kind === 'objective' && game.hover.inReach) { input.pressed.add(IK); rackJob.pressed = true; } }
    }
    else if (frame % 30 === 0) { input.mouseX = rr(200, 1400); input.mouseY = rr(150, 850); }
    grabTick();
    missionSeen[game.mission.stage] = (missionSeen[game.mission.stage] ?? 0) + DT;
    if (!holdF && !game.player.picking && !game.player.kick && t >= wander.until) {
      for (const k of wander.keys) input.keys.delete(k);
      const r = rng();
      if (r < 0.45 || inRespite) wander = { until: t + rr(1, 6), keys: [] };
      else { const keys = [pick(['KeyW', 'KeyS', 'KeyA', 'KeyD'])]; if (rng() < 0.4) keys.push(pick(['KeyW', 'KeyS', 'KeyA', 'KeyD'])); if (rng() < 0.25) keys.push('ShiftLeft'); else if (rng() < 0.15) input.pressed.add('KeyC'); wander = { until: t + rr(0.8, 3.5), keys }; }
      for (const k of wander.keys) input.keys.add(k);
      if (rng() < 0.3) { input.pressed.add('KeyQ'); qPresses++; }   // back to the wall (or off it) wherever he happens to be: with the wander keys held he sidles, walks off it, or sprints off it
    }
    if (game.player.wall) wallTime += DT;
    if (game.player.down) { if (downSince < 0) downSince = t; if (t - downSince > rr(8, 25)) { input.pressed.add('Enter'); downSince = -1; restartCheck = { source: 'Enter after down' }; tl('press Enter (restart after down)'); } } else downSince = -1;
    // the frame
    const missionBefore = game.mission;
    steppedAiOff = !game.aiEnabled;   // (the stimuli above may have switched it either way; nothing inside the step does)
    heldGoingIn = game.player.holding?.g ?? null;
    if (steppedAiOff) { witnessSnap.clear(); for (const gd of game.guards) witnessSnap.set(gd, JSON.stringify(gd.witness)); }   // what every witness record holds going INTO an AI-off frame (invariants: it must come out the same)
    try { W.step(DT); consecutiveExc = 0; }
    catch (e: any) { exceptions++; consecutiveExc++; violate('exception', String(e?.message ?? e).slice(0, 80), `${e?.stack ?? e}`); if (consecutiveExc > 120) { tl('ABORT: update throws every frame'); break; } }
    if (game.mission !== missionBefore) { missionRef = game.mission; stageIdx = 0; teleportedThisFrame = true; if (restartCheck) { /* restarted inside updatePlayer: guards have been stepped once since */ restartCheck = null; game.godMode = rng() < 0.4; if (game.escalation !== 0 || game.clearing || game.guards.some((g: any) => g.script || g.state !== 'patrol') || doors.list.some((d: any) => d.lockBroken || d.locked !== !!d.def.locked)) violate('restart-restore', 'Enter-path', `after Enter restart: ${game.escalationSummary()} clearing=${!!game.clearing} guards=${game.guards.map((g: any) => g.state + (g.script ? '+script' : '')).join('/')} doors=${dump().doors}`); } }
    // bookkeeping
    try { invariants(); } catch (e: any) { exceptions++; violate('exception', 'invariants', `${e?.stack ?? e}`); }
    if (game.escalation !== lastEsc) { escChanges++; tl(`ESC ${lastEsc} → ${game.escalation}: ${game.escalationSummary()}`); lastEsc = game.escalation; }
    if (TRACE) { const [who, a, b, every] = TRACE.split(':'); if (game.time >= +a && game.time <= +b && frame % Math.max(1, Math.round(every ? +every : HZ / 4)) === 0) {
      if (who === 'player') { const pl = game.player, w = pl.wall; fullLog.push(`${game.time.toFixed(3)}s f${frame} TRACE ${JSON.stringify({ pos: p2(pl.char.pos), vel: p2(pl.char.vel), crouch: pl.crouch, sprint: pl.sprinting, keys: [...input.keys].join(''), wall: w ? { u: f2(w.u), v: f2(w.v), off: f2(w.off), settle: f2(w.settle), from: p2(w.from), n: p2(w.n), d: f2(w.d) } : null, guards: game.guards.filter((x: any) => x.state !== 'dead').map((x: any) => `${routeName(x)}@${p2(x.char.pos)} ${f2(v3.distXZ(x.char.pos, pl.char.pos))}m`) })}`); }   // (the player's own line: chasing a pop / a wall-hold oddity)
      else { const gd = game.guards.find((x: any) => routeName(x) === who); if (gd) fullLog.push(`${game.time.toFixed(3)}s f${frame} TRACE ${JSON.stringify({ ...dump().guards.find((x: any) => x.name === who), doors: dump().doors })}`); }
    } }
    if (game.player.down && !wasDown) { smells.playerDeaths++; tl('player down'); } wasDown = game.player.down;
    { const dead = game.guards.filter((g: any) => g.state === 'dead').length; if (dead > lastDead) smells.guardDeaths += dead - lastDead; lastDead = dead; }
    escTime[game.escalation] += DT;
    for (const gd of game.guards) {
      const key = gd.script ? (game.clearing ? 'scripted-clear' : 'scripted') : gd.state; stateTime[key] = (stateTime[key] ?? 0) + DT;
      const ref = gd.path; if (ref !== lastPathRef.get(gd)) { lastPathRef.set(gd, ref); if (ref.length) { replans++; const w = replanWindow.get(gd) ?? []; w.push(game.time); while (w.length && game.time - w[0] > 1) w.shift(); replanWindow.set(gd, w); if (w.length > maxReplansPerSec) { maxReplansPerSec = w.length; maxReplansWho = `${routeName(gd)} @ ${game.time.toFixed(1)}s (${gd.script ? 'scripted ' + (game.clearingSummary() || '') : gd.state})`; } } }
    }
    if (game.clearing !== lastClearingRef) {
      if (game.clearing) {
        clearsStarted++; tl(`CLEAR started: ${game.clearingSummary()}`);
        // most clears get a quiet floor to work on; ~60 % of those get exactly one interrupt at a moment uniform over the whole job (so late phases get hit too)
        if (rng() < 0.75) { startRespite(rr(70, 140), 'let the clear run'); interruptAt = rng() < 0.6 ? game.time + rr(2, 110) : -1; if (interruptAt > 0) tl(`(interrupt scheduled at ${interruptAt.toFixed(0)} s)`); }
      }
      lastClearingRef = game.clearing;
    }
    if (frame % (HZ * 60) === 0 && frame > 0 && QUIET) console.log(`  [seed ${seed}] ${(frame / HZ / 60).toFixed(0)} min sim, ${((performance.now() - t0wall) / 1000).toFixed(0)} s wall, ${violations.length} violations logged, esc ${game.escalation}, guards ${game.guards.map((g: any) => g.state[0] + (g.script ? 's' : '')).join('')}`);
  }
  clearTap = null; roomTap = null;
  const simSeconds = frame * DT; const mins = simSeconds / 60;
  const guardMinutes = Object.values(stateTime).reduce((a, b) => a + b, 0) / 60;
  Object.assign(smells, {
    barksTotal: barkLog.length, barksPerMin: f2(barkLog.length / mins), barkRepeatsWithin5s: barkRepeats,
    topBarks: Object.entries(barkTexts).sort((a, b) => b[1] - a[1]).slice(0, 12).map(([k, n]) => `${n}× ${k}`),
    hudMessagesPerMin: f2(msgCount / mins),
    replansTotal: replans, replansPerGuardSecond: f2(replans / Math.max(1, guardMinutes * 60)), maxReplansInOneSecond: maxReplansPerSec, maxReplansWho,
    clearsStarted, clearOutcomes, clearEndPhases: clearInterruptPhases, roomEnds, interruptsFiredAtPhase: interruptPhases,
    escalationTimeShare: { calm: f2(escTime[0] / simSeconds), heightened: f2(escTime[1] / simSeconds), lockdown: f2(escTime[2] / simSeconds) }, escalationChanges: escChanges,
    guardStateTimeShare: Object.fromEntries(Object.entries(stateTime).map(([k, v]) => [k, f2(v / (guardMinutes * 60))])),
    missionStageSeconds: Object.fromEntries(Object.entries(missionSeen).map(([k, v]) => [k, Math.round(v)])), debriefs: smells.debriefs ?? 0,
    wallQPresses: qPresses, wallSecondsHeld: Math.round(wallTime), grabEnds,
    audioTop: [...W.audioCounts.entries()].sort((a, b) => b[1] - a[1]).slice(0, 10).map(([k, n]) => `${k} ${n}`),
    finalState: dump(),
  });
  if (LOG_DIR) await Bun.write(`${LOG_DIR}/soak-seed${seed}.log`, fullLog.join('\n') + '\n');
  return { seed, frames: frame, simSeconds: f2(simSeconds), wallSeconds: f2((performance.now() - t0wall) / 1000), exceptions, violations, violationCounts: vCounts, smells, stimuli: stimCounts, timelineTail: timeline.slice(-40) };
}

// ---------------------------------------------------------------- run all seeds
const reports: SeedReport[] = [];
for (const s of SEEDS) {
  console.log(`\n=== seed ${s} ===`);
  const r = await runSeed(s);
  reports.push(r);
  console.log(`--- seed ${s} done: ${r.frames} frames (${(r.simSeconds / 60).toFixed(1)} sim-min) in ${r.wallSeconds} s wall; exceptions ${r.exceptions}; violation kinds ${JSON.stringify(r.violationCounts)}`);
  console.log(`    stimuli: ${JSON.stringify(r.stimuli)}`);
  console.log(`    smells: ${JSON.stringify({ ...r.smells, finalState: undefined, topBarks: undefined, audioTop: undefined })}`);
  console.log(`    top barks: ${(r.smells.topBarks as string[]).join(' | ')}`);
}
console.log('\n=== summary ===');
const total: Record<string, number> = {};
for (const r of reports) for (const [k, n] of Object.entries(r.violationCounts)) total[k] = (total[k] ?? 0) + n;
console.log('violation counts across seeds:', JSON.stringify(total));
console.log('first report of each kind|key:');
const seen = new Set<string>();
for (const r of reports) for (const v of r.violations) { const k = `${v.kind}|${v.key}`; if (seen.has(k)) continue; seen.add(k); console.log(`  seed ${v.seed} f${v.frame} ${v.time}s ${v.kind} (${v.key}): ${v.msg.split('\n')[0].slice(0, 300)}`); }
if (JSON_OUT) { await Bun.write(JSON_OUT, JSON.stringify(reports, null, 1)); console.log(`wrote ${JSON_OUT}`); }
