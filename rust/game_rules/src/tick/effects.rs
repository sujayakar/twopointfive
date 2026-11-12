pub use ::game_types::effects::types;
pub use types::*;
use crate::game::Game;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec, vec::Vec};

type Emitter = crate::smoke_types::SmokeEmitter;
type Kind = crate::smoke_types::SmokeEmitterKind;
type Lattice = crate::smoke_types::SmokeLattice;

/// A stored `Track` called: its `{ pos, dir }`, read live, or nothing once what it rides has gone.
pub fn track_at(g: &Game, track: &Track) -> Option<(V3, V3)> {
    match *track {
        Track::Muzzle { by } => Some((g.ch[by].muzzle, g.ch[by].gun_dir)),
        Track::EjectPort { by } => {
            let from = g.ch[by].bone(crate::character::Bone::HandR).unwrap_or(g.ch[by].muzzle);
            Some((v3::add(v3::mad(from, g.ch[by].gun_dir, 0.06), [0.0, 0.06, 0.0]), [0.0, 1.0, 0.0]))
        }
        Track::Spark { slot, gen_ } => {
            let s = g.sparks.pool[slot];
            if s.alive && s.gen_ == gen_ { Some((s.pos, [0.0, 1.0, 0.0])) } else { None }
        }
        Track::Cigarette { by } => {
            // (while he is seen: the source lives a second and the game files the next while he smokes)
            if g.fog_of_war.hidden_chars.len() > 0 && g.fog_of_war.hidden_chars.has(by) { return None; }
            let (v, up) = (g.ch[by].vel, g.k.smokebreak.thread.speed);
            crate::smokebreak::tip_of(g, by).map(|p| (p, [v[0] * 0.5, up, v[2] * 0.5]))
        }
    }
}

/// `smoke.emit`: the game's own smoke where it keeps one, then the event for the host's solver (the gas of a shot, a stun's jets and trails, the can's
/// vent: the solver is the host's and hears of them the way it hears of a holed wall's puff).
pub(crate) fn smoke_emit(g: &mut Game, e: &Emitter) {
    crate::smoke_types::smoke_system_like_emit(g, e);
    crate::out_events::push_smoke_system_like_emit(g, e);
}

// ---------------------------------------------------------------- spawners
/// Muzzle frame for the emitter offsets: z = bore, y = up-ish, x = right.
pub fn muzzle_frame(o: V3, d: V3) -> crate::effects::MuzzleFrame {
    let z = v3::normalize(d);
    let mut y = v3::sub([0.0, 1.0, 0.0], v3::scale(z, z[1]));
    if v3::len(y) < 1e-3 {
        y = v3::sub([1.0, 0.0, 0.0], v3::scale(z, z[0]));
    }
    y = v3::normalize(y);
    MuzzleFrame { o: v3::copy(o), x: v3::cross(y, z), y, z }
}

/// An offset in the muzzle frame, as a point.
fn at3(muzzle: V3, f: &MuzzleFrame, off: V3) -> V3 {
    v3::mad(v3::mad(v3::mad(muzzle, f.x, off[0]), f.y, off[1]), f.z, off[2])
}

/// A shot from `muzzle` down `dir`: bore jet, side ports, the wisp; returns the light to fire.
pub fn spawn_shot(g: &mut crate::game::Game, muzzle: V3, dir: V3, o: &crate::effects::SpawnShotO) -> Vec<crate::effects::FxLight> {
    let gas = o.gas != Some(false); // `smoke = NO_GAS`: its emit does nothing, so no call is made
    let s = g.k.effects.shot;
    let d = v3::normalize(dir);
    let f = muzzle_frame(muzzle, d);
    let at = at3(muzzle, &f, s.jet_off);
    let jv: V3 = [d[0] * s.jet_speed, d[1] * s.jet_speed + s.jet_rise, d[2] * s.jet_speed];
    if gas {
        smoke_emit(g, &Emitter {
            pos: at, dir: jv, speed: v3::len(jv), radius: s.jet_radius, density: s.jet_density, temperature: s.jet_temp, ttl: s.jet_life, age: 0.0, kind: Kind::Jet,
            push: Some(s.jet_push), expand: Some(s.jet_expand), attack: Some(s.jet_attack), jitter: Some(s.jitter), lattice: Some(Lattice::Fine), confined: Some(true),
            ..Emitter::default()
        });
    }
    let yaw = m::atan2(d[0], d[2]);
    let mut i: i32 = 0;
    while (i as f64) < s.ports {
        let a = yaw + (if i % 2 == 0 { 1.0 } else { -1.0 }) * s.port_angle;
        let ps = s.jet_speed * s.port_speed_k;
        let pv: V3 = [m::sin(a) * ps, ps * 0.5, m::cos(a) * ps];
        if gas {
            smoke_emit(g, &Emitter {
                pos: at3(muzzle, &f, s.port_off), dir: pv, speed: v3::len(pv), radius: s.jet_radius * s.port_radius_k, density: s.jet_density * s.port_fraction, temperature: s.jet_temp * s.port_fraction, ttl: s.jet_life, age: 0.0, kind: Kind::Jet,
                push: Some(s.jet_push * s.port_push_k), expand: Some(s.jet_expand * s.port_expand_k), attack: Some(s.jet_attack), jitter: Some(s.jitter), lattice: Some(Lattice::Fine), confined: Some(true),
                ..Emitter::default()
            });
        }
        i += 1;
    }
    let wv: V3 = [d[0] * 0.25, d[1] * 0.25 + s.wisp_rise, d[2] * 0.25];
    if gas {
        smoke_emit(g, &Emitter {
            pos: at3(muzzle, &f, s.wisp_off), dir: wv, speed: v3::len(wv), radius: s.wisp_radius, density: s.wisp_density, temperature: s.wisp_temp, ttl: s.wisp_life, age: 0.0, kind: Kind::Wisp,
            push: Some(s.wisp_push), attack: Some(s.wisp_attack), jitter: Some(0.25), lattice: Some(Lattice::Fine), confined: Some(true), track: o.track.clone(),
            ..Emitter::default()
        });
    }
    if gas && flags().ejection_wisp {
        if let Some(eject) = &o.eject {
            if let Some((pos, _dir)) = track_at(g, eject) {
                smoke_emit(g, &Emitter {
                    pos, dir: [0.0, 1.0, 0.0], speed: 0.1, radius: s.eject_radius, density: s.eject_density, temperature: s.eject_temp, ttl: s.eject_life, age: 0.0, kind: Kind::Wisp,
                    lattice: Some(Lattice::Fine), confined: Some(true), track: Some(eject.clone()),
                    ..Emitter::default()
                });
            }
        }
    }
    vec![FxLight {
        pos: v3::mad(muzzle, d, s.standoff * 0.4),
        color: s.light_color,
        intensity: s.light_power * o.light_gain.unwrap_or(1.0),
        range: o.light_range.unwrap_or(s.light_range),
        ttl: s.light_duration,
        radius: None,
    }]
}

/// A burst's seeded draw in [0, 1).
fn rnd(seed: f64, k: f64) -> f64 {
    let t = m::sin(seed * 37.31 + k * 91.7) * 43758.5453;
    t - m::floor(t)
}
/// A direction with its seeded deviation.
fn wobble(seed: f64, dv: V3, amount: f64, k: f64) -> V3 {
    v3::normalize([
        dv[0] + (rnd(seed, k) - 0.5) * amount,
        dv[1] + (rnd(seed, k + 1.0) - 0.5) * amount,
        dv[2] + (rnd(seed, k + 2.0) - 0.5) * amount,
    ])
}

/// A stun canister coming apart at `at`: end jets, body vents, wisp, sparks and the smoke hung off them; returns the flash.
pub fn spawn_burst(g: &mut crate::game::Game, at: V3, seed: f64) -> crate::effects::FxLight {
    let b = g.k.effects.bang;
    let dens = b.density * b.density_gain;
    let temp = b.temp * b.temp_gain;
    let yaw = rnd(seed, 1.0) * m::PI * 2.0;
    let tilt = (rnd(seed, 2.0) - 0.5) * 2.0 * b.tilt;
    let ax: V3 = [m::cos(tilt) * m::sin(yaw), m::sin(tilt), m::cos(tilt) * m::cos(yaw)];
    let bias = (rnd(seed, 3.0) - 0.5) * 2.0 * b.asymmetry;
    let j = b.jitter;
    let mut si = 0.0;
    for side in [1.0, -1.0] {
        si += 1.0;
        let w = (1.0 + side * bias) * (1.0 + (rnd(seed, 40.0 + si) - 0.5) * j);
        if w <= 0.02 {
            continue;
        }
        let along = b.half_length * side * (0.6 + rnd(seed, 50.0 + si) * 0.8);
        let dv = wobble(seed, v3::scale(ax, side), j * 0.9, 60.0 + si * 3.0);
        let spd = b.vent_speed * (1.0 + (rnd(seed, 70.0 + si) - 0.5) * j * 0.8);
        smoke_emit(g, &Emitter {
            pos: v3::mad(at, ax, along), dir: dv, speed: spd, radius: b.radius * (1.0 + (rnd(seed, 80.0 + si) - 0.5) * j * 0.7), density: dens * w, temperature: temp * w, ttl: b.life * (1.0 + (rnd(seed, 90.0 + si) - 0.5) * j * 0.5), age: 0.0, kind: Kind::Jet,
            push: Some(b.end_push), expand: Some(b.expand), attack: Some(b.attack), jitter: Some(b.frame_jitter), lattice: Some(Lattice::Coarse), confined: Some(true),
            ..Emitter::default()
        });
    }
    let up: V3 = if m::abs(ax[1]) > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let p = v3::normalize(v3::cross(up, ax));
    let q = v3::cross(ax, p);
    let mut i: i32 = 0;
    while (i as f64) < b.body_vents {
        let fi = i as f64;
        let a2 = (fi / m::max(1.0, b.body_vents)) * m::PI * 2.0 + rnd(seed, 10.0 + fi) * 1.2;
        let c = m::cos(a2);
        let s = m::sin(a2);
        let dv = wobble(seed, [p[0] * c + q[0] * s, p[1] * c + q[1] * s, p[2] * c + q[2] * s], j * 0.7, 150.0 + fi * 3.0);
        let w = b.body_fraction * (0.5 + rnd(seed, 30.0 + fi)) * (1.0 + (rnd(seed, 120.0 + fi) - 0.5) * j);
        let along = (rnd(seed, 130.0 + fi) - 0.5) * 2.0 * b.half_length;
        let spd = b.vent_speed * b.body_speed_k * (1.0 + (rnd(seed, 140.0 + fi) - 0.5) * j);
        smoke_emit(g, &Emitter {
            pos: v3::mad(at, ax, along), dir: dv, speed: spd, radius: b.radius * b.body_radius_k * (1.0 + (rnd(seed, 170.0 + fi) - 0.5) * j * 0.8), density: dens * w, temperature: temp * w, ttl: b.life * (1.0 + (rnd(seed, 180.0 + fi) - 0.5) * j * 0.5), age: 0.0, kind: Kind::Jet,
            push: Some(b.body_push), expand: Some(b.expand * b.body_expand_k), attack: Some(b.attack), jitter: Some(b.frame_jitter), lattice: Some(Lattice::Coarse), confined: Some(true),
            ..Emitter::default()
        });
        i += 1;
    }
    smoke_emit(g, &Emitter {
        pos: v3::copy(at), dir: [0.0, 1.0, 0.0], speed: b.wisp_rise, radius: b.wisp_radius, density: b.wisp_density * b.density_gain, temperature: b.wisp_temp * b.temp_gain, ttl: b.wisp_life, age: 0.0, kind: Kind::Cloud,
        push: Some(b.wisp_push), attack: Some(b.wisp_attack), jitter: Some(0.3), lattice: Some(Lattice::Coarse),
        ..Emitter::default()
    });
    if b.trail_count > 0.0 {
        let k = g.k.effects.sparks;
        let idx = crate::sparks::sparks_emit_burst(g, at, crate::sparks::SparksEmitBurstO { count: b.trail_count, speed: k.speed, spread: k.spread, lift: k.lift, life: k.life }, seed);
        let room = crate::smoke_types::smoke_system_like_budget(g) - b.trail_reserve;
        let n = m::max(0.0, m::min_n(&[b.trail_count, idx.len() as f64, room]));
        let tl = g.k.effects.trail;
        let mut i: usize = 0;
        while (i as f64) < n {
            let slot = idx[i] as usize;
            let tr = Track::Spark { slot, gen_: g.sparks.pool[slot].gen_ };
            if let Some((pos, _dir)) = track_at(g, &tr) {
                smoke_emit(g, &Emitter {
                    pos: v3::copy(pos), dir: [0.0, 1.0, 0.0], speed: 0.0, push: Some(0.0), radius: tl.radius, min_voxels: Some(tl.min_voxels), density: tl.density * b.density_gain, temperature: tl.temp * b.temp_gain, ttl: tl.life, age: 0.0, kind: Kind::Trail,
                    attack: Some(tl.attack), jitter: Some(0.0), lattice: Some(Lattice::Coarse), confined: Some(true), prio: Some(0.0), track: Some(tr),
                    ..Emitter::default()
                });
            }
            i += 1;
        }
    }
    FxLight { pos: v3::add(at, [0.0, b.light_lift, 0.0]), color: b.light_color, intensity: b.light_power, range: b.light_range, ttl: b.light_duration, radius: Some(b.light_radius) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at_near(a: V3, b: V3, what: &str) { assert!(v3::dist(a, b) < 1e-9, "{what}: {a:?} against {b:?}"); }

    /// Where a shot's gas comes out: z down the bore, y as near up as that lets it, x to the side, each a metre and square to the others.
    #[test]
    fn a_muzzle_frame_is_square_to_the_bore_and_up_as_near_as_it_can_be() {
        for d in [[0.3, -0.2, 0.9], [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, -2.0, 0.0], [0.001, 2.0, 0.0]] {
            let f = muzzle_frame([1.5, 1.25, -3.0], d);
            assert_eq!(f.o, [1.5, 1.25, -3.0]);
            at_near(f.z, v3::normalize(d), "down the bore");
            for (a, b) in [(f.x, f.y), (f.y, f.z), (f.x, f.z)] { assert!(v3::dot(a, b).abs() < 1e-9, "{d:?}: {a:?} {b:?}"); }
            for a in [f.x, f.y] { assert!((v3::len(a) - 1.0).abs() < 1e-9, "{d:?}: {a:?}"); }
            at_near(v3::cross(f.y, f.z), f.x, "right-handed");
            if d[1].abs() < 1.0 { assert!(f.y[1] > 0.0, "{d:?}: up is up"); }
        }
    }

    /// A stun's vents differ burst to burst and jet to jet, the same each time it is replayed.
    #[test]
    fn a_bursts_draws_are_its_own() {
        for k in 0..300 {
            let (seed, k) = (7.0, k as f64);
            let r = rnd(seed, k);
            assert!((0.0..1.0).contains(&r), "{k}: {r}");
            assert_eq!(r, rnd(seed, k));
            assert_ne!(r, rnd(seed + 1.0, k), "{k}: the next burst");
            assert_ne!(r, rnd(seed, k + 1.0), "{k}: the next draw");
        }
    }

    #[test]
    fn a_wobbled_direction_is_a_direction_near_the_one_it_was() {
        let d = v3::normalize([0.3, 0.1, -0.9]);
        let w = wobble(7.0, d, 0.54, 63.0);
        assert!((v3::len(w) - 1.0).abs() < 1e-9);
        assert_ne!(w, d);
        assert!(v3::dot(w, d) > 0.8, "{w:?}");
        at_near(wobble(7.0, d, 0.0, 63.0), d, "no amount: no wobble");
    }

    /// A shot with no gas puts nothing in the air and is still a flash, at the gain asked for, just out of the muzzle.
    #[test]
    fn a_shot_with_no_gas_files_none_and_still_flashes() {
        let mut g = Game::blank();
        let o = SpawnShotO { gas: Some(false), light_gain: Some(0.5), ..Default::default() };
        let l = spawn_shot(&mut g, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], &o);
        let s = g.k.effects.shot;
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].intensity, s.light_power * 0.5);
        assert_eq!((l[0].range, l[0].ttl), (s.light_range, s.light_duration));
        assert_eq!(l[0].pos, v3::mad([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], s.standoff * 0.4));
        assert!(g.out.is_empty(), "no gas");
    }

    // rides the spark it was made on, and answers nothing once it dies or the slot goes to another.
    #[test]
    fn a_spark_track_lets_go_when_the_spark_dies_or_the_slot_is_reused() {
        let mut g = Game::blank();
        g.sparks.pool[3].alive = true;
        g.sparks.pool[3].gen_ = 5.0;
        g.sparks.pool[3].pos = [1.0, 2.0, 3.0];
        let tr = Track::Spark { slot: 3, gen_: 5.0 };
        assert_eq!(track_at(&mut g, &tr), Some(([1.0, 2.0, 3.0], [0.0, 1.0, 0.0])));
        g.sparks.pool[3].gen_ = 6.0;
        assert_eq!(track_at(&mut g, &tr), None);
        g.sparks.pool[3].gen_ = 5.0;
        g.sparks.pool[3].alive = false;
        assert_eq!(track_at(&mut g, &tr), None);
    }
}
