//! The types of `glasspen`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PaneType {
    Annealed,
    Laminated,
    Tempered,
}
impl PaneType {
    pub const fn as_str(self) -> &'static str {
        match self {
            PaneType::Annealed => "annealed",
            PaneType::Laminated => "laminated",
            PaneType::Tempered => "tempered",
        }
    }
}

/// (TypeScript: PenState, src/game/glasspen.ts:151)
#[derive(Clone, Debug)]
pub struct PenState {
    pub log: Vec<String>,
    pub rev: f64,
}
impl PenState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PenState {
            log: Vec::new(),
            rev: 0.0,
        }
    }
}
impl Default for PenState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneToll, src/game/glasspen.ts:179)
#[derive(Clone, Copy, Debug)]
pub struct PaneToll {
    pub type_: crate::glasspen::PaneType,
    pub n: V3,
    pub cos: f64,
    pub take: f64,
    pub e_out: f64,
    pub hole: bool,
}
impl PaneToll {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneToll {
            type_: crate::glasspen::PaneType::Annealed,
            n: [0.0; 3],
            cos: 0.0,
            take: 0.0,
            e_out: 0.0,
            hole: false,
        }
    }
}
impl Default for PaneToll { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PaneCross, src/game/glasspen.ts:194)
#[derive(Clone, Copy, Debug)]
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
impl PaneCross {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneCross {
            bi: 0.0,
            t: 0.0,
            at: [0.0; 3],
            exit: [0.0; 3],
            dir_in: [0.0; 3],
            dir_out: [0.0; 3],
            e_in: 0.0,
            turn_deg: 0.0,
            thru: 0.0,
            into: 0.0,
            type_: crate::glasspen::PaneType::Annealed,
            n: [0.0; 3],
            cos: 0.0,
            take: 0.0,
            e_out: 0.0,
            hole: false,
        }
    }
}
impl Default for PaneCross { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoundSeg, src/game/glasspen.ts:213)
#[derive(Clone, Copy, Debug)]
pub struct RoundSeg {
    pub o: V3,
    pub d: V3,
    pub t0: f64,
    pub len: f64,
    pub e: f64,
}
impl RoundSeg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundSeg {
            o: [0.0; 3],
            d: [0.0; 3],
            t0: 0.0,
            len: 0.0,
            e: 0.0,
        }
    }
}
impl Default for RoundSeg { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoundPath, src/game/glasspen.ts:214)
#[derive(Clone, Debug)]
pub struct RoundPath {
    pub segs: Vec<crate::glasspen::RoundSeg>,
    pub cross: Vec<crate::glasspen::PaneCross>,
    pub st: Option<crate::glasspen::RoundPathSt>,
    pub len: f64,
}
impl RoundPath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundPath {
            segs: Vec::new(),
            cross: Vec::new(),
            st: None,
            len: 0.0,
        }
    }
}
impl Default for RoundPath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:217 RoundPath.st, src/game/glasspen.ts:217)
#[derive(Clone, Copy, Debug)]
pub struct RoundPathSt {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl RoundPathSt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoundPathSt {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RoundPathSt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:221 StaticHit, src/game/glasspen.ts:221)
#[derive(Clone, Copy, Debug)]
pub struct StaticHit2 {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl StaticHit2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StaticHit2 {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for StaticHit2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShellGlass, src/game/glasspen.ts:251)
#[derive(Clone, Debug)]
pub struct ShellGlass {
    pub glass: JsMap<crate::Num, Vec<f64>>,
}
impl ShellGlass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShellGlass {
            glass: JsMap::new(),
        }
    }
}
impl Default for ShellGlass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShellMark {
    Join,
    More,
    First,
}
impl ShellMark {
    pub const fn as_str(self) -> &'static str {
        match self {
            ShellMark::Join => "join",
            ShellMark::More => "more",
            ShellMark::First => "first",
        }
    }
}

/// (TypeScript: inline at src/game/glasspen.ts:296 armThroughGlass ->, src/game/glasspen.ts:296)
#[derive(Clone, Copy, Debug)]
pub struct ArmThroughGlass {
    pub e: f64,
    pub panes: f64,
}
impl ArmThroughGlass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ArmThroughGlass {
            e: 0.0,
            panes: 0.0,
        }
    }
}
impl Default for ArmThroughGlass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glasspen.ts:308 shotPasses(panes), src/game/glasspen.ts:308)
#[derive(Clone, Copy, Debug)]
pub struct ShotPassesPanesItem {
    pub bi: f64,
    pub t: f64,
}
impl ShotPassesPanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShotPassesPanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for ShotPassesPanesItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GlasspenMod {
    /// (src/game/glasspen.ts:152 states)
    pub states: Option<crate::glasspen::PenState>,
}
impl GlasspenMod {
    pub fn blank() -> Self {
        GlasspenMod {
            states: None,
        }
    }
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

/// (src/game/glasspen.ts:110 OFF_WORD)
pub const OFF_WORD: &str = "/^(0|off|false|no)$/i";

/// (src/game/glasspen.ts:114 LS_KEY)
pub const LS_KEY: &str = "twopointsix.glasspen";

/// (src/game/glasspen.ts:186 PANE_TYPES: a table nothing writes)
pub fn pane_types() -> Vec<String> { alloc::vec![String::from("annealed"), String::from("tempered"), String::from("laminated")] }

/// (src/game/glasspen.ts:191 CROSS)
pub const CROSS: f64 = 25.0;

/// (src/game/glasspen.ts:191 LEG)
pub const LEG: f64 = 9.0;

/// (src/game/glasspen.ts:191 MAX_CROSS)
pub const MAX_CROSS: f64 = 16.0;
