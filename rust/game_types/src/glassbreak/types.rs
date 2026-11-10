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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassBlastOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
    pub who: Option<f64>,
}

/// what a fragment is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum FragmentKind {
    #[default] Piece,
    Dagger,
    Clump,
    Sheet,
}

/// what made it go
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum FragmentCause {
    #[default] Hit,
    Support,
    Blast,
    Dice,
    Rake,
}

/// where and when it comes down
#[derive(Clone, Copy, Debug, Default)]
pub struct FragmentFall {
    pub t: f64,
    pub pos: V3,
}

/// A FRAGMENT A PANE LET GO, as its one way out says it
#[derive(Clone, Debug, Default)]
pub struct Fragment {
    pub bi: f64,
    pub type_: crate::glasspen::PaneType,
    pub kind: FragmentKind,
    pub cause: FragmentCause,
    pub outline: Vec<V3>,
    pub centre: V3,
    pub normal: V3,
    pub thickness: f64,
    pub area: f64,
    pub velocity: V3,
    pub spin: V3,
    pub tint: V3,
    pub grain: f64,
    pub who: f64,
    pub seed: f64,
    pub key: f64,
    /// its going is the pane's crack and hearing event (one a pane a gap)
    pub heard: bool,
    pub fall: Option<FragmentFall>,
    /// its landing is scheduled already
    pub landed: bool,
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
    pub hooked: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassBreakPaneInfo {
    pub open_fraction: f64,
    pub gone: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct GlassbreakMod {
    #[is(crate::glassbreak::GlassbreakModGBHOST { shards: 0.0, hooked: 0.0 })] pub gb_host: crate::glassbreak::GlassbreakModGBHOST,
}

pub const NO_TYPE: f64 = 9.0;

pub const NO_OVER: f64 = 3.0;

pub const SKY_UNKNOWN: f64 = 2.0;

pub const GW_FLAGS: f64 = 1.0;

pub const G_GONE: f64 = 1.0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum GlassBreakViewOfKind {
    #[default] Blown,
    Wary,
    Counted,
}
