//! The types of `coverfire`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum Who {
    Guard(GuardId),
    Player,
}

/// (TypeScript: Know, src/game/coverfire.ts:94)
#[derive(Clone, Copy, Debug)]
pub struct Know {
    pub who: crate::coverfire::Who,
    pub at: V3,
    pub y0: f64,
    pub v: [f64; 2],
    pub t: f64,
    pub call: Option<bool>,
}
impl Know {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Know {
            who: crate::coverfire::Who::Guard(GuardId(0)),
            at: [0.0; 3],
            y0: 0.0,
            v: [0.0; 2],
            t: 0.0,
            call: None,
        }
    }
}
impl Default for Know { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Chew, src/game/coverfire.ts:96)
#[derive(Clone, Debug)]
pub struct Chew {
    pub id: f64,
    pub kind: String,
    pub who: crate::coverfire::Who,
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
impl Chew {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Chew {
            id: 0.0,
            kind: String::new(),
            who: crate::coverfire::Who::Guard(GuardId(0)),
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
impl Default for Chew { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Man, src/game/coverfire.ts:97)
#[derive(Clone, Debug)]
pub struct Man {
    pub know: Vec<crate::coverfire::Know>,
    pub chew: Option<crate::coverfire::Chew>,
    pub gave: JsMap<crate::Num, f64>,
    pub no: Option<crate::coverfire::ManNo>,
    pub said_t: f64,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            know: Vec::new(),
            chew: None,
            gave: JsMap::new(),
            no: None,
            said_t: 0.0,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/coverfire.ts:97 Man.no, src/game/coverfire.ts:97)
#[derive(Clone, Copy, Debug)]
pub struct ManNo {
    pub id: f64,
    pub until: f64,
}
impl ManNo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManNo {
            id: 0.0,
            until: 0.0,
        }
    }
}
impl Default for ManNo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/coverfire.ts:98)
#[derive(Clone, Debug)]
pub struct State {
    pub last_t: f64,
    pub men: JsMap<GuardId, crate::coverfire::Man>,
    pub serial: f64,
    pub base: JsMap<String, f64>,
    pub stats: crate::coverfire::StateStats,
    pub by_id: JsMap<crate::Num, crate::coverfire::StateByIdValue>,
    pub said_t: f64,
    pub prune_t: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            last_t: 0.0,
            men: JsMap::new(),
            serial: 0.0,
            base: JsMap::new(),
            stats: crate::coverfire::StateStats::zeroed(),
            by_id: JsMap::new(),
            said_t: 0.0,
            prune_t: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/coverfire.ts:98 State.stats, src/game/coverfire.ts:84)
#[derive(Clone, Copy, Debug)]
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
impl StateStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateStats {
            chews: 0.0,
            shots: 0.0,
            landed: 0.0,
            opened: 0.0,
            gave_up: 0.0,
            held_friend: 0.0,
            held_silent: 0.0,
            calls: 0.0,
            walked: 0.0,
            op_chews: 0.0,
            def_chews: 0.0,
            thru: 0.0,
            thru_shots: 0.0,
        }
    }
}
impl Default for StateStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/coverfire.ts:98 State.byId, src/game/coverfire.ts:98)
#[derive(Clone, Copy, Debug)]
pub struct StateByIdValue {
    pub landed: f64,
    pub gave: f64,
}
impl StateByIdValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateByIdValue {
            landed: 0.0,
            gave: 0.0,
        }
    }
}
impl Default for StateByIdValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OpLike, src/game/coverfire.ts:100)
#[derive(Clone, Debug)]
pub struct OpLike {
    pub gd: GuardId,
    pub meters: JsMap<GuardId, crate::coverfire::OpLikeMetersValue>,
    pub tgt: Option<GuardId>,
    pub aim: V3,
    pub reload_until: f64,
    pub rounds: f64,
    pub next: f64,
}
impl OpLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpLike {
            gd: GuardId(0),
            meters: JsMap::new(),
            tgt: None,
            aim: [0.0; 3],
            reload_until: 0.0,
            rounds: 0.0,
            next: 0.0,
        }
    }
}
impl Default for OpLike { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/coverfire.ts:100 OpLike.meters, src/game/coverfire.ts:100)
#[derive(Clone, Copy, Debug)]
pub struct OpLikeMetersValue {
    pub m: f64,
    pub seen_t: f64,
}
impl OpLikeMetersValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpLikeMetersValue {
            m: 0.0,
            seen_t: 0.0,
        }
    }
}
impl Default for OpLikeMetersValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Ask, src/game/coverfire.ts:147)
#[derive(Clone, Debug)]
pub struct Ask {
    pub index: f64,
    pub id: f64,
    pub kind: String,
    pub hit: V3,
    pub rounds: f64,
    pub layers: f64,
}
impl Ask {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ask {
            index: 0.0,
            id: 0.0,
            kind: String::new(),
            hit: [0.0; 3],
            rounds: 0.0,
            layers: 0.0,
        }
    }
}
impl Default for Ask { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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
impl BumpK {
    pub const fn as_str(self) -> &'static str {
        match self {
            BumpK::Thru => "thru",
            BumpK::Shots => "shots",
            BumpK::Calls => "calls",
            BumpK::Landed => "landed",
            BumpK::Opened => "opened",
            BumpK::Chews => "chews",
            BumpK::GaveUp => "gaveUp",
            BumpK::HeldFriend => "heldFriend",
            BumpK::HeldSilent => "heldSilent",
            BumpK::Walked => "walked",
            BumpK::OpChews => "opChews",
            BumpK::DefChews => "defChews",
            BumpK::ThruShots => "thruShots",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum KnownWho {
    Guard(GuardId),
    Player,
}

/// (TypeScript: inline at src/game/coverfire.ts:135 known ->, src/game/coverfire.ts:135)
#[derive(Clone, Copy, Debug)]
pub struct KnownItem {
    pub k: crate::coverfire::Know,
    pub call: bool,
}
impl KnownItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KnownItem {
            k: crate::coverfire::Know::zeroed(),
            call: false,
        }
    }
}
impl Default for KnownItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Ask2 {
    Clear,
    Ask(crate::coverfire::Ask),
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum CoverSeenWho {
    Guard(GuardId),
    Player,
}

/// (TypeScript: inline at src/game/coverfire.ts:386 coverfireOn ->, src/game/coverfire.ts:386)
#[derive(Clone, Debug)]
pub struct CoverfireOn {
    pub id: f64,
    pub kind: String,
    pub fired: f64,
    pub want: f64,
    pub to: V3,
}
impl CoverfireOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverfireOn {
            id: 0.0,
            kind: String::new(),
            fired: 0.0,
            want: 0.0,
            to: [0.0; 3],
        }
    }
}
impl Default for CoverfireOn { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CoverfireMod {
    /// (src/game/coverfire.ts:85 COVERFIRE_LOG)
    pub coverfire_log: Vec<String>,
    /// (src/game/coverfire.ts:87 COVERFIRE_WHY)
    pub coverfire_why: JsMap<String, f64>,
    /// (src/game/coverfire.ts:102 states)
    pub states: Option<crate::coverfire::State>,
}
impl CoverfireMod {
    pub fn blank() -> Self {
        CoverfireMod {
            coverfire_log: Vec::new(),
            coverfire_why: JsMap::new(),
            states: None,
        }
    }
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

/// (src/game/coverfire.ts:81 COVERFIRE_DEFAULTS: a table nothing writes)
pub fn coverfire_defaults() -> crate::knobs::CoverfireK { crate::knobs::page_coverfire() }

/// (src/game/coverfire.ts:89 ZERO: a table nothing writes)
pub fn zero() -> crate::coverfire::StateStats { crate::coverfire::StateStats { chews: 0.0, shots: 0.0, landed: 0.0, opened: 0.0, gave_up: 0.0, held_friend: 0.0, held_silent: 0.0, calls: 0.0, walked: 0.0, op_chews: 0.0, def_chews: 0.0, thru: 0.0, thru_shots: 0.0 } }
