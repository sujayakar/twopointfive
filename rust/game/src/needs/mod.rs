pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{need_spot, own_spot, mem, needs_mem, needs_trip, needs_busy, floor, log, frac, path_metres, departure_point, deal, needs_live, floor_calm, man_calm, interrupt_of, needs_tick, schedule_trip, path_len, cancel_trip, pick_line, partner_for, announce, partner_reply, needs_patrol, needs_send, needs_set_level, save_needs, load_needs, reset_needs, needs_summary};
