//! The types of `squad40`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Shot {
    pub gd: GuardId,
    pub from: V3,
    pub to: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub burst: V3,
    pub aimed: bool,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Turn {
    pub at: V3,
    pub t0: f64,
    pub fired_t: f64,
    pub shot: crate::squad40::Shot,
    pub mates: Vec<GuardId>,
    pub gd: GuardId,
    pub t: crate::orders::OrderTarget,
    pub a: Option<OrdersAssignmentId>,
    pub home: V3,
    pub home_yaw: f64,
    pub back: Option<crate::squadthrow::LentBack>,
}
impl Default for Turn {
    fn default() -> Self {
        Turn {
            at: [0.0; 3],
            t0: 0.0,
            fired_t: 0.0,
            shot: crate::squad40::Shot::default(),
            mates: Vec::new(),
            gd: GuardId(0),
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
            a: None,
            home: [0.0; 3],
            home_yaw: 0.0,
            back: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub rounds: JsMap<GuardId, f64>,
    pub brought: f64,
    pub last: JsMap<GuardId, f64>,
    pub turns: JsMap<GuardId, crate::squad40::Turn>,
    pub shown: Option<crate::squad40::StateShown>,
    /// the shot the menu's lit row would give, as it was this frame (Rust's own: not on the wire)
    pub plan: Option<crate::squad40::ShellPlan>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StateShown {
    pub t: f64,
    pub text: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Squad40ModSQUAD40LOGItem {
    pub t: f64,
    pub who: String,
    pub from: V3,
    pub at: V3,
    pub burst: V3,
    pub left: f64,
    pub turned: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Bodies {
    pub friends: Vec<CharId>,
    pub foes: Vec<CharId>,
    pub mates: Vec<CharId>,
}

#[derive(Clone, Debug)]
pub enum ShotOf {
    Close,
    Shot(crate::squad40::Shot),
}

/// THE SHELL AT A SPOT, and everything anybody is shown or told of it: the menu's row, its refusal, the picture, and the order given are all read off one of these.
#[derive(Clone, Debug, ToBytes)]
pub struct ShellPlan {
    pub at: V3,
    /// the men it was asked of
    pub men: Vec<GuardId>,
    /// the man and his shot; none: nobody has one
    pub shot: Option<crate::squad40::Shot>,
    /// why it may not be given, in the row's words
    pub no: Option<String>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct Squad40Mod {
    /// (src/game/squad40.ts:87 states)
    pub states: JsMap<PlayerId, crate::squad40::State>,
    /// (src/game/squad40.ts:90 SQUAD40_LOG)
    pub squad40_log: Vec<crate::squad40::Squad40ModSQUAD40LOGItem>,
}
impl Squad40Mod {
    pub fn blank() -> Self {
        Squad40Mod {
            states: JsMap::new(),
            squad40_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct Squad40Stats {
    pub staged: i32,
    pub fired: i32,
    pub turned: i32,
    pub refused: i32,
    pub gave_up: i32,
}

/// (src/game/squad40.ts:148 none)
pub const none: &str = "no 40 mm left.";
