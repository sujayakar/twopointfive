pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{shorthand, shorthand_id, bare, door_names_of, authored_door_names, name_doors, door_written_name, door_said_name};
