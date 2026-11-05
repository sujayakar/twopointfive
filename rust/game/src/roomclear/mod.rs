pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{fresh_seed_of, fresh_way_of, place_defenders, check_structure, check_squads, door_sides, same_bounds, prop_gap, wall_between, door_plan, listed, clear_words, room_clear_scenario_structure, room_clear_scenario_way_now, room_clear_scenario_start_now, room_clear_scenario_start_at, room_clear_scenario_new, room_clear_scenario_active, room_clear_scenario_can_start, room_clear_scenario_start, room_clear_scenario_stop, room_clear_scenario_restage, room_clear_scenario_clear, room_clear_scenario_note, room_clear_scenario_deal, room_clear_scenario_drill_room, room_clear_scenario_on_staged, room_clear_scenario_tick, room_clear_scenario_adopt, room_clear_scenario_rehome, room_clear_scenario_finish, room_clear_scenario_state, room_clear_scenario_team, room_clear_scenario_room_at, room_clear_scenario_objective, room_clear_scenario_first_open, room_clear_scenario_summary};
