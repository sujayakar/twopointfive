use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// For the check and a colleague's shots: one man's idea of one enemy (a copy: no caller writes to it).
pub fn idea_of(g: &crate::game::Game, gd: GuardId, who: crate::picture::Who) -> Option<crate::picture::Idea> {
    g.m.picture.states.as_ref()?.men.get(gd)?.ideas.get(who).copied()
}
