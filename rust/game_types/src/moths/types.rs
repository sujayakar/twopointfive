//! The types of `moths`.
use crate::{Blank, bytes::ToBytes, ids::*, V3};
use alloc::vec::Vec;

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Moths {
    pub lamps: Vec<crate::moths::MothsLampsItem>,
    pub moths: Vec<crate::moths::Moth>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MothsLampsItem {
    pub target: FixtureId,
    pub light: LightId,
    pub head: V3,
    pub act: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Moth {
    pub lamp: f64,
    pub r: f64,
    pub omega: f64,
    pub ph: f64,
    pub bob_f: f64,
    pub bob_ph: f64,
    pub wob: f64,
    pub dart_t: f64,
    pub dart_ph: f64,
    pub flap_ph: f64,
    pub away: V3,
    pub shown: f64,
    pub box_: BoxId,
}

