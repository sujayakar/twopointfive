//! The types of `furnblast`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: Furniture, src/game/furnblast.ts:143)
#[derive(Clone, Debug)]
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
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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

/// (TypeScript: Group, src/game/furnblast.ts:138)
#[derive(Clone, Debug)]
pub struct Group {
    pub id: f64,
    pub members: Vec<FurnblastMemberId>,
    pub a: crate::furnblast::AABB,
    pub heavy: f64,
    pub surf: crate::scene_materials::Surf,
}
impl Group {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Group {
            id: 0.0,
            members: Vec::new(),
            a: crate::furnblast::AABB::zeroed(),
            heavy: 0.0,
            surf: crate::scene_materials::Surf::Carpet,
        }
    }
}
impl Default for Group { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AABB, src/game/furnblast.ts:129)
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl AABB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AABB {
            x0: 0.0,
            x1: 0.0,
            y0: 0.0,
            y1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for AABB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:155 Furniture.last, src/game/furnblast.ts:155)
#[derive(Clone, Copy, Debug)]
pub struct FurnitureLast {
    pub t: f64,
    pub at: V3,
    pub k: f64,
}
impl FurnitureLast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureLast {
            t: 0.0,
            at: [0.0; 3],
            k: 0.0,
        }
    }
}
impl Default for FurnitureLast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:263 Furniture.rowKept, src/game/furnblast.ts:263)
#[derive(Clone, Debug)]
pub struct FurnitureRowKept {
    pub t: f64,
    pub first: u32,
    pub origins: Vec<V3>,
    pub gone: JsSet<FurnblastMemberId>,
}
impl FurnitureRowKept {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureRowKept {
            t: 0.0,
            first: 0,
            origins: Vec::new(),
            gone: JsSet::new(),
        }
    }
}
impl Default for FurnitureRowKept { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BlastFirst, src/game/furnblast.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct BlastFirst {
    pub group: f64,
    pub at: V3,
    pub shatter_m: f64,
    /// INVENTED: g.serial() where firstToGo makes the answer (furnblast.ts:243): rowTake's memo asks by identity whether this is the same ask (furnblast.ts:259)
    pub serial: u32,
}
impl BlastFirst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastFirst {
            group: 0.0,
            at: [0.0; 3],
            shatter_m: 0.0,
            serial: 0,
        }
    }
}
impl Default for BlastFirst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:565 Furniture.mouthMemo, src/game/furnblast.ts:565)
#[derive(Clone, Debug)]
pub struct FurnitureMouthMemo {
    pub t: f64,
    pub serial: f64,
    pub can: JsMap<crate::Num, bool>,
    pub under: JsMap<crate::Num, Vec<f64>>,
}
impl FurnitureMouthMemo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureMouthMemo {
            t: 0.0,
            serial: 0.0,
            can: JsMap::new(),
            under: JsMap::new(),
        }
    }
}
impl Default for FurnitureMouthMemo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreakRow, src/game/furnblast.ts:103)
#[derive(Clone, Debug)]
pub struct BreakRow {
    pub shatter: crate::furnblast::BreakRowShatter,
    pub apart: crate::furnblast::BreakRowApart,
    pub uneven: f64,
    pub fly: f64,
    pub spin: f64,
    pub land: crate::furnblast::BreakRowLand,
}
impl BreakRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRow {
            shatter: crate::furnblast::BreakRowShatter::zeroed(),
            apart: crate::furnblast::BreakRowApart::zeroed(),
            uneven: 0.0,
            fly: 0.0,
            spin: 0.0,
            land: crate::furnblast::BreakRowLand::zeroed(),
        }
    }
}
impl Default for BreakRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:103 BreakRow.shatter, src/game/furnblast.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct BreakRowShatter {
    pub len: f64,
    pub max: f64,
    pub strips: f64,
}
impl BreakRowShatter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRowShatter {
            len: 0.0,
            max: 0.0,
            strips: 0.0,
        }
    }
}
impl Default for BreakRowShatter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:103 BreakRow.apart, src/game/furnblast.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct BreakRowApart {
    pub len: f64,
    pub max: f64,
    pub strips: f64,
}
impl BreakRowApart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRowApart {
            len: 0.0,
            max: 0.0,
            strips: 0.0,
        }
    }
}
impl Default for BreakRowApart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:103 BreakRow.land, src/game/furnblast.ts:103)
#[derive(Clone, Debug)]
pub struct BreakRowLand {
    pub level: f64,
    pub sfx: Vec<crate::furnblast::BreakRowLandSfxItem>,
}
impl BreakRowLand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRowLand {
            level: 0.0,
            sfx: Vec::new(),
        }
    }
}
impl Default for BreakRowLand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:103 BreakRowLand.sfx, src/game/furnblast.ts:103)
#[derive(Clone, Debug)]
pub struct BreakRowLandSfxItem {
    pub name: String,
    pub rate: f64,
    pub gain: f64,
}
impl BreakRowLandSfxItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRowLandSfxItem {
            name: String::new(),
            rate: 0.0,
            gain: 0.0,
        }
    }
}
impl Default for BreakRowLandSfxItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FurnSave, src/game/furnblast.ts:115)
#[derive(Clone, Debug)]
pub struct FurnSave {
    pub v: f64,
    pub s: Vec<f64>,
    pub p: Vec<f64>,
    pub m: Option<Vec<f64>>,
}
impl FurnSave {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnSave {
            v: 0.0,
            s: Vec::new(),
            p: Vec::new(),
            m: None,
        }
    }
}
impl Default for FurnSave { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SteelPart, src/game/furnblast.ts:126)
#[derive(Clone, Copy, Debug)]
pub struct SteelPart {
    pub box_: BoxId,
    pub open: Option<crate::furnblast::SteelPartOpen>,
    pub seed: Option<f64>,
}
impl SteelPart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SteelPart {
            box_: BoxId(0),
            open: None,
            seed: None,
        }
    }
}
impl Default for SteelPart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:126 SteelPart.open, src/game/furnblast.ts:126)
#[derive(Clone, Copy, Debug)]
pub struct SteelPartOpen {
    pub axis: f64,
    pub sign: f64,
}
impl SteelPartOpen {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SteelPartOpen {
            axis: 0.0,
            sign: 0.0,
        }
    }
}
impl Default for SteelPartOpen { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Member, src/game/furnblast.ts:137)
#[derive(Clone, Debug)]
pub struct Member {
    pub i: f64,
    pub a: crate::furnblast::AABB,
    pub root: bool,
    pub ghost: bool,
    pub nb: Vec<FurnblastMemberId>,
    pub sup: Vec<FurnblastMemberId>,
    pub group: Option<crate::furnblast::Group>,
}
impl Member {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Member {
            i: 0.0,
            a: crate::furnblast::AABB::zeroed(),
            root: false,
            ghost: false,
            nb: Vec::new(),
            sup: Vec::new(),
            group: None,
        }
    }
}
impl Default for Member { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:142 NONE_DONE, src/game/furnblast.ts:142)
#[derive(Clone, Copy, Debug)]
pub struct NONEDONE {
}
impl NONEDONE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NONEDONE {
        }
    }
}
impl Default for NONEDONE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:112 breakClassOf(b), src/game/furnblast.ts:112)
#[derive(Clone, Debug)]
pub struct BreakClassOfB {
    pub does: Option<String>,
}
impl BreakClassOfB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakClassOfB {
            does: None,
        }
    }
}
impl Default for BreakClassOfB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:141 overPlan(a), src/game/furnblast.ts:141)
#[derive(Clone, Copy, Debug)]
pub struct OverPlanA {
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
}
impl OverPlanA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OverPlanA {
            x0: 0.0,
            z0: 0.0,
            x1: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for OverPlanA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:141 overPlan(q), src/game/furnblast.ts:141)
#[derive(Clone, Copy, Debug)]
pub struct OverPlanQ {
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
}
impl OverPlanQ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OverPlanQ {
            x0: 0.0,
            z0: 0.0,
            x1: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for OverPlanQ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:227 inventory ->, src/game/furnblast.ts:227)
#[derive(Clone, Debug)]
pub struct FurnitureInventory {
    pub groups: f64,
    pub members: f64,
    pub hidden: f64,
    pub gone_props: f64,
    pub list: Vec<crate::furnblast::FurnitureInventoryListItem>,
}
impl FurnitureInventory {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureInventory {
            groups: 0.0,
            members: 0.0,
            hidden: 0.0,
            gone_props: 0.0,
            list: Vec::new(),
        }
    }
}
impl Default for FurnitureInventory { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:227 FurnitureInventory.list, src/game/furnblast.ts:227)
#[derive(Clone, Debug)]
pub struct FurnitureInventoryListItem {
    pub id: f64,
    pub members: Vec<f64>,
    pub a: crate::furnblast::AABB,
    pub heavy: f64,
    pub surf: crate::scene_materials::Surf,
}
impl FurnitureInventoryListItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureInventoryListItem {
            id: 0.0,
            members: Vec::new(),
            a: crate::furnblast::AABB::zeroed(),
            heavy: 0.0,
            surf: crate::scene_materials::Surf::Carpet,
        }
    }
}
impl Default for FurnitureInventoryListItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:335 fixed(budget), src/game/furnblast.ts:335)
#[derive(Clone, Copy, Debug)]
pub struct FurnitureFixedBudget {
    pub chunks: f64,
}
impl FurnitureFixedBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureFixedBudget {
            chunks: 0.0,
        }
    }
}
impl Default for FurnitureFixedBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureBreakStaticMode {
    Apart,
    Shatter,
}
impl FurnitureBreakStaticMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            FurnitureBreakStaticMode::Apart => "apart",
            FurnitureBreakStaticMode::Shatter => "shatter",
        }
    }
}

/// (TypeScript: inline at src/game/furnblast.ts:379 breakStatic(budget), src/game/furnblast.ts:379)
#[derive(Clone, Copy, Debug)]
pub struct FurnitureBreakStaticBudget {
    pub chunks: f64,
}
impl FurnitureBreakStaticBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureBreakStaticBudget {
            chunks: 0.0,
        }
    }
}
impl Default for FurnitureBreakStaticBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureThrowPieceMode {
    Apart,
    Shatter,
}
impl FurnitureThrowPieceMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            FurnitureThrowPieceMode::Apart => "apart",
            FurnitureThrowPieceMode::Shatter => "shatter",
        }
    }
}

/// (TypeScript: inline at src/game/furnblast.ts:408 throwPiece(budget), src/game/furnblast.ts:408)
#[derive(Clone, Copy, Debug)]
pub struct FurnitureThrowPieceBudget {
    pub chunks: f64,
}
impl FurnitureThrowPieceBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureThrowPieceBudget {
            chunks: 0.0,
        }
    }
}
impl Default for FurnitureThrowPieceBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureSteelMode {
    Apart,
    Shatter,
}
impl FurnitureSteelMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            FurnitureSteelMode::Apart => "apart",
            FurnitureSteelMode::Shatter => "shatter",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureFragmentsMode {
    Apart,
    Shatter,
}
impl FurnitureFragmentsMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            FurnitureFragmentsMode::Apart => "apart",
            FurnitureFragmentsMode::Shatter => "shatter",
        }
    }
}

/// (TypeScript: inline at src/game/furnblast.ts:484 pushable(budget), src/game/furnblast.ts:484)
#[derive(Clone, Copy, Debug)]
pub struct FurniturePushableBudget {
    pub chunks: f64,
}
impl FurniturePushableBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurniturePushableBudget {
            chunks: 0.0,
        }
    }
}
impl Default for FurniturePushableBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:498 breakProp(budget), src/game/furnblast.ts:498)
#[derive(Clone, Copy, Debug)]
pub struct FurnitureBreakPropBudget {
    pub chunks: f64,
}
impl FurnitureBreakPropBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureBreakPropBudget {
            chunks: 0.0,
        }
    }
}
impl Default for FurnitureBreakPropBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/furnblast.ts:571 mouth(budget), src/game/furnblast.ts:571)
#[derive(Clone, Copy, Debug)]
pub struct FurnitureMouthBudget {
    pub chunks: f64,
}
impl FurnitureMouthBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FurnitureMouthBudget {
            chunks: 0.0,
        }
    }
}
impl Default for FurnitureMouthBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FurnitureNoteResetCause {
    Load,
    Reset,
}
impl FurnitureNoteResetCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            FurnitureNoteResetCause::Load => "load",
            FurnitureNoteResetCause::Reset => "reset",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct FurnblastMod {
    /// (src/game/furnblast.ts:117 FURN_LOG)
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

/// (src/game/furnblast.ts:104 FURN_BREAK)
pub const FURN_BREAK: bool = false;

/// (src/game/furnblast.ts:119 HIDE)
pub const HIDE: f64 = 3.0;

/// (src/game/furnblast.ts:121 STEEL_CARCASS_M)
pub const STEEL_CARCASS_M: f64 = 0.25;

/// (src/game/furnblast.ts:124 STEEL_SHEETS_ROOM)
pub const STEEL_SHEETS_ROOM: f64 = 16.0;

/// (src/game/furnblast.ts:128 STEEL_CARCASS_MOST)
pub const STEEL_CARCASS_MOST: f64 = 2.2;

/// (src/game/furnblast.ts:142 NONE_DONE: a table nothing writes)
pub fn none_done() -> crate::furnblast::NONEDONE { crate::furnblast::NONEDONE {  } }
