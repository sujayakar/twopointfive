pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{load_tactics, save_tactics, reset_tactics, s, give_stuns, stuns_of, tactics_on, flat, bearing_of, deg_apart, free_man, in_contact, suppressed, path_len, sample_path, cells_of_path, sector_cells, entry_bearing, los_open, last_cover_point, point_short_of_end, walls_about, door_near, door_label, danger_doors, danger_at, mark_danger, tactics_reset, cleared_for, tactics_cleared, set_cleared, tactics_man_down, tactics_body_found, body_duty_blocked, scan_rounds, dist_to_segment, tactics_penalty, tactics_role_of, role_record, lone_record, gate_record, plans_of, flank_call, plan_approach, end_plan, fit_for_role};
pub mod p_02;
pub use p_02::{tick_plans, advance, kneel_toward, ready_to_go, flankers_running, go_now, arrive, move_role, presser_of, role_of_man, armed_intruder, alone, lone_corner, lone_call, move_lone, tactics_alert, tactics_tick, side_of, along_of, door_pt, plan_for, stack_spot, slice_stations, entry_legs, step_gate, stand_gate, gate_mate, gate_call, try_engage, door_gate, loose_can, release_gate};
pub mod p_03;
pub use p_03::{run_gate, away_look, begin_slice, begin_cross, begin_toss, tick_gates, tactics_look, tactics_summary};
