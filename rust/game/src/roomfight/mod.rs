pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{drilled_on, drilled_k, flat, pick, eye_of, chest_of, pair_key, pair_smoke, lamp_glare, fight_live, close_lane, room_fight_sam_speed_of, room_fight_new, room_fight_on, room_fight_note, room_fight_engaged, room_fight_no_shot, room_fight_busy, room_fight_contact, room_fight_team_sees, room_fight_seen_by, room_fight_sight_on};
pub mod p_02;
pub use p_02::{room_fight_rate_on, room_fight_hot_for, room_fight_hot_of, room_fight_watch_of, room_fight_op_of, room_fight_short, room_fight_summary, room_fight_tick, room_fight_stop, room_fight_reset, room_fight_smoke, room_fight_room_at, room_fight_in_room, room_fight_free_near, room_fight_adopt, room_fight_tick_def, room_fight_evidence, room_fight_told, room_fight_entry_toward, room_fight_choose_watch, room_fight_apply_watch, room_fight_reactions, room_fight_blind, room_fight_drilled, room_fight_lost, room_fight_out_of_building, room_fight_threat_of, room_fight_bolt, room_fight_break_off, room_fight_tick_move, room_fight_stand_on, room_fight_lane_stop};
pub mod p_03;
pub use p_03::{room_fight_plant, room_fight_rally, room_fight_tick_free, room_fight_new_op, room_fight_friend_in_line, room_fight_sam_in_line, room_fight_speak, room_fight_cover_passes, room_fight_tick_op, room_fight_point, room_fight_cue, room_fight_door_shot, room_fight_hold_short, room_fight_let_go, room_fight_walk_on, room_fight_restore, room_fight_step_to, room_fight_hand_back, room_fight_funnel_of, room_fight_spot_free, room_fight_doorway, room_fight_side_step};
