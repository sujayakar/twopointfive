//! The types of `snow_tracks`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SnowTracks {
    pub data: Vec<u8>,
    pub w: f64,
    pub h: f64,
    pub dirty: Option<[f64; 4]>,
    pub prints: f64,
    pub hollows: f64,
    pub craters: f64,
    pub rects: Vec<[f64; 4]>,
    pub marks: f64,
    pub forming: Vec<crate::snow_tracks::Forming>,
    pub kicks: Vec<crate::snow_tracks::SnowKick>,
    pub kicked: f64,
    pub unseen: JsMap<crate::Num, crate::snow_tracks::UnseenSquare>,
    pub kept: f64,
    pub found: f64,
    pub lost: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Forming {
    pub x0: f64,
    pub z0: f64,
    pub w: f64,
    pub h: f64,
    pub goal: Vec<u8>,
    pub age: f64,
    pub delay: f64,
    pub sink: f64,
    pub thrown: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SnowKick {
    pub x: f64,
    pub z: f64,
    pub dx: f64,
    pub dz: f64,
    pub force: f64,
    pub wide: f64,
    pub serial: f64,
    pub t: f64,
    pub n: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct UnseenSquare {
    pub x: f64,
    pub z: f64,
    pub marks: Vec<crate::snow_tracks::Unseen>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Unseen {
    pub k: f64,
    pub a: Vec<f64>,
    pub dark: bool,
}

