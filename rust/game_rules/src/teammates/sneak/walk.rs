//! src/game/sneak.ts lines 1 to 414
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A number as a Map's key: JavaScript's Map takes −0 and 0 for one key, `Num` compares bits.
fn num_key(x: f64) -> crate::Num { crate::Num(x + 0.0) }

/// (sneak.ts clamp)
pub fn clamp(x: f64, a: f64, b: f64) -> f64 { m::max(a, m::min(b, x)) }

/// The panel's rows kept in the browser's storage: host work, nothing of the game moves. pairup.ts:85 calls it, so it answers rather than panics. (sneak.ts saveSneak)
pub fn save_sneak(g: &crate::game::Game) {}

/// Was this leaf seen to move inside the settle time? Read at the plan's rate, for the doors Sam is at. (sneak.ts leafMoved)
pub fn leaf_moved(g: &mut crate::game::Game, d: DoorId) -> bool {
    let angle = crate::doors::door_angle(g, d);
    let now = g.time;
    match g.m.sneak.leaf_t.get_mut(d) {
        None => { g.m.sneak.leaf_t.set(d, SneakModLeafTValue { a: angle, t: -1e9 }); }
        Some(r) => {
            if m::abs(r.a - angle) > 1e-4 { r.a = angle; r.t = now; } else if now < r.t { r.t = -1e9; }
        }
    }
    let t = g.m.sneak.leaf_t.get(d).unwrap().t;
    now - t < g.k.sneak.door_settle
}

/// (sneak.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// What he did, newest last. (sneak.ts note)
pub fn note(g: &crate::game::Game, gd: GuardId, what: &str) {
}

/// The sneak is live for the squad this frame; one read a frame. (sneak.ts sneakLive)
pub fn sneak_live(g: &mut crate::game::Game) -> bool {
    if !g.k.sneak.on { return false; }
    let was = g.m.sneak.live_at;
    if let Some(l) = was { if l.t == g.time { return l.v; } }
    // L.t is written before the questions below: one that asks again this frame gets the old v, as in the TypeScript.
    g.m.sneak.live_at = Some(SneakModLiveAtValue { t: g.time, v: match was { Some(l) => l.v, None => false } });
    let v = sneak_live_now(g);
    if let Some(l) = g.m.sneak.live_at.as_mut() { l.v = v; }
    v
}
fn sneak_live_now(g: &mut Game) -> bool {
    if !crate::silent::squad_silent(g, &crate::seats::here(g)) { return false; }
    if crate::silent::silent_compromised(g, &crate::seats::here(g)) { return false; }
    let pl = g.player;
    if g.pl[pl].down { return false; }
    if crate::pairup::pair_up(g, crate::pairup::PairUpRule::Hunted) && g.k.spotted.on {
        let squad = crate::orders::squad_orders_squad(g, &crate::seats::here(g));
        for gd in squad {
            if g.gd[gd].state != crate::game::GuardState::Dead && crate::spotted::spotted_mode(g, gd) == crate::spotted::SpottedMode::Caught { return false; }
        }
        return true;
    }
    for &h in g.guards.iter() {
        let r = &g.gd[h];
        if (r.state == crate::game::GuardState::Alert || r.state == crate::game::GuardState::Search)
            && r.team != crate::teams::Team::Operative
            && r.team != crate::teams::Team::Civilian { return false; }
    }
    true
}

/// (sneak.ts mate)
pub fn mate(g: &crate::game::Game, gd: GuardId) -> bool { g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].unit.is_some() }

/// This teammate is under the sneak this frame. (sneak.ts sneaking)
pub fn sneaking(g: &mut crate::game::Game, gd: GuardId) -> bool { g.k.sneak.on && mate(g, gd) && sneak_live(g) }

/// He is low because Sam is. (sneak.ts sneakLow)
pub fn sneak_low(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let pl = g.player;
    g.k.sneak.on && g.pl[pl].crouch && sneaking(g, gd)
}

/// A walk to a mark an order or the level's flow gave him: the pace he keeps to, low. (sneak.ts sneakPost)
pub fn sneak_post(g: &mut crate::game::Game, gd: GuardId, order: &str) -> Option<f64> {
    if sneak_low(g, gd) && crate::orders::rows::id_is(order, crate::orders::Is::WalksLow) { Some(g.k.sneak.low_pace) } else { None }
}

/// A crouched, sneaking teammate's footfall; None: silent's own. (sneak.ts sneakStep)
pub fn sneak_step(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> {
    let c = g.gd[gd].ch;
    if g.k.sneak.on && g.k.sneak.step > 0.0 && g.ch[c].anim.crouch > 0.5 && sneaking(g, gd) { Some(g.k.sneak.step) } else { None }
}

/// Is this man walking over where the teammate stands: the man sent to the breaker, or a suspicious man on his way to a sound. (sneak.ts sneakComing)
pub fn sneak_coming(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if crate::workthings::work_coming(g, gd, h) { return true; }
    if !g.k.sneak.on { return false; }
    let (end, goal) = {
        let r = &g.gd[h];
        if r.state != crate::game::GuardState::Suspicious || r.task.is_some() || r.body_duty.is_some() || r.hold || r.held.is_some() { return false; }
        match (r.last_known, r.path_goal) { (Some(l), Some(pg)) => (l, pg), _ => return false }
    };
    if flat(goal, end) > 1.0 { return false; }
    if g.gd[h].path_i >= g.gd[h].path.len() { return false; }
    let sight_t = g.gd[h].sight_t;
    if g.time >= sight_t && g.time - sight_t < g.k.spotted.fresh_secs { return false; }
    let gc = g.gd[gd].ch;
    let hc = g.gd[h].ch;
    let p = g.ch[gc].pos;
    let n = g.gd[h].path.len();
    let mut a = g.ch[hc].pos;
    let mut i = if g.gd[h].path_i < n { g.gd[h].path_i } else { n };
    while i <= n {
        let b = if i < n { g.gd[h].path[i] } else { end };
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let l2 = dx * dx + dz * dz;
        let k = if l2 > 1e-6 { m::max(0.0, m::min(1.0, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) } else { 0.0 };
        let qx = a[0] + dx * k;
        let qz = a[2] + dz * k;
        if m::hyp2(p[0] - qx, p[2] - qz) < g.k.sneak.noise_walk_m && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [qx, 0.45, qz], None) {
            g.stats.sneak.noise_walks += 1;
            return true;
        }
        a = b;
        i += 1;
    }
    false
}

/// What he is doing, for a HUD's line; None: he is walking with Sam. (sneak.ts sneakWord)
pub fn sneak_word(g: &crate::game::Game, gd: GuardId) -> Option<String> {
    if !g.k.sneak.on { return None; }
    let (mode, why) = match g.m.sneak.states.as_ref().and_then(|s| s.men.get(gd)) { Some(mm) => (mm.mode, mm.why), None => return None };
    // (`sneaking`, as the tick's `sneak_tick` last read the sneak live: a HUD's line asks and writes nothing)
    if !(mate(g, gd) && g.m.sneak.live_at.is_some_and(|l| l.v)) { return None; }
    let word = match mode {
        Mode::Wait => match why {
            Why::Door => "clear of the door",
            Why::Lit | Why::Near => "waiting on eyes",
            _ => "waiting on the light",
        },
        Mode::Cross => "crossing",
        Mode::Back => "backing off",
        _ => return None,
    };
    Some(String::from(word))
}

/// The defenders whose eyes matter to this man: alive, his enemies, inside an eye's reach of him, not dazzled, not asleep. (sneak.ts eyesFor)
pub fn eyes_for(g: &crate::game::Game, gd: GuardId) -> Vec<GuardId> {
    let mut out = Vec::new();
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let r = g.k.guard_eye.range + g.k.sneak.ahead_m + g.k.sneak.stand_m;
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if h == gd || g.gd[h].state == crate::game::GuardState::Dead { continue; }
        if !crate::teams::hostile_to(g, h, crate::teams::HostileToO::Guard(gd)) { continue; }
        let hc = g.gd[h].ch;
        if flat(g.ch[hc].pos, p) < r && !crate::control::man::blind(g, h) && !crate::doze::doze_asleep(g, h) { out.push(h); }
    }
    out
}

/// How lit two points are, the brighter, on the meter's own curve. (sneak.ts cpuLit)
pub fn cpu_lit(g: &mut crate::game::Game, a: V3, b: V3, own: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady: Option<bool>) -> f64 {
    let steady = steady.unwrap_or(true);
    let mut own = own;
    let at_a = match own.as_mut() {
        Some(f) => crate::guards::light_estimate_at(g, a, Some(&mut **f), Some(steady), None),
        None => crate::guards::light_estimate_at(g, a, None, Some(steady), None),
    };
    let at_b = match own.as_mut() {
        Some(f) => crate::guards::light_estimate_at(g, b, Some(&mut **f), Some(steady), None),
        None => crate::guards::light_estimate_at(g, b, None, Some(steady), None),
    };
    crate::game::game_visibility_of(g, m::max(at_a, at_b))
}

/// How lit a step is to a man low or upright on it; read at most twice a second per crumb. (sneak.ts litAt)
pub fn lit_at(g: &mut crate::game::Game, c_2: &mut crate::sneak::Crumb, low: bool) -> f64 {
    if c_2.low == low && g.time - c_2.t < 0.4 && g.time >= c_2.t { return c_2.lit; }
    let p = c_2.p;
    let y = if low { low_y() } else { up_y() };
    let cpu = cpu_lit(g, [p[0], y[0], p[2]], [p[0], y[1], p[2]], None, None);
    c_2.t = g.time;
    c_2.low = low;
    c_2.lit = cpu;
    c_2.lit
}

/// A defender's eye exactly as guards.ts canSeeChar builds it, its cones `wide` degrees wider. (sneak.ts eyeFor)
pub fn eye_for(g: &crate::game::Game, h: GuardId, wide: f64) -> crate::guards::Eye {
    let c = g.gd[h].ch;
    let pos = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(&b) => b, None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]) };
    let look = crate::guards::look_of(g, c);
    let k = g.k.guard_eye;
    crate::guards::Eye {
        pos,
        look,
        range: k.range,
        half_deg: m::min(180.0, k.half_deg + wide),
        near_m: k.near_m,
        near_deg: m::min(180.0, k.near_deg + wide),
        v_half_deg: None,
    }
}

/// A man stood on a spot, as the eye test takes its target: a chest and a head. (sneak.ts standIn)
pub fn stand_in(p: V3, low: bool) -> crate::sneak::Target {
    let y = if low { low_y() } else { up_y() };
    Target::Spot { chest: [p[0], y[0], p[2]], head: [p[0], y[1], p[2]] }
}

/// guards.ts eyeContact asked of what the sneak's eye test takes: a man's own body, or a spot (standIn: alive, never the player, so only `blind` shuts the eye). (sneak.ts:266, guards.ts:1330)
fn ask(g: &mut Game, e: crate::guards::Eye, pc: &Target, blind: bool) -> crate::guards::EyeContact {
    match *pc {
        Target::Man { c } => crate::guards::eye_contact(g, e, c, false, blind, 1.0, None),
        Target::Spot { chest, head } => {
            let lens = crate::scene_collision::EyeContactE { pos: e.pos, look: e.look, range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg, v_half_deg: e.v_half_deg };
            let r = crate::scene_collision::eye_contact(g, lens, chest, head, blind, false, crate::scene_boxes::box_flag().glass);
            crate::guards::EyeContact { visible: r.visible, dist: r.dist, in_cone: r.in_cone }
        }
    }
}

/// A target's exposure by the guards' own eye test: a line from each defender, how lit the target is (asked once, only if somebody has a line), and whether he is looking. (sneak.ts read)
pub fn read(g: &mut crate::game::Game, eyes: &[GuardId], pc: &crate::sneak::Target, lit: &mut dyn FnMut(&mut crate::game::Game) -> f64, low: Option<bool>, card: Option<f64>) -> crate::sneak::Exp {
    let low = low.unwrap_or(false);
    let card = card.unwrap_or(1.0);
    let (mut seen, mut watched, mut near) = (false, false, false);
    let mut by = None;
    let mut l = -1.0;
    let wide = g.k.sneak.cone_deg * g.k.sneak.skill;
    let keen = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    for &h in eyes {
        let e = eye_for(g, h, wide);
        let r = ask(g, crate::guards::Eye { half_deg: 180.0, near_deg: 180.0, ..e }, pc, false);
        if !r.visible { continue; }
        if l < 0.0 { l = lit(g); }
        if !seen_along(g, h, r.dist, keen, low, Some(card), None, &mut |_| l) { continue; }
        seen = true;
        if l < g.k.sneak.lit_at { near = true; }
        if ask(g, e, pc, true).in_cone { watched = true; by = Some(h); break; }
    }
    Exp { seen, watched, near, by }
}

/// … of a step of the trail, for a man low or upright on it. (sneak.ts exposure)
pub fn exposure(g: &mut crate::game::Game, eyes: &[GuardId], c_2: &mut crate::sneak::Crumb, low: bool, card: Option<f64>) -> crate::sneak::Exp {
    let card = card.unwrap_or(1.0);
    let pc = stand_in(c_2.p, low);
    read(g, eyes, &pc, &mut |g: &mut Game| lit_at(g, c_2, low), Some(low), Some(card))
}

/// … of the man where he stands: his own bones, his own meter. (sneak.ts exposureOf)
pub fn exposure_of(g: &mut crate::game::Game, eyes: &[GuardId], gd: GuardId) -> crate::sneak::Exp {
    let c = g.gd[gd].ch;
    let low = g.ch[c].anim.crouch > 0.5;
    let card = crate::kitload::load_seen(g, &crate::seats::here(g), crate::kitload::LoadSeenWho::Guard(gd));
    read(g, eyes, &Target::Man { c }, &mut |g: &mut Game| g.gd[gd].visibility, Some(low), Some(card))
}

/// The eye's rate on a man on a step: guards.ts sightGainOn's arithmetic. (sneak.ts eyeRate)
pub fn eye_rate(g: &crate::game::Game, h: GuardId, lit: f64, dist: f64, keen: f64, low: Option<bool>) -> f64 {
    let low = low.unwrap_or(true);
    let mut rate = g.tune.detect_rate * m::pow(lit, 1.5) * clamp(1.0 - (dist - 3.0) / 13.0, 0.2, 1.0) * (if low { 0.75 } else { 1.0 }) * keen;
    if dist < 1.8 { rate = m::max(rate, 3.5); }
    let st = g.gd[h].state;
    if st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search { rate * 2.2 } else { rate }
}

/// Does this eye gain on a man on a spot this lit, this far off. (sneak.ts eyeGains)
pub fn eye_gains(g: &crate::game::Game, h: GuardId, lit: f64, dist: f64, keen: f64, low: bool, card: Option<f64>, pad: Option<f64>) -> bool {
    let card = card.unwrap_or(1.0);
    let pad = pad.unwrap_or(0.0);
    eye_rate(g, h, m::min(1.0, lit + pad), dist, keen, Some(low)) * card > g.k.sneak.seen_rate
}

/// Is a man on a spot this lit, this far from this eye, seen as the sneak reads a step. (sneak.ts litSeen)
pub fn lit_seen(g: &mut crate::game::Game, h: GuardId, lit: f64, dist: f64, keen: f64, low: bool, card: Option<f64>, pad: Option<f64>) -> bool {
    if lit < g.k.sneak.lit_at { return false; }
    eye_gains(g, h, lit, dist, keen, low, card, pad)
}

/// With a clear line from this eye to a man on a step, is he seen: at the man's elbow, or lit enough for the eye. The one answer the partner's wait and his walk both take. `lit` is read only past the elbow.
pub fn seen_along(g: &mut crate::game::Game, h: GuardId, dist: f64, keen: f64, low: bool, card: Option<f64>, pad: Option<f64>, lit: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> bool {
    if dist < g.k.sneak.near_m { return true; }
    let l = lit(g);
    lit_seen(g, h, l, dist, keen, low, card, pad)
}

/// What crossing the steps t[first..end) low at the crossing pace would leave on the most stirred looker. (sneak.ts crossCost)
pub fn cross_cost(g: &mut crate::game::Game, eyes: &[GuardId], t: &mut [crate::sneak::Crumb], first: f64, end: f64, card: Option<f64>) -> f64 {
    let card = card.unwrap_or(1.0);
    let mut worst = 0.0;
    let wide = g.k.sneak.cone_deg * g.k.sneak.skill;
    let keen = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    for &h in eyes {
        let mut add = 0.0;
        let e = eye_for(g, h, wide);
        let mut k = first as usize;
        while (k as f64) < end {
            let pc = stand_in(t[k].p, true);
            let r = ask(g, e, &pc, false);
            if r.visible {
                let lit = lit_at(g, &mut t[k], true);
                let next = if t.len() - 1 < k + 1 { t.len() - 1 } else { k + 1 };
                add += eye_rate(g, h, lit, r.dist, keen, None) * m::max(0.4, flat(t[k].p, t[next].p)) / g.k.sneak.cross_pace;
            }
            k += 1;
        }
        if add > 0.0 { worst = m::max(worst, g.gd[h].awareness + g.k.sneak.margin * add * card); }
    }
    worst
}

/// How lit a nav cell is to a man low or upright on it, kept per cell for `age` seconds. (sneak.ts litCell)
pub fn lit_cell(g: &mut crate::game::Game, x: f64, z: f64, low: bool, age: Option<f64>) -> f64 {
    let age = age.unwrap_or(2.0);
    let cells = g.m.sneak.cell_lit.get_or_insert_with(JsMap::new);
    if cells.len() > 8000 { cells.clear(); }
    let c = g.col.nav.cell;
    let ix = m::floor(x / c);
    let iz = m::floor(z / c);
    let key = num_key((ix + 4096.0 * iz) * 2.0 + if low { 1.0 } else { 0.0 });
    let t = g.time;
    if let Some(was) = g.m.sneak.cell_lit.as_ref().and_then(|cs| cs.get(key)) {
        if t >= was[1] && t - was[1] < age { return was[0]; }
    }
    let px = (ix + 0.5) * c;
    let pz = (iz + 0.5) * c;
    let y = if low { low_y() } else { up_y() };
    let v = cpu_lit(g, [px, y[0], pz], [px, y[1], pz], None, None);
    g.m.sneak.cell_lit.get_or_insert_with(JsMap::new).set(key, [v, t]);
    v
}

/// The sneak's eye over any walk by `gd`: every living defender hostile to him and not dazzled, each as guards.ts builds his eye and, where `habit` gives it, as he mostly looks. (sneak.ts sneakWalk)
pub fn sneak_walk(g: &mut crate::game::Game, gd: GuardId, low: bool, habit: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> Option<[f64; 2]>>) -> crate::sneak::SneakWalk {
    g.stats.pairup.eye_asks += 1;
    let mut habit = habit;
    let wide = g.k.sneak.cone_deg * g.k.sneak.skill;
    let keen = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    let mut men: Vec<SneakWalkMan> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if h == gd || g.gd[h].state == crate::game::GuardState::Dead || !crate::teams::hostile_to(g, h, crate::teams::HostileToO::Guard(gd)) || crate::control::man::blind(g, h) || crate::doze::doze_asleep(g, h) { continue; }
        let e = eye_for(g, h, wide);
        let hab = match habit.as_mut() { Some(f) => (*f)(g, h), None => None };
        let mut looks = vec![walk_look(e)];
        if let Some(hh) = hab { looks.push(walk_look(crate::guards::Eye { look: [hh[0], 0.0, hh[1]], ..e })); }
        kept_of(g, h, low, e.pos[0], e.pos[2]);
        men.push(SneakWalkMan { h, x: e.pos[0], z: e.pos[2], open: crate::guards::Eye { half_deg: 180.0, near_deg: 180.0, ..e }, looks, line: JsMap::new() });
    }
    let reach = g.k.guard_eye.range + 0.5;
    let card = crate::kitload::load_seen(g, &crate::seats::here(g), crate::kitload::LoadSeenWho::Guard(gd));
    SneakWalk { by: None, gd, low, keen, card, reach, men, memo: JsMap::new() }
}

/// (sneak.ts sneakWalk, look)
fn walk_look(e: crate::guards::Eye) -> SneakWalkLook {
    let l = js::or(m::hyp2(e.look[0], e.look[2]), 1.0);
    SneakWalkLook {
        e,
        ew: crate::guards::Eye { half_deg: m::min(180.0, e.half_deg + 15.0), near_deg: m::min(180.0, e.near_deg + 15.0), ..e },
        lx: e.look[0] / l,
        lz: e.look[2] / l,
    }
}
/// The lines a route is priced by are kept between asks, per defender, while his eye has not moved a hand's breadth: two seconds at most. (sneak.ts sneakWalk, keptOf)
fn kept_of(g: &mut Game, h: GuardId, low: bool, x: f64, z: f64) {
    let now = g.time;
    let stale = match g.m.sneak.kept_lines.get(h) {
        None => true,
        Some(k) => k.low != low || now - k.t > 2.0 || now < k.t || m::hyp2(k.x - x, k.z - z) > 0.12,
    };
    if stale { g.m.sneak.kept_lines.set(h, SneakModKeptLinesValue { x, z, t: now, low, map: JsMap::new() }); }
}
/// (sneak.ts sneakWalk, litOf)
fn walk_lit_of(g: &mut Game, w: &mut SneakWalk, x: f64, z: f64, age: Option<f64>) -> f64 { lit_cell(g, x, z, w.low, age) }
/// The cheap part first (reach, and the cone in plan with a little slack), then the eye test's own cone. (sneak.ts sneakWalk, inCone)
fn walk_in_cone(g: &mut Game, mm: &SneakWalkMan, reach: f64, k: usize, wider: bool, x: f64, z: f64, pc: &Target) -> bool {
    let l = mm.looks[k];
    let e = if wider { l.ew } else { l.e };
    let dx = x - mm.x;
    let dz = z - mm.z;
    let d = m::hyp2(dx, dz);
    if d > reach || (d > 0.3 && (dx * l.lx + dz * l.lz) / d <= m::cos(m::min(180.0, (if d < e.near_m + 0.5 { e.near_deg } else { e.half_deg }) + 4.0) * DEG)) { return false; }
    ask(g, e, pc, true).in_cone
}
/// (sneak.ts sneakWalk, cell)
fn walk_cell(x: f64, z: f64) -> f64 { m::floor(x / 0.25) + 4096.0 * m::floor(z / 0.25) }
/// The line from a defender to a man on this floor, asked once per defender per quarter-metre: its length, or −1. (sneak.ts sneakWalk, line)
fn walk_line(g: &mut Game, w: &mut SneakWalk, q: usize, x: f64, z: f64, pc: &Target, fresh: bool) -> f64 {
    let key = num_key(walk_cell(x, z));
    let h = w.men[q].h;
    if let Some(&v) = w.men[q].line.get(key) { return v; }
    if !fresh {
        if let Some(&v) = g.m.sneak.kept_lines.get(h).and_then(|k| k.map.get(key)) { return v; }
    }
    let open = w.men[q].open;
    let r = ask(g, open, pc, false);
    let v = if r.visible { r.dist } else { -1.0 };
    w.men[q].line.set(key, v);
    if let Some(k) = g.m.sneak.kept_lines.get_mut(h) { k.map.set(key, v); }
    v
}
/// A man on this floor would be looked at: lit or at an elbow, with a line, inside a defender's cone. (sneak.ts SneakWalk.watched)
pub fn sneak_walk_watched(g: &mut crate::game::Game, this: &mut crate::sneak::SneakWalk, x: f64, z: f64, wider: Option<bool>) -> bool {
    let w = this;
    let wider = wider.unwrap_or(false);
    let key = num_key(walk_cell(x, z) * 2.0 + if wider { 1.0 } else { 0.0 });
    if let Some(&was) = w.memo.get(key) { return was; }
    let mut lit = -1.0;
    let mut r = false;
    let pc = stand_in([x, 0.0, z], w.low);
    for q in 0..w.men.len() {
        let mut any = false;
        for k in 0..w.men[q].looks.len() {
            if walk_in_cone(g, &w.men[q], w.reach, k, wider, x, z, &pc) { any = true; break; }
        }
        if !any { continue; }
        let d = walk_line(g, w, q, x, z, &pc, false);
        if d < 0.0 { continue; }
        let h = w.men[q].h;
        let (keen, low, card) = (w.keen, w.low, w.card);
        if seen_along(g, h, d, keen, low, Some(card), Some(MAP_SHORT), &mut |g: &mut Game| { if lit < 0.0 { lit = walk_lit_of(g, w, x, z, None); } lit }) { r = true; break; }
    }
    w.memo.set(key, r);
    r
}
/// What walking `pts` at `pace` would leave on the most stirred watcher; 0: nobody would see a step of it. (sneak.ts SneakWalk.cost)
pub fn sneak_walk_cost(g: &mut crate::game::Game, this: &mut crate::sneak::SneakWalk, pts: &[V3], pace: f64, skip: Option<V3>, skip_m: Option<f64>) -> f64 {
    let w = this;
    let skip_m = skip_m.unwrap_or(0.3);
    let mut add: Vec<Vec<f64>> = w.men.iter().map(|mm| vec![0.0; mm.looks.len()]).collect();
    let mut j = 1;
    while j < pts.len() {
        let a = pts[j - 1];
        let b = pts[j];
        j += 1;
        let n = m::max(1.0, m::ceil(flat(a, b) / 0.25));
        let dt = flat(a, b) / n / m::max(0.3, pace);
        let mut at = 0.0;
        while at < n {
            at += 1.0;
            let x = a[0] + (b[0] - a[0]) * at / n;
            let z = a[2] + (b[2] - a[2]) * at / n;
            if let Some(s) = skip { if m::hyp2(x - s[0], z - s[2]) <= skip_m { continue; } }
            let mut lit = -1.0;
            let pc = stand_in([x, 0.0, z], w.low);
            for q in 0..w.men.len() {
                let mut rate = -1.0;
                for k in 0..w.men[q].looks.len() {
                    if !walk_in_cone(g, &w.men[q], w.reach, k, false, x, z, &pc) { continue; }
                    if rate < 0.0 {
                        let d = walk_line(g, w, q, x, z, &pc, true);
                        if d < 0.0 { break; }
                        if lit < 0.0 { lit = walk_lit_of(g, w, x, z, Some(0.4)); }
                        let h = w.men[q].h;
                        if !seen_along(g, h, d, w.keen, w.low, Some(w.card), Some(MAP_SHORT), &mut |_| lit) { break; }
                        rate = eye_rate(g, h, lit, d, w.keen, Some(w.low)) * crate::kitload::load_cross(g, &crate::seats::here(g), crate::kitload::LoadCrossWho::Guard(w.gd));
                    }
                    add[q][k] += rate * dt;
                }
            }
        }
    }
    let mut worst = 0.0;
    w.by = None;
    for q in 0..w.men.len() {
        let v = m::max_n(&add[q]);
        if v <= 0.0 { continue; }
        let h = w.men[q].h;
        let c = g.gd[h].awareness + g.k.sneak.margin * v;
        if c > worst { worst = c; w.by = Some(h); }
    }
    worst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_keeps_the_languages_nan() {
        assert_eq!(clamp(2.0, 0.2, 1.0), 1.0);
        assert_eq!(clamp(-3.0, 0.2, 1.0), 0.2);
        assert!(clamp(f64::NAN, 0.2, 1.0).is_nan());
    }

    #[test]
    fn a_map_key_is_one_for_both_zeroes() {
        assert!(num_key(-0.0) == num_key(0.0));
        assert_eq!(num_key(walk_cell(-0.0, -0.0)).0.to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn a_cell_packs_as_the_typescript_does() {
        assert_eq!(walk_cell(0.3, 0.6), 1.0 + 4096.0 * 2.0);
        assert_eq!(walk_cell(-0.1, -0.1), -1.0 + 4096.0 * -1.0);
    }
}
