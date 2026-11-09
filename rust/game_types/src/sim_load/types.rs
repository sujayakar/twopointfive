//! The types of `sim_load`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct StartModule {
    pub exports: JsMap<String, crate::Opaque>,
    pub ms: f64,
}
