// Track B2a (volumetrics) — reference comparison of the volumetric channel.
//
//   npm run build
//   python3 tools/headless/run.py --bench-res 320 200 --bench-cap-s 2600 \
//     --scenario tools/headless/scenarios/vol-compare.js --json compare.json
//
// The reference renders the same volumetric MODEL by Monte Carlo (static
// lights with real shadow rays per step, real rays for the torch) and
// accumulates it as a 1/n average; the test configs render the shipped
// estimator (baked light volume + depth map + clamped short history).
// noVolume is a control: how far the image is from a reference that has
// the medium when the medium is removed.
(async () => {
  return await window.__compareToReference(
    { defaults: {}, extinctionOff: { volExtinction: false }, noVolume: { volumetric: 0 } },
    140, 40, {},
  );
})()
