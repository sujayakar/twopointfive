//! THE BRING-UP'S OWN PART of anim_characters: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/anim/characters.ts:78 CharacterRenderer.setInstance)
pub fn character_renderer_set_instance(g: &mut crate::game::Game, i: f64, joint_data: &[f32], tint: V3, tint2: V3, owner: f64, wet: Option<&mut dyn FnMut(&mut crate::game::Game, Vec<f32>, f64)>, parts: Option<f64>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/anim/characters.ts:88 CharacterRenderer.setMaterials)
pub fn character_renderer_set_materials(g: &mut crate::game::Game, i: f64, write: Option<&mut dyn FnMut(&mut crate::game::Game, Vec<f32>, f64)>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/anim/characters.ts:108 CharacterRenderer.finish)
pub fn character_renderer_finish(g: &mut crate::game::Game, count: f64) {}
