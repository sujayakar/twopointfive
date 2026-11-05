//! src/math/hyp.ts lines 1 to 140
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// V8's two-term length: scaled by the largest, squares Kahan-summed. (hyp.ts kahan2)
pub fn kahan2(a: f64, b: f64) -> f64 {
    let (x, y) = (m::abs(a), m::abs(b));
    let mut max = 0.0;
    if x > max { max = x; }
    if y > max { max = y; }
    if max == INF { return INF; }
    if x != x || y != y { return f64::NAN; }
    if max == 0.0 { return 0.0; }
    let mut n = x / max;
    let mut s = n * n - 0.0;
    let mut p = 0.0 + s;
    let mut comp = (p - 0.0) - s;
    let mut sum = p;
    n = y / max; s = n * n - comp; p = sum + s; comp = (p - sum) - s; sum = p;
    m::sqrt(sum) * max
}

/// V8's three-term length. (hyp.ts kahan3)
pub fn kahan3(a: f64, b: f64, c_2: f64) -> f64 {
    let (x, y, z) = (m::abs(a), m::abs(b), m::abs(c_2));
    let mut max = 0.0;
    if x > max { max = x; }
    if y > max { max = y; }
    if z > max { max = z; }
    if max == INF { return INF; }
    if x != x || y != y || z != z { return f64::NAN; }
    if max == 0.0 { return 0.0; }
    let mut n = x / max;
    let mut s = n * n - 0.0;
    let mut p = 0.0 + s;
    let mut comp = (p - 0.0) - s;
    let mut sum = p;
    n = y / max; s = n * n - comp; p = sum + s; comp = (p - sum) - s; sum = p;
    n = z / max; s = n * n - comp; p = sum + s; comp = (p - sum) - s; sum = p;
    m::sqrt(sum) * max
}

/// V8's four-term length. (hyp.ts kahan4)
pub fn kahan4(a: f64, b: f64, c_2: f64, d: f64) -> f64 {
    let (x, y, z, w) = (m::abs(a), m::abs(b), m::abs(c_2), m::abs(d));
    let mut max = 0.0;
    if x > max { max = x; }
    if y > max { max = y; }
    if z > max { max = z; }
    if w > max { max = w; }
    if max == INF { return INF; }
    if x != x || y != y || z != z || w != w { return f64::NAN; }
    if max == 0.0 { return 0.0; }
    let mut n = x / max;
    let mut s = n * n - 0.0;
    let mut p = 0.0 + s;
    let mut comp = (p - 0.0) - s;
    let mut sum = p;
    n = y / max; s = n * n - comp; p = sum + s; comp = (p - sum) - s; sum = p;
    n = z / max; s = n * n - comp; p = sum + s; comp = (p - sum) - s; sum = p;
    n = w / max; s = n * n - comp; p = sum + s; comp = (p - sum) - s; sum = p;
    m::sqrt(sum) * max
}

/// JavaScriptCore's two-term length: scaled by the largest, squares summed in order. (hyp.ts scaled2)
pub fn scaled2(a: f64, b: f64) -> f64 {
    let (x, y) = (m::abs(a), m::abs(b));
    if x == INF || y == INF { return INF; }
    if x != x || y != y { return f64::NAN; }
    let mut max = 0.0;
    if x > max { max = x; }
    if y > max { max = y; }
    if max == 0.0 { max = 1.0; }
    let (p, q) = (a / max, b / max);
    m::sqrt(0.0 + p * p + q * q) * max
}

/// JavaScriptCore's three-term length. (hyp.ts scaled3)
pub fn scaled3(a: f64, b: f64, c_2: f64) -> f64 {
    let (x, y, z) = (m::abs(a), m::abs(b), m::abs(c_2));
    if x == INF || y == INF || z == INF { return INF; }
    if x != x || y != y || z != z { return f64::NAN; }
    let mut max = 0.0;
    if x > max { max = x; }
    if y > max { max = y; }
    if z > max { max = z; }
    if max == 0.0 { max = 1.0; }
    let (p, q, r) = (a / max, b / max, c_2 / max);
    m::sqrt(0.0 + p * p + q * q + r * r) * max
}

/// JavaScriptCore's four-term length. (hyp.ts scaled4)
pub fn scaled4(a: f64, b: f64, c_2: f64, d: f64) -> f64 {
    let (x, y, z, w) = (m::abs(a), m::abs(b), m::abs(c_2), m::abs(d));
    if x == INF || y == INF || z == INF || w == INF { return INF; }
    if x != x || y != y || z != z || w != w { return f64::NAN; }
    let mut max = 0.0;
    if x > max { max = x; }
    if y > max { max = y; }
    if z > max { max = z; }
    if w > max { max = w; }
    if max == 0.0 { max = 1.0; }
    let (p, q, r, t) = (a / max, b / max, c_2 / max, d / max);
    m::sqrt(0.0 + p * p + q * q + r * r + t * t) * max
}

/// `Math.hypot(a, b)`, which in the game is the game's own. (hyp.ts native2)
pub fn native2(a: f64, b: f64) -> f64 {
    m::hyp2(a, b)
}

/// `Math.hypot(a, b, c)`. (hyp.ts native3)
pub fn native3(a: f64, b: f64, c_2: f64) -> f64 {
    m::hyp3(a, b, c_2)
}

/// `Math.hypot(a, b, c, d)`. (hyp.ts native4)
pub fn native4(a: f64, b: f64, c_2: f64, d: f64) -> f64 {
    m::hyp4(a, b, c_2, d)
}

/// The stream's next draw in [0, 1): the closure `r` of `sample`. (hyp.ts:82)
fn sample_r(s: &mut u32) -> f64 {
    *s = s.wrapping_add(0x6D2B79F5);
    let mut t = *s;
    t = m::imul((t ^ (t >> 15)) as i32, (t | 1) as i32) as u32;
    // `t + imul(…)` is a double in JavaScript, then ToInt32 by the `^=`: the same bits as a wrapping sum.
    t ^= t.wrapping_add(m::imul((t ^ (t >> 7)) as i32, (t | 61) as i32) as u32);
    (t ^ (t >> 14)) as f64 / 4294967296.0
}

/// The trial's inputs: a fixed stream over the magnitudes lengths take and the ones they do not. (hyp.ts sample)
pub fn sample(n: f64) -> Vec<f64> {
    let mut s: u32 = 0x9e3779b9;
    let odd: [f64; 19] = [
        0.0, -0.0, 1.0, -1.0, 0.5, 3.0, 4.0, 1e-320, -5e-324, 1.7976931348623157e308, 1e200, -1e-200,
        4503599627370497.0, // 2 ** 52 + 1
        7.450580596923828125e-9, // 2 ** -27
        f64::NAN, INF, -INF, 1e154, 1.5e-154,
    ];
    let mut out = vec![0.0; n as usize];
    for i in 0..out.len() {
        let k = sample_r(&mut s);
        out[i] = if k < 0.35 {
            (sample_r(&mut s) - 0.5) * 2.0
        } else if k < 0.6 {
            (sample_r(&mut s) - 0.5) * 60.0
        } else if k < 0.72 {
            (sample_r(&mut s) - 0.5) * 2e-3
        } else if k < 0.8 {
            let scale = sample_r(&mut s) - 0.5;
            scale * m::pow(10.0, m::floor(sample_r(&mut s) * 40.0 - 20.0))
        } else if k < 0.86 {
            let scale = sample_r(&mut s) - 0.5;
            scale * m::pow(2.0, m::floor(sample_r(&mut s) * 2000.0 - 1000.0))
        } else if k < 0.92 {
            m::round((sample_r(&mut s) - 0.5) * 16.0) / 4.0
        } else {
            odd[m::floor(sample_r(&mut s) * odd.len() as f64) as usize]
        };
    }
    out
}

/// The same number, zeros by sign; NaN is NaN. (hyp.ts same)
pub fn same(a: f64, b: f64) -> bool {
    if a == b { a != 0.0 || 1.0 / a == 1.0 / b } else { a != a && b != b }
}

/// Holds a candidate to `Math.hypot` over the trial's inputs. (hyp.ts agrees)
pub fn agrees(f: &mut dyn FnMut(Vec<f64>) -> f64, arity: f64) -> bool {
    let v = sample(TRIALS + 3.0);
    for i in 0..TRIALS as usize {
        let (a, b, c, d) = (v[i], v[i + 1], v[i + 2], v[i + 3]);
        let differs = if arity == 2.0 {
            !same(f(vec![a, b]), m::hyp2(a, b))
        } else if arity == 3.0 {
            !same(f(vec![a, b, c]), m::hyp3(a, b, c))
        } else {
            !same(f(vec![a, b, c, d]), m::hyp4(a, b, c, d))
        };
        if differs { return false; }
    }
    true
}

/// The game's own maths on or off for lengths: in Rust `crate::m::hyp2/3/4` are always the Kahan sums, so there is nothing to install. (hyp.ts hypExact)
pub fn hyp_exact(on: bool) {
    if held() { return; }
}

/// The Kahan sum for any number of terms: what `Math.hypot` becomes under the game's own maths. (hyp.ts kahanN)
pub fn kahan_n(v: &[f64]) -> f64 {
    let n = v.len();
    if n == 2 { return kahan2(v[0], v[1]); }
    if n == 3 { return kahan3(v[0], v[1], v[2]); }
    if n == 4 { return kahan4(v[0], v[1], v[2], v[3]); }
    let mut max = 0.0;
    let mut nan = false;
    for i in 0..n {
        let x = m::abs(v[i]);
        if x != x { nan = true; } else if x > max { max = x; }
    }
    if max == INF { return INF; }
    if nan { return f64::NAN; }
    if max == 0.0 { return 0.0; }
    let (mut sum, mut comp) = (0.0, 0.0);
    for i in 0..n {
        let k = m::abs(v[i]) / max;
        let s = k * k - comp;
        let p = sum + s;
        comp = (p - sum) - s;
        sum = p;
    }
    m::sqrt(sum) * max
}

#[cfg(test)]
mod tests {
    use super::*;

    const H0: u64 = 0xcbf29ce484222325;
    /// FNV-1a over a double's bits, every NaN one NaN: the same fold as the bun script that made the hashes below.
    fn fold(h: u64, x: f64) -> u64 {
        let bits = if x != x { 0x7ff8000000000000 } else { x.to_bits() };
        (h ^ bits).wrapping_mul(0x100000001b3)
    }
    fn over(v: &[f64], f: &dyn Fn(&[f64]) -> f64) -> u64 {
        let mut h = H0;
        for i in 0..39990 { h = fold(h, f(&v[i..i + 5])); }
        h
    }

    // Hashes from src/math/hyp.ts itself under bun, on its own sample(40003), 39990 calls an arity.
    #[test]
    fn the_stream_is_the_typescripts() {
        let v = sample(40003.0);
        let mut h = H0;
        for x in &v { h = fold(h, *x); }
        assert_eq!(h, 0x855c75d12f41270b);
    }

    #[test]
    fn the_sums_are_the_typescripts() {
        let v = sample(40003.0);
        assert_eq!(over(&v, &|w| kahan2(w[0], w[1])), 0x2cd3f26b18fd1d70);
        assert_eq!(over(&v, &|w| kahan3(w[0], w[1], w[2])), 0xcf92d5ad086b236a);
        assert_eq!(over(&v, &|w| kahan4(w[0], w[1], w[2], w[3])), 0x547b692cb80efd4e);
        assert_eq!(over(&v, &|w| scaled2(w[0], w[1])), 0x2cd3f26b18fd1d70);
        assert_eq!(over(&v, &|w| scaled3(w[0], w[1], w[2])), 0xabd8fdb2b4b43444);
        assert_eq!(over(&v, &|w| scaled4(w[0], w[1], w[2], w[3])), 0xc86bfe2b1936fe35);
        assert_eq!(over(&v, &|w| kahan_n(&w[..1])), 0x0ca3a9050b1453e4);
        assert_eq!(over(&v, &|w| kahan_n(w)), 0xc38e92a66b1d5bea);
        assert_eq!(over(&v, &|_| kahan_n(&[])), 0xcb2837eb9e06a6fd);
        assert_eq!(over(&v, &|w| if same(w[0], w[1]) { 1.0 } else { 0.0 }), 0xee1837eb9e06a6fd);
    }

    /// `crate::m`'s lengths (rust/sim's) are this file's Kahan sums: the fact `hyp_exact` having nothing to do rests on.
    #[test]
    fn the_games_lengths_are_these_sums() {
        assert!(agrees(&mut |a| kahan2(a[0], a[1]), 2.0));
        assert!(agrees(&mut |a| kahan3(a[0], a[1], a[2]), 3.0));
        assert!(agrees(&mut |a| kahan4(a[0], a[1], a[2], a[3]), 4.0));
        assert!(!agrees(&mut |a| scaled3(a[0], a[1], a[2]), 3.0));
    }
}
