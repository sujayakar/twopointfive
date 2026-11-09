//! The types of `rowset`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct RowSettings {
    pub hour: Option<crate::hours::HourId>,
    pub weather: Option<crate::weather_kinds::WeatherKind>,
}

/// (src/game/rowset.ts:29 BREAK)
pub const BREAK: bool = false;
