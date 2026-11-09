//! The types of `edge_look`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreakLike {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: String,
    pub box_: BoxId,
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub yb: Vec<f64>,
    pub open: Vec<u8>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Rubble {
    pub box_: FrameBoxId,
    pub t: f64,
    pub t_2: f64,
    pub p0: V3,
    pub p1: V3,
    pub vy: f64,
    pub q0: Quat,
    pub q1: Quat,
    pub foot: Option<DebrisChunkId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Frame {
    pub b: BoxId,
    pub along_x: bool,
    pub cs: f64,
    pub sn: f64,
    pub t: f64,
    pub l: f64,
    pub n: f64,
    pub w: f64,
    pub y0: f64,
    pub y_top: f64,
    pub y_sill: f64,
    pub along: V3,
    pub nrm: V3,
}

#[derive(Clone, Debug, ToBytes)]
pub struct EdgeLook {
    pub decor: JsMap<crate::Num, Vec<FrameBoxId>>,
    pub dirty: JsMap<crate::Num, crate::edge_look::BreakLike>,
    pub rubble: Vec<crate::edge_look::Rubble>,
    pub flying: f64,
    pub serial: f64,
    pub room: f64,
    pub blasted: JsMap<crate::Num, Vec<[f64; 2]>>,
    pub todo: Vec<crate::edge_look::EdgeLookTodoItem>,
    pub held: bool,
    pub foot_book: crate::debris::FootBook,
}
impl EdgeLook {
    /// what a new one holds
    pub fn blank() -> Self {
        EdgeLook {
            decor: JsMap::new(),
            dirty: JsMap::new(),
            rubble: Vec::new(),
            flying: 0.0,
            serial: 0.0,
            room: 1000000000.0,
            blasted: JsMap::new(),
            todo: Vec::new(),
            held: false,
            foot_book: crate::debris::FootBook { shoves: 0.0, stats: crate::debris::FootBookStats { kicked: 0.0, off_man: 0.0, walled: 0.0, gave_up: 0.0 } },
        }
    }
}
impl Default for EdgeLook { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct EdgeLookTodoItem {
    pub b: crate::edge_look::BreakLike,
    pub f: Option<crate::edge_look::Frame>,
    pub c: f64,
    pub dir: Option<V3>,
    pub blast: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EdgeLookModEDGE {
    pub on: bool,
    pub max_decor: f64,
    pub per_wall: f64,
    pub max_rubble: f64,
    pub box_slack: f64,
    pub port_out: f64,
    pub hole_out: f64,
    pub hole_top: f64,
    pub sill_hole: f64,
    pub sill_port: f64,
    pub run_k: f64,
    pub lintel_port: f64,
    pub lintel_hole: f64,
    pub lintel_odds: f64,
    pub lintel_cut: f64,
    pub per_upper: f64,
    pub per_sill: f64,
    pub per_cover: f64,
    pub spread: f64,
    pub throw_fire: f64,
    pub throw_blast: f64,
    pub blast: crate::edge_look::EdgeLookModEDGEBlast,
    pub cut_heap: crate::edge_look::EdgeLookModEDGECutHeap,
    pub scorch: crate::edge_look::EdgeLookModEDGEScorch,
    pub scorch_decals: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EdgeLookModEDGEBlast {
    pub more: f64,
    pub lumps: f64,
    pub slabs: f64,
    pub slab_k: f64,
    pub flung: f64,
    pub near_k: f64,
    pub flake_k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EdgeLookModEDGECutHeap {
    pub slabs: f64,
    pub slab: [f64; 2],
    pub slab_t: f64,
    pub smalls: f64,
    pub small: [f64; 2],
    pub tone: f64,
    pub core_paper: f64,
    pub core_tone: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EdgeLookModEDGEScorch {
    pub near: f64,
    pub far: f64,
    pub floor: f64,
    pub soot_near: f64,
    pub soot_far: f64,
    pub soot_floor: f64,
    pub rim: f64,
    pub soot_rim: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct EdgeLookMod {
    /// (src/game/edgeLook.ts:40 EDGE)
    pub edge: crate::edge_look::EdgeLookModEDGE,
}
impl EdgeLookMod {
    pub fn blank() -> Self {
        EdgeLookMod {
            edge: crate::edge_look::EdgeLookModEDGE { on: true, max_decor: 480.0, per_wall: 150.0, max_rubble: 520.0, box_slack: 160.0, port_out: 0.25, hole_out: 0.12, hole_top: 0.6, sill_hole: 0.26, sill_port: 0.14, run_k: 0.27, lintel_port: 0.42, lintel_hole: 0.44, lintel_odds: 0.85, lintel_cut: 0.52, per_upper: 9.0, per_sill: 6.0, per_cover: 2.0, spread: 4.0, throw_fire: 0.8, throw_blast: 0.9, blast: crate::edge_look::EdgeLookModEDGEBlast { more: 2.0, lumps: 0.62, slabs: 0.3, slab_k: 1.7, flung: 0.1, near_k: 0.7, flake_k: 2.0 }, cut_heap: crate::edge_look::EdgeLookModEDGECutHeap { slabs: 0.07, slab: [0.15, 0.34], slab_t: 0.03, smalls: 0.42, small: [0.35, 0.6], tone: 0.22, core_paper: 0.45, core_tone: 0.3 }, scorch: crate::edge_look::EdgeLookModEDGEScorch { near: 1.3, far: 1.0, floor: 1.3, soot_near: 1.0, soot_far: 0.75, soot_floor: 0.5, rim: 0.95, soot_rim: 0.85 }, scorch_decals: true },
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct EdgeLookStats {
    pub decor: i32,
    pub rubble: i32,
    pub flying: i32,
    pub scorch: i32,
    pub shed: i32,
    pub capped: i32,
    pub rebuilds: i32,
}

/// (src/game/edgeLook.ts:87 MKEY_LOOK)
pub const MKEY_LOOK: f64 = 150994944.0;

/// (src/game/edgeLook.ts:88 STEEL: a table nothing writes)
pub fn steel() -> V3 { [0.5, 0.52, 0.54] }

/// (src/game/edgeLook.ts:89 UP: a table nothing writes)
pub fn up() -> V3 { [0.0, 1.0, 0.0] }
