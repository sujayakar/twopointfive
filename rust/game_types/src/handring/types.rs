//! The types of `handring`.
use alloc::string::String;

#[derive(Clone, Debug, Default)]
pub struct EMPTYWHY {
    pub far: String,
    pub run: String,
    pub held: String,
    pub no: String,
}

pub const EMPTY_WORD: &str = "nothing in reach";

pub const EMPTY_SHOW_SECS: f64 = 1.6;

pub const EMPTY_NEAR_M: f64 = 2.2;

pub const EMPTY_SHY_M: f64 = 0.2;

pub fn empty_why() -> crate::handring::EMPTYWHY { crate::handring::EMPTYWHY { far: String::from("step closer"), run: String::from("stop sprinting"), held: String::from("not in this mission"), no: String::from("not now") } }
