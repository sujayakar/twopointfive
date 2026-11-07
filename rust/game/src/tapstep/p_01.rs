//! src/game/tapstep.ts lines 1 to 121
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// This seat's row of the table, written through by its key. Re-got after every call that takes `g`.
macro_rules! row {
    ($g:ident) => {
        $g.m.tapstep.state.get_mut($g.player).unwrap()
    };
    ($g:ident, $pl:ident) => {
        $g.m.tapstep.state.get_mut($pl).unwrap()
    };
}

/// The row a seat starts with. (tapstep.ts stateOf's literal)
fn fresh_row() -> St {
    St { st: 0.0, t: 9.0, at: -1.0, keys: false, dir: [0.0, 1.0], v: 0.0, a: 0.0, go: 0.0, left: 0.0, age: 0.0, secs: 0.0, done: 0.0, len: 0.0, least: 0.0, rush: 0.0, u: 1.0, n: 0.0, way: 1.0, stuck: 0.0, prev: None, asked: 0.0, man: None, mask_t: -9.0, set: false }
}

/// This seat's state, made on first asking; the row itself. (tapstep.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::tapstep::St {
    g.m.tapstep.state.get_or_insert_with(g.player, fresh_row)
}

/// Is he stepping this frame: the footfalls are the plants', not the cycle's. (tapstep.ts tapHeld)
pub fn tap_held(g: &mut crate::game::Game) -> bool {
    let on = g.k.tapstep.on;
    match g.m.tapstep.state.get(g.player) {
        Some(s) => on && s.at >= 0.0 && g.time - s.at < 0.1,
        None => false,
    }
}

/// A man's footfall: a plant just after it is under it. (tapstep.ts tapHears)
pub fn tap_hears(g: &mut crate::game::Game, pos: &[f64]) {
    let p = g.ch[g.pl[g.player].ch].pos;
    if m::hyp2(pos[0] - p[0], pos[2] - p[2]) <= g.k.tapstep.mask_m {
        let now = g.time;
        state_of(g).mask_t = now;
    }
}

/// Forget this seat's state. (tapstep.ts tapReset)
pub fn tap_reset(g: &mut crate::game::Game) {
    g.m.tapstep.state.delete(g.player);
}

/// A man whose back this way goes to, and the spot `keyM` behind him; standing on the spot already is no step to it. (tapstep.ts backOf)
pub fn back_of(g: &mut crate::game::Game, dx: f64, dz: f64, far: f64) -> Option<crate::tapstep::BackOf> {
    let p = g.ch[g.pl[g.player].ch].pos;
    let mut best: Option<BackOf> = None;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !crate::stalk::stalk_could(g, gd) { continue; }
        let SpotOf { x, z } = spot_of(g, gd);
        let (ax, az) = (x - p[0], z - p[2]);
        let d = m::hyp2(ax, az);
        if d > far || d < 0.15 || (ax * dx + az * dz) / d < g.k.tapstep.land_cos { continue; }
        if best.map_or(true, |b| d < b.d) { best = Some(BackOf { gd, x, z, d }); }
    }
    best
}

/// The spot `keyM` behind a man. (tapstep.ts spotOf)
pub fn spot_of(g: &mut crate::game::Game, gd: GuardId) -> crate::tapstep::SpotOf {
    let c = g.gd[gd].ch;
    let key_m = g.k.stalk.key_m;
    let (pos, yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
    SpotOf { x: pos[0] - m::sin(yaw) * key_m, z: pos[2] - m::cos(yaw) * key_m }
}

/// How long a step of `len` m takes from rest to rest. (tapstep.ts stepSecs)
pub fn step_secs(g: &mut crate::game::Game, len: f64) -> f64 {
    secs_of(&g.k.tapstep, len)
}
fn secs_of(t: &crate::knobs::TapstepK, len: f64) -> f64 {
    t.secs * m::sqrt(m::max(len, 0.05) / t.reach_m)
}

/// The foot leaves the ground; `pl` is the key of the row. (tapstep.ts lift)
pub fn lift(g: &mut crate::game::Game, pl: PlayerId, mx: f64, mz: f64, rush: f64, least: f64) {
    let c = g.pl[g.player].ch;
    let f = crate::character::character_forward(g, c);
    let far = g.k.tapstep.land_m;
    let near = back_of(g, mx, mz, far);
    let reach_m = g.k.tapstep.reach_m;
    let r = row!(g, pl);
    r.st = 1.0; r.dir = [mx, mz]; r.a = 0.0; r.v = 0.0; r.done = 0.0; r.age = 0.0;
    r.len = match near { Some(b) => b.d, None => reach_m };
    r.least = least; r.rush = rush; r.u = 0.0; r.n += 1.0; r.stuck = 0.0;
    r.man = near.map(|b| b.gd);
    r.set = false;
    r.way = if mx * f[0] + mz * f[2] < -0.3 { -1.0 } else { 1.0 };
    let len = r.len;
    let secs = step_secs(g, len) * (1.0 - g.k.tapstep.rushed * rush);
    let r = row!(g, pl);
    r.secs = secs; r.left = r.secs;
    g.stats.tapstep.steps += 1;
}

/// The foot comes down: its footfall, and his weight begins to settle; `pl` is the key of the row. (tapstep.ts plant)
pub fn plant(g: &mut crate::game::Game, pl: PlayerId) {
    let t = g.k.tapstep;
    let c = g.pl[pl].ch;
    let (mask_t, rush) = { let r = row!(g, pl); (r.mask_t, r.rush) };
    let under = g.time - mask_t <= t.mask_secs;
    let base = t.quiet + (t.scuff - t.quiet) * rush;
    let pos = g.ch[c].pos;
    let heard = base * crate::kitload::load_heard(g, crate::kitload::LoadHeardWho::Player);
    let level = crate::floor::tread(g, pos, heard, true) * (if under { t.mask } else { 1.0 });
    let pos = g.ch[c].pos;
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Step, pos: [pos[0], pos[1], pos[2]], time, loud: false, level: Some(level), serial, ..crate::game::GameEvent::zeroed() });
    g.pl[pl].noise = m::max(g.pl[pl].noise, level);
    let (rush, man) = { let r = row!(g, pl); (r.rush, r.man) };
    if rush > 0.0 { g.stats.tapstep.scuffs += 1; }
    if under { g.stats.tapstep.masked += 1; }
    if man.is_some() { g.stats.tapstep.lands += 1; }
    g.stats.tapstep.last = String::from(if man.is_some() { "landed behind him" } else if under { "under his footfall" } else if rush > 0.0 { "a scuff" } else { "quiet" });
    let r = row!(g, pl);
    r.st = 2.0; r.t = t.settle; r.a = 0.0; r.v = 0.0; r.man = None;
}

/// A step in the air comes round toward (x, z). (tapTick's `bend`)
fn bend(g: &mut Game, x: f64, z: f64, k: f64) {
    let r = row!(g);
    let a = r.dir[0] + (x - r.dir[0]) * k;
    let b = r.dir[1] + (z - r.dir[1]) * k;
    let l = m::hyp2(a, b);
    if l > 1e-3 { r.dir = [a / l, b / l]; }
}

/// The key let go fixes where the step lands; `least_secs` is how long a step of `least` takes. (tapTick)
fn key_let_go(s: &mut St, least_secs: f64) {
    s.set = true;
    s.left = m::min(s.left, m::max(0.22, least_secs - s.age));
    s.len = m::min(s.len, m::max(s.least, s.done + s.v * s.left * 0.5));
}

/// A frame of the step on the least-jerk way from where it is to rest at its end. (tapTick)
fn walk(s: &mut St, dt: f64) {
    let rem = s.len - s.done;
    let tau = m::max_n(&[s.left, dt, 1.0 / 240.0]);
    let (v, ac) = (s.v, s.a);
    let t2 = tau * tau;
    let h = m::min(dt, tau);
    let h2 = h * h;
    let c3 = (20.0 * rem - 12.0 * v * tau - 3.0 * ac * t2) / (2.0 * t2 * tau);
    let c4 = (-30.0 * rem + 16.0 * v * tau + 3.0 * ac * t2) / (2.0 * t2 * t2);
    let c5 = (12.0 * rem - 6.0 * v * tau - ac * t2) / (2.0 * t2 * t2 * tau);
    let by = v * h + ac * h2 / 2.0 + c3 * h2 * h + c4 * h2 * h2 + c5 * h2 * h2 * h;
    s.v = v + ac * h + 3.0 * c3 * h2 + 4.0 * c4 * h2 * h + 5.0 * c5 * h2 * h2;
    s.a = ac + 6.0 * c3 * h + 12.0 * c4 * h2 + 20.0 * c5 * h2 * h;
    s.done += by;
    s.go = if dt > 0.0 { m::max(0.0, by / dt) } else { 0.0 };
}

/// The legs follow the ground the step has covered. (tapTick)
fn legs_follow(s: &mut St, legs: f64, dt: f64) {
    if s.st == 0.0 { return; }
    let want = if s.st == 1.0 { v3::clamp(s.done / m::max(s.len, 0.05), 0.0, 1.0) } else { 1.0 };
    s.u += m::max(0.0, want - s.u) * (1.0 - m::exp(-legs * dt));
    if s.st == 2.0 && 1.0 - s.u < 0.01 { s.u = 1.0; }
}

/// A frame of it, crouched with his feet his own: the keys in, the step out. (tapstep.ts tapTick)
pub fn tap_tick(g: &mut crate::game::Game, mx: f64, mz: f64, ml: f64, dt: f64) -> crate::tapstep::TapMove {
    state_of(g);
    let t = g.k.tapstep;
    let c = g.pl[g.player].ch;
    let pos = g.ch[c].pos;
    let now = g.time;
    let (fresh, moved, asked) = {
        let s = row!(g);
        let fresh = ml > 0.0 && !s.keys;
        let moved = match s.prev { Some(q) => m::hyp2(pos[0] - q[0], pos[2] - q[2]), None => 0.0 };
        let asked = s.asked;
        s.keys = ml > 0.0; s.at = now; s.prev = Some([pos[0], pos[1], pos[2]]);
        (fresh, moved, asked)
    };

    let st = row!(g).st;
    if st == 2.0 {
        let s = row!(g);
        s.t -= dt;
        if fresh {
            let rush = v3::clamp(s.t / t.settle, 0.0, 1.0);
            let pl = g.player;
            lift(g, pl, mx, mz, rush, t.tap_m);
        } else if s.t <= 0.0 { s.st = 0.0; s.t = 0.0; }
    } else if st == 0.0 {
        row!(g).t += dt;
    }

    let st = row!(g).st;
    if st == 0.0 && ml > 0.0 {
        let beat = fresh && row!(g).t <= t.beat;
        if beat { g.stats.tapstep.beats += 1; }
        let pl = g.player;
        lift(g, pl, mx, mz, 0.0, if beat { t.beat_m } else { t.tap_m });
    } else if st == 1.0 {
        if asked > 0.2 * dt && moved < 0.25 * asked {
            let s = row!(g);
            s.stuck += 1.0;
            if s.stuck >= 3.0 { let pl = g.player; plant(g, pl); }
        } else { row!(g).stuck = 0.0; }
    }

    if row!(g).st == 1.0 {
        { let s = row!(g); s.age += dt; s.left -= dt; }
        let k = 1.0 - m::exp(-t.turn * dt);
        if let Some(man) = row!(g).man {
            let SpotOf { x, z } = spot_of(g, man);
            let p = g.ch[c].pos;
            let (ax, az) = (x - p[0], z - p[2]);
            let d = m::hyp2(ax, az);
            // (the keys taken away from it, or the chance gone: a plain step from here)
            if !crate::stalk::stalk_could(g, man) || (ml > 0.0 && ax * mx + az * mz < 0.0) { row!(g).man = None; }
            else {
                if d > 0.02 { bend(g, ax / d, az / d, k); }
                let s = row!(g);
                s.len = m::min(s.done + d, t.land_m + 0.25);
            }
        }
        if row!(g).man.is_none() {
            if ml > 0.0 { bend(g, mx, mz, k); }
            let dir = row!(g).dir;
            let ahead = back_of(g, dir[0], dir[1], 99.0);
            let gv = ahead.map(|a| g.ch[g.gd[a.gd].ch].vel);
            let gain = match gv { Some(v) => t.follow * m::max(0.0, v[0] * dir[0] + v[2] * dir[1]), None => 0.0 };
            let reach = t.reach_m + gain * (t.secs + t.settle);
            if ml > 0.0 {
                if row!(g).set {
                    let secs = step_secs(g, reach);
                    let s = row!(g);
                    s.left = m::max(s.left, secs - s.age);
                }
                let s = row!(g);
                s.len = m::max(s.len, reach);
                s.set = false;
            } else if !row!(g).set {
                // (the key let go fixes where it lands: the soonest he can stop softly, a tap's at the least)
                let least = row!(g).least;
                let secs = step_secs(g, least);
                key_let_go(row!(g), secs);
            }
        }
        // (the least-jerk way from where the step is to rest at its end, walked exactly and on its own numbers)
        let s = row!(g);
        walk(s, dt);
        if s.left <= dt / 2.0 { let pl = g.player; plant(g, pl); }
    }

    let s = row!(g);
    legs_follow(s, t.legs, dt);
    let (u, n, way) = (s.u, s.n, s.way);
    let an = &mut g.ch[c].anim;
    an.tap_u = u; an.tap_n = n; an.tap_way = way;
    let s = row!(g);
    if s.st != 1.0 { s.go = 0.0; }
    s.asked = s.go * dt;
    TapMove { mx: s.dir[0], mz: s.dir[1], speed: s.go }
}

/// His feet are not the step's this frame (he stood, ran, took a wall): the next press starts clean. (tapstep.ts tapOff)
pub fn tap_off(g: &mut crate::game::Game, ml: f64) {
    if let Some(s) = g.m.tapstep.state.get_mut(g.player) {
        s.st = 0.0; s.t = 9.0; s.v = 0.0; s.man = None; s.keys = ml > 0.0; s.asked = 0.0; s.prev = None; s.at = -1.0;
    }
    let c = g.pl[g.player].ch;
    g.ch[c].anim.tap_u = -1.0;
}


#[cfg(test)]
mod tests {
    use super::*;

    /// A step in the air, key down for 12 ticks then up, against the TypeScript's own bits (bun, EXACT.on, tapTick with no men about, moved as it asked): [speed, tapU] a tick.
    const TS: [u64; 44] = [
        0x3f721e8c52b39910, 0x3ef8c086e084c189,
        0x3f9e771075baa429, 0x3f2a5a4854417b85,
        0x3fb0118b23fe82a8, 0x3f37db4950bf20a0,
        0x3fbb9323b07b9192, 0x3f53df2c15047125,
        0x3fc73bb072d3c1df, 0x3f676082c45907d5,
        0x3fcf758bbc1693f3, 0x3f6fd9fa05d95dfe,
        0x3fd44b1f1dd3f9b1, 0x3f7dfb787fd5d264,
        0x3fdadec5f119c80d, 0x3f892522e28e3969,
        0x3fe00b6a3e7eebd5, 0x3f8ed971bbf35fce,
        0x3fe2be827fb54b3b, 0x3f976c0dff7c6881,
        0x3fe66b69d3867aeb, 0x3fa0caae68b8f92d,
        0x3fe932baa153d8e5, 0x3fa37d62fd0f1025,
        0x3febe313d1aa220d, 0x3fb4748575a68134,
        0x3fef1320b139191b, 0x3fc0020cf0ebaaee,
        0x3ff08b3610b2f4a1, 0x3fc2dd258f84558b,
        0x3ff151da010b05b8, 0x3fc96214338e9a9a,
        0x3ff20c0177a01f83, 0x3fd02ac4266aa6df,
        0x3ff258b8c45eedcc, 0x3fd1dd256e9982b4,
        0x3ff260453f462836, 0x3fd5a0c56a5b5adc,
        0x3ff215a8a80c0b72, 0x3fd9814179637449,
        0x3ff19e18ec9486a2, 0x3fdb5c3f1218eb54,
        0x3ff0e4f07294edd4, 0x3fdf515f86e32dfb,
    ];

    /// The tick's own steps for a step in the air, on the row alone: everything else the tick does goes through other parts' stubs.
    #[test]
    fn the_step_in_the_air_is_the_typescripts_to_the_bit() {
        let t = crate::knobs::page_tapstep();
        let secs = secs_of(&t, t.reach_m);
        let mut s = St { st: 1.0, keys: true, dir: [0.6, 0.8], len: t.reach_m, least: t.tap_m, u: 0.0, n: 1.0, secs, left: secs, ..fresh_row() };
        for i in 0..22 {
            let ml = if i < 12 { 1.0 } else { 0.0 };
            let dt = if i % 3 == 2 { 1.0 / 120.0 } else { 1.0 / 60.0 };
            s.age += dt;
            s.left -= dt;
            if ml > 0.0 { s.len = m::max(s.len, t.reach_m); s.set = false; }
            else if !s.set { let least_secs = secs_of(&t, s.least); key_let_go(&mut s, least_secs); }
            walk(&mut s, dt);
            legs_follow(&mut s, t.legs, dt);
            assert_eq!(s.go.to_bits(), TS[i * 2], "speed at tick {i}");
            assert_eq!(s.u.to_bits(), TS[i * 2 + 1], "tapU at tick {i}");
        }
    }
}
