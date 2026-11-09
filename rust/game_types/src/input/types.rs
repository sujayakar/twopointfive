//! The types of `input`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Blank)]
pub struct Input {
    pub keys: JsSet<String>,
    pub pressed: JsSet<String>,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub buttons: f64,
    pub clicked: f64,
    pub wheel: f64,
}
