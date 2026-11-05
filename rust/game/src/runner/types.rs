//! The types of `runner`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RunPhase {
    Low,
    Out,
    Gone,
    Mates,
    Say,
}
impl RunPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            RunPhase::Low => "low",
            RunPhase::Out => "out",
            RunPhase::Gone => "gone",
            RunPhase::Mates => "mates",
            RunPhase::Say => "say",
        }
    }
}

/// (TypeScript: Run, src/game/runner.ts:97)
#[derive(Clone, Copy, Debug)]
pub struct Run {
    pub phase: crate::runner::RunPhase,
    pub t: f64,
    pub fled: bool,
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
impl Run {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Run {
            phase: crate::runner::RunPhase::Low,
            t: 0.0,
            fled: false,
            from: [0.0; 3],
            goal: None,
            mate: None,
            told: 0.0,
            unseen: 0.0,
            at: [0.0; 3],
            stall: 0.0,
            is_out: false,
            tries: 0.0,
            plan: 0.0,
            face: None,
            bold: false,
            bare: false,
            drive: false,
            drive_t: 0.0,
            script: crate::game::GuardScript::zeroed(),
        }
    }
}
impl Default for Run { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: Foe, src/game/runner.ts:133)
#[derive(Clone, Copy, Debug)]
pub struct Foe {
    pub pos: V3,
    pub eye: V3,
    pub who: crate::picture::Who,
}
impl Foe {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Foe {
            pos: [0.0; 3],
            eye: [0.0; 3],
            who: crate::picture::Who::Guard(GuardId(0)),
        }
    }
}
impl Default for Foe { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RunnerSnap, src/game/runner.ts:419)
#[derive(Clone, Debug)]
pub struct RunnerSnap {
    pub men: Vec<Option<Vec<f64>>>,
}
impl RunnerSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunnerSnap {
            men: Vec::new(),
        }
    }
}
impl Default for RunnerSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/runner.ts:105 LINES, src/game/runner.ts:105)
#[derive(Clone, Debug)]
pub struct LINESValue {
    pub say: Vec<String>,
    pub he: Option<Vec<String>>,
    pub they: Option<Vec<String>>,
}
impl LINESValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESValue {
            say: Vec::new(),
            he: None,
            they: None,
        }
    }
}
impl Default for LINESValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/runner.ts:213 flood ->, src/game/runner.ts:213)
#[derive(Clone, Copy, Debug)]
pub struct Flood {
    pub out: Option<V3>,
    pub hidden: bool,
    pub corner: V3,
    pub shied: bool,
}
impl Flood {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Flood {
            out: None,
            hidden: false,
            corner: [0.0; 3],
            shied: false,
        }
    }
}
impl Default for Flood { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/runner.ts:288 planOut(F), src/game/runner.ts:288)
#[derive(Clone, Copy, Debug)]
pub struct PlanOutF {
    pub out: Option<V3>,
    pub shied: bool,
}
impl PlanOutF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlanOutF {
            out: None,
            shied: false,
        }
    }
}
impl Default for PlanOutF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/runner.ts:410 runnerOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct RunnerOf {
    pub at: V3,
    pub drive: bool,
    pub t: f64,
    pub from: V3,
    pub phase: crate::runner::RunPhase,
    pub plan: f64,
    pub goal: Option<V3>,
    pub face: Option<f64>,
    pub tries: f64,
    pub bare: bool,
    pub told: f64,
    pub mate: Option<GuardId>,
    pub drive_t: f64,
    pub stall: f64,
    pub bold: bool,
    pub fled: bool,
    pub unseen: f64,
    pub is_out: bool,
}
impl RunnerOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunnerOf {
            at: [0.0; 3],
            drive: false,
            t: 0.0,
            from: [0.0; 3],
            phase: crate::runner::RunPhase::Low,
            plan: 0.0,
            goal: None,
            face: None,
            tries: 0.0,
            bare: false,
            told: 0.0,
            mate: None,
            drive_t: 0.0,
            stall: 0.0,
            bold: false,
            fled: false,
            unseen: 0.0,
            is_out: false,
        }
    }
}
impl Default for RunnerOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RunnerMod {
    /// (src/game/runner.ts:99 stateOf)
    pub state_of: JsMap<GuardId, crate::runner::Run>,
    /// (src/game/runner.ts:191 rects)
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

/// (src/game/runner.ts:91 PHASES: a table nothing writes)
pub fn phases() -> Vec<String> { alloc::vec![String::from("say"), String::from("mates"), String::from("out"), String::from("low"), String::from("gone")] }

/// (src/game/runner.ts:105 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::runner::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("done"), crate::runner::LINESValue { say: alloc::vec![String::from("no. no, I'm done. I'm done!"), String::from("that's it — I'm done! I'm done!"), String::from("no more! I'm finished here!")], he: None, they: None }); t.set(String::from("mates"), crate::runner::LINESValue { say: alloc::vec![String::from("don't shoot — it's me! it's me!")], he: Some(alloc::vec![String::from("don't shoot, it's me! he's right behind me!"), String::from("it's me! hold your fire — he's coming!")]), they: Some(alloc::vec![String::from("don't shoot, it's me! they're right behind me!"), String::from("it's me! hold your fire — they're coming!")]) }); t } }
