// ===========================================================================
// Stable fluids (Stam 1999) driving the smoke medium.
//
// The simulation runs on its own world-aligned lattice (FP.origin, FP.cell,
// FP.dims — the density interface's box, resampled into the interface texture
// by `writeVolume` so a coarser debug lattice needs no other change). One
// step per frame: curl -> forces (buoyancy, vorticity confinement, source
// splats, dissipation) -> semi-Lagrangian velocity advection -> divergence ->
// Jacobi pressure -> gradient subtraction -> semi-Lagrangian scalar advection
// -> interface write. Semi-Lagrangian everything, so the game's variable dt
// (up to 50 ms) is unconditionally stable.
//
// Fields (ping-ponged 3D textures; storage textures are write-only, so every
// pass reads one copy and writes the other):
//   vel  rgba16f  xyz = m/s at cell centres (collocated grid)
//   scl  rgba16f  x = smoke density in the tracer's dimensionless density
//                 unit (sigma_t = U.volumetric * density), y = temperature
//                 (buoyancy's currency; cools on its own timescale)
//   curl rgba16f  xyz = vorticity, w = |vorticity|
//   prs, div  r32f   pressure solve state
// Solids: the baked occupancy buffer (byte per cell, x-fastest) plus the
// grid's six faces are walls — zero flux on wall faces, zero smoke, Neumann
// pressure. The velocity store is differenced as a staggered (MAC) field so
// that divergence, pressure gradient and the Jacobi Laplacian are one
// consistent operator triple; see the block above velAt for why.
// The room's ceiling is the grid top: row 12 (y 3.00-3.25) straddles the
// underside at 3.2 and stays fluid, matching the 5 cm overshoot the density
// contract documents.
// ===========================================================================

const MAX_FLUID_SOURCES: u32 = 32u;

struct Source {
  pos     : vec3f,
  /** Soft-sphere radius, metres; falloff (1 - r^2)^2. */
  radius  : f32,
  /** Velocity the medium is pushed toward inside the sphere, m/s. */
  vel     : vec3f,
  /** Push rate, 1/s: how hard the sphere drags the local velocity to `vel`. */
  push    : f32,
  /** Density and temperature added per second at the sphere's core. */
  density : f32,
  temp    : f32,
  _p0     : f32,
  _p1     : f32,
}

struct FluidParams {
  /** Sim lattice cells (x, y, z). */
  dims        : vec3u,
  sourceCount : u32,
  /** Metres per sim cell, per axis. */
  cell        : vec3f,
  dt          : f32,
  /** World minimum corner of the sim lattice. */
  origin      : vec3f,
  /** Density decay rate, 1/s (thinning smoke). */
  dissipation : f32,
  /** Interface (smoke volume) texture dims — the write pass's grid. */
  ifDims      : vec3u,
  /** Temperature decay rate, 1/s (air cooling). */
  cooling     : f32,
  ifCell      : vec3f,
  /** Upward acceleration per unit temperature, m/s^2. */
  buoyancy    : f32,
  ifOrigin    : vec3f,
  /** Downward acceleration per unit density, m/s^2 — dense smoke pools. */
  weight      : f32,
  /** Vorticity-confinement strength (Fedkiw's epsilon). */
  vorticity   : f32,
  _pad0       : f32,
  _pad1       : f32,
  _pad2       : f32,
  sources     : array<Source, MAX_FLUID_SOURCES>,
}

@group(0) @binding(0) var<uniform> FP : FluidParams;
@group(0) @binding(1) var linSamp : sampler;
/** Filterable inputs (rgba16f fields): sampled trilinearly by the advection. */
@group(0) @binding(2) var texA : texture_3d<f32>;
@group(0) @binding(3) var texB : texture_3d<f32>;
/** Unfiltered inputs (pressure, divergence, or a field only read by load). */
@group(0) @binding(4) var texC : texture_3d<f32>;
@group(0) @binding(5) var texD : texture_3d<f32>;
/** Static occupancy, one byte per cell packed four to a word; 1 = solid. */
@group(0) @binding(6) var<storage, read> occ : array<u32>;
@group(0) @binding(7) var outF0 : texture_storage_3d<rgba16float, write>;
@group(0) @binding(8) var outF1 : texture_storage_3d<rgba16float, write>;
@group(0) @binding(9) var outR  : texture_storage_3d<r32float, write>;

// ---------------------------------------------------------------------------

fn dimsI() -> vec3i { return vec3i(FP.dims); }

/** Solid: a static occupancy cell, or anything past the grid's six walls. */
fn solidAt(c: vec3i) -> bool {
  if (any(c < vec3i(0)) || any(c >= dimsI())) { return true; }
  let i = (u32(c.z) * FP.dims.y + u32(c.y)) * FP.dims.x + u32(c.x);
  return ((occ[i >> 2u] >> ((i & 3u) * 8u)) & 0xffu) != 0u;
}

fn cellCentre(g: vec3u) -> vec3f {
  return FP.origin + (vec3f(g) + 0.5) * FP.cell;
}

fn worldToUvw(p: vec3f) -> vec3f {
  return (p - FP.origin) / (vec3f(FP.dims) * FP.cell);
}

// The velocity store is read as a staggered (MAC) field: vel[c].xyz are the
// three fluxes on cell c's *minus* faces, not a cell-centre vector. Nothing
// about the storage changes — this is a choice of which difference operators
// are consistent with each other:
//
//   divergence  D+ : (vel[c+e].e - vel[c].e) / cell.e     (flux imbalance)
//   gradient    D- : (p[c] - p[c-e])        / cell.e      (across the same face)
//   D+ D-          = the compact 7-point Laplacian the jacobi kernel solves.
//
// The triple has to close, and changing one member means changing all three.
// Differencing velocity and pressure centrally is a 2h stencil: against the
// h-spacing Laplacian the solve converges to a pressure that does not null the
// divergence the instrument measures, and the h-scale divergence advection
// actually transports stays invisible to the operator. Measured, that costs a
// projection which reduces divergence by only ~1.5x whatever the iteration
// count, and a mass leak that does not respond to iterations at all.

/** Velocity at a cell, zero inside solids and past the walls. */
fn velAt(c: vec3i) -> vec3f {
  if (solidAt(c)) { return vec3f(0.0); }
  return textureLoad(texA, c, 0).xyz;
}

/** As velAt, for the pass whose velocity input is texB (scalar advection). */
fn velAtB(c: vec3i) -> vec3f {
  if (solidAt(c)) { return vec3f(0.0); }
  return textureLoad(texB, c, 0).xyz;
}

/**
 * Cell-centre velocity: the mean of the cell's opposing face fluxes. This is
 * the field a semi-Lagrangian trace must use — it is the centred
 * reconstruction of the face field the projection made divergence-free, so
 * the trace sees the flow the constraint actually controls.
 */
fn velCentre(c: vec3i) -> vec3f {
  let m = velAt(c);
  return 0.5 * (m + vec3f(
    velAt(c + vec3i(1, 0, 0)).x,
    velAt(c + vec3i(0, 1, 0)).y,
    velAt(c + vec3i(0, 0, 1)).z,
  ));
}

fn velCentreB(c: vec3i) -> vec3f {
  let m = velAtB(c);
  return 0.5 * (m + vec3f(
    velAtB(c + vec3i(1, 0, 0)).x,
    velAtB(c + vec3i(0, 1, 0)).y,
    velAtB(c + vec3i(0, 0, 1)).z,
  ));
}

/**
 * No flow through a wall. Applied wherever velocity is written, so the
 * divergence pass is handed a field that already satisfies the boundary
 * condition: the Jacobi solve is then never asked to remove a flux the
 * Neumann pressure cannot reach, and the projection preserves the zero
 * instead of a post-hoc clamp destroying the solve's work.
 */
fn wallFaces(c: vec3i, v: vec3f) -> vec3f {
  var o = v;
  if (solidAt(c - vec3i(1, 0, 0))) { o.x = 0.0; }
  if (solidAt(c - vec3i(0, 1, 0))) { o.y = 0.0; }
  if (solidAt(c - vec3i(0, 0, 1))) { o.z = 0.0; }
  return o;
}

// ---- curl: texA = vel -> outF0 = (vorticity, |vorticity|) ------------------

@compute @workgroup_size(4, 4, 4)
fn curl(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  let L = velAt(c - vec3i(1, 0, 0)); let R = velAt(c + vec3i(1, 0, 0));
  let D = velAt(c - vec3i(0, 1, 0)); let U = velAt(c + vec3i(0, 1, 0));
  let B = velAt(c - vec3i(0, 0, 1)); let F = velAt(c + vec3i(0, 0, 1));
  let inv = 0.5 / FP.cell;
  let w = vec3f(
    (U.z - D.z) * inv.y - (F.y - B.y) * inv.z,
    (F.x - B.x) * inv.z - (R.z - L.z) * inv.x,
    (R.y - L.y) * inv.x - (U.x - D.x) * inv.y,
  );
  textureStore(outF0, g, vec4f(w, length(w)));
}

// ---- forces: texA = vel, texB = scalars, texC = curl -> outF0 vel, outF1 scl -

fn curlMagAt(c: vec3i) -> f32 {
  return textureLoad(texC, clamp(c, vec3i(0), dimsI() - 1), 0).w;
}

@compute @workgroup_size(4, 4, 4)
fn forces(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) {
    textureStore(outF0, g, vec4f(0.0));
    textureStore(outF1, g, vec4f(0.0));
    return;
  }
  var v = textureLoad(texA, c, 0).xyz;
  let scl = textureLoad(texB, c, 0);
  var dens = scl.x;
  var temp = scl.y;
  let dt = FP.dt;

  // Buoyancy: heat lifts, mass sinks. This is the whole reason smoke moves
  // when nothing pushes it — the muzzle burst rises, the canister cloud pools.
  v.y = v.y + (FP.buoyancy * temp - FP.weight * dens) * dt;

  // Vorticity confinement (Fedkiw 2001): re-inject the small-scale curl the
  // semi-Lagrangian advection smears away — this is what reads as smoke.
  let om = textureLoad(texC, c, 0).xyz;
  let inv = 0.5 / FP.cell;
  let eta = vec3f(
    (curlMagAt(c + vec3i(1, 0, 0)) - curlMagAt(c - vec3i(1, 0, 0))) * inv.x,
    (curlMagAt(c + vec3i(0, 1, 0)) - curlMagAt(c - vec3i(0, 1, 0))) * inv.y,
    (curlMagAt(c + vec3i(0, 0, 1)) - curlMagAt(c - vec3i(0, 0, 1))) * inv.z,
  );
  let el = length(eta);
  if (el > 1e-5) {
    v = v + FP.vorticity * FP.cell.x * cross(eta / el, om) * dt;
  }

  // Sources: soft spheres adding density/temperature and dragging the local
  // velocity toward their own (a muzzle burst points down the barrel).
  let p = cellCentre(g);
  for (var i = 0u; i < FP.sourceCount; i = i + 1u) {
    let s = FP.sources[i];
    let dq = p - s.pos;
    let d2 = dot(dq, dq) / (s.radius * s.radius);
    if (d2 >= 1.0) { continue; }
    let fall = (1.0 - d2) * (1.0 - d2);
    dens = dens + s.density * fall * dt;
    temp = temp + s.temp * fall * dt;
    v = mix(v, s.vel, clamp(s.push * fall * dt, 0.0, 1.0));
  }

  dens = dens * exp(-FP.dissipation * dt);
  temp = temp * exp(-FP.cooling * dt);
  textureStore(outF0, g, vec4f(wallFaces(c, v), 0.0));
  textureStore(outF1, g, vec4f(max(dens, 0.0), max(temp, 0.0), 0.0, 0.0));
}

// ---- advectVel: texA = vel (sampled) -> outF0 = vel -------------------------

@compute @workgroup_size(4, 4, 4)
fn advectVel(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outF0, g, vec4f(0.0)); return; }
  let p = cellCentre(g);
  let vn = textureSampleLevel(
    texA, linSamp, worldToUvw(p - velCentre(c) * FP.dt), 0.0).xyz;
  textureStore(outF0, g, vec4f(wallFaces(c, vn), 0.0));
}

// ---- divergence: texA = vel -> outR = div ------------------------------------

@compute @workgroup_size(4, 4, 4)
fn divergence(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outR, g, vec4f(0.0)); return; }
  // Flux imbalance over the cell's six faces. A wall face reads zero (velAt
  // zeroes solids and every writer applies wallFaces), so no flux is ever
  // counted through geometry.
  let inv = 1.0 / FP.cell;
  let m = velAt(c);
  let d = (velAt(c + vec3i(1, 0, 0)).x - m.x) * inv.x
        + (velAt(c + vec3i(0, 1, 0)).y - m.y) * inv.y
        + (velAt(c + vec3i(0, 0, 1)).z - m.z) * inv.z;
  textureStore(outR, g, vec4f(d, 0.0, 0.0, 0.0));
}

// ---- jacobi: texC = pressure in, texD = divergence -> outR = pressure ----
// Solid neighbours are Neumann (dp/dn = 0): they mirror the centre pressure.

fn prsAt(c: vec3i, pc: f32) -> f32 {
  if (solidAt(c)) { return pc; }
  return textureLoad(texC, c, 0).x;
}

@compute @workgroup_size(4, 4, 4)
fn jacobi(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outR, g, vec4f(0.0)); return; }
  let pc = textureLoad(texC, c, 0).x;
  let div = textureLoad(texD, c, 0).x;
  let w = 1.0 / (FP.cell * FP.cell);
  let sum = (prsAt(c - vec3i(1, 0, 0), pc) + prsAt(c + vec3i(1, 0, 0), pc)) * w.x
          + (prsAt(c - vec3i(0, 1, 0), pc) + prsAt(c + vec3i(0, 1, 0), pc)) * w.y
          + (prsAt(c - vec3i(0, 0, 1), pc) + prsAt(c + vec3i(0, 0, 1), pc)) * w.z;
  textureStore(outR, g, vec4f((sum - div) / (2.0 * (w.x + w.y + w.z)), 0.0, 0.0, 0.0));
}

// ---- project: texA = vel, texC = pressure -> outF0 = divergence-free vel --

@compute @workgroup_size(4, 4, 4)
fn project(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outF0, g, vec4f(0.0)); return; }
  let pc = textureLoad(texC, c, 0).x;
  var v = textureLoad(texA, c, 0).xyz;
  let inv = 1.0 / FP.cell;
  // Backward gradient, taken across the very faces v's components live on —
  // the adjoint of the divergence above, so subtracting it removes exactly
  // the divergence the solve was given. A solid neighbour is Neumann
  // (mirrored pressure), which makes the gradient across a wall face vanish
  // and leaves the zero flux wallFaces already put there.
  v = v - vec3f(
    (pc - prsAt(c - vec3i(1, 0, 0), pc)) * inv.x,
    (pc - prsAt(c - vec3i(0, 1, 0), pc)) * inv.y,
    (pc - prsAt(c - vec3i(0, 0, 1), pc)) * inv.z,
  );
  textureStore(outF0, g, vec4f(wallFaces(c, v), 0.0));
}

// ---- advectScl: texA = scalars (loaded), texB = vel -> outF0 = scalars ---
// Solid-aware trilinear gather. Solid cells store zero density, so a
// hardware trilinear sample against a wall averages smoke with those zeros
// and destroys mass every step — worst under the ceiling, where the smoke
// pools with its largest contact area. The eight corners are gathered by
// hand instead: solid corners are dropped and the remaining weights
// renormalised, so a wall neither absorbs nor emits scalar. A footprint that
// is entirely solid keeps the cell's own value (nothing arrives from rock).

fn sampleScalarsFluid(p: vec3f, own: vec2f) -> vec2f {
  let f = (p - FP.origin) / FP.cell - 0.5;
  let base = vec3i(floor(f));
  let fr = f - floor(f);
  var acc = vec2f(0.0);
  var wsum = 0.0;
  for (var k = 0; k < 2; k = k + 1) {
    for (var j = 0; j < 2; j = j + 1) {
      for (var i = 0; i < 2; i = i + 1) {
        let cc = base + vec3i(i, j, k);
        if (solidAt(cc)) { continue; }
        let w = select(1.0 - fr.x, fr.x, i == 1)
              * select(1.0 - fr.y, fr.y, j == 1)
              * select(1.0 - fr.z, fr.z, k == 1);
        acc = acc + w * textureLoad(texA, cc, 0).xy;
        wsum = wsum + w;
      }
    }
  }
  if (wsum < 1e-4) { return own; }
  return acc / wsum;
}

@compute @workgroup_size(4, 4, 4)
fn advectScl(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outF0, g, vec4f(0.0)); return; }
  let p = cellCentre(g);
  let s = sampleScalarsFluid(
    p - velCentreB(c) * FP.dt, textureLoad(texA, c, 0).xy);
  textureStore(outF0, g, vec4f(max(s.x, 0.0), max(s.y, 0.0), 0.0, 0.0));
}

// ---- writeVolume: texA = scalars -> outF0 = the density interface texture --
// The interface grid (FP.ifDims/ifCell/ifOrigin) is B2a's fixed contract;
// the sim lattice may be coarser, so this resamples trilinearly. R = density,
// G/B/A written zero as the contract requires.

@compute @workgroup_size(4, 4, 4)
fn writeVolume(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.ifDims)) { return; }
  let p = FP.ifOrigin + (vec3f(g) + 0.5) * FP.ifCell;
  let d = textureSampleLevel(texA, linSamp, worldToUvw(p), 0.0).x;
  textureStore(outF0, g, vec4f(d, 0.0, 0.0, 0.0));
}
