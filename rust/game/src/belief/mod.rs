pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{duties_anew, cover, uncover, fix_at_sight, fix_is_sight, eyes_fix, eyes_fix_def, gun_point, word, word_stands, word_now};
