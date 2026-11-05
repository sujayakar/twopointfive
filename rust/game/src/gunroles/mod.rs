pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{use_on, gun_primary, gun_role, rank_of, line_of, gun_hand, gun_deal, gun_keeps_post, gun_file_cost, gun_file_ranks, gun_first, gun_before, gun_turn, gun_last, gun_blind_gap, gun_lane, gun_string, gun_string_gap, gun_string_grow};
