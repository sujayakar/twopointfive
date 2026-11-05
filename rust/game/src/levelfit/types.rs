//! The types of `levelfit`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: LevelFit, src/game/levelfit.ts:59)
#[derive(Clone, Copy, Debug)]
pub struct LevelFit {
    pub doors: f64,
    pub things: f64,
    pub parts: f64,
    pub sheets: f64,
    pub fixtures: f64,
    pub cameras: f64,
    pub lasers: f64,
    pub beacons: f64,
    pub lamps: f64,
}
impl LevelFit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelFit {
            doors: 0.0,
            things: 0.0,
            parts: 0.0,
            sheets: 0.0,
            fixtures: 0.0,
            cameras: 0.0,
            lasers: 0.0,
            beacons: 0.0,
            lamps: 0.0,
        }
    }
}
impl Default for LevelFit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreakRoom, src/game/levelfit.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct BreakRoom {
    pub boxes: f64,
    pub items: f64,
    pub cuts: f64,
}
impl BreakRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRoom {
            boxes: 0.0,
            items: 0.0,
            cuts: 0.0,
        }
    }
}
impl Default for BreakRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FitRow, src/game/levelfit.ts:102)
#[derive(Clone, Debug)]
pub struct FitRow {
    pub row: String,
    pub what: String,
    pub has: f64,
    pub cap: f64,
    pub table: Option<String>,
    pub play: Option<bool>,
}
impl FitRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FitRow {
            row: String::new(),
            what: String::new(),
            has: 0.0,
            cap: 0.0,
            table: None,
            play: None,
        }
    }
}
impl Default for FitRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Stood, src/game/levelfit.ts:104)
#[derive(Clone, Debug)]
pub struct Stood {
    pub level: crate::levelfit::StoodLevel,
    pub col: crate::levelfit::StoodCol,
    pub props: Option<crate::levelfit::StoodProps>,
    pub player: Option<PlayerId>,
    pub guards: Option<Vec<GuardId>>,
    pub civilians: Option<Vec<GuardId>>,
    pub seats: Option<Vec<SeatId>>,
}
impl Stood {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stood {
            level: crate::levelfit::StoodLevel::zeroed(),
            col: crate::levelfit::StoodCol::zeroed(),
            props: None,
            player: None,
            guards: Some(Vec::new()),
            civilians: Some(Vec::new()),
            seats: None,
        }
    }
}
impl Default for Stood { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:105 Stood.level, src/game/levelfit.ts:105 Stood.level)
#[derive(Clone, Debug)]
pub struct StoodLevel {
    pub doors: Vec<crate::Opaque>,
    pub props: Vec<crate::levelfit::StoodLevelPropsItem>,
    pub boxes: Vec<BoxId>,
    pub lights: Option<Vec<crate::Opaque>>,
    pub switchables: Option<Vec<crate::levelfit::StoodLevelSwitchablesItem>>,
    pub beacons: Option<Vec<crate::Opaque>>,
    pub devices: Option<Vec<crate::levelfit::StoodLevelDevicesItem>>,
    pub civilians: Option<Vec<crate::Opaque>>,
}
impl StoodLevel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StoodLevel {
            doors: Vec::new(),
            props: Vec::new(),
            boxes: Vec::new(),
            lights: None,
            switchables: None,
            beacons: None,
            devices: None,
            civilians: None,
        }
    }
}
impl Default for StoodLevel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:62 StoodLevel.props, src/game/levelfit.ts:62)
#[derive(Clone, Debug)]
pub struct StoodLevelPropsItem {
    pub parts: Vec<crate::Opaque>,
}
impl StoodLevelPropsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StoodLevelPropsItem {
            parts: Vec::new(),
        }
    }
}
impl Default for StoodLevelPropsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:62 StoodLevel.switchables, src/game/levelfit.ts:62)
#[derive(Clone, Debug)]
pub struct StoodLevelSwitchablesItem {
    pub boxes: Vec<f64>,
    pub group: String,
}
impl StoodLevelSwitchablesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StoodLevelSwitchablesItem {
            boxes: Vec::new(),
            group: String::new(),
        }
    }
}
impl Default for StoodLevelSwitchablesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:62 StoodLevel.devices, src/game/levelfit.ts:62)
#[derive(Clone, Debug)]
pub struct StoodLevelDevicesItem {
    pub kind: String,
}
impl StoodLevelDevicesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StoodLevelDevicesItem {
            kind: String::new(),
        }
    }
}
impl Default for StoodLevelDevicesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:106 Stood.col, src/game/levelfit.ts:106)
#[derive(Clone, Debug)]
pub struct StoodCol {
    pub boxes: Vec<BoxId>,
    pub dynamic_boxes: Vec<BoxId>,
}
impl StoodCol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StoodCol {
            boxes: Vec::new(),
            dynamic_boxes: Vec::new(),
        }
    }
}
impl Default for StoodCol { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:107 Stood.props, src/game/levelfit.ts:107)
#[derive(Clone, Debug)]
pub struct StoodProps {
    pub all: Vec<crate::levelfit::StoodPropsAllItem>,
}
impl StoodProps {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StoodProps {
            all: Vec::new(),
        }
    }
}
impl Default for StoodProps { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:107 StoodProps.all, src/game/levelfit.ts:107)
#[derive(Clone, Debug)]
pub struct StoodPropsAllItem {
    pub boxes: Vec<crate::Opaque>,
}
impl StoodPropsAllItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StoodPropsAllItem {
            boxes: Vec::new(),
        }
    }
}
impl Default for StoodPropsAllItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:54 handOverMost ->, src/game/levelfit.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct HandOverMost {
    pub door_men: f64,
    pub prop_men: f64,
}
impl HandOverMost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HandOverMost {
            door_men: 0.0,
            prop_men: 0.0,
        }
    }
}
impl Default for HandOverMost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/levelfit.ts:62 levelCounts(level), src/game/levelfit.ts:62)
#[derive(Clone, Debug)]
pub struct LevelCountsLevel {
    pub doors: Vec<crate::Opaque>,
    pub props: Vec<crate::levelfit::StoodLevelPropsItem>,
    pub boxes: Vec<BoxId>,
    pub lights: Option<Vec<crate::Opaque>>,
    pub switchables: Option<Vec<crate::levelfit::StoodLevelSwitchablesItem>>,
    pub beacons: Option<Vec<crate::Opaque>>,
    pub devices: Option<Vec<crate::levelfit::StoodLevelDevicesItem>>,
}
impl LevelCountsLevel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelCountsLevel {
            doors: Vec::new(),
            props: Vec::new(),
            boxes: Vec::new(),
            lights: None,
            switchables: None,
            beacons: None,
            devices: None,
        }
    }
}
impl Default for LevelCountsLevel { fn default() -> Self { Self::zeroed() } }
