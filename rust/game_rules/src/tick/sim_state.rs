pub use ::game_types::sim_state::types;
pub use types::*;

/// Where part `p` of a block starts, in bytes from the block's own start.
pub fn sim_part(p: f64) -> f64 {
    ::sim::state::PARTS[p as usize].off as f64
}
