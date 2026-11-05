pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{mulberry32, die_of, roll, roll_on, hash_str, sim_die, sim_dice_new, sim_dice_s, sim_dice_n, use_dice, dice_in_use, sim_random};
