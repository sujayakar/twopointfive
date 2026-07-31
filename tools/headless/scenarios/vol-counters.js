// Track B2a (volumetrics) — work-counter suite at the pinned bench pose.
//
//   npm run build
//   python3 tools/headless/run.py --bench-res 320 200 \
//     --scenario tools/headless/scenarios/vol-counters.js --json counters.json
//
// __bench pins the pose itself; each block is the counters of the run's
// last measured frame. The rebake spike (lightVolBakeRays) is read from
// __stats after a single frame following a static-intensity change, because
// __bench's warm-up frames would clear it before the reported frame.
(async () => {
  const st = window.__settings, rd = window.__renderer;
  st.counters = true;
  const pick = (r) => {
    const j = JSON.parse(r);
    const c = j.counters || {};
    return { frames: j.frames, truncated: j.truncated, res: j.res, frame: c.frame, perPixel: c.perPixel };
  };
  const out = {};
  out.defaults = pick(await window.__bench(3, true));
  st.flashVisVolumetric = false;
  out.noFlashmap = pick(await window.__bench(3, true));
  st.flashVisVolumetric = true;
  st.volExtinction = false;
  out.noExtinction = pick(await window.__bench(3, true));
  st.volExtinction = true;
  st.reference = true;
  out.referenceMode = pick(await window.__bench(2, true));
  st.reference = false;

  // The whole-volume light rebake on an OCP-style darkening.
  window.__pinResolution(160, 100);
  await window.__renderStill(3);
  rd.setStaticLightIntensity(6, 0);
  const t0 = performance.now();
  await window.__renderStill(1);
  await new Promise((r) => setTimeout(r, 400));
  out.rebakeFrame = {
    wallMs: Math.round(performance.now() - t0),
    lightVolBakeRays: rd.workCounters.latest ? rd.workCounters.latest.totals.lightVolBakeRays : null,
  };
  rd.setStaticLightIntensity(6, 4.95);
  return JSON.stringify(out);
})()
