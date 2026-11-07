//! THE BASE EVERYTHING ELSE USES: vectors, quaternions, the scalar helpers and the matrices (src/math/vec.ts). Faithful to the bit: the same order of
//! operations as the TypeScript, operand for operand, on the game's own maths (`crate::m`). Nothing here reassociates, factors or simplifies, even
//! where an operand is a literal 0 or 1: a signed zero comes out differently.
//!
//! `v3.foo` is `v3::foo`, `quat.foo` is `v3::quat::foo`, `m4.foo` is `v3::m4::foo`. One name moved, because the scalar `lerp` holds the name here:
//! `v3.lerp` is `v3::lerp3`. Everything takes and gives vectors by value (they are Copy), so a caller can pass `g.ch[c].pos` and still hold `g`.
use crate::js;
use crate::m;

pub use crate::m::{hyp2, hyp3, hyp4};

pub type V3 = [f64; 3];
/// x y z w
pub type Quat = [f64; 4];

/// `x || 1` on a number: 0, −0 and NaN become 1
#[inline]
fn or_one(l: f64) -> f64 {
    if l != l || l == 0.0 { 1.0 } else { l }
}

/// (vec.ts clamp) A NaN stays NaN; the bounds are not ordered first.
#[inline]
pub fn clamp(x: f64, a: f64, b: f64) -> f64 {
    if x < a { a } else if x > b { b } else { x }
}
/// (vec.ts lerp)
#[inline]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
/// (vec.ts smoothstep)
#[inline]
pub fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = clamp((x - a) / (b - a), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
/// (vec.ts DEG)
pub const DEG: f64 = m::PI / 180.0;
/// Wrap an angle to (−π, π]. (vec.ts wrapAngle)
#[inline]
pub fn wrap_angle(a: f64) -> f64 {
    let mut a = js::rem(a + m::PI, 2.0 * m::PI);
    if a <= 0.0 {
        a += 2.0 * m::PI;
    }
    a - m::PI
}
/// Move angle `a` toward `b` by at most `max_step`, the short way round. (vec.ts approachAngle)
#[inline]
pub fn approach_angle(a: f64, b: f64, max_step: f64) -> f64 {
    let d = wrap_angle(b - a);
    if m::abs(d) <= max_step { b } else { a + m::sign(d) * max_step }
}
/// (vec.ts damp)
#[inline]
pub fn damp(a: f64, b: f64, rate: f64, dt: f64) -> f64 {
    lerp(a, b, 1.0 - m::exp(-rate * dt))
}

/// (vec.ts v3.make)
#[inline]
pub fn make(x: Option<f64>, y: Option<f64>, z: Option<f64>) -> V3 {
    [x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0)]
}
/// (vec.ts v3.copy) A vector is a value here, so this is `a`; it stays so that a line translates letter for letter. Where the TypeScript copied to
/// break a share there is nothing to break; where it did NOT copy and two holders share one vector, see the conventions' ALIASING.
#[inline]
pub fn copy(a: V3) -> V3 {
    a
}
/// (vec.ts v3.add)
#[inline]
pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
/// (vec.ts v3.sub)
#[inline]
pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
/// (vec.ts v3.scale)
#[inline]
pub fn scale(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
/// `a + b × s` (vec.ts v3.mad)
#[inline]
pub fn mad(a: V3, b: V3, s: f64) -> V3 {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}
/// (vec.ts v3.mul)
#[inline]
pub fn mul(a: V3, b: V3) -> V3 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2]]
}
/// (vec.ts v3.dot)
#[inline]
pub fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
/// (vec.ts v3.cross)
#[inline]
pub fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
/// (vec.ts v3.len)
#[inline]
pub fn len(a: V3) -> f64 {
    hyp3(a[0], a[1], a[2])
}
/// (vec.ts v3.dist)
#[inline]
pub fn dist(a: V3, b: V3) -> f64 {
    hyp3(a[0] - b[0], a[1] - b[1], a[2] - b[2])
}
/// How far apart on the floor. (vec.ts v3.distXZ)
#[inline]
pub fn dist_xz(a: V3, b: V3) -> f64 {
    hyp2(a[0] - b[0], a[2] - b[2])
}
/// The yaw whose forward `[sin, 0, cos]` points along `d`; y ignored. (vec.ts v3.yawOf)
#[inline]
pub fn yaw_of(d: V3) -> f64 {
    m::atan2(d[0], d[2])
}
/// The yaw from one point toward another. (vec.ts v3.yawTo)
#[inline]
pub fn yaw_to(from: V3, to: V3) -> f64 {
    m::atan2(to[0] - from[0], to[2] - from[2])
}
/// A zero or NaN length divides by 1. (vec.ts v3.normalize)
#[inline]
pub fn normalize(a: V3) -> V3 {
    let l = or_one(hyp3(a[0], a[1], a[2]));
    [a[0] / l, a[1] / l, a[2] / l]
}
/// (vec.ts v3.lerp: the scalar `lerp` holds the plain name in this module)
#[inline]
pub fn lerp3(a: V3, b: V3, t: f64) -> V3 {
    [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)]
}
/// (vec.ts v3.min)
#[inline]
pub fn min(a: V3, b: V3) -> V3 {
    [m::min(a[0], b[0]), m::min(a[1], b[1]), m::min(a[2], b[2])]
}
/// (vec.ts v3.max)
#[inline]
pub fn max(a: V3, b: V3) -> V3 {
    [m::max(a[0], b[0]), m::max(a[1], b[1]), m::max(a[2], b[2])]
}

/// Rotations (vec.ts quat).
pub mod quat {
    use super::{hyp3, hyp4, lerp, or_one, Quat, V3};
    use crate::m;

    /// (vec.ts quat.ident)
    #[inline]
    pub fn ident() -> Quat {
        [0.0, 0.0, 0.0, 1.0]
    }
    /// (vec.ts quat.axisAngle)
    #[inline]
    pub fn axis_angle(axis: V3, ang: f64) -> Quat {
        let s = m::sin(ang / 2.0);
        [axis[0] * s, axis[1] * s, axis[2] * s, m::cos(ang / 2.0)]
    }
    /// (vec.ts quat.yaw)
    #[inline]
    pub fn yaw(ang: f64) -> Quat {
        [0.0, m::sin(ang / 2.0), 0.0, m::cos(ang / 2.0)]
    }
    /// (vec.ts quat.mul)
    #[inline]
    pub fn mul(a: Quat, b: Quat) -> Quat {
        let [ax, ay, az, aw] = a;
        let [bx, by, bz, bw] = b;
        [
            aw * bx + ax * bw + ay * bz - az * by,
            aw * by - ax * bz + ay * bw + az * bx,
            aw * bz + ax * by - ay * bx + az * bw,
            aw * bw - ax * bx - ay * by - az * bz,
        ]
    }
    /// (vec.ts quat.conj)
    #[inline]
    pub fn conj(a: Quat) -> Quat {
        [-a[0], -a[1], -a[2], a[3]]
    }
    /// A zero or NaN length divides by 1. (vec.ts quat.normalize)
    #[inline]
    pub fn normalize(a: Quat) -> Quat {
        let l = or_one(hyp4(a[0], a[1], a[2], a[3]));
        [a[0] / l, a[1] / l, a[2] / l, a[3] / l]
    }
    /// (vec.ts quat.dot)
    #[inline]
    pub fn dot(a: Quat, b: Quat) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
    }
    /// Normalized lerp along the short arc. (vec.ts quat.nlerp)
    pub fn nlerp(a: Quat, b: Quat, t: f64) -> Quat {
        let s = if dot(a, b) < 0.0 { -1.0 } else { 1.0 };
        normalize([lerp(a[0], b[0] * s, t), lerp(a[1], b[1] * s, t), lerp(a[2], b[2] * s, t), lerp(a[3], b[3] * s, t)])
    }
    /// (vec.ts quat.slerp)
    pub fn slerp(a: Quat, b: Quat, t: f64) -> Quat {
        let mut d = dot(a, b);
        let mut s = 1.0;
        if d < 0.0 {
            d = -d;
            s = -1.0;
        }
        if d > 0.9995 {
            return nlerp(a, b, t);
        }
        let th = m::acos(d);
        let sn = m::sin(th);
        let wa = m::sin((1.0 - t) * th) / sn;
        let wb = (m::sin(t * th) / sn) * s;
        [a[0] * wa + b[0] * wb, a[1] * wa + b[1] * wb, a[2] * wa + b[2] * wb, a[3] * wa + b[3] * wb]
    }
    /// The rotation taking local +X/+Y/+Z to the given orthonormal axes. (vec.ts quat.fromBasis)
    pub fn from_basis(x: V3, y: V3, z: V3) -> Quat {
        let (m00, m10, m20, m01, m11, m21, m02, m12, m22) = (x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]);
        let tr = m00 + m11 + m22;
        let q: Quat = if tr > 0.0 {
            let s = m::sqrt(tr + 1.0) * 2.0;
            [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
        } else if m00 > m11 && m00 > m22 {
            let s = m::sqrt(1.0 + m00 - m11 - m22) * 2.0;
            [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
        } else if m11 > m22 {
            let s = m::sqrt(1.0 + m11 - m00 - m22) * 2.0;
            [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
        } else {
            let s = m::sqrt(1.0 + m22 - m00 - m11) * 2.0;
            [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
        };
        normalize(q)
    }
    /// The short-arc rotation taking unit vector `a` onto unit vector `b`. (vec.ts quat.fromTo)
    pub fn from_to(a: V3, b: V3) -> Quat {
        let d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        if d < -0.9999 {
            let ax: V3 = if m::abs(a[0]) < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
            let c: V3 = [a[1] * ax[2] - a[2] * ax[1], a[2] * ax[0] - a[0] * ax[2], a[0] * ax[1] - a[1] * ax[0]];
            let l = or_one(hyp3(c[0], c[1], c[2]));
            return [c[0] / l, c[1] / l, c[2] / l, 0.0];
        }
        let c: V3 = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
        normalize([c[0], c[1], c[2], 1.0 + d])
    }
    /// (vec.ts quat.rotate)
    #[inline]
    pub fn rotate(q: Quat, v: V3) -> V3 {
        let [qx, qy, qz, qw] = q;
        let (tx, ty, tz) = (2.0 * (qy * v[2] - qz * v[1]), 2.0 * (qz * v[0] - qx * v[2]), 2.0 * (qx * v[1] - qy * v[0]));
        [v[0] + qw * tx + (qy * tz - qz * ty), v[1] + qw * ty + (qz * tx - qx * tz), v[2] + qw * tz + (qx * ty - qy * tx)]
    }
}

/// Matrices, column-major (vec.ts m4): only those the simulation's folders call.
///
/// A `Mat4` STORES singles because the TypeScript's is a Float32Array; THE ARITHMETIC IS IN DOUBLES and each result is rounded once as it is stored,
/// as a Float32Array does. Working in `f32` would round at every step and give other bits, so every read here is widened (`wide`) and every write
/// narrowed (`as f32`). A caller reading a cell writes `m[12] as f64`; the compiler refuses `m[12] * x` with an `f64`, which is the point of `f32` here.
///
/// The TypeScript's `out` parameter is gone: each function RETURNS the matrix. All of them write all sixteen cells and read their inputs in an order
/// that makes `out === a` and `out === b` harmless, so a value gives what the TypeScript gives for any sharing, and a caller holds no `&mut` into `g`.
pub mod m4 {
    use super::{or_one, Quat, V3};
    use crate::m;

    pub type Mat4 = [f32; 16];

    /// every cell as the double the TypeScript reads
    #[inline]
    fn wide(a: &Mat4) -> [f64; 16] {
        let mut d = [0.0; 16];
        let mut i = 0;
        while i < 16 {
            d[i] = a[i] as f64;
            i += 1;
        }
        d
    }

    /// (vec.ts m4.create)
    #[inline]
    pub fn create() -> Mat4 {
        let mut m = [0.0; 16];
        m[0] = 1.0;
        m[5] = 1.0;
        m[10] = 1.0;
        m[15] = 1.0;
        m
    }
    /// (vec.ts m4.copy)
    #[inline]
    pub fn copy(a: &Mat4) -> Mat4 {
        *a
    }
    /// `a × b`. (vec.ts m4.mul)
    pub fn mul(a: &Mat4, b: &Mat4) -> Mat4 {
        let [a00, a01, a02, a03, a10, a11, a12, a13, a20, a21, a22, a23, a30, a31, a32, a33] = wide(a);
        let mut out = [0.0; 16];
        for i in 0..4 {
            let (b0, b1, b2, b3) = (b[i * 4] as f64, b[i * 4 + 1] as f64, b[i * 4 + 2] as f64, b[i * 4 + 3] as f64);
            out[i * 4] = (b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30) as f32;
            out[i * 4 + 1] = (b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31) as f32;
            out[i * 4 + 2] = (b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32) as f32;
            out[i * 4 + 3] = (b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33) as f32;
        }
        out
    }
    /// (vec.ts m4.fromTRS)
    #[inline]
    pub fn from_trs(t: V3, q: Quat, s: V3) -> Mat4 {
        from_trss(t[0], t[1], t[2], q[0], q[1], q[2], q[3], s[0], s[1], s[2])
    }
    /// `from_trs` over scalars. (vec.ts m4.fromTRSs)
    #[allow(clippy::too_many_arguments)]
    pub fn from_trss(tx: f64, ty: f64, tz: f64, x: f64, y: f64, z: f64, w: f64, s0: f64, s1: f64, s2: f64) -> Mat4 {
        let (x2, y2, z2) = (x + x, y + y, z + z);
        let (xx, xy, xz, yy, yz, zz) = (x * x2, x * y2, x * z2, y * y2, y * z2, z * z2);
        let (wx, wy, wz) = (w * x2, w * y2, w * z2);
        [
            ((1.0 - (yy + zz)) * s0) as f32,
            ((xy + wz) * s0) as f32,
            ((xz - wy) * s0) as f32,
            0.0,
            ((xy - wz) * s1) as f32,
            ((1.0 - (xx + zz)) * s1) as f32,
            ((yz + wx) * s1) as f32,
            0.0,
            ((xz + wy) * s2) as f32,
            ((yz - wx) * s2) as f32,
            ((1.0 - (xx + yy)) * s2) as f32,
            0.0,
            tx as f32,
            ty as f32,
            tz as f32,
            1.0,
        ]
    }
    /// A `w` of 0 or NaN divides by 1. (vec.ts m4.transformPoint)
    #[inline]
    pub fn transform_point(m: &Mat4, p: V3) -> V3 {
        let [x, y, z] = p;
        let c = wide(m);
        let w = or_one(c[3] * x + c[7] * y + c[11] * z + c[15]);
        [(c[0] * x + c[4] * y + c[8] * z + c[12]) / w, (c[1] * x + c[5] * y + c[9] * z + c[13]) / w, (c[2] * x + c[6] * y + c[10] * z + c[14]) / w]
    }
    /// (vec.ts m4.getTranslation)
    #[inline]
    pub fn get_translation(m: &Mat4) -> V3 {
        [m[12] as f64, m[13] as f64, m[14] as f64]
    }
    /// `None` where the determinant is 0 or NaN; the TypeScript then leaves `out` as it was, so the caller keeps its old matrix. (vec.ts m4.invert)
    pub fn invert(a: &Mat4) -> Option<Mat4> {
        let [a00, a01, a02, a03, a10, a11, a12, a13, a20, a21, a22, a23, a30, a31, a32, a33] = wide(a);
        let (b00, b01, b02, b03) = (a00 * a11 - a01 * a10, a00 * a12 - a02 * a10, a00 * a13 - a03 * a10, a01 * a12 - a02 * a11);
        let (b04, b05, b06, b07) = (a01 * a13 - a03 * a11, a02 * a13 - a03 * a12, a20 * a31 - a21 * a30, a20 * a32 - a22 * a30);
        let (b08, b09, b10, b11) = (a20 * a33 - a23 * a30, a21 * a32 - a22 * a31, a21 * a33 - a23 * a31, a22 * a33 - a23 * a32);
        let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        if det != det || det == 0.0 {
            return None;
        }
        let det = 1.0 / det;
        Some([
            ((a11 * b11 - a12 * b10 + a13 * b09) * det) as f32,
            ((a02 * b10 - a01 * b11 - a03 * b09) * det) as f32,
            ((a31 * b05 - a32 * b04 + a33 * b03) * det) as f32,
            ((a22 * b04 - a21 * b05 - a23 * b03) * det) as f32,
            ((a12 * b08 - a10 * b11 - a13 * b07) * det) as f32,
            ((a00 * b11 - a02 * b08 + a03 * b07) * det) as f32,
            ((a32 * b02 - a30 * b05 - a33 * b01) * det) as f32,
            ((a20 * b05 - a22 * b02 + a23 * b01) * det) as f32,
            ((a10 * b10 - a11 * b08 + a13 * b06) * det) as f32,
            ((a01 * b08 - a00 * b10 - a03 * b06) * det) as f32,
            ((a30 * b04 - a31 * b02 + a33 * b00) * det) as f32,
            ((a21 * b02 - a20 * b04 - a23 * b00) * det) as f32,
            ((a11 * b07 - a10 * b09 - a12 * b06) * det) as f32,
            ((a00 * b09 - a01 * b07 + a02 * b06) * det) as f32,
            ((a31 * b01 - a30 * b03 - a32 * b00) * det) as f32,
            ((a20 * b03 - a21 * b01 + a22 * b00) * det) as f32,
        ])
    }
    /// Right-handed, WebGPU clip depth [0, 1]: near → 0, far → 1. (vec.ts m4.perspective)
    pub fn perspective(fovy: f64, aspect: f64, near: f64, far: f64) -> Mat4 {
        let f = 1.0 / m::tan(fovy / 2.0);
        let mut out = [0.0; 16];
        out[0] = (f / aspect) as f32;
        out[5] = f as f32;
        out[10] = (far / (near - far)) as f32;
        out[11] = -1.0;
        out[14] = ((far * near) / (near - far)) as f32;
        out
    }
    /// (vec.ts m4.lookAt)
    pub fn look_at(eye: V3, target: V3, up: V3) -> Mat4 {
        let z = super::normalize(super::sub(eye, target));
        let x = super::normalize(super::cross(up, z));
        let y = super::cross(z, x);
        [
            x[0] as f32,
            y[0] as f32,
            z[0] as f32,
            0.0,
            x[1] as f32,
            y[1] as f32,
            z[1] as f32,
            0.0,
            x[2] as f32,
            y[2] as f32,
            z[2] as f32,
            0.0,
            (-super::dot(x, eye)) as f32,
            (-super::dot(y, eye)) as f32,
            (-super::dot(z, eye)) as f32,
            1.0,
        ]
    }
}

#[cfg(test)]
mod tests {
    //! Every expected value is the TypeScript's own bits: src/math/vec.ts run under bun with the game's maths installed (`EXACT.on = true`, as
    //! src/sim/sim.ts mathsUp does), printed as hex. A NaN is any NaN: its sign and payload are not something the language fixes.
    use super::m4::Mat4;
    use super::{m4, quat};
    fn same(got: f64, want: u64) -> bool {
        let w = f64::from_bits(want);
        (got != got && w != w) || got.to_bits() == want
    }
    fn one(got: f64, want: u64) {
        assert!(same(got, want), "got {:#018x} ({}) want {:#018x} ({})", got.to_bits(), got, want, f64::from_bits(want));
    }
    fn many(got: &[f64], want: &[u64]) {
        assert_eq!(got.len(), want.len());
        for i in 0..got.len() {
            assert!(same(got[i], want[i]), "[{}] got {:#018x} ({}) want {:#018x} ({})", i, got[i].to_bits(), got[i], want[i], f64::from_bits(want[i]));
        }
    }
    fn singles(got: &[f32], want: &[u32]) {
        assert_eq!(got.len(), want.len());
        for i in 0..got.len() {
            let w = f32::from_bits(want[i]);
            assert!((got[i] != got[i] && w != w) || got[i].to_bits() == want[i], "[{}] got {:#010x} ({}) want {:#010x} ({})", i, got[i].to_bits(), got[i], want[i], w);
        }
    }
    fn mat(b: [u32; 16]) -> Mat4 {
        let mut m = [0.0; 16];
        for i in 0..16 {
            m[i] = f32::from_bits(b[i]);
        }
        m
    }
    #[test]
    fn t_deg() {
        one(super::DEG, 0x3f91df46a2529d39);
    }
    #[test]
    fn the_scalar_lerp_and_the_vector_lerp_are_both_here() {
        // (the one name that moved: `v3.lerp` is `lerp3`)
        one(super::lerp(1.0, 3.0, 0.5), 0x4000000000000000);
        many(&super::lerp3([1.0, 1.0, 1.0], [3.0, 3.0, 3.0], 0.5), &[0x4000000000000000, 0x4000000000000000, 0x4000000000000000]);
    }
    #[test]
    fn hyp_is_reexported_as_vec_ts_does() {
        one(super::hyp2(3.0, 4.0), 0x4014000000000000);
        one(super::hyp3(1.0, 2.0, 3.0), 0x400deeea11683f48);
        one(super::hyp4(1.0, 2.0, 3.0, 4.0), 0x4015e8add236a58f);
    }
    #[test]
    fn t_clamp() {
        one(super::clamp(5.0, 0.0, 1.0), 0x3ff0000000000000);
        one(super::clamp(-5.0, 0.0, 1.0), 0x0000000000000000);
        one(super::clamp(0.5, 0.0, 1.0), 0x3fe0000000000000);
        one(super::clamp(f64::NAN, 0.0, 1.0), 0x7ff8000000000000);
        one(super::clamp(-0.0, 0.0, 1.0), 0x8000000000000000);
        one(super::clamp(0.0, -0.0, 0.0), 0x0000000000000000);
        one(super::clamp(2.0, 3.0, 1.0), 0x4008000000000000);
    }
    #[test]
    fn t_lerp() {
        one(super::lerp(1.0, 3.0, 0.25), 0x3ff8000000000000);
        one(super::lerp(1.0, 3.0, -1.0), 0xbff0000000000000);
        one(super::lerp(1.0, 3.0, 2.0), 0x4014000000000000);
        one(super::lerp(0.1, 0.7, 0.3333333333333333), 0x3fd3333333333333);
        one(super::lerp(f64::INFINITY, 1.0, 0.5), 0xfff8000000000000);
        one(super::lerp(-0.0, -0.0, 0.5), 0x0000000000000000);
        one(super::lerp(1e+308, -1e+308, 0.5), 0xfff0000000000000);
    }
    #[test]
    fn t_smoothstep() {
        one(super::smoothstep(0.0, 1.0, 0.3), 0x3fcba5e353f7ced9);
        one(super::smoothstep(0.0, 1.0, -2.0), 0x0000000000000000);
        one(super::smoothstep(0.0, 1.0, 7.0), 0x3ff0000000000000);
        one(super::smoothstep(1.0, 1.0, 1.0), 0xfff8000000000000);
        one(super::smoothstep(1.0, 1.0, 2.0), 0x3ff0000000000000);
        one(super::smoothstep(2.0, 1.0, 1.7), 0x3fcba5e353f7cedb);
        one(super::smoothstep(0.1, 0.7, 0.33), 0x3fd500d5994bed7f);
    }
    #[test]
    fn t_wrap_angle() {
        one(super::wrap_angle(0.0), 0x0000000000000000);
        one(super::wrap_angle(-0.0), 0x0000000000000000);
        one(super::wrap_angle(3.141592653589793), 0x400921fb54442d18);
        one(super::wrap_angle(-3.141592653589793), 0x400921fb54442d18);
        one(super::wrap_angle(9.42477796076938), 0x400921fb54442d18);
        one(super::wrap_angle(-9.42477796076938), 0x400921fb54442d18);
        one(super::wrap_angle(6.283185307179586), 0x0000000000000000);
        one(super::wrap_angle(7.0), 0x3fe6f0255dde9740);
        one(super::wrap_angle(-7.0), 0xbfe6f0255dde9740);
        one(super::wrap_angle(1000000000.0), 0x3fe27a05fb767460);
        one(super::wrap_angle(-1000000000.0), 0xbfe27a061d97dd20);
        one(super::wrap_angle(1e-20), 0x0000000000000000);
        one(super::wrap_angle(-1e-20), 0x0000000000000000);
        one(super::wrap_angle(f64::NAN), 0x7ff8000000000000);
        one(super::wrap_angle(f64::INFINITY), 0xfff8000000000000);
    }
    #[test]
    fn t_approach_angle() {
        one(super::approach_angle(0.0, 1.0, 0.1), 0x3fb999999999999a);
        one(super::approach_angle(3.0, -3.0, 0.1), 0x4008cccccccccccd);
        one(super::approach_angle(-3.0, 3.0, 10.0), 0x4008000000000000);
        one(super::approach_angle(1.0, 1.0, 0.5), 0x3ff0000000000000);
        one(super::approach_angle(0.0, 3.141592653589793, 0.1), 0x3fb999999999999a);
        one(super::approach_angle(0.0, -3.141592653589793, 0.1), 0x3fb999999999999a);
        one(super::approach_angle(f64::NAN, 1.0, 0.1), 0x7ff8000000000000);
        one(super::approach_angle(7.0, -7.0, 0.25), 0x401b000000000000);
        one(super::approach_angle(0.3, 0.3000001, 0.0), 0x3fd3333333333333);
    }
    #[test]
    fn t_damp() {
        one(super::damp(0.0, 1.0, 2.0, 0.016), 0x3fa01fe7c004a9a0);
        one(super::damp(5.0, -5.0, 10.0, 1.0), 0xc013ff88fc9f750a);
        one(super::damp(1.0, 2.0, 0.0, 0.1), 0x3ff0000000000000);
        one(super::damp(1.0, 2.0, -1.0, 0.5), 0x3fd67b59c787e590);
        one(super::damp(0.0, 1.0, 1000.0, 1000.0), 0x3ff0000000000000);
        one(super::damp(0.1, 0.2, 8.0, 0.016666666666666666), 0x3fbccbaa04d67f16);
    }
    #[test]
    fn t_make() {
        many(&super::make(None, None, None), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::make(Some(1.5), None, None), &[0x3ff8000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::make(Some(1.0), Some(2.0), Some(3.0)), &[0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000]);
        many(&super::make(Some(-0.0), None, Some(-0.5)), &[0x8000000000000000, 0x0000000000000000, 0xbfe0000000000000]);
    }
    #[test]
    fn t_copy() {
        many(&super::copy([-4.5, 0.25, 7.125]), &[0xc012000000000000, 0x3fd0000000000000, 0x401c800000000000]);
        many(&super::copy([-0.0, -0.0, -0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
    }
    #[test]
    fn t_add() {
        many(&super::add([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), &[0xc00c000000000000, 0x4002000000000000, 0x4024400000000000]);
        many(&super::add([-0.0, -0.0, -0.0], [-0.0, -0.0, -0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::add([-0.0, -0.0, -0.0], [0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::add([0.1, 0.2, 0.3], [1.0, 2.0, 3.0]), &[0x3ff199999999999a, 0x400199999999999a, 0x400a666666666666]);
        many(&super::add([f64::NAN, 1.0, 2.0], [f64::INFINITY, 1.0, 0.0]), &[0x7ff8000000000000, 0x4000000000000000, 0x4000000000000000]);
    }
    #[test]
    fn t_sub() {
        many(&super::sub([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), &[0x4016000000000000, 0x3ffc000000000000, 0xc010800000000000]);
        many(&super::sub([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::sub([-0.0, -0.0, -0.0], [0.0, 0.0, 0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::sub([0.1, 0.2, 0.3], [1.0, 2.0, 3.0]), &[0xbfeccccccccccccd, 0xbffccccccccccccd, 0xc00599999999999a]);
        many(&super::sub([f64::INFINITY, 1.0, 0.0], [f64::INFINITY, 1.0, 0.0]), &[0xfff8000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_scale() {
        many(&super::scale([1.0, 2.0, 3.0], 0.1), &[0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333334]);
        many(&super::scale([-4.5, 0.25, 7.125], -0.0), &[0x0000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::scale([0.1, 0.2, 0.3], 3.0), &[0x3fd3333333333334, 0x3fe3333333333334, 0x3feccccccccccccc]);
        many(&super::scale([f64::INFINITY, 1.0, 0.0], 0.0), &[0xfff8000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_mad() {
        many(&super::mad([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125], 0.1), &[0x3fe199999999999a, 0x4000333333333333, 0x400db33333333333]);
        many(&super::mad([-0.0, -0.0, -0.0], [0.0, 0.0, 0.0], -1.0), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::mad([0.1, 0.2, 0.3], [0.1, 0.2, 0.3], 0.3333333333333333), &[0x3fc1111111111111, 0x3fd1111111111111, 0x3fd9999999999999]);
    }
    #[test]
    fn t_mul() {
        many(&super::mul([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), &[0xc012000000000000, 0x3fe0000000000000, 0x4035600000000000]);
        many(&super::mul([-0.0, -0.0, -0.0], [1.0, 2.0, 3.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::mul([0.1, 0.2, 0.3], [0.1, 0.2, 0.3]), &[0x3f847ae147ae147c, 0x3fa47ae147ae147c, 0x3fb70a3d70a3d70a]);
    }
    #[test]
    fn t_dot() {
        one(super::dot([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), 0x4031600000000000);
        one(super::dot([0.0, 0.0, 0.0], [-0.0, -0.0, -0.0]), 0x8000000000000000);
        one(super::dot([0.1, 0.2, 0.3], [0.6, 0.0, 0.8]), 0x3fd3333333333333);
        one(super::dot([1e+200, -1e+200, 1e+200], [1e+200, -1e+200, 1e+200]), 0x7ff0000000000000);
    }
    #[test]
    fn t_cross() {
        many(&super::cross([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), &[0x402b000000000000, 0xc034a00000000000, 0x4022800000000000]);
        many(&super::cross([0.0, 0.0, 0.0], [1.0, 2.0, 3.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::cross([0.1, 0.2, 0.3], [0.6, 0.0, 0.8]), &[0x3fc47ae147ae147c, 0x3fb9999999999998, 0xbfbeb851eb851eb8]);
        many(&super::cross([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&super::cross([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0xbff0000000000000]);
    }
    #[test]
    fn t_len() {
        one(super::len([1.0, 2.0, 3.0]), 0x400deeea11683f48);
        one(super::len([0.0, 0.0, 0.0]), 0x0000000000000000);
        one(super::len([-0.0, -0.0, -0.0]), 0x0000000000000000);
        one(super::len([1e+200, -1e+200, 1e+200]), 0x69821a2f9c3d2c96);
        one(super::len([1e-200, 2e-200, -1e-200]), 0x167dffd96714e202);
        one(super::len([f64::NAN, 1.0, 2.0]), 0x7ff8000000000000);
        one(super::len([f64::INFINITY, 1.0, 0.0]), 0x7ff0000000000000);
        one(super::len([0.1, 0.2, 0.3]), 0x3fd7f254dab9cc3b);
    }
    #[test]
    fn t_dist() {
        one(super::dist([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), 0x401c607e4dd83be7);
        one(super::dist([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]), 0x0000000000000000);
        one(super::dist([0.1, 0.2, 0.3], [0.6, 0.0, 0.8]), 0x3fe783ddb1a48e38);
        one(super::dist([1e+200, -1e+200, 1e+200], [0.0, 0.0, 0.0]), 0x69821a2f9c3d2c96);
    }
    #[test]
    fn t_dist_xz() {
        one(super::dist_xz([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), 0x401b800000000000);
        one(super::dist_xz([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]), 0x0000000000000000);
        one(super::dist_xz([0.1, 0.2, 0.3], [0.6, 0.0, 0.8]), 0x3fe6a09e667f3bcd);
        one(super::dist_xz([1e-200, 2e-200, -1e-200], [0.0, 0.0, 0.0]), 0x167151f68876f410);
        one(super::dist_xz([f64::NAN, 1.0, 2.0], [0.0, 0.0, 0.0]), 0x7ff8000000000000);
    }
    #[test]
    fn t_yaw_of() {
        one(super::yaw_of([1.0, 2.0, 3.0]), 0x3fd4978fa3269ee1);
        one(super::yaw_of([-4.5, 0.25, 7.125]), 0xbfe206afd30d3731);
        one(super::yaw_of([0.0, 0.0, 0.0]), 0x0000000000000000);
        one(super::yaw_of([-0.0, -0.0, -0.0]), 0xc00921fb54442d18);
        one(super::yaw_of([-0.0, 0.0, -1.0]), 0xc00921fb54442d18);
        one(super::yaw_of([0.0, 0.0, -1.0]), 0x400921fb54442d18);
        one(super::yaw_of([-1.0, 0.0, 0.0]), 0xbff921fb54442d18);
        one(super::yaw_of([f64::NAN, 1.0, 2.0]), 0x7ff8000000000000);
        one(super::yaw_of([f64::INFINITY, 1.0, 0.0]), 0x3ff921fb54442d18);
    }
    #[test]
    fn t_yaw_to() {
        one(super::yaw_to([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), 0xbfedac670561bb4f);
        one(super::yaw_to([-4.5, 0.25, 7.125], [1.0, 2.0, 3.0]), 0x4001b6e192ebbe44);
        one(super::yaw_to([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]), 0x0000000000000000);
        one(super::yaw_to([0.1, 0.2, 0.3], [0.6, 0.0, 0.8]), 0x3fe921fb54442d18);
    }
    #[test]
    fn t_normalize() {
        many(&super::normalize([1.0, 2.0, 3.0]), &[0x3fd11acee560242a, 0x3fe11acee560242a, 0x3fe9a83658103640]);
        many(&super::normalize([-4.5, 0.25, 7.125]), &[0xbfe1148c01590a56, 0x3f9e5d6aad101261, 0x3feb0b330222505e]);
        many(&super::normalize([0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::normalize([-0.0, -0.0, -0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::normalize([0.1, 0.2, 0.3]), &[0x3fd11acee560242a, 0x3fe11acee560242a, 0x3fe9a8365810363e]);
        many(&super::normalize([1e+200, -1e+200, 1e+200]), &[0x3fe279a74590331c, 0xbfe279a74590331c, 0x3fe279a74590331c]);
        many(&super::normalize([1e-200, 2e-200, -1e-200]), &[0x3fda20bd700c2c3f, 0x3fea20bd700c2c3f, 0xbfda20bd700c2c3f]);
        many(&super::normalize([f64::NAN, 1.0, 2.0]), &[0x7ff8000000000000, 0x3ff0000000000000, 0x4000000000000000]);
        many(&super::normalize([f64::INFINITY, 1.0, 0.0]), &[0xfff8000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_lerp3() {
        many(&super::lerp3([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125], 0.3), &[0xbfe4cccccccccccc, 0x3ff799999999999a, 0x4010f33333333333]);
        many(&super::lerp3([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125], -1.0), &[0x401a000000000000, 0x400e000000000000, 0xbff2000000000000]);
        many(&super::lerp3([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125], 2.0), &[0xc024000000000000, 0xbff8000000000000, 0x4026800000000000]);
        many(&super::lerp3([-0.0, -0.0, -0.0], [0.0, 0.0, 0.0], 0.5), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_min() {
        many(&super::min([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), &[0xc012000000000000, 0x3fd0000000000000, 0x4008000000000000]);
        many(&super::min([0.0, 0.0, 0.0], [-0.0, -0.0, -0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::min([-0.0, -0.0, -0.0], [0.0, 0.0, 0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&super::min([f64::NAN, 1.0, 2.0], [0.0, 0.0, 0.0]), &[0x7ff8000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_max() {
        many(&super::max([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125]), &[0x3ff0000000000000, 0x4000000000000000, 0x401c800000000000]);
        many(&super::max([0.0, 0.0, 0.0], [-0.0, -0.0, -0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::max([-0.0, -0.0, -0.0], [0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&super::max([f64::NAN, 1.0, 2.0], [0.0, 0.0, 0.0]), &[0x7ff8000000000000, 0x3ff0000000000000, 0x4000000000000000]);
    }
    #[test]
    fn t_q_ident() {
        many(&quat::ident(), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
    }
    #[test]
    fn t_q_axis_angle() {
        many(&quat::axis_angle([0.0, 1.0, 0.0], 1.0), &[0x0000000000000000, 0x3fdeaee8744b05f0, 0x0000000000000000, 0x3fec1528065b7d50]);
        many(&quat::axis_angle([0.2672612419124244, 0.5345224838248488, 0.8017837257372733], -2.5), &[0xbfd03b6bef5b3f32, 0xbfe03b6bef5b3f32, 0xbfe85921e708decb, 0x3fd42e3dd88bd952]);
        many(&quat::axis_angle([1.0, 0.0, 0.0], 0.0), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&quat::axis_angle([0.0, 0.0, 1.0], 21.991148575128552), &[0x8000000000000000, 0x8000000000000000, 0xbff0000000000000, 0xbcbee2c2d963a100]);
        many(&quat::axis_angle([0.7071067811865475, 0.7071067811865475, 0.0], 1000000.0), &[0x3fc0186ef3f60db9, 0x3fc0186ef3f60db9, 0x0000000000000000, 0xbfef7d6d81d1c85d]);
        many(&quat::axis_angle([0.0, 0.0, 0.0], 1.0), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3fec1528065b7d50]);
    }
    #[test]
    fn t_q_yaw() {
        many(&quat::yaw(0.0), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&quat::yaw(-0.0), &[0x0000000000000000, 0x8000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&quat::yaw(1.0), &[0x0000000000000000, 0x3fdeaee8744b05f0, 0x0000000000000000, 0x3fec1528065b7d50]);
        many(&quat::yaw(-1.0), &[0x0000000000000000, 0xbfdeaee8744b05f0, 0x0000000000000000, 0x3fec1528065b7d50]);
        many(&quat::yaw(3.141592653589793), &[0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x3c91a62633145c00]);
        many(&quat::yaw(-3.141592653589793), &[0x0000000000000000, 0xbff0000000000000, 0x0000000000000000, 0x3c91a62633145c00]);
        many(&quat::yaw(9.0), &[0x0000000000000000, 0xbfef47ed3dc74080, 0x0000000000000000, 0xbfcafb5b54583d6b]);
        many(&quat::yaw(-20.0), &[0x0000000000000000, 0x3fe1689ef5f34f53, 0x0000000000000000, 0xbfead9ac890c6b1f]);
        many(&quat::yaw(10000000.0), &[0x0000000000000000, 0xbfef3fd5fdc1315e, 0x0000000000000000, 0xbfcb8fc4119417b8]);
    }
    #[test]
    fn t_q_mul() {
        many(&quat::mul([0.1, 0.2, 0.3, 0.9], [-0.5, 0.5, -0.5, 0.5]), &[0xbfe4cccccccccccd, 0x3fdccccccccccccc, 0xbfc3333333333335, 0x3fe199999999999a]);
        many(&quat::mul([-0.5, 0.5, -0.5, 0.5], [0.1, 0.2, 0.3, 0.9]), &[0xbfc3333333333333, 0x3fe4cccccccccccc, 0xbfdccccccccccccd, 0x3fe199999999999a]);
        many(&quat::mul([0.0, 0.0, 0.0, 1.0], [0.1, 0.2, 0.3, 0.9]), &[0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333, 0x3feccccccccccccd]);
        many(&quat::mul([0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&quat::mul([0.7071067811865476, 0.0, 0.0, 0.7071067811865476], [0.0, 0.3826834323650898, 0.0, 0.9238795325112867]), &[0x3fe4e7ae9144f0fc, 0x3fd1517a7bdb3895, 0x3fd1517a7bdb3895, 0x3fe4e7ae9144f0fc]);
    }
    #[test]
    fn t_q_conj() {
        many(&quat::conj([0.1, 0.2, 0.3, 0.9]), &[0xbfb999999999999a, 0xbfc999999999999a, 0xbfd3333333333333, 0x3feccccccccccccd]);
        many(&quat::conj([0.0, 0.0, 0.0, 1.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000, 0x3ff0000000000000]);
        many(&quat::conj([0.0, 0.0, 0.0, 0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_q_normalize() {
        many(&quat::normalize([0.1, 0.2, 0.3, 0.9]), &[0x3fba43da0adc689a, 0x3fca43da0adc689a, 0x3fd3b2e388254e73, 0x3fed8c554c37f5ad]);
        many(&quat::normalize([0.1001, 0.2, 0.3, 0.9]), &[0x3fba4a8133dab927, 0x3fca43c7ea25f307, 0x3fd3b2d5ef9c7645, 0x3fed8c40e76ab167]);
        many(&quat::normalize([0.0, 0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&quat::normalize([-0.0, -0.0, -0.0, -0.0]), &[0x8000000000000000, 0x8000000000000000, 0x8000000000000000, 0x8000000000000000]);
        many(&quat::normalize([f64::NAN, 0.0, 0.0, 1.0]), &[0x7ff8000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&quat::normalize([1e+200, 1e+200, 0.0, 0.0]), &[0x3fe6a09e667f3bcc, 0x3fe6a09e667f3bcc, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_q_dot() {
        one(quat::dot([0.1, 0.2, 0.3, 0.9], [-0.5, 0.5, -0.5, 0.5]), 0x3fd6666666666667);
        one(quat::dot([0.1, 0.2, 0.3, 0.9], [0.1, 0.2, 0.3, 0.9]), 0x3fee666666666667);
        one(quat::dot([0.0, 0.0, 0.0, 0.0], [-0.1, -0.2, -0.3, -0.9]), 0x8000000000000000);
    }
    #[test]
    fn t_q_nlerp() {
        many(&quat::nlerp([0.1, 0.2, 0.3, 0.9], [-0.5, 0.5, -0.5, 0.5], 0.3), &[0xbfb86f47ad08e3be, 0x3fd624d8f4d00e66, 0x3fb25375c1c6aacf, 0x3fedc79f5ae2d591]);
        many(&quat::nlerp([0.1, 0.2, 0.3, 0.9], [-0.1, -0.2, -0.3, -0.9], 0.3), &[0x3fba43da0adc689a, 0x3fca43da0adc689a, 0x3fd3b2e388254e73, 0x3fed8c554c37f5ad]);
        many(&quat::nlerp([0.1, 0.2, 0.3, 0.9], [-0.5, 0.5, -0.5, 0.5], -1.0), &[0x3fd84bd4808dcaf5, 0xbfabc46092eb3116, 0x3fe317026501b1c1, 0x3fe68f8e775f17e4]);
        many(&quat::nlerp([0.1, 0.2, 0.3, 0.9], [-0.5, 0.5, -0.5, 0.5], 2.0), &[0xbfe2aea5fbe2741f, 0x3fdb2c94571ad775, 0xbfe6143886c5cf0f, 0x3fab2c94571ad773]);
        many(&quat::nlerp([0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.5), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_q_slerp() {
        many(&quat::slerp([0.1, 0.2, 0.3, 0.9], [-0.5, 0.5, -0.5, 0.5], 0.3), &[0xbfbc1feca81fd957, 0x3fd66b6febff883d, 0x3fa9d29564e8acc8, 0x3fed29c02da09901]);
        many(&quat::slerp([0.1, 0.2, 0.3, 0.9], [-0.1, -0.2, -0.3, -0.9], 0.3), &[0x3fb9dfb4f8c0f6c8, 0x3fc9dfb4f8c0f6c8, 0x3fd367c7ba90b915, 0x3fed1bab97d915a0]);
        many(&quat::slerp([0.1, 0.2, 0.3, 0.9], [0.1001, 0.2, 0.3, 0.9], 0.5), &[0x3fb9f063e65db8f6, 0x3fc9ed125accfae2, 0x3fd371cdc419bc29, 0x3fed2ab4a6269a3e]);
        many(&quat::slerp([0.7071067811865476, 0.0, 0.0, 0.7071067811865476], [0.0, 0.3826834323650898, 0.0, 0.9238795325112867], 0.7), &[0x3fce768ab7bd7bc2, 0x3fd24c4cc88a59ed, 0x0000000000000000, 0x3fedb415ef557168]);
        many(&quat::slerp([0.7071067811865476, 0.0, 0.0, 0.7071067811865476], [0.0, 0.3826834323650898, 0.0, 0.9238795325112867], -0.5), &[0x3fecb3b5214f84e4, 0xbfcaf01008668a93, 0x0000000000000000, 0x3fd8e32190e3ccda]);
        many(&quat::slerp([0.7071067811865476, 0.0, 0.0, 0.7071067811865476], [0.0, 0.3826834323650898, 0.0, 0.9238795325112867], 1.5), &[0xbfd8e32190e3ccd8, 0x3fdf1119f0d0ff70, 0x0000000000000000, 0x3fe90ead4f2c7b8a]);
        many(&quat::slerp([0.0, 0.0, 0.0, 1.0], [0.7071067811865476, 0.0, 0.0, 0.7071067811865476], 0.0), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&quat::slerp([0.0, 0.0, 0.0, 1.0], [0.7071067811865476, 0.0, 0.0, 0.7071067811865476], 1.0), &[0x3fe6a09e667f3bcd, 0x0000000000000000, 0x0000000000000000, 0x3fe6a09e667f3bcd]);
        many(&quat::slerp([0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], 0.5), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_q_from_basis() {
        many(&quat::from_basis([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&quat::from_basis([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]), &[0x3fe0000000000000, 0x3fe0000000000000, 0x3fe0000000000000, 0x3fe0000000000000]);
        many(&quat::from_basis([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]), &[0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000]);
        many(&quat::from_basis([1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0]), &[0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&quat::from_basis([-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]), &[0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&quat::from_basis([0.6, 0.0, 0.8], [0.0, 1.0, 0.0], [-0.8, 0.0, 0.6]), &[0x0000000000000000, 0xbfdc9f25c5bfedda, 0x0000000000000000, 0x3fec9f25c5bfedd9]);
        many(&quat::from_basis([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_q_from_to() {
        many(&quat::from_to([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x3fe6a09e667f3bcc, 0x3fe6a09e667f3bcc]);
        many(&quat::from_to([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000]);
        many(&quat::from_to([0.0, 1.0, 0.0], [0.0, -1.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0xbff0000000000000, 0x0000000000000000]);
        many(&quat::from_to([0.2672612419124244, 0.5345224838248488, 0.8017837257372733], [-0.2672612419124244, -0.5345224838248488, -0.8017837257372733]), &[0x0000000000000000, 0x3feaa027f059dce0, 0xbfe1c01aa03be895, 0x0000000000000000]);
        many(&quat::from_to([0.2672612419124244, 0.5345224838248488, 0.8017837257372733], [0.8017837257372733, -0.2672612419124244, 0.5345224838248488]), &[0x3fd279a74590331c, 0x3fd279a74590331e, 0xbfd279a74590331c, 0x3febb67ae8584ca9]);
        many(&quat::from_to([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        many(&quat::from_to([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
    }
    #[test]
    fn t_q_rotate() {
        many(&quat::rotate([0.7071067811865476, 0.0, 0.0, 0.7071067811865476], [1.0, 2.0, 3.0]), &[0x3ff0000000000000, 0xc008000000000002, 0x3ffffffffffffffe]);
        many(&quat::rotate([0.1, 0.2, 0.3, 0.9], [-4.5, 0.25, 7.125]), &[0xbfdd999999999994, 0xc006b33333333333, 0x401f599999999998]);
        many(&quat::rotate([0.0, 0.0, 0.0, 1.0], [-0.0, -0.0, -0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&quat::rotate([0.0, 0.0, 0.0, 0.0], [0.1, 0.2, 0.3]), &[0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333]);
        many(&quat::rotate([0.0, 0.3826834323650898, 0.0, 0.9238795325112867], [0.6, 0.0, 0.8]), &[0x3fefadaa8f7eed52, 0x0000000000000000, 0x3fc21a1851ff630a]);
    }
    #[test]
    fn t_m_create() {
        singles(&m4::create(), &[0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000]);
    }
    #[test]
    fn t_m_copy() {
        singles(&m4::copy(&mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000])), &[0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000]);
        singles(&m4::copy(&mat([0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x7fc00000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000])), &[0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x7fc00000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000]);
    }
    #[test]
    fn t_m_mul() {
        singles(&m4::mul(&mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000]), &mat([0x3e275213, 0xbd5f45a5, 0xbe7b83f2, 0x00000000, 0x3f6d9b02, 0x3fb1f7ae, 0x3e9e26aa, 0x00000000, 0x3fab1e42, 0xbf92c19f, 0x3f9267d1, 0x00000000, 0xc0533333, 0x3f333333, 0x414e6666, 0x3f800000])), &[0x3d1ed63c, 0x3d93f1a5, 0xbe93c0a1, 0x00000000, 0x3da19dee, 0x3fd28927, 0x3ed83cec, 0x00000000, 0x40052058, 0xbe251402, 0x3e74e76b, 0x00000000, 0x408e033b, 0xc090dbe4, 0x414e7bf5, 0x3f800000]);
        singles(&m4::mul(&mat([0x3e275213, 0xbd5f45a5, 0xbe7b83f2, 0x00000000, 0x3f6d9b02, 0x3fb1f7ae, 0x3e9e26aa, 0x00000000, 0x3fab1e42, 0xbf92c19f, 0x3f9267d1, 0x00000000, 0xc0533333, 0x3f333333, 0x414e6666, 0x3f800000]), &mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000])), &[0x3e86be4b, 0x3f95ee25, 0xbeb3b7be, 0x00000000, 0x3f88d22f, 0x3f43a858, 0x3f3bfd24, 0x00000000, 0x3f9ad95e, 0xbf91a20e, 0x3f65320e, 0x00000000, 0xc0a04e31, 0xc027fbf4, 0x413f3716, 0x3f800000]);
        singles(&m4::mul(&mat([0x3f796a52, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3fddb3d7, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf80068e, 0xbf800000, 0x00000000, 0x00000000, 0xbdccd74a, 0x00000000]), &mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000])), &[0x3f35277f, 0x3f875aef, 0x3ea1b770, 0x3ea1af28, 0xbf03456f, 0x3faf073e, 0xbea1b770, 0xbea1af28, 0x3edc8920, 0xbde00945, 0xbf651934, 0xbf650d79, 0x3fbb0fbe, 0xc0796a52, 0xbe4cd74a, 0xbdcccccd]);
        singles(&m4::mul(&mat([0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000]), &mat([0x3e275213, 0xbd5f45a5, 0xbe7b83f2, 0x00000000, 0x3f6d9b02, 0x3fb1f7ae, 0x3e9e26aa, 0x00000000, 0x3fab1e42, 0xbf92c19f, 0x3f9267d1, 0x00000000, 0xc0533333, 0x3f333333, 0x414e6666, 0x3f800000])), &[0x3e275213, 0xbd5f45a5, 0xbe7b83f2, 0x00000000, 0x3f6d9b02, 0x3fb1f7ae, 0x3e9e26aa, 0x00000000, 0x3fab1e42, 0xbf92c19f, 0x3f9267d1, 0x00000000, 0xc0533333, 0x3f333333, 0x414e6666, 0x3f800000]);
        singles(&m4::mul(&mat([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]), &mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000])), &[0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
    }
    #[test]
    fn t_m_from_trs() {
        singles(&m4::from_trs([1.0, 2.0, 3.0], [0.1, 0.2, 0.3, 0.9], [1.0, 1.0, 1.0]), &[0x3f3d70a4, 0x3f147ae1, 0xbe99999a, 0x00000000, 0xbf000000, 0x3f4ccccd, 0x3e99999a, 0x00000000, 0x3ed70a3d, 0xbd75c28f, 0x3f666666, 0x00000000, 0x3f800000, 0x40000000, 0x40400000, 0x3f800000]);
        singles(&m4::from_trs([-4.5, 0.25, 7.125], [-0.5, 0.5, -0.5, 0.5], [0.1, 0.2, 0.3]), &[0x00000000, 0xbdcccccd, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbe4ccccd, 0x00000000, 0x3e99999a, 0x00000000, 0x00000000, 0x00000000, 0xc0900000, 0x3e800000, 0x40e40000, 0x3f800000]);
        singles(&m4::from_trs([0.1, 0.2, 0.3], [0.0, 0.3826834323650898, 0.0, 0.9238795325112867], [1.0, 1.0, 1.0]), &[0x3f3504f3, 0x00000000, 0xbf3504f3, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x3f3504f3, 0x00000000, 0x3f3504f3, 0x00000000, 0x3dcccccd, 0x3e4ccccd, 0x3e99999a, 0x3f800000]);
        singles(&m4::from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0]), &[0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000]);
        singles(&m4::from_trs([1e+39, -1e+39, 1e-50], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), &[0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x7f800000, 0xff800000, 0x00000000, 0x3f800000]);
    }
    #[test]
    fn t_m_from_trss() {
        singles(&m4::from_trss(0.1, 0.2, 0.3, 0.1, 0.2, 0.3, 0.9, 1.1, 1.2, 1.3), &[0x3f50624e, 0x3f2353f8, 0xbea8f5c3, 0x00000000, 0xbf19999a, 0x3f75c28f, 0x3eb851ec, 0x00000000, 0x3f0bc6a8, 0xbd9fbe77, 0x3f95c28f, 0x00000000, 0x3dcccccd, 0x3e4ccccd, 0x3e99999a, 0x3f800000]);
        singles(&m4::from_trss(1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0), &[0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x3f800000, 0x40000000, 0x40400000, 0x3f800000]);
    }
    #[test]
    fn t_m_transform_point() {
        many(&m4::transform_point(&mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000]), [1.0, 2.0, 3.0]), &[0x4003fffffc000000, 0xbfcfffff70000000, 0x4008ccccc5000000]);
        many(&m4::transform_point(&mat([0x3e275213, 0xbd5f45a5, 0xbe7b83f2, 0x00000000, 0x3f6d9b02, 0x3fb1f7ae, 0x3e9e26aa, 0x00000000, 0x3fab1e42, 0xbf92c19f, 0x3f9267d1, 0x00000000, 0xc0533333, 0x3f333333, 0x414e6666, 0x3f800000]), [0.1, 0.2, 0.3]), &[0xc00593664f666666, 0x3fe41e0205666666, 0x402a8f8a7719999a]);
        many(&m4::transform_point(&mat([0x3f796a52, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3fddb3d7, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf80068e, 0xbf800000, 0x00000000, 0x00000000, 0xbdccd74a, 0x00000000]), [-4.5, 0.25, 7.125]), &[0x3fe3b0d09435e50d, 0xbfaf1dbb5e50d794, 0x3ff03a518ce98b3a]);
        many(&m4::transform_point(&mat([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]), [1.0, 2.0, 3.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
        many(&m4::transform_point(&mat([0x3f796a52, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3fddb3d7, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf80068e, 0xbf800000, 0x00000000, 0x00000000, 0xbdccd74a, 0x00000000]), [1.0, 2.0, 0.0]), &[0x3fef2d4a40000000, 0x400bb67ae0000000, 0xbfb99ae940000000]);
        many(&m4::transform_point(&mat([0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000]), [-0.0, -0.0, -0.0]), &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000]);
    }
    #[test]
    fn t_m_get_translation() {
        many(&m4::get_translation(&mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000])), &[0x3ff8000000000000, 0xc002000000000000, 0x3fb99999a0000000]);
        many(&m4::get_translation(&mat([0x3e275213, 0xbd5f45a5, 0xbe7b83f2, 0x00000000, 0x3f6d9b02, 0x3fb1f7ae, 0x3e9e26aa, 0x00000000, 0x3fab1e42, 0xbf92c19f, 0x3f9267d1, 0x00000000, 0xc0533333, 0x3f333333, 0x414e6666, 0x3f800000])), &[0xc00a666660000000, 0x3fe6666660000000, 0x4029ccccc0000000]);
    }
    #[test]
    fn t_m_invert() {
        singles(&m4::invert(&mat([0x3f39efd5, 0x3f1c4b74, 0xbea1af28, 0x00000000, 0xbf06bca2, 0x3f4a1af3, 0x3ea1af28, 0x00000000, 0x3ee25b9f, 0xbd8158ed, 0x3f650d79, 0x00000000, 0x3fc00000, 0xc0100000, 0x3dcccccd, 0x3f800000])).unwrap(), &[0x3f39efd5, 0xbf06bca1, 0x3ee25b9e, 0x00000000, 0x3f1c4b74, 0x3f4a1af2, 0xbd8158ec, 0x00000000, 0xbea1af29, 0x3ea1af28, 0x3f650d7a, 0x80000000, 0x3ea1af28, 0x40223081, 0xbf650d79, 0x3f800000]);
        singles(&m4::invert(&mat([0x3e275213, 0xbd5f45a5, 0xbe7b83f2, 0x00000000, 0x3f6d9b02, 0x3fb1f7ae, 0x3e9e26aa, 0x00000000, 0x3fab1e42, 0xbf92c19f, 0x3f9267d1, 0x00000000, 0xc0533333, 0x3f333333, 0x414e6666, 0x3f800000])).unwrap(), &[0x3fe863c6, 0x3ea46ed1, 0x3e9b3592, 0x80000000, 0xbf1b0cd0, 0x3ef65262, 0xbe851cc2, 0x00000000, 0xc02ea9da, 0x3ddae4e8, 0x3e84cb4c, 0x00000000, 0x42267bc5, 0xbf27dd7f, 0xc00a759b, 0x3f800000]);
        singles(&m4::invert(&mat([0x3f796a52, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3fddb3d7, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf80068e, 0xbf800000, 0x00000000, 0x00000000, 0xbdccd74a, 0x00000000])).unwrap(), &[0x3f836117, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0x3f13cd3a, 0x00000000, 0x80000000, 0x80000000, 0x80000000, 0x80000000, 0xc11ff7cf, 0x80000000, 0x80000000, 0xbf800000, 0x41200000]);
        assert!(m4::invert(&mat([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000])).is_none());
        assert!(m4::invert(&mat([0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x7fc00000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000])).is_none());
        singles(&m4::invert(&mat([0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000])).unwrap(), &[0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000]);
    }
    #[test]
    fn t_m_perspective() {
        singles(&m4::perspective(1.0471975511965976, 1.7777777777777777, 0.1, 500.0), &[0x3f796a52, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3fddb3d7, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf80068e, 0xbf800000, 0x00000000, 0x00000000, 0xbdccd74a, 0x00000000]);
        singles(&m4::perspective(0.7, 1.0, 1.0, 2.0), &[0x402f542b, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x402f542b, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xc0000000, 0xbf800000, 0x00000000, 0x00000000, 0xc0000000, 0x00000000]);
        singles(&m4::perspective(3.0, 0.5, 0.01, 10000.0), &[0x3e113bcd, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3d913bcd, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf800008, 0xbf800000, 0x00000000, 0x00000000, 0xbc23d715, 0x00000000]);
    }
    #[test]
    fn t_m_look_at() {
        singles(&m4::look_at([1.0, 2.0, 3.0], [-4.5, 0.25, 7.125], [0.0, 1.0, 0.0]), &[0xbf19999a, 0xbe4a1465, 0x3f46789a, 0x00000000, 0x00000000, 0x3f7816c1, 0x3e7c997e, 0x00000000, 0xbf4ccccd, 0x3e178f4c, 0xbf14da74, 0x00000000, 0x40400000, 0xc00bd4f8, 0x3ef39403, 0x3f800000]);
        singles(&m4::look_at([0.0, 10.0, 0.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]), &[0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x80000000, 0x80000000, 0xc1200000, 0x3f800000]);
        singles(&m4::look_at([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]), &[0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x80000000, 0x80000000, 0x80000000, 0x3f800000]);
        singles(&m4::look_at([0.1, 0.2, 0.3], [0.6, 0.0, 0.8], [0.1, 0.9, 0.2]), &[0xbf384066, 0x3e0d3ae9, 0xbf2e2f9a, 0x00000000, 0xbd9668d1, 0x3f759e1a, 0x3e8b5948, 0x00000000, 0x3f30bb29, 0x3e7bc20e, 0xbf2e2f9a, 0x00000000, 0xbdf6abe6, 0xbe8f127e, 0x3e5ef53f, 0x3f800000]);
    }
}
