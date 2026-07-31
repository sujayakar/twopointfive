// Track B2b (fluid) — the rendered still: a simulated cloud under a ceiling
// light, plus the numbers that back up whatever the image is said to show.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-still.js \
//     --shot fluid-still.png --json out.json
//
// MODE (edit below; a JS scenario cannot take arguments):
//   puff — __smokePuff's blob. It injects density and NO temperature, so the
//          solver's weight term is the only force on it: cold smoke, which
//          sinks and pools. This is the canister's regime.
//   warm — the same blob with temperature, i.e. what a muzzle burst or a
//          smouldering fixture puts in. Buoyancy dominates and it climbs.
// Two runs, two images, because "buoyant" and "pools on the floor" are
// different claims about different sources and one still cannot show both.
//
// The fixture is the conference room's warm fluorescent at (-20, 3.06, 1), the
// brightest practical in the level (power 3.4).
//
// Framing note worth keeping: `frameBody` calls `camera.update(dt, player)`
// every frame, so a scenario that aims the camera and then steps the world has
// its aim overwritten. __freezeClock is the way out — at dt = 0 the camera's
// target damps by nothing, so the aim survives, and the sim is frozen too (the
// renderer skips the fluid step at dt = 0), which is exactly a still: fixed
// field, fixed pose, only the path tracer's samples accumulating.
//
// Three captures, not two. The cloud's contribution is the difference between a
// frame with it and a frame without, but two accumulations of the same scene
// already differ by Monte Carlo noise, and at ten frames that noise is not
// small. So: two control captures to measure the noise floor, then the cloud,
// and every threshold below is expressed in units of that floor. Otherwise the
// "footprint" is the whole frame and the adjectives are measuring the sampler.
//   soft          — share of footprint pixels below a quarter of the peak
//                   change; a hard-edged blob would sit near its peak;
//   occluding     — negative-delta pixels: extinction against lit geometry;
//   in-scattering — positive-delta pixels: the fixture's light picked up by
//                   the medium where the background was dark;
//   self-shadowed — mean in-scatter in the footprint's top third against its
//                   bottom third. Lit from above and shadowing its own
//                   interior, the top must be the brighter end.
(async () => {
  const MODE = "puff";
  const RES = [384, 240];
  const DT_MS = 50;
  const SECONDS = 2.0;
  const FRAMES = 10;             // accumulation frames per capture, dt = 0
  const CLOUD = [-20, 1.45, 1.0];
  const FIXTURE = [-20, 3.06, 1.0];

  const R = window.__renderer, P = window.__player, cam = window.__camera;
  const F = window.__fluid, SM = window.__smoke;
  const cv = document.querySelector("canvas");
  const STEPS = Math.round(SECONDS * 1000 / DT_MS);

  window.__pause(true);
  window.__guards.frozen = true;
  const pinned = window.__pinResolution(RES[0], RES[1]);

  // Parked by the doorway, out of the sight line: the cloud is the subject, and
  // a character between the lens and it would be the thing being described.
  P.pos.x = -15.6; P.pos.z = 5.4; P.yaw = 0.6;
  P.velX = 0; P.velZ = 0; P.flashlightOn = false; P.carrying = false;
  window.__equipment.select(0);
  // The house three-quarter view, far enough back that a 2 m cloud reads as a
  // cloud in the room rather than as a veil over the lens.
  cam.yaw = Math.PI * 0.25; cam.pitch = Math.PI * 0.28; cam.distance = 14;
  const aimAtCloud = () => {
    for (let i = 0; i < 8; i++) {
      cam.update(0.5, { x: CLOUD[0], y: CLOUD[1], z: CLOUD[2] }, cv.width / cv.height);
    }
  };

  const luma = (a, o) => 0.2126 * a[o] + 0.7152 * a[o + 1] + 0.0722 * a[o + 2];

  const capture = async (withCloud) => {
    window.__freezeClock(false);
    F.reset();
    SM.reset(true);
    window.__canisters.reset();
    SM.silenced = false;
    if (withCloud) {
      if (MODE === "warm") {
        // Same one-frame delivery as puff(), with heat: buoyancy's currency.
        SM.spawn({
          pos: { x: CLOUD[0], y: CLOUD[1], z: CLOUD[2] }, radius: 0.9,
          density: 0, temp: 260, life: 0.02, instantAmount: 40,
        });
      } else {
        window.__smokePuff(CLOUD[0], CLOUD[1], CLOUD[2], 0.9, 40);
      }
    }
    await window.__renderStill(1, DT_MS);
    SM.silenced = true;                       // no inflow after the injection
    await window.__renderStill(STEPS - 1, DT_MS);
    // Fresh accumulator history, so the previous capture cannot bleed into this
    // one through reprojection. Re-pinning is how a pinned scenario asks for the
    // reallocation __bench gets from its resize pair.
    window.__pinResolution(RES[0], RES[1] + 1);
    window.__pinResolution(RES[0], RES[1]);
    aimAtCloud();
    window.__freezeClock(true);
    await window.__renderStill(FRAMES);
    const hdr = await R.readHDR();
    return { hdr, dens: await F.densityStats() };
  };

  const ctrlA = await capture(false);
  const ctrlB = await capture(false);         // identical protocol: the noise floor
  const cloud = await capture(true);          // last, so --shot catches it

  const w = cloud.hdr.width, h = cloud.hdr.height, n = w * h;
  let noiseSq = 0;
  for (let i = 0; i < n; i++) {
    const e = luma(ctrlA.hdr.data, i * 4) - luma(ctrlB.hdr.data, i * 4);
    noiseSq += e * e;
  }
  const noiseRms = Math.sqrt(noiseSq / n);

  const d = new Float32Array(n);
  let dMax = 0;
  for (let i = 0; i < n; i++) {
    d[i] = luma(cloud.hdr.data, i * 4) - luma(ctrlA.hdr.data, i * 4);
    if (Math.abs(d[i]) > dMax) dMax = Math.abs(d[i]);
  }
  // Four times the measured noise, or 2% of the peak change, whichever is
  // larger: the first keeps the sampler out of the footprint, the second keeps
  // the threshold from vanishing if the render ever gets quiet.
  const thr = Math.max(4 * noiseRms, 0.02 * dMax);

  let fp = 0, ring = 0, pos = 0, neg = 0, posSum = 0, negSum = 0;
  let minRow = h, maxRow = -1, sumRow = 0;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const v = d[y * w + x];
      if (Math.abs(v) <= thr) continue;
      fp++; sumRow += y;
      if (Math.abs(v) < dMax * 0.25) ring++;
      if (v > 0) { pos++; posSum += v; } else { neg++; negSum += v; }
      if (y < minRow) minRow = y;
      if (y > maxRow) maxRow = y;
    }
  }
  // Self-shadowing is read off the in-scattering pixels only: where the cloud
  // covers lit geometry its delta is dominated by extinction, which says
  // nothing about how the medium is lit inside.
  const third = fp > 0 ? (maxRow - minRow) / 3 : 0;
  let top = 0, bot = 0, topSum = 0, botSum = 0;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const v = d[y * w + x];
      if (v <= thr) continue;
      if (y <= minRow + third) { top++; topSum += v; }
      else if (y >= maxRow - third) { bot++; botSum += v; }
    }
  }
  const topMean = top > 0 ? topSum / top : 0;
  const botMean = bot > 0 ? botSum / bot : 0;

  let lit = 0, lmax = 0, lsum = 0, finite = true;
  for (let i = 0; i < n; i++) {
    const v = luma(cloud.hdr.data, i * 4);
    if (!Number.isFinite(v)) { finite = false; break; }
    lsum += v;
    if (v > lmax) lmax = v;
    if (v > 0.002) lit++;
  }

  const failures = [];
  if (`${R.renderWidth}x${R.renderHeight}` !== `${RES[0]}x${RES[1]}`) {
    failures.push(`render size drifted to ${R.renderWidth}x${R.renderHeight}`);
  }
  if (!finite) failures.push("non-finite pixel in the delivered frame");
  if (finite && lit < n * 0.15) failures.push(`only ${lit}/${n} pixels above 0.002 luma`);
  if (!(cloud.dens.mass > 0)) failures.push("no smoke in the field at capture");
  if (ctrlA.dens.mass !== 0) failures.push(`control frame carried mass ${ctrlA.dens.mass}`);
  if (!(dMax > 8 * noiseRms)) {
    failures.push(`peak change ${dMax} is not clear of the noise floor ${noiseRms}`);
  }
  if (!(fp > n * 0.01)) failures.push(`cloud changed only ${fp}/${n} pixels`);
  // Both signs must be present, and this is the substantive assert: a medium
  // that only brightens is not extinguishing anything, and one that only
  // darkens is not in-scattering. B2a's channel does both or the cloud is not
  // really in the light transport.
  if (!(pos > fp * 0.05)) failures.push(`only ${pos}/${fp} in-scattering pixels`);
  if (!(neg > fp * 0.05)) failures.push(`only ${neg}/${fp} occluding pixels`);
  // Buoyancy is a claim about the mode, so assert it per mode rather than
  // describing whichever way the cloud happened to go.
  const rose = cloud.dens.centroid[1] > CLOUD[1];
  if (MODE === "warm" && !rose) {
    failures.push(`warm cloud did not rise: centroid y ${cloud.dens.centroid[1]}`);
  }
  if (MODE === "puff" && rose) {
    failures.push(`cold cloud rose: centroid y ${cloud.dens.centroid[1]}`);
  }

  return {
    ok: failures.length === 0, failures, mode: MODE,
    res: `${R.renderWidth}x${R.renderHeight}`, pinned,
    cloud: CLOUD, fixture: FIXTURE, seconds: SECONDS,
    camera: {
      yaw: +cam.yaw.toFixed(4), pitch: +cam.pitch.toFixed(4), distance: cam.distance,
      pos: [+cam.pos.x.toFixed(2), +cam.pos.y.toFixed(2), +cam.pos.z.toFixed(2)],
      target: [+cam.target.x.toFixed(2), +cam.target.y.toFixed(2), +cam.target.z.toFixed(2)],
    },
    sim: {
      steps: F.steps, jacobi: F.lastJacobi, tune: { ...F.tune },
      mass: +cloud.dens.mass.toFixed(4), peak: +cloud.dens.maxDensity.toFixed(4),
      visibleCells: cloud.dens.visibleCells, rowCells: cloud.dens.rowCells,
      injectedAtY: CLOUD[1], centroid: cloud.dens.centroid.map((v) => +v.toFixed(3)),
      roseFromInjection: rose,
      bbox: cloud.dens.bbox && {
        min: cloud.dens.bbox.min.map((v) => +v.toFixed(2)),
        max: cloud.dens.bbox.max.map((v) => +v.toFixed(2)),
        size: cloud.dens.bbox.max.map((v, i) => +(v - cloud.dens.bbox.min[i]).toFixed(2)),
      },
      checksum: cloud.dens.checksum,
    },
    frame: {
      pixels: n, litFraction: +(lit / n).toFixed(4),
      meanLuma: +(lsum / n).toFixed(5), maxLuma: +lmax.toFixed(4),
    },
    contribution: {
      noiseRms: +noiseRms.toFixed(6), threshold: +thr.toFixed(6),
      peakAbsDeltaLuma: +dMax.toFixed(4),
      peakOverNoise: +(dMax / Math.max(noiseRms, 1e-9)).toFixed(1),
      footprintPixels: fp, footprintFraction: +(fp / n).toFixed(4),
      softEdgeFraction: +(ring / Math.max(fp, 1)).toFixed(3),
      inScatterPixels: pos, inScatterMeanDelta: +(posSum / Math.max(pos, 1)).toFixed(5),
      occludePixels: neg, occludeMeanDelta: +(negSum / Math.max(neg, 1)).toFixed(5),
      netDelta: +(posSum + negSum).toFixed(4),
      rows: { min: minRow, mid: +(sumRow / Math.max(fp, 1)).toFixed(1), max: maxRow },
      topThirdMeanInScatter: +topMean.toFixed(5),
      bottomThirdMeanInScatter: +botMean.toFixed(5),
      topOverBottom: botMean !== 0 ? +(topMean / botMean).toFixed(3) : null,
    },
  };
})()
