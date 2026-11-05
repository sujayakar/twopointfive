#![allow(unused_variables)]
//! Every function and method of `cloud_report`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/cloud/report.ts:32 madeKey)
pub fn made_key(level: crate::cloud_report::ReportLevel, where_: &str, first: crate::Opaque) -> String { todo!() }

/// (src/cloud/report.ts:37 once)
pub fn once(g: &mut crate::game::Game, where_: &str, level: crate::cloud_report::ReportLevel, parts: &[crate::Opaque], data: Option<&JsMap<String, crate::Opaque>>, skip: f64) -> Option<crate::cloud_report::Report> { todo!() }

/// (src/cloud/report.ts:46 words)
pub fn words(x: crate::Opaque) -> String { todo!() }

/// (src/cloud/report.ts:56 brief)
pub fn brief(x: crate::Opaque, top: Option<bool>) -> crate::Opaque { todo!() }

/// (src/cloud/report.ts:66 frames)
pub fn frames(stack: crate::Opaque, drop: Option<f64>) -> Vec<String> { todo!() }

/// (src/cloud/report.ts:70 hand)
pub fn hand(g: &mut crate::game::Game, r: crate::cloud_report::Report) { todo!() }

/// (src/cloud/report.ts:76 room)
pub fn room(g: &mut crate::game::Game, level: crate::cloud_report::ReportLevel) -> bool { todo!() }

/// (src/cloud/report.ts:77 make)
pub fn make(where_: &str, level: crate::cloud_report::ReportLevel, parts: &[crate::Opaque], data: Option<&JsMap<String, crate::Opaque>>, skip: f64) -> crate::cloud_report::Report { todo!() }

/// (src/cloud/report.ts:84 say)
pub fn say(g: &mut crate::game::Game, where_: &str, level: crate::cloud_report::ReportLevel, parts: &[crate::Opaque], data: Option<&JsMap<String, crate::Opaque>>, every_time: bool) { todo!() }

/// (src/cloud/report.ts:94 reportError)
pub fn report_error(g: &mut crate::game::Game, where_: &str, err: crate::Opaque, data: Option<&JsMap<String, crate::Opaque>>) { todo!() }

/// (src/cloud/report.ts:100 reportWarningLine)
pub fn report_warning_line(g: &mut crate::game::Game, where_: &str, said: &[crate::Opaque]) { todo!() }
