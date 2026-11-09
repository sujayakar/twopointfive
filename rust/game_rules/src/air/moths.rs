#![allow(unused_imports, unused_variables)]
//! MOTHS ROUND THE LIT SODIUM LAMPS AT NIGHT (src/game/moths.ts), whole. "Nobody's AI reads them", and no ray meets them; but each is a KEYED box of
//! the frame's list (MKEY.test + 0x4000 + i), and the fog of war keeps a record of every keyed loose thing of that list and caps its records, the
//! frame's room is budgeted with them in it, and the men's kit starts behind them in the list (`rig_from`). So the tick reads what this lane writes,
//! and the lane runs here, under its DRESS row, as it runs in the TypeScript.

pub use ::game_types::moths::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, vec, vec::Vec};

use crate::v3::quat;

/// integer hash → [0, 1): the module's dice, every choice off its own serial (src/game/moths.ts:28 h01)
fn h01(x: f64) -> f64 {
    let mut x = m::imul(js::to_i32(x) ^ 0x2545F491, 0x9E3779B1u32 as i32) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, 0x85EBCA77u32 as i32) as u32;
    x ^= x >> 13;
    (x >> 8) as f64 / 16777216.0
}
/// (src/game/moths.ts:29 smooth)
fn smooth(a: f64, b: f64, x: f64) -> f64 { let t = m::max(0.0, m::min(1.0, (x - a) / (b - a))); t * t * (3.0 - 2.0 * t) }

/// The sodium lamps (exterior spot lights hung high, found once off the light targets) and the moth pool dealt round them. (src/game/moths.ts:37 Moths.constructor)
pub fn moths_new(g: &mut crate::game::Game) -> crate::moths::Moths {
    let k = moths(); let mut out = Moths::blank();
    for t in g.targets.clone() {
        let (kind, group, light) = { let f = &g.tab.sim_lightshome_fixture[t]; (f.kind, f.group.clone(), f.light) };
        let Some(l) = light else { continue; };
        let (lk, lp) = { let x = &g.tab.light[l]; (x.kind, x.pos) };
        if kind == crate::sim_lightshome::FixtureKind::Analytic && lk == 1.0 && group == "exterior" && lp[1] > 3.5 { out.lamps.push(MothsLampsItem { target: t, light: l, head: [lp[0], lp[1] + 0.08, lp[2]], act: 0.0 }); }
    }
    let fl = crate::scene_boxes::box_flag();
    let mut i = 0.0;
    while i < k.max && !out.lamps.is_empty() {
        let hs = |n: f64| h01(i * 7919.0 + n * 131.0);
        // body along local +X (the heading), the span across local Z: rolling about the body flickers the span seen from above
        let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c: [0.0, -50.0, 0.0], h: [0.006, 0.0025, k.wing], albedo: Some(k.albedo),
            flags: Some((js::to_i32(fl.dynamic) | js::to_i32(fl.no_shadow)) as f64), rough: Some(0.8), mkey: Some(crate::scene_boxes::mkey().test + 16384.0 + i), ..Default::default() });
        let az = hs(9.0) * m::PI * 2.0;
        out.moths.push(Moth { lamp: js::rem(i, out.lamps.len() as f64), r: k.r_min + (k.r_max - k.r_min) * hs(1.0),
            omega: (k.omega_min + (k.omega_max - k.omega_min) * hs(2.0)) * (if hs(3.0) < 0.5 { -1.0 } else { 1.0 }), ph: hs(4.0) * 6.283, bob_f: 0.7 + 1.6 * hs(5.0), bob_ph: hs(6.0) * 6.283,
            wob: 0.3 + 0.7 * hs(7.0), dart_t: 4.0 + 6.0 * hs(8.0), dart_ph: hs(10.0) * 10.0, flap_ph: hs(11.0) * 6.283, away: [m::cos(az), -0.25 - 0.5 * hs(12.0), m::sin(az)], shown: 0.0, box_ });
        i += 1.0;
    }
    out
}

/// The deal back as constructed (an encounter restart: a restarted encounter repeats). (src/game/moths.ts:48 Moths.clear)
pub fn moths_clear(g: &mut crate::game::Game) {
    let n = g.moths.lamps.len() as f64;
    for (i, mo) in g.moths.moths.iter_mut().enumerate() { mo.shown = 0.0; mo.lamp = if n > 0.0 { js::rem(i as f64, n) } else { 0.0 }; }
    for l in g.moths.lamps.iter_mut() { l.act = 0.0; }
}

/// Once a frame (game.ts, after the wind): move the moths. (src/game/moths.ts:51 Moths.update)
pub fn moths_update(g: &mut crate::game::Game, dt: f64) {
    if g.moths.lamps.is_empty() { return; }
    let k = moths();
    let on = g.tune.moths && !crate::lighting::is_daylight(g);
    let rain = if g.weather.on { if g.weather.snow_rate > 0.0 { 1.0 } else { crate::weather::weather_rain_rate(g) } } else { 0.0 };   // (no moths at a lamp in falling snow)
    // each lamp's pull on its moths: lit × night × calm enough × not a downpour, eased so they gather and scatter over seconds
    for li in 0..g.moths.lamps.len() {
        let (t, l, head, act) = { let x = &g.moths.lamps[li]; (x.target, x.light, x.head, x.act) };
        let lit = on && crate::sim_lightshome::fixture_factor(g, t) > 0.05 && !crate::sim_lightshome::fixture_broken(g, t) && g.tab.light[l].enabled;
        let mut a = if lit { 1.0 } else { 0.0 };
        if a > 0.0 {
            let wv = crate::wind::wind_at(g, head, None);
            a *= 1.0 - smooth(k.gale_ms[0], k.gale_ms[1], m::hyp2(wv[0], wv[2]));
            a *= 1.0 - smooth(0.75, k.rain_max, rain);
        }
        let rate = if a > act { 1.0 / k.gather_secs } else { 1.0 / k.scatter_secs };
        g.moths.lamps[li].act = act + m::max(-rate * dt, m::min(rate * dt, a - act));
    }
    let t = g.time; let mut shown = 0;
    // who flies where: each lamp's QUOTA is its share of the dozen by pull; a moth over its lamp's quota scatters, and one away out of sight whose lamp
    // has no place for it goes over to the lamp with the most places unfilled: every move happens while the moth is invisible
    let n_l = g.moths.lamps.len();
    let mut sum_act = 0.0; for x in g.moths.lamps.iter() { sum_act += x.act; }
    let quota: Vec<f64> = g.moths.lamps.iter().map(|x| if sum_act > 1e-6 { m::min(k.per_lamp, x.act * k.max / m::max(1.0, sum_act)) } else { 0.0 }).collect();
    let mut held: Vec<f64> = vec![0.0; n_l]; for mo in g.moths.moths.iter() { held[mo.lamp as usize] += 1.0; }
    // (the TypeScript's `rankOf` is a Map by the moth: here by its place in the pool)
    let mut rank_of: Vec<f64> = Vec::with_capacity(g.moths.moths.len());
    { let mut seen: Vec<f64> = vec![0.0; n_l]; for mo in g.moths.moths.iter() { rank_of.push(seen[mo.lamp as usize]); seen[mo.lamp as usize] += 1.0; } }
    for i in 0..g.moths.moths.len() {
        let mo = g.moths.moths[i];
        if mo.shown > 0.01 { continue; }
        if quota[mo.lamp as usize] - rank_of[i] > 0.0 { continue; }   // his own lamp still wants him
        let (mut best, mut room) = (-1i64, 0.5);
        for kk in 0..n_l { let r = quota[kk] - held[kk]; if r > room { room = r; best = kk as i64; } }
        if best >= 0 && best as f64 != mo.lamp { held[mo.lamp as usize] -= 1.0; rank_of[i] = held[best as usize]; held[best as usize] += 1.0; g.moths.moths[i].lamp = best as f64; }
    }
    for i in 0..g.moths.moths.len() {
        let mut mo = g.moths.moths[i];
        let head = g.moths.lamps[mo.lamp as usize].head;
        let rank = rank_of[i];   // this moth's place in its lamp's queue: the lamp holds its quota of them
        let want = quota[mo.lamp as usize] - rank; let target = m::max(0.0, m::min(1.0, want));
        let rate = if target > mo.shown { 1.0 / k.gather_secs } else { 1.0 / k.scatter_secs };
        mo.shown += m::max(-rate * dt, m::min(rate * dt, target - mo.shown));
        g.moths.moths[i].shown = mo.shown;
        if mo.shown <= 0.01 { g.tab.boxes[mo.box_].c = [0.0, -50.0, 0.0]; continue; }
        // the orbit: round the head at its own rate, radius breathing, bobbing under the lens, and every dartT a half-second dart in at the lens and out again
        let th = mo.ph + mo.omega * t + 0.6 * m::sin(t * 0.7 * mo.wob + mo.bob_ph);
        let dart_x = js::rem(t + mo.dart_ph, mo.dart_t) / 0.55; let dart = if dart_x < 1.0 { m::sin(dart_x * m::PI) } else { 0.0 };
        let r = mo.r * (0.75 + 0.35 * m::sin(t * 1.3 * mo.wob + mo.ph)) * (1.0 - 0.8 * dart);
        let y = head[1] - k.below - k.bob * (0.5 + 0.5 * m::sin(t * mo.bob_f + mo.bob_ph)) + 0.18 * dart;
        let orbit: V3 = [head[0] + m::cos(th) * r, y, head[2] + m::sin(th) * r];
        let kk = 1.0 - mo.shown; let off = kk * kk * 3.5;   // scattering: out along its own bearing and down, three metres and more; gathering: back in along it
        let c: V3 = [orbit[0] + mo.away[0] * off, orbit[1] + mo.away[1] * off, orbit[2] + mo.away[2] * off];
        let heading = -(th + m::sign(mo.omega) * m::PI / 2.0);   // tangent to the orbit (Box yaw convention: local +X → (cos yaw, −sin yaw))
        let roll = 0.9 * m::sin((t * k.flap_hz + mo.flap_ph) * 6.283);   // the flutter: the wings' plane rocking fast
        let bx = &mut g.tab.boxes[mo.box_];
        bx.c = c; bx.yaw = 0.0; bx.rot = Some(quat::mul(quat::yaw(heading), quat::axis_angle([1.0, 0.0, 0.0], roll)));
        shown += 1;
    }
    g.stats.moths.shown = shown;
}

/// The moths' boxes into the frame's dynamic list (only the ones about). (src/game/moths.ts:100 Moths.boxes)
pub fn moths_boxes(g: &crate::game::Game, out: &mut Vec<AnyBoxId>) {
    for mo in g.moths.moths.iter() { if mo.shown > 0.01 { out.push(mo.box_.into()); } }
}
