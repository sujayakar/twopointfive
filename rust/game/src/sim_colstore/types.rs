//! The types of `sim_colstore`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/sim/colstore.ts:18 colsOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1611)
#[derive(Clone, Copy, Debug)]
pub struct ColsOf2 {
    pub bytes: f64,
}
impl ColsOf2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ColsOf2 {
            bytes: 0.0,
        }
    }
}
impl Default for ColsOf2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/colstore.ts:58 colDiff(O), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1611)
#[derive(Clone, Copy, Debug)]
pub struct ColDiffO {
    pub bytes: f64,
}
impl ColDiffO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ColDiffO {
            bytes: 0.0,
        }
    }
}
impl Default for ColDiffO { fn default() -> Self { Self::zeroed() } }
