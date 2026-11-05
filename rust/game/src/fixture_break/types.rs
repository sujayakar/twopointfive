//! The types of `fixture_break`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Piece, src/game/fixtureBreak.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub box_: BoxId,
    pub v: V3,
    pub axis: V3,
    pub rate: f64,
    pub ang: f64,
    pub q0: Quat,
    pub floor_y: f64,
    pub yaw: f64,
    pub delay: f64,
    pub rest: bool,
    pub bounced: bool,
    pub glass: bool,
    pub on: Option<BoxId>,
    pub on_x: f64,
    pub on_z: f64,
}
impl Piece {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Piece {
            box_: BoxId(0),
            v: [0.0; 3],
            axis: [0.0; 3],
            rate: 0.0,
            ang: 0.0,
            q0: [0.0; 4],
            floor_y: 0.0,
            yaw: 0.0,
            delay: 0.0,
            rest: false,
            bounced: false,
            glass: false,
            on: None,
            on_x: 0.0,
            on_z: 0.0,
        }
    }
}
impl Default for Piece { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Rec, src/game/fixtureBreak.ts:54)
#[derive(Clone, Debug)]
pub struct Rec {
    pub t: FixtureId,
    pub kind: f64,
    pub born: f64,
    pub seed: f64,
    pub seen: f64,
    pub hinge: V3,
    pub uh: V3,
    pub l: f64,
    pub w: f64,
    pub th: f64,
    pub om: f64,
    pub rest: f64,
    pub still: bool,
    pub hang: Vec<BoxId>,
    pub pieces: Vec<crate::fixture_break::Piece>,
    pub pop: f64,
    pub last_slot: f64,
    pub glass_out: bool,
    pub tip: V3,
    pub emits: f64,
    pub phase: f64,
    pub fall_at: f64,
    pub h0: V3,
    pub th0: f64,
    pub t0: f64,
    pub lie: V3,
    pub uh0: V3,
    pub skew: f64,
    pub lay: f64,
}
impl Rec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rec {
            t: FixtureId(0),
            kind: 0.0,
            born: 0.0,
            seed: 0.0,
            seen: 0.0,
            hinge: [0.0; 3],
            uh: [0.0; 3],
            l: 0.0,
            w: 0.0,
            th: 0.0,
            om: 0.0,
            rest: 0.0,
            still: false,
            hang: Vec::new(),
            pieces: Vec::new(),
            pop: 0.0,
            last_slot: 0.0,
            glass_out: false,
            tip: [0.0; 3],
            emits: 0.0,
            phase: 0.0,
            fall_at: 0.0,
            h0: [0.0; 3],
            th0: 0.0,
            t0: 0.0,
            lie: [0.0; 3],
            uh0: [0.0; 3],
            skew: 0.0,
            lay: 0.0,
        }
    }
}
impl Default for Rec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/fixtureBreak.ts:63)
#[derive(Clone, Debug)]
pub struct State {
    pub recs: Vec<crate::fixture_break::Rec>,
    pub by: JsMap<FixtureId, crate::fixture_break::Rec>,
    pub prev: JsMap<FixtureId, bool>,
    pub kind: JsMap<FixtureId, f64>,
    pub last_t: f64,
    pub scan: f64,
    pub keys: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            recs: Vec::new(),
            by: JsMap::new(),
            prev: JsMap::new(),
            kind: JsMap::new(),
            last_t: 0.0,
            scan: 0.0,
            keys: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fixtureBreak.ts:28 FIXLOOK, src/game/fixtureBreak.ts:28)
#[derive(Clone, Debug)]
pub struct FixtureBreakModFIXLOOK {
    pub on: bool,
    pub max_panels: f64,
    pub rest_deg: [f64; 2],
    pub swing_k: f64,
    pub swing_c: f64,
    pub kick: f64,
    pub hang_secs: f64,
    pub slide: f64,
    pub skew: f64,
    pub pops: Vec<f64>,
    pub pop_sparks: Vec<f64>,
    pub pop_gain: Vec<f64>,
    pub pop_tau: f64,
    pub glow_tip: f64,
    pub spark: crate::fixture_break::FixtureBreakModFIXLOOKSpark,
    pub spark_aside: f64,
    pub spark_down: f64,
    pub spark_up: f64,
    pub lead: V3,
    pub spit_from: f64,
    pub spit_secs: f64,
    pub spit_tau: f64,
    pub spit_rate: f64,
    pub spit_gain: f64,
    pub glass: f64,
    pub glass_size: f64,
    pub glass_at: f64,
    pub shards: f64,
    pub tray: V3,
    pub cover: V3,
    pub face: V3,
    pub tube: V3,
    pub soot: V3,
    pub diffuser: V3,
    pub shard: V3,
}
impl FixtureBreakModFIXLOOK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FixtureBreakModFIXLOOK {
            on: false,
            max_panels: 0.0,
            rest_deg: [0.0; 2],
            swing_k: 0.0,
            swing_c: 0.0,
            kick: 0.0,
            hang_secs: 0.0,
            slide: 0.0,
            skew: 0.0,
            pops: Vec::new(),
            pop_sparks: Vec::new(),
            pop_gain: Vec::new(),
            pop_tau: 0.0,
            glow_tip: 0.0,
            spark: crate::fixture_break::FixtureBreakModFIXLOOKSpark::zeroed(),
            spark_aside: 0.0,
            spark_down: 0.0,
            spark_up: 0.0,
            lead: [0.0; 3],
            spit_from: 0.0,
            spit_secs: 0.0,
            spit_tau: 0.0,
            spit_rate: 0.0,
            spit_gain: 0.0,
            glass: 0.0,
            glass_size: 0.0,
            glass_at: 0.0,
            shards: 0.0,
            tray: [0.0; 3],
            cover: [0.0; 3],
            face: [0.0; 3],
            tube: [0.0; 3],
            soot: [0.0; 3],
            diffuser: [0.0; 3],
            shard: [0.0; 3],
        }
    }
}
impl Default for FixtureBreakModFIXLOOK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fixtureBreak.ts:41 FixtureBreakModFIXLOOK.spark, src/game/fixtureBreak.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct FixtureBreakModFIXLOOKSpark {
    pub speed: f64,
    pub speed_jit: f64,
    pub spread: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
}
impl FixtureBreakModFIXLOOKSpark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FixtureBreakModFIXLOOKSpark {
            speed: 0.0,
            speed_jit: 0.0,
            spread: 0.0,
            life: 0.0,
            life_jit: 0.0,
            size: 0.0,
        }
    }
}
impl Default for FixtureBreakModFIXLOOKSpark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fixtureBreak.ts:92 FLOOR, src/game/fixtureBreak.ts:92)
#[derive(Clone, Copy, Debug)]
pub struct FixtureBreakModFLOOR {
    pub on: Option<BoxId>,
}
impl FixtureBreakModFLOOR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FixtureBreakModFLOOR {
            on: None,
        }
    }
}
impl Default for FixtureBreakModFLOOR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fixtureBreak.ts:192 SPARKS, src/game/fixtureBreak.ts:192)
#[derive(Clone, Copy, Debug)]
pub struct SPARKS {
    pub sparks: f64,
}
impl SPARKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SPARKS {
            sparks: 0.0,
        }
    }
}
impl Default for SPARKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fixtureBreak.ts:94 restOn ->, src/game/fixtureBreak.ts:94)
#[derive(Clone, Copy, Debug)]
pub struct RestOn {
    pub y: f64,
    pub flat: bool,
}
impl RestOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RestOn {
            y: 0.0,
            flat: false,
        }
    }
}
impl Default for RestOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/fixtureBreak.ts:185 glowAt(out), src/game/fixtureBreak.ts:185)
#[derive(Clone, Copy, Debug)]
pub struct GlowAtOut {
    pub sparks: f64,
}
impl GlowAtOut {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlowAtOut {
            sparks: 0.0,
        }
    }
}
impl Default for GlowAtOut { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct FixtureBreakMod {
    /// (src/game/fixtureBreak.ts:28 FIXLOOK)
    pub fixlook: crate::fixture_break::FixtureBreakModFIXLOOK,
    /// (src/game/fixtureBreak.ts:64 STATE)
    pub state: Option<crate::fixture_break::State>,
    /// (src/game/fixtureBreak.ts:92 FLOOR)
    pub floor: crate::fixture_break::FixtureBreakModFLOOR,
}
impl FixtureBreakMod {
    pub fn blank() -> Self {
        FixtureBreakMod {
            fixlook: crate::fixture_break::FixtureBreakModFIXLOOK { on: true, max_panels: 12.0, rest_deg: [55.0, 65.0], swing_k: 30.0, swing_c: 2.2, kick: 3.4, hang_secs: 1.9, slide: 0.35, skew: 0.5, pops: alloc::vec![0.62, 1.0, 1.5, 2.2, 3.1], pop_sparks: alloc::vec![30.0, 22.0, 13.0, 9.0, 6.0], pop_gain: alloc::vec![1.0, 0.8, 0.55, 0.4, 0.3], pop_tau: 0.07, glow_tip: 12.0, spark: crate::fixture_break::FixtureBreakModFIXLOOKSpark { speed: 6.0, speed_jit: 0.6, spread: 1.15, life: 0.42, life_jit: 0.5, size: 0.002 }, spark_aside: 0.9, spark_down: 0.55, spark_up: 0.5, lead: [0.018, 0.008, 0.012], spit_from: 3.6, spit_secs: 45.0, spit_tau: 16.0, spit_rate: 0.22, spit_gain: 0.2, glass: 26.0, glass_size: 0.013, glass_at: 0.15, shards: 5.0, tray: [0.4, 0.41, 0.42], cover: [0.55, 0.55, 0.56], face: [0.78, 0.78, 0.75], tube: [0.8, 0.8, 0.78], soot: [0.06, 0.055, 0.05], diffuser: [0.8, 0.83, 0.86], shard: [0.85, 0.9, 0.9] },
            state: None,
            floor: crate::fixture_break::FixtureBreakModFLOOR { on: None },
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FixtureBreakStats {
    pub panels: i32,
    pub lamps: i32,
    pub beats: i32,
    pub boxes: i32,
    pub sparks: i32,
}

/// (src/game/fixtureBreak.ts:68 LOOK)
pub const LOOK: f64 = 10.0;

/// (src/game/fixtureBreak.ts:69 DOWN: a table nothing writes)
pub fn down() -> V3 { [0.0, -1.0, 0.0] }

/// (src/game/fixtureBreak.ts:99 REST_AT: a table nothing writes)
pub fn rest_at() -> Vec<f64> { alloc::vec![0.15, 0.5, 0.85] }

/// (src/game/fixtureBreak.ts:192 SPARKS: a table nothing writes)
pub fn sparks() -> crate::fixture_break::SPARKS { crate::fixture_break::SPARKS { sparks: 0.0 } }
