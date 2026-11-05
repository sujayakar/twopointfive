//! The types of `squadnet`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: NetRow, src/game/squadnet.ts:63)
#[derive(Clone, Debug)]
pub struct NetRow {
    pub text: String,
    pub at: V3,
    pub t: f64,
    pub dur: f64,
}
impl NetRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetRow {
            text: String::new(),
            at: [0.0; 3],
            t: 0.0,
            dur: 0.0,
        }
    }
}
impl Default for NetRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SquadNet, src/game/squadnet.ts:66)
#[derive(Clone, Debug)]
pub struct SquadNet {
    pub rows: Vec<crate::squadnet::NetRow>,
    pub stats: crate::squadnet::SquadNetStats,
    pub down: Vec<String>,
    pub seeded: bool,
    pub click_due: bool,
    pub down_due: bool,
    pub last_t: f64,
}
impl SquadNet {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SquadNet {
            rows: Vec::new(),
            stats: crate::squadnet::SquadNetStats { lines: 0.0, downs: 0.0, whys: 0.0, quiets: 0.0, clears: 0.0, ends: 0.0 },
            down: Vec::new(),
            seeded: false,
            click_due: false,
            down_due: false,
            last_t: -1.0,
        }
    }
}
impl Default for SquadNet { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/squadnet.ts:68 SquadNet.stats, src/game/squadnet.ts:64)
#[derive(Clone, Copy, Debug)]
pub struct SquadNetStats {
    pub lines: f64,
    pub downs: f64,
    pub whys: f64,
    pub quiets: f64,
    pub clears: f64,
    pub ends: f64,
}
impl SquadNetStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadNetStats {
            lines: 0.0,
            downs: 0.0,
            whys: 0.0,
            quiets: 0.0,
            clears: 0.0,
            ends: 0.0,
        }
    }
}
impl Default for SquadNetStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadnet.ts:98 whyWords ->, src/game/squadnet.ts:98)
#[derive(Clone, Debug)]
pub struct WhyWords {
    pub row: String,
    pub ack: String,
}
impl WhyWords {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WhyWords {
            row: String::new(),
            ack: String::new(),
        }
    }
}
impl Default for WhyWords { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadnet.ts:128 inFlood(R), src/game/squadnet.ts:128)
#[derive(Clone, Debug)]
pub struct InFloodR {
    pub cells: JsSet<crate::Num>,
    pub entries: Vec<crate::squadnet::InFloodREntriesItem>,
}
impl InFloodR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        InFloodR {
            cells: JsSet::new(),
            entries: Vec::new(),
        }
    }
}
impl Default for InFloodR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadnet.ts:128 InFloodR.entries, src/game/squadnet.ts:128)
#[derive(Clone, Copy, Debug)]
pub struct InFloodREntriesItem {
    pub door: DoorId,
}
impl InFloodREntriesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        InFloodREntriesItem {
            door: DoorId(0),
        }
    }
}
impl Default for InFloodREntriesItem { fn default() -> Self { Self::zeroed() } }

/// (src/game/squadnet.ts:134 NET_ROW)
pub const NET_ROW: f64 = 26.0;

/// (src/game/squadnet.ts:137 NET_INSET)
pub const NET_INSET: f64 = 0.15;
