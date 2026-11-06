//! The types of `interact`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/interact.ts:44 emptyThing ->, src/game/interact.ts:44)
#[derive(Clone, Debug)]
pub struct EmptyThing {
    pub it: crate::game::Interactable,
    pub d: f64,
    pub why: String,
}
impl EmptyThing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EmptyThing {
            it: crate::game::Interactable::zeroed(),
            d: 0.0,
            why: String::new(),
        }
    }
}
impl Default for EmptyThing { fn default() -> Self { Self::zeroed() } }

/// (src/game/interact.ts:33 OPEN_ROWS: a table nothing writes)
pub fn open_rows() -> Vec<String> { alloc::vec![String::from("pull"), String::from("ease"), String::from("charge"), String::from("breach"), String::from("hammer"), String::from("wand")] }

/// (src/game/interact.ts:33 SHUT_ROWS: a table nothing writes)
pub fn shut_rows() -> Vec<String> { alloc::vec![String::from("pull"), String::from("ease"), String::from("charge"), String::from("breach"), String::from("hammer"), String::from("wand"), String::from("push"), String::from("pick"), String::from("kick"), String::from("rattle"), String::from("unlock"), String::from("code"), String::from("nocode")] }

/// (src/game/interact.ts:68 TAP_SECS)
pub const TAP_SECS: f64 = 0.25;

/// (src/game/interact.ts:71 WHEEL_STEP)
pub const WHEEL_STEP: f64 = 60.0;

/// (src/game/interact.ts:72 MOVE_KEYS: a table nothing writes)
pub fn move_keys() -> Vec<String> { alloc::vec![String::from("KeyW"), String::from("KeyA"), String::from("KeyS"), String::from("KeyD")] }

/// (src/game/interact.ts:116 RING_BODY_M)
pub const RING_BODY_M: f64 = 1.0;

/// (src/game/interact.ts:116 RING_THING_M)
pub const RING_THING_M: f64 = 3.5;

/// (src/game/interact.ts:120 STATE_OFF: a table nothing writes)
pub fn state_off() -> Vec<String> { alloc::vec![String::from("switch"), String::from("breaker")] }

/// (src/game/interact.ts:123 WHISTLE_WEDGE)
pub const WHISTLE_WEDGE: &str = "whistle";

/// (src/game/interact.ts:123 WHISTLE_CAPTION)
pub const WHISTLE_CAPTION: &str = "whistle — one guard will come to look";
