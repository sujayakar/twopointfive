pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{record, fin, refuse, keep, say_paint, made_of, drawn_as, paint_of, word_for};
