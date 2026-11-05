#![allow(unused_variables)]
//! Every function and method of `journal`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/journal.ts:202 knobTables)
pub fn knob_tables(g: &mut crate::game::Game) { todo!() }

/// (src/game/journal.ts:714 SessionRecorder.take)
pub fn session_recorder_take(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/journal.ts:723 SessionRecorder.constructor)
pub fn session_recorder_new(g: &mut crate::game::Game) -> crate::journal::SessionRecorder { todo!() }

/// (src/game/journal.ts:725 SessionRecorder.recording)
pub fn session_recorder_recording(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/journal.ts:728 SessionRecorder.request)
pub fn session_recorder_request(g: &mut crate::game::Game, opts: Option<&crate::journal::RecordOpts>) -> bool { todo!() }

/// (src/game/journal.ts:748 SessionRecorder.cut)
pub fn session_recorder_cut(g: &mut crate::game::Game, why: &str, lost: Option<bool>) { todo!() }

/// (src/game/journal.ts:757 SessionRecorder.slotWrote)
pub fn session_recorder_slot_wrote(g: &mut crate::game::Game, key: &str) { todo!() }

/// (src/game/journal.ts:758 SessionRecorder.slotRead)
pub fn session_recorder_slot_read(g: &mut crate::game::Game, key: &str, text: &mut dyn FnMut(&mut crate::game::Game) -> Option<String>) { todo!() }

/// (src/game/journal.ts:765 SessionRecorder.finish)
pub fn session_recorder_finish(g: &mut crate::game::Game, why: &str, between: Option<bool>, lost: Option<bool>) -> crate::journal::SessionFile { todo!() }

/// (src/game/journal.ts:782 SessionRecorder.frameStart)
pub fn session_recorder_frame_start(g: &mut crate::game::Game, dt: f64, input: InputId, view: crate::journal::View) { todo!() }

/// (src/game/journal.ts:812 SessionRecorder.frameEnd)
pub fn session_recorder_frame_end(g: &mut crate::game::Game) { todo!() }

/// (src/game/journal.ts:819 SessionRecorder.begin)
pub fn session_recorder_begin(g: &mut crate::game::Game) { todo!() }

/// (src/game/journal.ts:844 canStart)
pub fn can_start(g: &mut crate::game::Game) -> Option<String> { todo!() }
