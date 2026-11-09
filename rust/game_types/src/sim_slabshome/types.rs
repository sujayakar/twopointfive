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
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub home: crate::Opaque,
    pub b8: f64,
    pub b4: f64,
    pub cap: f64,
}
impl SlabStore {
    /// what a new one holds
    pub fn blank() -> Self {
        SlabStore {
            doors: 0.0,
            o: crate::sim_slabshome::SlabStoreO::default(),
            home: crate::Opaque,
            b8: 0.0,
            b4: 0.0,
            cap: 0.0,
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
pub struct SlabIo {
    pub x: crate::sim_sim::SimExports,
    pub par: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SLF {
    pub rest: f64,
    pub landed: f64,
    pub back: f64,
    pub bits: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SP {
    pub dt: f64,
    pub row: f64,
    pub height: f64,
    pub thick: f64,
    pub hu: f64,
    pub closed: f64,
    pub fcx: f64,
    pub fcz: f64,
    pub but: f64,
    pub ask: f64,
    pub scrape: f64,
    pub slam: f64,
    pub slide: f64,
    pub bounce: f64,
    pub tip: f64,
    pub landed: f64,
    pub land_x: f64,
    pub land_z: f64,
    pub rested: f64,
    pub nudged: f64,
    pub nudge_max: f64,
    pub laid: f64,
    pub rays: f64,
    pub gathers: f64,
    pub substeps: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimSlabshomeModSLABSSTEPPED {
    pub there: f64,
    pub here: f64,
    pub host: f64,
    pub asked: f64,
    pub differ: f64,
    pub first: String,
    pub rays: f64,
    pub gathers: f64,
    pub substeps: f64,
    pub landed: f64,
    pub rested: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimSlabshomeMod {
    /// (src/sim/slabshome.ts:120 IO)
    pub io: Option<crate::sim_slabshome::SlabIo>,
    /// (src/sim/slabshome.ts:135 SLABS_STEPPED)
    pub slabs_stepped: crate::sim_slabshome::SimSlabshomeModSLABSSTEPPED,
}
impl SimSlabshomeMod {
    pub fn blank() -> Self {
        SimSlabshomeMod {
            io: None,
            slabs_stepped: crate::sim_slabshome::SimSlabshomeModSLABSSTEPPED { there: 0.0, here: 0.0, host: 0.0, asked: 0.0, differ: 0.0, first: String::from(""), rays: 0.0, gathers: 0.0, substeps: 0.0, landed: 0.0, rested: 0.0 },
        }
    }
}

/// (src/sim/slabshome.ts:22 SLAB_CAP)
pub const SLAB_CAP: f64 = 64.0;

/// (src/sim/slabshome.ts:25 SLAB_F64: a table nothing writes)
pub fn slab_f64() -> [String; 17] { [String::from("px"), String::from("py"), String::from("pz"), String::from("vx"), String::from("vy"), String::from("vz"), String::from("a"), String::from("spin"), String::from("tip"), String::from("tipV"), String::from("t"), String::from("ang"), String::from("wedged"), String::from("bw"), String::from("bl"), String::from("bx"), String::from("by")] }

/// (src/sim/slabshome.ts:26 SLAB_U32: a table nothing writes)
pub fn slab_u32() -> [String; 4] { [String::from("flags"), String::from("who"), String::from("door"), String::from("row")] }

/// (src/sim/slabshome.ts:28 SLAB_HEAD)
pub const SLAB_HEAD: f64 = 16.0;

/// (src/sim/slabshome.ts:31 SLF: a table nothing writes)
pub fn slf() -> crate::sim_slabshome::SLF { crate::sim_slabshome::SLF { rest: 1.0, landed: 2.0, back: 4.0, bits: 8.0 } }

/// (src/sim/slabshome.ts:41 SLAB_BIT)
pub const SLAB_BIT: f64 = 64.0;

/// (src/sim/slabshome.ts:44 SWAPPED: a table nothing writes)
pub fn swapped() -> [String; 3] { [String::from("flags"), String::from("who"), String::from("door")] }

/// (src/sim/slabshome.ts:112 SP: a table nothing writes)
pub fn sp() -> crate::sim_slabshome::SP { crate::sim_slabshome::SP { dt: 0.0, row: 1.0, height: 2.0, thick: 3.0, hu: 4.0, closed: 5.0, fcx: 6.0, fcz: 7.0, but: 8.0, ask: 9.0, scrape: 10.0, slam: 11.0, slide: 12.0, bounce: 13.0, tip: 14.0, landed: 16.0, land_x: 17.0, land_z: 18.0, rested: 19.0, nudged: 20.0, nudge_max: 21.0, laid: 22.0, rays: 23.0, gathers: 24.0, substeps: 25.0 } }

/// (src/sim/slabshome.ts:114 SLAB_PARS)
pub const SLAB_PARS: f64 = 32.0;

/// (src/sim/slabshome.ts:116 SLAB_STEPPED)
pub const SLAB_STEPPED: f64 = 0.0;

/// (src/sim/slabshome.ts:116 SLAB_HOST)
pub const SLAB_HOST: f64 = 1.0;
