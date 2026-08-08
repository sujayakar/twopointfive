// Shared by rc / gather / volumetric shaders. Requires the includer to declare:
//   var diceTex: texture_3d<f32>;   (irradiance dice volume: 7 slabs along Y: +x -x +y -y +z -z mean)

fn c0Spacing() -> vec3f { return RC_SIZE / vec3f(frame.rcC0Dims); }

fn diceCoord(P: vec3f, slab: f32) -> vec3f {
  let dims = vec3f(frame.rcC0Dims);
  var g = (P - RC_MIN) / RC_SIZE;
  g = clamp(g, vec3f(0.5) / dims, vec3f(1.0) - vec3f(0.5) / dims);
  return vec3f(g.x, (slab + g.y) / 7.0, g.z);
}

// Irradiance at P for normal N from the dice volume (ambient-cube weighting by N^2).
fn diceIrradiance(P: vec3f, N: vec3f) -> vec3f {
  let n2 = N * N;
  var E = vec3f(0.0);
  let fx = select(1.0, 0.0, N.x >= 0.0); let fy = select(3.0, 2.0, N.y >= 0.0); let fz = select(5.0, 4.0, N.z >= 0.0);
  if (n2.x > 0.001) { E += n2.x * textureSampleLevel(diceTex, linSamp, diceCoord(P, fx), 0.0).xyz; }
  if (n2.y > 0.001) { E += n2.y * textureSampleLevel(diceTex, linSamp, diceCoord(P, fy), 0.0).xyz; }
  if (n2.z > 0.001) { E += n2.z * textureSampleLevel(diceTex, linSamp, diceCoord(P, fz), 0.0).xyz; }
  return E;
}
fn diceMeanRadiance(P: vec3f) -> vec3f {
  return textureSampleLevel(diceTex, linSamp, diceCoord(P, 6.0), 0.0).xyz;
}

// Outgoing radiance of a diffuse box surface hit at P: emission + albedo/pi * (shadowed direct + last-frame bounce).
fn hitRadiance(hit: Hit, P: vec3f) -> vec3f {
  if (hit.inside) { return vec3f(0.0); }
  var L = vec3f(0.0);
  var albedo: vec3f;
  if (hit.idx == CAPSULE_HIT) { albedo = hit.capAlbedo; }
  else {
    let bi = u32(hit.idx);
    let b = boxGeo[bi];
    if ((b.flags & (BOX_EMISSIVE | BOX_AREALIT)) == BOX_EMISSIVE) { L += boxEmissive(bi); }   // area-lit emitters glow on screen but light the scene through their analytic light
    albedo = boxAlbedo(bi);
  }
  var E = vec3f(0.0);
  if ((frame.flags & FLAG_DIRECT) != 0u) { E += directIrradiance(P, hit.n, 0u, frame.secMinE, false); }   // steady lights only (this feeds the cascades / dice); dimmer ones get unshadowed half credit
  if ((frame.flags & FLAG_BOUNCE) != 0u) {
    // sample a full probe spacing off the surface: the trilinear footprint then lies entirely on this side of a thin wall
    E += diceIrradiance(P + hit.n * c0Spacing().x, hit.n);
  }
  L += albedo * E * (1.0 / 3.14159265);
  return L;
}
