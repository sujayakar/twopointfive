//! The types of `snow_breath`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct BreathShownMenValue {
    pub who: CharId,
    pub at: f64,
    pub n: f64,
    pub steam: bool,
    pub src: f64,
}

