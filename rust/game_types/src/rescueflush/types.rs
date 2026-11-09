//! The types of `rescueflush`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct RescueFlush {
    pub fell_at: f64,
    pub next_at: f64,
    pub gone_at: f64,
    pub count: f64,
    pub sent: JsMap<GuardId, crate::rescueflush::Sent>,
    pub barred: JsSet<GuardId>,
}
impl RescueFlush {
    /// what a new one holds
    pub fn blank() -> Self {
        RescueFlush {
            fell_at: f64::NAN,
            next_at: f64::NAN,
            gone_at: f64::NAN,
            count: 0.0,
            sent: JsMap::new(),
            barred: JsSet::new(),
        }
    }
}
impl Default for RescueFlush { fn default() -> Self { Self::blank() } }

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

