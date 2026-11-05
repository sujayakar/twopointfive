#![allow(unused_variables)]
//! Every function and method of `math_dice`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/math/dice.ts:7 mulberry32)
pub fn mulberry32(a: f64) -> crate::FnField { todo!() }

/// (src/math/dice.ts:12 dieOf)
pub fn die_of(seed: f64) -> crate::math_dice::Die { todo!() }

/// (src/math/dice.ts:13 roll)
pub fn roll(d: &mut crate::math_dice::Die) -> f64 { todo!() }

/// (src/math/dice.ts:16 rollOn)
pub fn roll_on(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/math/dice.ts:18 hashStr)
pub fn hash_str(s: &str) -> f64 { todo!() }

/// (src/math/dice.ts:22 simDie)
pub fn sim_die(seed: f64, displaced: &mut dyn FnMut() -> f64) -> crate::FnField { todo!() }

/// (src/math/dice.ts:44 SimDice.constructor)
pub fn sim_dice_new(seed: f64) -> crate::math_dice::SimDice { todo!() }

/// (src/math/dice.ts:45 SimDice.s)
pub fn sim_dice_s(this: &mut crate::math_dice::SimDice) -> f64 { todo!() }

/// (src/math/dice.ts:47 SimDice.n)
pub fn sim_dice_n(this: &mut crate::math_dice::SimDice) -> f64 { todo!() }

/// (src/math/dice.ts:54 useDice)
pub fn use_dice(g: &mut crate::game::Game, d: Option<crate::math_dice::SimDice>) -> Option<crate::math_dice::SimDice> { todo!() }

/// (src/math/dice.ts:56 diceInUse)
pub fn dice_in_use(g: &mut crate::game::Game) -> Option<crate::math_dice::SimDice> { todo!() }

/// (src/math/dice.ts:58 simRandom)
pub fn sim_random(g: &mut crate::game::Game) -> f64 { todo!() }
