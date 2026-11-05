// PLAIN AIR (gpu/device.ts PLAIN_AIR; render/passes.ts traceJobs hands this text to the volumetric pass in place of volumetrics.wgsl): the pass's program on a load whose
// graphics driver stops while it compiles the real one. Same bindings, same two outputs (rgb = in-scattered radiance, a = transmittance toward the surface; react.g =
// the smoke's own transmittance), and NOT ONE RAY: no shadow ray, no walled ambient, no self-shadow march, no beam, no rain streak, no splash. What is left is what the
// game is played by:
//   smoke        every live domain's density marched along the pixel's ray (one march across the hull of them, the K = 0 march's stretch and steps), lit by the radiance
//                cache's mean radiance at the step and nothing else: a plume stands where it stands and hides what it hides, flat where the real march shapes it.
//                The density is taken as it is (smokeDensityAt): the real march's fogSmoke() asks the fog of war's sight, which walks the boxes — so here smoke is
//                drawn under the fog whatever FOG.air says, which is how the page ships
//   the weather  a picked weather's air (common.wgsl wxAir, the field the men's eyes are cut by) marched the way volumetrics.wgsl's wxAirMarch steps it, lit by the cache
//                and the open sky's share: the veil over the far wall, without the sun's shafts in it
// The haze's lamp cones, the rain and the lightning's lit sky are simply not drawn here. Prelude: scene + lighting_common + vol_common, as the real march's.
@group(1) @binding(0) var gDepth: texture_depth_2d;
@group(1) @binding(1) var diceTex: texture_3d<f32>;
@group(1) @binding(2) var outVol: texture_storage_2d<rgba16float, write>;
@group(1) @binding(3) var outReact: texture_storage_2d<rgba16float, write>;

const PLAIN_WX_STEP: f32 = 0.75;   // (volumetrics.wgsl WX_STEP and wxMaxSteps: the same veil at the same steps)
fn plainWxSteps() -> u32 { let n = (frame.volCfg.w >> 24u) & 31u; return select(max(n, 3u), max(frame.volSteps / 2u, 4u), n == 0u); }

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) gid: vec3u) {
  let odims = textureDimensions(outVol);
  if (gid.x >= odims.x || gid.y >= odims.y) { return; }
  var wxAirOn = false; if (WX_VOL) { if (WX_KINDS) { wxAirOn = frame.cardCfg.w != 0u; } }
  let smokeOn = (frame.flags & FLAG_SMOKE) != 0u && frame.numSmoke > 0u;
  if (!smokeOn && !wxAirOn) { textureStore(outVol, gid.xy, vec4f(0.0, 0.0, 0.0, 1.0)); textureStore(outReact, gid.xy, vec4f(0.0, 1.0, 0.0, 0.0)); return; }
  let vdiv = u32(frame.lossyCfg.y);
  let full = min(gid.xy * vdiv + vec2u(gid.y & 1u, gid.x & 1u) * (vdiv / 2u), vec2u(frame.screen) - 1u);
  let depth = textureLoad(gDepth, full, 0);
  let pix = vec2f(full) + 0.5;
  let Pfar = worldFromDepth(pix, select(depth, 1.0, depth >= 1.0));
  let ro = frame.camPos;
  var rd = Pfar - ro; var tSurf = length(rd); rd /= tSurf;
  if (depth >= 1.0) { tSurf = min(tSurf, 200.0); }
  let jitter = fract(ign(vec2f(gid.xy)) + f32(frame.frameIdx & 63u) * 0.61803399);   // (the smoke march's rotated dither: the resolve averages it)
  let invRd = safeInv(rd);
  var inscatter = vec3f(0.0);
  var T = 1.0;

  if (smokeOn) {
    var t0 = 1e30; var t1 = -1e30; var vox = 1.0;
    for (var i = 0u; i < frame.numSmoke; i++) {
      let sd = smoke[i];
      if (sd.live == 0u) { continue; }
      let tb = isectBounds(ro, invRd, sd.origin, sd.origin + sd.voxel * vec3f(sd.dims));
      if (tb.x <= tb.y && tb.y > 0.0 && tb.x < tSurf) { t0 = min(t0, max(tb.x, 0.0)); t1 = max(t1, min(tb.y, tSurf)); vox = min(vox, sd.voxel); }
    }
    if (t1 > t0) {
      let span = t1 - t0;
      let steps = clamp(u32(span / (vox * 1.5)) + 1u, 4u, 112u);
      let dt = span / f32(steps);
      var t = t0 + dt * jitter;
      for (var s = 0u; s < steps; s++) {
        let X = ro + rd * t;
        let dens = smokeDensityAt(X);
        if (dens > 0.002) {
          let sigT = dens * 6.0;   // (the march's own extinction and albedo)
          let stepT = exp(-sigT * dt);
          inscatter += T * diceMeanRadiance(X) * 0.92 * (1.0 - stepT);
          T *= stepT;
          if (T < 0.01) { break; }
        }
        t += dt;
      }
    }
  }
  let Tsmoke = T;

  var Tair = 1.0;
  if (WX_VOL) { if (WX_KINDS) { if (frame.cardCfg.w != 0u) {
    let top = CEIL_Y + WX_TOP;
    var ta = 0.0; var tb = tSurf; var any = true;
    if (abs(rd.y) > 1e-5) { let a0 = (top - ro.y) / rd.y; let a1 = -ro.y / rd.y; ta = max(ta, min(a0, a1)); tb = min(tb, max(a0, a1)); }
    else if (ro.y > top || ro.y < 0.0) { any = false; }
    if (any && tb > ta) {
      let alb = wxFog().tint;
      let n = clamp(u32(ceil((tb - ta) / PLAIN_WX_STEP)), 3u, plainWxSteps());
      let dt = (tb - ta) / f32(n);
      var acc = vec3f(0.0);
      for (var s = 0u; s < n; s++) {
        let X = ro + rd * (ta + (f32(s) + jitter) * dt);
        let air = wxAir(X);
        let sg = air.x * wxBank(X.xz);
        if (sg > 1e-5) {
          let stepT = exp(-sg * dt);
          var Lw = diceMeanRadiance(X); if (air.y <= 0.5) { Lw += frame.weatherB.xyz * 0.5; }
          acc += Tair * alb * Lw * (1.0 - stepT);
          Tair *= stepT;
        }
      }
      inscatter = inscatter * mix(1.0, Tair, 0.5) + acc * (0.5 + 0.5 * T);   // (the smoke's light through about half of the air, the air's through about half of the smoke: the real march's statement)
    }
  } } }

  textureStore(outVol, gid.xy, vec4f(inscatter, T * Tair));
  textureStore(outReact, gid.xy, vec4f(0.0, Tsmoke, 0.0, 0.0));
}
