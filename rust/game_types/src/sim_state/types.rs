//! The types of `sim_state`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct Layout {
    pub words: f64,
    pub games: f64,
    pub word: f64,
    pub off: Vec<f64>,
    pub len: Vec<f64>,
    pub pool: Vec<bool>,
}

pub fn sim_parts() -> [String; 13] { [String::from("head"), String::from("dice"), String::from("doors"), String::from("falls"), String::from("props"), String::from("slabs"), String::from("items"), String::from("men"), String::from("lights"), String::from("weather"), String::from("smoke"), String::from("poses"), String::from("panes")] }

pub const P_DOORS: f64 = 2.0;

pub const P_FALLS: f64 = 3.0;

pub const P_PROPS: f64 = 4.0;

pub const P_SLABS: f64 = 5.0;

pub const P_ITEMS: f64 = 6.0;

pub const P_WEATHER: f64 = 9.0;

