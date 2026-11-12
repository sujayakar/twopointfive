//! The types of `interact`.
use alloc::string::String;

#[derive(Clone, Debug, Default)]
pub struct EmptyThing {
    pub it: crate::game::Interactable,
    pub d: f64,
    pub why: String,
}

pub const RING_BODY_M: f64 = 1.0;

pub const RING_THING_M: f64 = 3.5;

