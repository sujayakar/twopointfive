pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{each_posed, man_singles, pose_home_tend};
