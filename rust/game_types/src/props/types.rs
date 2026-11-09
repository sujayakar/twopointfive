//! The types of `props`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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
impl PropEventSound {
    pub const fn as_str(self) -> &'static str {
        match self {
            PropEventSound::Thud => "thud",
            PropEventSound::Scrape => "scrape",
            PropEventSound::Roll => "roll",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum PropEventRoll {
    Start,
    Stop,
}
impl PropEventRoll {
    pub const fn as_str(self) -> &'static str {
        match self {
            PropEventRoll::Start => "start",
            PropEventRoll::Stop => "stop",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KindTune {
    pub decel: f64,
    pub visc: f64,
    pub rest: f64,
    pub scrape_rate: f64,
    pub thud_rate: f64,
}

#[derive(Clone, Debug, ToBytes)]
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
}
impl Prop {
    /// what a new one holds
    pub fn blank() -> Self {
        Prop {
            hx: 0.0,
            hz: 0.0,
            radius: 0.0,
            height: 0.0,
            mass: 0.0,
            inv_mass: 0.0,
            inv_i: 0.0,
            tune: crate::props::KindTune::default(),
            home: [0.0; 3],
            boxes: Vec::new(),
            def: crate::scene_level::PropDef::default(),
            id: 0.0,
        }
    }
}
impl Default for Prop { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes)]
pub struct PropSystem {
    pub props: Vec<PropId>,
    pub all: Vec<PropId>,
    pub store: crate::sim_propshome::PropStore,
    pub enabled: bool,
    pub friction_scale: f64,
    pub stats: crate::props::PropSystemStats,
    pub by_box: JsMap<BoxId, PropId>,
    pub muted: bool,
    pub said: Vec<crate::props::PropEvent>,
    pub stepping: bool,
    pub doorways: Vec<crate::props::PropSystemDoorwaysItem>,
}
impl PropSystem {
    /// what a new one holds
    pub fn blank() -> Self {
        PropSystem {
            props: Vec::new(),
            all: Vec::new(),
            store: crate::sim_propshome::PropStore::blank(),
            enabled: true,
            friction_scale: 1.0,
            stats: crate::props::PropSystemStats { awake: 0.0, ms: 0.0 },
            by_box: JsMap::new(),
            muted: true,
            said: Vec::new(),
            stepping: false,
            doorways: Vec::new(),
        }
    }
}
impl Default for PropSystem { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PropSystemStats {
    pub awake: f64,
    /// (scratch, read by nobody who did not just write it: in no bytes)
    #[bytes(skip)]
    pub ms: f64,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PropSystemNoiseSound {
    Thud,
    Scrape,
}
impl PropSystemNoiseSound {
    pub const fn as_str(self) -> &'static str {
        match self {
            PropSystemNoiseSound::Thud => "thud",
            PropSystemNoiseSound::Scrape => "scrape",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PropSystemThere {
    pub leaves: Option<Vec<f64>>,
    pub n: f64,
}

#[derive(Clone, Debug, Default)]
pub struct PropSystemHandOverT {
    pub leaves: Option<Vec<f64>>,
    pub n: f64,
}

/// (src/game/props.ts:41 KIND: a table nothing writes)
pub fn kind() -> JsMap<String, crate::props::KindTune> { { let mut t = JsMap::new(); t.set(String::from("chair"), crate::props::KindTune { decel: 1.6, visc: 0.5, rest: 0.25, scrape_rate: 1.35, thud_rate: 1.15 }); t.set(String::from("cardboard"), crate::props::KindTune { decel: 3.2, visc: 0.8, rest: 0.12, scrape_rate: 1.0, thud_rate: 0.8 }); t.set(String::from("bin"), crate::props::KindTune { decel: 3.0, visc: 0.8, rest: 0.3, scrape_rate: 1.2, thud_rate: 1.5 }); t.set(String::from("plant"), crate::props::KindTune { decel: 4.5, visc: 1.0, rest: 0.1, scrape_rate: 0.8, thud_rate: 0.95 }); t } }

/// (src/game/props.ts:47 MAX_V)
pub const MAX_V: f64 = 5.0;

/// (src/game/props.ts:47 MAX_W)
pub const MAX_W: f64 = 9.0;

/// (src/game/props.ts:48 SLEEP_V)
pub const SLEEP_V: f64 = 0.04;

/// (src/game/props.ts:48 SLEEP_W)
pub const SLEEP_W: f64 = 0.06;

/// (src/game/props.ts:48 SLEEP_T)
pub const SLEEP_T: f64 = 0.3;

/// (src/game/props.ts:49 DOOR_GUIDE)
pub const DOOR_GUIDE: f64 = 1.2;

/// (src/game/props.ts:50 SETTLE_PASSES)
pub const SETTLE_PASSES: f64 = 12.0;

/// (src/game/props.ts:51 Y_MIN)
pub const Y_MIN: f64 = 0.04;

/// (src/game/props.ts:53 NEAR)
pub const NEAR: f64 = 256.0;
