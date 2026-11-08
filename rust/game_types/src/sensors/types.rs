//! The types of `sensors`, a session file's own record of what was read of the picture, from when something was.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

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
    pub recs: Vec<Vec<Option<crate::sensors::SensorRecItem>>>,
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
