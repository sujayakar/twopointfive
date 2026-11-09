//! The types of `seenwalk`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's price of the floor, the penalty of nav.findPath and C.dearAlong: `dear` on lit floor in view of an uncleared room and on floor outside the box, 1 elsewhere. The box and the eyes are this walk's; the knobs, g.cleared, g.col.nav and the block memo (g.m.seenwalk.shared, shared by every walk of the tick) are read live from g (seenwalk.ts seenPrice)
#[derive(Clone, Debug)]
pub enum SeenPrice {
    Floor { x0: f64, x1: f64, z0: f64, z1: f64, eyes: Vec<crate::seenwalk::SharedEyesItem> },
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Shared {
    pub t: f64,
    pub known: f64,
    pub stamp: String,
    pub eyes: Vec<crate::seenwalk::SharedEyesItem>,
    pub memo: JsMap<crate::Num, f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SharedEyesItem {
    pub x: f64,
    pub z: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SeenwalkMod {
    /// (src/game/seenwalk.ts:49 shared)
    pub shared: Option<crate::seenwalk::Shared>,
}
impl SeenwalkMod {
    pub fn blank() -> Self {
        SeenwalkMod {
            shared: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SeenwalkStats {
    pub asks: i32,
    pub blocks: i32,
    pub lines: i32,
    pub in_view: i32,
    pub round: i32,
}

/// (src/game/seenwalk.ts:44 SEENWALK_DEFAULTS: a table nothing writes)
pub fn seenwalk_defaults() -> crate::knobs::SeenwalkK { crate::knobs::SeenwalkK { lit_at: 0.1, dear: 6.0, long: 1.35, less: 1.5, eye_step: 4.0, eye_y: 1.6, margin: 8.0 } }
