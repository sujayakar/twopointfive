//! src/game/tracers.ts lines 1 to 167
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A round just cycled out of `w`: is it a tracer? Every `every`-th out of the box, or fewer than `lastN` left. (tracers.ts isTracerRound)
pub fn is_tracer_round(g: &mut crate::game::Game, w: FirearmId) -> bool {
    let def = g.fa[w].def;
    let Some(t) = g.m.weapons.weapons.get(def).unwrap().tracer else { return false; };
    let k = if g.fa[w].infinite { g.fa[w].shots_fired } else { crate::weapons::firearm_spent(g, w) };
    (t.every > 0.0 && k > 0.0 && js::rem(k, t.every) == 0.0) || crate::weapons::firearm_rounds_ready(g, w) < t.last_n
}

/// The pool: every slot made once, with its lamp. The lights are `g.engine.lights`, reached by path. (tracers.ts Tracers.constructor)
pub fn tracers_new(g: &mut crate::game::Game, params: Option<crate::tracers::TracersParams>) -> crate::tracers::Tracers {
    let params = match params { Some(p) => p, None => crate::effects::tracer_fx() };
    let mut this = Tracers::blank();
    this.params = params;
    let n = m::max(1.0, js::to_i32(params.max_live) as f64);
    let mut i = 0.0;
    while i < n {
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
            ttl: LAMP_TTL,
            age: 0.0,
            peak_intensity: 0.0,
            ..crate::render_lights::RtLight::zeroed() // every optional field of the literal is absent
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
            box_: BoxId(0), // the streak's box is the picture's (see notes): nothing here makes or writes it
            lamp,
        });
        i += 1.0;
    }
    this
}

/// Every live streak, copied. (tracers.ts Tracers.each)
pub fn tracers_each(g: &mut crate::game::Game) -> Vec<crate::tracers::Tracer> {
    g.tracers.pool.iter().filter(|t| t.alive).copied().collect()
}

/// fireWeapon's one call, after the round was traced. Returns whether a streak went. (tracers.ts Tracers.round)
pub fn tracers_round(g: &mut crate::game::Game, w: Option<FirearmId>, def: &crate::weapons::WeaponDef, muzzle: V3, dir: V3, t_hit: f64, impact: crate::tracers::TracerImpact, owner: f64, force: Option<bool>) -> bool {
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

/// A free slot's place in the pool, else the oldest streak's (its light handed back first). (tracers.ts Tracers.slot)
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

/// A free slot, else the oldest streak's: the row itself. (tracers.ts Tracers.slot)
pub fn tracers_slot<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::tracers::Tracer {
    let i = slot_at(g);
    &mut g.tracers.pool[i]
}

/// Starts a streak in a slot, with a head light if the budget of lights has one to spare. (tracers.ts Tracers.spawn)
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
    t.light = None;
    let (lamp, slot_owner) = (t.lamp, t.owner);
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
            let mut set = core::mem::take(&mut g.engine.lights);
            crate::render_lights::light_set_add(g, &mut set, lamp);
            g.engine.lights = set;
        }
        g.tracers.pool[i].light = Some(lamp);
        g.tracers.lit += 1.0;
    }
    g.tracers.spawned += 1.0;
    g.tracers.live += 1.0;
}

/// Hands the light of the slot at `i` back to the set (idempotent). (tracers.ts Tracers.release)
fn release_at(g: &mut Game, i: usize) {
    let Some(l) = g.tracers.pool[i].light else { return; };
    let mut set = core::mem::take(&mut g.engine.lights);
    crate::render_lights::light_set_remove(g, &mut set, l);
    g.engine.lights = set;
    g.tracers.pool[i].light = None;
    g.tracers.lit = m::max(0.0, g.tracers.lit - 1.0);
}

/// Hands a streak's light back to the set (idempotent). `t` cannot be the pool's row while `g` is lent, so it is a copy: its slot is found by its lamp, which is the slot's for good, and both are written. (tracers.ts Tracers.release)
pub fn tracers_release(g: &mut crate::game::Game, t: &mut crate::tracers::Tracer) {
    if let Some(i) = g.tracers.pool.iter().position(|r| r.lamp == t.lamp) { release_at(g, i); }
    t.light = None;
}

/// One frame: a head that reached the end last frame retires, the others advance, one arriving now makes its ember, the lights ride the heads. (tracers.ts Tracers.update)
pub fn tracers_update(g: &mut crate::game::Game, dt: f64, out: &mut Vec<BoxId>, m_per_px: Option<f64>) {
    if g.tracers.live == 0.0 { return; }
    let (mut live, mut lit) = (0.0, 0.0);
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
        if t.s <= 1e-4 { continue; }
        g.tracers.pool[i].frames += 1.0;
    }
    g.tracers.live = live;
    g.tracers.lit = lit;
}

/// The brief glow where a tracer stops in geometry; nothing in a man, nothing for a round that met nothing. (tracers.ts Tracers.ember)
pub fn tracers_ember(g: &mut crate::game::Game, t: crate::tracers::Tracer) {
    if t.impact != TracerImpact::Solid { return; }
    let p = g.tracers.params;
    let at = [t.from[0] + t.dir[0] * (t.l - 0.03), t.from[1] + t.dir[1] * (t.l - 0.03), t.from[2] + t.dir[2] * (t.l - 0.03)];
    if p.ember_intensity > 0.0 {
        let mut set = core::mem::take(&mut g.engine.lights);
        let l = crate::render_lights::light_set_flash(g, &mut set, at, v3::copy(t.color), p.ember_intensity, p.ember_range, p.ember_ttl, 0.0);
        g.engine.lights = set;
        crate::fogofwar::fog_of_war_light_by(g, l, Some(t.owner));
    }
}

/// Everything gone at once: lights handed back, slots freed. (tracers.ts Tracers.clear)
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
            let mut t = Tracer::zeroed();
            t.lamp = LightId(i);
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
        tracers_update(&mut g, 0.0, &mut out, None); // a frozen still: the head has not left, nothing drawn
        assert_eq!((g.tracers.pool[0].frames, g.tracers.live), (0.0, 1.0));
        tracers_update(&mut g, 0.1, &mut out, None); // reaches the end: stays one frame
        let t = g.tracers.pool[0];
        assert_eq!((t.s, t.arrived, t.alive, t.frames, g.tracers.live), (1.0, true, true, 1.0, 1.0));
        tracers_update(&mut g, 0.0, &mut out, None); // dt 0 never retires
        assert!(g.tracers.pool[0].alive);
        tracers_update(&mut g, 0.1, &mut out, None);
        assert_eq!((g.tracers.pool[0].alive, g.tracers.live), (false, 0.0));
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
        assert_eq!((tracers_each(&mut g).len(), g.tracers.live), (0, 0.0));
    }) }

    /// A slot taken through the public door is the row itself, and what is written through it stays.
    #[test]
    fn a_write_through_the_slot_lands_in_the_pool() { on_a_big_stack(|| {
        let mut g = game(2);
        tracers_slot(&mut g).age = 7.0;
        assert_eq!(g.tracers.pool[0].age, 7.0);
    }) }

    /// The row's own numbers decide, through the weapons table. Only the cases that stop before Firearm's stubs.
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

    /// The constructor's slots, each with its own lamp in the table, as the literal has it, and not in the set.
    #[test]
    fn each_slot_is_made_with_its_own_lamp() { on_a_big_stack(|| {
        let mut g = Box::new(Game::blank());
        let (before, mut p) = (g.tab.light.len(), crate::effects::tracer_fx());
        p.max_live = 2.9;
        let t = tracers_new(&mut g, Some(p));
        assert_eq!((t.pool.len(), g.tab.light.len() - before, t.pool[0].dir, t.pool[0].color), (2, 2, [0.0, 0.0, 1.0], [1.0, 0.5, 0.2]));
        assert!(t.pool[0].lamp != t.pool[1].lamp && t.pool[0].light.is_none() && g.engine.lights.lights.is_empty());
        let l = &g.tab.light[t.pool[1].lamp];
        assert_eq!((l.pos, l.dir, l.range, l.radius, l.ttl, l.fixture_box, l.outer_deg, l.enabled), ([0.0, -10.0, 0.0], [0.0, -1.0, 0.0], 4.0, 0.06, 30.0, -1.0, 180.0, true));
        p.max_live = 0.0; // never fewer than one
        assert_eq!(tracers_new(&mut g, Some(p)).pool.len(), 1);
    }) }

    /// A lit streak's light rides 5 cm behind the head, its age held at 0, and the count of lit heads is the walk's.
    #[test]
    fn the_light_rides_the_head() { on_a_big_stack(|| {
        let mut g = game(1);
        let lamp = g.tab.light.push(crate::render_lights::RtLight::zeroed());
        g.tracers.pool[0].lamp = lamp;
        tracers_spawn(&mut g, [1.0, 2.0, 3.0], [0.0, 0.0, 1.0], 50.0, TracerImpact::None, [1.0; 3], 0.0);
        g.tracers.pool[0].light = Some(lamp); // lit by hand: light_set_add is a stub here
        g.tab.light[lamp].age = 9.0;
        let mut out = Vec::new();
        tracers_update(&mut g, 0.1, &mut out, None);
        let (l, s) = (&g.tab.light[lamp], g.tracers.pool[0].s);
        assert_eq!((l.age, l.enabled, l.pos, g.tracers.lit), (0.0, true, [1.0 + 0.0 * (s - 0.05), 2.0 + 0.0 * (s - 0.05), 3.0 + 1.0 * (s - 0.05)], 1.0));
    }) }

    /// `force` needs no gun and no row's tracer, and takes the default colour; without it a row with no tracer sends nothing; a contact shot never goes.
    #[test]
    fn a_round_goes_by_force_or_not_at_all() { on_a_big_stack(|| {
        let mut g = game(1);
        let pistol = g.m.weapons.weapons.get(crate::weapons::WeaponId::Fiveseven).unwrap().clone();
        let d = [0.0, 0.0, 1.0];
        assert!(!tracers_round(&mut g, None, &pistol, [0.0; 3], d, 10.0, TracerImpact::None, 0.0, None));
        assert!(!tracers_round(&mut g, None, &pistol, [0.0; 3], d, 0.1, TracerImpact::None, 0.0, Some(true)));
        assert!(!tracers_round(&mut g, None, &pistol, [0.0; 3], d, f64::INFINITY, TracerImpact::None, 0.0, Some(true)));
        assert_eq!(g.tracers.spawned, 0.0);
        assert!(tracers_round(&mut g, None, &pistol, [0.0; 3], d, 10.0, TracerImpact::None, 0.0, Some(true)));
        assert_eq!((g.tracers.spawned, g.tracers.pool[0].color), (1.0, [1.0, 0.42, 0.12]));
    }) }
}
