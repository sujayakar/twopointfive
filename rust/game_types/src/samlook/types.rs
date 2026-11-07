//! The types of `samlook`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: St, src/game/samlook.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct St {
    pub on: Option<GuardId>,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            on: None,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/samlook.ts:77 samSlipMan ->, src/game/samlook.ts:77)
#[derive(Clone, Copy, Debug)]
pub struct SamSlipMan {
    pub gd: GuardId,
    pub k: f64,
}
impl SamSlipMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SamSlipMan {
            gd: GuardId(0),
            k: 0.0,
        }
    }
}
impl Default for SamSlipMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/samlook.ts:85 slipNow ->, src/game/samlook.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct SlipNow {
    pub k: f64,
    pub lean: f64,
    pub at: V3,
}
impl SlipNow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlipNow {
            k: 0.0,
            lean: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for SlipNow { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SamlookMod {
    /// (src/game/samlook.ts:33 STATE)
    pub state: JsMap<PlayerId, crate::samlook::St>,
}
impl SamlookMod {
    pub fn blank() -> Self {
        SamlookMod {
            state: JsMap::new(),
        }
    }
}
