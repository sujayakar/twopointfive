//! THE BRING-UP'S OWN PART of look_decals: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY: tags a picture-only decal for the digest's count; the decal comes back as it went.   (lookDecals.ts lookDecal)
pub fn look_decal(g: &crate::game::Game, d: Option<&crate::render_decals::Decal>) -> Option<crate::render_decals::Decal> { d.cloned() }
