//! src/game/effects.ts lines 1 to 283
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

type Emitter = crate::smoke_types::SmokeEmitter;
type Kind = crate::smoke_types::SmokeEmitterKind;
type Lattice = crate::smoke_types::SmokeEmitterLattice;

/// A stored `Track` called: its `{ pos, dir }`, read live, or nothing once what it rides has gone. (combat.ts fireWeaponRaw track and port; sparks.ts tracker)
fn track_at(g: &Game, track: &Track) -> Option<(V3, V3)> {
    match *track {
        Track::Muzzle { by } => Some((g.ch[by].muzzle, g.ch[by].gun_dir)),
        Track::EjectPort { by } => {
            let from = match g.ch[by].bones.get(crate::character::Bone::HandR) {
                Some(hand) => *hand,
                None => g.ch[by].muzzle,
            };
            Some((v3::add(v3::mad(from, g.ch[by].gun_dir, 0.06), [0.0, 0.06, 0.0]), [0.0, 1.0, 0.0]))
        }
        Track::Spark { slot, gen_ } => {
            let s = g.sparks.pool[slot];
            if s.alive && s.gen_ == gen_ { Some((s.pos, [0.0, 1.0, 0.0])) } else { None }
        }
    }
}

/// `smoke.emit`: the game's own smoke where it keeps one, then the event for the host's solver (the gas of a shot, a stun's jets and trails, the can's
/// vent: the solver is the host's and hears of them the way it hears of a holed wall's puff).
fn smoke_emit(g: &mut Game, e: &Emitter) {
    crate::smoke_types::smoke_system_like_emit(g, e);
    crate::out_events::push_smoke_system_like_emit(g, e);
}

// ---------------------------------------------------------------- spawners
/// Muzzle frame for the emitter offsets: z = bore, y = up-ish, x = right. (effects.ts muzzleFrame)
pub fn muzzle_frame(o: V3, d: V3) -> crate::effects::MuzzleFrame {
    let z = v3::normalize(d);
    let mut y = v3::sub([0.0, 1.0, 0.0], v3::scale(z, z[1]));
    if v3::len(y) < 1e-3 {
        y = v3::sub([1.0, 0.0, 0.0], v3::scale(z, z[0]));
    }
    y = v3::normalize(y);
    MuzzleFrame { o: v3::copy(o), x: v3::cross(y, z), y, z }
}

/// A solver that takes nothing (spawnShot's `gas: false`). (effects.ts NO_GAS)
pub mod no_gas {
    /// (effects.ts NO_GAS.emit)
    pub fn emit() {}
    /// (effects.ts NO_GAS.push)
    pub fn push() {}
    /// (effects.ts NO_GAS.inSmokeDomain)
    pub fn in_smoke_domain() -> bool {
        false
    }
}

/// An offset in the muzzle frame, as a point. (effects.ts spawnShot at3)
fn at3(muzzle: V3, f: &MuzzleFrame, off: V3) -> V3 {
    v3::mad(v3::mad(v3::mad(muzzle, f.x, off[0]), f.y, off[1]), f.z, off[2])
}

/// A shot from `muzzle` down `dir`: bore jet, side ports, the wisp; returns the light to fire. (effects.ts spawnShot)
pub fn spawn_shot(g: &mut crate::game::Game, muzzle: V3, dir: V3, o: &crate::effects::SpawnShotO, sparks: bool) -> Vec<crate::effects::FxLight> {
    let gas = o.gas != Some(false); // `smoke = NO_GAS`: its emit does nothing, so no call is made
    let s = shot();
    let d = v3::normalize(dir);
    let f = muzzle_frame(muzzle, d);
    let at = at3(muzzle, &f, s.jet_off);
    let anchor = v3::mad(muzzle, d, 0.38);
    let jv: V3 = [d[0] * s.jet_speed, d[1] * s.jet_speed + s.jet_rise, d[2] * s.jet_speed];
    if gas {
        smoke_emit(g, &Emitter {
            pos: at, dir: jv, speed: v3::len(jv), radius: s.jet_radius, density: s.jet_density, temperature: s.jet_temp, ttl: s.jet_life, age: 0.0, kind: Kind::Jet,
            push: Some(s.jet_push), expand: Some(s.jet_expand), attack: Some(s.jet_attack), jitter: Some(s.jitter), lattice: Some(Lattice::Fine), anchor: Some(anchor), confined: Some(true),
            ..Emitter::zeroed()
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
                push: Some(s.jet_push * s.port_push_k), expand: Some(s.jet_expand * s.port_expand_k), attack: Some(s.jet_attack), jitter: Some(s.jitter), lattice: Some(Lattice::Fine), anchor: Some(anchor), confined: Some(true),
                ..Emitter::zeroed()
            });
        }
        i += 1;
    }
    let wv: V3 = [d[0] * 0.25, d[1] * 0.25 + s.wisp_rise, d[2] * 0.25];
    if gas {
        smoke_emit(g, &Emitter {
            pos: at3(muzzle, &f, s.wisp_off), dir: wv, speed: v3::len(wv), radius: s.wisp_radius, density: s.wisp_density, temperature: s.wisp_temp, ttl: s.wisp_life, age: 0.0, kind: Kind::Wisp,
            push: Some(s.wisp_push), attack: Some(s.wisp_attack), jitter: Some(0.25), lattice: Some(Lattice::Fine), anchor: Some(anchor), confined: Some(true), track: o.track.clone(),
            ..Emitter::zeroed()
        });
    }
    if gas && flags().ejection_wisp {
        if let Some(eject) = &o.eject {
            if let Some((pos, _dir)) = track_at(g, eject) {
                smoke_emit(g, &Emitter {
                    pos, dir: [0.0, 1.0, 0.0], speed: 0.1, radius: s.eject_radius, density: s.eject_density, temperature: s.eject_temp, ttl: s.eject_life, age: 0.0, kind: Kind::Wisp,
                    lattice: Some(Lattice::Fine), anchor: Some(anchor), confined: Some(true), track: Some(eject.clone()),
                    ..Emitter::zeroed()
                });
            }
        }
    }
    // unburnt powder: a narrow fast cone about the bore (effects.ts:220). `sparks` is the TypeScript's `sp`, the caller's `gpu ? null : g.sparks` (combat.ts:152).
    if sparks && flags().shot_sparks && s.spark_count > 0.0 {
        crate::sparks::sparks_emit_cone(g, v3::mad(muzzle, d, 0.04), [d[0], d[1] + 0.08, d[2]], crate::sparks::SparksEmitConeO { count: s.spark_count, speed: s.spark_speed, cone: s.spark_cone, life: s.spark_life }, o.seed);
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

/// A burst's seeded draw in [0, 1). (effects.ts spawnBurst rnd)
fn rnd(seed: f64, k: f64) -> f64 {
    let t = m::sin(seed * 37.31 + k * 91.7) * 43758.5453;
    t - m::floor(t)
}
/// A direction with its seeded deviation. (effects.ts spawnBurst wobble)
fn wobble(seed: f64, dv: V3, amount: f64, k: f64) -> V3 {
    v3::normalize([
        dv[0] + (rnd(seed, k) - 0.5) * amount,
        dv[1] + (rnd(seed, k + 1.0) - 0.5) * amount,
        dv[2] + (rnd(seed, k + 2.0) - 0.5) * amount,
    ])
}

/// A stun canister coming apart at `at`: end jets, body vents, wisp, sparks and the smoke hung off them; returns the flash. (effects.ts spawnBurst)
pub fn spawn_burst(g: &mut crate::game::Game, at: V3, seed: f64, o: Option<crate::effects::SpawnBurstO>) -> crate::effects::FxLight {
    let b = bang();
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
            ..Emitter::zeroed()
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
            ..Emitter::zeroed()
        });
        i += 1;
    }
    smoke_emit(g, &Emitter {
        pos: v3::copy(at), dir: [0.0, 1.0, 0.0], speed: b.wisp_rise, radius: b.wisp_radius, density: b.wisp_density * b.density_gain, temperature: b.wisp_temp * b.temp_gain, ttl: b.wisp_life, age: 0.0, kind: Kind::Cloud,
        push: Some(b.wisp_push), attack: Some(b.wisp_attack), jitter: Some(0.3), lattice: Some(Lattice::Coarse),
        ..Emitter::zeroed()
    });
    let spark_count = match o {
        Some(SpawnBurstO { spark_count: Some(n) }) => n,
        _ => b.spark_count,
    };
    if spark_count > 0.0 {
        let k = sparks();
        let idx = crate::sparks::sparks_emit_burst(g, at, crate::sparks::SparksEmitBurstO { count: spark_count, speed: k.speed, spread: k.spread, lift: k.lift, life: k.life }, seed);
        let n = if flags().trails {
            let room = crate::smoke_types::smoke_system_like_budget(g) - b.trail_reserve;
            m::max(0.0, m::min_n(&[b.trail_count, idx.len() as f64, room]))
        } else {
            0.0
        };
        let tl = trail();
        let mut i: usize = 0;
        while (i as f64) < n {
            let slot = idx[i] as usize;
            let tr = Track::Spark { slot, gen_: g.sparks.pool[slot].gen_ };
            if let Some((pos, _dir)) = track_at(g, &tr) {
                smoke_emit(g, &Emitter {
                    pos: v3::copy(pos), dir: [0.0, 1.0, 0.0], speed: 0.0, push: Some(0.0), radius: tl.radius, min_voxels: Some(tl.min_voxels), density: tl.density * b.density_gain, temperature: tl.temp * b.temp_gain, ttl: tl.life, age: 0.0, kind: Kind::Trail,
                    attack: Some(tl.attack), jitter: Some(0.0), lattice: Some(Lattice::Coarse), confined: Some(true), prio: Some(0.0), track: Some(tr),
                    ..Emitter::zeroed()
                });
            }
            i += 1;
        }
    }
    FxLight { pos: v3::add(at, [0.0, b.light_lift, 0.0]), color: b.light_color, intensity: b.light_power, range: b.light_range, ttl: b.light_duration, radius: Some(b.light_radius) }
}

/// The smoke can venting along `yaw` from `at`; returns the emission time for the hiss. (effects.ts spawnVent)
pub fn spawn_vent(g: &mut crate::game::Game, at: V3, yaw: f64, o: Option<crate::effects::SpawnVentO>) -> f64 {
    let o = o.unwrap_or_default();
    let v = vent();
    let seconds = o.seconds.unwrap_or(v.seconds);
    let dv: V3 = [m::sin(yaw) * v.speed, v.rise, m::cos(yaw) * v.speed];
    smoke_emit(g, &Emitter {
        pos: v3::copy(at), dir: dv, speed: v3::len(dv), radius: v.radius, density: v.density, temperature: v.temp, ttl: seconds, age: 0.0, kind: Kind::Vent,
        push: Some(v.push), expand: Some(v.expand), attack: Some(v.attack), jitter: Some(v.jitter), lattice: Some(Lattice::Coarse), prio: Some(2.0),
        ..Emitter::zeroed()
    });
    seconds
}

/// Where a can lying at `item_pos` vents from, for a jet along `yaw`. (effects.ts ventMouth)
pub fn vent_mouth(item_pos: V3, yaw: f64) -> V3 {
    let v = vent();
    [item_pos[0] + m::sin(yaw) * v.standoff, item_pos[1] + v.height, item_pos[2] + m::cos(yaw) * v.standoff]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(a: V3) -> [u64; 3] {
        [a[0].to_bits(), a[1].to_bits(), a[2].to_bits()]
    }

    // Expected values: the TypeScript's own, under bun with EXACT.on, printed as hex.
    #[test]
    fn muzzle_frame_matches_the_typescript() {
        let f = muzzle_frame([1.5, 1.25, -3.0], [0.3, -0.2, 0.9]);
        assert_eq!(bits(f.o), [0x3ff8000000000000, 0x3ff4000000000000, 0xc008000000000000]);
        assert_eq!(bits(f.x), [0x3fee5b9d136c6d97, 0x0000000000000000, 0xbfd43d136248490f]);
        assert_eq!(bits(f.y), [0x3fb0b318e600b27b, 0x3fef4fceaf414ea4, 0x3fc90ca559010bb8]);
        assert_eq!(bits(f.z), [0x3fd3cda44714c1ba, 0xbfca6785b41bacf8, 0x3fedb4766a9f2297]);
    }
    #[test]
    fn muzzle_frame_straight_up_takes_the_other_axis() {
        let f = muzzle_frame([0.0, 0.0, 0.0], [0.0, 2.0, 0.0]);
        assert_eq!(bits(f.x), [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000]);
        assert_eq!(bits(f.y), [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000]);
        assert_eq!(bits(f.z), [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000]);
    }
    #[test]
    fn vent_mouth_matches_the_typescript() {
        assert_eq!(bits(vent_mouth([2.0, 0.1, -4.0], 0.7)), [0x40005c5ae466fc9c, 0x3fcc28f5c28f5c29, 0xc00f925a2b5aa590]);
    }
    #[test]
    fn a_bursts_draws_match_the_typescript() {
        let want: [u64; 5] = [0x3feb2a76e57f0000, 0x3fe68ec915850000, 0x3feeb0d5125a0000, 0x3fe02c6fb59d4000, 0x3fd408f1063f0000];
        for (k, w) in [1.0, 2.0, 3.0, 41.0, 152.0].iter().zip(want.iter()) {
            assert_eq!(rnd(7.0, *k).to_bits(), *w);
        }
        assert_eq!(bits(wobble(7.0, [0.3, 0.1, -0.9], 0.54, 63.0)), [0x3fdfabe24fb459cf, 0x3fbc192de7c28739, 0xbfeb9595345febde]);
    }
    // `gas: false` must make no call: smoke_system_like_emit is a stub that panics, so reaching it fails this.
    #[test]
    fn a_shot_with_no_gas_files_none_and_still_flashes() {
        let mut g = Game::blank();
        let o = SpawnShotO { gas: Some(false), light_gain: Some(0.5), ..Default::default() };
        let l = spawn_shot(&mut g, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], &o, false);
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].intensity, 45.0);
        assert_eq!(l[0].range, 8.0);
        assert_eq!(l[0].ttl, 0.09);
        assert_eq!(l[0].pos, v3::mad([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 0.22 * 0.4));
    }
    // sparks.ts tracker: rides the spark it was made on, and answers nothing once it dies or the slot goes to another.
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
    #[test]
    fn no_gas_takes_nothing() {
        no_gas::emit();
        no_gas::push();
        assert!(!no_gas::in_smoke_domain());
    }
}
