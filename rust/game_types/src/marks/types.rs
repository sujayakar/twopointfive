//! The types of `marks`.
use crate::ids::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Surface2 {
    pub box_: BoxId,
    pub dynamic: bool,
}
