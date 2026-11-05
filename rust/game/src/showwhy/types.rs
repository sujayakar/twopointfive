//! The types of `showwhy`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Why {
    Door,
    Stack,
    Line,
    Hurt,
    Keep,
}
impl Why {
    pub const fn as_str(self) -> &'static str {
        match self {
            Why::Door => "door",
            Why::Stack => "stack",
            Why::Line => "line",
            Why::Hurt => "hurt",
            Why::Keep => "keep",
        }
    }
}

/// (TypeScript: WhyNote, src/game/showwhy.ts:88)
#[derive(Clone, Debug)]
pub struct WhyNote {
    pub t: f64,
    pub who: String,
    pub why: crate::showwhy::Why,
    pub signal: Option<crate::character::SignalKind>,
    pub line: Option<String>,
}
impl WhyNote {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WhyNote {
            t: 0.0,
            who: String::new(),
            why: crate::showwhy::Why::Door,
            signal: None,
            line: None,
        }
    }
}
impl Default for WhyNote { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShowWhyStats, src/game/showwhy.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct ShowWhyStats {
    pub waves: f64,
    pub squeezes: f64,
    pub fists: f64,
    pub ways: f64,
    pub lines: f64,
    pub holds: f64,
}
impl ShowWhyStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShowWhyStats {
            waves: 0.0,
            squeezes: 0.0,
            fists: 0.0,
            ways: 0.0,
            lines: 0.0,
            holds: 0.0,
        }
    }
}
impl Default for ShowWhyStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Owed, src/game/showwhy.ts:91)
#[derive(Clone, Debug)]
pub struct Owed {
    pub text: String,
    pub why: crate::showwhy::Why,
    pub by: f64,
}
impl Owed {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Owed {
            text: String::new(),
            why: crate::showwhy::Why::Door,
            by: 0.0,
        }
    }
}
impl Default for Owed { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Mem, src/game/showwhy.ts:92)
#[derive(Clone, Debug)]
pub struct Mem {
    pub t: f64,
    pub line_t0: f64,
    pub line_seen_t: f64,
    pub line_on: bool,
    pub line_end_t: f64,
    pub wave_t: f64,
    pub waves: f64,
    pub line_b: f64,
    pub door_t0: f64,
    pub door_seen_t: f64,
    pub door_shown: bool,
    pub door_t: f64,
    pub hurt_said: f64,
    pub lag_t: f64,
    pub lag_n: f64,
    pub keep_t0: f64,
    pub keep_said: bool,
    pub keep_said_t: f64,
    pub owed: Vec<crate::showwhy::Owed>,
}
impl Mem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mem {
            t: 0.0,
            line_t0: 0.0,
            line_seen_t: 0.0,
            line_on: false,
            line_end_t: 0.0,
            wave_t: 0.0,
            waves: 0.0,
            line_b: 0.0,
            door_t0: 0.0,
            door_seen_t: 0.0,
            door_shown: false,
            door_t: 0.0,
            hurt_said: 0.0,
            lag_t: 0.0,
            lag_n: 0.0,
            keep_t0: 0.0,
            keep_said: false,
            keep_said_t: 0.0,
            owed: Vec::new(),
        }
    }
}
impl Default for Mem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OpMem, src/game/showwhy.ts:109)
#[derive(Clone, Debug)]
pub struct OpMem {
    pub t: f64,
    pub turns: Vec<crate::showwhy::OpMemTurnsItem>,
    pub fist_t: f64,
    pub fists: f64,
}
impl OpMem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpMem {
            t: 0.0,
            turns: Vec::new(),
            fist_t: 0.0,
            fists: 0.0,
        }
    }
}
impl Default for OpMem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/showwhy.ts:109 OpMem.turns, src/game/showwhy.ts:109)
#[derive(Clone, Copy, Debug)]
pub struct OpMemTurnsItem {
    pub gd: GuardId,
    pub at: f64,
}
impl OpMemTurnsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpMemTurnsItem {
            gd: GuardId(0),
            at: 0.0,
        }
    }
}
impl Default for OpMemTurnsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/showwhy.ts:110)
#[derive(Clone, Debug)]
pub struct State {
    pub stats: crate::showwhy::ShowWhyStats,
    pub log: Vec<crate::showwhy::WhyNote>,
    pub after_you_t: f64,
    pub keep_say_t: f64,
    pub line_say_t: f64,
    pub t: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            stats: crate::showwhy::ShowWhyStats::zeroed(),
            log: Vec::new(),
            after_you_t: 0.0,
            keep_say_t: 0.0,
            line_say_t: 0.0,
            t: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/showwhy.ts:81 SHOWWHY_LINES, src/game/showwhy.ts:81)
#[derive(Clone, Debug)]
pub struct SHOWWHYLINES {
    pub line: String,
    pub door: String,
    pub hurt: String,
    pub lag: String,
    pub keep: String,
}
impl SHOWWHYLINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SHOWWHYLINES {
            line: String::new(),
            door: String::new(),
            hurt: String::new(),
            lag: String::new(),
            keep: String::new(),
        }
    }
}
impl Default for SHOWWHYLINES { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct ShowwhyMod {
    /// (src/game/showwhy.ts:112 mem)
    pub mem: JsMap<GuardId, crate::showwhy::Mem>,
    /// (src/game/showwhy.ts:113 opMem)
    pub op_mem: JsMap<DoorOpId, crate::showwhy::OpMem>,
    /// (src/game/showwhy.ts:114 states)
    pub states: JsMap<PlayerId, crate::showwhy::State>,
}
impl ShowwhyMod {
    pub fn blank() -> Self {
        ShowwhyMod {
            mem: JsMap::new(),
            op_mem: JsMap::new(),
            states: JsMap::new(),
        }
    }
}

/// (src/game/showwhy.ts:81 SHOWWHY_LINES: a table nothing writes)
pub fn showwhy_lines() -> crate::showwhy::SHOWWHYLINES { crate::showwhy::SHOWWHYLINES { line: String::from("you're in my line."), door: String::from("after you."), hurt: String::from("I'm hit."), lag: String::from("I can't keep up."), keep: String::from("I've got this door.") } }
