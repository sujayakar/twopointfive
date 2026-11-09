//! The types of `input`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub struct Input {
    pub keys: JsSet<String>,
    pub pressed: JsSet<String>,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub orbit_dx: f64,
    pub wheel_ctrl: f64,
    pub buttons: f64,
    pub clicked: f64,
    pub wheel: f64,
}
impl Input {
    /// what a new one holds
    pub fn blank() -> Self {
        Input {
            keys: JsSet::new(),
            pressed: JsSet::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            orbit_dx: 0.0,
            wheel_ctrl: 0.0,
            buttons: 0.0,
            clicked: 0.0,
            wheel: 0.0,
        }
    }
}
impl Default for Input { fn default() -> Self { Self::blank() } }
