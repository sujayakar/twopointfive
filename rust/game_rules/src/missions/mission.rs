pub use ::game_types::mission::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec::Vec};

/// Live: a run is on and it really is the player playing.
pub fn mission_live(g: &crate::game::Game) -> bool {
    let s = g.mission.stage;
    !g.aside && s != crate::game::MissionStage::Done && s != crate::game::MissionStage::Failed
}

/// One more of a counted thing on the run's card, if the run is live.
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

/// How far a floor point is from the face the errand's thing is taken from, across its width; never, with something solid between.
pub fn rack_dist(g: &crate::game::Game, p: V3) -> f64 {
    let (front, half_w) = (g.level.errand.front, g.level.errand.half_w);
    let d = v3::dist_xz(p, front);
    if d > 0.3 && crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [front[0] + (p[0] - front[0]) / d * 0.2, 1.0, front[2] + (p[2] - front[2]) / d * 0.2], None) {
        return m::INF;
    }
    m::max(0.0, d - half_w)
}

/// the way out he went by: the nearest whose line he is past
fn exit_taken(g: &crate::game::Game, p: V3) -> Option<crate::scene_scenarios::QuietExit> { nearest(g, p, |x| crate::quiet::out_through(x, p)) }

/// the way out nearest `p` as the crow flies
fn nearest_exit(g: &crate::game::Game, p: V3) -> Option<crate::scene_scenarios::QuietExit> { nearest(g, p, |_| true) }

fn nearest(g: &crate::game::Game, p: V3, may: impl Fn(&crate::scene_scenarios::QuietExit) -> bool) -> Option<crate::scene_scenarios::QuietExit> {
    let (mut best, mut bd) = (None, m::INF);
    for x in g.level.errand.exits.iter().filter(|x| may(x)) {
        let d = m::hyp2(p[0] - x.x, p[2] - x.z);
        if d < bd { bd = d; best = Some(x.clone()); }
    }
    best
}

/// Out through one of the errand's ways out.
fn out_now(g: &crate::game::Game, p: V3) -> bool { g.level.errand.exits.iter().any(|x| crate::quiet::out_through(x, p)) }

/// The mission's stage machine: infiltrate, drive, exfil, and the run's counts.
pub fn update_mission(g: &mut crate::game::Game, dt: f64) {
    use crate::game::MissionStage;
    if g.aside {
        g.pl[g.player].taking_over();
        return;
    }
    if g.mission.pulled { the_rack_goes_dark(g); }
    if g.mission.sweep_at > 0.0 && g.time >= g.mission.sweep_at {
        g.mission.sweep_at = -1.0;
        send_rack_check(g);
    }
    if g.mission.stage == MissionStage::Failed && g.mission.down_card_at > 0.0 && g.time >= g.mission.down_card_at {
        g.mission.down_card_at = -1.0;
    }
    if g.mission.stage == MissionStage::Done || g.mission.stage == MissionStage::Failed {
        g.pl[g.player].taking_over();
        return;
    }
    if crate::seats::all_down(g) {
        g.mission.stage = MissionStage::Failed;
        g.mission.down_card_at = g.time + DOWN_CARD_SECS;
        return;
    }
    g.mission.stats.elapsed += dt;
    if g.god_mode || g.player_invisible || !g.ai_enabled {
        g.mission.stats.sandbox = true;
    }
    crate::chronicle::tick_chronicle(g);
    count_alarms_and_sightings(g);
    // a mission that goes by its own rule has the floor: no stage of the stealth mission moves under it
    if g.staged.facts.own_rule { return; }
    match g.mission.stage {
        MissionStage::Infiltrate => reach_the_area(g),
        MissionStage::Drive => work_the_drive(g, dt),
        MissionStage::Exfil => get_out(g),
        _ => {}
    }
}

/// Every fixture of the rack off.
fn the_rack_goes_dark(g: &mut Game) {
    for &t in &g.rack_targets {
        g.tab.fixture[t].on = false;
    }
}

/// An alarm counts when it starts; a sighting counts when it is a fresh one after 4 s unseen: an alert guard with eyes on you.
fn count_alarms_and_sightings(g: &mut Game) {
    use crate::game::GuardState;
    let any_alert = g.guards.iter().any(|&gd| g.gd[gd].state == GuardState::Alert);
    if any_alert && !g.mission.alarm_on {
        g.mission.stats.alerts += 1.0;
    }
    g.mission.alarm_on = any_alert;
    let by = g.guards.iter().copied().find(|&gd| g.gd[gd].state == GuardState::Alert && g.gd[gd].saw_player_this_frame);
    if let Some(by) = by {
        if g.time - g.mission.spotted_t > 4.0 {
            g.mission.stats.spotted += 1.0;
            crate::chronicle::chron_spotted(g, Some(by));
        }
        g.mission.spotted_t = g.time;
    }
}

/// Inside the errand's area the drive begins.
fn reach_the_area(g: &mut Game) {
    let p = g.seat_pos(&crate::seats::here(g));
    let (x0, x1, z0, z1) = {
        let r = &g.level.errand.area;
        (r.x0, r.x1, r.z0, r.z1)
    };
    if p[0] > x0 && p[0] < x1 && p[2] > z0 && p[2] < z1 {
        let w = g.level.errand.clone();
        crate::chronicle::chron_objective(g, &format!("reached {}", w.place), None);
        g.mission.stage = crate::game::MissionStage::Drive;
        crate::dialogue::handler_beat(g, crate::dialogue::HandlerBeat::Server, Some(1.2));
    }
}

/// The pull: begun when he asks, stopped when he leaves it or does something else, done when its time is up.
fn work_the_drive(g: &mut Game, dt: f64) {
    let p = g.seat_pos(&crate::seats::here(g));
    let in_reach = rack_dist(g, p) < crate::consts::DRIVE_REACH;
    let pl = g.player;
    let Some(&at_it) = g.pl[pl].taking() else {
        begin_the_pull_if_asked(g);
        return;
    };
    if !in_reach || g.pl[pl].sprinting {
        g.pl[pl].taking_over();
    } else if at_it + dt >= crate::consts::DRIVE_SECS {
        drive_pulled(g);
    } else {
        if let Some(t) = g.pl[pl].taking_mut() { *t += dt; }
        crate::handwork::drive_hands(g, crate::consts::DRIVE_SECS, false);
    }
}

/// He has asked for the drive: the clicks are heard, and the first time it goes in the chronicle.
fn begin_the_pull_if_asked(g: &mut Game) {
    use crate::game::{InteractableKind, Verb};
    let asked = match &g.sets[g.seat].act {
        Some(a) => a.kind == InteractableKind::Objective && a.verb == Some(Verb::Drive),
        None => false,
    };
    if !asked { return; }
    let rf = g.level.errand.front;
    crate::deed::begin(g, crate::game::Deed::Take(0.0));
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Click, Some(rf), Some(0.8), None);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::MagOut, Some(rf), Some(0.4), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.55 }));
    let thing = g.level.errand.thing.clone();
    if !g.mission.hands_on {
        g.mission.hands_on = true;
        crate::chronicle::chron_objective(g, &format!("hands on {}", thing), Some(rf));
    }
}

/// Out through a way out, the mission is done.
fn get_out(g: &mut Game) {
    let p = g.seat_pos(&crate::seats::here(g));
    if out_now(g, p) {
        let by = exit_taken(g, p).map_or_else(|| g.level.errand.outs(), |x| x.name);
        crate::chronicle::chron_objective(g, &format!("out through {} — mission complete", by), None);
        g.mission.stage = crate::game::MissionStage::Done;
    }
}

/// The caddy comes free: the rack goes dark, the clunk is heard, and soon a man is sent to look.
pub fn drive_pulled(g: &mut crate::game::Game) {
    let w = g.level.errand.clone();
    let rf = w.front;
    crate::chronicle::chron_objective(g, &format!("{} is out of {}", w.thing, w.holder), Some(rf));
    crate::handwork::drive_hands(g, crate::consts::DRIVE_SECS, true);
    g.pl[g.player].taking_over();
    g.mission.pulled = true;
    g.mission.stage = crate::game::MissionStage::Exfil;
    g.mission.sweep_at = if w.told { g.time + 7.0 } else { -1.0 };
    the_rack_goes_dark(g);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Rack, Some(rf), Some(0.5), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LightOff, Some(rf), Some(0.7), None);
    let pl = g.player;
    let who = g.player_id(pl);
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
        ..crate::game::GameEvent::default()
    });
    g.pl[pl].noise = m::max(g.pl[pl].noise, 0.35);
    crate::dialogue::handler_beat(g, crate::dialogue::HandlerBeat::Drive, Some(1.5));
}

/// The dead rack is noticed: the nearest calm patrol goes to look at it.
pub fn send_rack_check(g: &mut crate::game::Game) {
    use crate::game::{GuardState, Errand, ErrandAt, ErrandThing};
    let pl = g.player;
    if crate::condition::down(g, g.pl[pl].ch) {
        return;
    }
    let front = g.level.errand.front;
    let area = { let r = &g.level.errand.area; crate::quiet::HolderStandArea { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 } };
    let goal: V3 = crate::quiet::holder_stand(g, front, Some(area)).pos.unwrap_or([front[0], 0.0, front[2]]);
    let mut best: Option<GuardId> = None;
    let mut bd = 1e9;
    for &gd in g.guards.iter() {
        let t = &g.gd[gd];
        if t.state != GuardState::Patrol || t.duty.holds || t.held.is_some() || t.script.is_some() || t.unit.is_some() || g.ch[t.ch].team != crate::teams::Team::Guard || t.errand.is_some() {
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
        g.gd[best].errand = Some(Errand::At(ErrandAt { kind: ErrandThing::Rack, pos: goal }));
        crate::mind::fix(g, best, Some(v3::copy(goal)), "mission");
        crate::mind::raise(g, best, 0.42, "mission");
        crate::mind::set_state(g, best, GuardState::Suspicious, "mission");
        g.gd[best].react_t = 1.2;
        g.gd[best].search_t = 0.0;
        g.gd[best].drop_path();
    }
    let line = format!("something's tripped in {} — going to take a look", g.level.errand.place);
    crate::game::game_say(g, best, &line, Some(true), None);
    let at = g.pos_of(best);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Radio, Some(at), Some(0.6), None);
    crate::dialogue::handler_beat(g, crate::dialogue::HandlerBeat::Rack, Some(2.6));
}

/// HOW THE RUN THAT IS UP HAS ENDED (None: it goes on): won or lost, the card's word and why, by the rule of whoever staged it. The verdict asks it each tick and latches the first answer.
pub fn ended(g: &crate::game::Game) -> Option<(bool, &'static str, String)> {
    if crate::seats::all_down(g) {
        let by = &g.chronicle.counts.down_by;
        return Some((false, "DOWN", if by.is_empty() { String::new() } else { format!("by {}", by) }));
    }
    use crate::game::MissionStage;
    use crate::scene_scenarios::MissionKind as K;
    match g.staged.up {
        Some(K::Roomclear) => crate::roomclear::room_clear_scenario_ended(g),
        Some(K::Rescue) => crate::rescue::rescue_ended(g),
        Some(K::Nightbreach) => crate::nightbreach::night_breach_ended(g),
        Some(K::Quiet) | None => match g.mission.stage { MissionStage::Done => Some((true, "OUT", String::new())), MissionStage::Failed => Some((false, "FAILED", String::new())), _ => None },
    }
}

/// The mission that is up has its men in an entry of its own, or a clock that runs.
pub fn going(g: &crate::game::Game) -> bool {
    use crate::scene_scenarios::MissionKind as K;
    match g.staged.up {
        Some(K::Roomclear | K::Nightbreach) => crate::nightbreach::night_breach_going(g),
        Some(K::Rescue) => crate::rescue::rescue_dynamic(g) && !g.staged.over,
        Some(K::Quiet) | None => false,
    }
}

/// The captives who live.
pub fn captives_up(g: &crate::game::Game) -> Vec<GuardId> { g.staged.captives.iter().copied().filter(|&h| !crate::condition::down(g, g.gd[h].ch)).collect() }

/// Who holds the captives now.
pub fn captor(g: &mut crate::game::Game) -> Option<GuardId> { if g.staged.captives.is_empty() { None } else { crate::rescue::rescue_captor(g) } }

/// A man of its foes whom the mission that is up leads itself (with the captives, or on their captor's word): it has claimed him, and no rule of a garrison has him.
pub fn leads(g: &crate::game::Game, gd: GuardId) -> bool { crate::owner::has(g, gd, crate::owner::by::MISSION) }

/// He lies in wait, and nothing said on the net may give his place away.
pub fn lies_in_wait(g: &crate::game::Game, gd: GuardId) -> bool { crate::mission::leads(g, gd) && crate::rescue::rescue_crew_hides(g, gd) }

/// The mission's own place for his gun, where it leads him and has one.
pub fn gun_place(g: &crate::game::Game, gd: GuardId) -> Option<V3> { if crate::mission::leads(g, gd) { crate::rescue::rescue_crew_look(g, gd) } else { None } }

/// The last warning before a captive dies, as the men at a door have it.
pub fn last_warning(g: &mut crate::game::Game, d: DoorId) -> bool { !g.staged.captives.is_empty() && crate::rescue::rescue_last_warning(g, d) }

/// A drill of the mission's own has this man, and its orders are not the menu's.
pub fn has(g: &crate::game::Game, gd: GuardId) -> bool { crate::nightbreach::drill_has(g, gd) }

/// A menu order came for a man of its drill: he is let go of (and with him the pair), or why not.
pub fn lets_go(g: &mut crate::game::Game, gd: GuardId) -> Option<String> { if crate::nightbreach::night_breach_drill_ops(g).contains(&gd) { crate::nightbreach::night_breach_call_off(g) } else { None } }

/// What a man of its drill is doing, in the menu's words.
pub fn doing(g: &crate::game::Game, gd: GuardId) -> Option<String> { crate::nightbreach::drill_doing(g, gd) }

/// What of its own waits at a door for the second word: the drill's pair (true), or an entry on the captives' room (false).
pub fn waits(g: &crate::game::Game) -> Vec<(DoorId, Vec<GuardId>, bool)> {
    let mut out: Vec<(DoorId, Vec<GuardId>, bool)> = crate::nightbreach::drill_waits(g).map(|(d, men)| (d, men, true)).into_iter().collect();
    out.extend(crate::rescue::rescue_waits(g).into_iter().map(|(d, men)| (d, men, false)));
    out
}

/// A man stood on its floor after the staging joins its foes.
pub fn adopt(g: &mut crate::game::Game, gd: GuardId, mark: &crate::nightbreach::NightBreachMark) { crate::nightbreach::night_breach_adopt(g, gd, mark); }

/// THE MISSIONS' TICK, each its own while it is up, and the squads its men are in.
pub(crate) fn mission_tick(g: &mut crate::game::Game, dt: f64, input: InputId, cam: Option<CameraId>) {
    crate::nightbreach::night_breach_tick(g, dt, cam);
    crate::quiet::quiet_mission_tick(g, cam);
    crate::enemysquads::squads_tick(g);
    crate::roomclear::room_clear_scenario_tick(g, dt);
    crate::rescue::rescue_tick(g, dt, input, cam);
}

/// THE MISSION'S ONE LINE AND ITS PLACE, as they stand now, and whose they are: a level's program while it runs, a lesson's step over its own scenario,
/// the room clear, the rescue, the night breach by itself, and else the stealth mission's own stages.
/// Empty while nobody plays. A read: the HUD's view of it is `view::MissionView`, and nothing of it is kept.
pub fn objective(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> crate::view::MissionView {
    use crate::view::{MissionOf, MissionView};
    if g.aside { return MissionView::default(); }
    let of = |of: MissionOf, text: String, pos: Option<V3>| MissionView { of, text, pos };
    if let Some(o) = crate::director::director_objective(g) { return of(MissionOf::Script, format!("objective ▸ {}", o.text), o.pos); }
    if let Some(o) = crate::lesson::lesson_objective(g, seat) { return of(MissionOf::Lesson, o.text, o.pos); }
    if let Some(o) = crate::roomclear::room_clear_scenario_objective(g, seat) { return of(MissionOf::Roomclear, crate::par::par_objective(g, &o.text), o.pos); }
    if let Some(o) = crate::rescue::rescue_objective(g, seat) { return of(MissionOf::Rescue, o.text, o.pos); }
    if let Some(o) = crate::nightbreach::night_breach_objective(g) { return of(MissionOf::Nightbreach, o.text, o.pos); }
    of(MissionOf::Stages, stage_words(g, seat), stage_place(g, seat))
}

/// the stealth mission's own place, by its stage
fn stage_place(g: &Game, seat: &crate::seats::SeatRef) -> Option<V3> {
    use crate::game::MissionStage;
    match g.mission.stage {
        MissionStage::Infiltrate => door_centre(g, &g.level.errand.door),
        MissionStage::Drive => Some(g.level.errand.front),
        // (the nearest of its ways out)
        MissionStage::Exfil => nearest_exit(g, g.seat_pos(seat)).map(|x| door_centre(g, &x.door).unwrap_or_else(|| crate::quiet::exit_point(&x, Some(0.0)))),
        _ => None,
    }
}

/// the stealth mission's own words, by its stage
fn stage_words(g: &Game, seat: &crate::seats::SeatRef) -> String {
    use crate::game::MissionStage;
    let s = g.mission.stage;
    let taking = g.pl[seat.player].taking();
    let w = &g.level.errand;
    match s {
        MissionStage::Infiltrate => format!("objective ▸ reach {}", w.place),
        MissionStage::Drive => match taking {
            Some(&t) => format!("objective ▸ taking {}… {}%", w.thing, drive_percent(t)),
            None => format!("objective ▸ take {} from {} (press Space in front of it, then stay on it)", w.thing, w.holder),
        },
        MissionStage::Exfil => format!("objective ▸ get out through {}", w.outs()),
        // (the run's clock and its counts stand still once it is done)
        MissionStage::Done => {
            let d = crate::chronicle::debrief_of(g, seat);
            let mut t = format!("mission complete — {}", crate::game::fmt_clock(g.mission.stats.elapsed));
            if !d.grade.is_empty() { t.push_str(" · "); t.push_str(&d.grade); }
            t
        }
        _ => String::from("mission failed — press enter to restart"),
    }
}

/// `Math.min(99, Math.round(m.driveT / DRIVE_SECS * 100))`, the pull's percent on the HUD.
fn drive_percent(drive_t: f64) -> String {
    js::num_str(m::min(99.0, m::round(drive_t / crate::consts::DRIVE_SECS * 100.0)))
}

/// `g.doors.byName(name)?.frameCentre ?? null`.
fn door_centre(g: &Game, name: &str) -> Option<V3> {
    crate::doors::doors_by_name(g, name).map(|d| g.dr[d].frame_centre)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::MissionStage;

    fn game() -> Game {
        let g = Game::blank();
        g
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
        g.aside = true;
        tally(&mut g, TallyK::Shots);
        assert_eq!(g.mission.stats.shots, 1.0);
    }

    #[test]
    fn with_nobody_playing_a_pull_is_let_go_of_and_nothing_else_moves() {
        let mut g = game();
        g.mission.stage = MissionStage::Drive;
        g.pl.push(crate::game::Player { deed: Some(crate::game::Deed::Take(1.0)), ..crate::game::Player::blank() });
        g.aside = true;
        update_mission(&mut g, 0.1);
        assert!(g.pl[g.player].deed.is_none());
        assert!(g.mission.stage == MissionStage::Drive);
        assert_eq!(g.mission.stats.elapsed, 0.0);
    }

    /// The HUD's percent stops at 99 until the drive is out.
    #[test]
    fn the_drive_percent_stops_at_99() {
        assert_eq!(drive_percent(0.0), "0");
        assert_eq!(drive_percent(crate::consts::DRIVE_SECS / 2.0), "50");
        assert_eq!(drive_percent(crate::consts::DRIVE_SECS * 0.9999), "99");
    }
}
