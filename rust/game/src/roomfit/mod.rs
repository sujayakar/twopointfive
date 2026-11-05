pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{compass, way_words, run_words, off_line, threats_of, sees, watchables_at, in_funnel, fit_spots, spare_run, fit_groups, take, fit_tick};
