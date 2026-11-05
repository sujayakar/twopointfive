//! THE BRING-UP'S OWN PART of scene_look: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The mission a host asked the men to be dressed for.   (look.ts suitAs)
// SKELETON-GAP: suit_as -> Option<Option<SuitAs>> (look.ts:26 `'quiet' | null | undefined`: undefined = nobody asked, null = no mission). None here is "nobody asked".
pub fn suit_as(g: &mut Game) -> Option<SuitAs> { g.m.scene_look.suit_asked.map(|_| SuitAs::Quiet) }
/// Do the men wear the look's dress here.   (look.ts lookDress)
pub fn look_dress(g: &mut Game) -> bool { g.k.look.on && g.m.scene_look.look_place }
/// The look on, in a place that is not the look's.   (look.ts lookOffice)
pub fn look_office(g: &mut Game) -> bool { g.k.look.on && !g.m.scene_look.look_place }
