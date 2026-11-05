//! The types of `debris`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Debris, src/game/debris.ts:81)
#[derive(Clone, Debug)]
pub struct Debris {
    pub list: Vec<DebrisChunkId>,
    pub serial: f64,
    pub frame: f64,
    pub stats: crate::debris::DebrisStats,
    pub fling: Option<V3>,
    pub serial0: f64,
    pub impulses: f64,
    pub foot_mu: f64,
    pub shoves: f64,
    pub scuff_at: f64,
    pub feet_stats: crate::debris::DebrisFeetStats,
}
impl Debris {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Debris {
            list: Vec::new(),
            serial: 1.0,
            frame: 0.0,
            stats: crate::debris::DebrisStats { spawned: 0.0, refused: 0.0, evicted: 0.0, landed: 0.0, peak_live: 0.0, peak_moving: 0.0, kicked: 0.0, told: 0.0 },
            fling: None,
            serial0: 1.0,
            impulses: 0.0,
            foot_mu: 0.3,
            shoves: 0.0,
            scuff_at: -1.0,
            feet_stats: crate::debris::DebrisFeetStats { kicked: 0.0, off_man: 0.0, walled: 0.0, gave_up: 0.0 },
        }
    }
}
impl Default for Debris { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/debris.ts:85 Debris.stats, src/game/debris.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct DebrisStats {
    pub spawned: f64,
    pub refused: f64,
    pub evicted: f64,
    pub landed: f64,
    pub peak_live: f64,
    pub peak_moving: f64,
    pub kicked: f64,
    pub told: f64,
}
impl DebrisStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DebrisStats {
            spawned: 0.0,
            refused: 0.0,
            evicted: 0.0,
            landed: 0.0,
            peak_live: 0.0,
            peak_moving: 0.0,
            kicked: 0.0,
            told: 0.0,
        }
    }
}
impl Default for DebrisStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:341 Debris.feetStats, src/game/debris.ts:341)
#[derive(Clone, Copy, Debug)]
pub struct DebrisFeetStats {
    pub kicked: f64,
    pub off_man: f64,
    pub walled: f64,
    pub gave_up: f64,
}
impl DebrisFeetStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DebrisFeetStats {
            kicked: 0.0,
            off_man: 0.0,
            walled: 0.0,
            gave_up: 0.0,
        }
    }
}
impl Default for DebrisFeetStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FootBook, src/game/debris.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct FootBook {
    pub shoves: f64,
    pub stats: crate::debris::FootBookStats,
}
impl FootBook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FootBook {
            shoves: 0.0,
            stats: crate::debris::FootBookStats::zeroed(),
        }
    }
}
impl Default for FootBook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:57 FootBook.stats, src/game/debris.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct FootBookStats {
    pub kicked: f64,
    pub off_man: f64,
    pub walled: f64,
    pub gave_up: f64,
}
impl FootBookStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FootBookStats {
            kicked: 0.0,
            off_man: 0.0,
            walled: 0.0,
            gave_up: 0.0,
        }
    }
}
impl Default for FootBookStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChunkKind {
    Tile,
    Shard,
    Housing,
    Clutter,
    Binder,
    Chunk,
}
impl ChunkKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChunkKind::Tile => "tile",
            ChunkKind::Shard => "shard",
            ChunkKind::Housing => "housing",
            ChunkKind::Clutter => "clutter",
            ChunkKind::Binder => "binder",
            ChunkKind::Chunk => "chunk",
        }
    }
}

/// (TypeScript: Chunk, src/game/debris.ts:25)
#[derive(Clone, Debug)]
pub struct Chunk {
    pub kind: crate::debris::ChunkKind,
    pub box_: BoxId,
    pub v: V3,
    pub w: V3,
    pub rest: bool,
    pub still_t: f64,
    pub age: f64,
    pub sounded: bool,
    pub level: f64,
    pub sfx: Vec<crate::debris::ChunkSfxItem>,
    pub serial: f64,
    pub who: f64,
    pub on: Option<PropId>,
    pub on_pose: f64,
    pub glass: Option<crate::shards::PlateMeta>,
    pub contact: Option<crate::debris::ChunkContact>,
    pub mat: Option<crate::break_table::BreakMeta>,
    pub mate: Option<BoxId>,
    pub foot: Option<bool>,
    pub foot_at: Option<f64>,
    pub foot_n: Option<f64>,
    pub foot_side: Option<f64>,
    pub dress: Option<bool>,
    pub lift: Option<f64>,
}
impl Chunk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Chunk {
            kind: crate::debris::ChunkKind::Tile,
            box_: BoxId(0),
            v: [0.0; 3],
            w: [0.0; 3],
            rest: false,
            still_t: 0.0,
            age: 0.0,
            sounded: false,
            level: 0.0,
            sfx: Vec::new(),
            serial: 0.0,
            who: 0.0,
            on: None,
            on_pose: 0.0,
            glass: None,
            contact: None,
            mat: None,
            mate: None,
            foot: None,
            foot_at: None,
            foot_n: None,
            foot_side: None,
            dress: None,
            lift: None,
        }
    }
}
impl Default for Chunk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:30 Chunk.sfx, src/game/debris.ts:30)
#[derive(Clone, Debug)]
pub struct ChunkSfxItem {
    pub name: String,
    pub rate: f64,
    pub gain: f64,
}
impl ChunkSfxItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChunkSfxItem {
            name: String::new(),
            rate: 0.0,
            gain: 0.0,
        }
    }
}
impl Default for ChunkSfxItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:38 Chunk.contact, src/game/debris.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct ChunkContact {
    pub keep_floor: f64,
    pub keep_wall: f64,
    pub rest: f64,
}
impl ChunkContact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChunkContact {
            keep_floor: 0.0,
            keep_wall: 0.0,
            rest: 0.0,
        }
    }
}
impl Default for ChunkContact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FootMan, src/game/debris.ts:59)
#[derive(Clone, Copy, Debug)]
pub struct FootMan {
    pub pos: V3,
    pub vel: V3,
}
impl FootMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FootMan {
            pos: [0.0; 3],
            vel: [0.0; 3],
        }
    }
}
impl Default for FootMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FootKnobs, src/game/debris.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct FootKnobs {
    pub radius: f64,
    pub reach: f64,
    pub lead: f64,
    pub slide: f64,
    pub inside: f64,
    pub floor: f64,
    pub heavy: f64,
    pub walk: f64,
    pub base: f64,
    pub gain: f64,
    pub aside: f64,
    pub v_max: f64,
    pub off: f64,
    pub spin: f64,
    pub per_frame: f64,
    pub cooldown: f64,
    pub give_up: f64,
    pub give_up_secs: f64,
    pub scuff: f64,
}
impl FootKnobs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FootKnobs {
            radius: 0.0,
            reach: 0.0,
            lead: 0.0,
            slide: 0.0,
            inside: 0.0,
            floor: 0.0,
            heavy: 0.0,
            walk: 0.0,
            base: 0.0,
            gain: 0.0,
            aside: 0.0,
            v_max: 0.0,
            off: 0.0,
            spin: 0.0,
            per_frame: 0.0,
            cooldown: 0.0,
            give_up: 0.0,
            give_up_secs: 0.0,
            scuff: 0.0,
        }
    }
}
impl Default for FootKnobs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:67 DEBRIS_IMP, src/game/debris.ts:67 DEBRIS_IMP)
#[derive(Clone, Copy, Debug)]
pub struct DEBRISIMP {
    pub gain: f64,
    pub up_k: f64,
    pub v_max: f64,
    pub wake: f64,
    pub tumble: f64,
}
impl DEBRISIMP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DEBRISIMP {
            gain: 0.0,
            up_k: 0.0,
            v_max: 0.0,
            wake: 0.0,
            tumble: 0.0,
        }
    }
}
impl Default for DEBRISIMP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:69 LANDING, src/game/debris.ts:69)
#[derive(Clone, Debug)]
pub struct LANDINGValue {
    pub level: f64,
    pub sfx: Vec<crate::debris::LANDINGValueSfxItem>,
}
impl LANDINGValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LANDINGValue {
            level: 0.0,
            sfx: Vec::new(),
        }
    }
}
impl Default for LANDINGValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:69 LANDINGValue.sfx, src/game/debris.ts:69)
#[derive(Clone, Debug)]
pub struct LANDINGValueSfxItem {
    pub name: String,
    pub rate: f64,
    pub gain: f64,
}
impl LANDINGValueSfxItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LANDINGValueSfxItem {
            name: String::new(),
            rate: 0.0,
            gain: 0.0,
        }
    }
}
impl Default for LANDINGValueSfxItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/debris.ts:278 footGap ->, src/game/debris.ts:278)
#[derive(Clone, Copy, Debug)]
pub struct DebrisFootGap {
    pub gap: f64,
    pub dx: f64,
    pub dz: f64,
    pub lx: f64,
    pub lz: f64,
}
impl DebrisFootGap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DebrisFootGap {
            gap: 0.0,
            dx: 0.0,
            dz: 0.0,
            lx: 0.0,
            lz: 0.0,
        }
    }
}
impl Default for DebrisFootGap { fn default() -> Self { Self::zeroed() } }

/// (src/game/debris.ts:62 G)
pub const G: f64 = 9.81;

/// (src/game/debris.ts:63 SLEEP_V)
pub const SLEEP_V: f64 = 0.22;

/// (src/game/debris.ts:63 SLEEP_W)
pub const SLEEP_W: f64 = 1.2;

/// (src/game/debris.ts:63 SLEEP_T)
pub const SLEEP_T: f64 = 0.25;

/// (src/game/debris.ts:67 DEBRIS_IMP: a table nothing writes)
pub fn debris_imp() -> crate::debris::DEBRISIMP { crate::debris::DEBRISIMP { gain: 0.35, up_k: 0.7, v_max: 3.0, wake: 0.1, tumble: 0.5 } }

/// (src/game/debris.ts:69 LANDING: a table nothing writes)
pub fn landing() -> JsMap<String, crate::debris::LANDINGValue> { { let mut t = JsMap::new(); t.set(String::from("tile"), crate::debris::LANDINGValue { level: 0.4, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 0.75, gain: 0.9 }, crate::debris::LANDINGValueSfxItem { name: String::from("magDrop"), rate: 0.5, gain: 0.35 }] }); t.set(String::from("housing"), crate::debris::LANDINGValue { level: 0.5, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 0.6, gain: 1.0 }, crate::debris::LANDINGValueSfxItem { name: String::from("lightBreak"), rate: 0.8, gain: 0.45 }] }); t.set(String::from("clutter"), crate::debris::LANDINGValue { level: 0.25, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 1.45, gain: 0.5 }] }); t.set(String::from("binder"), crate::debris::LANDINGValue { level: 0.25, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 1.1, gain: 0.55 }] }); t.set(String::from("chunk"), crate::debris::LANDINGValue { level: 0.2, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("propThud"), rate: 1.3, gain: 0.4 }] }); t.set(String::from("shard"), crate::debris::LANDINGValue { level: 0.3, sfx: alloc::vec![crate::debris::LANDINGValueSfxItem { name: String::from("glassCrack"), rate: 1.9, gain: 0.3 }, crate::debris::LANDINGValueSfxItem { name: String::from("casingTink"), rate: 0.6, gain: 0.35 }] }); t } }
