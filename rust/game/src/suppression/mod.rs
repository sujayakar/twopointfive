pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{sup_clear_reactions, keen_of, note_round, forget_suppression_clocks, watch_your_fire, sup_bark, suppression_lines, reacts, update_suppression, head_down, end_displace, holding_cover, cover_from, guard_fire_mods, flinching, sam_spread, suppression_summary};
