pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{band_of, knowledge_of, agitation_of, agitation_terms, temperament_of, gives_at, token_worst, said_text, intel_table, drive_away, intel_text, shaped_intel_text, check_on_self, intel_lead, run_on, badge_of};
pub mod p_02;
pub use p_02::{place_name, is_office, with_the, level_rects, rect_dist, own_place, watched_place, own_name, place_in, way_name, place_rects, room_name, route_name, door_name, num_word, dist_to_route, keyed_door_on, keyed_door_on_route, eta_words, heard_on_net, missing_known_to, known_down, staff_count, partner_of, held_since_of, gather, tokens_in, expand, stage_key_of, giving_stage, line_matches, needs_hold, pending_intel, intel_for, swallowed, pick_line, interrogate, prompt_for, dialogue_summary};
pub mod p_03;
pub use p_03::{sam_memory_of, peek_give, sam_stage_of, sam_facts, sam_line_fits, choose_sam_line, pick_sam_line, sam_line_for, sam_key_line_for, sam_remark_for, synth_ctx, blocked_phrase, lint_dialogue, coverage_matrix};
pub mod p_04;
pub use p_04::{sam_coverage, script_coverage, line_length_report, force_script, forced_script_of, peek_script, script_by_id, script_for, script_fits, script_live, beat_for, him_of};
pub mod p_05;
pub use p_05::{register_script_lines, radio_state, radio_live, radio_clear, handler_beat, tick_radio, pair_worst};
pub mod p_06;
pub use p_06::{needs_lines, civilian_lines, hostage_lines, snatch_lines, ambient_state, ambient_live, chatty, still_chatty, end_ambient, ambient_restart, pair_said, update_ambient};
