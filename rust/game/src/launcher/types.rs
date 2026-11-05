//! The types of `launcher`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Ring, src/game/launcher.ts:51)
#[derive(Clone, Copy, Debug)]
pub struct Ring {
    pub pos: V3,
    pub vel: V3,
    pub age: f64,
    pub shooter: CharId,
    pub spin: f64,
}
impl Ring {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ring {
            pos: [0.0; 3],
            vel: [0.0; 3],
            age: 0.0,
            shooter: CharId(0),
            spin: 0.0,
        }
    }
}
impl Default for Ring { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Ballistics, src/game/launcher.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct Ballistics {
    pub speed: f64,
    pub gravity: f64,
    pub drag: f64,
    pub radius: f64,
    pub life: f64,
    pub substeps: f64,
    pub solve_iters: f64,
}
impl Ballistics {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ballistics {
            speed: 0.0,
            gravity: 0.0,
            drag: 0.0,
            radius: 0.0,
            life: 0.0,
            substeps: 0.0,
            solve_iters: 0.0,
        }
    }
}
impl Default for Ballistics { fn default() -> Self { Self::zeroed() } }

/// (src/game/launcher.ts:48 STAGGER_LINES: a table nothing writes)
pub fn stagger_lines_table() -> Vec<String> { alloc::vec![String::from("agh — what the—"), String::from("AH! son of a—"), String::from("ow! what— who's there?!"), String::from("nngh— something hit me!"), String::from("jesus— my ribs—")] }
