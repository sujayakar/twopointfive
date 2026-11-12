//! The types of `samlook`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};

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
    pub state: JsMap<PlayerId, crate::samlook::St>,
}
