//! THE BRING-UP'S OWN PART of sensors: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// (Round one counted the probe's asks here for `Probe.room`. Since skeleton 5.1 p_sensors_01 keeps the table itself, `g.probe.pending`, and `probe_like_room` reads it:
// nothing is left in this part. The table is not carried by the bridge; a game compared from a mission's first tick starts with it empty on both sides.)
