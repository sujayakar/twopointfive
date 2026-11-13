//! The types of `workstow`.

#[derive(Clone, Copy, Debug, Default)]
pub struct StowFightA {
    pub yield_t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StowFightOp {
    pub stow_off_t: Option<f64>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WorkstowStats {
    pub mate_down: i32,
    pub mate_up: i32,
}
