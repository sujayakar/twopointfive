//! The types of `scenario_run`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub enum StartFromQuerySearch {
    Text(String),
    URLSearchParams(crate::query::Query),
}

#[derive(Clone, Debug, Default)]
pub struct StartFromQuery {
    pub why: Option<String>,
}

/// (src/game/scenarioRun.ts:18 WORDS: a table nothing writes)
pub fn words() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("nightbreach"), String::from("night breach")); t.set(String::from("roomclear"), String::from("room clear")); t.set(String::from("rescue"), String::from("hostage rescue")); t.set(String::from("quiet"), String::from("quiet mission")); t } }
