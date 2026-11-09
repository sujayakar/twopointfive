#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
pub use ::game_types::savestate::types;
pub use types::*;
pub mod control;
use crate::game::Game;
use crate::js::{self, JsMap, JsSet};
use alloc::{string::String, vec::Vec};

/// (src/game/savestate.ts:971 Checkpoints.request)
pub fn checkpoints_request(g: &mut crate::game::Game, why: &str, label: &str) { g.out.push(crate::out_events::OutEvent::CheckpointsRequest(crate::out_events::OutCheckpointsRequest { why: String::from(why), label: String::from(label) })); }

/// (src/game/savestate.ts:1093 Checkpoints.keep)
pub fn checkpoints_keep(g: &mut crate::game::Game, label: &str) { g.out.push(crate::out_events::OutEvent::CheckpointsKeep(crate::out_events::OutCheckpointsKeep { label: String::from(label) })); }

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:860 SaveStore.remove)
pub fn save_store_remove(g: &crate::game::Game, key: &str) {}

/// BOUNDARY: the host writes the machine's storage (localStorage) here. No store on this side: false, the interface's "nothing was stored". (src/game/savestate.ts:860 SaveStore.set)
pub fn save_store_set(g: &crate::game::Game, key: &str, val: &str) -> bool { false }

/// Load asked for inside a step: a save in memory waits for the end of this update, anything else is a file's load. (src/game/savestate.ts:1089 Checkpoints.loadInStep)
pub fn checkpoints_load_in_step(g: &mut crate::game::Game, key: &str) -> Option<String> {
    let run = match g.saves.shelf.m.get(String::from(key)) {
        Some(e) => e.info.run_id.clone(),
        None => return load_off_shelf(g, key),
    };
    if let Some(why) = mem_no(g, &run) { return Some(why); }
    g.saves.due = Some(String::from(key));
    None
}

/// Why a save in memory will not go back. `e.snap.game !== g` has no row: MemSave holds no snapshot and the shelf lives on this game. (src/game/savestate.ts:1090 Checkpoints.memNo)
pub(super) fn mem_no(g: &Game, run: &str) -> Option<String> {
    if run != g.saves.run_id { return Some(String::from("made in another run")); }
    if g.puppet.is_some() { return Some(String::from("nobody is playing")); }
    None
}

/// Checkpoints.load of a key not on the shelf (src/game/savestate.ts:1171). A file's save is the host's: its store is not on
/// this side, and a key with nothing behind it is "gone". The host that keeps saves hands them over on the shelf.
fn load_off_shelf(g: &Game, key: &str) -> Option<String> {
    Some(String::from("that checkpoint is gone"))
}

/// Every stored checkpoint, newest first (src/game/savestate.ts:1063 Checkpoints.slots). This game is the one "stepped elsewhere, which keeps its own
/// saves" (SNAPS.own): the store's index is another page's and is not read; the rows are the shelf's.
fn slots_now(g: &Game) -> Vec<crate::savestate::SlotInfo> {
    let mut all: Vec<crate::savestate::SlotInfo> = g.saves.shelf.m.values().map(|e| e.info.clone()).collect();
    js::sort_by(&mut all, |a, b| b.saved_at - a.saved_at);
    all
}

/// The newest stored checkpoint: this run's at or before `before_t`, else this run's newest, a regretted one last, else any run's. (src/game/savestate.ts:1140 Checkpoints.newest)
pub fn checkpoints_newest(g: &crate::game::Game, prefer_run: Option<bool>, before_t: Option<f64>) -> Option<crate::savestate::SlotInfo> {
    let prefer_run = prefer_run.unwrap_or(true);
    let before_t = before_t.unwrap_or(f64::INFINITY);
    let all = slots_now(g);
    if !prefer_run { return all.first().cloned(); }
    let mine: Vec<&crate::savestate::SlotInfo> = all.iter().filter(|x| x.run_id == g.saves.run_id).collect();
    let sound: Vec<&crate::savestate::SlotInfo> = mine.iter().copied().filter(|x| x.regret.as_deref().map_or(true, |r| r.is_empty())).collect();   // (`!x.regret`: an empty text is no regret)
    let before = |x: &&&crate::savestate::SlotInfo| x.mission_t <= before_t + 0.5;
    sound.iter().find(before).or(sound.first()).or(mine.iter().find(before)).or(mine.first()).map(|x| (**x).clone()).or_else(|| all.first().cloned())
}

/// Is this slot one of this run's. (src/game/savestate.ts:1154 Checkpoints.isThisRun)
pub fn checkpoints_is_this_run(g: &crate::game::Game, info: &crate::savestate::SlotInfo) -> bool {
    info.run_id == g.saves.run_id
}
