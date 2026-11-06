pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{sam_laser_boxes, sam_laser_on, ir_note, ir_viewer, ir_view_as, ir_made, laser_band, laser_on, paints_own};
