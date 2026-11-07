pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{can, aim_weighs, aim_top, aim_swing, aim_weigh_begin, aim_weigh_turn, aim_gun_point, aim_gun_on, aim_weigh_shot, aim_weigh_launch, aim_weigh_launch_keep, aim_weight_summary};
