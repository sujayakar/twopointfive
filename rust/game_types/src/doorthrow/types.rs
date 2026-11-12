//! The types of `doorthrow`.

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DoorthrowStats {
    pub startled: i32,
    pub pushed: i32,
    pub kicked: i32,
    pub carried: i32,
}
