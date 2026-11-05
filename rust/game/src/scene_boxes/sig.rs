#![allow(unused_variables)]
//! Every function and method of `scene_boxes`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/scene/boxes.ts:179 lookKeyed)
pub fn look_keyed(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/scene/boxes.ts:185 boxOf)
pub fn box_of(c_2: V3, h: V3, yaw: f64, albedo: V3, emissive: V3, flags: f64, owner: f64, rot: Option<Quat>, name: Option<&str>) -> BoxId { todo!() }

/// (src/scene/boxes.ts:193 trimDrawn)
pub fn trim_drawn(g: &mut crate::game::Game, all: &[BoxId], t: BoxId) -> bool { todo!() }

/// (src/scene/boxes.ts:195 boxDrawn)
pub fn box_drawn(g: &mut crate::game::Game, all: &[BoxId], b: BoxId) -> bool { todo!() }

/// (src/scene/boxes.ts:196 makeBox)
pub fn make_box(g: &mut crate::game::Game, p: &crate::scene_boxes::MakeBoxP) -> BoxId { todo!() }

/// (src/scene/boxes.ts:203 copyBox)
pub fn copy_box(g: &mut crate::game::Game, b: BoxId) -> BoxId { todo!() }

/// (src/scene/boxes.ts:212 wordOf)
pub fn word_of(b: crate::scene_boxes::WordOfB) -> crate::scene_boxes::WordOf { todo!() }

/// (src/scene/boxes.ts:219 solidGlassBox)
pub fn solid_glass_box(b: crate::scene_boxes::SolidGlassBoxB) -> bool { todo!() }

/// (src/scene/boxes.ts:331 BoxWorld.setAirGrid)
pub fn box_world_set_air_grid(g: &mut crate::game::Game, cells: &[f32]) { todo!() }

/// (src/scene/boxes.ts:335 BoxWorld.setGroundField)
pub fn box_world_set_ground_field(g: &mut crate::game::Game, rows: &[f32]) { todo!() }

/// (src/scene/boxes.ts:432 BoxWorld.rayAside)
pub fn box_world_ray_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_boxes::BoxWorldRayAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/scene/boxes.ts:483 BoxWorld.constructor)
pub fn box_world_new(g: &mut crate::game::Game, device: crate::Opaque, max_boxes: Option<f64>) -> crate::scene_boxes::BoxWorld { todo!() }

/// (src/scene/boxes.ts:506 BoxWorld.addStatic)
pub fn box_world_add_static(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/scene/boxes.ts:530 BoxWorld.packList)
pub fn box_world_pack_list(g: &mut crate::game::Game, ent: &mut crate::scene_boxes::BoxWorldPackListEnt) { todo!() }

/// (src/scene/boxes.ts:543 BoxWorld.repackMarkings)
pub fn box_world_repack_markings(g: &mut crate::game::Game, texel: Option<f64>) { todo!() }

/// (src/scene/boxes.ts:563 BoxWorld.resetCapsules)
pub fn box_world_reset_capsules(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxes.ts:565 BoxWorld.addCharacterCapsules)
pub fn box_world_add_character_capsules(g: &mut crate::game::Game, owner: f64, albedo: &crate::scene_boxes::BoxWorldAddCharacterCapsulesAlbedo, caps: &[Vec<f64>], bound: [f64; 4]) { todo!() }

/// (src/scene/boxes.ts:609 BoxWorld.footprint)
pub fn box_world_footprint(g: &mut crate::game::Game, b: BoxId) -> [f64; 2] { todo!() }

/// (src/scene/boxes.ts:691 BoxWorld.list)
pub fn box_world_list(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/boxes.ts:816 BoxWorld.relist)
pub fn box_world_relist(g: &mut crate::game::Game) { todo!() }

/// (src/scene/boxes.ts:1066 BoxWorld.raycast)
pub fn box_world_raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: Option<f64>, skip_flags: Option<f64>, skip_owner: Option<f64>, with_look: Option<bool>) -> Option<crate::scene_boxes::BoxWorldRaycast> { todo!() }

/// (src/scene/boxes.ts:1080 BoxWorld.rayTest)
pub fn box_world_ray_test(g: &mut crate::game::Game, i: f64, ro: V3, rd: V3, skip_flags: f64, skip_owner: f64) { todo!() }

/// (src/scene/boxes.ts:1090 BoxWorld.rayWalk)
pub fn box_world_ray_walk(g: &mut crate::game::Game, ro: V3, rd: V3, skip_flags: f64, skip_owner: f64) { todo!() }

/// (src/scene/boxes.ts:1116 BoxWorld.buildRayGrid)
pub fn box_world_build_ray_grid(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/scene/boxes.ts:1140 shapeCut)
pub fn shape_cut(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/scene/boxes.ts:1143 cutMet)
pub fn cut_met(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/scene/boxes.ts:1144 rayBox)
pub fn ray_box(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBox> { todo!() }

/// (src/scene/boxes.ts:1170 rayBoxRot)
pub fn ray_box_rot(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxRot> { todo!() }

/// (src/scene/boxes.ts:1191 rayBoxCut)
pub fn ray_box_cut(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxCut> { todo!() }

/// (src/scene/boxes.ts:1221 rayBoxRound)
pub fn ray_box_round(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxRound> { todo!() }


/// INVENTED: a box made to stay (a door's leaf, a prop, debris): a row of g.tab.box for good
pub fn box_new(g: &mut crate::game::Game, b: crate::scene_boxes::Box) -> BoxId { todo!() }
/// INVENTED: a box made for this frame's list (Game.dynBoxes is rebuilt each tick, game.ts:1926: a device's, a beam's, what hangs on a man, a thing in the air): its row is reused after boxes_frame_reset. Never keep its ID past that
pub fn box_frame(g: &mut crate::game::Game, b: crate::scene_boxes::Box) -> BoxId { todo!() }
/// INVENTED: the frame's boxes may be reused: called as the dyn boxes are rebuilt
pub fn boxes_frame_reset(g: &mut crate::game::Game) { todo!() }

/// INVENTED: makeBox's object as a value, in no table: boxOf's defaults, then every field given. For a box made only to be read and dropped; make_box is this and a push. (boxes.ts:182)
pub fn make_box_value(p: &crate::scene_boxes::MakeBoxP) -> crate::scene_boxes::Box { todo!() }
