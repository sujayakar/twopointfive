//! The types of `scenario_run`.
use alloc::string::String;

#[derive(Clone, Debug)]
pub enum StartFromQuerySearch {
    Text(String),
    URLSearchParams(crate::query::Query),
}

#[derive(Clone, Debug, Default)]
pub struct StartFromQuery {
    pub why: Option<String>,
}

