//! THE BRING-UP'S OWN PART of cloud_report: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/cloud/report.ts:70 hand)
pub fn hand(g: &mut crate::game::Game, r: crate::cloud_report::Report) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/cloud/report.ts:84 say)
pub fn say(g: &mut crate::game::Game, where_: &str, level: crate::cloud_report::ReportLevel, parts: &[crate::Opaque], data: Option<&JsMap<String, crate::Opaque>>, every_time: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/cloud/report.ts:94 reportError)
pub fn report_error(g: &mut crate::game::Game, where_: &str, err: crate::Opaque, data: Option<&JsMap<String, crate::Opaque>>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/cloud/report.ts:100 reportWarningLine)
pub fn report_warning_line(g: &mut crate::game::Game, where_: &str, said: &[crate::Opaque]) {}
