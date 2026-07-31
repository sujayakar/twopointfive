// Track B2b (fluid) — the torch beam into a canister cloud.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/fluid-beam.js \
//     --shot out.png --json out.json
//
// "The beam stops inside the cloud" is an image opinion unless the optical
// depth is the thing measured. Two measurements of the same beam:
//
//   solver   — F.columnDensity marches the simulation lattice trilinearly and
//              accumulates tau = sigma_t * integral(rho dl) with sigma_t =
//              settings.volumetric per unit density. This is the field the
//              volumetric march actually integrates, so it is the number the
//              rendered beam has to agree with.
//   gameplay — the same march through __sampleSmokeDensity, the quarter-
//              resolution field a few frames behind that B3's guard LOS will
//              use. Reported alongside because its box average smooths a
//              compact cloud hard, and anyone sizing an LOS threshold needs
//              to see by how much rather than assume the two agree.
//
// A thrown canister rolls where the physics sends it — 2.8 m off the beam axis
// on the first attempt at this scenario — so the emitter is pinned to a fixed
// point on the beam axis here, with the canister's own strength and lifetime.
// fluid-sequence.js covers the thrown-and-rolling case.
(async () => {
  const DT_MS = 50;
  const SETTLE_S = 3.0;
  const MARCH_M = 9.0, STEP_M = 0.1;
  const PIN_AT_M = 4.0;

  const R = window.__renderer, P = window.__player, st = window.__settings;
  const cam = window.__camera, inp = window.__input;
  window.__pause(true);
  window.__guards.frozen = true;
  R.resize(448, 280);
  const F = window.__fluid, SM = window.__smoke;
  F.reset();
  SM.reset(true);
  window.__canisters.reset();

  // The corridor pose vol-shot.js uses for its beam shots, torch in hand.
  const cv = document.querySelector("canvas");
  P.pos.x = -2; P.pos.z = -11; P.yaw = -0.6; P.velX = 0; P.velZ = 0;
  P.flashlightOn = true; P.carrying = false;
  window.__equipment.select(1);
  inp.mouseX = cv.width * 0.36; inp.mouseY = cv.height * 0.42;
  cam.yaw = Math.PI * 0.25; cam.pitch = Math.PI * 0.33; cam.distance = 26;
  for (let i = 0; i < 4; i++) {
    cam.update(0.5, { x: P.pos.x, y: P.pos.y + 1, z: P.pos.z }, cv.width / cv.height);
  }
  await window.__renderStill(4, DT_MS);   // let the aim rig settle

  const o = P.flashlightOrigin(), d = P.flashlightDir();
  const pin = {
    x: o.x + d.x * PIN_AT_M, y: o.y + d.y * PIN_AT_M, z: o.z + d.z * PIN_AT_M,
  };
  SM.silenced = false;
  SM.canisterCloud(() => pin, SETTLE_S + 2);
  await window.__renderStill(Math.round(SETTLE_S * 1000 / DT_MS), DT_MS);
  await window.__renderStill(6, DT_MS);   // the gameplay readback lags a few frames

  const sigma = st.volumetric;
  const from = [o.x, o.y, o.z], dir = [d.x, d.y, d.z];
  const solver = await F.columnDensity(from, dir, MARCH_M, STEP_M, sigma);

  let gTau = 0, gIntegral = 0, gPeak = 0, gHalf = null;
  const table = [];
  for (let q = 1; q <= Math.round(MARCH_M / STEP_M); q++) {
    const s = q * STEP_M;
    const rho = window.__sampleSmokeDensity(
      from[0] + dir[0] * s, from[1] + dir[1] * s, from[2] + dir[2] * s);
    if (rho > gPeak) gPeak = rho;
    gIntegral += rho * STEP_M;
    gTau += sigma * rho * STEP_M;
    if (gHalf === null && Math.exp(-gTau) <= 0.5) gHalf = +s.toFixed(2);
    if (Math.abs(s * 2 - Math.round(s * 2)) < 1e-6) {
      const sv = solver.samples[q - 1];
      table.push({
        s: +s.toFixed(2),
        solverRho: +sv.density.toFixed(4), solverT: +sv.transmittance.toFixed(5),
        gameplayRho: +rho.toFixed(4), gameplayT: +Math.exp(-gTau).toFixed(5),
      });
    }
  }

  const dens = await F.densityStats();
  const failures = [];
  // The claim under test: the beam is not merely dimmed but extinguished
  // inside the cloud, measured on the field the renderer marches.
  if (solver.halfAt === null) {
    failures.push(`solver transmittance never reached 0.5 over ${MARCH_M} m`);
  }
  if (gPeak <= 0) failures.push("gameplay density readback is empty on the beam axis");
  return {
    ok: failures.length === 0, failures,
    res: `${R.renderWidth}x${R.renderHeight}`,
    sigmaPerUnitDensity: sigma, volExtinction: st.volExtinction,
    beamOrigin: from.map((v) => +v.toFixed(2)), beamDir: dir.map((v) => +v.toFixed(3)),
    pinnedSourceAt: [+pin.x.toFixed(2), +pin.y.toFixed(2), +pin.z.toFixed(2)],
    solverBeam: {
      peakDensity: +solver.peak.toFixed(4), integral: +solver.integral.toFixed(4),
      tau: +solver.tau.toFixed(4),
      halfTransmittanceAt_m: solver.halfAt && +solver.halfAt.toFixed(2),
      tenPercentAt_m: solver.tenthAt && +solver.tenthAt.toFixed(2),
      transmittanceOut: +Math.exp(-solver.tau).toFixed(5),
    },
    gameplayBeam: {
      peakDensity: +gPeak.toFixed(4), integral: +gIntegral.toFixed(4),
      tau: +gTau.toFixed(4), halfTransmittanceAt_m: gHalf,
      transmittanceOut: +Math.exp(-gTau).toFixed(5),
      smoothingVsSolver: +(solver.peak / Math.max(gPeak, 1e-9)).toFixed(1),
    },
    march: table,
    cloud: {
      mass: +dens.mass.toFixed(4), peak: +dens.maxDensity.toFixed(4),
      visibleCells: dens.visibleCells,
      centroid: dens.centroid.map((v) => +v.toFixed(2)),
      bbox: dens.bbox && {
        min: dens.bbox.min.map((v) => +v.toFixed(2)),
        max: dens.bbox.max.map((v) => +v.toFixed(2)),
      },
    },
    sources: SM.count,
  };
})()
