// THE MOON IN THE ESTIMATE (src/game/sense.ts SENSE.moon; src/game/hours.ts moonLight; guards.ts lightEstimateAt) — held on the office at night, headless.
// EXIT 0 = every line passes · 1 = one fails.
//   bun run tools/qa/moonlight-check.ts          (TPS_PROFILE=shipped in front for the other stand: the check holds on both)
//   switch    down, the estimate is the one asked with no moon, to the bit, at every point; up, the one asked with it. (That the estimate with no moon
//             is the estimate as it WAS is not held here: the missions played with the switch down against the build before are — docs/agents/rust.md)
//   sky       points under the open sky with a line to the moon: the estimate is never under the moon's own light on the face turned most to it (its
//             luminance × its intensity × that cosine)
//   shadow    ON THE OFFICE: points under the open sky with a wall between them and the moon, and points under a roof, gain less than a quarter of
//             that (what comes of the night's sky and off moonlit ground, and no more) — the moon is SHADOWED, which is why the estimate may have it
//             at all. (A level with a dim moon and a bright sky would want another bar: the sky's term is not shadowed, it is let in by what is open)
//   twice     nobody adds the moon on top: the goggles' light of a place (nvg.ts nvgPlaceLight) with the switch up is the estimate's own, and the
//             level's moon is said to be in the estimate (hours.ts inEstimate) exactly while the switch is up
//   hour      the level started at another hour: the moon is out, the hour's sun stands in its place, and the switch adds nothing, to the bit; the
//             night put back, it adds what it added
//   put back  a game put back from a snapshot reads every point as it did, and its state's word is the one it had
// MOONLIGHT_BREAK=1 is the red path: nothing stands between a point and anything while the moon is asked, so `shadow` must FAIL.
// Not game code; nothing here needs a GPU, and nothing here has been seen on one. (How near the estimate with the moon stands to a GPU's picture is
// tools/qa/modelerr.ts's row `moonlit`, against the kept report.)
import { resolve } from 'path';
import { standUp } from './headless.ts';
import { noWalls } from './standwall.ts';
const ROOT = resolve(import.meta.dir, '../..');
const S: any = await import(`${ROOT}/src/game/sense.ts`), NV: any = await import(`${ROOT}/src/game/nvg.ts`), GD: any = await import(`${ROOT}/src/game/guards.ts`), HR: any = await import(`${ROOT}/src/game/hours.ts`), ME: any = await import(`${ROOT}/src/host/modelerr.ts`), RB: any = await import(`${ROOT}/src/game/rollback.ts`);
type Vec3 = [number, number, number];
const BREAK = process.env.MOONLIGHT_BREAK === '1';
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const verdict = (ok: boolean, what: string, detail: string) => { if (!ok) bad++; log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };

hush(true);
const W: any = await standUp({ uploadEvery: 1000, seed: 1 }), g = W.game; for (let i = 0; i < 30; i++) W.step(1 / 60);
hush(false);
// the points: rings round the error tool's own stations (free floor all over the level), at chest height
const pts: Vec3[] = [];
for (const s of ME.stationsOf(g, 6) as Vec3[]) for (const r of [0, 1.5, 3, 5]) for (let k = 0; k < (r ? 8 : 1); k++) {
  const a = k * Math.PI / 4, x = s[0] + Math.sin(a) * r, z = s[2] + Math.cos(a) * r, nav = g.col.nav;
  if (x > 0.5 && z > 0.5 && x < nav.W * nav.cell - 0.5 && z < nav.H * nav.cell - 0.5 && !nav.isBlocked(x, z)) pts.push([x, 1.2, z]);
}
const moon = HR.skyLampOf(g);   // (the game's own word for which light that is)
if (!moon) { log('=== moonlight-check: the level has no moon — FAIL ==='); process.exit(1); }
const to: Vec3 = [-moon.dir[0], -moon.dir[1], -moon.dir[2]], I = (0.2126 * moon.color[0] + 0.7152 * moon.color[1] + 0.0722 * moon.color[2]) * moon.intensity, direct = I * Math.max(Math.abs(to[0]), Math.abs(to[1]), Math.abs(to[2]));
const real = g.col.segmentBlocked.bind(g.col);
const est = (P: Vec3, m?: boolean) => GD.lightEstimateAt(g, P, undefined, false, null, m);
/** what the switch adds at P (the red path: asked of a level with no static standing, wall, roof or floor — standwall.ts noWalls: every box hidden, as
 *  the Rust module's copy reads it too; a question asked at once, no game is played under it) */
const adds = (P: Vec3) => { const back = BREAK ? noWalls(g) : null; try { return est(P, true) - est(P, false); } finally { back?.(); } };
const sees = (P: Vec3) => !real(P, [P[0] + to[0] * 40, P[1] + to[1] * 40, P[2] + to[2] * 40]), open = (P: Vec3) => !g.weather.underRoof(P[0], P[2]);

// ---- switch
{ S.SENSE.moon = false; let differ = 0; for (const P of pts) if (!Object.is(est(P), est(P, false))) differ++;
  S.SENSE.moon = true; let up = 0; for (const P of pts) if (Object.is(est(P), est(P, true))) up++; S.SENSE.moon = false;
  verdict(pts.length >= 60 && differ === 0 && up === pts.length, 'switch: down, the estimate is the one asked with no moon, to the bit; up, the one asked with it',
    `${pts.length} points · down: ${differ} differ from the estimate with no moon · up: ${up} are the estimate with the moon`); }
// ---- sky, shadow
{ const lit = pts.filter(P => open(P) && sees(P)), shade = pts.filter(P => open(P) && !sees(P)), roof = pts.filter(P => !open(P) && !sees(P));
  const a = lit.map(adds), b = shade.map(adds), c = roof.map(adds), min = (v: number[]) => v.length ? Math.min(...v) : NaN, max = (v: number[]) => v.length ? Math.max(...v) : NaN;
  const least = Math.min(...lit.map(P => est(P, true)));
  verdict(lit.length >= 5 && least >= direct * 0.999, 'sky: a point under the open sky with a line to the moon has the moon\'s own light on it',
    `the moon ${I.toFixed(3)} W/m² on a face turned to it, ${direct.toFixed(3)} on the meter's best face · ${lit.length} such points: the estimate with the moon is ${least.toFixed(3)} at least (the switch adds ${min(a).toFixed(3)} to ${max(a).toFixed(3)})`);
  verdict(shade.length >= 5 && roof.length >= 5 && max(b) < direct / 4 && max(c) < direct / 4, 'shadow: behind a wall or under a roof the moon is shadowed',
    `${shade.length} points under the sky with a wall between them and the moon: the switch adds at most ${max(b).toFixed(3)} · ${roof.length} under a roof: at most ${max(c).toFixed(3)} · the bar ${(direct / 4).toFixed(3)}${BREAK ? '\n    NOTHING STOOD IN THE MOON\'S WAY' : ''}`); }
// ---- twice
{ const place = (P: Vec3) => GD.lightEstimateAt(g, P, () => true); let over = 0, under = 0;
  S.SENSE.moon = true; const inUp = HR.inEstimate(g, moon); for (const P of pts) { const a = NV.nvgPlaceLight(g, P), b = place(P); if (a > b) over++; else if (a < b) under++; }
  S.SENSE.moon = false; const inDown = HR.inEstimate(g, moon); let adds = 0; for (const P of pts) if (NV.nvgPlaceLight(g, P) > place(P)) adds++;
  verdict(inUp && !inDown && over === 0 && under === 0 && adds >= 5, 'twice: with the switch up nobody adds the moon on top of the estimate',
    `the moon is in the estimate: up ${inUp}, down ${inDown} · the goggles' light of a place over ${pts.length} points, up: ${over} above the estimate's own, ${under} below · down: ${adds} with the goggles' own moon on top, as ever`); }
// ---- hour
{ const before = pts.map(adds); hush(true); HR.applyHour(g, 'dusk'); hush(false);
  let some = 0; for (const P of pts) if (!Object.is(est(P, true), est(P, false))) some++;
  hush(true); HR.applyHour(g, 'night'); hush(false);
  const after = pts.map(adds), back = after.every((v, i) => Object.is(v, before[i]));
  verdict(some === 0 && back, 'hour: at another hour the moon is out and the switch adds nothing; the night put back, it adds what it did',
    `dusk: ${some} of ${pts.length} points differ with the switch · night again: ${back ? 'every point as before' : 'NOT as before'}`); }
// ---- put back
{ S.SENSE.moon = true; for (let i = 0; i < 20; i++) W.step(1 / 60);
  const snap = RB.takeSnapshot(g), h0 = RB.stateHash(g), was = pts.map(P => est(P));
  for (let i = 0; i < 40; i++) W.step(1 / 60);
  RB.restoreSnapshot(snap); const h1 = RB.stateHash(g), now = pts.map(P => est(P)); let differ = 0; for (let i = 0; i < pts.length; i++) if (!Object.is(was[i], now[i])) differ++;
  S.SENSE.moon = false;
  verdict(h0 === h1 && differ === 0, 'put back: a game put back from a snapshot reads the moon as it did', `the state's word ${h0 === h1 ? 'the same' : 'ANOTHER'} · ${differ} of ${pts.length} points read differently`); }
log(bad ? `\n=== moonlight-check: ${bad} FAILED${BREAK ? ' (the red path: `shadow` must be one of them)' : ''} ===` : '\n=== moonlight-check: ALL PASS ===');
process.exit(bad ? 1 : 0);
