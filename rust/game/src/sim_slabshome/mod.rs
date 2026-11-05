pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{slab_cols, slab_store_new, slab_store_n, slab_store_fly, slab_store_row_of, slab_store_door_at, slab_store_swap, slab_store_enter, slab_store_rest, slab_store_leave, slab_store_load, slab_store_save, slab_io, slab_diff};
