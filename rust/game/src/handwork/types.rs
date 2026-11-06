//! The types of `handwork`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SamWork, src/game/handwork.ts:122)
#[derive(Clone, Copy, Debug)]
pub struct SamWork {
    pub kind: crate::handwork::SamWorkKind,
    pub p: V3,
    pub n: V3,
    pub at: Option<V3>,
}
impl SamWork {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamWork {
            kind: crate::handwork::SamWorkKind::Breaker,
            p: [0.0; 3],
            n: [0.0; 3],
            at: None,
        }
    }
}
impl Default for SamWork { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SamWorkKind {
    Breaker,
    Door,
    Drive,
    Wall,
    Switch,
    Defuse,
}
impl SamWorkKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SamWorkKind::Breaker => "breaker",
            SamWorkKind::Door => "door",
            SamWorkKind::Drive => "drive",
            SamWorkKind::Wall => "wall",
            SamWorkKind::Switch => "switch",
            SamWorkKind::Defuse => "defuse",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LiveHalf {
    Radio,
    Work,
    Swap,
}
impl LiveHalf {
    pub const fn as_str(self) -> &'static str {
        match self {
            LiveHalf::Radio => "radio",
            LiveHalf::Work => "work",
            LiveHalf::Swap => "swap",
        }
    }
}

/// (TypeScript: inline at src/game/handwork.ts:60 workAt(more), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Debug)]
pub struct WorkAtMore {
    pub keys: Option<Vec<[f64; 2]>>,
    pub hand: Option<f64>,
    pub dir: Option<f64>,
}
impl WorkAtMore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkAtMore {
            keys: None,
            hand: None,
            dir: None,
        }
    }
}
impl Default for WorkAtMore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handwork.ts:99 padHands(pad), src/game/handwork.ts:99)
#[derive(Clone, Debug)]
pub struct PadHandsPad {
    pub def: crate::handwork::PadHandsPadDef,
    pub n: V3,
}
impl PadHandsPad {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PadHandsPad {
            def: crate::handwork::PadHandsPadDef::zeroed(),
            n: [0.0; 3],
        }
    }
}
impl Default for PadHandsPad { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handwork.ts:99 PadHandsPad.def, src/game/handwork.ts:99)
#[derive(Clone, Debug)]
pub struct PadHandsPadDef {
    pub pos: V3,
    pub code: String,
    pub yaw: f64,
}
impl PadHandsPadDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PadHandsPadDef {
            pos: [0.0; 3],
            code: String::new(),
            yaw: 0.0,
        }
    }
}
impl Default for PadHandsPadDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handwork.ts:124 breakerFace ->, src/game/handwork.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct BreakerFace {
    pub p: V3,
    pub n: V3,
}
impl BreakerFace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakerFace {
            p: [0.0; 3],
            n: [0.0; 3],
        }
    }
}
impl Default for BreakerFace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handwork.ts:134 rackFace ->, src/game/handwork.ts:134)
#[derive(Clone, Copy, Debug)]
pub struct RackFace {
    pub p: V3,
    pub n: V3,
}
impl RackFace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RackFace {
            p: [0.0; 3],
            n: [0.0; 3],
        }
    }
}
impl Default for RackFace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/handwork.ts:182 teamWorkHand(h), src/game/handwork.ts:182)
#[derive(Clone, Copy, Debug)]
pub struct TeamWorkHandH {
    pub p: V3,
    pub n: V3,
    pub w: f64,
}
impl TeamWorkHandH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeamWorkHandH {
            p: [0.0; 3],
            n: [0.0; 3],
            w: 0.0,
        }
    }
}
impl Default for TeamWorkHandH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TeamWorkHandKind {
    Take,
    Lever,
}
impl TeamWorkHandKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TeamWorkHandKind::Take => "take",
            TeamWorkHandKind::Lever => "lever",
        }
    }
}

/// (TypeScript: inline at src/game/handwork.ts:191 leverStand ->, src/game/handwork.ts:191)
#[derive(Clone, Copy, Debug)]
pub struct LeverStand {
    pub yaw: f64,
    pub speed: f64,
}
impl LeverStand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LeverStand {
            yaw: 0.0,
            speed: 0.0,
        }
    }
}
impl Default for LeverStand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwapFeedHeld {
    Torch,
    Weapon,
}
impl SwapFeedHeld {
    pub const fn as_str(self) -> &'static str {
        match self {
            SwapFeedHeld::Torch => "torch",
            SwapFeedHeld::Weapon => "weapon",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwapHeldHeld {
    Torch,
    Weapon,
}
impl SwapHeldHeld {
    pub const fn as_str(self) -> &'static str {
        match self {
            SwapHeldHeld::Torch => "torch",
            SwapHeldHeld::Weapon => "weapon",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwapHeld {
    Torch,
    Weapon,
}
impl SwapHeld {
    pub const fn as_str(self) -> &'static str {
        match self {
            SwapHeld::Torch => "torch",
            SwapHeld::Weapon => "weapon",
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct HandworkStats {
    pub work: i32,
    pub radio: i32,
    pub swaps: i32,
    pub levers: i32,
}

/// (src/game/handwork.ts:90 WORK_TURN)
pub const WORK_TURN: f64 = 10.0;

/// (src/game/handwork.ts:94 KEY_AT: a table nothing writes)
pub fn key_at() -> JsMap<String, [f64; 2]> { { let mut m = JsMap::new(); for (r, row) in ["123", "456", "789", "*0#"].iter().enumerate() { for (c, ch) in row.chars().enumerate() { m.set(String::from(ch), [(c as f64 - 1.0) * 0.02, -0.015 + (1.5 - r as f64) * 0.0175]); } } m } }
