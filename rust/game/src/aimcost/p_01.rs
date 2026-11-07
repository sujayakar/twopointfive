//! src/game/aimcost.ts lines 1 to 119
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Did a round flying these legs MEET a fixture (its point `pos`, its own boxes) under the real-hit rule? `stopped`: its flight ended in one of the fixture's boxes. (aimcost.ts lampStruck)
pub fn lamp_struck(g: &mut crate::game::Game, pos: V3, boxes: &[Option<BoxId>], stopped: bool, legs: &[crate::aimcost::RoundLeg], bare: Option<f64>, slack: Option<f64>) -> bool {
    let bare = bare.unwrap_or(g.k.aimcost.bare);
    let slack = slack.unwrap_or(g.k.aimcost.slack);
    if stopped { return true; }
    let pad = g.k.aimcost.pad;
    for s in legs {
        let reach = s.end + slack;
        let to_t = v3::sub(pos, s.o);
        let depth = v3::dot(to_t, s.d);
        if depth <= reach && depth >= -bare {
            let along = v3::clamp(depth, 0.0, reach);
            if v3::len(v3::sub(to_t, v3::scale(s.d, along))) < bare { return true; }
        }
        for b in boxes {
            let Some(b) = *b else { continue };
            let (c, h) = seam_box_c_h(g, b);
            let to_b = v3::sub(c, s.o);
            let db = v3::dot(to_b, s.d);
            let r = m::hyp3(h[0], h[1], h[2]) + pad;
            if db > reach + r || db < -r { continue; }
            if v3::len(v3::sub(to_b, v3::scale(s.d, db))) > r { continue; }
            let grown = [h[0] + pad, h[1] + pad, h[2] + pad];
            if ray_grown(g, s.o, s.d, b, grown, reach) { return true; }
            // A ray that STARTS inside the box gets no answer from rayBox: no way in ahead, and a way in coming back from past it, is exactly that.
            let l = 2.0 * r + 0.01;
            if !ray_grown(g, s.o, s.d, b, grown, l) && ray_grown(g, v3::add(s.o, v3::scale(s.d, l)), v3::scale(s.d, -1.0), b, grown, l) { return true; }
        }
    }
    false
}

/// `rayBox(ro, rd, grown, tmax) != null`, `grown` being a copy of `b` with half sizes `h` (aimcost.ts:91 to 96). `h` is the only field that differs, so the row itself takes it for the one
/// call and has its own back before anything else can read it. `copy_box` is not used: its answer is a row of `g.col.boxes`, which never shrinks, and a shot must not add to the world.
fn ray_grown(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, h: V3, tmax: f64) -> bool {
    let (_, own) = seam_box_c_h(g, b);
    seam_box_set_h(g, b, h);
    let hit = crate::scene_boxes::ray_box(g, ro, rd, b, tmax).is_some();
    seam_box_set_h(g, b, own);
    hit
}

/// `b.c`, `b.h` of a box's row. (aimcost.ts:88)
fn seam_box_c_h(g: &mut crate::game::Game, b: BoxId) -> (V3, V3) { let x = &g.tab.boxes[b]; (x.c, x.h) }

/// `grown.h = h`, on the row itself for the one call. (aimcost.ts:91)
fn seam_box_set_h(g: &mut crate::game::Game, b: BoxId, h: V3) { g.tab.boxes[b].h = h; }

/// How near a round's legs pass a point (m; the legs' own lengths, nothing past where it stopped): the cheap test ahead of `lamp_struck`. (aimcost.ts legsNear)
pub fn legs_near(pos: V3, legs: &[crate::aimcost::RoundLeg]) -> f64 {
    let mut best = m::INF;
    for s in legs {
        let to_t = v3::sub(pos, s.o);
        let d = v3::len(v3::sub(to_t, v3::scale(s.d, v3::clamp(v3::dot(to_t, s.d), 0.0, s.end))));
        if d < best { best = d; }
    }
    best
}

/// The feet a man's hands still remember: the fastest he has gone, let go over `stop_tau` (and to nothing: `stop_bleed`). (aimcost.ts feetHold)
pub fn feet_hold(g: &mut crate::game::Game, prev: f64, speed: f64, dt: f64) -> f64 {
    let kept = prev * m::exp(-dt / m::max(g.k.aimcost.stop_tau, 1e-3)) - g.k.aimcost.stop_bleed * dt;
    let v = m::max(speed, kept);
    if v.is_finite() && v > 1e-4 { v } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aimcost::RoundLeg;

    /// Expected bits and booleans are the TypeScript's own, run under bun with EXACT on.
    fn legs() -> [RoundLeg; 2] {
        [RoundLeg { o: [1.25, 1.7, -3.1], d: [0.6, 0.0, 0.8], end: 7.3 }, RoundLeg { o: [5.625, 1.7, 2.5], d: [-0.8, 0.6, 0.0], end: 2.2 }]
    }

    #[test]
    fn legs_near_bits() {
        let l = legs();
        assert_eq!(legs_near([3.1, 2.2, 1.9], &l).to_bits(), 0x3ff45988b14c549a);
        assert_eq!(legs_near([-4.0, 0.3, 0.1], &l).to_bits(), 0x40193916e14beafc);
        assert_eq!(legs_near([20.0, 1.0, 20.0], &l).to_bits(), 0x40367848963bd91a);
        assert_eq!(legs_near([1.0, 2.0, 3.0], &[]).to_bits(), 0x7ff0000000000000);
    }

}
