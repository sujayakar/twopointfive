pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{stack_room_book_keep, book_of, centre, door_between, floor_of, steps_to, on_floor, stack_room_has, stack_room_keeps, stood_at, furnished, stack_room_fix, in_view, far_cells, stack_view_open, stack_view_reach, stack_view_sees, stack_view_fix, next_room, stack_room_probe, stack_room_summary};
