// Crouch bug matrix: does the flashlight still reach the radiosity solve (and
// the volumetric march) when the character is crouched?
//
// Pins the player at the corridor spawn, freezes the guards where they stand
// and puts their torches out, so the only moving light is the player's
// flashlight. The crouch idle clip sways the gun arm across the lens, so the
// depth-map COVERAGE is sampled across the clip (min/median/max); the light
// deltas are then taken with the game clock frozen (__freezeClock) so
// flash-on/flash-off and volumetric on/off see the exact same pose. Works
// before and after the indirectMode split: drives settings.indirectMode when
// present, else settings.radiosity.
//
// Numbers reported:
//   coverageLt10cm — fraction of flashlight depth-map layer 0 nearer than
//                    10 cm (the character's own gun/body), stand + crouch.
//   flashInject    — sum over patches of luminance(injected E), flashlight on
//                    minus off: the light the flashlight feeds the solve.
//   shaft          — crouched, volumetric 1.0, window at the player: depth-map
//                    march minus vol 0, and real-ray march minus vol 0.
(async () => {
  const st = window.__settings;
  const p = window.__player;
  const r = window.__renderer;
  const inp = window.__input;
  const eq = window.__equipment;
  const cam = window.__camera;
  const cv = document.querySelector("canvas");

  st.resolutionScale = 0.25;
  window.__resize();

  for (const g of window.__guards.all) {
    g.alertLeft = 1e9;
    g["alertYaw"] = g.yaw;
    g.light.intensity = 0;
  }

  const pin = () => {
    p.pos.x = -13; p.pos.z = 0.5;
    p.velX = 0; p.velZ = 0;
    p.carrying = false;
    eq.select(1);
    // Aim east: the campaign's crouch pose (hanging slide over the pinhole).
    inp.mouseX = cv.width * 0.88;
    inp.mouseY = cv.height * 0.50;
    cam.distance = 20;
  };

  const setSource = (patches) => {
    if ("indirectMode" in st) st.indirectMode = patches ? "radiosityRead" : "traced";
    else st.radiosity = patches;
  };

  const coverage = async () => {
    const d = await window.__readFlashmap(0);
    let n = 0;
    for (let i = 0; i < d.length; i++) if (d[i] < 0.1) n++;
    return n / d.length;
  };
  const injectSum = async () => {
    const rad = await window.__readRadiosity();
    let s = 0;
    for (let i = 0; i < rad.count; i++) {
      const o = i * 4;
      s += 0.2126 * rad.data[o] + 0.7152 * rad.data[o + 1] + 0.0722 * rad.data[o + 2];
    }
    return s;
  };
  const spread = (xs) => {
    const s = [...xs].sort((a, b) => a - b);
    return {
      min: +s[0].toFixed(3), median: +s[Math.floor(s.length / 2)].toFixed(3),
      max: +s[s.length - 1].toFixed(3),
    };
  };
  const project = (x, y, z, w, h) => {
    const m = cam.viewProj;
    const cx = m[0] * x + m[4] * y + m[8] * z + m[12];
    const cy = m[1] * x + m[5] * y + m[9] * z + m[13];
    const cw = m[3] * x + m[7] * y + m[11] * z + m[15];
    return [(cx / cw * 0.5 + 0.5) * w, (0.5 - cy / cw * 0.5) * h];
  };
  const meanLumNearPlayer = (hdr) => {
    const [px, py] = project(p.pos.x, 0.8, p.pos.z, hdr.width, hdr.height);
    const pad = 26;
    const x0 = Math.max(0, Math.floor(px - pad)), x1 = Math.min(hdr.width, Math.ceil(px + pad));
    const y0 = Math.max(0, Math.floor(py - pad)), y1 = Math.min(hdr.height, Math.ceil(py + pad));
    let s = 0, n = 0;
    for (let y = y0; y < y1; y++) {
      for (let x = x0; x < x1; x++) {
        const i = (y * hdr.width + x) * 4;
        const l = 0.2126 * hdr.data[i] + 0.7152 * hdr.data[i + 1] + 0.0722 * hdr.data[i + 2];
        if (!Number.isFinite(l)) continue;
        s += l; n++;
      }
    }
    return s / Math.max(n, 1);
  };

  const out = { res: `${r.renderWidth}x${r.renderHeight}` };
  setSource(true);
  st.debugView = 0;

  // ---- coverage, sampled across the clip -------------------------------------
  {
    const cov = [];
    for (let k = 0; k < 3; k++) {
      pin(); p.crouching = false; p.flashlightOn = true;
      await window.__renderStill(5);
      cov.push(await coverage());
    }
    out.coverageStandLt10cm = spread(cov);
  }
  {
    const cov = [];
    pin(); p.crouching = true; p.flashlightOn = true;
    await window.__renderStill(10); // settle the crouch transition
    for (let k = 0; k < 6; k++) {
      pin(); p.crouching = true; p.flashlightOn = true;
      await window.__renderStill(5);
      cov.push(await coverage());
    }
    out.coverageCrouchLt10cm = spread(cov);
  }

  // ---- exact-pose deltas: clock frozen at a settled crouch ------------------
  pin(); p.crouching = true; p.flashlightOn = true;
  await window.__renderStill(6);
  window.__freezeClock(true);
  out.frozenCoverageCrouchLt10cm = +(await (async () => { await window.__renderStill(3); return coverage(); })()).toFixed(3);

  const inj = async (flashOn) => {
    p.flashlightOn = flashOn;
    await window.__renderStill(6);
    return injectSum();
  };
  out.crouch = { flashInject: +((await inj(true)) - (await inj(false))).toFixed(3) };

  {
    const baseVol = st.volumetric;
    const shot = async () => { await window.__renderStill(8); return meanLumNearPlayer(await r.readHDR()); };
    p.flashlightOn = true;
    st.volumetric = 0;
    const volZero = await shot();
    st.volumetric = 1.0;
    st.flashVisVolumetric = true;
    const shaftMap = await shot();
    st.flashVisVolumetric = false;
    const shaftRays = await shot();
    st.flashVisVolumetric = true;
    st.volumetric = baseVol;
    out.crouch.shaft = {
      volZero: +volZero.toFixed(6),
      shaftDeltaMap: +(shaftMap - volZero).toFixed(6),
      shaftDeltaRays: +(shaftRays - volZero).toFixed(6),
    };
  }
  window.__freezeClock(false);

  // ---- standing, exact-pose flashlight injection --------------------------------
  pin(); p.crouching = false; p.flashlightOn = true;
  await window.__renderStill(10);
  window.__freezeClock(true);
  out.stand = { flashInject: +((await inj(true)) - (await inj(false))).toFixed(3) };
  window.__freezeClock(false);

  // Leave the page crouched with the beam on for a following screenshot.
  st.debugView = 0;
  pin();
  p.crouching = true;
  p.flashlightOn = true;
  await window.__renderStill(4);
  return out;
})()
