//! The types of `packside`.
use crate::V3;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default)]
pub struct PackExt {
    pub x0: f64,
    pub x1: f64,
    pub top0: f64,
    pub top1: f64,
    pub z0: f64,
    pub z1: f64,
}

/// a point of the back, in bind space, and the joint that carries it most
#[derive(Clone, Copy, Debug, Default)]
pub struct BackPt {
    pub j: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A rig's packs, and the back of the body under them.
#[derive(Clone, Debug, Default)]
pub struct Back {
    /// the chest joint's place in the skin's list
    pub chest: usize,
    /// each part that is a pack
    pub packs: Vec<Option<crate::packside::PackExt>>,
    /// the parts that are kit; the one that is a ram, and the one that is a plain pack
    pub kit: u32,
    pub ram: u32,
    pub pack: u32,
    /// the body's points behind the trunk's joints
    pub line: Vec<crate::packside::BackPt>,
    /// the hindmost point of each band up a strip at either end of its reach (his right, his left), where the side of any pack lies
    pub ends: [Vec<crate::packside::BackPt>; 2],
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

#[derive(Clone, Copy, Debug, Default)]
pub struct SideBindDims {
    pub grip: Option<f64>,
    pub thick: f64,
    pub head: Option<V3>,
    pub rear: Option<f64>,
    pub top: Option<f64>,
    pub len: Option<f64>,
}

pub const STRIP: f64 = 0.075;

pub const Y0: f64 = 0.9;

pub const Y1: f64 = 1.62;

pub const ZB: f64 = -0.15;

pub const BAND: f64 = 0.04;

pub const REACH: f64 = 0.14;

/// half the width of the strip of a back that a thing lies on
pub const HALF: f64 = 0.04;

/// bands of a back, from the floor up
pub const BANDS: usize = 41;
