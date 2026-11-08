//! The types of `rails`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct Rails {
    pub run: Option<u32>,
    pub answered: f64,
    pub orders_seen: f64,
    pub spared: JsMap<crate::Num, f64>,
    pub tap_t: f64,
    pub way_nv: f64,
    pub gave_step: f64,
    pub gave: f64,
    pub gave_t: f64,
    pub gave0: f64,
    pub act_t: f64,
    pub log: Vec<String>,
    pub running: bool,
    pub step_look: Option<crate::rails::RailsStepLook>,
    pub fresh: f64,
}
impl Rails {
    /// what a new one holds
    pub fn blank() -> Self {
        Rails {
            run: None,
            answered: -1.0,
            orders_seen: 0.0,
            spared: JsMap::new(),
            tap_t: -1000000000.0,
            way_nv: -1.0,
            gave_step: -1.0,
            gave: 0.0,
            gave_t: -1000000000.0,
            gave0: -1000000000.0,
            act_t: -1000000000.0,
            log: Vec::new(),
            running: false,
            step_look: None,
            fresh: 0.0,
        }
    }
}
impl Default for Rails { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct RailsRun {
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RailsStepLook {
    pub i: f64,
    pub eye: crate::fogofwar::FogEye,
}

#[derive(Clone, Debug, Default)]
pub struct Act {
    pub keys: Option<Vec<String>>,
    pub buttons: Option<f64>,
    pub wheel: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct SaidA {
    pub line: Option<String>,
    pub word: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LookoutEyeE {
    pub at: V3,
    pub yaw: f64,
    pub fov: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct RailsLesson {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RailsCard {
    pub title: Option<String>,
    pub text: String,
}

#[derive(Clone, Debug, Default)]
pub struct RailsOnWayS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RailsOrderRowsRowsItem {
    pub id: String,
}

#[derive(Clone, Debug, Default)]
pub struct RailsAnswerS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RailsPrompt {
    pub line: String,
    pub n: f64,
    pub of: f64,
    pub note: Option<String>,
    pub watch: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RailsLineMenItem {
    pub pos: V3,
    pub bones: crate::rails::RailsLineMenItemBones,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RailsLineMenItemBones {
    pub head: Option<V3>,
}

#[derive(Clone, Debug, Default)]
pub struct RailsWaitingS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RailsHoldWaitingS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct RailsStats {
    pub dropped: i32,
    pub cues: i32,
    pub starts: i32,
    pub kept: i32,
    pub spared: i32,
}

/// (src/game/rails.ts:124 BREAK)
pub const BREAK: bool = false;

/// (src/game/rails.ts:126 WAITS_BREAK)
pub const WAITS_BREAK: bool = false;

/// (src/game/rails.ts:129 ACTS: a table nothing writes)
pub fn acts() -> JsMap<String, crate::rails::Act> { { let mut t = JsMap::new(); t.set(String::from("move"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyW"), String::from("KeyA"), String::from("KeyS"), String::from("KeyD")]), buttons: None, wheel: None }); t.set(String::from("sprint"), crate::rails::Act { keys: Some(alloc::vec![String::from("ShiftLeft"), String::from("ShiftRight")]), buttons: None, wheel: None }); t.set(String::from("crouch"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyC")]), buttons: None, wheel: None }); t.set(String::from("wall"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyQ")]), buttons: None, wheel: None }); t.set(String::from("pace"), crate::rails::Act { keys: None, buttons: None, wheel: Some(true) }); t.set(String::from("aim"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("fire"), crate::rails::Act { keys: None, buttons: Some(1.0), wheel: None }); t.set(String::from("alt"), crate::rails::Act { keys: None, buttons: Some(4.0), wheel: None }); t.set(String::from("reload"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyR")]), buttons: None, wheel: None }); t.set(String::from("mode"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyZ")]), buttons: None, wheel: None }); t.set(String::from("slot:pistol"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit1")]), buttons: None, wheel: None }); t.set(String::from("slot:rifle"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit2")]), buttons: None, wheel: None }); t.set(String::from("slot:smoke"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit3")]), buttons: None, wheel: None }); t.set(String::from("slot:flash"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit4")]), buttons: None, wheel: None }); t.set(String::from("slot:frag"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit5")]), buttons: None, wheel: None }); t.set(String::from("stalk"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("hands"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyE"), String::from("Digit0")]), buttons: None, wheel: None }); t.set(String::from("use"), crate::rails::Act { keys: Some(alloc::vec![String::from("Space"), String::from("ArrowUp"), String::from("ArrowDown")]), buttons: None, wheel: None }); t.set(String::from("whistle"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyV")]), buttons: None, wheel: None }); t.set(String::from("light"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyL")]), buttons: None, wheel: None }); t.set(String::from("nvg"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyN")]), buttons: None, wheel: None }); t.set(String::from("order"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyB")]), buttons: None, wheel: None }); t.set(String::from("ring"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("ringPart"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("menu"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyX")]), buttons: Some(4.0), wheel: None }); t.set(String::from("who"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyT")]), buttons: None, wheel: None }); t.set(String::from("word"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyF")]), buttons: None, wheel: None }); t.set(String::from("tools"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyG"), String::from("KeyH")]), buttons: None, wheel: None }); t.set(String::from("goal"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("twice"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("camera"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("zoom"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("blow"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("load"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t } }

/// (src/game/rails.ts:160 STEP_EYE_ID)
pub const STEP_EYE_ID: f64 = 198.0;

/// (src/game/rails.ts:162 NOTE_SECS)
pub const NOTE_SECS: f64 = 2.6;

/// (src/game/rails.ts:164 MENU_M)
pub const MENU_M: f64 = 6.0;

/// (src/game/rails.ts:168 CARD_CHARS)
pub const CARD_CHARS: f64 = 260.0;

/// (src/game/rails.ts:170 FREE)
pub const FREE: &str = "/^(Escape|Tab|Enter|NumpadEnter|Blur|Paused|F\\d{1,2}|BracketLeft|BracketRight)$/";

/// (src/game/rails.ts:171 DIGIT)
pub const DIGIT: &str = "/^Digit[1-5]$/";

/// (src/game/rails.ts:172 RAILS_ACTIONS: a table nothing writes)
pub fn rails_actions() -> Vec<String> { crate::rails::acts().keys() }

/// (src/game/rails.ts:179 UNDO: a table nothing writes)
pub fn undo() -> Vec<[String; 3]> { alloc::vec![[String::from("push"), String::from("pull"), String::from("a door he opens cannot be shut")], [String::from("ease"), String::from("pull"), String::from("a door he eases open cannot be shut")], [String::from("drag"), String::from("drop"), String::from("a body he drags cannot be let go")], [String::from("lightsOff"), String::from("lightsOn"), String::from("a room's lights he puts out cannot be put on")], [String::from("breakerOff"), String::from("breakerOn"), String::from("the mains he cuts cannot be put back")]] }
