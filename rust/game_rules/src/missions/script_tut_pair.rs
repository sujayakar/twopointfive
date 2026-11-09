#![allow(unused_imports, unused_variables)]
//! levels/tut-pair.script.ts translated: its define pass and a step function for each beat.

pub use ::game_types::script_tut_pair::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use alloc::string::String;

/// The man by name, if he is still at his work.   (tut-pair.script.ts:13 calm)
fn calm(g: &mut Game, r: DirectorRunId, callsign: &str) -> Option<crate::director::UnitRef> {
    let u = crate::director::director_beat_unit(g, r, callsign)?;
    if crate::director::director_query_state(g, r, &u) == Some(crate::game::GuardState::Patrol) { Some(u) } else { None }
}

/// The define pass: the program 'tutorial', its three cues and their beats, in the script's order.   (tut-pair.script.ts:10 script)
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

/// The beat 'gate': the man at the pickup is one of several.   (tut-pair.script.ts:18-24)
pub(super) fn tut_pair_gate_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_pair::TutPairGateFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{director_beat_radio, director_beat_say, director_beat_wait};
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(director_beat_wait(g, r, 1.5, None)?))
        }
        1 => {
            if calm(g, r, "Kowalski").is_none() || calm(g, r, "Novak").is_none() { return Ok(None); }
            director_beat_radio(g, b, "Novak", "Kowalski. tea is up");
            f.at = 2;
            Ok(Some(director_beat_wait(g, r, 2.6, None)?))
        }
        _ => {
            if let Some(a) = calm(g, r, "Kowalski") { director_beat_say(g, r, b, &a, "in a minute", None)?; }
            Ok(None)
        }
    }
}

/// The beat 'park': the two at the lantern talk to each other.   (tut-pair.script.ts:27-33)
pub(super) fn tut_pair_park_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_pair::TutPairParkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{director_beat_say, director_beat_wait};
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(director_beat_wait(g, r, 1.2, None)?))
        }
        1 => {
            let Some(a) = calm(g, r, "Novak") else { return Ok(None); };
            if calm(g, r, "Reyes").is_none() { return Ok(None); }
            director_beat_say(g, r, b, &a, "anything out there?", None)?;
            f.at = 2;
            Ok(Some(director_beat_wait(g, r, 2.4, None)?))
        }
        _ => {
            if let Some(b2) = calm(g, r, "Reyes") {
                if calm(g, r, "Novak").is_some() { director_beat_say(g, r, b, &b2, "nothing. cold", None)?; }
            }
            Ok(None)
        }
    }
}

/// The beat 'yard': the last man is alone and has noticed nothing.   (tut-pair.script.ts:36-39)
pub(super) fn tut_pair_yard_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_pair::TutPairYardFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    use crate::director::{director_beat_say, director_beat_wait};
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(director_beat_wait(g, r, 1.4, None)?))
        }
        _ => {
            if let Some(a) = calm(g, r, "Okafor") { director_beat_say(g, r, b, &a, "long night", None)?; }
            Ok(None)
        }
    }
}
