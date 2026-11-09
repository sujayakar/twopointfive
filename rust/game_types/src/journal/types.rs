//! The types of `journal`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct View {
    pub client_width: f64,
    pub client_height: f64,
    pub width: f64,
    pub height: f64,
}
