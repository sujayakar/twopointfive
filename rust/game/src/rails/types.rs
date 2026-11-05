//! The types of `rails`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Act, src/game/rails.ts:128)
#[derive(Clone, Debug)]
pub struct Act {
    pub keys: Option<Vec<String>>,
    pub buttons: Option<f64>,
    pub wheel: Option<bool>,
}
impl Act {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Act {
            keys: None,
            buttons: None,
            wheel: None,
        }
    }
}
impl Default for Act { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Rails, src/game/rails.ts:215)
#[derive(Clone, Debug)]
pub struct Rails {
    pub run: Option<u32>,
    pub answered: f64,
    pub back: bool,
    pub orders_seen: f64,
    pub sky: crate::rails::RailsSky,
    pub spared: JsMap<crate::Num, f64>,
    pub tap_t: f64,
    pub moved: bool,
    pub way_nv: f64,
    pub gave_step: f64,
    pub gave: f64,
    pub gave_t: f64,
    pub gave0: f64,
    pub act_t: f64,
    pub log: Vec<String>,
    pub running: bool,
    pub lookout: Option<crate::fogofwar::FogEye>,
    pub step_look: Option<crate::rails::RailsStepLook>,
    pub fresh: f64,
}
impl Rails {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Rails {
            run: None,
            answered: -1.0,
            back: false,
            orders_seen: 0.0,
            sky: crate::rails::RailsSky { off: [0.0, 0.0, 0.0], mid: None, mid_t: -1.0, parked: false, was: None, view: None, again: false },
            spared: JsMap::new(),
            tap_t: -1000000000.0,
            moved: false,
            way_nv: -1.0,
            gave_step: -1.0,
            gave: 0.0,
            gave_t: -1000000000.0,
            gave0: -1000000000.0,
            act_t: -1000000000.0,
            log: Vec::new(),
            running: false,
            lookout: None,
            step_look: None,
            fresh: 0.0,
        }
    }
}
impl Default for Rails { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at quiet Rails.run, quiet Rails.run)
#[derive(Clone, Copy, Debug)]
pub struct RailsRun {
}
impl RailsRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsRun {
        }
    }
}
impl Default for RailsRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:225 Rails.sky, src/game/rails.ts:225)
#[derive(Clone, Copy, Debug)]
pub struct RailsSky {
    pub off: V3,
    pub mid: Option<V3>,
    pub mid_t: f64,
    pub parked: bool,
    pub was: Option<crate::rails::RailsSkyWas>,
    pub view: Option<crate::rails::RailsSkyView>,
    pub again: bool,
}
impl RailsSky {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsSky {
            off: [0.0; 3],
            mid: None,
            mid_t: 0.0,
            parked: false,
            was: None,
            view: None,
            again: false,
        }
    }
}
impl Default for RailsSky { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:225 RailsSky.was, src/game/rails.ts:225)
#[derive(Clone, Copy, Debug)]
pub struct RailsSkyWas {
    pub invisible: bool,
}
impl RailsSkyWas {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsSkyWas {
            invisible: false,
        }
    }
}
impl Default for RailsSkyWas { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:225 RailsSky.view, src/game/rails.ts:225)
#[derive(Clone, Copy, Debug)]
pub struct RailsSkyView {
    pub yaw: f64,
    pub pitch: f64,
    pub dist: f64,
}
impl RailsSkyView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsSkyView {
            yaw: 0.0,
            pitch: 0.0,
            dist: 0.0,
        }
    }
}
impl Default for RailsSkyView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:411 Rails.stepLook, src/game/rails.ts:411)
#[derive(Clone, Copy, Debug)]
pub struct RailsStepLook {
    pub i: f64,
    pub eye: crate::fogofwar::FogEye,
}
impl RailsStepLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsStepLook {
            i: 0.0,
            eye: crate::fogofwar::FogEye::zeroed(),
        }
    }
}
impl Default for RailsStepLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:166 said(A), src/game/rails.ts:166)
#[derive(Clone, Debug)]
pub struct SaidA {
    pub line: Option<String>,
    pub word: Option<String>,
}
impl SaidA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SaidA {
            line: None,
            word: None,
        }
    }
}
impl Default for SaidA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:169 lookoutEye(e), src/game/rails.ts:169)
#[derive(Clone, Copy, Debug)]
pub struct LookoutEyeE {
    pub at: V3,
    pub yaw: f64,
    pub fov: Option<f64>,
}
impl LookoutEyeE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LookoutEyeE {
            at: [0.0; 3],
            yaw: 0.0,
            fov: None,
        }
    }
}
impl Default for LookoutEyeE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:256 lesson ->, src/game/rails.ts:256)
#[derive(Clone, Debug)]
pub struct RailsLesson {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}
impl RailsLesson {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsLesson {
            l: crate::scene_scenarios::LessonData::zeroed(),
            i: 0.0,
        }
    }
}
impl Default for RailsLesson { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:286 card ->, src/game/rails.ts:286)
#[derive(Clone, Debug)]
pub struct RailsCard {
    pub title: Option<String>,
    pub text: String,
}
impl RailsCard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsCard {
            title: None,
            text: String::new(),
        }
    }
}
impl Default for RailsCard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:325 onWay(S), src/game/rails.ts:325)
#[derive(Clone, Debug)]
pub struct RailsOnWayS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}
impl RailsOnWayS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsOnWayS {
            l: crate::scene_scenarios::LessonData::zeroed(),
            i: 0.0,
        }
    }
}
impl Default for RailsOnWayS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:387 markUnder(canvas), src/game/rails.ts:387)
#[derive(Clone, Copy, Debug)]
pub struct RailsMarkUnderCanvas {
    pub client_width: f64,
    pub client_height: f64,
}
impl RailsMarkUnderCanvas {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsMarkUnderCanvas {
            client_width: 0.0,
            client_height: 0.0,
        }
    }
}
impl Default for RailsMarkUnderCanvas { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:394 orderRows(rows), src/game/rails.ts:394)
#[derive(Clone, Debug)]
pub struct RailsOrderRowsRowsItem {
    pub id: String,
}
impl RailsOrderRowsRowsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsOrderRowsRowsItem {
            id: String::new(),
        }
    }
}
impl Default for RailsOrderRowsRowsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:448 answer(S), src/game/rails.ts:448)
#[derive(Clone, Debug)]
pub struct RailsAnswerS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}
impl RailsAnswerS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsAnswerS {
            l: crate::scene_scenarios::LessonData::zeroed(),
            i: 0.0,
        }
    }
}
impl Default for RailsAnswerS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:476 prompt ->, src/game/rails.ts:476)
#[derive(Clone, Debug)]
pub struct RailsPrompt {
    pub line: String,
    pub n: f64,
    pub of: f64,
    pub note: Option<String>,
    pub watch: Option<bool>,
}
impl RailsPrompt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsPrompt {
            line: String::new(),
            n: 0.0,
            of: 0.0,
            note: None,
            watch: None,
        }
    }
}
impl Default for RailsPrompt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:519 lineMen ->, src/game/rails.ts:519)
#[derive(Clone, Copy, Debug)]
pub struct RailsLineMenItem {
    pub pos: V3,
    pub bones: crate::rails::RailsLineMenItemBones,
}
impl RailsLineMenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsLineMenItem {
            pos: [0.0; 3],
            bones: crate::rails::RailsLineMenItemBones::zeroed(),
        }
    }
}
impl Default for RailsLineMenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:519 RailsLineMenItem.bones, src/game/rails.ts:519)
#[derive(Clone, Copy, Debug)]
pub struct RailsLineMenItemBones {
    pub head: Option<V3>,
}
impl RailsLineMenItemBones {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsLineMenItemBones {
            head: None,
        }
    }
}
impl Default for RailsLineMenItemBones { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:537 waiting(S), src/game/rails.ts:537)
#[derive(Clone, Debug)]
pub struct RailsWaitingS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}
impl RailsWaitingS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsWaitingS {
            l: crate::scene_scenarios::LessonData::zeroed(),
            i: 0.0,
        }
    }
}
impl Default for RailsWaitingS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rails.ts:544 holdWaiting(S), src/game/rails.ts:544)
#[derive(Clone, Debug)]
pub struct RailsHoldWaitingS {
    pub l: crate::scene_scenarios::LessonData,
    pub i: f64,
}
impl RailsHoldWaitingS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsHoldWaitingS {
            l: crate::scene_scenarios::LessonData::zeroed(),
            i: 0.0,
        }
    }
}
impl Default for RailsHoldWaitingS { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct RailsStats {
    pub dropped: i32,
    pub cues: i32,
    pub starts: i32,
    pub kept: i32,
    pub put_back: i32,
    pub spared: i32,
}

/// (src/game/rails.ts:124 BREAK)
pub const BREAK: bool = false;

/// (src/game/rails.ts:126 WAITS_BREAK)
pub const WAITS_BREAK: bool = false;

/// (src/game/rails.ts:129 ACTS: a table nothing writes)
pub fn acts() -> JsMap<String, crate::rails::Act> { { let mut t = JsMap::new(); t.set(String::from("move"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyW"), String::from("KeyA"), String::from("KeyS"), String::from("KeyD")]), buttons: None, wheel: None }); t.set(String::from("pan"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyW"), String::from("KeyA"), String::from("KeyS"), String::from("KeyD")]), buttons: None, wheel: None }); t.set(String::from("sprint"), crate::rails::Act { keys: Some(alloc::vec![String::from("ShiftLeft"), String::from("ShiftRight")]), buttons: None, wheel: None }); t.set(String::from("crouch"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyC")]), buttons: None, wheel: None }); t.set(String::from("wall"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyQ")]), buttons: None, wheel: None }); t.set(String::from("pace"), crate::rails::Act { keys: None, buttons: None, wheel: Some(true) }); t.set(String::from("aim"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("fire"), crate::rails::Act { keys: None, buttons: Some(1.0), wheel: None }); t.set(String::from("alt"), crate::rails::Act { keys: None, buttons: Some(4.0), wheel: None }); t.set(String::from("reload"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyR")]), buttons: None, wheel: None }); t.set(String::from("mode"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyZ")]), buttons: None, wheel: None }); t.set(String::from("slot:pistol"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit1")]), buttons: None, wheel: None }); t.set(String::from("slot:rifle"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit2")]), buttons: None, wheel: None }); t.set(String::from("slot:smoke"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit3")]), buttons: None, wheel: None }); t.set(String::from("slot:flash"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit4")]), buttons: None, wheel: None }); t.set(String::from("slot:frag"), crate::rails::Act { keys: Some(alloc::vec![String::from("Digit5")]), buttons: None, wheel: None }); t.set(String::from("stalk"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("hands"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyE"), String::from("Digit0")]), buttons: None, wheel: None }); t.set(String::from("use"), crate::rails::Act { keys: Some(alloc::vec![String::from("Space"), String::from("ArrowUp"), String::from("ArrowDown")]), buttons: None, wheel: None }); t.set(String::from("whistle"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyV")]), buttons: None, wheel: None }); t.set(String::from("light"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyL")]), buttons: None, wheel: None }); t.set(String::from("nvg"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyN")]), buttons: None, wheel: None }); t.set(String::from("order"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyB")]), buttons: None, wheel: None }); t.set(String::from("ring"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("menu"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyX")]), buttons: Some(4.0), wheel: None }); t.set(String::from("who"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyT")]), buttons: None, wheel: None }); t.set(String::from("word"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyF")]), buttons: None, wheel: None }); t.set(String::from("tools"), crate::rails::Act { keys: Some(alloc::vec![String::from("KeyG"), String::from("KeyH")]), buttons: None, wheel: None }); t.set(String::from("goal"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("twice"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("camera"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("zoom"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("blow"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t.set(String::from("load"), crate::rails::Act { keys: None, buttons: None, wheel: None }); t } }

/// (src/game/rails.ts:160 LOOKOUT_ID)
pub const LOOKOUT_ID: f64 = 199.0;

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
