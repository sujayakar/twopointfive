//! The types of `destruction_switch`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DestructionSwitchModBLASTPACE {
    pub on: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DestructionSwitchMod {
    #[is(crate::destruction_switch::DestructionSwitchModBLASTPACE { on: true })] pub blast_pace: crate::destruction_switch::DestructionSwitchModBLASTPACE,
}

