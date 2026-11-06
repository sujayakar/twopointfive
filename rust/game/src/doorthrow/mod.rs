pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{side_of, man_of, door_man, team_of, door_throw_after, count, startle_man, throw_man, throw_sam};
