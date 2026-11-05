//! The types of `input`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Input, src/game/input.ts:4)
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
    pub suppress_click: bool,
    pub drag_off: bool,
    pub wheel: f64,
    pub has_focus: bool,
    pub lock_except: Option<JsSet<String>>,
}
impl Input {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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
            suppress_click: false,
            drag_off: false,
            wheel: 0.0,
            has_focus: true,
            lock_except: None,
        }
    }
}
impl Default for Input { fn default() -> Self { Self::blank() } }
