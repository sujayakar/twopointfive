#![allow(unused_imports, unused_variables)]
//! levels/tut-pair.script.ts translated: its define pass and a step function for each beat.

pub use ::game_types::script_tut_pair::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use alloc::string::String;

/// The man by name, if he is still at his work.
fn calm(g: &Game, r: DirectorRunId, callsign: &str) -> Option<crate::director::UnitRef> {
    let u = crate::director::director_beat_unit(g, callsign)?;
    if crate::director::director_query_state(g, r, &u) == Some(crate::game::GuardState::Patrol) { Some(u) } else { None }
}

/// The define pass: the program 'tutorial', its three cues and their beats, in the script's order.
pub(super) fn tut_pair_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_pair::TutPairVars, String> {
    use crate::director::{manifest_program, program_beat, program_on_signal};
    let script = crate::scene_level::LevelScriptFn::TutPair;
    manifest_program(m, "tutorial", &mut |p| {
        program_on_signal(p, "lesson:gate", "gate");
        program_beat(p, "gate", &script, None)?;
        program_on_signal(p, "lesson:park", "park");
        program_beat(p, "park", &script, None)?;
        program_on_signal(p, "lesson:yard", "yard");
        program_beat(p, "yard", &script, None)?;
        Ok(())
    })?;
    Ok(crate::script_tut_pair::TutPairVars::default())
}

/// The beat 'gate': the man at the pickup is one of several.
pub(super) fn tut_pair_gate_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_pair::TutPairGateFrame) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{director_beat_radio, director_beat_say, director_beat_wait};
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(director_beat_wait(g, r, 1.5)?))
        }
        1 => {
            if calm(g, r, "Kowalski").is_none() || calm(g, r, "Novak").is_none() { return Ok(None); }
            director_beat_radio(g, "Novak", "Kowalski. tea is up");
            f.at = 2;
            Ok(Some(director_beat_wait(g, r, 2.6)?))
        }
        _ => {
            if let Some(a) = calm(g, r, "Kowalski") { director_beat_say(g, r, &a, "in a minute", None)?; }
            Ok(None)
        }
    }
}

/// The beat 'park': the two at the lantern talk to each other.
pub(super) fn tut_pair_park_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_pair::TutPairParkFrame) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{director_beat_say, director_beat_wait};
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(director_beat_wait(g, r, 1.2)?))
        }
        1 => {
            let Some(a) = calm(g, r, "Novak") else { return Ok(None); };
            if calm(g, r, "Reyes").is_none() { return Ok(None); }
            director_beat_say(g, r, &a, "anything out there?", None)?;
            f.at = 2;
            Ok(Some(director_beat_wait(g, r, 2.4)?))
        }
        _ => {
            if let Some(b2) = calm(g, r, "Reyes") {
                if calm(g, r, "Novak").is_some() { director_beat_say(g, r, &b2, "nothing. cold", None)?; }
            }
            Ok(None)
        }
    }
}

/// The beat 'yard': the last man is alone and has noticed nothing.
pub(super) fn tut_pair_yard_step(g: &mut crate::game::Game, r: DirectorRunId, f: &mut crate::script_tut_pair::TutPairYardFrame) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{director_beat_say, director_beat_wait};
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(director_beat_wait(g, r, 1.4)?))
        }
        _ => {
            if let Some(a) = calm(g, r, "Okafor") { director_beat_say(g, r, &a, "long night", None)?; }
            Ok(None)
        }
    }
}
