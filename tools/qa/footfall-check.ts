// 'footfall' — ONE FOOTFALL A STEP (FOOTFALL.once, src/game/player.ts). A footfall is filed when the walk cycle crosses its half or turns over. The cycle runs
// backwards while he backpedals, and backpedalling is decided from where he aims against where he goes: with the cursor at his own feet that flips frame by
// frame, the cycle rocks over one edge, and every frame of it filed a footfall (seven running in a scripted creep up to a man's back: seven crouched steps in
// a tenth of a second to the man's ears).
//   bun run tools/qa/footfall-check.ts        (also `probes.ts footfall`; run it on BOTH stands: TPS_PROFILE=shipped in front for the second)
//   rock      crouched on the lobby's open floor, walking, and from the frame a footfall is filed the forward key and the back key by turns, a frame each,
//             for a third of a second (a scripted player hunting its mark; the drag's facing rule has no slack either). Key down: a footfall on five or
//             more frames running (the flaw). Key up: two at the most in that third of a second and no two of the whole run on frames next to each
//             other. On three seeds (the cycle starts each at another place).
//   held      the second place a footfall is filed, a man walked in the arm: the same rocking, the same two verdicts.
// FOOTFALL_BREAK=1 = the key left down where `rock` and `held` ask for it up: each must FAIL.   exit 0 = all hold · 1 = one does not
// (Cut 2026-11-07 to the two arms that cost under a minute and read the game: `rock` and `held`.)
import { standUp, ROOT } from './headless.ts';
import { sample } from './sweep.ts';

const { FOOTFALL, startGrab } = await import(`${ROOT}/src/game/player.ts`);
const { CLOSE } = await import(`${ROOT}/src/game/close.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { saveState } = await import(`${ROOT}/src/game/savestate.ts`);
const PR = await import(`${ROOT}/src/host/profile.ts`);
const JR = await import(`${ROOT}/src/game/journal.ts`);
const { STAND_ENV } = await import(`${ROOT}/src/host/standEnv.ts`);

type P3 = number[];
const BREAK = process.env.FOOTFALL_BREAK === '1', DT = 1 / 60, START: P3 = [5.2, 0, 19.5], SEEDS = [4411, 1, 2], FWD = 'KeyD', BACK = 'KeyA';   // (east along the lobby: some 17 m of clear floor)
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

interface Fall { f: number; level: number }
interface Ctx { W: any; g: any; f: number; falls: Fall[]; step(aim: 'ahead' | 'behind' | P3): void; keys(...k: string[]): void; A: any; moved(): number }
/** the lobby's open floor, the floor's men parked far off but one (`near`: stood 1.8 m in front of where the walk begins, his back to it); a walk is keys held
 *  and the cursor put 4 m ahead of his travel, 4 m behind it, or on a point, each frame. Every footfall filed is noted with the frame it was filed on. */
async function world(once: boolean, crouch: boolean, seed: number, near = false, owed = once): Promise<Ctx> {
  Math.random = seeded(seed); const W: any = await quiet(() => standUp({ seed } as any)), g = W.game; FOOTFALL.once = once; FOOTFALL.owed = owed;   // (`owed`: the second key, which is under the first: up wherever the first is, as the page has them, unless an arm asks)
  g.godMode = true; g.aiEnabled = false; g.playerInvisible = true;
  const [A, ...rest] = g.guards; rest.forEach((gd: any, i: number) => { gd.char.pos = [37.5, 0, 6 + i * 1.5]; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.heardUpTo = 1e12; });
  W.cam.yaw = 0; W.cam.rebuild?.(); g.teleportPlayer([...START]); g.player.crouch = crouch;
  const pl = g.player.char, seen = new Set<object>(); let path = 0, last = [...pl.pos];
  const C: Ctx = { W, g, f: 0, falls: [], A, moved: () => path, keys: (...k) => { W.input.keys.clear(); for (const x of k) W.input.keys.add(x); }, step: aim => {
    const v = pl.vel, sp = Math.hypot(v[0], v[2]), d = sp > 0.05 ? [v[0] / sp, v[2] / sp] : [1, 0], k = aim === 'ahead' ? 4 : -4;
    W.cursorAt(typeof aim === 'string' ? [pl.pos[0] + d[0] * k, 1.0, pl.pos[2] + d[1] * k] : aim); W.step(DT); g.player.visibility = 0.02; C.f++;
    path += Math.hypot(pl.pos[0] - last[0], pl.pos[2] - last[2]); last = [...pl.pos];
    for (const e of g.events) if (e.kind === 'step' && !seen.has(e)) { seen.add(e); C.falls.push({ f: C.f, level: e.level }); }
  } };
  A.char.pos = near ? [START[0] + 1.8, 0, START[2]] : [37.5, 0, 4.5]; A.char.bodyYaw = A.char.aimYaw = Math.PI / 2; A.path = []; A.pathGoal = null; A.hold = true; A.pinned = true; A.awareness = 0; if (!near) A.heardUpTo = 1e12;
  for (let i = 0; i < 30; i++) C.step([START[0] + 5, 1.0, START[2]]);
  return C;
}
/** the longest run of footfalls on frames next to each other, and the footfalls with each such run counted once */
const longest = (F: Fall[]) => { let best = F.length ? 1 : 0, run = 1; for (let i = 1; i < F.length; i++) { run = F[i].f === F[i - 1].f + 1 ? run + 1 : 1; best = Math.max(best, run); } return best; };


/** walk until a footfall is filed (the cycle has just crossed an edge), then `n` frames of the forward key and the back key by turns with the cursor where
 *  it was (a scripted player hunting its mark does exactly this: it is how the flaw was met) */
function rockRun(C: Ctx, n: number, after = 90, fwd = FWD, back = BACK, leg = 1): { from: number; inRock: number; at: P3 } {
  C.keys(fwd); let from = -1;
  for (let i = 0; i < 240 && from < 0; i++) { const had = C.falls.length; C.step('ahead'); if (i > 40 && C.falls.length > had) from = C.f; }
  const pl = C.g.player.char, v = pl.vel, sp = Math.hypot(v[0], v[2]) || 1, far: P3 = [pl.pos[0] + v[0] / sp * 5, 1.0, pl.pos[2] + v[2] / sp * 5];
  const at = [...pl.pos]; for (let i = 0; i < n; i++) { C.keys(Math.floor(i / leg) % 2 ? fwd : back); C.step(far); }
  const inRock = C.falls.filter(x => x.f > from && x.f <= from + n).length;
  if (after > 0) C.keys(fwd); else C.keys(); for (let i = 0; i < Math.abs(after); i++) C.step(far);
  return { from, inRock, at };
}
async function rock() {
  console.log('\n=== rock: forwards and backwards by turns, a frame each, at an edge of the cycle ===');
  const rows: string[] = [], asked = sample(SEEDS, 2); let mended = 0;
  for (const seed of asked) {
    const on = await world(!BREAK, true, seed), b = rockRun(on, 20);
    if (b.from > 0 && b.inRock <= 2 && longest(on.falls) <= 1 && on.falls.length >= 4) mended++;
    rows.push(`seed ${seed}: key up ${b.inRock}, ${longest(on.falls)} running, ${on.falls.length} in the run`);
  }
  verdict(mended === asked.length, 'key up: two footfalls at the most in that third of a second, no two of the run on frames next to each other, and he still treads before and after', `${mended} of ${asked.length} seeds`);
}
async function held() {
  console.log('\n=== held: a man walked in the arm (the second place a footfall is filed) ===');
  const run = async (once: boolean) => {
    const C = await world(false, true, SEEDS[0], true), g = C.g, A = C.A; CLOSE.on = true; A.pinned = true;
    g.teleportPlayer([A.char.pos[0] - 0.8, 0, A.char.pos[2]]); for (let i = 0; i < 40; i++) C.step([A.char.pos[0], 1.2, A.char.pos[2]]);
    startGrab(g, A); for (let i = 0; i < 90; i++) C.step([A.char.pos[0] + 3, 1.2, A.char.pos[2]]);
    const holding = !!g.player.holding; FOOTFALL.once = FOOTFALL.owed = once; C.falls.length = 0;
    const R = rockRun(C, 20, 200, BACK, FWD); return { holding, R, falls: C.falls.slice(), still: !!g.player.holding, after: C.falls.filter(x => x.f > R.from + 20).length };
  };
  const b = await run(!BREAK);
  verdict(b.holding && b.still && b.R.from > 0 && b.R.inRock <= 2 && longest(b.falls) <= 1 && b.after >= 2, 'key up: two at the most, none on frames running, and the walk after it still treads', `holding ${b.holding}, still at the end ${b.still} · ${b.R.inRock} footfalls in the 20 frames of it, ${longest(b.falls)} running · in the walk after it ${b.after}`);
}
// ---- A FOOTFALL THE LEGS OWE (FOOTFALL.owed, under FOOTFALL.once) ----

const STEADY = ['crouched', 'standing', 'sprinting', 'sprint on and off', 'up and down', 'stop and go', 'crouch stop and go'], TURNED = ['round a square', 'a slow sweep of the cursor', 'backwards'];

/** the whole card (probes.ts 'footfall'): true = every check passed. The switches are put back as they were found. */
export async function footfallCheck(): Promise<boolean> {
  const was = { ...FOOTFALL }, close = { ...CLOSE }, rnd = Math.random; allPass = true;
  try { await rock(); await held(); }
  finally { Object.assign(FOOTFALL, was); Object.assign(CLOSE, close); Math.random = rnd; }
  console.log(`\n=== footfall: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit(await footfallCheck() ? 0 : 1);
