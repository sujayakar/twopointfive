pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub use b_01::{firearm_apply, own_charges_back, load_state, apply_state, save_store_remove, mem_shelf_set, mem_shelf_clear, checkpoints_note, checkpoints_adopt, checkpoints_prime, checkpoints_request, checkpoints_tick, checkpoints_mark_regret, checkpoints_keep};
