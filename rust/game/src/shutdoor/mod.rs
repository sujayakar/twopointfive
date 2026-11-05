pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{flat, pt_of, handle_of, note, stands_open, in_shut_swing, walled, stands, shut_spot, spots_of, walk, from_file, plan, in_the_way, contact_through, why_not_shut, send, pull, finish, tick, register_shut_door};
