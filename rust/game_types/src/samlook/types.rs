//! The types of `samlook`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct St {
    pub on: Option<GuardId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SamSlipMan {
    pub gd: GuardId,
    pub k: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SlipNow {
    pub k: f64,
    pub lean: f64,
    pub at: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SamlookMod {
    /// (src/game/samlook.ts:33 STATE)
    pub state: JsMap<PlayerId, crate::samlook::St>,
}
impl SamlookMod {
    pub fn blank() -> Self {
        SamlookMod {
            state: JsMap::new(),
        }
    }
}
