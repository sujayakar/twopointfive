//! The types of `knobs_reg`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at quiet knobRegistry ->, quiet knobRegistry ->)
#[derive(Clone, Copy, Debug)]
pub struct KnobRegistryValue {
}
impl KnobRegistryValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KnobRegistryValue {
        }
    }
}
impl Default for KnobRegistryValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet knobsNamed ->, quiet knobsNamed ->)
#[derive(Clone, Copy, Debug)]
pub struct KnobsNamedValue {
}
impl KnobsNamedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KnobsNamedValue {
        }
    }
}
impl Default for KnobsNamedValue { fn default() -> Self { Self::zeroed() } }

