pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{save_last_stand, new_stats, flat, alive, pick, cell_of, in_room, note, fit_gd, fit, calm, movable, where_, spoken, voice, fill, net, net_at, tick_later, name, name_at, tick_moved, team_known, spot_in, way_to, can_come, stand_room, home_room, the_of, choose, start_check, tick_check, lone, tell_only, new_stand, anchor_of, member, holds_own};
pub mod p_02;
pub use p_02::{decide, tick_mover, come_by, tick_stand, fresh, dissolve, last_stand_tick, last_stand_stop, last_stand_man, last_stand_owns, last_stand_hint, last_stand_stats, last_stand_report, last_stand_summary};
