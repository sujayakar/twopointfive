//! The types of `consts`.

pub const PLAYER_ID: f64 = 1.0;

pub const NOBODY_ID: f64 = 2.0;

pub fn qrf_tint() -> [f64; 3] { [0.1, 0.11, 0.13] }

pub fn qrf_tint2() -> [f64; 3] { [0.05, 0.055, 0.06] }

pub const DRIVE_REACH: f64 = 1.1;

pub const DRIVE_SECS: f64 = 3.5;

pub const INTERACT_KEY: crate::input::types::Key = crate::input::types::Key::Space;
