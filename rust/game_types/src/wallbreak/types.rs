//! The types of `wallbreak`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// who is told of a row of the change feed, on the frame it is filed: WallReact = the game's one WallReact takes the row into its queue
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum ChangeListener {
    WallReact,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum BKind {
    #[default] Wall,
    Cover,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum LevelChangeKind {
    #[default] Door,
    Cover,
    Hole,
    Port,
    Reset,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LevelChange {
    pub serial: f64,
    pub t: f64,
    pub kind: crate::wallbreak::LevelChangeKind,
    pub cause: crate::wallbreak::LevelChangeCause,
    pub id: f64,
    pub pos: V3,
    pub rect: [f64; 4],
    pub opened: Vec<f64>,
    pub walkable: bool,
    pub who: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlastOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
    pub who: Option<f64>,
    pub chaos: Option<bool>,
    pub breach: Option<f64>,
    pub hung: Option<crate::wallbreak::BlastOptsHung>,
    /// how far it reaches glass and how hard, if not as it reaches the rest
    pub glass: Option<[f64; 2]>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlastOptsHung {
    pub wall: f64,
    pub r: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallOpening {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub standing: Vec<BoxId>,
    pub left: Vec<BoxId>,
    pub cells: f64,
    pub hole: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlastResult {
    pub walls: f64,
    pub cover: f64,
    pub doors: f64,
    pub opened: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlastEvent {
    pub at: V3,
    pub radius: f64,
    pub who: f64,
    /// how far it reaches glass and how hard
    pub glass: [f64; 2],
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Breakable {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: crate::scene_materials::Surf,
    pub box_: BoxId,
    pub flags0: f64,
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub yb: Vec<f64>,
    pub open: Vec<u8>,
    pub dmg: Vec<f32>,
    pub slots: Vec<f64>,
    pub live: f64,
    pub n_open: f64,
    pub hard: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AABB {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallBreak {
    pub authored: f64,
    pub serial: f64,
    pub changes: Vec<crate::wallbreak::LevelChange>,
    pub listeners: Vec<crate::wallbreak::ChangeListener>,
    pub list: Vec<WallbreakBreakableId>,
    pub owner: JsMap<crate::Num, WallbreakBreakableId>,
    pub hidden: JsMap<crate::Num, f64>,
    #[is(1.0)] pub counter: f64,
    pub dirty: bool,
    pub fixture_boxes: JsSet<crate::Num>,
    pub ghosts: Vec<f64>,
    pub wear: crate::wallwear::WallWear,
    pub skins: crate::wallskin::WallSkin,
    pub room: crate::levelfit::BreakRoom,
    pub shell_gaps: JsSet<crate::Num>,
    pub hung_now: Option<crate::wallbreak::WallBreakHungNow>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallBreakHungNow {
    pub wall: f64,
    pub at: V3,
    pub n: V3,
    pub set: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallBreakBreakableAt {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub open: f64,
    pub cells: f64,
    pub hard: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallBreakAllItem {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: crate::scene_materials::Surf,
    pub box_: BoxId,
    pub open: f64,
    pub cells: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallBreakRectBox {
    pub c: V3,
    pub h: V3,
}

#[derive(Clone, Debug, Default)]
pub struct WallBreakNoteC {
    pub kind: crate::wallbreak::LevelChangeKind,
    pub cause: crate::wallbreak::LevelChangeCause,
    pub who: f64,
    pub pos: V3,
    pub id: f64,
    pub rect: [f64; 4],
    pub opened: Vec<f64>,
    pub walkable: bool,
}

#[derive(Clone, Debug, Default)]
pub struct WallBreakPlanBlastItem {
    pub b: WallbreakBreakableId,
    pub cells: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct WallBreakPlanOfItem {
    pub b: WallbreakBreakableId,
    pub cells: Vec<f64>,
    pub d: f64,
}

pub const HIDE: f64 = 3.0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum LevelChangeCause {
    #[default] Blast,
    Fire,
    Load,
    Reset,
    Collapse,
}
