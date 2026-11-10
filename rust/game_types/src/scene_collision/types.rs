//! The types of `scene_collision`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the share of the wind a square metre of the plan has: crate::wind::wind_share_under(g, x, z, roofs)
#[derive(Clone, Debug)]
pub enum GridWindShare {
    Under { roofs: Option<Vec<Vec<f64>>> },
}
/// the flags a line of fire passes: BoxFlag.Bar if crate::winbars::bars_live(g), else 0
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum FirePass {
    Bars,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LampRows {
    pub leave: f64,
    pub steady: bool,
}

#[derive(Clone, Debug)]
pub struct GridWind {
    pub x: f64,
    pub z: f64,
    pub low: f64,
    pub gain: f64,
    pub full_at: f64,
    pub share: crate::scene_collision::GridWindShare,
}
impl Default for GridWind {
    fn default() -> Self {
        GridWind {
            x: 0.0,
            z: 0.0,
            low: 0.0,
            gain: 0.0,
            full_at: 0.0,
            share: crate::scene_collision::GridWindShare::Under { roofs: None },
        }
    }
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StaticCollision {
    pub nav: crate::scene_collision::NavGrid,
    pub sound: crate::scene_collision::SoundPaths,
    pub boxes: Vec<BoxId>,
    pub dynamic_boxes: Vec<BoxId>,
    pub fire_pass: Option<crate::scene_collision::FirePass>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NavGrid {
    #[is(0.5)] pub cell: f64,
    pub w: f64,
    pub h: f64,
    pub blocked: Vec<u8>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SoundPaths {
    pub cache: JsMap<String, crate::scene_collision::SoundPathsCacheValue>,
    pub now: f64,
    pub asking: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SoundPathsCacheValue {
    pub t: f64,
    pub r: crate::scene_collision::Propagation,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Propagation {
    pub direct: bool,
    pub routed: bool,
    pub path_len: f64,
    pub via: V3,
    pub apparent: V3,
    pub muffle: f64,
    pub carry: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LineBox {
    pub t0: f64,
    pub t1: f64,
    pub depth: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThingsRay {
    pub row: f64,
    pub t: f64,
    pub n: V3,
    pub box_: BoxId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SegmentBlockedPanesItem {
    pub bi: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EyeContactE {
    pub pos: V3,
    pub look: V3,
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub v_half_deg: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EyeContact {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GridStepK {
    pub decay: f64,
    pub spread: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub gone: f64,
    pub in_wind: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Raycast {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}

pub const FEET_Y: f64 = 0.2;

pub const BAKE_Y: f64 = 0.25;

pub const SIGHT_EYES: f64 = 7.0;

