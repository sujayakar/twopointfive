//! The types of `quiet`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: QuietDeal, src/game/quiet.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct QuietDeal {
    pub start: f64,
    pub spot: f64,
    pub patrol: f64,
}
impl QuietDeal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietDeal {
            start: 0.0,
            spot: 0.0,
            patrol: 0.0,
        }
    }
}
impl Default for QuietDeal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: QuietSnap, src/game/quiet.ts:49)
#[derive(Clone, Copy, Debug)]
pub struct QuietSnap {
    pub deal: crate::quiet::QuietDeal,
    pub pin: Option<f64>,
    pub squad: f64,
}
impl QuietSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietSnap {
            deal: crate::quiet::QuietDeal::zeroed(),
            pin: None,
            squad: 0.0,
        }
    }
}
impl Default for QuietSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Held, src/game/quiet.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct Held {
    pub pairup: bool,
    pub pair: bool,
    pub sneak: bool,
    pub spotted: bool,
    pub silent: bool,
    pub silent_start: bool,
}
impl Held {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Held {
            pairup: false,
            pair: false,
            sneak: false,
            spotted: false,
            silent: false,
            silent_start: false,
        }
    }
}
impl Default for Held { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: QuietMission, src/game/quiet.ts:73)
#[derive(Clone, Debug)]
pub struct QuietMission {
    pub active: bool,
    pub ops: Vec<GuardId>,
    pub squad: f64,
    pub pin: Option<f64>,
    pub deal: crate::quiet::QuietDeal,
    pub staged: f64,
    pub log: Vec<String>,
    pub data: Option<crate::scene_scenarios::QuietData>,
    pub authored: Option<crate::quiet::QuietMissionAuthored>,
    pub benched: Vec<GuardId>,
    pub was: Option<crate::quiet::Held>,
    pub mine: Option<crate::quiet::Held>,
    pub pending: Option<crate::quiet::QuietDeal>,
    pub fall_in: bool,
    pub cut: bool,
    pub failure: Option<String>,
}
impl QuietMission {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        QuietMission {
            active: false,
            ops: Vec::new(),
            squad: 0.0,
            pin: None,
            deal: crate::quiet::QuietDeal { start: 0.0, spot: 0.0, patrol: -1.0 },
            staged: 0.0,
            log: Vec::new(),
            data: None,
            authored: None,
            benched: Vec::new(),
            was: None,
            mine: None,
            pending: None,
            fall_in: false,
            cut: false,
            failure: None,
        }
    }
}
impl Default for QuietMission { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/quiet.ts:85 QuietMission.authored, src/game/quiet.ts:85)
#[derive(Clone, Debug)]
pub struct QuietMissionAuthored {
    pub spawn: V3,
    pub mission: crate::quiet::QuietMissionAuthoredMission,
}
impl QuietMissionAuthored {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietMissionAuthored {
            spawn: [0.0; 3],
            mission: crate::quiet::QuietMissionAuthoredMission::zeroed(),
        }
    }
}
impl Default for QuietMissionAuthored { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quiet.ts:85 QuietMissionAuthored.mission, src/scene/level.ts:197)
#[derive(Clone, Debug)]
pub struct QuietMissionAuthoredMission {
    pub server_room: crate::quiet::QuietMissionAuthoredMissionServerRoom,
    pub entry_door: String,
    pub rack: crate::quiet::QuietMissionAuthoredMissionRack,
    pub exfil_door: String,
    pub exfil_x: f64,
    pub exfil_z: f64,
    pub exfil_out: Option<[f64; 2]>,
}
impl QuietMissionAuthoredMission {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietMissionAuthoredMission {
            server_room: crate::quiet::QuietMissionAuthoredMissionServerRoom::zeroed(),
            entry_door: String::new(),
            rack: crate::quiet::QuietMissionAuthoredMissionRack::zeroed(),
            exfil_door: String::new(),
            exfil_x: 0.0,
            exfil_z: 0.0,
            exfil_out: None,
        }
    }
}
impl Default for QuietMissionAuthoredMission { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:197 QuietMissionAuthoredMission.serverRoom, src/scene/level.ts:197)
#[derive(Clone, Copy, Debug)]
pub struct QuietMissionAuthoredMissionServerRoom {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl QuietMissionAuthoredMissionServerRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietMissionAuthoredMissionServerRoom {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for QuietMissionAuthoredMissionServerRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/level.ts:197 QuietMissionAuthoredMission.rack, src/scene/level.ts:197)
#[derive(Clone, Copy, Debug)]
pub struct QuietMissionAuthoredMissionRack {
    pub index: f64,
    pub front: V3,
    pub half_w: f64,
}
impl QuietMissionAuthoredMissionRack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietMissionAuthoredMissionRack {
            index: 0.0,
            front: [0.0; 3],
            half_w: 0.0,
        }
    }
}
impl Default for QuietMissionAuthoredMissionRack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quiet.ts:287 holderStand(area), src/game/quiet.ts:287)
#[derive(Clone, Copy, Debug)]
pub struct HolderStandArea {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl HolderStandArea {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HolderStandArea {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for HolderStandArea { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quiet.ts:287 holderStand ->, src/game/quiet.ts:287)
#[derive(Clone, Copy, Debug)]
pub struct HolderStand {
    pub pos: Option<V3>,
    pub ok: bool,
    pub clear: bool,
    pub inside: bool,
}
impl HolderStand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HolderStand {
            pos: None,
            ok: false,
            clear: false,
            inside: false,
        }
    }
}
impl Default for HolderStand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quiet.ts:302 quietWords ->, src/game/quiet.ts:306)
#[derive(Clone, Debug)]
pub struct QuietWords {
    pub place: String,
    pub holder: String,
    pub thing: String,
    pub outs: String,
    pub bare: String,
}
impl QuietWords {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietWords {
            place: String::new(),
            holder: String::new(),
            thing: String::new(),
            outs: String::new(),
            bare: String::new(),
        }
    }
}
impl Default for QuietWords { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct QuietMod {
    /// (src/game/quiet.ts:60 holder)
    pub holder: Option<crate::quiet::QuietMission>,
}
impl QuietMod {
    pub fn blank() -> Self {
        QuietMod {
            holder: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct QuietStats {
    pub starts: i32,
    pub deals: i32,
    pub mates: i32,
}

/// (src/game/quiet.ts:53 EXIT_HALF_W)
pub const EXIT_HALF_W: f64 = 1.8;

/// (src/game/quiet.ts:55 EXIT_DEPTH)
pub const EXIT_DEPTH: f64 = 12.0;

/// (src/game/quiet.ts:362 FIRST_SIGHT_M)
pub const FIRST_SIGHT_M: f64 = 16.0;
