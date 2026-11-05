//! The types of `spotted`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SpottedMode {
    Calm,
    Caught,
    Wary,
}
impl SpottedMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            SpottedMode::Calm => "calm",
            SpottedMode::Caught => "caught",
            SpottedMode::Wary => "wary",
        }
    }
}

/// (TypeScript: Man, src/game/spotted.ts:101)
#[derive(Clone, Debug)]
pub struct Man {
    pub mode: crate::spotted::SpottedMode,
    pub since: f64,
    pub wary: Vec<GuardId>,
    pub by: Option<GuardId>,
    pub eyes: Option<GuardId>,
    pub wary_t: f64,
    pub caught_t: f64,
    pub spot: Option<V3>,
    pub what: Option<crate::spotted::ManWhat>,
    pub ask_t: f64,
    pub exposed: bool,
    pub trail: bool,
    pub said_t: f64,
    pub real: bool,
    pub told: bool,
    pub take: Option<crate::spotted::ManTake>,
    pub take_no: Option<crate::spotted::ManTakeNo>,
    pub real_t: f64,
    pub tick_t: f64,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            mode: crate::spotted::SpottedMode::Calm,
            since: 0.0,
            wary: Vec::new(),
            by: None,
            eyes: None,
            wary_t: 0.0,
            caught_t: 0.0,
            spot: None,
            what: None,
            ask_t: 0.0,
            exposed: false,
            trail: false,
            said_t: 0.0,
            real: false,
            told: false,
            take: None,
            take_no: None,
            real_t: 0.0,
            tick_t: 0.0,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ManWhat {
    Step,
    Freeze,
    Slip,
}
impl ManWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManWhat::Step => "step",
            ManWhat::Freeze => "freeze",
            ManWhat::Slip => "slip",
        }
    }
}

/// (TypeScript: inline at src/game/spotted.ts:115 Man.take, src/game/spotted.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct ManTake {
    pub foe: GuardId,
    pub t0: f64,
    pub born: f64,
    pub spot: V3,
    pub asked: Option<bool>,
    /// INVENTED by a review: see the proposals
    pub spot_serial: u32,
}
impl ManTake {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManTake {
            foe: GuardId(0),
            t0: 0.0,
            born: 0.0,
            spot: [0.0; 3],
            asked: None,
            spot_serial: 0,
        }
    }
}
impl Default for ManTake { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:117 Man.takeNo, src/game/spotted.ts:117)
#[derive(Clone, Copy, Debug)]
pub struct ManTakeNo {
    pub foe: GuardId,
    pub until: f64,
}
impl ManTakeNo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ManTakeNo {
            foe: GuardId(0),
            until: 0.0,
        }
    }
}
impl Default for ManTakeNo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:99 SPOTTED_DEFAULTS, src/game/spotted.ts:99)
#[derive(Clone, Copy, Debug)]
pub struct SPOTTEDDEFAULTS {
    pub on: bool,
    pub hold: bool,
    pub low: bool,
    pub freeze: bool,
    pub leash_m: f64,
    pub slip: bool,
    pub slip_m: f64,
    pub lit_at: f64,
    pub slip_wait: f64,
    pub wary_m: f64,
    pub ear_m: f64,
    pub linger_secs: f64,
    pub brace: bool,
    pub brace_secs: f64,
    pub brace_m: f64,
    pub close: bool,
    pub take_m: f64,
    pub take_reach: f64,
    pub take_secs: f64,
    pub fresh_secs: f64,
    pub caught_m: f64,
    pub last_secs: f64,
    pub keen: bool,
    pub gun: bool,
    pub step: bool,
    pub step_m: f64,
    pub hold_unaware: bool,
    pub face_deg: f64,
    pub close_m: f64,
    pub cue: bool,
}
impl SPOTTEDDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SPOTTEDDEFAULTS {
            on: false,
            hold: false,
            low: false,
            freeze: false,
            leash_m: 0.0,
            slip: false,
            slip_m: 0.0,
            lit_at: 0.0,
            slip_wait: 0.0,
            wary_m: 0.0,
            ear_m: 0.0,
            linger_secs: 0.0,
            brace: false,
            brace_secs: 0.0,
            brace_m: 0.0,
            close: false,
            take_m: 0.0,
            take_reach: 0.0,
            take_secs: 0.0,
            fresh_secs: 0.0,
            caught_m: 0.0,
            last_secs: 0.0,
            keen: false,
            gun: false,
            step: false,
            step_m: 0.0,
            hold_unaware: false,
            face_deg: 0.0,
            close_m: 0.0,
            cue: false,
        }
    }
}
impl Default for SPOTTEDDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:122 SPOTTED_LOG, src/game/spotted.ts:122)
#[derive(Clone, Debug)]
pub struct SpottedModSPOTTEDLOGItem {
    pub t: f64,
    pub who: String,
    pub to: crate::spotted::SpottedMode,
    pub what: Option<String>,
    pub of: Option<String>,
}
impl SpottedModSPOTTEDLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpottedModSPOTTEDLOGItem {
            t: 0.0,
            who: String::new(),
            to: crate::spotted::SpottedMode::Calm,
            what: None,
            of: None,
        }
    }
}
impl Default for SpottedModSPOTTEDLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:312 drops, src/game/spotted.ts:312)
#[derive(Clone, Debug)]
pub struct SpottedModDropsValue {
    pub down: JsSet<GuardId>,
    pub at: Vec<crate::spotted::SpottedModDropsValueAtItem>,
    pub seen: bool,
    pub t: f64,
}
impl SpottedModDropsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpottedModDropsValue {
            down: JsSet::new(),
            at: Vec::new(),
            seen: false,
            t: 0.0,
        }
    }
}
impl Default for SpottedModDropsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:312 SpottedModDropsValue.at, src/game/spotted.ts:312)
#[derive(Clone, Copy, Debug)]
pub struct SpottedModDropsValueAtItem {
    pub pos: V3,
    pub t: f64,
}
impl SpottedModDropsValueAtItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpottedModDropsValueAtItem {
            pos: [0.0; 3],
            t: 0.0,
        }
    }
}
impl Default for SpottedModDropsValueAtItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:170 spottedFeet ->, src/game/spotted.ts:170)
#[derive(Clone, Copy, Debug)]
pub struct SpottedFeet {
    pub goal: Option<V3>,
    pub speed: Option<f64>,
    pub crouch: bool,
    pub watch: Option<V3>,
    /// INVENTED by a review: see the proposals
    pub goal_serial: Option<u32>,
}
impl SpottedFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpottedFeet {
            goal: None,
            speed: None,
            crouch: false,
            watch: None,
            goal_serial: None,
        }
    }
}
impl Default for SpottedFeet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:313 fresh ->, src/game/spotted.ts:313)
#[derive(Clone, Copy, Debug)]
pub struct FreshItem {
    pub pos: V3,
    pub t: f64,
}
impl FreshItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FreshItem {
            pos: [0.0; 3],
            t: 0.0,
        }
    }
}
impl Default for FreshItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/spotted.ts:368 spottedOf ->, src/game/spotted.ts:368)
#[derive(Clone, Debug)]
pub struct SpottedOf {
    pub mode: crate::spotted::SpottedMode,
    pub what: Option<String>,
    pub exposed: bool,
    pub spot: Option<V3>,
    pub by: Option<String>,
    pub wary: Vec<String>,
}
impl SpottedOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpottedOf {
            mode: crate::spotted::SpottedMode::Calm,
            what: None,
            exposed: false,
            spot: None,
            by: None,
            wary: Vec::new(),
        }
    }
}
impl Default for SpottedOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SpottedMod {
    /// (src/game/spotted.ts:119 men)
    pub men: JsMap<GuardId, crate::spotted::Man>,
    /// (src/game/spotted.ts:122 SPOTTED_LOG)
    pub spotted_log: Vec<crate::spotted::SpottedModSPOTTEDLOGItem>,
    /// (src/game/spotted.ts:312 drops)
    pub drops: Option<crate::spotted::SpottedModDropsValue>,
}
impl SpottedMod {
    pub fn blank() -> Self {
        SpottedMod {
            men: JsMap::new(),
            spotted_log: Vec::new(),
            drops: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SpottedStats {
    pub wary: i32,
    pub caught: i32,
    pub calm: i32,
    pub slips: i32,
    pub freezes: i32,
    pub steps: i32,
    pub held: i32,
    pub held_unaware: i32,
    pub keen: i32,
    pub cues: i32,
    pub braces: i32,
    pub takes: i32,
    pub taken: i32,
}

/// (src/game/spotted.ts:99 SPOTTED_DEFAULTS: a table nothing writes)
pub fn spotted_defaults() -> crate::spotted::SPOTTEDDEFAULTS { crate::spotted::SPOTTEDDEFAULTS { on: false, hold: true, low: true, freeze: true, leash_m: 7.0, slip: true, slip_m: 3.2, lit_at: 0.3, slip_wait: 1.5, wary_m: 14.0, ear_m: 5.0, linger_secs: 1.5, brace: false, brace_secs: 3.0, brace_m: 7.0, close: true, take_m: 2.2, take_reach: 1.05, take_secs: 0.6, fresh_secs: 2.5, caught_m: 30.0, last_secs: 6.0, keen: true, gun: true, step: true, step_m: 3.2, hold_unaware: true, face_deg: 35.0, close_m: 6.0, cue: true } }

/// (src/game/spotted.ts:126 NONE: a table nothing writes)
pub fn none() -> Vec<()> { Vec::new() }

/// (src/game/spotted.ts:373 LS)
pub const LS: &str = "twopointsix.spotted";

/// (src/game/spotted.ts:374 KEPT: a table nothing writes)
pub fn kept() -> [String; 12] { [String::from("on"), String::from("hold"), String::from("low"), String::from("freeze"), String::from("slip"), String::from("brace"), String::from("close"), String::from("keen"), String::from("gun"), String::from("step"), String::from("holdUnaware"), String::from("cue")] }
