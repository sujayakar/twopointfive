//! THE GAME'S OWN MATHS — sin, cos, tan, asin, acos, atan, atan2, exp, log, log2, log10, pow, tanh and hypot from + − × ÷ and √ alone.
//!
//! This is src/math/exact.ts (and the Kahan sums of src/math/hyp.ts), statement for statement: the same operations in the same order on the same doubles, so
//! the same bits. The TypeScript file says why the game has maths of its own and where the algorithms come from (the Go standard library's portable math
//! package, after Cephes and FreeBSD's msun; their notices are in that file and cover this one). tools/qa/rust-check.ts folds one stream of inputs through both
//! and is red on a bit of difference.
//!
//! Do not tidy the arithmetic: `(a - b) - c` is not `a - (b + c)`, a polynomial written by Horner's rule stays one, and a term that looks like it cancels
//! (`n * n - 0.0`) is there because the other side has it.

use crate::pow10::{POW10, POW10_MIN};
use prim::{floor, sqrt, trunc};

const INF: f64 = f64::INFINITY;
const PI: f64 = 3.141592653589793;
const PI2: f64 = PI / 2.0;
const PI4: f64 = PI / 4.0;
const SQRT1_2: f64 = 0.7071067811865476;
const TWO52: f64 = 4503599627370496.0;
const MIN_NORMAL: f64 = 2.2250738585072014e-308;

// ---- the pieces of a double
/// f = frac × 2^exp with ½ ≤ |frac| < 1 (0, the infinities and NaN come back as they are, exp 0)
#[inline]
fn frexp(mut f: f64) -> (f64, i64) {
    if f == 0.0 || f != f || f == INF || f == -INF {
        return (f, 0);
    }
    let mut e: i64 = 0;
    if f.abs() < MIN_NORMAL {
        f *= TWO52;
        e = -52;
    }
    let bits = f.to_bits();
    let hi = (bits >> 32) as u32;
    let exp = e + ((hi >> 20) & 0x7ff) as i64 - 1022;
    let hi2 = (hi & 0x800f_ffff) | 0x3fe0_0000;
    (f64::from_bits(((hi2 as u64) << 32) | (bits & 0xffff_ffff)), exp)
}
/// frac × 2^exp
#[inline]
fn ldexp(mut frac: f64, mut exp: i64) -> f64 {
    if frac == 0.0 || frac != frac || frac == INF || frac == -INF {
        return frac;
    }
    if frac.abs() < MIN_NORMAL {
        frac *= TWO52;
        exp -= 52;
    }
    let bits = frac.to_bits();
    let hi = (bits >> 32) as u32;
    exp += ((hi >> 20) & 0x7ff) as i64 - 1023;
    if exp < -1075 {
        return if frac < 0.0 { -0.0 } else { 0.0 };
    }
    if exp > 1023 {
        return if frac < 0.0 { -INF } else { INF };
    }
    let mut m = 1.0;
    if exp < -1022 {
        exp += 53;
        m = 1.0 / 9007199254740992.0;
    }
    let hi2 = (hi & 0x800f_ffff) | (((exp + 1023) as u32) << 20);
    m * f64::from_bits(((hi2 as u64) << 32) | (bits & 0xffff_ffff))
}

// ---- sine, cosine, tangent: reduced into eighths of a turn by a π/4 held in three pieces, then one of two polynomials
const SIN0: f64 = 1.58962301576546568060e-10;
const SIN1: f64 = -2.50507477628578072866e-8;
const SIN2: f64 = 2.75573136213857245213e-6;
const SIN3: f64 = -1.98412698295895385996e-4;
const SIN4: f64 = 8.33333333332211858878e-3;
const SIN5: f64 = -1.66666666666666307295e-1;
const COS0: f64 = -1.13585365213876817300e-11;
const COS1: f64 = 2.08757008419747316778e-9;
const COS2: f64 = -2.75573141792967388112e-7;
const COS3: f64 = 2.48015872888517045348e-5;
const COS4: f64 = -1.38888888888730564116e-3;
const COS5: f64 = 4.16666666666665929218e-2;
const PI4A: f64 = 7.85398125648498535156e-1;
const PI4B: f64 = 3.77489470793079817668e-8;
const PI4C: f64 = 2.69515142907905952645e-15;
const FOPI: f64 = 1.2732395447351628; // 4/π, rounded once
const REDUCE: f64 = 536870912.0; // 2^29: from here up the reduction is done on the bits (trig_reduce)

#[inline]
fn sin_poly(z: f64, zz: f64) -> f64 {
    z + z * zz * ((((((SIN0 * zz) + SIN1) * zz + SIN2) * zz + SIN3) * zz + SIN4) * zz + SIN5)
}
#[inline]
fn cos_poly(zz: f64) -> f64 {
    1.0 - 0.5 * zz + zz * zz * ((((((COS0 * zz) + COS1) * zz + COS2) * zz + COS3) * zz + COS4) * zz + COS5)
}

// 4/π to 1280 bits, sixty-four at a time (the first word is the whole part)
const MPI4: [u64; 20] = [
    0x0000000000000001, 0x45f306dc9c882a53, 0xf84eafa3ea69bb81, 0xb6c52b3278872083, 0xfca2c757bd778ac3, 0x6e48dc74849ba5c0, 0x0c925dd413a32439,
    0xfc3bd63962534e7d, 0xd1046bea5d768909, 0xd338e04d68befc82, 0x7323ac7306a673e9, 0x3908bf177bf25076, 0x3ff12fffbc0b301f, 0xde5e2316b414da3e,
    0xda6cfd9e4f96136e, 0x9e8c7ecd3cbfd45a, 0xea4f758fd7cbe2f6, 0x7a0e73ef14a525d4, 0xd7f6bf623f1aba10, 0xac06608df8f6d757,
];
const M64: u128 = 0xffff_ffff_ffff_ffff;
/// x ≥ 2^29 reduced by multiples of π/4 (Payne and Hanek: the bits of x against the bits of 4/π): the remainder, and the eighth of the turn.
/// (The TypeScript does this on unbounded integers; every width below is the one its masks leave.)
fn trig_reduce(x: f64) -> (f64, i64) {
    let bits = x.to_bits();
    let hw = (bits >> 32) as u32;
    let lw = bits as u32;
    let exp = ((hw >> 20) & 0x7ff) as i64 - 1023 - 52;
    let ix: u128 = ((((hw & 0xf_ffff) | 0x10_0000) as u128) << 32) | lw as u128;
    let digit = ((exp + 61) / 64) as usize; // (exp + 61 ≥ 38 here: the quotient is the floor)
    let bs = ((exp + 61) % 64) as u32;
    let inv = 64 - bs;
    let word = |k: usize| -> u128 { ((MPI4[k] as u128) << bs | (MPI4[k + 1] as u128) >> inv) & M64 };
    let (z0, z1, z2) = (word(digit), word(digit + 1), word(digit + 2));
    let z2hi = (z2 * ix) >> 64;
    let z1p = z1 * ix;
    let z1hi = z1p >> 64;
    let z1lo = z1p & M64;
    let z0lo = (z0 * ix) & M64;
    let s = z1lo + z2hi;
    let lo = s & M64;
    let c = s >> 64;
    let mut hi = (z0lo + z1hi + c) & M64;
    let mut j = (hi >> 61) as i64;
    hi = ((hi << 3) & M64) | (lo >> 61);
    let lz = (hi as u64).leading_zeros();
    let sh = lz + 1;
    let e = 1023 - sh as u128;
    hi = ((hi << sh) & M64) | if sh <= 64 { lo >> (64 - sh) } else { lo << (sh - 64) };
    hi >>= 12;
    hi |= e << 52;
    let mut z = f64::from_bits(hi as u64);
    if j & 1 != 0 {
        j = (j + 1) & 7;
        z -= 1.0;
    }
    (z * PI4, j)
}
/// the angle below 2^29 into its eighth: (the remainder, the eighth before any mask)
#[inline]
fn eighth(x: f64) -> (f64, i64) {
    let mut j = floor(x * FOPI);
    let mut y = j;
    if (j as i64) & 1 != 0 {
        j += 1.0;
        y += 1.0;
    }
    (((x - y * PI4A) - y * PI4B) - y * PI4C, j as i64)
}

pub fn sin(mut x: f64) -> f64 {
    if x == 0.0 || x != x {
        return x;
    }
    if x == INF || x == -INF {
        return f64::NAN;
    }
    let mut sign = false;
    if x < 0.0 {
        x = -x;
        sign = true;
    }
    let (z, mut j) = if x >= REDUCE {
        trig_reduce(x)
    } else {
        let (z, j) = eighth(x);
        (z, j & 7)
    };
    if j > 3 {
        sign = !sign;
        j -= 4;
    }
    let zz = z * z;
    let y = if j == 1 || j == 2 { cos_poly(zz) } else { sin_poly(z, zz) };
    if sign { -y } else { y }
}
pub fn cos(x: f64) -> f64 {
    if x != x || x == INF || x == -INF {
        return f64::NAN;
    }
    let mut sign = false;
    let x = x.abs();
    let (z, mut j) = if x >= REDUCE {
        trig_reduce(x)
    } else {
        let (z, j) = eighth(x);
        (z, j & 7)
    };
    if j > 3 {
        j -= 4;
        sign = !sign;
    }
    if j > 1 {
        sign = !sign;
    }
    let zz = z * z;
    let y = if j == 1 || j == 2 { sin_poly(z, zz) } else { cos_poly(zz) };
    if sign { -y } else { y }
}
const TANP0: f64 = -1.30936939181383777646e4;
const TANP1: f64 = 1.15351664838587416140e6;
const TANP2: f64 = -1.79565251976484877988e7;
const TANQ1: f64 = 1.36812963470692954678e4;
const TANQ2: f64 = -1.32089234440210967447e6;
const TANQ3: f64 = 2.50083801823357915839e7;
const TANQ4: f64 = -5.38695755929454629881e7;
pub fn tan(mut x: f64) -> f64 {
    if x == 0.0 || x != x {
        return x;
    }
    if x == INF || x == -INF {
        return f64::NAN;
    }
    let mut sign = false;
    if x < 0.0 {
        x = -x;
        sign = true;
    }
    let (z, j) = if x >= REDUCE { trig_reduce(x) } else { eighth(x) };
    let zz = z * z;
    let mut y = if zz > 1e-17 {
        z + z * (zz * (((TANP0 * zz) + TANP1) * zz + TANP2) / ((((zz + TANQ1) * zz + TANQ2) * zz + TANQ3) * zz + TANQ4))
    } else {
        z
    };
    if j & 2 == 2 {
        y = -1.0 / y;
    }
    if sign { -y } else { y }
}

// ---- the arc functions, all of them through one arctangent
const ATP0: f64 = -8.750608600031904122785e-01;
const ATP1: f64 = -1.615753718733365076637e+01;
const ATP2: f64 = -7.500855792314704667340e+01;
const ATP3: f64 = -1.228866684490136173410e+02;
const ATP4: f64 = -6.485021904942025371773e+01;
const ATQ0: f64 = 2.485846490142306297962e+01;
const ATQ1: f64 = 1.650270098316988542046e+02;
const ATQ2: f64 = 4.328810604912902668951e+02;
const ATQ3: f64 = 4.853903996359136964868e+02;
const ATQ4: f64 = 1.945506571482613964425e+02;
const MOREBITS: f64 = 6.123233995736765886130e-17;
const TAN3PIO8: f64 = 2.41421356237309504880;
/// arctangent on [0, 0.66]
#[inline]
fn xatan(x: f64) -> f64 {
    let mut z = x * x;
    z = z * ((((ATP0 * z + ATP1) * z + ATP2) * z + ATP3) * z + ATP4) / (((((z + ATQ0) * z + ATQ1) * z + ATQ2) * z + ATQ3) * z + ATQ4);
    x * z + x
}
/// arctangent of x ≥ 0
#[inline]
fn satan(x: f64) -> f64 {
    if x <= 0.66 {
        return xatan(x);
    }
    if x > TAN3PIO8 {
        return PI2 - xatan(1.0 / x) + MOREBITS;
    }
    PI4 + xatan((x - 1.0) / (x + 1.0)) + 0.5 * MOREBITS
}
pub fn atan(x: f64) -> f64 {
    if x == 0.0 {
        return x;
    }
    if x > 0.0 { satan(x) } else { -satan(-x) }
}
/// arcsine on [0, ½]
#[inline]
fn asin_low(x: f64) -> f64 {
    satan(x / sqrt(1.0 - x * x))
}
pub fn asin(mut x: f64) -> f64 {
    if x == 0.0 {
        return x;
    }
    let mut sign = false;
    if x < 0.0 {
        x = -x;
        sign = true;
    }
    if x > 1.0 {
        return f64::NAN;
    }
    let t = if x > 0.5 { PI2 - 2.0 * asin_low(sqrt((1.0 - x) / 2.0)) } else { asin_low(x) };
    if sign { -t } else { t }
}
pub fn acos(x: f64) -> f64 {
    if x > 0.5 {
        return if x > 1.0 { f64::NAN } else { 2.0 * asin_low(sqrt((1.0 - x) / 2.0)) };
    }
    if x < -0.5 {
        return if x < -1.0 { f64::NAN } else { PI - 2.0 * asin_low(sqrt((1.0 + x) / 2.0)) };
    }
    PI2 - asin(x)
}
const PI34: f64 = 2.356194490192345; // 3π/4, rounded once
/// m ≥ 0 with s's sign (a negative zero's too)
#[inline]
fn signed(m: f64, s: f64) -> f64 {
    if s < 0.0 || (s == 0.0 && 1.0 / s < 0.0) { -m } else { m }
}
pub fn atan2(y: f64, x: f64) -> f64 {
    if y != y || x != x {
        return f64::NAN;
    }
    if y == 0.0 {
        return if x > 0.0 || (x == 0.0 && 1.0 / x > 0.0) { signed(0.0, y) } else { signed(PI, y) };
    }
    if x == 0.0 {
        return signed(PI2, y);
    }
    if x == INF {
        return if y == INF || y == -INF { signed(PI4, y) } else { signed(0.0, y) };
    }
    if x == -INF {
        return if y == INF || y == -INF { signed(PI34, y) } else { signed(PI, y) };
    }
    if y == INF || y == -INF {
        return signed(PI2, y);
    }
    let q = atan(y / x);
    if x < 0.0 {
        return if y < 0.0 { q - PI } else { q + PI };
    }
    q
}

// ---- e^x and the logarithms
const LN2HI: f64 = 6.93147180369123816490e-01;
const LN2LO: f64 = 1.90821492927058770002e-10;
const LOG2E: f64 = 1.44269504088896338700e+00;
const EXP_OVER: f64 = 7.09782712893383973096e+02;
const EXP_UNDER: f64 = -7.45133219101941108420e+02;
const NEAR_ZERO: f64 = 1.0 / 268435456.0;
const EP1: f64 = 1.66666666666666657415e-01;
const EP2: f64 = -2.77777777770155933842e-03;
const EP3: f64 = 6.61375632143793436117e-05;
const EP4: f64 = -1.65339022054652515390e-06;
const EP5: f64 = 4.13813679705723846039e-08;
pub fn exp(x: f64) -> f64 {
    if x != x || x == INF {
        return x;
    }
    if x == -INF {
        return 0.0;
    }
    if x > EXP_OVER {
        return INF;
    }
    if x < EXP_UNDER {
        return 0.0;
    }
    if -NEAR_ZERO < x && x < NEAR_ZERO {
        return 1.0 + x;
    }
    let k = if x < 0.0 {
        trunc(LOG2E * x - 0.5)
    } else if x > 0.0 {
        trunc(LOG2E * x + 0.5)
    } else {
        0.0
    };
    let hi = x - k * LN2HI;
    let lo = k * LN2LO;
    let r = hi - lo;
    let t = r * r;
    let c = r - t * (EP1 + t * (EP2 + t * (EP3 + t * (EP4 + t * EP5))));
    ldexp(1.0 - ((lo - (r * c) / (2.0 - c)) - hi), k as i64)
}
const L1: f64 = 6.666666666666735130e-01;
const L2: f64 = 3.999999999940941908e-01;
const L3: f64 = 2.857142874366239149e-01;
const L4: f64 = 2.222219843214978396e-01;
const L5: f64 = 1.818357216161805012e-01;
const L6: f64 = 1.531383769920937332e-01;
const L7: f64 = 1.479819860511658591e-01;
pub fn log(x: f64) -> f64 {
    if x != x || x == INF {
        return x;
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return -INF;
    }
    let (mut f1, mut ki) = frexp(x);
    if f1 < SQRT1_2 {
        f1 *= 2.0;
        ki -= 1;
    }
    let f = f1 - 1.0;
    let k = ki as f64;
    let s = f / (2.0 + f);
    let s2 = s * s;
    let s4 = s2 * s2;
    let t1 = s2 * (L1 + s4 * (L3 + s4 * (L5 + s4 * L7)));
    let t2 = s4 * (L2 + s4 * (L4 + s4 * L6));
    let r = t1 + t2;
    let hfsq = 0.5 * f * f;
    k * LN2HI - ((hfsq - (s * (hfsq + r) + k * LN2LO)) - f)
}
const INV_LN10: f64 = 0.4342944819032518;
const INV_LN2: f64 = 1.4426950408889634;
/// the whole number nearest y, a half going up (the language's Math.round)
#[inline]
pub fn round_half_up(y: f64) -> f64 {
    let f = floor(y);
    let r = if y - f >= 0.5 { f + 1.0 } else { f };
    if r == 0.0 && y < 0.0 { -0.0 } else { r }
}
pub fn log10(x: f64) -> f64 {
    let y = log(x) * INV_LN10;
    let r = round_half_up(y);
    // (a power of ten: its exponent, exactly — the product above may sit one place off it)
    if y != r && (y - r).abs() < 1e-9 && r >= -323.0 && r <= 308.0 && f64::from_bits(POW10[(r as i32 - POW10_MIN) as usize]) == x {
        return r;
    }
    y
}
pub fn log2(x: f64) -> f64 {
    let (mut frac, mut e) = frexp(x);
    if frac == 0.5 {
        return (e - 1) as f64; // (a power of two: its exponent, exactly)
    }
    if frac < SQRT1_2 {
        frac *= 2.0;
        e -= 1;
    }
    log(frac) * INV_LN2 + e as f64
}

// ---- x^y: the whole part of y by squaring on the fraction of x, the rest through e^(y ln x)
pub fn pow(x: f64, y: f64) -> f64 {
    if y != y {
        return f64::NAN;
    }
    if y == 0.0 {
        return 1.0;
    }
    if x != x {
        return f64::NAN;
    }
    if (x == 1.0 || x == -1.0) && (y == INF || y == -INF) {
        return f64::NAN;
    }
    if x == 1.0 {
        return 1.0;
    }
    if y == 1.0 {
        return x;
    }
    let ay = y.abs();
    let odd_int = ay < 9007199254740992.0 && floor(ay) == ay && (ay as u64) & 1 == 1;
    if x == 0.0 {
        let neg = 1.0 / x < 0.0;
        if y < 0.0 {
            return if neg && odd_int { -INF } else { INF };
        }
        return if neg && odd_int { x } else { 0.0 };
    }
    if y == INF || y == -INF {
        return if (x.abs() < 1.0) == (y == INF) { 0.0 } else { INF };
    }
    if x == INF {
        return if y < 0.0 { 0.0 } else { INF };
    }
    if x == -INF {
        return if y < 0.0 {
            if odd_int { -0.0 } else { 0.0 }
        } else if odd_int {
            -INF
        } else {
            INF
        };
    }
    if y == 0.5 {
        return sqrt(x);
    }
    if y == -0.5 {
        return 1.0 / sqrt(x);
    }
    let mut yi = floor(ay);
    let mut yf = ay - yi;
    if yf != 0.0 && x < 0.0 {
        return f64::NAN;
    }
    if yi >= 9223372036854775808.0 {
        return if x == -1.0 {
            1.0
        } else if (x.abs() < 1.0) == (y > 0.0) {
            0.0
        } else {
            INF
        };
    }
    let mut a1 = 1.0;
    let mut ae: i64 = 0;
    if yf != 0.0 {
        if yf > 0.5 {
            yf -= 1.0;
            yi += 1.0;
        }
        a1 = exp(yf * log(x));
    }
    let (mut x1, mut xe) = frexp(x);
    let mut i = yi as u64; // (a whole number below 2^63: halving it is a shift, its last bit its oddness)
    while i != 0 {
        if xe < -4096 || 4096 < xe {
            ae += xe; // (over or under: the answer is already an infinity or a zero)
            break;
        }
        if i & 1 == 1 {
            a1 *= x1;
            ae += xe;
        }
        x1 *= x1;
        xe *= 2;
        if x1 < 0.5 {
            x1 += x1;
            xe -= 1;
        }
        i >>= 1;
    }
    if y < 0.0 {
        a1 = 1.0 / a1;
        ae = -ae;
    }
    ldexp(a1, ae)
}

const TANHP0: f64 = -9.64399179425052238628e-1;
const TANHP1: f64 = -9.92877231001918586564e1;
const TANHP2: f64 = -1.61468768441708447952e3;
const TANHQ0: f64 = 1.12811678491632931402e2;
const TANHQ1: f64 = 2.23548839060100448583e3;
const TANHQ2: f64 = 4.84406305325125486048e3;
const HALF_MAXLOG: f64 = 0.5 * 8.8029691931113054295988e+01;
pub fn tanh(x: f64) -> f64 {
    let mut z = x.abs();
    if z > HALF_MAXLOG {
        return if x < 0.0 { -1.0 } else { 1.0 };
    }
    if z >= 0.625 {
        let s = exp(2.0 * z);
        z = 1.0 - 2.0 / (s + 1.0);
        return if x < 0.0 { -z } else { z };
    }
    if x == 0.0 {
        return x;
    }
    let s = x * x;
    x + x * s * ((TANHP0 * s + TANHP1) * s + TANHP2) / (((s + TANHQ0) * s + TANHQ1) * s + TANHQ2)
}

// ---- lengths: |x| scaled by the largest, the squares in a compensated sum (src/math/hyp.ts kahan2 / 3 / 4 / N)
/// one more scaled square into the running sum
#[inline]
fn kahan_step(x: f64, max: f64, sum: &mut f64, comp: &mut f64) {
    let n = x / max;
    let s = n * n - *comp;
    let p = *sum + s;
    *comp = (p - *sum) - s;
    *sum = p;
}
pub fn hypot2(a: f64, b: f64) -> f64 {
    let (x, y) = (a.abs(), b.abs());
    let mut max = 0.0;
    if x > max {
        max = x;
    }
    if y > max {
        max = y;
    }
    if max == INF {
        return INF;
    }
    if x != x || y != y {
        return f64::NAN;
    }
    if max == 0.0 {
        return 0.0;
    }
    let (mut sum, mut comp) = (0.0, 0.0);
    kahan_step(x, max, &mut sum, &mut comp);
    kahan_step(y, max, &mut sum, &mut comp);
    sqrt(sum) * max
}
pub fn hypot3(a: f64, b: f64, c: f64) -> f64 {
    let (x, y, z) = (a.abs(), b.abs(), c.abs());
    let mut max = 0.0;
    if x > max {
        max = x;
    }
    if y > max {
        max = y;
    }
    if z > max {
        max = z;
    }
    if max == INF {
        return INF;
    }
    if x != x || y != y || z != z {
        return f64::NAN;
    }
    if max == 0.0 {
        return 0.0;
    }
    let (mut sum, mut comp) = (0.0, 0.0);
    kahan_step(x, max, &mut sum, &mut comp);
    kahan_step(y, max, &mut sum, &mut comp);
    kahan_step(z, max, &mut sum, &mut comp);
    sqrt(sum) * max
}
pub fn hypot4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let (x, y, z, w) = (a.abs(), b.abs(), c.abs(), d.abs());
    let mut max = 0.0;
    if x > max {
        max = x;
    }
    if y > max {
        max = y;
    }
    if z > max {
        max = z;
    }
    if w > max {
        max = w;
    }
    if max == INF {
        return INF;
    }
    if x != x || y != y || z != z || w != w {
        return f64::NAN;
    }
    if max == 0.0 {
        return 0.0;
    }
    let (mut sum, mut comp) = (0.0, 0.0);
    kahan_step(x, max, &mut sum, &mut comp);
    kahan_step(y, max, &mut sum, &mut comp);
    kahan_step(z, max, &mut sum, &mut comp);
    kahan_step(w, max, &mut sum, &mut comp);
    sqrt(sum) * max
}
/// the same sum for any number of terms
pub fn hypot_n(v: &[f64]) -> f64 {
    match v.len() {
        2 => return hypot2(v[0], v[1]),
        3 => return hypot3(v[0], v[1], v[2]),
        4 => return hypot4(v[0], v[1], v[2], v[3]),
        _ => {}
    }
    let mut max = 0.0;
    let mut nan = false;
    for &t in v {
        let x = t.abs();
        if x != x {
            nan = true;
        } else if x > max {
            max = x;
        }
    }
    if max == INF {
        return INF;
    }
    if nan {
        return f64::NAN;
    }
    if max == 0.0 {
        return 0.0;
    }
    let (mut sum, mut comp) = (0.0, 0.0);
    for &t in v {
        kahan_step(t.abs(), max, &mut sum, &mut comp);
    }
    sqrt(sum) * max
}
