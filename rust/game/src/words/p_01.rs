//! src/game/words.ts lines 1 to 36
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The page's rule for the switch; all four rows are settled true on the page, so the address is not read.   (words.ts wordsFromQuery)
pub fn words_from_query(g: &mut crate::game::Game, q: crate::Opaque) {
    g.k.words.on = true;
    g.k.words.doors = true;
    g.k.words.radio_home = true;
    g.k.words.guide = true;
}

/// A line (or any value a line is picked by) as the dialogue pass has it; `on` is settled true on the page.   (words.ts w)
pub fn w<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, now: &T, was: &T) -> T { now.clone() }

/// A line as the writing guide has it; `on` and `guide` are settled true on the page.   (words.ts wg)
pub fn wg<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, now: &T, was: &T) -> T { now.clone() }
