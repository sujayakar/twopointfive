//! The types of `render_snow_field`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SnowField {
    pub data: Vec<u16>,
    pub w: f64,
    pub h: f64,
    pub quads: Vec<u32>,
    pub slab: f64,
    pub statics: f64,
    pub paint: [f64; 3],
    pub wind: [f64; 2],
    pub mound: Vec<f32>,
    pub cover: Vec<f32>,
    pub packed: Vec<f32>,
    pub old: Vec<f32>,
    pub roof: Vec<f32>,
    pub wall: Vec<u8>,
    pub hard: Vec<f32>,
    pub base: Option<Vec<f32>>,
    pub kept: Option<Vec<crate::render_snow_field::SnowFieldKeptItem>>,
    pub walks: Option<f64>,
    pub stats: crate::render_snow_field::SnowFieldStats,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SnowFieldKeptItem {
    pub x: f64,
    pub z: f64,
    pub r: f64,
    pub seed: f64,
    pub stood: Vec<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SnowFieldStats {
    pub mounds: f64,
    pub lays: f64,
    pub bare: f64,
    pub roofs: f64,
    pub quads: f64,
    pub max_mound: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SnowPaintOfB {
    pub does: Option<String>,
}

