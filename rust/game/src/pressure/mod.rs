pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{hooked, new_stats, state_of, way_out, forms, flat, pick, alive, note, cell_of, eye_of, target, picket_hears, picket_marks, post_picket, press_owns, press_wants, press_keeps, stays_home, gather_spot};
pub mod p_02;
pub use p_02::{stand_off_spot, run, charge, rooms_by, in_room, team_stands, last_alone, team_placed, heard_at, learn, loose, boots, team_boots, hears_team, watch_door, told_of, next_door, consider, drop, tick_press, last_man, tick_out, charges, fight_on, awaiting, stood_to, ready_man};
pub mod p_03;
pub use p_03::{report_stands, aim, room_for, stand_to, stand_and_fight, go_look, press_tick, press_stop, look_since, pressure_stats, pressure_summary, load_pressure, reset_pressure, save_pressure};
