//! The types of `glasspen`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum PaneType {
    #[default] Annealed,
    Laminated,
    Tempered,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PenState {
    pub log: Vec<String>,
    pub rev: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PaneCross {
    pub bi: f64,
    pub t: f64,
    pub at: V3,
    pub exit: V3,
    pub dir_in: V3,
    pub dir_out: V3,
    pub e_in: f64,
    pub turn_deg: f64,
    pub thru: f64,
    pub into: f64,
    pub type_: crate::glasspen::PaneType,
    pub n: V3,
    pub cos: f64,
    pub take: f64,
    pub e_out: f64,
    pub hole: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RoundSeg {
    pub o: V3,
    pub d: V3,
    pub t0: f64,
    pub len: f64,
    pub e: f64,
}

#[derive(Clone, Debug, Default)]
pub struct RoundPath {
    pub segs: Vec<crate::glasspen::RoundSeg>,
    pub cross: Vec<crate::glasspen::PaneCross>,
    pub st: Option<crate::glasspen::RoundPathSt>,
    pub len: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RoundPathSt {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}

#[derive(Clone, Debug, Default)]
pub struct ShellGlass {
    pub glass: JsMap<crate::Num, Vec<f64>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShellMark {
    Join,
    More,
    First,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ArmThroughGlass {
    pub e: f64,
    pub panes: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct GlasspenMod {
    /// (src/game/glasspen.ts:152 states)
    pub states: Option<crate::glasspen::PenState>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GlasspenStats {
    pub crossed: i32,
    pub spent: i32,
    pub via_hole: i32,
    pub via_arm: i32,
    pub shot_asked: i32,
    pub shot_through: i32,
    pub shot_refused: i32,
    pub shell_joined: i32,
}

/// (src/game/glasspen.ts:186 PANE_TYPES: a table nothing writes)
pub fn pane_types() -> Vec<String> { alloc::vec![String::from("annealed"), String::from("tempered"), String::from("laminated")] }

/// (src/game/glasspen.ts:191 CROSS)
pub const CROSS: f64 = 25.0;

/// (src/game/glasspen.ts:191 MAX_CROSS)
pub const MAX_CROSS: f64 = 16.0;
