//! THE BRING-UP'S OWN PART of host_api: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:75 HostApi.invalidateWorldCache)
pub fn host_api_invalidate_world_cache(g: &mut crate::game::Game) {}


/// HARNESS STAND-IN FOR A BRIDGE GAP (tools/blitz/README.md "Host steps"): called by the tick loop's runner between two ticks, never by the game.
/// The headless host's `engine.world.upload()` between two ticks, as much of it as the game reads: the boxes listed again (boxes.ts relist; the pack is the picture's).
pub fn host_world_upload(g: &mut crate::game::Game) {
    crate::scene_boxes::box_world_list(g);
    // (the engine's list now names this frame's boxes and no older frame's: the rows of the frames before are free again)
    crate::scene_boxes::boxes_frame_reset(g);
}
