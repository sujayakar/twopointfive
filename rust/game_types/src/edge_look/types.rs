//! The types of `edge_look`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreakLike {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: String,
    pub box_: BoxId,
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub yb: Vec<f64>,
    pub open: Vec<u8>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Rubble {
    pub box_: FrameBoxId,
    pub t: f64,
    pub t_2: f64,
    pub p0: V3,
    pub p1: V3,
    pub vy: f64,
    pub q0: Quat,
    pub q1: Quat,
    pub foot: Option<DebrisChunkId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Frame {
    pub b: BoxId,
    pub along_x: bool,
    pub t: f64,
    pub l: f64,
    pub n: f64,
    pub w: f64,
    pub y0: f64,
    pub y_top: f64,
    pub y_sill: f64,
    pub along: V3,
    pub nrm: V3,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct EdgeLook {
    pub decor: JsMap<crate::Num, Vec<FrameBoxId>>,
    pub dirty: JsMap<crate::Num, crate::edge_look::BreakLike>,
    pub rubble: Vec<crate::edge_look::Rubble>,
    pub flying: f64,
    pub serial: f64,
    #[is(1000000000.0)] pub room: f64,
    pub blasted: JsMap<crate::Num, Vec<[f64; 2]>>,
    pub todo: Vec<crate::edge_look::EdgeLookTodoItem>,
    pub held: bool,
    #[is(crate::debris::FootBook { shoves: 0.0, stats: crate::debris::DebrisFeetStats { kicked: 0.0, off_man: 0.0, walled: 0.0, gave_up: 0.0 } })] pub foot_book: crate::debris::FootBook,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct EdgeLookTodoItem {
    pub b: crate::edge_look::BreakLike,
    pub f: Option<crate::edge_look::Frame>,
    pub c: f64,
    pub dir: Option<V3>,
    pub blast: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct EdgeLookMod {
}

pub const MKEY_LOOK: f64 = 150994944.0;

pub fn up() -> V3 { [0.0, 1.0, 0.0] }
