//! The types of `cloud_report`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ReportLevel {
    #[default] Error,
    Warning,
}
impl ReportLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            ReportLevel::Error => "error",
            ReportLevel::Warning => "warning",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Report {
    pub where_: String,
    pub message: String,
    pub stack: Vec<String>,
    pub level: crate::cloud_report::ReportLevel,
    pub data: Option<JsMap<String, crate::Opaque>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct REPORT {
    pub held: f64,
    pub held_warnings: f64,
    pub stack: f64,
    pub message: f64,
    pub keys: f64,
    pub makes: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CloudReportModPlain {
    pub error: Option<crate::FnField>,
    pub warn: Option<crate::FnField>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CloudReportMod {
    /// (src/cloud/report.ts:28 sink)
    pub sink: Option<crate::FnField>,
    /// (src/cloud/report.ts:28 inside)
    pub inside: bool,
    /// (src/cloud/report.ts:28 heldErrors)
    pub held_errors: f64,
    /// (src/cloud/report.ts:28 heldWarnings)
    pub held_warnings: f64,
    /// (src/cloud/report.ts:29 held)
    pub held: Vec<crate::cloud_report::Report>,
    /// (src/cloud/report.ts:31 made)
    pub made: JsMap<String, crate::cloud_report::Report>,
    /// (src/cloud/report.ts:44 plain)
    pub plain: crate::cloud_report::CloudReportModPlain,
    /// (src/cloud/report.ts:83 printed)
    pub printed: JsSet<String>,
}
impl CloudReportMod {
    pub fn blank() -> Self {
        CloudReportMod {
            sink: None,
            inside: false,
            held_errors: 0.0,
            held_warnings: 0.0,
            held: Vec::new(),
            made: JsMap::new(),
            plain: crate::cloud_report::CloudReportModPlain { error: None, warn: None },
            printed: JsSet::new(),
        }
    }
}

/// (src/cloud/report.ts:26 REPORT: a table nothing writes)
pub fn report() -> crate::cloud_report::REPORT { crate::cloud_report::REPORT { held: 32.0, held_warnings: 16.0, stack: 6.0, message: 300.0, keys: 8.0, makes: 200.0 } }
