//! THE BRING-UP'S OWN PART of journal: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/journal.ts:202 knobTables)
pub fn knob_tables(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/journal.ts:748 SessionRecorder.cut)
pub fn session_recorder_cut(g: &mut crate::game::Game, why: &str, lost: Option<bool>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/journal.ts:757 SessionRecorder.slotWrote)
pub fn session_recorder_slot_wrote(g: &mut crate::game::Game, key: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/journal.ts:758 SessionRecorder.slotRead)
pub fn session_recorder_slot_read(g: &mut crate::game::Game, key: &str, text: &mut dyn FnMut(&mut crate::game::Game) -> Option<String>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/journal.ts:782 SessionRecorder.frameStart)
pub fn session_recorder_frame_start(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId, view: crate::journal::View) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/journal.ts:812 SessionRecorder.frameEnd)
pub fn session_recorder_frame_end(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/journal.ts:819 SessionRecorder.begin)
pub fn session_recorder_begin(g: &mut crate::game::Game) {}
