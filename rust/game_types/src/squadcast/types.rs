//! The types of `squadcast`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SquadCast, src/game/squadcast.ts:14)
#[derive(Clone, Debug)]
pub struct SquadCast {
    pub orders: crate::orders::SquadOrders,
    pub tools: crate::doortools::DoorTools,
}
impl SquadCast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadCast {
            orders: crate::orders::SquadOrders::blank(),
            tools: crate::doortools::DoorTools::blank(),
        }
    }
}
impl Default for SquadCast { fn default() -> Self { Self::zeroed() } }

/// (src/game/squadcast.ts:17 EVERY_KEY: a table nothing writes)
pub fn every_key() -> JsSet<String> { { let mut s = JsSet::new(); s.add(String::from("*")); s } }
