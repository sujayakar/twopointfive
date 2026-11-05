//! The types of `rescueflush`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RescueFlush, src/game/rescueflush.ts:63)
#[derive(Clone, Debug)]
pub struct RescueFlush {
    pub fell_at: f64,
    pub next_at: f64,
    pub gone_at: f64,
    pub count: f64,
    pub sent: JsMap<GuardId, crate::rescueflush::Sent>,
    pub barred: JsSet<GuardId>,
}
impl RescueFlush {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        RescueFlush {
            fell_at: f64::NAN,
            next_at: f64::NAN,
            gone_at: f64::NAN,
            count: 0.0,
            sent: JsMap::new(),
            barred: JsSet::new(),
        }
    }
}
impl Default for RescueFlush { fn default() -> Self { Self::blank() } }

/// (TypeScript: Sent, src/game/rescueflush.ts:60)
#[derive(Clone, Debug)]
pub struct Sent {
    pub at: f64,
    pub n: f64,
    pub goal: crate::rescueflush::Goal,
    pub via: Vec<V3>,
}
impl Sent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Sent {
            at: 0.0,
            n: 0.0,
            goal: crate::rescueflush::Goal::Out,
            via: Vec::new(),
        }
    }
}
impl Default for Sent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Goal {
    Out,
    Side,
    Noise,
    Room,
}
impl Goal {
    pub const fn as_str(self) -> &'static str {
        match self {
            Goal::Out => "out",
            Goal::Side => "side",
            Goal::Noise => "noise",
            Goal::Room => "room",
        }
    }
}

/// (TypeScript: RescueFlushSnap, src/game/rescueflush.ts:61)
#[derive(Clone, Debug)]
pub struct RescueFlushSnap {
    pub fell_at: f64,
    pub next_at: f64,
    pub gone_at: f64,
    pub count: f64,
    pub sent: Vec<crate::rescueflush::RescueFlushSnapSentItem>,
    pub barred: Vec<f64>,
}
impl RescueFlushSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueFlushSnap {
            fell_at: 0.0,
            next_at: 0.0,
            gone_at: 0.0,
            count: 0.0,
            sent: Vec::new(),
            barred: Vec::new(),
        }
    }
}
impl Default for RescueFlushSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescueflush.ts:61 RescueFlushSnap.sent, src/game/rescueflush.ts:61)
#[derive(Clone, Debug)]
pub struct RescueFlushSnapSentItem {
    pub id: f64,
    pub at: f64,
    pub n: f64,
    pub goal: crate::rescueflush::Goal,
    pub via: Vec<Vec<f64>>,
}
impl RescueFlushSnapSentItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueFlushSnapSentItem {
            id: 0.0,
            at: 0.0,
            n: 0.0,
            goal: crate::rescueflush::Goal::Out,
            via: Vec::new(),
        }
    }
}
impl Default for RescueFlushSnapSentItem { fn default() -> Self { Self::zeroed() } }
