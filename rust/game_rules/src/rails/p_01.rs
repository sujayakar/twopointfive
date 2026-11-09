use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A step's `after` line as its order stands: given (1), or given and waiting on his word (2).
pub fn said(a: &crate::scene_scenarios::LessonStepAfter, gave: f64) -> Option<String> {
    if gave == 2.0 { a.word.clone().or_else(|| a.line.clone()) } else { a.line.clone() }
}

/// A lookout as one of the fog's eyes.
pub fn lookout_eye(id_2: f64, e: crate::rails::LookoutEyeE) -> crate::fogofwar::FogEye {
    crate::fogofwar::FogEye {
        id: id_2,
        pos: e.at,
        look: [m::sin(e.yaw), 0.0, m::cos(e.yaw)],
        at: e.at,
        fov: Some(e.fov.unwrap_or(120.0)),
        near: Some(0.0),
        wand: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_scenarios::{LessonData, LessonStep, LessonStepAfter, RailsData, RailsDataWaits};

    #[test]
    fn said_takes_the_word_only_at_two() {
        let both = LessonStepAfter { line: Some(String::from("line")), on: None, word: Some(String::from("word")) };
        let no_word = LessonStepAfter { line: Some(String::from("line")), on: None, word: None };
        assert_eq!(said(&both, 1.0).as_deref(), Some("line"));
        assert_eq!(said(&both, 2.0).as_deref(), Some("word"));
        assert_eq!(said(&no_word, 2.0).as_deref(), Some("line"));
    }

    #[test]
    fn lookout_eye_defaults() {
        let e = lookout_eye(STEP_EYE_ID, LookoutEyeE { at: [1.0, 2.0, 3.0], yaw: 0.0, fov: None });
        assert_eq!(e.id, 198.0);
        assert_eq!(e.pos, [1.0, 2.0, 3.0]);
        assert_eq!(e.at, e.pos);
        assert_eq!(e.look, [0.0, 0.0, 1.0]);
        assert_eq!(e.fov, Some(120.0));
        assert_eq!(e.near, Some(0.0));
        assert_eq!(lookout_eye(STEP_EYE_ID, LookoutEyeE { at: [0.0; 3], yaw: 0.0, fov: Some(90.0) }).fov, Some(90.0));
    }

    #[test]
    fn a_card_size_reads_as_the_typescript_wrote_it() {
        assert_eq!(alloc::format!("{CARD_CHARS}"), "260");
    }

}
