//! The types of `math_dice`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Die, src/math/dice.ts:11)
#[derive(Clone, Copy, Debug)]
pub struct Die {
    pub dice: f64,
}
impl Die {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Die {
            dice: 0.0,
        }
    }
}
impl Default for Die { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DiceHome, src/math/dice.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct DiceHome {
    pub s: f64,
    pub n: f64,
}
impl DiceHome {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DiceHome {
            s: 0.0,
            n: 0.0,
        }
    }
}
impl Default for DiceHome { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SimDice, src/math/dice.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct SimDice {
    pub home: Option<crate::math_dice::DiceHome>,
    pub s0: f64,
    pub n0: f64,
    pub seed: f64,
}
impl SimDice {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SimDice {
            home: None,
            s0: 0.0,
            n0: 0.0,
            seed: 0.0,
        }
    }
}
impl Default for SimDice { fn default() -> Self { Self::blank() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MathDiceMod {
    /// (src/math/dice.ts:50 cur)
    pub cur: Option<crate::math_dice::SimDice>,
}
impl MathDiceMod {
    pub fn blank() -> Self {
        MathDiceMod {
            cur: None,
        }
    }
}
