// Track B2a (volumetrics) — screenshot scenarios for tools/headless/run.py.
//
//   npm run build
//   python3 tools/headless/run.py --scenario tools/headless/scenarios/vol-shot.js --shot out.png
//
// One shot per run (the harness screenshots the page once). Pick the shot
// with WHICH below; the camera is settled the way __bench's pinBenchPose
// settles it, so shots from separate runs share their framing.
//
//   fogon    pinned bench pose, defaults (fog on)          -> moon pools + torch beam in the haze
//   fogoff   same, fogAmount = 0                            -> control: no ambient medium
//   fluoro   smoke test blob under the warm conference-room fluorescent
//   moon     smoke test blob down in a moonlight shaft (blue, lit from below)
//   corner   dense blob in dark air with a moonlit pool behind it (occludes)
//   exton    dense blob in the flashlight beam, extinction on
//   extoff   same, extinction off (the beam sails through)
//
// The test blob is __smokePuff(x, y, z, radius, peakDensity): an instant
// source the fluid simulation injects on the next step and then carries
// (advects, curls, dissipates). It replaced __smokeTest, whose y was
// clamp(radius, 0.3, 1.6) — that y is spelled out below so the poses match.
// The world is paused and stepped with a fixed 50 ms dt so the injection
// lands on a known frame regardless of SwiftShader's wall clock.
(async () => {
  const WHICH = "fogon";

  const st = window.__settings, rd = window.__renderer;
  window.__pause(true);
  const settle = () => {
    const p = window.__player, cam = window.__camera, cv = document.querySelector("canvas");
    for (let i = 0; i < 4; i++) {
      cam.update(0.5, { x: p.pos.x, y: p.pos.y + 1, z: p.pos.z }, cv.width / cv.height);
    }
  };
  const pose = (px, pz, slot, mx, my, distance) => {
    const p = window.__player, eq = window.__equipment, inp = window.__input, cam = window.__camera;
    const cv = document.querySelector("canvas");
    p.pos.x = px; p.pos.z = pz; p.yaw = -0.6; p.velX = 0; p.velZ = 0;
    p.flashlightOn = slot === 1; p.carrying = false; eq.select(slot);
    inp.mouseX = cv.width * mx; inp.mouseY = cv.height * my;
    cam.yaw = Math.PI * 0.25; cam.pitch = Math.PI * 0.33; cam.distance = distance;
    settle();
  };

  switch (WHICH) {
    case "fogon":
      pose(-2, -11, 1, 0.36, 0.42, 26);
      break;
    case "fogoff":
      pose(-2, -11, 1, 0.36, 0.42, 26);
      st.fogAmount = 0;
      break;
    case "fluoro":
      pose(-19, 4.5, 0, 0.5, 0.5, 14);
      window.__smokePuff(-21.5, 1.2, 3.2, 1.2, 25);
      break;
    case "moon":
      pose(1.5, -13.5, 0, 0.5, 0.5, 14);
      window.__smokePuff(-1.2, 1.0, -16.5, 1.0, 15);
      break;
    case "corner":
      pose(1.5, -13.5, 0, 0.5, 0.5, 14);
      window.__smokePuff(-1.1, 1.0, -14.5, 1.0, 60);
      break;
    case "exton":
    case "extoff": {
      pose(-2, -11, 1, 0.36, 0.42, 26);
      await window.__renderStill(4, 50);   // let the aim rig settle before reading the beam
      const p = window.__player, o = p.flashlightOrigin(), d = p.flashlightDir();
      window.__smokePuff(o.x + d.x * 4, 1.3, o.z + d.z * 4, 1.3, 30);
      st.volExtinction = WHICH === "exton";
      break;
    }
    default:
      return `unknown WHICH: ${WHICH}`;
  }

  rd.resize(448, 280);
  await window.__renderStill(14, 50);
  return `shot: ${WHICH}`;
})()
