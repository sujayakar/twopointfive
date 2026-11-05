//! The types of `snow_breath`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: BreathShown, src/game/snowBreath.ts:103)
#[derive(Clone, Debug)]
pub struct BreathShown {
    pub tick: f64,
    pub how: String,
    pub steam: f64,
    pub steaming: f64,
    pub records: f64,
    pub births: f64,
    pub by: Vec<f64>,
    pub men: JsMap<crate::Num, crate::snow_breath::BreathShownMenValue>,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}
impl BreathShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreathShown {
            tick: 0.0,
            how: String::new(),
            steam: 0.0,
            steaming: 0.0,
            records: 0.0,
            births: 0.0,
            by: Vec::new(),
            men: JsMap::new(),
            serial: 0,
        }
    }
}
impl Default for BreathShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowBreath.ts:103 BreathShown.men, src/game/snowBreath.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct BreathShownMenValue {
    pub who: CharId,
    pub at: f64,
    pub n: f64,
    pub steam: bool,
    pub src: f64,
}
impl BreathShownMenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreathShownMenValue {
            who: CharId(0),
            at: 0.0,
            n: 0.0,
            steam: false,
            src: 0.0,
        }
    }
}
impl Default for BreathShownMenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreathCount, src/game/snowBreath.ts:92)
#[derive(Clone, Debug)]
pub struct BreathCount {
    pub active: bool,
    pub why: String,
    pub men: f64,
    pub shown: f64,
    pub breaths: f64,
    pub frames: f64,
}
impl BreathCount {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreathCount {
            active: false,
            why: String::new(),
            men: 0.0,
            shown: 0.0,
            breaths: 0.0,
            frames: 0.0,
        }
    }
}
impl Default for BreathCount { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Man, src/game/snowBreath.ts:104)
#[derive(Clone, Copy, Debug)]
pub struct Man {
    pub ph: f64,
    pub pace: f64,
    pub effort: f64,
    pub n: f64,
    pub left: f64,
    pub of: f64,
    pub gap: f64,
    pub step: f64,
    pub shows: bool,
    pub at: f64,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            ph: 0.0,
            pace: 0.0,
            effort: 0.0,
            n: 0.0,
            left: 0.0,
            of: 0.0,
            gap: 0.0,
            step: 0.0,
            shows: false,
            at: 0.0,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreathState, src/game/snowBreath.ts:105)
#[derive(Clone, Debug)]
pub struct BreathState {
    pub seed: f64,
    pub men: JsMap<CharId, crate::snow_breath::Man>,
    pub stats: crate::snow_breath::BreathCount,
    pub tick: f64,
}
impl BreathState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreathState {
            seed: 0.0,
            men: JsMap::new(),
            stats: crate::snow_breath::BreathCount::zeroed(),
            tick: 0.0,
        }
    }
}
impl Default for BreathState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowBreath.ts:38 SNOWBREATH, src/game/snowBreath.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct SNOWBREATH {
    pub every: f64,
    pub every_jit: f64,
    pub every_hard: f64,
    pub out: f64,
    pub puffs: f64,
    pub count: f64,
    pub speed: f64,
    pub spread: f64,
    pub down: f64,
    pub pitch: [f64; 2],
    pub carry: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
    pub size_jit: f64,
    pub opacity: f64,
    pub hard_k: f64,
    pub color: V3,
    pub mouth_ahead: f64,
    pub mouth_down: f64,
    pub min_head: f64,
    pub run_lo: f64,
    pub run_hi: f64,
    pub effort_up: f64,
    pub effort_down: f64,
}
impl SNOWBREATH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWBREATH {
            every: 0.0,
            every_jit: 0.0,
            every_hard: 0.0,
            out: 0.0,
            puffs: 0.0,
            count: 0.0,
            speed: 0.0,
            spread: 0.0,
            down: 0.0,
            pitch: [0.0; 2],
            carry: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
            size_jit: 0.0,
            opacity: 0.0,
            hard_k: 0.0,
            color: [0.0; 3],
            mouth_ahead: 0.0,
            mouth_down: 0.0,
            min_head: 0.0,
            run_lo: 0.0,
            run_hi: 0.0,
            effort_up: 0.0,
            effort_down: 0.0,
        }
    }
}
impl Default for SNOWBREATH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowBreath.ts:71 BREATHSTEAM, src/game/snowBreath.ts:71)
#[derive(Clone, Copy, Debug)]
pub struct BREATHSTEAM {
    pub radius: f64,
    pub cells: f64,
    pub density: f64,
    pub temp: f64,
    pub speed: f64,
    pub push: f64,
    pub expand: f64,
    pub attack: f64,
    pub jitter: f64,
    pub carry_k: f64,
    pub wake: f64,
    pub reach: f64,
}
impl BREATHSTEAM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BREATHSTEAM {
            radius: 0.0,
            cells: 0.0,
            density: 0.0,
            temp: 0.0,
            speed: 0.0,
            push: 0.0,
            expand: 0.0,
            attack: 0.0,
            jitter: 0.0,
            carry_k: 0.0,
            wake: 0.0,
            reach: 0.0,
        }
    }
}
impl Default for BREATHSTEAM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowBreath.ts:147 mouthOf ->, src/game/snowBreath.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct MouthOf {
    pub pos: V3,
    pub dir: V3,
}
impl MouthOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MouthOf {
            pos: [0.0; 3],
            dir: [0.0; 3],
        }
    }
}
impl Default for MouthOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SnowBreathMod {
    /// (src/game/snowBreath.ts:106 states)
    pub states: Option<crate::snow_breath::BreathState>,
}
impl SnowBreathMod {
    pub fn blank() -> Self {
        SnowBreathMod {
            states: None,
        }
    }
}

/// (src/game/snowBreath.ts:38 SNOWBREATH: a table nothing writes)
pub fn snowbreath() -> crate::snow_breath::SNOWBREATH { crate::snow_breath::SNOWBREATH { every: 3.6, every_jit: 0.18, every_hard: 1.3, out: 0.5, puffs: 3.0, count: 5.0, speed: 0.7, spread: 0.24, down: 0.12, pitch: [-0.45, 0.3], carry: 0.6, life: 1.6, life_jit: 0.25, size: 0.085, size_jit: 0.3, opacity: 0.66, hard_k: 1.5, color: [0.9, 0.93, 1.0], mouth_ahead: 0.1, mouth_down: 0.075, min_head: 0.75, run_lo: 1.9, run_hi: 4.0, effort_up: 4.0, effort_down: 12.0 } }

/// (src/game/snowBreath.ts:71 BREATHSTEAM: a table nothing writes)
pub fn breathsteam() -> crate::snow_breath::BREATHSTEAM { crate::snow_breath::BREATHSTEAM { radius: 0.034, cells: 1.5, density: 9.0, temp: 4.0, speed: 0.8, push: 12.0, expand: 2.0, attack: 0.08, jitter: 0.3, carry_k: 0.5, wake: 0.16, reach: 12.0 } }
