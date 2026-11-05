#![allow(unused_variables)]
//! Every function and method of `render_lights`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/lights.ts:96 lightFromDef)
pub fn light_from_def(g: &mut crate::game::Game, d: &crate::scene_level::LightDef) -> LightId { todo!() }

/// (src/render/lights.ts:108 packLight)
pub fn pack_light(g: &mut crate::game::Game, f: &mut Vec<f32>, u: &mut Vec<u32>, o: f64, l: LightId, inten: f64, kind_word: f64, volumetric: f64, owner_word: f64) { todo!() }

/// (src/render/lights.ts:159 LightSet.constructor)
pub fn light_set_new(g: &mut crate::game::Game, device: crate::Opaque) -> crate::render_lights::LightSet { todo!() }

/// (src/render/lights.ts:166 LightSet.add)
pub fn light_set_add(g: &mut crate::game::Game, l: LightId) -> LightId { todo!() }

/// (src/render/lights.ts:167 LightSet.remove)
pub fn light_set_remove(g: &mut crate::game::Game, l: LightId) { todo!() }

/// (src/render/lights.ts:170 LightSet.flash)
pub fn light_set_flash(g: &mut crate::game::Game, pos: V3, color: V3, intensity: f64, range: f64, ttl: f64, owner: f64) -> LightId { todo!() }

/// (src/render/lights.ts:196 LightSet.fogClass)
pub fn light_set_fog_class(g: &mut crate::game::Game, l: LightId) -> f64 { todo!() }

/// INVENTED, for LightSet.update (render/lights.ts:174): a flash's envelope and its retirement, off the list; the host calls it between ticks
pub fn light_set_update(g: &mut crate::game::Game, dt: f64, time: f64) { todo!() }

