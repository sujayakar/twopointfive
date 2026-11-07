// CLOSE-UP TAKEDOWNS AS MOVES (src/game/takedown.ts — the animator's half in character.ts, the fall in ragdollize, the breath in breathFrame), headless:
//   bun run tools/qa/takedown-check.ts [moves|partner|breathe|lower|beats …]      (probes.ts takedown)
// The switch and its halves (blow, victim, fall, partner, breathe, lower, kill, chokeBeats, haul) are settled on, and so is no-knife (CLOSE.noKnife): the check
// asks the page's side alone and writes no switch. Until 2026-11-07 every move was staged twice, switch off and on, and three probes asked only that:
// takedown-rules (off against on: the same frame, downKind, hearing events, chronicle rows, draws), takedown-off (nothing counted with it off) and
// takedown-harness (the two stands' row, the page's flag, a recording's tables). They are out, with every line or half that held a run to its off run (the
// body not where the old fall left it, the partner's old stand-in, nothing breathing with it off, the choke as it was with the lowering or the beats off,
// the head not up at once with a half off). And THE KNIFE is out: the three knife blows, the kill from the hold as a move (takedown-kill, with the breath
// line "a man killed never breathes") asked a build from before no-knife. The lowering's and the beats' lines also held the choke's haul off (TAKEDOWN.haul,
// settled on, tools/qa/haul-check.ts): they run with it now.
//   moves     one blow / one fall counted a move, the attacker in his move before it lands, the fall's style read once, the body at rest inside
//             four seconds on the floor, the fold forward and the topple backward
//   partner   the partner's blow lands inside his takedown: the man reacts on his feet (his head moves)
//   breathe   a man out cold breathes — the PICTURE's chest joint (the skeleton's globals / skin matrices) rises and falls while `bones`, the capsules and the
//             particles stand
//   lower     the choke lowers him: nothing under the floor, nothing pops, eased off both stand back up
//   beats     the choke's beats: hands on the arm at once, off it before he is lowered, the head to the side first, held at the bottom, and a
//             man grabbed again after a choke broken off starts with his hands where a fresh hold has them; Sam comes up slowly afterwards
// TAKEDOWN_BREAK=1 = no move is ever begun (the blow, the grab and the partner's order are not given): every part must fail at its staging line.
// The stagings are tools/qa/takedown-strip.ts's own (that file runs on import, so they are repeated here): Sam's blows through player.ts startTakedown, the
// hold through startGrab and the held rows, the partner's through the pair's 'take him now' order. Nothing here looks at a picture.
import { standUp, ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)

const { TAKEDOWN, TAKEDOWN_STATS } = await import(`${ROOT}/src/game/takedown.ts`);   // (TAKEDOWN is read for its numbers: partnerHit, breathHz)
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { startTakedown, startGrab } = await import(`${ROOT}/src/game/player.ts`);
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { takeFall } = await import(`${ROOT}/src/game/takedown.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { RAG, RAG_COUNT } = await import(`${ROOT}/src/game/ragdoll.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const SLM: any = await import(`${ROOT}/src/game/silent.ts`);
const PM: any = await import(`${ROOT}/src/game/pairorders.ts`);
const KN: any = await import(`${ROOT}/src/game/knobs.ts`);

type P3 = number[];
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const BREAK = process.env.TAKEDOWN_BREAK === '1';
const DT = 1 / 60, DEG = Math.PI / 180, EAST = Math.PI / 2, LOBBY: P3 = [9.5, 0, 19.5];
const cm = (x: number) => (Number.isFinite(x) ? (x * 100).toFixed(1) : String(x)), f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

// ---------------------------------------------------------------- the stagings (takedown-strip.ts's)
/** the die every run draws: the strip's seeded Math.random behind a counter (+ the game's own die's count where the stand gave it one: TPS_SEED) */
interface Die { n: number }
function die(seed: number): Die { const r = seeded(seed), D: Die = { n: 0 }; Math.random = () => { D.n++; return r(); }; return D; }
const drawn = (D: Die, g: any) => D.n + (g.dice?.n ?? 0);
/** the module's counts, zeroed for a run */
function zero() { Object.assign(TAKEDOWN_STATS, { blows: 0, falls: 0, partner: 0, breathing: 0 }); }

interface Stage {
  W: any; g: any; D: Die;
  /** who does it · the man it is done to (a Guard) */
  att: any; vic: any;
  /** one frame, as the staging steps it */
  step: () => void;
  /** the move begins (the span the rules are compared over starts here) */
  begin: () => void;
  /** asked after every frame of the span: something the staging does part way (the hold's row) */
  during?: (f: number) => void;
  /** frames the man must be out within */
  cap: number;
  /** the partner's: has the blow begun (his hands on) */
  hands?: () => boolean;
  /** the hold's: Sam put back behind the man where he now stands, crouched, as before the grab */
  behind?: () => void;
}
function place(gd: any, p: P3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
function park(gd: any, i: number) { gd.hold = true; gd.pinned = true; place(gd, [37.5, 0, 6 + i * 1.5], EAST); gd.heardUpTo = 1e12; }
/** the lobby's open floor: one man planted facing east, calm, the others parked; Sam's hands empty, unhurtable, nobody perceiving anything */
async function lobby(seed: number) {
  const D = die(seed); const W: any = await hushed(() => standUp()); const g = W.game; zero();
  g.godMode = true; g.aiEnabled = false; g.player.visibility = 0.02; g.player.slot = 'pistol'; g.player.holstered = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = g.guards; park(B, 0); park(C, 1); place(A, LOBBY, EAST); A.wp = 1; A.hold = true;
  W.cam.yaw = 0; W.cam.rebuild?.();
  const step = (n = 1) => { for (let i = 0; i < n; i++) { W.step(DT); g.player.visibility = 0.02; } };
  const samAt = (bDeg: number, d: number, crouch = false) => { const a = A.char.bodyYaw + bDeg * DEG; g.teleportPlayer([A.char.pos[0] + Math.sin(a) * d, 0, A.char.pos[2] + Math.cos(a) * d]); g.player.crouch = crouch;
    const pl = g.player.char; pl.bodyYaw = pl.aimYaw = Math.atan2(A.char.pos[0] - pl.pos[0], A.char.pos[2] - pl.pos[2]); W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); step(20); };
  return { W, g, A, D, step, samAt };
}
async function blow(bDeg: number, dist: number): Promise<Stage> {
  const S = await lobby(7001), { W, g, A, D } = S; S.samAt(bDeg, dist); S.step(1);
  return { W, g, D, att: g.player, vic: A, step: () => S.step(1), begin: () => startTakedown(g, A, undefined, false), cap: 120 };
}
/** THE KILL FROM THE HOLD as tools/qa/takedown-strip.ts stages it for its stills (that file imports these two; no line of this check asks the knife any
 *  more). The phase is put on the way player.ts setPhase puts one on, and ended as the rules made it at HOLD.secs.kill. */
export function setHoldPhase(g: any, phase: string) { const H = g.player.holding; if (!H) return; H.phase = phase; H.t = 0; H.windT = -1; H.fireQueued = false; if (H.g.held) H.g.held.phase = phase; }
export function killEnd(g: any, how: 'kill' | undefined = 'kill'): boolean {
  const pl = g.player, H = pl.holding; if (!H || H.phase !== 'kill' || H.t < HOLD.secs.kill) return false;
  const gd = H.g, f = pl.char.forward(); gd.held = null; gd.char.anim.heldPose = null; gd.talk.heldSince = -1;
  takeFall(g, gd, undefined, true, how); killGuard(g, gd, f, true, 'shot', 'player', 'knife — dead'); pl.holding = null; pl.char.anim.holdPose = null; return true;
}
async function hold(end: 'choke' | 'whip'): Promise<Stage> {
  const S = await lobby(7002), { W, g, A, D } = S; S.samAt(180, 0.8, true); g.player.holstered = end !== 'whip'; S.step(2);
  const ahead = () => { const c = g.player.char; W.cursorAt([c.pos[0] + Math.sin(c.bodyYaw) * 6, 1.0, c.pos[2] + Math.cos(c.bodyYaw) * 6]); };
  const behind = () => { S.samAt(180, 0.8, true); g.player.holstered = end !== 'whip'; S.step(2); };
  return { W, g, D, att: g.player, vic: A, step: () => { ahead(); S.step(1); }, begin: () => startGrab(g, A), during: f => { if (f === 90) { g.selectVerb('held', end); W.input.pressed.add('Space'); } }, cap: 90 + 360, behind };
}
/** the partner's: the room clear on the office with ONE teammate, silent; a defender stood on the open aisle, the partner told 'take him now' */
async function partner(): Promise<Stage> {
  Object.assign(NIGHTBREACH, { squad: 1 }); const D = die(10);
  const { W, g } = await hushed(async () => { const W: any = await standUp(); const g = W.game; zero();
    Object.assign(SLM.SILENT, { on: true }); Object.assign(PM.PAIR, { on: true }); g.godMode = true;
    const why = g.roomClear.start(10); if (why) throw new Error(`partner: the room clear did not start: ${why}`); return { W, g }; });
  const O = g.orders, step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } };
  step(30); SLM.setSilent(g, true); step(2);
  const mate = O.squad()[0], foe = g.guards.find((h: any) => h.callsign === 'Tango 1'); if (!mate || !foe) throw new Error('partner: no teammate / no Tango 1 on the floor');
  const put = (gd: any, p: P3, yaw: number) => { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.lastPos = [p[0], 0, p[2]]; gd.path = []; gd.pathGoal = null; };
  put(foe, [22, 0, 17.6], EAST); foe.hold = true; put(mate, [19, 0, 17.6], EAST); g.teleportPlayer([11, 0, 18.5]); step(30); PM.PAIR_LOG.length = 0;
  return { W, g, D, att: mate, vic: foe, step: () => step(1), cap: 960, hands: () => PM.PAIR_LOG.some((x: any) => String(x.what).startsWith('hands:')),
    begin: () => { const no = O.give('pairTake', { kind: 'foe', at: [foe.char.pos[0], 0, foe.char.pos[2]], foe }); if (no) throw new Error(`partner: 'take him now' refused: ${no}`); } };
}
/** what each move should count and leave: a blow of Sam's own (the hold's ends have none) · the fall's style · whether the man is out cold (breathes) or dead */
interface Move { title: string; stage: () => Promise<Stage>; blow: boolean; fall: string; kind: string; sam: boolean }
const MOVES: Record<string, Move> = {
  'knockout-behind': { title: 'the knockout from behind', stage: () => blow(180, 0.6), blow: true, fall: 'fold', kind: 'struck', sam: true },
  'knockout-side': { title: 'the knockout from his left side', stage: () => blow(90, 0.8), blow: true, fall: 'topple', kind: 'struck', sam: true },
  'knockout-front': { title: 'the knockout from in front', stage: () => blow(0, 0.75), blow: true, fall: 'topple', kind: 'struck', sam: true },
  'grab-choke': { title: 'the grab, the hold, the choke', stage: () => hold('choke'), blow: false, fall: 'sink', kind: 'choked', sam: true },
  'grab-whip': { title: 'the grab with the pistol out, the pistol whip', stage: () => hold('whip'), blow: false, fall: 'fold', kind: 'struck', sam: true },
  'partner': { title: "the partner's takedown ('take him now')", stage: partner, blow: false, fall: 'fold', kind: 'struck', sam: false },
};
const NAMES = Object.keys(MOVES);

// ---------------------------------------------------------------- one run, everything the lines ask measured in it
interface Breath { asleep: boolean; set: boolean; chest: number; joints: number; bones: number; caps: number; rag: number }
interface Run {
  move: string; err: string | null;
  /** RULES — up to and including the frame he is counted out (frames from `begin`; -1 never) */
  out: number; downKind: string; events: string[]; chron: string[]; draws: number; sam: P3;
  /** the whole run: draws to its end · the module's counts · frames anybody was in a blow / the attacker was, before it landed · frames anybody held a fall style or a breath between frames */
  drawsAll: number; stats: Record<string, number>; blowAny: number; blowAtt: number; fallSeen: number; breathSeen: number; fallAfter: string | null;
  /** the fall: s from out to at rest (asleep, or every particle under 2 cm/s; NaN = not inside 4 s) · every particle finite · the lowest particle (m) · the particles
   *  at 4 s · the head's signed distance (m) from where his feet were along the facing he had */
  rest: number; fastAt4: number; slept: number; trace: string[]; fastWho: string; finite: boolean; low: number; lie: number[]; headAlong: number;
  /** the partner's: the frame his hands went on · the frame the blow lands · the head bone's furthest move from where it was then, until he is out · on his feet all that time */
  hands: number; lands: number; headMove: number; headIdle: number; stood: boolean;
  breath: Breath | null;
}
const chars = (g: any): any[] => [g.player.char, ...g.guards.map((x: any) => x.char), ...g.civilians.map((x: any) => x.char)];
const copyBones = (c: any): Record<string, P3> => { const o: Record<string, P3> = {}; for (const k of Object.keys(c.bones)) o[k] = [...c.bones[k]]; return o; };
const bonesDiff = (a: Record<string, P3>, c: any) => { let m = 0; for (const k of Object.keys(a)) { const b = c.bones[k]; m = Math.max(m, b ? Math.hypot(a[k][0] - b[0], a[k][1] - b[1], a[k][2] - b[2]) : Infinity); } return m; };
const flatNums = (x: any, out: number[] = []): number[] => { if (typeof x === 'number') out.push(x); else if (x && typeof x === 'object') for (const v of Array.isArray(x) || ArrayBuffer.isView(x) ? Array.from(x as any) : Object.values(x)) flatNums(v, out); return out; };
const numsDiff = (a: ArrayLike<number>, b: ArrayLike<number>) => { let m = a.length === b.length ? 0 : Infinity; for (let i = 0; i < Math.min(a.length, b.length); i++) m = Math.max(m, Math.abs(a[i] - b[i])); return m; };

async function runOnce(move: string): Promise<Run> {
  const M = MOVES[move], rnd = Math.random, knobs = KN.knobsCopy(), squad = NIGHTBREACH.squad;
  const R: Run = { move, err: null, out: -1, downKind: '', events: [], chron: [], draws: 0, sam: [NaN, NaN, NaN], drawsAll: 0, stats: {}, blowAny: 0, blowAtt: 0, fallSeen: 0, breathSeen: 0, fallAfter: null,
    rest: NaN, fastAt4: NaN, slept: NaN, trace: [], fastWho: '', finite: true, low: Infinity, lie: [], headAlong: NaN, hands: -1, lands: -1, headMove: 0, headIdle: NaN, stood: true, breath: null };
  try {
    const S = await M.stage(), { g, vic, att } = S, vc = vic.char;
    const seen = new Set<any>(g.events), chron0 = g.chronicle.events.length, d0 = drawn(S.D, g);
    const feet: P3 = [vc.pos[0], 0, vc.pos[2]], fwd: P3 = [Math.sin(vc.bodyYaw), 0, Math.cos(vc.bodyYaw)];
    let head0: P3 | null = null, f = 0; const heads: P3[] = [];
    const look = () => {   // (between frames: what stands on anybody)
      for (const c of chars(g)) { if (c.anim?.blowing) R.blowAny++; if (c.fallAs) R.fallSeen++; if (c.breathe) R.breathSeen++; }
      if (R.out < 0 && att.char.anim.blowing) R.blowAtt++;
    };
    if (!BREAK) S.begin(); look();
    // 1. to the frame he is counted out
    while (R.out < 0 && f < S.cap) {
      S.during?.(f); S.step(); f++;
      for (const e of g.events) if (!seen.has(e)) { seen.add(e); R.events.push(`${f}:${e.kind}:${e.level ?? '-'}`); }
      if (S.hands) heads.push([...vc.bones.head]);
      if (S.hands && R.hands < 0 && S.hands()) { R.hands = f; R.lands = f + Math.round(TAKEDOWN.partnerHit * 60); }
      if (vic.state === 'dead') { const n = f - R.lands, a = heads[R.hands - 1 - n]; if (R.lands >= 0 && a) { R.headIdle = 0; for (let i = R.hands - 1 - n; i < R.hands - 1; i++) R.headIdle = Math.max(R.headIdle, Math.hypot(heads[i][0] - a[0], heads[i][1] - a[1], heads[i][2] - a[2])); }
        R.out = f; R.downKind = String(vic.downKind); R.chron = g.chronicle.events.slice(chron0).map((e: any) => e.kind); R.draws = drawn(S.D, g) - d0; R.sam = [...g.player.char.pos]; }
      else if (R.lands >= 0 && f >= R.lands) { const h: P3 = vc.bones.head; if (!head0) head0 = [...h]; else R.headMove = Math.max(R.headMove, Math.hypot(h[0] - head0[0], h[1] - head0[1], h[2] - head0[2])); R.stood &&= !vc.ragdoll && vc.alive !== false; }
      look();
    }
    if (R.out < 0) throw new Error(`the man was never counted out in ${S.cap} frames (state ${vic.state})`);
    R.fallAfter = vc.fallAs ?? null;
    // 2. the fall: four seconds of it
    const rag = vc.ragdoll; if (!rag) throw new Error('counted out with no ragdoll');
    for (let k = 1; k <= 240; k++) {
      const prev = Float64Array.from(rag.x as ArrayLike<number>); S.step(); look();
      let fast = 0, who = 0; for (let i = 0; i < RAG_COUNT; i++) { const d = Math.hypot(rag.x[i * 3] - prev[i * 3], rag.x[i * 3 + 1] - prev[i * 3 + 1], rag.x[i * 3 + 2] - prev[i * 3 + 2]) / DT; if (d > fast) { fast = d; who = i; } if (!Number.isFinite(d)) R.finite = false; }
      if (Number.isNaN(R.rest) && (rag.sleeping || fast < 0.02) && k > 6) R.rest = k * DT;   // (the first frames of a led fall hold the neck: 'slow' there is not 'at rest')
      else if (!Number.isNaN(R.rest) && !rag.sleeping && fast >= 0.02) R.rest = NaN;           // (…and it moved again: not at rest yet)
      R.fastAt4 = fast; if (k === 240) { const sp = g.player.char.pos; R.fastWho = fast > 0 ? `${Object.keys(RAG).find(n => RAG[n] === who)} at height ${cm(rag.x[who * 3 + 1])} cm, ${cm(Math.hypot(rag.x[who * 3] - sp[0], rag.x[who * 3 + 2] - sp[2]))} cm from Sam on the plan, moving ${[0, 1, 2].map(a => cm((rag.x[who * 3 + a] - prev[who * 3 + a]) / DT)).join(' / ')} cm/s x / y / z` : 'none'; }
      if (k % 30 === 0) R.trace.push(cm(fast)); if (Number.isNaN(R.slept) && rag.sleeping) R.slept = k * DT;
    }
    R.lie = Array.from(rag.x as ArrayLike<number>); for (let i = 0; i < RAG_COUNT; i++) R.low = Math.min(R.low, rag.x[i * 3 + 1]); R.finite &&= R.lie.every(Number.isFinite);
    const hp = rag.point(RAG.head); R.headAlong = (hp[0] - feet[0]) * fwd[0] + (hp[2] - feet[2]) * fwd[2];
    // 3. the breath: asleep, three seconds on, then one whole breath watched
    for (let k = 0; k < 480 && !rag.sleeping; k++) { S.step(); look(); if (rag.sleeping) R.slept = 4 + (k + 1) * DT; }
    for (let k = 0; k < 180; k++) { S.step(); look(); }
    const sk = vc.skel, node = vc.anim.nodes.spine2, chest = (): P3 => [sk.globals[node][12], sk.globals[node][13], sk.globals[node][14]];
    const b0 = copyBones(vc), c0 = flatNums(vc.capsules()), x0 = Float64Array.from(rag.x), j0 = sk.jointData ? Float32Array.from(sk.jointData) : null;
    const lo = chest(), hi = chest(); const B: Breath = { asleep: !!rag.sleeping, set: !!vc.breathe, chest: 0, joints: 0, bones: 0, caps: 0, rag: 0 };
    for (let k = 0, n = Math.ceil(60 / TAKEDOWN.breathHz) + 12; k < n; k++) {
      S.step(); look(); B.set ||= !!vc.breathe;
      const p = chest(); for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a], p[a]); hi[a] = Math.max(hi[a], p[a]); }
      if (j0) B.joints = Math.max(B.joints, numsDiff(j0, sk.jointData));
      B.bones = Math.max(B.bones, bonesDiff(b0, vc)); B.caps = Math.max(B.caps, numsDiff(c0, flatNums(vc.capsules()))); B.rag = Math.max(B.rag, numsDiff(x0, rag.x));
    }
    B.chest = Math.hypot(hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]); B.asleep &&= !!rag.sleeping; R.breath = B;
    R.drawsAll = drawn(S.D, g) - d0; R.stats = { ...TAKEDOWN_STATS };
  } catch (e) { R.err = e instanceof Error ? e.message : String(e); }
  finally { KN.knobsBack(knobs); NIGHTBREACH.squad = squad; Math.random = rnd; }
  return R;
}
const RUNS = new Map<string, Promise<Run>>();
const run = (move: string): Promise<Run> => { let p = RUNS.get(move); if (!p) RUNS.set(move, p = runOnce(move)); return p; };
const J = (o: unknown) => JSON.stringify(o);
const bad = (F: { verdict: Verdict }, ...rs: Run[]) => { const e = rs.filter(r => r.err); for (const r of e) F.verdict(false, `${r.move}: the staging ran`, r.err!); return e.length > 0; };

// ───────────────────────────────────────────────────────────── moves
export async function takedownMoves(): Promise<boolean> {
  const F = family('takedown-moves: one blow and one fall a move, the fall read once, the body at rest on the floor, forward for the fold and backward for the topple');
  for (const m of NAMES) {
    const M = MOVES[m], B = await run(m); if (bad(F, B)) continue;
    const wantBlows = M.sam && M.blow ? 1 : 0, wantPartner = M.sam ? 0 : 1;
    F.verdict(B.downKind === M.kind, `${m}: he is counted out, '${M.kind}'`, `out at frame ${B.out} · downKind ${B.downKind} · hearing events ${B.events.length} [${B.events.join(' ')}] · chronicle [${B.chron.join(' ')}] · draws to that frame ${B.draws} (to the run's end ${B.drawsAll})`);   // (what is left of takedown-rules' line)
    F.verdict(B.stats.blows === wantBlows && B.stats.partner === wantPartner && B.stats.falls === 1 && B.fallAfter === null && B.fallSeen === 0,
      `${m}: counted once (${M.fall}), the fall's style read once and gone`,
      `stats ${J(B.stats)} (want blows ${wantBlows} · partner ${wantPartner} · falls 1) · frames the attacker was in a blow before the man was out ${B.blowAtt} of ${B.out} · fallAs on the frame he went down ${J(B.fallAfter)} · frames a fall style stood between frames ${B.fallSeen}`);
    F.verdict(Number.isFinite(B.rest) && B.rest <= 4 && B.finite && B.low > -0.05, `${m}: the body comes to rest inside four seconds, every particle finite and on the floor`,
      `at rest ${f2(B.rest)} s after he went down · fastest particle at 4 s ${cm(B.fastAt4)} cm/s (${B.fastWho}; every half second: ${B.trace.join(' ')}) · the ragdoll asleep at ${f2(B.slept)} s · finite ${B.finite} · lowest particle ${cm(B.low)} cm`);
  }
  { const B = await run('knockout-behind'); if (!bad(F, B)) F.verdict(B.headAlong > 0.3, 'knockout from behind = FOLD: his head ends up in FRONT of where his feet were', `head ${f2(B.headAlong)} m along his facing from his feet`); }
  { const B = await run('knockout-front'); if (!bad(F, B)) F.verdict(B.headAlong < -0.3, 'knockout from in front = TOPPLE: his head ends up BEHIND where his feet were', `head ${f2(B.headAlong)} m along his facing from his feet`); }
  { const rows: string[] = []; for (const m of NAMES) { const B = await run(m); if (!B.err) rows.push(`${m} ${f2(B.headAlong)}`); }
    console.log(`    NOTE — every move's head, m along his facing from his feet: ${rows.join(' · ')}`); }
  return F.done();
}

// ───────────────────────────────────────────────────────────── partner
export async function takedownPartner(): Promise<boolean> {
  const F = family("takedown-partner: the partner's blow lands inside his takedown — the man reacts on his feet");
  const B = await run('partner'); if (bad(F, B)) return F.done();
  const span = (r: Run) => `hands on at frame ${r.hands} · the blow lands at ${r.lands} · out at ${r.out} (${r.out - r.hands} frames = ${f2((r.out - r.hands) * DT)} s after the hands)`;
  F.verdict(B.hands > 0 && B.lands < B.out, 'the hands go on, the blow lands, and then the man is counted out', span(B));
  F.verdict(B.stood && B.headMove > 0.04, 'from the blow landing until he is counted out he is on his feet (not down, no ragdoll) and his head moves more than 4 cm', `head bone moved ${cm(B.headMove)} cm · on his feet throughout ${B.stood} · downKind ${B.downKind}`);
  F.verdict(B.stats.partner === 1, 'the partner\'s blow is counted once', `stats ${J(B.stats)}`);
  return F.done();
}

// ───────────────────────────────────────────────────────────── breathe
export async function takedownBreathe(): Promise<boolean> {
  const F = family("takedown-breathe: a man out cold breathes — the picture's chest joint only; bones, capsules and particles stand");
  const say = (b: Breath) => `chest joint ${(b.chest * 1000).toFixed(2)} mm peak to peak · skin matrices moved ${b.joints.toExponential(1)} · bones ${b.bones.toExponential(1)} m · capsules ${b.caps.toExponential(1)} m · particles ${b.rag.toExponential(1)} m · asleep ${b.asleep} · breathe set ${b.set}`;
  console.log(`    NOTE — the picture's joints ARE reachable headless: Character.skel.globals[anim.nodes.spine2] (model space, what breathFrame writes through skel.update) and skel.jointData (the skin matrices); one breath = ${f2(1 / TAKEDOWN.breathHz)} s, watched from 3 s after the body slept`);
  for (const m of NAMES) {
    const B = await run(m); if (bad(F, B)) continue; const b = B.breath!;
    const still = (x: Breath) => x.bones <= 1e-9 && x.caps <= 1e-9 && x.rag <= 1e-9;
    F.verdict(b.asleep && b.set && b.chest > 0.003 && b.joints > 0, `${m} (out cold: ${B.downKind}): the picture's chest joint rises and falls more than 3 mm over a breath`, say(b));
    F.verdict(still(b), `${m}: what the game reads does not move at all — bones, capsules, particles (≤ 1e-9)`, say(b));
  }
  return F.done();
}

// ───────────────────────────────────────────────────────────── lower
interface ChokeRun { err: string | null; out: number; kind: string; man: Record<string, P3>[]; sam: Record<string, P3>[]; phase: string[] }
/** the grab, the hold, the choke, both skeletons after every frame from the grab (the tap on 'choke' is frame 90) until he is out. `ease` = a Space tap on that
 *  frame (the choke eased off: he breathes again and the hold goes on); the run then goes on `after` frames */
export async function chokeRun(ease = -1, after = 0): Promise<ChokeRun> {
  const rnd = Math.random, knobs = KN.knobsCopy(), R: ChokeRun = { err: null, out: -1, kind: '', man: [], sam: [], phase: [] };
  try {
    const S = await hold('choke'), { g, vic } = S; if (!BREAK) S.begin();
    for (let f = 0, end = ease >= 0 ? ease + after : S.cap; f < end; f++) {
      S.during?.(f); if (f === ease) S.W.input.pressed.add('Space'); S.step();
      if (vic.state === 'dead') { R.out = f + 1; R.kind = String(vic.downKind); break; }
      R.man.push(copyBones(vic.char)); R.sam.push(copyBones(g.player.char)); R.phase.push(String(g.player.holding?.phase ?? '-'));
    }
  } catch (e) { R.err = e instanceof Error ? e.message : String(e); }
  finally { KN.knobsBack(knobs); Math.random = rnd; }
  return R;
}
const dist3 = (a: P3, b: P3) => Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
const offFloor = (b: Record<string, P3>) => Math.min(...Object.entries(b).filter(([k]) => !/^(foot|toe)/.test(k)).map(([, p]) => p[1]));
const fastest = (R: ChokeRun, from: number): [number, string] => { let pop = 0, at = '';
  for (let i = from; i < R.man.length; i++) for (const [who, b] of [['man', R.man], ['Sam', R.sam]] as const) for (const k of Object.keys(b[i])) { const d = dist3(b[i][k], b[i - 1][k]); if (d > pop) { pop = d; at = `${who}'s ${k} at frame ${i}`; } }
  return [pop, at]; };
export async function takedownLower(): Promise<boolean> {
  const F = family("takedown-lower: the choke LOWERS him — nothing under the floor, nothing pops; eased off, both stand back up");
  const low = await chokeRun();
  if (low.err || low.out < 0) { F.verdict(false, 'the choke ran to its end', low.err ?? 'he was never out'); return F.done(); }
  const n = low.man.length, tap = 90;
  F.verdict(low.kind === 'choked', 'he is out cold', `out at frame ${low.out} · ${low.kind}`);
  { // nothing under the floor, nothing pops
    let floor = Infinity; const [pop, popAt] = fastest(low, tap + 1);
    for (let i = tap; i < n; i++) floor = Math.min(floor, offFloor(low.man[i]), offFloor(low.sam[i]));
    F.verdict(floor >= 0.08, 'no joint of either but the feet they stand on comes within 8 cm of the floor line', `lowest joint ${cm(floor)} cm up · on the choke's last frame the man's hips at ${f2(low.man[n - 1].hips[1])} m, Sam's at ${f2(low.sam[n - 1].hips[1])} m`); }
  // eased off part way down: both stand back up, the hold goes on; Sam's own trunk and arm come back out on the eased clock, so no joint of HIS pops
  for (const at of [tap + 150, tap + 165]) {
    const E = await chokeRun(at, 60);
    if (E.err || E.out >= 0 || E.man.length < at + 60) { F.verdict(false, `the choke eased off at frame ${at}: the hold goes on`, E.err ?? (E.out >= 0 ? `he was out at frame ${E.out}` : `only ${E.man.length} frames`)); continue; }
    const held = tap - 5, e = E.man.length - 1, dip = E.man[held].hips[1] - E.man[at - 1].hips[1], mUp = E.man[held].hips[1] - E.man[e].hips[1], sUp = E.sam[held].hips[1] - E.sam[e].hips[1];
    const [pop, popAt] = fastest(E, at);
    let hips = 0; for (let i = at; i <= e; i++) hips = Math.max(hips, dist3(E.man[i].hips, E.man[i - 1].hips), dist3(E.sam[i].hips, E.sam[i - 1].hips));
    F.verdict(dip >= 0.1 && E.phase[e] === 'held' && Math.abs(mUp) <= 0.03 && Math.abs(sUp) <= 0.03, `the choke eased off ${cm(dip)} cm down (a tap at frame ${at}): a second later both stand as they did in the hold`,
      `phase '${E.phase[e]}', his hips ${cm(mUp)} cm under the hold's, Sam's ${cm(sUp)} cm`);
    let sam = 0, samAt = ''; for (let i = at; i <= e; i++) for (const k of Object.keys(E.sam[i])) { const d = dist3(E.sam[i][k], E.sam[i - 1][k]); if (d > sam) { sam = d; samAt = `${k} at frame ${i}`; } }
    F.verdict(sam <= 0.04, 'eased off, no joint of Sam moves more than 4 cm in a frame: his trunk and arm come back out of the choke over half a second', `fastest ${cm(sam)} cm (${samAt})`);
    F.verdict(hips <= 0.02, 'on the way back up neither pelvis moves more than 2 cm a frame', `hips ${cm(hips)} cm a frame · the fastest joint of either ${cm(pop)} cm (${popAt})`);
  }
  return F.done();
}

const segDist = (p: P3, a: P3, b: P3) => { const ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]], t = Math.max(0, Math.min(1, ((p[0] - a[0]) * ab[0] + (p[1] - a[1]) * ab[1] + (p[2] - a[2]) * ab[2]) / (ab[0] ** 2 + ab[1] ** 2 + ab[2] ** 2))); return dist3(p, [a[0] + ab[0] * t, a[1] + ab[1] * t, a[2] + ab[2] * t]); };
/** a point in the man's own frame as he stood (his facing `yaw`): [to his left, up, ahead] of `o` */
const inFrame = (p: P3, o: P3, yaw: number): P3 => { const dx = p[0] - o[0], dz = p[2] - o[2]; return [dx * Math.cos(yaw) - dz * Math.sin(yaw), p[1] - o[1], dx * Math.sin(yaw) + dz * Math.cos(yaw)]; };

// ───────────────────────────────────────────────────────────── beats
export async function takedownBeats(): Promise<boolean> {
  const F = family("takedown-beats: the choke's beats off the clips — both his hands on the forearm a quarter second in, off it and hanging before he is lowered, his head to the side before it comes forward");
  const B = await chokeRun();
  if (B.err || B.out < 0) { F.verdict(false, 'the choke ran to its end', B.err ?? 'he was never out'); return F.done(); }
  const c0 = B.phase.indexOf('choke'), C = HOLD.secs.choke * 60, at = (u: number) => Math.min(B.man.length - 1, c0 + Math.round(u * C));
  F.verdict(B.kind === 'choked' && c0 > 0, 'the choke starts and he is out cold', `choke from frame ${c0} · out at ${B.out} · ${B.kind}`);
  { const i = c0 + 21, a = Math.max(segDist(B.man[i].handL, B.sam[i].forearmL, B.sam[i].handL), segDist(B.man[i].handR, B.sam[i].forearmL, B.sam[i].handL));
     }
  { const i = at(0.62), j = at(0.5), a = Math.min(B.man[i].neck[1] - B.man[i].handL[1], B.man[i].neck[1] - B.man[i].handR[1]);
     }
  { // HELD at the bottom: the last third of a second nothing sinks further
    const i = at(1 - 0.33 / HOLD.secs.choke), e = B.man.length - 1, a = Math.max(Math.abs(B.man[e].hips[1] - B.man[i].hips[1]), Math.abs(B.sam[e].hips[1] - B.sam[i].hips[1]));
    const c = B.man[e].chest, h = B.man[e].hips, bow = Math.atan2(Math.hypot(c[0] - h[0], c[2] - h[2]), c[1] - h[1]) / DEG;
    F.verdict(a <= 0.01, "the bottom is a beat of its own — over the choke's last 0.33 s neither pelvis moves 1 cm", `a pelvis moves ${cm(a)} cm at most over those frames · his trunk (hips to chest joint) bowed ${bow.toFixed(1)}° forward at the end`); }
  { const side = (i: number) => inFrame(B.man[i].head, B.man[i].neck, Math.PI / 2);
    const i0 = at(0.45), i1 = at(0.63), tip = Math.abs(side(i1)[0] - side(i0)[0]), fw = side(i1)[2] - side(i0)[2], fwEnd = side(at(0.9))[2] - side(i0)[2];
    F.verdict(tip >= 0.012 && fwEnd >= fw + 0.02, 'his head tips to the side first (its joint 1.2 cm or more across his neck by 63 % of the choke) and comes forward after', `head sideways ${cm(tip)} cm · forward ${cm(fw)} cm by then, ${cm(fwEnd)} cm by 90 %`); }
  for (const tap of [c0 + Math.round(0.3 * C), c0 + Math.round(0.6 * C)]) {   // eased off with his hands on the arm, and with them falling
    const E = await chokeRun(tap, 60);
    if (E.err || E.out >= 0 || E.man.length < tap + 60) { F.verdict(false, `the choke eased off at frame ${tap}: the hold goes on`, E.err ?? (E.out >= 0 ? `he was out at frame ${E.out}` : `only ${E.man.length} frames`)); continue; }
    const [pop, popAt] = fastest(E, tap);
    F.verdict(E.phase[E.man.length - 1] === 'held', `eased off at frame ${tap}: the hold goes on`, `phase '${E.phase[E.man.length - 1]}' · the fastest joint ${cm(pop)} cm a frame (${popAt}; the line that held it to the same ease-off with the beats off went with that run)`);
  }
  { const G = await regrabRun();   // a choke broken off, the man grabbed again: the beats start over with the hold
     }
  await riseLines(F, 24, 0.6, 108, 'he is out');
  return F.done();
}
/** the first grab and hold (90 frames), the choke to 62 %, the hold broken by freeHeld('shot'), a second and a half for the man to come loose, Sam behind him
 *  again and a second grab: how far each wrist is under his neck at frame k of each grab — the worst the second is under the first */
async function regrabRun(): Promise<{ err: string | null; broke: number; n: number; worst: number; at: string }> {
  const rnd = Math.random, knobs = KN.knobsCopy(), R = { err: null as string | null, broke: 0, n: 0, worst: -Infinity, at: '' };
  try {
    const S = await hold('choke'), { g, vic } = S, b = vic.char.bones, under = () => [b.neck[1] - b.handL[1], b.neck[1] - b.handR[1]];
    const first: number[][] = []; if (!BREAK) S.begin(); let f = 0;
    for (; f < 90; f++) { S.step(); first.push(under()); }
    for (; f < 600; f++) { S.during?.(f); S.step(); const H = g.player.holding; if (!H || vic.state === 'dead') throw new Error(`the hold ended by itself at frame ${f}`); if (H.phase === 'choke' && H.t >= 0.62 * HOLD.secs.choke) { R.broke = H.t / HOLD.secs.choke; break; } }
    g.freeHeld('shot'); for (let k = 0; k < 90; k++) S.step();
    if (g.player.holding || vic.state === 'dead' || vic.char.ragdoll) throw new Error(`after freeHeld: holding ${!!g.player.holding} · his state ${vic.state}`);
    S.behind!(); startGrab(g, vic); if (!g.player.holding) throw new Error('the second grab did not take');
    for (let k = 0; k < 60; k++) { S.step(); const u = under(); if (k >= 5) { R.n++; for (const s of [0, 1]) { const d = u[s] - first[k][s]; if (d > R.worst) { R.worst = d; R.at = `his ${s ? 'right' : 'left'} wrist at frame ${k + 1} of the grab`; } } } }
  } catch (e) { R.err = e instanceof Error ? e.message : String(e); }
  finally { KN.knobsBack(knobs); Math.random = rnd; }
  return R;
}
/** AFTER THE MAN IS OUT OF HIS ARM (the choke run to its end): Sam's head height for two seconds — standing where he is, walked off from that frame (W
 *  held), or standing still with the pistol drawn and raised from that frame ('aim': the holster flag off and the right button held — `upper` = the
 *  animator's upper mode from the frame it leaves 'none', `seen` = that frame). `left` = the share of the way back up still to go at frame k (against where
 *  the head is at the end of the run). */
async function riseRun(walk: boolean | 'aim'): Promise<{ err: string | null; down: number; left: (k: number) => number; gap: (k: number) => number; speed: number; upper: string; seen: number }> {
  const rnd = Math.random, knobs = KN.knobsCopy(), head: number[] = []; let err: string | null = null, held = NaN, speed = 0, upper = 'none', seen = -1;
  try {
    const S = await hold('choke'), { g, vic } = S, sam = g.player.char; if (!BREAK) S.begin();
    for (let f = 0; f < 600 && vic.state !== 'dead'; f++) { S.during?.(f); if (f === 89) held = sam.bones.head[1]; S.step(); if (vic.state !== 'dead') head[0] = sam.bones.head[1]; }
    if (vic.state !== 'dead') throw new Error('he was never out');
    for (let k = 1; k <= 120; k++) { if (walk === true) S.W.input.keys.add('KeyW'); if (walk === 'aim') { g.player.holstered = false; S.W.input.buttons |= 2; }
      S.step(); head.push(sam.bones.head[1]); speed = Math.max(speed, Math.hypot(sam.vel[0], sam.vel[2])); if (seen < 0 && sam.anim.upper !== 'none') { seen = k; upper = String(sam.anim.upper); } }
    S.W.input.keys.delete('KeyW'); S.W.input.buttons &= ~2;
  } catch (e) { err = e instanceof Error ? e.message : String(e); }
  finally { KN.knobsBack(knobs); Math.random = rnd; }
  const ref = head[head.length - 1];
  return { err, down: held - head[0], speed, upper, seen, left: k => (ref - head[k]) / (ref - head[0]), gap: k => ref - head[k] };
}
/** …its three lines: standing still he comes up slowly; walked off he is up at once; the gun raised does not wait for it either */
async function riseLines(F: { verdict: Verdict }, k: number, share: number, by: number, when: string) {
  const A = await riseRun(false), Wk = await riseRun(true), e = A.err ?? Wk.err;
  if (e) { F.verdict(false, 'after the choke: the runs ran', e); return; }
  const pc = (x: number) => `${Math.round(x * 100)} %`;
  F.verdict(A.down >= 0.2 && A.left(k) >= share && A.left(by) <= 0.1, `once ${when} Sam does not pop upright — standing where he is, ${f2(k * DT)} s on ${pc(share)} or more of the way back up is still to go, and by ${f2(by * DT)} s he is up (a tenth or less left)`,
    `his head was ${cm(A.down)} cm under the hold's as ${when} · still to go at ${f2(k * DT)} s: ${pc(A.left(k))} (${cm(A.gap(k))} cm) · at ${f2(by * DT)} s: ${pc(A.left(by))}`);
  F.verdict(A.down >= 0.2 && Wk.down >= 0.2 && Wk.speed >= 0.6 && Wk.left(k) <= 0.3 && Wk.left(k) <= A.left(k) - 0.3, `…and it never holds him — walked off from that frame, the pose lets go at the rate it always did: ${pc(0.3)} or less of the way up left at ${f2(k * DT)} s, and 30 points less than standing still`,
    `walking ${f2(Wk.speed)} m/s · still to go at ${f2(k * DT)} s: ${pc(Wk.left(k))} walking · ${pc(A.left(k))} standing`);
  const Am = await riseRun('aim'); if (Am.err) { F.verdict(false, 'after the choke, the gun raised: the run ran', Am.err); return; }
  const j = Math.max(Am.seen, 0) + 15;   // (a quarter second after the animator is first told: the old rate has 6 % left by then)
  F.verdict(Am.down >= 0.2 && Am.seen >= 1 && Am.seen <= 6 && Am.speed < 0.1 && Am.left(j) <= 0.25 && Am.left(j) <= A.left(j) - 0.25, `…nor does it hold his hands — standing STILL, the pistol drawn from that frame and the aim button held: a quarter second after the animator is first told (inside 0.1 s), a quarter or less of the way up is left, 25 points less than idle`,
    `the animator's upper mode leaves 'none' for '${Am.upper}' at frame ${Am.seen} · moving ${f2(Am.speed)} m/s · still to go at ${f2(j * DT)} s: ${pc(Am.left(j))} with the gun out · ${pc(A.left(j))} idle`);
}

/** the file's probes, by the names the runner calls them */
const SUBS: Record<string, () => Promise<boolean>> = { 'takedown-moves': takedownMoves, 'takedown-partner': takedownPartner, 'takedown-breathe': takedownBreathe, 'takedown-lower': takedownLower, 'takedown-beats': takedownBeats };
if (import.meta.main) {
  if (BREAK) console.log("TAKEDOWN_BREAK=1: no move is ever begun — moves / partner / breathe / lower / beats must fail at their staging lines");
  const ok = await asProbes(SUBS, wanted());   // (a name the file has not: said, nothing run, exit 2)
  if (ok !== null) console.log(ok ? '\n=== takedown: ALL PASS ===' : '\n=== takedown: FAILED ===');
  process.exit(exitCode(ok));
}
