pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{gun_hold_stats, live, flat, men_on, gun_hold_see, learn_shot, part_of, gun_hold_keeps, gun_hold_runs, gun_hold_free, expo, cell_of, centre, faces_him, front, shown, price, way_to, take, pick_cover, plan_round, goes_round, gun_hold_pool, bark, gun_hold_feet, gun_hold_summary, gun_hold_phase};
