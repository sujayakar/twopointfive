//! The types of `sim_propshome`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct PropCols {
    pub x: f64,
    pub z: f64,
    pub yaw: f64,
    pub flags: f64,
    pub idle: f64,
    pub bytes: f64,
    pub who: f64,
    pub w: f64,
    pub id: f64,
    pub vx: f64,
    pub vz: f64,
    pub cs: f64,
    pub sn: f64,
    pub push_t: f64,
    pub noise_t: f64,
    pub row: f64,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct PropStore {
    pub total: f64,
    pub o: crate::sim_propshome::PropStoreO,
}
impl PropStore {
    /// what a new one holds
    pub fn blank() -> Self {
        PropStore {
            total: 0.0,
            o: crate::sim_propshome::PropStoreO::default(),
        }
    }
}
impl Default for PropStore { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PropStoreO {
    pub x: f64,
    pub z: f64,
    pub yaw: f64,
    pub flags: f64,
    pub idle: f64,
    pub bytes: f64,
    pub who: f64,
    pub w: f64,
    pub id: f64,
    pub vx: f64,
    pub vz: f64,
    pub cs: f64,
    pub sn: f64,
    pub push_t: f64,
    pub noise_t: f64,
    pub row: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PropIo {
    pub max_contacts: f64,
    pub max_leaves: f64,
    pub max_ways: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PD {
    pub chair: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PK {
    pub quiet: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PY {
    pub half: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PE {
    pub rolling: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimPropshomeModPROPSSTEPPED {
    pub there: f64,
    pub noises: f64,
    pub swept: f64,
    pub guided: f64,
    pub pairs: f64,
    pub walls: f64,
    pub held: f64,
    pub substeps: f64,
    pub settled: f64,
    pub settle_walls: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimPropshomeMod {
    /// (src/sim/propshome.ts:103 IO)
    pub io: Option<crate::sim_propshome::PropIo>,
    /// (src/sim/propshome.ts:120 PROPS_STEPPED)
    pub props_stepped: crate::sim_propshome::SimPropshomeModPROPSSTEPPED,
}
impl SimPropshomeMod {
    pub fn blank() -> Self {
        SimPropshomeMod {
            io: None,
            props_stepped: crate::sim_propshome::SimPropshomeModPROPSSTEPPED { there: 0.0, noises: 0.0, swept: 0.0, guided: 0.0, pairs: 0.0, walls: 0.0, held: 0.0, substeps: 0.0, settled: 0.0, settle_walls: 0.0 },
        }
    }
}

/// (src/sim/propshome.ts:23 PROP_CAP)
pub const PROP_CAP: f64 = 128.0;

/// (src/sim/propshome.ts:27 PROP_PARTS)
pub const PROP_PARTS: f64 = 512.0;

/// (src/sim/propshome.ts:30 PROP_F64: a table nothing writes)
pub fn prop_f64() -> [String; 11] { [String::from("x"), String::from("z"), String::from("yaw"), String::from("vx"), String::from("vz"), String::from("w"), String::from("cs"), String::from("sn"), String::from("idle"), String::from("pushT"), String::from("noiseT")] }

/// (src/sim/propshome.ts:31 PROP_U32: a table nothing writes)
pub fn prop_u32() -> [String; 4] { [String::from("flags"), String::from("who"), String::from("id"), String::from("row")] }

/// (src/sim/propshome.ts:33 PROP_HEAD)
pub const PROP_HEAD: f64 = 16.0;

/// (src/sim/propshome.ts:87 PD: a table nothing writes)
pub fn pd() -> crate::sim_propshome::PD { crate::sim_propshome::PD { chair: 10.0 } }

/// (src/sim/propshome.ts:89 PK: a table nothing writes)
pub fn pk() -> crate::sim_propshome::PK { crate::sim_propshome::PK { quiet: 7.0 } }

/// (src/sim/propshome.ts:91 PY: a table nothing writes)
pub fn py() -> crate::sim_propshome::PY { crate::sim_propshome::PY { half: 4.0 } }

/// (src/sim/propshome.ts:93 PE: a table nothing writes)
pub fn pe() -> crate::sim_propshome::PE { crate::sim_propshome::PE { rolling: 9.0 } }

/// (src/sim/propshome.ts:94 PROP_EV)
pub const PROP_EV: f64 = 10.0;

/// (src/sim/propshome.ts:99 PROP_PARS)
pub const PROP_PARS: f64 = 24.0;
