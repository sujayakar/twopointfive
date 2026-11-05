//! The types of `wallreact`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Breach, src/game/wallreact.ts:111)
#[derive(Clone, Debug)]
pub struct Breach {
    pub key: String,
    pub wall: f64,
    pub c0: [f64; 2],
    pub u: [f64; 2],
    pub n: [f64; 2],
    pub lo: f64,
    pub hi: f64,
    pub at: V3,
    pub half: f64,
    pub into: Option<WallreactBreachId>,
    pub walk: bool,
    pub t: f64,
    pub cause: crate::wallreact::BreachCause,
    pub who: f64,
    pub blast_by: Option<f64>,
    pub ops: f64,
    pub men: JsMap<GuardId, f64>,
    pub said: JsSet<crate::teams::Team>,
    pub tell: Vec<crate::wallreact::BreachTellItem>,
    pub opening: Option<CoverOpeningId>,
}
impl Breach {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Breach {
            key: String::new(),
            wall: 0.0,
            c0: [0.0; 2],
            u: [0.0; 2],
            n: [0.0; 2],
            lo: 0.0,
            hi: 0.0,
            at: [0.0; 3],
            half: 0.0,
            into: None,
            walk: false,
            t: 0.0,
            cause: crate::wallreact::BreachCause::Blast,
            who: 0.0,
            blast_by: None,
            ops: 0.0,
            men: JsMap::new(),
            said: JsSet::new(),
            tell: Vec::new(),
            opening: None,
        }
    }
}
impl Default for Breach { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BreachCause {
    Blast,
    Fire,
    Load,
    Reset,
    Collapse,
}
impl BreachCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            BreachCause::Blast => "blast",
            BreachCause::Fire => "fire",
            BreachCause::Load => "load",
            BreachCause::Reset => "reset",
            BreachCause::Collapse => "collapse",
        }
    }
}

/// (TypeScript: inline at src/game/wallreact.ts:123 Breach.tell, src/game/wallreact.ts:123)
#[derive(Clone, Copy, Debug)]
pub struct BreachTellItem {
    pub by: GuardId,
    pub at: f64,
}
impl BreachTellItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachTellItem {
            by: GuardId(0),
            at: 0.0,
        }
    }
}
impl Default for BreachTellItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Mind, src/game/wallreact.ts:127)
#[derive(Clone, Copy, Debug)]
pub struct Mind {
    pub ev: f64,
    pub line: bool,
    pub line_t: f64,
    pub t: f64,
}
impl Mind {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mind {
            ev: 0.0,
            line: false,
            line_t: 0.0,
            t: 0.0,
        }
    }
}
impl Default for Mind { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Gone, src/game/wallreact.ts:129)
#[derive(Clone, Copy, Debug)]
pub struct Gone {
    pub rect: [f64; 4],
    pub b: Option<WallreactBreachId>,
    pub who: f64,
}
impl Gone {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Gone {
            rect: [0.0; 4],
            b: None,
            who: 0.0,
        }
    }
}
impl Default for Gone { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Pend, src/game/wallreact.ts:130)
#[derive(Clone, Debug)]
pub struct Pend {
    pub gd: GuardId,
    pub also: Option<Vec<crate::wallreact::Gone>>,
    pub until: f64,
    pub next: f64,
    pub rect: [f64; 4],
    pub b: Option<WallreactBreachId>,
    pub who: f64,
}
impl Pend {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pend {
            gd: GuardId(0),
            also: None,
            until: 0.0,
            next: 0.0,
            rect: [0.0; 4],
            b: None,
            who: 0.0,
        }
    }
}
impl Default for Pend { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallReact, src/game/wallreact.ts:153)
#[derive(Clone, Debug)]
pub struct WallReact {
    pub breaches: Vec<WallreactBreachId>,
    pub queue: Vec<crate::wallbreak::LevelChange>,
    pub stale: bool,
    pub pend: Vec<crate::wallreact::Pend>,
    pub flood_all: Option<Vec<f64>>,
    pub minds: JsMap<GuardId, JsMap<WallreactBreachId, crate::wallreact::Mind>>,
    pub watching: JsMap<GuardId, WallreactBreachId>,
    pub last: f64,
    pub last_ev: f64,
    pub last_sweep: f64,
    pub moved_t: JsMap<GuardId, f64>,
    pub unsub: bool,
}
impl WallReact {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        WallReact {
            breaches: Vec::new(),
            queue: Vec::new(),
            stale: false,
            pend: Vec::new(),
            flood_all: None,
            minds: JsMap::new(),
            watching: JsMap::new(),
            last: -1.0,
            last_ev: -1.0,
            last_sweep: -1.0,
            moved_t: JsMap::new(),
            unsub: false,
        }
    }
}
impl Default for WallReact { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/wallreact.ts:107 WALLREACT_DEBUG, src/game/wallreact.ts:107)
#[derive(Clone, Copy, Debug)]
pub struct WALLREACTDEBUG {
}
impl WALLREACTDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLREACTDEBUG {
        }
    }
}
impl Default for WALLREACTDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallreact.ts:108 WALLREACT_TEST, src/game/wallreact.ts:108)
#[derive(Clone, Copy, Debug)]
pub struct WALLREACTTEST {
}
impl WALLREACTTEST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WALLREACTTEST {
        }
    }
}
impl Default for WALLREACTTEST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum WallReactHeadGd {
    Guard(GuardId),
    Sam,
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactManKnowsHow {
    Heard,
    Saw,
    Told,
    Made,
}
impl WallReactManKnowsHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallReactManKnowsHow::Heard => "heard",
            WallReactManKnowsHow::Saw => "saw",
            WallReactManKnowsHow::Told => "told",
            WallReactManKnowsHow::Made => "made",
        }
    }
}

/// (TypeScript: inline at src/game/wallreact.ts:418 posts ->, src/game/wallreact.ts:418)
#[derive(Clone, Copy, Debug)]
pub struct WallReactPostsItem {
    pub gd: GuardId,
    pub a: OrdersAssignmentId,
}
impl WallReactPostsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallReactPostsItem {
            gd: GuardId(0),
            a: OrdersAssignmentId(0),
        }
    }
}
impl Default for WallReactPostsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallreact.ts:423 charOf ->, src/game/wallreact.ts:423)
#[derive(Clone, Copy, Debug)]
pub struct WallReactCharOf {
    pub pos: V3,
    pub team: crate::teams::Team,
}
impl WallReactCharOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallReactCharOf {
            pos: [0.0; 3],
            team: crate::teams::Team::Guard,
        }
    }
}
impl Default for WallReactCharOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WallReactAsks {
    Text(String),
    Arm1(crate::wallreact::WallReactAsksArm1),
}

/// (TypeScript: inline at src/game/wallreact.ts:444 asks ->, src/game/wallreact.ts:444)
#[derive(Clone, Copy, Debug)]
pub struct WallReactAsksArm1 {
    pub x: crate::wallreact::Gone,
    pub threat: V3,
}
impl WallReactAsksArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallReactAsksArm1 {
            x: crate::wallreact::Gone::zeroed(),
            threat: [0.0; 3],
        }
    }
}
impl Default for WallReactAsksArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactReconsider {
    Fine,
    Moved,
    Later,
}
impl WallReactReconsider {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallReactReconsider::Fine => "fine",
            WallReactReconsider::Moved => "moved",
            WallReactReconsider::Later => "later",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactReconsider10 {
    Fine,
    Moved,
    Later,
}
impl WallReactReconsider10 {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallReactReconsider10::Fine => "fine",
            WallReactReconsider10::Moved => "moved",
            WallReactReconsider10::Later => "later",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactShift {
    Fine,
    Moved,
    Later,
}
impl WallReactShift {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallReactShift::Fine => "fine",
            WallReactShift::Moved => "moved",
            WallReactShift::Later => "later",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactOpMove {
    Fine,
    Moved,
    Later,
}
impl WallReactOpMove {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallReactOpMove::Fine => "fine",
            WallReactOpMove::Moved => "moved",
            WallReactOpMove::Later => "later",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WallreactMod {
    /// (src/game/wallreact.ts:103 WALLREACT_LOG)
    pub wallreact_log: Vec<String>,
    /// (src/game/wallreact.ts:153)
    pub wall_react: crate::wallreact::WallReact,
}
impl WallreactMod {
    pub fn blank() -> Self {
        WallreactMod {
            wallreact_log: Vec::new(),
            wall_react: crate::wallreact::WallReact::blank(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WallreactStats {
    pub rows: i32,
    pub breaches: i32,
    pub merged: i32,
    pub learned_ops: i32,
    pub learned_men: i32,
    pub told: i32,
    pub moved_def: i32,
    pub moved_ai: i32,
    pub moved_op: i32,
    pub no_cover: i32,
    pub watches: i32,
    pub replans: i32,
    pub openings: i32,
}

/// (src/game/wallreact.ts:107 WALLREACT_DEBUG: a table nothing writes)
pub fn wallreact_debug() -> crate::wallreact::WALLREACTDEBUG { crate::wallreact::WALLREACTDEBUG {  } }

/// (src/game/wallreact.ts:108 WALLREACT_TEST: a table nothing writes)
pub fn wallreact_test() -> crate::wallreact::WALLREACTTEST { crate::wallreact::WALLREACTTEST {  } }

/// (src/game/wallreact.ts:138 EYE)
pub const EYE: f64 = 1.55;

/// (src/game/wallreact.ts:138 CHEST)
pub const CHEST: f64 = 1.1;
