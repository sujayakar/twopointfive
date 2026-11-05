//! The types of `cloud_report`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ReportLevel {
    Error,
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

/// (TypeScript: Report, src/cloud/report.ts:23)
#[derive(Clone, Copy, Debug)]
pub struct Report {
    pub level: crate::cloud_report::ReportLevel,
}
impl Report {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Report {
            level: crate::cloud_report::ReportLevel::Error,
        }
    }
}
impl Default for Report { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/cloud/report.ts:26 REPORT, src/cloud/report.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct REPORT {
    pub held: f64,
    pub held_warnings: f64,
    pub stack: f64,
    pub message: f64,
    pub keys: f64,
    pub makes: f64,
}
impl REPORT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        REPORT {
            held: 0.0,
            held_warnings: 0.0,
            stack: 0.0,
            message: 0.0,
            keys: 0.0,
            makes: 0.0,
        }
    }
}
impl Default for REPORT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/cloud/report.ts:44 plain, src/cloud/report.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct CloudReportModPlain {
    pub error: Option<crate::FnField>,
    pub warn: Option<crate::FnField>,
}
impl CloudReportModPlain {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CloudReportModPlain {
            error: None,
            warn: None,
        }
    }
}
impl Default for CloudReportModPlain { fn default() -> Self { Self::zeroed() } }

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
