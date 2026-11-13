pub use ::game_types::wallsnap::types;
pub use types::*;
use crate::m;

/// Within 0..1; a NaN stays a NaN.
fn clamp01(x: f64) -> f64 {
    if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
}

/// Seconds the take runs for a glide of `dist` metres and a turn of `turn` radians.
pub fn ws_take_secs(g: &crate::game::Game, dist: f64, turn: f64) -> f64 {
    take_secs(&g.k.wallsnap, dist, turn)
}

fn take_secs(w: &crate::knobs::WallsnapK, dist: f64, turn: f64) -> f64 {
    let t = w.take_base + w.take_per_m * dist + w.take_per_turn * m::abs(turn) / m::PI;
    m::min(w.take_max, m::max(w.take_min, t)) * m::max(0.25, w.slow_take)
}

/// How far out (m) the key-less take's clock starts for a man closing on the face at `into` m/s.
pub fn ws_press_near(g: &crate::game::Game, into: f64) -> f64 {
    press_near(&g.k.wallsnap, into)
}

fn press_near(w: &crate::knobs::WallsnapK, into: f64) -> f64 {
    m::max(w.press_near, w.press_flow + w.press_secs * m::max(0.0, into))
}

/// Ease-out by power p: 0 to 0, 1 to 1, fastest at the start.
pub fn ws_ease(s: f64, p: f64) -> f64 {
    1.0 - m::pow(1.0 - clamp01(s), p)
}

/// The landing's settle, t seconds after the back arrived.
pub fn ws_hit(g: &crate::game::Game, t: f64) -> f64 {
    hit(g.k.wallsnap.hit_peak, t)
}

fn hit(a: f64, t: f64) -> f64 {
    if t < 0.0 || t > 8.0 * a { 0.0 } else { (t / a) * m::exp(1.0 - t / a) }
}

/// The cursor's point about a wall's end: angle (degrees) from the wall's line carried on, and distance from the corner.
pub fn ws_corner_angle(a: f64, b: f64) -> crate::wallsnap::WsCornerAngle {
    crate::wallsnap::WsCornerAngle { deg: m::atan2(-b, a) * 180.0 / m::PI, dist: m::hyp2(a, b) }
}

/// The angle (degrees) up to which a man `back` short of the end and `off` the plane already sees past the corner.
pub fn ws_seen_deg(back: f64, off: f64) -> f64 {
    m::atan2(off, back) * 180.0 / m::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The take to the wall is longer for a longer glide or a bigger turn, either way round, inside its bounds, and the whole of it stretches by the slow row, never to under a quarter.
    #[test]
    fn the_take_runs_longer_for_further_and_for_more_turn_inside_its_bounds() {
        let mut w = crate::knobs::WallsnapK::page();
        w.slow_take = 1.0;
        assert_eq!(take_secs(&w, 0.0, 0.0), m::max(w.take_min, w.take_base));
        assert!(take_secs(&w, 0.5, 0.0) > take_secs(&w, 0.25, 0.0));
        assert!(take_secs(&w, 0.25, 1.0) > take_secs(&w, 0.25, 0.0));
        assert_eq!(take_secs(&w, 0.25, 1.0), take_secs(&w, 0.25, -1.0));
        assert_eq!(take_secs(&w, 100.0, 3.0), w.take_max);
        w.take_base = 0.0;
        assert_eq!(take_secs(&w, 0.0, 0.0), w.take_min);
        w.take_base = 0.2;
        let plain = take_secs(&w, 0.1, 0.5);
        w.slow_take = 2.0;
        assert_eq!(take_secs(&w, 0.1, 0.5), plain * 2.0);
        w.slow_take = 0.1;
        assert_eq!(take_secs(&w, 0.1, 0.5), plain * 0.25);
    }

    /// A man closing fast starts the take further out; standing or backing off, the near distance is the least of it.
    #[test]
    fn a_man_closing_fast_is_taken_from_further_out() {
        let w = crate::knobs::WallsnapK::page();
        assert_eq!(press_near(&w, 0.0), w.press_near);
        assert_eq!(press_near(&w, -2.0), w.press_near);
        assert_eq!(press_near(&w, 4.7), w.press_flow + w.press_secs * 4.7);
        assert!(press_near(&w, 4.7) > w.press_near);
    }

    /// The glide goes from nowhere to all the way, fastest at the start, and stops at its ends.
    #[test]
    fn the_glide_is_fastest_at_the_start() {
        assert_eq!((ws_ease(0.0, 2.0), ws_ease(1.0, 2.0), ws_ease(-1.0, 2.0), ws_ease(2.0, 3.0)), (0.0, 1.0, 0.0, 1.0));
        assert_eq!(ws_ease(0.125, 2.0), 1.0 - 0.875 * 0.875, "a quarter of the way in the first eighth of the time, as the knob's note has it");
        assert!(ws_ease(0.1, 2.0) - ws_ease(0.0, 2.0) > ws_ease(1.0, 2.0) - ws_ease(0.9, 2.0));
    }

    /// The landing's settle rises to all of it at its peak, dies away, and is nothing outside its span.
    #[test]
    fn the_landing_peaks_and_dies_away() {
        let a = crate::knobs::WallsnapK::page().hit_peak;
        assert_eq!(hit(a, a), 1.0);
        assert_eq!((hit(a, 0.0), hit(a, -0.1), hit(a, 8.0 * a + 1e-6)), (0.0, 0.0, 0.0));
        assert!(hit(a, a * 0.5) < 1.0 && hit(a, a * 2.0) < 1.0);
        assert!(hit(a, a * 2.0) > hit(a, a * 4.0));
        assert!(hit(a, 8.0 * a) > 0.0 && hit(a, 8.0 * a) < 0.02, "small enough to stop on");
    }

    /// The cursor about a wall's end: on the wall's line carried on it is at 0 degrees, round past the corner it climbs to a right angle, and back along the wall's own side the sign flips.
    #[test]
    fn a_cursor_round_a_corner_is_measured_from_the_wall_carried_on() {
        let r = ws_corner_angle(3.0, 0.0);
        assert_eq!(r.deg + 0.0, 0.0);
        assert_eq!(r.dist, 3.0);
        let r = ws_corner_angle(3.0, -4.0);
        assert_eq!(r.dist, 5.0);
        assert!(r.deg > 0.0 && r.deg < 90.0);
        assert_eq!(ws_corner_angle(0.0, -2.0).deg, 90.0);
        assert!(ws_corner_angle(1.0, 1.0).deg < 0.0);
        assert_eq!(ws_corner_angle(0.0, 0.0).dist, 0.0);
    }

    /// Up at the corner he already sees square round it; further back, or closer to the plane, he sees less.
    #[test]
    fn a_man_sees_less_past_the_corner_the_further_back_he_stands() {
        assert_eq!(ws_seen_deg(0.0, 0.2), 90.0);
        assert_eq!(ws_seen_deg(1.0, 0.0), 0.0);
        assert!(ws_seen_deg(0.6, 0.35) > ws_seen_deg(1.2, 0.35));
        assert!(ws_seen_deg(0.6, 0.35) > ws_seen_deg(0.6, 0.2));
        assert!(ws_seen_deg(1.7, -0.3) < 0.0, "behind the plane");
    }
}
