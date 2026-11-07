//! THE BRING-UP'S OWN PART of math_dice: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// TEMP-BRINGUP: WHAT STANDS IN Math.random's PLACE. A game with no die of its own (`Game.dice` null: the play-through harness's games) draws off the
// HOST's Math.random (dice.ts simRandom's first line), which the harness has seeded. That stream is no part of the game's state, so the bring-up's host
// hands the tick the draws the TypeScript tick took, in order, and the Rust tick must take the same ones.
// SKELETON-GAP: a host seam for Math.random (`host_api::math_random(g) -> f64`), or a rule that every game compared has its own die.
struct Draws(core::cell::UnsafeCell<(Vec<f64>, usize)>);
unsafe impl Sync for Draws {}
static DRAWS: Draws = Draws(core::cell::UnsafeCell::new((Vec::new(), 0)));

/// the host's draws for the tick about to run
pub fn host_draws_set(v: &[f64]) { unsafe { *DRAWS.0.get() = (v.to_vec(), 0); } }
/// (taken, handed)
pub fn host_draws_count() -> (usize, usize) { unsafe { let d = &*DRAWS.0.get(); (d.1, d.0.len()) } }
/// one draw off the host's stream; past its end is the Rust tick drawing where the TypeScript's did not
pub fn host_draw() -> f64 {
    unsafe {
        let d = &mut *DRAWS.0.get();
        if d.1 >= d.0.len() { panic!("bring-up: the Rust tick asks for draw {} off Math.random and the TypeScript tick took {}", d.1 + 1, d.0.len()); }
        d.1 += 1;
        d.0[d.1 - 1]
    }
}
