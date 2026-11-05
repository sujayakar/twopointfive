#![allow(unused_variables)]
//! Every function and method of `math_side`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/math/side.ts:18 named)
pub fn named(g: &mut crate::game::Game, t: &crate::math_side::Table, name: &str) { todo!() }

/// (src/math/side.ts:78 rawKey)
pub fn raw_key<K: Clone + core::fmt::Debug>(k: &K) -> K { todo!() }

/// (src/math/side.ts:110 rollbackVar)
pub fn rollback_var<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, name: &str, get: &mut dyn FnMut(&mut crate::game::Game) -> T, set: &mut dyn FnMut(&mut crate::game::Game, T)) { todo!() }

/// (src/math/side.ts:116 rollbackRoot)
pub fn rollback_root<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, name: &str, obj: &T) -> T { todo!() }

/// (src/math/side.ts:124 wallClock)
pub fn wall_clock<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, o: &T, keys: &[crate::math_side::WallClockKeys]) -> T { todo!() }

/// (src/math/side.ts:125 wallClockKeys)
pub fn wall_clock_keys(g: &mut crate::game::Game) -> Option<Vec<String>> { todo!() }

/// (src/math/side.ts:137 sideRegistry)
pub fn side_registry(g: &mut crate::game::Game) -> crate::math_side::SideRegistry { todo!() }
