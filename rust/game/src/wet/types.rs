//! The types of `wet`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WetMem, src/game/wet.ts:50)
#[derive(Clone, Copy, Debug)]
pub struct WetMem {
    pub sky_t: f64,
    pub open: bool,
    pub prints_left: f64,
    pub last_out_wet_t: f64,
    pub drip_acc: f64,
}
impl WetMem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WetMem {
            sky_t: 0.0,
            open: false,
            prints_left: 0.0,
            last_out_wet_t: 0.0,
            drip_acc: 0.0,
        }
    }
}
impl Default for WetMem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PrintRec, src/game/wet.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct PrintRec {
    pub d: crate::render_decals::Decal,
    pub t0: f64,
    pub k0: f64,
}
impl PrintRec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PrintRec {
            d: crate::render_decals::Decal::zeroed(),
            t0: 0.0,
            k0: 0.0,
        }
    }
}
impl Default for PrintRec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wet.ts:55 prints, src/game/wet.ts:55)
#[derive(Clone, Debug)]
pub struct WetModPrintsValue {
    pub list: Vec<crate::wet::PrintRec>,
    pub age_t: f64,
}
impl WetModPrintsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WetModPrintsValue {
            list: Vec::new(),
            age_t: 0.0,
        }
    }
}
impl Default for WetModPrintsValue { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WetMod {
    /// (src/game/wet.ts:51 mem)
    pub mem: JsMap<CharId, crate::wet::WetMem>,
    /// (src/game/wet.ts:55 prints)
    pub prints: Option<crate::wet::WetModPrintsValue>,
    /// (src/game/wet.ts:131 dripN)
    pub drip_n: f64,
}
impl WetMod {
    pub fn blank() -> Self {
        WetMod {
            mem: JsMap::new(),
            prints: None,
            drip_n: 0.0,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WetStats {
    pub sky_tests: i32,
    pub prints: i32,
    pub drips: i32,
}

/// (src/game/wet.ts:29 WET_DEFAULTS: a table nothing writes)
pub fn wet_defaults() -> crate::knobs::WetK { crate::knobs::WetK { soak_per_sec: 0.16666666666666666, dry_per_sec: 0.04, bead_runoff_secs: 3.0, bead_density: 0.6, bead_size: 0.0075, trail_len: 0.11, rough_wet: 0.12, rough_dry: 0.55, darken: 1.0, darken_mul: 0.35, spec_gain: 1.0, drip_secs: 5.0, prints: 10.0, line_band: 0.12, print_fade_secs: 30.0 } }

/// (src/game/wet.ts:71 WET_SKY_SECS)
pub const WET_SKY_SECS: f64 = 0.25;

/// (src/game/wet.ts:71 SKY_TOP)
pub const SKY_TOP: f64 = 9.0;

/// (src/game/wet.ts:71 SKY_RING)
pub const SKY_RING: f64 = 0.35;

/// (src/game/wet.ts:72 SKY_OFFSETS: a table nothing writes)
pub fn sky_offsets() -> Vec<[f64; 2]> { alloc::vec![[0.0, 0.0], [SKY_RING, 0.0], [-SKY_RING, 0.0], [0.0, SKY_RING], [0.0, -SKY_RING]] }

/// (src/game/wet.ts:130 DRIP_FROM: a table nothing writes)
pub fn drip_from() -> [String; 5] { [String::from("handL"), String::from("handR"), String::from("head"), String::from("hips"), String::from("hips")] }

/// (src/game/wet.ts:189 WET_LINE_ALL)
pub const WET_LINE_ALL: f64 = 10000.0;

/// (src/game/wet.ts:194 KNOB_ORDER: a table nothing writes)
pub fn knob_order() -> Vec<String> { alloc::vec![String::from("beadDensity"), String::from("beadSize"), String::from("trailLen"), String::from("roughWet"), String::from("roughDry"), String::from("darken"), String::from("darkenMul"), String::from("specGain")] }

/// (src/game/wet.ts:219 WET_RANGE: a table nothing writes)
pub fn wet_range() -> JsMap<String, [f64; 2]> { { let mut t = JsMap::new(); t.set(String::from("soakPerSec"), [0.02, 1.0]); t.set(String::from("dryPerSec"), [0.005, 0.5]); t.set(String::from("beadRunoffSecs"), [0.5, 15.0]); t.set(String::from("beadDensity"), [0.0, 1.0]); t.set(String::from("beadSize"), [0.003, 0.02]); t.set(String::from("trailLen"), [0.02, 0.3]); t.set(String::from("roughWet"), [0.03, 0.5]); t.set(String::from("roughDry"), [0.2, 0.65]); t.set(String::from("darken"), [0.0, 1.5]); t.set(String::from("darkenMul"), [0.0, 0.6]); t.set(String::from("specGain"), [0.0, 2.0]); t.set(String::from("dripSecs"), [0.0, 30.0]); t.set(String::from("prints"), [0.0, 30.0]); t.set(String::from("printFadeSecs"), [5.0, 120.0]); t.set(String::from("lineBand"), [0.005, 0.6]); t } }
