//! src/game/mission.ts lines 1 to 175
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Live: a run is on and it really is the player playing. (mission.ts missionLive)
pub fn mission_live(g: &crate::game::Game) -> bool {
    let s = g.mission.stage;
    !g.quiet_utility && g.puppet.is_none() && s != crate::game::MissionStage::Done && s != crate::game::MissionStage::Failed
}

/// One more of a counted thing on the run's card, if the run is live. (mission.ts tally)
pub fn tally(g: &mut crate::game::Game, k: crate::mission::TallyK) {
    if mission_live(g) {
        let s = &mut g.mission.stats;
        match k {
            crate::mission::TallyK::Alerts => s.alerts += 1.0,
            crate::mission::TallyK::Spotted => s.spotted += 1.0,
            crate::mission::TallyK::Bodies => s.bodies += 1.0,
            crate::mission::TallyK::Shots => s.shots += 1.0,
            crate::mission::TallyK::Kills => s.kills += 1.0,
            crate::mission::TallyK::Knockouts => s.knockouts += 1.0,
            crate::mission::TallyK::Keys => s.keys += 1.0,
        }
    }
}

/// Planar distance from a floor point to the mission rack's face. (mission.ts rackDist)
pub fn rack_dist(g: &mut crate::game::Game, p: V3) -> f64 {
    let front = g.level.mission.rack.front;
    let half_w = g.level.mission.rack.half_w;
    if crate::quiet::quiet_of(g) {
        let d = m::hyp2(p[0] - front[0], p[2] - front[2]);
        if d > 0.3 && crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [front[0] + (p[0] - front[0]) / d * 0.2, 1.0, front[2] + (p[2] - front[2]) / d * 0.2], None) {
            return m::INF;
        }
        return m::max(0.0, d - half_w);
    }
    rack_dist_plain(front, half_w, p)
}

/// rackDist's arm for a rack on the plain night: anywhere across its width counts. (mission.ts:50)
fn rack_dist_plain(front: V3, half_w: f64, p: V3) -> f64 {
    m::hyp2(m::max(0.0, m::abs(p[0] - front[0]) - half_w), p[2] - front[2])
}

/// Out through the mission's exit, or any exit of a quiet mission. (mission.ts outNow)
pub fn out_now(g: &crate::game::Game, p: V3) -> bool {
    if crate::quiet::quiet_of(g) {
        let exits = crate::quiet::quiet_mission_exits(g);
        return exits.iter().any(|x| crate::quiet::out_through(x, p));
    }
    out_plain(g.level.mission.exfil_x, g.level.mission.exfil_z, g.level.mission.exfil_out, p)
}

/// outNow's arm for the level's one exit: past its point going its way, or east past its x; inside its frame. (mission.ts:56-58)
fn out_plain(exfil_x: f64, exfil_z: f64, exfil_out: Option<[f64; 2]>, p: V3) -> bool {
    if let Some(o) = exfil_out {
        return (p[0] - exfil_x) * o[0] + (p[2] - exfil_z) * o[1] > 0.0 && m::abs((p[0] - exfil_x) * o[1] - (p[2] - exfil_z) * o[0]) < 1.8;
    }
    p[0] > exfil_x && m::abs(p[2] - exfil_z) < 1.8
}

/// The mission's stage machine: infiltrate, drive, exfil, and the run's counts. (mission.ts updateMission)
pub fn update_mission(g: &mut crate::game::Game, dt: f64, input: InputId) {
    use crate::game::{GuardState, InteractableKind, MissionStage, Verb};
    if (g.quiet_utility || g.puppet.is_some()) && !g.mission_in_tour {
        g.mission.drive_t = -1.0;
        return;
    }
    if g.mission.pulled {
        let mut i = 0;
        while i < g.rack_targets.len() {
            let t = g.rack_targets[i];
            if crate::sim_lightshome::fixture_on(g, t) {
                let on = crate::sim_lightshome::light_in().on;
                crate::sim_lightshome::light_home_in(g, on, Some(t), Some(0.0), None, None, None);
            }
            i += 1;
        }
    }
    if g.mission.sweep_at > 0.0 && g.time >= g.mission.sweep_at {
        g.mission.sweep_at = -1.0;
        send_rack_check(g);
    }
    if g.mission.stage == MissionStage::Failed && g.mission.down_card_at > 0.0 && g.time >= g.mission.down_card_at {
        g.mission.down_card_at = -1.0;
        g.mission.debrief = true;
    }
    if g.mission.stage == MissionStage::Done || g.mission.stage == MissionStage::Failed {
        return;
    }
    if crate::seats::all_down(g) {
        g.mission.stage = MissionStage::Failed;
        g.mission.drive_t = -1.0;
        g.mission.down_card_at = if g.mission_in_tour { -1.0 } else { g.time + DOWN_CARD_SECS };
        return;
    }
    g.mission.stats.elapsed += dt;
    if g.god_mode || g.player_invisible || !g.ai_enabled {
        g.mission.stats.sandbox = true;
    }
    if !g.mission_in_tour {
        crate::chronicle::tick_chronicle(g);
    }
    let any_alert = g.guards.iter().any(|&gd| g.gd[gd].state == GuardState::Alert);
    if any_alert && !g.mission.alarm_on {
        g.mission.stats.alerts += 1.0;
    }
    g.mission.alarm_on = any_alert;
    // fully detected: an alert guard with eyes on you, counted in episodes (a fresh one after 4 s unseen)
    let by = g.guards.iter().copied().find(|&gd| g.gd[gd].state == GuardState::Alert && g.gd[gd].saw_player_this_frame);
    if let Some(by) = by {
        if g.time - g.mission.spotted_t > 4.0 {
            g.mission.stats.spotted += 1.0;
            crate::chronicle::chron_spotted(g, Some(by));
        }
        g.mission.spotted_t = g.time;
    }
    if crate::roomclear::room_clear_scenario_active(g) || g.rescue.active {
        // the room clear or the hostage rescue has the floor: no stage of the stealth mission moves under it
    } else if g.mission.stage == MissionStage::Infiltrate {
        let p = player_pos(g, &crate::seats::here(g));
        let (x0, x1, z0, z1) = {
            let r = &g.level.mission.server_room;
            (r.x0, r.x1, r.z0, r.z1)
        };
        if p[0] > x0 && p[0] < x1 && p[2] > z0 && p[2] < z1 {
            if let Some(w) = crate::quiet::quiet_words(g) {
                crate::chronicle::chron_objective(g, &format!("reached {}", w.place), None);
                g.mission.stage = MissionStage::Drive;
                crate::game::game_msg(g, &format!("{} — {} is in {}", w.place, w.thing, w.holder));
            } else {
                crate::chronicle::chron_objective(g, "reached the server room", None);
                g.mission.stage = MissionStage::Drive;
                let n = js::num_str(g.level.mission.rack.index + 1.0);
                crate::game::game_msg(g, &format!("you've reached the server room. the drive is in rack {}, the marked one", n));
                crate::dialogue::handler_beat(g, crate::dialogue::HandlerBeat::Server, Some(1.2));
            }
        }
    } else if g.mission.stage == MissionStage::Drive {
        let rf = g.level.mission.rack.front;
        let p = player_pos(g, &crate::seats::here(g));
        let in_reach = rack_dist(g, p) < crate::consts::DRIVE_REACH;
        let pl = g.player;
        if g.mission.drive_t < 0.0 {
            let asked = match &g.act {
                Some(a) => a.kind == InteractableKind::Objective && a.verb == Some(Verb::Drive),
                None => false,
            };
            if asked {
                g.mission.drive_t = 0.0;
                crate::audio_iface::game_audio_like_play(g, "click", Some(rf), Some(0.8), None);
                crate::audio_iface::game_audio_like_play(g, "magOut", Some(rf), Some(0.4), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.55 }));
                let w = crate::quiet::quiet_words(g);
                let line = match &w {
                    Some(w) => format!("you're taking {}. stay on it", w.thing),
                    None => String::from("you're pulling the drive. stay on it"),
                };
                crate::game::game_msg(g, &line);
                if !g.mission.hands_on {
                    g.mission.hands_on = true;
                    let line = match &w {
                        Some(w) => format!("hands on {}", w.thing),
                        None => String::from("hands on the drive"),
                    };
                    crate::chronicle::chron_objective(g, &line, Some(rf));
                }
            }
        } else if !in_reach || g.pl[pl].dragging.is_some() || g.pl[pl].takedown.is_some() || g.pl[pl].holding.is_some() || g.pl[pl].sprinting {
            cancel_drive(g);
        } else {
            g.mission.drive_t += dt;
            if g.mission.drive_t >= crate::consts::DRIVE_SECS {
                drive_pulled(g);
            } else {
                crate::handwork::drive_hands(g, crate::consts::DRIVE_SECS, false);
            }
        }
    } else if g.mission.stage == MissionStage::Exfil {
        let p = player_pos(g, &crate::seats::here(g));
        if out_now(g, p) {
            let x = crate::quiet::exit_taken(g, p);
            let line = match &x {
                Some(x) => format!("out through {} — mission complete", x.name),
                None => String::from("out through the fire exit — mission complete"),
            };
            crate::chronicle::chron_objective(g, &line, None);
            g.mission.stage = MissionStage::Done;
            g.mission.debrief = true;
            let d = crate::chronicle::debrief_of(g, &crate::seats::here(g));
            let mut line = format!("exfiltrated — {}", crate::game::fmt_clock(g.mission.stats.elapsed));
            if !d.grade.is_empty() {
                line.push_str(" · ");
                line.push_str(&d.grade);
            }
            crate::game::game_msg(g, &line);
        }
    }
}

/// Hands off the drive before it is out: the pull starts over next time. (mission.ts cancelDrive)
pub fn cancel_drive(g: &mut crate::game::Game) {
    if g.mission.drive_t < 0.0 {
        return;
    }
    g.mission.drive_t = -1.0;
    let w = crate::quiet::quiet_words(g);
    let line = match &w {
        Some(w) => format!("let go of {}", w.thing),
        None => String::from("let go of the drive"),
    };
    crate::game::game_msg(g, &line);
}

/// The caddy comes free: the rack goes dark, the clunk is heard, and soon a man is sent to look. (mission.ts drivePulled)
pub fn drive_pulled(g: &mut crate::game::Game) {
    let rf = g.level.mission.rack.front;
    let n = js::num_str(g.level.mission.rack.index + 1.0);
    let w = crate::quiet::quiet_words(g);
    let line = match &w {
        Some(w) => format!("{} is out of {}", w.thing, w.holder),
        None => format!("the drive is out — rack {} goes dark", n),
    };
    crate::chronicle::chron_objective(g, &line, Some(rf));
    crate::handwork::drive_hands(g, crate::consts::DRIVE_SECS, true);
    g.mission.drive_t = -1.0;
    g.mission.pulled = true;
    g.mission.stage = crate::game::MissionStage::Exfil;
    g.mission.sweep_at = if crate::takecheck::take_told(g) { g.time + 7.0 } else { -1.0 };
    let mut i = 0;
    while i < g.rack_targets.len() {
        let t = g.rack_targets[i];
        let on = crate::sim_lightshome::light_in().on;
        crate::sim_lightshome::light_home_in(g, on, Some(t), Some(0.0), None, None, None);
        i += 1;
    }
    crate::audio_iface::game_audio_like_play(g, "rack", Some(rf), Some(0.5), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
    crate::audio_iface::game_audio_like_play(g, "lightOff", Some(rf), Some(0.7), None);
    let pl = g.player;
    let who = g.ch[g.pl[pl].ch].id;
    let time = g.time;
    let serial = g.serial();
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos: [rf[0], 0.0, rf[2]],
        time,
        loud: false,
        level: Some(0.35),
        who: Some(who),
        serial,
        ..crate::game::GameEvent::zeroed()
    });
    g.pl[pl].noise = m::max(g.pl[pl].noise, 0.35);
    if let Some(w) = &w {
        crate::game::game_msg(g, &format!("{} out. leave through {}", w.bare, w.outs));
        return;
    }
    crate::game::game_msg(g, &format!("the drive is out and rack {} is dark. leave through the fire exit", n));
    crate::dialogue::handler_beat(g, crate::dialogue::HandlerBeat::Drive, Some(1.5));
}

/// The dead rack is noticed: the nearest calm patrol goes to look at it. (mission.ts sendRackCheck)
pub fn send_rack_check(g: &mut crate::game::Game) {
    use crate::game::{GuardState, GuardTask, GuardTaskArm0, GuardTaskArm0Kind};
    let pl = g.player;
    if g.pl[pl].down {
        return;
    }
    let front = g.level.mission.rack.front;
    let w = crate::quiet::quiet_words(g);
    let goal: V3 = if w.is_some() {
        let area = {
            let r = &g.level.mission.server_room;
            crate::quiet::HolderStandArea { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }
        };
        match crate::quiet::holder_stand(g, front, Some(area)).pos {
            Some(pos) => pos,
            None => [front[0], 0.0, front[2]],
        }
    } else {
        [front[0], 0.0, front[2] + 1.1]
    };
    let mut best: Option<GuardId> = None;
    let mut bd = 1e9;
    for &gd in g.guards.iter() {
        let t = &g.gd[gd];
        if t.state != GuardState::Patrol || t.hold || t.held.is_some() || t.script.is_some() || t.unit.is_some() || t.team != crate::teams::Team::Guard || t.task.is_some() {
            continue;
        }
        let d = v3::dist(g.ch[t.ch].pos, goal);
        if d < bd {
            bd = d;
            best = Some(gd);
        }
    }
    let Some(best) = best else { return };
    {
        g.gd[best].task = Some(GuardTask::Arm0(GuardTaskArm0 { kind: GuardTaskArm0Kind::Rack, pos: goal }));
        crate::mind::fix(g, best, Some(v3::copy(goal)), "mission");
        crate::mind::raise(g, best, 0.42, "mission");
        crate::mind::set_state(g, best, GuardState::Suspicious, "mission");
        g.gd[best].react_t = 1.2;
        g.gd[best].search_t = 0.0;
        g.gd[best].path = Vec::new();
        g.gd[best].path_serial = g.gd[best].path_serial.wrapping_add(1);
        g.gd[best].path_goal = None; g.gd[best].path_goal_orbit = false;   // (guard_set_path_goal's rule, for the row in hand)
    }
    if let Some(w) = &w {
        crate::game::game_say(g, best, &format!("something's tripped in {} — going to take a look", w.place), Some(true), None, None);
        let at = g.ch[g.gd[best].ch].pos;
        crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.6), None);
        return;
    }
    let n = js::num_str(g.level.mission.rack.index + 1.0);
    crate::game::game_say(g, best, &format!("rack {} just dropped off the network — going to take a look", n), Some(true), None, None);
    let at = g.ch[g.gd[best].ch].pos;
    crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.6), None);
    crate::dialogue::handler_beat(g, crate::dialogue::HandlerBeat::Rack, Some(2.6));
}

/// Where the HUD's objective marker points; None: nothing to point at, or the tour is running. (mission.ts objectivePos)
pub fn objective_pos(g: &mut crate::game::Game) -> Option<V3> {
    use crate::game::MissionStage;
    if (g.quiet_utility || g.touring) && !g.mission_in_tour {
        return None;
    }
    if let Some(o) = crate::director::director_objective(g) {
        return o.pos;
    }
    if let Some(o) = crate::lesson::lesson_objective(g) {
        return o.pos;
    }
    if let Some(o) = crate::snatch::snatch_objective(g) {
        return o.pos;
    }
    if let Some(o) = crate::defuse::defuse_objective(g) {
        return o.pos;
    }
    if let Some(o) = crate::roomclear::room_clear_scenario_objective(g) {
        return o.pos;
    }
    if let Some(o) = crate::rescue::rescue_objective(g) {
        return o.pos;
    }
    if let Some(o) = crate::nightbreach::night_breach_objective(g) {
        return o.pos;
    }
    let s = g.mission.stage;
    if s == MissionStage::Exfil {
        let p = player_pos(g, &crate::seats::here(g));
        if let Some(x) = crate::quiet::nearest_exit(g, p) {
            return Some(match crate::doors::doors_by_name(g, &x.door) {
                Some(d) => g.dr[d].frame_centre,
                None => crate::quiet::exit_point(&x, Some(0.0)),
            });
        }
    }
    match s {
        MissionStage::Infiltrate => door_centre(g, &g.level.mission.entry_door.clone()),
        MissionStage::Drive => Some(g.level.mission.rack.front),
        MissionStage::Exfil => door_centre(g, &g.level.mission.exfil_door.clone()),
        _ => None,
    }
}

/// The HUD's one objective line; empty while the tour runs. (mission.ts objectiveText)
pub fn objective_text(g: &mut crate::game::Game) -> String {
    use crate::game::MissionStage;
    if (g.quiet_utility || g.touring) && !g.mission_in_tour {
        return String::new();
    }
    if let Some(o) = crate::director::director_objective(g) {
        return format!("objective ▸ {}", o.text);
    }
    if let Some(o) = crate::lesson::lesson_objective(g) {
        return o.text;
    }
    if let Some(o) = crate::defuse::defuse_objective(g) {
        return o.text;
    }
    if let Some(o) = crate::snatch::snatch_objective(g) {
        return o.text;
    }
    if let Some(o) = crate::roomclear::room_clear_scenario_objective(g) {
        return crate::par::par_objective(g, &o.text);
    }
    if let Some(o) = crate::rescue::rescue_objective(g) {
        return o.text;
    }
    if let Some(o) = crate::nightbreach::night_breach_objective(g) {
        return o.text;
    }
    let s = g.mission.stage;
    let drive_t = g.mission.drive_t;
    let n = js::num_str(g.level.mission.rack.index + 1.0);
    if let Some(w) = crate::quiet::quiet_words(g) {
        match s {
            MissionStage::Infiltrate => return format!("objective ▸ reach {}", w.place),
            MissionStage::Drive if drive_t >= 0.0 => return format!("objective ▸ taking {}… {}%", w.thing, drive_percent(drive_t)),
            MissionStage::Drive => return format!("objective ▸ take {} from {} (press Space in front of it, then stay on it)", w.thing, w.holder),
            MissionStage::Exfil => return format!("objective ▸ get out through {}", w.outs),
            _ => {}
        }
    }
    match s {
        MissionStage::Infiltrate => String::from("objective ▸ reach the server room"),
        MissionStage::Drive if drive_t >= 0.0 => format!("objective ▸ pulling the drive… {}%", drive_percent(drive_t)),
        MissionStage::Drive => format!("objective ▸ pull the drive from rack {} (press Space in front of it, then stay on it)", n),
        MissionStage::Exfil => String::from("objective ▸ get out through the fire exit"),
        MissionStage::Done => {
            let key = g.mission.serial;
            if let Some(t) = g.m.mission.done_line.get(key) {
                return t.clone();
            }
            let d = crate::chronicle::debrief_of(g, &crate::seats::here(g));
            let mut t = format!("mission complete — {}", crate::game::fmt_clock(g.mission.stats.elapsed));
            if !d.grade.is_empty() {
                t.push_str(" · ");
                t.push_str(&d.grade);
            }
            g.m.mission.done_line.set(key, t.clone());
            t
        }
        _ => String::from("mission failed — press enter to restart"),
    }
}

/// `Math.min(99, Math.round(m.driveT / DRIVE_SECS * 100))`, the pull's percent on the HUD (mission.ts:166, 169).
fn drive_percent(drive_t: f64) -> String {
    js::num_str(m::min(99.0, m::round(drive_t / crate::consts::DRIVE_SECS * 100.0)))
}

/// `g.doors.byName(name)?.frameCentre ?? null` (mission.ts:150).
fn door_centre(g: &Game, name: &str) -> Option<V3> {
    crate::doors::doors_by_name(g, name).map(|d| g.dr[d].frame_centre)
}

/// `g.player.char.pos`, read where the TypeScript's alias `p` is read.
fn player_pos(g: &Game, seat: &crate::seats::SeatRef) -> V3 {
    g.ch[g.pl[seat.player].ch].pos
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::MissionStage;

    fn game() -> Game {
        let g = Game::blank();
        g
    }

    /// rackDist's plain arm against mission.ts's arithmetic run under bun with EXACT on.
    #[test]
    fn rack_dist_plain_has_the_typescripts_bits() {
        let (front, half_w) = ([3.7, 0.0, -2.3], 0.45);
        assert_eq!(rack_dist_plain(front, half_w, [5.1, 0.0, -1.9]).to_bits(), 0x3ff07e0f66afed04);
        assert_eq!(rack_dist_plain(front, half_w, [3.8, 0.0, 4.4]).to_bits(), 0x401acccccccccccd);
    }

    #[test]
    fn out_now_goes_by_the_exits_own_way_out() {
        assert!(out_plain(10.0, 2.0, None, [10.5, 0.0, 3.0]));
        assert!(!out_plain(10.0, 2.0, None, [9.5, 0.0, 3.0]));
        assert!(!out_plain(10.0, 2.0, None, [10.5, 0.0, 3.9]));
        let o = Some([0.0, 1.0]);
        assert!(out_plain(10.0, 2.0, o, [10.5, 0.0, 3.0]));
        assert!(!out_plain(10.0, 2.0, o, [12.0, 0.0, 3.0]));
        assert!(!out_plain(10.0, 2.0, o, [10.5, 0.0, 1.0]));
    }

    #[test]
    fn a_tally_counts_only_on_a_live_run() {
        let mut g = game();
        g.mission.stage = MissionStage::Infiltrate;
        tally(&mut g, TallyK::Shots);
        assert_eq!(g.mission.stats.shots, 1.0);
        g.mission.stage = MissionStage::Done;
        tally(&mut g, TallyK::Shots);
        assert_eq!(g.mission.stats.shots, 1.0);
        g.mission.stage = MissionStage::Infiltrate;
        g.quiet_utility = true;
        tally(&mut g, TallyK::Shots);
        assert_eq!(g.mission.stats.shots, 1.0);
    }

    #[test]
    fn the_tour_lets_go_of_a_pull_and_nothing_else_moves() {
        let mut g = game();
        g.mission.stage = MissionStage::Drive;
        g.mission.drive_t = 1.0;
        g.quiet_utility = true;
        update_mission(&mut g, 0.1, InputId(0));
        assert_eq!(g.mission.drive_t, -1.0);
        assert!(g.mission.stage == MissionStage::Drive);
        assert_eq!(g.mission.stats.elapsed, 0.0);
    }

    /// The HUD's percent, against the TypeScript's for the same times; it stops at 99 until the drive is out.
    #[test]
    fn the_drive_percent_stops_at_99() {
        assert_eq!(drive_percent(0.0), "0");
        assert_eq!(drive_percent(crate::consts::DRIVE_SECS / 2.0), "50");
        assert_eq!(drive_percent(crate::consts::DRIVE_SECS * 0.9999), "99");
    }

    #[test]
    fn the_tour_has_no_objective() {
        let mut g = game();
        g.touring = true;
        assert!(objective_pos(&mut g).is_none());
        assert_eq!(objective_text(&mut g), "");
    }
}
