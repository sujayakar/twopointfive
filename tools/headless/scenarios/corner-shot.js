// Body-shadows-bounce-light pose, leaving the page in a chosen indirect mode
// with the "indirect only" debug view. Set window.__cornerMode before
// evaluating (e.g. via a tiny preamble scenario), default "gather".
//
// Player 1.2 m south of the conference room's north wall (z = -4.5, warm
// paint), facing south into the room: the beam pools on the red carpet and
// the pool's bounce lights the wall behind the character. gather traces
// bounce 1 for real, so the character occludes that bounce onto the wall;
// radiosityRead reads a bake that never saw the character.
(async () => {
  const st = window.__settings;
  const p = window.__player;
  const inp = window.__input;
  const eq = window.__equipment;
  const cam = window.__camera;
  const cv = document.querySelector("canvas");
  const mode = window.__cornerMode || "gather";
  const view = window.__cornerView === undefined ? 5 : window.__cornerView;
  const exposure = window.__cornerExposure === undefined ? 6.0 : window.__cornerExposure;

  st.resolutionScale = 0.45;
  window.__resize();
  for (const g of window.__guards.all) { g.alertLeft = 1e9; g["alertYaw"] = g.yaw; g.light.intensity = 0; }
  // Flashlight-only lighting: every practical and the moon go dark, so the
  // wall behind the character is lit by nothing but the beam's bounce.
  const nLights = window.__scene.lights.length;
  for (let i = 0; i < nLights; i++) window.__renderer.setStaticLightIntensity(i, 0);
  st.skyIntensity = 0;
  st.ambient = 0;

  const project = (x, y, z, w, h) => {
    const m = cam.viewProj;
    const cx = m[0] * x + m[4] * y + m[8] * z + m[12];
    const cy = m[1] * x + m[5] * y + m[9] * z + m[13];
    const cw = m[3] * x + m[7] * y + m[11] * z + m[15];
    return [(cx / cw * 0.5 + 0.5) * w, (0.5 - cy / cw * 0.5) * h];
  };

  st.indirectMode = mode;
  st.debugView = view;
  st.exposure = exposure;
  cam.distance = 9;
  for (let k = 0; k < 8; k++) {
    p.pos.x = -19; p.pos.z = -3.3;
    p.velX = 0; p.velZ = 0;
    p.flashlightOn = true; p.carrying = false; p.crouching = false;
    eq.select(1);
    const [mx, my] = project(-19.5, 0, 0.8, cv.width, cv.height);
    inp.mouseX = mx; inp.mouseY = my;
    await window.__renderStill(1);
  }
  window.__freezeClock(true);
  await window.__renderStill(25);
  return { mode, view, exposure, aim: [inp.mouseX, inp.mouseY] };
})()
