//! The types of `rowset`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum RowOfQuerySearch {
    Text(String),
    URLSearchParams(crate::Opaque),
}

/// (TypeScript: inline at src/game/rowset.ts:43 rowSettings ->, src/game/rowset.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct RowSettings {
    pub hour: Option<crate::hours::HourId>,
    pub weather: Option<crate::weather_kinds::WeatherKind>,
}
impl RowSettings {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RowSettings {
            hour: None,
            weather: None,
        }
    }
}
impl Default for RowSettings { fn default() -> Self { Self::zeroed() } }

/// (src/game/rowset.ts:29 BREAK)
pub const BREAK: bool = false;
