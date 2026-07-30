// Track B2a (volumetrics) — reference comparison of the volumetric channel.
//
//   npm run build
//   python3 tools/headless/run.py --bench-res 320 200 --bench-cap-s 2600 \
//     --scenario tools/headless/scenarios/vol-compare.js --json compare.json
//
// The reference renders the same volumetric MODEL by Monte Carlo (static
// lights with real static-scene shadow rays per step, real rays for the
// torch) and accumulates it as a 1/n average; the test configs render the
// shipped estimator (baked light volume + depth map + clamped short history).
//
// MODE selects which table:
//   "whole"     — full beauty; dominated by surface transport noise, the
//                 medium's signal is in the relBias deltas between configs.
//                 noVolume is a control: how far the image is from a
//                 reference that has the medium when the medium is removed.
//   "inscatter" — both sides at debug view 8 (volume in-scatter only): the
//                 surface drops out and the error is the in-scatter
//                 estimator's own. The extinction-off row is a scale bar,
//                 not a config.
const MODE = "whole";
(async () => {
  if (MODE === "inscatter") {
    return await window.__compareToReference(
      {
        inscatter: { debugView: 8 },
        inscatterExtinctionOff: { debugView: 8, volExtinction: false },
      },
      140, 40, { debugView: 8 },
    );
  }
  return await window.__compareToReference(
    { defaults: {}, extinctionOff: { volExtinction: false }, noVolume: { volumetric: 0 } },
    140, 40, {},
  );
})()
