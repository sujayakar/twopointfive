//! The types of `planlay`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: LayState, src/game/planlay.ts:54)
#[derive(Clone, Debug)]
pub struct LayState {
    pub laid: Vec<crate::planlay::LaidOrder>,
    pub kept: Option<Vec<crate::planlay::LaidOrder>>,
    pub kept_for: String,
    pub try_key: String,
    pub started: bool,
    pub sending: bool,
    pub restored_t: f64,
}
impl LayState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LayState {
            laid: Vec::new(),
            kept: None,
            kept_for: String::new(),
            try_key: String::new(),
            started: false,
            sending: false,
            restored_t: 0.0,
        }
    }
}
impl Default for LayState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LaidOrder, src/game/planlay.ts:48)
#[derive(Clone, Debug)]
pub struct LaidOrder {
    pub id: String,
    pub who: Vec<String>,
    pub kind: crate::planlay::LaidOrderKind,
    pub at: V3,
    pub door: Option<f64>,
    pub man: Option<String>,
    pub found: Option<crate::planlay::LaidOrderFound>,
    pub pick: Option<V3>,
    pub t: f64,
}
impl LaidOrder {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaidOrder {
            id: String::new(),
            who: Vec::new(),
            kind: crate::planlay::LaidOrderKind::None,
            at: [0.0; 3],
            door: None,
            man: None,
            found: None,
            pick: None,
            t: 0.0,
        }
    }
}
impl Default for LaidOrder { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LaidOrderKind {
    None,
    Door,
    Floor,
}
impl LaidOrderKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LaidOrderKind::None => "none",
            LaidOrderKind::Door => "door",
            LaidOrderKind::Floor => "floor",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LaidOrderFound {
    Wall,
    Thing,
}
impl LaidOrderFound {
    pub const fn as_str(self) -> &'static str {
        match self {
            LaidOrderFound::Wall => "wall",
            LaidOrderFound::Thing => "thing",
        }
    }
}

/// (TypeScript: LaySnap, src/game/planlay.ts:55)
#[derive(Clone, Debug)]
pub struct LaySnap {
    pub laid: Vec<crate::planlay::LaidOrder>,
    pub kept: Option<Vec<crate::planlay::LaidOrder>>,
    pub kept_for: String,
    pub started: bool,
}
impl LaySnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaySnap {
            laid: Vec::new(),
            kept: None,
            kept_for: String::new(),
            started: false,
        }
    }
}
impl Default for LaySnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LaidWhere, src/game/planlay.ts:50)
#[derive(Clone, Debug)]
pub struct LaidWhere {
    pub kind: crate::planlay::LaidWhereKind,
    pub at: V3,
    pub pick: Option<V3>,
    pub door: Option<f64>,
    pub man: Option<String>,
    pub found: Option<crate::planlay::LaidWhereFound>,
}
impl LaidWhere {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaidWhere {
            kind: crate::planlay::LaidWhereKind::None,
            at: [0.0; 3],
            pick: None,
            door: None,
            man: None,
            found: None,
        }
    }
}
impl Default for LaidWhere { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LaidWhereKind {
    None,
    Door,
    Floor,
}
impl LaidWhereKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LaidWhereKind::None => "none",
            LaidWhereKind::Door => "door",
            LaidWhereKind::Floor => "floor",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LaidWhereFound {
    Wall,
    Thing,
}
impl LaidWhereFound {
    pub const fn as_str(self) -> &'static str {
        match self {
            LaidWhereFound::Wall => "wall",
            LaidWhereFound::Thing => "thing",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum TargetOf {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum LayTarget {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: inline at src/game/planlay.ts:170 layNote ->, src/game/planlay.ts:170)
#[derive(Clone, Debug)]
pub struct LayNoteItem {
    pub key: String,
    pub text: String,
}
impl LayNoteItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LayNoteItem {
            key: String::new(),
            text: String::new(),
        }
    }
}
impl Default for LayNoteItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct PlanlayMod {
    /// (src/game/planlay.ts:60 WORD_ROWS)
    pub word_rows: JsSet<String>,
}
impl PlanlayMod {
    pub fn blank() -> Self {
        PlanlayMod {
            word_rows: { let mut s = JsSet::new(); for w in ["go", "thrWord"] { s.add(String::from(w)); } s },
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct PlanlayStats {
    pub laid: i32,
    pub replaced: i32,
    pub sent: i32,
    pub refused: i32,
    pub dropped: i32,
    pub relaid: i32,
    pub struck: i32,
}
