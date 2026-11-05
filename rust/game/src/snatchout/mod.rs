pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{flat, who, snatch_out_new, snatch_out_reset, snatch_out_van, snatch_out_on, snatch_out_ruled, snatch_out_free, snatch_out_owns, snatch_out_planted, snatch_out_rows, snatch_out_word, snatch_out_tick, snatch_out_downed, snatch_out_switched, snatch_out_into, snatch_out_fighting, snatch_out_mover, snatch_out_mine, snatch_out_drop, snatch_out_rejoin, snatch_out_let_go, snatch_out_spot, snatch_out_plant, snatch_out_turn, snatch_out_bound, snatch_out_objective, snatch_out_rows_2};
