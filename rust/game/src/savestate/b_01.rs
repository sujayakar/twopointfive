//! THE BRING-UP'S OWN PART of savestate: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:408 firearmApply)
pub fn firearm_apply(g: &mut crate::game::Game, w: FirearmId, s: &crate::savestate::FirearmSnap) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:471 ownChargesBack)
pub fn own_charges_back(g: &mut crate::game::Game, own: Option<&crate::savestate::OwnCharges>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:669 loadState)
pub fn load_state(g: &mut crate::game::Game, s: &crate::savestate::SaveState, opts: Option<&crate::savestate::LoadStateOpts>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:688 applyState)
pub fn apply_state(g: &mut crate::game::Game, s: &crate::savestate::SaveState, prev_loads: f64, prev_saves: f64, opts: &crate::savestate::ApplyStateOpts) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:860 SaveStore.remove)
pub fn save_store_remove(g: &mut crate::game::Game, key: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:885 MemShelf.set)
pub fn mem_shelf_set(g: &mut crate::game::Game, key: &str, e: &crate::savestate::MemSave) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:888 MemShelf.clear)
pub fn mem_shelf_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:934 Checkpoints.note)
pub fn checkpoints_note(g: &mut crate::game::Game, what: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:936 Checkpoints.adopt)
pub fn checkpoints_adopt(g: &mut crate::game::Game, a: Option<&crate::savestate::AutosaveSnap>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:951 Checkpoints.prime)
pub fn checkpoints_prime(g: &mut crate::game::Game) {}


/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:979 Checkpoints.tick)
pub fn checkpoints_tick(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:1051 Checkpoints.markRegret)
pub fn checkpoints_mark_regret(g: &mut crate::game::Game, key: &str, why: &str) {}


/// BOUNDARY: the host reads the machine's storage (localStorage) here. No store on this side: None, what `localStorage?.getItem` / a failed read gives. (src/game/savestate.ts:860 SaveStore.get)
pub fn save_store_get(g: &mut crate::game::Game, key: &str) -> Option<String> { None }

/// BOUNDARY: the host writes the machine's storage (localStorage) here. No store on this side: false, the interface's "nothing was stored". (src/game/savestate.ts:860 SaveStore.set)
pub fn save_store_set(g: &mut crate::game::Game, key: &str, val: &str) -> bool { false }

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
fn mem_no(g: &mut Game, run: &str) -> Option<String> {
    if run != g.saves.run_id { return Some(String::from("made in another run")); }
    if g.touring || g.puppet.is_some() { return Some(String::from("a show is running \u{2014} stop it first")); }
    None
}

/// Checkpoints.load of a key not on the shelf: `read` finds nothing or an empty text, gone; else the file's own load. (src/game/savestate.ts:1171, :1169)
fn load_off_shelf(g: &mut Game, key: &str) -> Option<String> {
    match save_store_get(g, key) {
        Some(raw) if !raw.is_empty() => crate::savestate::checkpoints_load(g, &crate::savestate::CheckpointsLoadFrom::Text(String::from(key))),
        _ => Some(String::from("that checkpoint is gone")),
    }
}

/// Every stored checkpoint, newest first: the index's rows (no text under the index: none), then the shelf's. (src/game/savestate.ts:1063 Checkpoints.slots, :1055 index)
fn slots_now(g: &mut Game) -> Vec<crate::savestate::SlotInfo> {
    let mut all: Vec<crate::savestate::SlotInfo> = match save_store_get(g, &crate::savestate::key_index()) {
        Some(raw) if !raw.is_empty() => crate::savestate::checkpoints_index(g).slots.values().cloned().collect(),
        _ => Vec::new(),
    };
    all.extend(g.saves.shelf.m.values().map(|e| e.info.clone()));
    js::sort_by(&mut all, |a, b| b.saved_at - a.saved_at);
    all
}

/// The newest stored checkpoint: this run's at or before `before_t`, else this run's newest, a regretted one last, else any run's. (src/game/savestate.ts:1140 Checkpoints.newest)
pub fn checkpoints_newest(g: &mut crate::game::Game, prefer_run: Option<bool>, before_t: Option<f64>) -> Option<crate::savestate::SlotInfo> {
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
pub fn checkpoints_is_this_run(g: &mut crate::game::Game, info: &crate::savestate::SlotInfo) -> bool {
    info.run_id == g.saves.run_id
}
