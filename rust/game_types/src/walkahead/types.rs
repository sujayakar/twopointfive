//! The types of `walkahead`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Ahead {
    pub pos: V3,
    pub vel: V3,
    pub aim_yaw: f64,
    pub body_yaw: f64,
    pub crouch: bool,
    pub follow: V3,
    pub aim: Option<V3>,
    pub free: bool,
}
