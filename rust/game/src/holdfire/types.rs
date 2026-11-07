//! The types of `holdfire`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LiftWhy {
    Shot,
    Order,
    Drill,
}
impl LiftWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            LiftWhy::Shot => "shot",
            LiftWhy::Order => "order",
            LiftWhy::Drill => "drill",
        }
    }
}

/// (TypeScript: State, src/game/holdfire.ts:72)
#[derive(Clone, Debug)]
pub struct State {
    pub lift_t: f64,
    pub why: Option<crate::holdfire::LiftWhy>,
    pub cast: Vec<GuardId>,
    pub by: Option<PlayerId>,
    pub loud_t: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            lift_t: 0.0,
            why: None,
            cast: Vec::new(),
            by: None,
            loud_t: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/holdfire.ts:70 HOLDFIRE_LOG, src/game/holdfire.ts:70)
#[derive(Clone, Debug)]
pub struct HoldfireModHOLDFIRELOGItem {
    pub t: f64,
    pub why: crate::holdfire::LiftWhy,
    pub what: String,
}
impl HoldfireModHOLDFIRELOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldfireModHOLDFIRELOGItem {
            t: 0.0,
            why: crate::holdfire::LiftWhy::Shot,
            what: String::new(),
        }
    }
}
impl Default for HoldfireModHOLDFIRELOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/holdfire.ts:126 holdFireOrder(o), src/game/holdfire.ts:126)
#[derive(Clone, Copy, Debug)]
pub struct HoldFireOrderO {
    pub waits: Option<bool>,
    pub blown: Option<bool>,
    pub why: Option<crate::holdfire::LiftWhy>,
    pub by: Option<PlayerId>,
    pub silent: Option<bool>,
}
impl HoldFireOrderO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoldFireOrderO {
            waits: None,
            blown: None,
            why: None,
            by: None,
            silent: None,
        }
    }
}
impl Default for HoldFireOrderO { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct HoldfireMod {
    /// (src/game/holdfire.ts:70 HOLDFIRE_LOG)
    pub holdfire_log: Vec<crate::holdfire::HoldfireModHOLDFIRELOGItem>,
    /// (src/game/holdfire.ts:74 states)
    pub states: JsMap<PlayerId, crate::holdfire::State>,
}
impl HoldfireMod {
    pub fn blank() -> Self {
        HoldfireMod {
            holdfire_log: Vec::new(),
            states: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct HoldfireStats {
    pub held: i32,
    pub held_glass: i32,
    pub lifts: [i32; 3],
}

/// (src/game/holdfire.ts:62 LIFT_ORDERS: a table nothing writes)
pub fn lift_orders() -> Vec<String> { alloc::vec![String::from("openClear"), String::from("bigClear"), String::from("bangClear"), String::from("fragClear"), String::from("charge"), String::from("threshold"), String::from("thresholdHold"), String::from("outside"), String::from("thrStep"), String::from("thrBang"), String::from("thrWord"), String::from("go"), String::from("wallCharge"), String::from("fragThere"), String::from("stunThere"), String::from("shellThere")] }

/// (src/game/holdfire.ts:64 BLOW_ORDERS: a table nothing writes)
pub fn blow_orders() -> Vec<String> { alloc::vec![String::from("charge"), String::from("wallCharge")] }

/// (src/game/holdfire.ts:66 LOUD_ORDER)
pub const LOUD_ORDER: &str = "loud";

/// (src/game/holdfire.ts:100 WAITS: a table nothing writes)
pub fn waits() -> Vec<String> { alloc::vec![String::from("place"), String::from("standoff"), String::from("armed")] }
