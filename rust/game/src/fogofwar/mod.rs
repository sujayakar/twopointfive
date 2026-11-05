pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{field_deg, field_near, gaze_of, fog_eye_count, fog_flags};
pub mod p_02;
pub use p_02::{thing_key_of, is_prop, ghost_box, fog_through, fog_of_war_show, fog_of_war_page_look, fog_of_war_new, fog_of_war_active, fog_of_war_on_team, fog_of_war_update, fog_of_war_eye_of, fog_of_war_caller, fog_of_war_side_of, fog_of_war_call_words, fog_of_war_tell_calls, fog_of_war_in_field, fog_of_war_flush_asks, fog_of_war_reset, fog_of_war_sight_blocked, fog_of_war_thin_block, fog_of_war_line_to_man, fog_of_war_sees, fog_of_war_hand_eyes, fog_of_war_sees_here, fog_of_war_eye_on_body, fog_of_war_eye_on, fog_of_war_door_seen, fog_of_war_door_held, fog_of_war_door_shown, fog_of_war_door_of_box, fog_of_war_door_boxes};
pub mod p_03;
pub use p_03::{fog_of_war_lamp_seen, fog_of_war_full_clear, fog_of_war_lamp_face, fog_of_war_face_seen, fog_of_war_faces_lift, fog_of_war_faces_back, fog_of_war_snapshot, fog_of_war_restore, fog_of_war_lamp_lit, fog_of_war_lamp_known, fog_of_war_full_block, fog_of_war_lamp_take, fog_of_war_lit_block, fog_of_war_spot_seen, fog_of_war_ground_look, fog_of_war_cell_seen_at, fog_of_war_thing_seen, fog_of_war_span, fog_of_war_sees_thing, fog_of_war_things, fog_of_war_beam_runs, fog_of_war_seen, fog_of_war_sighting, fog_of_war_drawn, fog_of_war_fired_on, fog_of_war_last_seen, fog_of_war_hidden, fog_of_war_ours_on, fog_of_war_ours_id, fog_of_war_light_by, fog_of_war_begin, fog_of_war_done, fog_of_war_doing, fog_of_war_hush, fog_of_war_air_hides, fog_of_war_filter, fog_of_war_everybody, fog_of_war_raw, fog_of_war_summary};
