pub use ::game_types::tracers::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A slot's streak: one thin emissive box, dynamic and shadowless, parked under the floor.
fn streak_box_p() -> crate::scene_boxes::MakeBoxP {
    let f = crate::scene_boxes::box_flag();
    crate::scene_boxes::MakeBoxP {
        c: [0.0, -10.0, 0.0],
        h: [0.006, 0.006, 0.05],
        albedo: Some([0.04, 0.03, 0.02]),
        emissive: Some([0.0, 0.0, 0.0]),
        flags: Some((js::to_i32(f.dynamic) | js::to_i32(f.no_shadow)) as f64),
        ..Default::default()
    }
}

/// A round just cycled out of `w`: is it a tracer? Every `every`-th out of the box, or fewer than `lastN` left.
fn is_tracer_round(g: &crate::game::Game, w: FirearmId) -> bool {
    let def = g.fa[w].def;
    let Some(t) = g.weapon(def).tracer else { return false; };
    let k = if g.fa[w].infinite { g.fa[w].shots_fired } else { crate::weapons::firearm_spent(g, w) };
    (t.every > 0.0 && k > 0.0 && js::rem(k, t.every) == 0.0) || crate::weapons::firearm_rounds_ready(g, w) < t.last_n
}

/// The pool: every slot made once, with its lamp. The lights are `g.engine.lights`, reached by path.
pub fn tracers_new(g: &mut crate::game::Game, params: Option<crate::tracers::TracersParams>) -> crate::tracers::Tracers {
    let params = match params { Some(p) => p, None => crate::effects::tracer_fx() };
    let mut this = Tracers::blank();
    this.params = params;
    let n = m::max(1.0, js::to_i32(params.max_live) as f64);
    let mut i = 0.0;
    while i < n {
        let box_ = crate::scene_boxes::make_box(g, &streak_box_p());
        let lamp = g.tab.light.push(crate::render_lights::RtLight {
            name: String::from("tracer"),
            group: String::from("tracer"),
            kind: 0.0,
            pos: [0.0, -10.0, 0.0],
            dir: [0.0, -1.0, 0.0],
            color: [1.0, 0.5, 0.2],
            intensity: 0.0,
            range: params.light_range,
            inner_deg: 0.0,
            outer_deg: 180.0,
            radius: params.light_radius,
            volumetric: 1.0,
            owner: 0.0,
            enabled: true,
            fixture_box: -1.0,
            flicker: 0.0,
            ttl: LAMP_TTL,
            age: 0.0,
            peak_intensity: 0.0,
            ..crate::render_lights::RtLight::default() // every optional field of the literal is absent
        });
        this.pool.push(Tracer {
            alive: false,
            from: [0.0, 0.0, 0.0],
            dir: [0.0, 0.0, 1.0],
            l: 0.0,
            s: 0.0,
            v: 0.0,
            age: 0.0,
            frames: 0.0,
            color: [1.0, 0.5, 0.2],
            owner: 0.0,
            impact: TracerImpact::None,
            arrived: false,
            light: None,
            box_,
            lamp,
        });
        i += 1.0;
    }
    this
}

/// fireWeapon's one call, after the round was traced. Returns whether a streak went.
pub(super) fn tracers_round(g: &mut crate::game::Game, w: Option<FirearmId>, def: &crate::weapons::WeaponDef, muzzle: V3, dir: V3, t_hit: f64, impact: crate::tracers::TracerImpact, owner: f64, force: Option<bool>) -> bool {
    let force = force.unwrap_or(false);
    let spec = def.tracer;
    if spec.is_none() && !force { return false; }
    if !force {
        // (a round off a row the shooter does not carry has no counters to be the third of anything)
        let Some(w) = w else { return false; };
        if !(g.fa[w].def == def.id && is_tracer_round(g, w)) { return false; }
    }
    if !(t_hit >= g.tracers.params.min_path) || !t_hit.is_finite() { return false; }
    let color = match spec { Some(t) => t.color, None => [1.0, 0.42, 0.12] };
    tracers_spawn(g, muzzle, dir, t_hit, impact, color, owner);
    true
}

/// A free slot's place in the pool, else the oldest streak's (its light handed back first).
fn slot_at(g: &mut Game) -> usize {
    let (mut oldest, mut oa) = (0, -1.0);
    for i in 0..g.tracers.pool.len() {
        let t = &g.tracers.pool[i];
        if !t.alive { return i; }
        if t.age > oa { oa = t.age; oldest = i; }
    }
    release_at(g, oldest);
    g.tracers.pool[oldest].alive = false;
    g.tracers.live = m::max(0.0, g.tracers.live - 1.0);
    oldest
}

/// Starts a streak in a slot, with a head light if the budget of lights has one to spare.
pub fn tracers_spawn(g: &mut crate::game::Game, from: V3, dir: V3, l: f64, impact: crate::tracers::TracerImpact, color: V3, owner: f64) {
    let p = g.tracers.params;
    let i = slot_at(g);
    let dl = js::or(m::hyp3(dir[0], dir[1], dir[2]), 1.0);
    let t = &mut g.tracers.pool[i];
    t.alive = true; t.arrived = false; t.age = 0.0; t.frames = 0.0; t.s = 0.0; t.l = l; t.impact = impact; t.owner = (js::to_i32(owner) & 0xff) as f64;
    t.from = from;
    t.dir = [dir[0] / dl, dir[1] / dl, dir[2] / dl];
    t.color = color;
    t.v = m::min(p.speed, l / m::max(1e-3, p.min_life));
    let unit = t.dir;
    t.light = None;
    let (lamp, slot_owner, b) = (t.lamp, t.owner, t.box_);
    let mut row = g.tab.boxes.edit(b);
    row.owner = slot_owner;
    row.rot = Some(v3::quat::from_to([0.0, 0.0, 1.0], unit));
    row.yaw = v3::yaw_of(unit);
    drop(row);
    if g.tracers.lit < p.max_lights {
        let l = &mut g.tab.light[lamp];
        l.color = color;
        l.intensity = p.light_intensity;
        l.peak_intensity = l.intensity;
        l.range = p.light_range;
        l.radius = p.light_radius;
        l.owner = slot_owner;
        l.age = 0.0;
        l.ttl = LAMP_TTL;
        l.enabled = true;
        l.pos = from;
        if !g.engine.lights.lights.contains(&lamp) {
            crate::render_lights::light_set_add(g, lamp);
        }
        g.tracers.pool[i].light = Some(lamp);
        g.tracers.lit += 1.0;
    }
    g.tracers.spawned += 1.0;
    g.tracers.live += 1.0;
}

/// Hands the light of the slot at `i` back to the set (idempotent).
fn release_at(g: &mut Game, i: usize) {
    let Some(l) = g.tracers.pool[i].light else { return; };
    crate::render_lights::light_set_remove(g, l);
    g.tracers.pool[i].light = None;
    g.tracers.lit = m::max(0.0, g.tracers.lit - 1.0);
}

/// One frame: a head that reached the end last frame retires, the others advance, one arriving now makes its ember, the lights ride the heads, each streak's box is rewritten tail to head and goes out.
pub fn tracers_update(g: &mut crate::game::Game, dt: f64, out: &mut Vec<AnyBoxId>) {
    if g.tracers.live == 0.0 { return; }
    let p = g.tracers.params;
    let (mut live, mut lit) = (0.0, 0.0);
    let hw = p.thickness * 0.5;
    for i in 0..g.tracers.pool.len() {
        if !g.tracers.pool[i].alive { continue; }
        if g.tracers.pool[i].arrived && dt > 0.0 {
            release_at(g, i);
            g.tracers.pool[i].alive = false;
            continue;
        }
        let t = &mut g.tracers.pool[i];
        t.age += dt;
        t.s = m::min(t.l, t.s + t.v * dt);
        if t.s >= t.l - 1e-6 && !t.arrived {
            t.arrived = true;
            let row = *t;
            tracers_ember(g, row);
        }
        let t = g.tracers.pool[i];
        if let Some(l) = t.light {
            // (5 cm short of the head: never inside the surface it stops at)
            let lamp = &mut g.tab.light[l];
            lamp.age = 0.0;
            lamp.enabled = true;
            lamp.pos = [t.from[0] + t.dir[0] * (t.s - 0.05), t.from[1] + t.dir[1] * (t.s - 0.05), t.from[2] + t.dir[2] * (t.s - 0.05)];
            lit += 1.0;
        }
        live += 1.0;
        if t.s <= 1e-4 { continue; } // nothing has left the muzzle yet
        let tail = m::max(0.0, t.s - t.v * p.shutter);
        let len = m::max(0.05, t.s - tail);
        let mid = t.s - len * 0.5;
        let mut b = g.tab.boxes.edit(t.box_);
        b.c = [t.from[0] + t.dir[0] * mid, t.from[1] + t.dir[1] * mid, t.from[2] + t.dir[2] * mid];
        b.h = [hw, hw, len * 0.5];
        b.emissive = [t.color[0] * p.emissive, t.color[1] * p.emissive, t.color[2] * p.emissive];
        drop(b);
        crate::thinline::thin_note(g, t.box_.into(), hw, 0.6);
        g.tracers.pool[i].frames += 1.0;
        out.push(t.box_.into());
    }
    g.tracers.live = live;
    g.tracers.lit = lit;
}

/// The splash where a tracer stops in geometry, its shooter's doing: a few sparks back off the surface and a brief glow. Nothing in a man, nothing for a round that met nothing.
fn tracers_ember(g: &mut crate::game::Game, t: crate::tracers::Tracer) {
    if t.impact != TracerImpact::Solid { return; }
    let p = g.tracers.params;
    let at = [t.from[0] + t.dir[0] * (t.l - 0.03), t.from[1] + t.dir[1] * (t.l - 0.03), t.from[2] + t.dir[2] * (t.l - 0.03)];
    let was = crate::fogofwar::fog_of_war_begin(g, Some(t.owner));
    if p.ember_sparks > 0.0 {
        let back = [-t.dir[0], -t.dir[1] + 0.3, -t.dir[2]];
        crate::particle_fx::ember_fx(g, at, back, crate::particle_fx::EmberFxO { count: p.ember_sparks, speed: p.ember_speed, cone: p.ember_cone, life: p.ember_life });
    }
    if p.ember_intensity > 0.0 {
        let l = crate::render_lights::light_set_flash(g, at, v3::copy(t.color), p.ember_intensity, p.ember_range, p.ember_ttl, 0.0);
        crate::fogofwar::fog_of_war_light_by(g, l, Some(t.owner));
    }
    crate::fogofwar::fog_of_war_done(g, was);
}

/// Everything gone at once: lights handed back, slots freed.
pub fn tracers_clear(g: &mut crate::game::Game) {
    for i in 0..g.tracers.pool.len() {
        if !g.tracers.pool[i].alive { continue; }
        release_at(g, i);
        g.tracers.pool[i].alive = false;
    }
    g.tracers.live = 0.0;
    g.tracers.lit = 0.0;
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    /// `Game::blank()` is built on the stack, and in a debug build it is larger than a test thread's.
    fn on_a_big_stack(f: fn()) {
        std::thread::Builder::new().stack_size(1 << 28).spawn(f).unwrap().join().unwrap();
    }

    /// A game whose pool has `n` slots and no light budget, so the set's stubs are not reached.
    fn game(n: u32) -> Box<Game> {
        let mut g = Box::new(Game::blank());
        g.tracers = Tracers::blank();
        g.tracers.params.max_lights = 0.0;
        for i in 0..n {
            let mut t = Tracer::default();
            t.lamp = LightId(i);
            t.box_ = crate::scene_boxes::make_box(&mut g, &streak_box_p());
            g.tracers.pool.push(t);
        }
        g
    }

    #[test]
    fn a_short_shot_is_slowed_arrives_shows_once_and_goes() { on_a_big_stack(|| {
        let mut g = game(2);
        tracers_spawn(&mut g, [0.0, 1.0, 0.0], [0.0, 0.0, 2.0], 1.0, TracerImpact::Flesh, [1.0, 0.0, 0.0], 0x1ff as f64);
        let t = g.tracers.pool[0];
        assert_eq!((t.owner, t.dir, t.v), (255.0, [0.0, 0.0, 1.0], 1.0 / 0.042));
        assert_eq!((g.tracers.live, g.tracers.spawned), (1.0, 1.0));
        let mut out = Vec::new();
        tracers_update(&mut g, 0.0, &mut out); // a frozen still: the head has not left, nothing drawn
        assert_eq!((g.tracers.pool[0].frames, g.tracers.live, out.len()), (0.0, 1.0, 0));
        tracers_update(&mut g, 0.1, &mut out); // reaches the end: stays one frame, drawn there
        let t = g.tracers.pool[0];
        assert_eq!((t.s, t.arrived, t.alive, t.frames, g.tracers.live), (1.0, true, true, 1.0, 1.0));
        assert_eq!(out, vec![AnyBoxId::World(t.box_)]);
        tracers_update(&mut g, 0.0, &mut out); // dt 0 never retires, and redraws where it is
        assert!(g.tracers.pool[0].alive);
        assert_eq!(out.len(), 2);
        tracers_update(&mut g, 0.1, &mut out);
        assert_eq!((g.tracers.pool[0].alive, g.tracers.live, out.len()), (false, 0.0, 2));
    }) }

    #[test]
    fn a_full_pool_gives_up_its_oldest() { on_a_big_stack(|| {
        let mut g = game(2);
        for _ in 0..2 { tracers_spawn(&mut g, [0.0; 3], [0.0, 0.0, 1.0], 50.0, TracerImpact::None, [1.0; 3], 0.0); }
        g.tracers.pool[0].age = 0.2;
        g.tracers.pool[1].age = 0.3;
        tracers_spawn(&mut g, [5.0, 0.0, 0.0], [0.0, 0.0, 1.0], 50.0, TracerImpact::None, [1.0; 3], 0.0);
        assert_eq!((g.tracers.pool[1].from, g.tracers.pool[1].age, g.tracers.pool[0].from), ([5.0, 0.0, 0.0], 0.0, [0.0; 3]));
        assert_eq!((g.tracers.live, g.tracers.spawned), (2.0, 3.0));
        tracers_clear(&mut g);
        assert_eq!((g.tracers.pool.iter().filter(|t| t.alive).count(), g.tracers.live), (0, 0.0));
    }) }

    /// The row's own numbers decide, through the weapons table.
    #[test]
    fn the_row_says_which_round_is_a_tracer() { on_a_big_stack(|| {
        let mut g = game(1);
        let mut gun = crate::weapons::Firearm::blank();
        gun.def = crate::weapons::WeaponId::Aks74u;
        gun.infinite = true;
        gun.shots_fired = 3.0;
        let w = g.fa.push(gun);
        assert!(is_tracer_round(&mut g, w)); // every third of the AKS-74U's
        g.fa[w].def = crate::weapons::WeaponId::Fiveseven;
        assert!(!is_tracer_round(&mut g, w)); // a pistol's row declares none
    }) }

    /// A pool's slots, each with its own box parked under the floor and its own lamp in the table, not yet in the set of lights; never fewer than one.
    #[test]
    fn each_slot_is_made_with_its_own_lamp() { on_a_big_stack(|| {
        let mut g = Box::new(Game::blank());
        let (before, mut p) = (g.tab.light.len(), crate::effects::tracer_fx());
        p.max_live = 2.9;
        let t = tracers_new(&mut g, Some(p));
        assert_eq!((t.pool.len(), g.tab.light.len() - before, t.pool[0].dir, t.pool[0].color), (2, 2, [0.0, 0.0, 1.0], [1.0, 0.5, 0.2]));
        assert!(t.pool[0].lamp != t.pool[1].lamp && t.pool[0].light.is_none() && g.engine.lights.lights.is_empty());
        assert!(t.pool[0].box_ != t.pool[1].box_);
        let b = &g.tab.boxes[t.pool[1].box_];
        assert_eq!((b.c, b.h, b.albedo, b.emissive, b.flags), ([0.0, -10.0, 0.0], [0.006, 0.006, 0.05], [0.04, 0.03, 0.02], [0.0; 3], 10.0));
        let l = &g.tab.light[t.pool[1].lamp];
        assert_eq!((l.pos, l.dir, l.range, l.radius, l.ttl, l.fixture_box, l.outer_deg, l.enabled), ([0.0, -10.0, 0.0], [0.0, -1.0, 0.0], 4.0, 0.06, 30.0, -1.0, 180.0, true));
        p.max_live = 0.0; // never fewer than one
        assert_eq!(tracers_new(&mut g, Some(p)).pool.len(), 1);
    }) }

    /// A lit streak's light rides 5 cm behind the head, its age held at 0, and the count of lit heads is the walk's.
    #[test]
    fn the_light_rides_the_head() { on_a_big_stack(|| {
        let mut g = game(1);
        let lamp = g.tab.light.push(crate::render_lights::RtLight::default());
        g.tracers.pool[0].lamp = lamp;
        tracers_spawn(&mut g, [1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 50.0, TracerImpact::None, [1.0; 3], 0.0);
        g.tracers.pool[0].light = Some(lamp); // lit by hand: this game has no set of lights
        g.tab.light[lamp].age = 9.0;
        let mut out = Vec::new();
        tracers_update(&mut g, 0.1, &mut out);
        let (l, s) = (&g.tab.light[lamp], g.tracers.pool[0].s);
        assert_eq!((l.age, l.enabled, l.pos, g.tracers.lit), (0.0, true, [1.0 + 0.0 * (s - 0.05), 2.0 + 0.0 * (s - 0.05), 3.0 + 1.0 * (s - 0.05)], 1.0));
    }) }

    /// A streak is a thin lit box, its head where the round has got to and its tail a shutter's flight behind, as thick as the effects table says whatever view is shown, pointing along the round. It stops at the end of its flight.
    #[test]
    fn a_streak_is_a_thin_lit_box_behind_the_head() { on_a_big_stack(|| {
        let mut g = game(1);
        let p = g.tracers.params;
        let mut def = g.weapon(crate::weapons::WeaponId::Fiveseven).clone();
        def.tracer = Some(crate::weapons::TracerSpec { every: 3.0, last_n: 3.0, color: [1.0, 0.6, 0.25] });
        let (from, dir, l): (V3, V3, f64) = ([1.25, 1.5, -3.75], [0.6, 0.0, 0.8], 7.3);
        assert!(tracers_round(&mut g, None, &def, from, [1.2, 0.0, 1.6], l, TracerImpact::None, 0x1ff as f64, Some(true)));
        let t = g.tracers.pool[0];
        let b = t.box_;
        assert!(v3::dist(t.dir, dir) < 1e-12, "the way the round went, as a unit: {:?}", t.dir);
        assert_eq!(t.v, m::min(p.speed, l / p.min_life));
        assert!(v3::dist(v3::quat::rotate(g.tab.boxes[b].rot.unwrap(), [0.0, 0.0, 1.0]), dir) < 1e-12, "the box lies along the round");
        assert_eq!(g.tab.boxes[b].yaw, v3::yaw_of(dir));
        assert_eq!(g.tab.boxes[b].owner, 255.0, "the shooter's byte");
        let mut out = Vec::new();
        for dt in [0.016, 0.002] {
            tracers_update(&mut g, dt, &mut out);
            let s = g.tracers.pool[0].s;
            let (c, h, e) = (g.tab.boxes[b].c, g.tab.boxes[b].h, g.tab.boxes[b].emissive);
            assert_eq!((h[0], h[1]), (p.thickness * 0.5, p.thickness * 0.5));
            let head = v3::mad(c, dir, h[2]);
            let want = v3::mad(from, dir, s);
            assert!(v3::dist(head, want) < 1e-9, "the head {:?} is where the round is {:?}", head, want);
            assert!(h[2] * 2.0 <= t.v * p.shutter + 1e-9 && h[2] * 2.0 <= s + 1e-9, "the tail is no further back than a shutter or the muzzle");
            assert_eq!(e, [1.0 * p.emissive, 0.6 * p.emissive, 0.25 * p.emissive]);
        }
        tracers_update(&mut g, 10.0, &mut out);
        assert_eq!(g.tracers.pool[0].s, l, "no further than the end of its flight");
        assert_eq!((g.tracers.pool[0].frames, out.len()), (3.0, 3), "drawn once a frame");
    }) }

    /// `force` needs no gun and no row's tracer, and takes the default colour; without it a row with no tracer sends nothing; a contact shot never goes.
    #[test]
    fn a_round_goes_by_force_or_not_at_all() { on_a_big_stack(|| {
        let mut g = game(1);
        let pistol = g.weapon(crate::weapons::WeaponId::Fiveseven).clone();
        let d = [0.0, 0.0, 1.0];
        assert!(!tracers_round(&mut g, None, &pistol, [0.0; 3], d, 10.0, TracerImpact::None, 0.0, None));
        assert!(!tracers_round(&mut g, None, &pistol, [0.0; 3], d, 0.1, TracerImpact::None, 0.0, Some(true)));
        assert!(!tracers_round(&mut g, None, &pistol, [0.0; 3], d, f64::INFINITY, TracerImpact::None, 0.0, Some(true)));
        assert_eq!(g.tracers.spawned, 0.0);
        assert!(tracers_round(&mut g, None, &pistol, [0.0; 3], d, 10.0, TracerImpact::None, 0.0, Some(true)));
        assert_eq!((g.tracers.spawned, g.tracers.pool[0].color), (1.0, [1.0, 0.42, 0.12]));
    }) }
}
