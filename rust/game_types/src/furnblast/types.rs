//! The types of `furnblast`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum BlastSource {
    Frag,
    Door,
    Wall,
}
impl BlastSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            BlastSource::Frag => "frag",
            BlastSource::Door => "door",
            BlastSource::Wall => "wall",
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub struct Furniture {
    pub groups: Option<Vec<crate::furnblast::Group>>,
    pub member_at: JsMap<crate::Num, FurnblastMemberId>,
    pub hidden: JsMap<crate::Num, f64>,
    pub gone_props: f64,
    pub gone: JsSet<crate::Num>,
    pub gone_mouth: JsSet<crate::Num>,
    pub kept: JsSet<crate::Num>,
    pub fronts: Vec<V3>,
    pub counter: f64,
    pub last: Option<crate::furnblast::FurnitureLast>,
    pub row_kept: Option<crate::furnblast::FurnitureRowKept>,
    pub fling: f64,
    pub mouth_memo: crate::furnblast::FurnitureMouthMemo,
}
impl Furniture {
    /// what a new one holds
    pub fn blank() -> Self {
        Furniture {
            groups: None,
            member_at: JsMap::new(),
            hidden: JsMap::new(),
            gone_props: 0.0,
            gone: JsSet::new(),
            gone_mouth: JsSet::new(),
            kept: JsSet::new(),
            fronts: Vec::new(),
            counter: 1.0,
            last: None,
            row_kept: None,
            fling: 1.0,
            mouth_memo: crate::furnblast::FurnitureMouthMemo { t: -1.0, serial: -1.0, can: JsMap::new(), under: JsMap::new() },
        }
    }
}
impl Default for Furniture { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Group {
    pub id: f64,
    pub members: Vec<FurnblastMemberId>,
    pub a: crate::furnblast::AABB,
    pub heavy: f64,
    pub surf: crate::scene_materials::Surf,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AABB {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FurnitureLast {
    pub t: f64,
    pub at: V3,
    pub k: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FurnitureRowKept {
    pub t: f64,
    pub first: u32,
    pub origins: Vec<V3>,
    pub gone: JsSet<FurnblastMemberId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlastFirst {
    pub group: f64,
    pub at: V3,
    pub shatter_m: f64,
    /// INVENTED: g.serial() where firstToGo makes the answer: rowTake's memo asks by identity whether this is the same ask
    pub serial: u32,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FurnitureMouthMemo {
    pub t: f64,
    pub serial: f64,
    pub can: JsMap<crate::Num, bool>,
    pub under: JsMap<crate::Num, Vec<f64>>,
}

#[derive(Clone, Debug, Default)]
pub struct BreakRow {
    pub shatter: crate::furnblast::BreakRowShatter,
    pub apart: crate::furnblast::BreakRowApart,
    pub uneven: f64,
    pub fly: f64,
    pub spin: f64,
    pub land: crate::furnblast::BreakRowLand,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreakRowShatter {
    pub len: f64,
    pub max: f64,
    pub strips: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreakRowApart {
    pub len: f64,
    pub max: f64,
    pub strips: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BreakRowLand {
    pub level: f64,
    pub sfx: Vec<crate::furnblast::BreakRowLandSfxItem>,
}

#[derive(Clone, Debug, Default)]
pub struct BreakRowLandSfxItem {
    pub name: String,
    pub rate: f64,
    pub gain: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SteelPart {
    pub box_: BoxId,
    pub open: Option<crate::furnblast::SteelPartOpen>,
    pub seed: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SteelPartOpen {
    pub axis: f64,
    pub sign: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Member {
    pub i: f64,
    pub a: crate::furnblast::AABB,
    pub root: bool,
    pub ghost: bool,
    pub nb: Vec<FurnblastMemberId>,
    pub sup: Vec<FurnblastMemberId>,
    pub group: Option<crate::furnblast::Group>,
}

#[derive(Clone, Debug, Default)]
pub struct BreakClassOfB {
    pub does: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OverPlanA {
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OverPlanQ {
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureSteelMode {
    Apart,
    Shatter,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureFragmentsMode {
    Apart,
    Shatter,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureNoteResetCause {
    Load,
    Reset,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct FurnblastMod {
    pub furn_log: Vec<String>,
}
impl FurnblastMod {
    pub fn blank() -> Self {
        FurnblastMod {
            furn_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FurnblastStats {
    pub blasts: i32,
    pub merged: i32,
    pub groups: i32,
    pub shielded: i32,
    pub statics: i32,
    pub razed: i32,
    pub items: i32,
    pub chunks: i32,
    pub demoted: i32,
    pub left: i32,
    pub props_thrown: i32,
    pub props_broken: i32,
    pub props_shielded: i32,
    pub nav_opened: i32,
    pub mouth: i32,
    pub mouth_left: i32,
}

pub fn furn_break() -> Vec<(crate::scene_materials::BreakClass, crate::furnblast::BreakRow)> { alloc::vec![(crate::scene_materials::BreakClass::Wood, crate::furnblast::BreakRow { shatter: crate::furnblast::BreakRowShatter { len: 0.9, max: 2.0, strips: 3.0 }, apart: crate::furnblast::BreakRowApart { len: 1.2, max: 2.0, strips: 2.0 }, uneven: 0.6, fly: 1.0, spin: 1.0, land: crate::furnblast::BreakRowLand { level: 0.25, sfx: alloc::vec![crate::furnblast::BreakRowLandSfxItem { name: String::from("propThud"), rate: 1.15, gain: 0.5 }] } }), (crate::scene_materials::BreakClass::Board, crate::furnblast::BreakRow { shatter: crate::furnblast::BreakRowShatter { len: 0.45, max: 3.0, strips: 2.0 }, apart: crate::furnblast::BreakRowApart { len: 9.0, max: 1.0, strips: 1.0 }, uneven: 0.4, fly: 1.1, spin: 1.2, land: crate::furnblast::BreakRowLand { level: 0.15, sfx: alloc::vec![crate::furnblast::BreakRowLandSfxItem { name: String::from("propThud"), rate: 1.5, gain: 0.3 }] } }), (crate::scene_materials::BreakClass::Metal, crate::furnblast::BreakRow { shatter: crate::furnblast::BreakRowShatter { len: 0.7, max: 2.0, strips: 1.0 }, apart: crate::furnblast::BreakRowApart { len: 9.0, max: 1.0, strips: 1.0 }, uneven: 0.5, fly: 0.8, spin: 0.7, land: crate::furnblast::BreakRowLand { level: 0.45, sfx: alloc::vec![crate::furnblast::BreakRowLandSfxItem { name: String::from("propThud"), rate: 0.7, gain: 0.9 }, crate::furnblast::BreakRowLandSfxItem { name: String::from("magDrop"), rate: 0.5, gain: 0.4 }] } }), (crate::scene_materials::BreakClass::Hard, crate::furnblast::BreakRow { shatter: crate::furnblast::BreakRowShatter { len: 0.4, max: 3.0, strips: 2.0 }, apart: crate::furnblast::BreakRowApart { len: 0.8, max: 2.0, strips: 1.0 }, uneven: 0.5, fly: 1.0, spin: 1.0, land: crate::furnblast::BreakRowLand { level: 0.2, sfx: alloc::vec![crate::furnblast::BreakRowLandSfxItem { name: String::from("propThud"), rate: 1.3, gain: 0.4 }] } }), (crate::scene_materials::BreakClass::Soft, crate::furnblast::BreakRow { shatter: crate::furnblast::BreakRowShatter { len: 9.0, max: 1.0, strips: 1.0 }, apart: crate::furnblast::BreakRowApart { len: 9.0, max: 1.0, strips: 1.0 }, uneven: 0.0, fly: 1.2, spin: 1.3, land: crate::furnblast::BreakRowLand { level: 0.1, sfx: alloc::vec![crate::furnblast::BreakRowLandSfxItem { name: String::from("propThud"), rate: 0.9, gain: 0.3 }] } })] }

pub const HIDE: f64 = 3.0;

pub const STEEL_CARCASS_M: f64 = 0.25;

pub const STEEL_SHEETS_ROOM: f64 = 16.0;

pub const STEEL_CARCASS_MOST: f64 = 2.2;

