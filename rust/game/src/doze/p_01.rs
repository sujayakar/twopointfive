//! src/game/doze.ts lines 1 to 374
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the one record: GAMES and REC are one table here (g.m.doze.rec), since a game's guards are that game's

/// a changed row back onto the books (doze.ts:81, D is a live object there). Nothing on a row already dropped, as a write to a detached object.
fn flush(g: &mut Game, gd: GuardId, d: Doze) {
    if let Some(r) = g.m.doze.rec.get_mut(gd) { *r = d; }
}
/// g.lightHome.in(LIGHT_IN.on, t, v) (doze.ts:113, 119, 121, 273, 279)
fn switch(g: &mut Game, t: FixtureId, v: f64) {
    crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().on, Some(t), Some(v), None, None, None);
}
fn is_held_seat(p: Phase) -> bool { p != Phase::No && p != Phase::Spent && p != Phase::Stir }
fn why_word(w: WakeWhy) -> DozeWhy {
    match w {
        WakeWhy::Light => DozeWhy::Light,
        WakeWhy::Alarm => DozeWhy::Alarm,
        WakeWhy::Noise => DozeWhy::Noise,
        WakeWhy::Taken => DozeWhy::Taken,
        WakeWhy::Bump => DozeWhy::Bump,
        WakeWhy::Net => DozeWhy::Net,
    }
}

/// every 5 cm offset inside `pull_m`, nearest first (doze.ts:89-91, the building of PULLS)
fn build_pulls(pull_m: f64) -> Vec<[f64; 2]> {
    let mut v: Vec<[f64; 2]> = Vec::new();
    let n = m::ceil(pull_m / 0.05);
    let mut ix = -n;
    while ix <= n {
        let mut iz = -n;
        while iz <= n {
            if m::hyp2(ix, iz) * 0.05 <= pull_m + 1e-6 { v.push([ix * 0.05, iz * 0.05]); }
            iz += 1.0;
        }
        ix += 1.0;
    }
    js::sort_by(&mut v, |a, b| js::or(m::hyp2(a[0], a[1]) - m::hyp2(b[0], b[1]), js::or(a[0] - b[0], a[1] - b[1])));
    v
}
/// how far, and which way, a man will pull a chair up (doze.ts pulls); kept until `pull_m` or the maths change.
pub fn pulls(g: &mut crate::game::Game) -> Vec<[f64; 2]> {
    let (pull_m, epoch) = (g.k.doze.pull_m, g.m.math_exact.maths_epoch);
    if g.m.doze.pulls.is_none() || g.m.doze.pulls_m != pull_m || g.m.doze.pulls_e != epoch {
        g.m.doze.pulls_m = pull_m;
        g.m.doze.pulls_e = epoch;
        g.m.doze.pulls = Some(build_pulls(pull_m));
    }
    g.m.doze.pulls.clone().unwrap()
}
/// this game's table of records, made if absent (doze.ts mapOf). Nothing to do: the table is g.m.doze.rec, always there.
pub fn map_of(g: &mut crate::game::Game) {}

/// distance on the floor (doze.ts flat)
pub fn flat(a: &[f64], x: f64, z: f64) -> f64 { m::hyp2(a[0] - x, a[2] - z) }
/// which way a man in this chair faces (doze.ts chairFace)
pub fn chair_face(g: &mut crate::game::Game, p: PropId) -> f64 {
    let y = crate::props::prop_yaw(g, p);
    m::atan2(-m::sin(y), -m::cos(y))
}
/// the run is one a man may doze in (doze.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    g.ai_enabled && !g.quiet_utility && !g.touring && !g.puppet.is_some() && !g.pl[g.player].down
        && !g.level.scenarios.as_ref().map_or(false, |s| s.lesson.is_some())
        && crate::guards::escalation_of(g) == 0.0 && !g.alarm.episode
        && !crate::sim_lightshome::mains_active(g)
}
/// a staff man on a calm patrol with nothing on his mind or in his hands (doze.ts calm)
pub fn calm(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let m = &g.gd[gd];
    m.team == crate::teams::Team::Guard && m.unit.is_none() && m.civ.is_none() && m.state == crate::game::GuardState::Patrol && m.awareness < 0.15 && m.held.is_none()
        && m.script.is_none() && m.task.is_none() && m.body_duty.is_none() && !m.pinned && m.leader.is_none() && m.post.is_none() && !m.drawn
}
/// put his record on this game's books (doze.ts put)
pub fn put(g: &mut crate::game::Game, gd: GuardId, d: crate::doze::Doze) {
    g.m.doze.rec.set(gd, d);
}
/// take his record off the books (doze.ts drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) {
    g.m.doze.rec.delete(gd);
    let c = g.gd[gd].ch;
    g.ch[c].anim.slump = 0.0;
}
/// the man at a screen is taken off the books: what he switched on goes off with him (doze.ts dark)
pub fn dark(g: &mut crate::game::Game, gd: GuardId) {
    let Some(d) = g.m.doze.rec.get(gd).copied() else { return };
    if d.phase == Phase::Work && !d.mon_was {
        if let Some(mon) = d.mon { switch(g, mon, 0.0); }
    }
}
/// a screen a man was got away from stays lit `idle_secs` (doze.ts leftLit)
pub fn left_lit(g: &mut crate::game::Game, gd: GuardId) {
    let Some(d) = g.m.doze.rec.get(gd).copied() else { return };
    if d.phase != Phase::Work || d.mon_was { return; }
    let Some(mon) = d.mon else { return };
    if !crate::sim_lightshome::fixture_on(g, mon) || crate::sim_lightshome::fixture_broken(g, mon) { return; }
    let until = g.time + g.k.doze.idle_secs;
    g.m.doze.left.get_or_insert_with(JsMap::new).set(mon, until);
}
/// every left screen off, the table gone (doze.ts leftOff)
pub fn left_off(g: &mut crate::game::Game) {
    let Some(l) = g.m.doze.left.as_ref() else { return };
    for t in l.keys() { switch(g, t, 0.0); }
    g.m.doze.left = None;
}
/// a left screen blanks when its time comes, or is gone when it is dead (doze.ts leftTick)
pub fn left_tick(g: &mut crate::game::Game) {
    if g.m.doze.left.is_none() { return; }
    let mut walk = 0usize;
    while let Some(t) = g.m.doze.left.as_ref().and_then(|l| l.next_key(&mut walk)) {
        let Some(&at) = g.m.doze.left.as_ref().and_then(|l| l.get(t)) else { continue };
        if g.time >= at || !crate::sim_lightshome::fixture_on(g, t) || crate::sim_lightshome::fixture_broken(g, t) {
            if crate::sim_lightshome::fixture_on(g, t) && !crate::sim_lightshome::fixture_broken(g, t) { switch(g, t, 0.0); }
            if let Some(l) = g.m.doze.left.as_mut() { l.delete(t); }
        }
    }
    if g.m.doze.left.as_ref().map_or(false, |l| l.is_empty()) { g.m.doze.left = None; }
}
/// a new record (doze.ts blank)
pub fn blank(phase: crate::doze::Phase, wp: f64, t: f64) -> crate::doze::Doze {
    Doze { phase, wp, chair: None, mon: None, mon_was: false, say: -1.0, to: [0.0, 0.0, 0.0], at: [0.0, 0.0, 0.0], face: 0.0, cx: 0.0, cz: 0.0, cyaw: 0.0, t: 0.0, seen: t, until: 0.0, lit: -1.0, ask: 0.0, push: 0.0, why: None, woke_at: -1.0 }
}

/// the length of his walk on the grid, infinite with no path (doze.ts walkLen)
pub fn walk_len(g: &mut crate::game::Game, from: &[f64], to: V3) -> f64 {
    let Some(path) = crate::scene_collision::nav_grid_find_path(g, [from[0], from[1], from[2]], to, None, None) else { return f64::INFINITY };
    let mut len = 0.0;
    let mut prev: V3 = [from[0], from[1], from[2]];
    for q in path.iter() {
        len += flat(&prev, q[0], q[2]);
        prev = *q;
    }
    len
}
/// in the chair (doze.ts seated)
pub fn seated(d: crate::doze::Doze) -> bool { d.phase == Phase::Down || d.phase == Phase::Asleep || d.phase == Phase::Work }
/// a number drawn from [from, to] (doze.ts span)
pub fn span(g: &mut crate::game::Game, r: &[f64]) -> f64 {
    r[0] + crate::math_dice::sim_random(g) * (r[1] - r[0])
}
/// the lines, for the words lint (doze.ts dozeLines)
pub fn doze_lines() -> Vec<String> { screen_lines() }

/// a chair at a screen: the screen, and the place nearest the chair from which a man facing it has floor under him and it in his eye (doze.ts screenSeat)
pub fn screen_seat(g: &mut crate::game::Game, gd: GuardId, p: PropId) -> Option<crate::doze::Seat> {
    let (px, pz) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p));
    let (screen_m, pull_m, chair_m) = (g.k.doze.screen_m, g.k.doze.pull_m, g.k.doze.chair_m);
    let mut out = touch();
    let targets = g.targets.len();
    for ti in 0..targets {
        let t = g.targets[ti];
        { let f = &g.tab.sim_lightshome_fixture[t];
          if f.group != "monitors" || f.kind != crate::sim_lightshome::FixtureKind::Emissive { continue; } }
        if crate::sim_lightshome::fixture_broken(g, t) { continue; }
        let tpos = g.tab.sim_lightshome_fixture[t].pos;
        if flat(&tpos, px, pz) > screen_m + pull_m { continue; }
        let used = g.m.doze.rec.values().any(|d| d.mon == Some(t) && is_held_seat(d.phase));
        if used { continue; }
        let first = g.tab.sim_lightshome_fixture[t].area_lights.as_ref().and_then(|a| a.first()).map(|a| a.light);
        let n = first.map(|l| g.tab.light[l].dir);
        let eye = first.map_or(tpos, |l| g.tab.light[l].pos);
        for off in pulls(g).iter() {
            let (ox, oz) = (off[0], off[1]);
            let ax = px + ox;
            let az = pz + oz;
            let dx = ax - tpos[0];
            let dz = az - tpos[2];
            let d = m::hyp2(dx, dz);
            if d < 0.5 || d > screen_m { continue; }
            if let Some(n) = n { if (n[0] * dx + n[2] * dz) / d < 0.5 { continue; } }
            let rx = ax - dx / d * 0.33;
            let rz = az - dz / d * 0.33;
            if crate::scene_collision::circle_touches(g, rx, rz, 0.27, 0.1, 1.5, &mut out) != 0.0 || crate::scene_collision::segment_blocked(g, [rx, 1.05, rz], [eye[0], eye[1], eye[2]], None, None, None) { continue; }
            let to: Option<V3> = if crate::scene_collision::nav_grid_is_blocked(g, rx, rz) { crate::scene_collision::nav_grid_nearest_free_point(g, rx, rz) } else { Some([rx, 0.0, rz]) };
            let Some(to) = to else { continue };
            if flat(&to, rx, rz) > 0.8 || crate::scene_collision::segment_blocked(g, [to[0], 0.3, to[2]], [rx, 0.3, rz], None, None, None) { continue; }
            let c = g.gd[gd].ch;
            let pos = g.ch[c].pos;
            if walk_len(g, &pos, to) > chair_m * 1.6 { break; }
            return Some(Seat { chair: p, at: [ax, 0.0, az], face: m::atan2(-dx, -dz), mon: Some(t), to });
        }
    }
    None
}
/// the chair for this wait and the way he will sit in it (doze.ts findChair)
pub fn find_chair(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doze::Seat> {
    let route = g.gd[gd].route;
    let wp_i = g.gd[gd].wp;
    let wp = g.tab.route[route].points[wp_i];
    let mut best: Option<Seat> = None;
    let mut bd = g.k.doze.chair_m;
    let mut scr: Option<Seat> = None;
    let mut sd = g.k.doze.chair_m;
    let mut pi = 0usize;
    while pi < g.props.props.len() {
        let p = g.props.props[pi];
        pi += 1;
        if g.tab.props_prop[p].def.kind != crate::scene_level_data::PropKindName::Chair || !crate::props::prop_asleep(g, p) { continue; }
        let (px, pz) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p));
        let d = flat(&wp, px, pz);
        if d >= bd && d >= sd { continue; }
        let mut taken = g.m.doze.rec.values().any(|r| r.chair == Some(p) && is_held_seat(r.phase));
        if !taken {
            for ci in 0..g.civilians.len() {
                let cv = g.civilians[ci];
                let pts = &g.tab.route[g.gd[cv].route].points;
                let tasks = g.gd[cv].civ.and_then(|id| g.cv[id].def.tasks.as_ref());
                for i in 0..pts.len() {
                    if tasks.and_then(|t| t.get(i)) == Some(&crate::scene_level::CivilianTask::Sit) && flat(&pts[i], px, pz) < 0.8 { taken = true; }
                }
            }
        }
        if taken { continue; }
        if d < sd {
            if let Some(s) = screen_seat(g, gd, p) { scr = Some(s); sd = d; }
        }
        if d >= bd || crate::calmtorch::work_lit(g, px, pz, None) { continue; }
        let f0 = chair_face(g, p);
        let mut face = f64::NAN;
        let mut k = 0i32;
        while k < 8 && face.is_nan() {
            let turn = (if k % 2 != 0 { -1.0 } else { 1.0 }) * m::ceil(k as f64 / 2.0);
            let f = f0 + turn * m::PI / 4.0;
            let (sx, sz) = (m::sin(f), m::cos(f));
            k += 1;
            if crate::scene_collision::nav_grid_is_blocked(g, px + sx * 0.33, pz + sz * 0.33)
                || crate::scene_collision::segment_blocked(g, [px, 0.45, pz], [px + sx * 0.85, 0.45, pz + sz * 0.85], None, None, None) { continue; }
            face = f;
        }
        if face.is_nan() { continue; }
        let to: V3 = [px + m::sin(face) * 0.33, 0.0, pz + m::cos(face) * 0.33];
        let c = g.gd[gd].ch;
        let pos = g.ch[c].pos;
        if walk_len(g, &pos, to) > g.k.doze.chair_m * 1.6 { continue; }
        best = Some(Seat { chair: p, at: [px, 0.0, pz], face, mon: None, to });
        bd = d;
    }
    let wait = g.tab.route[route].wait.get(wp_i).copied().unwrap_or(0.0);
    if wait >= g.k.doze.nap_wait { best.or(scr) } else { scr.or(best) }
}

/// the patrol's seam: true = his feet are this file's this frame, walking to the chair (doze.ts dozePatrol)
pub fn doze_patrol(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool {
    let d = match g.m.doze.rec.get(gd).copied() {
        Some(d) => d,
        None => {
            if g.gd[gd].wait < g.k.doze.min_wait || !live(g) || !calm(g, gd) { return false; }
            let route = g.gd[gd].route;
            let wp_i = g.gd[gd].wp;
            let wp = g.tab.route[route].points[wp_i];
            let stand = g.tab.route[route].wait.get(wp_i).copied().unwrap_or(0.0);
            let c = if stand > g.k.doze.max_wait || crate::calmtorch::work_lit(g, wp[0], wp[2], None) || !crate::darkfoe::dark_foe_blind(g, gd) { None } else { find_chair(g, gd) };
            let Some(c) = c else {
                let no = blank(Phase::No, wp_i as f64, g.time);
                put(g, gd, no);
                return false;
            };
            let mut d = blank(Phase::Going, wp_i as f64, g.time);
            d.chair = Some(c.chair);
            d.face = c.face;
            d.at = c.at;
            d.cx = crate::props::prop_x(g, c.chair);
            d.cz = crate::props::prop_z(g, c.chair);
            d.mon = c.mon;
            d.to = c.to;
            put(g, gd, d);
            d
        }
    };
    if d.phase == Phase::No {
        if g.gd[gd].wait <= 0.0 || g.gd[gd].wp as f64 != d.wp { drop(g, gd); }
        return false;
    }
    if d.phase != Phase::Going { return false; }
    let mut d = d;
    let ch = d.chair.unwrap();
    d.seen = g.time;
    flush(g, gd, d);
    if !live(g) || !calm(g, gd) || g.gd[gd].hold
        || g.gd[gd].wait < g.k.doze.rise_secs + g.k.doze.nod_secs + 4.0
        || (!crate::props::prop_asleep(g, ch) && m::hyp2(crate::props::prop_x(g, ch) - d.cx, crate::props::prop_z(g, ch) - d.cz) > 0.15)
        || crate::guards::not_getting_anywhere(g, gd, dt, 6.0)
    {
        let wp = d.wp;
        drop(g, gd);
        let no = blank(Phase::No, wp, g.time);
        put(g, gd, no);
        return false;
    }
    let to = d.to;
    crate::guards::go_to(g, gd, to, None);
    if crate::guards::follow_path(g, gd, dt, walk) || { let c = g.gd[gd].ch; flat(&g.ch[c].pos, to[0], to[2]) < 0.3 } {
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_serial = g.gd[gd].path_serial.wrapping_add(1);
        g.gd[gd].path_goal = None;
        g.gd[gd].path_goal_orbit = false;
        if !crate::close::close_seat(g, gd, Some(&d.at[..]), Some(d.face)) {
            let wp = d.wp;
            drop(g, gd);
            let no = blank(Phase::No, wp, g.time);
            put(g, gd, no);
            return false;
        }
        d.phase = Phase::Down;
        d.t = 0.0;
        d.cx = crate::props::prop_x(g, ch);
        d.cz = crate::props::prop_z(g, ch);
        d.cyaw = m::atan2(-m::sin(d.face), -m::cos(d.face));
        d.push = crate::props::prop_push_t(g, ch);
        flush(g, gd, d);
        g.stats.doze.sat += 1;
    }
    g.gd[gd].wait -= dt;
    true
}

/// out of the chair, whatever had him in it (doze.ts up)
pub fn up(g: &mut crate::game::Game, gd: GuardId) {
    crate::close::close_unseat(g, gd, None);
    let c = g.gd[gd].ch;
    g.ch[c].anim.slump = 0.0;
}
/// woken: up from this frame and awake for the run (doze.ts wake). His row is read by `gd` and written back by it.
pub fn wake(g: &mut crate::game::Game, gd: GuardId, why: crate::doze::WakeWhy, at: Option<&[f64]>) {
    let Some(mut d) = g.m.doze.rec.get(gd).copied() else { return };
    left_lit(g, gd);
    up(g, gd);
    d.why = Some(why_word(why));
    d.woke_at = g.time;
    d.phase = Phase::Spent;
    d.chair = None;
    flush(g, gd, d);
    g.stats.doze.woke[why as usize] += 1;
    if why == WakeWhy::Noise || why == WakeWhy::Light || why == WakeWhy::Bump {
        let raise = if why == WakeWhy::Bump { g.k.doze.bump_aw } else { g.k.doze.wake_aw };
        g.gd[gd].awareness = m::max(g.gd[gd].awareness, raise);
        if let Some(at) = at {
            g.gd[gd].last_known = Some([at[0], 0.0, at[2]]);
            g.gd[gd].fix_stamp = g.gd[gd].fix_stamp.wrapping_add(1);
        }
        d.phase = Phase::Stir;
        d.until = g.time + g.k.doze.stir_secs;
        flush(g, gd, d);
    }
}

/// the senses' seam: true = he is asleep and his senses were run here, ears only, at `hear_k`; a sound that counted wakes him (doze.ts dozeDeaf)
pub fn doze_deaf(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.m.doze.rec.get(gd).map_or(true, |d| d.phase != Phase::Asleep) { return false; }
    let (aw, lk, st) = (g.gd[gd].awareness, g.gd[gd].fix_stamp, g.gd[gd].state);
    let hear_k = g.k.doze.hear_k;
    crate::guards::hearing_check(g, gd, Some(hear_k), Some(false), None);
    if g.gd[gd].awareness > aw || g.gd[gd].fix_stamp != lk || g.gd[gd].state != st { wake(g, gd, WakeWhy::Noise, None); }
    g.gd[gd].saw_player_this_frame = false;
    g.gd[gd].saw_foe_this_frame = false;
    true
}

/// once a frame: the seated men's clocks, the chair held under them, what wakes a sleeper that is not a sound, the rise at the end of the wait (doze.ts dozeTick)
pub fn doze_tick(g: &mut crate::game::Game, dt: f64) {
    left_tick(g);
    if g.m.doze.rec.is_empty() { return; }
    let mut walk = 0usize;
    while let Some(gd) = g.m.doze.rec.next_key(&mut walk) {
        let Some(mut d) = g.m.doze.rec.get(gd).copied() else { continue };
        if d.phase == Phase::No || d.phase == Phase::Spent { continue; }
        let c = g.gd[gd].ch;
        if !g.guards.contains(&gd) || g.gd[gd].state == crate::game::GuardState::Dead {
            if g.gd[gd].state != crate::game::GuardState::Dead { dark(g, gd); } else { left_lit(g, gd); }
            if seated(d) { up(g, gd); }
            drop(g, gd);
            continue;
        }
        if d.phase == Phase::Going {
            if g.time - d.seen > 0.5 { drop(g, gd); }
            continue;
        }
        if d.phase == Phase::Stir {
            if g.time >= d.until {
                d.phase = Phase::Spent;
                flush(g, gd, d);
            } else {
                g.gd[gd].react_t = m::max(g.gd[gd].react_t, d.until - g.time);
            }
            continue;
        }
        if d.phase == Phase::Up {
            if g.time >= d.until
                && (d.chair.is_none() || g.gd[gd].state != crate::game::GuardState::Patrol
                    || { let ch = d.chair.unwrap(); let pos = g.ch[c].pos; let (x, z) = (crate::props::prop_x(g, ch), crate::props::prop_z(g, ch)); flat(&pos, x, z) > 0.75 }
                    || g.time >= d.until + 30.0)
            { drop(g, gd); }
            continue;
        }
        if !seated(d) { continue; }
        let ch = d.chair.unwrap();
        if g.gd[gd].held.is_some() {
            left_lit(g, gd);
            d.why = Some(DozeWhy::Taken);
            d.woke_at = g.time;
            d.phase = Phase::Spent;
            d.chair = None;
            flush(g, gd, d);
            g.ch[c].anim.slump = 0.0;
            g.stats.doze.woke[WakeWhy::Taken as usize] += 1;
            continue;
        }
        if crate::props::prop_push_t(g, ch) > d.push + 1e-6 {
            let who = shover_at(g, ch, c);
            wake(g, gd, WakeWhy::Bump, Some(&who[..]));
            continue;
        }
        if g.gd[gd].state != crate::game::GuardState::Patrol || g.gd[gd].awareness >= 0.3 {
            wake(g, gd, WakeWhy::Net, None);
            continue;
        }
        if !live(g) || g.gd[gd].script.is_some() || g.gd[gd].task.is_some() || g.gd[gd].body_duty.is_some() || !crate::close::close_has_seat(g, gd) {
            wake(g, gd, WakeWhy::Alarm, None);
            continue;
        }
        if d.mon.is_some() {
            let k = 1.0 - m::exp(-4.0 * dt);
            d.cx += (d.at[0] - d.cx) * k;
            d.cz += (d.at[2] - d.cz) * k;
            flush(g, gd, d);
        }
        crate::props::prop_set_x(g, ch, d.cx);
        crate::props::prop_set_z(g, ch, d.cz);
        crate::props::prop_set_w(g, ch, 0.0);
        crate::props::prop_set_vz(g, ch, 0.0);
        crate::props::prop_set_vx(g, ch, 0.0);
        let yaw = crate::props::prop_yaw(g, ch);
        if yaw != d.cyaw {
            let mut dy = d.cyaw - yaw;
            dy = m::atan2(m::sin(dy), m::cos(dy));
            let turned = if m::abs(dy) < 0.02 { d.cyaw } else { yaw + dy * m::min(1.0, 6.0 * dt) };
            crate::props::prop_set_yaw(g, ch, turned);
        }
        crate::props::prop_set_dirty(g, ch, true);
        crate::props::prop_place(g, ch);
        if g.gd[gd].wait - dt <= g.k.doze.rise_secs {
            if d.phase == Phase::Work && !d.mon_was {
                if let Some(mon) = d.mon { switch(g, mon, 0.0); }
            }
            up(g, gd);
            crate::character::anim_tell(g, c, crate::character::TellKind::Neck);
            d.phase = Phase::Up;
            d.until = g.time + 1.2;
            flush(g, gd, d);
            g.stats.doze.rose += 1;
            continue;
        }
        g.gd[gd].wait -= dt;
        d.t += dt;
        flush(g, gd, d);
        if d.phase == Phase::Down {
            g.ch[c].anim.slump = if d.mon.is_some() { 0.15 } else { 0.35 };
            if !crate::close::close_seated(g, gd) || g.gd[gd].awareness >= 0.15 { continue; }
            if let Some(mon) = d.mon {
                if d.t >= g.k.doze.settle_secs {
                    d.phase = Phase::Work;
                    d.t = 0.0;
                    flush(g, gd, d);
                    // (`on && !LEFT.delete`: the delete runs only for a screen that is on)
                    d.mon_was = crate::sim_lightshome::fixture_on(g, mon) && !g.m.doze.left.as_mut().map_or(false, |l| l.delete(mon).is_some());
                    flush(g, gd, d);
                    switch(g, mon, 1.0);
                    let first = g.k.doze.say_first.clone();
                    let t = g.time;
                    d.say = t + span(g, &first);
                    flush(g, gd, d);
                    g.stats.doze.worked += 1;
                }
            } else if d.t >= g.k.doze.nod_secs {
                d.phase = Phase::Asleep;
                d.t = 0.0;
                d.lit = -1.0;
                d.ask = js::rem(g.ch[c].id, 4.0) / 4.0 * g.k.doze.ask_secs;
                flush(g, gd, d);
                g.stats.doze.slept += 1;
            }
            continue;
        }
        if d.phase == Phase::Work {
            g.ch[c].anim.slump = 0.3;
            if let Some(mon) = d.mon {
                if crate::sim_lightshome::fixture_broken(g, mon) || !crate::sim_lightshome::fixture_on(g, mon) || (d.t > 1.0 && crate::sim_lightshome::fixture_factor(g, mon) < 0.5) {
                    let at = g.tab.sim_lightshome_fixture[mon].pos;
                    wake(g, gd, WakeWhy::Light, Some(&at[..]));
                    continue;
                }
                let at = g.tab.sim_lightshome_fixture[mon].pos;
                crate::attention::attention_lend(g, gd, at, Some(crate::attention::AttentionLendO { ears: Some(1.0), noises: Some(1.0) }));
            }
            let bubble_up = match g.gd[gd].bubble.as_ref() { Some(b) => g.time - b.t < b.dur + 1.0, None => false };
            if d.say >= 0.0 && g.time >= d.say && !bubble_up {
                let lines = screen_lines();
                let n = lines.len();
                if g.m.doze.said.as_ref().map_or(true, |s| s.len() >= n) { g.m.doze.said = Some(Vec::new()); }
                let mut left: Vec<usize> = Vec::new();
                { let said = g.m.doze.said.as_ref().unwrap();
                  for i in 0..n { if !said.contains(&(i as f64)) { left.push(i); } } }
                let len = left.len() as f64;
                let k = left[js::rem(m::floor(crate::math_dice::sim_random(g) * len), len) as usize];
                g.m.doze.said.as_mut().unwrap().push(k as f64);
                let every = g.k.doze.say_every.clone();
                let t = g.time;
                d.say = t + span(g, &every);
                flush(g, gd, d);
                crate::game::game_say(g, gd, &lines[k], Some(false), Some(false), Some(false));
                g.stats.doze.lines += 1;
            }
            continue;
        }
        g.ch[c].anim.slump = 1.0;
        d.ask += dt;
        flush(g, gd, d);
        if d.ask >= g.k.doze.ask_secs {
            let head = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.15, 0.0]) };
            let l = face_light(g, gd, head);
            d.lit = if l < g.k.doze.light_at { -1.0 } else if d.lit < 0.0 { 0.0 } else { d.lit + d.ask };
            d.ask = 0.0;
            flush(g, gd, d);
            if d.lit >= g.k.doze.light_secs {
                let at = lamp_on(g, gd, head);
                wake(g, gd, WakeWhy::Light, Some(&at[..]));
                continue;
            }
        }
    }
}
/// where the man who shoved the chair stands: a player, else a guard, else the sleeper himself (doze.ts:265)
fn shover_at(g: &mut Game, ch: PropId, c: CharId) -> V3 {
    let who = crate::props::prop_last_who(g, ch);
    if let Some(pl) = crate::seats::man_by_id(g, who) { return g.ch[g.pl[pl].ch].pos; }
    for &x in g.guards.iter() {
        let xc = g.gd[x].ch;
        if g.ch[xc].id == who { return g.ch[xc].pos; }
    }
    g.ch[c].pos
}

/// the light on a sleeper's face that is news to him: everything but his own side's lamps and the room's screens (doze.ts faceLight)
pub fn face_light(g: &mut crate::game::Game, gd: GuardId, head: V3) -> f64 {
    let mut off: Vec<LightId> = Vec::new();
    for ti in 0..g.targets.len() {
        let t = g.targets[ti];
        if g.tab.sim_lightshome_fixture[t].group != "monitors" { continue; }
        let Some(items) = g.tab.sim_lightshome_fixture[t].area_lights.clone() else { continue };
        for al in items.iter() {
            if g.tab.light[al.light].enabled {
                g.tab.light[al.light].enabled = false;
                off.push(al.light);
            }
        }
    }
    let me = g.ch[g.gd[gd].ch].id;
    let team = g.gd[gd].team;
    let mut skip = move |g: &mut Game, owner: f64| -> bool { owner == me || own_side_owns(g, owner, team) };
    let est = crate::guards::light_estimate_at(g, head, Some(&mut skip), None, None, None);
    let v = crate::game::game_visibility_of(g, est);
    for l in off { g.tab.light[l].enabled = true; }
    v
}
/// a man of this team carries the lamp with this owner (doze.ts:307, 314)
fn own_side_owns(g: &Game, owner: f64, team: crate::teams::Team) -> bool {
    g.guards.iter().any(|&x| g.ch[g.gd[x].ch].id == owner && g.gd[x].team == team)
}
/// where the light on his face comes from: the nearest carried lamp of another side through clear air, else his own spot (doze.ts lampOn)
pub fn lamp_on(g: &mut crate::game::Game, gd: GuardId, head: V3) -> V3 {
    let mut best: Option<V3> = None;
    let mut bd = 14.0;
    let me = g.ch[g.gd[gd].ch].id;
    let team = g.gd[gd].team;
    let mut i = 0usize;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        if !g.tab.light[l].enabled || g.tab.light[l].intensity <= 0.0 { continue; }
        let owner = g.tab.light[l].owner;
        if !js::truthy(owner) || owner == me || own_side_owns(g, owner, team) { continue; }
        let pos = g.tab.light[l].pos;
        let d = v3::dist(pos, head);
        if d >= bd || d > g.tab.light[l].range || crate::scene_collision::segment_blocked(g, head, pos, None, None, None) { continue; }
        best = Some([pos[0], 0.0, pos[2]]);
        bd = d;
    }
    match best { Some(b) => b, None => g.ch[g.gd[gd].ch].pos }
}

/// a man on his way down into, in, or just up out of his chair is no contact for the furniture (doze.ts dozeInChair)
pub fn doze_in_chair(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(d) = g.m.doze.rec.get(gd).copied() else { return false };
    let Some(ch) = d.chair else { return false };
    if seated(d) || d.phase == Phase::Up { return true; }
    if d.phase != Phase::Going { return false; }
    let pos = g.ch[g.gd[gd].ch].pos;
    let (x, z) = (crate::props::prop_x(g, ch), crate::props::prop_z(g, ch));
    flat(&pos, x, z) < 1.2
}
/// is he asleep this frame (doze.ts dozeAsleep)
pub fn doze_asleep(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.doze.rec.get(gd).map_or(false, |d| d.phase == Phase::Asleep)
}
/// his torch is dark: sat down to it, or asleep (doze.ts dozeDark)
pub fn doze_dark(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.doze.rec.get(gd).map_or(false, |d| seated(*d))
}
/// what he is at in his chair (doze.ts dozeAt)
pub fn doze_at(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doze::DozeAt> {
    match g.m.doze.rec.get(gd).map(|d| d.phase) {
        Some(Phase::Asleep) => Some(DozeAt::Asleep),
        Some(Phase::Work) => Some(DozeAt::Screen),
        _ => None,
    }
}
/// his ears this frame: [noises, foot]; headphones on at his screen, whole for everybody else (doze.ts dozeEar)
pub fn doze_ear(g: &mut crate::game::Game, gd: GuardId) -> [f64; 2] {
    if g.m.doze.rec.get(gd).map_or(false, |d| d.phase == Phase::Work) {
        g.m.doze.ears = [g.k.doze.noises, g.k.doze.ears];
        return g.m.doze.ears;
    }
    whole()
}
/// was he woken out of a doze inside the last `secs` seconds, and by what (doze.ts dozeWoke)
pub fn doze_woke(g: &mut crate::game::Game, gd: GuardId, secs: Option<f64>) -> Option<crate::doze::DozeWhy> {
    let secs = secs.unwrap_or(6.0);
    let d = g.m.doze.rec.get(gd).copied()?;
    let why = d.why?;
    if why != DozeWhy::Time && d.woke_at >= 0.0 && g.time - d.woke_at < secs { Some(why) } else { None }
}
/// where he is in his breath, 0‥1, or −1 awake (doze.ts dozeBreath)
pub fn doze_breath(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.doze.rec.get(gd) {
        Some(d) if d.phase == Phase::Asleep => js::rem(d.t, g.k.doze.breath_secs) / g.k.doze.breath_secs,
        _ => -1.0,
    }
}
/// the HUD's mark over him (doze.ts dozeMark)
pub fn doze_mark(g: &mut crate::game::Game, gd: GuardId) -> String {
    if g.m.doze.rec.get(gd).map_or(false, |d| d.phase == Phase::Work) {
        return String::from(if g.gd[gd].awareness > 0.05 { "" } else { "♪" });
    }
    let b = doze_breath(g, gd);
    String::from(if b < 0.0 { "" } else if b < 1.0 / 3.0 { "z" } else if b < 2.0 / 3.0 { "zZ" } else { "zZz" })
}
/// what the rule has of a man (doze.ts dozeOf)
pub fn doze_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doze::DozeOf> {
    g.m.doze.rec.get(gd).map(|d| DozeOf { phase: d.phase, why: d.why, secs: d.t, wp: d.wp })
}
/// the panel's line: who is at what (doze.ts dozeSummary)
pub fn doze_summary(g: &mut crate::game::Game) -> String {
    let mut rows: Vec<String> = Vec::new();
    for (gd, d) in g.m.doze.rec.iter() {
        if d.phase == Phase::No { continue; }
        let what = match d.phase {
            Phase::Asleep => alloc::format!("asleep {} s", js::to_fixed(d.t, 0)),
            Phase::Work => alloc::format!("at his screen {} s", js::to_fixed(d.t, 0)),
            Phase::Spent | Phase::Stir => alloc::format!("woken ({})", d.why.map_or("null", |w| w.as_str())),
            Phase::Going => String::from("to his chair"),
            Phase::Down => String::from("sitting down"),
            _ => String::from("getting up"),
        };
        rows.push(alloc::format!("{} {}", g.gd[gd].callsign, what));
    }
    alloc::format!("dozing ▸ {}", if rows.is_empty() { String::from("nobody") } else { rows.join(" · ") })
}
/// everybody up, the records gone (doze.ts dozeReset)
pub fn doze_reset(g: &mut crate::game::Game, load: Option<bool>) {
    let load = load.unwrap_or(false);
    g.m.doze.said = None;
    if load { g.m.doze.left = None; } else { left_off(g); }
    let rows: Vec<(GuardId, Doze)> = g.m.doze.rec.iter().map(|(k, v)| (k, *v)).collect();
    for (gd, d) in rows {
        if !load { dark(g, gd); }
        if seated(d) { up(g, gd); }
        drop(g, gd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FNV-1a over the doubles' bits, with EXACT.on the TypeScript's own (see notes.md)
    fn fnv(v: &[[f64; 2]]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for p in v { for x in p { for b in x.to_bits().to_le_bytes() { h ^= b as u64; h = h.wrapping_mul(0x100000001b3); } } }
        h
    }
    #[test]
    fn pulls_match_the_typescript_bits() {
        let v = build_pulls(0.5);
        assert_eq!(v.len(), PULLS_LEN);
        assert_eq!(v[0], [0.0, 0.0]);
        assert_eq!(fnv(&v), PULLS_FNV);
    }
    #[test]
    fn a_new_record_holds_the_typescript_numbers() {
        let d = blank(Phase::No, 3.0, 7.5);
        assert_eq!((d.say, d.lit, d.woke_at, d.seen, d.wp), (-1.0, -1.0, -1.0, 7.5, 3.0));
        assert!(d.chair.is_none() && d.mon.is_none() && d.why.is_none() && !d.mon_was);
    }
    #[test]
    fn seated_is_down_asleep_work() {
        for (p, s) in [(Phase::Down, true), (Phase::Asleep, true), (Phase::Work, true), (Phase::No, false), (Phase::Going, false), (Phase::Stir, false), (Phase::Up, false), (Phase::Spent, false)] {
            assert_eq!(seated(blank(p, 0.0, 0.0)), s);
        }
    }
    const PULLS_LEN: usize = 317;
    const PULLS_FNV: u64 = 0x171863a9a6126e3d;
}
