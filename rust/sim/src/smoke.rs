//! THE SMOKE ALONG A LINE — src/game/sense.ts CpuSmoke.trans: how much of the light along a → b comes through the game's own smoke (a gaussian blob an
//! emitter, spreading and thinning), a blob the other side of a wall from the line counting for nothing. The rule is the TypeScript's, statement for
//! statement. The blobs themselves (what is let out, the step) stay the host's: it writes the live ones before it asks, and they are scratch of that question.
//!
//! exp and the lines' lengths are the game's own, so this answers only where EXACT.on.

use crate::lines;
use crate::maths::exp;
use crate::world::{ASK_HOST, js_max, js_min, F_GLASS};
use prim::sqrt;

/// blobs one question can carry (sense.ts SENSE.smoke.cap is 48)
pub const MAX_BLOBS: usize = 64;
/// DOUBLES A BLOB: where it was let out (x y z), its width² at birth, its age, what it holds
pub const BLOB: usize = 6;

static mut BLOBS: [f64; MAX_BLOBS * BLOB] = [0.0; MAX_BLOBS * BLOB];

/// where a question's blobs are written (a byte address)
pub fn blobs_addr() -> usize {
    core::ptr::addr_of!(BLOBS) as usize
}

const PI: f64 = 3.141592653589793;

/// sense.ts erf (Abramowitz & Stegun 7.1.26) on the game's own exp
#[inline]
fn erf(x: f64) -> f64 {
    let s = if x < 0.0 { -1.0 } else { 1.0 };
    let a = if x < 0.0 { -x } else { x };
    let t = 1.0 / (1.0 + 0.3275911 * a);
    s * (1.0 - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t + 0.254829592) * t * exp(-a * a))
}

/// CpuSmoke.trans over the first `n` blobs (the host answers 1 itself when it holds none). The model's numbers come with the question (SENSE.smoke: `sigma`,
/// `spread`, `widest`, `rise`, `rise_rate`). The transmittance goes to `out`. 0, or ASK_HOST.
#[allow(clippy::too_many_arguments)]
pub fn trans(w: usize, set: i32, n: usize, a: [f64; 3], b: [f64; 3], sigma: f64, spread: f64, widest: f64, rise: f64, rise_rate: f64, out: &mut f64) -> u32 {
    let blobs = unsafe { &*core::ptr::addr_of!(BLOBS) };
    if n > MAX_BLOBS {
        return ASK_HOST;
    }
    let (dx, dy, dz) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    let len = sqrt(dx * dx + dy * dy + dz * dz);
    if len < 1e-6 {
        *out = 1.0;
        return 0;
    }
    let (ux, uy, uz) = (dx / len, dy / len, dz / len);
    let mut tau = 0.0;
    for i in 0..n {
        let bl = &blobs[i * BLOB..(i + 1) * BLOB];
        let (bx, by0, bz, r2, age, mass) = (bl[0], bl[1], bl[2], bl[3], bl[4], bl[5]);
        if mass <= 0.0 {
            continue;
        }
        let s2 = js_min(r2 + 2.0 * spread * age, widest * widest);
        let s = sqrt(s2);
        let cy = by0 + js_min(rise, rise_rate * age);
        let (px, py, pz) = (bx - a[0], cy - a[1], bz - a[2]);
        let t = px * ux + py * uy + pz * uz;
        let d2 = px * px + py * py + pz * pz - t * t;
        if d2 > 9.0 * s2 || t < -3.0 * s || t > len + 3.0 * s {
            continue;
        }
        let tc = if t < 0.0 {
            0.0
        } else if t > len {
            len
        } else {
            t
        };
        match lines::sight_line(w, set, F_GLASS, bx, cy, bz, a[0] + ux * tc, a[1] + uy * tc, a[2] + uz * tc) {
            None => return ASK_HOST,
            Some(true) => continue,
            Some(false) => {}
        }
        tau += sigma * mass / (PI * s2) * exp(-js_max(d2, 0.0) / s2) * 0.5 * (erf((len - t) / s) - erf(-t / s));
    }
    *out = if tau <= 0.0 { 1.0 } else { exp(-tau) };
    0
}
