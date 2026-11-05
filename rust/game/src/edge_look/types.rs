//! The types of `edge_look`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: EdgeLook, src/game/edgeLook.ts:84)
#[derive(Clone, Debug)]
pub struct EdgeLook {
    pub decor: JsMap<crate::Num, Vec<BoxId>>,
    pub dirty: JsMap<crate::Num, crate::edge_look::BreakLike>,
    pub rubble: Vec<crate::edge_look::Rubble>,
    pub flying: f64,
    pub serial: f64,
    pub room: f64,
    pub blasted: JsMap<crate::Num, Vec<[f64; 2]>>,
    pub todo: Vec<crate::edge_look::EdgeLookTodoItem>,
    pub held: bool,
    pub last: f64,
    pub foot_book: crate::debris::FootBook,
}
impl EdgeLook {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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
            last: 0.0,
            foot_book: crate::debris::FootBook { shoves: 0.0, stats: crate::debris::FootBookStats { kicked: 0.0, off_man: 0.0, walled: 0.0, gave_up: 0.0 } },
        }
    }
}
impl Default for EdgeLook { fn default() -> Self { Self::blank() } }

/// (TypeScript: BreakLike, src/game/edgeLook.ts:75)
#[derive(Clone, Debug)]
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
impl BreakLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakLike {
            id: 0.0,
            kind: crate::wallbreak::BKind::Wall,
            surf: String::new(),
            box_: BoxId(0),
            nx: 0.0,
            ny: 0.0,
            nz: 0.0,
            yb: Vec::new(),
            open: Vec::new(),
        }
    }
}
impl Default for BreakLike { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Rubble, src/game/edgeLook.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct Rubble {
    pub box_: BoxId,
    pub t: f64,
    pub t_2: f64,
    pub p0: V3,
    pub p1: V3,
    pub vy: f64,
    pub q0: Quat,
    pub q1: Quat,
    pub foot: Option<DebrisChunkId>,
}
impl Rubble {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rubble {
            box_: BoxId(0),
            t: 0.0,
            t_2: 0.0,
            p0: [0.0; 3],
            p1: [0.0; 3],
            vy: 0.0,
            q0: [0.0; 4],
            q1: [0.0; 4],
            foot: None,
        }
    }
}
impl Default for Rubble { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/edgeLook.ts:112 EdgeLook.todo, src/game/edgeLook.ts:112)
#[derive(Clone, Debug)]
pub struct EdgeLookTodoItem {
    pub b: crate::edge_look::BreakLike,
    pub f: Option<crate::edge_look::Frame>,
    pub c: f64,
    pub dir: Option<V3>,
    pub blast: bool,
}
impl EdgeLookTodoItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EdgeLookTodoItem {
            b: crate::edge_look::BreakLike::zeroed(),
            f: None,
            c: 0.0,
            dir: None,
            blast: false,
        }
    }
}
impl Default for EdgeLookTodoItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Frame, src/game/edgeLook.ts:82)
#[derive(Clone, Copy, Debug)]
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
impl Frame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Frame {
            b: BoxId(0),
            along_x: false,
            cs: 0.0,
            sn: 0.0,
            t: 0.0,
            l: 0.0,
            n: 0.0,
            w: 0.0,
            y0: 0.0,
            y_top: 0.0,
            y_sill: 0.0,
            along: [0.0; 3],
            nrm: [0.0; 3],
        }
    }
}
impl Default for Frame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/edgeLook.ts:40 EDGE, src/game/edgeLook.ts:40)
#[derive(Clone, Copy, Debug)]
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
impl EdgeLookModEDGE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EdgeLookModEDGE {
            on: false,
            max_decor: 0.0,
            per_wall: 0.0,
            max_rubble: 0.0,
            box_slack: 0.0,
            port_out: 0.0,
            hole_out: 0.0,
            hole_top: 0.0,
            sill_hole: 0.0,
            sill_port: 0.0,
            run_k: 0.0,
            lintel_port: 0.0,
            lintel_hole: 0.0,
            lintel_odds: 0.0,
            lintel_cut: 0.0,
            per_upper: 0.0,
            per_sill: 0.0,
            per_cover: 0.0,
            spread: 0.0,
            throw_fire: 0.0,
            throw_blast: 0.0,
            blast: crate::edge_look::EdgeLookModEDGEBlast::zeroed(),
            cut_heap: crate::edge_look::EdgeLookModEDGECutHeap::zeroed(),
            scorch: crate::edge_look::EdgeLookModEDGEScorch::zeroed(),
            scorch_decals: false,
        }
    }
}
impl Default for EdgeLookModEDGE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/edgeLook.ts:57 EdgeLookModEDGE.blast, src/game/edgeLook.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct EdgeLookModEDGEBlast {
    pub more: f64,
    pub lumps: f64,
    pub slabs: f64,
    pub slab_k: f64,
    pub flung: f64,
    pub near_k: f64,
    pub flake_k: f64,
}
impl EdgeLookModEDGEBlast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EdgeLookModEDGEBlast {
            more: 0.0,
            lumps: 0.0,
            slabs: 0.0,
            slab_k: 0.0,
            flung: 0.0,
            near_k: 0.0,
            flake_k: 0.0,
        }
    }
}
impl Default for EdgeLookModEDGEBlast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/edgeLook.ts:61 EdgeLookModEDGE.cutHeap, src/game/edgeLook.ts:61)
#[derive(Clone, Copy, Debug)]
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
impl EdgeLookModEDGECutHeap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EdgeLookModEDGECutHeap {
            slabs: 0.0,
            slab: [0.0; 2],
            slab_t: 0.0,
            smalls: 0.0,
            small: [0.0; 2],
            tone: 0.0,
            core_paper: 0.0,
            core_tone: 0.0,
        }
    }
}
impl Default for EdgeLookModEDGECutHeap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/edgeLook.ts:64 EdgeLookModEDGE.scorch, src/game/edgeLook.ts:64)
#[derive(Clone, Copy, Debug)]
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
impl EdgeLookModEDGEScorch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EdgeLookModEDGEScorch {
            near: 0.0,
            far: 0.0,
            floor: 0.0,
            soot_near: 0.0,
            soot_far: 0.0,
            soot_floor: 0.0,
            rim: 0.0,
            soot_rim: 0.0,
        }
    }
}
impl Default for EdgeLookModEDGEScorch { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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

/// (src/game/edgeLook.ts:77 MKEY_LOOK)
pub const MKEY_LOOK: f64 = 150994944.0;

/// (src/game/edgeLook.ts:78 STEEL: a table nothing writes)
pub fn steel() -> V3 { [0.5, 0.52, 0.54] }

/// (src/game/edgeLook.ts:78 CORE: a table nothing writes)
pub fn core() -> V3 { [0.8, 0.79, 0.75] }

/// (src/game/edgeLook.ts:78 PAPER: a table nothing writes)
pub fn paper() -> V3 { [0.52, 0.47, 0.39] }

/// (src/game/edgeLook.ts:79 UP: a table nothing writes)
pub fn up() -> V3 { [0.0, 1.0, 0.0] }
