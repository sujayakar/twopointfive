//! The types of `wallwear`.
use crate::{Blank, bytes::ToBytes};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallWear {
    #[is(true)] pub on: bool,
}

