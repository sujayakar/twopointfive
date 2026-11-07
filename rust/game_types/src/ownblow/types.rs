//! The types of `ownblow`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what the nearest charge of his own does when set off (ownblow.ts ownSet): a door's goes by door_tools_trigger(g, Some(d)), a wall's by wall_charge_blow(g, Some(w), None), its result dropped
#[derive(Clone, Debug)]
pub enum OwnSetGo {
    Door { d: ChargeId },
    Wall { w: WallChargeId },
}

/// (TypeScript: inline at src/game/ownblow.ts:43 ownSet ->, src/game/ownblow.ts:43)
#[derive(Clone, Debug)]
pub struct OwnSet {
    pub at: V3,
    pub words: String,
    pub where_: String,
    pub fatal: bool,
    pub go: crate::ownblow::OwnSetGo,
}
impl OwnSet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnSet {
            at: [0.0; 3],
            words: String::new(),
            where_: String::new(),
            fatal: false,
            go: crate::ownblow::OwnSetGo::Door { d: ChargeId(0) },
        }
    }
}
impl Default for OwnSet { fn default() -> Self { Self::zeroed() } }

/// (src/game/ownblow.ts:33 SAM_BY)
pub const SAM_BY: &str = "Sam";
