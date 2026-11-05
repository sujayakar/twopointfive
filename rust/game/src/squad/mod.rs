pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{flat, strafe_along_path, run_guard_script, end_guard_script, order, sweep_start, sweep_at, room_clear_new, room_clear_start, room_clear_end, room_clear_intact, room_clear_complete, room_clear_men, room_clear_once, room_clear_say, room_clear_body_here, room_clear_route, room_clear_advance, room_clear_ahead, room_clear_enter, room_clear_enter_sweep, room_clear_shove, room_clear_tick, rest_secs, room_centre, next_door_of, break_off};
pub mod p_02;
pub use p_02::{perceive, in_bounds, path_len, clearing_new, clearing_cap, clearing_name, clearing_summary, clearing_cancel, clearing_men, clearing_say, clearing_tick, clearing_muster, clearing_read_doors, clearing_approach, clearing_start_drill, clearing_next_room};
