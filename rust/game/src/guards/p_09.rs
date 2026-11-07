//! src/game/guards.ts lines 2914 to 3311
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::character::Bone;
use crate::game::{GuardBodyDuty, GuardBodyDutyRole, GuardBodyDutyStage, GuardBubble, GuardState, GuardTask, GuardTaskCheckOn};
use crate::teams::Team;


// ---- small readers

/// `g.audio.play(name, at, vol)`
fn play(g: &mut Game, name: &str, at: V3, vol: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(at), Some(vol), None);
}
/// `g.audio.play(name, at, vol, { rate })`
fn play_rate(g: &mut Game, name: &str, at: V3, vol: f64, rate: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(at), Some(vol), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
}
/// `ordersOf(g, gd)`'s `sams && mine(gd)` (seats.ts:246): the book on Game with one book, his seat's otherwise, asked inside that seat.
fn orders_sams_mine(g: &mut Game, gd: GuardId) -> bool {
    if crate::seats::one_book(g) { return crate::orders::squad_orders_sams(g) && crate::orders::squad_orders_mine(g, gd); }
    let s = crate::seats::seat_of(g, gd);
    crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::orders::squad_orders_sams(g) && crate::orders::squad_orders_mine(g, gd))
}
/// `gd.lastKnown = at`: a new object in the TypeScript, so every whole write bumps `fix_stamp` (doze.ts:244's identity question).
fn write_fix(d: &mut crate::game::Guard, at: V3) {
    d.last_known = Some(at);
    d.fix_stamp = d.fix_stamp.wrapping_add(1);
}
/// `gd.path = []; gd.pathGoal = null`: a new array, so `path_serial` moves (`gd.path === W.arr` elsewhere), and the goal is no longer the standoff's point (`path_goal_orbit`).
fn no_path(d: &mut crate::game::Guard) {
    d.path = Vec::new();
    d.path_serial = d.path_serial.wrapping_add(1);
    d.path_goal = None;
    d.path_goal_orbit = false;
}
/// his head bone, or his feet (`x.char.bones.head ?? x.char.pos`: where a radio crackle sounds)
fn head_or_pos(g: &Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(Bone::Head) { Some(h) => *h, None => g.ch[c].pos }
}
/// `g.level.routes[gd.routeI]`'s points, if that route exists
fn route_points(g: &Game, gd: GuardId) -> Option<Vec<V3>> {
    g.level.routes.get(g.gd[gd].route_i as usize).map(|r| g.tab.route[*r].points.clone())
}
/// `g.level.routes[M.routeI]?.points[0] ?? M.char.pos`
fn post_of(g: &Game, m: GuardId) -> V3 {
    match route_points(g, m).and_then(|p| p.first().copied()) { Some(p) => p, None => g.ch[g.gd[m].ch].pos }
}

/// His head, or where it would be.   (guards.ts headOf)
pub fn head_of(g: &mut crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.7, 0.0]) }
}

/// A man says something: a bubble over his head, and a log row if it went over the radio or is out of sight.   (guards.ts say)
pub fn say(g: &mut crate::game::Game, gd: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) {
    let radio = radio.unwrap_or(false);
    let force = force.unwrap_or(false);
    let row = row.unwrap_or(true);
    if g.gd[gd].held.is_some() && !force { return; }
    let net = radio || crate::silent::silent_hush(g, gd);
    let dur = bubble_secs(text);
    g.gd[gd].bubble = Some(GuardBubble { text: String::from(text), t: g.time, dur, radio: net });
    if net { crate::handwork::radio_hand(g, gd, dur); }
    if !net && row { crate::cues::cues_overheard(g, gd, text); }
    if radio && g.gd[gd].unit.is_some() { crate::cues::cues_heard(g, gd, text); }
    if radio { crate::chronicle::chron_radio(g, gd, text); }
    if row && (radio || crate::fogofwar::fog_of_war_hidden(g, gd) || { let h = head_of(g, gd); !on_screen(g, h) }) {
        let unit = g.gd[gd].unit.is_some();
        let cs = &g.gd[gd].callsign;
        let who = if net {
            if unit { format!("{} (radio): ", cs) } else { String::from("radio: ") }
        } else if force || unit {
            format!("{}: ", cs)
        } else {
            String::from("guard: ")
        };
        let line = format!("{}{}", who, text);
        crate::game::game_msg(g, &line);
    }
    if orders_sams_mine(g, gd) && { let h = head_of(g, gd); !on_screen(g, h) } {
        crate::squadnet::squad_net_line(g);
    }
}

/// Fresh guards at the start of their routes; spawned units are not stood up again.   (guards.ts resetGuards)
pub fn reset_guards(g: &mut crate::game::Game) {
    for pl in crate::seats::men(g) {
        g.pl[pl].dragging = None;
        g.pl[pl].takedown = None;
        g.pl[pl].holding = None;
        let c = g.pl[pl].ch;
        g.ch[c].anim.hold_pose = None;
    }
    let old = g.guards.clone();
    for &gd in &old {
        if g.gd[gd].unit.is_some() { release_unit(g, gd); }
    }
    end_engagement(g, EndEngagementOutcome::Reset);
    let old = g.guards.clone();
    let mut fresh: Vec<(f64, RouteId, i32, LightId, V3)> = Vec::new();
    for &gd in &old {
        if g.gd[gd].unit.is_some() { continue; }
        let c = g.gd[gd].ch;
        fresh.push((g.ch[c].id, g.gd[gd].route, g.gd[gd].route_i, g.gd[gd].flashlight, g.ch[c].tint));
    }
    for &gd in &old {
        let c = g.gd[gd].ch;
        crate::sim_men_store::man_leaves(g, c);
    }
    let mut shift: Vec<GuardId> = Vec::with_capacity(fresh.len());
    let (men, rig) = (g.men, g.rig);
    for (id, route, route_i, fl, tint) in fresh { shift.push(crate::game::guard_new(g, men, rig, id, route, route_i, fl, tint, None)); }
    g.guards = shift.clone();
    for gd in shift {
        let c = g.gd[gd].ch;
        crate::character::character_update(g, c, 0.0);
    }
    crate::opening::opening_start(g);
    crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().fix, None, Some(0.0), None, None, None);
    crate::seats::each_seat(g, &mut |v: &mut Game| { v.aim_guard = None; });
    g.escalation = 0.0;
    g.escalation_t = -1.0;
    g.alarm.episode = false;
    g.alarm.placed = false;
    g.alarm.apart_t = 0.0;
    g.clearing = None;
    g.cleared_at.clear();
    let r = rollcall_state(g);
    r.stage = RollcallStage::Idle;
    r.next_at = -1.0;
    r.caller = None;
    r.missing = None;
    crate::game::game_note(g, "guards reset");
}

// ---- alarm escalation

/// The alarm level the guards act on: the game's, except under the showcase tour.   (guards.ts escalationOf)
pub fn escalation_of(g: &mut crate::game::Game) -> f64 {
    if g.quiet_utility { 0.0 } else { g.escalation }
}
/// The living man best placed to radio something about `pos`: the nearest not mid-fight, else the nearest at all, never `except`.   (guards.ts spokesman)
pub fn spokesman(g: &mut crate::game::Game, pos: Option<V3>, except: Option<GuardId>) -> Option<GuardId> {
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    let guards = g.guards.clone();
    let mut pass = 0;
    while pass < 2 && best.is_none() {
        for &gd in &guards {
            if off_net(g, gd)
                || Some(gd) == except
                || (pass == 0 && (g.gd[gd].state == GuardState::Alert || crate::doze::doze_asleep(g, gd)))
                || g.gd[gd].team != Team::Guard
                || g.gd[gd].unit.is_some()
            { continue; }
            let d = match pos { Some(p) => v3::dist(g.ch[g.gd[gd].ch].pos, p), None => 0.0 };
            if d < bd { bd = d; best = Some(gd); }
        }
        pass += 1;
    }
    best
}

/// Raise the floor to at least `to` and (re)arm that level's clock: one radio line per actual change.   (guards.ts raiseEscalation)
pub fn raise_escalation(g: &mut crate::game::Game, to: f64, except: Option<GuardId>, cause: Option<crate::guards::RaiseEscalationCause>, another: Option<bool>) {
    use crate::guards::RaiseEscalationCause as C;
    let cause = cause.unwrap_or(C::Search);
    let another = another.unwrap_or(false);
    let prev = g.escalation;
    let lvl = if to > prev { to } else { prev };
    g.escalation = lvl;
    g.escalation_t = g.time + if lvl >= 2.0 { LOCKDOWN_SECS } else { HEIGHTENED_SECS };
    if lvl > prev { crate::chronicle::chron_alarm(g, lvl, true, Some(cause.as_str())); }
    let again = g.time - g.alarm.raised_at < 1.5;
    g.alarm.raised_at = g.time;
    if again && lvl <= prev && cause != C::Body { return; }
    let at = if g.alarm.placed { Some(g.alarm.pos) } else { None };
    let sp = spokesman(g, at, except);
    let line: String = if lvl <= prev {
        match cause {
            C::Body => format!("{} — stay locked down, clear everything around him", if another { "another man down" } else { "man down confirmed" }),
            C::Missing => String::from("and a man we can't raise — it stays locked down, find him"),
            C::Hostage => String::from("copy — he's got one of ours. stay locked down. box him in. no shot unless it's clean"),
            C::Device => String::from("copy, control — it stays locked down, get to it"),
            C::Civilian => String::from("copy — another one saw him. stay on it, get over there"),
            C::Search => String::from("still nothing — it stays locked down"),
        }
    } else if lvl >= 2.0 {
        if cause == C::Hostage {
            String::from("he's got one of ours — lock it down. weapons out, everyone box him in and hold your fire")
        } else {
            let lead = match cause {
                C::Body => if another { "another man down — " } else { "man down — " },
                C::Missing => "a man missing on top of it — ",
                C::Device => "copy, control — ",
                C::Civilian => "copy — that's a witness. ",
                _ => "",
            };
            format!("{}lock it down: pairs, weapons out, nobody wanders", lead)
        }
    } else {
        match cause {
            C::Body => String::from("we have a man down — weapons out, everyone sharp"),
            C::Missing => String::from("we've got a man off the net and off his route — weapons out, treat it as real"),
            C::Device => String::from("copy, control — weapons out, everyone sharp, somebody get eyes on it"),
            C::Civilian => String::from(if !crate::dialogue::is_office(g) { "copy — somebody's here who shouldn't be. weapons out, treat it as real" } else { "copy — somebody's on the floor who shouldn't be. weapons out, treat it as real" }),
            _ => String::from("nothing here — stay sharp, check your corners"),
        }
    };
    if let Some(sp) = sp {
        crate::game::game_say(g, sp, &line, Some(true), None, None);
        let at = head_or_pos(g, sp);
        play(g, "radio", at, 0.7);
    }
    if lvl >= 2.0 {
        assign_lockdown(g, prev >= 2.0, except);
        if cause != C::Hostage {
            let why = match cause {
                C::Body => PlanClearsCause::Body,
                C::Missing => PlanClearsCause::Missing,
                _ => PlanClearsCause::Search,
            };
            plan_clears(g, Some(false), Some(why));
        }
    }
}

/// Panel: the floor calm again at once, without touching the men themselves.   (guards.ts standDownEscalation)
pub fn stand_down_escalation(g: &mut crate::game::Game) {
    g.escalation = 0.0;
    g.escalation_t = -1.0;
    g.alarm.episode = false;
    g.alarm.placed = false;
    if g.clearing.is_some() { crate::squad::clearing_cancel(g); }
    g.clearing = None;
    g.cleared_at.clear();
    clear_lockdown(g);
}

// ---- the net's radio check

/// A man who cannot answer the net: down, or held with Sam's arm across his throat.   (guards.ts offNet)
pub fn off_net(g: &mut crate::game::Game, x: GuardId) -> bool {
    g.gd[x].state == GuardState::Dead || g.gd[x].held.is_some()
}
/// The net's radio-check state for this game, made on first touch; the row itself.   (guards.ts rollcallState)
pub fn rollcall_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::guards::Rollcall {
    g.m.guards.rollcalls.get_or_insert_with(|| Rollcall { next_at: -1.0, stage: RollcallStage::Idle, t0: 0.0, caller: None, missing: None, answered: false, checks: 0.0 })
}
/// Seconds to the next check, jittered: a die.   (guards.ts rollcallInterval)
pub fn rollcall_interval(g: &mut crate::game::Game) -> f64 {
    let secs = if escalation_of(g) >= 1.0 { ROLLCALL_ALARMED_SECS } else { ROLLCALL_CALM_SECS };
    secs * (1.0 + (crate::math_dice::sim_random(g) * 2.0 - 1.0) * ROLLCALL_JITTER)
}
/// Who calls the check: a living man with his hands free for the radio, one on his route for choice.   (guards.ts radioCaller)
pub fn radio_caller(g: &mut crate::game::Game) -> Option<GuardId> {
    let mut best: Option<GuardId> = None;
    let mut br = -1;
    let guards = g.guards.clone();
    for x in guards {
        if off_net(g, x)
            || g.gd[x].state == GuardState::Alert
            || g.gd[x].script.is_some()
            || matches!(&g.gd[x].body_duty, Some(b) if b.stage != GuardBodyDutyStage::Fan)
            || g.gd[x].team != Team::Guard
            || crate::doze::doze_asleep(g, x)
        { continue; }
        let r = if g.gd[x].state == GuardState::Patrol { 2 } else { 1 };
        if r > br { br = r; best = Some(x); }
    }
    best
}
/// Somebody already out walking this man's route.   (guards.ts checkerFor)
pub fn checker_for(g: &mut crate::game::Game, m: GuardId) -> Option<GuardId> {
    g.guards.iter().copied().find(|&x| g.gd[x].state != GuardState::Dead && matches!(&g.gd[x].task, Some(GuardTask::CheckOn(k)) if k.who == m))
}
/// A silent man is being looked for right now: the floor's alarm level holds meanwhile.   (guards.ts stillLooking)
pub fn still_looking(g: &mut crate::game::Game) -> bool {
    rollcall_state(g).stage == RollcallStage::Asked
        || g.guards.iter().any(|&x| g.gd[x].state != GuardState::Dead && matches!(&g.gd[x].task, Some(GuardTask::CheckOn(_))))
}
/// calm, free of orders, errands and bodies, and of Sam's arm   (guards.ts pickChecker's `free`)
fn checker_free(g: &Game, x: GuardId, m: GuardId) -> bool {
    let d = &g.gd[x];
    x != m && d.state == GuardState::Patrol && !d.hold && d.script.is_none() && d.task.is_none() && d.body_duty.is_none() && d.held.is_none() && d.team == Team::Guard
}
/// The living man best placed to go round M's route, nearest any point of it; `prefer` first.   (guards.ts pickChecker)
pub fn pick_checker(g: &mut crate::game::Game, m: GuardId, prefer: Option<GuardId>) -> Option<GuardId> {
    let pts = match route_points(g, m) { Some(p) => p, None => vec![g.ch[g.gd[m].ch].pos] };
    if let Some(p) = prefer { if checker_free(g, p, m) { return Some(p); } }
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    for &x in &g.guards {
        if !checker_free(g, x, m) { continue; }
        let mut d = f64::INFINITY;
        let at = g.ch[g.gd[x].ch].pos;
        for &p in &pts { d = m::min(d, v3::dist_xz(p, at)); }
        if d < bd { bd = d; best = Some(x); }
    }
    best
}
/// Send `gd` round M's route: the checkOn errand from the waypoint of M's nearest him.   (guards.ts sendCheckOn)
pub fn send_check_on(g: &mut crate::game::Game, gd: GuardId, m: GuardId, line: &str) {
    let pts = route_points(g, m).unwrap_or_default();
    let mut i0 = 0usize;
    let mut bd = f64::INFINITY;
    let me = g.ch[g.gd[gd].ch].pos;
    for (i, &p) in pts.iter().enumerate() {
        let d = v3::dist_xz(p, me);
        if d < bd { bd = d; i0 = i; }
    }
    let pos: V3 = if !pts.is_empty() { v3::copy(pts[i0]) } else { v3::copy(post_of(g, m)) };
    let t0 = g.time;
    g.gd[gd].task = Some(GuardTask::CheckOn(GuardTaskCheckOn { pos, who: m, wp: i0 as f64, left: m::max(1.0, pts.len() as f64), t0 }));
    write_fix(&mut g.gd[gd], v3::copy(pos));
    crate::mind::raise(g, gd, 0.42, "guards");
    if g.gd[gd].state == GuardState::Patrol { crate::mind::set_state(g, gd, GuardState::Suspicious, "guards"); g.gd[gd].react_t = 0.8; }
    g.gd[gd].search_t = 0.0;
    no_path(&mut g.gd[gd]);
    g.gd[gd].body_duty = None;
    g.gd[gd].stall_ref = None;
    crate::game::game_say(g, gd, line, Some(true), None, None);
    let at = head_or_pos(g, gd);
    play_rate(g, "radio", at, 0.5, 1.05);
}
/// One frame of the errand's bookkeeping: over if the man has been found, capped, or on to the next waypoint.   (guards.ts checkOnTick)
pub fn check_on_tick(g: &mut crate::game::Game, gd: GuardId) {
    let k = match &g.gd[gd].task { Some(GuardTask::CheckOn(k)) => *k, _ => return };
    let who = k.who;
    if g.gd[who].found || !off_net(g, who) || !g.guards.contains(&who) {
        g.gd[gd].task = None;
        if g.gd[who].found && g.gd[who].state == GuardState::Dead && g.guards.contains(&who) && g.gd[gd].body_duty.is_none() && !g.quiet_utility {
            let t0 = g.time;
            g.gd[gd].body_duty = Some(GuardBodyDuty { body: who, role: GuardBodyDutyRole::Called, stage: GuardBodyDutyStage::To, t: 0.0, t0, spot: None });
            let body = body_pos(g, who);
            write_fix(&mut g.gd[gd], body);
            crate::mind::raise(g, gd, 0.45, "guards");
            g.gd[gd].react_t = 0.5;
            g.gd[gd].search_t = 0.0;
            no_path(&mut g.gd[gd]);
        }
        return;
    }
    if g.time - k.t0 > CHECK_CAP_SECS { check_on_done(g, gd, who, true); return; }
    let near = match g.gd[gd].last_known { Some(l) => v3::dist_xz(l, k.pos) < 0.6, None => false };
    if g.gd[gd].react_t <= 0.0 && near && g.gd[gd].search_t > CHECK_LOOK_SECS { advance_check_on(g, gd); }
}
/// The next of the missing man's waypoints, or, the last one looked at, he is not on his route.   (guards.ts advanceCheckOn)
pub fn advance_check_on(g: &mut crate::game::Game, gd: GuardId) {
    let who = match &g.gd[gd].task { Some(GuardTask::CheckOn(k)) => k.who, _ => return };
    let pts = route_points(g, who).unwrap_or_default();
    let left = match &mut g.gd[gd].task {
        Some(GuardTask::CheckOn(k)) => { k.left -= 1.0; k.left }
        _ => return,
    };
    if left <= 0.0 || pts.is_empty() { check_on_done(g, gd, who, false); return; }
    let d = &mut g.gd[gd];
    let mut pos = [0.0; 3];
    if let Some(GuardTask::CheckOn(k)) = &mut d.task {
        k.wp = js::rem(k.wp + 1.0, pts.len() as f64);
        pos = v3::copy(pts[k.wp as usize]);
        k.pos = pos;
    }
    write_fix(d, v3::copy(pos));
    d.search_t = 0.0;
    no_path(d);
    d.stall_ref = None;
    if left == m::floor(pts.len() as f64 / 2.0) {
        let line = format!("halfway around — nothing yet. {}, if you can hear this, sound off", g.gd[who].callsign);
        crate::game::game_say(g, gd, &line, Some(true), None, None);
        let at = head_or_pos(g, gd);
        play_rate(g, "radio", at, 0.4, 1.05);
    }
}
/// The walk is over without him: the line, and, once per missing man, the floor goes up a step on it.   (guards.ts checkOnDone)
pub fn check_on_done(g: &mut crate::game::Game, gd: GuardId, m: GuardId, capped: bool) {
    g.gd[gd].task = None;
    g.gd[gd].search_t = 0.0;
    { let to = v3::clamp(g.gd[gd].awareness, 0.2, 0.3); crate::mind::set(g, gd, to, "guards"); }
    let line = if capped {
        format!("walked most of it — no sign of {}. he's not at his post", g.gd[m].callsign)
    } else {
        format!("walked his whole route — {} is not at his post, no sign of him anywhere", g.gd[m].callsign)
    };
    crate::game::game_say(g, gd, &line, Some(true), None, None);
    let at = head_or_pos(g, gd);
    play(g, "radio", at, 0.7);
    if !g.gd[m].missing_raised {
        g.gd[m].missing_raised = true;
        let post = post_of(g, m);
        g.alarm.pos[0] = post[0];
        g.alarm.pos[1] = 0.0;
        g.alarm.pos[2] = post[2];
        g.alarm.placed = true;
        let to = if escalation_of(g) >= 1.0 { 2.0 } else { 1.0 };
        raise_escalation(g, to, Some(gd), Some(RaiseEscalationCause::Missing), None);
    }
}
/// (guards.ts updateRollcall's `callerOk`)
fn caller_ok(g: &mut Game, x: Option<GuardId>) -> bool {
    match x {
        None => false,
        Some(x) => !off_net(g, x) && g.gd[x].state != GuardState::Alert && g.gd[x].script.is_none() && g.guards.contains(&x),
    }
}
/// (guards.ts updateRollcall's `silent`)
fn silent(g: &mut Game, x: GuardId) -> bool {
    off_net(g, x) && !g.gd[x].found && checker_for(g, x).is_none()
}
/// The check is over for now: idle, the next one on its clock.
fn rollcall_idle(g: &mut Game) {
    let next = g.time + rollcall_interval(g);
    let r = rollcall_state(g);
    r.stage = RollcallStage::Idle;
    r.next_at = next;
}
/// Once a frame: the radio check's clock and its short exchange.   (guards.ts updateRollcall)
pub fn update_rollcall(g: &mut crate::game::Game, dt: f64) {
    if g.quiet_utility || !g.ai_enabled {
        let r = rollcall_state(g);
        r.stage = RollcallStage::Idle;
        r.next_at = -1.0;
        return;
    }
    if g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()).and_then(|q| q.radio_check) == Some(false) {
        let r = rollcall_state(g);
        r.stage = RollcallStage::Idle;
        r.next_at = -1.0;
        return;
    }
    if rollcall_state(g).next_at < 0.0 {
        let next = g.time + rollcall_interval(g);
        rollcall_state(g).next_at = next;
    }
    let staff: Vec<GuardId> = g.guards.iter().copied().filter(|&x| g.gd[x].team == Team::Guard).collect();
    let living: Vec<GuardId> = staff.iter().copied().filter(|&x| g.gd[x].state != GuardState::Dead).collect();
    match rollcall_state(g).stage {
        RollcallStage::Idle => {
            if escalation_of(g) >= 1.0 {
                let soon = g.time + ROLLCALL_ALARMED_SECS * (1.0 + ROLLCALL_JITTER);
                let r = rollcall_state(g);
                r.next_at = m::min(r.next_at, soon);
            }
            if g.time < rollcall_state(g).next_at { return; }
            if living.is_empty() || g.pl[g.player].down {
                let next = g.time + rollcall_interval(g);
                rollcall_state(g).next_at = next;
                return;
            }
            if g.alarm.episode
                || living.iter().any(|&x| g.gd[x].state == GuardState::Alert)
                || matches!(&g.clearing, Some(c) if c.stage != crate::squad::ClearStage::Done)
                || g.time - g.alarm.raised_at < ROLLCALL_QUIET_AFTER_SECS
            {
                let next = g.time + ROLLCALL_BUSY_RETRY_SECS;
                rollcall_state(g).next_at = next;
                return;
            }
            let caller = match radio_caller(g) {
                Some(c) => c,
                None => {
                    let next = g.time + ROLLCALL_BUSY_RETRY_SECS;
                    rollcall_state(g).next_at = next;
                    return;
                }
            };
            let mut news = false;
            for &x in &staff {
                if x != caller && (!off_net(g, x) || (!g.gd[x].found && !g.gd[x].missing_raised && checker_for(g, x).is_none())) { news = true; break; }
            }
            if !news {
                let next = g.time + rollcall_interval(g);
                rollcall_state(g).next_at = next;
                return;
            }
            let now = g.time;
            let r = rollcall_state(g);
            r.stage = RollcallStage::Called;
            r.t0 = now;
            r.caller = Some(caller);
            r.missing = None;
            r.answered = false;
            r.checks += 1.0;
            crate::game::game_say(g, caller, "radio check — sound off", Some(true), None, None);
            let at = head_or_pos(g, caller);
            play(g, "radio", at, 0.5);
        }
        RollcallStage::Called => {
            let el = g.time - rollcall_state(g).t0;
            if !rollcall_state(g).answered && el >= ROLLCALL_ANSWER_SECS {
                rollcall_state(g).answered = true;
                let caller = rollcall_state(g).caller;
                let mut answers: Vec<String> = Vec::new();
                for &x in &staff {
                    if Some(x) != caller && !off_net(g, x) { answers.push(format!("…{}.", g.gd[x].callsign)); }
                }
                if !answers.is_empty() {
                    let line = format!("radio: {}", answers.join(" "));
                    crate::game::game_msg(g, &line);
                }
                let mut missing: Option<GuardId> = None;
                for &x in &staff {
                    if silent(g, x) && !g.gd[x].missing_raised { missing = Some(x); break; }
                }
                if missing.is_none() {
                    for &x in &staff {
                        if silent(g, x) { missing = Some(x); break; }
                    }
                }
                rollcall_state(g).missing = missing;
                if missing.is_none() { rollcall_idle(g); }
                return;
            }
            let missing = rollcall_state(g).missing;
            if let (Some(who), true) = (missing, el >= ROLLCALL_ASK_SECS) {
                let caller = rollcall_state(g).caller;
                if !caller_ok(g, caller) {
                    let c = radio_caller(g);
                    rollcall_state(g).caller = c;
                }
                if g.gd[who].found || !off_net(g, who) || !g.guards.contains(&who) { rollcall_idle(g); return; }
                let caller = rollcall_state(g).caller;
                if g.gd[who].missing_raised {
                    if let Some(c) = caller {
                        let line = format!("and still nothing from {} — keep looking for him", g.gd[who].callsign);
                        crate::game::game_say(g, c, &line, Some(true), None, None);
                        let at = head_or_pos(g, c);
                        play(g, "radio", at, 0.5);
                    }
                    rollcall_idle(g);
                    return;
                }
                if let Some(c) = caller {
                    let cs = g.gd[who].callsign.clone();
                    let line = format!("{}? … {}, sound off", cs, cs);
                    crate::game::game_say(g, c, &line, Some(true), None, None);
                    let at = head_or_pos(g, c);
                    play(g, "radio", at, 0.55);
                }
                crate::netquiet::net_quiet_asked(g, caller, who, &staff);
                if g.gd[who].missed_at < 0.0 { g.gd[who].missed_at = g.time; }
                rollcall_state(g).stage = RollcallStage::Asked;
            }
        }
        RollcallStage::Asked => {
            let missing = rollcall_state(g).missing;
            let el = g.time - rollcall_state(g).t0;
            let who = match missing {
                Some(w) if off_net(g, w) && !g.gd[w].found && g.guards.contains(&w) && checker_for(g, w).is_none() => w,
                _ => { rollcall_idle(g); return; }
            };
            if el < ROLLCALL_SEND_SECS { return; }
            let caller = rollcall_state(g).caller;
            if !caller_ok(g, caller) {
                let c = radio_caller(g);
                rollcall_state(g).caller = c;
            }
            let checker = match pick_checker(g, who, None) {
                Some(c) => c,
                None => {
                    if el > ROLLCALL_SEND_GIVEUP_SECS {
                        let next = g.time + ROLLCALL_BUSY_RETRY_SECS;
                        let r = rollcall_state(g);
                        r.stage = RollcallStage::Idle;
                        r.next_at = next;
                    }
                    return;
                }
            };
            let caller = rollcall_state(g).caller;
            let cs = g.gd[who].callsign.clone();
            if let Some(c) = caller {
                if c != checker {
                    let line = format!("nothing from {}. {}, go walk his route and check on him", cs, g.gd[checker].callsign);
                    crate::game::game_say(g, c, &line, Some(true), None, None);
                    let at = head_or_pos(g, c);
                    play(g, "radio", at, 0.55);
                }
            }
            let line = if caller == Some(checker) {
                format!("nothing from {} — going to walk his route and check on him", cs)
            } else {
                format!("copy — going to check on {}", cs)
            };
            send_check_on(g, checker, who, &line);
            rollcall_idle(g);
        }
    }
}
/// Lockdown follower, each frame on patrol: when he last had his leader in plain sight, and what he does about losing him.   (guards.ts partnerWatch)
pub fn partner_watch(g: &mut crate::game::Game, gd: GuardId) {
    let l = match g.gd[gd].leader { Some(l) => l, None => return };
    let c = g.gd[gd].ch;
    let lc = g.gd[l].ch;
    let eye = match g.ch[c].bones.get(Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]) };
    let at = if g.gd[l].state != GuardState::Dead {
        match g.ch[lc].bones.get(Bone::Chest) { Some(b) => *b, None => v3::add(g.ch[lc].pos, [0.0, 1.1, 0.0]) }
    } else {
        v3::add(body_pos(g, l), [0.0, 0.4, 0.0])
    };
    let seen = v3::dist(eye, at) < 16.0 && !crate::scene_collision::segment_blocked(g, eye, at, None, None, None);
    if seen || g.gd[gd].leader_seen_t < 0.0 {
        g.gd[gd].leader_seen_t = g.time;
        g.gd[gd].leader_seen_pos = v3::copy(g.ch[lc].pos);
        return;
    }
    let unseen = g.time - g.gd[gd].leader_seen_t;
    let at_spot = v3::dist_xz(g.ch[c].pos, g.gd[gd].leader_seen_pos) < PARTNER_SPOT_M;
    if !g.ai_enabled || unseen < PARTNER_UNSEEN_SECS || (!at_spot && unseen < 2.0 * PARTNER_UNSEEN_SECS) { return; }
    g.gd[gd].leader_seen_t = g.time;
    let (me, him) = (g.gd[gd].callsign.clone(), g.gd[l].callsign.clone());
    if !off_net(g, l) {
        let ask = format!("{}, where'd you go?", him);
        crate::game::game_say(g, gd, &ask, Some(true), None, None);
        let answer = format!("{} — on me, close up", me);
        crate::game::game_say(g, l, &answer, Some(true), None, None);
        play(g, "radio", eye, 0.5);
        return;
    }
    if rollcall_state(g).stage != RollcallStage::Idle || g.gd[l].found || checker_for(g, l).is_some() { return; }
    let line = format!("{}? …I've lost {} — he was just ahead of me. {}, sound off", him, him, him);
    crate::game::game_say(g, gd, &line, Some(true), None, None);
    play(g, "radio", eye, 0.6);
    if g.gd[l].missed_at < 0.0 { g.gd[l].missed_at = g.time; }
    let now = g.time;
    let r = rollcall_state(g);
    r.stage = RollcallStage::Asked;
    r.t0 = now - ROLLCALL_ASK_SECS;
    r.caller = Some(gd);
    r.missing = Some(l);
    r.answered = true;
    r.checks += 1.0;
}

// ---- room clearing

/// planClears' row of a room: its plans (one per door it can be taken from), how near any of its doors is, how it ranks
#[derive(Clone)]
struct RoomRow {
    plans: Vec<crate::scene_level::RoomClearPlan>,
    d: f64,
    tampered: bool,
    inside: bool,
}
/// Locked down: hand the pair the rooms whose doors are nearest the last fix, the tampered ones first.   (guards.ts planClears)
pub fn plan_clears(g: &mut crate::game::Game, force: Option<bool>, cause: Option<crate::guards::PlanClearsCause>) -> bool {
    let force = force.unwrap_or(false);
    let cause = cause.unwrap_or(PlanClearsCause::Search);
    if g.quiet_utility || !g.ai_enabled || escalation_of(g) < 2.0 { return false; }
    if g.clearing.is_some() {
        if !force && cause != PlanClearsCause::Body { return false; }
        crate::squad::clearing_cancel(g);
        g.clearing = None;
    }
    let mut pair: (Option<GuardId>, Option<GuardId>) = (None, None);
    for &gd in &g.guards {
        if g.gd[gd].state == GuardState::Dead || g.gd[gd].held.is_some() { continue; }
        if let Some(l) = g.gd[gd].leader {
            if g.gd[l].state != GuardState::Dead && g.gd[l].held.is_none() { pair = (Some(l), Some(gd)); }
        }
    }
    let (a, b) = pair;
    let a = match a {
        Some(a) => a,
        None => {
            let alive: Vec<GuardId> = g.guards.iter().copied()
                .filter(|&x| g.gd[x].state != GuardState::Dead && g.gd[x].held.is_none() && g.gd[x].team == Team::Guard && g.gd[x].unit.is_none())
                .collect();
            if alive.len() != 1 { return false; }
            alive[0]
        }
    };
    let at = if g.alarm.placed { g.alarm.pos } else { g.ch[g.gd[a].ch].pos };
    let mut by_room: JsMap<String, RoomRow> = JsMap::default();
    for p in g.level.clear_plans.clone().unwrap_or_default() {
        let d = match crate::doors::doors_by_name(g, &p.door) { Some(d) => d, None => continue };
        let k = match &p.room { Some(r) => r.clone(), None => p.door.clone() };
        let dist = v3::dist(g.dr[d].frame_centre, at);
        let tampered = door_tamper(g, d) == Some(DoorTamper::Forced)
            || (g.dr[d].def.locked == Some(true) && crate::doors::door_noticed(g, d) && g.time - crate::doors::door_noticed_t(g, d) < TAMPER_MEMORY_SECS);
        let inside = match p.bounds.clone() { Some(b) => crate::squad::in_bounds(at, Some(b), None), None => false };
        let r = by_room.get_or_insert_with(k, || RoomRow { plans: Vec::new(), d: f64::INFINITY, tampered: false, inside: false });
        r.plans.push(p);
        r.d = m::min(r.d, dist);
        if tampered { r.tampered = true; }
        if inside { r.inside = true; }
    }
    let n = if g.clear_rooms > 0.0 { g.clear_rooms } else if cause == PlanClearsCause::Body { 3.0 } else { 2.0 };
    let now = g.time;
    let mut ranked: Vec<RoomRow> = Vec::new();
    for (k, r) in by_room.iter() {
        let recent = match g.cleared_at.get(k) { Some(t) => now - *t < RECLEAR_SECS, None => false };
        if !recent || r.inside { ranked.push(r.clone()); }
    }
    js::sort_by(&mut ranked, |x, y| js::or((y.tampered as i32 - x.tampered as i32) as f64, x.d - y.d));
    let rooms: Vec<Vec<crate::scene_level::RoomClearPlan>> = ranked.into_iter().take(n as usize).map(|r| r.plans).collect();
    if rooms.is_empty() {
        crate::game::game_note(g, "nothing near to clear — those rooms were done a minute ago");
        return false;
    }
    let mut c = crate::squad::clearing_new(g, &rooms, a, b);
    c.serial = g.serial();
    g.clearing = Some(c);
    true
}
/// `v[0].room ?? v[0].door`
fn room_label(v: &[crate::scene_level::RoomClearPlan]) -> String {
    match &v[0].room { Some(r) => r.clone(), None => v[0].door.clone() }
}
/// Panel: 'clear nearest room now': the fix where the player stands, the floor locked down if it was not.   (guards.ts forceClear)
pub fn force_clear(g: &mut crate::game::Game) {
    if g.quiet_utility { crate::game::game_msg(g, "no clear under the tour"); return; }
    let p = g.ch[g.pl[g.player].ch].pos;
    g.alarm.pos[0] = p[0];
    g.alarm.pos[1] = 0.0;
    g.alarm.pos[2] = p[2];
    g.alarm.placed = true;
    if g.escalation < 2.0 {
        raise_escalation(g, 2.0, None, None, None);
    } else {
        plan_clears(g, Some(true), Some(PlanClearsCause::Panel));
    }
    let line = match &g.clearing {
        None => String::from(if !g.ai_enabled { "no clear: guard AI is off" } else { "no clear: nobody to send / no plans" }),
        Some(c) => {
            let names: Vec<String> = c.rooms.iter().map(|r| room_label(r)).collect();
            format!("clear ordered: {}", names.join(", "))
        }
    };
    crate::game::game_note(g, &line);
}

/// One level down: lockdown to a shorter heightened spell, then calm.   (guards.ts stepDownEscalation)
pub fn step_down_escalation(g: &mut crate::game::Game) {
    let lvl: f64 = if g.escalation >= 2.0 { 1.0 } else { 0.0 };
    g.escalation = lvl;
    g.escalation_t = if lvl >= 1.0 { g.time + STEPDOWN_SECS } else { -1.0 };
    crate::chronicle::chron_alarm(g, lvl, false, Some("time"));
    if lvl < 2.0 { clear_lockdown(g); g.cleared_at.clear(); }
    if lvl == 0.0 { g.alarm.placed = false; }
    let at = if g.alarm.placed { Some(g.alarm.pos) } else { None };
    let sp = spokesman(g, at, None);
    if let Some(sp) = sp {
        crate::game::game_say(g, sp, if lvl >= 1.0 { "ease off — singles, back on your routes; weapons stay out" } else { "stand down, back to normal" }, Some(true), None, None);
        let at = head_or_pos(g, sp);
        play_rate(g, "radio", at, 0.6, 0.95);
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
        if g.gd[gd].state == GuardState::Dead || g.gd[gd].team != Team::Guard { continue; }
        living += 1.0;
        if g.gd[gd].state == GuardState::Alert {
            any_alert = true;
            hunting = true;
            if let Some(lk) = g.gd[gd].last_known { g.alarm.pos[0] = lk[0]; g.alarm.pos[1] = 0.0; g.alarm.pos[2] = lk[2]; g.alarm.placed = true; }
        } else if g.gd[gd].state == GuardState::Search { hunting = true; }
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

/// A man the floor knows is dead.   (guards.ts superviseLockdown's `knownDead`)
fn known_dead(g: &Game, x: GuardId) -> bool {
    g.gd[x].state == GuardState::Dead && (g.gd[x].found || g.alarm.episode || g.gd[x].missed_at >= 0.0)
}

/// Keep the lockdown duties dealt, as far as the men know.   (guards.ts superviseLockdown)
pub fn supervise_lockdown(g: &mut crate::game::Game, dt: f64, living: f64) {
    let (mut follower, mut postman): (Option<GuardId>, Option<GuardId>) = (None, None);
    let mut standing = 0_i32;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].team != Team::Guard || g.gd[gd].unit.is_some() { continue; }
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
        match follower {
            None => true,
            Some(fo) => known_dead(g, g.gd[fo].leader.unwrap()) || (standing >= 3 && living >= 3.0 && posts && postman.is_none()),
        }
    } else {
        follower.is_some() || postman.is_some()
    };
    if let Some(fo) = follower {
        if !redo && g.gd[fo].state != GuardState::Dead {
            let l = g.gd[fo].leader.unwrap();
            let calm = g.gd[fo].state == GuardState::Patrol && g.gd[l].state == GuardState::Patrol;
            let (fc, lc) = (g.gd[fo].ch, g.gd[l].ch);
            g.alarm.apart_t = if calm && v3::dist(g.ch[fc].pos, g.ch[lc].pos) > 8.0 { g.alarm.apart_t + dt } else { 0.0 };
            if g.alarm.apart_t > 5.0 { redo = true; }
        }
    }
    if redo && living >= 1.0 { assign_lockdown(g, true, None); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_write_through_the_rollcall_row_lands() {
        let mut g = Game::blank();
        assert_eq!(rollcall_state(&mut g).next_at, -1.0);
        let r = rollcall_state(&mut g);
        r.stage = RollcallStage::Asked;
        r.checks += 1.0;
        assert!(rollcall_state(&mut g).stage == RollcallStage::Asked);
        assert_eq!(g.m.guards.rollcalls.as_ref().unwrap().checks, 1.0);
    }

}
