//! The types of `lesson`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a step's done-test, the one enum of it: one arm a row of LESSON_TESTS, read whole from a level file's `{ is: '<kind>', … }` (game_start `records.rs`), each word typed: a word no type holds refuses the level
#[derive(Clone, Debug, ToBytes)]
pub enum LessonTest {
    In { area: crate::scene_scenarios::Rect },
    Near { at: V3, r: Option<f64> },
    Flag { flag: crate::lesson::LessonFlag, value: Option<bool> },
    Slot { slot: crate::game::SlotKind, drawn: Option<bool> },
    Pace { below: Option<f64>, above: Option<f64> },
    Meter { which: crate::lesson::LessonMeter, below: Option<f64>, above: Option<f64> },
    Stage { stage: crate::game::MissionStage },
    Door { door: String, state: crate::lesson::LessonDoorState },
    Target { name: String, on: bool },
    Men { down: Option<f64> },
    Did { what: String, on: Option<String>, n: Option<f64> },
    Downed { how: crate::lesson::LessonHow, n: Option<f64> },
    Body { area: crate::scene_scenarios::Rect },
    Calm { route: Option<String> },
    Loaded,
    Reload,
    Launcher { on: Option<crate::weapons::RifleAttachment>, fired: bool },
    Burst { n: Option<f64> },
    Threw { kind: Option<crate::lesson::LessonThrown>, quick: bool, area: Option<crate::scene_scenarios::Rect> },
    OrderTo { who: crate::orders::Who },
    StackOn { door: String, who: Option<crate::orders::SquadOrdersMenSel>, set: bool },
    PairsOn { doors: Option<f64> },
    Word { doors: Option<f64> },
    Taken { room: String },
    Following,
    Mate { area: Option<crate::scene_scenarios::Rect>, at: Option<V3>, r: Option<f64>, by: Option<f64>, away: Option<f64> },
    Pair { what: crate::lesson::LessonPairWhat, n: Option<f64> },
    Cue { name: String },
    View { turned: Option<f64>, zoomed: Option<f64> },
    Squad { area: crate::scene_scenarios::Rect, n: Option<f64> },
    Stacks { doors: Vec<String>, n: Option<f64>, set: bool },
    Watching { at: V3, r: Option<f64>, n: Option<f64> },
    All { of: Vec<crate::lesson::LessonTest> },
    Any { of: Vec<crate::lesson::LessonTest> },
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Lesson {
    pub i: f64,
    pub done_at: JsMap<String, f64>,
    pub log: Vec<String>,
    pub seen: Option<crate::lesson::StepSeen>,
    pub run: Option<u32>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StepSeen {
    pub t0: f64,
    pub n: JsMap<String, f64>,
    pub chron: f64,
    pub heard: JsSet<u32>,
    pub held: f64,
    pub done_t: Option<f64>,
    pub idle: Option<bool>,
    pub last_order: Option<u32>,
    pub last_pair: Option<u32>,
    pub chron0: f64,
    pub held_at: f64,
    pub words: JsSet<DoorId>,
    pub yaw0: f64,
    pub dist0: f64,
    pub yaw_last: f64,
    pub turned: f64,
    pub done_i: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TestKind {
}

#[derive(Clone, Debug, Default)]
pub struct LessonObjective {
    pub text: String,
    pub pos: Option<V3>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LessonStats {
    pub started: i32,
    pub steps: i32,
    pub finished: i32,
}

pub const WHEEL_RING: &str = "Hold Space, then 2 kicks it";

/// what a `door` test asks of a door
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToBytes)]
pub enum LessonDoorState { Open, Shut, Unlocked, Broken, Charged }
impl LessonDoorState {
    pub const fn as_str(self) -> &'static str { match self { LessonDoorState::Open => "open", LessonDoorState::Shut => "shut", LessonDoorState::Unlocked => "unlocked", LessonDoorState::Broken => "broken", LessonDoorState::Charged => "charged" } }
}
/// which of the player's meters a `meter` test reads
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToBytes)]
pub enum LessonMeter { Noise, Light }
impl LessonMeter {
    pub const fn as_str(self) -> &'static str { match self { LessonMeter::Noise => "noise", LessonMeter::Light => "light" } }
}
/// how a `downed` test asks a man to be put down
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToBytes)]
pub enum LessonHow { Takedown, Choke, Whip, Ring, Knife }
impl LessonHow {
    pub const fn as_str(self) -> &'static str { match self { LessonHow::Takedown => "takedown", LessonHow::Choke => "choke", LessonHow::Whip => "whip", LessonHow::Ring => "ring", LessonHow::Knife => "knife" } }
}
/// the `kind` of a thrown row of the chronicle, as a `threw` test names it
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToBytes)]
pub enum LessonThrown { Smoke, Flash, Frag, He40, Cam }
impl LessonThrown {
    pub const fn as_str(self) -> &'static str { match self { LessonThrown::Smoke => "smoke", LessonThrown::Flash => "flash", LessonThrown::Frag => "frag", LessonThrown::He40 => "he40", LessonThrown::Cam => "cam" } }
}
/// the word of the pair's log a `pair` test waits for
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToBytes)]
pub enum LessonPairWhat { Mark, Now, Set, Go, GoF, GoHands, Down }
impl LessonPairWhat {
    pub const fn as_str(self) -> &'static str { match self { LessonPairWhat::Mark => "mark", LessonPairWhat::Now => "now", LessonPairWhat::Set => "set", LessonPairWhat::Go => "go", LessonPairWhat::GoF => "go:F", LessonPairWhat::GoHands => "go:hands", LessonPairWhat::Down => "down" } }
}
/// a flag a `flag` test can read off the player and the team's posture
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToBytes)]
pub enum LessonFlag { Crouch, Sprint, Wall, Peek, Lean, Nv, Light, Holstered, Gun, RingsOut, ShellsOut, Auto, Setup, SetupShut, Dragging, Dark, ChargeSet, ChargeClear, WallBlown, Holding, Picking, Coding, Menu, OwnRing, Silent }

pub fn lesson_tests() -> JsMap<String, crate::lesson::TestKind> { { let mut t = JsMap::new(); t.set(String::from("in"), crate::lesson::TestKind {  }); t.set(String::from("near"), crate::lesson::TestKind {  }); t.set(String::from("flag"), crate::lesson::TestKind {  }); t.set(String::from("slot"), crate::lesson::TestKind {  }); t.set(String::from("pace"), crate::lesson::TestKind {  }); t.set(String::from("meter"), crate::lesson::TestKind {  }); t.set(String::from("stage"), crate::lesson::TestKind {  }); t.set(String::from("door"), crate::lesson::TestKind {  }); t.set(String::from("target"), crate::lesson::TestKind {  }); t.set(String::from("men"), crate::lesson::TestKind {  }); t.set(String::from("did"), crate::lesson::TestKind {  }); t.set(String::from("downed"), crate::lesson::TestKind {  }); t.set(String::from("body"), crate::lesson::TestKind {  }); t.set(String::from("calm"), crate::lesson::TestKind {  }); t.set(String::from("loaded"), crate::lesson::TestKind {  }); t.set(String::from("reload"), crate::lesson::TestKind {  }); t.set(String::from("launcher"), crate::lesson::TestKind {  }); t.set(String::from("burst"), crate::lesson::TestKind {  }); t.set(String::from("threw"), crate::lesson::TestKind {  }); t.set(String::from("orderTo"), crate::lesson::TestKind {  }); t.set(String::from("stackOn"), crate::lesson::TestKind {  }); t.set(String::from("pairsOn"), crate::lesson::TestKind {  }); t.set(String::from("word"), crate::lesson::TestKind {  }); t.set(String::from("taken"), crate::lesson::TestKind {  }); t.set(String::from("following"), crate::lesson::TestKind {  }); t.set(String::from("mate"), crate::lesson::TestKind {  }); t.set(String::from("pair"), crate::lesson::TestKind {  }); t.set(String::from("cue"), crate::lesson::TestKind {  }); t.set(String::from("view"), crate::lesson::TestKind {  }); t.set(String::from("squad"), crate::lesson::TestKind {  }); t.set(String::from("stacks"), crate::lesson::TestKind {  }); t.set(String::from("watching"), crate::lesson::TestKind {  }); t.set(String::from("all"), crate::lesson::TestKind {  }); t.set(String::from("any"), crate::lesson::TestKind {  }); t } }

pub fn stages() -> Vec<String> { alloc::vec![String::from("infiltrate"), String::from("drive"), String::from("exfil"), String::from("done")] }

