// /demo/dynamics — repeated detonations, which is where the runaway was seen.
//
//   npm run build
//   python3 tools/headless/run.py --page /demo/dynamics.html \
//     --scenario tools/headless/scenarios/dynamics-stack.js --json out.json \
//     --arg '{"fires":6,"interval":0.6}'
//
// dynamics-mass.js silences every emitter to isolate solver error. This does
// the opposite and reproduces the report: bursts stacked on top of each other
// while the previous one is still moving. Mass is SUPPOSED to rise here — six
// bursts inject six bursts' worth. What must not happen is the peak density
// climbing without bound, which is what saturated fp16 and turned the field
// white; a healthy field's peak settles near what one burst produces however
// many are stacked, because they spread into each other rather than multiply.
//
// The pass condition is on `peak`, not on mass, for that reason. FP16_MAX is
// the tell: 65504 in the density texture is not a large number, it is the
// largest representable one, and one step later it is inf and then NaN.
(async () => {
  const A = window.__scenarioArg || {};
  const D = window.__dyn;
  if (!D) return { ok: false, failures: ["__dyn missing — wrong page?"] };

  const FIRES = A.fires ?? 6;
  const INTERVAL = A.interval ?? 0.6;
  const TAIL = A.tail ?? 4.0;
  const DT_MS = 50;
  const F = D.fluid;
  const failures = [];

  D.resize(160, 100);
  D.pause(true);
  D.setKind(1);
  D.params.dbg.openFaces = A.openFaces ?? 15;
  if (A.tune) Object.assign(D.params.solver, A.tune);

  const snap = async (t, fired) => {
    const s = await F.densityStats();
    return {
      t: +t.toFixed(2), fired,
      mass: +s.mass.toFixed(3),
      peak: +s.maxDensity.toFixed(1),
      cells: s.nonzeroCells,
      shellPeak: s.shellPeak.map((v) => +v.toFixed(1)),
    };
  };

  D.reset();
  const series = [];
  let t = 0;
  const stepTo = async (target) => {
    const need = Math.round((target - t) / (DT_MS / 1000));
    if (need > 0) { await D.step(need, DT_MS); t += need * DT_MS / 1000; }
  };

  for (let i = 0; i < FIRES; i++) {
    // Reroll the can each time, or every burst is the same burst and the
    // stack is one detonation rendered six times.
    D.params.burst.seed = 3 + i * 7;
    D.fire();
    await stepTo(t + INTERVAL);
    series.push(await snap(t, i + 1));
  }
  const endOfFiring = series[series.length - 1];
  for (let q = 1; q <= 4; q++) {
    await stepTo(t + TAIL / 4);
    series.push(await snap(t, FIRES));
  }

  const FP16_MAX = 65504;
  const worst = series.reduce((a, b) => (b.peak > a.peak ? b : a), series[0]);
  const bad = series.find((s) => !Number.isFinite(s.mass) || s.peak >= FP16_MAX);
  if (bad) failures.push(`density saturated fp16 at t=${bad.t} (peak ${bad.peak})`);
  const wiped = series.find((s, i) => i > 0 && s.cells === 0);
  if (wiped) failures.push(`field wiped to zero cells at t=${wiped.t}`);

  return {
    ok: failures.length === 0,
    failures,
    fires: FIRES, interval: INTERVAL,
    series,
    peakOfPeaks: worst.peak,
    peakAtLastShot: endOfFiring.peak,
    // What the stack cost relative to one burst's own peak.
    peakVsFirstShot: series[0].peak > 0
      ? +(worst.peak / series[0].peak).toFixed(2) : null,
  };
})()
