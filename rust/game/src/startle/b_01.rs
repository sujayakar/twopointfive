//! THE BRING-UP'S OWN PART of startle: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// ROUND THREE: the script startle.ts lent for the frame (guards.ts:2134), as the run left it, put back on his row: startle.ts:133 reads its `arrived` (conventions 4).
pub fn startle_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(s)) = (ran, g.m.startle.state_of.get_mut(gd)) { if s.script.is_some() { s.script = Some(ran); } }
}
