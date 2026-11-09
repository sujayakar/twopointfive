//! THE BRING-UP'S OWN PART of verdict: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The file a mission row names for the level this game stands on (None: the bundled office). Read back off the level's own name.   (verdict.ts rowFile)
pub fn row_file(g: &crate::game::Game) -> Option<String> {
    let name = crate::scene_level_load::level_name_of(g)?;
    if name.is_empty() || name == "office" { return None; }
    Some(alloc::format!("{}.json", name))
}

/// The mission that is up is the stealth game's kind on a row that leans loud: nothing grades that run as a sneak. VERDICT.loudCard is settled ON on the page.   (verdict.ts loudRow)
pub fn loud_row(g: &crate::game::Game) -> bool {
    g.staged.facts.unseen && crate::scenario_run::running_row(g, row_file(g).as_deref()).and_then(|e| e.lean) == Some(crate::ui_missions::Lean::Loud)
}
