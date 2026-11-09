//! The types of `sim_itemshome`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct ItemStore {
    pub at: Vec<f64>,
    pub o: crate::sim_itemshome::ItemStoreO,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub home: crate::Opaque,
    pub b8: f64,
    pub b4: f64,
    pub cap: f64,
}
impl ItemStore {
    /// what a new one holds
    pub fn blank() -> Self {
        ItemStore {
            at: Vec::new(),
            o: crate::sim_itemshome::ItemStoreO::default(),
            home: crate::Opaque,
            b8: 0.0,
            b4: 0.0,
            cap: 0.0,
        }
    }
}
impl Default for ItemStore { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ItemStoreO {
    pub kind: f64,
    pub fill: f64,
    pub radius: f64,
    pub yaw: f64,
    pub flags: f64,
    pub bytes: f64,
    pub who: f64,
    pub vx: f64,
    pub vz: f64,
    pub serial: f64,
    pub on: f64,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
    pub vy: f64,
    pub spin: f64,
    pub tumble: f64,
    pub tumble_rate: f64,
    pub age: f64,
    pub life: f64,
    pub restitution: f64,
    pub friction: f64,
    pub heading: f64,
    pub on_pose: f64,
    pub fuse: f64,
    pub emit_t: f64,
    pub bounces: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum F {
    Fill,
    Radius,
    Yaw,
    Vx,
    Vz,
    Px,
    Py,
    Pz,
    Vy,
    Spin,
    Tumble,
    TumbleRate,
    Age,
    Life,
    Restitution,
    Friction,
    Heading,
    OnPose,
    Fuse,
    EmitT,
}
impl F {
    pub const fn as_str(self) -> &'static str {
        match self {
            F::Fill => "fill",
            F::Radius => "radius",
            F::Yaw => "yaw",
            F::Vx => "vx",
            F::Vz => "vz",
            F::Px => "px",
            F::Py => "py",
            F::Pz => "pz",
            F::Vy => "vy",
            F::Spin => "spin",
            F::Tumble => "tumble",
            F::TumbleRate => "tumbleRate",
            F::Age => "age",
            F::Life => "life",
            F::Restitution => "restitution",
            F::Friction => "friction",
            F::Heading => "heading",
            F::OnPose => "onPose",
            F::Fuse => "fuse",
            F::EmitT => "emitT",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum U {
    Kind,
    Flags,
    Who,
    Serial,
    On,
    Bounces,
}
impl U {
    pub const fn as_str(self) -> &'static str {
        match self {
            U::Kind => "kind",
            U::Flags => "flags",
            U::Who => "who",
            U::Serial => "serial",
            U::On => "on",
            U::Bounces => "bounces",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ItemIo {
    pub x: crate::sim_sim::SimExports,
    pub par: f64,
    pub ev: f64,
    pub max_events: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ITF {
    pub settled: f64,
    pub fired: f64,
    pub carried: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IP {
    pub dt: f64,
    pub from: f64,
    pub ask: f64,
    pub bog: f64,
    pub rifle_share: f64,
    pub row: f64,
    pub events: f64,
    pub rays: f64,
    pub thing_rays: f64,
    pub substeps: f64,
    pub walked: f64,
    pub rested: f64,
    pub snow_top: f64,
    pub snow_bank: f64,
    pub snow_packed: f64,
    pub snow_old: f64,
    pub snow_dead: f64,
    pub snow_grip: f64,
    pub snow_stick: f64,
    pub roofs: f64,
    pub roof: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IE {
    pub kind: f64,
    pub row: f64,
    pub a: f64,
    pub p: f64,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IEK {
    pub bounce: f64,
    pub impulse: f64,
    pub gone: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimItemshomeModITEMSSTEPPED {
    pub there: f64,
    pub here: f64,
    pub host: f64,
    pub walks: f64,
    pub asked: f64,
    pub differ: f64,
    pub first: String,
    pub rays: f64,
    pub thing_rays: f64,
    pub substeps: f64,
    pub rested: f64,
    pub events: f64,
    pub steps_there: f64,
    pub steps_here: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ItemCols {
    pub kind: f64,
    pub fill: f64,
    pub radius: f64,
    pub yaw: f64,
    pub flags: f64,
    pub bytes: f64,
    pub who: f64,
    pub vx: f64,
    pub vz: f64,
    pub serial: f64,
    pub on: f64,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
    pub vy: f64,
    pub spin: f64,
    pub tumble: f64,
    pub tumble_rate: f64,
    pub age: f64,
    pub life: f64,
    pub restitution: f64,
    pub friction: f64,
    pub heading: f64,
    pub on_pose: f64,
    pub fuse: f64,
    pub emit_t: f64,
    pub bounces: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimItemshomeMod {
    /// (src/sim/itemshome.ts:121 IO)
    pub io: Option<crate::sim_itemshome::ItemIo>,
    /// (src/sim/itemshome.ts:137 ITEMS_STEPPED)
    pub items_stepped: crate::sim_itemshome::SimItemshomeModITEMSSTEPPED,
}
impl SimItemshomeMod {
    pub fn blank() -> Self {
        SimItemshomeMod {
            io: None,
            items_stepped: crate::sim_itemshome::SimItemshomeModITEMSSTEPPED { there: 0.0, here: 0.0, host: 0.0, walks: 0.0, asked: 0.0, differ: 0.0, first: String::from(""), rays: 0.0, thing_rays: 0.0, substeps: 0.0, rested: 0.0, events: 0.0, steps_there: 0.0, steps_here: 0.0 },
        }
    }
}

/// (src/sim/itemshome.ts:31 ITEM_CAP)
pub const ITEM_CAP: f64 = 32.0;

/// (src/sim/itemshome.ts:33 ITEM_ROWS)
pub const ITEM_ROWS: f64 = 64.0;

/// (src/sim/itemshome.ts:36 ITEM_F64: a table nothing writes)
pub fn item_f64() -> [String; 20] { [String::from("px"), String::from("py"), String::from("pz"), String::from("vx"), String::from("vy"), String::from("vz"), String::from("yaw"), String::from("spin"), String::from("tumble"), String::from("tumbleRate"), String::from("age"), String::from("life"), String::from("radius"), String::from("restitution"), String::from("friction"), String::from("heading"), String::from("onPose"), String::from("fuse"), String::from("emitT"), String::from("fill")] }

/// (src/sim/itemshome.ts:37 ITEM_U32: a table nothing writes)
pub fn item_u32() -> [String; 6] { [String::from("flags"), String::from("kind"), String::from("bounces"), String::from("who"), String::from("on"), String::from("serial")] }

/// (src/sim/itemshome.ts:39 ITEM_HEAD)
pub const ITEM_HEAD: f64 = 16.0;

/// (src/sim/itemshome.ts:41 ITF: a table nothing writes)
pub fn itf() -> crate::sim_itemshome::ITF { crate::sim_itemshome::ITF { settled: 1.0, fired: 2.0, carried: 4.0 } }

/// (src/sim/itemshome.ts:43 ITEM_KINDS: a table nothing writes)
pub fn item_kinds() -> [String; 9] { [String::from("smoke"), String::from("mag"), String::from("flash"), String::from("frag"), String::from("pistol"), String::from("torch"), String::from("rifle"), String::from("ring"), String::from("shell")] }

/// (src/sim/itemshome.ts:49 IC: a table nothing writes)
pub fn ic() -> JsMap<String, f64> { { let mut t = JsMap::default(); for (i, k) in crate::sim_itemshome::item_f64().into_iter().chain(crate::sim_itemshome::item_u32()).enumerate() { t.set(k, i as f64); } t } }

/// (src/sim/itemshome.ts:50 NF)
pub const NF: f64 = 20.0;

/// (src/sim/itemshome.ts:50 NC)
pub const NC: f64 = 26.0;

/// (src/sim/itemshome.ts:54 ITEM_BIT)
pub const ITEM_BIT: f64 = 128.0;

/// (src/sim/itemshome.ts:104 IP: a table nothing writes)
pub fn ip() -> crate::sim_itemshome::IP { crate::sim_itemshome::IP { dt: 0.0, from: 1.0, ask: 2.0, bog: 3.0, rifle_share: 4.0, row: 8.0, events: 9.0, rays: 10.0, thing_rays: 11.0, substeps: 12.0, walked: 13.0, rested: 14.0, snow_top: 16.0, snow_bank: 17.0, snow_packed: 18.0, snow_old: 19.0, snow_dead: 20.0, snow_grip: 21.0, snow_stick: 22.0, roofs: 23.0, roof: 24.0 } }

/// (src/sim/itemshome.ts:106 ITEM_PARS)
pub const ITEM_PARS: f64 = 40.0;

/// (src/sim/itemshome.ts:108 ITEM_ROOFS)
pub const ITEM_ROOFS: f64 = 4.0;

/// (src/sim/itemshome.ts:111 IE: a table nothing writes)
pub fn ie() -> crate::sim_itemshome::IE { crate::sim_itemshome::IE { kind: 0.0, row: 1.0, a: 2.0, p: 3.0, v: 6.0 } }

/// (src/sim/itemshome.ts:112 ITEM_EV)
pub const ITEM_EV: f64 = 9.0;

/// (src/sim/itemshome.ts:113 IEK: a table nothing writes)
pub fn iek() -> crate::sim_itemshome::IEK { crate::sim_itemshome::IEK { bounce: 0.0, impulse: 1.0, gone: 2.0 } }

/// (src/sim/itemshome.ts:115 ITEM_ROW_EVENTS)
pub const ITEM_ROW_EVENTS: f64 = 6.0;

/// (src/sim/itemshome.ts:117 ITEMS_HOST)
pub const ITEMS_HOST: f64 = 1.0;
