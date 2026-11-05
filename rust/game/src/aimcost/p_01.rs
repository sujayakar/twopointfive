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
            let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
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
    let own = g.col.boxes[b].h;
    g.col.boxes[b].h = h;
    let hit = crate::scene_boxes::ray_box(g, ro, rd, b, tmax).is_some();
    g.col.boxes[b].h = own;
    hit
}

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

/// The status line, on the page's branch of both switches (`AIMCOST.lamp`, `AIMCOST.move` are settled on). (aimcost.ts aimCostSummary)
pub fn aim_cost_summary(g: &mut crate::game::Game) -> String {
    let k = &g.k.aimcost;
    alloc::format!("lamps: a real hit (bulb {} m) \u{b7} moving \u{d7}{}, held {} s \u{b7} lamps hit {}, passed near {}", k.bare, k.move_k, k.stop_tau, g.stats.aimcost.lamp_hits, g.stats.aimcost.lamp_near)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aimcost::RoundLeg;

    /// A blank game: its knob tables are at the page's values.
    fn with_game(f: fn(&mut crate::game::Game)) {
        let mut g = alloc::boxed::Box::new(crate::game::Game::blank());
        f(&mut g);
    }

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

    #[test]
    fn feet_hold_bits() { with_game(feet_hold_bits_on); }
    fn feet_hold_bits_on(g: &mut crate::game::Game) {
        for (prev, speed, dt, want) in [
            (3.4, 0.0, 1.0 / 60.0, 0x4009ebfa068ec7e0u64),
            (0.05, 0.0, 1.0 / 60.0, 0x3fa78e6262219693),
            (2.5, 1.1, 1.0 / 120.0, 0x400385d40ffd852d),
            (1.2, 1.5, 0.1, 0x3ff8000000000000),
            (0.00011, 0.0, 0.016, 0),
            (f64::INFINITY, 0.0, 0.1, 0),
            (f64::NAN, 1.0, 0.1, 0),
            (1.0, 0.0, 0.0, 0x3ff0000000000000),
        ] {
            assert_eq!(feet_hold(g, prev, speed, dt).to_bits(), want, "{prev} {speed} {dt}");
        }
    }

    /// The bulb's own point only (no boxes, so rayBox's stub is not reached).
    #[test]
    fn lamp_struck_point() { with_game(lamp_struck_point_on); }
    fn lamp_struck_point_on(g: &mut crate::game::Game) {
        let l = legs();
        let past = [1.25 + 0.6 * 7.4, 1.7, -3.1 + 0.8 * 7.4];
        assert!(!lamp_struck(g, [3.1, 2.2, 1.9], &[], false, &l, None, None));
        assert!(lamp_struck(g, [3.1, 2.2, 1.9], &[], true, &l, None, None));
        assert!(lamp_struck(g, past, &[], false, &l, None, None));
        assert!(!lamp_struck(g, past, &[], false, &l, None, Some(0.05)));
        let one = &l[..1];
        assert_eq!(lamp_struck(g, [1.35, 1.8, -3.1], &[], false, one, None, None), LAMP[0]);
        assert_eq!(lamp_struck(g, [1.35, 1.8, -3.1], &[], false, one, Some(0.16), None), LAMP[1]);
        assert_eq!(lamp_struck(g, [1.25 - 0.06, 1.7, -3.1 - 0.08], &[], false, one, None, None), LAMP[2]);
        assert_eq!(lamp_struck(g, [1.25 - 0.6 * 0.2, 1.7, -3.1 - 0.8 * 0.2], &[], false, one, None, None), LAMP[3]);
    }
    const LAMP: [bool; 4] = [false, true, true, false];

    /// A box the line stays well clear of is passed over by the cheap tests, before any rayBox; a missing box is skipped.
    #[test]
    fn lamp_struck_far_box() { with_game(lamp_struck_far_box_on); }
    fn lamp_struck_far_box_on(g: &mut crate::game::Game) {
        let mut row = crate::scene_boxes::Box::zeroed();
        row.c = [50.0, 1.7, 50.0];
        row.h = [0.2, 0.2, 0.2];
        let far = g.col.boxes.push(row);
        let l = legs();
        assert!(!lamp_struck(g, [30.0, 1.7, 30.0], &[None, Some(far)], false, &l, None, None));
        assert_eq!(g.col.boxes[far].h, [0.2, 0.2, 0.2]);
    }

    #[test]
    fn summary_reads_as_the_page() { with_game(summary_on); }
    fn summary_on(g: &mut crate::game::Game) {
        g.stats.aimcost.lamp_hits = 3;
        g.stats.aimcost.lamp_near = 7;
        assert_eq!(aim_cost_summary(g), "lamps: a real hit (bulb 0.12 m) \u{b7} moving \u{d7}2, held 0.35 s \u{b7} lamps hit 3, passed near 7");
    }
}
