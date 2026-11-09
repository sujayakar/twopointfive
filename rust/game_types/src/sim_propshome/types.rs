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
    pub epoch: f64,
    pub total: f64,
    pub o: crate::sim_propshome::PropStoreO,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub home: crate::Opaque,
    pub b8: f64,
    pub b4: f64,
    pub cap: f64,
}
impl PropStore {
    /// what a new one holds
    pub fn blank() -> Self {
        PropStore {
            epoch: -1.0,
            total: 0.0,
            o: crate::sim_propshome::PropStoreO::default(),
            home: crate::Opaque,
            b8: 0.0,
            b4: 0.0,
            cap: 0.0,
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
    pub x: crate::sim_sim::SimExports,
    pub def: f64,
    pub con: f64,
    pub leaf: f64,
    pub way: f64,
    pub ev: f64,
    pub par: f64,
    pub max_contacts: f64,
    pub max_leaves: f64,
    pub max_ways: f64,
    pub max_events: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PT {
    pub asleep: f64,
    pub dirty: f64,
    pub rolling: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PD {
    pub hx: f64,
    pub hz: f64,
    pub radius: f64,
    pub height: f64,
    pub mass: f64,
    pub inv_mass: f64,
    pub inv_i: f64,
    pub decel: f64,
    pub visc: f64,
    pub rest: f64,
    pub chair: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PK {
    pub x: f64,
    pub z: f64,
    pub vx: f64,
    pub vz: f64,
    pub radius: f64,
    pub who: f64,
    pub mass: f64,
    pub quiet: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PY {
    pub cx: f64,
    pub cz: f64,
    pub ux: f64,
    pub uz: f64,
    pub half: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PE {
    pub prop: f64,
    pub sound: f64,
    pub roll: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub who: f64,
    pub level: f64,
    pub speed: f64,
    pub rolling: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PP {
    pub dt: f64,
    pub props: f64,
    pub contacts: f64,
    pub leaves: f64,
    pub ways: f64,
    pub flags: f64,
    pub friction: f64,
    pub cas_decel: f64,
    pub cas_visc: f64,
    pub cas_spin: f64,
    pub cas_start: f64,
    pub cas_stop: f64,
    pub events: f64,
    pub wrong: f64,
    pub swept: f64,
    pub guided: f64,
    pub pairs: f64,
    pub walls: f64,
    pub held: f64,
    pub substeps: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PF {
    pub enabled: f64,
    pub castor: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimPropshomeModPROPSSTEPPED {
    pub there: f64,
    pub here: f64,
    pub noises: f64,
    pub asked: f64,
    pub differ: f64,
    pub first: String,
    pub swept: f64,
    pub guided: f64,
    pub pairs: f64,
    pub walls: f64,
    pub held: f64,
    pub substeps: f64,
    pub settled: f64,
    pub settled_here: f64,
    pub settle_walls: f64,
    pub settle_moved: f64,
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
            props_stepped: crate::sim_propshome::SimPropshomeModPROPSSTEPPED { there: 0.0, here: 0.0, noises: 0.0, asked: 0.0, differ: 0.0, first: String::from(""), swept: 0.0, guided: 0.0, pairs: 0.0, walls: 0.0, held: 0.0, substeps: 0.0, settled: 0.0, settled_here: 0.0, settle_walls: 0.0, settle_moved: 0.0 },
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

/// (src/sim/propshome.ts:35 PT: a table nothing writes)
pub fn pt() -> crate::sim_propshome::PT { crate::sim_propshome::PT { asleep: 1.0, dirty: 2.0, rolling: 4.0 } }

/// (src/sim/propshome.ts:37 PROP_SOUNDS: a table nothing writes)
pub fn prop_sounds() -> [String; 3] { [String::from("scrape"), String::from("thud"), String::from("roll")] }

/// (src/sim/propshome.ts:43 PROP_BIT)
pub const PROP_BIT: f64 = 32.0;

/// (src/sim/propshome.ts:83 SWAPPED: a table nothing writes)
pub fn swapped() -> [String; 3] { [String::from("flags"), String::from("who"), String::from("id")] }

/// (src/sim/propshome.ts:87 PD: a table nothing writes)
pub fn pd() -> crate::sim_propshome::PD { crate::sim_propshome::PD { hx: 0.0, hz: 1.0, radius: 2.0, height: 3.0, mass: 4.0, inv_mass: 5.0, inv_i: 6.0, decel: 7.0, visc: 8.0, rest: 9.0, chair: 10.0 } }

/// (src/sim/propshome.ts:89 PK: a table nothing writes)
pub fn pk() -> crate::sim_propshome::PK { crate::sim_propshome::PK { x: 0.0, z: 1.0, vx: 2.0, vz: 3.0, radius: 4.0, who: 5.0, mass: 6.0, quiet: 7.0 } }

/// (src/sim/propshome.ts:91 PY: a table nothing writes)
pub fn py() -> crate::sim_propshome::PY { crate::sim_propshome::PY { cx: 0.0, cz: 1.0, ux: 2.0, uz: 3.0, half: 4.0 } }

/// (src/sim/propshome.ts:93 PE: a table nothing writes)
pub fn pe() -> crate::sim_propshome::PE { crate::sim_propshome::PE { prop: 0.0, sound: 1.0, roll: 2.0, x: 3.0, y: 4.0, z: 5.0, who: 6.0, level: 7.0, speed: 8.0, rolling: 9.0 } }

/// (src/sim/propshome.ts:94 PROP_EV)
pub const PROP_EV: f64 = 10.0;

/// (src/sim/propshome.ts:96 PP: a table nothing writes)
pub fn pp() -> crate::sim_propshome::PP { crate::sim_propshome::PP { dt: 0.0, props: 1.0, contacts: 2.0, leaves: 3.0, ways: 4.0, flags: 5.0, friction: 6.0, cas_decel: 7.0, cas_visc: 8.0, cas_spin: 9.0, cas_start: 10.0, cas_stop: 11.0, events: 12.0, wrong: 13.0, swept: 14.0, guided: 15.0, pairs: 16.0, walls: 17.0, held: 18.0, substeps: 19.0 } }

/// (src/sim/propshome.ts:98 PF: a table nothing writes)
pub fn pf() -> crate::sim_propshome::PF { crate::sim_propshome::PF { enabled: 1.0, castor: 2.0 } }

/// (src/sim/propshome.ts:99 PROP_PARS)
pub const PROP_PARS: f64 = 24.0;
