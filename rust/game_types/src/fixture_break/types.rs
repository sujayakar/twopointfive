//! The types of `fixture_break`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Piece {
    pub box_: FrameBoxId,
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

#[derive(Clone, Debug, ToBytes, Default)]
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
    pub hang: Vec<FrameBoxId>,
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub recs: Vec<crate::fixture_break::Rec>,
    pub prev: JsMap<FixtureId, bool>,
    pub kind: JsMap<FixtureId, f64>,
    pub last_t: f64,
    pub scan: f64,
    pub keys: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FixtureBreakModFIXLOOKSpark {
    pub speed: f64,
    pub speed_jit: f64,
    pub spread: f64,
    pub life: f64,
    pub life_jit: f64,
    pub size: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FixtureBreakModFLOOR {
    pub on: Option<BoxId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RestOn {
    pub y: f64,
    pub flat: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlowAtOut {
    pub sparks: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
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

