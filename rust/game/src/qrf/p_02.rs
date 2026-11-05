//! src/game/qrf.ts lines 426 to 877
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: each waits on the skeleton (notes.md, ## Skeleton)
/// qrf.ts:571 `g.audio.play(name, pos, volume, {rate})`: game_audio_like_play's opts is a map of Opaque, which cannot hold the rate
fn seam_audio_play_rate(g: &mut Game, name: &str, pos: V3, volume: f64, rate: f64) { todo!("needs: game_audio_like_play's opts able to hold {{rate}} (qrf.ts:571)") }
/// qrf.ts:525 `spawnUnit` throws where a man cannot be stood up, and the caller catches it; spawn_unit returns GuardId and cannot say so
fn seam_spawn_unit(g: &mut Game, o: &crate::guards::UnitOpts) -> Result<GuardId, String> { todo!("needs: a spawn_unit that can refuse (qrf.ts:525)") }

// ---------------------------------------------------------------- small private helpers
fn pos_of(g: &Game, x: GuardId) -> V3 { g.ch[g.gd[x].ch].pos }
fn player_pos(g: &Game) -> V3 { g.ch[g.pl[g.player].ch].pos }
fn char_id(g: &Game, x: GuardId) -> f64 { g.ch[g.gd[x].ch].id }
fn nav_blocked(g: &mut Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
fn arrived(g: &Game, x: GuardId) -> bool { g.gd[x].script.map_or(false, |s| s.arrived.unwrap_or(false)) }
/// a string the TypeScript tests for truth: null and '' are both false
fn word(s: &Option<String>) -> Option<&str> { s.as_deref().filter(|w| !w.is_empty()) }
/// `!y.door || g.doors.byName(y.door)`
fn way_open(g: &mut Game, y: &crate::scene_scenarios::ResponseWay) -> bool {
    match word(&y.door) { None => true, Some(d) => crate::doors::doors_by_name(g, d).is_some() }
}
/// `M.door ? g.doors.byName(M.door) : undefined`
fn marks_door(g: &mut Game, mk: &crate::qrf::Marks) -> Option<DoorId> {
    match word(&mk.door) { None => None, Some(d) => crate::doors::doors_by_name(g, d) }
}
/// a whole write of a man's fix (Guard.fix_stamp counts them)
fn set_last_known(g: &mut Game, x: GuardId, at: V3) {
    g.gd[x].last_known = Some(at);
    g.gd[x].fix_stamp = g.gd[x].fix_stamp.wrapping_add(1);
}
/// `face: null` in a RoutePatch: its absent field means "leave it", so the null is written after the call (order() only assigns them)
fn clear_face(g: &mut Game, x: GuardId) { if let Some(s) = g.gd[x].script.as_mut() { s.face = None; } }
/// `aimAt: null` in a RoutePatch, the same way
fn clear_aim(g: &mut Game, x: GuardId) { if let Some(s) = g.gd[x].script.as_mut() { s.aim_at = None; } }
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
    if tied { if let Some(s) = g.gd[x].script.as_mut() { s.goal = Some(st); } }
    (st, tied)
}

// ---------------------------------------------------------------- the calls
/// The radio's words for this level's team (null: it has none).   (qrf.ts responseWords)
pub fn response_words(g: &mut crate::game::Game) -> Option<crate::qrf::ResponseWords> {
    let r = response_of(g)?;
    Some(crate::qrf::ResponseWords { site: r.site, at: r.at, team: r.team })
}

/// What this call gets: the way in, how many, how long, off the game's own die.   (qrf.ts dealt)
pub fn dealt(g: &mut crate::game::Game, reason: crate::qrf::QrfReason) -> crate::qrf::Dealt {
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
    let a = crate::math_dice::sim_random(g);   // (three draws whatever the level allows)
    let b = crate::math_dice::sim_random(g);
    let c = crate::math_dice::sim_random(g);
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

/// The silent net: Sam's gunfire heard, then every man of the shift off the net: a radio check, then the team on the silence alone. Whether it asked now.   (qrf.ts silentNet)
pub fn silent_net(g: &mut crate::game::Game) -> bool {
    if g.qrf.silent_spent { return false; }
    let (mut roster, mut on) = (0, 0);
    for i in 0..g.guards.len() {
        let x = &g.gd[g.guards[i]];
        if x.unit.is_some() || x.team != crate::teams::Team::Guard { continue; }
        roster += 1;
        if x.state != crate::game::GuardState::Dead && x.held.is_none() { on += 1; }
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

/// Once a frame from game.ts, after the guards and the alarm.   (qrf.ts updateQrf)
pub fn update_qrf(g: &mut crate::game::Game, dt: f64) {
    use crate::director::EventKind;
    use crate::qrf::{QrfReason, QrfState};
    let rd = raid(g);
    if rd && raid_over(g) && (g.qrf.state == QrfState::Requested || g.qrf.state == QrfState::Inbound) {
        note(g, "called-off:the run is over");
        let log = core::mem::take(&mut g.qrf.log);
        g.qrf = crate::qrf::Qrf::blank();
        g.qrf.log = log;
        return;
    }
    // the trigger's memory: gunfire-level reports, each once
    let mut i = 0;
    while i < g.events.len() {
        let e = g.events[i];
        i += 1;
        if e.level.unwrap_or(0.0) < 1.0 || (e.kind != EventKind::Shot && e.kind != EventKind::GuardShot) { continue; }
        if g.qrf.heard_events.has(e.serial) { continue; }
        g.qrf.heard_events.add(e.serial);
        if e.kind == EventKind::Shot && !rd {
            for j in 0..g.guards.len() {
                let x = g.guards[j];
                if g.gd[x].unit.is_none() && g.gd[x].state != crate::game::GuardState::Dead && g.gd[x].held.is_none() && v3::dist(e.pos, pos_of(g, x)) <= 80.0 {
                    g.qrf.loud_t = g.time;
                    break;
                }
            }
        } else if rd {
            for x in callers(g) {
                if v3::dist(e.pos, pos_of(g, x)) <= 80.0 {
                    g.qrf.loud_t = g.time;
                    if !g.qrf.heard_at.has(x) { let t = g.time; g.qrf.heard_at.set(x, t); }
                    if g.qrf.raid_t < -1e8 {
                        g.qrf.raid_t = g.time;
                        let what = format!("raid-heard:{}", g.gd[x].callsign);
                        note(g, &what);
                    }
                }
            }
        }
        if g.qrf.state == QrfState::Deployed && g.time - g.qrf.deployed_at > SETTLE_IN_SECS { let t = g.time; g.qrf.shots_since.push(t); }
    }
    while !g.qrf.shots_since.is_empty() && g.time - g.qrf.shots_since[0] > GUNFIGHT_WINDOW { g.qrf.shots_since.remove(0); }
    if net_rule(g) && g.ai_enabled && has_response(g) { read_net(g); }
    if g.qrf.state == QrfState::Idle {
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
        return;
    }
    // the telegraph (runs on under anything; nothing new is SAID under its flag)
    let quiet = g.quiet_utility || g.touring;
    let mk = marks_of(g);
    let own = mk.is_some();
    if g.qrf.state == QrfState::Requested && g.time - g.qrf.requested_at >= ANSWER_SECS {
        g.qrf.state = QrfState::Inbound;
        g.qrf.answered = true;
        let n = m::min(4.0, if g.qrf.size > 0.0 { g.qrf.size } else { v3::clamp(m::round(g.tune.qrf_size), 2.0, 4.0) });
        let secs = m::round(g.qrf.arrive_at - g.time);
        let silent = g.qrf.reason == Some(QrfReason::Silent);
        if !quiet && !own {
            let text = if silent { format!("all stations — QRF is rolling to two-six: {} men, west stairwell, {} seconds", n, secs) }
                else { format!("copy two-six. QRF is rolling — {} men, west stairwell, {} seconds. lock down and hold what you've got", n, secs) };
            control(g, &text);
        } else if !quiet {
            let mk = mk.as_ref().unwrap();
            let said = if net_rule(g) { format!(" {}.", brief(g)) } else { String::new() };
            let text = if silent { format!("all stations — {} is rolling to {}: {} men, by {}, {} seconds.{}", mk.team, mk.site, n, mk.name, secs, said) }
                else { format!("copy {}. {} is rolling — {} men, by {}, {} seconds.{} lock down and hold what you've got", mk.site, mk.team, n, mk.name, secs, said) };
            control(g, &text);
        }
        note(g, "control");
    }
    if (g.qrf.state == QrfState::Inbound || g.qrf.state == QrfState::Requested) && !g.qrf.said30 && g.qrf.arrive_at - g.time <= CALL30 {
        g.qrf.said30 = true;
        if g.qrf.arrive_at - g.qrf.requested_at > CALL30 + 5.0 && !quiet {
            let silent = g.qrf.reason == Some(QrfReason::Silent);
            let text = match mk.as_ref() {
                None => if silent { String::from("all stations — QRF is thirty seconds out from two-six, coming up the west stairs") }
                    else { String::from(crate::words::wg(g, &"two-six, QRF is thirty seconds out — coming up the west stairs. check your fire on that side", &"two-six, QRF is thirty seconds out — coming up the west stairs. check your fire that end")) },
                Some(mk) => if silent { format!("all stations — {} is thirty seconds out from {}, {}", mk.team, mk.site, mk.coming) }
                    else { let side = crate::words::wg(g, &"on that side", &"that end"); format!("{}, {} is thirty seconds out — {}. check your fire {}", mk.site, mk.team, mk.coming, side) },
            };
            control(g, &text);
            note(g, "thirty");
        }
    }
    if (g.qrf.state == QrfState::Inbound || g.qrf.state == QrfState::Requested) && g.qrf.arrive_at - g.time <= CALL10 { arriving(g, quiet); }
    if g.qrf.state == QrfState::Arriving && g.time >= g.qrf.arrive_at { deploy(g, quiet); }
    if g.qrf.state == QrfState::Deployed && !g.qrf.everyone_said && g.time - g.qrf.deployed_at > 4.5 {
        g.qrf.everyone_said = true;
        if !quiet && !g.qrf.men.is_empty() {
            let text = match mk.as_ref() {
                None => String::from("all stations — that's the response team on your floor, and that's everyone we've got tonight. make it count"),
                Some(mk) => format!("all stations — that's {} with you now, and that's everyone we've got tonight. make it count", mk.team),
            };
            control(g, &text);
        }
    }
    if g.qrf.state == QrfState::Arriving || g.qrf.state == QrfState::Deployed { tick_men(g, dt); }
}

/// T−10: the men exist from here, stood at the muster, the lead's word on the net.   (qrf.ts arriving)
pub fn arriving(g: &mut crate::game::Game, quiet: bool) {
    use crate::qrf::{Phase, QrfState};
    g.qrf.state = QrfState::Arriving;
    g.qrf.said10 = true;
    re_deal(g, quiet);
    let mk = marks_of(g).unwrap();
    let want = if g.qrf.size > 0.0 { g.qrf.size } else { v3::clamp(m::round(g.tune.qrf_size), 2.0, 4.0) };
    let room = room_for(g);
    let n = m::min(want, room);
    let door = marks_door(g, &mk);
    let yaw = v3::yaw_to(mk.muster[0], mk.entry[0]);
    let mut i = 0usize;
    while (i as f64) < n {
        let at = match door {
            Some(d) if i == 0 => {
                let side = crate::doors::doors_side(g, d, mk.muster[0]);
                crate::handleside::tool_spot(g, d, side, mk.muster[0], None, None).map_or(mk.muster[0], |s| s.pos)
            }
            _ => match mk.muster.get(i) {
                Some(&p) => p,
                None => { let what = format!("spawn-failed:{}:no muster mark", i); note(g, &what); break; }   // (the TypeScript's spawnUnit throws on the missing mark: caught the same way)
            },
        };
        let team = match side_of_team(g) { crate::qrf::SideOfTeam::Guard => crate::teams::Team::Guard, crate::qrf::SideOfTeam::Hostile => crate::teams::Team::Hostile };
        let budget = match budget_of(g) { crate::qrf::BudgetOf::Arm0(b) => Some(b.budget), crate::qrf::BudgetOf::Arm1(_) => None };
        let opts = crate::guards::UnitOpts {
            team, pos: at, yaw: Some(yaw), weapon: Some(crate::weapons::WeaponId::Aks74u), name: Some(format!("{}-{}", mk.tag, i + 1)), light: Some(true),
            tint: Some(crate::consts::qrf_tint()), tint2: Some(crate::consts::qrf_tint2()), carbine_rig: None, pair: None, budget,
        };
        let gd = match seam_spawn_unit(g, &opts) {
            Ok(gd) => gd,
            Err(e) => { let short: String = e.chars().take(40).collect(); let what = format!("spawn-failed:{}:{}", i, short); note(g, &what); break; }
        };
        let seq = g.event_seq;
        let x = &mut g.gd[gd];
        x.trained = true;
        x.hold = false;
        x.state = crate::game::GuardState::Search;
        x.awareness = HUNT_AWARENESS;
        x.foe = Some(crate::game::Foe::Player);
        x.heard_up_to = seq;
        // at the high port at the muster, facing the door
        crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(None), face: Some(Some(yaw)), stance: Some(Some(crate::character::Stance::HighReady)), upper: Some(Some(crate::character::UpperMode::None)), aim_at: Some(None), crouch: Some(Some(false)), ..Default::default() });
        g.qrf.men.push(gd);
        i += 1;
    }
    g.qrf.bangs = g.qrf.men.len() as f64;
    let count = g.qrf.men.len();
    if n < want { let what = format!("capped:{}/{}", count, want); note(g, &what); }
    if count == 0 {
        g.qrf.state = QrfState::Deployed;
        g.qrf.deployed_at = g.time;
        if !quiet {
            let text = crate::words::wg(g, &format!("{} — {} is held up, nobody's coming. you're on your own", mk.site, mk.team), &format!("{} — {} is held up, nobody is coming. you are on your own", mk.site, mk.team));
            control(g, &text);
        }
        note(g, "nobody");
        return;
    }
    if mk.stairs { crate::audio_iface::game_audio_like_play(g, "stairBoots", Some([mk.muster[0][0], 0.3, mk.muster[0][2]]), Some(0.75), None); }
    if !quiet {
        let clear = match word(&mk.clear) { Some(c) => format!(" {},", c), None => String::new() };
        let lead = g.qrf.men[0];
        let text = format!("{} at {} — {} of us, ten seconds.{} check your fire", mk.tag, mk.name, count, clear);
        radio(g, Some(lead), &text);
    }
    let what = format!("ten:{}", count);
    note(g, &what);
    g.qrf.phase = Phase::Landing;
    g.qrf.phase_t = 0.0;
}

/// Ten seconds out: an enemy now stands at the dealt way's muster: another clear way is taken, and control says so.   (qrf.ts reDeal)
pub fn re_deal(g: &mut crate::game::Game, quiet: bool) {
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
    if !quiet {
        let text = format!("{} — change of plan: {} is {}", mk.site, mk.team, mk.coming);
        control(g, &text);
    }
    let what = format!("re-dealt:way {}", other);
    note(g, &what);
}

/// T: the bar goes down, the leaf is thrown open off its real hinge, and they file in.   (qrf.ts deploy)
pub fn deploy(g: &mut crate::game::Game, quiet: bool) {
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
    if !quiet {
        let text = format!("{} {} — {} in by {}, moving as one element", mk.tag, mk.at, alive(g).len(), mk.name);
        radio(g, lead, &text);
    }
    note(g, "door");
}

/// Throw a shut (or nearly shut) leaf open away from `by` at a kick's pace, breaking nothing.   (qrf.ts flingOpen)
pub fn fling_open(g: &mut crate::game::Game, d: DoorId, by: GuardId) {
    use crate::doors::{door_angle, door_pos, door_set_closing, door_set_creak_armed, door_set_last_who, door_set_latched, door_set_quiet_t, door_set_since_bash, door_set_vel, doors_side};
    if m::abs(door_angle(g, d)) > 1.0 { return; }
    let side = doors_side(g, d, pos_of(g, by));
    door_set_latched(g, d, false);
    door_set_closing(g, d, false);
    door_set_vel(g, d, -side * 7.5);
    door_set_since_bash(g, d, 0.0);
    door_set_quiet_t(g, d, -0.5);
    door_set_creak_armed(g, d, false);
    let who = char_id(g, by);
    door_set_last_who(g, d, who);
    let at = door_pos(g, d);
    seam_audio_play_rate(g, "doorBash", at, 0.9, 0.85);
}

// ---------------------------------------------------------------- the men, every frame
/// The element's frame: housekeeping, the door pulled to, the tier outside, the phase, then the scripted men's senses.   (qrf.ts tickMen)
pub fn tick_men(g: &mut crate::game::Game, dt: f64) {
    use crate::game::GuardState;
    use crate::qrf::Phase;
    g.qrf.phase_t += dt;
    // men taken out of the world from under us: forget them
    { let guards = &g.guards; g.qrf.men.retain(|x| guards.contains(x)); if let Some(p) = g.qrf.perim.as_mut() { p.men.retain(|r| guards.contains(&r.gd)); } }
    man_down(g);
    let mut el: Vec<GuardId> = alive(g);
    el.retain(|x| !g.qrf.perim.as_ref().map_or(false, |p| p.men.iter().any(|r| r.gd == *x)));
    // trained: short beats; nobody of theirs talks himself calm
    let mut all = alive(g);
    if let Some(p) = g.qrf.perim.as_ref() { for r in &p.men { if g.gd[r.gd].state != GuardState::Dead { all.push(r.gd); } } }
    for x in all {
        let cap = g.k.trained.react_cap;
        let t = g.time;
        let s = &mut g.gd[x];
        if t >= s.dazzled_until && t >= s.stagger_until && s.react_t > cap { s.react_t = cap; }
        if s.script.is_none() && (s.state == GuardState::Patrol || s.state == GuardState::Suspicious) {
            s.state = GuardState::Search;
            s.awareness = m::max(s.awareness, HUNT_AWARENESS);
            s.search_t = 0.0;
        }
        s.body_duty = None;
        s.task = None;
    }
    // DOORSTAY: the last man through pulls the stair door to, once they are all on the floor and clear of it
    if g.qrf.state == crate::qrf::QrfState::Deployed && !g.qrf.door_pulled && crate::doors::doors_stay(g) && g.time - g.qrf.deployed_at > 1.5 {
        let mk = marks_of(g);
        let d = match mk.as_ref() { Some(k) => marks_door(g, k), None => None };
        let up = alive(g);
        let shut = match d {
            None => true,
            Some(d) => crate::doors::door_off(g, d) || crate::doors::door_latched(g, d) || g.dr[d].def.closer == Some(false) || up.is_empty(),
        };
        if shut { g.qrf.door_pulled = true; }
        else {
            let d = d.unwrap();
            let land = crate::doors::doors_pull_face(g, d, mk.as_ref().unwrap().muster[0]);
            let (mut last, mut ld, mut through) = (None, f64::INFINITY, true);
            let frame = g.dr[d].frame_centre;
            for &x in &up {
                let p = pos_of(g, x);
                let dd = flat(p, frame);
                if crate::doors::doors_pull_face(g, d, p) == land || dd < 1.5 { through = false; }
                if dd < ld { ld = dd; last = Some(x); }
            }
            if let (true, Some(l)) = (through, last) {
                g.qrf.door_pulled = true;
                if ld < 6.0 {
                    let who = char_id(g, l);
                    crate::doors::doors_pull_to(g, d, who);
                    note(g, "door-to");
                }
            }
        }
    }
    if g.qrf.perim.is_some() { tick_perimeter(g, dt); } else if g.qrf.state == crate::qrf::QrfState::Deployed { maybe_perimeter(g); }
    if el.is_empty() {
        if g.qrf.phase != Phase::Fight { drop_drill(g); g.qrf.phase = Phase::Fight; note(g, "wiped"); }
        return;
    }
    if g.qrf.state == crate::qrf::QrfState::Arriving {   // at the muster: ears open, nothing else
        let mut seer = None;
        for &x in &el { if perceive_scripted(g, x, dt) && seer.is_none() { seer = Some(x); } }
        if let Some(seer) = seer {
            let no_door = marks_of(g).map_or(true, |k| word(&k.door).is_none());
            if no_door {
                note(g, "seen-at-the-muster");
                let quiet = g.quiet_utility || g.touring;
                deploy(g, quiet);
                to_fight(g, &el, seer);
            }
        }
        return;
    }
    match g.qrf.phase {
        Phase::File => phase_file(g, &mut el),
        Phase::Fan => if g.qrf.phase_t > ENTRY_HOLD { pick_target(g, &mut el, true); },
        Phase::Move => phase_move(g, &mut el, dt),
        Phase::Stack => phase_stack(g, &mut el),
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
    // the scripted men's senses: eyes on him and all go to the fight, unless still filing with him not close
    let mut contact = None;
    for &x in &el {
        if g.gd[x].script.is_none() { continue; }
        if perceive_scripted(g, x, dt) && contact.is_none() { contact = Some(x); }
    }
    if let Some(c) = contact {
        let near = g.qrf.phase != Phase::File || {
            let foe = match crate::teams::foe_char(g, c) { Some(f) => g.ch[f].pos, None => player_pos(g) };
            flat(pos_of(g, c), foe) < 8.0
        };
        if near { to_fight(g, &el, c); }
        else {
            g.qrf.fix = Some(v3::copy(g.gd[c].last_known.unwrap_or(player_pos(g))));
            g.qrf.fix_t = g.time;
            g.qrf.cue_rush = true;
            g.gd[c].awareness = m::min(g.gd[c].awareness, 0.84);
        }
    }
}

/// The senses updateGuard would have run for a man the runner drives. A noise is a cue; eyes on him is contact (true).   (qrf.ts perceiveScripted)
pub fn perceive_scripted(g: &mut crate::game::Game, m: GuardId, dt: f64) -> bool {
    use crate::game::{Foe, GuardState};
    if !g.ai_enabled { return false; }
    let mut ids: Vec<f64> = Vec::new();
    for i in 0..g.qrf.men.len() { ids.push(char_id(g, g.qrf.men[i])); }
    if let Some(p) = g.qrf.perim.as_ref() { for r in &p.men { ids.push(char_id(g, r.gd)); } }
    for i in 0..g.events.len() {
        if g.events[i].id.is_none() { g.event_seq += 1.0; g.events[i].id = Some(g.event_seq); }
    }
    let can_at = match g.qrf.can { Some(c) if !g.qrf.can_land_t.is_nan() => Some(crate::throwables::item_pos(g, c)), _ => None };
    let fix_before = g.gd[m].last_known;
    // his hearing runs on the events LESS the element's own; what it writes on an event (`announced`) is on the same object in the TypeScript: written back
    let all = core::mem::take(&mut g.events);
    let mut kept: Vec<usize> = Vec::new();
    let mut lent = Vec::new();
    for (i, e) in all.iter().enumerate() {
        let own = e.who.map_or(false, |w| ids.contains(&w)) || (e.kind == crate::director::EventKind::Bang && can_at.map_or(false, |c| flat(e.pos, c) < 1.5));
        if !own { kept.push(i); lent.push(*e); }
    }
    g.events = lent;
    crate::guards::hearing_check(g, m, None, None, None);
    let lent = core::mem::replace(&mut g.events, all);
    for (j, &i) in kept.iter().enumerate() { g.events[i] = lent[j]; }
    g.gd[m].heard_up_to = g.event_seq;
    let (mut seen, mut glimpse) = (false, false);
    let vis = crate::guards::can_see(g, m);
    if vis.visible {
        let rate = crate::guards::sight_gain(g, m, vis.dist);
        if rate > 0.05 {
            glimpse = true;
            let t = g.time;
            let pp = player_pos(g);
            g.gd[m].awareness = v3::clamp(g.gd[m].awareness + rate * dt, 0.0, 1.0);
            set_last_known(g, m, v3::copy(pp));
            g.gd[m].sight_pos = v3::copy(pp);
            g.gd[m].sight_t = t;
            if rate > 0.25 { g.gd[m].last_seen_t = t; }
            seen = g.gd[m].awareness >= 0.85;
        }
    }
    let pp = player_pos(g);
    let gp = pos_of(g, m);
    if !g.pl[g.player].down && !g.player_invisible && crate::m::hyp2(pp[0] - gp[0], pp[2] - gp[2]) < 0.62 {
        g.gd[m].awareness = 1.0;
        set_last_known(g, m, v3::copy(pp));
        g.gd[m].last_seen_t = g.time;
        seen = true;
    }
    if crate::guards::standoff_perceive(g, m, None) { seen = true; }
    g.gd[m].saw_player_this_frame = seen;
    g.gd[m].saw_foe_this_frame = seen;
    if seen { g.gd[m].foe = Some(Foe::Player); }
    // the enemy is whoever is against them on this level, seen by the same eye at the same rate
    if !seen {
        let mut i = 0;
        while i < g.guards.len() {
            let o = g.guards[i];
            i += 1;
            if g.gd[o].state == GuardState::Dead || g.gd[o].civ.is_some() || !crate::teams::hostile_to(g, m, crate::teams::HostileToO::Guard(o)) || crate::rails::rails_down(g, o) { continue; }
            let oc = g.gd[o].ch;
            let vo = crate::guards::can_see_char(g, m, oc, false);
            if !vo.visible { continue; }
            let rate = crate::guards::sight_gain_on(g, m, o, vo.dist);
            if rate <= 0.05 { continue; }
            glimpse = true;
            let t = g.time;
            let op = g.ch[oc].pos;
            g.gd[m].awareness = v3::clamp(g.gd[m].awareness + rate * dt, 0.0, 1.0);
            set_last_known(g, m, v3::copy(op));
            g.gd[m].sight_pos = v3::copy(op);
            g.gd[m].sight_t = t;
            if rate > 0.25 { g.gd[m].last_seen_t = t; }
            if g.gd[m].awareness >= 0.85 {
                seen = true;
                g.gd[m].foe = Some(Foe::Guard(o));
                g.gd[m].saw_foe_this_frame = true;
                break;
            }
        }
    }
    if g.gd[m].awareness > g.gd[m].witness.peak_awareness { g.gd[m].witness.peak_awareness = g.gd[m].awareness; }
    if seen { return true; }
    if glimpse { return false; }   // (the meter is filling on him: no cue, no clamp)
    // a cue: a noise reached his ears THIS frame and put his fix somewhere new: the element's fix follows it
    let (h_last, h_shot, h_kick) = { let h = &g.gd[m].witness.heard; (h.last_t, h.last_shot_t, h.kick) };
    if let Some(lk) = g.gd[m].last_known {
        if h_last >= g.time - 1e-6 && fix_before.map_or(true, |f| flat(f, lk) > 1.0) {
            g.qrf.fix = Some(v3::copy(lk));
            g.qrf.fix_t = g.time;
            g.qrf.cue_rush = h_shot >= g.time - 1e-6 || (h_kick > 0.0 && h_last >= g.time - 1e-6);
        }
    }
    g.gd[m].awareness = v3::clamp(g.gd[m].awareness, HUNT_AWARENESS * 0.85, 0.8);
    // a colleague down in his view: found, said once, and they push on
    if g.time - g.gd[m].saw_body_t > 6.0 {
        if let Some(o) = crate::guards::body_in_view(g, m) {
            let t = g.time;
            g.gd[m].saw_body_t = t;
            let victim = g.gd[o].callsign.clone();
            let breathing = crate::game::is_breathing(g, o);
            g.gd[m].witness.saw_body = Some(crate::game::WitnessSawBody { victim: victim.clone(), t, breathing });
            let mc = g.gd[m].ch;
            let eye = match g.ch[mc].bones.get(crate::character::Bone::Head) { Some(&b) => b, None => v3::add(g.ch[mc].pos, [0.0, 1.65, 0.0]) };
            let bp = crate::guards::body_pos(g, o);
            let team = g.gd[m].team;
            let mut n = 0;
            let mut i = 0;
            while i < g.guards.len() {
                let x = g.guards[i];
                i += 1;
                if g.gd[x].state != GuardState::Dead || g.gd[x].found || g.gd[x].team != team { continue; }
                let take = x == o || {
                    let xp = crate::guards::body_pos(g, x);
                    flat(xp, bp) < 3.0 && !crate::scene_collision::segment_blocked(g, eye, v3::add(xp, [0.0, 0.3, 0.0]), None, None, None)
                };
                if take {
                    g.gd[x].found = true;
                    crate::mission::tally(g, crate::mission::TallyK::Bodies);
                    n += 1;
                }
            }
            if n != 0 {
                let the = crate::words::w(g, &"", &"the ");
                let room = crate::dialogue::room_name(g, bp);
                let many = if n > 1 { format!(" — {} of them", n) } else { String::new() };
                let how = if g.gd[o].wound.is_some() { "hit, breathing" } else if crate::game::is_breathing(g, o) { "out cold, breathing" } else { "dead" };
                let text = format!("{} down in {}{}{}, {}. pushing on", victim, the, room, many, how);
                radio(g, Some(m), &text);
            }
            let what = format!("found:{}", victim);
            note(g, &what);
        }
    }
    false
}

/// A response man went down since last frame: the nearest of theirs with him in view says so on the net.   (qrf.ts manDown)
pub fn man_down(g: &mut crate::game::Game) {
    use crate::game::GuardState;
    let mut theirs = g.qrf.men.clone();
    if let Some(p) = g.qrf.perim.as_ref() { for r in &p.men { theirs.push(r.gd); } }
    for &d in &theirs {
        let key = crate::Num(char_id(g, d));
        if g.gd[d].state != GuardState::Dead || g.qrf.seen_ids.has(key) { continue; }
        g.qrf.seen_ids.add(key);
        g.qrf.lost += 1.0;
        let bp = crate::guards::body_pos(g, d);
        let (mut sp, mut bd) = (None, f64::INFINITY);
        for &o in &theirs {
            if o == d || g.gd[o].state == GuardState::Dead || !g.guards.contains(&o) { continue; }
            let dd = flat(pos_of(g, o), bp);
            if dd < bd && (dd < 6.0 || crate::guards::point_in_view(g, o, v3::add(bp, [0.0, 0.4, 0.0]), Some(14.0), Some(100.0))) { bd = dd; sp = Some(o); }
        }
        let callsign = g.gd[d].callsign.clone();
        if let Some(s) = sp {
            if !g.gd[d].found { g.gd[d].found = true; crate::mission::tally(g, crate::mission::TallyK::Bodies); }
            let t = g.time;
            let breathing = crate::game::is_breathing(g, d);
            g.gd[s].witness.saw_body = Some(crate::game::WitnessSawBody { victim: callsign.clone(), t, breathing });
            let room = crate::dialogue::room_name(g, bp);
            let him = if g.gd[d].down_by == Some(crate::game::DownBy::Player) { " it's him — on me!" } else { "" };
            let text = format!("man down — {} is hit, {}!{}", callsign, room, him);
            radio(g, Some(s), &text);
        }
        let what = format!("man-down:{}{}", callsign, if sp.is_some() { "" } else { ":unseen" });
        note(g, &what);
    }
    // the run ending on one of THEIR rounds
    if g.pl[g.player].down && !g.qrf.sam_was_down && !g.qrf.got_sam {
        for &o in &theirs {
            if let Some(ls) = g.gd[o].aim.last_shot.as_ref() {
                if m::abs(ls.t - g.time) < 1e-6 {
                    g.qrf.got_sam = true;
                    let what = format!("got-sam:{}", g.gd[o].callsign);
                    note(g, &what);
                    break;
                }
            }
        }
    }
    g.qrf.sam_was_down = g.pl[g.player].down;
}

// ---------------------------------------------------------------- entry
/// One man through the doorway every FILE_GAP, each to his fan mark facing his sector.   (qrf.ts phaseFile)
pub fn phase_file(g: &mut crate::game::Game, el: &mut Vec<GuardId>) {
    let due = m::min(el.len() as f64, m::floor(g.qrf.phase_t / FILE_GAP) + 1.0);
    let mk = marks_of(g).unwrap();
    while g.qrf.filed < due {
        let i = g.qrf.filed as usize;
        let x = match el.get(i) { Some(&x) => x, None => break };
        g.gd[x].state = crate::game::GuardState::Search;
        g.gd[x].awareness = m::max(g.gd[x].awareness, HUNT_AWARENESS);
        let e = mk.entry.get(i).copied().unwrap_or(mk.entry[0]);
        let ef = mk.entry_face.get(i).copied().unwrap_or(m::PI / 2.0);
        route(g, x, &[e], Some(crate::qrf::RoutePatch { speed: Some(2.4), face: Some(ef), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), aim_at: Some(sector_point(e, ef, None, None)), crouch: Some(i == 3), strafe: Some(false), ..Default::default() }));
        g.qrf.filed += 1.0;
        let what = format!("in:{}", i + 1);
        note(g, &what);
    }
    for &x in el.iter() { advance(g, x); }
    let all_in = g.qrf.filed >= el.len() as f64 && el.iter().all(|&x| {
        let p = pos_of(g, x);
        arrived(g, x) || flat(p, g.gd[x].script.and_then(|s| s.goal).unwrap_or(p)) < 0.9
    });
    if all_in || g.qrf.phase_t > FILE_GAP * el.len() as f64 + m::max(6.0, flat(mk.muster[0], mk.entry[0]) / 1.6) {
        g.qrf.phase = crate::qrf::Phase::Fan;
        g.qrf.phase_t = 0.0;
        note(g, "fan");
    }
}

/// A point `d` along the yaw from `p` at height `y`: where a man's sector looks.   (qrf.ts sectorPoint)
pub fn sector_point(p: V3, yaw: f64, d: Option<f64>, y: Option<f64>) -> V3 {
    let d = d.unwrap_or(5.0);
    let y = y.unwrap_or(1.0);
    [p[0] + m::sin(yaw) * d, y, p[2] + m::cos(yaw) * d]
}

// ---------------------------------------------------------------- targeting
/// Choose where the element goes next and set it moving. `fresh`: re-read the floor's fix first.   (qrf.ts pickTarget)
pub fn pick_target(g: &mut crate::game::Game, el: &mut Vec<GuardId>, fresh: bool) {
    use crate::qrf::{Phase, Target, TargetKind};
    let bf = fix_of(g);
    if let Some(b) = bf {
        if (fresh && !(net_rule(g) && g.qrf.fix.is_some() && b.t < g.qrf.fix_t - 1e-6)) || g.qrf.fix.is_none() || b.t > g.qrf.fix_t {
            g.qrf.fix = Some(b.pos);
            g.qrf.fix_t = m::max(g.qrf.fix_t, b.t);
        }
    }
    let fix: V3 = match g.qrf.fix {
        Some(f) => f,
        None => v3::copy(marks_of(g).and_then(|k| k.entry.first().copied()).unwrap_or(pos_of(g, el[0]))),
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
                d = m::min(d, path_len(g, fix, [dc[0], 0.0, dc[2]]));
            }
        }
        if is_in { d = -1.0; }
        if best.as_ref().map_or(true, |b| d < b.2) { best = Some((k, ps, d)); }
    }
    let from = pos_of(g, el[0]);
    let prev = g.qrf.target.clone();
    let prev_phase = g.qrf.phase;
    // the fix out on the open floor and not where they already stand: close on IT first
    let fix_open = !plans.iter().any(|p| holds(p));
    let point = |g: &mut Game| {
        let room = crate::dialogue::room_name(g, fix);
        let pos = nearest_free(g, fix).unwrap_or(v3::copy(fix));
        Target { kind: TargetKind::Point, plan: None, room, pos, word: None }
    };
    let held_here = prev.as_ref().map_or(false, |t| t.kind == TargetKind::Point && flat(t.pos, fix) < RETARGET_M && prev_phase == Phase::Hold);
    let mut next: Target = if fix_open && flat(from, fix) > 5.0 && !held_here {
        point(g)
    } else if let Some((k, ps, _)) = best.as_ref() {
        let (mut plan, mut bl) = (&ps[0], f64::INFINITY);
        for p in ps.iter() {
            let l = path_len(g, from, p.stack[0]);
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
        if next.kind == TargetKind::Point && flat(from, next.pos) < 3.0 { g.qrf.swept_t = m::max(g.qrf.swept_t, g.qrf.fix_t); }
        if next.kind == TargetKind::Point { next.word = Some(g.qrf.fix_t); }
        if g.qrf.fix.is_none() || g.qrf.swept_t >= g.qrf.fix_t {
            if let Some(s) = sweep_next(g, from, fix) { next = s; sweep = true; }
        }
    }
    g.qrf.target_at = g.time;
    let same = prev.as_ref().map_or(false, |t| t.kind == next.kind && t.room == next.room && flat(t.pos, next.pos) < 2.0);
    if same && prev_phase == Phase::Move { return; }   // (the walk goes on as it is, and nothing is said twice)
    if same && next.kind == TargetKind::Point && flat(from, next.pos) < 3.0 { g.qrf.phase = Phase::Hold; g.qrf.phase_t = 0.0; return; }
    g.qrf.target = Some(next.clone());
    start_move(g, el);
    let lead = el[0];
    let text = if next.kind == TargetKind::Room {
        let said = door_said(g, &next.plan.as_ref().unwrap().door);
        format!("element moving — {}, by the {}", next.room, said)
    } else if sweep {
        if bf.is_some() { format!("nothing here — element moving on, {}", next.room) } else { format!("nothing on the net — element sweeping, {}", next.room) }
    } else if net_rule(g) && bf.is_none() {
        let name = marks_of(g).map_or(String::from("the door"), |k| k.name);
        format!("nothing on the net — element clearing from {}", name)
    } else {
        format!("element moving — {}, on the last {}", next.room, if net_rule(g) { "word" } else { "fix" })
    };
    radio(g, Some(lead), &text);
    let what = format!("target:{}:{}{}", next.kind.as_str(), next.room, if sweep { ":sweep" } else { "" });
    note(g, &what);
}

/// The next of the level's named places to walk to: the nearest by the walk from `fix` nobody of theirs has stood on lately.   (qrf.ts sweepNext)
pub fn sweep_next(g: &mut crate::game::Game, from: V3, fix: V3) -> Option<crate::qrf::Target> {
    let (mut best, mut bd) = (None, f64::INFINITY);
    for p in sweep_places(g) {
        let c = match nearest_free(g, p.pos) { Some(c) => c, None => continue };
        if flat(from, c) < 3.0 { let t = g.time; g.qrf.cleared.set(p.name.clone(), t); }
        if g.time - g.qrf.cleared.get(p.name.clone()).copied().unwrap_or(-1e9) < RECLEAR_SECS { continue; }
        let start = if g.qrf.fix.is_some() { fix } else { from };
        let d = path_len(g, start, c);
        if d < bd {
            bd = d;
            best = Some(crate::qrf::Target { kind: crate::qrf::TargetKind::Point, plan: None, room: p.name, pos: c, word: None });
        }
    }
    best
}

/// The list: the block's own `sweep`, or the quiet block's spots, a raid's quiet rooms and scenarios.places' areas by their middles.   (qrf.ts sweepPlaces)
pub fn sweep_places(g: &mut crate::game::Game) -> Vec<crate::qrf::SweepPlacesItem> {
    use crate::qrf::SweepPlacesItem;
    fn mid(b: &crate::scene_scenarios::Rect) -> V3 { [(b.x0 + b.x1) / 2.0, 0.0, (b.z0 + b.z1) / 2.0] }
    let mut out: Vec<SweepPlacesItem> = Vec::new();
    if let Some(s) = g.level.scenarios.as_ref() {
        if let Some(sw) = s.response.as_ref().and_then(|r| r.sweep.as_ref()) {
            if !sw.is_empty() {
                return sw.iter().map(|p| SweepPlacesItem { name: p.name.clone(), pos: p.pos }).collect();
            }
        }
        if let Some(q) = s.quiet.as_ref() {
            for r in &q.spots { out.push(SweepPlacesItem { name: r.place.clone(), pos: mid(&r.area) }); }
        }
    }
    if raid(g) {
        let rooms = g.room_clear.rooms.clone();
        for r in &rooms {
            let mut held = false;
            for &h in &r.defenders {
                if g.gd[h].state != crate::game::GuardState::Dead && !crate::runner::runner_owns(g, h) { held = true; break; }
            }
            if !held { out.push(SweepPlacesItem { name: r.def.name.clone(), pos: mid(&r.def.bounds) }); }
        }
    }
    if let Some(a) = g.level.scenarios.as_ref().and_then(|s| s.places.as_ref()) {
        for r in &a.areas { out.push(SweepPlacesItem { name: r.name.clone(), pos: mid(&r.bounds) }); }
    }
    out
}

fn cr_at(p: V3) -> String { format!("({}, {})", p[0], p[2]) }
fn cr_free(g: &mut Game, p: V3) -> bool { !nav_blocked(g, p[0], p[2]) }
fn cr_walk(g: &mut Game, a: V3, b: V3) -> f64 { path_len(g, [a[0], 0.0, a[2]], [b[0], 0.0, b[2]]) }
fn cr_apart(out: &mut Vec<String>, ps: &[V3], what: &str) {
    for i in 0..ps.len() {
        for j in i + 1..ps.len() {
            if flat(ps[i], ps[j]) < 0.6 { out.push(format!("{}: marks {} and {} are {:.2} m apart", what, i + 1, j + 1, flat(ps[i], ps[j]))); }
        }
    }
}

/// The level's block against its built floor: what is wrong, in words (nothing: the block stands).   (qrf.ts checkResponse)
pub fn check_response(g: &mut crate::game::Game) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let r = match response_of(g) { Some(r) => r, None => return out };
    let floor = match g.level.routes.first() {
        Some(&id) => g.tab.route[id].points.first().copied().unwrap_or(g.level.player_spawn),
        None => g.level.player_spawn,
    };
    for (k, y) in r.ways.iter().enumerate() {
        let way = format!("way {} '{}'", k + 1, y.name);
        if let Some(name) = word(&y.door) {
            if crate::doors::doors_by_name(g, name).is_none() { out.push(format!("{}: no door '{}' on this level", way, name)); }
        }
        for (i, &p) in y.muster.iter().enumerate() {
            if !cr_free(g, p) { out.push(format!("{}: muster mark {} {} is not free floor", way, i + 1, cr_at(p))); }
        }
        for (i, &p) in y.entry.iter().enumerate() {
            if !cr_free(g, p) { out.push(format!("{}: fan mark {} {} is not free floor", way, i + 1, cr_at(p))); }
        }
        cr_apart(&mut out, &y.muster, &format!("{} muster", way));
        cr_apart(&mut out, &y.entry, &format!("{} fan", way));
        for (i, &p) in y.muster.iter().enumerate() {
            let e = match y.entry.get(i) { Some(&e) => e, None => continue };
            if !cr_free(g, p) || !cr_free(g, e) { continue; }
            let l = cr_walk(g, p, e);
            if !(l <= 30.0) {
                let how = if l == f64::INFINITY { String::from("no walk at all") } else { format!("{:.1} m", l) };
                out.push(format!("{}: man {}'s walk from muster to his fan mark is {} (30 m at most)", way, i + 1, how));
            }
        }
        if let Some(&e0) = y.entry.first() {
            if cr_free(g, e0) && cr_walk(g, e0, floor) == f64::INFINITY { out.push(format!("{}: no walk from the fan {} to the level's floor {}", way, cr_at(e0), cr_at(floor))); }
        }
        if let Some(name) = word(&y.door) {
            if let (Some(d), Some(&m0), Some(&e0)) = (crate::doors::doors_by_name(g, name), y.muster.first(), y.entry.first()) {
                if crate::doors::doors_pull_face(g, d, m0) == crate::doors::doors_pull_face(g, d, e0) { out.push(format!("{}: the muster and the fan are on the same side of '{}'", way, name)); }
            }
        }
    }
    let clear_m = g.k.response.clear_m;
    let starts = g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()).map_or(Vec::new(), |q| q.starts.clone());
    for s in &starts {
        if !r.ways.iter().any(|y| flat(s.sam, y.muster[0]) >= clear_m) {
            let name = s.name.clone().unwrap_or_else(|| cr_at(s.sam));
            out.push(format!("start '{}': every way's muster is within {} m of it", name, clear_m));
        }
    }
    let outside = r.outside.clone().unwrap_or_default();
    for (i, p) in r.posts.iter().flatten().enumerate() {
        if !cr_free(g, p.pos) { out.push(format!("post {} '{}' {} is not free floor", i + 1, p.name, cr_at(p.pos))); continue; }
        let mut reached = false;
        for &o in &outside {
            if cr_free(g, o) && cr_walk(g, o, p.pos) < f64::INFINITY { reached = true; break; }
        }
        if !reached { out.push(format!("post {} '{}': no walk to it from any outside mark", i + 1, p.name)); }
    }
    for (i, &p) in outside.iter().enumerate() {
        if !cr_free(g, p) { out.push(format!("outside mark {} {} is not free floor", i + 1, cr_at(p))); }
    }
    let first = r.ways.first().and_then(|y| y.entry.first().copied());
    for (i, p) in r.sweep.iter().flatten().enumerate() {
        if !cr_free(g, p.pos) { out.push(format!("sweep place {} '{}' {} is not free floor", i + 1, p.name, cr_at(p.pos))); }
        else if let Some(e0) = first {
            if cr_walk(g, e0, p.pos) == f64::INFINITY { out.push(format!("sweep place {} '{}': no walk to it from the first way in", i + 1, p.name)); }
        }
    }
    out
}

/// The walk's length from `a` to `b` on the nav grid (Infinity: none).   (qrf.ts pathLen)
pub fn path_len(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let p = match crate::scene_collision::nav_grid_find_path(g, a, b, None, None) { Some(p) => p, None => return f64::INFINITY };
    let (mut l, mut q) = (0.0, a);
    for r in p { l += flat(q, r); q = r; }
    l
}

/// `p` on the floor, or the nearest free point to it.   (qrf.ts nearestFree)
pub fn nearest_free(g: &mut crate::game::Game, p: V3) -> Option<V3> {
    if nav_blocked(g, p[0], p[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, p[0], p[2]) } else { Some([p[0], 0.0, p[2]]) }
}

// ---------------------------------------------------------------- movement as one element
/// Where the walk to a room ends: a length past the stack marks along the stack line, away from the door.   (qrf.ts approachEnd)
pub fn approach_end(g: &mut crate::game::Game, p: &crate::scene_level::RoomClearPlan) -> V3 {
    let s = p.stack;
    let u = v3::normalize([s[1][0] - s[0][0], 0.0, s[1][2] - s[0][2]]);
    for k in [1.6, 1.1, 0.6, 0.0] {
        let q: V3 = [s[0][0] + u[0] * k, 0.0, s[0][2] + u[2] * k];
        if !nav_blocked(g, q[0], q[2]) && !crate::scene_collision::solid_blocked(g, [s[0][0], 0.45, s[0][2]], [q[0], 0.45, q[2]], None) { return q; }
    }
    v3::copy(s[0])
}

/// The point man takes the leg to the target; each man behind walks to one station point of his own, moved under him.   (qrf.ts startMove)
pub fn start_move(g: &mut crate::game::Game, el: &[GuardId]) {
    use crate::character::{Stance, UpperMode};
    let lead = el[0];
    g.qrf.phase = crate::qrf::Phase::Move;
    g.qrf.phase_t = 0.0;
    g.qrf.trail = vec![v3::copy(pos_of(g, lead))];
    g.qrf.station_t = 0.0;
    g.qrf.point_id = char_id(g, lead);
    let t = g.qrf.target.clone().unwrap();
    let goal = if t.kind == crate::qrf::TargetKind::Room { approach_end(g, t.plan.as_ref().unwrap()) } else { t.pos };
    g.qrf.move_speed = if g.qrf.cue_rush { RUSH_SPEED } else { MOVE_SPEED };
    g.qrf.cue_rush = false;
    let speed = g.qrf.move_speed;
    route(g, lead, &[goal], Some(crate::qrf::RoutePatch { speed: Some(speed), strafe: Some(false), stance: Some(Stance::None), upper: Some(UpperMode::Aim), crouch: Some(false), ..Default::default() }));
    clear_face(g, lead);
    clear_aim(g, lead);
    for (i, &x) in el.iter().enumerate() {
        if i == 0 { continue; }
        g.gd[x].state = crate::game::GuardState::Search;
        let p = pos_of(g, x);
        let (st, _) = write_station(g, x, p);
        g.qrf.wps.delete(x);
        let aim = *aim_of(g, x);
        crate::squad::order(g, x, crate::squad::OrderPatch { goal: Some(Some(st)), speed: Some(Some(speed)), face: Some(None), strafe: Some(Some(false)), stance: Some(Some(Stance::None)), upper: Some(Some(UpperMode::Aim)), crouch: Some(Some(false)), aim_at: Some(Some(aim)), held: true, ..Default::default() });
    }
}

/// His aim point, made on first asking: the row itself.   (qrf.ts aimOf)
pub fn aim_of<'g>(g: &'g mut crate::game::Game, m: GuardId) -> &'g mut V3 {
    let p = pos_of(g, m);
    g.qrf.aim_pts.get_or_insert_with(m, || [p[0], 1.0, p[2] + 1.0])
}

/// His station, made on first asking: the row itself.   (qrf.ts stationOf)
pub fn station_of<'g>(g: &'g mut crate::game::Game, m: GuardId) -> &'g mut V3 {
    let p = pos_of(g, m);
    g.qrf.stations.get_or_insert_with(m, || v3::copy(p))
}

/// The element walking: the trail, the stations down the file, the pace, a fresher fix, and arriving.   (qrf.ts phaseMove)
pub fn phase_move(g: &mut crate::game::Game, el: &mut Vec<GuardId>, dt: f64) {
    use crate::character::{Stance, UpperMode};
    use crate::qrf::TargetKind;
    let lead = el[0];
    let t = match g.qrf.target.clone() { Some(t) => t, None => { pick_target(g, el, true); return; } };
    if g.gd[lead].script.is_none() || char_id(g, lead) != g.qrf.point_id { start_move(g, el); return; }   // (the point man died: the new #1 takes the walk up)
    // the trail the file walks in: a crumb every 0.6 m
    let lp = pos_of(g, lead);
    let tail = g.qrf.trail[g.qrf.trail.len() - 1];
    if flat(tail, lp) > 0.6 {
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
            if nav_blocked(g, want[0], want[2]) || (side != 0.0 && !crate::scene_collision::nav_grid_walkable(g, back, want)) {
                want = if nav_blocked(g, back[0], back[2]) { nearest_free(g, back).unwrap_or(back) } else { back };
            }
            let (st, tied) = write_station(g, x, want);
            if !tied {   // (a man fresh off another leg is put on his station)
                let aim = *aim_of(g, x);
                crate::squad::order(g, x, crate::squad::OrderPatch { goal: Some(Some(st)), face: Some(None), strafe: Some(Some(false)), stance: Some(Some(Stance::None)), upper: Some(Some(UpperMode::Aim)), crouch: Some(Some(false)), aim_at: Some(Some(aim)), held: true, ..Default::default() });
            }
            let p = pos_of(g, x);
            let gap = flat(p, st);
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
        for (i, &x) in el[1..].iter().enumerate() { stretch.push(flat(pos_of(g, x), lp) - (i + 1) as f64 * FILE_SPACING); }
        let worst = m::max_n(&stretch);
        let pace = g.qrf.move_speed * (if worst > 5.0 { 0.35 } else if worst > 3.0 { 0.7 } else { 1.0 });
        g.gd[lead].script.as_mut().unwrap().speed = Some(pace);
    }
    // a fresher fix elsewhere: re-target (not more than every few seconds, and only for a real move of it)
    if let Some(b) = fix_of(g) {
        if b.t > g.qrf.fix_t + 0.5 && flat(b.pos, g.qrf.fix.unwrap_or(b.pos)) > RETARGET_M && g.time - g.qrf.target_at > 3.0 {
            g.qrf.fix = Some(b.pos);
            g.qrf.fix_t = b.t;
            pick_target(g, el, false);
            return;
        }
    }
    if let Some(f) = g.qrf.fix {
        if g.qrf.fix_t > g.qrf.target_at && flat(f, t.pos) > RETARGET_M && g.time - g.qrf.target_at > 3.0 && g.qrf.cue_rush { pick_target(g, el, false); return; }
    }
    if g.time - g.qrf.target_at > RETARGET_SECS && g.qrf.phase_t > RETARGET_SECS { pick_target(g, el, true); return; }
    // there: a room: into the stack; a point: hold all round, then choose again
    let there = arrived(g, lead) || (flat(lp, g.gd[lead].script.and_then(|s| s.goal).unwrap_or(lp)) < 1.0 && g.gd[lead].speed < 0.2 && g.qrf.phase_t > 2.0);
    let eta = 60.0;
    if there || g.qrf.phase_t > eta {
        if t.kind == TargetKind::Room && flat(lp, t.plan.as_ref().unwrap().stack[0]) < 4.0 {
            if !(g.tune.qrf_squad && start_drill(g, el)) { start_stack(g, el); }
        } else if t.kind == TargetKind::Room {
            let verb = crate::words::wg(g, &"into", &"onto");
            let text = format!("can't get {} the {} from here — next", verb, t.room);
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

    /// the TypeScript's own bits (Math through src/math/exact.ts): sectorPoint([6.7, 0, 11.2], 2.4), and with d = 3, y = 0.25
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
