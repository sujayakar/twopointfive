//! THE BRING-UP'S OWN PART of verdict: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/verdict.ts:139 Verdict.tick)
pub fn verdict_tick(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/verdict.ts:156 Verdict.fresh)
pub fn verdict_fresh(g: &mut crate::game::Game, tail: Option<u32>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/verdict.ts:172 Verdict.roster)
pub fn verdict_roster(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/verdict.ts:189 Verdict.watch)
pub fn verdict_watch(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/verdict.ts:245 Verdict.close)
pub fn verdict_close(g: &mut crate::game::Game, end: &crate::verdict::VerdictCloseEnd) {}

/// The file a mission row names for the level this game stands on (None: the bundled office). Read back off the level's own name.   (verdict.ts rowFile)
pub fn row_file(g: &mut crate::game::Game) -> Option<String> {
    let name = crate::scene_level_load::level_name_of(g)?;
    if name.is_empty() || name == "office" { return None; }
    let file = alloc::format!("{}.json", name);
    for (k, v) in crate::scene_look::look_levels().iter() {
        if *v == file { return Some(k); }
    }
    Some(file)
}

/// The mission that is up is the stealth game's kind on a row that leans loud: nothing grades that run as a sneak. VERDICT.loudCard is settled ON on the page.   (verdict.ts loudRow)
pub fn loud_row(g: &mut crate::game::Game) -> bool {
    let file = row_file(g);
    let run = crate::scenario_run::running_scenario(g, file.as_deref(), None);
    match run {
        Some(run) => {
            run.kind == crate::scenario_run::RunningScenarioKind::Quiet
                && run.entry.and_then(|e| e.lean) == Some(crate::ui_missions::Lean::Loud)
        }
        None => false,
    }
}
