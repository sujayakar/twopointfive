pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{lesson_level, plain_fog, quiet_like, quiet_cams_of_link, quiet_rung_of_query, same_fog, fog_of_query, fog_of_link, start_scenario, start_from_query, refusal_words, active_scenario, team_of, running_scenario, quit_scenario};
