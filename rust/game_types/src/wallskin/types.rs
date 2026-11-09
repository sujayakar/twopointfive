//! The types of `wallskin`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Brk, src/game/wallskin.ts:71)
#[derive(Clone, Debug)]
pub struct Brk {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: String,
    pub box_: BoxId,
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub yb: Vec<f64>,
    pub open: Vec<u8>,
    pub slots: Vec<f64>,
    pub live: f64,
}
impl Brk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Brk {
            id: 0.0,
            kind: crate::wallbreak::BKind::Wall,
            surf: String::new(),
            box_: BoxId(0),
            nx: 0.0,
            ny: 0.0,
            nz: 0.0,
            yb: Vec::new(),
            open: Vec::new(),
            slots: Vec::new(),
            live: 0.0,
        }
    }
}
impl Default for Brk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Seat, src/game/wallskin.ts:77)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Seat {
    pub k: f64,
    pub n: f64,
    pub face: f64,
    pub t: f64,
    pub side: f64,
    pub near: f64,
    pub r: [f64; 4],
}
impl Seat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Seat {
            k: 0.0,
            n: 0.0,
            face: 0.0,
            t: 0.0,
            side: 0.0,
            near: 0.0,
            r: [0.0; 4],
        }
    }
}
impl Default for Seat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallSkin, src/game/wallskin.ts:115)
#[derive(Clone, Debug, ToBytes)]
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
    pub room: f64,
    pub was: bool,
    pub failed: bool,
    pub marked: JsSet<crate::Num>,
    pub stats: crate::wallskin::WallSkinStats,
}
impl WallSkin {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        WallSkin {
            cut: JsMap::new(),
            out: JsSet::new(),
            laid: JsMap::new(),
            seats: JsMap::new(),
            on: JsMap::new(),
            dirty: JsSet::new(),
            cand: Vec::new(),
            solid: Vec::new(),
            last: 0.0,
            room: 1000000000.0,
            was: false,
            failed: false,
            marked: JsSet::new(),
            stats: crate::wallskin::WallSkinStats { cut: 0.0, whole: 0.0, pieces: 0.0, capped: 0.0, coarse: 0.0, uncut: 0.0 },
        }
    }
}
impl Default for WallSkin { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/wallskin.ts:121 WallSkin.laid, src/game/wallskin.ts:121)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct WallSkinLaidValueItem {
    pub r: [f64; 4],
    pub slot: f64,
}
impl WallSkinLaidValueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSkinLaidValueItem {
            r: [0.0; 4],
            slot: 0.0,
        }
    }
}
impl Default for WallSkinLaidValueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallskin.ts:135 WallSkin.stats, src/game/wallskin.ts:135)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct WallSkinStats {
    pub cut: f64,
    pub whole: f64,
    pub pieces: f64,
    pub capped: f64,
    pub coarse: f64,
    pub uncut: f64,
}
impl WallSkinStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSkinStats {
            cut: 0.0,
            whole: 0.0,
            pieces: 0.0,
            capped: 0.0,
            coarse: 0.0,
            uncut: 0.0,
        }
    }
}
impl Default for WallSkinStats { fn default() -> Self { Self::zeroed() } }

/// (src/game/wallskin.ts:78 EPS)
pub const EPS: f64 = 0.0001;

/// (src/game/wallskin.ts:78 ALL: a table nothing writes)
pub fn all() -> [f64; 4] { [-1000000000.0, 1000000000.0, -1000000000.0, 1000000000.0] }

/// (src/game/wallskin.ts:78 SLOTS)
pub const SLOTS: f64 = 16.0;

/// (src/game/wallskin.ts:80 FLUSH)
pub const FLUSH: f64 = 0.002;
