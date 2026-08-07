// Long randomized headless soak of the gameplay layer: the real Game (see headless.ts for what is real and what is stubbed) stepped at a fixed rate for tens of
// simulated minutes per seed under a seeded random schedule of stimuli — gunshot / kick / bang events, kills, the player teleported onto guards, lit in their
// cones, bumping them, doors picked / kicked / slammed, blackouts, restarts and guard resets, escalation raised / stood down, forced room clears interrupted at
// whatever phase they are in, AI and tour toggles — while a set of invariants is asserted every frame and softer smells are counted.
//
//   bun run tools/qa/soak.ts [--seeds 1,2,3] [--minutes 20] [--hz 60] [--quiet] [--json out.json] [--logdir dir]
//
// Everything random goes through one seeded PRNG per stream (the game's own Math.random included), so a violation is reproducible from its seed + frame
// (re-running the same seed replays the same frames exactly; ~1 s of wall clock per simulated minute). Invariants checked every frame: no exception; nothing
// NaN / out of the world; guard state / awareness / speed sane; no dead man scripted, no orphan script, no clearing while the tour flag or AI-off is up;
// escalation level and clock consistent, no lockdown duty below lockdown; doors never latched-open / locked-and-broken / out of range; pick ∈ [0,1];
// followers' leaders alive; nobody stuck (ordered to move 4 s, went nowhere — reported with the doors / props / nav facts around him), overlapping a colleague
// > 2 s, popping > 0.3 m in a frame, or standing inside static geometry; mission stages only forward; leaks; and everything authored restored by a restart.
// Softer smells are counted (barks/min and repeats, path re-plans, clear outcomes by phase, time per alarm level, deaths). tools/qa/probes.ts has the
// deterministic single-situation repros for what this found.
import { standUp, ROOT, Headless } from './headless.ts';

const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { killGuard, fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { startPicking, startKick, startTakedown } = await import(`${ROOT}/src/game/player.ts`);
const { planClears, escalationOf, nearKnownBody, bodyPos } = await import(`${ROOT}/src/game/guards.ts`);
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
    player: { pos: p2(game.player.char.pos), down: game.player.down, crouch: game.player.crouch, sprint: game.player.sprinting, picking: game.player.picking?.def.name ?? null, kick: game.player.kick?.door.def.name ?? null, takedown: !!game.player.takedown, dragging: !!game.player.dragging, vis: f2(game.player.visibility) },
    guards: game.guards.map((gd: any) => ({ name: routeName(gd), state: gd.state, pos: p2(gd.char.pos), yaw: f2(gd.char.bodyYaw), speed: f2(gd.speed), aw: f2(gd.awareness), reactT: f2(gd.reactT), searchT: f2(gd.searchT), wp: gd.wp, wait: f2(gd.wait),
      path: `${gd.pathI}/${gd.path.length}`, pathGoal: p2(gd.pathGoal), lastKnown: p2(gd.lastKnown), stuckT: f2(gd.stuckT), hold: gd.hold, pinned: gd.pinned, drawn: gd.drawn, task: gd.task?.kind ?? null,
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
  const origSay = game.say.bind(game);
  game.say = (gd: any, text: string, radio = false) => {
    const who = routeName(gd);
    for (let i = barkLog.length - 1; i >= 0 && game.time - barkLog[i].t < 5; i--) if (barkLog[i].who === who && barkLog[i].text === text) { barkRepeats++; if (barkRepeats <= 12) tl(`SMELL bark repeat <5s ${who}: "${text}"`); break; }
    barkLog.push({ t: game.time, who, text }); barkTexts[text] = (barkTexts[text] ?? 0) + 1;
    fullLog.push(`${game.time.toFixed(2)}s f${frame}   ${radio ? 'RADIO' : 'SAY'} ${who} (${gd.script ? 'scripted' : gd.state}${gd.bodyDuty ? ' ' + gd.bodyDuty.role + ':' + gd.bodyDuty.stage : ''} @${f2(gd.char.pos[0])},${f2(gd.char.pos[2])}): ${text}`);   // (the per-seed log file gets every line said; the console timeline only the smells)
    if (gd.state === 'dead') violate('dead-speaks', text, `${who} is dead but says "${text}"`);
    if ((text === 'clear here — staying on it' || text === 'must have been nothing') && nearKnownBody(game, gd.char.pos, 4)) { const b = nearKnownBody(game, gd.char.pos, 4); violate('line-over-body', who, `${who} says "${text}" ${v3.distXZ(gd.char.pos, bodyPos(b)).toFixed(1)} m from ${routeName(b)}'s found body (duty ${gd.bodyDuty ? gd.bodyDuty.role + ':' + gd.bodyDuty.stage : 'none'})`); }
    return origSay(gd, text, radio);
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
  function releaseKeys() { for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', 'KeyF', 'AltLeft']) input.keys.delete(k); wander = { until: game.time + rr(1, 4), keys: [] }; holdF = null; }
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
      if (gd.state !== 'patrol' || gd.script || gd.leader || gd.post || gd.task || gd.hold || gd.pinned || gd.awareness !== 0) bad.push(`guard ${i} state=${gd.state} script=${!!gd.script} leader=${!!gd.leader} post=${!!gd.post} aw=${gd.awareness}`);
      if (v3.distXZ(gd.char.pos, r0) > 0.01 || gd.wp !== 0) bad.push(`guard ${i} not at route start (${p2(gd.char.pos)} vs ${p2(r0)}, wp ${gd.wp})`);
    });
    for (const [t, on] of game.initialOn) if (t.on !== on) bad.push(`light ${t.name} on=${t.on}≠${on}`);
    if (game.targets.some((t: any) => t.broken || t.disabledUntil !== -1)) bad.push('a fixture still broken/disabled');
    if (game.blackout.active) bad.push('blackout still active');
    if (game.mission.stage !== 'infiltrate' || game.mission.pulled || game.mission.driveT !== -1) bad.push(`mission ${game.mission.stage}`);
    { const pl = game.player; if (pl.down || v3.distXZ(pl.char.pos, level.playerSpawn) > 0.01 || pl.picking || pl.kick || pl.dragging || pl.takedown || pl.pendingThrow || pl.throwHeld || pl.fHeld || pl.doorHold !== 0 || pl.doorCracking || pl.crouch || pl.sprinting || pl.char.anim.lockpick || pl.char.anim.kicking || pl.hitsLeft !== 1 || pl.slot !== 1) bad.push(`player pos ${p2(pl.char.pos)} down=${pl.down} picking=${!!pl.picking} kick=${!!pl.kick} drag=${!!pl.dragging} takedown=${!!pl.takedown} throw=${!!pl.pendingThrow}/${pl.throwHeld} fHeld=${pl.fHeld} doorHold=${pl.doorHold} crouch=${pl.crouch} sprint=${pl.sprinting} lockpick=${pl.char.anim.lockpick} kicking=${pl.char.anim.kicking} hits=${pl.hitsLeft} slot=${pl.slot}`); }
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
    { name: 'kill-guard', perMin: 0.45, when: () => living().length > 0, run: () => { const gd = pick(living()) as any; const quiet = rng() < 0.35; const a = rr(-Math.PI, Math.PI); killGuard(game, gd, [Math.sin(a), 0, Math.cos(a)], quiet); return `${routeName(gd)} (${gd.script ? 'scripted' : gd.state}${quiet ? ', quiet' : ''})`; } },
    { name: 'player-shoots-guard', perMin: 0.2, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; fireWeapon(game, game.player.char, v3.add(gd.char.pos, [0, 1.25, 0]), true); return routeName(gd); } },
    { name: 'player-shoots-wall', perMin: 0.25, when: () => !game.player.down, run: () => { const p = randomFloorPoint(); fireWeapon(game, game.player.char, [p[0], 1.2, p[2]], true); return null; } },
    { name: 'teleport-near-guard', perMin: 0.6, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 0.8, 3.5, null); if (!p) return 'no spot'; teleport(p, `near ${routeName(gd)}`); return null; } },
    { name: 'bump-guard', perMin: 0.35, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 0.35, 0.6, null); if (!p) return 'no spot'; teleport(p, `bumping ${routeName(gd)} (${gd.script ? 'scripted' : gd.state})`); return null; } },
    { name: 'lit-in-cone', perMin: 0.45, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 2, 7, true); if (!p) return 'no spot'; teleport(p, `in front of ${routeName(gd)}`); game.player.visibility = rr(0.6, 1); visUntil = game.time + rr(3, 10); return `vis ${game.player.visibility.toFixed(2)}`; } },
    { name: 'sneak-behind', perMin: 0.3, when: () => living().length > 0 && !game.player.down, run: () => { const gd = pick(living()) as any; const p = nearGuardPoint(gd, 0.7, 0.95, false); if (!p) return 'no spot'; teleport(p, `behind ${routeName(gd)}`); if (rng() < 0.7 && gd.state !== 'alert') { startTakedown(game, gd); return 'takedown'; } return null; } },
    { name: 'pick-door', perMin: 0.45, when: () => !game.player.down && lockedDoors().some((d: any) => d.latched), run: () => {
      const d = pick(lockedDoors().filter((x: any) => x.latched)) as any; const from = keySideOf(d);
      const spot = doors.workSpot(d, from); teleport([spot.pos[0] + rr(-0.3, 0.3), 0, spot.pos[2] + rr(-0.3, 0.3)], `to pick ${d.def.name}`);
      const full = rng() < 0.7; const secs = full ? Doors.PICK_SECS + rr(1.5, 3) : rr(0.8, 3.5);
      if (rng() < 0.5) { startPicking(game, d); input.keys.add('KeyF'); holdF = { until: game.time + secs, door: d }; return `${d.def.name} direct ${full ? 'full' : 'partial'} ${secs.toFixed(1)}s`; }
      cursorDoor = { until: game.time + secs + 0.5, door: d }; input.keys.add('KeyF'); holdF = { until: game.time + secs, door: d }; return `${d.def.name} via F-hold ${full ? 'full' : 'partial'} ${secs.toFixed(1)}s`;
    } },
    { name: 'kick-door', perMin: 0.35, when: () => !game.player.down, run: () => {
      const cands = lockedDoors().filter((x: any) => x.latched); const d = (cands.length && rng() < 0.8 ? pick(cands) : pick(doors.list)) as any;
      const spot = doors.kickSpot(d, keySideOf(d)); teleport([spot.pos[0] + rr(-0.2, 0.2), 0, spot.pos[2] + rr(-0.2, 0.2)], `to kick ${d.def.name}`);
      if (rng() < 0.5) { startKick(game, d); return `${d.def.name} direct`; }
      game.player.crouch = false; cursorDoor = { until: game.time + 1.0, door: d }; input.pressed.add('KeyF'); return `${d.def.name} via F-tap (lockedOut=${doors.lockedOut(d, game.player.char.pos)})`;
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
      else if (r < 0.26) { const gd = rng() < 0.5 || !C.b ? C.a : C.b; killGuard(game, gd, [1, 0, 0], rng() < 0.5); what = `killed ${routeName(gd)}`; }
      else if (r < 0.34) { const third = living().find((x: any) => x !== C.a && x !== C.b); if (third) { killGuard(game, third, [0, 0, 1], true); what = 'killed third man quietly'; } else what = 'no third'; }
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
  ];
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
  const stuck = new Map<any, { t0: number; p0: Vec3; want: number; lastReport: number }>();
  const noprog = new Map<any, { t0: number; p0: Vec3; lastReport: number }>();
  const closeT = new Map<string, number>(); const closeReported = new Map<string, number>();
  const dutyBad = new Map<any, number>(); const lingerT = new Map<any, number>(); const lingerRep = new Map<any, number>();
  let leaderBad = 0; let exceptions = 0; let consecutiveExc = 0; let overdueFrames = 0;
  const finiteChar = (c: any) => Number.isFinite(c.pos[0]) && Number.isFinite(c.pos[1]) && Number.isFinite(c.pos[2]) && Number.isFinite(c.bodyYaw) && Number.isFinite(c.aimYaw) && Number.isFinite(c.aimPitch) && Number.isFinite(c.vel[0]) && Number.isFinite(c.vel[2]);

  function invariants() {
    const t = game.time;
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
      // stuck: wanted to move (speed > 0.3) essentially the whole 4 s window while scripted / pathing, and went nowhere
      if (gd.state !== 'dead') {
        const pathing = (gd.path.length > 0 && gd.pathI < gd.path.length) || (gd.script && gd.script.goal && !gd.script.arrived);
        let s = stuck.get(gd); if (!s) { s = { t0: t, p0: v3.copy(c.pos), want: 0, lastReport: -100 }; stuck.set(gd, s); }
        if (gd.speed > 0.3 && pathing) s.want += DT;
        if (t - s.t0 >= 4) {
          const moved = v3.distXZ(c.pos, s.p0);
          if (s.want >= 3.8 && moved < 0.05 && t - s.lastReport > 20) {
            s.lastReport = t;
            const nearDoors = doors.list.filter((d: any) => Math.hypot(d.def.hinge[0] - c.pos[0], d.def.hinge[1] - c.pos[2]) < d.def.width + 0.8).map((d: any) => `${d.def.name}@${d.angle.toFixed(2)}${Math.abs(d.angle - d.maxA) < 0.02 || Math.abs(d.angle - d.minA) < 0.02 ? '(AT STOP)' : ''}${d.latched ? 'L' : ''}`).join(',') || 'none';
            const q = v3.copy(c.pos); const inStatic = col.collideCircle(q, c.radius, 0.2, 1.5, 1) ? v3.distXZ(q, c.pos).toFixed(2) : '0';
            const nearProps = game.props.props.filter((p: any) => Math.hypot(p.x - c.pos[0], p.z - c.pos[2]) < Math.max(p.def.half[0], p.def.half[1]) + 0.75).map((p: any) => `${p.def.name}(${p.def.kind} ${p.def.mass}kg)@${p.x.toFixed(2)},${p.z.toFixed(2)}${Math.hypot(p.x - p.home[0], p.z - p.home[1]) > 0.05 ? ' moved ' + Math.hypot(p.x - p.home[0], p.z - p.home[1]).toFixed(2) + 'm' : ''}`).join(',') || 'none';
            violate('stuck', `${name}:${gd.script ? 'script' : gd.state}`, `guard ${name} ordered at ${gd.speed.toFixed(2)} m/s for ${s.want.toFixed(1)} of 4 s but moved ${moved.toFixed(3)} m at ${p2(c.pos)} — ${gd.script ? `scripted goal ${p2(gd.script.goal)} stance ${gd.script.stance} (${game.clearingSummary() || 'no clearing'})` : `${gd.state} path ${gd.pathI}/${gd.path.length} next wp ${p2(gd.path[gd.pathI])} → ${p2(gd.pathGoal)}`} stuckT ${gd.stuckT.toFixed(2)}; doors in reach: ${nearDoors}; props in reach: ${nearProps}; static overlap ${inStatic} m; nav cell blocked: ${nav.isBlocked(c.pos[0], c.pos[2])}; path to goal exists: ${gd.pathGoal ? !!nav.findPath(c.pos, gd.pathGoal) : 'n/a'}`);
          }
          s.t0 = t; s.p0 = v3.copy(c.pos); s.want = 0;
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
    // per-frame displacement: nobody alive moves more than 0.3 m in a frame unless the harness teleported them; through a wall is its own kind
    {
      const plc = game.player.char;
      const prev = lastPos.get('player'); const cur = v3.copy(plc.pos);
      if (prev && !teleportedThisFrame && !game.player.down && game.mission === missionRef) {
        const d = v3.distXZ(prev, cur);
        if (d > 0.3) { const wall = col.segmentBlocked([prev[0], 0.6, prev[2]], [cur[0], 0.6, cur[2]]) && col.segmentBlocked([prev[0], 1.2, prev[2]], [cur[0], 1.2, cur[2]]); violate(wall ? 'popped-through-wall' : 'popped', 'player', `player jumped ${d.toFixed(2)} m in one frame ${p2(prev)} → ${p2(cur)}${wall ? ' THROUGH static geometry' : ''} (picking ${!!game.player.picking} kick ${!!game.player.kick} takedown ${!!game.player.takedown}; nearest guard ${Math.min(...living().map((g: any) => v3.distXZ(g.char.pos, cur))).toFixed(2)} m)`); }
      }
      lastPos.set('player', cur);
      for (const gd of game.guards) {
        const p0 = lastPos.get(gd); const c1 = v3.copy(gd.char.pos);
        if (p0 && gd.state !== 'dead') { const d = v3.distXZ(p0, c1); if (d > 0.3) { const wall = col.segmentBlocked([p0[0], 0.6, p0[2]], [c1[0], 0.6, c1[2]]); violate(wall ? 'popped-through-wall' : 'popped', routeName(gd), `guard ${routeName(gd)} jumped ${d.toFixed(2)} m in one frame ${p2(p0)} → ${p2(c1)}${wall ? ' THROUGH static geometry' : ''} (${gd.script ? 'scripted ' + (game.clearingSummary() || '') : gd.state})`); } }
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
    if (!game.quietUtility && esc > 0 && t >= game.escalationT && !game.alarm.episode && !hunting) { if (++overdueFrames >= 2) violate('esc-clock', 'overdue-stepdown', `level ${esc} clock lapsed ${(t - game.escalationT).toFixed(1)} s ago, no episode, nobody hunting, yet no step-down for ${overdueFrames} frames`); } else overdueFrames = 0;   // (one frame is legit: a clear finishing inside updateEscalation puts the pair back on patrol after this frame's step-down check)
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
    // geometry overlap (every 10th frame): a living character noticeably inside a static
    if (frame % 10 === 0) {
      for (const gd of game.guards) { if (gd.state === 'dead') continue; const q = v3.copy(gd.char.pos); if (col.collideCircle(q, 0.22, 0.3, 1.4, 1) && v3.distXZ(q, gd.char.pos) > 0.06) violate('in-geometry', routeName(gd), `guard ${routeName(gd)} ${v3.distXZ(q, gd.char.pos).toFixed(2)} m inside static geometry at ${p2(gd.char.pos)} (${gd.script ? 'scripted ' + game.clearingSummary() : gd.state})`); }
      if (!pl.down) { const q = v3.copy(pl.char.pos); if (col.collideCircle(q, 0.25, 0.3, pl.crouch ? 0.9 : 1.5, 1) && v3.distXZ(q, pl.char.pos) > 0.08) violate('in-geometry', 'player', `player ${v3.distXZ(q, pl.char.pos).toFixed(2)} m inside static geometry at ${p2(pl.char.pos)} (picking ${!!pl.picking} kick ${!!pl.kick} takedown ${!!pl.takedown} drag ${!!pl.dragging})`); }
    }
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
    if (holdF && t >= holdF.until) { input.keys.delete('KeyF'); tl(`released F (${holdF.door.def.name}: locked=${holdF.door.locked} pick=${holdF.door.pick.toFixed(2)})`); holdF = null; }
    if (cursorDoor) { if (t >= cursorDoor.until) cursorDoor = null; else W.cursorAt(cursorDoor.door.pos); }
    else if (rackJob) {   // hands on the rack: cursor on its marker, one F press once it hovers in reach, then hold still until the drive is out (or time is up)
      if (t >= rackJob.until || game.mission.stage !== 'drive' || game.player.down) { tl(`rack job over: stage ${game.mission.stage} pulled=${game.mission.pulled} driveT=${game.mission.driveT.toFixed(2)}`); rackJob = null; }
      else { W.cursorAt(level.mission.rack.front); if (!rackJob.pressed && game.hover?.kind === 'objective' && game.hover.inReach) { input.pressed.add('KeyF'); rackJob.pressed = true; } }
    }
    else if (frame % 30 === 0) { input.mouseX = rr(200, 1400); input.mouseY = rr(150, 850); }
    missionSeen[game.mission.stage] = (missionSeen[game.mission.stage] ?? 0) + DT;
    if (!holdF && !game.player.picking && !game.player.kick && t >= wander.until) {
      for (const k of wander.keys) input.keys.delete(k);
      const r = rng();
      if (r < 0.45 || inRespite) wander = { until: t + rr(1, 6), keys: [] };
      else { const keys = [pick(['KeyW', 'KeyS', 'KeyA', 'KeyD'])]; if (rng() < 0.4) keys.push(pick(['KeyW', 'KeyS', 'KeyA', 'KeyD'])); if (rng() < 0.25) keys.push('ShiftLeft'); else if (rng() < 0.15) input.pressed.add('KeyC'); wander = { until: t + rr(0.8, 3.5), keys }; }
      for (const k of wander.keys) input.keys.add(k);
    }
    if (game.player.down) { if (downSince < 0) downSince = t; if (t - downSince > rr(8, 25)) { input.pressed.add('Enter'); downSince = -1; restartCheck = { source: 'Enter after down' }; tl('press Enter (restart after down)'); } } else downSince = -1;
    // the frame
    const missionBefore = game.mission;
    try { W.step(DT); consecutiveExc = 0; }
    catch (e: any) { exceptions++; consecutiveExc++; violate('exception', String(e?.message ?? e).slice(0, 80), `${e?.stack ?? e}`); if (consecutiveExc > 120) { tl('ABORT: update throws every frame'); break; } }
    if (game.mission !== missionBefore) { missionRef = game.mission; stageIdx = 0; teleportedThisFrame = true; if (restartCheck) { /* restarted inside updatePlayer: guards have been stepped once since */ restartCheck = null; game.godMode = rng() < 0.4; if (game.escalation !== 0 || game.clearing || game.guards.some((g: any) => g.script || g.state !== 'patrol') || doors.list.some((d: any) => d.lockBroken || d.locked !== !!d.def.locked)) violate('restart-restore', 'Enter-path', `after Enter restart: ${game.escalationSummary()} clearing=${!!game.clearing} guards=${game.guards.map((g: any) => g.state + (g.script ? '+script' : '')).join('/')} doors=${dump().doors}`); } }
    // bookkeeping
    try { invariants(); } catch (e: any) { exceptions++; violate('exception', 'invariants', `${e?.stack ?? e}`); }
    if (game.escalation !== lastEsc) { escChanges++; tl(`ESC ${lastEsc} → ${game.escalation}: ${game.escalationSummary()}`); lastEsc = game.escalation; }
    if (TRACE) { const [who, a, b, every] = TRACE.split(':'); if (game.time >= +a && game.time <= +b && frame % Math.max(1, Math.round(every ? +every : HZ / 4)) === 0) { const gd = game.guards.find((x: any) => routeName(x) === who); if (gd) fullLog.push(`${game.time.toFixed(3)}s f${frame} TRACE ${JSON.stringify({ ...dump().guards.find((x: any) => x.name === who), doors: dump().doors })}`); } }
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
