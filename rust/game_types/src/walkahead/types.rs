//! The types of `walkahead`.
use crate::V3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ahead {
    pub pos: V3,
    pub aim_yaw: f64,
    pub follow: V3,
    pub aim: Option<V3>,
    pub free: bool,
}
