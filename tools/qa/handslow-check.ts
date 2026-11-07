// 'handslow' — CROUCHED, HIS HANDS STAY LOW UNTIL THE MOVE (CLOSE.handsLow, src/game/close.ts closeReady). The maintainer, of a still from the pair's mission with
// Sam crouched a metre behind a guard: "the weirdness in this case was his hand". The grab's tell holds the open left hand out for the man's neck; at the crouch
// (with `reachUp`) that is a hand 1.15 m up over a head 0.9 m up, a third to half a metre short of anybody: from the game's camera, a hand put up.
//   bun run tools/qa/handslow-check.ts        (also `probes.ts handslow`)
// The key is settled on: the check asks the page's side alone and writes no switch. Until 2026-11-07 it also had `low` (crouched at three gaps, key up
// against key down: "the tell is the knife hand's alone", the left hand under his own head and under 0.9 m), the comparisons with the key down in `stand`
// and the harness part (the two stands' row, the page's flag, a recording from before the key). `low` asked for the KNIFE hand's tell ('strike'), which
// no-knife (CLOSE.noKnife, settled on) took away: crouched in reach the page's tell is the grab's with the left hand low (0.42 m up, his head 0.65, on
// the run of 2026-11-07). A build from before that rule: out by ruling.
//   offer     crouched at three spots behind a standing man: the grab is on offer there (close.ts closeAt) and its row is in the list.
//   stand     on his feet behind the same man the tell is the grab's; crouched with the pistol up it is 'grabGun'.
// HANDSLOW_BREAK=1 = Sam is stood 2.6 m back, out of reach, where `offer` and `stand` ask for him behind the man: they must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: nothing here is a picture (tools/qa/close-stills.ts + pose-mesh-raster.py draw the same staging on the worn mesh).
import { standUp, ROOT } from './headless.ts';
import { asProbe } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)

const { closeAt } = await import(`${ROOT}/src/game/close.ts`);

type P3 = number[];
const BREAK = process.env.HANDSLOW_BREAK === '1', DT = 1 / 60, EAST = Math.PI / 2, LOBBY: P3 = [10.2, 0, 19.5];
const f2 = (n: number) => n.toFixed(2);
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

/** the lobby's open floor (reachup-check.ts's staging): the shift parked, one man stood facing east, Sam hands empty behind him */
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
interface Read { hand: P3; head: P3; crouch: number; ready: string | null; grab: boolean; row: boolean }
async function stopped(gap: number, crouch: boolean, pistol = false): Promise<Read> {
  const S = await lobby(), g = S.g, c = g.player.char; if (pistol) g.player.holstered = false; S.samAt(BREAK ? 2.6 : gap, crouch);
  return { hand: S.at(c, 'handL'), head: S.at(c, 'head'), crouch: c.anim.crouch, ready: c.anim.ready,
    grab: !!closeAt(g, S.A, c.pos).grab, row: g.nearby.some((x: any) => x.kind === 'guard' && x.guard === S.A && x.verb === 'grab') };
}
const GAPS = [1.0, 0.85, 0.72];

async function offer() {
  console.log('\n=== offer: the grab is still there to take ===');
  const rows: string[] = []; let bad = 0;
  for (const gap of GAPS) { const U = await stopped(gap, true); if (!U.grab || !U.row) bad++; rows.push(`${f2(gap)} m: the grab on offer ${U.grab}, its row in the list ${U.row}`); }
  verdict(bad === 0, 'crouched at the same three spots: the grab is on offer and its row is in the list', rows.join(' · '));
}
async function stand() {
  console.log('\n=== stand: on his feet, and with the pistol up, the tell is what it was ===');
  const U = await stopped(0.85, false);
  verdict(U.ready === 'grab', 'standing 0.85 m behind him: the grab\'s tell', `the tell ${U.ready} · the hand ${f2(U.hand[1])} m up`);
  const GU = await stopped(0.85, true, true);
  verdict(GU.ready === 'grabGun', 'crouched 0.85 m behind him with the pistol up: the gun keeps its aim', `the tell ${GU.ready}`);
}

/** the whole card (probes.ts 'handslow'): true = every check passed */
export async function handsLowCheck(): Promise<boolean> {
  const rnd = Math.random; allPass = true;
  try { await offer(); await stand(); }
  finally { Math.random = rnd; }
  console.log(`\n=== handslow: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit(await asProbe('handslow', handsLowCheck) ? 0 : 1);
