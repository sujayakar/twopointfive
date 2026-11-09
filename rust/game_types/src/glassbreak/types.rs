//! The types of `glassbreak`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct GlassBreak {
    pub sheet_list: Option<Vec<f64>>,
    pub view_of: Option<crate::glassbreak::GlassBreakViewOf>,
    pub gust_dirty: bool,
    pub gusts: JsMap<crate::Num, WindWindSourceId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassBreakViewOf {
    pub kind: crate::glassbreak::GlassBreakViewOfKind,
    pub from: V3,
    pub size: crate::glassbreak::GlassBlastOpts,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GlassBreakViewOfKind {
    #[default] Blown,
    Wary,
    Counted,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassBlastOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
    pub who: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassImpact {
    pub pane: bool,
    pub live: bool,
    pub type_: crate::glasspen::PaneType,
    pub thickness: f64,
    pub was_open: bool,
    pub punched: bool,
    pub diced: bool,
    pub released: f64,
    pub cracked: bool,
}

#[derive(Clone, Debug, Default)]
pub struct GlassFragment {
    pub kind: crate::glassbreak::GlassFragmentKind,
    pub outline: Vec<V3>,
    pub centre: V3,
    pub normal: V3,
    pub area: f64,
    pub velocity: V3,
    pub spin: V3,
    pub grain: f64,
    pub who: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GlassFragmentKind {
    #[default] Piece,
    Sheet,
    Dagger,
    Clump,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassFragmentFall {
    pub t: f64,
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Clump {
    pub k: f64,
    pub key: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Dice {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Pending {
}

#[derive(Clone, Debug, Default)]
pub struct PaneState {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PaneFrame {
    pub key: f64,
    pub u_axis: f64,
    pub ht: f64,
    pub box_: BoxId,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassbreakModGBHOST {
    pub shards: f64,
    pub chips: f64,
    pub refused: f64,
    pub hooked: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TALLYAT {
    pub panes: Option<f64>,
    pub dropped: Option<f64>,
    pub impacts: Option<f64>,
    pub sites: Option<f64>,
    pub runs: Option<f64>,
    pub shards: Option<f64>,
    pub diced: Option<f64>,
    pub clumps: Option<f64>,
    pub sheets: Option<f64>,
    pub blasts: Option<f64>,
    pub thinks: Option<f64>,
    pub raked: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct GlassBreakLetGoF {
    pub type_: crate::glasspen::PaneType,
    pub kind: crate::glassbreak::GlassBreakLetGoFKind,
    pub outline: Vec<V3>,
    pub centre: V3,
    pub normal: V3,
    pub thickness: f64,
    pub area: f64,
    pub velocity: V3,
    pub spin: V3,
    pub tint: V3,
    pub grain: f64,
    pub cause: crate::glassbreak::GlassBreakLetGoFCause,
    pub who: f64,
    pub seed: f64,
    pub fall: Option<crate::glassbreak::GlassFragmentFall>,
    pub landed: Option<bool>,
    pub key: f64,
    pub heard: bool,
    pub bi: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GlassBreakLetGoFKind {
    #[default] Piece,
    Sheet,
    Dagger,
    Clump,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GlassBreakLetGoFCause {
    #[default] Blast,
    Hit,
    Dice,
    Support,
    Rake,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassBreakSeeThroughView {
    Blown,
    Wary,
    Counted,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassBreakPaneInfo {
    pub open_fraction: f64,
    pub gone: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct GlassbreakMod {
    /// (src/game/glassbreak.ts:193 GB_HOST)
    #[is(crate::glassbreak::GlassbreakModGBHOST { shards: 0.0, chips: 0.0, refused: 0.0, hooked: 0.0 })] pub gb_host: crate::glassbreak::GlassbreakModGBHOST,
}

/// (src/game/glassbreak.ts:195 TALLY_AT: a table nothing writes)
pub fn tally_at() -> crate::glassbreak::TALLYAT { crate::glassbreak::TALLYAT { panes: Some(0.0), dropped: Some(4.0), impacts: Some(1.0), sites: Some(2.0), runs: Some(3.0), shards: None, diced: Some(5.0), clumps: Some(6.0), sheets: Some(7.0), blasts: Some(8.0), thinks: Some(9.0), raked: Some(10.0) } }

/// (src/game/glassbreak.ts:200 GB_HEADER_ROWS)
pub const GB_HEADER_ROWS: f64 = 4.0;

/// (src/game/glassbreak.ts:200 GB_SITE_ROWS)
pub const GB_SITE_ROWS: f64 = 4.0;

/// (src/game/glassbreak.ts:207 NO_TYPE)
pub const NO_TYPE: f64 = 9.0;

/// (src/game/glassbreak.ts:207 NO_OVER)
pub const NO_OVER: f64 = 3.0;

/// (src/game/glassbreak.ts:207 SKY_UNKNOWN)
pub const SKY_UNKNOWN: f64 = 2.0;

/// (src/game/glassbreak.ts:209 OUT_LEN)
pub const OUT_LEN: f64 = 32.0;

/// (src/game/glassbreak.ts:209 O_PANE)
pub const O_PANE: f64 = 0.0;

/// (src/game/glassbreak.ts:209 O_TYPE)
pub const O_TYPE: f64 = 1.0;

/// (src/game/glassbreak.ts:209 O_KIND)
pub const O_KIND: f64 = 2.0;

/// (src/game/glassbreak.ts:209 O_PTS)
pub const O_PTS: f64 = 3.0;

/// (src/game/glassbreak.ts:209 O_NPTS)
pub const O_NPTS: f64 = 4.0;

/// (src/game/glassbreak.ts:209 O_CX)
pub const O_CX: f64 = 5.0;

/// (src/game/glassbreak.ts:209 O_NX)
pub const O_NX: f64 = 8.0;

/// (src/game/glassbreak.ts:209 O_THICK)
pub const O_THICK: f64 = 11.0;

/// (src/game/glassbreak.ts:209 O_AREA)
pub const O_AREA: f64 = 12.0;

/// (src/game/glassbreak.ts:209 O_VX)
pub const O_VX: f64 = 13.0;

/// (src/game/glassbreak.ts:209 O_SX)
pub const O_SX: f64 = 16.0;

/// (src/game/glassbreak.ts:209 O_GRAIN)
pub const O_GRAIN: f64 = 19.0;

/// (src/game/glassbreak.ts:209 O_CAUSE)
pub const O_CAUSE: f64 = 20.0;

/// (src/game/glassbreak.ts:209 O_WHO)
pub const O_WHO: f64 = 21.0;

/// (src/game/glassbreak.ts:209 O_SEED)
pub const O_SEED: f64 = 22.0;

/// (src/game/glassbreak.ts:209 O_KEY)
pub const O_KEY: f64 = 23.0;

/// (src/game/glassbreak.ts:209 O_HEARD)
pub const O_HEARD: f64 = 24.0;

/// (src/game/glassbreak.ts:209 O_FALL_T)
pub const O_FALL_T: f64 = 25.0;

/// (src/game/glassbreak.ts:209 O_FALL_X)
pub const O_FALL_X: f64 = 26.0;

/// (src/game/glassbreak.ts:209 O_LANDED)
pub const O_LANDED: f64 = 29.0;

/// (src/game/glassbreak.ts:216 GW_COLS)
pub const GW_COLS: f64 = 2.0;

/// (src/game/glassbreak.ts:216 GW_ROWS)
pub const GW_ROWS: f64 = 3.0;

/// (src/game/glassbreak.ts:216 GW_DSEED)
pub const GW_DSEED: f64 = 4.0;

/// (src/game/glassbreak.ts:216 GW_N)
pub const GW_N: f64 = 5.0;

/// (src/game/glassbreak.ts:216 GW_SEED)
pub const GW_SEED: f64 = 13.0;

/// (src/game/glassbreak.ts:216 GW_OPEN)
pub const GW_OPEN: f64 = 21.0;

/// (src/game/glassbreak.ts:216 GW_FLAGS)
pub const GW_FLAGS: f64 = 1.0;

/// (src/game/glassbreak.ts:216 G_GONE)
pub const G_GONE: f64 = 1.0;

/// (src/game/glassbreak.ts:216 G_DICE)
pub const G_DICE: f64 = 2.0;

/// (src/game/glassbreak.ts:216 G_RUN)
pub const G_RUN: f64 = 4.0;

/// (src/game/glassbreak.ts:216 G_SITE)
pub const G_SITE: f64 = 9.0;

/// (src/game/glassbreak.ts:216 G_DICE_AT)
pub const G_DICE_AT: f64 = 72.0;
