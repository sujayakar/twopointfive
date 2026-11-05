//! The types of `lightswitch`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WallSwitch, src/game/lightswitch.ts:75)
#[derive(Clone, Debug)]
pub struct WallSwitch {
    pub id: f64,
    pub pos: V3,
    pub n: V3,
    pub stand: V3,
    pub floor: V3,
    pub door: DoorId,
    pub room: f64,
    pub targets: Vec<FixtureId>,
}
impl WallSwitch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSwitch {
            id: 0.0,
            pos: [0.0; 3],
            n: [0.0; 3],
            stand: [0.0; 3],
            floor: [0.0; 3],
            door: DoorId(0),
            room: 0.0,
            targets: Vec::new(),
        }
    }
}
impl Default for WallSwitch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Hand, src/game/lightswitch.ts:88)
#[derive(Clone, Debug)]
pub struct Hand {
    pub c: CharId,
    pub sw: crate::lightswitch::WallSwitch,
    pub at: f64,
    pub dir: f64,
    pub dur: f64,
    pub plant: bool,
}
impl Hand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Hand {
            c: CharId(0),
            sw: crate::lightswitch::WallSwitch::zeroed(),
            at: 0.0,
            dir: 0.0,
            dur: 0.0,
            plant: false,
        }
    }
}
impl Default for Hand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: St, src/game/lightswitch.ts:89)
#[derive(Clone, Debug)]
pub struct St {
    pub all: Vec<crate::lightswitch::WallSwitch>,
    pub list: Vec<crate::lightswitch::WallSwitch>,
    pub hands: Vec<crate::lightswitch::Hand>,
    pub serial: f64,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            all: Vec::new(),
            list: Vec::new(),
            hands: Vec::new(),
            serial: 0.0,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Plate, src/game/lightswitch.ts:120)
#[derive(Clone, Copy, Debug)]
pub struct Plate {
    pub door: DoorId,
    pub n: V3,
    pub pos: V3,
    pub floor: V3,
    pub room: f64,
    pub stand: V3,
}
impl Plate {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Plate {
            door: DoorId(0),
            n: [0.0; 3],
            pos: [0.0; 3],
            floor: [0.0; 3],
            room: 0.0,
            stand: [0.0; 3],
        }
    }
}
impl Default for Plate { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lightswitch.ts:231 switchHand ->, src/game/lightswitch.ts:231)
#[derive(Clone, Copy, Debug)]
pub struct SwitchHand {
    pub p: V3,
    pub n: V3,
}
impl SwitchHand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SwitchHand {
            p: [0.0; 3],
            n: [0.0; 3],
        }
    }
}
impl Default for SwitchHand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/lightswitch.ts:258 switchRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct SwitchRowsRowOver {
    pub kind: Option<crate::lightswitch::SwitchRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}
impl SwitchRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SwitchRowsRowOver {
            kind: None,
            pos: None,
            line1: None,
            line2: None,
            in_reach: None,
            off: None,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for SwitchRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwitchRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}
impl SwitchRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SwitchRowsRowOverKind::Cam => "cam",
            SwitchRowsRowOverKind::Guard => "guard",
            SwitchRowsRowOverKind::Breaker => "breaker",
            SwitchRowsRowOverKind::Door => "door",
            SwitchRowsRowOverKind::Light => "light",
            SwitchRowsRowOverKind::Body => "body",
            SwitchRowsRowOverKind::Objective => "objective",
            SwitchRowsRowOverKind::Held => "held",
            SwitchRowsRowOverKind::Wall => "wall",
            SwitchRowsRowOverKind::Switch => "switch",
            SwitchRowsRowOverKind::Charge => "charge",
            SwitchRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct LightswitchMod {
    /// (src/game/lightswitch.ts:90 states)
    pub states: Option<crate::lightswitch::St>,
}
impl LightswitchMod {
    pub fn blank() -> Self {
        LightswitchMod {
            states: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct LightswitchStats {
    pub plates: i32,
    pub flips: i32,
    pub guard_on: i32,
    pub events: i32,
    pub gone: i32,
}

/// (src/game/lightswitch.ts:121 WALL_TOP)
pub const WALL_TOP: f64 = 2.2;

/// (src/game/lightswitch.ts:200 NONE: a table nothing writes)
pub fn none() -> Vec<crate::lightswitch::WallSwitch> { Vec::new() }

/// (src/game/lightswitch.ts:303 PLATE: a table nothing writes)
pub fn plate() -> V3 { [0.62, 0.6, 0.55] }

/// (src/game/lightswitch.ts:303 ROCKER: a table nothing writes)
pub fn rocker() -> V3 { [0.74, 0.72, 0.66] }

/// (src/game/lightswitch.ts:303 GLOW: a table nothing writes)
pub fn glow() -> V3 { [1.6, 0.8, 0.16] }

/// (src/game/lightswitch.ts:304 SW_KEY)
pub const SW_KEY: f64 = 4194304.0;
