pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{goals_on, goal_parts, goal_men, goal_ring_on, plan_goal, goal_shift, goal_input};
