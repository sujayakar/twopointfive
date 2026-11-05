pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{new_stats, squad_net_new, squad_net_reset, squad_net_line, why_words, room_call, plain_call, in_flood};
