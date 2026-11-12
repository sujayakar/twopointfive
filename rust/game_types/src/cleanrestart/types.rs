//! The types of `cleanrestart`.

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CleanrestartStats {
    pub restarts: i32,
}
