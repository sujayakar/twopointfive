pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{smoke_system_like_emit, smoke_system_like_push, smoke_system_like_keep_hot, smoke_system_like_clear_all, smoke_system_like_obstacles_changed, smoke_system_like_set_wind, smoke_system_like_set_stir};
