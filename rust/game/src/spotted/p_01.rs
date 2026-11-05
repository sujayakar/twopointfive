//! src/game/spotted.ts lines 1 to 381
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ------------------------------------------------------------ small helpers
fn pos(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn sam_pos(g: &Game) -> V3 { g.ch[g.pl[g.player].ch].pos }
/// a man's row, made by spotted_tick before anything here asks for it; it borrows all of `g`: use it for a few lines and let it go
fn man(g: &mut Game, gd: GuardId) -> &mut crate::spotted::Man {
    g.m.spotted.men.get_mut(gd).expect("spotted: a man's row is made by spotted_tick or looked for first")
}
fn log(g: &mut Game, t: f64, gd: GuardId, to: crate::spotted::SpottedMode, what: Option<&str>, of: Option<GuardId>) {
    let who = g.gd[gd].callsign.clone();
    let of = of.map(|o| g.gd[o].callsign.clone());
    g.m.spotted.spotted_log.push(SpottedModSPOTTEDLOGItem { t, who, to, what: what.map(String::from), of });
}
/// any man of `wary` has a line to a body knelt at `q`
fn any_in_line(g: &mut Game, wary: &[GuardId], q: &[f64]) -> bool {
    for &h in wary { if in_line(g, h, q, true) { return true; } }
    false
}
/// nothing solid between him and `h` at waist height (hands: `open`)
fn open_to(g: &mut Game, p: V3, h: GuardId) -> bool {
    let c = pos(g, h);
    !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [c[0], 1.0, c[2]], None)
}
/// cover's `shot`: a line from `q` to the man `hp`, and none of ours standing in it
fn shot_from(g: &mut Game, gd: GuardId, q: &[f64], hp: V3) -> bool {
    if !(shot_line(g, q, &hp, 1.2) || shot_line(g, q, &hp, 1.6)) { return false; }
    let chest: V3 = [hp[0], 1.2, hp[2]];
    crate::roomfight::room_fight_friend_in_line(g, [q[0], 1.4, q[2]], chest, gd, None).is_none()
}
/// a new man's row (spotted.ts:330)
fn new_man(t: f64) -> crate::spotted::Man {
    crate::spotted::Man {
        mode: crate::spotted::SpottedMode::Calm, since: t, wary: Vec::new(), by: None, eyes: None, wary_t: -100.0, caught_t: -100.0,
        spot: None, what: None, ask_t: -1.0, exposed: false, trail: false, said_t: -100.0, real: false, told: false,
        take: None, take_no: None, real_t: -1.0, tick_t: t,
    }
}

/// flat distance (spotted.ts flat)
pub fn flat(a: &[f64], b: &[f64]) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// a man's eyes: the head bone, or 1.6 over his feet (spotted.ts eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    if let Some(h) = g.ch[c].bones.get(crate::character::Bone::Head) { return *h; }
    let p = g.ch[c].pos;
    [p[0], p[1] + 1.6, p[2]]
}

/// a line from a man's eyes to a body at `q`, asked of the walls and furniture only: no door leaves (spotted.ts inLine)
pub fn in_line(g: &mut crate::game::Game, h: GuardId, q: &[f64], low: bool) -> bool {
    let eye = eye_of(g, h);
    let dyn_boxes = core::mem::take(&mut g.col.dynamic_boxes);
    let seen = !crate::scene_collision::segment_blocked(g, eye, [q[0], if low { 0.75 } else { 1.25 }, q[2]], None, None, None)
        || !crate::scene_collision::segment_blocked(g, eye, [q[0], if low { 1.05 } else { 1.6 }, q[2]], None, None, None);
    g.col.dynamic_boxes = dyn_boxes;
    seen
}

/// the meter's reading for a man knelt at `q` (spotted.ts litAt)
pub fn lit_at(g: &mut crate::game::Game, q: &[f64]) -> f64 {
    let e = crate::guards::light_estimate_at(g, [q[0], 0.9, q[2]], None, None, None, None);
    crate::game::game_visibility_of(g, e)
}

/// his foe is Sam on his feet, or a live operative (spotted.ts ours)
pub fn ours(g: &mut crate::game::Game, h: GuardId) -> bool {
    match g.gd[h].foe {
        Some(crate::game::Foe::Player) => !g.pl[g.player].down,
        Some(crate::game::Foe::Guard(f)) => g.gd[f].team == crate::teams::Team::Operative && g.gd[f].state != crate::game::GuardState::Dead,
        None => false,
    }
}

/// an alert man with a real sighting of one of them inside `freshSecs` (spotted.ts hasThem)
pub fn has_them(g: &mut crate::game::Game, h: GuardId) -> bool {
    let since = g.time - g.gd[h].last_seen_t;
    g.gd[h].state == crate::game::GuardState::Alert && since >= 0.0 && since < g.k.spotted.fresh_secs && ours(g, h)
}

/// the switch on, under the silent posture, alive (spotted.ts live)
pub fn live(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.k.spotted.on && crate::silent::silent_man(g, gd) && g.gd[gd].state != crate::game::GuardState::Dead
}

/// his mode this frame (spotted.ts spottedMode)
pub fn spotted_mode(g: &mut crate::game::Game, gd: GuardId) -> crate::spotted::SpottedMode {
    if !live(g, gd) { return crate::spotted::SpottedMode::Calm; }
    match g.m.spotted.men.get(gd) { Some(r) => r.mode, None => crate::spotted::SpottedMode::Calm }
}

/// the alert man who has them, while this teammate is caught (spotted.ts spottedBy)
pub fn spotted_by(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let row = g.m.spotted.men.get(gd).map(|r| (r.mode, r.by));
    if !live(g, gd) { return None; }
    match row {
        Some((crate::spotted::SpottedMode::Caught, Some(by))) if g.gd[by].state != crate::game::GuardState::Dead => Some(by),
        _ => None,
    }
}

/// roomfight.ts's fire gate, beside silent.ts's own: this round is held (spotted.ts spottedHolds)
pub fn spotted_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if !g.k.spotted.on || !crate::silent::silent_man(g, gd) || crate::silent::reached(g, gd) || crate::silent::quiet_gun(g, gd) || crate::silent::urgent(g) { return false; }
    if g.m.spotted.men.get(gd).and_then(|r| r.take).map(|t| t.foe) == Some(h) { return true; }
    let st = g.gd[h].state;
    if st == crate::game::GuardState::Suspicious { g.stats.spotted.held += 1; return true; }
    if st != crate::game::GuardState::Alert && st != crate::game::GuardState::Search { return false; }
    if danger_now(g, h, gd) { return false; }
    g.stats.spotted.held_unaware += 1;
    true
}

/// an alert / searching man is a danger to one of them now (spotted.ts dangerNow)
pub fn danger_now(g: &mut crate::game::Game, h: GuardId, gd: GuardId) -> bool {
    let since = g.time - g.gd[h].last_seen_t;
    if since >= 0.0 && since < g.k.spotted.fresh_secs && ours(g, h) { return true; }
    let hp = pos(g, h);
    let cos = m::cos(g.k.spotted.face_deg * m::PI / 180.0);
    let yaw = g.ch[g.gd[h].ch].aim_yaw;
    let (fx, fz) = (m::sin(yaw), m::cos(yaw));
    let close_m = g.k.spotted.close_m;
    let ps: [Option<V3>; 2] = [Some(pos(g, gd)), if g.pl[g.player].down { None } else { Some(sam_pos(g)) }];
    for p in ps {
        let Some(p) = p else { continue };
        let (dx, dz) = (p[0] - hp[0], p[2] - hp[2]);
        let d = m::hyp2(dx, dz);
        if d < close_m { return true; }
        if (dx * fx + dz * fz) / js::or(d, 1.0) > cos {
            let eye = eye_of(g, h);
            if !crate::scene_collision::segment_blocked(g, eye, [p[0], 1.25, p[2]], None, None, None) { return true; }
        }
    }
    false
}

/// roomfight.ts's eyes: caught, he shoots on a sighting (spotted.ts spottedKeen)
pub fn spotted_keen(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.k.spotted.on || g.m.spotted.men.get(gd).map(|r| r.mode) != Some(crate::spotted::SpottedMode::Caught) || !crate::silent::silent_man(g, gd) { return false; }
    g.stats.spotted.keen += 1;
    true
}

/// orders.ts, as it poses him: where his feet go and how he carries himself; None: as ordered (spotted.ts spottedFeet)
pub fn spotted_feet(g: &mut crate::game::Game, gd: GuardId, follower: bool, aside: bool) -> Option<crate::spotted::SpottedFeet> {
    if !g.k.spotted.on { return None; }
    let (take, mode, by, eyes, spot, what, exposed) = match g.m.spotted.men.get(gd) {
        Some(r) => (r.take, r.mode, r.by, r.eyes, r.spot, r.what, r.exposed),
        None => return None,
    };
    if !crate::silent::silent_man(g, gd) { return None; }
    let pace = g.k.silent.pace;
    if let Some(tk) = take {
        if !aside {
            let c = pos(g, tk.foe);
            return Some(SpottedFeet { goal: Some(tk.spot), speed: Some(if pace > 0.0 { pace } else { 1.6 }), crouch: false, watch: Some([c[0], 1.2, c[2]]), goal_serial: Some(tk.spot_serial) });
        }
    }
    if mode == crate::spotted::SpottedMode::Calm { return None; }
    let on = if mode == crate::spotted::SpottedMode::Caught { by } else if follower { eyes } else { None };
    let watch: Option<V3> = match on {
        Some(o) if g.gd[o].state != crate::game::GuardState::Dead => { let c = pos(g, o); Some([c[0], 1.2, c[2]]) }
        _ => None,
    };
    if mode == crate::spotted::SpottedMode::Caught {
        let step = follower && !aside && spot.is_some() && what == Some(crate::spotted::ManWhat::Step);
        if !step && watch.is_none() { return None; }
        return Some(SpottedFeet { goal: if step { spot } else { None }, speed: if step && pace > 0.0 { Some(pace) } else { None }, crouch: false, watch, goal_serial: None });
    }
    let crouch = exposed || what.is_some();
    if !follower || aside || spot.is_none() {
        if !crouch && watch.is_none() { return None; }
        return Some(SpottedFeet { goal: None, speed: None, crouch, watch, goal_serial: None });
    }
    let speed = if what == Some(crate::spotted::ManWhat::Freeze) { 0.0 } else { m::min(1.2, if pace > 0.0 { pace } else { 1.2 }) };
    Some(SpottedFeet { goal: spot, speed: Some(speed), crouch, watch, goal_serial: None })
}

/// a change of mode: counted, logged, the spell's fields cleared (spotted.ts set)
pub fn set(g: &mut crate::game::Game, gd: GuardId, to: crate::spotted::SpottedMode, of: Option<GuardId>) {
    let t = g.time;
    let r = man(g, gd);
    if r.mode == to { return; }
    r.mode = to; r.since = t; r.spot = None; r.what = None; r.ask_t = -1.0; r.trail = false; r.eyes = None; r.told = false; r.real_t = -1.0;
    match to {
        crate::spotted::SpottedMode::Calm => g.stats.spotted.calm += 1,
        crate::spotted::SpottedMode::Wary => g.stats.spotted.wary += 1,
        crate::spotted::SpottedMode::Caught => g.stats.spotted.caught += 1,
    }
    log(g, t, gd, to, None, of);
    if g.m.spotted.spotted_log.len() > 200 { g.m.spotted.spotted_log.drain(0..100); }
}

/// a line said once by the man himself, no sooner than `gap` after his last (spotted.ts cue)
pub fn cue(g: &mut crate::game::Game, gd: GuardId, text: &str, gap: Option<f64>) {
    let gap = gap.unwrap_or(4.0);
    let t = g.time;
    let r = man(g, gd);
    if t - r.said_t < gap { return; }
    r.said_t = t;
    g.stats.spotted.cues += 1;
    crate::game::game_say(g, gd, text, None, None, None);
}

/// (spotted.ts ring) a generator: the skeleton's return type cannot hold it; `ring_spots` is its whole body
pub fn ring(g: &mut crate::game::Game, p: V3, max_m: f64) -> crate::Opaque { todo!("needs: ring to return Vec<V3>; then its body is ring_spots") }

/// candidate spots round `p`, nearest first: on the floor, a straight walk from where he stands, off every doorway and off Sam (spotted.ts ring)
fn ring_spots(g: &mut Game, p: V3, max_m: f64) -> Vec<V3> {
    let sam = sam_pos(g);
    let mut spots = Vec::new();
    let mut r = 0.8;
    while r <= max_m + 1e-6 {
        for k in 0..12 {
            let a = (k as f64) * m::PI / 6.0 + r * 0.7;
            let q: V3 = [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || !crate::scene_collision::nav_grid_walkable(g, p, q) || flat(&q, &sam) < 0.9 { continue; }
            let mut in_door = false;
            for &d in g.doors.list.iter() { if flat(&q, &g.dr[d].frame_centre) < 1.0 { in_door = true; break; } }
            if in_door { continue; }
            spots.push(q);
        }
        r += 0.8;
    }
    spots
}

/// between Sam's gun and the man it may be on (spotted.ts inSamsLine)
pub fn in_sams_line(g: &mut crate::game::Game, q: V3, h: GuardId) -> bool {
    let (a, b) = (sam_pos(g), pos(g, h));
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]);
    let l = m::hyp2(dx, dz);
    if l < 0.5 { return false; }
    let al = ((q[0] - a[0]) * dx + (q[2] - a[2]) * dz) / l;
    let side = m::abs((q[0] - a[0]) * dz - (q[2] - a[2]) * dx) / l;
    al > 0.0 && al < l + 1.0 && side < 0.7
}

/// SUSPICIOUS: where he goes: out of the line if lit in it, else still; Sam past the leash, he follows (spotted.ts hide)
pub fn hide(g: &mut crate::game::Game, gd: GuardId) {
    let k = g.k.spotted;
    let (p, sam) = (pos(g, gd), sam_pos(g));
    let d_sam = flat(&p, &sam);
    let wary = man(g, gd).wary.clone();
    let mut seen: Vec<GuardId> = Vec::new();
    for &h in wary.iter() { if in_line(g, h, &p, true) { seen.push(h); } }
    let mut eyes: Option<GuardId> = None;
    for &b in seen.iter() {
        eyes = Some(match eyes {
            Some(a) if flat(&pos(g, a), &p) <= flat(&pos(g, b), &p) => a,
            Some(_) => b,
            None => b,
        });
    }
    let exposed = !seen.is_empty() || wary.iter().any(|&h| flat(&pos(g, h), &p) < k.ear_m);
    let r = man(g, gd);
    r.eyes = eyes;
    r.exposed = exposed;
    if d_sam > k.leash_m { r.trail = true; } else if d_sam < k.leash_m * 0.45 { r.trail = false; }
    let mut follow = r.trail;
    if !follow && (!crate::orders::squad_orders_sam_stopped(g) || d_sam > k.leash_m * 0.45) {
        for &h in wary.iter() { if crate::sneak::sneak_coming(g, gd, h) { follow = true; break; } }
    }
    if follow { let r = man(g, gd); r.spot = None; r.what = None; return; }
    let r = man(g, gd);
    let (what, spot, real, real_t) = (r.what, r.spot, r.real, r.real_t);
    if what == Some(crate::spotted::ManWhat::Slip) {
        if let Some(sp) = spot {
            if !any_in_line(g, &wary, &sp) || lit_at(g, &sp) < k.lit_at { return; }
        }
    }
    if !seen.is_empty() && real_t >= 0.0 && g.time - real_t >= k.slip_wait && lit_at(g, &p) >= k.lit_at {
        let mut best: Option<V3> = None;
        let mut bs = f64::INFINITY;
        for q in ring_spots(g, p, k.slip_m) {
            if flat(&q, &sam) > k.leash_m || wary.iter().any(|&h| in_sams_line(g, q, h)) { continue; }
            let out = !any_in_line(g, &wary, &q);
            if !out && lit_at(g, &q) >= k.lit_at { continue; }
            let cost = flat(&p, &q) + if out { 0.0 } else { 2.0 };
            if cost < bs { bs = cost; best = Some(q); }
        }
        if let Some(b) = best {
            g.stats.spotted.slips += 1;
            if real { cue(g, gd, "in his line. moving off it.", Some(1.0)); man(g, gd).told = true; }
            let r = man(g, gd);
            r.spot = Some(b);
            r.what = Some(crate::spotted::ManWhat::Slip);
            return;
        }
    }
    if exposed {
        let frozen = what == Some(crate::spotted::ManWhat::Freeze);
        let far = match spot { Some(sp) => flat(&p, &sp) > 0.6, None => true };
        if !frozen || far {
            if !frozen {
                g.stats.spotted.freezes += 1;
                if real {
                    cue(g, gd, if !seen.is_empty() { "he's looking. holding still." } else { "he's heard something. going low." }, None);
                    man(g, gd).told = true;
                }
            }
            man(g, gd).spot = Some([p[0], 0.0, p[2]]);
        }
        man(g, gd).what = Some(crate::spotted::ManWhat::Freeze);
        return;
    }
    let r = man(g, gd);
    r.spot = None;
    r.what = None;
}

/// a round from a man stood at `q` to a body at `hp`, stopped a body's breadth short of it (spotted.ts shotLine)
pub fn shot_line(g: &mut crate::game::Game, q: &[f64], hp: &[f64], y: f64) -> bool {
    let (dx, dz) = (hp[0] - q[0], hp[2] - q[2]);
    let l = m::hyp2(dx, dz);
    if l < 0.5 { return true; }
    let k = (l - 0.3) / l;
    !crate::scene_collision::shot_blocked(g, [q[0], 1.35, q[2]], [q[0] + dx * k, 1.35 + (y - 1.35) * k, q[2] + dz * k], None, None)
}

/// CAUGHT: following with no shot on the man who has them: the nearest spot that has one, off Sam's line (spotted.ts cover)
pub fn cover(g: &mut crate::game::Game, gd: GuardId) {
    let k = g.k.spotted;
    let p = pos(g, gd);
    let r = man(g, gd);
    let (by, what, spot) = (r.by, r.what, r.spot);
    let h = match by {
        Some(h) if g.gd[h].state != crate::game::GuardState::Dead => h,
        _ => { let r = man(g, gd); r.spot = None; r.what = None; return; }
    };
    let hp = pos(g, h);
    if what == Some(crate::spotted::ManWhat::Step) {
        if let Some(sp) = spot {
            if shot_from(g, gd, &sp, hp) && !in_sams_line(g, sp, h) { return; }
        }
    }
    if shot_from(g, gd, &p, hp) { let r = man(g, gd); r.spot = None; r.what = None; return; }
    let mut best: Option<V3> = None;
    let mut bs = f64::INFINITY;
    for q in ring_spots(g, p, k.step_m) {
        if !shot_from(g, gd, &q, hp) || in_sams_line(g, q, h) || flat(&q, &hp) < 2.5 { continue; }
        let cost = flat(&p, &q) + if lit_at(g, &q) >= k.lit_at { 1.5 } else { 0.0 };
        if cost < bs { bs = cost; best = Some(q); }
    }
    if best.is_some() { g.stats.spotted.steps += 1; }
    let r = man(g, gd);
    r.spot = best;
    r.what = if best.is_some() { Some(crate::spotted::ManWhat::Step) } else { None };
}

/// `h` has turned to `gd`: roused, a line between them, and his fix on this man or squared up to him (spotted.ts onHim)
pub fn on_him(g: &mut crate::game::Game, h: GuardId, gd: GuardId) -> bool {
    let st = g.gd[h].state;
    if st != crate::game::GuardState::Suspicious && st != crate::game::GuardState::Alert && st != crate::game::GuardState::Search { return false; }
    let (p, hp) = (pos(g, gd), pos(g, h));
    let eye = eye_of(g, h);
    if crate::scene_collision::segment_blocked(g, eye, [p[0], 1.25, p[2]], None, None, None) { return false; }
    let saw = m::max(g.gd[h].sight_t, g.gd[h].last_seen_t);
    let his_fix = matches!(g.gd[h].foe, Some(crate::game::Foe::Guard(f)) if f == gd);
    if his_fix && g.time - saw >= 0.0 && g.time - saw < g.k.spotted.fresh_secs { return true; }
    let (dx, dz) = (p[0] - hp[0], p[2] - hp[2]);
    let d = js::or(m::hyp2(dx, dz), 1.0);
    let yaw = g.ch[g.gd[h].ch].aim_yaw;
    (dx * m::sin(yaw) + dz * m::cos(yaw)) / d > 0.5
}

/// AT ARM'S LENGTH: picks the man, closes, and `takeSecs` after his hands are on him the man is out cold (spotted.ts hands)
pub fn hands(g: &mut crate::game::Game, gd: GuardId) {
    let k = g.k.spotted;
    let p = pos(g, gd);
    let t = g.time;
    if let Some(tk) = man(g, gd).take {
        let h = tk.foe;
        let on = tk.t0 >= 0.0;
        let st = g.gd[h].state;
        let gone = st == crate::game::GuardState::Dead || g.gd[h].held.is_some() || !g.guards.contains(&h)
            || (!on && (crate::silent::reached(g, gd)
                || flat(&p, &pos(g, h)) > (if st == crate::game::GuardState::Suspicious || (tk.asked == Some(true) && st == crate::game::GuardState::Patrol) { k.take_m + 1.0 } else { k.take_reach + 0.3 })
                || !open_to(g, p, h)));
        let r = man(g, gd);
        if gone { r.take = None; }
        else if !on && t - tk.born > 3.0 { r.take_no = Some(ManTakeNo { foe: h, until: t + 4.0 }); r.take = None; }
    }
    let r = man(g, gd);
    if let Some(n) = r.take_no {
        if t >= n.until || t < n.until - 4.0 { r.take_no = None; }
    }
    if man(g, gd).take.is_none() && !crate::silent::reached(g, gd) && !crate::silent::quiet_gun(g, gd) {
        let take_no = man(g, gd).take_no;
        let mut best: Option<GuardId> = None;
        let mut bd = k.take_m;
        let mut i = 0;
        while i < g.guards.len() {
            let h = g.guards[i];
            i += 1;
            let st = g.gd[h].state;
            if st == crate::game::GuardState::Dead || g.gd[h].held.is_some() { continue; }
            let (mine, his) = (g.gd[gd].team, g.gd[h].team);
            if !crate::teams::is_hostile(g, mine, his) { continue; }
            if g.pl[g.player].takedown.as_ref().map(|td| td.g) == Some(h) || take_no.map(|n| n.foe) == Some(h) { continue; }
            let d = flat(&p, &pos(g, h));
            let limit = if st == crate::game::GuardState::Suspicious { bd } else { m::min(bd, k.take_reach + 0.3) };
            if d >= limit || !on_him(g, h, gd) || !open_to(g, p, h) { continue; }
            bd = d;
            best = Some(h);
        }
        if let Some(best) = best {
            let spot_serial = g.serial();
            man(g, gd).take = Some(ManTake { foe: best, t0: -1.0, born: t, spot: [p[0], 0.0, p[2]], asked: None, spot_serial });
            g.stats.spotted.takes += 1;
            cue(g, gd, "he's on me. taking him.", Some(0.0));
            let mode = man(g, gd).mode;
            log(g, t, gd, mode, Some("take"), Some(best));
        }
    }
    let Some(tk) = man(g, gd).take else { return };
    let h = tk.foe;
    let hp = pos(g, h);
    let (dx, dz) = (p[0] - hp[0], p[2] - hp[2]);
    let d = js::or(m::hyp2(dx, dz), 1.0);
    let mut t0 = tk.t0;
    if let Some(row) = man(g, gd).take.as_mut() {
        row.spot[0] = hp[0] + dx / d * 0.6;
        row.spot[2] = hp[2] + dz / d * 0.6;
    }
    if d < k.take_reach + (if t0 >= 0.0 { 0.4 } else { 0.0 }) && open_to(g, p, h) {
        if t0 < 0.0 {
            t0 = t;
            if let Some(row) = man(g, gd).take.as_mut() { row.t0 = t; }
            let c = g.gd[gd].ch;
            crate::character::anim_strike(g, c, None, None);
            crate::takedown::partner_begin(g, gd, h, true);
        }
        if t - t0 >= k.take_secs {
            man(g, gd).take = None;
            g.stats.spotted.taken += 1;
            crate::takedown::partner_fall(g, h, true, Some(gd));
            let line = alloc::format!("{} — {} is out cold", g.gd[gd].callsign, g.gd[h].callsign);
            crate::combat::kill_guard(g, h, [-dx / d, 0.0, -dz / d], Some(true), Some(crate::game::DownKind::Struck), Some(crate::game::DownBy::Guard), Some(&line), None);
            g.gd[h].killer_id = g.ch[g.gd[gd].ch].id;
            let mode = man(g, gd).mode;
            log(g, t, gd, mode, Some("taken"), Some(h));
        }
    } else {
        if t0 >= 0.0 { crate::takedown::partner_cancel(g, gd, h); }
        if let Some(row) = man(g, gd).take.as_mut() { row.t0 = -1.0; }
    }
}

/// asked for by an order layer whose stalk on `foe` has just been blown: go on and take him by hand (spotted.ts spottedTake)
pub fn spotted_take(g: &mut crate::game::Game, gd: GuardId, foe: GuardId) -> bool {
    let k = g.k.spotted;
    if !k.on || !crate::silent::silent_man(g, gd) { return false; }
    let Some(row) = g.m.spotted.men.get(gd) else { return false };
    let (take, take_no) = (row.take, row.take_no);
    if let Some(tk) = take { return tk.foe == foe; }
    let st = g.gd[foe].state;
    if st == crate::game::GuardState::Dead || g.gd[foe].held.is_some() { return false; }
    let (mine, his) = (g.gd[gd].team, g.gd[foe].team);
    if !crate::teams::is_hostile(g, mine, his) || crate::silent::reached(g, gd) || crate::silent::quiet_gun(g, gd) || take_no.map(|n| n.foe) == Some(foe) { return false; }
    let (p, c) = (pos(g, gd), pos(g, foe));
    let limit = if st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search { k.take_reach + 0.3 } else { k.take_m };
    if flat(&p, &c) >= limit || crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [c[0], 1.0, c[2]], None) { return false; }
    let now = g.time;
    let spot_serial = g.serial();
    man(g, gd).take = Some(ManTake { foe, t0: -1.0, born: now, spot: [p[0], 0.0, p[2]], asked: Some(true), spot_serial });
    g.stats.spotted.takes += 1;
    cue(g, gd, "taking him.", Some(0.0));
    let mode = man(g, gd).mode;
    log(g, now, gd, mode, Some("take"), Some(foe));
    true
}

/// his hands are ON this man, closing does not count (spotted.ts spottedGripped)
pub fn spotted_gripped(g: &mut crate::game::Game, h: GuardId) -> bool {
    if !g.k.spotted.on { return false; }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || !crate::silent::silent_man(g, gd) { continue; }
        if let Some(tk) = g.m.spotted.men.get(gd).and_then(|r| r.take) {
            if tk.foe == h && tk.t0 >= 0.0 && g.time >= tk.t0 && g.time - tk.t0 < g.k.spotted.take_secs + 0.5 { return true; }
        }
    }
    false
}

/// the men dropped in the last `braceSecs` (spotted.ts fresh). Nothing calls it on the page: SPOTTED.brace is settled off
pub fn fresh(g: &mut crate::game::Game) -> Vec<crate::spotted::FreshItem> {
    let now = g.time;
    let brace_secs = g.k.spotted.brace_secs;
    let d = g.m.spotted.drops.get_or_insert_with(|| SpottedModDropsValue { down: JsSet::new(), at: Vec::new(), seen: false, t: now });
    if now - d.t > 0.25 || now < d.t { d.seen = false; d.at.clear(); }
    d.t = now;
    for &h in g.guards.iter() {
        if g.gd[h].state != crate::game::GuardState::Dead || d.down.has(h) { continue; }
        d.down.add(h);
        if d.seen && g.gd[h].team != crate::teams::Team::Operative {
            let c = g.ch[g.gd[h].ch].pos;
            d.at.push(SpottedModDropsValueAtItem { pos: [c[0], 0.0, c[2]], t: now });
            g.stats.spotted.braces += 1;
        }
    }
    d.seen = true;
    while !d.at.is_empty() && (now - d.at[0].t > brace_secs || now < d.at[0].t) { d.at.remove(0); }
    d.at.iter().map(|a| FreshItem { pos: a.pos, t: a.t }).collect()
}

/// once a frame, after silent.ts has the posture and before the orders pose the men (spotted.ts spottedTick)
pub fn spotted_tick(g: &mut crate::game::Game) {
    if !g.k.spotted.on || !crate::silent::squad_silent(g) { return; }
    let k = g.k.spotted;
    let t = g.time;
    for gd in crate::orders::squad_orders_squad(g) {
        if g.gd[gd].state == crate::game::GuardState::Dead || !crate::silent::silent_man(g, gd) { continue; }
        if !g.m.spotted.men.has(gd) { g.m.spotted.men.set(gd, new_man(t)); }
        let r = man(g, gd);
        if t - r.tick_t > 0.25 || t < r.tick_t { r.take = None; r.take_no = None; }
        r.tick_t = t;
        r.wary.clear();
        r.real = false;
        let mut by: Option<GuardId> = None;
        let mut bd = f64::INFINITY;
        let mut i = 0;
        while i < g.guards.len() {
            let h = g.guards[i];
            i += 1;
            let (mine, his) = (g.gd[gd].team, g.gd[h].team);
            if g.gd[h].state == crate::game::GuardState::Dead || !crate::teams::is_hostile(g, mine, his) { continue; }
            let d = flat(&pos(g, gd), &pos(g, h));
            if has_them(g, h) {
                if d < k.caught_m && d < bd { bd = d; by = Some(h); }
            } else if g.gd[h].state == crate::game::GuardState::Suspicious && d < k.wary_m {
                let r = man(g, gd);
                r.wary.push(h);
                r.real = true;
            }
        }
        let r = man(g, gd);
        if !r.real { r.real_t = -1.0; } else if r.real_t < 0.0 { r.real_t = t; }
        hands(g, gd);
        let old_by = man(g, gd).by;
        if let Some(b) = by {
            let r = man(g, gd);
            r.by = Some(b);
            r.caught_t = t;
        } else if let Some(b) = old_by {
            if g.gd[b].state == crate::game::GuardState::Dead { man(g, gd).by = None; }
        }
        let r = man(g, gd);
        if !r.wary.is_empty() { r.wary_t = t; }
        let caught_t = r.caught_t;
        let wary = !r.wary.is_empty() || t - r.wary_t < k.linger_secs;
        let was = r.mode;
        let caught = by.is_some() || t - caught_t < k.last_secs || crate::silent::reached(g, gd);
        if caught {
            set(g, gd, crate::spotted::SpottedMode::Caught, by);
            if was != crate::spotted::SpottedMode::Caught {
                let text = match by {
                    Some(b) if matches!(g.gd[b].foe, Some(crate::game::Foe::Player)) => "he's made you! covering!",
                    Some(_) => "he's made me! contact!",
                    None => "contact! covering you!",
                };
                cue(g, gd, text, Some(0.0));
            }
            let r = man(g, gd);
            if t - r.ask_t > 0.4 { r.ask_t = t; cover(g, gd); }
        } else if wary {
            let first = man(g, gd).wary.first().copied();
            set(g, gd, crate::spotted::SpottedMode::Wary, first);
            let r = man(g, gd);
            if r.wary.is_empty() { r.eyes = None; }
            else if t - r.ask_t > 0.4 { r.ask_t = t; hide(g, gd); }
            let r = man(g, gd);
            if r.real && !r.told && r.what.is_some() {
                r.told = true;
                cue(g, gd, "he's heard it. holding still.", None);
            }
        } else {
            man(g, gd).eyes = None;
            if was != crate::spotted::SpottedMode::Calm {
                let told = man(g, gd).told;
                set(g, gd, crate::spotted::SpottedMode::Calm, None);
                if was == crate::spotted::SpottedMode::Caught || told {
                    let text = if was == crate::spotted::SpottedMode::Caught { "lost us. going quiet." } else { "he's settled. moving." };
                    cue(g, gd, text, Some(2.0));
                }
            }
            man(g, gd).exposed = false;
        }
    }
}

/// the squad line's word while a silent squad is not yet 'compromised'; None: silent.ts's own (spotted.ts spottedWord)
pub fn spotted_word(g: &mut crate::game::Game) -> Option<String> {
    if !g.k.spotted.on { return None; }
    let mut wary = false;
    for gd in crate::orders::squad_orders_squad(g) {
        let mode = spotted_mode(g, gd);
        if mode == crate::spotted::SpottedMode::Caught { return Some(String::from("SILENT · contact")); }
        if mode == crate::spotted::SpottedMode::Wary { wary = true; }
    }
    if wary { Some(String::from("SILENT · low")) } else { None }
}

/// the panel's one line (spotted.ts spottedSummary)
pub fn spotted_summary(g: &mut crate::game::Game) -> String {
    if !g.k.spotted.on { return String::from("off"); }
    let squad = crate::orders::squad_orders_squad(g);
    let sq: Vec<GuardId> = squad.into_iter().filter(|&gd| g.gd[gd].state != crate::game::GuardState::Dead).collect();
    if sq.is_empty() { return String::from("on · no squad"); }
    if !crate::silent::squad_silent(g) { return String::from("on · the squad is loud (it only acts on a silent squad)"); }
    let mut parts: Vec<String> = Vec::new();
    for gd in sq {
        let row = g.m.spotted.men.get(gd);
        let mode = row.map_or("calm", |r| r.mode.as_str());
        let what = match row.and_then(|r| r.what) { Some(w) => alloc::format!(" ({})", w.as_str()), None => String::new() };
        parts.push(alloc::format!("{} {}{}", g.gd[gd].callsign, mode, what));
    }
    parts.join(" · ")
}

/// what a man is doing, for the probes and the tally (spotted.ts spottedOf)
pub fn spotted_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::spotted::SpottedOf> {
    let r = g.m.spotted.men.get(gd)?;
    Some(SpottedOf {
        mode: r.mode,
        what: r.what.map(|w| String::from(w.as_str())),
        exposed: r.exposed,
        spot: r.spot,
        by: r.by.map(|b| g.gd[b].callsign.clone()),
        wary: r.wary.iter().map(|&h| g.gd[h].callsign.clone()).collect(),
    })
}

/// the debugger panel's row, kept in the browser's storage: host work, not translated (spotted.ts saveSpotted)
pub fn save_spotted(g: &mut crate::game::Game) { todo!("host only: localStorage, the panel's rows") }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spotted::SpottedMode as Mode;

    /// a blank game with one man, whose row is a new man's; leaked, so a test holds it as a plain reference
    fn game() -> (&'static mut Game, GuardId) {
        let g: &'static mut Game = Box::leak(Box::new(Game::blank()));
        let gd = add_man(g);
        g.time = 100.0;
        g.m.spotted.men.set(gd, new_man(100.0));
        (g, gd)
    }
    fn add_man(g: &mut Game) -> GuardId {
        let c = g.ch.push(crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        g.gd.push(man)
    }

    #[test]
    fn a_change_of_mode_writes_the_row_and_ends_the_spell() {
        let (g, gd) = game();
        let r = man(g, gd);
        r.spot = Some([1.0, 0.0, 2.0]); r.what = Some(crate::spotted::ManWhat::Freeze); r.ask_t = 99.0; r.trail = true; r.told = true; r.real_t = 98.0; r.eyes = Some(gd);
        set(g, gd, Mode::Caught, None);
        let r = g.m.spotted.men.get(gd).unwrap();
        assert_eq!(r.mode, Mode::Caught);
        assert_eq!(r.since, 100.0);
        assert!(r.spot.is_none() && r.what.is_none() && r.eyes.is_none() && !r.trail && !r.told);
        assert_eq!((r.ask_t, r.real_t), (-1.0, -1.0));
        assert_eq!(g.stats.spotted.caught, 1);
        assert_eq!(g.m.spotted.spotted_log.len(), 1);
        // the same mode again is nothing
        man(g, gd).since = 7.0;
        set(g, gd, Mode::Caught, None);
        assert_eq!(g.m.spotted.men.get(gd).unwrap().since, 7.0);
        assert_eq!(g.stats.spotted.caught, 1);
        assert_eq!(g.m.spotted.spotted_log.len(), 1);
    }

    #[test]
    fn the_log_drops_its_oldest_hundred_past_two_hundred() {
        let (g, gd) = game();
        for i in 0..201 { set(g, gd, if i % 2 == 0 { Mode::Wary } else { Mode::Calm }, None); }
        assert_eq!(g.m.spotted.spotted_log.len(), 101);
    }

    #[test]
    fn fresh_learns_who_is_down_then_reports_new_men_until_brace_secs() {
        let (g, _) = game();
        let old = add_man(g);
        g.gd[old].state = crate::game::GuardState::Dead;
        g.guards.push(old);
        assert!(fresh(g).is_empty());   // the first tick only learns
        let new = add_man(g);
        g.gd[new].state = crate::game::GuardState::Dead;
        g.gd[new].team = crate::teams::Team::Hostile;
        let c = g.gd[new].ch;
        g.ch[c].pos = [3.0, 1.0, 4.0];
        g.guards.push(new);
        g.time = 100.1;
        let at = fresh(g);
        assert_eq!(at.len(), 1);
        assert_eq!(at[0].pos, [3.0, 0.0, 4.0]);
        assert_eq!(g.stats.spotted.braces, 1);
        // still within reach of the last ask, past `braceSecs` of the drop: a step at a time, gone
        let secs = g.k.spotted.brace_secs;
        let mut t = 100.1;
        while t < 100.1 + secs + 0.2 { t += 0.2; g.time = t; let _ = fresh(g); }
        assert!(fresh(g).is_empty());
        assert_eq!(g.stats.spotted.braces, 1);
    }

    #[test]
    fn fresh_forgets_after_a_gap_or_a_clock_gone_back() {
        let (g, _) = game();
        let _ = fresh(g);
        let a = add_man(g);
        g.gd[a].state = crate::game::GuardState::Dead;
        g.gd[a].team = crate::teams::Team::Hostile;
        g.guards.push(a);
        g.time = 100.5;   // not asked for over a quarter second: whoever is down now is old news
        assert!(fresh(g).is_empty());
        assert_eq!(g.stats.spotted.braces, 0);
        let b = add_man(g);
        g.gd[b].state = crate::game::GuardState::Dead;
        g.gd[b].team = crate::teams::Team::Hostile;
        g.guards.push(b);
        g.time = 50.0;   // the clock went back
        assert!(fresh(g).is_empty());
        assert_eq!(g.stats.spotted.braces, 0);
    }
}
