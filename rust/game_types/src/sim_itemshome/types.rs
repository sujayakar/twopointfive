//! The types of `sim_itemshome`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct ItemStore {
    pub o: crate::sim_itemshome::ItemStoreO,
    pub cap: f64,
}
impl ItemStore {
    /// what a new one holds
    pub fn blank() -> Self {
        ItemStore {
            o: crate::sim_itemshome::ItemStoreO::default(),
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

#[derive(Clone, Copy, Debug, Default)]
pub struct ITF {
    pub settled: f64,
    pub fired: f64,
    pub carried: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimItemshomeModITEMSSTEPPED {
    pub there: f64,
    pub host: f64,
    pub walks: f64,
    pub rays: f64,
    pub thing_rays: f64,
    pub substeps: f64,
    pub rested: f64,
    pub events: f64,
    pub steps_there: f64,
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
    pub items_stepped: crate::sim_itemshome::SimItemshomeModITEMSSTEPPED,
}
impl SimItemshomeMod {
    pub fn blank() -> Self {
        SimItemshomeMod {
            items_stepped: crate::sim_itemshome::SimItemshomeModITEMSSTEPPED { there: 0.0, host: 0.0, walks: 0.0, rays: 0.0, thing_rays: 0.0, substeps: 0.0, rested: 0.0, events: 0.0, steps_there: 0.0 },
        }
    }
}

pub const ITEM_CAP: f64 = 32.0;

pub const ITEM_ROWS: f64 = 64.0;

pub fn item_f64() -> [String; 20] { [String::from("px"), String::from("py"), String::from("pz"), String::from("vx"), String::from("vy"), String::from("vz"), String::from("yaw"), String::from("spin"), String::from("tumble"), String::from("tumbleRate"), String::from("age"), String::from("life"), String::from("radius"), String::from("restitution"), String::from("friction"), String::from("heading"), String::from("onPose"), String::from("fuse"), String::from("emitT"), String::from("fill")] }

pub fn item_u32() -> [String; 6] { [String::from("flags"), String::from("kind"), String::from("bounces"), String::from("who"), String::from("on"), String::from("serial")] }

pub const ITEM_HEAD: f64 = 16.0;

pub fn itf() -> crate::sim_itemshome::ITF { crate::sim_itemshome::ITF { settled: 1.0, fired: 2.0, carried: 4.0 } }

pub fn item_kinds() -> [String; 9] { [String::from("smoke"), String::from("mag"), String::from("flash"), String::from("frag"), String::from("pistol"), String::from("torch"), String::from("rifle"), String::from("ring"), String::from("shell")] }

pub fn ic() -> JsMap<String, f64> { { let mut t = JsMap::default(); for (i, k) in crate::sim_itemshome::item_f64().into_iter().chain(crate::sim_itemshome::item_u32()).enumerate() { t.set(k, i as f64); } t } }

pub const NF: f64 = 20.0;

pub const NC: f64 = 26.0;

pub const ITEM_ROOFS: f64 = 4.0;

