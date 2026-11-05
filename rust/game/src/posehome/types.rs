//! The types of `posehome`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Cast, src/game/posehome.ts:29)
#[derive(Clone, Debug)]
pub struct Cast {
    pub player: PlayerId,
    pub guards: Vec<GuardId>,
    pub civilians: Vec<GuardId>,
    pub seats: Vec<SeatId>,
}
impl Cast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Cast {
            player: PlayerId(0),
            guards: Vec::new(),
            civilians: Vec::new(),
            seats: Vec::new(),
        }
    }
}
impl Default for Cast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/posehome.ts:38 manSingles(g), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct ManSinglesG {
    pub player: Option<PlayerId>,
    pub guards: Option<Vec<GuardId>>,
    pub civilians: Option<Vec<GuardId>>,
    pub seats: Option<Vec<SeatId>>,
}
impl ManSinglesG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManSinglesG {
            player: None,
            guards: Some(Vec::new()),
            civilians: Some(Vec::new()),
            seats: None,
        }
    }
}
impl Default for ManSinglesG { fn default() -> Self { Self::zeroed() } }
