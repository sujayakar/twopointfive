pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{cut_boxes_on, cut_plane, cut_through, cut_support, cut_verts, cut_record, cut_bounds, cut_fit, cut_to_world, cut_prism, cut_clip};
