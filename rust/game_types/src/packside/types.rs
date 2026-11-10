//! The types of `packside`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct PackExt {
    pub x0: f64,
    pub x1: f64,
    pub top0: f64,
    pub top1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BackPt {
    pub j: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Memo {
    pub j3: f64,
    pub ext: Vec<Option<crate::packside::PackExt>>,
    pub kit: f64,
    pub ram: f64,
    pub pack: f64,
    pub body: f64,
    pub back: JsMap<crate::Num, Vec<crate::packside::BackPt>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SidePlace {
    pub o: V3,
    pub x: V3,
    pub y: V3,
    pub z: V3,
    pub pivot: Option<V3>,
    pub len: Option<f64>,
    pub r: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SideBindWhat {
    Rifle,
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SideBindDims {
    pub grip: Option<f64>,
    pub thick: f64,
    pub head: Option<V3>,
    pub rear: Option<f64>,
    pub top: Option<f64>,
    pub len: Option<f64>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct PacksideMod {
    pub memo: JsMap<RigId, crate::packside::Memo>,
}

pub const STRIP: f64 = 0.075;

pub const Y0: f64 = 0.9;

pub const Y1: f64 = 1.62;

pub const ZB: f64 = -0.15;

pub const BAND: f64 = 0.04;

pub const REACH: f64 = 0.14;
