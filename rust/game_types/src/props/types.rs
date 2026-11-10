//! The types of `props`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct PropContact {
    pub pos: V3,
    pub vel: V3,
    pub radius: f64,
    pub who: f64,
    pub mass: f64,
    pub quiet: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PropEvent {
    pub prop: PropId,
    pub sound: crate::props::PropEventSound,
    pub pos: V3,
    pub who: f64,
    pub level: f64,
    pub speed: f64,
    pub rolling: bool,
    pub roll: Option<crate::props::PropEventRoll>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PropEventSound {
    #[default] Thud,
    Scrape,
    Roll,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum PropEventRoll {
    Start,
    Stop,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KindTune {
    pub decel: f64,
    pub visc: f64,
    pub rest: f64,
    pub scrape_rate: f64,
    pub thud_rate: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Prop {
    pub hx: f64,
    pub hz: f64,
    pub radius: f64,
    pub height: f64,
    pub mass: f64,
    pub inv_mass: f64,
    pub inv_i: f64,
    pub tune: crate::props::KindTune,
    pub home: [f64; 3],
    pub boxes: Vec<BoxId>,
    pub def: crate::scene_level::PropDef,
    pub id: f64,
    // where it stands and how it turns; the cos and sin of `yaw` are written where the yaw is written
    pub x: f64,
    pub z: f64,
    pub yaw: f64,
    pub cs: f64,
    pub sn: f64,
    pub vx: f64,
    pub vz: f64,
    pub w: f64,
    /// how long it has been still
    pub idle: f64,
    /// last time a man or a leaf was pushing it
    pub push_t: f64,
    /// last noise it made, the noise's rate limit
    pub noise_t: f64,
    /// who moved it last, blamed for its noise
    pub who: f64,
    pub asleep: bool,
    /// moved since its boxes were placed
    pub dirty: bool,
    /// coasting on its castors
    pub rolling: bool,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct PropSystem {
    pub props: Vec<PropId>,
    pub all: Vec<PropId>,
    /// the system's clock
    pub time: f64,
    #[is(true)] pub enabled: bool,
    #[is(1.0)] pub friction_scale: f64,
    #[is(true)] pub muted: bool,
    pub doorways: Vec<crate::props::PropSystemDoorwaysItem>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PropSystemDoorwaysItem {
    pub cx: f64,
    pub cz: f64,
    pub ux: f64,
    pub uz: f64,
    pub half: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PropSystemRaycast {
    pub t: f64,
    pub n: V3,
    pub prop: PropId,
    pub box_: BoxId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PropSystemRaycastHere {
    pub t: f64,
    pub n: V3,
    pub prop: PropId,
    pub box_: BoxId,
}

pub fn kind() -> JsMap<String, crate::props::KindTune> { { let mut t = JsMap::new(); t.set(String::from("chair"), crate::props::KindTune { decel: 1.6, visc: 0.5, rest: 0.25, scrape_rate: 1.35, thud_rate: 1.15 }); t.set(String::from("cardboard"), crate::props::KindTune { decel: 3.2, visc: 0.8, rest: 0.12, scrape_rate: 1.0, thud_rate: 0.8 }); t.set(String::from("bin"), crate::props::KindTune { decel: 3.0, visc: 0.8, rest: 0.3, scrape_rate: 1.2, thud_rate: 1.5 }); t.set(String::from("plant"), crate::props::KindTune { decel: 4.5, visc: 1.0, rest: 0.1, scrape_rate: 0.8, thud_rate: 0.95 }); t } }

pub const MAX_V: f64 = 5.0;

pub const MAX_W: f64 = 9.0;

/// things a level can have: the box world's copy lists no more (sim::world::MAX_THINGS)
pub const PROP_CAP: f64 = 128.0;

/// boxes of all the furniture of a level, likewise (sim::world::MAX_LEAVES)
pub const PROP_PARTS: f64 = 512.0;
