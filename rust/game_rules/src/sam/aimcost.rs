pub use ::game_types::aimcost::types;
pub use types::*;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};

/// Did a round flying these legs MEET a fixture (its point `pos`, its own boxes) under the real-hit rule? `stopped`: its flight ended in one of the fixture's boxes.
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
            let (c, h) = { let x = &g.tab.boxes[b]; (x.c, x.h) };
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

/// `rayBox(ro, rd, grown, tmax) != null`, `grown` being a copy of `b` with half sizes `h`. `h` is the only field that differs, so the row itself takes it for the one
/// call and has its own back before anything else can read it. `copy_box` is not used: its answer is a row of `g.col.boxes`, which never shrinks, and a shot must not add to the world.
fn ray_grown(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, h: V3, tmax: f64) -> bool {
    let own = g.tab.boxes[b].h;
    g.tab.boxes.edit(b).h = h;
    let hit = crate::scene_boxes::ray_box(g, ro, rd, b.into(), tmax).is_some();
    g.tab.boxes.edit(b).h = own;
    hit
}

/// How near a round's legs pass a point (m; the legs' own lengths, nothing past where it stopped): the cheap test ahead of `lamp_struck`.
pub fn legs_near(pos: V3, legs: &[crate::aimcost::RoundLeg]) -> f64 {
    let mut best = m::INF;
    for s in legs {
        let to_t = v3::sub(pos, s.o);
        let d = v3::len(v3::sub(to_t, v3::scale(s.d, v3::clamp(v3::dot(to_t, s.d), 0.0, s.end))));
        if d < best { best = d; }
    }
    best
}

/// The feet a man's hands still remember: the fastest he has gone, let go over `stop_tau` (and to nothing: `stop_bleed`).
pub fn feet_hold(g: &crate::game::Game, prev: f64, speed: f64, dt: f64) -> f64 {
    let kept = prev * m::exp(-dt / m::max(g.k.aimcost.stop_tau, 1e-3)) - g.k.aimcost.stop_bleed * dt;
    let v = m::max(speed, kept);
    if v.is_finite() && v > 1e-4 { v } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aimcost::RoundLeg;
    use crate::v3::V3;

    /// A round flew along +x for 5 m, then along +z for 4 m and stopped.
    fn legs() -> [RoundLeg; 2] {
        [RoundLeg { o: [0.0; 3], d: [1.0, 0.0, 0.0], end: 5.0 }, RoundLeg { o: [5.0, 0.0, 0.0], d: [0.0, 0.0, 1.0], end: 4.0 }]
    }

    /// A failure means a lamp is struck by a round that never came near it, or missed by one that passed it: the legs count only as far as the round flew.
    #[test]
    fn a_rounds_legs_are_measured_only_as_far_as_it_flew() {
        let l = legs();
        let near = |p: V3, want: f64| { let d = legs_near(p, &l); assert!((d - want).abs() < 1e-12, "{p:?}: {d}"); };
        near([2.0, 3.0, 0.0], 3.0);        // beside the first leg
        near([-4.0, 0.0, -3.0], 5.0);      // behind where it started
        near([5.0, 0.0, 7.0], 3.0);        // past where it stopped
        near([4.0, 0.0, 2.0], 1.0);        // the nearer of the two legs
        assert_eq!(legs_near([1.0, 2.0, 3.0], &[]), f64::INFINITY);
    }

}
