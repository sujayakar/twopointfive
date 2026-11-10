#![allow(unused_imports, unused_variables)]
#![allow(unused_variables, unused_imports)]
//! levels/tut-quiet.script.ts translated: its define pass and a step function for each beat.

pub use ::game_types::script_tut_quiet::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// the script's locals, as this run's define pass left them
fn tut_quiet_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_quiet::TutQuietVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutQuiet(v) => v, _ => panic!("the run is not of levels/tut-quiet.script.ts") } }

/// where he looks from, where he smokes, and the lane's side of the level
const LOOK: V3 = [26.4, 0.0, 17.75];
const SMOKE: V3 = [30.6, 0.0, 17.7];
const CROSS_WALL_X: f64 = 25.5;
/// how long he looks, how long the smoke lasts, and how far off boots are heard with his back turned
const LOOK_SECS: f64 = 3.6;
const SMOKE_SECS: f64 = 50.0;
const RUN_LEVEL: f64 = 0.34;
const RUN_WITHIN: f64 = 6.0;
const WALK_LEVEL: f64 = 0.14;
const WALK_WITHIN: f64 = 2.4;

// The ids of night's six c.until tests, in source order.
const TEST_TAKEN: u32 = 0;
const TEST_MET: u32 = 1;
const TEST_LIT: u32 = 2;
const TEST_AT_GAP: u32 = 3;
const TEST_PAST: u32 = 4;
const TEST_AT_CHAIR: u32 = 5;

// night's places that are not yields: `for (;;)`, the order round the wall, and the reading of `r` after the smoke. 1 to 6 are its yields.
const NIGHT_LOOP: u16 = 7;
const NIGHT_ROUND: u16 = 8;
const NIGHT_SMOKED: u16 = 9;

///  the define pass.
pub(super) fn tut_quiet_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_quiet::TutQuietVars, String> {
    let walk = crate::director::manifest_volume(m, "tq_walk");
    let beam = crate::director::manifest_volume(m, "tq_beam");
    let past = crate::director::manifest_volume(m, "tq_past");
    crate::director::manifest_program(m, "tutorial", &mut |p| {
        crate::director::program_on_signal(p, "lesson:lane", "lane");
        crate::director::program_beat(p, "lane", &crate::scene_level::LevelScriptFn::TutQuiet, None)?;
        crate::director::program_on_start(p, "night");
        crate::director::program_beat(p, "night", &crate::scene_level::LevelScriptFn::TutQuiet, None)?;
        crate::director::program_on_signal(p, "lesson:store", "store");
        crate::director::program_beat(p, "store", &crate::scene_level::LevelScriptFn::TutQuiet, None)?;
        Ok(())
    })?;
    Ok(TutQuietVars { walk: Some(walk), beam: Some(beam), past: Some(past) })
}

/// (tut-quiet.script.ts:34 beat 'lane') a voice behind the wall the player creeps along.
pub(super) fn tut_quiet_lane_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_quiet::TutQuietLaneFrame) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, 1.2)?))
        }
        1 => {
            let Some(a) = super::script_tut_pair::calm(g, r, "Novak") else { return Ok(None) };
            crate::director::director_beat_say(g, r, &a, "cold tonight", None)?;
            f.at = 2;
            Ok(Some(crate::director::director_beat_wait(g, r, 3.6)?))
        }
        2 => {
            if super::script_tut_pair::calm(g, r, "Novak").is_none() || super::script_tut_pair::calm(g, r, "Kowalski").is_none() { return Ok(None); }
            crate::director::director_beat_radio(g, "Kowalski", "anything out back?");
            f.at = 3;
            Ok(Some(crate::director::director_beat_wait(g, r, 2.4)?))
        }
        3 => {
            if let Some(a) = super::script_tut_pair::calm(g, r, "Novak") { crate::director::director_beat_say(g, r, &a, "nothing. quiet", None)?; }
            Ok(None)
        }
        at => Err(format!("tut-quiet lane: no resume point {at}")),
    }
}

/// he is the player's, grabbed or down
fn taken(g: &Game, r: DirectorRunId, u: &crate::director::UnitRef) -> bool {
    !crate::director::director_query_alive(g, r, u) || crate::director::director_query_signalled(g, r, "lesson:held")
}

fn gone() -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id: TEST_TAKEN })
}

/// he has come round the long wall's end
fn in_strip(g: &Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<bool, String> {
    Ok(crate::director::director_query_pos_unit(g, r, u)?[2] < 18.5)
}

fn met() -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id: TEST_MET })
}

fn lit() -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id: TEST_LIT })
}

/// q.inside(v, 'sam') of a volume the define pass named
fn sam_inside(g: &Game, seat: &crate::seats::SeatRef, r: DirectorRunId, v: &Option<crate::director::VolumeRef>) -> bool {
    let id = &v.as_ref().expect("tut-quiet: the define pass names its volumes").id;
    crate::director::director_inside_volume(g, seat, r, id, &crate::director::DirectorInsideVolumeWho::Sam)
}

/// what turns him from his smoke (tut-quiet.script.ts:80 heard, :82 turns)
fn turns(g: &Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Vec<crate::director::Cond> {
    let heard = crate::director::EventFilter::Heard { by: u.clone(), run_level: RUN_LEVEL, run_within: RUN_WITHIN, walk_level: WALK_LEVEL, walk_within: WALK_WITHIN };
    vec![
        crate::director::director_beat_until(crate::director::CondTest::Script { id: TEST_PAST }),
        gone(),
        crate::director::director_beat_on(g, r, crate::director::EventViewKind::Step, None, Some(heard)),
        crate::director::director_beat_on(g, r, crate::director::EventViewKind::Whistle, None, None),
    ]
}

/// (beat 'night') the man who comes to the whistle: looks through the gap, finds nothing, has a smoke.
pub(super) fn tut_quiet_night_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_quiet::TutQuietNightFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{director_beat_any as any, director_beat_wait as wait_for};
    use crate::game::GuardState::{Alert, Patrol};
    loop {
        match f.at {
            0 => {
                let Some(u) = crate::director::director_beat_unit(g, "Novak") else { return Ok(None) };
                f.u = Some(u);
                f.at_2 = None;
                f.at = NIGHT_LOOP;
            }
            NIGHT_LOOP => {
                if f.at_2.is_some() {
                    f.at = NIGHT_ROUND;
                    continue;
                }
                let whistle = crate::director::director_beat_on(g, r, crate::director::EventViewKind::Whistle, None, Some(crate::director::EventFilter::XBelow { x: CROSS_WALL_X }));
                f.at = 1;
                return Ok(Some(any(vec![whistle, gone()])));
            }
            1 => {
                f.w = Some(got.clone());
                if got.index != 0.0 || got.event.is_none() { return Ok(None); }
                let wait = wait_for(g, r, 0.9)?;
                f.at = 2;
                return Ok(Some(any(vec![wait, gone()])));
            }
            2 => {
                let u = night_man(f);
                if taken(g, r, &u) { return Ok(None); }
                if crate::director::director_query_state(g, r, &u) != Some(crate::game::GuardState::Suspicious) {
                    f.at = NIGHT_LOOP;
                    continue;
                }
                let w = f.w.as_ref().and_then(|w| w.event.as_ref()).expect("tut-quiet night: w holds its event");
                f.at_2 = Some([w.pos[0], 0.1, w.pos[2]]);
                f.at = NIGHT_ROUND;
            }
            NIGHT_ROUND => {
                let u = night_man(f);
                go_round_to_look(g, r, b, &u, night_spot(f))?;
                let at_gap = crate::director::director_beat_until(crate::director::CondTest::Script { id: TEST_AT_GAP });
                let (seen, lost) = (met(), gone());
                let wait = wait_for(g, r, 40.0)?;
                f.at = 3;
                return Ok(Some(any(vec![at_gap, seen, lost, wait])));
            }
            3 => {
                f.r = Some(got.clone());
                let u = night_man(f);
                if lost_or_met(g, r, b, &u, got.index)? { return Ok(None); }
                if got.index == 3.0 {
                    crate::director::director_beat_release(g, r, b, &u, Some(Patrol))?;
                    f.at_2 = None;
                    f.at = NIGHT_LOOP;
                    continue;
                }
                crate::director::director_beat_order(g, r, b, &u, &crate::director::OrderPatch2 { aim_at: Some(Some(crate::director::Place::List(night_spot(f)))), ..crate::director::OrderPatch2::default() })?;
                let wait = wait_for(g, r, LOOK_SECS)?;
                let (light, lost) = (lit(), gone());
                f.at = 4;
                return Ok(Some(any(vec![wait, light, lost])));
            }
            4 => {
                f.r = Some(got.clone());
                let u = night_man(f);
                if lost_or_met(g, r, b, &u, got.index)? { return Ok(None); }
                crate::director::director_beat_say(g, r, &u, "nothing. wind", None)?;
                go_for_a_smoke(g, r, b, &u)?;
                let mut of = vec![crate::director::director_beat_until(crate::director::CondTest::Script { id: TEST_AT_CHAIR })];
                of.extend(turns(g, r, &u));
                of.push(wait_for(g, r, SMOKE_SECS)?);
                f.at = 5;
                return Ok(Some(any(of)));
            }
            5 => {
                f.r = Some(got.clone());
                if got.index != 0.0 {
                    f.at = NIGHT_SMOKED;
                    continue;
                }
                let u = night_man(f);
                crate::director::director_beat_order(g, r, b, &u, &crate::director::OrderPatch2 { light: Some(Some(false)), ..crate::director::OrderPatch2::default() })?;
                let mut of = vec![wait_for(g, r, SMOKE_SECS)?];
                of.extend(turns(g, r, &u));
                f.at = 6;
                return Ok(Some(any(of)));
            }
            6 => {
                f.r = Some(got.clone());
                f.at = NIGHT_SMOKED;
            }
            NIGHT_SMOKED => {
                let u = night_man(f);
                let (index, whistled) = {
                    let last = f.r.as_ref().expect("tut-quiet night: r is set before it is read");
                    (last.index, last.event.as_ref().map(|e| e.pos))
                };
                if index == 2.0 { return Ok(None); }
                if index == 1.0 || index == 3.0 {
                    crate::director::director_beat_release(g, r, b, &u, Some(Alert))?;
                    return Ok(None);
                }
                if index == 4.0 {
                    if let Some(pos) = whistled {
                        f.at_2 = Some([pos[0], 0.1, pos[2]]);
                        f.at = NIGHT_LOOP;
                        continue;
                    }
                }
                crate::director::director_beat_release(g, r, b, &u, Some(Patrol))?;
                f.at_2 = None;
                f.at = NIGHT_LOOP;
            }
            at => return Err(format!("tut-quiet night: no resume point {at}")),
        }
    }
}

/// The man of the beat.
fn night_man(f: &crate::script_tut_quiet::TutQuietNightFrame) -> crate::director::UnitRef {
    f.u.clone().expect("tut-quiet night: u is set at its start")
}

/// Where he heard the whistle.
fn night_spot(f: &crate::script_tut_quiet::TutQuietNightFrame) -> V3 {
    f.at_2.expect("tut-quiet night: at is set before the order")
}

/// The beat ends if the player has him (third of the conditions), or has been met (second): then he is released to fight.
fn lost_or_met(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, index: f64) -> Result<bool, String> {
    if index == 2.0 { return Ok(true); }
    if index == 1.0 {
        crate::director::director_beat_release(g, r, b, u, Some(crate::game::GuardState::Alert))?;
        return Ok(true);
    }
    Ok(false)
}

/// He walks to the gap with his flashlight up and on, facing where he heard it.
fn go_round_to_look(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, at: V3) -> Result<(), String> {
    crate::director::director_beat_order(g, r, b, u, &crate::director::OrderPatch2 {
        goal: Some(Some(crate::director::Place::List(LOOK))),
        face: Some(Some(crate::director::util::yaw_to(LOOK, at))),
        aim_at: Some(None),
        upper: Some(Some(crate::character::UpperMode::Torch)),
        light: Some(Some(true)),
        smoke: Some(Some(false)),
        ..crate::director::OrderPatch2::default()
    })
}

/// He walks to his chair to smoke, light still on, and turns his back on the lane.
fn go_for_a_smoke(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef) -> Result<(), String> {
    crate::director::director_beat_order(g, r, b, u, &crate::director::OrderPatch2 {
        goal: Some(Some(crate::director::Place::List(SMOKE))),
        face: Some(Some(m::PI / 2.0)),
        aim_at: Some(None),
        upper: Some(Some(crate::character::UpperMode::None)),
        light: Some(Some(true)),
        smoke: Some(Some(true)),
        ..crate::director::OrderPatch2::default()
    })
}

///  night's c.until closures, by the TEST_ ids.
pub(super) fn tut_quiet_night_test(g: &crate::game::Game, r: DirectorRunId, f: &crate::script_tut_quiet::TutQuietNightFrame, id: u32) -> Result<bool, String> {
    let u = f.u.clone().expect("tut-quiet night: u is set at its start");
    let vars = tut_quiet_vars(g, r).clone();
    match id {
        TEST_TAKEN => Ok(taken(g, r, &u)),
        TEST_MET => Ok(in_strip(g, r, &u)? && sam_inside(g, &crate::seats::here(g), r, &vars.walk)),
        TEST_LIT => Ok(sam_inside(g, &crate::seats::here(g), r, &vars.walk) || sam_inside(g, &crate::seats::here(g), r, &vars.beam)),
        TEST_AT_GAP | TEST_AT_CHAIR => crate::director::director_query_arrived(g, r, &u),
        TEST_PAST => Ok(sam_inside(g, &crate::seats::here(g), r, &vars.past)),
        _ => Err(format!("tut-quiet night: no test {id}")),
    }
}

/// (tut-quiet.script.ts:96 beat 'store') by ear, from the dark: two men in the lit room ahead.
pub(super) fn tut_quiet_store_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_quiet::TutQuietStoreFrame) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, 1.4)?))
        }
        1 => {
            let Some(a) = super::script_tut_pair::calm(g, r, "Kowalski") else { return Ok(None) };
            crate::director::director_beat_say(g, r, &a, "Reyes. how long?", None)?;
            f.at = 2;
            Ok(Some(crate::director::director_beat_wait(g, r, 2.4)?))
        }
        2 => {
            if let Some(a) = super::script_tut_pair::calm(g, r, "Reyes") {
                if super::script_tut_pair::calm(g, r, "Kowalski").is_some() { crate::director::director_beat_say(g, r, &a, "still counting", None)?; }
            }
            Ok(None)
        }
        at => Err(format!("tut-quiet store: no resume point {at}")),
    }
}
