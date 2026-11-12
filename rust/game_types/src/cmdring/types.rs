//! The types of `cmdring`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RingState {
    pub ax: f64,
    pub ay: f64,
    pub z: f64,
    pub t0: f64,
    pub lit: f64,
    pub armed: bool,
    pub free: Option<bool>,
    pub more: Option<bool>,
    pub at: Option<V3>,
    pub ox: Option<f64>,
    pub oy: Option<f64>,
    pub mx: Option<f64>,
    pub my: Option<f64>,
    pub clear: Option<bool>,
    pub pt: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct FlatRow {
    pub id: String,
    pub word: bool,
    pub free: bool,
    pub targets: Vec<String>,
    pub listed: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
pub struct FlatSeats {
    pub row: Vec<f64>,
    pub more: f64,
    pub over: Vec<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RingLatch {
    Stay,
    Hold,
    Give,
}

