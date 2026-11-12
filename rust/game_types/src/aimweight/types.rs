//! The types of `aimweight`.
use crate::{Blank, bytes::ToBytes};

#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct AimWeight {
    pub vel: f64,
    pub r: f64,
    pub y: f64,
    pub live: bool,
    pub was: bool,
    #[is(true)] pub landed: bool,
    #[is(true)] pub on: bool,
    pub err: f64,
    pub launch_t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AimSwingW {
    pub vel: f64,
}

