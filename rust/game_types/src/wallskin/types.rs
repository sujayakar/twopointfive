//! The types of `wallskin`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Seat {
    pub k: f64,
    pub n: f64,
    pub face: f64,
    pub t: f64,
    pub side: f64,
    pub near: f64,
    pub r: [f64; 4],
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallSkin {
    pub cut: JsMap<crate::Num, Vec<FrameBoxId>>,
    pub out: JsSet<crate::Num>,
    pub laid: JsMap<crate::Num, Vec<crate::wallskin::WallSkinLaidValueItem>>,
    pub seats: JsMap<crate::Num, Vec<crate::wallskin::Seat>>,
    pub on: JsMap<crate::Num, Vec<f64>>,
    pub dirty: JsSet<crate::Num>,
    pub cand: Vec<f64>,
    pub solid: Vec<u8>,
    pub last: f64,
    #[is(1000000000.0)] pub room: f64,
    pub was: bool,
    pub failed: bool,
    pub marked: JsSet<crate::Num>,
    #[is(crate::wallskin::WallSkinStats { cut: 0.0, whole: 0.0, pieces: 0.0, capped: 0.0, coarse: 0.0, uncut: 0.0 })] pub stats: crate::wallskin::WallSkinStats,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallSkinLaidValueItem {
    pub r: [f64; 4],
    pub slot: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallSkinStats {
    pub cut: f64,
    pub whole: f64,
    pub pieces: f64,
    pub capped: f64,
    pub coarse: f64,
    pub uncut: f64,
}

pub const EPS: f64 = 0.0001;

pub fn all() -> [f64; 4] { [-1000000000.0, 1000000000.0, -1000000000.0, 1000000000.0] }

pub const SLOTS: f64 = 16.0;

pub const FLUSH: f64 = 0.002;
