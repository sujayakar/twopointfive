//! Bodies for skeleton stubs of scene_markings that translated code calls and no part had (the stub round).
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// The public helpers of this module (texelOf, faceIndex, faceHalf, resolveAlbedo, faceAreas) are still the skeleton's empty stubs and panic, so the bodies below
// use these private copies. MarkRes's Rust order is not MARK_RES's: its index is written out here (markings.ts:32).

