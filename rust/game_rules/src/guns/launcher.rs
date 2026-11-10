pub use ::game_types::launcher::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::flight::Met;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec::Vec};

/// A table of the ring's as a Ballistics (launchVel and arcSolve read only these numbers).
fn ballistics_of(k: &crate::knobs::RingK) -> Ballistics {
    Ballistics { speed: k.speed, gravity: k.gravity, drag: k.drag, radius: k.radius, life: k.life, substeps: k.substeps, solve_iters: k.solve_iters }
}

/// The launch velocity to pass through `target`: dead at it, raised by however far a simulated flight falls short, solveIters times.
pub fn launch_vel(from: V3, target: V3, r: crate::launcher::Ballistics) -> V3 {
    let d = v3::sub(target, from);
    let l = v3::len(d);
    if l < 1e-3 { return [0.0, 0.0, r.speed]; }
    let mut aim = v3::copy(target);
    let mut v0: V3 = v3::scale(v3::normalize(d), r.speed);
    let mut it = 0.0;
    while it < r.solve_iters {
        v0 = v3::scale(v3::normalize(v3::sub(aim, from)), r.speed);
        let hd = m::hyp2(d[0], d[2]);
        let mut p = v3::copy(from);
        let mut v = v3::copy(v0);
        let h = 1.0 / 240.0;
        let mut y_at = m::NAN;
        let mut i = 0.0;
        while i < r.life / h {
            let k = m::exp(-r.drag * h);
            v = [v[0] * k, v[1] * k - r.gravity * h, v[2] * k];
            let np: V3 = v3::mad(p, v, h);
            let a = m::hyp2(p[0] - from[0], p[2] - from[2]);
            let b = m::hyp2(np[0] - from[0], np[2] - from[2]);
            if b >= hd {
                let u = if b > a { (hd - a) / (b - a) } else { 1.0 };
                y_at = p[1] + (np[1] - p[1]) * u;
                break;
            }
            p = np;
            i += 1.0;
        }
        if !y_at.is_finite() { break; }
        aim = [aim[0], aim[1] + (target[1] - y_at), aim[2]];
        it += 1.0;
    }
    v0
}

/// Where a ring fired now would go, for the preview.
pub fn ring_solve(g: &crate::game::Game, from: V3, target: V3) -> crate::throwables::ThrowSolution {
    let r = ballistics_of(&g.k.ring);
    arc_solve(g, from, target, r)
}

/// ringSolve for anything the tube throws: stepped as a live ring at 60 Hz, until it meets a static, a leaf, a prop or the floor.
pub fn arc_solve(g: &crate::game::Game, from: V3, target: V3, r: crate::launcher::Ballistics) -> crate::throwables::ThrowSolution {
    let v0 = launch_vel(from, target, r);
    let mut points: Vec<V3> = Vec::new();
    let mut p = v3::copy(from);
    let mut v = v3::copy(v0);
    let mut land = v3::copy(target);
    let h = 1.0 / 60.0 / r.substeps;
    let mut t_end = 0.0;
    let mut i = 0.0;
    while i * h < r.life {
        if js::rem(i, r.substeps) == 0.0 { points.push(v3::copy(p)); }
        let k = m::exp(-r.drag * h);
        v = [v[0] * k, v[1] * k - r.gravity * h, v[2] * k];
        let np = v3::mad(p, v, h);
        let seg = v3::sub(np, p);
        let len = v3::len(seg);
        t_end = i * h;
        if len > 1e-6 {
            let dir = v3::scale(seg, 1.0 / len);
            let reach = len + r.radius;
            if let Some(h) = crate::props::level_ray(g, p, dir, reach) {
                land = v3::mad(p, dir, m::max(0.0, h.t - r.radius));
                points.push(land);
                break;
            }
        }
        if np[1] < r.radius {
            land = [np[0], r.radius, np[2]];
            points.push(land);
            break;
        }
        p = np;
        land = v3::copy(np);
        i += 1.0;
    }
    crate::throwables::ThrowSolution { v0, t: t_end, points, land }
}

/// The point to send a ring at: the head of the man under the reticle, led by his ground velocity; else the fixture or world point the cursor names.
pub fn ring_aim_point(g: &crate::game::Game, seat: &crate::seats::SeatRef, from: V3) -> V3 {
    if let Some(gd) = seat.aim_guard {
        let c = g.gd[gd].ch;
        let head_bone = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(b) => *b, None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]) };
        let head = v3::add(head_bone, [0.0, 0.04, 0.0]);
        let tof = v3::dist(from, head) / (g.k.ring.speed * 0.9);
        let vel = g.ch[c].vel;
        return v3::mad(head, [vel[0], 0.0, vel[2]], tof);
    }
    if let Some(t) = seat.aim_target { return v3::copy(g.tab.fixture[t].pos); }
    let ap = seat.aim_point;
    [ap[0], ap[1] + 0.03, ap[2]]
}

/// Let one ring go: in the air from this frame, the launcher's sound at the tube, a 'prop' noise at the shooter.
pub fn fire_ring(g: &mut crate::game::Game, shooter: CharId, from: V3, target: V3) -> crate::launcher::Ring {
    let vel = launch_vel(from, target, ballistics_of(&g.k.ring));
    let spin = crate::dice::draw_body(g, "launcher::fire_ring#1", shooter) * m::PI;
    let ring = Ring { pos: v3::copy(from), vel, age: 0.0, shooter, spin };
    g.rings.push(ring);
    let level = g.k.ring.launch_noise;
    crate::flight::launched(g, shooter, from, 0.8, None, level);
    ring
}

/// One frame of every ring in the air.
pub fn update_rings(g: &mut crate::game::Game, dt: f64) {
    let k = g.k.ring;
    let b = ballistics_of(&k);
    let h = dt / b.substeps;
    let off = |v: V3, n: V3| crate::flight::bounce(v, n, k.restitution, k.keep_tangent);
    let mut i = g.rings.len();
    while i > 0 {
        i -= 1;
        let mut r = g.rings[i];
        let mut done = false;
        let mut s = 0.0;
        while s < b.substeps && !done {
            s += 1.0;
            r.age += h;
            r.spin += 25.0 * h;
            let bodies = crate::flight::bodies(g, r.shooter);
            let Some(leg) = crate::flight::leg(g, &b, h, r.pos, &mut r.vel, &bodies) else { continue };
            let Some(met) = leg.met else { r.pos = leg.to; continue };
            done = true;
            match met {
                Met::Man(c, t) => ring_hits_character(g, r, c, leg.dir, v3::mad(r.pos, leg.dir, t)),
                Met::Prop(ph) => {
                    let at = v3::mad(r.pos, leg.dir, m::max(0.0, ph.t - b.radius));
                    let who = g.ch[r.shooter].id;
                    crate::props::prop_system_apply_impulse(g, ph.prop, at, [leg.dir[0] * k.prop_j, 0.0, leg.dir[2] * k.prop_j], who, Some(true));
                    clack(g, at, None);
                    crate::marks::chip_mark(g, v3::mad(r.pos, leg.dir, ph.t), ph.n, Some(crate::marks::Surface2 { box_: ph.box_, dynamic: true }), None);
                    spend(g, r, at, off(r.vel, ph.n));
                }
                Met::Static(st) => {
                    let at = v3::mad(r.pos, leg.dir, m::max(0.0, st.t - b.radius));
                    clack(g, at, None);
                    let surface = crate::marks::static_surface(g, st.index);
                    crate::marks::chip_mark(g, v3::mad(r.pos, leg.dir, st.t), st.n, surface, None);
                    spend(g, r, at, off(r.vel, st.n));
                }
                Met::Floor => {
                    let at: V3 = [leg.to[0], b.radius, leg.to[2]];
                    clack(g, at, Some(0.6));
                    spend(g, r, at, off(r.vel, [0.0, 1.0, 0.0]));
                }
            }
        }
        if !done && r.age > b.life {
            spend(g, r, r.pos, v3::scale(r.vel, 0.3));
            done = true;
        }
        // (a restart under this walk empties g.rings; the row is the walked copy until here)
        if i < g.rings.len() {
            if done { g.rings.remove(i); } else { g.rings[i] = r; }
        }
    }
}

/// The ring's flight is over at `at`.
fn spend(g: &mut crate::game::Game, r: crate::launcher::Ring, at: V3, vel: V3) {
    let most = g.k.ring.spent_speed_max;
    crate::flight::to_item(g, crate::throwables::ItemKind::Ring, r.shooter, at, vel, most, crate::throwables::ItemBounce::RingSkitter);
}

/// The ring meeting something hard: a clack, sparks, and a noise the room goes to look at.
pub fn clack(g: &mut crate::game::Game, at: V3, k: Option<f64>) {
    let k = k.unwrap_or(1.0);
    crate::audio_iface::play_at_rate(g, "canisterBounce", at, 0.55 * k, 1.5);
    crate::particle_fx::clack_fx(g, at);
    let level = g.k.ring.clack_noise * k;
    crate::flight::noise(g, [at[0], 0.0, at[2]], level, 0.0, Some(true));
}

/// A ring has found a man at `at`: high is out cold, low is struck; the ring drops at his feet.
fn ring_hits_character(g: &mut crate::game::Game, r: crate::launcher::Ring, c: CharId, dir: V3, at: V3) {
    let neck = match g.ch[c].bones.get(crate::character::Bone::Neck) { Some(b) => *b, None => v3::add(g.ch[c].pos, [0.0, 1.42, 0.0]) };
    let high = at[1] >= neck[1] - g.k.ring.ko_below_neck;
    match crate::character::guard_of(g, c) {
        Some(gd) if high => {
            crate::audio_iface::play_at_rate(g, "bodyHit", at, 0.6, 0.75);
            let by = if crate::character::is_player(g, r.shooter) { crate::game::DownBy::Player } else { crate::game::DownBy::Guard };
            crate::combat::kill_guard(g, gd, [dir[0], 0.0, dir[2]], Some(true), Some(crate::game::DownKind::Struck), Some(by), crate::game::With::Ring);
            let level = g.k.ring.ko_noise;
            crate::flight::thump_at(g, c, level, r.shooter);
        }
        _ => crate::flight::struck(g, c, dir, r.shooter),
    }
    let drop_at = v3::mad(at, dir, -0.05);
    let vx = -dir[0] * 1.5 + (crate::dice::draw_body(g, "launcher::ring_hits_character#1", c) - 0.5);
    let vy = 1.0 + crate::dice::draw_body(g, "launcher::ring_hits_character#2", c);
    let vz = -dir[2] * 1.5 + (crate::dice::draw_body(g, "launcher::ring_hits_character#3", c) - 0.5);
    spend(g, r, drop_at, [vx, vy, vz]);
}

/// Rings in flight as boxes: under the issue kit (GEAR.on, settled) the sponge round nose first along its flight, rolling about it. The tick's gathering of dynamic boxes calls this.
pub fn ring_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let rings: Vec<(V3, V3, f64)> = g.rings.iter().map(|r| (r.pos, r.vel, r.spin)).collect();
    struct R { pos: V3, vel: V3, spin: f64 }
    for r in rings.into_iter().map(|(pos, vel, spin)| R { pos, vel, spin }) {
        let sp = v3::len(r.vel);
        if sp < 1e-3 { continue; }
        let z = v3::scale(r.vel, 1.0 / sp);
        let up: V3 = if m::abs(z[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
        let x0 = v3::normalize(v3::cross(up, z));
        let y0 = v3::cross(z, x0);
        let (ca, sa) = (m::cos(r.spin), m::sin(r.spin));
        let rot = v3::quat::from_basis(v3::mad(v3::scale(x0, ca), y0, sa), v3::mad(v3::scale(x0, -sa), y0, ca), z);
        crate::throwables::sponge_boxes(g, out, v3::copy(r.pos), rot, v3::yaw_of(z));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const R: Ballistics = Ballistics { speed: 30.0, gravity: 9.81, drag: 0.4, radius: 0.04, life: 1.5, substeps: 4.0, solve_iters: 2.0 };

    /// The height of a round fired at `v0` where it has come `range` across the floor, None if its life ends first.
    fn height_at(from: V3, v0: V3, range: f64, r: Ballistics) -> Option<f64> {
        let (mut p, mut v) = (from, v0);
        let h = 1.0 / 240.0;
        let mut t = 0.0;
        while t < r.life {
            let k = m::exp(-r.drag * h);
            v = [v[0] * k, v[1] * k - r.gravity * h, v[2] * k];
            let np = v3::mad(p, v, h);
            if m::hyp2(np[0] - from[0], np[2] - from[2]) >= range { return Some(np[1]); }
            p = np;
            t += h;
        }
        None
    }

    /// The round leaves at the row's speed, on the target's bearing, raised so that it comes down at the target's height, where the dead-on shot falls short.
    #[test]
    fn a_shot_is_raised_to_land_on_the_target() {
        for (from, target, r) in [
            ([0.0, 1.5, 0.0], [10.0, 1.6, 7.0], R),
            ([3.25, 1.2, -8.5], [-8.75, 0.3, 7.125], R),
            ([0.5, 1.4, 0.25], [20.0, 3.0, -15.0], Ballistics { speed: 76.0, drag: 0.12, solve_iters: 3.0, ..R }),
        ] {
            let v = launch_vel(from, target, r);
            let range = m::hyp2(target[0] - from[0], target[2] - from[2]);
            assert!((v3::len(v) - r.speed).abs() < 1e-9, "speed {}", v3::len(v));
            assert!((v[0] * (target[2] - from[2]) - v[2] * (target[0] - from[0])).abs() < 1e-6 && v[0] * (target[0] - from[0]) + v[2] * (target[2] - from[2]) > 0.0, "off its bearing: {:?}", v);
            let there = height_at(from, v, range, r).unwrap();
            assert!((there - target[1]).abs() < 0.05, "{:?} to {:?}: at {} over {}", from, target, there, target[1]);
            let dead_on = v3::scale(v3::normalize(v3::sub(target, from)), r.speed);
            assert!(height_at(from, dead_on, range, r).unwrap() < target[1] - 0.05, "the dead-on shot would have done as well");
        }
    }

    /// With nothing to work out it goes straight at the target: past what the round can fly, straight up, or with no way to tell where the target is (then along the world's +z).
    #[test]
    fn a_shot_with_nothing_to_solve_goes_straight() {
        let far = launch_vel([0.0, 2.0, 0.0], [80.0, 1.7, 3.0], R);
        let dead_on = v3::scale(v3::normalize([80.0, -0.3, 3.0]), R.speed);
        assert!(v3::dist(far, dead_on) < 1e-9, "{:?}", far);
        assert_eq!(launch_vel([0.0, 1.0, 0.0], [0.0, 6.0, 0.0], R), [0.0, R.speed, 0.0]);
        assert_eq!(launch_vel([1.0, 1.0, 1.0], [1.0, 1.0, 1.0005], R), [0.0, 0.0, R.speed]);
    }
}
