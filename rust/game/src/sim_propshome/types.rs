//! The types of `sim_propshome`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: PropCols, src/sim/propshome.ts:40)
#[derive(Clone, Copy, Debug)]
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
impl PropCols {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropCols {
            x: 0.0,
            z: 0.0,
            yaw: 0.0,
            flags: 0.0,
            idle: 0.0,
            bytes: 0.0,
            who: 0.0,
            w: 0.0,
            id: 0.0,
            vx: 0.0,
            vz: 0.0,
            cs: 0.0,
            sn: 0.0,
            push_t: 0.0,
            noise_t: 0.0,
            row: 0.0,
        }
    }
}
impl Default for PropCols { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PropStore, src/sim/propshome.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct PropStore {
    pub epoch: f64,
    pub total: f64,
    pub o: crate::sim_propshome::PropStoreO,
    pub home: crate::Opaque,
    pub b8: f64,
    pub b4: f64,
    pub cap: f64,
}
impl PropStore {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        PropStore {
            epoch: -1.0,
            total: 0.0,
            o: crate::sim_propshome::PropStoreO::zeroed(),
            home: crate::Opaque,
            b8: 0.0,
            b4: 0.0,
            cap: 0.0,
        }
    }
}
impl Default for PropStore { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/sim/colstore.ts:28 PropStore.O, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1611)
#[derive(Clone, Copy, Debug)]
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
impl PropStoreO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropStoreO {
            x: 0.0,
            z: 0.0,
            yaw: 0.0,
            flags: 0.0,
            idle: 0.0,
            bytes: 0.0,
            who: 0.0,
            w: 0.0,
            id: 0.0,
            vx: 0.0,
            vz: 0.0,
            cs: 0.0,
            sn: 0.0,
            push_t: 0.0,
            noise_t: 0.0,
            row: 0.0,
        }
    }
}
impl Default for PropStoreO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PropIo, src/sim/propshome.ts:102)
#[derive(Clone, Copy, Debug)]
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
impl PropIo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropIo {
            x: crate::sim_sim::SimExports::zeroed(),
            def: 0.0,
            con: 0.0,
            leaf: 0.0,
            way: 0.0,
            ev: 0.0,
            par: 0.0,
            max_contacts: 0.0,
            max_leaves: 0.0,
            max_ways: 0.0,
            max_events: 0.0,
        }
    }
}
impl Default for PropIo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:35 PT, src/sim/propshome.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct PT {
    pub asleep: f64,
    pub dirty: f64,
    pub rolling: f64,
}
impl PT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PT {
            asleep: 0.0,
            dirty: 0.0,
            rolling: 0.0,
        }
    }
}
impl Default for PT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:87 PD, src/sim/propshome.ts:87)
#[derive(Clone, Copy, Debug)]
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
impl PD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PD {
            hx: 0.0,
            hz: 0.0,
            radius: 0.0,
            height: 0.0,
            mass: 0.0,
            inv_mass: 0.0,
            inv_i: 0.0,
            decel: 0.0,
            visc: 0.0,
            rest: 0.0,
            chair: 0.0,
        }
    }
}
impl Default for PD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:89 PK, src/sim/propshome.ts:89)
#[derive(Clone, Copy, Debug)]
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
impl PK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PK {
            x: 0.0,
            z: 0.0,
            vx: 0.0,
            vz: 0.0,
            radius: 0.0,
            who: 0.0,
            mass: 0.0,
            quiet: 0.0,
        }
    }
}
impl Default for PK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:91 PY, src/sim/propshome.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct PY {
    pub cx: f64,
    pub cz: f64,
    pub ux: f64,
    pub uz: f64,
    pub half: f64,
}
impl PY {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PY {
            cx: 0.0,
            cz: 0.0,
            ux: 0.0,
            uz: 0.0,
            half: 0.0,
        }
    }
}
impl Default for PY { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:93 PE, src/sim/propshome.ts:93)
#[derive(Clone, Copy, Debug)]
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
impl PE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PE {
            prop: 0.0,
            sound: 0.0,
            roll: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            who: 0.0,
            level: 0.0,
            speed: 0.0,
            rolling: 0.0,
        }
    }
}
impl Default for PE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:96 PP, src/sim/propshome.ts:96)
#[derive(Clone, Copy, Debug)]
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
impl PP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PP {
            dt: 0.0,
            props: 0.0,
            contacts: 0.0,
            leaves: 0.0,
            ways: 0.0,
            flags: 0.0,
            friction: 0.0,
            cas_decel: 0.0,
            cas_visc: 0.0,
            cas_spin: 0.0,
            cas_start: 0.0,
            cas_stop: 0.0,
            events: 0.0,
            wrong: 0.0,
            swept: 0.0,
            guided: 0.0,
            pairs: 0.0,
            walls: 0.0,
            held: 0.0,
            substeps: 0.0,
        }
    }
}
impl Default for PP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:98 PF, src/sim/propshome.ts:98)
#[derive(Clone, Copy, Debug)]
pub struct PF {
    pub enabled: f64,
    pub castor: f64,
}
impl PF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PF {
            enabled: 0.0,
            castor: 0.0,
        }
    }
}
impl Default for PF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/propshome.ts:120 PROPS_STEPPED, src/sim/propshome.ts:120)
#[derive(Clone, Debug)]
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
impl SimPropshomeModPROPSSTEPPED {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SimPropshomeModPROPSSTEPPED {
            there: 0.0,
            here: 0.0,
            noises: 0.0,
            asked: 0.0,
            differ: 0.0,
            first: String::new(),
            swept: 0.0,
            guided: 0.0,
            pairs: 0.0,
            walls: 0.0,
            held: 0.0,
            substeps: 0.0,
            settled: 0.0,
            settled_here: 0.0,
            settle_walls: 0.0,
            settle_moved: 0.0,
        }
    }
}
impl Default for SimPropshomeModPROPSSTEPPED { fn default() -> Self { Self::zeroed() } }

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
