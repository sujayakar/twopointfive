//! The types of `snow_blast`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct SnowBlastShown {
    pub owed: Vec<crate::snow_blast::Owed>,
    pub seen: f64,
    pub turn: f64,
    pub puffs: f64,
    pub thrown: f64,
    pub throw_frames: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Owed {
    pub serial: f64,
    pub puff_left: f64,
    pub flake_left: f64,
}

