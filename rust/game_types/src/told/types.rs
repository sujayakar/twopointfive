//! The types of `told`.
use crate::{Words, bytes::ToBytes};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum ToldRung {
    #[default] Bare,
    Guided,
    Standard,
}

