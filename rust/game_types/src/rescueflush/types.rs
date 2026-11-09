//! The types of `rescueflush`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RescueFlush {
    #[is(f64::NAN)] pub fell_at: f64,
    #[is(f64::NAN)] pub next_at: f64,
    #[is(f64::NAN)] pub gone_at: f64,
    pub count: f64,
    pub sent: JsMap<GuardId, crate::rescueflush::Sent>,
    pub barred: JsSet<GuardId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Sent {
    pub at: f64,
    pub n: f64,
    pub goal: crate::rescueflush::Goal,
    pub via: Vec<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum Goal {
    #[default] Out,
    Side,
    Noise,
    Room,
}

