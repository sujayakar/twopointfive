//! src/math/side.ts lines 1 to 145
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The game a key stands for. A view object does not exist in Rust (seats swap on `g`), so every key answers itself. (side.ts rawKey)
pub fn raw_key<K: Clone + core::fmt::Debug>(k: &K) -> K {
    k.clone()
}
