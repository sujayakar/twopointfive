//! The types of `marks`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Surface2 {
    pub box_: BoxId,
    pub dynamic: bool,
}
