#![allow(unused_variables)]
//! Every function and method of `anim_skeleton`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/anim/skeleton.ts:24 benchFor)
pub fn bench_for(n: f64, owed: Option<bool>) -> Option<crate::sim_sim::SimExports> { todo!() }

/// (src/anim/skeleton.ts:42 layerFits)
pub fn layer_fits(what: &str, n: f64, max: f64, min: Option<f64>) { todo!() }

/// (src/anim/skeleton.ts:45 benchNames)
pub fn bench_names(x: crate::sim_sim::SimExports, table: f64) -> Vec<String> { todo!() }

/// (src/anim/skeleton.ts:51 layerSlot)
pub fn layer_slot(g: &mut crate::game::Game, ch: RigId, pose: &crate::anim_skeleton::Pose, cast: &crate::anim_skeleton::LayerCast) -> f64 { todo!() }

/// (src/anim/skeleton.ts:84 layerRan)
pub fn layer_ran(g: &mut crate::game::Game, code: f64, fk: &mut crate::anim_skeleton::PoseFK) { todo!() }

/// (src/anim/skeleton.ts:100 colsOf)
pub fn cols_of<T: Clone + core::fmt::Debug>(c_2: crate::anim_skeleton::LayerCols, self_: &T) -> crate::anim_skeleton::ColsOf { todo!() }

/// (src/anim/skeleton.ts:108 colsToBench)
pub fn cols_to_bench<T: Clone + core::fmt::Debug>(c_2: crate::anim_skeleton::LayerCols, self_: &T, at: Option<f64>) { todo!() }

/// (src/anim/skeleton.ts:109 colsFromBench)
pub fn cols_from_bench<T: Clone + core::fmt::Debug>(c_2: crate::anim_skeleton::LayerCols, self_: &T, at: Option<f64>) { todo!() }

/// (src/anim/skeleton.ts:111 colsJoin)
pub fn cols_join(a: crate::anim_skeleton::LayerCols, b: crate::anim_skeleton::LayerCols) -> crate::anim_skeleton::LayerCols { todo!() }

/// (src/anim/skeleton.ts:113 benchRig)
pub fn bench_rig(g: &mut crate::game::Game, ch: RigId, owed: Option<bool>) -> Option<crate::sim_sim::SimExports> { todo!() }

/// (src/anim/skeleton.ts:123 benchPut)
pub fn bench_put(p: &crate::anim_skeleton::Pose, slot: f64) { todo!() }

/// (src/anim/skeleton.ts:126 benchGet)
pub fn bench_get(p: &mut crate::anim_skeleton::Pose, slot: f64) { todo!() }

/// (src/anim/skeleton.ts:127 benchPutFK)
pub fn bench_put_fk(fk: &crate::anim_skeleton::PoseFK) { todo!() }

/// (src/anim/skeleton.ts:128 benchGetFK)
pub fn bench_get_fk(fk: &mut crate::anim_skeleton::PoseFK) { todo!() }

/// (src/anim/skeleton.ts:135 poseRows)
pub fn pose_rows(n: f64) -> f64 { todo!() }

/// (src/anim/skeleton.ts:148 Pose.constructor)
pub fn pose_new(num_nodes: f64) -> crate::anim_skeleton::Pose { todo!() }

/// (src/anim/skeleton.ts:149 Pose.cols)
pub fn pose_cols(this: &mut crate::anim_skeleton::Pose) -> Vec<f32> { todo!() }

/// (src/anim/skeleton.ts:151 Pose.copy)
pub fn pose_copy(this: &mut crate::anim_skeleton::Pose, src: &crate::anim_skeleton::Pose) -> crate::anim_skeleton::Pose { todo!() }

/// (src/anim/skeleton.ts:152 Pose.rot)
pub fn pose_rot(this: &mut crate::anim_skeleton::Pose, i: f64) -> Quat { todo!() }

/// (src/anim/skeleton.ts:153 Pose.setRot)
pub fn pose_set_rot(this: &mut crate::anim_skeleton::Pose, i: f64, q: Quat) { todo!() }

/// (src/anim/skeleton.ts:154 Pose.pos)
pub fn pose_pos(this: &mut crate::anim_skeleton::Pose, i: f64) -> V3 { todo!() }

/// (src/anim/skeleton.ts:199 stageOpen)
pub fn stage_open(g: &mut crate::game::Game, s: crate::Opaque) -> bool { todo!() }

/// (src/anim/skeleton.ts:211 stageShut)
pub fn stage_shut(g: &mut crate::game::Game, s: crate::Opaque) { todo!() }

/// (src/anim/skeleton.ts:216 slotOf)
pub fn slot_of(p: &crate::anim_skeleton::Pose) -> f64 { todo!() }

/// (src/anim/skeleton.ts:218 lend)
pub fn lend(p: &crate::anim_skeleton::Pose, place: f64) -> f64 { todo!() }

/// (src/anim/skeleton.ts:220 known)
pub fn known(ok: f64) { todo!() }

/// (src/anim/skeleton.ts:222 restPose)
pub fn rest_pose(ch: &crate::anim_gltf::GltfCharacter, out: &mut crate::anim_skeleton::Pose) -> crate::anim_skeleton::Pose { todo!() }

/// (src/anim/skeleton.ts:239 sideOf)
pub fn side_of<'g>(g: &'g mut crate::game::Game, clip: ClipId) -> &'g mut crate::anim_skeleton::ClipSide { todo!() }

/// (src/anim/skeleton.ts:249 clipIn)
pub fn clip_in(g: &mut crate::game::Game, x: crate::sim_sim::SimExports, clip: ClipId) -> crate::anim_skeleton::ClipSide { todo!() }

/// (src/anim/skeleton.ts:264 clipNo)
pub fn clip_no(g: &mut crate::game::Game, clip: ClipId, nodes: f64) -> f64 { todo!() }

/// (src/anim/skeleton.ts:274 layerClipsFit)
pub fn layer_clips_fit(g: &mut crate::game::Game, clips: &[ClipId], nodes: f64) { todo!() }

/// (src/anim/skeleton.ts:302 sampleClip)
pub fn sample_clip(clip: &crate::anim_gltf::Clip, t: f64, loop_: bool, out: &mut crate::anim_skeleton::Pose) { todo!() }

/// (src/anim/skeleton.ts:355 blendPose)
pub fn blend_pose(a: Option<&crate::anim_skeleton::Pose>, b: &crate::anim_skeleton::Pose, w: f64, mask: Option<&[f32]>, out: &mut crate::anim_skeleton::Pose, singles: bool) { todo!() }

/// (src/anim/skeleton.ts:411 SkeletonInstance.constructor)
pub fn skeleton_instance_new(g: &mut crate::game::Game, ch: RigId) -> crate::anim_skeleton::SkeletonInstance { todo!() }

/// (src/anim/skeleton.ts:420 SkeletonInstance.update)
pub fn skeleton_instance_update(this: &mut crate::anim_skeleton::SkeletonInstance, pose: &crate::anim_skeleton::Pose, world: &crate::v3::m4::Mat4, twists: Option<&[crate::anim_skeleton::TwistSpec]>, overrides: Option<&[Option<crate::anim_skeleton::NodeOverride>]>, rig: &crate::anim_gltf::GltfCharacter) { todo!() }

/// (src/anim/skeleton.ts:465 SkeletonInstance.nodePos)
pub fn skeleton_instance_node_pos(this: &mut crate::anim_skeleton::SkeletonInstance, node: f64) -> V3 { todo!() }

/// (src/anim/skeleton.ts:469 SkeletonInstance.bakeLocalRotations)
pub fn skeleton_instance_bake_local_rotations(this: &mut crate::anim_skeleton::SkeletonInstance, pose: &mut crate::anim_skeleton::Pose, rig: &crate::anim_gltf::GltfCharacter) -> crate::anim_skeleton::Pose { todo!() }

/// (src/anim/skeleton.ts:480 buildMask)
pub fn build_mask(g: &mut crate::game::Game, ch: RigId, entries: &[crate::anim_skeleton::BuildMaskEntriesItem]) -> Vec<f32> { todo!() }

/// (src/anim/skeleton.ts:496 PoseFK.constructor)
pub fn pose_fk_new(g: &mut crate::game::Game, ch: RigId) -> crate::anim_skeleton::PoseFK { todo!() }

/// (src/anim/skeleton.ts:501 PoseFK.run)
pub fn pose_fk_run(this: &mut crate::anim_skeleton::PoseFK, pose: &crate::anim_skeleton::Pose, rig: &crate::anim_gltf::GltfCharacter) -> crate::anim_skeleton::PoseFK { todo!() }
