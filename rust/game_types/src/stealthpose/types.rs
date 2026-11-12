//! The types of `stealthpose`.
use crate::bytes::ToBytes;

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StealthFeed {
    pub free: bool,
    pub cable: bool,
}
