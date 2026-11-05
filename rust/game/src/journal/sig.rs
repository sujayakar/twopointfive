#![allow(unused_variables)]
//! Every function and method of `journal`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/journal.ts:207 knobTables)
pub fn knob_tables(g: &mut crate::game::Game) { todo!() }

/// (src/game/journal.ts:725 SessionRecorder.take)
pub fn session_recorder_take(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/journal.ts:734 SessionRecorder.constructor)
pub fn session_recorder_new(g: &mut crate::game::Game) -> crate::journal::SessionRecorder { todo!() }

/// (src/game/journal.ts:736 SessionRecorder.recording)
pub fn session_recorder_recording(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/journal.ts:739 SessionRecorder.request)
pub fn session_recorder_request(g: &mut crate::game::Game, opts: Option<&crate::journal::RecordOpts>) -> bool { todo!() }

/// (src/game/journal.ts:759 SessionRecorder.cut)
pub fn session_recorder_cut(g: &mut crate::game::Game, why: &str, lost: Option<bool>) { todo!() }

/// (src/game/journal.ts:768 SessionRecorder.slotWrote)
pub fn session_recorder_slot_wrote(g: &mut crate::game::Game, key: &str) { todo!() }

/// (src/game/journal.ts:769 SessionRecorder.slotRead)
pub fn session_recorder_slot_read(g: &mut crate::game::Game, key: &str, text: &mut dyn FnMut(&mut crate::game::Game) -> Option<String>) { todo!() }

/// (src/game/journal.ts:776 SessionRecorder.finish)
pub fn session_recorder_finish(g: &mut crate::game::Game, why: &str, between: Option<bool>, lost: Option<bool>) -> crate::journal::SessionFile { todo!() }

/// (src/game/journal.ts:793 SessionRecorder.frameStart)
pub fn session_recorder_frame_start(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId, view: crate::journal::View) { todo!() }

/// (src/game/journal.ts:823 SessionRecorder.frameEnd)
pub fn session_recorder_frame_end(g: &mut crate::game::Game) { todo!() }

/// (src/game/journal.ts:830 SessionRecorder.begin)
pub fn session_recorder_begin(g: &mut crate::game::Game) { todo!() }

/// (src/game/journal.ts:855 canStart)
pub fn can_start(g: &mut crate::game::Game) -> Option<String> { todo!() }
