//! The types of `samlook`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct St {
    pub on: Option<GuardId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SlipNow {
    pub k: f64,
    pub lean: f64,
    pub at: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SamlookMod {
    /// (src/game/samlook.ts:33 STATE)
    pub state: JsMap<PlayerId, crate::samlook::St>,
}
