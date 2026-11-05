#![allow(unused_variables)]
//! Every function and method of `render_char_materials`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/charMaterials.ts:37 slotOfMaterialName)
pub fn slot_of_material_name(name: Option<&str>) -> f64 { todo!() }

/// (src/render/charMaterials.ts:64 proxyOfJoint)
pub fn proxy_of_joint(name: &str) -> f64 { todo!() }

/// (src/render/charMaterials.ts:81 standInSlots)
pub fn stand_in_slots(g: &mut crate::game::Game, ch: RigId, dom: &[i32], joint_name: &[String]) -> Vec<u8> { todo!() }

/// (src/render/charMaterials.ts:111 bakeCavity)
pub fn bake_cavity(g: &mut crate::game::Game, m: AnimGltfMeshDataId, per_part: Option<bool>) -> Vec<f32> { todo!() }

/// (src/render/charMaterials.ts:148 castPart)
pub fn cast_part(g: &mut crate::game::Game, m: AnimGltfMeshDataId) -> Option<Vec<u8>> { todo!() }

/// (src/render/charMaterials.ts:151 meshAuthored)
pub fn mesh_authored(g: &mut crate::game::Game, m: AnimGltfMeshDataId) -> bool { todo!() }

/// (src/render/charMaterials.ts:162 charSlots)
pub fn char_slots(g: &mut crate::game::Game, ch: RigId) -> crate::render_char_materials::CharSlotData { todo!() }

/// (src/render/charMaterials.ts:407 charMatsOf)
pub fn char_mats_of(g: &mut crate::game::Game, side: crate::anim_gltf::CastSide, suit: Option<bool>) -> crate::render_char_materials::SideTable { todo!() }

/// (src/render/charMaterials.ts:411 resolveSlot)
pub fn resolve_slot(t: &crate::render_char_materials::SideTable, slot: crate::render_char_materials::CharSlot, stand_in: Option<bool>) -> crate::render_char_materials::ResolveSlot { todo!() }

/// (src/render/charMaterials.ts:426 placeholderTints)
pub fn placeholder_tints(g: &mut crate::game::Game, side: crate::anim_gltf::CastSide) -> f64 { todo!() }

/// (src/render/charMaterials.ts:440 charMatsVersion)
pub fn char_mats_version(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/render/charMaterials.ts:443 die)
pub fn die(id_2: f64, salt: f64) -> f64 { todo!() }

/// (src/render/charMaterials.ts:447 pickIndex)
pub fn pick_index(id_2: f64, salt: f64, n: f64) -> f64 { todo!() }

/// (src/render/charMaterials.ts:448 lobeRough)
pub fn lobe_rough(r: f64) -> f64 { todo!() }

/// (src/render/charMaterials.ts:451 lobeGain)
pub fn lobe_gain(m: &crate::render_char_materials::SlotMat) -> f64 { todo!() }

/// (src/render/charMaterials.ts:454 slotAlbedo)
pub fn slot_albedo(g: &mut crate::game::Game, side: crate::anim_gltf::CastSide, slot: crate::render_char_materials::CharSlot, tint: V3, tint2: V3, id_2: f64, stand_in: Option<bool>, suit: Option<bool>) -> V3 { todo!() }

/// (src/render/charMaterials.ts:472 writeCharPalette)
pub fn write_char_palette(g: &mut crate::game::Game, out: &mut Vec<f32>, o: f64, side: crate::anim_gltf::CastSide, tint: V3, tint2: V3, id_2: f64, stand_in: Option<bool>, suit: Option<bool>) { todo!() }

/// (src/render/charMaterials.ts:488 proxyAlbedos)
pub fn proxy_albedos(sd: &crate::render_char_materials::CharSlotData, pal: &[f32], o: f64, wet_k: Option<f64>, whole: Option<bool>, mask: Option<f64>) -> Vec<V3> { todo!() }
