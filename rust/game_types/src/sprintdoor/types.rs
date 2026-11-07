//! The types of `sprintdoor`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: DoorStamps, src/game/sprintdoor.ts:51)
#[derive(Clone, Copy, Debug)]
pub struct DoorStamps {
    pub down: bool,
    pub door_rip_until: f64,
    pub door_bash_at: f64,
    pub door_bash_lean: f64,
}
impl DoorStamps {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorStamps {
            down: false,
            door_rip_until: -1.0,
            door_bash_at: -10.0,
            door_bash_lean: 0.0,
        }
    }
}
impl Default for DoorStamps { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sprintdoor.ts:52 live(pl), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Copy, Debug)]
pub struct LivePl {
    pub down: bool,
}
impl LivePl {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LivePl {
            down: false,
        }
    }
}
impl Default for LivePl { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SprintdoorStats {
    pub bashes: i32,
    pub rips: i32,
    pub kicks: i32,
}

/// (src/game/sprintdoor.ts:61 KICK_OFF)
pub const KICK_OFF: f64 = 0.76;

/// (src/game/sprintdoor.ts:61 KICK_EASE)
pub const KICK_EASE: f64 = 12.0;

/// (src/game/sprintdoor.ts:64 KICK_NEAR)
pub const KICK_NEAR: f64 = 0.06;

/// (src/game/sprintdoor.ts:89 AGAIN)
pub const AGAIN: f64 = 0.4;

/// (src/game/sprintdoor.ts:91 SETTLE)
pub const SETTLE: f64 = 0.2;

/// (src/game/sprintdoor.ts:133 HEAD_SIGN)
pub const HEAD_SIGN: f64 = -1.0;
