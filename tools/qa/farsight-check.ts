// A LIT MAN IS SEEN FROM PAST 16 m — the check behind src/game/guards.ts `GUARD_EYE.far` / `eyeReach` (a settled rule: the game has no other side of it, so
// nothing here asks one. Until 2026-11-07 six lines asked the switch off, or on and off alike). The fault it holds shut: every eye stopped at 16 m whatever
// the light, so a man could stand lit at 17 to 21 m in a guard's view for as long as he liked and never be seen.
//   bun run tools/qa/farsight-check.ts            (also `probes.ts farsight`)
//   FARSIGHT_BREAK=1 bun run tools/qa/farsight-check.ts   the meter is fed dark wherever the check lights a man: the lit lines must FAIL (exit 1)
// What it asks, on the office's open south strip (one man planted facing east, the other two parked; Sam's light meter fed by hand — headless nothing else writes it):
//   reach   eyeReach's own numbers off the table as it stands: `range` under `farLo` on the meter, half way at the middle, `farRange` from `farHi`; `range` under a show
//   eye     canSee over distance × light: seen exactly where the distance is inside the reach
//   dark    in full light the notice builds slower with every step out
//   men     one man's eye on another (canSeeChar off the watched man's own meter): lit at 20 m seen, dark at 20 m not; lit inside 16 m seen
//   notice  Sam standing still in full light in the man's view: noticed at 20 m and at 25 m, slower the further out, slower than at 12 m; never in the dark
//           at 20 m
// exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { GUARD_EYE, eyeReach, canSee, canSeeChar, sightGain, sightGainOn } = await import(`${ROOT}/src/game/guards.ts`);
type Vec3 = [number, number, number];
const DT = 1 / 60, EAST = Math.PI / 2, AT: Vec3 = [4, 0, 2.5], f2 = (n: number) => n.toFixed(2);
const BREAK = process.env.FARSIGHT_BREAK === '1', lux = (lit: number) => BREAK ? Math.min(lit, 0.05) : lit;   // (the break word: every meter the check feeds is a dark one)

function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
async function stage(seed: number) {
  const W: any = await standUp(); const g = W.game;
  g.godMode = true; g.player.visibility = 0.02; g.player.slot = 'pistol'; g.player.holstered = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = g.guards;
  for (const [gd, i] of [[B, 0], [C, 1]] as [any, number][]) { gd.hold = true; place(gd, [37.5, 0, 6 + i * 1.5], EAST); gd.heardUpTo = 1e12; }
  place(A, AT, EAST); A.wp = 1; A.hold = true; A.heardUpTo = 1e12;
  for (const h of [0.3, 1.1, 1.65]) if (g.col.segmentBlocked([AT[0], h, AT[2]], [AT[0] + 28, h, AT[2]])) throw new Error(`farsight: the south strip is no longer a clear 28 m at ${h} m up — the level moved under the check`);
  return { W, g, A, B, pl: g.player };
}
/** Sam stood `d` m in front of the man, lit `lit`, one settled frame run */
function samAt(S: any, d: number, lit0: number, frames = 1) { const { W, g } = S, lit = lux(lit0); g.teleportPlayer([AT[0] + d, 0, AT[2]]); g.player.crouch = false; g.player.visibility = lit; for (let i = 0; i < frames; i++) { W.step(DT); g.player.visibility = lit; } }

export async function farsightCheck(): Promise<boolean> {
  let fails = 0, checks = 0;
  const say = (ok: boolean, what: string) => { checks++; if (!ok) fails++; console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what}`); };
  const log = console.log, die = Math.random; console.log = () => { }; let S: any; Math.random = mulberry32(7);
  try { S = await stage(7); } finally { console.log = log; }
  const R = GUARD_EYE.range as number, FR = GUARD_EYE.farRange as number, LO = GUARD_EYE.farLo as number, HI = GUARD_EYE.farHi as number, MID = (LO + HI) / 2, step = (t: number) => t * t * (3 - 2 * t);
  const reachOf = (l: number) => l >= HI ? FR : l <= LO ? R : R + (FR - R) * step((l - LO) / (HI - LO));
  try {
    const { g, A, B } = S;
    // --- reach
    { const got = [0.02, LO - 0.01, MID, HI, 1].map(l => eyeReach(g, l)), want = [R, R, (R + FR) / 2, FR, FR];
      say(FR > R && got.every((v, i) => Math.abs(v - want[i]) < 1e-6), `reach: ${R} m in the dark, ${f2((R + FR) / 2)} at ${f2(MID)} on the meter, ${FR} from ${HI} (got ${got.map(f2).join(' / ')})`);
      const tr = g.touring; g.touring = true; const show = eyeReach(g, 1); g.touring = tr;
      say(show === R, `reach: ${R} m under a show, whatever the light (${show})`); }
    // --- eye
    const DIST = [R - 4, R - 0.5, R + 2, (R + FR) / 2 - 0.5, (R + FR) / 2 + 1, FR - 0.5, FR + 1], LIT = [0.05, LO - 0.1, MID, 1];
    const calmA = () => { A.awareness = 0; A.state = 'patrol'; };
    const read = () => { const r = canSee(g, A); return { vis: r.visible as boolean, rate: r.visible ? sightGain(g, A, r.dist) as number : 0 }; };
    const ON: { vis: boolean; rate: number }[][] = [];   // each cell: Sam stood there three frames (the teleport's own motion gone), the man calm again
    for (const l of LIT) { const a: typeof ON[0] = []; for (const d of DIST) { calmA(); samAt(S, d, l, 3); calmA(); a.push(read()); } ON.push(a); }
    { let bad = 0, far = 0; LIT.forEach((l, i) => DIST.forEach((d, j) => { const want = d < reachOf(l); if (ON[i][j].vis !== want) bad++; if (ON[i][j].vis && d > R) far++; }));
      say(bad === 0 && far > 0, `eye: Sam is seen exactly where he stands inside the reach his light gives (${LIT.length * DIST.length} cells, ${bad} wrong; ${far} seen past 16 m; full light: ${DIST.filter((_, j) => ON[3][j].vis).join(', ')} m)`);
    }
    // --- dark
    { const r = ON[3].map(c => c.rate); let mono = true; for (let j = 1; j < r.length; j++) if (ON[3][j].vis && !(r[j] < r[j - 1])) mono = false;
      say(mono && r[3] > 0, `dark: in full light the rate falls with every step out (${r.map(f2).join(' / ')} per s at ${DIST.join(' / ')} m)`); }
    // --- men
    { const put = (d: number, lit: number) => { S.g.teleportPlayer([AT[0] + 2, 0, AT[2] + 12]); place(B, [AT[0] + d, 0, AT[2]], -EAST); B.visibility = lux(lit); };
      const ask = () => { const r = canSeeChar(g, A, B.char, false); return { vis: r.visible as boolean, rate: r.visible ? sightGainOn(g, A, B, r.dist) as number : 0 }; };
      const D = R + (FR - R) * 0.4; put(D, 1); const lit = ask(); put(D, 0.05); const dark = ask(); put(R - 4, 1); const near = ask();
      say(lit.vis && lit.rate > 0 && !dark.vis, `men: one man's eye on another at ${f2(D)} m — lit he is seen (${f2(lit.rate)} per s), in the dark not`);
      say(near.vis && near.rate > lit.rate, `men: lit inside ${R} m he is seen, and sooner (${f2(near.rate)} per s)`);
      place(B, [37.5, 0, 6], EAST); B.visibility = 0; }
    // --- notice
    const secsTo = (d: number, lit0: number, max = 10) => { const lit = lux(lit0); A.awareness = 0; A.state = 'patrol'; samAt(S, d, lit); for (let f = 1; f <= max * 60; f++) { S.W.step(DT); g.player.visibility = lit; if (A.awareness >= 0.85 || A.state === 'alert') return f / 60; } return Infinity; };
    const calm = (d: number, lit0: number, secs = 8) => { const lit = lux(lit0); A.awareness = 0; A.state = 'patrol'; samAt(S, d, lit); let top = 0; for (let f = 1; f <= secs * 60; f++) { S.W.step(DT); g.player.visibility = lit; top = Math.max(top, A.awareness); } return top; };
    { const darkTop = calm(20, 0.05);
      say(darkTop === 0, `notice: in the dark at 20 m he is not noticed in 8 s (awareness ${f2(darkTop)})`);
      const dA = R - 4, dB = R + (FR - R) * 0.4, dC = FR - 1, tA = secsTo(dA, 1), tB = secsTo(dB, 1), tC = secsTo(dC, 1, 20);
      say(Number.isFinite(tB) && Number.isFinite(tC) && tA < tB && tB < tC, `notice: standing still in full light he has the man's full attention in ${f2(tA)} s at ${f2(dA)} m, ${f2(tB)} s at ${f2(dB)} m, ${f2(tC)} s at ${f2(dC)} m`); }
  } finally { Math.random = die; }
  console.log(checks === 0 ? '=== farsight-check: NO VERDICT ===' : fails ? `=== farsight-check: ${fails} FAILED ===` : '=== farsight-check: ALL PASS ===');
  return fails === 0 && checks > 0;
}
if (import.meta.main) process.exit((await farsightCheck()) ? 0 : 1);
