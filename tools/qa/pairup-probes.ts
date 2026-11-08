// The 'pairup' family (game/pairup.ts — THE PAIR: the one switch over the three parts of two person stealth, and the rules that only exist once the three are
// in one build). Headless: the whole Game over the fake device, the room clear on the office (placement seed 10) with a squad of one — Blue One — beside Sam,
// the silent order on (as it ships), the defenders the probe needs stood on the open aisle by hand and held there (feet, gaze, state written every frame),
// the rest left behind their rooms' doors. Each part's own behaviour is its own family's (pair · sneak · spotted); this one asks what happens where two meet.
//
//   pairup-switch      everything off by default; the switch on turns the three parts on (from the Game's constructor too: the rows are on the menu), the
//                      panel's row turned off turns them off, the switch off with no word touches nothing, a part put back by the environment (SNEAK=off)
//                      stays back; a rule is never in force with the switch off
//   pairup-feet        FEET: a following partner beside a suspicious defender — the order written to him that frame is spotted.ts's spot and pace, not the
//                      sneak's step along Sam's trail; and he is low when either part says low
//   pairup-hunted      a defender hunting far from the pair: the sneak stays live (the partner low behind a crouched Sam);
//                      the pair CAUGHT (an alert man with a fresh sighting of Sam inside 30 m): not live either way
//   pairup-caught-job  CAUGHT ENDS A TAKE JOB: on his way to a man's back when a third defender makes the pair — the job is off, he holds with his gun on
//                      the man who has them, nothing said by the orders part; sent and inside the heel ring, he finishes it; and while caught a take
//                      order is refused at the menu ("they're onto us. no hands.")
//   pairup-one-eye     ONE judgement of 'seen' for the stalk: a lit stretch a second defender looks along is exposed (and by whom), and not once he turns
//                      away or the light goes; dark floor in front of a defender, past the sneak's elbow and inside the orders' front, is clear, with what
//                      the defender's own eye makes of a body walked there; the plain cases
//   pairup-rejoin      a take order given from Sam's shoulder: the man down, he falls in again; given from a hold he holds
//   pairup-voice       ONE VOICE: a sneak's wait that would be said over the net is not said while spotted.ts has him — and is with spotted off
//   pairup-word        the squad line says why a following partner is not at Sam's shoulder
//   pairup-grip        a man with the partner's hands on him sees nobody until they are off: turned to face the partner the frame the hands go on, he never
//                      stirs and goes down
//   pairup-stay        the hostage rescue with ONE partner, silent, on 'fall in': the scenario's flow leaves him at Sam's shoulder down the whole corridor
//   pairup-clatter     a quiet takedown lays the weapon down: no 'prop' clatter is filed from the hands going on to three seconds after the man is down — the
//                      partner's on a take order, and Sam's own (whose one 'prop' is the fall's thud, his own, never a weapon's) — and nobody else stirs; the
//                      same armed man SHOT lets it bounce, and that is heard of
// 2026-11-07: THE RULE ROWS ARE SETTLED AND NOT WRITTEN HERE (PAIRUP.oneEye / hunted / rejoin / stay / word / hands, PAIR.grip / gripBody, SNEAK.stalkAmbient),
// and the quiet-net rule is no longer held off under the rig. PAIRUP.on and the three parts' own switches are a mission's and are still thrown. Out with the
// rules' off sides: pairup-identity whole (every line moved the rule rows), the 'rule off' / 'without it' / 'as it was' line of hunted, rejoin, word,
// grip (two), hands (two), stay; the orders' own rule beside the one eye (three halves); the moon's rule-off halves, its roofed CONTROL (on against off,
// no keeper) and its recording. STALKAMBIENT_BREAK=1 is a staging now: the partner's meter is not held at the moon's half — the moon lines must fail.
import { standUp, ROOT } from './headless.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { startTakedown } = await import(`${ROOT}/src/game/player.ts`);
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const SLM: any = await import(`${ROOT}/src/game/silent.ts`);
const PM: any = await import(`${ROOT}/src/game/pairorders.ts`);
const SNM: any = await import(`${ROOT}/src/game/sneak.ts`);
const SPM: any = await import(`${ROOT}/src/game/spotted.ts`);
const PUM: any = await import(`${ROOT}/src/game/pairup.ts`);
const { RESCUE } = await import(`${ROOT}/src/game/rescue.ts`);
const SILENT: any = SLM.SILENT, PAIR: any = PM.PAIR, SNEAK: any = SNM.SNEAK, SPOTTED: any = SPM.SPOTTED, PAIRUP: any = PUM.PAIRUP;

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const clone = (o: any) => JSON.parse(JSON.stringify(o));
// (the tables as this process loaded them — the environment's word included — are what every probe starts from and what it leaves behind)
const TABLES: Record<string, any> = { NIGHTBREACH, SILENT, PAIR, SNEAK, SPOTTED, PAIRUP };
const LOADED = Object.fromEntries(Object.entries(TABLES).map(([k, T]) => [k, k === 'NIGHTBREACH' ? { ...T } : clone(T)]));
const STATS: Record<string, any> = { PAIR: PM.PAIR_STATS, SNEAK: SNM.SNEAK_STATS, SPOTTED: SPM.SPOTTED_STATS, PAIRUP: PUM.PAIRUP_STATS };
const LOGS: any[][] = [PM.PAIR_LOG, SNM.SNEAK_LOG, SPM.SPOTTED_LOG];
const ENV = ['PAIR', 'PAIRORDERS', 'SNEAK', 'SPOTTED'];
/** every table back as loaded, the three parts' switches and the pair's OFF (a probe says what it wants on) */
/** the stand's own word for the quiet-net rule (netquiet.ts), read once the first game stands and put back at every reset */
const reset = () => { for (const [k, T] of Object.entries(TABLES)) Object.assign(T, k === 'NIGHTBREACH' ? LOADED[k] : clone(LOADED[k])); PAIRUP.on = PAIR.on = SNEAK.on = SPOTTED.on = false; };
/** a probe: the tables, the tallies, the logs, the environment and the dice put back as it found them (and the pair's four switches off), however it ended */
const guarded = (fn: () => Promise<boolean>) => async () => {
  const stats = clone(STATS), logs = LOGS.map(l => [...l]), env = ENV.map(k => process.env[k]), rnd = Math.random;
  try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; }
  finally { for (const [k, T] of Object.entries(TABLES)) Object.assign(T, k === 'NIGHTBREACH' ? LOADED[k] : clone(LOADED[k])); for (const k of Object.keys(STATS)) Object.assign(STATS[k], stats[k]);
    PAIRUP.on = PAIR.on = SNEAK.on = SPOTTED.on = false;   // (the runner does not snapshot these tables: every probe leaves the switch and the three parts OFF)
    LOGS.forEach((l, i) => { l.length = 0; l.push(...logs[i]); }); ENV.forEach((k, i) => { if (env[i] === undefined) delete process.env[k]; else process.env[k] = env[i]; }); Math.random = rnd; }
};
/** the rule rows, whatever the table holds today */
const RULES: string[] = Object.keys(PUM.PAIRUP_DEFAULTS).filter(k => k !== 'on');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : '—'), f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—'), f3 = (x: number) => (Number.isFinite(x) ? x.toFixed(3) : '—');
const at = (p: number[]) => `(${f1(p[0])}, ${f1(p[2])})`;
const yawTo = (a: number[], b: number[]) => Math.atan2(b[0] - a[0], b[2] - a[2]);
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
/** open floor in the office's main aisle, well clear of every room's door (pair-probes.ts's own marks) */
const AISLE_W: number[] = [13, 0, 17.6], AISLE_MID: number[] = [22, 0, 17.6], EAST = Math.PI / 2, WEST = -Math.PI / 2;

/** a defender held by the probe: where he stands, where he looks, and (when given) his state, his meter and whom his eyes have */
interface Pin { at: number[]; yaw: number; state?: string; aw?: number; eyesOn?: 'sam' | 'mate' | null }
export interface RigOpts {
  /** PAIRUP.on (before the Game is made: its constructor is what turns the parts on) and its rule rows */
  up?: boolean;
  /** the parts by their OWN switches (after the Game is made), and their numbers */
  parts?: boolean | { pair?: boolean; sneak?: boolean; spotted?: boolean }; pair?: Record<string, unknown>; sneak?: Record<string, unknown>; spotted?: Record<string, unknown>;
  seed?: number; quiet?: boolean;
}
/** the room clear on the office with ONE teammate, the silent order on the menu and given, the pair as `o` says */
export async function rig(o: RigOpts = {}) {
  reset(); Object.assign(NIGHTBREACH, { squad: 1 }); PAIRUP.on = !!o.up;
  const seed = o.seed ?? 10; Math.random = seeded(seed);
  const W: any = await quiet(() => standUp()); const game = W.game, input = W.input;
  SILENT.on = true;   // (the shipped default: the harness stands the game up with the order off the menu)
  if (o.parts !== undefined) { const P = typeof o.parts === 'boolean' ? { pair: o.parts, sneak: o.parts, spotted: o.parts } : o.parts; if (P.pair !== undefined) PAIR.on = P.pair; if (P.sneak !== undefined) SNEAK.on = P.sneak; if (P.spotted !== undefined) SPOTTED.on = P.spotted; }
  Object.assign(PAIR, o.pair ?? {}); Object.assign(SNEAK, o.sneak ?? {}); Object.assign(SPOTTED, o.spotted ?? {});
  PM.PAIR_LOG.length = 0;
  const O = game.orders, P = game.pair; game.godMode = true;
  const said: string[] = [];
  { const msg = game.msg.bind(game); game.msg = (t: string) => { if (t !== 'teleported') said.push(t); msg(t); }; }   // (Sam is walked by teleports: 'teleported' is not a line)
  { const say = game.say.bind(game); game.say = (gd: any, t: string, r?: boolean) => { said.push(`[${gd.callsign}] ${t}`); return say(gd, t, r); }; }
  const why = await quiet(() => game.roomClear.start(seed));
  const seen = new Set<any>(), pins = new Map<any, Pin>(), gaze = new WeakMap<object, number>();
  /** his GAZE (chest and head: all the eye test and the partner's reads take of where he looks) fixed on a yaw — the men's own idle glancing is dice */
  const stare = (h: any, yaw: number) => { const c = h.char;
    if (!gaze.has(c)) { const dir = () => { const y = gaze.get(c)!; return [Math.sin(y), 0, Math.cos(y)]; }; Object.defineProperty(c, 'chestDir', { get: dir, set: () => { }, configurable: true }); c.headDir = dir; }
    gaze.set(c, yaw); };
  const pose = () => { for (const [h, p] of pins) { if (h.state === 'dead') continue; const c = h.char;
    c.pos[0] = p.at[0]; c.pos[2] = p.at[2]; c.vel[0] = c.vel[2] = 0; c.bodyYaw = c.aimYaw = p.yaw; stare(h, p.yaw); h.hold = true; h.pinned = true; h.lastPos = [p.at[0], 0, p.at[2]]; h.path = []; h.pathGoal = null;
    if (p.state !== undefined) h.state = p.state; if (p.aw !== undefined) h.awareness = p.aw;
    if (p.eyesOn === 'sam') { h.foe = 'player'; h.lastSeenT = game.time; h.lastKnown = [...game.player.char.pos]; } else if (p.eyesOn === 'mate') { h.foe = R.mate; h.lastSeenT = game.time; h.lastKnown = [...R.mate.char.pos]; } else if (p.eyesOn === null) { h.foe = null; h.lastSeenT = -100; }
    h.fireCd = Math.max(h.fireCd ?? 0, 0.5); } };   // (he never gets a round off: the family is about the partner)
  const R = { W, game, input, O, P, why, said, opShots: 0, frames: 0, post: null as null | (() => void),
    get mate(): any { return O.squad()[0]; }, get hostiles(): any[] { return game.guards.filter((h: any) => h.team === 'hostile'); },
    tango(n: number): any { return game.guards.find((h: any) => h.callsign === `Tango ${n}`); },
    step(n = 1) { for (let i = 0; i < n; i++) {
      game.player.visibility = 0.05; pose();
      W.step(DT); R.frames++; pose();
      for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'guardShot' && game.guards.some((x: any) => x.team === 'operative' && x.char.id === e.who)) R.opShots++; }
      R.post?.(); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    /** a man stood somewhere by hand, facing `yaw` (the partner: once; a defender: see `pin`) */
    put(gd: any, p: number[], yaw: number) { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.lastPos = [p[0], 0, p[2]]; gd.path = []; gd.pathGoal = null; if (gd.script) gd.script.path = undefined; },
    /** a defender held from here on: his feet, his gaze, and whatever of his state the probe writes (the rest stays his own) */
    pin(h: any, p: Pin) { pins.set(h, { ...(pins.get(h) ?? {}), ...p }); pose(); },
    turn(h: any, yaw: number) { const p = pins.get(h); if (p) p.yaw = yaw; pose(); },
    /** Sam walked (a stride of teleports a frame: his trail is laid as his own feet would lay it) along the nav path to `to`, at `pace` */
    walk(to: number[], pace = 1.2, secs = 40): boolean { const path: number[][] = game.col.nav.findPath(game.player.char.pos, to) ?? [to]; path.push(to); let i = 0;
      for (let f = 0; f < secs * 60; f++) { const p = game.player.char.pos; while (i < path.length - 1 && flat(path[i], p) < 0.3) i++; const wp = path[i], d = flat(wp, p); if (i === path.length - 1 && d < 0.05) return true;
        const s = Math.min(d, pace * DT); game.teleportPlayer([p[0] + (wp[0] - p[0]) / (d || 1) * s, 0, p[2] + (wp[2] - p[2]) / (d || 1) * s]); R.step(); }
      return false; },
    foe(h: any) { return { kind: 'foe', at: [h.char.pos[0], 0, h.char.pos[2]], foe: h }; },
    silent(): boolean { return SLM.squadSilent(game); },
    status(): string { return O.statusOf(R.mate); },
    log(): string { return PM.PAIR_LOG.map((x: any) => `${f1(x.t)} ${x.what}`).join(' | '); },
    lines(re: RegExp): string[] { return said.filter(t => re.test(t)); },
    /** the meter's reading for a man LOW on a spot, as the sneak reads a step (the CPU's steady direct light at his chest and head) */
    lit(p: number[]): number { const L = (y: number) => GDM.lightEstimateAt(game, [p[0], y, p[2]], undefined, true); return game.visibilityOf(Math.max(L(0.81), L(0.98))); },
    /** every other defender taken off the floor (quietly: no body is found, nobody is roused) */
    only(...keep: any[]) { for (const h of R.hostiles) if (!keep.includes(h) && h.state !== 'dead') killGuard(game, h, [0, 0, 1], true); },
  };
  await quiet(() => { R.step(30); if (o.quiet !== false) { SLM.setSilent(game, true); R.step(2); } });
  return R;
}
export type Rig = Awaited<ReturnType<typeof rig>>;
const crouchOf = (gd: any): number => gd.char.anim.crouch ?? 0;
/** the order last written to a man (squad.ts order(): what his script runner walks him by this frame) */
const written = (gd: any): { goal: number[] | null; speed: number; crouch: boolean } => { const s = gd.script; return { goal: s?.goal ? [...s.goal] : null, speed: s?.speed ?? NaN, crouch: !!s?.crouch }; };
/** the partner's own lines (a held defender shouts every frame he is written 'alert': not what any probe here reads) */
const mine = (R: Rig): string[] => R.said.filter(t => t.startsWith(`[${R.mate.callsign}] `));
const crouchKey = (R: Rig) => { R.input.pressed.add('KeyC'); R.step(3); return !!R.game.player.crouch; };
const fallIn = (R: Rig) => { R.O.give('fallIn', R.O.targetAt(R.game.player.char.pos), 'all'); R.step(5); };

// ---------------------------------------------------------------- the switch
export const pairupSwitch = guarded(async () => {
  const { verdict, done } = family('pairup-switch: one switch over the three parts — off by default, on turns them on, a part put back stays back');
  const D = PUM.PAIRUP_DEFAULTS, ons = () => `orders ${PAIR.on ? 'on' : 'off'} · sneak ${SNEAK.on ? 'on' : 'off'} · spotted ${SPOTTED.on ? 'on' : 'off'}`, all = (v: boolean) => PAIR.on === v && SNEAK.on === v && SPOTTED.on === v;
  const word = ENV.filter(k => process.env[k] !== undefined);   // (a run started with the environment's word for a part: the loaded tables carry it, the DEFAULTS never do)
  verdict(D.on === false && RULES.length >= 4 && RULES.every(k => D[k] === true) && SNM.SNEAK_DEFAULTS.on === false && SPM.SPOTTED_DEFAULTS.on === false
    && (word.length > 0 || (LOADED.PAIRUP.on === false && LOADED.PAIR.on === false && LOADED.SNEAK.on === false && LOADED.SPOTTED.on === false)), 'as shipped: the switch off and the three parts off, every rule row on behind it',
    `PAIRUP ${JSON.stringify(D)} · as loaded: pair ${LOADED.PAIRUP.on}, orders ${LOADED.PAIR.on}, sneak ${LOADED.SNEAK.on}, spotted ${LOADED.SPOTTED.on}${word.length ? ` (the environment's word: ${word.join(', ')})` : ''}`);
  for (const k of ENV) delete process.env[k];
  reset(); PAIRUP.on = true; PUM.pairUpApply(); const a = ons(), aOk = all(true);
  PAIRUP.on = false; PUM.pairUpApply(true); const b = ons(), bOk = all(false);
  verdict(aOk && bOk, 'the switch on turns the three on; the row turned off turns the three off', `on: ${a} · off (the row): ${b}`);
  reset(); PAIR.on = true; SPOTTED.on = true; PUM.pairUpApply(); const c = ons(), cOk = PAIR.on && !SNEAK.on && SPOTTED.on;
  reset(); PUM.pairUpApply(); const c2 = all(false);
  verdict(cOk && c2, 'the switch off and no word: nothing is touched', `orders and spotted on by their own switches, then the apply: ${c} · all off, then the apply: ${c2 ? 'all off' : ons()}`);
  reset(); process.env.SNEAK = 'off'; PAIRUP.on = true; PUM.pairUpApply(); const e = ons(), eOk = PAIR.on && !SNEAK.on && SPOTTED.on;
  delete process.env.SNEAK; reset(); process.env.PAIRORDERS = '0'; PAIRUP.on = true; PUM.pairUpApply(); const e2 = ons(), e2Ok = !PAIR.on && SNEAK.on && SPOTTED.on; delete process.env.PAIRORDERS;
  verdict(eOk && e2Ok, 'a part put back by the environment stays back under the switch', `SNEAK=off: ${e} · PAIRORDERS=0: ${e2}`);
  reset(); const rules = RULES; const offs = rules.map(k => PUM.pairUp(k)); PAIRUP.on = true; const onsR = rules.map(k => PUM.pairUp(k));
  verdict(offs.every(x => x === false) && onsR.every(x => x === true), 'a rule is in force only with the switch on', `switch off, rows on: ${offs.join(' ')} · switch on: ${onsR.join(' ')}`);
  // (the Game's constructor is what applies the switch: a NEW game with it on has the pair's rows on the menu once the one teammate is under orders)
  { const R = await rig({ up: true }); const T1 = R.tango(1); R.only(T1); R.pin(T1, { at: AISLE_MID, yaw: EAST }); R.game.teleportPlayer([11, 0, 18.5]); R.put(R.mate, AISLE_W, EAST); R.step(10);
    const up = R.P.up, parts = all(true), rows: string[] = R.O.rowsFor(R.foe(T1)).map((d: any) => d.id), err = R.O.give('pairMark', R.foe(T1), 'all'); R.step(5); const job = !!R.P.jobOf(R.mate);
    const Q = await rig({}); const U1 = Q.tango(1); Q.only(U1); Q.pin(U1, { at: AISLE_MID, yaw: EAST }); Q.game.teleportPlayer([11, 0, 18.5]); Q.put(Q.mate, AISLE_W, EAST); Q.step(10);
    const up0 = Q.P.up, rows0: string[] = Q.O.rowsFor(Q.foe(U1)).map((d: any) => d.id), err0 = Q.O.give('pairMark', Q.foe(U1), 'all');
    verdict(up && parts && rows.includes('pairMark') && err === null && job && !up0 && !rows0.includes('pairMark') && err0 !== null, 'a new Game with the switch on: the three parts on, the pair up, its rows on a defender\'s menu and given',
      `on: ${a} · pair up ${up} · a defender's rows [${rows.join(' ')}] · 'take him · on your mark' ${err ?? 'given'}, a job ${job} · off: pair up ${up0}, rows [${rows0.join(' ')}], the order: ${err0}`); }
  return done();
});

// ---------------------------------------------------------------- feet
/** the corridor, a suspicious man at one end with his eyes down it, the pair further along with Sam walking on: every frame's order as written to the partner */
async function feetRun(o: { lit: boolean; spotted: boolean; samLow?: boolean }) {
  const R = await rig({ up: true, parts: { spotted: o.spotted } }); const g = R.game, m = R.mate;
  const W = R.tango(2); R.only(W);
  if (o.lit) { R.pin(W, { at: [8.5, 0, 11], yaw: EAST, state: 'suspicious', aw: 0.4, eyesOn: null }); g.teleportPlayer([15.2, 0, 11]); R.put(m, [14.5, 0, 11], EAST); }
  else { R.pin(W, { at: [27, 0, 11], yaw: WEST, state: 'suspicious', aw: 0.4, eyesOn: null }); g.teleportPlayer([20.9, 0, 11]); R.put(m, [20.3, 0, 11], EAST); }
  fallIn(R); const samLow = o.samLow ? crouchKey(R) : false;
  const rows: { f: number; what: string | null; spot: number[] | null; goal: number[] | null; speed: number; crouch: boolean; here: number[]; word: string | null; lit: number; trail: number }[] = [], trail: number[][] = [];
  R.post = () => { const sp = SPM.spottedOf(m), w = written(m), s = g.player.char.pos; trail.push([s[0], 0, s[2]]);
    rows.push({ f: R.frames, what: sp?.what ?? null, spot: sp?.spot ? [...sp.spot] : null, ...w, here: [m.char.pos[0], 0, m.char.pos[2]], word: SNM.sneakWord(g, m), lit: m.visibility, trail: w.goal ? Math.min(...trail.map(t => flat(t, w.goal!))) : NaN }); };
  // (lit: Sam stands a moment first. spotted.ts holds the partner still for `slipWait` after the man turned before it moves him off the line, and once Sam
  //  has walked on up the corridor every spot out of the man's line is in SAM's line to him: the slip has to be found while Sam still stands here)
  if (o.lit) R.step(Math.ceil(60 * (SPOTTED.slipWait + 0.6)));
  R.walk(o.lit ? [18.5, 0, 11] : [24.5, 0, 11], 1.0, 5); R.step(60 * 2); R.post = null;
  return { R, rows, samLow, lines: mine(R) };
}
export const pairupFeet = guarded(async () => {
  const { verdict, done } = family('pairup-feet: beside a suspicious man the partner\'s feet are spotted.ts\'s, not the sneak\'s — and he is low if either says low');
  // lit floor in the man's line: spotted slips him off it at its own pace; the sneak alone would stand him waiting on the trail
  { const on = await feetRun({ lit: true, spotted: true }), off = await feetRun({ lit: true, spotted: false });
    const got = on.rows.filter(r => r.what && r.spot), bad = got.filter(r => !r.goal || flat(r.goal, r.spot!) > 0.01 || Math.abs(r.speed - (r.what === 'freeze' ? 0 : Math.min(1.2, SILENT.pace))) > 1e-6);
    const slips = got.filter(r => r.what === 'slip'), s0 = slips[0], lit0 = s0 ? on.rows.find(r => r.f === s0.f)!.lit : NaN;
    const waits = off.rows.filter(r => r.word === 'waiting on eyes'), w0 = waits[0], offTrail = off.rows.filter(r => !(r.trail < 1.6)).length;   // (his place in the file is 1.3 m back along it)
    const held = s0 ? got.filter(r => r.what === 'freeze' && r.f < s0.f).length : 0;   // (… and until `slipWait` is up he is held still on his own spot: that is spotted.ts's too)
    verdict(slips.length > 30 && bad.length === 0 && (SPOTTED.slipWait <= 0.5 || held > 0) && lit0 >= SPOTTED.litAt && on.lines.some(t => /moving off it/.test(t)) && !on.lines.some(t => /it's lit and he's looking/.test(t)), 'lit and in his line: the order written is spotted.ts\'s spot at spotted.ts\'s pace, every frame it gives one',
      s0 ? `${got.length} frames with a spot (${held} held still first, ${slips.length} a slip), ${bad.length} where the written goal / speed were not it · first: meter ${f2(lit0)}, spot ${at(s0.spot!)}, written goal ${at(s0.goal!)} at ${f2(s0.speed)} m/s · said: ${on.lines.filter(t => /line|still/.test(t)).join(' | ')}` : `spotted.ts never gave a slip · ${got.length} frames with a spot, by kind ${JSON.stringify(got.reduce((o: Record<string, number>, r) => (o[r.what!] = (o[r.what!] ?? 0) + 1, o), {}))} · first frames ${on.rows.slice(0, 400).filter((_, i) => i % 40 === 0).map(r => `${r.f}:${r.what ?? '-'}@${at(r.here)} lit ${f2(r.lit)} w=${r.word ?? '-'}`).join(' ')} · said: ${on.lines.join(' | ')}`);
    verdict(waits.length > 30 && offTrail === 0 && !!w0 && w0.speed === 0 && off.rows.every(r => r.what === null) && off.lines.some(t => /it's lit and he's looking/.test(t)), 'the same floor with spotted off: the sneak\'s own step — stood on Sam\'s trail, waiting on eyes',
      w0 ? `${waits.length} frames 'waiting on eyes', goal ${at(w0.goal!)} (${f2(w0.trail)} m off Sam's trail) at ${f2(w0.speed)} m/s · written goals further than a file place from the trail: ${offTrail} frames · said: ${off.lines.filter(t => /lit|cross/.test(t)).join(' | ')}` : `the sneak never waited (${off.lines.join(' | ')})`);
    const same = s0 && off.rows.find(r => r.f === s0.f);
    verdict(!!same && !!s0 && flat(same.goal!, s0.goal!) > 0.5 && s0.speed > 1 && flat(s0.goal!, s0.here) > 0.5, 'the two parts wanted different things of him that frame, and spotted.ts had him',
      same && s0 ? `frame ${s0.f}: with spotted ${at(s0.goal!)} at ${f2(s0.speed)} m/s · without ${at(same.goal!)} at ${f2(same.speed)} m/s (${f2(flat(same.goal!, s0.goal!))} m apart)` : 'no frame to compare'); }
  // dark floor: spotted freezes him where he is (the sneak would walk him on up the trail), and LOW comes from either part
  { const on = await feetRun({ lit: false, spotted: true }), off = await feetRun({ lit: false, spotted: false });
    const got = on.rows.filter(r => r.what === 'freeze'), bad = got.filter(r => !r.goal || !r.spot || flat(r.goal, r.spot) > 0.01 || r.speed !== 0), f0 = got[0];
    const walking = off.rows.filter(r => r.speed > 0), o0 = f0 && off.rows.find(r => r.f === f0.f), upright = walking.filter(r => !r.crouch).length;
    verdict(got.length > 30 && bad.length === 0 && !!o0 && o0.speed > 0 && flat(o0.goal!, o0.here) > 0.3, 'dark and near him: frozen on his own spot where the sneak alone walks him on',
      f0 && o0 ? `${got.length} frames frozen, ${bad.length} not written so · frame ${f0.f}: with spotted goal ${at(f0.goal!)} = where he stands ${at(f0.here)}, ${f2(f0.speed)} m/s · without: goal ${at(o0.goal!)} up the trail at ${f2(o0.speed)} m/s` : 'never frozen');
    const lowS = got.filter(r => r.crouch).length;
    verdict(got.length > 0 && lowS === got.length && upright > 30, 'low where spotted says low: its knee under an upright Sam, on a walk the sneak alone makes upright',
      `Sam upright, spotted says low: ${lowS}/${got.length} frames written low (the sneak alone on that walk: ${upright}/${walking.length} upright)`); }
  return done();
});

// ---------------------------------------------------------------- hunted
export const pairupHunted = guarded(async () => {
  const { verdict, done } = family('pairup-hunted: a hunt somewhere else does not stop the sneak — being caught does');
  const run = async () => {
    const R = await rig({ up: true }); const g = R.game, m = R.mate, T = R.tango(3), home = [...T.char.pos];
    g.teleportPlayer([10, 0, 20]); R.put(m, [9.3, 0, 20.3], 0); fallIn(R); const samLow = crouchKey(R);
    const read = () => ({ live: SNM.sneakLive(g) as boolean, sneaking: SNM.sneaking(g, m) as boolean, mode: SPM.spottedMode(g, m) as string, crouch: crouchOf(m), d: flat(T.char.pos, m.char.pos), loud: SLM.silentCompromised(g) as boolean });
    R.walk([10, 0, 22], 0.9, 4); const calm = read();
    R.pin(T, { at: home, yaw: 0, state: 'search', aw: 1, eyesOn: null }); R.walk([8.5, 0, 22.5], 0.9, 5); const search = read();
    R.pin(T, { at: home, yaw: 0, state: 'alert', aw: 1, eyesOn: null }); R.walk([10, 0, 21], 0.9, 5); const alert = read();
    R.pin(T, { at: home, yaw: 0, state: 'alert', aw: 1, eyesOn: 'sam' }); R.step(3); const caught3 = read(); R.walk([9, 0, 19.5], 0.9, 4); const caught = read();
    return { samLow, calm, search, alert, caught3, caught };
  };
  type Read = { live: boolean; sneaking: boolean; mode: string; crouch: number; d: number; loud: boolean };
  const w = (r: Read) => `sneak ${r.live ? 'live' : 'not live'}, partner ${r.sneaking ? 'sneaking' : 'not'} (crouch ${f1(r.crouch)}), ${r.mode}, hunter ${f1(r.d)} m off`;
  const A = await run();
  verdict(A.samLow && A.calm.live && A.search.live && A.search.sneaking && A.search.crouch > 0.5 && A.search.mode === 'calm' && A.alert.live && A.alert.crouch > 0.5 && !A.search.loud, 'a man searching (then alert with no sighting) far off — the sneak stays live, the partner low behind a crouched Sam',
    `searching: ${w(A.search)} · alert, no sighting: ${w(A.alert)}`);
  verdict(A.caught.mode === 'caught' && !A.caught3.live && !A.caught.live && A.caught.d < SPOTTED.caughtM && A.caught.crouch < 0.5, 'caught (an alert man with a fresh sighting of Sam inside 30 m): not live',
    `three frames on: ${w(A.caught3)} · later: ${w(A.caught)}`);
  return done();
});

// ---------------------------------------------------------------- caught ends a take job
export const pairupCaughtJob = guarded(async () => {
  const { verdict, done } = family('pairup-caught-job: the pair made by a third man ends a take job — unless he is sent and inside the heel ring');
  const stage = async (give = true) => { const R = await rig({ up: true }); const T1 = R.tango(1), T2 = R.tango(2); R.only(T1, T2);
    R.pin(T1, { at: AISLE_MID, yaw: EAST }); R.pin(T2, { at: [20, 0, 11], yaw: EAST });   // (the third man: in the corridor, a wall between him and the aisle)
    R.game.teleportPlayer([11, 0, 18.5]); R.put(R.mate, AISLE_W, EAST); R.step(10); if (give) R.O.give('pairMark', R.foe(T1), 'all'); return { R, T1, T2 }; };
  const ownLines = /onto us|seen me|going loud|he's watched|watching him|^\[[^\]]+\] he's down/;
  { const { R, T1, T2 } = await stage(); const m = R.mate; R.step(120);
    const J = R.P.jobOf(m), d0 = flat(m.char.pos, T1.char.pos), c0 = PUM.PAIRUP_STATS.caughtOff, n0 = mine(R).length, f0 = R.frames;
    const staged = !!J && !J.set && !J.go && !J.going;
    R.pin(T2, { at: [20, 0, 11], yaw: EAST, state: 'alert', aw: 1, eyesOn: 'sam' });
    const ended = R.until(() => !R.P.jobOf(m), 0.2), took = R.frames - f0, A = R.O.of(m), watch: number[] | null = A?.watch ?? null, c1 = PUM.PAIRUP_STATS.caughtOff - c0;
    const order = A?.order, there = A?.there, mode = SPM.spottedMode(R.game, m), log = R.log(); R.step(120); const lines = mine(R).slice(n0);
    verdict(staged && d0 > PAIR.heelRing && ended && took <= 2 && order === 'hold' && !!watch && flat(watch, T2.char.pos) < 0.3 && mode === 'caught', 'on his way to the man\'s back when a third defender makes them: the job is off within two frames, he holds with his gun on the one who has them',
      `the job ${staged ? 'staging' : 'NOT staging'}, ${f1(d0)} m from his man · ${ended ? `off after ${took} frame${took === 1 ? '' : 's'}` : 'still on'} · order '${order}', '${there}', watch ${watch ? at(watch) : '—'} (Tango 2 at ${at(T2.char.pos)}) · ${mode}`);
    verdict(/off:caught/.test(log) && /blown:.* caught by Tango 2/.test(log) && c1 === 1 && !lines.some(t => ownLines.test(t)) && T1.state !== 'dead' && R.O.of(m)?.order === 'hold', 'it is written down, counted once, and the orders part says nothing over spotted.ts\'s own line',
      `log: ${log} · caughtOff +${c1} · his lines since: ${lines.join(' | ') || '(none)'} · two seconds on: order '${R.O.of(m)?.order}', Tango 1 ${T1.state}`); }
  { const { R, T1, T2 } = await stage(); const m = R.mate;
    const set = R.until(() => !!R.P.jobOf(m)?.set, 30), sent = set && R.P.mark('api'); R.step(1);
    const J = R.P.jobOf(m), d0 = flat(m.char.pos, T1.char.pos), going = !!J?.going, c0 = PUM.PAIRUP_STATS.caughtOff;
    R.pin(T2, { at: [20, 0, 11], yaw: EAST, state: 'alert', aw: 1, eyesOn: 'sam' });
    let kept = 0, caught = 0; for (let i = 0; i < 6; i++) { R.step(); if (R.P.jobOf(m) && R.O.of(m)?.order === 'pairMark') kept++; if (SPM.spottedMode(R.game, m) === 'caught') caught++; }
    const down = R.until(() => T1.state === 'dead', 5), log = R.log();
    verdict(set && sent && going && d0 < PAIR.heelRing && caught >= 5 && kept === 6 && PUM.PAIRUP_STATS.caughtOff === c0 && down && /down:Tango 1/.test(log) && !/off:caught/.test(log), 'sent and inside the heel ring: caught does not end it — he finishes the man',
      `sent ${f2(d0)} m behind his man (heel ring ${PAIR.heelRing} m), going ${going} · six frames on: caught on ${caught}, the job still his on ${kept} · caughtOff +${PUM.PAIRUP_STATS.caughtOff - c0} · Tango 1 ${T1.state} · log: ${log}`); }
  // caught BEFORE the order: the take rows are refused at the menu, in words
  { const { R, T1, T2 } = await stage(false); const m = R.mate, t = R.foe(T1), valid = (id: string) => R.O.defs.find((d: any) => d.id === id)?.valid?.(R.O, t);
    const v0 = valid('pairMark'); R.pin(T2, { at: [20, 0, 11], yaw: EAST, state: 'alert', aw: 1, eyesOn: 'sam' }); const caught = R.until(() => SPM.spottedMode(R.game, m) === 'caught', 1);
    const v1 = valid('pairMark'), v2 = valid('pairTake'), e1 = R.O.give('pairMark', t, 'all'), e2 = R.O.give('pairTake', t, 'all'); R.step(5);
    verdict(v0 === true && caught && /^they're onto us\. (no hands|can't grab him)\.$/.test(String(v1)) && v2 === v1 && e1 !== null && e2 !== null && !R.P.jobOf(m) && T1.state !== 'dead', 'caught before the order: a take is refused at the menu, in words',
      `calm: ${v0 === true ? 'valid' : v0} · caught: on your mark '${v1}', now '${v2}' · given anyway: ${e1} / ${e2} · a job ${!!R.P.jobOf(m)}`); }
  return done();
});

// ---------------------------------------------------------------- one eye

// ---------------------------------------------------------------- rejoin
export const pairupRejoin = guarded(async () => {
  const { verdict, done } = family('pairup-rejoin: the man down, a partner sent from Sam\'s shoulder comes back to it');
  const run = async (from: 'fallIn' | 'hold') => {
    const R = await rig({ up: true }); const m = R.mate, T1 = R.tango(1); R.only(T1);
    R.pin(T1, { at: AISLE_MID, yaw: EAST }); R.game.teleportPlayer([11, 0, 18.5]); R.put(m, AISLE_W, EAST); R.step(10);
    R.O.give(from, R.O.targetAt(from === 'hold' ? m.char.pos : R.game.player.char.pos), 'all'); R.step(30);
    const before = R.O.of(m)?.order, r0 = PUM.PAIRUP_STATS.rejoins, n0 = mine(R).length;
    R.O.give('pairMark', R.foe(T1), 'all'); const set = R.until(() => !!R.P.jobOf(m)?.set, 30), sent = set && R.P.mark('api'), down = R.until(() => T1.state === 'dead', 15); R.step(30);
    const A = R.O.of(m), lines = mine(R).slice(n0);
    return { ok: before === from && set && sent && down, order: A?.order as string, following: A?.post === null, rejoins: PUM.PAIRUP_STATS.rejoins - r0, line: lines.find(t => /he's down/.test(t)) ?? '(nothing)', noted: /rejoin:/.test(R.log()), status: R.status() };
  };
  const w = (r: Awaited<ReturnType<typeof run>>) => `order '${r.order}' ('${r.status}'), rejoins +${r.rejoins}, said: ${r.line}`;
  const a = await run('fallIn'), b = await run('hold');
  verdict(a.ok && a.order === 'fallIn' && a.following && a.rejoins === 1 && a.noted && /he's down\. on you\./.test(a.line), 'sent while on \'fall in\': the man down, he falls in again', w(a));
  verdict(b.ok && b.order === 'hold' && !b.following && b.rejoins === 0 && !b.noted && !/on you/.test(b.line), 'sent from a hold: he holds where the man fell', w(b));
  return done();
});

// ---------------------------------------------------------------- one voice · the squad line's word
/** the corridor's dark east end: a man beyond the lamp at x 22‥23 looking back along it, Sam walked on across that pool, the partner left waiting short of it */
async function waitRun(o: RigOpts) {
  const R = await rig(o); const g = R.game, m = R.mate, W = R.tango(2); R.only(W);
  R.pin(W, { at: [27, 0, 11], yaw: WEST, state: 'suspicious', aw: 0.4, eyesOn: null }); g.teleportPlayer([20.9, 0, 11]); R.put(m, [20.3, 0, 11], EAST); fallIn(R);
  const s0 = clone(SNM.SNEAK_STATS), modes = new Set<string>(), words = new Set<string>();
  R.post = () => { modes.add(SPM.spottedMode(g, m)); const w = SNM.sneakWord(g, m); if (w) words.add(w); };
  R.walk([24.5, 0, 11], 1.0, 6); R.step(60 * 9); R.post = null;
  return { R, waits: SNM.SNEAK_STATS.waits - s0.waits, cues: SNM.SNEAK_STATS.cues - s0.cues, modes: [...modes], words: [...words], lines: mine(R) };
}
export const pairupVoice = guarded(async () => {
  const { verdict, done } = family('pairup-voice: one voice — a wait the sneak would call over the net is not called while spotted.ts has him');
  const on = await waitRun({ up: true }), off = await waitRun({ up: true, parts: { spotted: false } });
  const cue = /it's lit|can't cross/;
  verdict(on.waits >= 1 && on.cues === 0 && on.modes.includes('wary') && !on.lines.some(t => cue.test(t)) && on.words.includes('waiting on eyes'), 'wary beside a suspicious man: the sneak waits (a watched lit stretch ahead) and says nothing of it',
    `sneak waits +${on.waits}, cues +${on.cues} · spotted modes seen: ${on.modes.join(', ')} · his word: ${on.words.join(', ')} · his lines: ${on.lines.join(' | ') || '(none)'}`);
  verdict(off.waits >= 1 && off.cues >= 1 && off.lines.some(t => cue.test(t)), 'the same wait with spotted off: said', `sneak waits +${off.waits}, cues +${off.cues} · his lines: ${off.lines.filter(t => cue.test(t)).join(' | ')}`);
  return done();
});
export const pairupWord = guarded(async () => {
  const { verdict, done } = family('pairup-word: the squad line says why a following partner is not at Sam\'s shoulder');
  const r = await waitRun({ up: true }), R = r.R, m = R.mate, word = SNM.sneakWord(R.game, m), gap = flat(m.char.pos, R.game.player.char.pos);
  const withW = R.status();
  verdict(word === 'waiting on eyes' && withW.endsWith(` · ${word}`), 'waiting short of a watched stretch: the line carries the sneak\'s word', `${f1(gap)} m behind Sam · '${withW}'`);
  return done();
});

// ---------------------------------------------------------------- hands on: he sees nobody
export const pairupGrip = guarded(async () => {
  const { verdict, done } = family('pairup-grip: a man with the partner\'s hands on him sees nobody until they are off');
  /** the take on the lit aisle; the frame the hands go on the man's head and chest come round at the partner an arm's length behind him (his state is his own) */
  const run = async () => {
    const R = await rig({ up: true }); const g = R.game, m = R.mate, T1 = R.tango(1); R.only(T1);
    R.pin(T1, { at: AISLE_MID, yaw: EAST }); g.teleportPlayer([11, 0, 18.5]); R.put(m, AISLE_W, EAST); R.step(10);
    R.O.give('pairMark', R.foe(T1), 'all'); const ready = R.until(() => !!R.P.jobOf(m)?.set, 30) && R.P.mark('api') && R.until(() => (R.P.jobOf(m)?.t0 ?? -1) >= 0, 5);
    const d = flat(m.char.pos, T1.char.pos), lit = m.visibility, aw0 = T1.awareness, t0 = g.time; let frames = 0, gripped = 0, seen = 0, peak = aw0; const states = new Set<string>([T1.state]);
    R.post = () => { if (T1.state === 'dead') return; R.turn(T1, yawTo(T1.char.pos, m.char.pos)); frames++; if (PM.pairGripped(g, T1)) gripped++; if (GDM.canSeeChar(g, T1, m.char, false).visible) seen++; peak = Math.max(peak, T1.awareness); states.add(T1.state); };
    R.turn(T1, yawTo(T1.char.pos, m.char.pos)); const down = R.until(() => T1.state === 'dead', 5), secs = g.time - t0; R.post = null;
    return { ok: ready && down, d, lit, frames, gripped, seen, peak, aw0, secs, states: [...states].filter(x => x !== 'dead'), kind: T1.downKind as string, shout: R.said.filter(t => /^\[Tango 1\]|^guard: /.test(t)).slice(-2) };
  };
  const w = (r: Awaited<ReturnType<typeof run>>) => `hands on ${f2(r.d)} m behind him (the partner's meter ${f2(r.lit)}), ${r.frames} frames to the floor (${f2(r.secs)} s): gripped on ${r.gripped}, his eye has the partner on ${r.seen} · awareness ${f2(r.aw0)} → peak ${f2(r.peak)}, states ${r.states.join(' → ')} · ${r.kind}`;
  const a = await run();
  verdict(a.ok && a.frames > 20 && a.gripped === a.frames && a.seen === 0 && a.peak < 0.05 && a.states.join() === 'patrol' && a.kind === 'struck', 'PAIR.grip: turned to face the partner with the hands on, he sees nobody, never stirs, and goes down', w(a));
  /** the grip's other half (PAIR.gripBody): a colleague out cold on the lit aisle behind him, a metre and a half off; the frame the hands go on he comes round at the partner — and at the body.
   *  Asked of guards.ts bodySeen, the one test every finder's road takes */
  const body = async (cut = false) => {
    const R = await rig({ up: true }); const g = R.game, m = R.mate, T1 = R.tango(1), T2 = R.tango(2); R.only(T1, T2);
    R.pin(T1, { at: AISLE_MID, yaw: EAST }); R.pin(T2, { at: [AISLE_MID[0] - 1.6, 0, AISLE_MID[2] + 0.5], yaw: EAST }); R.step(3); killGuard(g, T2, [0, 0, 1], true);   // (stood there for his bones to be there, then dropped)
    g.teleportPlayer([11, 0, 18.5]); R.put(m, AISLE_W, EAST); R.step(30);
    const before = GDM.bodySeen(g, T1, T2);   // (his back to it: not in his view)
    R.O.give('pairMark', R.foe(T1), 'all'); const ready = R.until(() => !!R.P.jobOf(m)?.set, 30) && R.P.mark('api') && R.until(() => (R.P.jobOf(m)?.t0 ?? -1) >= 0, 5);
    let frames = 0, gripped = 0, seen = 0, cutAt = -1, after = 0, afterGrip = 0, afterSeen = 0;
    R.post = () => { if (T1.state === 'dead') return;
      if (cutAt >= 0) { after++; if (PM.pairGripped(g, T1)) afterGrip++; if (GDM.bodySeen(g, T1, T2)) afterSeen++; return; }   // (the hands came off: he is no longer blind to it)
      if (cut && frames >= 5) { cutAt = R.frames; killGuard(g, m, [1, 0, 0], false, 'shot', 'guard'); return; }
      R.turn(T1, yawTo(T1.char.pos, m.char.pos)); frames++; if (PM.pairGripped(g, T1)) gripped++; if (GDM.bodySeen(g, T1, T2)) seen++; };
    R.turn(T1, yawTo(T1.char.pos, m.char.pos)); const down = R.until(() => T1.state === 'dead', cut ? 1 : 5); R.post = null;
    return { ok: ready && (down || cut), down, before, frames, gripped, seen, after, afterGrip, afterSeen, dist: flat(T1.char.pos, T2.char.bones?.hips ?? T2.char.pos), kind: T2.state as string };
  };
  const wb = (r: Awaited<ReturnType<typeof body>>) => `a man down ${f2(r.dist)} m from him (${r.kind}), in his view before the hands: ${r.before} · hands on ${r.frames} frames, gripped on ${r.gripped}: bodySeen on ${r.seen}`;
  const c = await body();
  verdict(c.ok && !c.before && c.frames > 20 && c.gripped === c.frames && c.seen === 0, 'PAIR.gripBody: turned round onto a colleague\'s body with the hands on him, he finds none (the line below has him find it the frame the hands are off)', wb(c));
  // (the grip is the JOB's: it holds while the partner's job on the man stands, at most PAIR.secs + 0.5 s from the hands going on — and ends with the job)
  const e = await body(true);
  verdict(e.ok && !e.down && e.frames >= 5 && e.seen === 0 && e.after > 20 && e.afterGrip === 0 && e.afterSeen === e.after, 'the partner shot with his hands on the man: the hold on the body ends with the hands — from the next frame he has it', `hands on ${e.frames} frames (bodySeen on ${e.seen}), then the partner shot: ${e.after} frames on, gripped ${e.afterGrip}, bodySeen on ${e.afterSeen}`);
  return done();
});

// ---------------------------------------------------------------- hands: a blown take order finished by spotted.ts's hands · the grip covers them
export const pairupHands = guarded(async () => {
  const { verdict, done } = family('pairup-hands: a take order blown at the man\'s back is finished by hand, and a man in those hands sees nobody');
  /** on 'fall in', then 'take him on your mark': he is set behind the man — who then rouses (0.6, suspicious) with his back still turned and no mark given.
   *  Once hands are on, the man's head and chest come round at the partner (his state stays the probe's) */
  const run = async (twist: 'settle' | 'shot' | 'loud' | null = null) => {
    const R = await rig({ up: true }); const g = R.game, m = R.mate, T1 = R.tango(1); R.only(T1);
    R.pin(T1, { at: AISLE_MID, yaw: EAST }); g.teleportPlayer([11, 0, 18.5]); R.put(m, AISLE_W, EAST); R.step(10); fallIn(R); R.step(30);
    R.O.give('pairMark', R.foe(T1), 'all'); const set = R.until(() => !!R.P.jobOf(m)?.set, 30);
    const d0 = flat(m.char.pos, T1.char.pos), f0 = PUM.PAIRUP_STATS.finishes, k0 = SPM.SPOTTED_STATS.taken, b0 = PM.PAIR_STATS.blown, s0 = R.opShots, n0 = mine(R).length;
    R.pin(T1, { at: AISLE_MID, yaw: EAST, state: 'suspicious', aw: 0.6, eyesOn: null });
    let frames = 0, gripped = 0, seen = 0, handed = 0, cut = -1, after = 0, afterGrip = 0;
    R.post = () => { if (T1.state === 'dead') return; if (R.P.jobOf(m)?.handed) { handed++;
        if (twist === 'settle') R.pin(T1, { at: AISLE_MID, yaw: EAST, state: 'suspicious', aw: 0.3, eyesOn: null }); }   // (… he settles under the bar again with the partner closing)
      if (cut >= 0) { after++; if (PM.pairGripped(g, T1)) afterGrip++; return; }
      if (!SPM.spottedGripped(g, T1)) return;
      // (hands on, and then nobody tends them: the partner is shot / the squad is told loud — the man must not stay blind)
      if (twist === 'shot' && frames >= 5) { cut = R.frames; killGuard(g, m, [1, 0, 0], false, 'shot', 'guard'); return; }
      if (twist === 'loud' && frames >= 5) { cut = R.frames; SLM.setSilent(g, false); return; }
      R.turn(T1, yawTo(T1.char.pos, m.char.pos)); frames++; if (PM.pairGripped(g, T1)) gripped++; if (GDM.canSeeChar(g, T1, m.char, false).visible) seen++; };
    const down = R.until(() => T1.state === 'dead', 6); R.post = null; R.step(40); const A = R.O.of(m), log = R.log();
    return { set, d0, down, kind: T1.downKind as string, shots: R.opShots - s0, finishes: PUM.PAIRUP_STATS.finishes - f0, taken: SPM.SPOTTED_STATS.taken - k0, blown: PM.PAIR_STATS.blown - b0, handed, frames, gripped, seen, after, afterGrip,
      order: A?.order as string, following: A?.post === null, job: !!R.P.jobOf(m), log, lines: mine(R).slice(n0) };
  };
  const w = (r: Awaited<ReturnType<typeof run>>) => `set ${f2(r.d0)} m behind him · ${r.down ? `down (${r.kind})` : 'still up six seconds on'} · rounds ${r.shots} · finished by hand +${r.finishes}, spotted's takes +${r.taken}, blown +${r.blown} · the job handed on ${r.handed} frames · hands on ${r.frames} frames: gripped ${r.gripped}, his eye has the partner ${r.seen} · then '${r.order}'${r.following ? ' (following)' : ''} · said: ${r.lines.join(' | ') || '(nothing)'}`;
  const a = await run();
  verdict(a.set && a.d0 < SPOTTED.takeM && a.down && a.kind === 'struck' && a.shots === 0 && a.finishes === 1 && a.taken === 1 && a.blown === 1 && a.handed > 0 && /finish:/.test(a.log) && !/off:blown/.test(a.log),
    'the man rouses with the partner set at his back — he is taken by hand, quietly, the blown job counted once', w(a));
  verdict(a.order === 'fallIn' && a.following && !a.job && a.lines.some(t => /he's down\. on you\./.test(t)), '… and the partner, sent from Sam\'s shoulder, falls in again', `order '${a.order}', a job ${a.job}`);
  verdict(a.frames > 20 && a.gripped === a.frames && a.seen === 0, '… and with those hands on him the man, turned to face the partner, sees nobody (PAIR.grip)', `hands on ${a.frames} frames: gripped ${a.gripped}, seen ${a.seen}`);
  // the man settling under the bar again while the partner closes does not hand the job back to the stalk: it is those hands' until they finish or give him up
  const d = await run('settle');
  verdict(d.set && d.down && d.kind === 'struck' && d.finishes === 1 && d.blown === 1 && d.shots === 0 && !/hands:/.test(d.log.slice(d.log.indexOf('handed:'))) && !/off:blown/.test(d.log), 'the man settles again as the partner closes: the job stays those hands\' and is finished once, not started over', w(d));
  // hands nobody is tending any more do not blind the man: the partner shot with his hands on, the squad told loud under them
  const e = await run('shot'), f = await run('loud');
  verdict(e.frames >= 5 && e.after > 20 && e.afterGrip === 0 && !e.down && e.finishes === 0, 'the partner shot with his hands on the man: the grip is off that frame and the man is not out', `hands on ${e.frames} frames, then shot: ${e.after} frames on, gripped ${e.afterGrip} · ${e.down ? 'the man went down all the same' : 'the man is up'}`);
  verdict(f.frames >= 5 && f.afterGrip === 0 && (f.after > 20 || (f.down && f.finishes === 0)), 'the squad told loud with his hands on the man: the grip is off that frame (a loud partner shoots him: no hands finish it)', `hands on ${f.frames} frames, then loud: ${f.after} frames with the man up, gripped ${f.afterGrip} · ${f.down ? `the man is down (${f.kind}), rounds ${f.shots}, finished by hand +${f.finishes}` : 'the man is up'}`);
  return done();
});

// ---------------------------------------------------------------- stay: the rescue's flow leaves a silent pair's one partner on Sam
export const pairupStay = guarded(async () => {
  const { verdict, done } = family('pairup-stay: the hostage rescue with one silent partner — the flow leaves him at Sam\'s shoulder');
  const RP: any = await import('./rescue-probes.ts'); const keep = { ...RESCUE };   // (rescue-probes.ts's rig: the scenario started, Sam walked down the corridor as rescue-flow-pass walks him)
  try {
    const run = async () => {
      reset(); PAIRUP.on = true;
      const T = await RP.rig(12, { doorMan: 'inside', roamers: 0, squad: 1 }); const g = T.game, O = g.orders; SILENT.on = true; T.secs(0.3);
      const m = O.squad()[0], n = O.squad().length; SLM.setSilent(g, true); O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); T.step(5);
      const s0 = PUM.PAIRUP_STATS.stays; let frames = 0, away = 0, far = 0; const orders = new Set<string>();
      const walk = (to: number[]) => { const p = g.player.char.pos, from = [p[0], 0, p[2]], k = Math.max(1, Math.ceil(flat(from, to) / (1.55 * DT)));
        for (let i = 1; i <= k; i++) { g.teleportPlayer([from[0] + (to[0] - from[0]) * i / k, 0, from[2] + (to[2] - from[2]) * i / k]); T.step(); frames++; const o = O.of(m)?.order ?? '—'; orders.add(o); if (o !== 'fallIn') away++; far = Math.max(far, flat(m.char.pos, g.player.char.pos)); } };
      walk([8.2, 0, 11.2]); walk([24.6, 0, 11.2]);
      const takes: string[] = T.R.log.filter((l: any) => l.what.startsWith('flow:take:')).map((l: any) => l.what);
      return { n, frames, away, far, orders: [...orders], takes, stays: PUM.PAIRUP_STATS.stays - s0, hush: SLM.squadSilent(g) && !SLM.silentCompromised(g), up: g.pair.up as boolean };
    };
    const w = (r: Awaited<ReturnType<typeof run>>) => `${f1(r.frames / 60)} s of Sam walking the corridor: his order ${r.orders.join(' / ')} (off 'fall in' on ${r.away} frames), the flow's takes ${r.takes.length ? r.takes.join(' · ') : 'none'}, never more than ${f1(r.far)} m from Sam · asked and left ${r.stays} times`;
    const a = await run();
    verdict(a.n === 1 && a.up && a.hush && a.away === 0 && a.takes.length === 0 && a.stays > 0, 'one partner, silent, not compromised — he stays on \'fall in\' past every opening', w(a));
  } finally { Object.assign(RESCUE, keep); }
  return done();
});

// ---------------------------------------------------------------- a quiet takedown makes no clatter
export const pairupClatter = guarded(async () => {
  const { verdict, done } = family('pairup-clatter: a man taken quietly has his weapon laid down — no clatter filed, nobody else stirs; shot, it bounces and is heard of');
  type Prop = { t: number; level: number; who: number };
  /** Tango 1 armed on the aisle with his back to the pair, Tango 2 a wall away in the corridor (his meter his own: the witness); `how` puts Tango 1 down */
  const run = async (how: 'partner' | 'sam' | 'shot') => {
    const R = await rig({ up: true }); const g = R.game, m = R.mate, T1 = R.tango(1), T2 = R.tango(2); R.only(T1, T2);
    R.pin(T1, { at: AISLE_MID, yaw: EAST }); R.pin(T2, { at: [20, 0, 11], yaw: EAST }); g.teleportPlayer([11, 0, 18.5]); R.put(m, AISLE_W, EAST); R.step(10);
    let ready = true;
    if (how === 'partner') { R.O.give('pairMark', R.foe(T1), 'all'); ready = R.until(() => !!R.P.jobOf(m)?.set, 30) && R.P.mark('api') && R.until(() => (R.P.jobOf(m)?.t0 ?? -1) >= 0, 5); }
    else { R.O.give('hold', R.O.targetAt(m.char.pos), 'all'); R.step(30); if (how === 'sam') { g.teleportPlayer([T1.char.pos[0] - 0.6, 0, T1.char.pos[2]]); R.step(2); } }
    // (every event already filed is old: the count is of what is filed from here — the hands going on — to three seconds after he is down)
    const old = new Set<any>(g.events), props: Prop[] = [], t0 = g.time, aw0 = T2.awareness, st0 = T2.state; let peak = aw0;
    R.post = () => { for (const e of g.events) { if (old.has(e)) continue; old.add(e); if (e.kind === 'prop') props.push({ t: e.time - t0, level: e.level, who: e.who }); } peak = Math.max(peak, T2.awareness); };
    if (how === 'sam') startTakedown(g, T1); else if (how === 'shot') killGuard(g, T1, [1, 0, 0], false, 'shot', 'guard');
    const down = R.until(() => T1.state === 'dead', 6), tDown = g.time - t0; R.step(60 * (how === 'shot' ? 5 : 3)); R.post = null;
    const clatter = props.filter(p => p.who === 0), other = props.filter(p => p.who !== 0);
    return { ok: ready && down, tDown, props, clatter, other, sam: g.player.char.id as number, item: (T1.dropped?.kind ?? null) as string | null, kind: T1.downKind as string, stir: peak - aw0, witness: `${T2.callsign} ${st0} → ${T2.state}, awareness ${f2(aw0)} → peak ${f2(peak)}`, hush: R.silent() && !SLM.silentCompromised(g) };
  };
  const ev = (ps: Prop[]) => ps.length ? ps.map(p => `level ${f2(p.level)} at +${f2(p.t)} s`).join(', ') : 'none';
  const a = await run('partner');
  verdict(a.ok && !!a.item && a.props.length === 0 && a.stir < 0.01 && a.hush, 'the partner\'s takedown on a take order: no \'prop\' event from his hands going on to three seconds after the man is down',
    `'prop' events ${a.props.length} · down (${a.kind}) +${f2(a.tDown)} s after the hands, his ${a.item} laid down · ${a.witness} · the squad still silent ${a.hush}`);
  const b = await run('sam');
  // (player.ts updateTakedown files ONE 'prop' of its own as the cross lands — the fall's thud, level 0.3, under Sam's id: his noise by design, not a weapon let go)
  verdict(b.ok && !!b.item && b.clatter.length === 0 && b.other.length <= 1 && b.other.every(p => p.who === b.sam && p.level <= 0.3 && p.t <= b.tDown + 0.05) && b.stir < 0.01, 'Sam\'s own takedown: no weapon clatter either (its one \'prop\' is the fall\'s thud, his own), and nobody else stirs',
    `weapon clatters (who 0) ${b.clatter.length} · Sam's own thud: ${ev(b.other)} · down (${b.kind}) +${f2(b.tDown)} s, his ${b.item} laid down · ${b.witness}`);
  const c = await run('shot');
  verdict(c.ok && !!c.item && c.clatter.length >= 1 && c.clatter.every(p => p.level > 0.2 && p.level <= 0.5), 'the same armed man shot: the weapon leaves his hand, bounces, and the clatter is filed (≤ 0.5)',
    `weapon clatters ${c.clatter.length}: ${ev(c.clatter)} · his ${c.item} dropped · ${c.witness}`);
  return done();
});

if (import.meta.main) {
  const all: Record<string, () => Promise<boolean>> = { switch: pairupSwitch, feet: pairupFeet, hunted: pairupHunted, 'caught-job': pairupCaughtJob, rejoin: pairupRejoin, voice: pairupVoice, word: pairupWord, grip: pairupGrip, hands: pairupHands, stay: pairupStay, clatter: pairupClatter };
  const want = process.argv.slice(2).filter(a => !a.startsWith('--')); let ok = true;
  for (const k of want.length ? want : Object.keys(all)) { if (!all[k]) { console.log(`unknown: ${k} (${Object.keys(all).join(' ')})`); process.exit(2); } ok = (await all[k]()) && ok; }
  process.exit(ok ? 0 : 1);
}
