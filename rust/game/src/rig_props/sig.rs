#![allow(unused_variables)]
//! Every function and method of `rig_props`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rigProps.ts:268 assignTree)
pub fn assign_tree(dst: &mut JsMap<String, crate::rig_props::AssignTreeDstValue>, src: &JsMap<String, crate::rig_props::AssignTreeSrcValue>) { todo!() }

/// (src/game/rigProps.ts:276 cloneRig)
pub fn clone_rig(r: crate::rig_props::Rig) -> crate::rig_props::Rig { todo!() }

/// (src/game/rigProps.ts:278 assignRig)
pub fn assign_rig(dst: &mut crate::rig_props::Rig, src: crate::rig_props::Rig) { todo!() }

/// (src/game/rigProps.ts:280 clearAdjs)
pub fn clear_adjs(rig: crate::rig_props::Rig) { todo!() }

/// (src/game/rigProps.ts:286 rigSource)
pub fn rig_source(rig: Option<crate::rig_props::Rig>) -> String { todo!() }

/// (src/game/rigProps.ts:305 frameFrom)
pub fn frame_from(o: V3, along: V3, up_hint: Option<V3>) -> crate::rig_props::Frame { todo!() }

/// (src/game/rigProps.ts:311 rotAxis)
pub fn rot_axis(v: V3, axis: V3, ang: f64) -> V3 { todo!() }

/// (src/game/rigProps.ts:313 applyAdj)
pub fn apply_adj(f: crate::rig_props::Frame, adj: Option<crate::rig_props::Adj>) -> crate::rig_props::Frame { todo!() }

/// (src/game/rigProps.ts:328 ZERO_ADJ)
pub fn zero_adj() -> crate::rig_props::Adj { todo!() }

/// (src/game/rigProps.ts:330 adjOf)
pub fn adj_of<T: Clone + core::fmt::Debug>(owner: &mut T, key: crate::Opaque) -> crate::FnField { todo!() }

/// (src/game/rigProps.ts:334 wristTieBoxes)
pub fn wrist_tie_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/rigProps.ts:345 seg)
pub fn seg(g: &mut crate::game::Game, out: &mut Vec<BoxId>, g_2: crate::rig_props::Frame, s: &mut crate::rig_props::Seg, albedo: V3, emissive: Option<V3>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, path: Option<&str>, label: Option<&str>, depth: Option<f64>, name: Option<&str>) { todo!() }

/// (src/game/rigProps.ts:354 segAt)
pub fn seg_at(g: &mut crate::game::Game, out: &mut Vec<BoxId>, g_2: crate::rig_props::Frame, s: &mut crate::rig_props::Seg, adj: Option<crate::rig_props::Adj>, albedo: V3, emissive: V3, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, path: Option<&str>, label: Option<&str>, depth: Option<f64>, name: Option<&str>) { todo!() }

/// (src/game/rigProps.ts:367 headFlatForward)
pub fn head_flat_forward(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/rigProps.ts:379 gogglesBoxes)
pub fn goggles_boxes(g: &mut crate::game::Game, c_2: CharId, nv: bool, out: &mut Vec<BoxId>, g_2: &mut crate::rig_props::RigGoggles, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, k: Option<f64>) { todo!() }

/// (src/game/rigProps.ts:442 fitFrame)
pub fn fit_frame(g_2: crate::rig_props::Frame, d: V3) -> crate::rig_props::Frame { todo!() }

/// (src/game/rigProps.ts:445 pistolFrame)
pub fn pistol_frame(g: &mut crate::game::Game, c_2: CharId, hand: V3, p: crate::rig_props::PistolFrameP, in_hand: Option<bool>) -> crate::rig_props::PistolFrame { todo!() }

/// (src/game/rigProps.ts:522 mixFrame)
pub fn mix_frame(a: crate::rig_props::Frame, b: crate::rig_props::Frame, k: f64) -> crate::rig_props::Frame { todo!() }

/// (src/game/rigProps.ts:528 mixAdj)
pub fn mix_adj(a: Option<crate::rig_props::Adj>, b: Option<crate::rig_props::Adj>, k: f64) -> Option<crate::rig_props::Adj> { todo!() }

/// (src/game/rigProps.ts:536 holsterFrameOf)
pub fn holster_frame_of(th: V3, kn: V3, hp: V3, fwd: V3, h: Option<crate::rig_props::RigHolster>) -> crate::rig_props::Frame { todo!() }

/// (src/game/rigProps.ts:543 kitGunFrame)
pub fn kit_gun_frame(g: &mut crate::game::Game, c_2: CharId, hand: V3, p: crate::rig_props::KitGunFrameP, g_2: crate::rig_props::Frame, roll: f64) -> crate::rig_props::Frame { todo!() }

/// (src/game/rigProps.ts:552 magBoxL)
pub fn mag_box_l(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, kind: crate::slotsetup::SetupGun, fit: Option<bool>, own: Option<V3>, col: Option<V3>) { todo!() }

/// (src/game/rigProps.ts:565 rigBoxes)
pub fn rig_boxes(g: &mut crate::game::Game, c_2: CharId, o: crate::rig_props::RigOpts, out: &mut Vec<BoxId>, rig: Option<crate::rig_props::Rig>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) { todo!() }

/// (src/game/rigProps.ts:569 kitBoxes)
pub fn kit_boxes(g: &mut crate::game::Game, c_2: CharId, o: crate::rig_props::RigOpts, out: &mut Vec<BoxId>, rig: crate::rig_props::Rig, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) { todo!() }

/// (src/game/rigProps.ts:640 pistolAs)
pub fn pistol_as(p: crate::rig_props::RigPistol, m: crate::gunlook::PistolModel) -> crate::rig_props::RigPistol { todo!() }

/// (src/game/rigProps.ts:641 modelCol)
pub fn model_col(c_2: crate::gunlook::ModelCol, black: bool) -> V3 { todo!() }

/// (src/game/rigProps.ts:647 longModelBoxes)
pub fn long_model_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>, l: &crate::gunlook::LongModel, g_2: crate::rig_props::Frame, lens: V3, black: bool, no_mag: bool, cover: f64) { todo!() }

/// (src/game/rigProps.ts:658 longLookOf)
pub fn long_look_of(g: &mut crate::game::Game, c_2: CharId, held: bool) -> Option<crate::rig_props::LongLookOf> { todo!() }

/// (src/game/rigProps.ts:664 pistolLookOf)
pub fn pistol_look_of(g: &mut crate::game::Game, c_2: CharId, held: bool) -> Option<crate::gunlook::PistolModel> { todo!() }

/// (src/game/rigProps.ts:666 modelInHands)
pub fn model_in_hands(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::GunModelId, l: &crate::gunlook::LongModel, out: &mut Vec<BoxId>, g_2: crate::rig_props::Frame, lens: V3, black: bool, no_mag: bool) { todo!() }

/// (src/game/rigProps.ts:673 modelGrip)
pub fn model_grip(g: &mut crate::game::Game, id_2: crate::weapons::GunModelId, fit: Option<bool>, rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::ModelGrip> { todo!() }

/// (src/game/rigProps.ts:680 modelMuzzleAhead)
pub fn model_muzzle_ahead(g: &mut crate::game::Game, id_2: crate::weapons::GunModelId, rig: Option<crate::rig_props::Rig>) -> Option<f64> { todo!() }

/// (src/game/rigProps.ts:684 holsterBoxes)
pub fn holster_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, parts: crate::rig_props::HolsterBoxesParts, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, holster_rig: Option<crate::rig_props::RigHolster>) { todo!() }

/// (src/game/rigProps.ts:707 knifeBoxes)
pub fn knife_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, k: Option<&mut crate::rig_props::RigKnife>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) { todo!() }

/// (src/game/rigProps.ts:722 sheathFrame)
pub fn sheath_frame(g: &mut crate::game::Game, c_2: CharId, s: Option<crate::rig_props::RigSheath>) -> Option<crate::rig_props::Frame> { todo!() }

/// (src/game/rigProps.ts:733 sheathBoxes)
pub fn sheath_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, full: Option<bool>, rig: Option<crate::rig_props::Rig>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) { todo!() }

/// (src/game/rigProps.ts:746 heldCanBoxes)
pub fn held_can_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::game::ThrowKind, out: &mut Vec<BoxId>, k: Option<&mut crate::rig_props::RigCan>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, fit: Option<bool>, left: Option<bool>) { todo!() }

/// (src/game/rigProps.ts:760 freeCanBoxes)
pub fn free_can_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::game::ThrowKind, out: &mut Vec<BoxId>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) { todo!() }

/// (src/game/rigProps.ts:763 carbineFrame)
pub fn carbine_frame(g: &mut crate::game::Game, c_2: CharId, hand: V3, c_3: crate::rig_props::RigCarbine) -> crate::rig_props::CarbineFrame { todo!() }

/// (src/game/rigProps.ts:766 carbineBoxes)
pub fn carbine_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, c_3: &mut crate::rig_props::RigCarbine, lens: V3, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, at: Option<crate::rig_props::Frame>, as_: Option<crate::weapons::GunModelId>) { todo!() }

/// (src/game/rigProps.ts:786 gripAdj)
pub fn grip_adj(g: &mut crate::game::Game, g_2: crate::rig_props::RigCarbineGripL) -> crate::rig_props::Adj { todo!() }

/// (src/game/rigProps.ts:791 carbineGrip)
pub fn carbine_grip(rig: Option<crate::rig_props::Rig>) -> crate::rig_props::CarbineGrip { todo!() }

/// (src/game/rigProps.ts:793 carbineMuzzleAhead)
pub fn carbine_muzzle_ahead(rig: Option<crate::rig_props::Rig>) -> f64 { todo!() }

/// (src/game/rigProps.ts:795 longGripOf)
pub fn long_grip_of(group_adj: Option<crate::rig_props::Adj>, g_2: crate::rig_props::LongGripOfG) -> crate::rig_props::LongGripOf { todo!() }

/// (src/game/rigProps.ts:801 muzzleAheadOf)
pub fn muzzle_ahead_of(group_adj: Option<crate::rig_props::Adj>, part: crate::rig_props::Seg) -> f64 { todo!() }

/// (src/game/rigProps.ts:820 slingFrontHand)
pub fn sling_front_hand(g: &mut crate::game::Game, s: Option<crate::rig_props::RigSc20k>, rest: Option<f64>) -> crate::rig_props::SlingFrontHand { todo!() }

/// (src/game/rigProps.ts:840 slungBase)
pub fn slung_base(g: &mut crate::game::Game, c_2: CharId, p: crate::rig_props::RigSc20kSlung, front: bool, rest: Option<f64>) -> crate::rig_props::SlungBase { todo!() }

/// (src/game/rigProps.ts:859 slingSwing)
pub fn sling_swing(g: &mut crate::game::Game, c_2: CharId, cf: crate::rig_props::Frame, o: V3, z: V3) -> f64 { todo!() }

/// (src/game/rigProps.ts:873 slingRestWant)
pub fn sling_rest_want(g: &mut crate::game::Game, c_2: CharId) -> f64 { todo!() }

/// (src/game/rigProps.ts:874 sc20kFrame)
pub fn sc20k_frame(g: &mut crate::game::Game, c_2: CharId, s: crate::rig_props::RigSc20k, where_: crate::rig_props::Sc20kFrameWhere, front: Option<bool>, rest: Option<f64>) -> crate::rig_props::Sc20kFrame { todo!() }

/// (src/game/rigProps.ts:885 bareBarrel)
pub fn bare_barrel(can: crate::rig_props::Seg) -> crate::rig_props::Seg { todo!() }

/// (src/game/rigProps.ts:889 sc20kBoxes)
pub fn sc20k_boxes(g: &mut crate::game::Game, c_2: CharId, out: &mut Vec<BoxId>, s: Option<&mut crate::rig_props::RigSc20k>, where_: Option<crate::rig_props::Sc20kBoxesWhere>, lens: Option<V3>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>, m: Option<crate::rig_props::RigM4>, kit: Option<crate::rig_props::KitDraw>, fit: Option<bool>, front: Option<bool>, rest: Option<f64>, black: Option<bool>) { todo!() }

/// (src/game/rigProps.ts:923 rifleM4)
pub fn rifle_m4(g: &mut crate::game::Game, rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::RigM4> { todo!() }

/// (src/game/rigProps.ts:925 sc20kGrip)
pub fn sc20k_grip(rig: Option<crate::rig_props::Rig>, fit: Option<bool>) -> crate::rig_props::Sc20kGrip { todo!() }

/// (src/game/rigProps.ts:933 longHold)
pub fn long_hold(weapon: crate::rig_props::LongHoldWeapon, rig: Option<crate::rig_props::Rig>) -> crate::inhand::InHandWord { todo!() }

/// (src/game/rigProps.ts:938 torchHold)
pub fn torch_hold(beam: V3, rig: Option<crate::rig_props::Rig>) -> crate::inhand::InHandWord { todo!() }

/// (src/game/rigProps.ts:943 sc20kMuzzleAhead)
pub fn sc20k_muzzle_ahead(rig: Option<crate::rig_props::Rig>) -> f64 { todo!() }

/// (src/game/rigProps.ts:945 sc20kLightPose)
pub fn sc20k_light_pose(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::Sc20kLightPose { todo!() }

/// (src/game/rigProps.ts:951 sc20kLauncherMuzzle)
pub fn sc20k_launcher_muzzle(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> V3 { todo!() }

/// (src/game/rigProps.ts:956 carbineLightPose)
pub fn carbine_light_pose(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::CarbineLightPose { todo!() }

/// (src/game/rigProps.ts:964 rigLightPose)
pub fn rig_light_pose(g: &mut crate::game::Game, c_2: CharId, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::RigLightPose { todo!() }

/// (src/game/rigProps.ts:971 rigTorchPos)
pub fn rig_torch_pos(g: &mut crate::game::Game, c_2: CharId, dir: V3, torch: bool, rig: Option<crate::rig_props::Rig>, weapon: Option<crate::rig_props::RigTorchPosWeapon>) -> V3 { todo!() }

/// (src/game/rigProps.ts:980 railLightDef)
pub fn rail_light_def(owner: f64, intensity: f64) -> LightId { todo!() }

/// (src/game/rigProps.ts:984 torchLightDef)
pub fn torch_light_def(name: &str, owner: f64, intensity: f64, inner_deg: f64, outer_deg: f64) -> LightId { todo!() }

/// (src/game/rigProps.ts:992 torchCarryDir)
pub fn torch_carry_dir(g: &mut crate::game::Game, c_2: CharId, prev: V3, dt: Option<f64>, low: Option<crate::rig_props::TorchCarryDirLow>) -> V3 { todo!() }

/// (src/game/rigProps.ts:997 torchCarryWant)
pub fn torch_carry_want(g: &mut crate::game::Game, c_2: CharId, tan: Option<f64>) -> V3 { todo!() }

/// (src/game/rigProps.ts:1034 bigClearOf)
pub fn big_clear_of(g: &mut crate::game::Game, c_2: CharId, kind: crate::rig_props::BigClearOfKind, rig: Option<crate::rig_props::Rig>) -> f64 { todo!() }

/// (src/game/rigProps.ts:1041 packSideDims)
pub fn pack_side_dims(g: &mut crate::game::Game, what: crate::rig_props::PackSideDimsWhat, rig: Option<crate::rig_props::Rig>) -> crate::rig_props::PackSideDims { todo!() }

/// (src/game/rigProps.ts:1049 packSideBind)
pub fn pack_side_bind(g: &mut crate::game::Game, c_2: CharId, what: crate::rig_props::PackSideBindWhat, rig: Option<crate::rig_props::Rig>) -> Option<crate::packside::SidePlace> { todo!() }

/// (src/game/rigProps.ts:1054 packSide)
pub fn pack_side(g: &mut crate::game::Game, c_2: CharId, what: crate::rig_props::PackSideWhat, rig: Option<crate::rig_props::Rig>) -> Option<crate::rig_props::Frame> { todo!() }

/// (src/game/rigProps.ts:1063 bigBackOf)
pub fn big_back_of(hips: V3, mid: V3, th_l: V3, th_r: V3, ua_l: V3, ua_r: V3, p: crate::rig_props::RigSc20kSlung, clear: Option<f64>) -> crate::rig_props::BigBackOf { todo!() }

/// (src/game/rigProps.ts:1074 bigBoxes)
pub fn big_boxes(g: &mut crate::game::Game, c_2: CharId, kind: crate::rig_props::BigBoxesKind, k: f64, out: &mut Vec<BoxId>, rig: Option<crate::rig_props::Rig>, clear: Option<bool>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) { todo!() }

/// (src/game/rigProps.ts:1094 hungBoxes)
pub fn hung_boxes(g: &mut crate::game::Game, c_2: CharId, weapon: crate::rig_props::HungBoxesWeapon, out: &mut Vec<BoxId>, rig: Option<crate::rig_props::Rig>, black: Option<bool>, meta: Option<&mut Vec<crate::rig_props::RigNodeMeta>>) { todo!() }
