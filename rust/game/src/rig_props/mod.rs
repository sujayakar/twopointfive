pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{assign_tree, clone_rig, assign_rig, clear_adjs, rig_source, frame_from, rot_axis, apply_adj, zero_adj, adj_of, wrist_tie_boxes, seg, seg_at, head_flat_forward, goggles_boxes};
pub mod p_02;
pub use p_02::{pistol_frame, mix_frame, holster_frame_of, long_look_of, model_grip, model_muzzle_ahead, sheath_frame, carbine_frame, carbine_grip, carbine_muzzle_ahead, long_grip_of, muzzle_ahead_of, sling_front_hand, fit_frame, mix_adj, kit_gun_frame, mag_box_l, rig_boxes, kit_boxes, pistol_as, model_col, long_model_boxes, pistol_look_of, model_in_hands, holster_boxes, knife_boxes, sheath_boxes, held_can_boxes, free_can_boxes, carbine_boxes, grip_adj};
pub mod p_03;
pub use p_03::{slung_base, sling_swing, sling_rest_want, sc20k_frame, bare_barrel, sc20k_boxes, rifle_m4, sc20k_grip, long_hold, torch_hold, sc20k_muzzle_ahead, sc20k_light_pose, sc20k_launcher_muzzle, carbine_light_pose, rig_light_pose, rig_torch_pos, rail_light_def, torch_light_def, torch_carry_dir, torch_carry_want, big_clear_of, pack_side_dims, pack_side_bind, pack_side, big_back_of, big_boxes, hung_boxes};
