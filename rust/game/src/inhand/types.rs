//! The types of `inhand`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum InHandWord {
    Pistol,
    Torch(crate::inhand::InHandWordTorch),
    Long(crate::inhand::InHandWordLong),
}

/// (TypeScript: inline at src/game/inhand.ts:60 InHandWord, src/game/inhand.ts:60)
#[derive(Clone, Copy, Debug)]
pub struct InHandWordTorch {
    pub beam: V3,
    pub at: V3,
    pub axis: V3,
    pub fwd: f64,
    pub palm: f64,
}
impl InHandWordTorch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        InHandWordTorch {
            beam: [0.0; 3],
            at: [0.0; 3],
            axis: [0.0; 3],
            fwd: 0.0,
            palm: 0.0,
        }
    }
}
impl Default for InHandWordTorch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/inhand.ts:60 InHandWord, src/game/inhand.ts:60)
#[derive(Clone, Copy, Debug)]
pub struct InHandWordLong {
    pub by: V3,
}
impl InHandWordLong {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        InHandWordLong {
            by: [0.0; 3],
        }
    }
}
impl Default for InHandWordLong { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/inhand.ts:71 BELT_TORCH, src/game/inhand.ts:71)
#[derive(Clone, Debug)]
pub struct BELTTORCH {
    pub was: Vec<f64>,
    pub at: Vec<f64>,
}
impl BELTTORCH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BELTTORCH {
            was: Vec::new(),
            at: Vec::new(),
        }
    }
}
impl Default for BELTTORCH { fn default() -> Self { Self::zeroed() } }

/// (src/game/inhand.ts:61 IN_PISTOL: a table nothing writes)
pub fn in_pistol() -> crate::inhand::InHandWord { crate::inhand::InHandWord::Pistol }

/// (src/game/inhand.ts:71 BELT_TORCH: a table nothing writes)
pub fn belt_torch() -> crate::inhand::BELTTORCH { crate::inhand::BELTTORCH { was: alloc::vec![0.15, 0.115], at: alloc::vec![0.21, 0.16] } }
