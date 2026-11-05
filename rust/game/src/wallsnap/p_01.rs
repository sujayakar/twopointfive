//! src/game/wallsnap.ts lines 1 to 128
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Within 0..1; a NaN stays a NaN. (wallsnap.ts clamp01)
pub fn clamp01(x: f64) -> f64 {
    if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
}

/// Seconds the take runs for a glide of `dist` metres and a turn of `turn` radians. (wallsnap.ts wsTakeSecs)
pub fn ws_take_secs(g: &mut crate::game::Game, dist: f64, turn: f64) -> f64 {
    take_secs(&g.k.wallsnap, dist, turn)
}

fn take_secs(w: &crate::knobs::WallsnapK, dist: f64, turn: f64) -> f64 {
    let t = w.take_base + w.take_per_m * dist + w.take_per_turn * m::abs(turn) / m::PI;
    m::min(w.take_max, m::max(w.take_min, t)) * (if w.slow { m::max(0.25, w.slow_take) } else { 1.0 })
}

/// How far out (m) the key-less take's clock starts for a man closing on the face at `into` m/s. (wallsnap.ts wsPressNear)
pub fn ws_press_near(g: &mut crate::game::Game, into: f64) -> f64 {
    press_near(&g.k.wallsnap, into)
}

fn press_near(w: &crate::knobs::WallsnapK, into: f64) -> f64 {
    if w.press_run { m::max(w.press_near, w.press_flow + w.press_secs * m::max(0.0, into)) } else { w.press_near }
}

/// Ease-out by power p: 0 to 0, 1 to 1, fastest at the start. (wallsnap.ts wsEase)
pub fn ws_ease(s: f64, p: f64) -> f64 {
    1.0 - m::pow(1.0 - clamp01(s), p)
}

/// The landing's settle, t seconds after the back arrived. (wallsnap.ts wsHit)
pub fn ws_hit(g: &mut crate::game::Game, t: f64) -> f64 {
    hit(g.k.wallsnap.hit_peak, t)
}

fn hit(a: f64, t: f64) -> f64 {
    if t < 0.0 || t > 8.0 * a { 0.0 } else { (t / a) * m::exp(1.0 - t / a) }
}

/// The cursor's point about a wall's end: angle (degrees) from the wall's line carried on, and distance from the corner. (wallsnap.ts wsCornerAngle)
pub fn ws_corner_angle(a: f64, b: f64) -> crate::wallsnap::WsCornerAngle {
    crate::wallsnap::WsCornerAngle { deg: m::atan2(-b, a) * 180.0 / m::PI, dist: m::hyp2(a, b) }
}

/// The angle (degrees) up to which a man `back` short of the end and `off` the plane already sees past the corner. (wallsnap.ts wsSeenDeg)
pub fn ws_seen_deg(back: f64, off: f64) -> f64 {
    m::atan2(off, back) * 180.0 / m::PI
}

#[cfg(test)]
mod tests {
    use super::*;
    /// The TypeScript's own bits under EXACT, with the page's rows (slow and pressRun on), then the rows flipped.
    #[test]
    fn take_secs_bits() {
        let mut w = crate::knobs::page_wallsnap();
        let on: [(f64, f64, u64); 5] = [
            (0.7, 0.9, 0x3fdd560e7241d318),
            (0.0, 0.0, 0x3fceb851eb851eb8),
            (3.3, -3.1, 0x3fe3333333333333),
            (0.25, 1.234, 0x3fd4efc903b94307),
            (0.123, 2.5, 0x3fd56e3b22544036),
        ];
        for (d, t, bits) in on { assert_eq!(take_secs(&w, d, t).to_bits(), bits); }
        w.slow = false;
        let off: [(f64, f64, u64); 4] = [
            (0.7, 0.9, 0x3fcd560e7241d318),
            (0.0, 0.0, 0x3fbeb851eb851eb8),
            (3.3, -3.1, 0x3fd3333333333333),
            (0.25, 1.234, 0x3fc4efc903b94307),
        ];
        for (d, t, bits) in off { assert_eq!(take_secs(&w, d, t).to_bits(), bits); }
        w.slow = true;
        w.slow_take = 0.1; // the floor of a quarter
        assert_eq!(take_secs(&w, 0.25, 1.234).to_bits(), 0x3fa4efc903b94307);
    }

    #[test]
    fn press_near_bits() {
        let mut w = crate::knobs::page_wallsnap();
        for (i, bits) in [(0.0, 0x3fe3333333333333u64), (1.55, 0x3fe3333333333333), (4.7, 0x3ff0bc6a7ef9db23), (-2.0, 0x3fe3333333333333)] {
            assert_eq!(press_near(&w, i).to_bits(), bits);
        }
        w.press_run = false;
        assert_eq!(press_near(&w, 4.7).to_bits(), 0x3fe3333333333333);
    }

    #[test]
    fn ease_bits() {
        for (s, p, bits) in [
            (0.125, 2.0, 0x3fce000000000000u64),
            (0.37, 3.0, 0x3fe7ff9d6f112fd3),
            (-1.0, 2.0, 0x0000000000000000),
            (2.0, 3.0, 0x3ff0000000000000),
            (0.777, 2.5, 0x3fef3f9f9b4fedf8),
        ] {
            assert_eq!(ws_ease(s, p).to_bits(), bits);
        }
    }

    #[test]
    fn hit_bits() {
        let a = crate::knobs::page_wallsnap().hit_peak;
        for (t, bits) in [
            (0.0, 0x0000000000000000u64),
            (0.05, 0x3ff0000000000000),
            (0.013, 0x3fe1702ccae6cb7b),
            (0.31, 0x3fa1830426ca6c44),
            (0.4, 0x3f7de16b9c24a98f),
            (0.4000001, 0x0000000000000000),
            (-0.1, 0x0000000000000000),
        ] {
            assert_eq!(hit(a, t).to_bits(), bits);
        }
    }

    #[test]
    fn corner_bits() {
        for (a, b, deg, dist) in [
            (1.3, -0.7, 0x403c4cfe547314b1u64, 0x3ff79fabe917e2f5u64),
            (-0.4, 0.9, 0xc05c7d996b5b24ba, 0x3fef842f2f055704),
            (0.0, 0.0, 0x8000000000000000, 0x0000000000000000),
            (0.61, 2.2, 0xc052a02d2b3f86aa, 0x40024396c45e848d),
        ] {
            let r = ws_corner_angle(a, b);
            assert_eq!((r.deg.to_bits(), r.dist.to_bits()), (deg, dist));
        }
    }

    #[test]
    fn seen_bits() {
        for (back, off, bits) in [
            (0.6, 0.35, 0x403e41a5ddaed655u64),
            (0.0, 0.2, 0x4056800000000000),
            (1.7, -0.3, 0xc0240415edb47031),
        ] {
            assert_eq!(ws_seen_deg(back, off).to_bits(), bits);
        }
    }

    #[test]
    fn clamp_keeps_nan() {
        assert_eq!(clamp01(-0.5), 0.0);
        assert_eq!(clamp01(0.5), 0.5);
        assert_eq!(clamp01(1.5), 1.0);
        assert!(clamp01(f64::NAN).is_nan());
    }
}
