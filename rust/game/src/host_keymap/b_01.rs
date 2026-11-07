//! THE BRING-UP'S OWN PART of host_keymap: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY: the host's keymap names the camera's key here at the swap ('Space' while the page's camSpace holds and Space is no row's). (src/host/keymap.ts:266 camCap)
pub fn cam_cap(g: &mut crate::game::Game) -> String { String::from("Ctrl") }
