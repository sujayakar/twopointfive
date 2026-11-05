//! THE BRING-UP'S OWN PART of sensors: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// TEMP-BRINGUP: THE PROBE'S TABLE OF ASKS, as much of it as the game reads back. render/probe.ts keeps the frame's asks (`pending`, 64 at most) and the game
// reads `room` (fogofwar.ts:680). The table is the host's; until a host seam hands `room` in, the bring-up counts the asks it is sent. A host that draws
// nothing never empties the table (the TypeScript's headless one does not either), so the count runs on from what the host says it held as the tick opened.
// SKELETON-GAP: ProbeLike { room } filled by the host each tick, or the table as state (pending keys and ranks) if a headless host is to be matched past 64 asks.
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
static PENDING: AtomicUsize = AtomicUsize::new(0);
/// (render/probe.ts:13 MAX_QUERIES)
pub const MAX_QUERIES: usize = 64;
/// what the host's table held as the tick opened
pub fn host_probe_pending_set(n: usize) { PENDING.store(n.min(MAX_QUERIES), Relaxed); }
/// one ask into the table (a full table swaps asks about by rank and age: its length stays)
pub fn host_probe_put() { let n = PENDING.load(Relaxed); if n < MAX_QUERIES { PENDING.store(n + 1, Relaxed); } }
/// Probe.room
pub fn host_probe_room() -> f64 { (MAX_QUERIES - PENDING.load(Relaxed)) as f64 }
