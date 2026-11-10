//! The types of `interact`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct EmptyThing {
    pub it: crate::game::Interactable,
    pub d: f64,
    pub why: String,
}

pub const RING_BODY_M: f64 = 1.0;

pub const RING_THING_M: f64 = 3.5;

