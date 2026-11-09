//! The types of `seenwalk`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SeenwalkMod {
    /// (src/game/seenwalk.ts:49 shared)
    pub shared: Option<crate::seenwalk::Shared>,
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

