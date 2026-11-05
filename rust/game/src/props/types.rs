//! The types of `props`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: PropContact, src/game/props.ts:24)
#[derive(Clone, Copy, Debug)]
pub struct PropContact {
    pub pos: V3,
    pub vel: V3,
    pub radius: f64,
    pub who: f64,
    pub mass: f64,
    pub quiet: bool,
}
impl PropContact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropContact {
            pos: [0.0; 3],
            vel: [0.0; 3],
            radius: 0.0,
            who: 0.0,
            mass: 0.0,
            quiet: false,
        }
    }
}
impl Default for PropContact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PropEvent, src/game/props.ts:25)
#[derive(Clone, Copy, Debug)]
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
impl PropEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropEvent {
            prop: PropId(0),
            sound: crate::props::PropEventSound::Thud,
            pos: [0.0; 3],
            who: 0.0,
            level: 0.0,
            speed: 0.0,
            rolling: false,
            roll: None,
        }
    }
}
impl Default for PropEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PropEventSound {
    Thud,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: KindTune, src/game/props.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct KindTune {
    pub decel: f64,
    pub visc: f64,
    pub rest: f64,
    pub scrape_rate: f64,
    pub thud_rate: f64,
}
impl KindTune {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KindTune {
            decel: 0.0,
            visc: 0.0,
            rest: 0.0,
            scrape_rate: 0.0,
            thud_rate: 0.0,
        }
    }
}
impl Default for KindTune { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Prop, src/game/props.ts:73)
#[derive(Clone, Debug)]
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
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Prop {
            hx: 0.0,
            hz: 0.0,
            radius: 0.0,
            height: 0.0,
            mass: 0.0,
            inv_mass: 0.0,
            inv_i: 0.0,
            tune: crate::props::KindTune::zeroed(),
            home: [0.0; 3],
            boxes: Vec::new(),
            def: crate::scene_level::PropDef::zeroed(),
            id: 0.0,
        }
    }
}
impl Default for Prop { fn default() -> Self { Self::blank() } }

/// (TypeScript: PropSystem, src/game/props.ts:165)
#[derive(Clone, Debug)]
pub struct PropSystem {
    pub props: Vec<PropId>,
    pub all: Vec<PropId>,
    pub store: crate::sim_propshome::PropStore,
    pub enabled: bool,
    pub friction_scale: f64,
    pub stats: crate::props::PropSystemStats,
    pub near: Vec<i32>,
    pub by_box: JsMap<BoxId, PropId>,
    pub muted: bool,
    pub said: Vec<crate::props::PropEvent>,
    pub stepping: bool,
    pub doorways: Vec<crate::props::PropSystemDoorwaysItem>,
}
impl PropSystem {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        PropSystem {
            props: Vec::new(),
            all: Vec::new(),
            store: crate::sim_propshome::PropStore::blank(),
            enabled: true,
            friction_scale: 1.0,
            stats: crate::props::PropSystemStats { awake: 0.0, ms: 0.0 },
            near: alloc::vec![0; 256],
            by_box: JsMap::new(),
            muted: true,
            said: Vec::new(),
            stepping: false,
            doorways: Vec::new(),
        }
    }
}
impl Default for PropSystem { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/props.ts:176 PropSystem.stats, src/game/props.ts:176)
#[derive(Clone, Copy, Debug)]
pub struct PropSystemStats {
    pub awake: f64,
    pub ms: f64,
}
impl PropSystemStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropSystemStats {
            awake: 0.0,
            ms: 0.0,
        }
    }
}
impl Default for PropSystemStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/props.ts:192 PropSystem.doorways, src/game/props.ts:192)
#[derive(Clone, Copy, Debug)]
pub struct PropSystemDoorwaysItem {
    pub cx: f64,
    pub cz: f64,
    pub ux: f64,
    pub uz: f64,
    pub half: f64,
}
impl PropSystemDoorwaysItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropSystemDoorwaysItem {
            cx: 0.0,
            cz: 0.0,
            ux: 0.0,
            uz: 0.0,
            half: 0.0,
        }
    }
}
impl Default for PropSystemDoorwaysItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/props.ts:136 sat, src/game/props.ts:136)
#[derive(Clone, Copy, Debug)]
pub struct PropsModSat {
    pub nx: f64,
    pub nz: f64,
    pub depth: f64,
    pub px: f64,
    pub pz: f64,
}
impl PropsModSat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropsModSat {
            nx: 0.0,
            nz: 0.0,
            depth: 0.0,
            px: 0.0,
            pz: 0.0,
        }
    }
}
impl Default for PropsModSat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/props.ts:587 cc, src/game/props.ts:587)
#[derive(Clone, Copy, Debug)]
pub struct PropsModCc {
    pub nx: f64,
    pub nz: f64,
    pub pen: f64,
    pub px: f64,
    pub pz: f64,
}
impl PropsModCc {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropsModCc {
            nx: 0.0,
            nz: 0.0,
            pen: 0.0,
            px: 0.0,
            pz: 0.0,
        }
    }
}
impl Default for PropsModCc { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/props.ts:263 raycast ->, src/game/props.ts:263)
#[derive(Clone, Copy, Debug)]
pub struct PropSystemRaycast {
    pub t: f64,
    pub n: V3,
    pub prop: PropId,
    pub box_: BoxId,
}
impl PropSystemRaycast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropSystemRaycast {
            t: 0.0,
            n: [0.0; 3],
            prop: PropId(0),
            box_: BoxId(0),
        }
    }
}
impl Default for PropSystemRaycast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/props.ts:271 raycastHere ->, src/game/props.ts:271)
#[derive(Clone, Copy, Debug)]
pub struct PropSystemRaycastHere {
    pub t: f64,
    pub n: V3,
    pub prop: PropId,
    pub box_: BoxId,
}
impl PropSystemRaycastHere {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropSystemRaycastHere {
            t: 0.0,
            n: [0.0; 3],
            prop: PropId(0),
            box_: BoxId(0),
        }
    }
}
impl Default for PropSystemRaycastHere { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/props.ts:337 there ->, src/game/props.ts:337)
#[derive(Clone, Debug)]
pub struct PropSystemThere {
    pub leaves: Option<Vec<f64>>,
    pub n: f64,
}
impl PropSystemThere {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropSystemThere {
            leaves: None,
            n: 0.0,
        }
    }
}
impl Default for PropSystemThere { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/props.ts:359 handOver(T), src/game/props.ts:359)
#[derive(Clone, Debug)]
pub struct PropSystemHandOverT {
    pub leaves: Option<Vec<f64>>,
    pub n: f64,
}
impl PropSystemHandOverT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropSystemHandOverT {
            leaves: None,
            n: 0.0,
        }
    }
}
impl Default for PropSystemHandOverT { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct PropsMod {
    /// (src/game/props.ts:136 sat)
    pub sat: crate::props::PropsModSat,
    /// (src/game/props.ts:587 cc)
    pub cc: crate::props::PropsModCc,
}
impl PropsMod {
    pub fn blank() -> Self {
        PropsMod {
            sat: crate::props::PropsModSat { nx: 0.0, nz: 0.0, depth: 0.0, px: 0.0, pz: 0.0 },
            cc: crate::props::PropsModCc { nx: 0.0, nz: 0.0, pen: 0.0, px: 0.0, pz: 0.0 },
        }
    }
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
