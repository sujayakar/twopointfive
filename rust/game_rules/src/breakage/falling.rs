pub use ::game_types::falling::types;
pub use types::*;
use crate::game::Game;
use crate::m;
use crate::v3::V3;

/// The ray a falling thing asks of what it lands on: the `t` of `{ t } | null`. Statics, or the statics and then the props up to the statics' `t`, the props' answer first.
fn ask(g: &Game, world: &crate::falling::FallWorld, ro: V3, rd: V3, tmax: f64) -> Option<f64> {
    match world {
        crate::falling::FallWorld::Statics => crate::scene_collision::raycast(g, ro, rd, tmax).map(|h| h.t),
        crate::falling::FallWorld::StaticsAndProps => crate::props::level_ray(g, ro, rd, tmax).map(|h| h.t),
    }
}

/// The time a fall from `h` takes at downward speed `vy`.
fn t_of(vy: f64, gravity: f64, h: f64) -> f64 {
    (-vy + m::sqrt(vy * vy + 2.0 * gravity * h)) / gravity
}

/// Where and when something let go at `c` with velocity `v` reaches what is under it, a share `reach_k` of the way sideways.
pub(super) fn fall_of(g: &crate::game::Game, c_2: V3, v: V3, gravity: f64, reach_k: f64, world: crate::falling::FallWorld) -> crate::falling::FallOf {
    fall_along(&mut |ro, rd, tmax| ask(g, &world, ro, rd, tmax), c_2, v, gravity, reach_k)
}

/// `fall_of`'s sums, over whatever answers the ray.
fn fall_along(ray: &mut dyn FnMut(V3, V3, f64) -> Option<f64>, c_2: V3, v: V3, gravity: f64, reach_k: f64) -> crate::falling::FallOf {
    let c = c_2;
    let vy = -v[1];
    let mut reach = m::hyp2(v[0], v[2]) * t_of(vy, gravity, m::max(0.02, c[1])) * reach_k;
    let d: V3 = if reach > 1e-4 {
        [
            v[0] * reach_k * t_of(vy, gravity, m::max(0.02, c[1])) / reach,
            0.0,
            v[2] * reach_k * t_of(vy, gravity, m::max(0.02, c[1])) / reach,
        ]
    } else {
        [0.0, 0.0, 0.0]
    };
    if reach > 0.05 {
        if let Some(w) = ray([c[0], c[1], c[2]], d, reach + 0.05) {
            reach = m::max(0.0, w - 0.05);
        }
    }
    let x = c[0] + d[0] * reach;
    let z = c[2] + d[2] * reach;
    let hit = ray([x, c[1], z], [0.0, -1.0, 0.0], c[1] + 0.5);
    let h = m::max(0.02, match hit { Some(t) => t, None => c[1] });
    crate::falling::FallOf { t: t_of(vy, gravity, h), pos: [x, c[1] - h, z] }
}

/// The same question asked along the arc, for a thing thrown: when and where it comes to lie.
pub(super) fn flight_of(g: &crate::game::Game, c_2: V3, v: V3, gravity: f64, world: crate::falling::FallWorld) -> crate::falling::FlightOf {
    flight_along(&mut |ro, rd, tmax| ask(g, &world, ro, rd, tmax), c_2, v, gravity)
}

/// `flight_of`'s sums, over whatever answers the ray.
fn flight_along(ray: &mut dyn FnMut(V3, V3, f64) -> Option<f64>, c_2: V3, v: V3, gravity: f64) -> crate::falling::FlightOf {
    let c = c_2;
    let up = v[1];
    let t_all = (up + m::sqrt(up * up + 2.0 * gravity * m::max(0.02, c[1]))) / gravity;
    let (mut px, mut py, mut pz, mut at) = (c[0], c[1], c[2], 0.0);
    let mut k = 1.0;
    while k <= CHORDS {
        let tk = t_all * k / CHORDS;
        let qx = c[0] + v[0] * tk;
        let qy = m::max(0.02, c[1] + up * tk - 0.5 * gravity * tk * tk);
        let qz = c[2] + v[2] * tk;
        let (dx, dy, dz) = (qx - px, qy - py, qz - pz);
        let len = m::sqrt(dx * dx + dy * dy + dz * dz);
        if len > 1e-6 {
            if let Some(hit) = ray([px, py, pz], [dx / len, dy / len, dz / len], len + 0.05) {
                let s = m::max(0.0, hit - 0.05) / len;
                let (x, y, z) = (px + dx * s, py + dy * s, pz + dz * s);
                let under = ray([x, y, z], [0.0, -1.0, 0.0], y + 0.5);
                let h = m::max(0.0, match under { Some(t) => t, None => y });
                return crate::falling::FlightOf {
                    t: at + (tk - at) * m::min(1.0, hit / len) + m::sqrt(2.0 * h / gravity),
                    pos: [x, y - h, z],
                };
            }
        }
        px = qx;
        py = qy;
        pz = qz;
        at = tk;
        k += 1.0;
    }
    crate::falling::FlightOf { t: t_all, pos: [px, 0.0, pz] }
}

#[cfg(test)]
mod tests {
    use super::*;

    const G: f64 = 9.81;
    /// The world of these tests: a wall at x = 2, a floor at y = 0 that stops at x = 10.
    fn ray(ro: V3, rd: V3, tmax: f64) -> Option<f64> {
        let mut best = f64::INFINITY;
        if rd[0] > 0.0 && ro[0] <= 2.0 {
            let t = (2.0 - ro[0]) / rd[0];
            if t < best { best = t; }
        }
        if rd[1] < 0.0 && ro[1] >= 0.0 {
            let t = ro[1] / -rd[1];
            if ro[0] + rd[0] * t < 10.0 && t < best { best = t; }
        }
        if best <= tmax { Some(best) } else { None }
    }
    fn near(a: f64, b: f64, by: f64) { assert!((a - b).abs() < by, "{a} {b}"); }
    fn fall(c: V3, v: V3, k: f64) -> crate::falling::FallOf { fall_along(&mut ray, c, v, G, k) }
    fn flight(c: V3, v: V3) -> crate::falling::FlightOf { flight_along(&mut ray, c, v, G) }

    #[test]
    fn a_thing_let_go_falls_straight_down_in_the_time_of_its_drop() {
        let a = fall([0.0, 3.1, 0.0], [0.0; 3], 0.8);
        near(a.t, m::sqrt(2.0 * 3.1 / G), 1e-12);
        assert_eq!(a.pos, [0.0, 0.0, 0.0]);
    }

    /// Where the sound and the litter of a falling thing are. A failure means glass is heard or lies where it did not come down.
    #[test]
    fn a_thing_pushed_sideways_goes_a_share_of_the_way_it_would_fly() {
        let (c, v): (V3, V3) = ([0.3, 1.37, -0.7], [1.3, 0.0, -2.1]);
        let a = fall(c, v, 0.8);
        let t = m::sqrt(2.0 * 1.37 / G);
        near(a.t, t, 1e-12);
        near(a.pos[0], 0.3 + 1.3 * t * 0.8, 1e-12);
        near(a.pos[2], -0.7 - 2.1 * t * 0.8, 1e-12);
        assert_eq!(a.pos[1], 0.0);
        assert!(fall(c, [1.3, -2.0, -2.1], 0.8).t < t && fall(c, [1.3, 2.0, -2.1], 0.8).t > t, "thrown down it is there sooner, thrown up later");
    }

    #[test]
    fn a_wall_stops_a_fall_five_centimetres_short_of_it() {
        let a = fall([1.2, 1.37, 0.25], [31.3, 0.1, 0.0], 1.0);
        near(a.pos[0], 1.95, 1e-9);
        assert_eq!(a.pos[1], 0.0);
    }

    #[test]
    fn past_the_floor_s_end_it_is_taken_to_fall_its_own_height() {
        let a = fall([11.0, 1.5, 0.0], [0.0; 3], 0.8);
        near(a.t, m::sqrt(2.0 * 1.5 / G), 1e-12);
        assert_eq!(a.pos, [11.0, 0.0, 0.0]);
    }

    /// A thing thrown: asked along its arc. It stops a hand short of the floor and drops the rest, so it lies a moment later than the sum says.
    #[test]
    fn a_throw_comes_to_lie_where_its_arc_meets_the_floor() {
        let (c, v): (V3, V3) = ([-3.0, 1.1, 0.0], [1.1, 2.7, 0.6]);
        let t = (2.7 + m::sqrt(2.7 * 2.7 + 2.0 * G * 1.1)) / G;
        let b = flight(c, v);
        assert_eq!(b.pos[1], 0.0);
        near(b.pos[0], -3.0 + 1.1 * t, 0.05);
        near(b.pos[2], 0.6 * t, 0.05);
        assert!(b.t >= t - 1e-9 && b.t < t + 0.15, "{} {t}", b.t);
    }

    #[test]
    fn a_throw_that_meets_a_wall_drops_from_there() {
        let b = flight([0.5, 1.2, 0.5], [7.9, 3.3, -1.2]);
        near(b.pos[0], 1.95, 0.01);
        assert_eq!(b.pos[1], 0.0);
        assert!(b.t > (2.0 - 0.5) / 7.9, "{}", b.t);
    }
}
