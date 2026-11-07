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

/// HOW THE MISSION ENDED, asked of the game as it stands: None while it is being played, else whether it was won, the end's word and why. A read: it
/// writes nothing and keeps nothing, so the first tick it answers is the end and the asker keeps the answer (the TypeScript's verdict latches it on
/// the puppet, with the men's tracks and the card, which stay the page's). While the answer is None it reads a few flags and allocates nothing.
/// (verdict.ts Verdict.ended, under Verdict.live)
pub fn verdict_ended(g: &mut crate::game::Game) -> Option<(bool, &'static str, String)> {
    use crate::scenario_run::ActiveScenario as K;
    if !g.k.verdict.on || g.touring || g.quiet_utility || g.puppet.is_some() || g.mission_in_tour { return None; }
    let kind = crate::scenario_run::active_scenario(g);
    let roomclear = kind == Some(K::Roomclear);
    // (defuse.ts Defuse.active, without its copy of the level's block: this pass is the running mission's, on a level with devices and rooms)
    let defuse = roomclear && g.defuse.run == Some(g.mission.serial) && !g.room_clear.rooms.is_empty()
        && g.level.scenarios.as_ref().and_then(|s| s.defuse.as_ref()).map_or(false, |d| !d.devices.is_empty());
    if defuse && g.defuse.end == Some(crate::defuse::DefuseEnd::Failed) { return Some((false, "BLOWN", String::new())); }   // (the blast that fails it can take him with it: the charge is the why)
    let mut all_down = true;
    for i in 0..g.seats.len() { let s = g.seats[i]; let p = crate::cmd::seat_man(g, s); if !g.pl[p].down { all_down = false; break; } }
    if all_down {
        let by = &g.chronicle.counts.down_by;
        return Some((false, "DOWN", if by.is_empty() { String::new() } else { alloc::format!("by {}", by) }));
    }
    if roomclear {
        use crate::roomclear::RoomClearEnd as E;
        let end = g.room_clear.end;
        if crate::snatch::snatch_active(g) {
            return match g.snatch.end {
                Some(crate::snatch::SnatchEnd::Won) => Some((true, "TAKEN ALIVE", g.snatch.why.clone())),
                Some(crate::snatch::SnatchEnd::Lost) => Some((false, if g.snatch.lost == Some(crate::snatch::SnatchLost::Gone) { "GOT AWAY" } else { "DEAD" }, g.snatch.why.clone())),
                None => if end == Some(E::Wiped) { Some((false, "WIPED", g.room_clear.end_why.clone())) } else { None },
            };
        }
        if defuse {
            let d = g.defuse.end;
            if d == Some(crate::defuse::DefuseEnd::Safe) && end == Some(E::Taken) { return Some((true, "DEFUSED", g.room_clear.end_why.clone())); }   // (defuse.ts Defuse.won)
            if d == Some(crate::defuse::DefuseEnd::Failed) { return Some((false, "BLOWN", String::new())); }
            if d == Some(crate::defuse::DefuseEnd::Over) || end == Some(E::Wiped) { return Some((false, "WIPED", g.room_clear.end_why.clone())); }
            return None;
        }
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
