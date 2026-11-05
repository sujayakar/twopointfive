//! THE BRING-UP'S OWN PART of morale: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ROUND THREE: THE LENT SCRIPT COMES HOME (conventions 4, "an object lent and written back"). guards.ts:2145 lends `S.script` to the man for the frame and
// runGuardScript writes `arrived` (and its clocks) INTO THAT OBJECT, which morale.ts:478-500 reads. p_guards_07 hands back what the run left; the skeleton's stub
// (INVENTED, no origin line) was on nobody's list, and four parts' notes said it would panic. None back: the run ended the script (squad.ts:137), nothing to put home.
pub fn morale_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(s)) = (ran, g.m.morale.state_of.get_mut(gd)) { s.script = ran; }
}
