//! The types of `doorforce`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Belief, src/game/doorforce.ts:95)
#[derive(Clone, Copy, Debug)]
pub struct Belief {
    pub door: DoorId,
    pub sure: f64,
    pub at: V3,
    pub t: f64,
    pub seen: bool,
}
impl Belief {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Belief {
            door: DoorId(0),
            sure: 0.0,
            at: [0.0; 3],
            t: 0.0,
            seen: false,
        }
    }
}
impl Default for Belief { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Stand, src/game/doorforce.ts:97)
#[derive(Clone, Copy, Debug)]
pub struct Stand {
    pub door: DoorId,
    pub t0: f64,
    pub n: f64,
    pub fired_t: f64,
    pub next: f64,
    pub lay: f64,
    pub fired: f64,
    pub no_shot_t: f64,
    pub to: Option<V3>,
    pub to_key: [f64; 3],
}
impl Stand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stand {
            door: DoorId(0),
            t0: 0.0,
            n: 0.0,
            fired_t: 0.0,
            next: 0.0,
            lay: 0.0,
            fired: 0.0,
            no_shot_t: 0.0,
            to: None,
            to_key: [0.0; 3],
        }
    }
}
impl Default for Stand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Kick, src/game/doorforce.ts:98)
#[derive(Clone, Copy, Debug)]
pub struct Kick {
    pub door: DoorId,
    pub spot: V3,
    pub yaw: f64,
    pub phase: crate::doorforce::KickPhase,
    pub t0: f64,
    pub set_t: f64,
}
impl Kick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Kick {
            door: DoorId(0),
            spot: [0.0; 3],
            yaw: 0.0,
            phase: crate::doorforce::KickPhase::Kick,
            t0: 0.0,
            set_t: 0.0,
        }
    }
}
impl Default for Kick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KickPhase {
    Kick,
    Set,
    Walk,
}
impl KickPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            KickPhase::Kick => "kick",
            KickPhase::Set => "set",
            KickPhase::Walk => "walk",
        }
    }
}

/// (TypeScript: Man, src/game/doorforce.ts:100)
#[derive(Clone, Debug)]
pub struct Man {
    pub beliefs: JsMap<DoorId, crate::doorforce::Belief>,
    pub saw: Option<crate::doorforce::ManSaw>,
    pub on: Option<crate::doorforce::Stand>,
    pub kick: Option<crate::doorforce::Kick>,
    pub rest: JsMap<DoorId, f64>,
    pub served: f64,
    pub cf_t: f64,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            beliefs: JsMap::new(),
            saw: None,
            on: None,
            kick: None,
            rest: JsMap::new(),
            served: 0.0,
            cf_t: 0.0,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorforce.ts:100 Man.saw, src/game/doorforce.ts:100)
#[derive(Clone, Copy, Debug)]
pub struct ManSaw {
    pub at: V3,
    pub t: f64,
    pub used: bool,
}
impl ManSaw {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManSaw {
            at: [0.0; 3],
            t: 0.0,
            used: false,
        }
    }
}
impl Default for ManSaw { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/doorforce.ts:102)
#[derive(Clone, Debug)]
pub struct State {
    pub last_t: f64,
    pub men: JsMap<GuardId, crate::doorforce::Man>,
    pub kickers: JsMap<DoorId, GuardId>,
    pub boots: JsMap<crate::Num, f64>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            last_t: 0.0,
            men: JsMap::new(),
            kickers: JsMap::new(),
            boots: JsMap::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorforce.ts:152 doorForceHear(prop), src/game/doorforce.ts:152)
#[derive(Clone, Copy, Debug)]
pub struct DoorForceHearProp {
    pub direct: bool,
    pub routed: bool,
    pub path_len: f64,
    pub carry: f64,
}
impl DoorForceHearProp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorForceHearProp {
            direct: false,
            routed: false,
            path_len: 0.0,
            carry: 0.0,
        }
    }
}
impl Default for DoorForceHearProp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorforce.ts:246 doorForce ->, src/game/doorforce.ts:246)
#[derive(Clone, Copy, Debug)]
pub struct DoorForce {
    pub aim: V3,
    pub move_: f64,
}
impl DoorForce {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorForce {
            aim: [0.0; 3],
            move_: 0.0,
        }
    }
}
impl Default for DoorForce { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorforce.ts:310 kickStep ->, src/game/doorforce.ts:310)
#[derive(Clone, Copy, Debug)]
pub struct KickStep {
    pub aim: V3,
    pub move_: f64,
}
impl KickStep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KickStep {
            aim: [0.0; 3],
            move_: 0.0,
        }
    }
}
impl Default for KickStep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorforce.ts:351 doorForceBelief ->, src/game/doorforce.ts:351)
#[derive(Clone, Copy, Debug)]
pub struct DoorForceBelief {
    pub sure: f64,
    pub at: V3,
    pub seen: bool,
    pub age: f64,
}
impl DoorForceBelief {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorForceBelief {
            sure: 0.0,
            at: [0.0; 3],
            seen: false,
            age: 0.0,
        }
    }
}
impl Default for DoorForceBelief { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorforce.ts:355 doorForceOn ->, src/game/doorforce.ts:355)
#[derive(Clone, Copy, Debug)]
pub struct DoorForceOn {
    pub door: DoorId,
    pub fired: f64,
    pub kick: Option<crate::doorforce::DoorForceOnKick>,
}
impl DoorForceOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorForceOn {
            door: DoorId(0),
            fired: 0.0,
            kick: None,
        }
    }
}
impl Default for DoorForceOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorForceOnKick {
    Kick,
    Set,
    Walk,
}
impl DoorForceOnKick {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorForceOnKick::Kick => "kick",
            DoorForceOnKick::Set => "set",
            DoorForceOnKick::Walk => "walk",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DoorforceMod {
    /// (src/game/doorforce.ts:90 DOORFORCE_LOG)
    pub doorforce_log: Vec<String>,
    /// (src/game/doorforce.ts:103 states)
    pub states: Option<crate::doorforce::State>,
}
impl DoorforceMod {
    pub fn blank() -> Self {
        DoorforceMod {
            doorforce_log: Vec::new(),
            states: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DoorforceStats {
    pub heard: i32,
    pub seen: i32,
    pub stands: i32,
    pub shots: i32,
    pub held_friend: i32,
    pub kicks: i32,
    pub gave: i32,
    pub no_kick: i32,
}

/// (src/game/doorforce.ts:92 BREAK)
pub const BREAK: bool = false;
