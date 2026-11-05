//! The types of `disarm`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Strip, src/game/disarm.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct Strip {
    pub t: f64,
    pub kind: crate::slotsetup::SetupGun,
    pub supp: bool,
    pub at: V3,
    pub item: Option<ItemId>,
    pub rap: bool,
    pub gone: bool,
    pub yaw: f64,
}
impl Strip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Strip {
            t: 0.0,
            kind: crate::slotsetup::SetupGun::Pistol,
            supp: false,
            at: [0.0; 3],
            item: None,
            rap: false,
            gone: false,
            yaw: 0.0,
        }
    }
}
impl Default for Strip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/disarm.ts:39 DISARM_GAIN, src/game/disarm.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct DISARMGAIN {
    pub catch: f64,
    pub strip: f64,
    pub down: f64,
    pub long: f64,
}
impl DISARMGAIN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DISARMGAIN {
            catch: 0.0,
            strip: 0.0,
            down: 0.0,
            long: 0.0,
        }
    }
}
impl Default for DISARMGAIN { fn default() -> Self { Self::zeroed() } }

/// (src/game/disarm.ts:39 DISARM_GAIN: a table nothing writes)
pub fn disarm_gain() -> crate::disarm::DISARMGAIN { crate::disarm::DISARMGAIN { catch: 0.5, strip: 0.45, down: 0.4, long: 0.8 } }

/// (src/game/disarm.ts:41 LIFE)
pub const LIFE: f64 = 240.0;

/// (src/game/disarm.ts:42 DEG)
pub const DEG: f64 = 0.017453292519943295;
