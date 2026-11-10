//! The types of `falling`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// WHAT A FALLING THING LANDS ON: the statics, or the statics and the props. A pane's glass does not land on furniture.
#[derive(Clone, Copy, Debug)]
pub enum FallWorld {
    Statics,
    StaticsAndProps,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FallOf {
    pub t: f64,
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FlightOf {
    pub t: f64,
    pub pos: V3,
}

pub const CHORDS: f64 = 6.0;
