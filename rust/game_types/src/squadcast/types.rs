//! The types of `squadcast`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadCast {
    pub orders: crate::orders::SquadOrders,
    pub tools: crate::doortools::DoorTools,
}

pub fn every_key() -> JsSet<String> { { let mut s = JsSet::new(); s.add(String::from("*")); s } }
