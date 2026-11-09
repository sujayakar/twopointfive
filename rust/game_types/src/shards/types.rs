//! The types of `shards`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

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

#[derive(Clone, Debug, Default)]
pub struct GlassFragmentLike {
    pub pane: f64,
    pub type_: crate::glasspen::PaneType,
    pub kind: crate::shards::GlassFragmentLikeKind,
    pub outline: Vec<V3>,
    pub centre: V3,
    pub normal: V3,
    pub thickness: f64,
    pub area: f64,
    pub velocity: V3,
    pub spin: V3,
    pub tint: V3,
    pub grain: f64,
    pub cause: crate::shards::GlassFragmentLikeCause,
    pub who: f64,
    pub seed: f64,
    pub fall: Option<crate::shards::GlassFragmentLikeFall>,
    pub landed: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GlassFragmentLikeKind {
    #[default] Piece,
    Sheet,
    Dagger,
    Clump,
}
impl GlassFragmentLikeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassFragmentLikeKind::Piece => "piece",
            GlassFragmentLikeKind::Sheet => "sheet",
            GlassFragmentLikeKind::Dagger => "dagger",
            GlassFragmentLikeKind::Clump => "clump",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GlassFragmentLikeCause {
    #[default] Blast,
    Hit,
    Dice,
    Support,
    Rake,
}
impl GlassFragmentLikeCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassFragmentLikeCause::Blast => "blast",
            GlassFragmentLikeCause::Hit => "hit",
            GlassFragmentLikeCause::Dice => "dice",
            GlassFragmentLikeCause::Support => "support",
            GlassFragmentLikeCause::Rake => "rake",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassFragmentLikeFall {
    pub t: f64,
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShardsTune {
    pub glass_shards: Option<bool>,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Pending {
    pub t: f64,
    pub pos: V3,
    pub name: crate::sim_fallshome::FallSound,
    pub gain: f64,
    pub rate: f64,
    pub amount: f64,
    pub who: f64,
    pub event: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Life {
    pub t: f64,
    pub n: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct GlassShards {
    pub lives: Vec<crate::shards::Life>,
    pub live_est: f64,
    pub breaks: Vec<crate::shards::GlassShardsBreaksItem>,
    pub recs_this_frame: f64,
    pub sounds_this_frame: f64,
    pub sound_at: JsMap<String, f64>,
    pub sound_owed: JsMap<String, f64>,
    pub deferred: Vec<crate::shards::GlassShardsDeferredItem>,
    pub deferred_n: f64,
    pub crunch_n: f64,
    pub log: Vec<String>,
    pub serial: f64,
}
impl GlassShards {
    /// what a new one holds
    pub fn blank() -> Self {
        GlassShards {
            lives: Vec::new(),
            live_est: 0.0,
            breaks: Vec::new(),
            recs_this_frame: 0.0,
            sounds_this_frame: 0.0,
            sound_at: JsMap::new(),
            sound_owed: JsMap::new(),
            deferred: Vec::new(),
            deferred_n: 0.0,
            crunch_n: 0.0,
            log: Vec::new(),
            serial: 0.0,
        }
    }
}
impl Default for GlassShards { fn default() -> Self { Self::blank() } }

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
    pub dropped: f64,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassShardsTestKind {
    Piece,
    Sheet,
    Dagger,
    Clump,
}
impl GlassShardsTestKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassShardsTestKind::Piece => "piece",
            GlassShardsTestKind::Sheet => "sheet",
            GlassShardsTestKind::Dagger => "dagger",
            GlassShardsTestKind::Clump => "clump",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CutPlateBox {
    pub box_: BoxId,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub dropped: f64,
    pub poly: Vec<[f64; 2]>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ShardsStats {
    pub fragments: i32,
    pub crossings: i32,
    pub plates: i32,
    pub plates_refused: i32,
    pub cut_plates: i32,
    pub cut_split: i32,
    pub cut_dropped: i32,
    pub crumbled: i32,
    pub breaks: i32,
    pub emits: i32,
    pub born: i32,
    pub trimmed: i32,
    pub recs_deferred: i32,
    pub sounds: i32,
    pub sounds_merged: i32,
    pub spalls: i32,
    pub events: i32,
    pub crunches: i32,
    pub peak_live: i32,
    pub peak_plates: i32,
}

/// (src/game/shards.ts:128 SHARDS_DEFAULTS: a table nothing writes)
pub fn shards_defaults() -> crate::knobs::ShardsK { crate::knobs::page_shards() }

/// (src/game/shards.ts:131 LS_KEY)
pub const LS_KEY: &str = "twopointsix.shards";

/// (src/game/shards.ts:133 LIVE_CAP_MAX)
pub const LIVE_CAP_MAX: f64 = 8000.0;

/// (src/game/shards.ts:677 EDGE_GREEN: a table nothing writes)
pub fn edge_green() -> V3 { [0.74, 0.94, 0.86] }
