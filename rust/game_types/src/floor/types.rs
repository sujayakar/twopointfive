//! The types of `floor`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum FloorMat {
    #[default] Carpet,
    Lino,
    Stone,
    Tile,
    Asphalt,
    Grass,
}
impl FloorMat {
    pub const fn as_str(self) -> &'static str {
        match self {
            FloorMat::Carpet => "carpet",
            FloorMat::Lino => "lino",
            FloorMat::Stone => "stone",
            FloorMat::Tile => "tile",
            FloorMat::Asphalt => "asphalt",
            FloorMat::Grass => "grass",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FLOORValue {
    pub loud: f64,
    pub hard: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BUILDING {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FINISHESItem {
    pub mat: crate::floor::FloorMat,
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
}

pub fn floor() -> &'static [(crate::floor::FloorMat, crate::floor::FLOORValue)] { &[(crate::floor::FloorMat::Carpet, crate::floor::FLOORValue { loud: 0.6, hard: false }), (crate::floor::FloorMat::Lino, crate::floor::FLOORValue { loud: 1.0, hard: true }), (crate::floor::FloorMat::Stone, crate::floor::FLOORValue { loud: 1.1, hard: true }), (crate::floor::FloorMat::Tile, crate::floor::FLOORValue { loud: 1.05, hard: true }), (crate::floor::FloorMat::Asphalt, crate::floor::FLOORValue { loud: 0.9, hard: true }), (crate::floor::FloorMat::Grass, crate::floor::FLOORValue { loud: 0.45, hard: false })] }

pub fn building() -> crate::floor::BUILDING { crate::floor::BUILDING { x0: 4.0, x1: 36.0, z0: 4.0, z1: 24.0 } }

pub const GRASS_EDGE: f64 = 3.2;

pub fn finishes() -> &'static [crate::floor::FINISHESItem] { &[crate::floor::FINISHESItem { mat: crate::floor::FloorMat::Lino, x0: 4.0, z0: 10.0, x1: 36.0, z1: 12.2 }, crate::floor::FINISHESItem { mat: crate::floor::FloorMat::Stone, x0: 4.0, z0: 16.0, x1: 12.0, z1: 24.0 }, crate::floor::FINISHESItem { mat: crate::floor::FloorMat::Tile, x0: 14.0, z0: 4.0, x1: 22.0, z1: 10.0 }, crate::floor::FINISHESItem { mat: crate::floor::FloorMat::Lino, x0: 28.0, z0: 12.2, x1: 36.0, z1: 24.0 }] }
