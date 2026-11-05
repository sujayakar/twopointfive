//! The types of `sensors`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// who answers a sight line while the senses are on the CPU (game.ts senseStep); None = the probe
#[derive(Clone, Debug)]
pub enum SensorsOwn {
    CpuSmokeAlong,
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SensorMode {
    Live,
    Record,
    Replay,
}
impl SensorMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            SensorMode::Live => "live",
            SensorMode::Record => "record",
            SensorMode::Replay => "replay",
        }
    }
}

/// (TypeScript: ProbeLike, src/game/sensors.ts:23)
#[derive(Clone, Debug)]
pub struct ProbeLike {
    pub results: JsMap<String, [f64; 3]>,
}
impl ProbeLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ProbeLike {
            results: JsMap::new(),
        }
    }
}
impl Default for ProbeLike { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum SensorEnc {
    Num(f64),
    List([f64; 1]),
    Nan,
    Inf,
    Inf2,
    V0,
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SensorRecItem {
    Text(String),
    Num(f64),
    List([f64; 1]),
}

/// (TypeScript: SensorJournal, src/game/sensors.ts:36)
#[derive(Clone, Debug)]
pub struct SensorJournal {
    pub keys: Vec<String>,
    pub recs: Vec<Vec<Option<crate::sensors::SensorJournalRecsItemItem>>>,
    pub frames: f64,
}
impl SensorJournal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SensorJournal {
            keys: Vec::new(),
            recs: Vec::new(),
            frames: 0.0,
        }
    }
}
impl Default for SensorJournal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SensorJournalRecsItemItem {
    Text(String),
    Num(f64),
    List([f64; 1]),
}

/// (TypeScript: SensorDivergence, src/game/sensors.ts:37)
#[derive(Clone, Debug)]
pub struct SensorDivergence {
    pub f: f64,
    pub kind: crate::sensors::SensorDivergenceKind,
    pub key: String,
    pub detail: String,
}
impl SensorDivergence {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SensorDivergence {
            f: 0.0,
            kind: crate::sensors::SensorDivergenceKind::UnjournaledRead,
            key: String::new(),
            detail: String::new(),
        }
    }
}
impl Default for SensorDivergence { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SensorDivergenceKind {
    UnjournaledRead,
    Unasked,
}
impl SensorDivergenceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SensorDivergenceKind::UnjournaledRead => "unjournaled-read",
            SensorDivergenceKind::Unasked => "unasked",
        }
    }
}

/// (TypeScript: SensorStats, src/game/sensors.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct SensorStats {
    pub reads: f64,
    pub asks: f64,
    pub strays: f64,
    pub frames: f64,
    pub recs: f64,
    pub divergences: f64,
    pub unasked: f64,
    pub unjournaled: f64,
}
impl SensorStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SensorStats {
            reads: 0.0,
            asks: 0.0,
            strays: 0.0,
            frames: 0.0,
            recs: 0.0,
            divergences: 0.0,
            unasked: 0.0,
            unjournaled: 0.0,
        }
    }
}
impl Default for SensorStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SensorStream, src/game/sensors.ts:73)
#[derive(Clone, Debug)]
pub struct SensorStream {
    pub mode: crate::sensors::SensorMode,
    pub frame: f64,
    pub in_frame: bool,
    pub stats: crate::sensors::SensorStats,
    pub first_divergence: Option<crate::sensors::SensorDivergence>,
    pub divergence_log: Vec<crate::sensors::SensorDivergence>,
    pub keys: Vec<String>,
    pub key_idx: JsMap<String, f64>,
    pub recs: Vec<Vec<Option<crate::sensors::SensorStreamRecsItemItem>>>,
    pub bytes: f64,
    pub prev_seq: Vec<f64>,
    pub prev_vals: Vec<Option<f64>>,
    pub cur_seq: Vec<f64>,
    pub cur_vals: Vec<Option<f64>>,
    pub src: Option<crate::sensors::SensorJournal>,
    pub cursor: f64,
    pub seq: Vec<f64>,
    pub vals: Vec<Option<f64>>,
    pub pending: JsMap<String, Vec<Option<f64>>>,
    pub last_served: JsMap<String, Option<f64>>,
}
impl SensorStream {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SensorStream {
            mode: crate::sensors::SensorMode::Live,
            frame: -1.0,
            in_frame: false,
            stats: crate::sensors::SensorStats { reads: 0.0, asks: 0.0, strays: 0.0, frames: 0.0, recs: 0.0, divergences: 0.0, unasked: 0.0, unjournaled: 0.0 },
            first_divergence: None,
            divergence_log: Vec::new(),
            keys: Vec::new(),
            key_idx: JsMap::new(),
            recs: Vec::new(),
            bytes: 16.0,
            prev_seq: Vec::new(),
            prev_vals: Vec::new(),
            cur_seq: Vec::new(),
            cur_vals: Vec::new(),
            src: None,
            cursor: 0.0,
            seq: Vec::new(),
            vals: Vec::new(),
            pending: JsMap::new(),
            last_served: JsMap::new(),
        }
    }
}
impl Default for SensorStream { fn default() -> Self { Self::blank() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SensorStreamRecsItemItem {
    Text(String),
    Num(f64),
    List([f64; 1]),
}

/// (TypeScript: Sensors, src/game/sensors.ts:88)
#[derive(Clone, Debug)]
pub struct Sensors {
    pub s: crate::sensors::SensorStream,
    pub own: Option<crate::sensors::SensorsOwn>,
    pub own_vals: JsMap<String, f64>,
}
impl Sensors {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Sensors {
            s: crate::sensors::SensorStream::blank(),
            own: None,
            own_vals: JsMap::new(),
        }
    }
}
impl Default for Sensors { fn default() -> Self { Self::blank() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SensorsPushRItem {
    Text(String),
    Num(f64),
    List([f64; 1]),
}
