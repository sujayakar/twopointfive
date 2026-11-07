pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{is_hostile, defender_words, spoken_name, staff_vs_operatives, team_of_char, team_of_id, hostile_to, foe_char, foe_down, foe_crouched};
