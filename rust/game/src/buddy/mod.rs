pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{new_stats, state_of, flat, pick, alive, note, cell_of, own, mates, team_known, spot_by, way_to, free, answerer, send, consider, tick_errand, nearest_entry, buddy_owns, buddy_tick, buddy_stop, buddy_stats, buddy_summary};
