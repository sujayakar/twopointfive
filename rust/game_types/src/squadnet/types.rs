//! The types of `squadnet`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct SquadNet {
    pub rows: Vec<crate::squadnet::NetRow>,
    /// (counts: in no bytes)
    #[bytes(skip)]
    pub stats: crate::squadnet::SquadNetStats,
    pub down: Vec<String>,
    pub seeded: bool,
    pub click_due: bool,
    pub down_due: bool,
    pub last_t: f64,
}
impl SquadNet {
    /// what a new one holds
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NetRow {
    pub text: String,
    pub at: V3,
    pub t: f64,
    pub dur: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadNetStats {
    pub lines: f64,
    pub downs: f64,
    pub whys: f64,
    pub quiets: f64,
    pub clears: f64,
    pub ends: f64,
}

#[derive(Clone, Debug, Default)]
pub struct WhyWords {
    pub row: String,
    pub ack: String,
}

#[derive(Clone, Debug, Default)]
pub struct InFloodR {
    pub cells: JsSet<crate::Num>,
    pub entries: Vec<crate::squadnet::InFloodREntriesItem>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct InFloodREntriesItem {
    pub door: DoorId,
}

