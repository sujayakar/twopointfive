// 'reachup' — CROUCHED BEHIND A STANDING MAN, THE GRAB'S REACH GOES UP FOR HIS NECK (CLOSE.reachUp → anim.readyUp, character.ts READY_UP). The maintainer, off
// the stealth tutorial mission with a still of Sam crouched behind a sentry: "the closer than ever animations are still a bit wonky". The tell's free hand is
// measured off Sam's own shoulder, which at the crouch is level with the man's belt: the open hand went to the small of his back and Sam's face to his seat.
//   bun run tools/qa/reachup-check.ts        (also `probes.ts reachup`)
// The key is settled on, and so is `handsLow` (tools/qa/handslow-check.ts), which this card used to hold off: the check asks the page's side alone and
// writes neither. Until 2026-11-07 it also had `up` (crouched at three gaps, the grab's tell with the left hand over 1.05 m up, against the key down) and
// `ease` (crouched in to reach and back out, the wrist's largest one-frame move against the key down): both asked for the grab's tell AT THE CROUCH, which
// `handsLow` took away (crouched, the tell is the knife hand's), so they asked a build from before that rule and went by ruling, with the comparisons with
// the key down and the harness part (the two stands' row, the page's flag, a recording from before the key).
//   stand     on his feet behind the same man: the grab's tell, the hand up at his neck; crouched with the pistol up 'grabGun'.
// REACHUP_BREAK=1 = Sam is stood 2.6 m back, out of reach, where `stand` asks for him behind the man: both lines must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: nothing here is a picture (tools/qa/close-stills.ts + pose-mesh-raster.py draw the same staging on the worn mesh).
import { standUp, ROOT } from './headless.ts';
import { asProbe } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)


type P3 = number[];
const BREAK = process.env.REACHUP_BREAK === '1', DT = 1 / 60, EAST = Math.PI / 2, LOBBY: P3 = [10.2, 0, 19.5];
const f2 = (n: number) => n.toFixed(2);
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

/** the lobby's open floor (close-stills.ts's staging): the shift parked, one man stood facing east, Sam hands empty behind him */
async function lobby() {
  Math.random = seeded(7301); const W: any = await quiet(() => standUp()), g = W.game;
  g.godMode = true; g.aiEnabled = false; g.player.visibility = 0.02; g.player.slot = 'pistol'; g.player.holstered = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const place = (gd: any, p: P3) => { gd.char.pos = [...p]; gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = EAST; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.pinned = true; gd.char.update(0); gd.char.update(DT); };
  const [A, ...rest] = g.guards; rest.forEach((gd: any, i: number) => { place(gd, [37.5, 0, 6 + i * 1.5]); gd.heardUpTo = 1e12; }); place(A, LOBBY); A.wp = 1;
  W.cam.yaw = 0; W.cam.rebuild?.();
  const step = (n = 1) => { for (let i = 0; i < n; i++) { W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); W.step(DT); g.player.visibility = 0.02; } };
  const samAt = (d: number, crouch: boolean) => { g.teleportPlayer([A.char.pos[0] - d, 0, A.char.pos[2]]); g.player.crouch = crouch; const pl = g.player.char; pl.bodyYaw = pl.aimYaw = EAST; step(90); };
  const at = (c: any, node: string): P3 => { const p = c.skel.nodePos(c.anim.nodes[node]), w = c.world; return [w[0] * p[0] + w[4] * p[1] + w[8] * p[2] + w[12], w[1] * p[0] + w[5] * p[1] + w[9] * p[2] + w[13], w[2] * p[0] + w[6] * p[1] + w[10] * p[2] + w[14]]; };
  return { W, g, A, step, samAt, at };
}
interface Read { hand: P3; head: P3; neck: P3; crouch: number; ready: string | null }
async function stopped(gap: number, crouch: boolean, pistol = false): Promise<Read> {
  const S = await lobby(), c = S.g.player.char; if (pistol) S.g.player.holstered = false; S.samAt(BREAK ? 2.6 : gap, crouch);
  return { hand: S.at(c, 'handL'), head: S.at(c, 'head'), neck: S.at(S.A.char, 'neck'), crouch: c.anim.crouch, ready: c.anim.ready };
}

async function stand() {
  console.log('\n=== stand: on his feet the tell is what it was ===');
  const U = await stopped(0.85, false);
  verdict(U.ready === 'grab' && U.hand[1] > 1.2, 'standing 0.85 m behind him: the grab\'s tell, the hand up at his neck', `the tell ${U.ready} · the hand ${f2(U.hand[1])} m up (his neck ${f2(U.neck[1])})`);
  const GU = await stopped(0.85, true, true);
  verdict(GU.ready === 'grabGun', 'crouched 0.85 m behind him with the pistol up: the gun keeps its aim', `the tell ${GU.ready}`);
}
/** the whole card (probes.ts 'reachup'): true = every check passed */
export async function reachUpCheck(): Promise<boolean> {
  const rnd = Math.random; allPass = true;
  try { await stand(); }
  finally { Math.random = rnd; }
  console.log(`\n=== reachup: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit(await asProbe('reachup', reachUpCheck) ? 0 : 1);
