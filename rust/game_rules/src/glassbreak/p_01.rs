//! src/game/glassbreak.ts lines 1 to 219
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the one gate every entry point takes first (glassbreak.ts glassBreakLive). GLASSFX.on is settled on.
pub fn glass_break_live(g: &crate::game::Game) -> bool {
    g.tune.glass_break && crate::destruction::chaos_of(g) > 0.0
}

