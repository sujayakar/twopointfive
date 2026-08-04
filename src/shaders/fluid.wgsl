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
// The lattice's own top face is the ceiling, and that is a DEVIATION from the
// density contract, which says to treat y >= 3.2 as a solid top wall. Row 12
// (y 3.00-3.25) straddles the ceiling underside at 3.2 and stays fluid: 0.25 m
// cells cannot tile 3.2, and the row is 80% real air, so solidifying it would
// delete the layer ceiling-hugging smoke lives in. The cost is that up to 5 cm
// of smoke can sit inside the invisible ceiling slab; camera rays ignore that
// slab, so those 5 cm are marched (~1.5% of a floor-to-ceiling column).
// ===========================================================================

// Must match FLUID_MAX_SOURCES in engine/fluid.ts; the params struct is
// sized from it on both sides and a mismatch is a silent buffer overrun.
const MAX_FLUID_SOURCES: u32 = 96u;

/**
 * Ceiling on |div*dt| in the scalar advection's volume correction.
 *
 * One step of the semi-Lagrangian map can only represent so much volume
 * change before the departure points fold over each other; past that the
 * correction is extrapolating. Clamping keeps a very violent source stable
 * rather than correct, which is the right trade for something that lasts a
 * quarter of a second.
 */
const MAX_DIV_DT: f32 = 2.0;

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
  /**
   * Expansion rate at the core, 1/s — the target divergence this source asks
   * the projection to LEAVE IN the field rather than remove.
   *
   * This exists because splatting a radial velocity does not make an
   * explosion. The projection's entire job is deleting divergence, so a
   * radial `vel` splat is removed within the same step and an explosive
   * source just appears instead of expanding. Feeding the RHS instead means
   * the pressure solve itself produces the outward push, and it stays
   * consistent with the walls (a burst in a corner vents the only way it can)
   * because the same Neumann condition applies.
   */
  expand  : f32,
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

  // ---- Second-instance fields ------------------------------------------
  // All zero on the coarse global lattice, which is what makes that lattice
  // bit-identical to before this block existed. They describe how a SECOND,
  // finer FluidSim instance relates to the coarse one it lives inside.

  /** OCC_LOCAL_BYTE: this lattice owns its bake. OCC_GLOBAL_BIT: window into a shared field. */
  occMode     : u32,
  /**
   * Bitmask of lattice faces that are OPEN outflow rather than solid wall
   * (bit 0 = -x, 1 = +x, 2 = -z, 3 = +z). The coarse lattice's faces are the
   * room; a fine lattice's lateral faces are arbitrary cuts through open air
   * and must not behave like walls. y faces are never open — floor and
   * ceiling are real on both.
   */
  openFaces   : u32,
  /** PEER_NONE, or PEER_COARSE when this lattice should read/write a coarse peer. */
  peerMode    : u32,
  /** This lattice's origin in the shared occupancy field's cell coordinates. */
  occOffset   : vec3i,
  /** Width, in this lattice's cells, of the rim where peer coupling ramps out. */
  rimCells    : f32,
  /** Dimensions of the shared occupancy field. */
  occDims     : vec3u,
  _pad1       : f32,
  peerOrigin  : vec3f,
  _pad2       : f32,
  peerCell    : vec3f,
  _pad3       : f32,
  peerDims    : vec3u,
  _pad4       : f32,

  sources     : array<Source, MAX_FLUID_SOURCES>,
}

const OCC_LOCAL_BYTE : u32 = 0u;
const OCC_GLOBAL_BIT : u32 = 1u;
const PEER_NONE      : u32 = 0u;
const PEER_COARSE    : u32 = 1u;

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

/**
 * Solid: a static occupancy cell, or anything past a wall.
 *
 * Two forms. The coarse global lattice owns a byte-per-cell bake sized to
 * itself, and everything past its bounds is wall because its bounds ARE the
 * room. A fine local lattice owns no bake; it reads a window into a level-wide
 * bit field, and everything past its LATERAL bounds is ordinary air that
 * happens to lie outside the box — so the out-of-bounds answer has to come
 * from the shared field rather than from a blanket `true`. Its y bounds are
 * still real floor and ceiling.
 */
fn solidAt(c: vec3i) -> bool {
  if (FP.occMode == OCC_LOCAL_BYTE) {
    if (any(c < vec3i(0)) || any(c >= dimsI())) { return true; }
    let i = (u32(c.z) * FP.dims.y + u32(c.y)) * FP.dims.x + u32(c.x);
    return ((occ[i >> 2u] >> ((i & 3u) * 8u)) & 0xffu) != 0u;
  }
  // Floor and ceiling are real on both lattices.
  if (c.y < 0 || c.y >= dimsI().y) { return true; }
  let g = c + FP.occOffset;
  // Off the level entirely — outside the baked field, so genuinely wall.
  if (any(g < vec3i(0)) || any(g >= vec3i(FP.occDims))) { return true; }
  let i = (u32(g.z) * FP.occDims.y + u32(g.y)) * FP.occDims.x + u32(g.x);
  return ((occ[i >> 5u] >> (i & 31u)) & 1u) != 0u;
}

/**
 * Is cell `c` outside the lattice through a face marked OPEN?
 *
 * Only meaningful on a fine local lattice. Its lateral bounds are arbitrary
 * cuts through open air, so smoke must be able to leave through them and
 * pressure must not build against them. Bits: 0 = -x, 1 = +x, 2 = -z, 3 = +z.
 * y is never open — floor and ceiling are real.
 */
fn throughOpenFace(c: vec3i) -> bool {
  if (FP.openFaces == 0u) { return false; }
  let d = dimsI();
  if (c.x < 0)    { return (FP.openFaces & 1u) != 0u; }
  if (c.x >= d.x) { return (FP.openFaces & 2u) != 0u; }
  if (c.z < 0)    { return (FP.openFaces & 4u) != 0u; }
  if (c.z >= d.z) { return (FP.openFaces & 8u) != 0u; }
  return false;
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
  // Clamp, do not zero, through an open face: zero velocity IS a no-slip wall,
  // and it is what would stop a jet dead at the edge of the fine box.
  if (throughOpenFace(c)) {
    return textureLoad(texA, clamp(c, vec3i(0), dimsI() - vec3i(1)), 0).xyz;
  }
  if (solidAt(c)) { return vec3f(0.0); }
  return textureLoad(texA, c, 0).xyz;
}

/** As velAt, for the pass whose velocity input is texB (scalar advection). */
fn velAtB(c: vec3i) -> vec3f {
  if (throughOpenFace(c)) {
    return textureLoad(texB, clamp(c, vec3i(0), dimsI() - vec3i(1)), 0).xyz;
  }
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

// The full velocity vector at each of the cell's three minus faces. The
// on-axis component is stored there already; the two transverse ones are the
// mean of the four parallel fluxes surrounding that face, which is the
// standard MAC reconstruction.
fn faceVelX(c: vec3i) -> vec3f {
  let ex = vec3i(1, 0, 0);
  let m = velAt(c);
  let b = velAt(c - ex);
  return vec3f(
    m.x,
    0.25 * (m.y + velAt(c + vec3i(0, 1, 0)).y + b.y + velAt(c - ex + vec3i(0, 1, 0)).y),
    0.25 * (m.z + velAt(c + vec3i(0, 0, 1)).z + b.z + velAt(c - ex + vec3i(0, 0, 1)).z),
  );
}

fn faceVelY(c: vec3i) -> vec3f {
  let ey = vec3i(0, 1, 0);
  let m = velAt(c);
  let b = velAt(c - ey);
  return vec3f(
    0.25 * (m.x + velAt(c + vec3i(1, 0, 0)).x + b.x + velAt(c - ey + vec3i(1, 0, 0)).x),
    m.y,
    0.25 * (m.z + velAt(c + vec3i(0, 0, 1)).z + b.z + velAt(c - ey + vec3i(0, 0, 1)).z),
  );
}

fn faceVelZ(c: vec3i) -> vec3f {
  let ez = vec3i(0, 0, 1);
  let m = velAt(c);
  let b = velAt(c - ez);
  return vec3f(
    0.25 * (m.x + velAt(c + vec3i(1, 0, 0)).x + b.x + velAt(c - ez + vec3i(1, 0, 0)).x),
    0.25 * (m.y + velAt(c + vec3i(0, 1, 0)).y + b.y + velAt(c - ez + vec3i(0, 1, 0)).y),
    m.z,
  );
}

/**
 * Cell-centre velocity AND divergence from one set of face loads.
 *
 * Scalar advection needs both, and both are built from the same four MAC
 * samples — the cell's own minus faces and its three plus-face neighbours — so
 * reading them once is free next to reading them twice.
 */
struct TraceB {
  /** Centred reconstruction, the field a semi-Lagrangian trace must follow. */
  vel : vec3f,
  /** Flux imbalance over the six faces, 1/s. Zero except where a source expands. */
  div : f32,
}

fn traceB(c: vec3i) -> TraceB {
  let m = velAtB(c);
  let px = velAtB(c + vec3i(1, 0, 0)).x;
  let py = velAtB(c + vec3i(0, 1, 0)).y;
  let pz = velAtB(c + vec3i(0, 0, 1)).z;
  let inv = 1.0 / FP.cell;
  var t : TraceB;
  t.vel = 0.5 * (m + vec3f(px, py, pz));
  t.div = (px - m.x) * inv.x + (py - m.y) * inv.y + (pz - m.z) * inv.z;
  return t;
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
//
// Compact (h) differences, not the 2h central ones this used to take.
//
// vel is a MAC field — vel[c].xyz are the fluxes on c's MINUS faces — so the
// natural curl is a difference across ONE cell, between a face and its
// neighbour's face. Differencing over 2h instead low-passes the vorticity: the
// field it produced was smooth at the cell scale, so the confinement force
// built from it only ever re-injected swirl at twice the grid size. Turning
// `vorticity` up made the rolls stronger, never finer, which is exactly the
// symptom of measuring curl on a smoothed field.
//
// This also early-outs on solids. The previous version deliberately did not,
// so a solid cell was handed a vorticity differenced from its fluid
// neighbours' velocities and the confinement gradient beside an obstacle had
// one endpoint inside it. That was a known deviation kept only to protect
// measurements this change invalidates anyway.

fn edgeCurl(c: vec3i) -> vec3f {
  let m = velAt(c);
  let mx = velAt(c - vec3i(1, 0, 0));
  let my = velAt(c - vec3i(0, 1, 0));
  let mz = velAt(c - vec3i(0, 0, 1));
  let inv = 1.0 / FP.cell;
  // Each component differences the two face values that straddle the edge it
  // lives on, which is the adjoint-consistent partner of the divergence and
  // gradient stencils above.
  return vec3f(
    (m.z - my.z) * inv.y - (m.y - mz.y) * inv.z,
    (m.x - mz.x) * inv.z - (m.z - mx.z) * inv.x,
    (m.y - mx.y) * inv.x - (m.x - my.x) * inv.y,
  );
}

@compute @workgroup_size(4, 4, 4)
fn curl(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outF0, g, vec4f(0.0)); return; }

  // The three components do not share a location. With the minus-face
  // convention, edgeCurl's .x sits on the x-aligned edge at (i+1/2, j, k), .y
  // at (i, j+1/2, k), .z at (i, j, k+1/2) — three different edges of the cell,
  // none of them its centre. Both consumers in `forces` assume a cell-centred
  // vector: curlMagAt is centrally differenced for eta, and `om` is crossed
  // with the result. Storing the edge values directly makes the confinement
  // force a cross product of two quantities sampled a half cell apart on
  // different axes, and the error is largest exactly where |curl| varies
  // fastest — the thin shear layers confinement exists to preserve.
  //
  // So each component is averaged from its four parallel edges to the centre.
  // The x-edges bounding this cell are at j+{0,1}, k+{0,1}; the y-edges at
  // i+{0,1}, k+{0,1}; the z-edges at i+{0,1}, j+{0,1}.
  let ex = vec3i(1, 0, 0);
  let ey = vec3i(0, 1, 0);
  let ez = vec3i(0, 0, 1);
  let w000 = edgeCurl(c);
  let wx = edgeCurl(c + ex);
  let wy = edgeCurl(c + ey);
  let wz = edgeCurl(c + ez);
  let wxy = edgeCurl(c + ex + ey);
  let wxz = edgeCurl(c + ex + ez);
  let wyz = edgeCurl(c + ey + ez);
  let w = vec3f(
    (w000.x + wy.x + wz.x + wyz.x) * 0.25,
    (w000.y + wx.y + wz.y + wxz.y) * 0.25,
    (w000.z + wx.z + wy.z + wxy.z) * 0.25,
  );
  textureStore(outF0, g, vec4f(w, length(w)));
}

// ---- forces: texA = vel, texB = scalars, texC = curl -> outF0 vel, outF1 scl -

fn curlMagAt(c: vec3i) -> f32 {
  // Masked, not clamped: a clamped read outside the lattice duplicates the
  // edge cell and fabricates a gradient there, and a solid cell now stores
  // zero rather than a vorticity borrowed from its fluid neighbours.
  if (solidAt(c)) { return 0.0; }
  return textureLoad(texC, c, 0).w;
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
  //
  // v.y is the flux on the cell's y-MINUS face, half a cell below the centre
  // where scl lives, so the driving temperature is the average across that
  // face. Using the cell's own value instead registers the force half a cell
  // high, which is a systematic upward bias at the resolution the confinement
  // is trying to preserve detail at. Below a solid the average degenerates to
  // the cell's own value (Neumann: no gradient into a wall), which also keeps
  // the floor row from being driven by whatever the occupancy buffer's
  // interior happens to hold.
  let below = c - vec3i(0, 1, 0);
  var sclB = scl;
  if (!solidAt(below)) { sclB = textureLoad(texB, below, 0); }
  let fTemp = 0.5 * (scl.y + sclB.y);
  let fDens = 0.5 * (scl.x + sclB.x);
  v.y = v.y + (FP.buoyancy * fTemp - FP.weight * fDens) * dt;

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
    // Fedkiw's epsilon * h. h is the SMALLEST cell dimension, not cell.x:
    // setScale(2) doubles cell.x and cell.z while keeping cell.y, so scaling
    // by cell.x would run the halved lattice at twice the confinement
    // strength — a different simulation, not a coarser one. min() is
    // scale-invariant because cell.y never changes and x/z only grow.
    let h = min(FP.cell.x, min(FP.cell.y, FP.cell.z));
    v = v + FP.vorticity * h * cross(eta / el, om) * dt;
  }

  // Sources: soft spheres adding density/temperature and dragging the local
  // velocity toward their own (a muzzle burst points down the barrel).
  let p = cellCentre(g);
  let half = 0.5 * FP.cell;
  for (var i = 0u; i < FP.sourceCount; i = i + 1u) {
    let s = FP.sources[i];
    let dq = p - s.pos;
    // Radius comes straight from the caller (`__smokePuff` is exposed), and a
    // zero would make d2 = 0/0 at the cell whose centre coincides with the
    // source: WGSL does not pin that result, and `max` below does not pin its
    // NaN handling either, so the field could carry a NaN until reset().
    let r = max(s.radius, 1e-4);
    let inv2 = 1.0 / (r * r);
    let d2 = dot(dq, dq) * inv2;
    // The scalars are cell-centred and the velocity is face-centred, so they
    // sample the sphere at four different points. df holds the squared radius
    // at each of the three minus faces; evaluating all of them at the centre
    // shifts the entire push half a cell along the diagonal, which at a 0.6 m
    // source and 0.25 m cells is a fifth of the sphere.
    let df = vec3f(
      dot(vec3f(dq.x - half.x, dq.y, dq.z), vec3f(dq.x - half.x, dq.y, dq.z)),
      dot(vec3f(dq.x, dq.y - half.y, dq.z), vec3f(dq.x, dq.y - half.y, dq.z)),
      dot(vec3f(dq.x, dq.y, dq.z - half.z), vec3f(dq.x, dq.y, dq.z - half.z)),
    ) * inv2;
    // Widened: a cell centre outside the sphere can still own a face inside
    // it, so the old centre-only early-out would clip the push by a face.
    if (d2 >= 1.0 && all(df >= vec3f(1.0))) { continue; }
    let fall = select(0.0, (1.0 - d2) * (1.0 - d2), d2 < 1.0);
    dens = dens + s.density * fall * dt;
    temp = temp + s.temp * fall * dt;
    let ff = max(vec3f(1.0) - df, vec3f(0.0));
    v = mix(v, s.vel, clamp(s.push * ff * ff * dt, vec3f(0.0), vec3f(1.0)));
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
  // Each stored component is a flux on a DIFFERENT face, so each is traced
  // with the velocity at its own face rather than all three sharing the
  // cell-centre vector. A single centred trace advects the x flux along the
  // trajectory of a point half a cell to its right, which under shear is the
  // wrong streamline — it smears the shear layer at exactly the rate
  // confinement is trying to counteract.
  //
  // The sample coordinate takes NO half-cell offset. worldToUvw(cellCentre)
  // already lands on texel c, and texel c stores c's minus-face value, so the
  // departure point's offset and the lattice's offset are the same half cell
  // and cancel: worldToUvw(q + half_e) == worldToUvw(p - u*dt) identically.
  // Adding one shifts the field by a full cell per step.
  let ux = textureSampleLevel(
    texA, linSamp, worldToUvw(p - faceVelX(c) * FP.dt), 0.0).x;
  let uy = textureSampleLevel(
    texA, linSamp, worldToUvw(p - faceVelY(c) * FP.dt), 0.0).y;
  let uz = textureSampleLevel(
    texA, linSamp, worldToUvw(p - faceVelZ(c) * FP.dt), 0.0).z;
  textureStore(outF0, g, vec4f(wallFaces(c, vec3f(ux, uy, uz)), 0.0));
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

  // Expanding sources are subtracted from the RHS, not added to velocity.
  // Solving lap(p) = div(v) - S and then subtracting grad(p) leaves exactly
  // div(v) = S, so the cell really does push its neighbours apart; adding the
  // same thing to `vel` would be undone by this step. S is zero for every
  // ordinary emitter, which is why the field is packed but unused by them.
  var s = 0.0;
  if (FP.sourceCount > 0u) {
    let p = cellCentre(g);
    for (var i = 0u; i < FP.sourceCount; i = i + 1u) {
      let src = FP.sources[i];
      if (src.expand == 0.0) { continue; }
      let dq = p - src.pos;
      let r = max(src.radius, 1e-4);
      let d2 = dot(dq, dq) / (r * r);
      if (d2 >= 1.0) { continue; }
      let fall = (1.0 - d2) * (1.0 - d2);
      s = s + src.expand * fall;
    }
  }
  // A source may not ask for more divergence than one step can represent, and
  // never more than advectScl's MAX_DIV_DT will pay back. At 60 Hz this is
  // 96/s and never binds at the presets in use; at the game's 50 ms dt cap it
  // is 32/s and does, which is the case worth measuring.
  textureStore(outR, g, vec4f(d - min(s, 1.6 / FP.dt), 0.0, 0.0, 0.0));
}

// ---- jacobi: texC = pressure in, texD = divergence -> outR = pressure ----
// Solid neighbours are Neumann (dp/dn = 0): they mirror the centre pressure.

fn prsAt(c: vec3i, pc: f32) -> f32 {
  // Open face: Dirichlet p = 0, NOT the Neumann mirror. Returning `pc` here
  // would say "no pressure gradient across this face", i.e. a wall, which is
  // the opposite of outflow. Keeping 0 while the caller keeps the full
  // 2*(wx+wy+wz) denominator is the correct discretisation of an open
  // boundary, and it also makes the fine lattice's Poisson system
  // non-singular — unlike the coarse one, which is all-Neumann and defined
  // only up to a constant.
  if (throughOpenFace(c)) { return 0.0; }
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

/** Gather result plus the extremes of the corners it actually used. */
struct Gathered {
  value : vec2f,
  lo    : vec2f,
  hi    : vec2f,
}

/**
 * As sampleScalarsFluid, but also reporting the min/max over the SAME corner
 * set — the solid-aware one, not the plain trilinear eight. MacCormack's
 * limiter must clamp against the corners the forward gather really read, or a
 * cell beside a wall gets clamped to a bound that includes cells the gather
 * deliberately dropped.
 */
fn gatherScalars(p: vec3f, own: vec2f) -> Gathered {
  let f = (p - FP.origin) / FP.cell - 0.5;
  let base = vec3i(floor(f));
  let fr = f - floor(f);
  var acc = vec2f(0.0);
  var wsum = 0.0;
  var lo = vec2f(1e30);
  var hi = vec2f(-1e30);
  for (var k = 0; k < 2; k = k + 1) {
    for (var j = 0; j < 2; j = j + 1) {
      for (var i = 0; i < 2; i = i + 1) {
        let cc = base + vec3i(i, j, k);
        // Wall: drop the weight, so the remaining corners renormalise and
        // smoke piles against geometry. Open face: KEEP the weight with a zero
        // value, so smoke crossing the fine box's lateral edge thins out.
        // Those are opposite behaviours and the distinction is the difference
        // between a box that contains smoke and one that lets it leave.
        if (throughOpenFace(cc)) {
          let wo = select(1.0 - fr.x, fr.x, i == 1)
                 * select(1.0 - fr.y, fr.y, j == 1)
                 * select(1.0 - fr.z, fr.z, k == 1);
          wsum = wsum + wo;
          continue;
        }
        if (solidAt(cc)) { continue; }
        let w = select(1.0 - fr.x, fr.x, i == 1)
              * select(1.0 - fr.y, fr.y, j == 1)
              * select(1.0 - fr.z, fr.z, k == 1);
        let s = textureLoad(texA, cc, 0).xy;
        acc = acc + w * s;
        wsum = wsum + w;
        lo = min(lo, s);
        hi = max(hi, s);
      }
    }
  }
  var g : Gathered;
  if (wsum < 1e-4) {
    g.value = own; g.lo = own; g.hi = own;
    return g;
  }
  g.value = acc / wsum;
  g.lo = lo;
  g.hi = hi;
  return g;
}

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
        // Wall: drop the weight, so the remaining corners renormalise and
        // smoke piles against geometry. Open face: KEEP the weight with a zero
        // value, so smoke crossing the fine box's lateral edge thins out.
        // Those are opposite behaviours and the distinction is the difference
        // between a box that contains smoke and one that lets it leave.
        if (throughOpenFace(cc)) {
          let wo = select(1.0 - fr.x, fr.x, i == 1)
                 * select(1.0 - fr.y, fr.y, j == 1)
                 * select(1.0 - fr.z, fr.z, k == 1);
          wsum = wsum + wo;
          continue;
        }
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

// ---------------------------------------------------------------------------
// MacCormack scalar advection, in two passes.
//
// A single semi-Lagrangian gather is first-order: it smears the field a little
// every step, and after a few dozen steps a canister cloud is a featureless
// lozenge whatever the simulation put there. MacCormack estimates that error
// and subtracts it, which is second-order accurate for the same gather twice.
//
//   phi_hat   = A(phi^n)              forward  (pass 1, advectSclFwd)
//   phi_tilde = A*(phi_hat)           backward (pass 2)
//   phi^n+1   = phi_hat + (phi^n - phi_tilde) / 2
//
// BOTH legs run in the undeformed frame — neither A nor A* carries a volume
// factor. The single factor V = exp(-div*dt) is applied once, to density only,
// AFTER the correction and the limiter, in advectSclMac; the derivation and the
// reason the order is not interchangeable are stated there. Splitting the
// factor across the two legs (A with exp(-div*dt), A* with exp(+div*dt)) looks
// natural and is wrong: it leaves a residual (V-1)*E and re-creates the mass
// bug the factor exists to fix. Do not "restore" it here.
//
// The correction is unlimited by construction and overshoots at sharp edges —
// with fp16 storage and max(x, 0) that shows up as ringing and as mass loss.
// It is clamped to the extremes of the corners the FORWARD gather actually
// read, which is what makes the scheme monotone enough to ship.

/** As sampleScalarsFluid, but gathering phi_hat out of texC. */
fn sampleHatFluid(p: vec3f, own: vec2f) -> vec2f {
  let f = (p - FP.origin) / FP.cell - 0.5;
  let base = vec3i(floor(f));
  let fr = f - floor(f);
  var acc = vec2f(0.0);
  var wsum = 0.0;
  for (var k = 0; k < 2; k = k + 1) {
    for (var j = 0; j < 2; j = j + 1) {
      for (var i = 0; i < 2; i = i + 1) {
        let cc = base + vec3i(i, j, k);
        // Wall: drop the weight, so the remaining corners renormalise and
        // smoke piles against geometry. Open face: KEEP the weight with a zero
        // value, so smoke crossing the fine box's lateral edge thins out.
        // Those are opposite behaviours and the distinction is the difference
        // between a box that contains smoke and one that lets it leave.
        if (throughOpenFace(cc)) {
          let wo = select(1.0 - fr.x, fr.x, i == 1)
                 * select(1.0 - fr.y, fr.y, j == 1)
                 * select(1.0 - fr.z, fr.z, k == 1);
          wsum = wsum + wo;
          continue;
        }
        if (solidAt(cc)) { continue; }
        let w = select(1.0 - fr.x, fr.x, i == 1)
              * select(1.0 - fr.y, fr.y, j == 1)
              * select(1.0 - fr.z, fr.z, k == 1);
        acc = acc + w * textureLoad(texC, cc, 0).xy;
        wsum = wsum + w;
      }
    }
  }
  if (wsum < 1e-4) { return own; }
  return acc / wsum;
}

@compute @workgroup_size(4, 4, 4)
fn advectSclFwd(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outF0, g, vec4f(0.0)); return; }
  let p = cellCentre(g);
  let t = traceB(c);
  let s = sampleScalarsFluid(p - t.vel * FP.dt, textureLoad(texA, c, 0).xy);

  // Divergence correction: the gather is a MAP, and a map that changes volume
  // changes density.
  //
  // A gather reads whatever value sat at the departure point and writes it
  // here unchanged, which is only conservative when the flow preserves volume.
  // With an expanding source it does not: many cells trace back into the same
  // dense core and each copies its value, so the same smoke is written several
  // times over. Measured before this line existed, an identical burst held 3.79
  // mass units at expand = 0 and 20.21 at expand = 70 — 5.3x the smoke out of
  // nothing, and unbounded, since the error compounds every step until fp16
  // saturates.
  //
  // Conservation says rho_new * dV_new = rho_old * dV_old, and the parcel's
  // volume grows as dV_new/dV_old = 1 + div*dt, so the gathered value owes a
  // factor of 1/(1 + div*dt) ~ exp(-div*dt).
  //
  // That factor is NOT applied here — this pass is the undeformed forward leg.
  // It is applied once, to density only, after the correction and the limiter
  // in advectSclMac. See the ordering note there.
  textureStore(outF0, g, vec4f(max(s.x, 0.0), max(s.y, 0.0), 0.0, 0.0));
}

/**
 * MacCormack corrector. texA = phi^n, texB = velocity, texC = phi_hat.
 */
@compute @workgroup_size(4, 4, 4)
fn advectSclMac(@builtin(global_invocation_id) g: vec3u) {
  if (any(g >= FP.dims)) { return; }
  let c = vec3i(g);
  if (solidAt(c)) { textureStore(outF0, g, vec4f(0.0)); return; }
  let p = cellCentre(g);
  let t = traceB(c);
  let own = textureLoad(texA, c, 0).xy;
  let hat = textureLoad(texC, c, 0).xy;

  // Forward gather again, only for its corner extremes — the limiter's bounds.
  let fwd = gatherScalars(p - t.vel * FP.dt, own);

  // Backward through phi_hat. No volume factor on either leg: the whole
  // MacCormack correction happens in the UNDEFORMED frame, and the volume
  // change is applied once at the end.
  let tilde = sampleHatFluid(p + t.vel * FP.dt, hat);

  // Order is correct -> clamp -> volume, and it is not interchangeable.
  //
  // With G(rho) = rho(lambda x) + E, the reverse composition gives
  // G^R G rho = rho + 2E, so the correction -E cancels the error exactly and
  // V * [rho(lambda x) + E - E] = V * rho(lambda x) is exact. Splitting the
  // factor across the two legs instead leaves a residual (V - 1) * E — that is
  // anti-diffusion at up to 1/V = 7.4x its proper weight under a strong
  // expansion — and it quantises phi_hat at 0.135x magnitude in fp16 before
  // re-amplifying it 7.4x.
  //
  // Clamping AFTER the volume factor is the natural-looking mistake and it is
  // the worst one: the limiter's bounds come from the undeformed field, so an
  // expanding cell gets pushed back up to `lo` and the 20.21-vs-3.79 mass bug
  // returns through the limiter.
  let corrected = clamp(hat + (own - tilde) * 0.5, fwd.lo, fwd.hi);
  let vol = exp(-clamp(t.div * FP.dt, -MAX_DIV_DT, MAX_DIV_DT));
  // Density only. `vol` is the parcel's volume Jacobian, and it applies to
  // EXTENSIVE quantities — mass per unit volume changes when the volume does.
  // Temperature is intensive: DT/Dt = 0 under advection, so a parcel carries
  // its temperature into a bigger volume unchanged. Scaling .y here would have
  // an expanding source destroy its own buoyancy currency (expand = 70 at
  // 60 Hz is exp(-70/60) ~ 0.31 per step, so the flashbang's 30-unit core
  // temperature is gone in a handful of frames) — invisible in every recorded
  // fluid-mass phase because they all run with sources = 0 and hence vol = 1.
  var o = vec2f(corrected.x * vol, corrected.y);

  // Restriction: pull the fine lattice's result down into this coarse cell.
  //
  // Folded into this pass rather than given one of its own because a storage
  // texture cannot be read and written in the same shader, and this pass
  // already owns the write to scl[0]. texD is otherwise a dummy here.
  //
  // It is what makes the fine lattice REAL to everything downstream: the
  // interface texture, the coarse readback, smokeTransmittanceAbove and the
  // detection band all read the coarse field, so without this a player would
  // stand inside a dense canister cloud and register as unconcealed.
  if (FP.peerMode == PEER_COARSE) {
    let pw = cellCentre(g);
    let ext = vec3f(FP.peerDims) * FP.peerCell;
    let fu = (pw - FP.peerOrigin) / ext;
    if (all(fu > vec3f(0.0)) && all(fu < vec3f(1.0))) {
      // Taper only in the outermost cells, not across a tenth of the box.
      //
      // A wide ramp leaves the coarse field holding almost none of the fine
      // cloud near the boundary — and since the fine lattice's lateral faces
      // are open outflow, smoke crossing them is discarded. Between the two,
      // the cloud was being CLIPPED to the box: a hard square edge where it
      // should have carried on as coarse smoke. Restricting nearly to the edge
      // means whatever leaves the fine lattice has already been handed to the
      // coarse one, which then advects it onward.
      let e = min(min(fu.x, 1.0 - fu.x), min(fu.z, 1.0 - fu.z));
      let w = smoothstep(0.0, 0.02, e);
      if (w > 0.0) {
        // Box-average the fine cells inside this coarse cell. The lattices are
        // exact integer refinements, so this is a plain mean with no filtering.
        let r = vec3i(max(vec3f(1.0), round(FP.cell / FP.peerCell)));
        let base = vec3i(floor(fu * vec3f(FP.peerDims)));
        var acc = vec2f(0.0);
        var n = 0.0;
        for (var k = 0; k < r.z; k = k + 1) {
          for (var j = 0; j < r.y; j = j + 1) {
            for (var i = 0; i < r.x; i = i + 1) {
              let q = clamp(base + vec3i(i, j, k), vec3i(0), vec3i(FP.peerDims) - vec3i(1));
              acc = acc + textureLoad(texD, q, 0).xy;
              n = n + 1.0;
            }
          }
        }
        if (n > 0.0) { o = mix(o, acc / n, w); }
      }
    }
  }
  textureStore(outF0, g, vec4f(max(o.x, 0.0), max(o.y, 0.0), 0.0, 0.0));
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
