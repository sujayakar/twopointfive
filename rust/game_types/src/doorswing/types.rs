//! The types of `doorswing`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SwingDeal {
    pub swing: f64,
    pub why: crate::doorswing::SwingWhy,
    pub into: Option<String>,
    pub out: Option<alloc::boxed::Box<crate::doorswing::SwingDeal>>,
    pub clear: Option<alloc::boxed::Box<crate::doorswing::SwingDeal>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum SwingWhy {
    #[default] Mission,
    Floor,
    Authored,
    Folds,
    Outside,
    Plan,
    Room,
    Hostages,
    Lock,
    #[word("stands open")] StandsOpen,
    Wider,
    #[word("leaf room")] LeafRoom,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FloorsValue {
    pub plus: f64,
    pub minus: f64,
}

pub const MIN_OPEN: f64 = 0.9;

pub const ROOF_PTS: f64 = 6.0;

pub const ROOF_UNDER: f64 = 1.9;

pub const LEAF_MAN: f64 = 0.3;

pub const LEAF_STEP: f64 = 0.1;

pub const LEAF_WIN: f64 = 4.0;

pub const LEAF_REST: f64 = 1.84;

pub const LEAF_CUT: f64 = 1.0;

pub const LEAF_RING: f64 = 0.25;

pub const LEAF_WALL: f64 = 0.5;
