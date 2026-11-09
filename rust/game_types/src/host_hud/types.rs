//! The types of `host_hud`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct View {
    pub client_width: f64,
    pub client_height: f64,
    pub width: f64,
    pub height: f64,
}
