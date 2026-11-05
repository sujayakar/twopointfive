//! The types of `par`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Medal {
    None,
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

/// (TypeScript: ParResult, src/game/par.ts:33)
#[derive(Clone, Debug)]
pub struct ParResult {
    pub secs: f64,
    pub lost: f64,
    pub of: f64,
    pub penalty: f64,
    pub total: f64,
    pub shown: f64,
    pub medal: crate::par::Medal,
    pub next: Option<crate::par::ParResultNext>,
    pub line: String,
}
impl ParResult {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParResult {
            secs: 0.0,
            lost: 0.0,
            of: 0.0,
            penalty: 0.0,
            total: 0.0,
            shown: 0.0,
            medal: crate::par::Medal::None,
            next: None,
            line: String::new(),
        }
    }
}
impl Default for ParResult { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/par.ts:33 ParResult.next, src/game/par.ts:33)
#[derive(Clone, Copy, Debug)]
pub struct ParResultNext {
    pub medal: crate::par::Medal,
    pub bar: f64,
    pub short: f64,
}
impl ParResultNext {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParResultNext {
            medal: crate::par::Medal::None,
            bar: 0.0,
            short: 0.0,
        }
    }
}
impl Default for ParResultNext { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ParSnap, src/game/par.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct ParSnap {
    pub t0: Option<f64>,
    pub said: bool,
}
impl ParSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParSnap {
            t0: None,
            said: false,
        }
    }
}
impl Default for ParSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Par, src/game/par.ts:51)
#[derive(Clone, Debug)]
pub struct Par {
    pub run: Option<u32>,
    pub origin: V3,
    pub order0: Option<u32>,
    pub fired0: f64,
    pub t0: f64,
    pub res: Option<crate::par::ParResult>,
    pub said: bool,
    pub said_row: Option<crate::par::ParSaidRow>,
    pub best_words: String,
    pub log: Vec<String>,
}
impl Par {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Par {
            run: None,
            origin: [0.0, 0.0, 0.0],
            order0: None,
            fired0: 0.0,
            t0: f64::NAN,
            res: None,
            said: false,
            said_row: None,
            best_words: String::from(""),
            log: Vec::new(),
        }
    }
}
impl Default for Par { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at quiet Par.run, quiet Par.run)
#[derive(Clone, Copy, Debug)]
pub struct ParRun {
}
impl ParRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParRun {
        }
    }
}
impl Default for ParRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Par.order0, quiet Par.order0)
#[derive(Clone, Copy, Debug)]
pub struct ParOrder0 {
}
impl ParOrder0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParOrder0 {
        }
    }
}
impl Default for ParOrder0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/par.ts:59 Par.saidRow, src/game/par.ts:59)
#[derive(Clone, Debug)]
pub struct ParSaidRow {
    pub text: String,
}
impl ParSaidRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParSaidRow {
            text: String::new(),
        }
    }
}
impl Default for ParSaidRow { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ParStats {
    pub started: i32,
    pub finished: i32,
}

/// (src/game/par.ts:27 PAR_START_M)
pub const PAR_START_M: f64 = 1.0;

/// (src/game/par.ts:30 MEDALS: a table nothing writes)
pub fn medals() -> Vec<String> { alloc::vec![String::from("gold"), String::from("silver"), String::from("bronze")] }
