//! The types of `rescueend`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RescueEnd, src/game/rescueend.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct RescueEnd {
    pub lead: Option<CharId>,
    pub led: bool,
    pub clear_at: f64,
}
impl RescueEnd {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        RescueEnd {
            lead: None,
            led: false,
            clear_at: f64::NAN,
        }
    }
}
impl Default for RescueEnd { fn default() -> Self { Self::blank() } }

/// (TypeScript: RescueEndSnap, src/game/rescueend.ts:46)
#[derive(Clone, Copy, Debug)]
pub struct RescueEndSnap {
    pub lead: Option<f64>,
    pub clear_at: f64,
    pub led: bool,
}
impl RescueEndSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueEndSnap {
            lead: None,
            clear_at: 0.0,
            led: false,
        }
    }
}
impl Default for RescueEndSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescueend.ts:69 rows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct RescueEndRowsRowOver {
    pub kind: Option<crate::rescueend::RescueEndRowsRowOverKind>,
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
impl RescueEndRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueEndRowsRowOver {
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
impl Default for RescueEndRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueEndRowsRowOverKind {
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
impl RescueEndRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueEndRowsRowOverKind::Cam => "cam",
            RescueEndRowsRowOverKind::Guard => "guard",
            RescueEndRowsRowOverKind::Breaker => "breaker",
            RescueEndRowsRowOverKind::Door => "door",
            RescueEndRowsRowOverKind::Light => "light",
            RescueEndRowsRowOverKind::Body => "body",
            RescueEndRowsRowOverKind::Objective => "objective",
            RescueEndRowsRowOverKind::Held => "held",
            RescueEndRowsRowOverKind::Wall => "wall",
            RescueEndRowsRowOverKind::Switch => "switch",
            RescueEndRowsRowOverKind::Charge => "charge",
            RescueEndRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueEndWon {
    Out,
    Cleared,
}
impl RescueEndWon {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueEndWon::Out => "out",
            RescueEndWon::Cleared => "cleared",
        }
    }
}

/// (TypeScript: inline at src/game/rescueend.ts:107 objective ->, src/game/rescueend.ts:107)
#[derive(Clone, Debug)]
pub struct RescueEndObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl RescueEndObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueEndObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for RescueEndObjective { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescueend.ts:124 rescueEndRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct RescueEndRows2RowOver {
    pub kind: Option<crate::rescueend::RescueEndRows2RowOverKind>,
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
impl RescueEndRows2RowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueEndRows2RowOver {
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
impl Default for RescueEndRows2RowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueEndRows2RowOverKind {
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
impl RescueEndRows2RowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueEndRows2RowOverKind::Cam => "cam",
            RescueEndRows2RowOverKind::Guard => "guard",
            RescueEndRows2RowOverKind::Breaker => "breaker",
            RescueEndRows2RowOverKind::Door => "door",
            RescueEndRows2RowOverKind::Light => "light",
            RescueEndRows2RowOverKind::Body => "body",
            RescueEndRows2RowOverKind::Objective => "objective",
            RescueEndRows2RowOverKind::Held => "held",
            RescueEndRows2RowOverKind::Wall => "wall",
            RescueEndRows2RowOverKind::Switch => "switch",
            RescueEndRows2RowOverKind::Charge => "charge",
            RescueEndRows2RowOverKind::Cancel => "cancel",
        }
    }
}
