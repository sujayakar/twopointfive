//! The types of `posehome`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct Cast {
    pub player: PlayerId,
    pub guards: Vec<GuardId>,
    pub civilians: Vec<GuardId>,
    pub seats: Vec<SeatId>,
}
