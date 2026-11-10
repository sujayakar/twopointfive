//! The types of `rowset`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct RowSettings {
    pub weather: Option<crate::weather_kinds::WeatherKind>,
}

pub const BREAK: bool = false;
