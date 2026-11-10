//! The types of `ringlone`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct RINGBOX {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LONE {
    pub r: f64,
    pub h: f64,
    pub pad: f64,
    pub char: f64,
    pub max: f64,
    pub lead: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxOf {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub px: f64,
    pub py: f64,
    pub left: bool,
}

/// px, in the ring's own: the wheel's hole, and the middle's own ring beside a pill
pub const RING_HOLE: f64 = 17.0;

/// px: the wheel's rim (a wedge is drawn from the hole out to it)
pub const RING_RIM: f64 = 44.0;

/// px: how far each way from its middle a ring's wedges and words may lie
pub fn ring_box() -> crate::ringlone::RINGBOX { crate::ringlone::RINGBOX { x: 190.0, y: 80.0 } }

/// px: a pill's numbers. The cursor is read on them and the HUD draws on them (`view.look.ring.lone`); no recording carries them, so one changed moves the place a recorded cursor is read on
pub fn lone() -> crate::ringlone::LONE { crate::ringlone::LONE { r: 34.0, h: 22.0, pad: 10.0, char: 6.6, max: 20.0, lead: 12.0 } }
