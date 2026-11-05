pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{live, fresh, side_at, off_at, pt_of, handle_at, flat, walled, stands, at_handle, face_of, handle_spot, find_face, ajar_of, wants_hand, ahead, release, pull_open, bend, halt, handle_gate, handle_held, gate, way_to, step_to, hand_press, tool_spot, near_stand, tool_step, latch_first};
