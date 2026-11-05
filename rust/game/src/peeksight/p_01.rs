//! src/game/peeksight.ts lines 1 to 63
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The player's eye while he peeks round the end of a wall, in place of his own (`own`), or None: no hold, no peek in.   (peeksight.ts peekEye)
pub fn peek_eye(g: &mut crate::game::Game, own: crate::fogofwar::FogEye) -> Option<crate::fogofwar::FogEye> {
    let pl = g.player;
    let w = g.pl[pl].wall?;
    if !(w.peek > 0.0) || g.pl[pl].down { return None; }
    let (out, off, turn_deg) = (g.k.peeksight.out, g.k.peeksight.off, g.k.peeksight.turn_deg);
    let e = ease(w.peek);
    let (s, t, n) = (w.peek_dir, w.along, w.n);
    let (along, inward) = lean(own.at, s, t, n, w.d, w.lo, w.hi, out, off);
    let mut reach = e;
    if crate::scene_collision::solid_blocked(g, own.at, lean_at(own.at, t, n, along, inward, reach), None) {
        let (mut lo, mut hi) = (0.0, reach);
        for _ in 0..5 {
            let mid = (lo + hi) / 2.0;
            if crate::scene_collision::solid_blocked(g, own.at, lean_at(own.at, t, n, along, inward, mid), None) { hi = mid; } else { lo = mid; }
        }
        reach = m::max(0.0, lo - 0.08);
        g.stats.peeksight.short += 1;
    }
    g.stats.peeksight.frames += 1;
    let p = lean_at(own.at, t, n, along, inward, reach);
    if matches!(w.kind, Some(crate::player::WallHoldKind::Stack)) { return Some(eye_at(own, p, own.look)); }
    Some(eye_at(own, p, turned_look(own.look, s, t, n, e, turn_deg)))
}

/// (peeksight.ts:40 `e`)
fn ease(peek: f64) -> f64 {
    peek * peek * (3.0 - 2.0 * peek)
}

/// How far along the wall and how far toward its plane the head goes at full peek.   (peeksight.ts:42-43)
fn lean(own_at: V3, s: f64, t: V3, n: V3, d: f64, lo: f64, hi: f64, out: f64, off: f64) -> (f64, f64) {
    let u = own_at[0] * t[0] + own_at[2] * t[2];
    let dist = own_at[0] * n[0] + own_at[2] * n[2] - d;
    let along = m::max(0.0, if s > 0.0 { hi + out - u } else { u - (lo - out) }) * s;
    let inward = m::max(0.0, dist - off);
    (along, inward)
}

/// The head's place a share `k` of the way out.   (peeksight.ts:44 the closure `at`)
fn lean_at(own_at: V3, t: V3, n: V3, along: f64, inward: f64, k: f64) -> V3 {
    [own_at[0] + (t[0] * along - n[0] * inward) * k, own_at[1], own_at[2] + (t[2] * along - n[2] * inward) * k]
}

/// The look turned with the peek, by angle, from his own to `turn_deg`: by way of 'along the wall', never the room behind him.   (peeksight.ts:54-56)
fn turned_look(own_look: V3, s: f64, t: V3, n: V3, e: f64, turn_deg: f64) -> V3 {
    let ox = (own_look[0] * t[0] + own_look[2] * t[2]) * s;
    let oy = -(own_look[0] * n[0] + own_look[2] * n[2]);
    let from = m::atan2(oy, ox);
    let a = from + (turn_deg * m::PI / 180.0 - from) * e;
    let cs = m::cos(a) * s;
    let sn = m::sin(a);
    [t[0] * cs - n[0] * sn, 0.0, t[2] * cs - n[2] * sn]
}

/// The eye the peek gives: `pos` and `at` both the head's place; fov, near and wand absent, as the TypeScript's literal leaves them.   (peeksight.ts:51,56)
fn eye_at(own: crate::fogofwar::FogEye, p: V3, look: V3) -> crate::fogofwar::FogEye {
    crate::fogofwar::FogEye { id: own.id, pos: p, look, at: p, fov: None, near: None, wand: None }
}

/// A line for the debugger: PEEKSIGHT.on is settled on the page, so always "on".   (peeksight.ts peekSummary)
pub fn peek_summary(g: &mut crate::game::Game) -> String {
    let peeking = match g.pl[g.player].wall {
        Some(w) if w.peek > 0.0 => alloc::format!(" · peeking {}%", js::to_i32(w.peek * 100.0)),
        _ => String::new(),
    };
    let stats = &g.stats.peeksight;
    alloc::format!("on{} · frames {}, stopped short {}", peeking, stats.frames, stats.short)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The arithmetic with nothing in the way, against the TypeScript's own bits (bun, EXACT.on, page knobs out 0.3, off 0.1, turnDeg 90).
    fn check(peek: f64, s: f64, t: V3, n: V3, d: f64, lo: f64, hi: f64, at: V3, look: V3, want: [u64; 6]) {
        let e = ease(peek);
        let (along, inward) = lean(at, s, t, n, d, lo, hi, 0.3, 0.1);
        let p = lean_at(at, t, n, along, inward, e);
        let l = turned_look(look, s, t, n, e, 90.0);
        let got = [p[0], p[1], p[2], l[0], l[1], l[2]];
        for i in 0..6 { assert_eq!(got[i].to_bits(), want[i], "component {}", i); }
    }

    #[test]
    fn eye_matches_the_typescript_going_up_the_wall() {
        check(0.37, 1.0, [0.8, 0.0, 0.6], [-0.6, 0.0, 0.8], 1.25, -2.5, 3.75, [1.1, 1.6, 2.3], [0.31, 0.0, -0.95],
            [0x3ff8b057831616e5, 0x3ff999999999999a, 0x40050eed9df51562, 0x3fd9eabfcebfcba3, 0x0000000000000000, 0xbfed4240c2d9e128]);
    }

    #[test]
    fn eye_matches_the_typescript_going_down_the_wall_at_full_peek() {
        check(1.0, -1.0, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], -4.2, 7.1, 12.9, [8.4, 1.1, -3.9], [-0.7071, 0.0, 0.7071],
            [0x401b333333333333, 0x3ff199999999999a, 0xc010666666666667, 0xbc91a62633145c00, 0x0000000000000000, 0xbff0000000000000]);
    }
}
