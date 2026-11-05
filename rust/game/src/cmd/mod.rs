pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{sample_cmd, cmd_text, cmd_from_text, num_word, word_num, check_cmd, check_cmd_in, still_cmd, honest_copy, load_lens, seat_new, seat_man, seat_load, seat_hand, seat_handed, seat_swallow, seat_take_cut};
