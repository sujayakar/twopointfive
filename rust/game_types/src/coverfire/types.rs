//! The types of `coverfire`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Know {
    pub who: CharId,
    pub at: V3,
    pub y0: f64,
    pub v: [f64; 2],
    pub t: f64,
    pub call: Option<bool>,
}
impl Default for Know {
    fn default() -> Self {
        Know {
            who: CharId(0),
            at: [0.0; 3],
            y0: 0.0,
            v: [0.0; 2],
            t: 0.0,
            call: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub struct Chew {
    pub id: f64,
    pub kind: String,
    pub who: CharId,
    pub to: V3,
    pub hit: V3,
    pub h0: V3,
    pub kt: f64,
    pub n: f64,
    pub fired: f64,
    pub want: f64,
    pub layers: f64,
    pub budget: f64,
    pub fresh: bool,
    pub lay: f64,
    pub t: f64,
    pub next: f64,
    pub k: f64,
    pub call: bool,
    pub seen: bool,
}
impl Default for Chew {
    fn default() -> Self {
        Chew {
            id: 0.0,
            kind: String::new(),
            who: CharId(0),
            to: [0.0; 3],
            hit: [0.0; 3],
            h0: [0.0; 3],
            kt: 0.0,
            n: 0.0,
            fired: 0.0,
            want: 0.0,
            layers: 0.0,
            budget: 0.0,
            fresh: false,
            lay: 0.0,
            t: 0.0,
            next: 0.0,
            k: 0.0,
            call: false,
            seen: false,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Man {
    pub know: Vec<crate::coverfire::Know>,
    pub chew: Option<crate::coverfire::Chew>,
    pub gave: JsMap<crate::Num, f64>,
    pub no: Option<crate::coverfire::ManNo>,
    pub said_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ManNo {
    pub id: f64,
    pub until: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub last_t: f64,
    pub men: JsMap<GuardId, crate::coverfire::Man>,
    pub serial: f64,
    pub stats: crate::coverfire::StateStats,
    pub by_id: JsMap<crate::Num, crate::coverfire::StateByIdValue>,
    pub said_t: f64,
    pub prune_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateStats {
    pub chews: f64,
    pub shots: f64,
    pub landed: f64,
    pub opened: f64,
    pub gave_up: f64,
    pub held_friend: f64,
    pub held_silent: f64,
    pub calls: f64,
    pub walked: f64,
    pub op_chews: f64,
    pub def_chews: f64,
    pub thru: f64,
    pub thru_shots: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateByIdValue {
    pub landed: f64,
    pub gave: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Ask {
    pub id: f64,
    pub kind: String,
    pub hit: V3,
    pub rounds: f64,
    pub layers: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BumpK {
    Thru,
    Shots,
    Calls,
    Landed,
    Opened,
    Chews,
    GaveUp,
    HeldFriend,
    HeldSilent,
    Walked,
    OpChews,
    DefChews,
    ThruShots,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KnownItem {
    pub k: crate::coverfire::Know,
    pub call: bool,
}

#[derive(Clone, Debug)]
pub enum Ask2 {
    Clear,
    Ask(crate::coverfire::Ask),
}

#[derive(Clone, Debug, Default)]
pub struct CoverfireOn {
    pub kind: String,
    pub fired: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct CoverfireMod {
    pub coverfire_log: Vec<String>,
    pub coverfire_why: JsMap<String, f64>,
    pub states: Option<crate::coverfire::State>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CoverfireStats {
    pub chews: i32,
    pub shots: i32,
    pub landed: i32,
    pub opened: i32,
    pub gave_up: i32,
    pub held_friend: i32,
    pub held_silent: i32,
    pub calls: i32,
    pub walked: i32,
    pub op_chews: i32,
    pub def_chews: i32,
    pub thru: i32,
    pub thru_shots: i32,
}

pub fn zero() -> crate::coverfire::StateStats { crate::coverfire::StateStats { chews: 0.0, shots: 0.0, landed: 0.0, opened: 0.0, gave_up: 0.0, held_friend: 0.0, held_silent: 0.0, calls: 0.0, walked: 0.0, op_chews: 0.0, def_chews: 0.0, thru: 0.0, thru_shots: 0.0 } }
