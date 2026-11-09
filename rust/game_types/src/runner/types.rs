//! The types of `runner`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RunPhase {
    #[default] Low,
    Out,
    Gone,
    Mates,
    Say,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Run {
    pub phase: crate::runner::RunPhase,
    pub t: f64,
    pub from: V3,
    pub goal: Option<V3>,
    pub mate: Option<GuardId>,
    pub told: f64,
    pub unseen: f64,
    pub at: V3,
    pub stall: f64,
    pub is_out: bool,
    pub tries: f64,
    pub plan: f64,
    pub face: Option<f64>,
    pub bold: bool,
    pub bare: bool,
    pub drive: bool,
    pub drive_t: f64,
    pub script: crate::game::GuardScript,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LineKind {
    Done,
    Mates,
}
impl LineKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineKind::Done => "done",
            LineKind::Mates => "mates",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Foe {
    pub pos: V3,
    pub eye: V3,
    pub who: crate::picture::Who,
}
impl Default for Foe {
    fn default() -> Self {
        Foe {
            pos: [0.0; 3],
            eye: [0.0; 3],
            who: crate::picture::Who::Guard(GuardId(0)),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LINESValue {
    pub say: Vec<String>,
    pub he: Option<Vec<String>>,
    pub they: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Flood {
    pub out: Option<V3>,
    pub hidden: bool,
    pub corner: V3,
    pub shied: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlanOutF {
    pub out: Option<V3>,
    pub shied: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct RunnerMod {
    /// (src/game/runner.ts:101 stateOf)
    pub state_of: JsMap<GuardId, crate::runner::Run>,
    /// (src/game/runner.ts:194 rects)
    pub rects: Option<[f64; 4]>,
}
impl RunnerMod {
    pub fn blank() -> Self {
        RunnerMod {
            state_of: JsMap::new(),
            rects: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct RunnerStats {
    pub said: i32,
    pub dropped: i32,
    pub to_mates: i32,
    pub told: i32,
    pub out: i32,
    pub gone: i32,
    pub low: i32,
    pub bold: i32,
}

/// (src/game/runner.ts:108 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::runner::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("done"), crate::runner::LINESValue { say: alloc::vec![String::from("no. no, I'm done. I'm done!"), String::from("that's it — I'm done! I'm done!"), String::from("no more! I'm finished here!")], he: None, they: None }); t.set(String::from("mates"), crate::runner::LINESValue { say: alloc::vec![String::from("don't shoot — it's me! it's me!")], he: Some(alloc::vec![String::from("don't shoot, it's me! he's right behind me!"), String::from("it's me! hold your fire — he's coming!")]), they: Some(alloc::vec![String::from("don't shoot, it's me! they're right behind me!"), String::from("it's me! hold your fire — they're coming!")]) }); t } }
