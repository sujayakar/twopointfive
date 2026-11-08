//! The types of `shards`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: PaneCrossLike, src/game/shards.ts:51)
#[derive(Clone, Copy, Debug)]
pub struct PaneCrossLike {
    pub bi: f64,
    pub type_: crate::glasspen::PaneType,
    pub exit: V3,
    pub dir_out: V3,
    pub take: f64,
    pub e_out: f64,
    pub hole: bool,
}
impl PaneCrossLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PaneCrossLike {
            bi: 0.0,
            type_: crate::glasspen::PaneType::Annealed,
            exit: [0.0; 3],
            dir_out: [0.0; 3],
            take: 0.0,
            e_out: 0.0,
            hole: false,
        }
    }
}
impl Default for PaneCrossLike { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GlassFragmentLike, src/game/shards.ts:52)
#[derive(Clone, Debug)]
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
impl GlassFragmentLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassFragmentLike {
            pane: 0.0,
            type_: crate::glasspen::PaneType::Annealed,
            kind: crate::shards::GlassFragmentLikeKind::Piece,
            outline: Vec::new(),
            centre: [0.0; 3],
            normal: [0.0; 3],
            thickness: 0.0,
            area: 0.0,
            velocity: [0.0; 3],
            spin: [0.0; 3],
            tint: [0.0; 3],
            grain: 0.0,
            cause: crate::shards::GlassFragmentLikeCause::Blast,
            who: 0.0,
            seed: 0.0,
            fall: None,
            landed: None,
        }
    }
}
impl Default for GlassFragmentLike { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassFragmentLikeKind {
    Piece,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassFragmentLikeCause {
    Blast,
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

/// (TypeScript: inline at src/game/shards.ts:63 GlassFragmentLike.fall, src/game/shards.ts:63)
#[derive(Clone, Copy, Debug)]
pub struct GlassFragmentLikeFall {
    pub t: f64,
    pub pos: V3,
}
impl GlassFragmentLikeFall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassFragmentLikeFall {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for GlassFragmentLikeFall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShardsTune, src/game/shards.ts:130)
#[derive(Clone, Copy, Debug)]
pub struct ShardsTune {
    pub glass_shards: Option<bool>,
}
impl ShardsTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShardsTune {
            glass_shards: None,
        }
    }
}
impl Default for ShardsTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PlateMeta, src/game/shards.ts:148)
#[derive(Clone, Debug)]
pub struct PlateMeta {
    pub gen_: f64,
    pub area: f64,
    pub tint: V3,
    pub type_: crate::glasspen::PaneType,
    pub seed: f64,
    pub littered: Option<bool>,
    pub poly: Option<Vec<[f64; 2]>>,
}
impl PlateMeta {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlateMeta {
            gen_: 0.0,
            area: 0.0,
            tint: [0.0; 3],
            type_: crate::glasspen::PaneType::Annealed,
            seed: 0.0,
            littered: None,
            poly: None,
        }
    }
}
impl Default for PlateMeta { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Pending, src/game/shards.ts:152)
#[derive(Clone, Copy, Debug)]
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
impl Pending {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pending {
            t: 0.0,
            pos: [0.0; 3],
            name: crate::sim_fallshome::FallSound::Empty,
            gain: 0.0,
            rate: 0.0,
            amount: 0.0,
            who: 0.0,
            event: 0.0,
        }
    }
}
impl Default for Pending { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Life, src/game/shards.ts:153)
#[derive(Clone, Copy, Debug)]
pub struct Life {
    pub t: f64,
    pub n: f64,
}
impl Life {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Life {
            t: 0.0,
            n: 0.0,
        }
    }
}
impl Default for Life { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GlassShards, src/game/shards.ts:158)
#[derive(Clone, Debug)]
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
    pub pane_mem: JsMap<crate::Num, crate::shards::GlassShardsPaneMemValue>,
    pub crunch_n: f64,
    pub log: Vec<String>,
    pub ghost: DebrisChunkId,
    pub last_plate: Option<DebrisChunkId>,
    pub serial: f64,
}
impl GlassShards {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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
            pane_mem: JsMap::new(),
            crunch_n: 0.0,
            log: Vec::new(),
            ghost: DebrisChunkId(0),
            last_plate: None,
            serial: 0.0,
        }
    }
}
impl Default for GlassShards { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/shards.ts:165 GlassShards.breaks, src/game/shards.ts:165)
#[derive(Clone, Copy, Debug)]
pub struct GlassShardsBreaksItem {
    pub c: DebrisChunkId,
    pub speed: f64,
}
impl GlassShardsBreaksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsBreaksItem {
            c: DebrisChunkId(0),
            speed: 0.0,
        }
    }
}
impl Default for GlassShardsBreaksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:171 GlassShards.deferred, src/game/shards.ts:171)
#[derive(Clone, Copy, Debug)]
pub struct GlassShardsDeferredItem {
    pub pos: V3,
    pub dir: V3,
    pub count: f64,
    pub o: crate::shards::SprayOpts,
    pub seed: f64,
}
impl GlassShardsDeferredItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsDeferredItem {
            pos: [0.0; 3],
            dir: [0.0; 3],
            count: 0.0,
            o: crate::shards::SprayOpts::zeroed(),
            seed: 0.0,
        }
    }
}
impl Default for GlassShardsDeferredItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SprayOpts, src/game/shards.ts:674)
#[derive(Clone, Copy, Debug)]
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
impl SprayOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SprayOpts {
            speed: 0.0,
            speed_jit: 0.0,
            spread: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
            aspect: 0.0,
            tumble: 0.0,
            nrm_jit: 0.0,
            pos_jit: 0.0,
            vadd: None,
            color: [0.0; 3],
        }
    }
}
impl Default for SprayOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:175 GlassShards.paneMem, src/game/shards.ts:175)
#[derive(Clone, Copy, Debug)]
pub struct GlassShardsPaneMemValue {
    pub hits: f64,
    pub sites: f64,
    pub t: f64,
    pub cause: crate::shards::GlassShardsPaneMemValueCause,
    pub diced_t: f64,
}
impl GlassShardsPaneMemValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsPaneMemValue {
            hits: 0.0,
            sites: 0.0,
            t: 0.0,
            cause: crate::shards::GlassShardsPaneMemValueCause::Blast,
            diced_t: 0.0,
        }
    }
}
impl Default for GlassShardsPaneMemValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassShardsPaneMemValueCause {
    Blast,
    Hit,
}
impl GlassShardsPaneMemValueCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassShardsPaneMemValueCause::Blast => "blast",
            GlassShardsPaneMemValueCause::Hit => "hit",
        }
    }
}

/// (TypeScript: inline at src/game/shards.ts:337 cutPlates(fr), src/game/shards.ts:693)
#[derive(Clone, Copy, Debug)]
pub struct GlassShardsCutPlatesFr {
    pub u: V3,
    pub v: V3,
    pub lu: f64,
    pub lv: f64,
    pub mid: V3,
}
impl GlassShardsCutPlatesFr {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsCutPlatesFr {
            u: [0.0; 3],
            v: [0.0; 3],
            lu: 0.0,
            lv: 0.0,
            mid: [0.0; 3],
        }
    }
}
impl Default for GlassShardsCutPlatesFr { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:357 plate(cut), src/game/shards.ts:357)
#[derive(Clone, Debug)]
pub struct GlassShardsPlateCut {
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub dropped: f64,
}
impl GlassShardsPlateCut {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsPlateCut {
            cut: None,
            dropped: 0.0,
        }
    }
}
impl Default for GlassShardsPlateCut { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:570 fallOf ->, src/game/shards.ts:570)
#[derive(Clone, Copy, Debug)]
pub struct GlassShardsFallOf {
    pub t: f64,
    pub pos: V3,
}
impl GlassShardsFallOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsFallOf {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for GlassShardsFallOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:571 schedule(fall), src/game/shards.ts:571)
#[derive(Clone, Copy, Debug)]
pub struct GlassShardsScheduleFall {
    pub t: f64,
    pub pos: V3,
}
impl GlassShardsScheduleFall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsScheduleFall {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for GlassShardsScheduleFall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shards.ts:577 landing(fall), src/game/shards.ts:577)
#[derive(Clone, Copy, Debug)]
pub struct GlassShardsLandingFall {
    pub t: f64,
    pub pos: V3,
}
impl GlassShardsLandingFall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassShardsLandingFall {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for GlassShardsLandingFall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/shards.ts:747 cutPlateBox ->, src/game/shards.ts:747)
#[derive(Clone, Debug)]
pub struct CutPlateBox {
    pub box_: BoxId,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub dropped: f64,
    pub poly: Vec<[f64; 2]>,
}
impl CutPlateBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CutPlateBox {
            box_: BoxId(0),
            cut: None,
            dropped: 0.0,
            poly: Vec::new(),
        }
    }
}
impl Default for CutPlateBox { fn default() -> Self { Self::zeroed() } }

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
