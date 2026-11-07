pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{no, bump, chew_live, live, state_of, man_of, flat, char_of, up_of, name_of, log, weight_of, see, heard, known, ask, cover_lane, worth, budget_of, stays, choose, cover_seen, cover_blind, cover_op, coverfire_tick, coverfire_report, coverfire_saw, coverfire_on, coverfire_reset};
