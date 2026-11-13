//! The types of `floor`.
use crate::{Words, bytes::ToBytes};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum FloorMat {
    #[default] Carpet,
    Lino,
    Stone,
    Tile,
    Asphalt,
    Grass,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FLOORValue {
    pub loud: f64,
    pub hard: bool,
}

pub fn floor() -> &'static [(crate::floor::FloorMat, crate::floor::FLOORValue)] { &[(crate::floor::FloorMat::Carpet, crate::floor::FLOORValue { loud: 0.6, hard: false }), (crate::floor::FloorMat::Lino, crate::floor::FLOORValue { loud: 1.0, hard: true }), (crate::floor::FloorMat::Stone, crate::floor::FLOORValue { loud: 1.1, hard: true }), (crate::floor::FloorMat::Tile, crate::floor::FLOORValue { loud: 1.05, hard: true }), (crate::floor::FloorMat::Asphalt, crate::floor::FLOORValue { loud: 0.9, hard: true }), (crate::floor::FloorMat::Grass, crate::floor::FLOORValue { loud: 0.45, hard: false })] }
