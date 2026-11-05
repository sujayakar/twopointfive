pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{response_of, raid, raid_over, side_of_team, callers, raid_voice, response_up, response_team, marks_of, room_for, budget_of, most_men, qrf_owns, qrf_summary, own_drill_asked, stored_tune, load_qrf_tune, save_qrf_tune, reset_qrf_tune, flat, note, control, control_row, door_said, radio, staff_voice, bodies_found, live, has_response, response_here, alive, best_fix, on_their_net, net_rule, read_net, net_word, fix_of, brief, reset_qrf, qrf_stand_down, request_qrf, clear_of_foes, response_words};
pub mod p_02;
pub use p_02::{dealt, silent_net, update_qrf, arriving, re_deal, deploy, fling_open, tick_men, perceive_scripted, man_down, phase_file, sector_point, pick_target, sweep_next, sweep_places, check_response, path_len, nearest_free, approach_end, start_move, aim_of, station_of, phase_move};
pub mod p_03;
pub use p_03::{trail_point, trail_dir, hold360, stack_marks_for, start_stack, phase_stack, phase_breach, their_bang, phase_bang, start_flow, flow_legs, phase_flow, start_drill, drop_drill, phase_drill, to_fight, phase_fight, route, advance, maybe_perimeter, call_perimeter, to_post, tick_perimeter};
