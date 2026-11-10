//! The types of `squadnet`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SquadNet {
    pub rows: Vec<crate::squadnet::NetRow>,
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

