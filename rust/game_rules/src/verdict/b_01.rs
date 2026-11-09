//! THE BRING-UP'S OWN PART of verdict: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The file a mission row names for the level this game stands on (None: the bundled office). Read back off the level's own name.
pub fn row_file(g: &crate::game::Game) -> Option<String> {
    let name = crate::scene_level_load::level_name_of(g)?;
    if name.is_empty() || name == "office" { return None; }
    Some(alloc::format!("{}.json", name))
}

/// The mission that is up is the stealth game's kind on a row that leans loud: nothing grades that run as a sneak. VERDICT.loudCard is settled ON on the page.
pub fn loud_row(g: &crate::game::Game) -> bool {
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

/// This tick's end, by the mission kind's own rule (None: still running). The tick asks it and latches the first answer.
pub(crate) fn ended(g: &crate::game::Game) -> Option<(bool, &'static str, String)> {
    use crate::scenario_run::ActiveScenario as K;
    let kind = crate::scenario_run::active_scenario(g);
    let roomclear = kind == Some(K::Roomclear);
    let mut all_down = true;
    for i in 0..g.seats.len() { let s = g.seats[i]; let p = crate::cmd::seat_man(g, s); if !g.pl[p].down { all_down = false; break; } }
    if all_down {
        let by = &g.chronicle.counts.down_by;
        return Some((false, "DOWN", if by.is_empty() { String::new() } else { alloc::format!("by {}", by) }));
    }
    if roomclear {
        use crate::roomclear::RoomClearEnd as E;
        let end = g.room_clear.end;
        return match end { Some(E::Taken) => Some((true, "CLEAR", g.room_clear.end_why.clone())), Some(E::Wiped) => Some((false, "WIPED", g.room_clear.end_why.clone())), None => None };
    }
    if kind == Some(K::Rescue) {
        return g.rescue.result.as_ref().map(|r| if r.state == crate::rescue::RescueResultState::Rescued { (true, "RESCUED", r.why.clone()) } else { (false, "LOST", r.why.clone()) });
    }
    if kind == Some(K::Nightbreach) {
        let h = &g.night_breach.hostiles;
        if !h.is_empty() && h.iter().all(|&x| g.gd[x].state == crate::game::GuardState::Dead || g.gd[x].fled) { return Some((true, "CLEAR", String::new())); }
        return if g.mission.stage == crate::game::MissionStage::Failed { Some((false, "FAILED", String::new())) } else { None };
    }
    match g.mission.stage { crate::game::MissionStage::Done => Some((true, "OUT", String::new())), crate::game::MissionStage::Failed => Some((false, "FAILED", String::new())), _ => None }
}
