pub use ::game_types::sheet_model::types;
pub use types::*;
use crate::m;
use crate::v3::{self, V3};

/// Linear falloff of an impulse: 1 − d/r inside the radius, 0 outside.
fn impulse_falloff(d: f64, r: f64) -> f64 {
    if !(d < r) || r <= 0.0 { return 0.0; }
    1.0 - m::max(0.0, d) / r
}

/// The push one impulse gives a thing of response `g_2` at `p`: Δv and the scalar kick, or None when it does nothing.
pub(super) fn impulse_kick(imp: crate::sheet_model::ImpulseLike, p: V3, g_2: crate::sheet_model::ImpulseResponse, j: Option<[f64; 2]>) -> Option<crate::sheet_model::ImpulseKick> {
    if g_2.gain <= 0.0 { return None; }
    let to = v3::sub(p, imp.pos);
    let d = v3::len(to);
    let mut s = imp.strength * g_2.gain * impulse_falloff(d, imp.radius);
    if s < crate::sheet_model::IMP_MIN_KICK { return None; }
    let radial: V3 = if d < 0.02 { [0.0, 1.0, 0.0] } else { [to[0] / d, m::max(0.0, to[1] / d), to[2] / d] };
    let dir: V3 = [radial[0] + imp.bias[0], radial[1] + imp.up * g_2.up_k + imp.bias[1], radial[2] + imp.bias[2]];
    let j = match j {
        Some(j) => j,
        None => return Some(crate::sheet_model::ImpulseKick { dv: v3::scale(dir, s), s }),
    };
    let jit = crate::sheet_model::imp_jitter();
    s *= 1.0 - jit.mag / 2.0 + jit.mag * j[0];
    let a = j[1] * 6.2831853;
    Some(crate::sheet_model::ImpulseKick {
        dv: [dir[0] * s + m::cos(a) * jit.side * s, dir[1] * s, dir[2] * s + m::sin(a) * jit.side * s],
        s,
    })
}
