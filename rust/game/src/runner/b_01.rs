//! THE BRING-UP'S OWN PART of runner: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the script runner.ts lent for the frame (guards.ts:2144), as the run left it, put back on his row (conventions 4; see morale's b_01).
pub fn runner_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(r)) = (ran, g.m.runner.state_of.get_mut(gd)) { r.script = ran; }
}
