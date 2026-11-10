//! The types of `scenario_run`.
#![allow(unused_imports)]
use crate::{Words, ids::*, js::*, Quat, V3};
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

