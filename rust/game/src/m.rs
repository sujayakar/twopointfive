//! THE GAME'S ARITHMETIC, by the names the TypeScript calls it (`Math.sin` → `m::sin`). The rounded functions are the game's own (rust/sim
//! maths.rs: the same bits on every machine, the bits the TypeScript game gets from src/math/exact.ts); √, floor and trunc are exact by the
//! standard (rust/prim). Nothing here is the standard library's float method, and a translated file names none either.
pub use prim::{floor, sqrt, trunc};
pub use sim::maths::{acos, asin, atan, atan2, cos, exp, log, log10, log2, pow, sin, tan, tanh};
/// `Math.hypot` of two, three and four numbers (src/math/hyp.ts hyp2, hyp3, hyp4)
pub use sim::maths::{hypot2 as hyp2, hypot3 as hyp3, hypot4 as hyp4, hypot_n as hyp_n};

pub const PI: f64 = core::f64::consts::PI;
pub const INF: f64 = f64::INFINITY;
pub const NAN: f64 = f64::NAN;

/// `Math.abs`: −0 → 0, NaN → NaN
#[inline]
pub fn abs(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & !(1u64 << 63))
}
/// `Math.min(a, b)`: NaN if either is NaN; −0 is below +0
#[inline]
pub fn min(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        return NAN;
    }
    if a == 0.0 && b == 0.0 {
        return if a.to_bits() >> 63 == 1 { a } else { b };
    }
    if a < b { a } else { b }
}
/// `Math.max(a, b)`: NaN if either is NaN; +0 is above −0
#[inline]
pub fn max(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        return NAN;
    }
    if a == 0.0 && b == 0.0 {
        return if a.to_bits() >> 63 == 0 { a } else { b };
    }
    if a > b { a } else { b }
}
/// `Math.min(...)` of any count (none → +∞)
pub fn min_n(v: &[f64]) -> f64 {
    let mut r = INF;
    for &x in v {
        r = min(r, x);
    }
    r
}
/// `Math.max(...)` of any count (none → −∞)
pub fn max_n(v: &[f64]) -> f64 {
    let mut r = -INF;
    for &x in v {
        r = max(r, x);
    }
    r
}
/// `Math.sign`: ±1, and 0, −0 and NaN as they are
#[inline]
pub fn sign(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
}
/// `Math.round`: the nearest whole number, a half going up (−0.5 → −0)
#[inline]
pub fn round(x: f64) -> f64 {
    if x != x || x == INF || x == -INF {
        return x;
    }
    sim::maths::round_half_up(x)
}
/// `Math.ceil`
#[inline]
pub fn ceil(x: f64) -> f64 {
    let f = floor(x);
    if f == x || x != x { x } else if f == -1.0 && x > -1.0 { -0.0 } else { f + 1.0 }
}
/// `Math.fround`: the nearest single, as a double (the one place a single is named: it is what the TypeScript asks for)
#[inline]
pub fn fround(x: f64) -> f64 {
    (x as f32) as f64
}
/// `Math.imul`
#[inline]
pub fn imul(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}
/// `Math.clz32`
#[inline]
pub fn clz32(x: u32) -> u32 {
    x.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_language_s_edges() {
        assert!(min(1.0, NAN).is_nan() && max(NAN, 1.0).is_nan());
        assert_eq!(min(0.0, -0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(max(-0.0, 0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(min_n(&[]), INF);
        assert_eq!(max_n(&[]), -INF);
        assert_eq!(round(2.5), 3.0);
        assert_eq!(round(-2.5), -2.0);
        assert_eq!(round(-0.4).to_bits(), (-0.0f64).to_bits());
        assert_eq!(round(0.49999999999999994), 0.0);
        assert_eq!(ceil(1.2), 2.0);
        assert_eq!(ceil(-1.2), -1.0);
        assert_eq!(ceil(-0.3).to_bits(), (-0.0f64).to_bits());
        assert_eq!(ceil(3.0), 3.0);
        assert_eq!(sign(-0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(abs(-0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(imul(0x7fffffff, 2), -2);
    }
}
