//! The types of `squadnet`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SquadNet {
    pub rows: Vec<crate::squadnet::NetRow>,
    /// (counts: in no bytes)
    #[bytes(skip)]
    #[is(crate::squadnet::SquadNetStats { lines: 0.0, downs: 0.0, whys: 0.0, quiets: 0.0, clears: 0.0, ends: 0.0 })] pub stats: crate::squadnet::SquadNetStats,
    pub down: Vec<String>,
    pub seeded: bool,
    pub click_due: bool,
    pub down_due: bool,
    #[is(-1.0)] pub last_t: f64,
}

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

