//! The types of `wand`.
use crate::{bytes::ToBytes, ids::*, V3};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Wanding {
    pub door: DoorId,
    pub side: f64,
    pub pan: f64,
    pub live: bool,
    pub off: f64,
    pub age: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WandLens {
    pub pos: V3,
    pub yaw: f64,
}

