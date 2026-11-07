pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{team_slow, walk_lead, team_walk, team_walk_of, quiet, is_run, team_leave, live, fighting, blast_run, team_cap, team_at_walk, run_why, team_ran, team_run_ago, team_run_legs, team_accel, team_sprinting, team_tread, team_follow, team_fight, team_dash_now, team_dash, team_told, team_blast_pace};
