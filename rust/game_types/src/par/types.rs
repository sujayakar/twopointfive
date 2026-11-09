//! The types of `par`: a room clear run against a clock (rust/game_rules/src/par).
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (on the wire: its word in small letters)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Medal {
    #[default] None,
    Gold,
    Silver,
    Bronze,
}
impl Medal {
    pub const fn as_str(self) -> &'static str {
        match self {
            Medal::None => "none",
            Medal::Gold => "gold",
            Medal::Silver => "silver",
            Medal::Bronze => "bronze",
        }
    }
}

/// a finished run against its par
#[derive(Clone, Debug, ToBytes, Default)]
pub struct ParResult {
    pub secs: f64,
    pub lost: f64,
    pub penalty: f64,
    pub total: f64,
    pub shown: f64,
    pub medal: crate::par::Medal,
    pub next: Option<crate::par::ParResultNext>,
    pub line: String,
}

/// the medal above the one earned, its bar, and how many whole seconds faster it needed
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ParResultNext {
    pub medal: crate::par::Medal,
    pub bar: f64,
    pub short: f64,
}

/// A seat's mark as the clock was armed: where its man stood, and the last order on its log then (by the row's serial: the log is cut from the front).
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ParMark {
    pub origin: V3,
    pub order0: Option<u32>,
}

/// THE PAR CLOCK'S RECORD: the game's own, written by the tick (`par::par_tick`), and all of it on the wire: the page draws `live`, `t0` and `res`,
/// and a game stood again from a page's copy takes the clock up where it was.
#[derive(Clone, Debug, ToBytes)]
pub struct Par {
    /// the mission this clock is for, by its serial (a restart makes a new mission: the clock starts over)
    pub run: Option<u32>,
    /// every seat's mark, in seat order: the first thing ANY of them does starts the clock
    pub marks: Vec<crate::par::ParMark>,
    /// the rounds and throws so far as the clock was armed
    pub fired0: f64,
    /// when the clock started on the game's clock (NaN: not yet)
    pub t0: f64,
    /// the finished run (None: not taken yet, wiped, or another mission's)
    pub res: Option<crate::par::ParResult>,
    /// a par is live now: the level has the block, its room clear is up, no show runs (written every tick)
    pub live: bool,
}
impl Par {
    /// what a new one holds
    pub fn blank() -> Self {
        Par {
            run: None,
            marks: Vec::new(),
            fired0: 0.0,
            t0: f64::NAN,
            res: None,
            live: false,
        }
    }
}
impl Default for Par { fn default() -> Self { Self::blank() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ParStats {
    pub started: i32,
    pub finished: i32,
}

/// how far a player walks from his mark before the clock is his (metres): more than a shuffle, less than the way to any door
pub const PAR_START_M: f64 = 1.0;
