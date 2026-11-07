#![allow(unused_variables)]
//! Every function and method of `savestate`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/savestate.ts:377 KEY_AUTO)
pub fn key_auto(i: f64) -> String { todo!() }

/// (src/game/savestate.ts:379 KEY_HAND)
pub fn key_hand(i: f64) -> String { todo!() }

/// (src/game/savestate.ts:379 KEY_PIC)
pub fn key_pic(key: &str) -> String { todo!() }

/// (src/game/savestate.ts:379 KEY_MEM)
pub fn key_mem(slot: crate::savestate::KeyMemSlot, i: f64) -> String { todo!() }

/// (src/game/savestate.ts:382 c3)
pub fn c3(p: &[f64]) -> [f64; 3] { todo!() }

/// (src/game/savestate.ts:383 c3n)
pub fn c3n(p: Option<&[f64]>) -> Option<[f64; 3]> { todo!() }

/// (src/game/savestate.ts:384 r3)
pub fn r3(p: &[f64], k: Option<f64>) -> [f64; 3] { todo!() }

/// (src/game/savestate.ts:385 fin)
pub fn fin(x: f64, d: Option<f64>) -> f64 { todo!() }

/// (src/game/savestate.ts:387 infOut)
pub fn inf_out(x: f64) -> Option<f64> { todo!() }

/// (src/game/savestate.ts:388 infIn)
pub fn inf_in(x: Option<f64>) -> f64 { todo!() }

/// (src/game/savestate.ts:389 deepClone)
pub fn deep_clone<T: Clone + core::fmt::Debug>(x: &T) -> T { todo!() }

/// (src/game/savestate.ts:392 levelFingerprint)
pub fn level_fingerprint(g: &mut crate::game::Game) -> crate::savestate::SaveStateLevel { todo!() }

/// (src/game/savestate.ts:406 firearmSnap)
pub fn firearm_snap(g: &mut crate::game::Game, w: FirearmId) -> crate::savestate::FirearmSnap { todo!() }

/// (src/game/savestate.ts:408 firearmApply)
pub fn firearm_apply(g: &mut crate::game::Game, w: FirearmId, s: &crate::savestate::FirearmSnap) { todo!() }

/// (src/game/savestate.ts:416 rosterWanted)
pub fn roster_wanted(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/savestate.ts:421 runNo)
pub fn run_no(g: &mut crate::game::Game, mem: bool, authored: Option<bool>) -> Option<String> { todo!() }

/// (src/game/savestate.ts:457 squadUp)
pub fn squad_up(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/savestate.ts:460 canSnap)
pub fn can_snap(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/savestate.ts:464 ownCharges)
pub fn own_charges(g: &mut crate::game::Game) -> crate::savestate::OwnCharges2 { todo!() }

/// (src/game/savestate.ts:471 ownChargesBack)
pub fn own_charges_back(g: &mut crate::game::Game, own: Option<&crate::savestate::OwnCharges>) { todo!() }

/// (src/game/savestate.ts:480 playerBusy)
pub fn player_busy(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/savestate.ts:509 saveState)
pub fn save_state(g: &mut crate::game::Game, opts: Option<&crate::savestate::SaveOpts>) -> crate::savestate::SaveState { todo!() }

/// (src/game/savestate.ts:644 canLoad)
pub fn can_load(g: &mut crate::game::Game, s: Option<&crate::savestate::SaveState>) -> Option<String> { todo!() }

/// (src/game/savestate.ts:670 loadState)
pub fn load_state(g: &mut crate::game::Game, s: &crate::savestate::SaveState, opts: Option<&crate::savestate::LoadStateOpts>) { todo!() }

/// (src/game/savestate.ts:689 applyState)
pub fn apply_state(g: &mut crate::game::Game, s: &crate::savestate::SaveState, prev_loads: f64, prev_saves: f64, opts: &crate::savestate::ApplyStateOpts) { todo!() }

/// (src/game/savestate.ts:853 localToWorld)
pub fn local_to_world(g: &mut crate::game::Game, b: BoxId, p: [f64; 3], point: bool) -> V3 { todo!() }

/// (src/game/savestate.ts:861 SaveStore.get)
pub fn save_store_get(g: &mut crate::game::Game, key: &str) -> Option<String> { todo!() }

/// (src/game/savestate.ts:861 SaveStore.set)
pub fn save_store_set(g: &mut crate::game::Game, key: &str, val: &str) -> bool { todo!() }

/// (src/game/savestate.ts:861 SaveStore.remove)
pub fn save_store_remove(g: &mut crate::game::Game, key: &str) { todo!() }

/// (src/game/savestate.ts:869 memoryStore)
pub fn memory_store() -> crate::savestate::MemoryStore { todo!() }

/// (src/game/savestate.ts:885 MemShelf.get)
pub fn mem_shelf_get(g: &mut crate::game::Game, key: &str) -> Option<crate::savestate::MemSave> { todo!() }

/// (src/game/savestate.ts:886 MemShelf.set)
pub fn mem_shelf_set(g: &mut crate::game::Game, key: &str, e: &crate::savestate::MemSave) { todo!() }

/// (src/game/savestate.ts:888 MemShelf.all)
pub fn mem_shelf_all(g: &mut crate::game::Game) -> Vec<crate::savestate::MemSave> { todo!() }

/// (src/game/savestate.ts:889 MemShelf.clear)
pub fn mem_shelf_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/savestate.ts:905 roomHere)
pub fn room_here(g: &mut crate::game::Game, p: V3) -> String { todo!() }

/// (src/game/savestate.ts:934 Checkpoints.constructor)
pub fn checkpoints_new(g: &mut crate::game::Game) -> crate::savestate::Checkpoints { todo!() }

/// (src/game/savestate.ts:940 Checkpoints.put)
pub fn checkpoints_put(g: &mut crate::game::Game, key: &str, val: &str) -> bool { todo!() }

/// (src/game/savestate.ts:941 Checkpoints.note)
pub fn checkpoints_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/savestate.ts:943 Checkpoints.adopt)
pub fn checkpoints_adopt(g: &mut crate::game::Game, a: Option<&crate::savestate::AutosaveSnap>) { todo!() }

/// (src/game/savestate.ts:953 Checkpoints.basedNow)
pub fn checkpoints_based_now(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/savestate.ts:958 Checkpoints.prime)
pub fn checkpoints_prime(g: &mut crate::game::Game) { todo!() }

/// (src/game/savestate.ts:971 Checkpoints.request)
pub fn checkpoints_request(g: &mut crate::game::Game, why: &str, label: &str) { g.out.push(crate::out_events::OutEvent::CheckpointsRequest(crate::out_events::OutCheckpointsRequest { why: String::from(why), label: String::from(label) })); }

/// (src/game/savestate.ts:978 Checkpoints.gate)
pub fn checkpoints_gate(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/savestate.ts:986 Checkpoints.tick)
pub fn checkpoints_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/savestate.ts:1032 Checkpoints.write)
pub fn checkpoints_write(g: &mut crate::game::Game, slot: crate::savestate::CheckpointsWriteSlot, label: Option<&str>, force: Option<bool>) -> crate::savestate::SaveResult { todo!() }

/// (src/game/savestate.ts:1058 Checkpoints.markRegret)
pub fn checkpoints_mark_regret(g: &mut crate::game::Game, key: &str, why: &str) { todo!() }

/// (src/game/savestate.ts:1062 Checkpoints.index)
pub fn checkpoints_index(g: &mut crate::game::Game) -> crate::savestate::SlotIndex { todo!() }

/// (src/game/savestate.ts:1070 Checkpoints.slots)
pub fn checkpoints_slots(g: &mut crate::game::Game) -> Vec<crate::savestate::SlotInfo> { todo!() }

/// (src/game/savestate.ts:1074 Checkpoints.handSlot)
pub fn checkpoints_hand_slot(g: &mut crate::game::Game, idx: &crate::savestate::SlotIndex) -> f64 { todo!() }

/// (src/game/savestate.ts:1093 Checkpoints.keep)
pub fn checkpoints_keep(g: &mut crate::game::Game, label: &str) { g.out.push(crate::out_events::OutEvent::CheckpointsKeep(crate::out_events::OutCheckpointsKeep { label: String::from(label) })); }

/// (src/game/savestate.ts:1096 Checkpoints.loadInStep)
pub fn checkpoints_load_in_step(g: &mut crate::game::Game, key: &str) -> Option<String> { todo!() }

/// (src/game/savestate.ts:1097 Checkpoints.memNo)
pub fn checkpoints_mem_no(g: &mut crate::game::Game, e: &crate::savestate::MemSave) -> Option<String> { todo!() }

/// (src/game/savestate.ts:1099 Checkpoints.writeMem)
pub fn checkpoints_write_mem(g: &mut crate::game::Game, slot: crate::savestate::CheckpointsWriteMemSlot, label: &str, authored: Option<bool>) -> crate::savestate::SaveResult { todo!() }

/// (src/game/savestate.ts:1127 Checkpoints.loadMem)
pub fn checkpoints_load_mem(g: &mut crate::game::Game, key: &str) -> Option<String> { todo!() }

/// (src/game/savestate.ts:1156 Checkpoints.newest)
pub fn checkpoints_newest(g: &mut crate::game::Game, prefer_run: Option<bool>, before_t: Option<f64>) -> Option<crate::savestate::SlotInfo> { todo!() }

/// (src/game/savestate.ts:1170 Checkpoints.isThisRun)
pub fn checkpoints_is_this_run(g: &mut crate::game::Game, info: &crate::savestate::SlotInfo) -> bool { todo!() }

/// (src/game/savestate.ts:1174 Checkpoints.tapeBegin)
pub fn checkpoints_tape_begin(g: &mut crate::game::Game) -> crate::savestate::TapeSlots { todo!() }

/// (src/game/savestate.ts:1185 Checkpoints.read)
pub fn checkpoints_read(g: &mut crate::game::Game, key: &str) -> Option<crate::savestate::SaveState> { todo!() }

/// (src/game/savestate.ts:1187 Checkpoints.load)
pub fn checkpoints_load(g: &mut crate::game::Game, from: &crate::savestate::CheckpointsLoadFrom) -> Option<String> { todo!() }

/// (src/game/savestate.ts:1222 newRunId)
pub fn new_run_id(g: &mut crate::game::Game) -> String { todo!() }

pub mod local_store {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/savestate.ts:864 localStore.get)
    pub fn get(k: &str) -> Option<String> { todo!() }

    /// (src/game/savestate.ts:865 localStore.set)
    pub fn set(k: &str, v: &str) -> bool { todo!() }

    /// (src/game/savestate.ts:866 localStore.remove)
    pub fn remove(k: &str) { todo!() }

}
