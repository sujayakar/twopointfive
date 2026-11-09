//! The types of `sim_slabshome`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SlabNow {
    pub p: [f64; 3],
    pub v: [f64; 3],
    pub a: f64,
    pub spin: f64,
    pub tip: f64,
    pub tip_v: f64,
    pub side: f64,
    pub rest: bool,
    pub landed: bool,
    pub who: f64,
    pub t: f64,
    pub ang: f64,
    pub bits: Option<[f64; 4]>,
    pub wedged: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SlabCols {
    pub door: f64,
    pub t: f64,
    pub a: f64,
    pub flags: f64,
    pub bytes: f64,
    pub who: f64,
    pub vx: f64,
    pub vz: f64,
    pub row: f64,
    pub by: f64,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
    pub vy: f64,
    pub spin: f64,
    pub wedged: f64,
    pub tip: f64,
    pub tip_v: f64,
    pub ang: f64,
    pub bw: f64,
    pub bl: f64,
    pub bx: f64,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct SlabStore {
    pub doors: f64,
    pub o: crate::sim_slabshome::SlabStoreO,
}
impl SlabStore {
    /// what a new one holds
    pub fn blank() -> Self {
        SlabStore {
            doors: 0.0,
            o: crate::sim_slabshome::SlabStoreO::default(),
        }
    }
}
impl Default for SlabStore { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SlabStoreO {
    pub door: f64,
    pub t: f64,
    pub a: f64,
    pub flags: f64,
    pub bytes: f64,
    pub who: f64,
    pub vx: f64,
    pub vz: f64,
    pub row: f64,
    pub by: f64,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
    pub vy: f64,
    pub spin: f64,
    pub wedged: f64,
    pub tip: f64,
    pub tip_v: f64,
    pub ang: f64,
    pub bw: f64,
    pub bl: f64,
    pub bx: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SLF {
    pub rest: f64,
    pub landed: f64,
    pub back: f64,
    pub bits: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimSlabshomeModSLABSSTEPPED {
    pub there: f64,
    pub here: f64,
    pub host: f64,
    pub rays: f64,
    pub gathers: f64,
    pub substeps: f64,
    pub landed: f64,
    pub rested: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimSlabshomeMod {
    /// (src/sim/slabshome.ts:135 SLABS_STEPPED)
    pub slabs_stepped: crate::sim_slabshome::SimSlabshomeModSLABSSTEPPED,
}
impl SimSlabshomeMod {
    pub fn blank() -> Self {
        SimSlabshomeMod {
            slabs_stepped: crate::sim_slabshome::SimSlabshomeModSLABSSTEPPED { there: 0.0, here: 0.0, host: 0.0, rays: 0.0, gathers: 0.0, substeps: 0.0, landed: 0.0, rested: 0.0 },
        }
    }
}

/// (src/sim/slabshome.ts:22 SLAB_CAP)
pub const SLAB_CAP: f64 = 64.0;

/// (src/sim/slabshome.ts:25 SLAB_F64: a table nothing writes)
pub fn slab_f64() -> [String; 17] { [String::from("px"), String::from("py"), String::from("pz"), String::from("vx"), String::from("vy"), String::from("vz"), String::from("a"), String::from("spin"), String::from("tip"), String::from("tipV"), String::from("t"), String::from("ang"), String::from("wedged"), String::from("bw"), String::from("bl"), String::from("bx"), String::from("by")] }

/// (src/sim/slabshome.ts:28 SLAB_HEAD)
pub const SLAB_HEAD: f64 = 16.0;

/// (src/sim/slabshome.ts:31 SLF: a table nothing writes)
pub fn slf() -> crate::sim_slabshome::SLF { crate::sim_slabshome::SLF { rest: 1.0, landed: 2.0, back: 4.0, bits: 8.0 } }

/// (src/sim/slabshome.ts:41 SLAB_BIT)
pub const SLAB_BIT: f64 = 64.0;

/// (src/sim/slabshome.ts:44 SWAPPED: a table nothing writes)
pub fn swapped() -> [String; 3] { [String::from("flags"), String::from("who"), String::from("door")] }

