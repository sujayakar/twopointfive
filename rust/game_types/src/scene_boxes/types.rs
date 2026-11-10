//! The types of `scene_boxes`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Box {
    pub c: V3,
    pub h: V3,
    pub yaw: f64,
    pub albedo: V3,
    pub emissive: V3,
    pub flags: f64,
    pub owner: f64,
    pub cut_height: f64,
    pub wet_k: Option<f64>,
    pub rot: Option<Quat>,
    pub dc: Option<V3>,
    pub dh: Option<V3>,
    pub name: Option<String>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub aniso: Option<bool>,
    pub marks: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub mark_gain: Option<f64>,
    pub fog_emit: Option<[f64; 3]>,
    pub fog_mark: Option<f64>,
    pub mkey: Option<f64>,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub cut_core: Option<V3>,
    pub cut_glass: Option<crate::scene_cut::CutGlass>,
    pub cut_round: Option<f64>,
    pub cut_arc: Option<f64>,
    pub cut_old: Option<bool>,
    pub cut_core_stuff: Option<crate::scene_stuff::BoxStuff>,
    pub round: Option<bool>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct BoxWorld {
    pub statics: Vec<BoxId>,
    pub dynamics: Vec<AnyBoxId>,
    pub max_boxes: f64,
    pub geom_serial: f64,
    pub glass_open: Option<crate::scene_collision::PaneRule>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxFlag {
    pub no_primary: f64,
    pub no_shadow: f64,
    pub cutaway: f64,
    pub dynamic: f64,
    pub emissive: f64,
    pub area_lit: f64,
    pub gloss: f64,
    pub sound_seal: f64,
    pub glass: f64,
    pub thin: f64,
    pub bar: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MKEY {
    pub door: f64,
    pub prop: f64,
    pub item: f64,
    pub rig: f64,
    pub device: f64,
    pub test: f64,
    pub debris: f64,
    pub look: f64,
    pub skin: f64,
}

#[derive(Clone, Debug, Default)]
pub struct MakeBoxP {
    pub c: V3,
    pub h: V3,
    pub yaw: Option<f64>,
    pub albedo: Option<V3>,
    pub emissive: Option<V3>,
    pub flags: Option<f64>,
    pub owner: Option<f64>,
    pub cut_height: Option<f64>,
    pub wet_k: Option<f64>,
    pub rot: Option<Quat>,
    pub dc: Option<V3>,
    pub dh: Option<V3>,
    pub name: Option<String>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub aniso: Option<bool>,
    pub marks: Option<Vec<crate::scene_markings::MarkingDef>>,
    pub mark_gain: Option<f64>,
    pub fog_emit: Option<[f64; 3]>,
    pub fog_mark: Option<f64>,
    pub mkey: Option<f64>,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub cut_core: Option<V3>,
    pub cut_glass: Option<crate::scene_cut::CutGlass>,
    pub cut_round: Option<f64>,
    pub cut_arc: Option<f64>,
    pub cut_old: Option<bool>,
    pub cut_core_stuff: Option<crate::scene_stuff::BoxStuff>,
    pub round: Option<bool>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WordOfB {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WordOf {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldRaycast {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBox {
    pub t: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBoxCut {
    pub t: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBoxRound {
    pub t: f64,
    pub n: V3,
}

pub const BOX_WORLD_RAY_PAD: f64 = 0.001;

pub const BOX_WORLD_RS: f64 = 12.0;

pub fn box_flag() -> crate::scene_boxes::BoxFlag { crate::scene_boxes::BoxFlag { no_primary: 1.0, no_shadow: 2.0, cutaway: 4.0, dynamic: 8.0, emissive: 16.0, area_lit: 32.0, gloss: 128.0, sound_seal: 65536.0, glass: 131072.0, thin: 262144.0, bar: 524288.0 } }

pub const SHEET_MASK: f64 = 131083.0;

pub fn mkey() -> crate::scene_boxes::MKEY { crate::scene_boxes::MKEY { door: 16777216.0, prop: 33554432.0, item: 50331648.0, rig: 67108864.0, device: 83886080.0, test: 117440512.0, debris: 134217728.0, look: 150994944.0, skin: 167772160.0 } }

