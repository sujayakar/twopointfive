//! The types of `fire`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FireStats, src/game/fire.ts:156)
#[derive(Clone, Copy, Debug)]
pub struct FireStats {
    pub fires: f64,
    pub lit: f64,
    pub frames: f64,
    pub cracks: f64,
    pub lo: f64,
    pub hi: f64,
    pub sum: f64,
    pub n: f64,
}
impl FireStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireStats {
            fires: 0.0,
            lit: 0.0,
            frames: 0.0,
            cracks: 0.0,
            lo: 0.0,
            hi: 0.0,
            sum: 0.0,
            n: 0.0,
        }
    }
}
impl Default for FireStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FireShown, src/game/fire.ts:222)
#[derive(Clone, Debug)]
pub struct FireShown {
    pub n: f64,
    pub time: f64,
    pub stir: Option<Vec<f64>>,
    pub stir_at: Option<Vec<f64>>,
    pub bed: Vec<f64>,
    pub smoke: Vec<f64>,
    pub smokes: f64,
    pub beds: f64,
    pub ember: Vec<f64>,
    pub embers: f64,
    pub gas: Vec<f64>,
    pub gas_mix: Vec<f64>,
    pub gases: f64,
    pub soot_mix: Option<Vec<f64>>,
    pub kept: Option<f64>,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}
impl FireShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireShown {
            n: 0.0,
            time: 0.0,
            stir: None,
            stir_at: None,
            bed: Vec::new(),
            smoke: Vec::new(),
            smokes: 0.0,
            beds: 0.0,
            ember: Vec::new(),
            embers: 0.0,
            gas: Vec::new(),
            gas_mix: Vec::new(),
            gases: 0.0,
            soot_mix: None,
            kept: None,
            serial: 0,
        }
    }
}
impl Default for FireShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:167 FIRE_LOOK_SMOKE, src/game/fire.ts:167)
#[derive(Clone, Copy, Debug)]
pub struct FIRELOOKSMOKE {
    pub up: f64,
    pub radius: f64,
    pub density: f64,
    pub puff_every: f64,
    pub puff_secs: f64,
    pub puff_up: f64,
    pub puff_radius: f64,
    pub puff_off: f64,
    pub puff_density: f64,
    pub puff_temp: f64,
    pub puff_speed: f64,
    pub puff_push: f64,
    pub puff_expand: f64,
    pub soot: V3,
    pub soot_ease: f64,
    pub ember_up: f64,
    pub ember_spread: f64,
    pub ember_jit: f64,
    pub ember_speed: f64,
    pub ember_fog: bool,
    pub snow: crate::fire::FIRELOOKSMOKESnow,
}
impl FIRELOOKSMOKE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FIRELOOKSMOKE {
            up: 0.0,
            radius: 0.0,
            density: 0.0,
            puff_every: 0.0,
            puff_secs: 0.0,
            puff_up: 0.0,
            puff_radius: 0.0,
            puff_off: 0.0,
            puff_density: 0.0,
            puff_temp: 0.0,
            puff_speed: 0.0,
            puff_push: 0.0,
            puff_expand: 0.0,
            soot: [0.0; 3],
            soot_ease: 0.0,
            ember_up: 0.0,
            ember_spread: 0.0,
            ember_jit: 0.0,
            ember_speed: 0.0,
            ember_fog: false,
            snow: crate::fire::FIRELOOKSMOKESnow::zeroed(),
        }
    }
}
impl Default for FIRELOOKSMOKE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:198 FIRELOOKSMOKE.snow, src/game/fire.ts:198)
#[derive(Clone, Copy, Debug)]
pub struct FIRELOOKSMOKESnow {
    pub speed: f64,
    pub reach: f64,
    pub spread: f64,
    pub mouth: f64,
    pub top: f64,
}
impl FIRELOOKSMOKESnow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FIRELOOKSMOKESnow {
            speed: 0.0,
            reach: 0.0,
            spread: 0.0,
            mouth: 0.0,
            top: 0.0,
        }
    }
}
impl Default for FIRELOOKSMOKESnow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:230 fireShown ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Debug)]
pub struct FireShown2 {
    pub n: f64,
    pub time: f64,
    pub stir: Option<Vec<f64>>,
    pub stir_at: Option<Vec<f64>>,
    pub bed: Vec<f64>,
    pub smoke: Vec<f64>,
    pub smokes: f64,
    pub beds: f64,
    pub ember: Vec<f64>,
    pub embers: f64,
    pub gas: Vec<f64>,
    pub gas_mix: Vec<f64>,
    pub gases: f64,
    pub soot_mix: Option<Vec<f64>>,
    pub kept: Option<f64>,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}
impl FireShown2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireShown2 {
            n: 0.0,
            time: 0.0,
            stir: None,
            stir_at: None,
            bed: Vec::new(),
            smoke: Vec::new(),
            smokes: 0.0,
            beds: 0.0,
            ember: Vec::new(),
            embers: 0.0,
            gas: Vec::new(),
            gas_mix: Vec::new(),
            gases: 0.0,
            soot_mix: None,
            kept: None,
            serial: 0,
        }
    }
}
impl Default for FireShown2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:247 firePops ->, src/game/fire.ts:247)
#[derive(Clone, Copy, Debug)]
pub struct FirePopsItem {
    pub at: f64,
    pub hard: f64,
    pub seed: f64,
}
impl FirePopsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FirePopsItem {
            at: 0.0,
            hard: 0.0,
            seed: 0.0,
        }
    }
}
impl Default for FirePopsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:269 mouthOf ->, src/game/fire.ts:269)
#[derive(Clone, Copy, Debug)]
pub struct MouthOf {
    pub pos: V3,
    pub r: f64,
    pub lid: f64,
}
impl MouthOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MouthOf {
            pos: [0.0; 3],
            r: 0.0,
            lid: 0.0,
        }
    }
}
impl Default for MouthOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fire.ts:279 windAt ->, src/game/fire.ts:279)
#[derive(Clone, Copy, Debug)]
pub struct WindAt {
    pub wv: V3,
    pub wl: f64,
    pub wind: [f64; 2],
}
impl WindAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WindAt {
            wv: [0.0; 3],
            wl: 0.0,
            wind: [0.0; 2],
        }
    }
}
impl Default for WindAt { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct FireMod {
    /// (src/game/fire.ts:223 states)
    pub states: Option<crate::fire::FireStats>,
}
impl FireMod {
    pub fn blank() -> Self {
        FireMod {
            states: None,
        }
    }
}

/// (src/game/fire.ts:46 FIRE_SIZE)
pub const FIRE_SIZE: f64 = 1.1;

/// (src/game/fire.ts:48 FIRE_SHIMMER)
pub const FIRE_SHIMMER: f64 = 0.019;

/// (src/game/fire.ts:48 FIRE_PLUME)
pub const FIRE_PLUME: f64 = 1.0;

/// (src/game/fire.ts:48 FIRE_EMBERS)
pub const FIRE_EMBERS: f64 = 60.0;

/// (src/game/fire.ts:51 FIRE_BRIGHT)
pub const FIRE_BRIGHT: f64 = 1.25;

/// (src/game/fire.ts:51 FIRE_REACH)
pub const FIRE_REACH: f64 = 1.4;

/// (src/game/fire.ts:51 FIRE_LIFT)
pub const FIRE_LIFT: f64 = 0.5;

/// (src/game/fire.ts:51 FIRE_LIFT_WAS)
pub const FIRE_LIFT_WAS: f64 = 0.25;

/// (src/game/fire.ts:51 FIRE_WARM: a table nothing writes)
pub fn fire_warm() -> Vec<f64> { alloc::vec![1.0, 0.72, 0.51] }

/// (src/game/fire.ts:51 FIRE_WANDER)
pub const FIRE_WANDER: f64 = 0.14;

/// (src/game/fire.ts:51 FIRE_WANDER_UP)
pub const FIRE_WANDER_UP: f64 = 0.1;

/// (src/game/fire.ts:51 FIRE_WANDER_WAS)
pub const FIRE_WANDER_WAS: f64 = 0.06;

/// (src/game/fire.ts:51 FIRE_WANDER_UP_WAS)
pub const FIRE_WANDER_UP_WAS: f64 = 0.04;

/// (src/game/fire.ts:52 FIRE_REACT)
pub const FIRE_REACT: f64 = 1.0;

/// (src/game/fire.ts:52 FIRE_LEAN)
pub const FIRE_LEAN: f64 = 0.2;

/// (src/game/fire.ts:52 FIRE_LEAN_WAS)
pub const FIRE_LEAN_WAS: f64 = 0.07;

/// (src/game/fire.ts:52 FIRE_STEPS)
pub const FIRE_STEPS: f64 = 40.0;

/// (src/game/fire.ts:167 FIRE_LOOK_SMOKE: a table nothing writes)
pub fn fire_look_smoke() -> crate::fire::FIRELOOKSMOKE { crate::fire::FIRELOOKSMOKE { up: 0.8, radius: 0.2, density: 0.8, puff_every: 0.42, puff_secs: 0.26, puff_up: 0.95, puff_radius: 0.24, puff_off: 0.09, puff_density: 3.2, puff_temp: 30.0, puff_speed: 1.25, puff_push: 9.0, puff_expand: 2.2, soot: [0.46, 0.48, 0.53], soot_ease: 0.5, ember_up: 0.72, ember_spread: 0.2, ember_jit: 0.4, ember_speed: 1.5, ember_fog: true, snow: crate::fire::FIRELOOKSMOKESnow { speed: 2.6, reach: 2.2, spread: 0.2, mouth: 1.3, top: 4.5 } } }

/// (src/game/fire.ts:234 TAU)
pub const TAU: f64 = 6.283185307179586;

/// (src/game/fire.ts:236 BED_K)
pub const BED_K: f64 = 0.6666666666666666;

/// (src/game/fire.ts:320 NO_JETS: a table nothing writes)
pub fn no_jets() -> Vec<crate::smoke_types::WindGpuSource> { Vec::new() }
