use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small readers

/// `c.bones.hips ?? c.pos`
fn hips_of(g: &Game, c: CharId) -> V3 {
    g.ch[c].bone(crate::character::Bone::Hips).unwrap_or(g.ch[c].pos)
}
/// `c.bones.headTop?.[1] ?? c.pos[1] + 1.75`
fn crown_of(g: &Game, c: CharId) -> f64 {
    match g.ch[c].bones.get(crate::character::Bone::HeadTop) {
        Some(b) => b[1],
        None => g.ch[c].pos[1] + 1.75,
    }
}
/// a write through bodyAftermath's `D`: the record is his while he has one
fn with_duty(g: &mut Game, gd: GuardId, f: impl FnOnce(&mut crate::game::GuardBodyDuty)) {
    if let Some(d) = g.gd[gd].body_duty.as_mut() { f(d); }
}
/// is this man out of the reckoning for a colleague's line: gd himself, the dead, the fallen, an enemy (the walks of 700, 716, 798, 801)
fn no_colleague(g: &Game, gd: GuardId, p: GuardId) -> bool {
    if p == gd || crate::condition::down(g, g.gd[p].ch) { return true; }
    let c = g.gd[p].ch;
    if !g.ch[c].alive { return true; }
    let (a, b) = (g.team(gd), g.team(p));
    a.foe_of(b)
}

// ---------------------------------------------------------------- what a man sees

/// A world point in this man's open view: range, field, nothing solid between; light is not asked.
pub fn point_in_view(g: &crate::game::Game, gd: GuardId, p: V3, range: Option<f64>, half_deg: Option<f64>) -> bool {
    let range = range.unwrap_or(12.0);
    let half_deg = half_deg.unwrap_or(60.0);
    if crate::doze::doze_asleep(g, gd) { return false; }
    let c = g.gd[gd].ch;
    let eye = g.ch[c].bone_or(crate::character::Bone::Head, 1.65);
    let look = crate::guards::look_of(g, c);
    let to = v3::sub(p, eye);
    let l = v3::len(to);
    if l > range { return false; }
    if m::hyp2(to[0], to[2]) < 0.3 { return true; }
    if v3::dot([look[0], 0.0, look[2]], v3::normalize([to[0], 0.0, to[2]])) < m::cos(half_deg * v3::DEG) { return false; }
    !crate::scene_collision::segment_blocked(g, eye, p, None, None, None)
}

/// A light going out that this man's eye has a line to: the fixture, or the floor under it. No cone.
pub(super) fn light_out_seen(g: &crate::game::Game, gd: GuardId, e: crate::guards::LightOutSeenE) -> bool {
    let c = g.gd[gd].ch;
    let eye = g.ch[c].bone_or(crate::character::Bone::Head, 1.65);
    let src: V3 = match e.from { Some(f) => f, None => [e.pos[0], 2.4, e.pos[2]] };
    let to = v3::sub(src, eye);
    let l = v3::len(to);
    if l < 0.6 || !crate::scene_collision::segment_blocked(g, eye, v3::mad(eye, to, (l - 0.4) / l), None, None, None) { return true; }
    !crate::scene_collision::segment_blocked(g, eye, [e.pos[0], 0.15, e.pos[2]], None, None, None)
}

/// A man goes down at Sam's hand: every other man awake with the victim in view, or eyes on Sam, remembers it.
pub fn witness_kill(g: &mut crate::game::Game, victim: GuardId, how: crate::game::DownKind, by: crate::game::DownBy) {
    if !g.ai_enabled || by != crate::game::DownBy::Player { return; }
    let vc = g.gd[victim].ch;
    let at = g.ch[vc].bone_or(crate::character::Bone::Chest, 1.1);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == victim || crate::condition::down(g, g.gd[o].ch) || crate::control::man::blind(g, o) || crate::doze::doze_asleep(g, o) { continue; }
        if (!g.player_invisible && point_in_view(g, o, at, None, None)) || g.gd[o].saw_player_this_frame {
            let kind = match how {
                crate::game::DownKind::Shot => crate::game::WitnessSawActKind::Shot,
                crate::game::DownKind::Struck => crate::game::WitnessSawActKind::Struck,
                crate::game::DownKind::Choked => crate::game::WitnessSawActKind::Choked,
            };
            let act = crate::game::WitnessSawAct { kind, victim: g.gd[victim].callsign.clone(), t: g.time };
            g.gd[o].witness.saw_act = Some(act);
        }
    }
}

/// What took a man over the alert line just now.
pub(super) fn alert_cause(g: &crate::game::Game, gd: GuardId, seen: bool) -> crate::game::WitnessAlertedBy {
    if seen || g.time - g.gd[gd].last_seen_t < 0.5 { return crate::game::WitnessAlertedBy::Sight; }
    if g.gd[gd].witness.heard.last_t >= g.time - 1e-6 { return crate::game::WitnessAlertedBy::Sound; }
    if g.time - g.gd[gd].sight_t < 1.0 { return crate::game::WitnessAlertedBy::Sight; }
    if g.gd[gd].body_duty.is_some() || g.time - g.gd[gd].saw_body_t < 2.0 { return crate::game::WitnessAlertedBy::Body; }
    crate::game::WitnessAlertedBy::Radio
}

/// Floor point under a body: its pelvis once the ragdoll has it.
pub fn body_pos(g: &crate::game::Game, o: GuardId) -> V3 {
    let c = g.gd[o].ch;
    let h = hips_of(g, c);
    [h[0], 0.0, h[2]]
}

/// A found body lying within `r` metres of `p`, if any.
pub fn near_known_body(g: &crate::game::Game, p: V3, r: Option<f64>) -> Option<GuardId> {
    let r = r.unwrap_or(4.0);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if crate::condition::down(g, g.gd[o].ch) && g.gd[o].found && v3::dist_xz(body_pos(g, o), p) < r { return Some(o); }
    }
    None
}

/// What a keyed door gives away: its keep kicked out, or standing unlocked.
pub fn door_tamper(g: &crate::game::Game, d: DoorId) -> Option<crate::guards::DoorTamper> {
    if !g.dr[d].def.locked.unwrap_or(false) { return None; }
    if g.dr[d].leaf.lock_broken { return Some(crate::guards::DoorTamper::Forced); }
    if !g.dr[d].leaf.locked { return Some(crate::guards::DoorTamper::Picked); }
    None
}

// ---------------------------------------------------------------- a found body: the floor's short script

/// Where a man stops by a body: `stand` metres short of it, a free cell he can walk to with the body in view.
fn cover_spot(g: &mut crate::game::Game, gd: GuardId, bp: V3, stand: f64) -> V3 {
    let from = g.pos_of(gd);
    let reach = crate::scene_collision::nav_grid_find_path(g, from, bp, None, None);
    let foot: V3 = match reach.as_ref().and_then(|r| r.last()) { Some(w) => *w, None => v3::copy(from) };
    let base = v3::yaw_to(bp, from);
    for da in [0.0, 0.5, -0.5, 1.0, -1.0, 1.6, -1.6, 2.3, -2.3, m::PI] {
        for r in [stand, stand + 0.5, m::max(1.0, stand - 0.4)] {
            let a = base + da;
            let p: V3 = [bp[0] + m::sin(a) * r, 0.0, bp[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || !crate::scene_collision::nav_grid_walkable(g, foot, p) || crate::scene_collision::segment_blocked(g, [p[0], 1.0, p[2]], [bp[0], 0.4, bp[2]], None, None, None) { continue; }
            if !crate::places::taken(g, gd, p, 0.9) { return p; }
        }
    }
    v3::copy(foot)
}

/// Where a man widens out to from a body: his arm of a fan dealt round it; None = he lets it go where he stands.
fn fan_spot(g: &mut crate::game::Game, gd: GuardId, b: GuardId, bp: V3) -> Option<V3> {
    let from = g.pos_of(gd);
    let k = g.gd[b].body_fan;
    g.gd[b].body_fan = k + 1.0;
    const ARMS: [f64; 6] = [0.0, 2.1, -2.1, 1.05, -1.05, m::PI];
    let base = v3::yaw_to(bp, from) + ARMS[js::rem(k, 6.0) as usize];
    let mut paths = 0;
    for da in [0.0, 0.35, -0.35, 0.7, -0.7, 1.1, -1.1, 1.6, -1.6] {
        for r in [6.0, 7.5, 5.0, 4.5] {
            let a = base + da;
            let p: V3 = [bp[0] + m::sin(a) * r, 0.0, bp[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { continue; }
            if crate::places::taken(g, gd, p, 2.5) { continue; }
            paths += 1;
            if paths > 12 { return None; }
            let path = match crate::scene_collision::nav_grid_find_path(g, from, p, None, None) { Some(path) if !path.is_empty() => path, _ => continue };
            if v3::dist_xz(path[path.len() - 1], bp) < 4.0 { continue; }
            if crate::places::walk_len(from, &path) < 16.0 { return Some(p); }
        }
    }
    None
}

/// Movement watchdog: true once he has covered less than half a metre in `secs` seconds.
pub fn not_getting_anywhere(g: &mut crate::game::Game, gd: GuardId, dt: f64, secs: f64) -> bool {
    let p = g.pos_of(gd);
    let fresh = match g.gd[gd].stall_ref { None => true, Some(r) => v3::dist_xz(p, r) > 0.5 };
    if fresh {
        g.gd[gd].stall_ref = Some(v3::copy(p));
        g.gd[gd].stall_t = 0.0;
        return false;
    }
    g.gd[gd].stall_t += dt;
    g.gd[gd].stall_t > secs
}

/// The alert chase is over without him: down to 'search' from where he stands.
pub fn drop_to_search(g: &mut crate::game::Game, gd: GuardId, bark: Option<&str>) {
    crate::mind::set_state(g, gd, crate::game::GuardState::Search, "guards");
    g.gd[gd].search_t = 0.0;
    crate::mind::cap(g, gd, 0.8, "guards");
    g.gd[gd].search_plan = None;
    reset_chase(g, gd);
    g.gd[gd].drop_path();
    if let Some(b) = bark {
        if !b.is_empty() { crate::game::game_say(g, gd, b, None, None); }   // (`if (bark)`: an empty line is no line)
    }
    crate::chronicle::chron_lost(g, gd);
}

/// Every clock the alert chase keeps, back to a fresh chase.
pub fn reset_chase(g: &mut crate::game::Game, gd: GuardId) {
    let man = &mut g.gd[gd];
    man.alert_best = 1e9;
    man.alert_stall_t = 0.0;
    man.alert_ref = None;
    man.chase_t = 0.0;
    man.shot_blocked_t = 0.0;
    man.line_in_t = 0.0;
    man.stall_ref = None;
    man.stall_t = 0.0;
    man.line_side = 0.0;
    man.line_gain = -1.0;
    man.line_stuck_t = 0.0;
    man.line_fail = None;
}

/// What the stages of a man's part in the found-body script share.
#[derive(Clone, Copy)]
struct BodyDuty {
    gd: GuardId,
    dt: f64,
    c: CharId,
    /// the body
    b: GuardId,
    /// where it lies, on the floor
    bp: V3,
    d0: crate::game::GuardBodyDuty,
    /// how far it is from him
    d_b: f64,
    see_body: bool,
    on_body: V3,
}

/// One frame of a man's part in the found-body script: his move speed and aim point, or None when it no longer applies.
pub(super) fn body_aftermath(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::guards::BodyAftermath> {
    use crate::game::GuardBodyDutyStage as Stage;
    let d0 = g.gd[gd].body_duty.unwrap();
    let (b, stage, c) = (d0.body, d0.stage, g.gd[gd].ch);
    if duty_has_lapsed(g, gd, d0) {
        g.gd[gd].body_duty = None;
        return None;
    }
    let bp = body_pos(g, b);
    let eye = g.ch[c].bone_or(crate::character::Bone::Head, 1.65);
    let d_b = v3::dist_xz(g.ch[c].pos, bp);
    let see_body = d_b < 8.0 && !crate::scene_collision::segment_blocked(g, eye, [bp[0], 0.4, bp[2]], None, None, None);
    let on_body: V3 = [bp[0], 0.35, bp[2]];
    crate::mind::raise(g, gd, if stage == Stage::Fan { 0.16 } else { 0.3 }, "guards");
    let w = BodyDuty { gd, dt, c, b, bp, d0, d_b, see_body, on_body };
    Some(match stage {
        Stage::To => goes_to_the_body(g, w),
        Stage::Cover => covers_the_body(g, w),
        Stage::Fan => fans_out(g, w),
    })
}

/// He is posted or sent elsewhere, the body has got up or gone, it has taken too long, or his mind is on something else.
fn duty_has_lapsed(g: &mut Game, gd: GuardId, d0: crate::game::GuardBodyDuty) -> bool {
    use crate::game::{GuardBodyDutyRole as Role, GuardBodyDutyStage as Stage};
    let b = d0.body;
    let errand = g.gd[gd].task.is_some() && !(d0.role == Role::Finder && d0.stage != Stage::Fan);
    if g.gd[gd].hold || errand || !g.ai_enabled || !crate::condition::down(g, g.gd[b].ch) || !g.guards.contains(&b) || g.time - d0.t0 > BODY_DUTY_CAP { return true; }
    if g.tune.tactics && crate::tactics::body_duty_blocked(g, gd, b) { return true; }
    let bp = body_pos(g, b);
    if g.gd[gd].task.is_none() {
        if let Some(lk) = g.gd[gd].last_known {
            if v3::dist_xz(lk, d0.spot.unwrap_or(bp)) > 2.5 && v3::dist_xz(lk, bp) > 2.5 { return true; }
        }
    }
    false
}

/// Walking to his spot by the body, after his moment; near enough he takes up his place or widens out.
fn goes_to_the_body(g: &mut Game, w: BodyDuty) -> crate::guards::BodyAftermath {
    use crate::game::GuardBodyDutyRole as Role;
    let BodyDuty { gd, dt, c, bp, d0, d_b, see_body, on_body, .. } = w;
    let sp = match d0.spot {
        Some(sp) => sp,
        None => {
            let s = cover_spot(g, gd, bp, if d0.role == Role::Finder { 1.5 } else { 2.1 });
            with_duty(g, gd, |d| d.spot = Some(s));
            crate::mind::fix(g, gd, Some(v3::copy(s)), "guards");
            g.gd[gd].drop_path();
            g.gd[gd].stall_ref = None;
            s
        }
    };
    if g.gd[gd].react_t > 0.0 {
        g.gd[gd].react_t -= dt;
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        return crate::guards::BodyAftermath { move_: 0.0, aim: Some(if see_body { on_body } else { [bp[0], 1.0, bp[2]] }) };
    }
    go_to(g, gd, sp, None);
    let speed = g.tune.guard_investigate;
    let arrived = follow_path(g, gd, dt, speed) || not_getting_anywhere(g, gd, dt, 3.0);
    let move_ = g.gd[gd].speed;
    if arrived || (d_b < 2.6 && see_body && v3::dist_xz(g.ch[c].pos, sp) < 1.5) {
        with_duty(g, gd, |d| d.t = 0.0);
        g.gd[gd].drop_path();
        if (arrived && d_b < 3.5) || see_body {
            stands_by_the_body(g, w);
        } else {
            let s = widens_out(g, w);
            if let Some(s) = s { crate::mind::fix(g, gd, Some(v3::copy(s)), "guards"); }
        }
    }
    crate::guards::BodyAftermath { move_, aim: if see_body { Some(on_body) } else { None } }
}

/// At the body: it counts another visit, and the first man there who did not find it says what he sees, unless he is on the radio.
fn stands_by_the_body(g: &mut Game, w: BodyDuty) {
    use crate::game::{GuardBodyDutyRole as Role, GuardBodyDutyStage as Stage};
    let BodyDuty { gd, b, d0, .. } = w;
    g.gd[b].body_visits += 1.0;
    g.gd[b].body_visit_t = g.time;
    with_duty(g, gd, |d| d.stage = Stage::Cover);
    let radioing = match g.gd[gd].bubble.as_ref() { Some(u) => u.radio && g.time - u.t < 5.0, None => false };
    if d0.role != Role::Finder && !g.gd[b].body_remarked && !radioing {
        g.gd[b].body_remarked = true;
        let line = if crate::game::is_breathing(g, b) {
            "…he's out cold. someone did this by hand. — stay sharp"
        } else {
            "…christ. — stay sharp"
        };
        crate::game::game_say(g, gd, line, None, None);
    }
}

/// He goes on to the fan round the body: his arm of it, his search clock from none. The spot, or None: he lets it go where he stands.
fn widens_out(g: &mut Game, w: BodyDuty) -> Option<V3> {
    use crate::game::GuardBodyDutyStage as Stage;
    let BodyDuty { gd, b, bp, .. } = w;
    with_duty(g, gd, |d| d.stage = Stage::Fan);
    let s = fan_spot(g, gd, b, bp);
    with_duty(g, gd, |d| d.spot = s);
    g.gd[gd].stall_ref = None;
    g.gd[gd].search_t = 0.0;
    s
}

/// Stands over the body, longest the man who found it until somebody else has come, then widens out.
fn covers_the_body(g: &mut Game, w: BodyDuty) -> crate::guards::BodyAftermath {
    use crate::game::GuardBodyDutyRole as Role;
    let BodyDuty { gd, dt, c, b, d0, on_body, .. } = w;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let t = d0.t + dt;
    with_duty(g, gd, |d| d.t = t);
    let done = if d0.role == Role::Finder {
        t > 6.0 || (t > 3.0 && g.gd[b].body_visits >= 2.0 && g.time - g.gd[b].body_visit_t > 1.5)
    } else {
        t > 1.8
    };
    if done {
        with_duty(g, gd, |d| d.t = 0.0);
        let s = widens_out(g, w);
        g.gd[gd].drop_path();
        if let Some(s) = s { crate::mind::fix(g, gd, Some(v3::copy(s)), "guards"); }
    }
    crate::guards::BodyAftermath { move_: 0.0, aim: Some(on_body) }
}

/// Walks out to his arm of the fan looking about him, then stands looking round until he lets it go.
fn fans_out(g: &mut Game, w: BodyDuty) -> crate::guards::BodyAftermath {
    let BodyDuty { gd, dt, c, d0, .. } = w;
    let mut move_ = 0.0;
    let Some(sp) = d0.spot else {
        crate::mind::cap(g, gd, 0.1, "guards");
        return crate::guards::BodyAftermath { move_, aim: None };
    };
    let mut arrived = g.gd[gd].search_t > 0.0;
    if !arrived {
        go_to(g, gd, sp, None);
        let speed = g.tune.guard_walk * 1.3;
        arrived = follow_path(g, gd, dt, speed) || not_getting_anywhere(g, gd, dt, 3.0);
        move_ = g.gd[gd].speed;
    }
    if !arrived {
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(g.gd[gd].look_phase * 0.9) * 0.7, 2.0, dt);
        g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
    } else {
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.gd[gd].search_t += dt;
        g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 1.1) * 1.1;
        g.ch[c].anim.look_yaw_extra = m::sin(g.gd[gd].look_phase * 1.9) * 0.4;
        g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
        if g.gd[gd].search_t > 4.0 { crate::mind::cap(g, gd, 0.1, "guards"); }
    }
    crate::guards::BodyAftermath { move_, aim: None }
}

/// The fix a man is left with the frame Sam goes out of sight: led 0.9 s along his way, as far as the floor is open.
pub(super) fn lead_fix(g: &crate::game::Game, from: V3, vel: V3) -> V3 {
    let p: V3 = [from[0], 0.0, from[2]];
    let (vx, vz) = (vel[0], vel[2]);
    let sp = m::hyp2(vx, vz);
    if sp < 0.5 { return p; }
    let mut shut: Vec<BoxId> = Vec::new();
    let list = g.doors.list.clone();
    for d in list {
        if g.dr[d].leaf.latched { shut.push(g.dr[d].box_); }
    }
    let rd: V3 = [vx / sp, 0.0, vz / sp];
    let mut fix = p;
    let mut q: V3 = [0.0, 0.0, 0.0];
    let mut prev: V3 = [p[0], 1.0, p[2]];
    let mut s = 0.05;
    while s < 0.93 {
        q[0] = p[0] + vx * s;
        q[1] = 0.0;
        q[2] = p[2] + vz * s;
        let at: V3 = [q[0], 1.0, q[2]];
        let mut stop = crate::scene_collision::collide_circle(g, &mut q, 0.15, 0.25, 1.5, Some(1.0)) || crate::scene_collision::clamp_to_world(&mut q, 0.15);
        if !stop {
            for &b in shut.iter() {
                if crate::scene_boxes::ray_box(g, prev, rd, b.into(), sp * 0.05).is_some() { stop = true; break; }
            }
        }
        if stop { break; }
        prev = at;
        if !crate::scene_collision::nav_grid_is_blocked(g, at[0], at[2]) { fix = [at[0], 0.0, at[2]]; }
        s += 0.05;
    }
    fix
}

/// Where a man's line of fire is judged from: the gun shoulder, or for a shouldered long gun the trigger-hand wrist.
pub fn shot_origin(g: &crate::game::Game, c_2: CharId) -> V3 {
    if g.ch[c_2].anim.long_gun {
        if let Some(h) = g.ch[c_2].bones.get(crate::character::Bone::HandR) { return v3::copy(*h); }
    }
    crate::combat::shoulder(g, c_2)
}

/// Nothing a round would stop in between his gun and `to`, less the body's breadth at the far end.
pub fn clear_shot(g: &crate::game::Game, c_2: CharId, to: V3) -> bool {
    let from = shot_origin(g, c_2);
    let d = v3::sub(to, from);
    let l = v3::len(d);
    if l < 0.35 { return true; }
    let energy = crate::glasspen::shot_energy(g, c_2, None);
    !crate::scene_collision::shot_blocked(g, from, v3::mad(from, d, (l - 0.3) / l), Some(0.02), energy)
}

/// A pinned man's one question: his side's rounds have nothing for a pane, and standing glass is the first thing on the line.
pub(super) fn pane_first(g: &crate::game::Game, c_2: CharId, to: V3) -> bool {
    if crate::glasspen::shot_energy(g, c_2, None) != Some(0.0) { return false; }
    let from = shot_origin(g, c_2);
    let d = v3::sub(to, from);
    let l = v3::len(d);
    if l < 0.35 { return false; }
    let h = match crate::scene_collision::raycast(g, from, v3::scale(d, 1.0 / l), l - 0.3) { Some(h) => h, None => return false };
    if !(h.index < 1e6) { return false; }
    let i = h.index as usize;
    // (the level's list IS col.boxes in the TypeScript, game.ts: the one Rust keeps live is g.col.boxes; `?.` is a place past its end)
    let flags = match g.col.boxes.get(i).copied() { Some(b) => g.tab.boxes[b].flags, None => 0.0 };
    (js::to_i32(flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) != 0
}

// ---------------------------------------------------------------- muzzle discipline

/// Where a colleague stands against the aim segment: how far along, his signed offset, his 3D distance, the clearance he is owed.
fn line_geom(g: &crate::game::Game, from: V3, to: V3, o: GuardId) -> Option<crate::guards::LineGeom> {
    let c = g.gd[o].ch;
    let h = hips_of(g, c);
    let y0 = g.ch[c].pos[1] + 0.05;
    let y1 = crown_of(g, c);
    let vel = g.ch[c].vel;
    line_geom_of(from, to, h, y0, y1, vel)
}
/// lineGeom's arithmetic, over what it reads of the man: his pelvis `h`, the span of his body axis `y0`..`y1`, his velocity
fn line_geom_of(from: V3, to: V3, h: V3, y0: f64, y1: f64, vel: V3) -> Option<crate::guards::LineGeom> {
    let (dx, dy, dz) = (to[0] - from[0], to[1] - from[1], to[2] - from[2]);
    let l2 = dx * dx + dz * dz;
    if l2 < 1e-4 { return None; }
    let (ax, az) = (h[0], h[2]);
    let t = ((ax - from[0]) * dx + (az - from[2]) * dz) / l2;
    let (px, pz, py) = (from[0] + dx * t, from[2] + dz * t, from[1] + dy * t);
    let l = m::sqrt(l2);
    let e = ((ax - from[0]) * -dz + (az - from[2]) * dx) / l;
    let ddy = if py < y0 { y0 - py } else if py > y1 { py - y1 } else { 0.0 };
    let clear = MUZZLE_CLEAR_M + m::min(MUZZLE_LEAD_MAX_M, MUZZLE_LEAD_SECS * m::hyp2(vel[0], vel[2]));
    Some(crate::guards::LineGeom { t, e, d: m::hyp3(px - ax, pz - az, ddy), clear })
}

/// The nearest living colleague in this man's intended line of aim, or None.
pub fn colleague_in_line(g: &crate::game::Game, gd: GuardId, to: V3, except: Option<GuardId>) -> Option<GuardId> {
    let c = g.gd[gd].ch;
    let from = crate::combat::shoulder(g, c);
    let l = v3::dist_xz(to, from);
    if l < 0.01 { return None; }
    let t_max = 1.0 + MUZZLE_PAST_M / l;
    let mut best: Option<GuardId> = None;
    let mut bt = m::INF;
    let near = g.team(gd).ours() && crate::roomfight::fight_live(g);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if Some(o) == except || no_colleague(g, gd, o) { continue; }
        let geom = match line_geom(g, from, to, o) { Some(x) => x, None => continue };
        if geom.t < 0.0 || geom.t > t_max || geom.t >= bt { continue; }
        let lane = if near { crate::roomfight::close_lane(g, geom.clear, geom.t, l) } else { geom.clear };
        if geom.d > lane { continue; }
        bt = geom.t;
        best = Some(o);
    }
    best
}

/// The last look before the round goes: a living colleague within a body's breadth of the segment the round will fly.
pub fn round_past_colleague(g: &crate::game::Game, gd: GuardId, to: V3, except: Option<GuardId>, cone_per_m: Option<f64>) -> bool {
    let cone_per_m = cone_per_m.unwrap_or(0.0);
    let from = g.ch[g.gd[gd].ch].muzzle;
    let (dx, dy, dz) = (to[0] - from[0], to[1] - from[1], to[2] - from[2]);
    let l2 = dx * dx + dz * dz;
    if l2 < 0.0025 { return false; }
    let l = m::sqrt(l2);
    let t_max = 1.0 + 0.3 / l;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if Some(o) == except || no_colleague(g, gd, o) { continue; }
        let c = g.gd[o].ch;
        let h = hips_of(g, c);
        let t = v3::clamp(((h[0] - from[0]) * dx + (h[2] - from[2]) * dz) / l2, 0.0, t_max);
        let (px, py, pz) = (from[0] + dx * t, from[1] + dy * t, from[2] + dz * t);
        let y0 = g.ch[c].pos[1] + 0.05;
        let y1 = crown_of(g, c);
        let ddy = if py < y0 { y0 - py } else if py > y1 { py - y1 } else { 0.0 };
        let clear = m::max(0.32, MUZZLE_CLEAR_M + m::min(MUZZLE_LEAD_MAX_M, MUZZLE_LEAD_SECS * m::hyp2(g.ch[c].vel[0], g.ch[c].vel[2]))) + cone_per_m * t * l;
        if m::hyp3(px - h[0], pz - h[2], ddy) < clear { return true; }
    }
    false
}

/// The point MUZZLE_REACH_M out along where his gun is held, or where that line meets the floor.
pub fn aim_ahead(g: &crate::game::Game, c_2: CharId) -> V3 {
    let from = crate::combat::shoulder(g, c_2);
    aim_ahead_of(from, g.ch[c_2].aim_pitch, g.ch[c_2].aim_yaw)
}
/// aimAhead's arithmetic
fn aim_ahead_of(from: V3, pitch: f64, yaw: f64) -> V3 {
    let (cp, sy) = (m::cos(pitch), m::sin(pitch));
    let mut reach = MUZZLE_REACH_M;
    if sy < -1e-3 { reach = m::min(reach, (from[1] - 0.02) / -sy); }
    [from[0] + m::sin(yaw) * cp * reach, from[1] + sy * reach, from[2] + m::cos(yaw) * cp * reach]
}

/// This frame's muzzle discipline: a colleague in the line puts the gun down, a line clear long enough lets it back up.
pub fn muzzle_check(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>, dt: f64, except: Option<GuardId>) -> Option<GuardId> {
    g.gd[gd].muzzle_eval_t = g.time;
    let o = match to { Some(t) => colleague_in_line(g, gd, t, except), None => None };
    if o.is_some() {
        g.gd[gd].muzzle_down = true;
        g.gd[gd].muzzle_clear_t = 0.0;
    } else if g.gd[gd].muzzle_down {
        let up = if to.is_none() {
            true
        } else {
            g.gd[gd].muzzle_clear_t += dt;
            g.gd[gd].muzzle_clear_t > MUZZLE_UP_SECS
        };
        if up {
            g.gd[gd].muzzle_down = false;
            g.gd[gd].muzzle_clear_t = 0.0;
        }
    }
    o
}

/// In a fight, the side-step that opens his line round colleague `o`: under way, none from here, or nobody to step round.
pub(super) fn line_step(g: &mut crate::game::Game, gd: GuardId, aim_at: V3, o: Option<GuardId>, dt: f64) -> crate::guards::LineStep {
    let c = g.gd[gd].ch;
    if let Some(ob) = o { calls_out_the_line(g, gd, ob); }
    if step_goes_on(g, gd, c, dt) { return crate::guards::LineStep::Step; }
    let Some(ob) = o else { return crate::guards::LineStep::Clear };
    let from = crate::combat::shoulder(g, c);
    let geom = match line_geom(g, from, aim_at, ob) { Some(x) if !(x.t > 0.97) => x, _ => return crate::guards::LineStep::None };
    let pos = g.ch[c].pos;
    let (dx, dz) = (aim_at[0] - pos[0], aim_at[2] - pos[2]);
    let l = m::hyp2(dx, dz);
    if l < 0.3 { return crate::guards::LineStep::None; }
    let (lx, lz) = (-dz / l, dx / l);
    let need = geom.clear + LINE_STEP_MARGIN_M;
    let k = 1.0 / m::max(0.03, 1.0 - geom.t);
    let mut cands: Vec<f64> = Vec::new();
    for s in [(geom.e - need) * k, (geom.e + need) * k] {
        if m::abs(s) <= LINE_STEP_MAX_M && m::abs(s) > 0.02 { cands.push(s); }
    }
    js::sort_by(&mut cands, |a, b| m::abs(*a) - m::abs(*b));
    if js::to_i32(g.gd[gd].stuck_n) & 1 != 0 { cands.reverse(); }
    // (re-read: a stall just above may have written it)
    let failed: i32 = match g.gd[gd].line_fail {
        Some(f) if g.time - f.t < LINE_FAIL_SECS && v3::dist_xz(pos, f.pos) < LINE_FAIL_M => js::to_i32(f.sides),
        _ => 0,
    };
    let w = StepLook { gd, ob, aim_at, from, pos, lx, lz, l, radius: g.ch[c].radius };
    for &s in cands.iter() {
        if failed & (if s < 0.0 { 1 } else { 2 }) != 0 { continue; }
        let Some(cand) = step_that_opens_it(g, w, s) else { continue };
        crate::game::guard_set_path(g, gd, vec![cand]);
        let man = &mut g.gd[gd];
        man.path_i = 0;
        man.path_goal = Some(v3::copy(cand));
        man.path_goal_orbit = false;
        man.stall_ref = None;
        man.line_side = if s < 0.0 { -1.0 } else { 1.0 };
        man.line_gain = -1.0;
        man.line_stuck_t = 0.0;
        return crate::guards::LineStep::Step;
    }
    crate::guards::LineStep::None
}

/// Now and then one of them says so: the crossing man, or the man whose line it is.
fn calls_out_the_line(g: &mut Game, gd: GuardId, ob: GuardId) {
    if g.time - g.gd[gd].line_bark_t > LINE_BARK_SECS && g.time - g.gd[ob].line_bark_t > LINE_BARK_SECS {
        let now = g.time;
        g.gd[ob].line_bark_t = now;
        g.gd[gd].line_bark_t = now;
        let oc = g.gd[ob].ch;
        if m::hyp2(g.ch[oc].vel[0], g.ch[oc].vel[2]) > 0.4 {
            crate::game::game_say(g, ob, "crossing!", None, None);
        } else {
            crate::game::game_say(g, gd, "move — you're in my line", None, None);
        }
    }
}

/// His side-step ends: no path, and his line's clocks back to none.
fn step_ended(g: &mut Game, gd: GuardId) {
    g.gd[gd].drop_path();
    let man = &mut g.gd[gd];
    man.stall_ref = None;
    man.line_side = 0.0;
    man.line_gain = -1.0;
    man.line_stuck_t = 0.0;
}

/// A side-step already under way: true while it goes on; it ends done, or given up with that side remembered when it is handed back or gets nowhere.
fn step_goes_on(g: &mut Game, gd: GuardId, c: CharId, dt: f64) -> bool {
    let f_ = g.gd[gd].line_fail;
    let pos0 = g.ch[c].pos;
    let fail_live = match f_ { Some(f) => g.time - f.t < LINE_FAIL_SECS && v3::dist_xz(pos0, f.pos) < LINE_FAIL_M, None => false };
    let under_way = g.gd[gd].line_side != 0.0 && g.gd[gd].path.len() == 1 && g.gd[gd].path_i == 0 && match g.gd[gd].path_goal { Some(goal) => v3::dist_xz(goal, g.gd[gd].path[0]) < 1e-3, None => false };
    if !under_way { return false; }
    let wp = g.gd[gd].path[0];
    let (dx, dz) = (wp[0] - pos0[0], wp[2] - pos0[2]);
    let d = m::hyp2(dx, dz);
    let back = if g.gd[gd].line_gain >= 0.0 { d - g.gd[gd].line_gain } else { 0.0 };
    if g.gd[gd].line_step_m > 1e-4 && back > 0.5 * g.gd[gd].line_step_m {
        g.gd[gd].line_stuck_t += dt;
    } else {
        g.gd[gd].line_stuck_t = m::max(0.0, g.gd[gd].line_stuck_t - 2.0 * dt);
    }
    if d < 0.05 {
        step_ended(g, gd);
        return false;
    }
    if g.gd[gd].line_stuck_t > LINE_STUCK_SECS || not_getting_anywhere(g, gd, dt, 1.2) {
        let side = g.gd[gd].line_side;
        let bit: i32 = if side < 0.0 { 1 } else if side > 0.0 { 2 } else { 0 };
        let now = g.time;
        g.gd[gd].line_fail = Some(match (fail_live, f_) {
            (true, Some(f)) => crate::game::GuardLineFail { pos: f.pos, sides: (js::to_i32(f.sides) | bit) as f64, t: now },
            _ => crate::game::GuardLineFail { pos: v3::copy(pos0), sides: bit as f64, t: now },
        });
        g.gd[gd].stuck_n += 1.0;
        step_ended(g, gd);
        return false;
    }
    let speed = v3::damp(g.gd[gd].speed, g.tune.guard_investigate * v3::clamp(d / 0.45, 0.35, 1.0), 6.0, dt);
    g.gd[gd].speed = speed;
    let step = m::min(d, speed * dt);
    g.ch[c].pos[0] += dx / d * step;
    g.ch[c].pos[2] += dz / d * step;
    g.ch[c].vel = [dx / d * speed, 0.0, dz / d * speed];
    g.gd[gd].line_gain = d - step;
    g.gd[gd].line_step_m = step;
    true
}

/// What a new side-step is weighed against.
#[derive(Clone, Copy)]
struct StepLook {
    gd: GuardId,
    ob: GuardId,
    aim_at: V3,
    from: V3,
    pos: V3,
    lx: f64,
    lz: f64,
    l: f64,
    radius: f64,
}

/// Where a side-step of `s` metres would put him, if that floor is free, clear of everybody, and opens his line without closing it on another man.
fn step_that_opens_it(g: &Game, w: StepLook, s: f64) -> Option<V3> {
    let StepLook { gd, ob, aim_at, from, pos, lx, lz, l, radius } = w;
    let mut cand: V3 = [pos[0] + lx * s, 0.0, pos[2] + lz * s];
    let (mut ox, mut oz) = (lx * s, lz * s);
    if crate::scene_collision::nav_grid_is_blocked(g, cand[0], cand[2]) || !crate::scene_collision::nav_grid_walkable(g, pos, cand) || crate::scene_collision::solid_blocked(g, [pos[0], 0.5, pos[2]], [cand[0], 0.5, cand[2]], None) { return None; }
    let mut fit: V3 = [cand[0], 0.0, cand[2]];
    crate::scene_collision::collide_circle(g, &mut fit, radius, 0.2, 1.5, None);
    if v3::dist_xz(fit, cand) > 0.02 {
        ox = fit[0] - pos[0];
        oz = fit[2] - pos[2];
        let gf = line_geom(g, [from[0] + ox, from[1], from[2] + oz], aim_at, ob);
        let no_room = match gf { None => true, Some(x) => x.d < x.clear + 0.04 };
        if no_room || crate::scene_collision::nav_grid_is_blocked(g, fit[0], fit[2]) || m::hyp2(ox, oz) < 0.05 { return None; }
        cand = [fit[0], 0.0, fit[2]];
    }
    let o2 = ox * ox + oz * oz;
    let walks_into_one = g.guards.iter().any(|&p| {
        if no_colleague(g, gd, p) { return false; }
        let q = g.pos_of(p);
        let u = v3::clamp(((q[0] - pos[0]) * ox + (q[2] - pos[2]) * oz) / o2, 0.0, 1.0);
        m::hyp2(pos[0] + ox * u - q[0], pos[2] + oz * u - q[2]) < LINE_WALK_CLEAR_M
    });
    if walks_into_one { return None; }
    let from2: V3 = [from[0] + ox, from[1], from[2] + oz];
    let lines_up_another = g.guards.iter().any(|&p| {
        if p == ob || no_colleague(g, gd, p) { return false; }
        line_geom(g, from2, aim_at, p).map_or(false, |g2| g2.t >= 0.0 && g2.t <= 1.0 + MUZZLE_PAST_M / l && g2.d <= g2.clear + LINE_STEP_MARGIN_M)
    });
    if lines_up_another { return None; }
    Some(cand)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(a: &[f64]) -> Vec<u64> { a.iter().map(|x| x.to_bits()).collect() }

    // The expected words are the TypeScript's own: lineGeom and aimAhead's source run under bun with EXACT.on.
    #[test]
    fn line_geom_is_the_typescripts_bits() {
        let x = line_geom_of([0.1, 1.4, 0.2], [3.3, 1.1, 4.9], [1.37, 0.91, 2.11], 0.0 + 0.05, 1.83, [0.7, 0.0, -0.4]).unwrap();
        assert_eq!(bits(&[x.t, x.e, x.d, x.clear]), [0x3fd9d0d69e66051a, 0x3f99c0d942568028, 0x3f99c0d94256801b, 0x3fe2f094994da159]);
        let x = line_geom_of([0.1, 1.4, 0.2], [0.4, 0.9, 9.1], [0.31, 0.2, 5.77], 0.2 + 0.05, 0.2 + 1.75, [2.1, 0.0, 1.9]).unwrap();
        assert_eq!(bits(&[x.t, x.e, x.d, x.clear]), [0x3fe40797c42fe1a0, 0xbf96c4a8424a9e85, 0x3f96c4a8424a9e8d, 0x3fe8000000000000]);
        assert!(line_geom_of([1.0, 1.0, 1.0], [1.005, 5.0, 1.005], [0.0; 3], 0.0, 1.0, [0.0; 3]).is_none());
    }

    #[test]
    fn aim_ahead_is_the_typescripts_bits() {
        assert_eq!(bits(&aim_ahead_of([0.2, 1.45, -0.3], -0.4, 0.77)), [0x40046fa613722776, 0x3f947ae147ae1480, 0x4001067c11618aec]);
        assert_eq!(bits(&aim_ahead_of([1.1, 0.1 + 1.4, 2.2], 0.05, -2.3)), [0xc0136ec85ce57064, 0x3ffe65b7a8bfd90f, 0xc008fd061a86f980]);
    }

    /// a game with one man and his body; nothing below reaches another part
    fn one_man() -> (Box<Game>, GuardId, CharId) {
        let mut g = Box::new(Game::blank());
        let c = crate::character::man_new(&mut g, crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let gd = g.gd.push(man);
        (g, gd, c)
    }

    #[test]
    fn the_watchdog_fires_on_ground_not_covered() {
        let (mut g, gd, c) = one_man();
        assert!(!not_getting_anywhere(&mut g, gd, 1.0, 2.5));   // takes its reference
        assert!(!not_getting_anywhere(&mut g, gd, 1.0, 2.5));
        assert!(!not_getting_anywhere(&mut g, gd, 1.0, 2.5));
        assert!(not_getting_anywhere(&mut g, gd, 1.0, 2.5));
        g.ch[c].pos[0] += 0.6;
        assert!(!not_getting_anywhere(&mut g, gd, 1.0, 2.5));   // moved: a fresh clock
        assert_eq!(g.gd[gd].stall_t, 0.0);
    }

    #[test]
    fn the_gun_comes_back_up_after_the_beat() {
        let (mut g, gd, _) = one_man();
        g.gd[gd].muzzle_down = true;
        assert_eq!(muzzle_check(&mut g, gd, Some([5.0, 1.0, 5.0]), 0.15, None), None);
        assert!(g.gd[gd].muzzle_down);
        muzzle_check(&mut g, gd, Some([5.0, 1.0, 5.0]), 0.1, None);
        assert!(!g.gd[gd].muzzle_down);
        assert_eq!(g.gd[gd].muzzle_clear_t, 0.0);
        g.gd[gd].muzzle_down = true;
        muzzle_check(&mut g, gd, None, 0.0, None);   // the pistol is not up at all: at once
        assert!(!g.gd[gd].muzzle_down);
    }

    #[test]
    fn a_step_handed_back_is_dropped_and_its_side_remembered() {
        let (mut g, gd, c) = one_man();
        g.time = 12.0;
        g.ch[c].pos = [1.0, 0.0, 1.0];
        let man = &mut g.gd[gd];
        man.path = vec![[2.0, 0.0, 1.0]];
        man.path_i = 0;
        man.path_goal = Some([2.0, 0.0, 1.0]);
        man.line_side = -1.0;
        man.line_step_m = 0.1;
        man.line_gain = 0.0;   // he is a metre short though last frame left him none: all of it handed back
        let n = man.stuck_n;
        assert_eq!(line_step(&mut g, gd, [9.0, 1.0, 9.0], None, 0.5), crate::guards::LineStep::Clear);
        let man = &g.gd[gd];
        let f = man.line_fail.unwrap();
        assert_eq!((f.sides, f.t, f.pos), (1.0, 12.0, [1.0, 0.0, 1.0]));
        assert_eq!((man.stuck_n, man.line_side, man.line_gain, man.line_stuck_t), (n + 1.0, 0.0, -1.0, 0.0));
        assert!(man.path.is_empty() && man.path_goal.is_none() && man.stall_ref.is_none());
    }
}
