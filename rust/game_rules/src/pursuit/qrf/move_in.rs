use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small private helpers
/// where a man with no foe named looks: the player nearest him
pub(super) fn near_man_pos(g: &Game, gd: GuardId) -> V3 { let pl = crate::seats::nearest_or_first(g, &g.ch[g.gd[gd].ch].pos); g.player_pos(pl) }
fn arrived(g: &Game, x: GuardId) -> bool { g.gd[x].script.map_or(false, |s| s.arrived.unwrap_or(false)) }
/// a string the TypeScript tests for truth: null and '' are both false
fn word(s: &Option<String>) -> Option<&str> { s.as_deref().filter(|w| !w.is_empty()) }
/// `!y.door || g.doors.byName(y.door)`
fn way_open(g: &Game, y: &crate::scene_scenarios::ResponseWay) -> bool {
    match word(&y.door) { None => true, Some(d) => crate::doors::doors_by_name(g, d).is_some() }
}
/// `M.door ? g.doors.byName(M.door) : undefined`
fn marks_door(g: &Game, mk: &crate::qrf::Marks) -> Option<DoorId> {
    match word(&mk.door) { None => None, Some(d) => crate::doors::doors_by_name(g, d) }
}
/// `st[0] = want[0]; st[1] = 0; st[2] = want[2]` on his station. In the TypeScript the station IS his script's goal once he has been ordered onto it, so the
/// goal creeps with it and no new leg starts. Returns the station and whether his goal was that very point (by value: the Vec3 is no object here).
fn write_station(g: &mut Game, x: GuardId, want: V3) -> (V3, bool) {
    let old = *station_of(g, x);
    let tied = g.gd[x].script.map_or(false, |s| s.goal == Some(old));
    let st = station_of(g, x);
    st[0] = want[0];
    st[1] = 0.0;
    st[2] = want[2];
    let st = *st;
    if tied {
        if let Some(s) = g.gd[x].script.as_mut() { s.goal = Some(st); }
        // (… and so does the runner's settle watch: squad.ts:81 keeps the goal OBJECT it was made for (`w.goal !== s.goal`), which is this station, so a
        //  station written in place leaves the watch standing with its best and its clock. By value alone every creep of the file made a fresh watch
        //  here, and a man the TypeScript stands still as "stuck short of his mark" went on easing onto it.)
        if let Some(mut w) = g.m.squad.settle_watch.get(x).copied() { if w.goal == old { w.goal = st; g.m.squad.settle_watch.set(x, w); } }
    }
    (st, tied)
}

// ---------------------------------------------------------------- the calls
/// What this call gets: the way in, how many, how long, off the game's own die.
pub(super) fn dealt(g: &mut crate::game::Game, reason: crate::qrf::QrfReason) -> crate::qrf::Dealt {
    let qrf_eta = g.tune.qrf_eta;
    let r = response_of(g);
    let sandbox = v3::clamp(m::round(g.tune.qrf_size), 2.0, 4.0);
    let r = match r {
        Some(r) if reason != crate::qrf::QrfReason::Panel => r,
        _ => return crate::qrf::Dealt { way: 0.0, men: sandbox, eta: qrf_eta },
    };
    let men = r.men.unwrap_or([sandbox, sandbox]);
    let eta = r.eta.unwrap_or([qrf_eta, qrf_eta]);
    let mut open: Vec<usize> = Vec::new();
    for (i, y) in r.ways.iter().enumerate() { if way_open(g, y) { open.push(i); } }
    let a = crate::dice::draw_world(g, "qrf::dealt#1");
    let b = crate::dice::draw_world(g, "qrf::dealt#2");
    let c = crate::dice::draw_world(g, "qrf::dealt#3");
    let mut clear: Vec<usize> = Vec::new();
    for &i in &open { if clear_of_foes(g, r.ways[i].muster[0]) { clear.push(i); } }
    let from = if !clear.is_empty() { clear } else { open };
    let step = g.k.response.eta_step;
    let span = men[1] - men[0] + 1.0;
    let n = from.len() as f64;
    let way = if from.is_empty() { 0.0 } else { from[js::rem(m::floor(a * n), n) as usize] as f64 };
    crate::qrf::Dealt {
        way,
        men: men[0] + js::rem(m::floor(b * span), span),
        eta: v3::clamp(m::round((eta[0] + c * (eta[1] - eta[0])) / step) * step, eta[0], eta[1]),
    }
}

/// The silent net: Sam's gunfire heard, then every man of the shift off the net: a radio check, then the team on the silence alone. Whether it asked now.
fn silent_net(g: &mut crate::game::Game) -> bool {
    if g.qrf.silent_spent { return false; }
    let (mut roster, mut on) = (0, 0);
    for i in 0..g.guards.len() {
        let x = &g.gd[g.guards[i]];
        if x.unit.is_some() || g.ch[x.ch].team != crate::teams::Team::Guard { continue; }
        roster += 1;
        if !g.cond[x.ch].down && x.held.is_none() { on += 1; }
    }
    if roster == 0 || on != 0 || (g.qrf.silent_at < 0.0 && g.time - g.qrf.loud_t >= LOUD_MEMORY_SECS) || !response_here(g) {
        g.qrf.silent_at = -1.0;
        g.qrf.silent_said = false;
        return false;
    }
    if g.qrf.silent_at < 0.0 { g.qrf.silent_at = g.time; note(g, "silent"); }
    let t = g.time - g.qrf.silent_at;
    if !g.qrf.silent_said && t >= SILENT_CHECK_SECS {
        g.qrf.silent_said = true;
        let text = match marks_of(g) {
            None => String::from("two-six, control — radio check. shots reported on your floor and nothing since. any station on two-six, respond"),
            Some(mk) => format!("{}, control — radio check. shots reported {} and nothing since. any station {}, respond", mk.site, mk.at, mk.at),
        };
        control_row(g, &text);
        note(g, "radio-check");
    }
    if t < SILENT_GO_SECS { return false; }
    g.qrf.silent_spent = true;   // (asked for, or refused for want of room: either way once)
    request_qrf(g, crate::qrf::QrfReason::Silent)
}

/// Once a frame, after the guards and the alarm.
pub fn update_qrf(g: &mut crate::game::Game, dt: f64) {
    use crate::qrf::QrfState;
    let rd = raid(g);
    if rd && raid_over(g) && on_the_road(g) { return call_off(g); }
    hear_gunfire(g, rd);
    while !g.qrf.shots_since.is_empty() && g.time - g.qrf.shots_since[0] > GUNFIGHT_WINDOW { g.qrf.shots_since.remove(0); }
    if g.ai_enabled && has_response(g) { read_net(g); }
    if g.qrf.state == QrfState::Idle { return before_the_call(g, rd); }
    // the telegraph (runs on under anything; nothing new is SAID under its flag)
    let mk = marks_of(g);
    if g.qrf.state == QrfState::Requested && g.time - g.qrf.requested_at >= ANSWER_SECS { rolling_call(g, mk.as_ref()); }
    if on_the_road(g) && !g.qrf.said30 && g.qrf.arrive_at - g.time <= CALL30 { thirty_out_call(g, mk.as_ref()); }
    if on_the_road(g) && g.qrf.arrive_at - g.time <= CALL10 { arriving(g); }
    if g.qrf.state == QrfState::Arriving && g.time >= g.qrf.arrive_at { deploy(g); }
    if g.qrf.state == QrfState::Deployed && !g.qrf.everyone_said && g.time - g.qrf.deployed_at > 4.5 { everyone_call(g, mk.as_ref()); }
    if g.qrf.state == QrfState::Arriving || g.qrf.state == QrfState::Deployed { tick_men(g, dt); }
}

/// Asked for and not here yet: nobody stands at the muster so far.
fn on_the_road(g: &crate::game::Game) -> bool {
    g.qrf.state == crate::qrf::QrfState::Requested || g.qrf.state == crate::qrf::QrfState::Inbound
}

/// The run is over with the team still to come: it is all forgotten but the log.
fn call_off(g: &mut crate::game::Game) {
    note(g, "called-off:the run is over");
    let log = core::mem::take(&mut g.qrf.log);
    g.qrf = crate::qrf::Qrf::blank();
    g.qrf.log = log;
}

/// The trigger's memory: gunfire-level reports, each once.
fn hear_gunfire(g: &mut crate::game::Game, rd: bool) {
    use crate::director::EventKind;
    for i in 0..g.events.len() {
        let e = g.events[i];
        if e.level.unwrap_or(0.0) < 1.0 || (e.kind != EventKind::Shot && e.kind != EventKind::GuardShot) { continue; }
        if g.qrf.heard_events.has(e.serial) { continue; }
        g.qrf.heard_events.add(e.serial);
        if e.kind == EventKind::Shot && !rd {
            if shift_in_earshot(g, e.pos) { g.qrf.loud_t = g.time; }
        } else if rd {
            callers_hear(g, e.pos);
        }
        if g.qrf.state == crate::qrf::QrfState::Deployed && g.time - g.qrf.deployed_at > SETTLE_IN_SECS { let t = g.time; g.qrf.shots_since.push(t); }
    }
}

/// A man of the shift, up and in nobody's arm, stands within hearing of that shot.
fn shift_in_earshot(g: &crate::game::Game, at: V3) -> bool {
    g.guards.iter().any(|&x| g.gd[x].unit.is_none() && !crate::condition::down(g, g.gd[x].ch) && g.gd[x].held.is_none() && v3::dist(at, g.pos_of(x)) <= 80.0)
}

/// On a raid each man able to call who hears a round marks when he did, and the first of them starts the raid's clock.
fn callers_hear(g: &mut crate::game::Game, at: V3) {
    for x in callers(g) {
        if !(v3::dist(at, g.pos_of(x)) <= 80.0) { continue; }
        g.qrf.loud_t = g.time;
        if !g.qrf.heard_at.has(x) { let t = g.time; g.qrf.heard_at.set(x, t); }
        if g.qrf.raid_t < -1e8 {
            g.qrf.raid_t = g.time;
            let what = format!("raid-heard:{}", g.gd[x].callsign);
            note(g, &what);
        }
    }
}

/// Nobody has been asked for yet: a raid's callers, the silent net, then gunfire or bodies under an alarm ask for the team.
fn before_the_call(g: &mut crate::game::Game, rd: bool) {
    use crate::qrf::QrfReason;
    if !g.tune.qrf || !live(g) || !g.tune.qrf_silent { g.qrf.silent_at = -1.0; g.qrf.silent_said = false; }
    if !g.tune.qrf || !live(g) { return; }
    if rd {
        if !raid_over(g) && g.time - g.qrf.loud_t < LOUD_MEMORY_SECS && raid_voice(g).is_some() && !request_qrf(g, QrfReason::Gunfire) {
            g.qrf.raid_t = -1e9;
            g.qrf.heard_at.clear();
        }
        return;
    }
    if g.tune.qrf_silent && silent_net(g) { return; }
    if crate::guards::escalation_of(g) < 2.0 { return; }
    if g.time - g.qrf.loud_t < LOUD_MEMORY_SECS { request_qrf(g, QrfReason::Gunfire); }
    else if bodies_found(g) >= 2.0 { request_qrf(g, QrfReason::Bodies); }
}

/// How many are wanted: what was dealt, or the sandbox's own number.
fn team_size(g: &crate::game::Game) -> f64 {
    if g.qrf.size > 0.0 { g.qrf.size } else { v3::clamp(m::round(g.tune.qrf_size), 2.0, 4.0) }
}

/// The call was asked for long enough ago: the team is rolling, and control says how many, by where and how long.
fn rolling_call(g: &mut crate::game::Game, mk: Option<&crate::qrf::Marks>) {
    use crate::qrf::{QrfReason, QrfState};
    g.qrf.state = QrfState::Inbound;
    let n = m::min(4.0, team_size(g));
    let secs = m::round(g.qrf.arrive_at - g.time);
    let (n, secs) = (js::num_str(n), js::num_str(secs));
    let silent = g.qrf.reason == Some(QrfReason::Silent);
    let text = match mk {
        None => if silent { format!("all stations — QRF is rolling to two-six: {} men, west stairwell, {} seconds", n, secs) }
            else { format!("copy two-six. QRF is rolling — {} men, west stairwell, {} seconds. lock down and hold what you've got", n, secs) },
        Some(mk) => {
            let said = format!(" {}.", brief(g));
            if silent { format!("all stations — {} is rolling to {}: {} men, by {}, {} seconds.{}", mk.team, mk.site, n, mk.name, secs, said) }
            else { format!("copy {}. {} is rolling — {} men, by {}, {} seconds.{} lock down and hold what you've got", mk.site, mk.team, n, mk.name, secs, said) }
        }
    };
    control(g, &text);
    note(g, "control");
}

/// Thirty seconds out, said once, and only for a team that took longer than that to come.
fn thirty_out_call(g: &mut crate::game::Game, mk: Option<&crate::qrf::Marks>) {
    g.qrf.said30 = true;
    if !(g.qrf.arrive_at - g.qrf.requested_at > CALL30 + 5.0) { return; }
    let silent = g.qrf.reason == Some(crate::qrf::QrfReason::Silent);
    let text = match mk {
        None => if silent { String::from("all stations — QRF is thirty seconds out from two-six, coming up the west stairs") }
            else { String::from("two-six, QRF is thirty seconds out — coming up the west stairs. check your fire on that side") },
        Some(mk) => if silent { format!("all stations — {} is thirty seconds out from {}, {}", mk.team, mk.site, mk.coming) }
            else { format!("{}, {} is thirty seconds out — {}. check your fire on that side", mk.site, mk.team, mk.coming) },
    };
    control(g, &text);
    note(g, "thirty");
}

/// The team is in, and control says that's all there is, if anybody came.
fn everyone_call(g: &mut crate::game::Game, mk: Option<&crate::qrf::Marks>) {
    g.qrf.everyone_said = true;
    if g.qrf.men.is_empty() { return; }
    let text = match mk {
        None => String::from("all stations — that's the response team on your floor, and that's everyone we've got tonight. make it count"),
        Some(mk) => format!("all stations — that's {} with you now, and that's everyone we've got tonight. make it count", mk.team),
    };
    control(g, &text);
}

/// T−10: the men exist from here, stood at the muster, the lead's word on the net.
fn arriving(g: &mut crate::game::Game) {
    use crate::qrf::{Phase, QrfState};
    g.qrf.state = QrfState::Arriving;
    re_deal(g);
    let mk = marks_of(g).unwrap();
    let want = team_size(g);
    let room = room_for(g);
    let n = m::min(want, room);
    let door = marks_door(g, &mk);
    let yaw = v3::yaw_to(mk.muster[0], mk.entry[0]);
    let mut i = 0usize;
    while (i as f64) < n {
        let at = match door {
            Some(d) if i == 0 => {
                let side = crate::doors::doors_side(g, d, mk.muster[0]);
                crate::handleside::tool_spot(g, d, side, mk.muster[0], None).map_or(mk.muster[0], |s| s.pos)
            }
            _ => match mk.muster.get(i) {
                Some(&p) => p,
                None => { let what = format!("spawn-failed:{}:no muster mark", i); note(g, &what); break; }   // (the TypeScript's spawnUnit throws on the missing mark: caught the same way)
            },
        };
        let team = side_of_team(g);
        let budget = match budget_of(g) { crate::qrf::BudgetOf::Arm0(b) => Some(b.budget), crate::qrf::BudgetOf::Arm1(_) => None };
        let opts = crate::guards::UnitOpts {
            team, pos: at, yaw: Some(yaw), weapon: Some(crate::weapons::WeaponId::Aks74u), name: Some(format!("{}-{}", mk.tag, i + 1)), light: Some(true),
            tint: Some(crate::consts::qrf_tint()), tint2: Some(crate::consts::qrf_tint2()), carbine_rig: None, pair: None, budget,
        };
        let gd = match crate::guards::spawn_unit(g, &opts) {
            Ok(gd) => gd,
            Err(e) => { let short: String = e.chars().take(40).collect(); let what = format!("spawn-failed:{}:{}", i, short); note(g, &what); break; }
        };
        let seq = g.event_seq;
        crate::control::set_hold(g, gd, false, "qrf");
        g.gd[gd].trained = true;
        crate::mind::set_state(g, gd, crate::game::GuardState::Search, "qrf");
        crate::mind::set(g, gd, HUNT_AWARENESS, "qrf");
        let first = crate::seats::faced(g, gd);
        crate::mind::foe(g, gd, Some(first), "qrf");
        g.gd[gd].heard_up_to = seq;
        // at the high port at the muster, facing the door
        crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(None), face: Some(Some(yaw)), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)), aim_at: Some(None), crouch: Some(Some(false)), ..Default::default() });
        g.qrf.men.push(gd);
        i += 1;
    }
    g.qrf.bangs = g.qrf.men.len() as f64;
    let count = g.qrf.men.len();
    if n < want { let what = format!("capped:{}/{}", count, js::num_str(want)); note(g, &what); }
    if count == 0 {
        g.qrf.state = QrfState::Deployed;
        g.qrf.deployed_at = g.time;
        let text = format!("{} — {} is held up, nobody's coming. you're on your own", mk.site, mk.team);
        control(g, &text);
        note(g, "nobody");
        return;
    }
    if mk.stairs { crate::audio_iface::game_audio_like_play(g, "stairBoots", Some([mk.muster[0][0], 0.3, mk.muster[0][2]]), Some(0.75), None); }
    let clear = match word(&mk.clear) { Some(c) => format!(" {},", c), None => String::new() };
    let lead = g.qrf.men[0];
    let text = format!("{} at {} — {} of us, ten seconds.{} check your fire", mk.tag, mk.name, count, clear);
    radio(g, Some(lead), &text);
    let what = format!("ten:{}", count);
    note(g, &what);
    g.qrf.phase = Phase::Landing;
    g.qrf.phase_t = 0.0;
}

/// Ten seconds out: an enemy now stands at the dealt way's muster: another clear way is taken, and control says so.
fn re_deal(g: &mut crate::game::Game) {
    let r = match response_of(g) { Some(r) => r, None => return };
    if r.ways.len() < 2 { return; }
    let way = g.qrf.way as usize;
    if clear_of_foes(g, r.ways[way].muster[0]) { return; }
    let mut other = None;
    for (i, y) in r.ways.iter().enumerate() {
        if i != way && way_open(g, y) && clear_of_foes(g, y.muster[0]) { other = Some(i); break; }
    }
    let other = match other { Some(i) => i, None => return };
    g.qrf.way = other as f64;
    let mk = marks_of(g).unwrap();
    let text = format!("{} — change of plan: {} is {}", mk.site, mk.team, mk.coming);
    control(g, &text);
    let what = format!("re-dealt:way {}", other);
    note(g, &what);
}

/// T: the bar goes down, the leaf is thrown open off its real hinge, and they file in.
fn deploy(g: &mut crate::game::Game) {
    g.qrf.state = crate::qrf::QrfState::Deployed;
    g.qrf.deployed_at = g.time;
    g.qrf.phase = crate::qrf::Phase::File;
    g.qrf.phase_t = 0.0;
    g.qrf.filed = 0.0;
    let mk = marks_of(g).unwrap();
    let door = marks_door(g, &mk);
    let lead = alive(g).first().copied();
    if let (Some(d), Some(l)) = (door, lead) { fling_open(g, d, l); }
    if let Some(d) = door { if mk.stairs { let at = g.dr[d].frame_centre; crate::audio_iface::game_audio_like_play(g, "stairDoor", Some(at), Some(1.0), None); } }
    if crate::mission::mission_live(g) {
        g.mission.stats.qrf = m::max(g.mission.stats.qrf, 1.0);
        g.mission.stats.qrf_at = g.mission.stats.elapsed;
        g.mission.stats.qrf_who = Some(mk.team.clone());
    }
    let text = format!("{} {} — {} in by {}, moving as one element", mk.tag, mk.at, alive(g).len(), mk.name);
    radio(g, lead, &text);
    note(g, "door");
}

/// Throw a shut (or nearly shut) leaf open away from `by` at a kick's pace, breaking nothing.
fn fling_open(g: &mut crate::game::Game, d: DoorId, by: GuardId) {
    use crate::doors::{door_pos, doors_side};
    if m::abs(g.dr[d].leaf.angle) > 1.0 { return; }
    let side = doors_side(g, d, g.pos_of(by));
    g.dr[d].leaf.latched = false;
    g.dr[d].leaf.closing = false;
    g.dr[d].leaf.vel = -side * 7.5;
    g.dr[d].leaf.since_bash = 0.0;
    g.dr[d].leaf.quiet_t = -0.5;
    g.dr[d].leaf.creak_armed = false;
    let who = g.char_id(by);
    g.dr[d].leaf.last_who = who;
    let at = door_pos(g, d);
    crate::audio_iface::game_audio_like_play(g, "doorBash", Some(at), Some(0.9), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.85 }));
}

// ---------------------------------------------------------------- the men, every frame
/// The element's frame: housekeeping, the door pulled to, the tier outside, the phase, then the scripted men's senses.
fn tick_men(g: &mut crate::game::Game, dt: f64) {
    use crate::qrf::{Phase, QrfState};
    g.qrf.phase_t += dt;
    forget_the_removed(g);
    man_down(g);
    let mut el = on_the_element(g);
    stay_on_the_hunt(g);
    if g.qrf.state == QrfState::Deployed && !g.qrf.door_pulled && g.time - g.qrf.deployed_at > 1.5 { pull_the_door_to(g); }
    if g.qrf.perim.is_some() { tick_perimeter(g, dt); } else if g.qrf.state == QrfState::Deployed { maybe_perimeter(g); }
    if el.is_empty() {
        if g.qrf.phase != Phase::Fight { drop_drill(g); g.qrf.phase = Phase::Fight; note(g, "wiped"); }
        return;
    }
    if g.qrf.state == QrfState::Arriving { return ears_at_the_muster(g, &el, dt); }
    match g.qrf.phase {
        Phase::File => phase_file(g, &el),
        Phase::Fan => if g.qrf.phase_t > ENTRY_HOLD { pick_target(g, &mut el, true); },
        Phase::Move => phase_move(g, &mut el, dt),
        Phase::Stack => phase_stack(g, &el),
        Phase::Breach => phase_breach(g, &mut el),
        Phase::Bang => phase_bang(g, &el),
        Phase::Flow => phase_flow(g, &mut el),
        Phase::Drill => phase_drill(g, &mut el),
        Phase::Called => if g.qrf.phase_t > 1.2 { hold360(g, &el); },
        Phase::Hold => if g.qrf.phase_t > HOLD360_SECS { pick_target(g, &mut el, false); },
        Phase::Fight => { phase_fight(g, &mut el); return; }   // the AI has them: no script senses to run
        Phase::Regroup => if g.qrf.phase_t > 1.5 || el.iter().all(|&x| arrived(g, x)) { pick_target(g, &mut el, true); },
        Phase::Landing => {}
    }
    scripted_senses(g, &el, dt);
}

/// Men taken out of the world from under us are forgotten.
fn forget_the_removed(g: &mut crate::game::Game) {
    let guards = &g.guards;
    g.qrf.men.retain(|x| guards.contains(x));
    if let Some(p) = g.qrf.perim.as_mut() { p.men.retain(|r| guards.contains(&r.gd)); }
}

/// The living men who move as the element: not the ones posted outside.
fn on_the_element(g: &crate::game::Game) -> Vec<GuardId> {
    let mut el = alive(g);
    el.retain(|x| !g.qrf.perim.as_ref().map_or(false, |p| p.men.iter().any(|r| r.gd == *x)));
    el
}

/// Trained: short beats; nobody of theirs talks himself calm, or takes up a body or a chore.
fn stay_on_the_hunt(g: &mut crate::game::Game) {
    use crate::game::GuardState;
    let mut all = alive(g);
    if let Some(p) = g.qrf.perim.as_ref() { for r in &p.men { if !crate::condition::down(g, g.gd[r.gd].ch) { all.push(r.gd); } } }
    for x in all {
        let cap = g.k.trained.react_cap;
        if !crate::condition::is_blind(g, g.gd[x].ch) && !crate::condition::is_folded(g, g.gd[x].ch) && g.gd[x].react_t > cap { g.gd[x].react_t = cap; }
        if g.gd[x].script.is_none() && (g.gd[x].state == GuardState::Patrol || g.gd[x].state == GuardState::Suspicious) {
            crate::mind::set_state(g, x, GuardState::Search, "qrf");
            crate::mind::raise(g, x, HUNT_AWARENESS, "qrf");
            g.gd[x].search_t = 0.0;
        }
        g.gd[x].body_duty = None;
        g.gd[x].task = None;
    }
}

/// DOORSTAY: the last man through pulls the stair door to, once they are all on the floor and clear of it.
fn pull_the_door_to(g: &mut crate::game::Game) {
    let mk = marks_of(g);
    let d = mk.as_ref().and_then(|k| marks_door(g, k));
    let up = alive(g);
    let open = d.filter(|&d| !(g.dr[d].leaf.off || g.dr[d].leaf.latched || g.dr[d].def.closer == Some(false) || up.is_empty()));
    let Some(d) = open else { g.qrf.door_pulled = true; return };
    let land = crate::doors::doors_pull_face(g, d, mk.as_ref().unwrap().muster[0]);
    let Some((last, ld)) = last_man_through(g, d, land, &up) else { return };
    g.qrf.door_pulled = true;
    if ld < 6.0 {
        let who = g.char_id(last);
        crate::doors::doors_pull_to(g, d, who);
        note(g, "door-to");
    }
}

/// The man nearest the door and how far off he is, if all of them are past it and clear of its frame.
fn last_man_through(g: &crate::game::Game, d: DoorId, land: bool, up: &[GuardId]) -> Option<(GuardId, f64)> {
    let (mut last, mut ld, mut through) = (None, f64::INFINITY, true);
    let frame = g.dr[d].frame_centre;
    for &x in up {
        let p = g.pos_of(x);
        let dd = dist_xz(p, frame);
        if crate::doors::doors_pull_face(g, d, p) == land || dd < 1.5 { through = false; }
        if dd < ld { ld = dd; last = Some(x); }
    }
    if through { last.map(|l| (l, ld)) } else { None }
}

/// At the muster: ears open, nothing else. A man who sees him there takes them all to the fight, where no door stands in the way.
fn ears_at_the_muster(g: &mut crate::game::Game, el: &[GuardId], dt: f64) {
    let mut seer = None;
    for &x in el { if perceive_scripted(g, x, dt) && seer.is_none() { seer = Some(x); } }
    let Some(seer) = seer else { return };
    let no_door = marks_of(g).map_or(true, |k| word(&k.door).is_none());
    if no_door {
        note(g, "seen-at-the-muster");
        deploy(g);
        to_fight(g, el, seer);
    }
}

/// The scripted men's senses: eyes on him and all go to the fight, unless still filing with him not close.
fn scripted_senses(g: &mut crate::game::Game, el: &[GuardId], dt: f64) {
    let mut contact = None;
    for &x in el {
        if g.gd[x].script.is_none() { continue; }
        if perceive_scripted(g, x, dt) && contact.is_none() { contact = Some(x); }
    }
    let Some(c) = contact else { return };
    let near = g.qrf.phase != crate::qrf::Phase::File || {
        let foe = match g.gd[c].foe { Some(f) => g.ch[f].pos, None => near_man_pos(g, c) };
        dist_xz(g.pos_of(c), foe) < 8.0
    };
    if near { return to_fight(g, el, c); }
    g.qrf.fix = Some(v3::copy(g.gd[c].last_known.unwrap_or(near_man_pos(g, c))));
    g.qrf.fix_t = g.time;
    g.qrf.cue_rush = true;
    crate::mind::cap(g, c, 0.84, "qrf");
}

/// The senses updateGuard would have run for a man the runner drives. A noise is a cue; eyes on him is contact (true).
pub fn perceive_scripted(g: &mut crate::game::Game, m: GuardId, dt: f64) -> bool {
    crate::seats::standing_for(g, m, &mut |g| perceive_here(g, m, dt))
}
fn perceive_here(g: &mut crate::game::Game, m: GuardId, dt: f64) -> bool {
    if !g.ai_enabled { return false; }
    let ids = ids_of_theirs(g);
    number_the_events(g);
    let fix_before = g.gd[m].last_known;
    hear_all_but_their_own(g, m, &ids);
    g.gd[m].heard_up_to = g.event_seq;
    let mut sight = eyes_on_the_player(g, m, dt);
    if player_at_arms_length(g, m) { sight.seen = true; }
    if crate::guards::standoff_perceive(g, m, None) { sight.seen = true; }
    g.gd[m].saw_player_this_frame = sight.seen;
    g.gd[m].saw_foe_this_frame = sight.seen;
    if sight.seen { let c = crate::seats::faced(g, m); crate::mind::foe(g, m, Some(c), "qrf"); }
    if !sight.seen { eyes_on_his_foes(g, m, dt, &mut sight); }
    if g.gd[m].awareness > g.gd[m].witness.peak_awareness { g.gd[m].witness.peak_awareness = g.gd[m].awareness; }
    if sight.seen { return true; }
    if sight.glimpse { return false; }   // (the meter is filling on him: no cue, no clamp)
    noise_moves_the_fix(g, m, fix_before);
    let to = v3::clamp(g.gd[m].awareness, HUNT_AWARENESS * 0.85, 0.8);
    crate::mind::set(g, m, to, "qrf");
    if g.time - g.gd[m].saw_body_t > 6.0 {
        if let Some(o) = crate::guards::body_in_view(g, m) { body_found(g, m, o); }
    }
    false
}

/// What his eyes made of it this frame: `seen` is contact; `glimpse` is the meter filling on somebody.
#[derive(Clone, Copy)]
struct Sight { seen: bool, glimpse: bool }

/// The characters' ids of the men of the response, the element and the tier outside.
fn ids_of_theirs(g: &crate::game::Game) -> Vec<f64> {
    let mut ids: Vec<f64> = Vec::new();
    for i in 0..g.qrf.men.len() { ids.push(g.char_id(g.qrf.men[i])); }
    if let Some(p) = g.qrf.perim.as_ref() { for r in &p.men { ids.push(g.char_id(r.gd)); } }
    ids
}

/// Every event gets its number, the first time anybody listens for it.
fn number_the_events(g: &mut crate::game::Game) {
    for i in 0..g.events.len() {
        if g.events[i].id.is_none() { g.event_seq += 1.0; g.events[i].id = Some(g.event_seq); }
    }
}

/// His hearing runs on the events less the element's own (and a can of theirs going off where it landed); what it writes on an event (`announced`) is written back.
fn hear_all_but_their_own(g: &mut crate::game::Game, m: GuardId, ids: &[f64]) {
    let can_at = match g.qrf.can { Some(c) if !g.qrf.can_land_t.is_nan() => Some(g.tab.item[c].pos), _ => None };
    let all = core::mem::take(&mut g.events);
    let mut kept: Vec<usize> = Vec::new();
    let mut lent = Vec::new();
    for (i, e) in all.iter().enumerate() {
        let own = e.who.map_or(false, |w| ids.contains(&w)) || (e.kind == crate::director::EventKind::Bang && can_at.map_or(false, |c| dist_xz(e.pos, c) < 1.5));
        if !own { kept.push(i); lent.push(*e); }
    }
    g.events = lent;
    crate::guards::hearing_check(g, m, None, None, None);
    let lent = core::mem::replace(&mut g.events, all);
    for (j, &i) in kept.iter().enumerate() { g.events[i] = lent[j]; }
}

/// His meter fills at that rate on what he sees there and his fix goes to it; a quick rate counts as having seen it.
fn sighted(g: &mut crate::game::Game, m: GuardId, at: V3, rate: f64, dt: f64) {
    let t = g.time;
    let to = v3::clamp(g.gd[m].awareness + rate * dt, 0.0, 1.0);
    crate::mind::set(g, m, to, "qrf");
    crate::mind::fix(g, m, Some(v3::copy(at)), "qrf");
    g.gd[m].sight_pos = v3::copy(at);
    g.gd[m].sight_t = t;
    if rate > 0.25 { g.gd[m].last_seen_t = t; }
}

/// The player in his eye, at a rate that counts: seen once the meter is high enough.
fn eyes_on_the_player(g: &mut crate::game::Game, m: GuardId, dt: f64) -> Sight {
    let nothing = Sight { seen: false, glimpse: false };
    let vis = crate::guards::can_see(g, m);
    if !vis.visible { return nothing; }
    let rate = crate::guards::sight_gain(g, m, vis.dist);
    if !(rate > 0.05) { return nothing; }
    let pp = g.seat_pos(&crate::seats::here(g));
    sighted(g, m, pp, rate, dt);
    Sight { seen: g.gd[m].awareness >= 0.85, glimpse: true }
}

/// The player standing up against him, seen or not: he knows at once.
fn player_at_arms_length(g: &mut crate::game::Game, m: GuardId) -> bool {
    let pp = g.seat_pos(&crate::seats::here(g));
    let gp = g.pos_of(m);
    let close = !crate::condition::down(g, g.pl[g.player].ch) && !g.player_invisible && v3::dist_xz(pp, gp) < 0.62;
    if !close { return false; }
    crate::mind::set(g, m, 1.0, "qrf");
    crate::mind::fix(g, m, Some(v3::copy(pp)), "qrf");
    g.gd[m].last_seen_t = g.time;
    true
}

/// The enemy is whoever is against them on this level, seen by the same eye at the same rate; the first he is sure of is his foe.
fn eyes_on_his_foes(g: &mut crate::game::Game, m: GuardId, dt: f64, sight: &mut Sight) {
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if crate::condition::down(g, g.gd[o].ch) || !g.team(m).foe_of(g.team(o)) || crate::rails::rails_down(g, o) { continue; }
        let oc = g.gd[o].ch;
        let vo = crate::guards::can_see_char(g, m, oc);
        if !vo.visible { continue; }
        let rate = crate::guards::sight_gain_on(g, m, o, vo.dist);
        if rate <= 0.05 { continue; }
        sight.glimpse = true;
        let op = g.ch[oc].pos;
        sighted(g, m, op, rate, dt);
        if g.gd[m].awareness >= 0.85 {
            sight.seen = true;
            crate::mind::foe(g, m, Some(oc), "qrf");
            g.gd[m].saw_foe_this_frame = true;
            return;
        }
    }
}

/// A cue: a noise reached his ears this frame and put his fix somewhere new, so the element's fix follows it.
fn noise_moves_the_fix(g: &mut crate::game::Game, m: GuardId, fix_before: Option<V3>) {
    let (h_last, h_shot, h_kick) = { let h = &g.gd[m].witness.heard; (h.last_t, h.last_shot_t, h.kick) };
    let Some(lk) = g.gd[m].last_known else { return };
    if h_last >= g.time - 1e-6 && fix_before.map_or(true, |f| dist_xz(f, lk) > 1.0) {
        g.qrf.fix = Some(v3::copy(lk));
        g.qrf.fix_t = g.time;
        g.qrf.cue_rush = h_shot >= g.time - 1e-6 || (h_kick > 0.0 && h_last >= g.time - 1e-6);
    }
}

/// A colleague down in his view: found, said once, and they push on.
fn body_found(g: &mut crate::game::Game, m: GuardId, o: GuardId) {
    let t = g.time;
    g.gd[m].saw_body_t = t;
    let victim = g.gd[o].callsign.clone();
    let breathing = crate::game::is_breathing(g, o);
    g.gd[m].witness.saw_body = Some(crate::game::WitnessSawBody { victim: victim.clone(), breathing });
    let bp = crate::guards::body_pos(g, o);
    let n = bodies_found_there(g, m, o, bp);
    if n != 0 {
        let room = crate::dialogue::room_name(g, bp);
        let many = if n > 1 { format!(" — {} of them", n) } else { String::new() };
        let how = if crate::game::is_breathing(g, o) { "out cold, breathing" } else { "dead" };
        let text = format!("{} down in {}{}, {}. pushing on", victim, room, many, how);
        radio(g, Some(m), &text);
    }
    let what = format!("found:{}", victim);
    note(g, &what);
}

/// How many of his side not yet found lie there to be counted: that body, and any within 3 m of it he has a line to.
fn bodies_found_there(g: &mut crate::game::Game, m: GuardId, o: GuardId, bp: V3) -> usize {
    let mc = g.gd[m].ch;
    let eye = match g.ch[mc].bones.get(crate::character::Bone::Head) { Some(&b) => b, None => v3::add(g.ch[mc].pos, [0.0, 1.65, 0.0]) };
    let team = g.team(m);
    let mut n = 0;
    for i in 0..g.guards.len() {
        let x = g.guards[i];
        if !crate::condition::down(g, g.gd[x].ch) || g.gd[x].found || g.team(x) != team { continue; }
        let take = x == o || {
            let xp = crate::guards::body_pos(g, x);
            dist_xz(xp, bp) < 3.0 && !crate::scene_collision::segment_blocked(g, eye, v3::add(xp, [0.0, 0.3, 0.0]), None, None, None)
        };
        if take {
            g.gd[x].found = true;
            crate::mission::tally(g, crate::mission::TallyK::Bodies);
            n += 1;
        }
    }
    n
}

/// A response man went down since last frame: the nearest of theirs with him in view says so on the net.
fn man_down(g: &mut crate::game::Game) {
    use crate::game::GuardState;
    let mut theirs = g.qrf.men.clone();
    if let Some(p) = g.qrf.perim.as_ref() { for r in &p.men { theirs.push(r.gd); } }
    for &d in &theirs {
        let key = crate::Num(g.char_id(d));
        if !crate::condition::down(g, g.gd[d].ch) || g.qrf.seen_ids.has(key) { continue; }
        g.qrf.seen_ids.add(key);
        g.qrf.lost += 1.0;
        let bp = crate::guards::body_pos(g, d);
        let (mut sp, mut bd) = (None, f64::INFINITY);
        for &o in &theirs {
            if o == d || crate::condition::down(g, g.gd[o].ch) || !g.guards.contains(&o) { continue; }
            let dd = dist_xz(g.pos_of(o), bp);
            if dd < bd && (dd < 6.0 || crate::guards::point_in_view(g, o, v3::add(bp, [0.0, 0.4, 0.0]), Some(14.0), Some(100.0))) { bd = dd; sp = Some(o); }
        }
        let callsign = g.gd[d].callsign.clone();
        if let Some(s) = sp {
            if !g.gd[d].found { g.gd[d].found = true; crate::mission::tally(g, crate::mission::TallyK::Bodies); }
            let breathing = crate::game::is_breathing(g, d);
            g.gd[s].witness.saw_body = Some(crate::game::WitnessSawBody { victim: callsign.clone(), breathing });
            let room = crate::dialogue::room_name(g, bp);
            let him = if g.gd[d].down_by == Some(crate::game::DownBy::Player) { " it's him — on me!" } else { "" };
            let text = format!("man down — {} is hit, {}!{}", callsign, room, him);
            radio(g, Some(s), &text);
        }
        let what = format!("man-down:{}{}", callsign, if sp.is_some() { "" } else { ":unseen" });
        note(g, &what);
    }
    // the run ending on one of THEIR rounds
    let down = crate::seats::all_down(g);
    if down && !g.qrf.sam_was_down && !g.qrf.got_sam {
        for &o in &theirs {
            if let Some(ls) = g.arms[g.gd[o].ch].aim.last_shot.as_ref() {
                if m::abs(ls.t - g.time) < 1e-6 {
                    g.qrf.got_sam = true;
                    let what = format!("got-sam:{}", g.gd[o].callsign);
                    note(g, &what);
                    break;
                }
            }
        }
    }
    g.qrf.sam_was_down = down;
}

// ---------------------------------------------------------------- entry
/// One man through the doorway every FILE_GAP, each to his fan mark facing his sector.
fn phase_file(g: &mut crate::game::Game, el: &Vec<GuardId>) {
    let due = m::min(el.len() as f64, m::floor(g.qrf.phase_t / FILE_GAP) + 1.0);
    let mk = marks_of(g).unwrap();
    while g.qrf.filed < due {
        let i = g.qrf.filed as usize;
        let x = match el.get(i) { Some(&x) => x, None => break };
        crate::mind::set_state(g, x, crate::game::GuardState::Search, "qrf");
        crate::mind::raise(g, x, HUNT_AWARENESS, "qrf");
        let e = mk.entry.get(i).copied().unwrap_or(mk.entry[0]);
        let ef = mk.entry_face.get(i).copied().unwrap_or(m::PI / 2.0);
        route(g, x, &[e], Some(crate::squad::OrderPatch { speed: Some(Some(2.4)), face: Some(Some(ef)), stance: Some(Some(crate::character::Stance::None)), upper: Some(Some(crate::character::UpperMode::Aim)), aim_at: Some(Some(sector_point(e, ef, None, None))), crouch: Some(Some(i == 3)), strafe: Some(Some(false)), ..Default::default() }));
        g.qrf.filed += 1.0;
        let what = format!("in:{}", i + 1);
        note(g, &what);
    }
    for &x in el.iter() { advance(g, x); }
    let all_in = g.qrf.filed >= el.len() as f64 && el.iter().all(|&x| {
        let p = g.pos_of(x);
        arrived(g, x) || dist_xz(p, g.gd[x].script.and_then(|s| s.goal).unwrap_or(p)) < 0.9
    });
    if all_in || g.qrf.phase_t > FILE_GAP * el.len() as f64 + m::max(6.0, dist_xz(mk.muster[0], mk.entry[0]) / 1.6) {
        g.qrf.phase = crate::qrf::Phase::Fan;
        g.qrf.phase_t = 0.0;
        note(g, "fan");
    }
}

/// A point `d` along the yaw from `p` at height `y`: where a man's sector looks.
pub(super) fn sector_point(p: V3, yaw: f64, d: Option<f64>, y: Option<f64>) -> V3 {
    let d = d.unwrap_or(5.0);
    let y = y.unwrap_or(1.0);
    [p[0] + m::sin(yaw) * d, y, p[2] + m::cos(yaw) * d]
}

// ---------------------------------------------------------------- targeting
/// Choose where the element goes next and set it moving. `fresh`: re-read the floor's fix first.
pub(super) fn pick_target(g: &mut crate::game::Game, el: &mut Vec<GuardId>, fresh: bool) {
    use crate::qrf::{Phase, Target, TargetKind};
    let bf = fix_of(g);
    if let Some(b) = bf {
        if (fresh && !(g.qrf.fix.is_some() && b.t < g.qrf.fix_t - 1e-6)) || g.qrf.fix.is_none() || b.t > g.qrf.fix_t {
            g.qrf.fix = Some(b.pos);
            g.qrf.fix_t = m::max(g.qrf.fix_t, b.t);
        }
    }
    let fix: V3 = match g.qrf.fix {
        Some(f) => f,
        None => v3::copy(marks_of(g).and_then(|k| k.entry.first().copied()).unwrap_or(g.pos_of(el[0]))),
    };
    let plans = g.level.clear_plans.clone().unwrap_or_default();
    let mut rooms: JsMap<String, Vec<crate::scene_level::RoomClearPlan>> = JsMap::new();
    for p in &plans {
        let k = p.room.clone().unwrap_or_else(|| p.door.clone());
        rooms.get_or_insert_with(k, Vec::new).push(p.clone());
    }
    let holds = |p: &crate::scene_level::RoomClearPlan| p.bounds.is_some() && crate::squad::in_bounds(fix, p.bounds, None);
    let mut best: Option<(String, &Vec<crate::scene_level::RoomClearPlan>, f64)> = None;
    for (k, ps) in rooms.iter() {
        let spent = g.tune.qrf_squad && g.qrf.spent.as_ref().map_or(false, |s| s.room == k && s.fix_t >= g.qrf.fix_t);
        let is_in = ps.iter().any(|p| holds(p)) && !spent;
        let t = m::max(g.qrf.cleared.get(k.clone()).copied().unwrap_or(-1e9), g.cleared_at.get(k.clone()).copied().unwrap_or(-1e9));
        let recent = g.time - t < RECLEAR_SECS;
        if recent && !is_in { continue; }
        let mut d = f64::INFINITY;
        for p in ps {
            if let Some(door) = crate::doors::doors_by_name(g, &p.door) {
                let dc = g.dr[door].frame_centre;
                d = m::min(d, crate::places::on_foot(g, fix, [dc[0], 0.0, dc[2]]));
            }
        }
        if is_in { d = -1.0; }
        if best.as_ref().map_or(true, |b| d < b.2) { best = Some((k, ps, d)); }
    }
    let from = g.pos_of(el[0]);
    let prev = g.qrf.target.clone();
    let prev_phase = g.qrf.phase;
    // the fix out on the open floor and not where they already stand: close on IT first
    let fix_open = !plans.iter().any(|p| holds(p));
    let point = |g: &mut Game| {
        let room = crate::dialogue::room_name(g, fix);
        let pos = nearest_free(g, fix).unwrap_or(v3::copy(fix));
        Target { kind: TargetKind::Point, plan: None, room, pos, word: None }
    };
    let held_here = prev.as_ref().map_or(false, |t| t.kind == TargetKind::Point && dist_xz(t.pos, fix) < RETARGET_M && prev_phase == Phase::Hold);
    let mut next: Target = if fix_open && dist_xz(from, fix) > 5.0 && !held_here {
        point(g)
    } else if let Some((k, ps, _)) = best.as_ref() {
        let (mut plan, mut bl) = (&ps[0], f64::INFINITY);
        for p in ps.iter() {
            let l = crate::places::on_foot(g, from, p.stack[0]);
            if l < bl { bl = l; plan = p; }
        }
        Target { kind: TargetKind::Room, plan: Some(plan.clone()), room: k.clone(), pos: v3::copy(plan.stack[0]), word: None }
    } else {
        point(g)
    };
    // a floor with no room plans: the word's place first, then the level's own named places
    let mut sweep = false;
    if plans.is_empty() {
        if let Some(t) = prev.as_ref() {
            if t.kind == TargetKind::Point && prev_phase == Phase::Hold {
                let now = g.time;
                g.qrf.cleared.set(t.room.clone(), now);
                if let Some(w) = t.word { g.qrf.swept_t = m::max(g.qrf.swept_t, w); }
            }
        }
        if next.kind == TargetKind::Point && dist_xz(from, next.pos) < 3.0 { g.qrf.swept_t = m::max(g.qrf.swept_t, g.qrf.fix_t); }
        if next.kind == TargetKind::Point { next.word = Some(g.qrf.fix_t); }
        if g.qrf.fix.is_none() || g.qrf.swept_t >= g.qrf.fix_t {
            if let Some(s) = sweep_next(g, from, fix) { next = s; sweep = true; }
        }
    }
    g.qrf.target_at = g.time;
    let same = prev.as_ref().map_or(false, |t| t.kind == next.kind && t.room == next.room && dist_xz(t.pos, next.pos) < 2.0);
    if same && prev_phase == Phase::Move { return; }   // (the walk goes on as it is, and nothing is said twice)
    if same && next.kind == TargetKind::Point && dist_xz(from, next.pos) < 3.0 { g.qrf.phase = Phase::Hold; g.qrf.phase_t = 0.0; return; }
    g.qrf.target = Some(next.clone());
    start_move(g, el);
    let lead = el[0];
    let text = if next.kind == TargetKind::Room {
        let said = door_said(g, &next.plan.as_ref().unwrap().door);
        format!("element moving — {}, by the {}", next.room, said)
    } else if sweep {
        if bf.is_some() { format!("nothing here — element moving on, {}", next.room) } else { format!("nothing on the net — element sweeping, {}", next.room) }
    } else if bf.is_none() {
        let name = marks_of(g).map_or(String::from("the door"), |k| k.name);
        format!("nothing on the net — element clearing from {}", name)
    } else {
        format!("element moving — {}, on the last {}", next.room, "word")
    };
    radio(g, Some(lead), &text);
    let what = format!("target:{}:{}{}", next.kind.as_str(), next.room, if sweep { ":sweep" } else { "" });
    note(g, &what);
}

/// The next of the level's named places to walk to: the nearest by the walk from `fix` nobody of theirs has stood on lately.
fn sweep_next(g: &mut crate::game::Game, from: V3, fix: V3) -> Option<crate::qrf::Target> {
    let (mut best, mut bd) = (None, f64::INFINITY);
    for p in sweep_places(g) {
        let c = match nearest_free(g, p.pos) { Some(c) => c, None => continue };
        if dist_xz(from, c) < 3.0 { let t = g.time; g.qrf.cleared.set(p.name.clone(), t); }
        if g.time - g.qrf.cleared.get(p.name.clone()).copied().unwrap_or(-1e9) < RECLEAR_SECS { continue; }
        let start = if g.qrf.fix.is_some() { fix } else { from };
        let d = crate::places::on_foot(g, start, c);
        if d < bd {
            bd = d;
            best = Some(crate::qrf::Target { kind: crate::qrf::TargetKind::Point, plan: None, room: p.name, pos: c, word: None });
        }
    }
    best
}

/// The list: the block's own `sweep`, or the quiet block's spots, a raid's quiet rooms and scenarios.places' areas by their middles.
fn sweep_places(g: &crate::game::Game) -> Vec<crate::qrf::SweepPlacesItem> {
    use crate::qrf::SweepPlacesItem;
    fn mid(b: &crate::scene_scenarios::Rect) -> V3 { [(b.x0 + b.x1) / 2.0, 0.0, (b.z0 + b.z1) / 2.0] }
    let mut out: Vec<SweepPlacesItem> = Vec::new();
    if let Some(s) = g.level.scenarios.as_ref() {
        if let Some(sw) = s.response.as_ref().and_then(|r| r.sweep.as_ref()) {
            if !sw.is_empty() {
                return sw.iter().map(|p| SweepPlacesItem { name: p.name.clone(), pos: p.pos }).collect();
            }
        }
    }
    for r in g.level.spots() { out.push(SweepPlacesItem { name: r.place.clone(), pos: mid(&r.area) }); }
    if raid(g) {
        let rooms = g.staged.rooms.clone();
        for r in &rooms {
            let mut held = false;
            for &h in &r.defenders {
                if !crate::condition::down(g, g.gd[h].ch) && !crate::owner::has(g, h, crate::owner::by::RUNNER) { held = true; break; }
            }
            if !held { out.push(SweepPlacesItem { name: r.def.name.clone(), pos: mid(&r.def.bounds) }); }
        }
    }
    if let Some(a) = g.level.scenarios.as_ref().and_then(|s| s.places.as_ref()) {
        for r in &a.areas { out.push(SweepPlacesItem { name: r.name.clone(), pos: mid(&r.bounds) }); }
    }
    out
}
/// `p` on the floor, or the nearest free point to it.
pub(super) fn nearest_free(g: &crate::game::Game, p: V3) -> Option<V3> {
    if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, p[0], p[2]) } else { Some([p[0], 0.0, p[2]]) }
}

// ---------------------------------------------------------------- movement as one element
/// Where the walk to a room ends: a length past the stack marks along the stack line, away from the door.
fn approach_end(g: &crate::game::Game, p: &crate::scene_level::RoomClearPlan) -> V3 {
    let s = p.stack;
    let u = v3::normalize([s[1][0] - s[0][0], 0.0, s[1][2] - s[0][2]]);
    for k in [1.6, 1.1, 0.6, 0.0] {
        let q: V3 = [s[0][0] + u[0] * k, 0.0, s[0][2] + u[2] * k];
        if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && !crate::scene_collision::low_blocked(g, s[0], q) { return q; }
    }
    v3::copy(s[0])
}

/// The point man takes the leg to the target; each man behind walks to one station point of his own, moved under him.
fn start_move(g: &mut crate::game::Game, el: &[GuardId]) {
    use crate::character::{Stance, UpperMode};
    let lead = el[0];
    g.qrf.phase = crate::qrf::Phase::Move;
    g.qrf.phase_t = 0.0;
    g.qrf.trail = vec![v3::copy(g.pos_of(lead))];
    g.qrf.station_t = 0.0;
    g.qrf.point_id = g.char_id(lead);
    let t = g.qrf.target.clone().unwrap();
    let goal = if t.kind == crate::qrf::TargetKind::Room { approach_end(g, t.plan.as_ref().unwrap()) } else { t.pos };
    g.qrf.move_speed = if g.qrf.cue_rush { RUSH_SPEED } else { MOVE_SPEED };
    g.qrf.cue_rush = false;
    let speed = g.qrf.move_speed;
    route(g, lead, &[goal], Some(crate::squad::OrderPatch { speed: Some(Some(speed)), face: Some(None), strafe: Some(Some(false)), stance: Some(Some(Stance::None)), upper: Some(Some(UpperMode::Aim)), aim_at: Some(None), crouch: Some(Some(false)), ..Default::default() }));
    for (i, &x) in el.iter().enumerate() {
        if i == 0 { continue; }
        crate::mind::set_state(g, x, crate::game::GuardState::Search, "qrf");
        let p = g.pos_of(x);
        let (st, _) = write_station(g, x, p);
        g.qrf.wps.delete(x);
        let aim = *aim_of(g, x);
        crate::squad::order(g, x, crate::squad::OrderPatch { goal: Some(Some(st)), speed: Some(Some(speed)), face: Some(None), strafe: Some(Some(false)), stance: Some(Some(Stance::None)), upper: Some(Some(UpperMode::Aim)), crouch: Some(Some(false)), aim_at: Some(Some(aim)), held: true, ..Default::default() });
    }
}

/// His aim point, made on first asking: the row itself.
fn aim_of<'g>(g: &'g mut crate::game::Game, m: GuardId) -> &'g mut V3 {
    let p = g.pos_of(m);
    g.qrf.aim_pts.get_or_insert_with(m, || [p[0], 1.0, p[2] + 1.0])
}

/// His station, made on first asking: the row itself.
fn station_of<'g>(g: &'g mut crate::game::Game, m: GuardId) -> &'g mut V3 {
    let p = g.pos_of(m);
    g.qrf.stations.get_or_insert_with(m, || v3::copy(p))
}

/// The element walking: the trail, the stations down the file, the pace, a fresher fix, and arriving.
fn phase_move(g: &mut crate::game::Game, el: &mut Vec<GuardId>, dt: f64) {
    use crate::character::{Stance, UpperMode};
    use crate::qrf::TargetKind;
    let lead = el[0];
    let t = match g.qrf.target.clone() { Some(t) => t, None => { pick_target(g, el, true); return; } };
    if g.gd[lead].script.is_none() || g.char_id(lead) != g.qrf.point_id { start_move(g, el); return; }   // (the point man died: the new #1 takes the walk up)
    // the trail the file walks in: a crumb every 0.6 m
    let lp = g.pos_of(lead);
    let tail = g.qrf.trail[g.qrf.trail.len() - 1];
    if dist_xz(tail, lp) > 0.6 {
        g.qrf.trail.push(v3::copy(lp));
        if g.qrf.trail.len() > 80 { g.qrf.trail.remove(0); }
    }
    g.qrf.station_t -= dt;
    if g.qrf.station_t <= 0.0 {
        g.qrf.station_t = STATION_SECS;
        let lead_ch = g.gd[lead].ch;
        let fwd = crate::character::character_forward(g, lead_ch);
        for (k, &x) in el.iter().enumerate() {
            if k == 0 { continue; }
            let along = k as f64 * FILE_SPACING;
            let back = trail_point(&g.qrf.trail, lp, along);
            let dir = trail_dir(&g.qrf.trail, lp, along).unwrap_or(fwd);
            let side: f64 = if k == 1 { 1.0 } else if k == 2 { -1.0 } else { 0.0 };
            let mut want: V3 = [back[0] + dir[2] * FILE_OFFSET * side, 0.0, back[2] - dir[0] * FILE_OFFSET * side];
            if crate::scene_collision::nav_grid_is_blocked(g, want[0], want[2]) || (side != 0.0 && !crate::scene_collision::nav_grid_walkable(g, back, want)) {
                want = if crate::scene_collision::nav_grid_is_blocked(g, back[0], back[2]) { nearest_free(g, back).unwrap_or(back) } else { back };
            }
            let (st, tied) = write_station(g, x, want);
            if !tied {   // (a man fresh off another leg is put on his station)
                let aim = *aim_of(g, x);
                crate::squad::order(g, x, crate::squad::OrderPatch { goal: Some(Some(st)), face: Some(None), strafe: Some(Some(false)), stance: Some(Some(Stance::None)), upper: Some(Some(UpperMode::Aim)), crouch: Some(Some(false)), aim_at: Some(Some(aim)), held: true, ..Default::default() });
            }
            let p = g.pos_of(x);
            let gap = dist_xz(p, st);
            let pace = g.qrf.move_speed * (if gap > 3.5 { 1.4 } else if gap > 1.8 { 1.15 } else if gap < 0.5 { 0.75 } else { 1.0 });
            g.gd[x].script.as_mut().unwrap().speed = Some(pace);
            // sectors: two and three quarter out ahead, four looks back down the trail every few seconds
            let yaw_f = m::atan2(dir[0], dir[2]);
            let yaw = if k == 3 { if m::sin(g.time * 0.9) > 0.3 { yaw_f + m::PI } else { yaw_f } } else { yaw_f + (if k == 1 { -1.0 } else { 1.0 }) };
            let a = aim_of(g, x);
            a[0] = p[0] + m::sin(yaw) * 5.0;
            a[1] = 1.0;
            a[2] = p[2] + m::cos(yaw) * 5.0;
            let a = *a;
            g.gd[x].script.as_mut().unwrap().aim_at = Some(a);
        }
        // the point man eases off if the file has stretched
        let mut stretch: Vec<f64> = vec![0.0];
        for (i, &x) in el[1..].iter().enumerate() { stretch.push(dist_xz(g.pos_of(x), lp) - (i + 1) as f64 * FILE_SPACING); }
        let worst = m::max_n(&stretch);
        let pace = g.qrf.move_speed * (if worst > 5.0 { 0.35 } else if worst > 3.0 { 0.7 } else { 1.0 });
        g.gd[lead].script.as_mut().unwrap().speed = Some(pace);
    }
    // a fresher fix elsewhere: re-target (not more than every few seconds, and only for a real move of it)
    if let Some(b) = fix_of(g) {
        if b.t > g.qrf.fix_t + 0.5 && dist_xz(b.pos, g.qrf.fix.unwrap_or(b.pos)) > RETARGET_M && g.time - g.qrf.target_at > 3.0 {
            g.qrf.fix = Some(b.pos);
            g.qrf.fix_t = b.t;
            pick_target(g, el, false);
            return;
        }
    }
    if let Some(f) = g.qrf.fix {
        if g.qrf.fix_t > g.qrf.target_at && dist_xz(f, t.pos) > RETARGET_M && g.time - g.qrf.target_at > 3.0 && g.qrf.cue_rush { pick_target(g, el, false); return; }
    }
    if g.time - g.qrf.target_at > RETARGET_SECS && g.qrf.phase_t > RETARGET_SECS { pick_target(g, el, true); return; }
    // there: a room: into the stack; a point: hold all round, then choose again
    let there = arrived(g, lead) || (dist_xz(lp, g.gd[lead].script.and_then(|s| s.goal).unwrap_or(lp)) < 1.0 && g.gd[lead].speed < 0.2 && g.qrf.phase_t > 2.0);
    let eta = 60.0;
    if there || g.qrf.phase_t > eta {
        if t.kind == TargetKind::Room && dist_xz(lp, t.plan.as_ref().unwrap().stack[0]) < 4.0 {
            if !(g.tune.qrf_squad && start_drill(g, el)) { start_stack(g, el); }
        } else if t.kind == TargetKind::Room {
            let text = format!("can't get into the {} from here — next", t.room);
            radio(g, Some(lead), &text);
            let when = g.time - RECLEAR_SECS + 30.0;
            g.qrf.cleared.set(t.room.clone(), when);
            hold360(g, el);
        } else {
            hold360(g, el);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own bits: sectorPoint([6.7, 0, 11.2], 2.4), and with d = 3, y = 0.25
    #[test]
    fn sector_point_bits() {
        let a = sector_point([6.7, 0.0, 11.2], 2.4, None, None);
        assert_eq!(a.map(f64::to_bits), SECTOR_A);
        let b = sector_point([6.7, 0.0, 11.2], 2.4, Some(3.0), Some(0.25));
        assert_eq!(b.map(f64::to_bits), SECTOR_B);
    }
    const SECTOR_A: [u64; 3] = [0x40242795f33397e7, 0x3ff0000000000000, 0x401e0d581bf249d9];
    const SECTOR_B: [u64; 3] = [0x402173e9547b1db4, 0x3fd0000000000000, 0x4021f9c3648b3f1d];
}
