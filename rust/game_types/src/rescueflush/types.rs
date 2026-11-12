//! The types of `rescueflush`.
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, V3};
use alloc::vec::Vec;

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
    pub via: Vec<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum Goal {
    #[default] Out,
    Side,
    Noise,
    Room,
}

