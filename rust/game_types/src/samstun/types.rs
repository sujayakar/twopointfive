//! The types of `samstun`.

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SamstunStats {
    pub staggered: i32,
    pub dazzled: i32,
}
