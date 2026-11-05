pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{cleared_rooms_new, cleared_rooms_reset, cleared_rooms_build, cleared_rooms_room_at, cleared_rooms_known_at, cleared_rooms_room_by, cleared_rooms_has_floor, cleared_rooms_rooms_of, cleared_rooms_is_known, cleared_rooms_unknown_eyes, cleared_rooms_mark, cleared_rooms_tick, cleared_rooms_dear_for, cleared_rooms_dear_only, cleared_rooms_dear_along, cleared_rooms_summary};
