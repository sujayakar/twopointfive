#![allow(unused_imports, unused_variables)]
//! THE BRING-UP'S OWN PART of startle: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)

pub use ::game_types::startle::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec::Vec};

/// the script startle.ts lent for the frame, as the run left it, put back on his row: startle.ts:133 reads its `arrived` (conventions 4).
pub(in crate::man) fn startle_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(s)) = (ran, g.m.startle.state_of.get_mut(gd)) { if s.script.is_some() { s.script = Some(ran); } }
}

/// A man's record as it starts: never startled, never calm.
pub fn blank() -> crate::startle::St {
    crate::startle::St {
        calm_t: -100.0,
        calm_torch: false,
        by_sight: false,
        at: -1.0,
        until: -1.0,
        toward: [0.0, 0.0, 0.0],
        pre: false,
        torch: false,
        choose: false,
        call: false,
        draw_until: -1.0,
        run: None,
        run_at: -1.0,
        ref_: None,
        ref_t: -1.0,
        script: None,
    }
}

/// THE START, for any caller: he jolts toward `at`; for `secs` his gun stays down. False = not given.
pub fn give_startle(g: &mut crate::game::Game, gd: GuardId, at: V3, secs: Option<f64>, torch: Option<bool>) -> bool {
    let secs = secs.unwrap_or(g.k.startle.beat);
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || g.gd[gd].civ.is_some() || crate::owner::has(g, gd, crate::owner::by::RUNNER) { return false; }
    if g.time < g.m.startle.state_of.get_or_insert_with(gd, blank).until { return false; }
    let c = g.gd[gd].ch;
    let torch = match torch { Some(t) => t, None => crate::guards::hand_carry(g, gd) == crate::guards::HandCarry::Torch };
    let (now, p) = (g.time, g.ch[c].pos);
    let s = g.m.startle.state_of.get_mut(gd).unwrap();
    s.torch = torch; s.at = now; s.until = now + secs; s.toward = [at[0], 0.0, at[2]]; s.pre = false; s.choose = false;
    crate::character::anim_startle_from(g, c, Some([at[0] - p[0], 0.0, at[2] - p[2]]));
    g.stats.startle.given += 1;
    true
}

/// updateGuard, right after the state transitions: the end of a beat, the calm he last stood in, the first sight itself.
pub(in crate::man) fn startle_sight(g: &mut crate::game::Game, gd: GuardId, prev: crate::game::GuardState, seen: bool) {
    if let Some(s) = g.m.startle.state_of.get_mut(gd) {
        if s.until >= 0.0 && g.time >= s.until {
            let late = g.time - s.until > g.k.startle.late;
            s.until = -1.0; s.torch = false;
            if s.choose {
                s.choose = false;
                if !late { choose(g, gd); } else { s.call = false; }
            }
        }
    }
    if g.touring || g.quiet_utility {
        if let Some(s) = g.m.startle.state_of.get_mut(gd) { s.pre = false; if s.until < 0.0 { s.torch = false; } }
        return;
    }
    if !g.ai_enabled || g.gd[gd].civ.is_some() { return; }
    let team = g.gd[gd].team;
    if !crate::teams::is_hostile(g, team, crate::teams::Team::Player) { return; }
    let state = g.gd[gd].state;
    let now = g.time;
    if state == crate::game::GuardState::Patrol {
        g.m.startle.state_of.get_or_insert_with(gd, blank).calm_t = now;
        let carried = crate::guards::hand_carry(g, gd) == crate::guards::HandCarry::Torch;
        let s = g.m.startle.state_of.get_mut(gd).unwrap();
        s.calm_torch = carried; s.by_sight = false; s.pre = false;
        return;
    }
    let fresh = g.k.startle.fresh;
    let Some(s) = g.m.startle.state_of.get_mut(gd) else { return };
    if prev == crate::game::GuardState::Patrol { s.by_sight = seen; }
    let unaware = s.at < 0.0 && now - s.calm_t <= fresh;
    s.pre = unaware && state == crate::game::GuardState::Suspicious && s.by_sight;
    if s.pre { s.torch = s.calm_torch; } else if s.until < 0.0 { s.torch = false; }
    if !unaware || state != crate::game::GuardState::Alert || !seen { return; }
    let calm_torch = s.calm_torch;
    if now < g.gd[gd].dazzled_until || now < g.gd[gd].stagger_until { return; }
    let at = if now - g.gd[gd].sight_t < 0.1 { g.gd[gd].sight_pos } else { g.gd[gd].last_known.unwrap_or(g.gd[gd].sight_pos) };
    let beat = g.k.startle.beat;
    if give_startle(g, gd, at, Some(beat), Some(calm_torch)) {
        g.m.startle.state_of.get_mut(gd).unwrap().choose = true;
        g.stats.startle.seen += 1;
        let call = match &g.gd[gd].bubble { Some(b) => b.t == g.time && b.text == "CONTACT!", None => false };
        g.m.startle.state_of.get_mut(gd).unwrap().call = call;
        if call { g.gd[gd].bubble = None; }
    }
}

/// updateGuard, at the pose write: his gun is held down this frame.
pub(in crate::man) fn startle_gun_down(g: &crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    match g.m.startle.state_of.get(gd) { Some(s) => now < s.until || s.pre || s.run.is_some(), None => false }
}

/// handCarry: the hand torch is still what he carries (he had it when the start took him).
pub(in crate::man) fn startle_torch(g: &crate::game::Game, gd: GuardId) -> bool {
    let Some(s) = g.m.startle.state_of.get(gd) else { return false };
    s.torch && (g.gd[gd].state == crate::game::GuardState::Alert || g.gd[gd].state == crate::game::GuardState::Suspicious) && g.gd[gd].script.is_none()
}

/// Is he in a start right now, anybody's.
pub fn startled(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    match g.m.startle.state_of.get(gd) { Some(s) => now < s.until, None => false }
}

/// The seconds of a start still on him, and of the draw after it (0: none): the least his reaction clock may read.
pub fn startle_left(g: &crate::game::Game, gd: GuardId) -> f64 {
    let now = g.time;
    let Some(s) = g.m.startle.state_of.get(gd) else { return 0.0 };
    if now < s.until { s.until - now } else if now < s.draw_until { s.draw_until - now } else { 0.0 }
}

/// Is he running for cover off a start.
pub(in crate::man) fn startle_running(g: &crate::game::Game, gd: GuardId) -> bool {
    match g.m.startle.state_of.get(gd) { Some(s) => s.run.is_some(), None => false }
}

/// The run ends here; on his own feet he holds the spot as a man shifted under fire holds his.
fn stop(g: &mut crate::game::Game, gd: GuardId, hold: bool) -> Option<crate::game::GuardScript> {
    g.m.startle.state_of.get_mut(gd).unwrap().run = None;
    let now = g.time;
    crate::owner::release(g, gd, crate::owner::by::STARTLE, now);
    let s = g.m.startle.state_of.get_mut(gd).unwrap();
    s.ref_ = None; s.script = None;
    if hold { crate::suppression::end_displace(g, gd, "ran for it"); }
    None
}

/// THE RULE'S OWN STATE, moved a tick (the first of a rule's three parts, control/): a run ends where he cannot go on with it, where he is stuck, where
/// he has arrived or it has taken too long; else its stuck watch is kept and its script is made.
pub(in crate::man) fn startle_transition(g: &mut crate::game::Game, gd: GuardId) {
    let Some(run) = g.m.startle.state_of.get(gd).and_then(|s| s.run) else { return };
    let now = g.time;
    let m = &g.gd[gd];
    if !g.ai_enabled || g.touring || g.quiet_utility || m.state != crate::game::GuardState::Alert || m.held.is_some() || m.script.is_some() || now < m.dazzled_until || now < m.stagger_until { stop(g, gd, false); return; }
    let pos = g.ch[m.ch].pos;
    let (stuck_secs, stuck_m, run_secs) = (g.k.startle.stuck_secs, g.k.startle.stuck_m, g.k.startle.run_secs);
    let s = g.m.startle.state_of.get_mut(gd).unwrap();
    match s.ref_ {
        None => { s.ref_ = Some(v3::copy(pos)); s.ref_t = now; }
        Some(r) => {
            if now - s.ref_t >= stuck_secs {
                if v3::dist_xz(pos, r) < stuck_m { stop(g, gd, true); return; }
                s.ref_ = Some(v3::copy(pos)); s.ref_t = now;
            }
        }
    }
    if s.script.and_then(|x| x.arrived).unwrap_or(false) || v3::dist_xz(pos, run) < 0.4 || now - s.run_at > run_secs { stop(g, gd, true); return; }
    if s.script.is_none() {
        s.script = Some(crate::game::GuardScript {
            goal: Some(run),
            speed: Some(g.tune.guard_chase),
            face: None,
            strafe: Some(false),
            aim_at: None,
            upper: Some(crate::character::UpperMode::None),
            stance: Some(crate::character::Stance::None),
            crouch: Some(true),
            ..Default::default()
        });
    }
}

/// WHAT IT WANTS OF HIM (the second part; it only reads): the order a man running for cover follows this tick, None = he is not this rule's.
pub(in crate::man) fn startle_propose(g: &crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> {
    let s = g.m.startle.state_of.get(gd)?;
    if s.run.is_some() { s.script } else { None }
}

/// The beat is over: he draws (the fight as it was, from here) or he runs.
pub fn choose(g: &mut crate::game::Game, gd: GuardId) {
    let now = g.time;
    if g.gd[gd].state != crate::game::GuardState::Alert || g.gd[gd].held.is_some() || now < g.gd[gd].dazzled_until || now < g.gd[gd].stagger_until { return; }
    if runs(g, gd) {
        g.stats.startle.ran += 1;
    } else {
        g.stats.startle.drew += 1;
        let until = g.time + g.k.startle.draw;
        let row = g.m.startle.state_of.get_mut(gd).unwrap();
        row.draw_until = until;
        if row.call { crate::game::game_say(g, gd, "CONTACT!", Some(false), Some(false), Some(false)); }
    }
    g.m.startle.state_of.get_mut(gd).unwrap().call = false;
}

/// A green man alone, caught inside `run_m` and not inside `near_m`, runs for it.
pub fn runs(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let toward = g.m.startle.state_of.get(gd).unwrap().toward;
    let threat = g.gd[gd].last_known.unwrap_or(toward);
    let far = v3::dist_xz(g.ch[c].pos, threat);
    if crate::morale::training_of(g, gd) != crate::morale::TrainingOf::Green || far > g.k.startle.run_m || far < g.k.startle.near_m || crate::morale::has_mate(g, gd) { return false; }
    let clear_secs = g.k.startle.clear_secs;
    if let Some(post) = crate::roomfight::room_fight_bolt(g, gd, clear_secs) { return post; }
    if g.gd[gd].pinned || g.gd[gd].standoff.is_some() || !g.tune.suppression { return false; }
    let Some(p) = crate::suppression::cover_from(g, gd, threat) else { return false };
    let pos = g.ch[c].pos;
    let way = crate::scene_collision::nav_grid_find_path(g, pos, p, None, None);
    if way.map_or(true, |w| w.is_empty()) { return false; }
    let now = g.time;
    g.m.startle.state_of.get_mut(gd).unwrap().run = Some(p);
    crate::owner::claim(g, gd, crate::owner::by::STARTLE, now, None);
    let row = g.m.startle.state_of.get_mut(gd).unwrap();
    row.run_at = now; row.ref_ = None; row.script = None;
    crate::game::guard_set_path(g, gd, Vec::new());
    let man = &mut g.gd[gd];
    man.path_goal = None; man.path_goal_orbit = false; man.stall_ref = None;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GuardState;

    const GD: GuardId = GuardId(0);

    #[test]
    fn a_new_row_has_the_makers_numbers_not_zeros() {
        let s = blank();
        assert_eq!((s.calm_t, s.at, s.until, s.draw_until, s.run_at, s.ref_t), (-100.0, -1.0, -1.0, -1.0, -1.0, -1.0));
        assert!(s.run.is_none() && s.ref_.is_none() && s.script.is_none());
    }

    #[test]
    fn the_clocks_run_from_the_start_through_the_draw_to_nothing() {
        let mut g = Game::blank();
        assert_eq!(startle_left(&mut g, GD), 0.0);
        assert!(!startled(&mut g, GD) && !startle_gun_down(&mut g, GD) && !startle_running(&mut g, GD));
        g.m.startle.state_of.set(GD, St { until: 2.0, draw_until: 2.5, ..blank() });
        g.time = 1.0;
        assert_eq!(startle_left(&mut g, GD), 1.0);
        assert!(startled(&mut g, GD) && startle_gun_down(&mut g, GD));
        g.time = 2.25;
        assert_eq!(startle_left(&mut g, GD), 0.25);
        assert!(!startled(&mut g, GD) && !startle_gun_down(&mut g, GD));
        g.time = 2.5;
        assert_eq!(startle_left(&mut g, GD), 0.0);
    }

    #[test]
    fn a_beat_over_too_long_ago_chooses_nothing_and_a_show_clears_the_hold() {
        let mut g = Game::blank();
        g.touring = true;
        g.m.startle.state_of.set(GD, St { until: 1.0, torch: true, pre: true, choose: true, call: true, ..blank() });
        g.time = 1.0 + g.k.startle.late + 0.5;
        startle_sight(&mut g, GD, GuardState::Patrol, false);
        let s = g.m.startle.state_of.get(GD).unwrap();
        assert_eq!(s.until, -1.0);
        assert!(!s.choose && !s.call && !s.torch && !s.pre);
        assert_eq!(s.draw_until, -1.0);
    }
}
