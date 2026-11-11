//! The types of `startle`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct St {
    pub calm_t: f64,
    pub calm_torch: bool,
    pub by_sight: bool,
    pub at: f64,
    pub until: f64,
    pub toward: V3,
    pub pre: bool,
    pub torch: bool,
    pub choose: bool,
    pub call: bool,
    pub draw_until: f64,
    pub run: Option<V3>,
    pub run_at: f64,
    pub ref_: Option<V3>,
    pub ref_t: f64,
    pub script: Option<crate::game::GuardScript>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StartleMod {
    pub state_of: JsMap<GuardId, crate::startle::St>,
}

