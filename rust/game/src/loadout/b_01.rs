//! Bodies for skeleton stubs of loadout that translated code calls and no part had (the stub round).
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The words for a size: 'solo', 'you + one', 'you + the pair', 'you + three', 'you + four'. (loadout.ts sizeWords)
pub fn size_words(n: f64) -> String {
    String::from(if n >= 4.0 { "you + four" } else if n == 3.0 { "you + three" } else if n == 2.0 { "you + the pair" } else if n == 1.0 { "you + one" } else { "solo" })
}

