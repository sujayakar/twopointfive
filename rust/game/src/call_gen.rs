//! THE CALL SURFACE: every function of the tick that something outside the tick calls, by its row. An arm's number is its row's `id` in tools/blitz/calls.json; a row that leaves the table may take its arm with it, and no other number moves.
#![allow(unused_mut, unused_variables, unused_imports, clippy::all)]
use crate::io::*; use crate::ids::*; use crate::js::*; use crate::{Quat, V3}; use alloc::{string::String, vec::Vec};
/// how the host asks a die a caller handed in (the face sets it; a native runner has none, and the game's own die answers)
pub static mut HOST_ROLL: Option<fn() -> f64> = None;
fn die_of(given: bool) -> impl FnMut(&mut crate::game::Game) -> f64 { move |g| { if given { if let Some(f) = unsafe { HOST_ROLL } { return f(); } } crate::math_dice::sim_random(g) } }
pub fn call(g: &mut crate::game::Game, f: u32, args: &[u8]) -> Vec<u8> { let known = crate::id_tables::id_tables(g); let mut i = R::measured(args, &known); let mut o = W::new(); match f {
0 => { let a1: Vec<crate::scene_cut::CutPlane> = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: f64 = Io::r(&mut i); let a6: f64 = Io::r(&mut i); let a7: f64 = Io::r(&mut i); let a8: f64 = Io::r(&mut i); let a9: f64 = Io::r(&mut i); let r = crate::scene_cut::cut_clip(g, &a1, a2, a3, a4, a5, a6, a7, a8, a9); r.w(&mut o);  }
1 => { let a1: Vec<f64> = Io::r(&mut i); let a2: String = Io::r(&mut i); let a3: crate::scene_stuff::SayPaintSaid = Io::r(&mut i); crate::scene_stuff::say_paint(g, &a1, &a2, a3);  }
2 => { let a1: crate::scene_stuff::BoxStuff = Io::r(&mut i); let a2: [f64; 3] = Io::r(&mut i); let a3: Option<crate::scene_stuff::MadeOfFinish> = Io::r(&mut i); let r = crate::scene_stuff::made_of(g, a1, a2, a3); r.w(&mut o);  }
3 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: AnyBoxId = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let r = crate::scene_boxes::ray_box(g, a1, a2, a3, a4); r.w(&mut o);  }
5 => { let a1: AnimGltfCastInfoId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: bool = Io::r(&mut i); let r = crate::anim_gltf::cast_goggles(g, a1, a2, a3); r.w(&mut o);  }
6 => { let a1: AnimGltfCastInfoId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::anim_gltf::cast_suit(g, a1, a2); r.w(&mut o);  }
7 => {  crate::scene_box_mirror::leaf_wrote(g);  }
8 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: BoxId = Io::r(&mut i); let r = crate::scene_collision::line_box(g, a1, a2, a3); r.w(&mut o);  }
9 => { let a1: f64 = Io::r(&mut i); crate::scene_collision::sound_paths_tick(g, a1);  }
10 => {  crate::scene_collision::sound_paths_invalidate(g);  }
11 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::scene_collision::sound_paths_propagate(g, a1, a2, a3); r.w(&mut o);  }
12 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: crate::sparks::SparksEmitConeO = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let r = crate::sparks::sparks_emit_cone(g, a1, a2, a3, a4); r.w(&mut o);  }
14 => { let a1: f64 = Io::r(&mut i); crate::sparks::sparks_update(g, a1);  }
15 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<crate::effects::SpawnBurstO> = Io::r(&mut i); let r = crate::effects::spawn_burst(g, a1, a2, a3); r.w(&mut o);  }
17 => { let a1: PoseSetId = Io::r(&mut i); let r = crate::anim_skeleton::pose_set_in(g, a1); r.w(&mut o);  }
18 => { let a1: Vec<ClipId> = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::anim_skeleton::layer_clips_fit(g, &a1, a2);  }
20 => { let a1: RagdollId = Io::r(&mut i); crate::ragdoll::ragdoll_wake(g, a1);  }
24 => { let a1: RagdollId = Io::r(&mut i); let a2: Vec<f64> = Io::r(&mut i); let a3: Option<Vec<f64>> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); crate::ragdoll::ragdoll_settle_at(g, a1, &a2, a3.as_deref(), a4);  }
28 => { let a1: FirearmId = Io::r(&mut i); let r = crate::gunreload::feed_begin(g, a1); r.w(&mut o);  }
29 => { let a1: CharId = Io::r(&mut i); let a2: crate::weapons::WeaponDef = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<f64> = Io::r(&mut i); let a6: Option<FirearmId> = Io::r(&mut i); crate::gunreload::feed_tick(g, a1, &a2, a3, a4, a5, a6);  }
30 => { let a1: CharId = Io::r(&mut i); let a2: bool = Io::r(&mut i); let mut a3: Vec<AnyBoxId> = Io::r(&mut i); let a4: crate::rig_props::RigGoggles = Io::r(&mut i); let mut a5: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); let a6: Option<f64> = Io::r(&mut i); crate::rig_props::goggles_boxes(g, a1, a2, &mut a3, a4, a5.as_mut(), a6); a3.w(&mut o); a5.w(&mut o); }
31 => { let a1: CharId = Io::r(&mut i); let a2: crate::rig_props::RigOpts = Io::r(&mut i); let mut a3: Vec<AnyBoxId> = Io::r(&mut i); let a4: Option<crate::rig_props::Rig> = Io::r(&mut i); let mut a5: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); crate::rig_props::rig_boxes(g, a1, a2, &mut a3, a4, a5.as_mut()); a3.w(&mut o); a5.w(&mut o); }
32 => { let a1: CharId = Io::r(&mut i); let mut a2: Vec<AnyBoxId> = Io::r(&mut i); let a3: crate::rig_props::HolsterBoxesParts = Io::r(&mut i); let mut a4: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); let a5: Option<crate::rig_props::RigHolster> = Io::r(&mut i); crate::rig_props::holster_boxes(g, a1, &mut a2, a3, a4.as_mut(), a5); a2.w(&mut o); a4.w(&mut o); }
33 => { let a1: CharId = Io::r(&mut i); let mut a2: Vec<AnyBoxId> = Io::r(&mut i); let a3: Option<crate::rig_props::RigKnife> = Io::r(&mut i); let mut a4: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); crate::rig_props::knife_boxes(g, a1, &mut a2, a3, a4.as_mut()); a2.w(&mut o); a4.w(&mut o); }
34 => { let a1: CharId = Io::r(&mut i); let a2: crate::game::ThrowKind = Io::r(&mut i); let mut a3: Vec<AnyBoxId> = Io::r(&mut i); let mut a4: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); crate::rig_props::free_can_boxes(g, a1, a2, &mut a3, a4.as_mut()); a3.w(&mut o); a4.w(&mut o); }
35 => { let a1: CharId = Io::r(&mut i); let mut a2: Vec<AnyBoxId> = Io::r(&mut i); let a3: Option<crate::rig_props::RigSc20k> = Io::r(&mut i); let a4: Option<crate::rig_props::Sc20kBoxesWhere> = Io::r(&mut i); let a5: Option<V3> = Io::r(&mut i); let mut a6: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); let a7: Option<crate::rig_props::RigM4> = Io::r(&mut i); let a8: Option<crate::rig_props::KitDraw> = Io::r(&mut i); let a9: Option<bool> = Io::r(&mut i); let a10: Option<bool> = Io::r(&mut i); let a11: Option<f64> = Io::r(&mut i); let a12: Option<bool> = Io::r(&mut i); crate::rig_props::sc20k_boxes(g, a1, &mut a2, a3, a4, a5, a6.as_mut(), a7, a8, a9, a10, a11, a12); a2.w(&mut o); a6.w(&mut o); }
36 => { let a1: CharId = Io::r(&mut i); let a2: crate::rig_props::BigBoxesKind = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let mut a4: Vec<AnyBoxId> = Io::r(&mut i); let a5: Option<crate::rig_props::Rig> = Io::r(&mut i); let a6: Option<bool> = Io::r(&mut i); let mut a7: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); crate::rig_props::big_boxes(g, a1, a2, a3, &mut a4, a5, a6, a7.as_mut()); a4.w(&mut o); a7.w(&mut o); }
37 => { let a1: CharId = Io::r(&mut i); let a2: crate::rig_props::HungBoxesWeapon = Io::r(&mut i); let mut a3: Vec<AnyBoxId> = Io::r(&mut i); let a4: Option<crate::rig_props::Rig> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); let mut a6: Option<Vec<crate::rig_props::RigNodeMeta>> = Io::r(&mut i); crate::rig_props::hung_boxes(g, a1, a2, &mut a3, a4, a5, a6.as_mut()); a3.w(&mut o); a6.w(&mut o); }
38 => { let a1: CharId = Io::r(&mut i); crate::character::anim_fire(g, a1);  }
39 => { let a1: CharId = Io::r(&mut i); crate::character::anim_hit(g, a1);  }
40 => { let a1: CharId = Io::r(&mut i); crate::character::anim_flinch(g, a1);  }
43 => { let a1: CharId = Io::r(&mut i); let a2: Option<V3> = Io::r(&mut i); crate::character::anim_hit_from(g, a1, a2);  }
45 => { let a1: CharId = Io::r(&mut i); let a2: Option<crate::character::StrikeKind> = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); crate::character::anim_strike(g, a1, a2, a3);  }
47 => { let a1: CharId = Io::r(&mut i); crate::character::anim_taken_cancel(g, a1);  }
49 => { let a1: CharId = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let a3: Option<crate::character::ReloadBeat> = Io::r(&mut i); crate::character::anim_reload(g, a1, a2, a3);  }
51 => { let a1: CharId = Io::r(&mut i); let a2: bool = Io::r(&mut i); let a3: bool = Io::r(&mut i); crate::character::anim_nv_flip(g, a1, a2, a3);  }
53 => { let a1: CharId = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); crate::character::anim_throw_item(g, a1, a2, a3);  }
54 => { let a1: CharId = Io::r(&mut i); let a2: Option<crate::character::AnimKickDoorOpts> = Io::r(&mut i); crate::character::anim_kick_door(g, a1, a2);  }
55 => { let a1: CharId = Io::r(&mut i); let a2: crate::character::SignalKind = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); crate::character::anim_signal(g, a1, a2, a3);  }
56 => { let a1: CharId = Io::r(&mut i); let a2: crate::character::TellKind = Io::r(&mut i); crate::character::anim_tell(g, a1, a2);  }
60 => { let a1: CharId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::character::character_update(g, a1, a2);  }
62 => { let a1: CharId = Io::r(&mut i); let a2: bool = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: bool = Io::r(&mut i); crate::character::character_ragdollize(g, a1, a2, a3, a4);  }
64 => { let a1: DoorId = Io::r(&mut i); crate::doors::door_reset(g, a1);  }
67 => { let a1: DoorId = Io::r(&mut i); crate::doors::door_place(g, a1);  }
68 => { let a1: f64 = Io::r(&mut i); let mut a2: Vec<crate::doors::DoorContact> = Io::r(&mut i); crate::doors::doors_update(g, a1, &mut a2); a2.w(&mut o); }
69 => { let a1: f64 = Io::r(&mut i); let a2: Vec<crate::doors::DoorContact> = Io::r(&mut i); crate::doors::doors_update_here(g, a1, &a2);  }
70 => { let a1: DoorId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<JsSet<String>> = Io::r(&mut i); let a6: Option<f64> = Io::r(&mut i); let a7: Option<f64> = Io::r(&mut i); crate::doors::doors_crack(g, a1, a2, a3, a4, a5.as_ref(), a6, a7);  }
71 => { let a1: DoorId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<JsSet<String>> = Io::r(&mut i); crate::doors::doors_use(g, a1, a2, a3, a4.as_ref());  }
72 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::doors::doors_pull_to(g, a1, a2);  }
73 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); crate::doors::doors_pull(g, a1, a2, a3);  }
74 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); crate::doors::doors_rattle(g, a1, a2, a3);  }
75 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::doors::doors_pick(g, a1, a2, a3); r.w(&mut o);  }
76 => { let a1: DoorId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let r = crate::doors::doors_kick_in(g, a1, a2, a3, a4); r.w(&mut o);  }
77 => { let a1: PropId = Io::r(&mut i); crate::props::prop_place(g, a1);  }
79 => { let a1: PropId = Io::r(&mut i); crate::props::prop_system_remove(g, a1);  }
80 => {  crate::props::prop_system_reset(g);  }
81 => { let a1: Option<f64> = Io::r(&mut i); crate::props::prop_system_scatter(g, a1);  }
82 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::props::prop_system_raycast(g, a1, a2, a3); r.w(&mut o);  }
84 => { let a1: PropId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); crate::props::prop_system_apply_impulse(g, a1, a2, a3, a4, a5);  }
85 => { let a1: f64 = Io::r(&mut i); let mut a2: Vec<crate::props::PropContact> = Io::r(&mut i); let a3: Vec<BoxId> = Io::r(&mut i); crate::props::prop_system_update(g, a1, &mut a2, &a3); a2.w(&mut o); }
87 => { let a1: bool = Io::r(&mut i); crate::teams::staff_vs_operatives(g, a1);  }
88 => { let a1: GuardId = Io::r(&mut i); let r = crate::opening::opening_spot(g, a1); r.w(&mut o);  }
89 => {  let r = crate::opening::opening_start(g); r.w(&mut o);  }
90 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: bool = Io::r(&mut i); let r = crate::floor::tread(g, a1, a2, a3); r.w(&mut o);  }
91 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::MemberKit = Io::r(&mut i); let r = crate::kitload::load_of(g, a1, &a2); r.w(&mut o);  }
92 => { let a1: f64 = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::kitload::load_ref(g, a1, a2); r.w(&mut o);  }
93 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::MemberKit = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let r = crate::kitload::load_worn(g, a1, &a2, a3); r.w(&mut o);  }
94 => { let a1: V3 = Io::r(&mut i); crate::marks::bang_scorch(g, a1);  }
96 => { let mut a1: crate::cmdring::RingState = Io::r(&mut i); let a2: InputId = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<f64> = Io::r(&mut i); let a6: Option<f64> = Io::r(&mut i); let r = crate::cmdring::ring_want(g, &mut a1, a2, a3, a4, a5, a6); r.w(&mut o); a1.w(&mut o); }
97 => { let a1: GuardId = Io::r(&mut i); crate::workstow::stow_fired(g, a1);  }
98 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); crate::belief::cover(g, a1, a2);  }
99 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); crate::belief::word(g, a1, a2);  }
100 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::launcher::ring_solve(g, a1, a2); r.w(&mut o);  }
101 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: crate::launcher::Ballistics = Io::r(&mut i); let r = crate::launcher::arc_solve(g, a1, a2, a3); r.w(&mut o);  }
102 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::launcher::fire_ring(g, a1, a2, a3); r.w(&mut o);  }
104 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: CharId = Io::r(&mut i); crate::launcher::stagger_guard(g, a1, a2, a3);  }
105 => {  let r = crate::holdfire::hold_fire_lifted(g); r.w(&mut o);  }
106 => { let a1: CharId = Io::r(&mut i); crate::holdfire::hold_fire_shot(g, a1);  }
107 => { let a1: String = Io::r(&mut i); let a2: Option<crate::holdfire::HoldFireOrderO> = Io::r(&mut i); crate::holdfire::hold_fire_order(g, &a1, a2);  }
108 => {  let r = crate::holdfire::hold_fire_loud(g); r.w(&mut o);  }
109 => { let a1: Option<PlayerId> = Io::r(&mut i); crate::holdfire::hold_fire_drill(g, a1);  }
110 => { let a1: GuardId = Io::r(&mut i); let r = crate::holdfire::hold_fire_holds_target(g, a1); r.w(&mut o);  }
111 => { let a1: GuardId = Io::r(&mut i); let r = crate::holdfire::hold_fire_glass(g, a1); r.w(&mut o);  }
112 => {  let r = crate::holdfire::hold_fire_summary(g); r.w(&mut o);  }
113 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); crate::owner::claim_man(g, a1, a2, a3, a4);  }
114 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); crate::owner::release_man(g, a1, a2, a3);  }
115 => { let a1: GuardId = Io::r(&mut i); let a2: crate::picture::Who = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); crate::picture::picture_tell(g, a1, a2, a3, a4);  }
116 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::picture::exposed_at(g, a1, a2, a3); r.w(&mut o);  }
117 => { let a1: GuardId = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); let r = crate::picture::exposure_asker(g, a1, a2); r.w(&mut o);  }
118 => { let a1: Vec<GuardId> = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::picture::exposed_for(g, &a1, a2, a3); r.w(&mut o);  }
119 => { let a1: Vec<GuardId> = Io::r(&mut i); let a2: Vec<Vec<crate::orders::StackPost>> = Io::r(&mut i); let r = crate::picture::safest_sets(g, &a1, &a2); r.w(&mut o);  }
120 => {  let r = crate::picture::picture_views(g); r.w(&mut o);  }
122 => { let a1: crate::shards::GlassFragmentLike = Io::r(&mut i); let r = crate::shards::glass_shards_release(g, &a1); r.w(&mut o);  }
126 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::shards::glass_shards_step(g, a1, a2); r.w(&mut o);  }
127 => { let a1: f64 = Io::r(&mut i); crate::shards::glass_shards_update(g, a1);  }
128 => {  crate::shards::glass_shards_reset(g);  }
130 => { let a1: V3 = Io::r(&mut i); let a2: Option<V3> = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let a4: Option<crate::shards::GlassShardsTestKind> = Io::r(&mut i); let a5: Option<crate::glasspen::PaneType> = Io::r(&mut i); let r = crate::shards::glass_shards_test(g, a1, a2, a3, a4, a5); r.w(&mut o);  }
131 => { let a1: crate::sim_lightshome::FixtureDef = Io::r(&mut i); let r = crate::sim_lightshome::light_home_add(g, &a1); r.w(&mut o);  }
136 => { let a1: Vec<crate::debris::FootMan> = Io::r(&mut i); let a2: crate::debris::FootKnobs = Io::r(&mut i); let a3: f64 = Io::r(&mut i); crate::edge_look::edge_look_feet(g, &a1, a2, a3);  }
137 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); crate::edge_look::edge_look_boxes(g, &mut a1); a1.w(&mut o); }
141 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); crate::wallskin::wall_skin_boxes(g, &mut a1); a1.w(&mut o); }
145 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::wallbreak::wall_break_hung_at(g, a1, a2, a3); r.w(&mut o);  }
146 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: crate::weapons::WeaponDef = Io::r(&mut i); let a6: f64 = Io::r(&mut i); crate::wallbreak::wall_break_round(g, a1, a2, a3, a4, &a5, a6);  }
147 => { let a1: V3 = Io::r(&mut i); let a2: Option<crate::wallbreak::BlastOpts> = Io::r(&mut i); let r = crate::wallbreak::wall_break_would_open(g, a1, a2); r.w(&mut o);  }
160 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::doorblast::door_blast(g, a1, a2, a3); r.w(&mut o);  }
161 => { let a1: DoorId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); let r = crate::doorblast::tear_off(g, a1, a2, a3, a4); r.w(&mut o);  }
162 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: V3 = Io::r(&mut i); let a5: f64 = Io::r(&mut i); crate::doorblast::leaf_round(g, a1, a2, a3, a4, a5);  }
163 => { let a1: DoorId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<V3> = Io::r(&mut i); let r = crate::doorblast::splinter(g, a1, a2, a3, a4); r.w(&mut o);  }
164 => { let a1: f64 = Io::r(&mut i); crate::doorblast::door_slabs(g, a1);  }
165 => { let a1: DoorId = Io::r(&mut i); crate::doorblast::pose_slab(g, a1);  }
166 => { let a1: f64 = Io::r(&mut i); crate::fire::update_fire(g, a1);  }
167 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: V3 = Io::r(&mut i); let r = crate::winbars::bar_meet(g, a1, a2, a3, a4); r.w(&mut o);  }
168 => { let a1: Vec<crate::winbars::BarMet> = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: bool = Io::r(&mut i); let r = crate::winbars::bars_settle(g, &a1, a2, a3, a4); r.w(&mut o);  }
169 => { let a1: crate::wallbreak::BlastEvent = Io::r(&mut i); crate::winbars::bars_blast(g, a1);  }
170 => {  crate::winbars::reset_bars(g);  }
172 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); crate::destruction::destruction_boxes(g, &mut a1); a1.w(&mut o); }
173 => {  crate::destruction::destruction_reset(g);  }
175 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: crate::destruction::DestructionRoundHit = Io::r(&mut i); let a5: crate::weapons::WeaponDef = Io::r(&mut i); crate::destruction::destruction_round(g, a1, a2, a3, &a4, &a5);  }
176 => { let a1: V3 = Io::r(&mut i); let a2: Option<crate::wallbreak::BlastOpts> = Io::r(&mut i); let r = crate::destruction::destruction_blast(g, a1, a2); r.w(&mut o);  }
177 => { let a1: V3 = Io::r(&mut i); let a2: Option<crate::destruction::DestructionDoorsForOpts> = Io::r(&mut i); let r = crate::destruction::destruction_doors_for(g, a1, a2); r.w(&mut o);  }
178 => { let a1: V3 = Io::r(&mut i); let a2: Option<crate::destruction::DestructionWallsForOpts> = Io::r(&mut i); let r = crate::destruction::destruction_walls_for(g, a1, a2); r.w(&mut o);  }
180 => { let a1: V3 = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); crate::destruction::destruction_bang(g, a1, a2, a3, a4);  }
181 => { let a1: V3 = Io::r(&mut i); let a2: Option<crate::destruction::DestructionTestImpulseKind> = Io::r(&mut i); let a3: Option<V3> = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); crate::destruction::destruction_test_impulse(g, a1, a2, a3, a4);  }
182 => { let a1: V3 = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); crate::destruction::destruction_thud(g, a1, a2, a3);  }
188 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: bool = Io::r(&mut i); crate::floormess::mess_wall(g, a1, a2, a3, a4);  }
191 => { let a1: DebrisChunkId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::break_table::breaks_landed(g, a1, a2);  }
192 => { let a1: DebrisChunkId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::break_table::breaks_kicked(g, a1, a2);  }
203 => { let a1: crate::debris::ChunkKind = Io::r(&mut i); let a2: BoxId = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: V3 = Io::r(&mut i); let a5: Option<f64> = Io::r(&mut i); let a6: Option<bool> = Io::r(&mut i); let r = crate::debris::debris_spawn_raw(g, a1, a2, a3, a4, a5, a6); r.w(&mut o);  }
205 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); crate::debris::debris_boxes(g, &mut a1); a1.w(&mut o); }
208 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::wind::wind_storm_share(g, a1, a2); r.w(&mut o);  }
209 => {  let r = crate::wind::wind_storm_drift(g); r.w(&mut o);  }
210 => { let a1: f64 = Io::r(&mut i); crate::wind::wind_update(g, a1);  }
213 => { let a1: crate::devices::DevicesSnap = Io::r(&mut i); crate::devices::devices_restore(g, &a1);  }
216 => { let a1: String = Io::r(&mut i); let a2: crate::devices::LearnCodeHow = Io::r(&mut i); let a3: Option<GuardId> = Io::r(&mut i); let r = crate::devices::learn_code(g, &a1, a2, a3); r.w(&mut o);  }
217 => { let a1: BoxId = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::glasspen::pane_type(g, a1, a2); r.w(&mut o);  }
218 => {  crate::glasspen::reset_glass_pen(g);  }
219 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<f64> = Io::r(&mut i); let r = crate::glasspen::pane_toll(g, a1, a2, a3, a4, a5); r.w(&mut o);  }
220 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: f64 = Io::r(&mut i); let a6: f64 = Io::r(&mut i); let r = crate::glasspen::pane_crossing(g, a1, a2, a3, a4, a5, a6); r.w(&mut o);  }
221 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<Option<crate::glasspen::RoundPathSt>> = Io::r(&mut i); let a6: Option<f64> = Io::r(&mut i); let r = crate::glasspen::round_path(g, a1, a2, a3, a4, a5, a6); r.w(&mut o);  }
222 => { let a1: Vec<crate::glasspen::PaneCross> = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let mut a4: Option<crate::glasspen::ShellGlass> = Io::r(&mut i); crate::glasspen::commit_crossings(g, &a1, a2, a3, a4.as_mut()); a4.w(&mut o); }
223 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let mut a4: Option<crate::glasspen::ShellGlass> = Io::r(&mut i); let r = crate::glasspen::arm_through_glass(g, a1, a2, a3, a4.as_mut()); r.w(&mut o); a4.w(&mut o); }
224 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Vec<crate::scene_collision::SegmentBlockedPanesItem> = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); let r = crate::glasspen::shot_passes(g, a1, a2, &a3, a4); r.w(&mut o);  }
225 => { let a1: Vec<V3> = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let r = crate::glasspen::shot_through_glass(g, &a1, a2, a3); r.w(&mut o);  }
226 => { let a1: GuardId = Io::r(&mut i); let a2: crate::weapons::WeaponId = Io::r(&mut i); crate::gunroles::gun_hand(g, a1, a2);  }
227 => { let a1: Vec<crate::gunroles::DealMan> = Io::r(&mut i); let r = crate::gunroles::gun_deal(g, &a1); r.w(&mut o);  }
228 => { let a1: GuardId = Io::r(&mut i); let r = crate::doorway::leaf_stalled(g, a1); r.w(&mut o);  }
229 => { let a1: GuardId = Io::r(&mut i); let a2: Option<V3> = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: bool = Io::r(&mut i); let r = crate::doorway::off_threshold(g, a1, a2, a3, a4); r.w(&mut o);  }
230 => { let a1: Vec<GuardId> = Io::r(&mut i); let r = crate::startfile::team_starts_fallen_in(g, &a1); r.w(&mut o);  }
231 => {  let r = crate::issue::issue_stage(g); r.w(&mut o);  }
232 => { let a1: Option<crate::nightbreach::NightBreachLayout> = Io::r(&mut i); let r = crate::nightbreach::night_breach_start(g, a1.as_ref()); r.w(&mut o);  }
233 => {  crate::nightbreach::night_breach_stop(g);  }
234 => { let a1: GuardId = Io::r(&mut i); let a2: crate::nightbreach::NightBreachMark = Io::r(&mut i); crate::nightbreach::night_breach_adopt(g, a1, &a2);  }
235 => {  crate::nightbreach::night_breach_restage(g);  }
237 => {  let r = crate::nightbreach::night_breach_order(g); r.w(&mut o);  }
241 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::handleside::handle_spot(g, a1, a2); r.w(&mut o);  }
242 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); let r = crate::handleside::tool_spot(g, a1, a2, a3, a4, a5); r.w(&mut o);  }
243 => { let a1: V3 = Io::r(&mut i); let a2: Vec<Option<BoxId>> = Io::r(&mut i); let a3: bool = Io::r(&mut i); let a4: Vec<crate::aimcost::RoundLeg> = Io::r(&mut i); let a5: Option<f64> = Io::r(&mut i); let a6: Option<f64> = Io::r(&mut i); let r = crate::aimcost::lamp_struck(g, a1, &a2, a3, &a4, a5, a6); r.w(&mut o);  }
244 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::aim::track_guard(g, a1, a2);  }
245 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::aim::guard_round(g, a1, a2, a3); r.w(&mut o);  }
246 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: bool = Io::r(&mut i); let r = crate::aim::aim_brake(g, a1, a2, a3); r.w(&mut o);  }
247 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); crate::aim::note_bore(g, a1, a2);  }
248 => { let a1: GuardId = Io::r(&mut i); let r = crate::aim::bore_fouled(g, a1); r.w(&mut o);  }
249 => { let a1: f64 = Io::r(&mut i); crate::aim::track_player(g, a1);  }
250 => {  let r = crate::aim::sam_aim(g); r.w(&mut o);  }
251 => {  crate::qrf::reset_qrf(g);  }
252 => {  crate::qrf::qrf_stand_down(g);  }
253 => { let a1: crate::qrf::QrfReason = Io::r(&mut i); let r = crate::qrf::request_qrf(g, a1); r.w(&mut o);  }
255 => { let a1: Option<String> = Io::r(&mut i); let r = crate::qrf::call_perimeter(g, a1.as_deref()); r.w(&mut o);  }
256 => { let a1: GuardId = Io::r(&mut i); let r = crate::runner::runner_break(g, a1); r.w(&mut o);  }
257 => {  crate::cues::cues_reset(g);  }
258 => { let a1: String = Io::r(&mut i); crate::cues::cues_raw(g, &a1);  }
259 => { let a1: GuardId = Io::r(&mut i); let a2: String = Io::r(&mut i); crate::cues::cues_heard(g, a1, &a2);  }
260 => { let a1: GuardId = Io::r(&mut i); let a2: String = Io::r(&mut i); let a3: f64 = Io::r(&mut i); crate::cues::cues_net(g, a1, &a2, a3);  }
261 => { let a1: GuardId = Io::r(&mut i); let a2: String = Io::r(&mut i); let r = crate::cues::cues_call(g, a1, &a2); r.w(&mut o);  }
262 => { let a1: GuardId = Io::r(&mut i); let a2: String = Io::r(&mut i); let r = crate::cues::cues_overheard(g, a1, &a2); r.w(&mut o);  }
263 => { let a1: Vec<crate::cues::CuesAdoptMsgsItem> = Io::r(&mut i); crate::cues::cues_adopt(g, &a1);  }
265 => {  crate::cues::cues_tick(g);  }
266 => { let a1: Option<GuardId> = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let a3: Vec<GuardId> = Io::r(&mut i); crate::netquiet::net_quiet_asked(g, a1, a2, &a3);  }
267 => { let a1: GuardId = Io::r(&mut i); let a2: Vec<GuardId> = Io::r(&mut i); let a3: Vec<GuardId> = Io::r(&mut i); crate::netquiet::net_quiet_closed(g, a1, &a2, &a3);  }
268 => { let a1: Option<GuardId> = Io::r(&mut i); let a2: Vec<GuardId> = Io::r(&mut i); crate::netquiet::net_quiet_body(g, a1, &a2);  }
269 => { let a1: GuardId = Io::r(&mut i); crate::netquiet::net_quiet_fall(g, a1);  }
270 => { let a1: f64 = Io::r(&mut i); crate::netquiet::net_quiet_tick(g, a1);  }
271 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: bool = Io::r(&mut i); let r = crate::netquiet::net_quiet_pose(g, a1, a2, a3); r.w(&mut o);  }
273 => { let a1: Option<crate::roomclear::CheckStructureS> = Io::r(&mut i); let r = crate::roomclear::check_structure(g, a1.as_ref()); r.w(&mut o);  }
274 => { let a1: crate::roomclear::CheckSquadsS = Io::r(&mut i); let r = crate::roomclear::check_squads(g, &a1); r.w(&mut o);  }
275 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::roomclear::wall_between(g, a1, a2); r.w(&mut o);  }
277 => { let a1: Option<f64> = Io::r(&mut i); let r = crate::roomclear::room_clear_scenario_start(g, a1); r.w(&mut o);  }
278 => {  crate::roomclear::room_clear_scenario_stop(g);  }
279 => {  crate::roomclear::room_clear_scenario_restage(g);  }
281 => { let a1: GuardId = Io::r(&mut i); let a2: crate::roomclear::Placement = Io::r(&mut i); crate::roomclear::room_clear_scenario_adopt(g, a1, &a2);  }
283 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::stackroom::stack_room_has(g, a1, a2, a3); r.w(&mut o);  }
284 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<Vec<crate::orders::StackPost>> = Io::r(&mut i); let a4: Vec<V3> = Io::r(&mut i); let a5: Vec<GuardId> = Io::r(&mut i); let r = crate::stackroom::stack_view_fix(g, a1, a2, a3.as_deref(), &a4, &a5); r.w(&mut o);  }
285 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::stackroom::stack_room_probe(g, a1, a2); r.w(&mut o);  }
286 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Vec<V3> = Io::r(&mut i); let r = crate::rearwatch::rear_ways(g, a1, a2, &a3); r.w(&mut o);  }
287 => {  crate::rearwatch::rear_watch_tick(g);  }
288 => { let a1: V3 = Io::r(&mut i); let a2: CoverOpeningId = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::rearwatch::rear_sees(g, a1, a2, a3); r.w(&mut o);  }
289 => { let a1: V3 = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let a3: Option<V3> = Io::r(&mut i); let a4: Option<JsSet<String>> = Io::r(&mut i); let r = crate::roomread::room_read(g, a1, a2, a3, a4.as_ref()); r.w(&mut o);  }
290 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Vec<V3> = Io::r(&mut i); let r = crate::roomfit::watchables_at(g, a1, a2, a3, &a4); r.w(&mut o);  }
291 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: V3 = Io::r(&mut i); let a5: Option<Vec<V3>> = Io::r(&mut i); let a6: Option<Vec<String>> = Io::r(&mut i); let r = crate::roomfit::fit_spots(g, a1, a2, a3, a4, a5.as_deref(), a6.as_deref()); r.w(&mut o);  }
293 => {  let r = crate::cover::cover_ready(g); r.w(&mut o);  }
300 => {  let r = crate::cover::cover_summary(g); r.w(&mut o);  }
301 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::gunslots::side_out(g, a1, a2); r.w(&mut o);  }
302 => { let a1: GuardId = Io::r(&mut i); let a2: Option<crate::game::Foe> = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::coverfire::cover_seen(g, a1, a2, a3); r.w(&mut o);  }
304 => { let a1: GuardId = Io::r(&mut i); let a2: crate::game::Foe = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); crate::coverfire::coverfire_saw(g, a1, a2, a3);  }
309 => { let a1: DoorId = Io::r(&mut i); let a2: crate::teams::Team = Io::r(&mut i); let a3: crate::tactics::DangerWhy = Io::r(&mut i); let a4: Option<String> = Io::r(&mut i); let r = crate::tactics::mark_danger(g, a1, a2, a3, a4.as_deref()); r.w(&mut o);  }
310 => { let a1: GuardId = Io::r(&mut i); crate::tactics::tactics_man_down(g, a1);  }
311 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::tactics::tactics_penalty(g, a1, a2); r.w(&mut o);  }
312 => { let a1: Vec<GuardId> = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<String> = Io::r(&mut i); let a4: Option<GuardId> = Io::r(&mut i); let r = crate::tactics::plan_approach(g, &a1, a2, a3.as_deref(), a4); r.w(&mut o);  }
315 => { let a1: GuardId = Io::r(&mut i); let a2: Option<crate::tactics::TacticsPenalty> = Io::r(&mut i); let r = crate::lanes::lane_penalty(g, a1, a2.as_ref()); r.w(&mut o);  }
316 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::lanes::lane_covers(g, a1, a2); r.w(&mut o);  }
317 => { let a1: GuardId = Io::r(&mut i); crate::lanes::lane_reload(g, a1);  }
318 => {  crate::lanes::lanes_reset(g);  }
320 => { let a1: RoomfightDefId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<DoorId> = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); crate::pressure::run(g, a1, a2, a3, a4);  }
321 => { let a1: RoomfightDefId = Io::r(&mut i); let a2: DoorId = Io::r(&mut i); let a3: Option<RoomfightRoomId> = Io::r(&mut i); let r = crate::pressure::charge(g, a1, a2, a3); r.w(&mut o);  }
322 => { let a1: RoomfightDefId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); crate::pressure::loose(g, a1, a2);  }
323 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::pressure::boots(g, a1, a2); r.w(&mut o);  }
324 => { let a1: crate::difficulty::Difficulty = Io::r(&mut i); let a2: Option<String> = Io::r(&mut i); let r = crate::difficulty::preset_levers(g, a1, a2.as_deref()); r.w(&mut o);  }
325 => { let a1: String = Io::r(&mut i); crate::difficulty::difficulty_here(g, &a1);  }
326 => {  let r = crate::difficulty::difficulty_now(g); r.w(&mut o);  }
327 => { let a1: Option<String> = Io::r(&mut i); let r = crate::difficulty::levers_shown(g, a1.as_deref()); r.w(&mut o);  }
328 => {  let r = crate::difficulty::levers_mixed(g); r.w(&mut o);  }
329 => { let a1: crate::difficulty::Difficulty = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); crate::difficulty::set_difficulty(g, a1, a2);  }
330 => { let a1: crate::difficulty::Difficulty = Io::r(&mut i); crate::difficulty::set_base(g, a1);  }
331 => { let a1: crate::difficulty::LeverKey = Io::r(&mut i); let a2: Option<String> = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let a4: Option<String> = Io::r(&mut i); crate::difficulty::set_lever(g, a1, a2.as_deref(), a3, a4.as_deref());  }
332 => { let a1: Option<bool> = Io::r(&mut i); crate::difficulty::set_cone_visit(g, a1);  }
333 => {  crate::difficulty::difficulty_custom(g);  }
334 => { let a1: String = Io::r(&mut i); let r = crate::difficulty::difficulty_href(g, &a1); r.w(&mut o);  }
335 => { let a1: String = Io::r(&mut i); let r = crate::difficulty::levers_href(g, &a1); r.w(&mut o);  }
336 => { let a1: Option<crate::difficulty::DifficultyWordsLevel> = Io::r(&mut i); let a2: Option<crate::difficulty::DifficultyWordsSquads> = Io::r(&mut i); let r = crate::difficulty::difficulty_words(g, a1, a2.as_ref()); r.w(&mut o);  }
337 => { let a1: Option<bool> = Io::r(&mut i); let r = crate::difficulty::levers_line(g, a1); r.w(&mut o);  }
338 => { let a1: PlayerId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); crate::stickycam::sticky_issue(g, a1, a2, a3);  }
339 => { let a1: PlayerId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::stickycam::fire_cam(g, a1, a2, a3); r.w(&mut o);  }
340 => { let a1: PlayerId = Io::r(&mut i); let r = crate::stickycam::cam_to_take(g, a1); r.w(&mut o);  }
341 => { let a1: PlayerId = Io::r(&mut i); let r = crate::stickycam::take_cam(g, a1); r.w(&mut o);  }
342 => {  let r = crate::quietfog::quiet_fog(g); r.w(&mut o);  }
343 => {  let r = crate::quietfog::quiet_cams(g); r.w(&mut o);  }
344 => { let a1: bool = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); crate::quietfog::set_quiet_cams(g, a1, a2);  }
345 => { let a1: crate::quietfog::QuietRung = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); crate::quietfog::set_quiet_fog(g, a1, a2);  }
347 => { let a1: String = Io::r(&mut i); let a2: bool = Io::r(&mut i); let r = crate::quietfog::quiet_fog_href(g, &a1, a2); r.w(&mut o);  }
348 => {  crate::quietfog::quiet_fog_end(g);  }
367 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::calmtorch::place_light(g, a1, a2); r.w(&mut o);  }
368 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let r = crate::calmtorch::work_lit(g, a1, a2, a3); r.w(&mut o);  }
369 => { let a1: GuardId = Io::r(&mut i); let r = crate::calmtorch::place_read(g, a1); r.w(&mut o);  }
370 => {  crate::calmtorch::calm_torch_posed(g);  }
371 => { let a1: GuardId = Io::r(&mut i); let r = crate::darkfoe::dark_foe_blind(g, a1); r.w(&mut o);  }
372 => { let a1: RoomfightDefId = Io::r(&mut i); let r = crate::darkfoe::dark_foe_spot(g, a1); r.w(&mut o);  }
373 => { let a1: GuardId = Io::r(&mut i); let a2: Option<RoomfightDefId> = Io::r(&mut i); let r = crate::darkfoe::dark_foe_kept(g, a1, a2); r.w(&mut o);  }
374 => {  let r = crate::darkfoe::dark_foe_summary(g); r.w(&mut o);  }
375 => {  let r = crate::whistle::try_whistle(g); r.w(&mut o);  }
376 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<crate::attention::AttentionLendO> = Io::r(&mut i); crate::attention::attention_lend(g, a1, a2, a3);  }
378 => { let a1: Option<GuardId> = Io::r(&mut i); let a2: Option<crate::needs::NeedKind> = Io::r(&mut i); let r = crate::needs::needs_send(g, a1, a2); r.w(&mut o);  }
379 => { let a1: GuardId = Io::r(&mut i); let a2: crate::needs::NeedKind = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::needs::needs_set_level(g, a1, a2, a3); r.w(&mut o);  }
380 => { let a1: CharId = Io::r(&mut i); let a2: crate::weapons::PelletSpecBreach = Io::r(&mut i); let r = crate::gunwall::breach_aim(g, a1, a2); r.w(&mut o);  }
381 => {  let r = crate::lightswitch::wall_switches(g); r.w(&mut o);  }
382 => { let a1: crate::lightswitch::WallSwitch = Io::r(&mut i); let a2: bool = Io::r(&mut i); let a3: CharId = Io::r(&mut i); crate::lightswitch::flip_switch(g, &a1, a2, a3);  }
383 => { let a1: PlayerId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::lightswitch::switch_near(g, a1, a2); r.w(&mut o);  }
384 => { let a1: PlayerId = Io::r(&mut i); let a2: crate::game::Verb = Io::r(&mut i); let r = crate::lightswitch::switch_act(g, a1, a2); r.w(&mut o);  }
387 => {  let r = crate::interact::cancel_sustained(g); r.w(&mut o);  }
389 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); crate::samstun::stagger_player(g, a1, a2);  }
391 => { let a1: GuardId = Io::r(&mut i); let r = crate::stalk::stalk_key_can(g, a1); r.w(&mut o);  }
393 => { let a1: f64 = Io::r(&mut i); let r = crate::darkdoor::dark_room(g, a1); r.w(&mut o);  }
394 => { let a1: GuardId = Io::r(&mut i); let r = crate::darkdoor::dark_door_green(g, a1); r.w(&mut o);  }
395 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let r = crate::startle::give_startle(g, a1, a2, a3, a4); r.w(&mut o);  }
396 => { let a1: GuardId = Io::r(&mut i); let a2: crate::game::GuardState = Io::r(&mut i); let a3: bool = Io::r(&mut i); crate::startle::startle_sight(g, a1, a2, a3);  }
397 => { let a1: CharId = Io::r(&mut i); let a2: Option<GuardId> = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: V3 = Io::r(&mut i); let a5: bool = Io::r(&mut i); crate::flinch::flinch_round(g, a1, a2, a3, a4, a5);  }
400 => { let a1: Vec<GuardId> = Io::r(&mut i); crate::rescueflow::rescue_flow_release(g, &a1);  }
404 => { let a1: Option<CharId> = Io::r(&mut i); let r = crate::rescueend::rescue_end_word(g, a1); r.w(&mut o);  }
408 => {  crate::rescueflush::rescue_flush_reset(g);  }
413 => { let a1: Option<crate::scene_scenarios::RescueData> = Io::r(&mut i); let r = crate::rescue::check_rescue(g, a1.as_ref()); r.w(&mut o);  }
414 => {  let r = crate::rescue::rescue_start(g); r.w(&mut o);  }
415 => {  crate::rescue::rescue_stop(g);  }
416 => {  crate::rescue::rescue_restage(g);  }
420 => {  let r = crate::rescue::rescue_press(g); r.w(&mut o);  }
424 => {  let r = crate::rescue::rescue_captor(g); r.w(&mut o);  }
425 => { let a1: DoorId = Io::r(&mut i); let r = crate::rescue::rescue_last_warning(g, a1); r.w(&mut o);  }
426 => {  crate::rescuecheck::rescue_check_reset(g);  }
439 => { let a1: PlayerId = Io::r(&mut i); let a2: crate::doors::DoorContact = Io::r(&mut i); crate::sprintdoor::sprint_door_after(g, a1, &a2);  }
440 => { let a1: V3 = Io::r(&mut i); let a2: Vec<V3> = Io::r(&mut i); let r = crate::seenwalk::seen_price(g, a1, &a2); r.w(&mut o);  }
444 => { let a1: f64 = Io::r(&mut i); crate::wet::update_wetness(g, a1);  }
447 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<Vec<CharId>> = Io::r(&mut i); let r = crate::he40::shell_solve(g, a1, a2, a3.as_deref()); r.w(&mut o);  }
448 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::he40::fire_shell(g, a1, a2, a3); r.w(&mut o);  }
449 => {  let r = crate::samlook::sam_slip_man(g); r.w(&mut o);  }
453 => { let a1: f64 = Io::r(&mut i); crate::doorpath::door_path_tick(g, a1);  }
455 => { let a1: V3 = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::irlight::ir_light_at(g, a1, a2); r.w(&mut o);  }
456 => { let a1: GuardId = Io::r(&mut i); let r = crate::irlight::ir_on_man(g, a1); r.w(&mut o);  }
457 => { let a1: V3 = Io::r(&mut i); let r = crate::nvg::nvg_place_light(g, a1); r.w(&mut o);  }
459 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::nvg::nvg_sight(g, a1, a2); r.w(&mut o);  }
462 => {  crate::entrypoints::entry_points_tick(g);  }
463 => { let a1: V3 = Io::r(&mut i); crate::entrypoints::entry_points_mark(g, a1);  }
464 => { let a1: V3 = Io::r(&mut i); let a2: Option<DoorId> = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::entrypoints::entry_points_risk_at(g, a1, a2, a3); r.w(&mut o);  }
468 => {  let r = crate::unseen::unseen_live(g); r.w(&mut o);  }
469 => { let a1: GuardId = Io::r(&mut i); let r = crate::unseen::under_unseen(g, a1); r.w(&mut o);  }
472 => {  let r = crate::unseen::unseen_summary(g); r.w(&mut o);  }
473 => { let a1: f64 = Io::r(&mut i); crate::squadsmoke::squad_smoke_stage(g, a1);  }
474 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let a4: Option<V3> = Io::r(&mut i); let r = crate::squadsmoke::squad_arc_to(g, a1, a2, a3, a4); r.w(&mut o);  }
475 => { let a1: Vec<GuardId> = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::squadsmoke::squad_smoke_thrower(g, &a1, a2); r.w(&mut o);  }
476 => { let a1: Vec<(GuardId, f64)> = Io::r(&mut i); crate::squad40::squad40_stage(g, &a1);  }
479 => { let a1: Vec<GuardId> = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: crate::squadthrow::SquadThrowKind = Io::r(&mut i); let r = crate::squadthrow::squad_thrower(g, &a1, a2, a3); r.w(&mut o);  }
480 => { let a1: Vec<GuardId> = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: crate::squadthrow::SquadThrowKind = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); let r = crate::squadthrow::squad_throw_plan(g, &a1, a2, a3, a4, a5); r.w(&mut o);  }
481 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<crate::squadthrow::SquadFragCatchesMoved> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); let a6: Option<crate::squadthrow::SquadFragCatchesBurst> = Io::r(&mut i); let r = crate::squadthrow::squad_frag_catches(g, a1, a2, a3, a4, a5, a6.as_ref()); r.w(&mut o);  }
482 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::squadthrow::squad_stun_catches(g, a1, a2); r.w(&mut o);  }
486 => {  crate::squadnet::squad_net_tick(g);  }
487 => { let a1: crate::orders::OrderTargetDoor = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); let r = crate::autopilot::auto_entries(g, &crate::seats::here(g), a1, a2); r.w(&mut o);  }
488 => { let a1: crate::orders::OrderTarget = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); let a3: Option<Vec<GuardId>> = Io::r(&mut i); let r = crate::autopilot::auto_pick(g, &crate::seats::here(g), &a1, a2, a3.as_deref()); r.w(&mut o);  }
489 => { let a1: crate::orders::OrderTarget = Io::r(&mut i); let r = crate::autopilot::auto_quick(g, &a1); r.w(&mut o);  }
490 => { let a1: crate::orders::OrderTarget = Io::r(&mut i); let a2: Option<crate::autopilot::AutoPick> = Io::r(&mut i); let r = crate::autopilot::auto_hint(g, &a1, a2.as_ref()); r.w(&mut o);  }
491 => {  let r = crate::kitanim::kit_sling_rest(g); r.w(&mut o);  }
494 => { let mut a1: crate::kitanim::KitShowV = Io::r(&mut i); let r = crate::kitanim::kit_show(g, &mut a1); r.w(&mut o); a1.w(&mut o); }
495 => { let a1: crate::fogofwar::FogEye = Io::r(&mut i); let r = crate::peeksight::peek_eye(g, a1); r.w(&mut o);  }
496 => { let a1: f64 = Io::r(&mut i); crate::smokebreak::update_smoke_break(g, a1);  }
499 => { let a1: Option<crate::cmd::PlayerCmd> = Io::r(&mut i); let a2: Option<V3> = Io::r(&mut i); let r = crate::cmd::check_cmd(g, a1.as_ref(), a2); r.w(&mut o);  }
500 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<crate::weapons::WeaponId> = Io::r(&mut i); let a6: Option<CharId> = Io::r(&mut i); let r = crate::friendlyfire::friend_in_cone(g, a1, a2, a3, a4, a5, a6); r.w(&mut o);  }
501 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<CharId> = Io::r(&mut i); crate::friendlyfire::ff_past(g, a1, a2, a3);  }
502 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let r = crate::noshove::no_shove_keep(g, a1, a2, a3, a4); r.w(&mut o);  }
504 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); crate::doorforce::door_force_saw(g, a1, a2, a3);  }
506 => { let a1: crate::glassbreak::GlassBreakLetGoF = Io::r(&mut i); crate::glassbreak::glass_break_let_go(g, &a1);  }
508 => { let a1: f64 = Io::r(&mut i); let r = crate::glassbreak::glass_break_type_of_pane(g, a1); r.w(&mut o);  }
509 => { let a1: f64 = Io::r(&mut i); let a2: Option<crate::glasspen::PaneType> = Io::r(&mut i); crate::glassbreak::glass_break_set_type(g, a1, a2);  }
513 => { let a1: V3 = Io::r(&mut i); let a2: Option<crate::glassbreak::GlassBlastOpts> = Io::r(&mut i); let r = crate::glassbreak::glass_break_blast(g, a1, a2); r.w(&mut o);  }
515 => { let a1: f64 = Io::r(&mut i); crate::glassbreak::glass_break_update(g, a1);  }
516 => { let a1: f64 = Io::r(&mut i); let r = crate::glassbreak::glass_break_pane_info(g, a1); r.w(&mut o);  }
520 => {  let r = crate::glassbreak::glass_break_break_test(g); r.w(&mut o);  }
521 => {  let r = crate::glassbreak::glass_break_summary(g); r.w(&mut o);  }
522 => {  crate::glassbreak::glass_break_reset(g);  }
523 => {  crate::glass::reset_glass(g);  }
524 => { let a1: BoxId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); crate::glass::glass_event(g, a1, a2, a3, a4);  }
525 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<V3> = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: f64 = Io::r(&mut i); let a6: Option<crate::glass::GlassImpactCause> = Io::r(&mut i); let a7: Option<bool> = Io::r(&mut i); let a8: Option<crate::glass::GlassImpactOpts> = Io::r(&mut i); let r = crate::glass::glass_impact(g, a1, a2, a3, a4, a5, a6, a7, a8); r.w(&mut o);  }
526 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<V3> = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<crate::glass::GlassHitCause> = Io::r(&mut i); let a6: Option<bool> = Io::r(&mut i); let r = crate::glass::glass_hit(g, a1, a2, a3, a4, a5, a6); r.w(&mut o);  }
527 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::glass::crack_near(g, a1, a2, a3); r.w(&mut o);  }
529 => {  let r = crate::glass::crack_test(g); r.w(&mut o);  }
530 => { let a1: f64 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let r = crate::wallpen::thin_toll(g, a1, a2, a3, a4); r.w(&mut o);  }
531 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<Option<crate::glasspen::RoundPathSt>> = Io::r(&mut i); let a6: Option<f64> = Io::r(&mut i); let r = crate::wallpen::thin_path(g, a1, a2, a3, a4, a5, a6); r.w(&mut o);  }
532 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); let a5: Option<crate::wallpen::ThinLineFirst> = Io::r(&mut i); let r = crate::wallpen::thin_line(g, a1, a2, a3, a4, a5); r.w(&mut o);  }
533 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let r = crate::wallpen::flight_len(g, a1, a2, a3, a4); r.w(&mut o);  }
534 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: Option<CharId> = Io::r(&mut i); crate::suppression::note_round(g, a1, a2, a3, a4);  }
537 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::suppression::cover_from(g, a1, a2); r.w(&mut o);  }
538 => { let mut a1: crate::condition::Man = Io::r(&mut i); let r = crate::condition::condition_hurt(g, &mut a1); r.w(&mut o); a1.w(&mut o); }
540 => { let a1: CharId = Io::r(&mut i); let a2: CharId = Io::r(&mut i); let r = crate::roomfight::pair_smoke(g, a1, a2); r.w(&mut o);  }
541 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::roomfight::room_fight_sight_on(g, a1, a2); r.w(&mut o);  }
544 => { let a1: V3 = Io::r(&mut i); let r = crate::roomfight::room_fight_room_at(g, a1); r.w(&mut o);  }
548 => {  let r = crate::enemysquads::squad_ways_round(g); r.w(&mut o);  }
549 => { let a1: GuardId = Io::r(&mut i); let r = crate::morale::in_the_dark(g, a1); r.w(&mut o);  }
550 => { let a1: GuardId = Io::r(&mut i); let r = crate::morale::has_mate(g, a1); r.w(&mut o);  }
551 => { let a1: GuardId = Io::r(&mut i); let r = crate::morale::training_of(g, a1); r.w(&mut o);  }
552 => { let a1: GuardId = Io::r(&mut i); let r = crate::morale::leader_of(g, a1); r.w(&mut o);  }
553 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::morale::morale_shock(g, a1, a2);  }
554 => { let a1: GuardId = Io::r(&mut i); crate::morale::morale_man_down(g, a1);  }
555 => { let a1: Option<bool> = Io::r(&mut i); crate::morale::morale_reset(g, a1);  }
556 => { let a1: f64 = Io::r(&mut i); crate::morale::morale_tick(g, a1);  }
557 => {  let r = crate::morale::morale_roster(g); r.w(&mut o);  }
559 => { let a1: RoomfightRoomId = Io::r(&mut i); let r = crate::doortools::open_floor(g, a1); r.w(&mut o);  }
562 => {  crate::doortools::door_tools_sync(g);  }
563 => {  crate::doortools::door_tools_reset(g);  }
564 => { let a1: f64 = Io::r(&mut i); crate::doortools::door_tools_alone(g, a1);  }
565 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::doortools::door_tools_set_back(g, a1, a2);  }
568 => { let a1: crate::doortools::DoorToolsOrderKind = Io::r(&mut i); let a2: Vec<GuardId> = Io::r(&mut i); let a3: DoorId = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<DoorGateId> = Io::r(&mut i); let a6: Option<bool> = Io::r(&mut i); let r = crate::doortools::door_tools_order(g, a1, &a2, a3, a4, a5, a6); r.w(&mut o);  }
577 => {  let r = crate::doortools::door_tools_cancel_sam(g); r.w(&mut o);  }
578 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Vec<GuardId> = Io::r(&mut i); let a4: Option<Vec<GuardId>> = Io::r(&mut i); let r = crate::doortools::door_tools_stack_posts(g, a1, a2, &a3, a4.as_deref()); r.w(&mut o);  }
579 => { let a1: Option<crate::orders::Who> = Io::r(&mut i); let r = crate::doortools::door_tools_go(g, a1); r.w(&mut o);  }
583 => { let a1: Option<ChargeId> = Io::r(&mut i); crate::doortools::door_tools_trigger(g, a1);  }
585 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let r = crate::doortools::door_tools_pods(g, a1, a2, a3); r.w(&mut o);  }
587 => { let a1: Option<ChargeId> = Io::r(&mut i); let r = crate::doortools::door_tools_blow(g, a1); r.w(&mut o);  }
591 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: bool = Io::r(&mut i); let a4: crate::combat::FireOpts = Io::r(&mut i); let a5: crate::weapons::WeaponDef = Io::r(&mut i); crate::shotgun::fire_shell(g, a1, a2, a3, &a4, &a5);  }
592 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Option<Vec<V3>> = Io::r(&mut i); let a4: Option<V3> = Io::r(&mut i); let a5: Option<crate::shotgun::BreachSpotClose> = Io::r(&mut i); let r = crate::shotgun::breach_spot(g, a1, a2, a3.as_deref(), a4, a5); r.w(&mut o);  }
593 => { let a1: DoorId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: crate::bigkit::SamBigSpotTool = Io::r(&mut i); let r = crate::bigkit::sam_big_spot(g, a1, a2, a3); r.w(&mut o);  }
594 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: crate::bigkit::BigSpotsTool = Io::r(&mut i); let r = crate::bigkit::big_spots(g, a1, a2, a3); r.w(&mut o);  }
595 => { let a1: DoorId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::bigkit::sam_big_far(g, a1, a2); r.w(&mut o);  }
596 => { let a1: DoorId = Io::r(&mut i); crate::bigkit::sam_big_start(g, a1);  }
598 => { let a1: GuardId = Io::r(&mut i); crate::bigkit::gun_down(g, a1);  }
599 => { let a1: GuardId = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); crate::bigkit::hammer_in(g, a1, a2);  }
600 => {  let r = crate::player::can_stand(g); r.w(&mut o);  }
601 => { let mut a1: crate::player::Legs = Io::r(&mut i); let r = crate::player::crouch_key(g, &mut a1); r.w(&mut o); a1.w(&mut o); }
603 => {  let r = crate::player::can_wall_snap(g); r.w(&mut o);  }
605 => {  crate::player::release_wall(g);  }
607 => {  let r = crate::player::launch_cancel(g); r.w(&mut o);  }
609 => { let a1: GuardId = Io::r(&mut i); let a2: Option<crate::close::CloseKind> = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); crate::player::start_takedown(g, a1, a2, a3, a4);  }
611 => { let a1: GuardId = Io::r(&mut i); let a2: Option<crate::close::CloseKind> = Io::r(&mut i); crate::player::start_grab(g, a1, a2);  }
613 => {  crate::player::ease_off_hold(g);  }
614 => { let a1: GuardId = Io::r(&mut i); let a2: bool = Io::r(&mut i); let mut a2_f = die_of(a2); crate::player::take_key(g, a1, if a2 { Some(&mut a2_f as &mut dyn FnMut(&mut crate::game::Game) -> f64) } else { None });  }
615 => { let a1: crate::player::FreeHeldWhy = Io::r(&mut i); crate::player::free_held(g, a1);  }
616 => { let a1: DoorId = Io::r(&mut i); crate::player::start_picking(g, a1);  }
617 => { let a1: Option<String> = Io::r(&mut i); crate::player::stop_picking(g, a1.as_deref());  }
619 => { let a1: DoorId = Io::r(&mut i); crate::player::start_wand(g, a1);  }
620 => { let a1: DoorId = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); let a3: Option<crate::player::StartKickTool> = Io::r(&mut i); crate::player::start_kick(g, a1, a2, a3);  }
622 => { let a1: GuardId = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::player::body_dist(g, a1, a2); r.w(&mut o);  }
623 => { let a1: crate::throwables::ThrowSolution = Io::r(&mut i); let a2: crate::game::ThrowKind = Io::r(&mut i); crate::player::do_throw(g, &a1, a2);  }
624 => { let a1: InputId = Io::r(&mut i); let a2: CameraId = Io::r(&mut i); let a3: crate::host_hud::View = Io::r(&mut i); crate::player::update_aim(g, a1, a2, a3);  }
625 => { let a1: CameraId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: f64 = Io::r(&mut i); crate::player::build_interactables(g, a1, a2, a3, a4, a5);  }
626 => {  crate::player::pick_hover(g);  }
627 => { let a1: Option<GuardId> = Io::r(&mut i); crate::player::toggle_drag(g, a1);  }
628 => {  crate::player::cancel_actions(g);  }
629 => { let a1: V3 = Io::r(&mut i); crate::player::teleport_player(g, a1);  }
630 => { let a1: GuardId = Io::r(&mut i); let a2: Vec<f64> = Io::r(&mut i); let r = crate::close::close_at(g, a1, &a2); r.w(&mut o);  }
631 => {  crate::close::close_tick(g);  }
633 => { let a1: GuardId = Io::r(&mut i); let a2: Option<Vec<f64>> = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let r = crate::close::close_seat(g, a1, a2.as_deref(), a3); r.w(&mut o);  }
634 => { let a1: GuardId = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); crate::close::close_unseat(g, a1, a2);  }
635 => { let a1: GuardId = Io::r(&mut i); let a2: Option<crate::close::CloseKind> = Io::r(&mut i); let a3: bool = Io::r(&mut i); let a4: Option<crate::takedown::TakeFallHow> = Io::r(&mut i); crate::takedown::take_fall(g, a1, a2, a3, a4);  }
636 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::spotted::spotted_holds_round(g, a1, a2); r.w(&mut o);  }
637 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::spotted::spotted_take(g, a1, a2); r.w(&mut o);  }
638 => {  crate::spotted::spotted_tick(g);  }
642 => { let a1: Option<crate::pairorders::PairOrdersMarkBy> = Io::r(&mut i); let r = crate::pairorders::pair_orders_mark(g, a1); r.w(&mut o);  }
643 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::pairorders::pair_orders_watcher(g, a1, a2); r.w(&mut o);  }
647 => { let a1: V3 = Io::r(&mut i); let r = crate::pairorders::pair_orders_lit_at(g, a1); r.w(&mut o);  }
649 => {  let r = crate::sneak::sneak_live(g); r.w(&mut o);  }
650 => { let a1: GuardId = Io::r(&mut i); let r = crate::sneak::sneaking(g, a1); r.w(&mut o);  }
651 => { let a1: GuardId = Io::r(&mut i); let r = crate::sneak::sneak_low(g, a1); r.w(&mut o);  }
653 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::sneak::sneak_coming(g, a1, a2); r.w(&mut o);  }
655 => {  let r = crate::sneak::sneak_summary(g); r.w(&mut o);  }
656 => { let a1: bool = Io::r(&mut i); let a2: Option<crate::silent::By> = Io::r(&mut i); crate::silent::set_silent(g, a1, a2);  }
657 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::silent::silent_pace(g, a1, a2); r.w(&mut o);  }
658 => {  let r = crate::silent::urgent(g); r.w(&mut o);  }
659 => { let a1: GuardId = Io::r(&mut i); let r = crate::silent::silent_step(g, a1); r.w(&mut o);  }
660 => { let a1: GuardId = Io::r(&mut i); let r = crate::silent::silent_dark(g, a1); r.w(&mut o);  }
661 => { let a1: GuardId = Io::r(&mut i); let r = crate::silent::silent_hush(g, a1); r.w(&mut o);  }
662 => { let a1: GuardId = Io::r(&mut i); let r = crate::silent::silent_ease(g, a1); r.w(&mut o);  }
663 => {  let r = crate::silent::silent_no_boot(g); r.w(&mut o);  }
664 => {  let r = crate::silent::silent_pick(g); r.w(&mut o);  }
665 => { let a1: DoorId = Io::r(&mut i); let r = crate::silent::silent_boot(g, a1); r.w(&mut o);  }
667 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::silent::silent_holds_round(g, a1, a2); r.w(&mut o);  }
668 => { let a1: GuardId = Io::r(&mut i); let r = crate::silent::silent_holds_target(g, a1); r.w(&mut o);  }
670 => {  crate::silent::silent_tick(g);  }
674 => { let a1: InputId = Io::r(&mut i); let a2: CameraId = Io::r(&mut i); let a3: f64 = Io::r(&mut i); crate::rails::rails_gate(g, a1, a2, a3);  }
677 => { let a1: f64 = Io::r(&mut i); crate::rails::rails_tick(g, a1);  }
678 => {  crate::rails::rails_put_back(g);  }
679 => {  crate::rails::rails_resume(g);  }
682 => { let a1: CharId = Io::r(&mut i); let r = crate::cast::cast_parts(g, a1); r.w(&mut o);  }
683 => { let a1: Vec<CharId> = Io::r(&mut i); let mut a2: Vec<AnyBoxId> = Io::r(&mut i); let a3: f64 = Io::r(&mut i); crate::stature::drawn_boxes(g, &a1, &mut a2, a3); a2.w(&mut o); }
684 => { let a1: CharId = Io::r(&mut i); let a2: bool = Io::r(&mut i); let r = crate::combat::firearm_of(g, a1, a2); r.w(&mut o);  }
685 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: bool = Io::r(&mut i); let a4: Option<crate::combat::FireOpts> = Io::r(&mut i); crate::combat::fire_weapon(g, a1, a2, a3, a4.as_ref());  }
686 => { let a1: CharId = Io::r(&mut i); let r = crate::combat::arm_through(g, a1); r.w(&mut o);  }
687 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::combat::solid_between(g, a1, a2); r.w(&mut o);  }
688 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let a4: Option<crate::game::DownKind> = Io::r(&mut i); let a5: Option<crate::game::DownBy> = Io::r(&mut i); let a6: Option<String> = Io::r(&mut i); crate::combat::kill_guard(g, a1, a2, a3, a4, a5, a6.as_deref());  }
689 => { let a1: CharId = Io::r(&mut i); let a2: crate::combat::DropFromHandKind = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: V3 = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); let a6: Option<bool> = Io::r(&mut i); let a7: Option<V3> = Io::r(&mut i); let r = crate::combat::drop_from_hand(g, a1, a2, a3, a4, a5, a6, a7); r.w(&mut o);  }
690 => { let a1: V3 = Io::r(&mut i); crate::combat::hit_player(g, a1);  }
691 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); crate::combat::hand_props(g, &mut a1); a1.w(&mut o); }
692 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); crate::combat::hidden_props(g, &mut a1); a1.w(&mut o); }
693 => { let a1: GuardId = Io::r(&mut i); let mut a2: Vec<AnyBoxId> = Io::r(&mut i); crate::combat::laser_boxes(g, a1, &mut a2); a2.w(&mut o); }
694 => { let a1: CharId = Io::r(&mut i); let a2: Option<GuardId> = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::combat::laser_reach(g, a1, a2, a3); r.w(&mut o);  }
697 => { let a1: ItemId = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); let a3: Option<crate::combat::DetonateStunSam> = Io::r(&mut i); crate::combat::detonate_stun(g, a1, a2, a3);  }
698 => { let a1: crate::game::ThrowKind = Io::r(&mut i); let a2: V3 = Io::r(&mut i); crate::combat::drop_throwable(g, a1, a2);  }
702 => { let a1: InputId = Io::r(&mut i); crate::director::director_inject_input(g, a1);  }
704 => { let a1: String = Io::r(&mut i); let a2: Option<crate::director::DirectorSignalData> = Io::r(&mut i); crate::director::director_signal(g, &a1, a2.as_ref());  }
706 => { let a1: Option<bool> = Io::r(&mut i); crate::director::director_stop(g, a1);  }
709 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::snow_bog::snow_bog_at(g, a1, a2, a3); r.w(&mut o);  }
712 => { let a1: crate::throwables::ItemKind = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: Option<crate::throwables::ThrowablesSpawnOpts> = Io::r(&mut i); let r = crate::throwables::throwables_spawn(g, a1, a2, a3, a4); r.w(&mut o);  }
714 => { let a1: f64 = Io::r(&mut i); crate::throwables::throwables_update(g, a1);  }
715 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); crate::throwables::throwables_boxes(g, &mut a1); a1.w(&mut o); }
716 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<crate::frag::GlassView> = Io::r(&mut i); let r = crate::frag::burst_line(g, a1, a2, a3); r.w(&mut o);  }
717 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: Option<crate::frag::GlassView> = Io::r(&mut i); let r = crate::frag::exposure(g, a1, a2, a3, a4); r.w(&mut o);  }
718 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: Option<crate::frag::GlassView> = Io::r(&mut i); let r = crate::frag::exposed_to(g, a1, a2, a3, a4); r.w(&mut o);  }
719 => { let a1: V3 = Io::r(&mut i); let a2: crate::frag::BlastSpec = Io::r(&mut i); let r = crate::frag::blast(g, a1, &a2); r.w(&mut o);  }
721 => { let a1: ItemId = Io::r(&mut i); let r = crate::frag::come_down(g, a1); r.w(&mut o);  }
722 => { let a1: crate::game::ThrowKind = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::frag::rest_of_throw(g, a1, a2, a3); r.w(&mut o);  }
723 => { let a1: ItemId = Io::r(&mut i); let r = crate::frag::frag_rest_guess(g, a1); r.w(&mut o);  }
724 => { let a1: crate::frag::FleeSpotC = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: Option<f64> = Io::r(&mut i); let a6: Option<crate::frag::FleeSpotWise> = Io::r(&mut i); let a7: Option<GuardId> = Io::r(&mut i); let r = crate::frag::flee_spot(g, a1, a2, a3, a4, a5, a6.as_ref(), a7); r.w(&mut o);  }
725 => { let a1: crate::frag::FleeByDoorC = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let a5: f64 = Io::r(&mut i); let a6: crate::frag::FleeByDoorWise = Io::r(&mut i); let a7: Option<GuardId> = Io::r(&mut i); let r = crate::frag::flee_by_door(g, a1, a2, a3, a4, a5, &a6, a7); r.w(&mut o);  }
726 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::frag::flee_mates(g, a1, a2); r.w(&mut o);  }
728 => { let a1: GuardId = Io::r(&mut i); let r = crate::frag::frag_flee(g, a1); r.w(&mut o);  }
729 => { let a1: Option<f64> = Io::r(&mut i); let r = crate::frag::frag_step_off(g, a1); r.w(&mut o);  }
731 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::frag::own_clear(g, a1, a2, a3); r.w(&mut o);  }
734 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::teampace::team_tread(g, a1, a2, a3); r.w(&mut o);  }
735 => { let a1: GuardId = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); crate::teampace::team_fight(g, a1, a2);  }
736 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); let r = crate::teampace::team_dash_now(g, a1, a2, a3, a4, a5); r.w(&mut o);  }
737 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let r = crate::teampace::team_dash(g, a1, a2, a3, a4); r.w(&mut o);  }
738 => { let a1: GuardId = Io::r(&mut i); let r = crate::teampace::team_told(g, a1); r.w(&mut o);  }
739 => { let a1: GuardId = Io::r(&mut i); let a2: Option<crate::squad::EndGuardScriptState> = Io::r(&mut i); crate::squad::end_guard_script(g, a1, a2);  }
740 => { let a1: GuardId = Io::r(&mut i); let a2: crate::squad::OrderPatch = Io::r(&mut i); crate::squad::order(g, a1, a2);  }
741 => { let a1: RoomClearId = Io::r(&mut i); crate::squad::room_clear_start(g, a1);  }
742 => { let a1: RoomClearId = Io::r(&mut i); let a2: Option<crate::squad::RoomClearEndState> = Io::r(&mut i); crate::squad::room_clear_end(g, a1, a2);  }
743 => { let a1: RoomClearId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); crate::squad::room_clear_tick(g, a1, a2, a3);  }
747 => { let a1: Option<f64> = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::quiet::quiet_mission_start(g, a1, a2); r.w(&mut o);  }
748 => {  crate::quiet::quiet_mission_stop(g);  }
750 => {  crate::quiet::quiet_mission_current(g);  }
751 => {  crate::quiet::quiet_mission_restage(g);  }
754 => { let a1: V3 = Io::r(&mut i); let a2: Option<crate::quiet::HolderStandArea> = Io::r(&mut i); let r = crate::quiet::holder_stand(g, a1, a2); r.w(&mut o);  }
755 => {  let r = crate::quiet::check_quiet(g); r.w(&mut o);  }
756 => {  let r = crate::workthings::breaker_errand(g); r.w(&mut o);  }
757 => {  crate::workthings::work_things_keep(g); let r = crate::workthings::work_things_things(g, &crate::seats::here(g)); r.w(&mut o);  }
762 => { let a1: CharId = Io::r(&mut i); let r = crate::workthings::work_hand(g, a1); r.w(&mut o);  }
763 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::workthings::work_passes(g, a1, a2); r.w(&mut o);  }
764 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let r = crate::workthings::work_coming(g, a1, a2); r.w(&mut o);  }
765 => { let a1: CharId = Io::r(&mut i); let r = crate::workthings::work_job(g, a1); r.w(&mut o);  }
766 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<LightId> = Io::r(&mut i); let r = crate::lighting::lens_clear(g, a1, a2, a3, a4); r.w(&mut o);  }
768 => { let a1: GuardId = Io::r(&mut i); let r = crate::lighting::breaker_reset_near(g, a1); r.w(&mut o);  }
769 => { let a1: f64 = Io::r(&mut i); crate::lighting::update_targets(g, a1);  }
770 => { let a1: f64 = Io::r(&mut i); let a2: crate::lighting::SetBlackoutCause = Io::r(&mut i); crate::lighting::set_blackout(g, a1, a2);  }
771 => {  crate::lighting::end_blackout(g);  }
772 => { let a1: FixtureId = Io::r(&mut i); let a2: Option<crate::lighting::LightEventCause> = Io::r(&mut i); let a3: Option<V3> = Io::r(&mut i); crate::lighting::light_event(g, a1, a2, a3);  }
773 => { let a1: Option<FixtureId> = Io::r(&mut i); crate::lighting::ocp(g, a1);  }
774 => { let a1: bool = Io::r(&mut i); crate::lighting::set_daylight(g, a1);  }
775 => {  crate::lighting::repair_lights(g);  }
776 => { let a1: Option<crate::weather_kinds::WeatherKind> = Io::r(&mut i); let r = crate::weather::weather_set_kind_2(g, a1); r.w(&mut o);  }
778 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::weather::weather_under_roof(g, a1, a2); r.w(&mut o);  }
779 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::weather::weather_rain_bed(g, a1, a2); r.w(&mut o);  }
780 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::weather::weather_air_trans(g, a1, a2); r.w(&mut o);  }
782 => { let a1: crate::weather::WeatherSnap = Io::r(&mut i); crate::weather::weather_restore(g, &a1);  }
783 => { let a1: bool = Io::r(&mut i); crate::weather::weather_set_enabled_2(g, a1);  }
784 => { let a1: Option<crate::weather::WeatherStrikeOpts> = Io::r(&mut i); let r = crate::weather::weather_strike(g, a1); r.w(&mut o);  }
785 => { let a1: f64 = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); crate::weather::weather_update(g, a1, a2);  }
786 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let r = crate::weather::hear_path(g, a1, a2, a3); r.w(&mut o);  }
787 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::weather::air_trans(g, a1, a2); r.w(&mut o);  }
788 => { let a1: f64 = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); crate::hours::set_sun_strength(g, a1, a2);  }
789 => {  crate::hours::hour_weather_changed(g);  }
790 => {  crate::hours::sun_to_cloud(g);  }
791 => { let a1: crate::hours::HourId = Io::r(&mut i); let r = crate::hours::apply_hour(g, a1); r.w(&mut o);  }
792 => {  crate::hours::hour_world_changed(g);  }
793 => { let a1: V3 = Io::r(&mut i); let mut a2: Vec<f64> = Io::r(&mut i); crate::hours::hour_light(g, a1, &mut a2); a2.w(&mut o); }
794 => { let a1: V3 = Io::r(&mut i); let mut a2: Vec<f64> = Io::r(&mut i); crate::hours::moon_light(g, a1, &mut a2); a2.w(&mut o); }
795 => { let a1: V3 = Io::r(&mut i); let r = crate::hours::hour_light_at(g, a1); r.w(&mut o);  }
796 => { let a1: crate::scene_scenario_list::ScenarioSpec = Io::r(&mut i); let r = crate::scenario_run::start_scenario(g, &a1); r.w(&mut o);  }
797 => { let a1: Option<bool> = Io::r(&mut i); let r = crate::scenario_run::quit_scenario(g, a1); r.w(&mut o);  }
800 => { let a1: crate::chronicle::ChronKind = Io::r(&mut i); let a2: Option<JsMap<String, Option<crate::chronicle::ChronicleDataValue>>> = Io::r(&mut i); let a3: Option<V3> = Io::r(&mut i); let r = crate::chronicle::chronicle(g, a1, a2.as_ref(), a3); r.w(&mut o);  }
801 => { let a1: GuardId = Io::r(&mut i); let a2: crate::game::DownKind = Io::r(&mut i); let a3: Option<String> = Io::r(&mut i); let r = crate::chronicle::chron_down(g, a1, a2, a3.as_deref()); r.w(&mut o);  }
802 => { let a1: Option<String> = Io::r(&mut i); let r = crate::chronicle::chron_hit(g, a1.as_deref()); r.w(&mut o);  }
803 => { let a1: crate::chronicle::ChronDoorKind = Io::r(&mut i); let a2: DoorId = Io::r(&mut i); let r = crate::chronicle::chron_door(g, a1, a2); r.w(&mut o);  }
805 => { let a1: crate::mission::TallyK = Io::r(&mut i); crate::mission::tally(g, a1);  }
806 => { let a1: V3 = Io::r(&mut i); let r = crate::mission::rack_dist(g, a1); r.w(&mut o);  }
807 => { let a1: f64 = Io::r(&mut i); let a2: InputId = Io::r(&mut i); crate::mission::update_mission(g, a1, a2);  }
808 => {  crate::mission::drive_pulled(g);  }
809 => {  crate::mission::send_rack_check(g);  }
812 => { let a1: f64 = Io::r(&mut i); let r = crate::kit::kit_default(g, a1); r.w(&mut o);  }
813 => { let a1: Option<JsMap<String, crate::kit::KitValue>> = Io::r(&mut i); crate::kit::kit_stealth_set(g, a1.as_ref());  }
814 => { let a1: Option<crate::kit::KitHand> = Io::r(&mut i); crate::kit::kit_mission(g, a1);  }
815 => { let a1: Option<f64> = Io::r(&mut i); let a2: Option<Vec<crate::kit::MemberKit>> = Io::r(&mut i); let r = crate::kit::kit_worn(g, a1, a2.as_deref()); r.w(&mut o);  }
816 => { let a1: Option<bool> = Io::r(&mut i); let r = crate::kit::kit_own(g, a1); r.w(&mut o);  }
817 => { let a1: f64 = Io::r(&mut i); let r = crate::kit::kit_shown(g, a1); r.w(&mut o);  }
818 => { let a1: f64 = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::kit::kit_issued_shown(g, a1, a2); r.w(&mut o);  }
819 => { let a1: f64 = Io::r(&mut i); let a2: Option<crate::kit::MemberKit> = Io::r(&mut i); let r = crate::kit::kit_cost(g, a1, a2.as_ref()); r.w(&mut o);  }
820 => { let a1: Option<f64> = Io::r(&mut i); let r = crate::kit::kit_team(g, a1); r.w(&mut o);  }
821 => { let a1: String = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::kit::kit_loadout(g, &a1, a2); r.w(&mut o);  }
822 => { let a1: Option<f64> = Io::r(&mut i); let r = crate::kit::kit_loadouts(g, a1); r.w(&mut o);  }
823 => { let a1: Option<f64> = Io::r(&mut i); let r = crate::kit::kit_loadout_of(g, a1); r.w(&mut o);  }
824 => { let a1: f64 = Io::r(&mut i); let r = crate::kit::kit_rows(g, a1); r.w(&mut o);  }
825 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitRow = Io::r(&mut i); let a3: Option<crate::kit::MemberKit> = Io::r(&mut i); let r = crate::kit::kit_row_cost(g, a1, &a2, a3.as_ref()); r.w(&mut o);  }
826 => { let a1: f64 = Io::r(&mut i); let a2: Option<crate::kit::MemberKit> = Io::r(&mut i); let r = crate::kit::kit_words(g, a1, a2.as_ref()); r.w(&mut o);  }
827 => { let a1: f64 = Io::r(&mut i); let r = crate::kit::kit_changed(g, a1); r.w(&mut o);  }
828 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitField = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::kit::kit_peek(g, a1, a2, a3); r.w(&mut o);  }
829 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitField = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::kit::kit_try(g, a1, a2, a3); r.w(&mut o);  }
830 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitField = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::kit::kit_step(g, a1, a2, a3); r.w(&mut o);  }
831 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitField = Io::r(&mut i); let a3: crate::kit::KitValue = Io::r(&mut i); let r = crate::kit::kit_set(g, a1, a2, &a3); r.w(&mut o);  }
832 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitPreset = Io::r(&mut i); let r = crate::kit::kit_preset_card(g, a1, a2); r.w(&mut o);  }
833 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitPreset = Io::r(&mut i); let r = crate::kit::kit_preset(g, a1, a2); r.w(&mut o);  }
834 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::KitPreset = Io::r(&mut i); let r = crate::kit::kit_preset_why(g, a1, a2); r.w(&mut o);  }
835 => { let a1: crate::kit::KitPreset = Io::r(&mut i); crate::kit::kit_preset_all(g, a1);  }
836 => { let a1: f64 = Io::r(&mut i); let r = crate::kit::kit_preset_of(g, a1); r.w(&mut o);  }
837 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::MemberKit = Io::r(&mut i); let r = crate::kit::kit_paste_card(g, a1, &a2); r.w(&mut o);  }
838 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::MemberKit = Io::r(&mut i); let r = crate::kit::kit_paste_peek(g, a1, &a2); r.w(&mut o);  }
839 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::MemberKit = Io::r(&mut i); let r = crate::kit::kit_paste(g, a1, &a2); r.w(&mut o);  }
840 => { let a1: f64 = Io::r(&mut i); let a2: crate::kit::MemberKit = Io::r(&mut i); let r = crate::kit::kit_wears(g, a1, &a2); r.w(&mut o);  }
841 => { let a1: f64 = Io::r(&mut i); let a2: Option<JsMap<String, crate::kit::KitValue>> = Io::r(&mut i); let r = crate::kit::kit_sanitize(g, a1, a2.as_ref()); r.w(&mut o);  }
842 => { let a1: Option<bool> = Io::r(&mut i); crate::kit::kit_issued_page(g, a1);  }
843 => { let a1: Option<crate::kit::KitLessonPageRow> = Io::r(&mut i); crate::kit::kit_lesson_page(g, a1);  }
844 => {  crate::kit::kit_reset(g);  }
845 => {  crate::kit::kit_player(g);  }
846 => {  crate::kit::kit_solo(g);  }
847 => { let a1: bool = Io::r(&mut i); crate::kit::kit_solo_if(g, a1);  }
848 => { let a1: Option<bool> = Io::r(&mut i); crate::kit::kit_solo_stand(g, a1);  }
849 => {  crate::kit::kit_fresh(g);  }
850 => {  crate::kit::kit_show(g);  }
851 => { let a1: Vec<GuardId> = Io::r(&mut i); crate::kit::kit_stage(g, &a1);  }
852 => { let a1: f64 = Io::r(&mut i); let a2: Option<crate::kit::KitPoolIssued> = Io::r(&mut i); let r = crate::kit::kit_pool(g, a1, a2); r.w(&mut o);  }
854 => {  crate::fogofwar::fog_flags(g);  }
856 => {  crate::fogofwar::fog_of_war_page_look(g);  }
857 => { let a1: f64 = Io::r(&mut i); crate::fogofwar::fog_of_war_update(g, a1);  }
859 => {  crate::fogofwar::fog_of_war_reset(g);  }
864 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::fogofwar::fog_of_war_spot_seen(g, a1, a2); r.w(&mut o);  }
865 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::fogofwar::fog_of_war_ground_look(g, a1, a2); r.w(&mut o);  }
866 => { let a1: f64 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: f64 = Io::r(&mut i); let r = crate::fogofwar::fog_of_war_thing_seen(g, a1, a2, a3, a4); r.w(&mut o);  }
868 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); let r = crate::fogofwar::fog_of_war_things(g, &mut a1); r.w(&mut o); a1.w(&mut o); }
869 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::fogofwar::fog_of_war_beam_runs(g, a1, a2, a3); r.w(&mut o);  }
870 => { let a1: crate::game::GameEvent = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::guards::glass_heard(g, a1, a2); r.w(&mut o);  }
872 => { let a1: GuardId = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); crate::guards::hearing_check(g, a1, a2, a3, a4);  }
873 => {  let r = crate::guards::player_ground_vel(g); r.w(&mut o);  }
874 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::guards::motion_factor(g, a1, a2); r.w(&mut o);  }
875 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::guards::sight_gain(g, a1, a2); r.w(&mut o);  }
876 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::guards::sight_gain_on(g, a1, a2, a3); r.w(&mut o);  }
877 => { let a1: GuardId = Io::r(&mut i); let r = crate::guards::body_in_view(g, a1); r.w(&mut o);  }
878 => { let a1: GuardId = Io::r(&mut i); let a2: GuardId = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let r = crate::guards::body_seen(g, a1, a2, a3); r.w(&mut o);  }
879 => {  let r = crate::guards::body_light_summary(g); r.w(&mut o);  }
880 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); let a4: Option<f64> = Io::r(&mut i); let r = crate::guards::point_in_view(g, a1, a2, a3, a4); r.w(&mut o);  }
881 => { let a1: GuardId = Io::r(&mut i); crate::guards::reset_chase(g, a1);  }
883 => { let a1: CharId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::guards::clear_shot(g, a1, a2); r.w(&mut o);  }
886 => { let a1: GuardId = Io::r(&mut i); let a2: crate::guards::Pair = Io::r(&mut i); let r = crate::guards::flank_shot(g, a1, a2); r.w(&mut o);  }
887 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::guards::step_along_path(g, a1, a2, a3); r.w(&mut o);  }
888 => {  let r = crate::guards::standoff_summary(g); r.w(&mut o);  }
889 => { let a1: GuardId = Io::r(&mut i); let r = crate::guards::can_see(g, a1); r.w(&mut o);  }
890 => { let a1: GuardId = Io::r(&mut i); let a2: CharId = Io::r(&mut i); let a3: bool = Io::r(&mut i); let r = crate::guards::can_see_char(g, a1, a2, a3); r.w(&mut o);  }
891 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let r = crate::guards::eye_blocked(g, a1, a2, a3); r.w(&mut o);  }
892 => { let a1: crate::guards::Eye = Io::r(&mut i); let a2: CharId = Io::r(&mut i); let a3: bool = Io::r(&mut i); let a4: bool = Io::r(&mut i); let a5: f64 = Io::r(&mut i); let a6: Option<f64> = Io::r(&mut i); let r = crate::guards::eye_contact(g, a1, a2, a3, a4, a5, a6); r.w(&mut o);  }
893 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); crate::guards::go_to(g, a1, a2, a3);  }
894 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let r = crate::guards::beam_walled_at(g, a1, a2, a3, a4); r.w(&mut o);  }
895 => { let a1: V3 = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::guards::beam_ray_end(g, a1, a2, a3); r.w(&mut o);  }
896 => { let a1: GuardId = Io::r(&mut i); let r = crate::guards::beam_lens(g, a1); r.w(&mut o);  }
897 => { let a1: GuardId = Io::r(&mut i); let r = crate::guards::gun_on_lit_man(g, a1); r.w(&mut o);  }
898 => { let a1: GuardId = Io::r(&mut i); let r = crate::guards::team_lit(g, a1); r.w(&mut o);  }
899 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::guards::team_pace(g, a1, a2); r.w(&mut o);  }
900 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::guards::follow_path(g, a1, a2, a3); r.w(&mut o);  }
902 => { let a1: GuardId = Io::r(&mut i); let a2: Option<Vec<f32>> = Io::r(&mut i); crate::guards::score_spots(g, a1, a2.as_deref());  }
904 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::guards::update_guard(g, a1, a2);  }
905 => {  let r = crate::guards::steady_lamps(g); r.w(&mut o);  }
907 => { let a1: GuardId = Io::r(&mut i); crate::guards::gun_back(g, a1);  }
908 => { let a1: GuardId = Io::r(&mut i); crate::guards::start_reload_beat(g, a1);  }
909 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::guards::tick_reload_beat(g, a1, a2);  }
910 => { let a1: GuardId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: bool = Io::r(&mut i); let a5: bool = Io::r(&mut i); let a6: Option<GuardId> = Io::r(&mut i); crate::guards::burst_trigger(g, a1, a2, a3, a4, a5, a6);  }
911 => { let a1: GuardId = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); crate::guards::attach_flashlight(g, a1, a2);  }
912 => { let a1: GuardId = Io::r(&mut i); let a2: String = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); crate::guards::say(g, a1, &a2, a3, a4, a5);  }
913 => {  crate::guards::reset_guards(g);  }
914 => { let a1: f64 = Io::r(&mut i); let a2: Option<GuardId> = Io::r(&mut i); let a3: Option<crate::guards::RaiseEscalationCause> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); crate::guards::raise_escalation(g, a1, a2, a3, a4);  }
915 => {  crate::guards::stand_down_escalation(g);  }
917 => { let a1: Option<bool> = Io::r(&mut i); let a2: Option<crate::guards::PlanClearsCause> = Io::r(&mut i); let r = crate::guards::plan_clears(g, a1, a2); r.w(&mut o);  }
919 => { let a1: crate::guards::UnitOpts = Io::r(&mut i); match crate::guards::spawn_unit(g, &a1) { Ok(r) => { o.b.push(0); r.w(&mut o); } Err(e) => { o.b.push(1); e.w(&mut o); } }  }
920 => { let a1: GuardId = Io::r(&mut i); let r = crate::guards::despawn_unit(g, a1); r.w(&mut o);  }
921 => {  crate::guards::despawn_units(g);  }
922 => { let a1: V3 = Io::r(&mut i); let r = crate::dialogue::place_name(g, a1); r.w(&mut o);  }
923 => { let a1: V3 = Io::r(&mut i); let r = crate::dialogue::place_in(g, a1); r.w(&mut o);  }
924 => {  let r = crate::dialogue::place_rects(g); r.w(&mut o);  }
925 => { let a1: V3 = Io::r(&mut i); let r = crate::dialogue::room_name(g, a1); r.w(&mut o);  }
926 => { let a1: GuardId = Io::r(&mut i); let r = crate::dialogue::gather(g, a1); r.w(&mut o);  }
927 => { let mut a1: crate::dialogue::DialogueCtx = Io::r(&mut i); let a2: GameTalkId = Io::r(&mut i); let a3: bool = Io::r(&mut i); let mut a3_f = die_of(a3); let r = crate::dialogue::pick_line(g, &mut a1, a2, &mut a3_f); r.w(&mut o); a1.w(&mut o); }
928 => { let a1: GuardId = Io::r(&mut i); let a2: bool = Io::r(&mut i); let mut a2_f = die_of(a2); let r = crate::dialogue::interrogate(g, a1, if a2 { Some(&mut a2_f as &mut dyn FnMut(&mut crate::game::Game) -> f64) } else { None }); r.w(&mut o);  }
929 => { let a1: GuardId = Io::r(&mut i); let r = crate::dialogue::dialogue_summary(g, a1); r.w(&mut o);  }
930 => { let mut a1: crate::dialogue::DialogueCtx = Io::r(&mut i); let a2: GameTalkId = Io::r(&mut i); let r = crate::dialogue::peek_give(g, &mut a1, a2); r.w(&mut o); a1.w(&mut o); }
931 => { let mut a1: crate::dialogue::DialogueCtx = Io::r(&mut i); let a2: GameTalkId = Io::r(&mut i); let a3: crate::character::HoldVariant = Io::r(&mut i); let mut a4: Option<crate::dialogue::SamMemory> = Io::r(&mut i); let a5: bool = Io::r(&mut i); let mut a5_f = die_of(a5); let r = crate::dialogue::pick_sam_line(g, &mut a1, a2, a3, a4.as_mut(), &mut a5_f); r.w(&mut o); a1.w(&mut o); a4.w(&mut o); }
932 => { let a1: GameTalkId = Io::r(&mut i); let a2: Option<String> = Io::r(&mut i); crate::dialogue::force_script(g, a1, a2.as_deref());  }
933 => { let a1: crate::dialogue::HandlerBeat = Io::r(&mut i); let a2: Option<f64> = Io::r(&mut i); let r = crate::dialogue::handler_beat(g, a1, a2); r.w(&mut o);  }
934 => {  crate::civilians::reset_civilians(g);  }
935 => { let a1: crate::civilians::SpawnHostageDef = Io::r(&mut i); let r = crate::civilians::spawn_hostage(g, &a1); r.w(&mut o);  }
937 => {  let r = crate::civilians::civilian_comply(g); r.w(&mut o);  }
939 => { let a1: bool = Io::r(&mut i); crate::civilians::set_civilians(g, a1);  }
940 => { let a1: Option<GuardId> = Io::r(&mut i); let a2: Option<crate::civilians::CivPanicWhat> = Io::r(&mut i); let r = crate::civilians::civ_panic(g, a1, a2); r.w(&mut o);  }
941 => {  let r = crate::civilians::check_civ_spots(g); r.w(&mut o);  }
943 => { let a1: crate::orders::OrderTarget = Io::r(&mut i); let r = crate::planmode::goal_men(g, &a1); r.w(&mut o);  }
944 => { let a1: crate::orders::OrderTarget = Io::r(&mut i); crate::planmode::plan_goal(g, &a1);  }
954 => { let a1: String = Io::r(&mut i); let a2: Option<crate::orders::OrderTarget> = Io::r(&mut i); let a3: Option<crate::orders::Who> = Io::r(&mut i); let a4: Option<crate::orders::GiveOpts> = Io::r(&mut i); let r = crate::orders::squad_orders_give(g, &a1, a2.as_ref(), a3, a4.as_ref()); r.w(&mut o);  }
956 => { let a1: GuardId = Io::r(&mut i); let a2: crate::orders::SquadOrdersAssignA = Io::r(&mut i); crate::orders::squad_orders_assign(g, a1, &a2);  }
957 => {  crate::orders::squad_orders_loaded(g);  }
963 => { let a1: Option<f64> = Io::r(&mut i); let r = crate::orders::squad_orders_pick(g, a1); r.w(&mut o);  }
967 => { let a1: Vec<GuardId> = Io::r(&mut i); let a2: crate::orders::OrderTarget = Io::r(&mut i); let r = crate::orders::squad_orders_quick_for(g, &a1, &a2); r.w(&mut o);  }
973 => { let a1: V3 = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let r = crate::orders::squad_orders_spots_at(g, a1, a2, a3); r.w(&mut o);  }
974 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: Vec<GuardId> = Io::r(&mut i); let a4: Option<Vec<GuardId>> = Io::r(&mut i); let r = crate::orders::squad_orders_stack_posts(g, a1, a2, &a3, a4.as_deref()); r.w(&mut o);  }
977 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let r = crate::orders::squad_orders_glass_eyes(g, a1, a2, a3, a4); r.w(&mut o);  }
984 => { let a1: GuardId = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<DoorId> = Io::r(&mut i); let a4: Option<DoorId> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); let r = crate::orders::squad_orders_route(g, a1, a2, a3, a4, a5); r.w(&mut o);  }
989 => { let a1: GuardId = Io::r(&mut i); let a2: crate::orders::OrderTarget = Io::r(&mut i); let r = crate::orders::squad_orders_has_line(g, a1, &a2); r.w(&mut o);  }
994 => { let a1: f64 = Io::r(&mut i); let r = crate::seats::seats_stand(g, a1); r.w(&mut o);  }
995 => {  let r = crate::seats::deal_seats(g); r.w(&mut o);  }
996 => { let a1: f64 = Io::r(&mut i); let r = crate::wallcharge::sky_faces(g, a1); r.w(&mut o);  }
997 => { let a1: f64 = Io::r(&mut i); let r = crate::wallcharge::outer_wall(g, a1); r.w(&mut o);  }
998 => { let a1: crate::wallcharge::WallSpot = Io::r(&mut i); let r = crate::wallcharge::charge_indoors(g, &a1); r.w(&mut o);  }
999 => { let a1: crate::wallcharge::WallSpot = Io::r(&mut i); let r = crate::wallcharge::safe_radius(g, &a1); r.w(&mut o);  }
1000 => { let a1: V3 = Io::r(&mut i); let a2: Option<V3> = Io::r(&mut i); let r = crate::wallcharge::wall_at(g, a1, a2); r.w(&mut o);  }
1001 => { let a1: V3 = Io::r(&mut i); let a2: crate::orders::OrderTarget = Io::r(&mut i); crate::scene_collision::stands(g); let mut k = Vec::new(); let r = crate::wallcharge::wall_target(g, &crate::seats::here(g), a1, &a2, &mut k); crate::wallcharge::sky_kept(g, k); r.w(&mut o);  }
1002 => {  crate::wallcharge::wall_charge_reset(g);  }
1003 => { let a1: Option<f64> = Io::r(&mut i); crate::wallcharge::wall_charge_set_carried(g, a1);  }
1004 => { let a1: f64 = Io::r(&mut i); crate::wallcharge::wall_charge_raise_carried(g, a1);  }
1005 => { let a1: crate::orders::OrderTarget = Io::r(&mut i); let r = crate::wallcharge::wall_charge_spot(g, &a1); r.w(&mut o);  }
1006 => { let a1: crate::wallcharge::WallSpot = Io::r(&mut i); let r = crate::wallcharge::wall_charge_open_at(g, &a1); r.w(&mut o);  }
1010 => { let a1: crate::wallcharge::WallSpot = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let a3: Option<crate::wallcharge::WallChargeAwayMarksValueR> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let r = crate::wallcharge::wall_charge_blast_share(g, &a1, a2, a3, a4); r.w(&mut o);  }
1011 => { let a1: Option<WallChargeId> = Io::r(&mut i); let r = crate::wallcharge::wall_charge_too_close(g, a1); r.w(&mut o);  }
1016 => { let a1: Option<WallChargeId> = Io::r(&mut i); let a2: Option<bool> = Io::r(&mut i); let r = crate::wallcharge::wall_charge_blow(g, a1, a2); r.w(&mut o);  }
1017 => { let a1: crate::wallcharge::WallSpot = Io::r(&mut i); let r = crate::wallcharge::wall_charge_hole_at(g, &a1); r.w(&mut o);  }
1019 => { let a1: CharId = Io::r(&mut i); let a2: crate::weapons::WeaponDef = Io::r(&mut i); crate::particle_fx::casing_fx(g, a1, &a2);  }
1021 => { let a1: crate::particle_fx::TestBurstKind = Io::r(&mut i); let a2: V3 = Io::r(&mut i); let r = crate::particle_fx::test_burst(g, a1, a2); r.w(&mut o);  }
1028 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::doorswing::leaf_cut(g, a1, a2); r.w(&mut o);  }
1029 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::doorswing::leaf_swings(g, a1, a2); r.w(&mut o);  }
1030 => {  let r = crate::doorswing::deal_swings(g); r.w(&mut o);  }
1031 => { let a1: Option<FirearmId> = Io::r(&mut i); let a2: crate::weapons::WeaponDef = Io::r(&mut i); let a3: V3 = Io::r(&mut i); let a4: V3 = Io::r(&mut i); let a5: f64 = Io::r(&mut i); let a6: crate::tracers::TracerImpact = Io::r(&mut i); let a7: f64 = Io::r(&mut i); let a8: Option<bool> = Io::r(&mut i); let r = crate::tracers::tracers_round(g, a1, &a2, a3, a4, a5, a6, a7, a8); r.w(&mut o);  }
1032 => { let a1: f64 = Io::r(&mut i); let mut a2: Vec<AnyBoxId> = Io::r(&mut i); let a3: Option<f64> = Io::r(&mut i); crate::tracers::tracers_update(g, a1, &mut a2, a3); a2.w(&mut o); }
1033 => {  crate::tracers::tracers_clear(g);  }
1036 => { let a1: V3 = Io::r(&mut i); let r = crate::cleared::cleared_rooms_room_at(g, a1); r.w(&mut o);  }
1037 => { let a1: V3 = Io::r(&mut i); let r = crate::cleared::cleared_rooms_known_at(g, a1); r.w(&mut o);  }
1038 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::cleared::cleared_rooms_room_by(g, a1, a2); r.w(&mut o);  }
1040 => { let a1: V3 = Io::r(&mut i); crate::cleared::cleared_rooms_mark(g, a1);  }
1042 => {  let r = crate::cleared::cleared_rooms_summary(g); r.w(&mut o);  }
1043 => { let a1: DoorId = Io::r(&mut i); let a2: f64 = Io::r(&mut i); let r = crate::shutdoor::shut_spot(g, a1, a2); r.w(&mut o);  }
1044 => { let mut a1: Vec<AnyBoxId> = Io::r(&mut i); let a2: f64 = Io::r(&mut i); crate::fixture_break::fixture_break_boxes(g, &mut a1, a2); a1.w(&mut o); }
1045 => { let a1: f64 = Io::r(&mut i); crate::moths::moths_update(g, a1);  }
1048 => {  crate::game::game_rehand(g);  }
1049 => { let a1: GuardId = Io::r(&mut i); let a2: String = Io::r(&mut i); let a3: Option<bool> = Io::r(&mut i); let a4: Option<bool> = Io::r(&mut i); let a5: Option<bool> = Io::r(&mut i); crate::game::game_say(g, a1, &a2, a3, a4, a5);  }
1050 => { let a1: String = Io::r(&mut i); crate::game::game_note(g, &a1);  }
1051 => {  crate::game::game_reset_guards(g);  }
1052 => {  crate::game::game_escalate(g);  }
1053 => {  crate::game::game_stand_down(g);  }
1055 => {  let r = crate::game::game_body_light_summary(g); r.w(&mut o);  }
1056 => { let a1: GuardId = Io::r(&mut i); let a2: bool = Io::r(&mut i); let mut a2_f = die_of(a2); let r = crate::game::game_interrogate(g, a1, if a2 { Some(&mut a2_f as &mut dyn FnMut(&mut crate::game::Game) -> f64) } else { None }); r.w(&mut o);  }
1057 => { let a1: crate::game::GameFreeHeldWhy = Io::r(&mut i); crate::game::game_free_held(g, a1);  }
1058 => { let a1: bool = Io::r(&mut i); let mut a1_f = die_of(a1); let r = crate::game::game_interrogate_nearest(g, if a1 { Some(&mut a1_f as &mut dyn FnMut(&mut crate::game::Game) -> f64) } else { None }); r.w(&mut o);  }
1059 => {  let r = crate::game::game_dialogue_summary(g); r.w(&mut o);  }
1060 => {  crate::game::game_clear_nearest_room(g);  }
1061 => { let a1: V3 = Io::r(&mut i); crate::game::game_teleport_player(g, a1);  }
1063 => {  crate::game::game_repair_lights(g);  }
1064 => {  crate::game::game_clear_aftermath(g);  }
1065 => { let a1: bool = Io::r(&mut i); crate::game::game_set_loadout(g, a1);  }
1066 => {  crate::game::game_stand_lights(g);  }
1067 => {  crate::game::game_reset_level_state(g);  }
1068 => {  let r = crate::game::game_crack_test(g); r.w(&mut o);  }
1069 => {  let r = crate::game::game_wet_summary(g); r.w(&mut o);  }
1070 => { let a1: Option<String> = Io::r(&mut i); crate::game::game_restart(g, a1.as_deref());  }
1071 => {  crate::game::game_restart_encounter(g);  }
1072 => { let a1: f64 = Io::r(&mut i); let a2: crate::game::GameSetBlackoutCause = Io::r(&mut i); crate::game::game_set_blackout(g, a1, a2);  }
1073 => {  crate::game::game_end_blackout(g);  }
1074 => { let a1: FixtureId = Io::r(&mut i); let a2: Option<crate::game::GameLightEventCause> = Io::r(&mut i); let a3: Option<V3> = Io::r(&mut i); crate::game::game_light_event(g, a1, a2, a3);  }
1075 => { let a1: Option<FixtureId> = Io::r(&mut i); crate::game::game_ocp(g, a1);  }
1076 => { let a1: Option<GuardId> = Io::r(&mut i); crate::game::game_toggle_drag(g, a1);  }
1078 => { let a1: InputId = Io::r(&mut i); let a2: CameraId = Io::r(&mut i); let a3: crate::host_hud::View = Io::r(&mut i); crate::game::game_host_after(g, a1, a2, a3);  }
1081 => { let a1: crate::game::InteractableKind = Io::r(&mut i); let a2: Option<crate::game::Verb> = Io::r(&mut i); let r = crate::game::game_select_verb(g, a1, a2); r.w(&mut o);  }
1082 => {  let r = crate::game::game_escape(g); r.w(&mut o);  }
1083 => { let a1: crate::game::ThrowKind = Io::r(&mut i); let a2: V3 = Io::r(&mut i); crate::game::game_drop_throwable(g, a1, a2);  }
1087 => { let a1: FirearmId = Io::r(&mut i); let r = crate::weapons::firearm_fire(g, a1); r.w(&mut o);  }
1089 => { let a1: FirearmId = Io::r(&mut i); crate::weapons::firearm_cancel_reload(g, a1);  }
1090 => { let a1: FirearmId = Io::r(&mut i); crate::weapons::firearm_rearm(g, a1);  }
1091 => { let a1: FirearmId = Io::r(&mut i); crate::weapons::firearm_fit(g, a1);  }
1093 => { let a1: FirearmId = Io::r(&mut i); let r = crate::weapons::rifle_cycle_attachment(g, a1); r.w(&mut o);  }
1094 => { let a1: SeatId = Io::r(&mut i); let a2: Vec<crate::cmd::PlayerCmd> = Io::r(&mut i); let a3: f64 = Io::r(&mut i); let mut a4: Option<Vec<V3>> = Io::r(&mut i); let r = crate::walkahead::walk_ahead(g, a1, &a2, a3, a4.as_mut()); r.w(&mut o); a4.w(&mut o); }
1095 => { let a1: Option<bool> = Io::r(&mut i); let r = crate::doortools::door_tools_together(g, a1); r.w(&mut o);  }
1096 => { let a1: f64 = Io::r(&mut i); let a2: Vec<f64> = Io::r(&mut i); let r = crate::sense::sense_tool(g, a1, &a2); r.w(&mut o);  }
_ => panic!("bridge: no call of row {}", f),
} assert!(i.p == args.len(), "bridge: {} bytes left over in the call of row {}", args.len() - i.p, f); o.b }