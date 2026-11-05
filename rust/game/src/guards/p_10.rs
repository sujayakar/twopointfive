//! src/game/guards.ts lines 3293 to 3631
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Seconds a follower who has lost sight of his leader still trails his true position.   (guards.ts PARTNER_TRUST_SECS)
const PARTNER_TRUST_SECS: f64 = 3.0;
/// The character budget a caller gets without asking for more, Sam included.   (guards.ts MAX_CHARACTERS)
const MAX_CHARACTERS: f64 = 8.0;
/// Ids handed to units: clear of Sam's 1 and the staff's 10..15, inside the owner byte.   (guards.ts UNIT_ID0, UNIT_ID1)
const UNIT_ID0: f64 = 20.0;
const UNIT_ID1: f64 = 250.0;

/// A side's jacket.   (guards.ts TEAM_TINT)
fn team_tint(t: crate::teams::Team) -> V3 {
    match t {
        crate::teams::Team::Guard => [0.42, 0.36, 0.25],
        crate::teams::Team::Operative => [0.07, 0.09, 0.1],
        crate::teams::Team::Hostile => [0.36, 0.33, 0.24],
        crate::teams::Team::Player => [0.05, 0.06, 0.07],
        crate::teams::Team::Civilian => [0.75, 0.62, 0.08],
    }
}
/// A side's trousers.   (guards.ts TEAM_TINT2)
fn team_tint2(t: crate::teams::Team) -> V3 {
    match t {
        crate::teams::Team::Guard => [0.02, 0.02, 0.025],
        crate::teams::Team::Operative => [0.05, 0.07, 0.06],
        crate::teams::Team::Hostile => [0.09, 0.08, 0.05],
        crate::teams::Team::Player => [0.1, 0.32, 0.18],
        crate::teams::Team::Civilian => [0.05, 0.06, 0.1],
    }
}

// The skeleton's gaps, one seam each (notes.md, ## Skeleton).
/// `g.audio.play(name, pos, gain, { rate })`: the call is real; `opts` holds no numbers yet, so the rate does not reach the host (notes.md, ## Play).
fn seam_audio_play(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(pos), Some(gain), None);
}
fn seam_steady_lights(g: &mut Game) -> f64 { todo!("needs: a light's ttl behind LightId, for g.engine.lights.lights") }
fn seam_man_leaves(g: &mut Game, ch: CharId, gd: GuardId) { todo!("needs: ManCols of a body and a man for man_leaves") }

/// One more round for a side.   (guards.ts tickEngagement)
fn rounds_add(r: &mut crate::guards::EngagementRounds, t: crate::teams::Team) {
    let slot = match t {
        crate::teams::Team::Guard => &mut r.guard,
        crate::teams::Team::Operative => &mut r.operative,
        crate::teams::Team::Hostile => &mut r.hostile,
        crate::teams::Team::Player => &mut r.player,
        crate::teams::Team::Civilian => &mut r.civilian,
    };
    *slot = Some(slot.unwrap_or(0.0) + 1.0);
}
/// The record's copy of the rounds: a side that fired none stays absent.   (guards.ts endEngagement)
fn rounds_copy(r: &crate::guards::EngagementRounds) -> crate::guards::EngageResultRounds {
    crate::guards::EngageResultRounds { guard: r.guard, operative: r.operative, hostile: r.hostile, player: r.player, civilian: r.civilian }
}

/// One level down: lockdown to a shorter heightened spell, then calm.   (guards.ts stepDownEscalation)
pub fn step_down_escalation(g: &mut crate::game::Game) {
    let lvl: f64 = if g.escalation >= 2.0 { 1.0 } else { 0.0 };
    g.escalation = lvl;
    g.escalation_t = if lvl >= 1.0 { g.time + crate::guards::STEPDOWN_SECS } else { -1.0 };
    crate::chronicle::chron_alarm(g, lvl, false, Some("time"));
    if lvl < 2.0 { clear_lockdown(g); g.cleared_at.clear(); }
    if lvl == 0.0 { g.alarm.placed = false; }
    let at = if g.alarm.placed { Some(g.alarm.pos) } else { None };
    let sp = spokesman(g, at, None);
    if let Some(sp) = sp {
        crate::game::game_say(g, sp, if lvl >= 1.0 { "ease off — singles, back on your routes; weapons stay out" } else { "stand down, back to normal" }, Some(true), None, None);
        let c = g.gd[sp].ch;
        let pos = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => g.ch[c].pos };
        seam_audio_play(g, "radio", pos, 0.6, 0.95);
    }
}

/// Once a frame after the guards: the alarm episode, stepping down, the lockdown duties.   (guards.ts updateEscalation)
pub fn update_escalation(g: &mut crate::game::Game, dt: f64) {
    update_rollcall(g, dt);
    crate::dialogue::update_ambient(g, dt);
    tick_engagement(g);
    if g.quiet_utility { return; }
    let (mut any_alert, mut hunting, mut living) = (false, false, 0.0_f64);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].team != crate::teams::Team::Guard { continue; }
        living += 1.0;
        if g.gd[gd].state == crate::game::GuardState::Alert {
            any_alert = true;
            hunting = true;
            if let Some(lk) = g.gd[gd].last_known { g.alarm.pos[0] = lk[0]; g.alarm.pos[1] = 0.0; g.alarm.pos[2] = lk[2]; g.alarm.placed = true; }
        } else if g.gd[gd].state == crate::game::GuardState::Search { hunting = true; }
    }
    if any_alert {
        if g.ai_enabled { g.alarm.episode = true; }
    } else if g.alarm.episode && !hunting {
        g.alarm.episode = false;
        if living > 0.0 && !g.pl[g.player].down && g.ai_enabled {
            let to = if g.escalation >= 1.0 { 2.0 } else { 1.0 };
            raise_escalation(g, to, None, None, None);
        }
    }
    if g.escalation > 0.0 && !g.alarm.episode && !hunting && !still_looking(g) && g.time >= g.escalation_t { step_down_escalation(g); }
    if g.escalation >= 2.0 { supervise_lockdown(g, dt, living); }
    if g.clearing.is_some() && !crate::squad::clearing_tick(g, dt) { g.clearing = None; }
}

/// A man the floor knows is dead.   (guards.ts superviseLockdown knownDead)
fn known_dead(g: &mut Game, x: GuardId) -> bool {
    g.gd[x].state == crate::game::GuardState::Dead && (g.gd[x].found || g.alarm.episode || g.gd[x].missed_at >= 0.0)
}

/// Keep the lockdown duties dealt, as far as the men know.   (guards.ts superviseLockdown)
pub fn supervise_lockdown(g: &mut crate::game::Game, dt: f64, living: f64) {
    let (mut follower, mut postman): (Option<GuardId>, Option<GuardId>) = (None, None);
    let mut standing = 0_i32;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].team != crate::teams::Team::Guard || g.gd[gd].unit.is_some() { continue; }
        if g.gd[gd].held.is_some() {
            if g.gd[gd].leader.is_some() || g.gd[gd].post.is_some() { g.gd[gd].leader = None; g.gd[gd].post = None; }
            continue;
        }
        if known_dead(g, gd) { continue; }
        standing += 1;
        if g.gd[gd].leader.is_some() { follower = Some(gd); }
        if g.gd[gd].post.is_some() { postman = Some(gd); }
    }
    let posts = g.level.chokepoints.as_ref().map_or(0, |c| c.len()) > 0;
    let mut redo = if standing >= 2 {
        follower.is_none()
            || known_dead(g, g.gd[follower.unwrap()].leader.unwrap())
            || (standing >= 3 && living >= 3.0 && posts && postman.is_none())
    } else {
        follower.is_some() || postman.is_some()
    };
    if let Some(fo) = follower {
        if !redo && g.gd[fo].state != crate::game::GuardState::Dead {
            let l = g.gd[fo].leader.unwrap();
            let calm = g.gd[fo].state == crate::game::GuardState::Patrol && g.gd[l].state == crate::game::GuardState::Patrol;
            let (fc, lc) = (g.gd[fo].ch, g.gd[l].ch);
            g.alarm.apart_t = if calm && v3::dist(g.ch[fc].pos, g.ch[lc].pos) > 8.0 { g.alarm.apart_t + dt } else { 0.0 };
            if g.alarm.apart_t > 5.0 { redo = true; }
        }
    }
    if redo && living >= 1.0 { assign_lockdown(g, true, None); }
}

/// The same chokepoint: the TypeScript's `===` of two rows of the level's list; a man's post is a copy here.   (guards.ts assignLockdown)
fn same_post(a: &crate::scene_level::Chokepoint, b: &crate::scene_level::Chokepoint) -> bool {
    a.name == b.name && a.pos == b.pos && a.yaw == b.yaw
}

/// Deal the lockdown duties: the nearest two pair up, a third holds the junction nearest the fix.   (guards.ts assignLockdown)
pub fn assign_lockdown(g: &mut crate::game::Game, redeal: bool, except: Option<GuardId>) {
    let (mut old_lead, mut old_follow, mut old_post_man): (Option<GuardId>, Option<GuardId>, Option<GuardId>) = (None, None, None);
    let mut old_post: Option<crate::scene_level::Chokepoint> = None;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        if let Some(l) = g.gd[gd].leader { old_follow = Some(gd); old_lead = Some(l); }
        if g.gd[gd].post.is_some() { old_post = g.gd[gd].post.clone(); old_post_man = Some(gd); }
    }
    clear_lockdown(g);
    g.alarm.apart_t = 0.0;
    let mut alive: Vec<GuardId> = Vec::new();
    for &x in g.guards.iter() {
        let m_ = &g.gd[x];
        if m_.state != crate::game::GuardState::Dead && m_.held.is_none() && m_.team == crate::teams::Team::Guard && m_.unit.is_none() { alive.push(x); }
    }
    if alive.len() < 2 { return; }
    let (mut a, mut b, mut bd) = (alive[0], alive[1], m::INF);
    let clearing: Option<(GuardId, Option<GuardId>)> = g.clearing.as_ref().map(|c| (c.a, c.b));
    let kept = match clearing {
        Some((ca, Some(cb))) if alive.contains(&ca) && alive.contains(&cb) => Some((ca, cb)),
        _ => None,
    };
    if let Some((ca, cb)) = kept {
        a = ca;
        b = cb;
    } else {
        for i in 0..alive.len() {
            for j in (i + 1)..alive.len() {
                let (ci, cj) = (g.gd[alive[i]].ch, g.gd[alive[j]].ch);
                let d = v3::dist(g.ch[ci].pos, g.ch[cj].pos);
                if d < bd { bd = d; a = alive[i]; b = alive[j]; }
            }
        }
    }
    let lead = if clearing.map_or(false, |(ca, _)| a == ca) { a } else if route_length(g, a) >= route_length(g, b) { a } else { b };
    let follow = if lead == a { b } else { a };
    let lead_c = g.gd[lead].ch;
    let lead_pos = g.ch[lead_c].pos;
    let now = g.time;
    {
        let f = &mut g.gd[follow];
        f.leader = Some(lead);
        f.follow_t = 0.0;
        f.leader_still_t = 0.0;
        f.path = Vec::new();
        f.path_goal = None;
        f.leader_seen_t = now;
        f.leader_seen_pos = v3::copy(lead_pos);
    }
    if redeal
        && (Some(lead) != old_lead || Some(follow) != old_follow)
        && Some(lead) != except
        && g.gd[lead].state != crate::game::GuardState::Alert
        && !g.gd[lead].bubble.as_ref().map_or(false, |bu| now - bu.t < 0.5)
    {
        crate::game::game_say(g, lead, "you're with me — close up", None, None, None);
    }
    let third = alive.iter().copied().find(|&x| x != a && x != b);
    let cps: Vec<crate::scene_level::Chokepoint> = g.level.chokepoints.clone().unwrap_or_default();
    if let Some(third) = third {
        if !cps.is_empty() {
            let third_c = g.gd[third].ch;
            let at = if g.alarm.placed { g.alarm.pos } else { g.ch[third_c].pos };
            let (mut bi, mut cd) = (0, m::INF);
            for (k, cp) in cps.iter().enumerate() {
                let d = v3::dist(cp.pos, at);
                if d < cd { cd = d; bi = k; }
            }
            let best = cps[bi].clone();
            {
                let t = &mut g.gd[third];
                t.post = Some(best.clone());
                t.follow_t = 0.0;
                t.path = Vec::new();
                t.path_goal = None;
            }
            if (old_post.as_ref().map_or(true, |o| !same_post(o, &best)) || Some(third) != old_post_man)
                && Some(third) != except
                && g.gd[third].state != crate::game::GuardState::Alert
            {
                crate::game::game_say(g, third, &format!("holding the {}", best.name), Some(true), None, None);
            }
        }
    }
}

/// Take every man off his lockdown duty.   (guards.ts clearLockdown)
pub fn clear_lockdown(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].leader.is_some() || g.gd[gd].post.is_some() { g.gd[gd].path = Vec::new(); g.gd[gd].path_goal = None; }
        g.gd[gd].leader = None;
        g.gd[gd].post = None;
    }
}

/// How long a man's route is, round the loop.   (guards.ts routeLength)
pub fn route_length(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let Some(&r) = g.level.routes.get(g.gd[gd].route_i) else { return 0.0; };
    let pts = &g.tab.route[r].points;
    let mut s = 0.0;
    for i in 0..pts.len() { s += v3::dist(pts[i], pts[(i + 1) % pts.len()]); }
    s
}

/// Lockdown follower on patrol: keep station behind-left of the leader as he knows him. True while walking.   (guards.ts followLeader)
pub fn follow_leader(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool {
    let l = g.gd[gd].leader.unwrap();
    let (c, lc) = (g.gd[gd].ch, g.gd[l].ch);
    let fresh = g.gd[gd].leader_seen_t < 0.0 || g.time - g.gd[gd].leader_seen_t < PARTNER_TRUST_SECS;
    let lp = if fresh { g.ch[lc].pos } else { g.gd[gd].leader_seen_pos };
    g.gd[gd].follow_t -= dt;
    let lvel = g.ch[lc].vel;
    g.gd[gd].leader_still_t = if !fresh || m::hyp2(lvel[0], lvel[2]) < 0.15 { g.gd[gd].leader_still_t + dt } else { 0.0 };
    if g.gd[gd].follow_t <= 0.0 {
        g.gd[gd].follow_t = 0.5;
        let f = crate::character::character_forward(g, lc);
        let lv: V3 = if fresh { g.ch[lc].vel } else { [0.0, 0.0, 0.0] };
        let mut goal = g.gd[gd].follow_goal;
        goal[0] = lp[0] - f[0] * 1.3 + f[2] * 0.45;
        goal[1] = 0.0;
        goal[2] = lp[2] - f[2] * 1.3 - f[0] * 0.45;
        if crate::scene_collision::nav_grid_is_blocked(g, goal[0], goal[2]) || !crate::scene_collision::nav_grid_walkable(g, lp, goal) {
            goal[0] = lp[0] - f[0] * 1.2;
            goal[2] = lp[2] - f[2] * 1.2;
        }
        if crate::scene_collision::nav_grid_is_blocked(g, goal[0], goal[2]) || !crate::scene_collision::nav_grid_walkable(g, lp, goal) {
            goal[0] = lp[0];
            goal[2] = lp[2];
        } else if !crate::scene_collision::nav_grid_is_blocked(g, goal[0] + lv[0] * 0.6, goal[2] + lv[2] * 0.6) {
            goal[0] += lv[0] * 0.6;
            goal[2] += lv[2] * 0.6;
        }
        g.gd[gd].follow_goal = goal;
        let p = g.ch[c].pos;
        let off = m::hyp2(goal[0] - p[0], goal[2] - p[2]);
        let stale = match g.gd[gd].path_goal {
            None => true,
            Some(pg) => m::hyp2(pg[0] - goal[0], pg[2] - goal[2]) > 0.5 || (g.gd[gd].path_i >= g.gd[gd].path.len() && off > 0.7),
        };
        if stale && off > 0.35 { go_to(g, gd, goal, Some(true)); }
    }
    let p = g.ch[c].pos;
    let gap = m::hyp2(lp[0] - p[0], lp[2] - p[2]);
    let speed = if gap > 4.5 { m::max(walk, g.tune.guard_investigate * 1.1) } else if gap > 2.0 { walk * 1.3 } else { walk };
    let arrived = follow_path(g, gd, dt, speed);
    if arrived {
        let look_at = g.gd[gd].leader_still_t > 6.0 && gap < 2.2;
        let lp = if fresh { g.ch[lc].pos } else { lp };
        let p = g.ch[c].pos;
        let want = if look_at { m::atan2(lp[0] - p[0], lp[2] - p[2]) } else { g.ch[lc].body_yaw + 0.45 };
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, want, 3.0 * dt);
        let phase = g.gd[gd].look_phase;
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.6) * (if look_at { 0.25 } else { 0.6 }), 2.0, dt);
        g.ch[c].anim.look_yaw_extra = m::sin(phase * 1.1) * (if look_at { 0.15 } else { 0.4 });
    } else {
        let phase = g.gd[gd].look_phase;
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.7) * 0.3, 2.0, dt);
    }
    !arrived
}

/// Lockdown post on patrol: walk to the junction, then stand on it sweeping. True while walking.   (guards.ts holdPost)
pub fn hold_post(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool {
    let (ppos, pyaw) = { let p_ = g.gd[gd].post.as_ref().unwrap(); (p_.pos, p_.yaw) };
    let c = g.gd[gd].ch;
    g.gd[gd].follow_t -= dt;
    let p = g.ch[c].pos;
    let replan = g.gd[gd].follow_t <= 0.0
        && m::hyp2(ppos[0] - p[0], ppos[2] - p[2]) > 0.45
        && match g.gd[gd].path_goal {
            None => true,
            Some(pg) => g.gd[gd].path_i >= g.gd[gd].path.len() || m::hyp2(pg[0] - ppos[0], pg[2] - ppos[2]) > 0.5,
        };
    if replan {
        g.gd[gd].follow_t = 0.5;
        go_to(g, gd, ppos, Some(true));
        let push = match g.gd[gd].path.last() {
            None => true,
            Some(last) => m::hyp2(last[0] - ppos[0], last[2] - ppos[2]) > 0.02,
        };
        if push { g.gd[gd].path.push(v3::copy(ppos)); }
    }
    let arrived = follow_path(g, gd, dt, walk);
    let phase = g.gd[gd].look_phase;
    if arrived {
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, pyaw, 2.5 * dt);
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.45) * 0.75, 2.0, dt);
        g.ch[c].anim.look_yaw_extra = m::sin(phase * 0.9) * 0.45;
    } else {
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.7) * 0.3, 2.0, dt);
    }
    !arrived
}

/// Housekeeping on a calm pass, one door a frame: pull a leaf to, throw a bolt again.   (guards.ts lockdownDoors)
pub fn lockdown_doors(g: &mut crate::game::Game, gd: GuardId, calm: Option<bool>) {
    let calm = calm.unwrap_or(false);
    let c = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    let mut di = 0;
    while di < g.doors.list.len() {
        let d = g.doors.list[di];
        di += 1;
        let (def_locked, def_exterior, def_closer) = { let def = &g.dr[d].def; (def.locked.unwrap_or(false), def.exterior.unwrap_or(false), def.closer) };
        if crate::doors::door_off(g, d) { continue; }
        let barred = if calm {
            !(def_locked || def_exterior) || def_closer == Some(false)
        } else {
            def_exterior && !(crate::doors::doors_stay(g) && def_closer != Some(false))
        };
        if barred || g.time - crate::doors::door_guard_t(g, d) < 12.0 || crate::doors::door_quiet_t(g, d) < 0.6 { continue; }
        let angle = crate::doors::door_angle(g, d);
        let open = !crate::doors::door_latched(g, d) && (calm || m::abs(angle) > 0.03);
        let relock = !calm && def_locked && !crate::doors::door_locked(g, d) && !crate::doors::door_lock_broken(g, d);
        if !open && !relock { continue; }
        let fc = g.dr[d].frame_centre;
        let p = g.ch[c].pos;
        let (dx, dz) = (fc[0] - p[0], fc[2] - p[2]);
        let dd = m::hyp2(dx, dz);
        if dd > 2.0 || dd < 0.9 || (dx * f[0] + dz * f[2]) / dd > 0.55 { continue; }
        if open {
            let (hinge, cd, width) = { let def = &g.dr[d].def; (def.hinge, def.closed_dir, def.width) };
            let (px, pz) = (p[0] - hinge[0], p[2] - hinge[1]);
            let th = m::atan2(-m::sin(cd) * px + m::cos(cd) * pz, m::cos(cd) * px + m::sin(cd) * pz);
            if th * angle > 0.0 && m::abs(th) < m::abs(angle) + 0.4 && m::hyp2(px, pz) < width + 0.45 { continue; }
        }
        let mut busy = false;
        for &o in g.guards.iter() {
            if o == gd || g.gd[o].state == crate::game::GuardState::Dead { continue; }
            let oc = g.gd[o].ch;
            let op = g.ch[oc].pos;
            if m::hyp2(op[0] - fc[0], op[2] - fc[2]) < 2.0 { busy = true; break; }
        }
        if busy { continue; }
        let now = g.time;
        crate::doors::door_set_guard_t(g, d, now);
        if open { let who = g.ch[c].id; crate::doors::doors_pull_to(g, d, who); }
        if relock {
            let line = if crate::doors::door_noticed(g, d) { "locking this back up" } else if open { "shutting this — and it stays locked" } else { "someone left this unlocked — locking it" };
            crate::game::game_say(g, gd, line, None, None, None);
            crate::doors::door_set_locked(g, d, true);
            crate::doors::door_set_noticed(g, d, true);
            let now = g.time;
            crate::doors::door_set_noticed_t(g, d, now);
            let at = crate::doors::door_pos(g, d);
            seam_audio_play(g, "lockOpen", at, 0.6, 0.85);
        }
        break;
    }
}

/// 0..1 tension for the score.   (guards.ts tension)
pub fn tension(g: &mut crate::game::Game, theirs: Option<bool>) -> f64 {
    let theirs = theirs.unwrap_or(false);
    let black = if crate::sim_lightshome::mains_active(g) { 0.18 } else { 0.0 };
    let esc = if escalation_of(g) >= 2.0 { 0.22 } else if escalation_of(g) >= 1.0 { 0.12 } else { 0.0 };
    let mut x = m::max(black, esc);
    let live = theirs && !(g.touring || crate::director::director_showing(g));
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let state = g.gd[gd].state;
        if state == crate::game::GuardState::Dead || (live && g.gd[gd].team == crate::teams::Team::Operative && state != crate::game::GuardState::Alert) { continue; }
        let clearing = g.gd[gd].script.is_some() && g.clearing.as_ref().map_or(false, |c| gd == c.a || Some(gd) == c.b);
        let s = if clearing { 0.3 }
            else if state == crate::game::GuardState::Alert { 1.0 }
            else if state == crate::game::GuardState::Search { 0.68 }
            else if state == crate::game::GuardState::Suspicious { 0.45 + g.gd[gd].awareness * 0.2 }
            else if crate::condition::condition_calm(g, gd) { 0.0 }
            else { g.gd[gd].awareness * 0.35 };
        x = m::max(x, s);
    }
    x
}

/// Stand an AI man of a side, held on a one-point route of his own. A spent budget is a panic: the TypeScript's throw.   (guards.ts spawnUnit)
pub fn spawn_unit(g: &mut crate::game::Game, o: &crate::guards::UnitOpts) -> GuardId {
    if !g.tune.teams { panic!("spawnUnit: sides are switched off (Game.tune.teams) — no unit may be spawned"); }
    if o.team == crate::teams::Team::Player { panic!("spawnUnit: the 'player' side is the player alone — spawn an 'operative' for a man on his side"); }
    if o.team == crate::teams::Team::Civilian { panic!("spawnUnit: civilians are not units — civilians.ts stands them up from the level's civilians block (Game.civilians)"); }
    let chars = 1.0 + g.guards.len() as f64;
    let cap = m::max(MAX_CHARACTERS, m::min(o.budget.unwrap_or(0.0), crate::guards::MAX_SKINNED));
    if chars + 1.0 > cap {
        if cap > MAX_CHARACTERS {
            panic!("spawnUnit: {} characters already stand (the player + {} guards/units) and this caller's budget is {} — despawn a unit or reset the guards first", chars, g.guards.len(), cap);
        }
        panic!("spawnUnit: {} characters already stand (the player + {} guards/units) and the shadow-capsule budget is {} (scene/boxes.ts BoxWorld.addCharacterCapsules) — despawn a unit or reset the guards first", chars, g.guards.len(), MAX_CHARACTERS);
    }
    let steady = seam_steady_lights(g);
    if steady + 1.0 > crate::render_lights::MAX_LIGHTS {
        panic!("spawnUnit: {} steady lights already exist and the light buffer holds {} (render/lights.ts) — his weapon light would never upload; despawn a unit first", steady, crate::render_lights::MAX_LIGHTS);
    }
    let mg = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    let mut used: Vec<f64> = Vec::new();
    for pl in crate::seats::men(g, &mg) { let c = g.pl[pl].ch; used.push(g.ch[c].id); }
    for &x in g.guards.iter() { let c = g.gd[x].ch; used.push(g.ch[c].id); }
    let mut id = UNIT_ID0;
    while used.contains(&id) && id <= UNIT_ID1 { id += 1.0; }
    if id > UNIT_ID1 { panic!("spawnUnit: no character id left in {}‥{} (the owner byte) — despawn units first", UNIT_ID0, UNIT_ID1); }
    g.m.guards.unit_serial += 1.0;
    let n = g.m.guards.unit_serial;
    let name: String = match &o.name { Some(s) => s.clone(), None => format!("{}-{}", o.team.as_str(), n) };
    let weapon = if o.carbine_rig.unwrap_or(false) {
        crate::weapons::WeaponId::Aks74u
    } else {
        match o.weapon {
            Some(w) => w,
            None => match o.team {
                crate::teams::Team::Operative => crate::weapons::WeaponId::Fn2000,
                crate::teams::Team::Hostile => crate::weapons::WeaponId::Aks74u,
                _ => crate::weapons::WeaponId::Guard9mm,
            },
        }
    };
    let route = g.tab.route.push(crate::scene_level::PatrolRoute {
        name: name.clone(),
        points: vec![[o.pos[0], 0.0, o.pos[2]]],
        wait: vec![0.0],
        weapon: Some(weapon),
        keys: Some(Vec::new()),
    });
    let def = crate::rig_props::torch_light_def(&format!("unitlight{}", id), id, g.tune.flash_intensity, g.tune.flash_inner, g.tune.flash_outer);
    let fl = crate::render_lights::light_set_add(&mut g.engine.lights, def);
    let look = crate::game::GuardNewLook {
        tint2: Some(match o.tint2 { Some(t) => t, None => team_tint2(o.team) }),
        yaw: Some(o.yaw.unwrap_or(0.0)),
        name: Some(name.clone()),
        team: Some(o.team),
    };
    let rig = g.rig;
    let tint = match o.tint { Some(t) => t, None => team_tint(o.team) };
    let gd = crate::game::guard_new(g, crate::Opaque, rig, id, route, -1.0, fl, tint, Some(&look));
    let now = g.time;
    g.gd[gd].unit = Some(crate::game::UnitInfo { name, spawned_at: now, light: o.light != Some(false), pair: o.pair, seat: None });
    g.gd[gd].hold = true;
    g.gd[gd].heard_up_to = g.event_seq;
    let c = g.gd[gd].ch;
    crate::character::character_update(g, c, 0.0);
    g.guards.push(gd);
    gd
}

/// What of a unit lives outside his row: his weapon light, his meter keys.   (guards.ts releaseUnit)
pub fn release_unit(g: &mut crate::game::Game, gd: GuardId) {
    let fl = g.gd[gd].flashlight;
    crate::render_lights::light_set_remove(&mut g.engine.lights, fl);
    let c = g.gd[gd].ch;
    let id = g.ch[c].id;
    crate::game::game_light_forget(g, &format!("unitC{}", id));
    crate::game::game_light_forget(g, &format!("unitH{}", id));
}

/// Take a spawned man back out, and every hand on him. True if he was one.   (guards.ts despawnUnit)
pub fn despawn_unit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].unit.is_none() { return false; }
    let Some(i) = g.guards.iter().position(|&x| x == gd) else { return false; };
    if g.pl[g.player].holding.as_ref().map(|h| h.g) == Some(gd) { crate::game::game_free_held(g, crate::game::GameFreeHeldWhy::Cancel); }
    if g.pl[g.player].dragging == Some(gd) { crate::game::game_toggle_drag(g, None); }
    if g.pl[g.player].takedown.as_ref().map(|t| t.g) == Some(gd) { g.pl[g.player].takedown = None; }
    release_unit(g, gd);
    g.guards.remove(i);
    let at = i as f64;
    if g.spectate == at { g.spectate = -1.0; } else if g.spectate > at { g.spectate -= 1.0; }
    if g.aim_guard == Some(gd) { g.aim_guard = None; }
    let mut k = 0;
    while k < g.guards.len() {
        let o = g.guards[k];
        k += 1;
        if matches!(g.gd[o].foe, Some(crate::game::Foe::Guard(x)) if x == gd) {
            g.gd[o].foe = None;
            crate::aim::aim_state_clear(&mut g.gd[o].aim);
        }
        if let Some(s) = g.gd[o].standoff { if g.tab.guards_standoff[s].pair == gd { g.gd[o].standoff = None; } }
        if g.gd[o].body_duty.as_ref().map_or(false, |b| b.body == gd) { g.gd[o].body_duty = None; }
        if g.gd[o].leader == Some(gd) { g.gd[o].leader = None; }
        if matches!(&g.gd[o].task, Some(crate::game::GuardTask::CheckOn(t)) if t.who == gd) { g.gd[o].task = None; }
    }
    let c = g.gd[gd].ch;
    seam_man_leaves(g, c, gd);
    true
}

/// Take every spawned man out.   (guards.ts despawnUnits)
pub fn despawn_units(g: &mut crate::game::Game) {
    for gd in g.guards.clone() {
        if g.gd[gd].unit.is_some() { despawn_unit(g, gd); }
    }
}

/// Every spawned unit standing or lying in the game.   (guards.ts units)
pub fn units(g: &mut crate::game::Game) -> Vec<GuardId> {
    g.guards.iter().copied().filter(|&x| g.gd[x].unit.is_some()).collect()
}

/// The engagement on this game, done or not. A copy: re-ask for it to see it move.   (guards.ts engagementOf)
pub fn engagement_of(g: &mut crate::game::Game) -> Option<crate::guards::Engagement> {
    g.m.guards.engagements.clone()
}

/// Arm one man for the fight against `foes`.   (guards.ts armForFight)
pub fn arm_for_fight(g: &mut crate::game::Game, gd: GuardId, foes: &[GuardId], pin: bool, delay: Option<f64>) {
    let delay = delay.unwrap_or(0.0);
    if g.gd[gd].state == crate::game::GuardState::Dead { return; }
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert)); }
    {
        let m_ = &mut g.gd[gd];
        m_.hold = false;
        m_.pinned = pin;
        m_.state = crate::game::GuardState::Alert;
        m_.awareness = 1.0;
        m_.search_t = 0.0;
        m_.body_duty = None;
        m_.search_plan = None;
        m_.standoff = None;
    }
    let r = crate::math_dice::sim_random(g);
    g.gd[gd].react_t = 0.15 + r * 0.35 + delay;
    let r = crate::math_dice::sim_random(g);
    g.gd[gd].fire_cd = 0.4 + r * 0.4;
    g.gd[gd].reload_t = -1.0;
    reset_chase(g, gd);
    g.gd[gd].witness.alerted_by = Some(crate::game::WitnessAlertedBy::Radio);
    g.gd[gd].witness.alert_t = g.time;
    let (mut best, mut best_rate, mut seen): (Option<GuardId>, f64, bool) = (None, -1.0, false);
    for &f in foes {
        let fc = g.gd[f].ch;
        if g.gd[f].state == crate::game::GuardState::Dead || !g.ch[fc].alive { continue; }
        let vis = can_see_char(g, gd, fc, false);
        let rate = if vis.visible {
            1.0 + sight_gain_on(g, gd, f, vis.dist)
        } else {
            let mc = g.gd[gd].ch;
            1.0 / (1.0 + v3::dist_xz(g.ch[mc].pos, g.ch[fc].pos))
        };
        if rate > best_rate { best_rate = rate; best = Some(f); seen = vis.visible; }
    }
    g.gd[gd].foe = best.map(crate::game::Foe::Guard);
    crate::aim::aim_state_clear(&mut g.gd[gd].aim);
    if let Some(b) = best {
        let bc = g.gd[b].ch;
        let bp = g.ch[bc].pos;
        let now = g.time;
        let m_ = &mut g.gd[gd];
        m_.last_known = Some(v3::copy(bp));
        if seen { m_.last_seen_t = now; m_.sight_pos = v3::copy(bp); m_.sight_t = now; m_.led_t = now; }
    }
}

/// Put both groups in the fight now. The answer is a copy: `engagement_of` to see it move.   (guards.ts engage)
pub fn engage(g: &mut crate::game::Game, operatives: &[GuardId], hostiles: &[GuardId], opts: Option<crate::guards::EngageOpts>) -> crate::guards::Engagement {
    let opts = opts.unwrap_or_default();
    end_engagement(g, crate::guards::EndEngagementOutcome::Superseded);
    let pin = opts.pin.unwrap_or(false);
    for &gd in operatives { arm_for_fight(g, gd, hostiles, pin, None); }
    for &gd in hostiles { arm_for_fight(g, gd, operatives, pin, Some(m::max(0.0, opts.surprise.unwrap_or(0.0)))); }
    let mut alive: JsSet<GuardId> = JsSet::new();
    for &x in operatives.iter().chain(hostiles.iter()) {
        if g.gd[x].state != crate::game::GuardState::Dead { alive.add(x); }
    }
    let e = crate::guards::Engagement {
        operatives: operatives.to_vec(),
        hostiles: hostiles.to_vec(),
        t0: g.time,
        timeout: opts.timeout.unwrap_or(45.0),
        done: false,
        result: None,
        kills: Vec::new(),
        rounds: crate::guards::EngagementRounds::zeroed(),
        alive,
    };
    g.m.guards.engagements = Some(e.clone());
    e
}

/// Name of the character with this id, for the record.   (guards.ts nameOfId)
pub fn name_of_id(g: &mut crate::game::Game, id_2: f64) -> String {
    if let Some(pl) = crate::seats::man_by_id(g, id_2) { return crate::seats::player_name(g, pl); }
    for &x in g.guards.iter() {
        let c = g.gd[x].ch;
        if g.ch[c].id == id_2 { return g.gd[x].callsign.clone(); }
    }
    if id_2 < 0.0 { String::from("—") } else { format!("#{}", id_2) }
}

/// A list's men still in the fight: alive, on the list, not fled.   (guards.ts endEngagement living)
fn living_count(g: &mut Game, xs: &[GuardId]) -> f64 {
    let mut n = 0.0;
    for &x in xs {
        if g.gd[x].state != crate::game::GuardState::Dead && g.guards.contains(&x) && !crate::runner::runner_fled(g, x) { n += 1.0; }
    }
    n
}

/// Close the running engagement with `outcome`.   (guards.ts endEngagement)
pub fn end_engagement(g: &mut crate::game::Game, outcome: crate::guards::EndEngagementOutcome) {
    let (ops, hos, t0, kills, rounds) = match g.m.guards.engagements.as_mut() {
        None => return,
        Some(e) if e.done => return,
        Some(e) => {
            e.done = true;
            (e.operatives.clone(), e.hostiles.clone(), e.t0, e.kills.clone(), rounds_copy(&e.rounds))
        }
    };
    let result_outcome = match outcome {
        crate::guards::EndEngagementOutcome::Wiped => crate::guards::EngageResultOutcome::Wiped,
        crate::guards::EndEngagementOutcome::Reset => crate::guards::EngageResultOutcome::Reset,
        crate::guards::EndEngagementOutcome::Cleared => crate::guards::EngageResultOutcome::Cleared,
        crate::guards::EndEngagementOutcome::Timeout => crate::guards::EngageResultOutcome::Timeout,
        crate::guards::EndEngagementOutcome::Superseded => crate::guards::EngageResultOutcome::Superseded,
    };
    let secs = g.time - t0;
    let operatives_left = living_count(g, &ops);
    let hostiles_left = living_count(g, &hos);
    if let Some(e) = g.m.guards.engagements.as_mut() {
        e.result = Some(crate::guards::EngageResult { outcome: result_outcome, secs, kills, rounds, operatives_left, hostiles_left });
    }
}

/// Is anybody of this list still in the fight?   (guards.ts tickEngagement living)
fn living_any(g: &mut Game, xs: &[GuardId]) -> bool {
    for &x in xs {
        if g.gd[x].state != crate::game::GuardState::Dead && g.guards.contains(&x) && !crate::runner::runner_fled(g, x) { return true; }
    }
    false
}

/// Once a frame: book this frame's rounds and deaths into the engagement, end it when decided.   (guards.ts tickEngagement)
pub fn tick_engagement(g: &mut crate::game::Game) {
    let (t0, timeout, ops, hos, alive) = match g.m.guards.engagements.as_ref() {
        None => return,
        Some(e) if e.done => return,
        Some(e) => (e.t0, e.timeout, e.operatives.clone(), e.hostiles.clone(), e.alive.keys()),
    };
    let mut i = 0;
    while i < g.events.len() {
        let (time, kind, level, who) = { let e = &g.events[i]; (e.time, e.kind, e.level, e.who) };
        i += 1;
        if time == g.time && (kind == crate::director::EventKind::Shot || kind == crate::director::EventKind::GuardShot) && level.unwrap_or(0.0) >= 1.0 {
            let t = if kind == crate::director::EventKind::Shot { crate::teams::Team::Player } else { crate::teams::team_of_id(g, who.unwrap_or(-1.0)) };
            if let Some(e) = g.m.guards.engagements.as_mut() { rounds_add(&mut e.rounds, t); }
        }
    }
    let mut gone = false;
    for x in alive {
        if !g.guards.contains(&x) {
            gone = true;
            if let Some(e) = g.m.guards.engagements.as_mut() { e.alive.delete(x); }
            continue;
        }
        if g.gd[x].state == crate::game::GuardState::Dead {
            if let Some(e) = g.m.guards.engagements.as_mut() { e.alive.delete(x); }
            let (victim, team, killer, down_by) = { let m_ = &g.gd[x]; (m_.callsign.clone(), m_.team, m_.killer_id, m_.down_by) };
            let by = if killer >= 0.0 {
                name_of_id(g, killer)
            } else {
                match down_by { Some(d) => String::from(d.as_str()), None => String::from("—") }
            };
            let t = g.time - t0;
            if let Some(e) = g.m.guards.engagements.as_mut() {
                e.kills.push(crate::guards::EngageResultKillsItem { victim, team, by, t });
            }
        }
    }
    if gone { end_engagement(g, crate::guards::EndEngagementOutcome::Reset); }
    else if !living_any(g, &hos) { end_engagement(g, crate::guards::EndEngagementOutcome::Cleared); }
    else if !living_any(g, &ops) { end_engagement(g, crate::guards::EndEngagementOutcome::Wiped); }
    else if g.time - t0 > timeout { end_engagement(g, crate::guards::EndEngagementOutcome::Timeout); }
}
