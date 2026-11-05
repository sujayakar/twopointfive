#![allow(unused_variables)]
//! Every function and method of `anim_characters`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/anim/characters.ts:33 CharacterRenderer.constructor)
pub fn character_renderer_new(g: &mut crate::game::Game, device: crate::Opaque, ch: RigId) -> crate::anim_characters::CharacterRenderer { todo!() }

/// (src/anim/characters.ts:78 CharacterRenderer.setInstance)
pub fn character_renderer_set_instance(g: &mut crate::game::Game, i: f64, joint_data: &[f32], tint: V3, tint2: V3, owner: f64, wet: Option<&mut dyn FnMut(&mut crate::game::Game, Vec<f32>, f64)>, parts: Option<f64>) { todo!() }

/// (src/anim/characters.ts:88 CharacterRenderer.setMaterials)
pub fn character_renderer_set_materials(g: &mut crate::game::Game, i: f64, write: Option<&mut dyn FnMut(&mut crate::game::Game, Vec<f32>, f64)>) { todo!() }

/// (src/anim/characters.ts:108 CharacterRenderer.finish)
pub fn character_renderer_finish(g: &mut crate::game::Game, count: f64) { todo!() }

/// (src/anim/characters.ts:119 partSpansOf)
pub fn part_spans_of(indices: &[u32], part: Option<&[u8]>) -> Option<Vec<u32>> { todo!() }
