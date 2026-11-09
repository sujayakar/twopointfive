// The falling SHEET's dynamics — the CPU reference of what render/shaders/particles.wgsl `sheetAir` integrates for the P_SHEET particle kind (a leaf of
// office paper knocked off a desk by a burst: destruction.ts). One model, written twice: the shader flies the thousands you see, this file flies the same
// equations for the QA probes (settle time, no NaN, the flutter looks like flutter — tools/qa probes 'destruction') and is the place the numbers are argued.
// Keep the two in step: SHEET below = particles.ts KIND_DEFAULTS.sheet's slots (gravity · drag = KT · restitution = KN_LIN · keep = KN_Q · sleepV · fade · FLUT ·
// TRANS) + the two constants the kind block has no slot for (ANG_DAMP, SPIN_MAX), which particles_common.wgsl repeats.
//
// The model (phenomenological, sign-correct, cheap — Tanabe & Kaneko's falling-paper regimes without their lift polar):
//   state      p, v, unit normal n (two-sided: everything below is invariant under n → −n), angular velocity ω ⊥ n, in-plane spin φ (looks only)
//   gravity    v.y −= g·dt
//   drag       ANISOTROPIC: split v into the slip through the sheet vn = v·n and the slip along it vt = v − n·vn; the face is a plate — its normal force
//              grows with the whole airspeed × the through-slip (the flat plate's |v|²·sin α: a GLIDING sheet gets lift, not only a mushing one), + a little
//              linear: vn ← vn·e^−(KN_LIN + KN_Q·|v|)·dt, so face-down it reaches ≈ √(g / KN_Q) ≈ 1 m/s like an 80 g/m² sheet does — the edge is nothing,
//              vt ← vt·e^−KT·dt; so a tilted sheet GLIDES: the through-slip dies, the along-slip persists and gravity's in-plane share feeds it
//   flutter    the plate's centre of pressure sits ahead of its centre (toward where it is sliding), and the normal force acts there: torque
//              τ = r_cp × F = (c/4·v̂t) × (−k·vn·n)  →  angular acceleration  α = FLUT · (−vn) · (vt × n)  — bilinear in the two slips (∝ 'in-plane slip ×
//              normal slip'), and it pitches the LEADING EDGE UP: the glide steepens into a stall, the along-slip reverses down the new slope, the sheet swoops
//              back — the falling-leaf oscillation; a hard enough kick tumbles it end over end instead (both regimes are real). ω is damped (ANG_DAMP: air
//              resists a turning sheet strongly) and kept ⊥ n (spin about the normal is φ's, cosmetic); n turns by ω.
//   ground     (the shader: traceClosest against the level) an up-facing hit lays it toward flat, kills the through-slip, rubs the along-slip; slow enough and
//              it SLEEPS flat on the surface like a chip. Walls: slide down them. Here: a floor plane at y = 0 stands in for the level.
//   gust       (impulses, below) for a few tenths of a second after a blast has kicked it the AIR MOVES WITH THE LEAF: drag and flutter torque are taken
//              against a co-moving parcel, i.e. scaled by (1 − GUST_K·g), g = gust ÷ GUST_TAU falling to 0 — the leaf is CARRIED up a metre and let go
//              into still air at the top, where the flutter takes over. gust = 0 (every sheet no impulse has touched) = the equations exactly as they were.
import { Vec3, v3 } from '../math/vec';

/** the sheet kind's numbers (particles.ts KIND_DEFAULTS.sheet mirrors the first eight; the shader reads them from the kind block) */
export const SHEET = {
  gravity: 9.8,
  KT: 0.5,        // along-the-sheet (skin) drag, 1/s
  KN_LIN: 1.5,    // through-the-sheet drag, linear part, 1/s
  KN_Q: 9.0,      // …× airspeed part, 1/m: terminal face-down ≈ √(g / KN_Q) ≈ 1 m/s
  sleepV: 0.28,   // m/s: slower than this on an up face after a touch → lies down and sleeps
  fade: 4.0,      // s of fade at the end of life
  FLUT: 25.0,     // flutter torque gain (rad/s² per (m/s)²): a sheet let go tilted at chest height rocks two or three times and lands ~half a metre off (sheetModel sweep)
  TRANS: 0.3,     // translucency: the share of the light on the BACK face that shows through (a sheet between the eye and a torch glows)
  ANG_DAMP: 1.8,  // 1/s on ω
  SPIN_MAX: 3.0,  // rad/s: in-plane spin dealt per sheet off its seed (× how fast it moves; nothing at rest)
  life: 60,       // s a sheet lies there before it fades (the LONG ring recycles the oldest before that under load)
};

export interface SheetState { p: Vec3; v: Vec3; n: Vec3; w: Vec3; asleep: boolean; age: number; /** s of blast wind still carrying it (impulses; absent / 0 = still air) */ gust?: number; }




// ================================================================== IMPULSES (render/particles.ts Particles.impulse → particles.wgsl applyImpulses; sites in
// game/destruction.ts): a game event — a stun charge going off, a round landing beside settled litter, a body or a boot or a shoved desk thumping the floor —
// pushes every live particle of the kinds it names within its radius, and WAKES the ones lying asleep so paper already on the floor goes up again. One record
// = { pos, radius, strength = the peak Δv (m/s) a SHEET gets at the centre, up = the upward share added to the radial push (leaves LIFT, then flutter down),
// bias = a direction folded in (the round's travel along the surface it struck), kinds, seed, an optional occlusion eye }. The push on a particle at distance
// d is Δv = strength × gain(kind) × (1 − d / radius) × (radial‖ + up·upK(kind)·ŷ + bias) — radial with its downward part dropped (a charge on the floor
// pushes the floor's litter out along the floor, never into it), the sum NOT renormalised (up and bias add to a unit radial); |v| is clamped per kind after
// the kick and a resting particle is only woken by a kick of at least wake(kind). This table is the kernel's third row per kind (particles.ts packs it into
// the uniform block, so the panel edits it live); the sheet's SHAPE response (the fresh tilt and tumble that put it back into the flutter model) is the two
// constants under it, which particles_common.wgsl repeats. The probes fly the reference below; the kernel flies what you see.
/** falloff of an impulse with distance: linear, 1 − d/r inside the radius, 0 outside — leaves within half the radius fly, ones near the edge just shiver */
export function impulseFalloff(d: number, r: number): number { if (!(d < r) || r <= 0) return 0; return 1 - Math.max(0, d) / r; }
/** below this scalar kick (m/s) an impulse does nothing to a particle at all (the far fringe: not worth the wake, the ray or the write) */
export const IMP_MIN_KICK = 0.03;
/** per-kind response: gain on the record's strength · multiplier on its upward share · |v| cap after the kick (m/s) · the least kick that wakes a resting one (m/s) */
export interface ImpulseResponse { gain: number; upK: number; vMax: number; wake: number; }
export const IMP_RESPONSE = {
  spark:  { gain: 0,    upK: 0,   vMax: 0,   wake: 0 },      // ignored: a spark is gone before anything could push it, and a pushed streak would read as a second ricochet
  debris: { gain: 0.4,  upK: 0.6, vMax: 4.0, wake: 0.25 },   // chips / splinters / crumbs: a moderate hop and skitter, another landing angle
  dust:   { gain: 0.7,  upK: 0.3, vMax: 3.0, wake: 0 },      // a puff is shoved along by the air (it never sleeps)
  casing: { gain: 0.3,  upK: 0.5, vMax: 3.5, wake: 0.3 },    // brass is heavy for its size: a short hop, a roll, the twinkle of it turning
  sheet:  { gain: 1.0,  upK: 1.0, vMax: 6.0, wake: 0.12 },   // THE reference kind: strength is quoted as a sheet's Δv at the centre
  rain:   { gain: 0,    upK: 0,   vMax: 0,   wake: 0 },      // ignored like the spark: a rain drop (game/rainFx.ts) is gone before a push could show, and the impulses leave its ring alone
} satisfies Record<string, ImpulseResponse>;
export interface ImpulseLike { pos: Vec3; radius: number; strength: number; up: number; bias: Vec3; }
/** per-particle raggedness of a push (a blast front is not smooth: two leaves a hand apart do not get the same shove — without it a pile lifts as one clump):
 *  the kick × (1 − MAG/2 ‥ 1 + MAG/2) off one hash, plus SIDE × the kick sideways along a hashed bearing */
export const IMP_JITTER = { MAG: 0.6, SIDE: 0.35 };
/** The push one impulse gives a particle of response `G` at `p`: the velocity change and the scalar kick s (strength × gain × falloff), or null when it does
 *  nothing (outside the radius, under IMP_MIN_KICK, a kind that ignores impulses). `j` = the particle's two hashes for this record (magnitude, bearing) —
 *  omitted = the smooth mean push (the sweeps); the kernel always has them. Mirrors the per-impulse body of particles.wgsl applyImpulses. */
export function impulseKick(imp: ImpulseLike, p: Vec3, G: ImpulseResponse, j?: [number, number]): { dv: Vec3; s: number } | null {
  if (G.gain <= 0) return null;
  const to = v3.sub(p, imp.pos); const d = v3.len(to);
  let s = imp.strength * G.gain * impulseFalloff(d, imp.radius);
  if (s < IMP_MIN_KICK) return null;
  const radial: Vec3 = d < 0.02 ? [0, 1, 0] : [to[0] / d, Math.max(0, to[1] / d), to[2] / d];
  const dir: Vec3 = [radial[0] + imp.bias[0], radial[1] + imp.up * G.upK + imp.bias[1], radial[2] + imp.bias[2]];
  if (!j) return { dv: v3.scale(dir, s), s };
  s *= 1 - IMP_JITTER.MAG / 2 + IMP_JITTER.MAG * j[0]; const a = j[1] * 6.2831853;
  return { dv: [dir[0] * s + Math.cos(a) * IMP_JITTER.SIDE * s, dir[1] * s, dir[2] * s + Math.sin(a) * IMP_JITTER.SIDE * s], s };
}
