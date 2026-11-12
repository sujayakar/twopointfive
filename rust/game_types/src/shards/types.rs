//! The types of `shards`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default)]
pub struct PaneCrossLike {
    pub bi: f64,
    pub type_: crate::glasspen::PaneType,
    pub exit: V3,
    pub dir_out: V3,
    pub take: f64,
    pub e_out: f64,
    pub hole: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlateMeta {
    pub gen_: f64,
    pub area: f64,
    pub tint: V3,
    pub type_: crate::glasspen::PaneType,
    pub seed: f64,
    pub littered: Option<bool>,
    pub poly: Option<Vec<[f64; 2]>>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Life {
    pub t: f64,
    pub n: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct GlassShards {
    pub lives: Vec<crate::shards::Life>,
    pub live_est: f64,
    pub breaks: Vec<crate::shards::GlassShardsBreaksItem>,
    pub recs_this_frame: f64,
    pub sounds_this_frame: f64,
    pub sound_at: JsMap<crate::audio_iface::Sfx, f64>,
    pub sound_owed: JsMap<crate::audio_iface::Sfx, f64>,
    pub deferred: Vec<crate::shards::GlassShardsDeferredItem>,
    pub deferred_n: f64,
    pub crunch_n: f64,
    pub serial: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassShardsBreaksItem {
    pub c: DebrisChunkId,
    pub speed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GlassShardsDeferredItem {
    pub pos: V3,
    pub dir: V3,
    pub count: f64,
    pub o: crate::shards::SprayOpts,
    pub seed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SprayOpts {
    pub speed: f64,
    pub speed_jit: f64,
    pub spread: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
    pub aspect: f64,
    pub tumble: f64,
    pub nrm_jit: f64,
    pub pos_jit: f64,
    pub vadd: Option<V3>,
    pub color: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassShardsCutPlatesFr {
    pub u: V3,
    pub v: V3,
    pub lu: f64,
    pub lv: f64,
    pub mid: V3,
}

#[derive(Clone, Debug, Default)]
pub struct GlassShardsPlateCut {
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassShardsFallOf {
    pub t: f64,
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassShardsScheduleFall {
    pub t: f64,
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassShardsLandingFall {
    pub t: f64,
    pub pos: V3,
}

#[derive(Clone, Debug, Default)]
pub struct CutPlateBox {
    pub box_: BoxId,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub poly: Vec<[f64; 2]>,
}

