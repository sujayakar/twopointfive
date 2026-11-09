// A glass SHARD's dynamics — the CPU reference of what render/shaders/particles.wgsl integrates for the P_SHARD particle kind (a fragment of a pane that has let
// go: rust/game_rules/src/shards files them when the fracture model releases a piece). One model, written twice, like the falling sheet (render/sheetModel.ts): the shader
// flies the thousands you see, this file flies the same equations
// and is the place the numbers are argued. Keep the two in step: SHARD below = particles.ts KIND_DEFAULTS.shard's
// eight slots (gravity · drag · restitution · keep · sleepV · fade · ROUGH · SCATTER) + the constants the kind block has no slot for, which
// particles_common.wgsl repeats as SHARD_*.
// What is shared exactly: the equations, every constant, the hash FUNCTION and the emit kernel's draws (sprayShards below = the same seeds, streams and cone). What is
// not: the per-touch draws are keyed here by the touch count and in the kernel by the particle's age bits — so this file predicts the DISTRIBUTION the GPU flies
// (reach, hops, time to rest, rest tilts), not any one fragment's path.
//
// The model (a fragment of float glass is DENSE and STIFF: it does not flutter, it tumbles and it bounces):
//   state      p, v, unit normal n (two-sided), angular velocity ω (any axis: a shard spins about its normal too — that part only turns the outline), spin φ
//   flight     gravity; drag e^(−k·dt) with k = drag × REF_SIZE ÷ size capped at DRAG_MAX — at the reference centimetre a fragment keeps its speed across a
//              room, a 3 mm fine is stopped by the air inside a metre or two (area ÷ mass ∝ 1 ÷ size: the fines hang in a cloud near the pane, the big pieces
//              carry); ω barely damped (ANG_DAMP), n turned by ω
//   floor      (the shader: traceClosest against the level; here a floor plane) the normal speed comes back × restitution × (1 − E_JIT/2 ‥ 1 + E_JIT/2) hashed per
//              touch — an irregular piece lands on a corner one time and its flat the next, so no two hops are the same height; the slide keeps `keep` of
//              itself and is DEFLECTED by up to ±SKITTER rad to a hashed side (the corner it struck throws it sideways: that is the skitter); ω is dealt
//              afresh ∝ the speed it leaves at (SPIN_K rad/s per m/s, ≤ SPIN_MAX) about a hashed in-plane axis
//   wall       comes off at WALL_E of its normal speed, WALL_KEEP of its slide, half its tumble
//   rest       slower than sleepV after a floor touch → it lies down and SLEEPS: its normal goes to the surface's tilted by REST_TILT × h² toward a hashed side
//              (most lie flat, a few are propped on a neighbour or their own bevel) — so a field of litter mirrors a light at many slightly different angles and
//              only some of it glints at once, and which ones changes as the eye or the torch moves
import type { ImpulseResponse } from './sheetModel';

/** the shard kind's numbers (particles.ts KIND_DEFAULTS.shard mirrors the first eight; particles_common.wgsl repeats the rest as SHARD_<NAME>) */
export const SHARD = {
  gravity: 9.8,
  drag: 0.5,          // 1/s on a fragment REF_SIZE across (× REF_SIZE ÷ size, ≤ DRAG_MAX)
  restitution: 0.4,   // float glass on a hard floor (the mean; each touch draws its own)
  keep: 0.78,         // of the slide kept on a floor touch
  sleepV: 0.3,        // m/s
  fade: 5.0,          // s of fade at the end of life
  ROUGH: 0.07,        // the glint lobe's width (GGX α) of a clean fracture face, + the emitter's angular size
  SCATTER: 0.1,       // share of the irradiance on it the body and the cut edges send on diffusely (the green of the edge, the frost of the bruised face)
  REF_SIZE: 0.01,
  DRAG_MAX: 6.0,
  ANG_DAMP: 0.3,      // 1/s on ω in flight
  E_JIT: 0.8,         // spread of the per-touch restitution (× 0.6 ‥ 1.4)
  SKITTER: 0.45,      // rad: the most a floor touch deflects the slide
  SPIN_K: 28.0,       // rad/s of tumble dealt per m/s of speed after a touch
  SPIN_MAX: 60.0,     // rad/s
  WALL_E: 0.3,
  WALL_KEEP: 0.8,
  REST_TILT: 0.35,    // rad: the most a resting shard's normal is off the surface's (× h²)
  GLINT_CAP: 14.0,    // per-light ceiling on the mirror lobe × Fresnel (a fragment is a little mirror: the cap is what stops one aligned sliver being a lamp)
  F0: 0.04,           // glass
  life: 60,           // s a mid-sized fragment lies there (rust/game_rules/src/shards deals each class its own)
};
/** what an impulse (a stun charge, a round beside it, a boot) does to glass litter: lighter than brass, it hops and skitters on (particles.ts impulseResponse.shard) */
export const SHARD_IMP: ImpulseResponse = { gain: 0.45, upK: 0.5, vMax: 4.5, wake: 0.25 };

// the kernel's hash (particles.wgsl pcg / rnd), bit for bit: integer ops only
export function pcg(v: number): number { const s = (Math.imul(v, 747796405) + 2891336453) >>> 0; const w = Math.imul(((s >>> ((s >>> 28) + 4)) ^ s) >>> 0, 277803737) >>> 0; return ((w >>> 22) ^ w) >>> 0; }
export function rnd(seed: number, k: number): number { return (pcg((seed ^ Math.imul(k, 0x9E3779B9)) >>> 0) >>> 8) / 16777216; }
