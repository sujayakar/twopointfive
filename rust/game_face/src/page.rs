//! THE FACE'S EXPORTS FOR A HOST THAT SHOWS THE GAME (the page, src/host/rustgame.ts): the Rust game steps, and after each step only what the picture
//! reads goes back, as the filtered export (`game::puppet::export_puppet`, tools/blitz/PUPPET.md) with the step's out events on its tail.
//!
//!   face_new_at(state, n, slot) -> h   a game from a start (the bridge's bytes) OVER THE BLOCK THAT ALREADY STANDS in rust/sim's slot `slot`: the host's
//!                                      own code stood the level through this same module's `rust/sim` exports, so the block is not handed over and not
//!                                      copied anywhere else. The host says which slot (src/sim/state.ts holds the slots); the game is pinned to it.
//!   face_puppet(h, start)              the filtered export at face_out. `start` = 1: the long list and every row (after face_new_at, and it fills the
//!                                      memo of what was last sent); 0: what changed since the last call. The out events ride on its tail, and the
//!                                      game's list of them is emptied here: they are a step's output, read once.
//!   face_probe_asks(h)                 THE LIGHT PROBE IS THE HOST'S (render/probe.ts: the GPU answers a frame or more later). What the tick asked of it
//!                                      and the host has not yet taken, at face_out: u32 count, then for each its key (u32 length, utf-8), the point
//!                                      (3 f64), whose own proxies are skipped (f64), a byte and the far end of a line (3 f64) where the byte is 1,
//!                                      and its rank (f64). Nothing is taken: the host calls this after each step and holds its own table to it.
//!                                      Then u32 count and the keys the tick still holds an ANSWER for: a key it forgot inside the tick (a sight
//!                                      line picked up again starts from no answer: `sensors_forget`) is gone from the list, and the host lets its
//!                                      own answer for it go, or it would hand the stale one straight back before the next step.
//!   face_probe_took(h)                 the host's frame has sent the table to the GPU: it is empty again here, and whose turn it is on a full frame is
//!                                      kept (`probe_like_take_asks`)
//!   face_probe_results(h, p, n)        the answers the host holds now, before a step: u32 count, each a key and its three numbers
//!   face_poke(h, what, v) -> 1 | 0     a host's switch written between ticks, by number: 0 god mode · 1 the AI on · 2 infinite ammunition · 3 the
//!                                      player unseen · 4 the launcher's held button let go unsent (the pause card: host/loop.ts). `v` != 0 = on.
//!   face_mem() -> bytes                the module's memory now (a host holds its typed arrays to this: a memory that grew has detached them)
//!
//! A GAME KEPT (a checkpoint, the start a restart goes back to). The host's save controller (src/game/savestate.ts) decides when and under which label; what
//! it keeps is the Rust game's own, held here and named by a number the controller's slot holds:
//!   face_keep(h, how) -> k             how 0: THE GAME'S OWN VALUE (a clone of the whole `Game`, every field the bridge does not carry with it: a closure
//!                                      in flight, the probe's table, the counters) and its block of rust/sim's. Nothing is lost by it, and it lives only
//!                                      as long as this module. how 1: THE BRIDGE'S BYTES (`io::export`) and the block: what a file holds.
//!   face_keep_given(s, n, b, bn) -> k  a kept game from a file's two parts
//!   face_kept_state(k) · face_kept_block(k)   at face_out: the bridge's bytes of kept `k` (a clone is exported now) · its block
//!   face_kept_drop(k) · face_kept_n()  let go · how many are held
//!   face_back(h, k) -> 1 | 0           game `h` IS kept `k` again, in the slot `h` stands in (the kept one stays kept). A clone is cloned back; bytes go
//!                                      through the start's own lines (`game_run::start`: `io::import`, so what the bridge does not carry starts blank).
//!                                      What `face_puppet` last sent is forgotten (memo.rs `forget`): the host applies the whole state next.
//!   face_loaded(h, label, n, loads, saves)   STAND-IN: WHAT THE SAVE CONTROLLER DOES TO THE GAME ONCE A SAVE IN MEMORY IS PUT BACK (savestate.ts loadMem,
//!                                      the lines after the snapshot is restored: :1124 to :1134). A ring of the hand's that merely stood at the save is
//!                                      put away, a gun's setup is not up, the squad's ring is put away, the run's counts of loads and saves are the
//!                                      abandoned moment's (handed in: the kept game holds the saved moment's), the chronicle takes its row, and a
//!                                      tutorial's program is to be started again. After `face_back`, before the next step. Belongs in rust/game's
//!                                      savestate (`checkpoints_load_mem`, an empty body today: the controller is the host's).
//!   face_saves_in(h, p, n)             THE CONTROLLER'S SHELF AS THE TICK READS IT (`g.saves`: Enter once down asks for this run's newest checkpoint,
//!                                      player.ts:493): the run's name and each slot's row. u32 count-prefixed utf-8 for a text; the run, u32 rows, then
//!                                      each: key, slot (u8: 0 auto, 1 manual), n (f64), label, saved_at, mission_t (f64), stage, its run, a byte and
//!                                      the regret's words where the byte is 1.
//!   face_saves_asked(h) -> bits        what the tick asked of the controller, read once: 1 a save in memory is due back (its key at face_out) ·
//!                                      2 the player restarted (`game_restart` ran: the host's camera owes a cut)
//!
//! AFTER A PANIC. The module aborts on a panic and the host sees a trap; the game of that call is gone (lib.rs `with` takes it out for the call, so no
//! borrow is left behind), and every game KEPT is still whole. The host goes on in the same instance:
//!   face_stand(k, slot) -> h | 0       A NEW GAME that is kept `k`, standing in `slot` (as `face_back`, with no game to stand it in). Its serial counter
//!                                      starts no lower than the highest any export has shown the host (`serial_seen`): a number means one thing.
//!   face_debug_panic(h, how)           A PLANTED PANIC, for the check of that road (the page's `?rustpanic=<tick>`): how 0 inside the borrow of game `h`,
//!                                      as a rule of the tick would (the game is lost); how 1 outside any game (nothing is lost but the call).
use std::cell::RefCell;

/// a game kept: its own value, or the bridge's bytes of it; and its block
enum Kept { Own(Box<game::game::Game>, Vec<u8>), Bytes(Vec<u8>, Vec<u8>) }

thread_local! {
    /// the games kept, by number less one
    static KEPT: RefCell<Vec<Option<Kept>>> = const { RefCell::new(Vec::new()) };
}
fn block_len() -> usize { core::mem::size_of::<sim::state::State>() }
fn block_of(slot: usize) -> Vec<u8> { unsafe { std::slice::from_raw_parts(sim::state::addr(slot) as *const u8, block_len()).to_vec() } }
fn keep(k: Kept) -> u32 {
    KEPT.with(|v| { let mut v = v.borrow_mut(); match v.iter().position(|x| x.is_none()) { Some(i) => { v[i] = Some(k); i as u32 + 1 } None => { v.push(Some(k)); v.len() as u32 } } })
}

/// # Safety
/// `state`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_new_at(state: *const u8, n: usize, slot: usize) -> u32 {
    super::hook();
    if slot >= sim::state::GAMES { return 0; }
    // (the start's own lines take a block as bytes and write it into the slot: handed the slot's own bytes, the block stands as it stood)
    let len = core::mem::size_of::<sim::state::State>();
    let block = std::slice::from_raw_parts(sim::state::addr(slot) as *const u8, len).to_vec();
    let (g, said) = game_run::start(super::bytes(state, n), &block, slot);
    super::out(said.join("\n").into_bytes());
    let h = super::put(super::Held { g, cmd: None, cmds: None, after: Vec::new() });
    crate::memo::forget(h);
    h
}

#[no_mangle]
pub extern "C" fn face_mem() -> usize {
    core::arch::wasm32::memory_size(0) * 65536
}

#[no_mangle]
pub extern "C" fn face_probe_asks(h: u32) {
    let b = super::with(h, |x| {
        let mut o: Vec<u8> = Vec::new();
        o.extend_from_slice(&(x.g.probe.pending.len() as u32).to_le_bytes());
        for a in x.g.probe.pending.iter() {
            o.extend_from_slice(&(a.key.len() as u32).to_le_bytes());
            o.extend_from_slice(a.key.as_bytes());
            for v in a.pos.iter() { o.extend_from_slice(&v.to_le_bytes()); }
            o.extend_from_slice(&a.skip_owner.to_le_bytes());
            match a.b.as_ref() { None => o.push(0), Some(b) => { o.push(1); for v in b.iter() { o.extend_from_slice(&v.to_le_bytes()); } } }
            o.extend_from_slice(&a.rank.to_le_bytes());
        }
        // (…and the keys the tick still holds an answer for: one it forgot inside the tick, `sensors_forget`, is not among them)
        let keys = x.g.probe.results.keys();
        o.extend_from_slice(&(keys.len() as u32).to_le_bytes());
        for k in keys.iter() { o.extend_from_slice(&(k.len() as u32).to_le_bytes()); o.extend_from_slice(k.as_bytes()); }
        o
    });
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_probe_took(h: u32) {
    super::with(h, |x| { let _ = game::sensors::probe_like_take_asks(&mut x.g); });
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_probe_results(h: u32, p: *const u8, n: usize) {
    let b = super::bytes(p, n);
    let u32_at = |at: usize| u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize;
    let f64_at = |at: usize| f64::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3], b[at + 4], b[at + 5], b[at + 6], b[at + 7]]);
    super::with(h, |x| {
        let mut at = 4;
        for _ in 0..u32_at(0) {
            let len = u32_at(at); at += 4;
            let key = String::from_utf8_lossy(&b[at..at + len]).into_owned(); at += len;
            x.g.probe.results.set(key, [f64_at(at), f64_at(at + 8), f64_at(at + 16)]); at += 24;
        }
    });
}
#[no_mangle]
pub extern "C" fn face_poke(h: u32, what: u32, v: f64) -> u32 {
    super::with(h, |x| {
        let g = &mut x.g;
        match what {
            0 => g.god_mode = v != 0.0,
            1 => g.ai_enabled = v != 0.0,
            2 => g.infinite_ammo = v != 0.0,
            3 => g.player_invisible = v != 0.0,
            4 => { let pl = g.player; g.pl[pl].launch_held = None; }
            _ => return 0,
        }
        1
    })
}
// ---- a game kept
#[no_mangle]
pub extern "C" fn face_keep(h: u32, how: u32) -> u32 {
    let k = super::with(h, |x| {
        let block = block_of(x.g.sim);
        if how == 0 { let mut g = x.g.clone(); g.out.clear(); Kept::Own(g, block) } else { Kept::Bytes(game::io::export(&x.g), block) }
    });
    keep(k)
}
/// # Safety
/// the two pointers name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_keep_given(state: *const u8, n: usize, block: *const u8, bn: usize) -> u32 {
    super::hook();
    keep(Kept::Bytes(super::bytes(state, n).to_vec(), super::bytes(block, bn).to_vec()))
}
#[no_mangle]
pub extern "C" fn face_kept_state(k: u32) {
    let b = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_ref()) { Some(Kept::Own(g, _)) => game::io::export(g), Some(Kept::Bytes(s, _)) => s.clone(), None => Vec::new() });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_kept_block(k: u32) {
    let b = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_ref()) { Some(Kept::Own(_, b)) | Some(Kept::Bytes(_, b)) => b.clone(), None => Vec::new() });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_kept_drop(k: u32) {
    KEPT.with(|v| if let Some(x) = v.borrow_mut().get_mut((k as usize).wrapping_sub(1)) { *x = None; });
}
#[no_mangle]
pub extern "C" fn face_kept_n() -> usize {
    KEPT.with(|v| v.borrow().iter().flatten().count())
}
#[no_mangle]
pub extern "C" fn face_back(h: u32, k: u32) -> u32 {
    let (slot, serial_next) = super::with(h, |x| (x.g.sim, x.g.serial_next));
    let made: Option<Box<game::game::Game>> = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_ref()) {
        Some(Kept::Own(g, block)) => {
            unsafe { core::ptr::copy_nonoverlapping(block.as_ptr(), sim::state::addr(slot) as *mut u8, block.len().min(block_len())); }
            let mut g = g.clone();
            game::io::set_slot(&mut g, slot);
            // (the box world's copy in rust/sim stands outside the block and is the abandoned moment's: written whole again as the next tick opens, as the
            //  TypeScript's is after a snapshot is put back: boxMirror.ts `putBacks`)
            g.col.mirror.put_backs = -1.0;
            Some(g)
        }
        Some(Kept::Bytes(state, block)) => Some(game_run::start(state, block, slot).0),
        None => None,
    });
    let Some(mut g) = made else { return 0 };
    // (the serial counter never goes back in a game: what was dealt after the save is still held on the host's side, and a number means one thing)
    g.serial_next = g.serial_next.max(serial_next);
    super::with(h, |x| { x.g = g; x.cmd = None; x.cmds = None; x.after.clear(); });
    crate::memo::forget(h);
    1
}

thread_local! {
    /// the highest serial counter any export has shown the host
    static SERIAL_SEEN: core::cell::Cell<u32> = const { core::cell::Cell::new(0) };
}
/// an export is about to show the host this game: its serial counter is noted (what the host holds was dealt below it)
pub fn serial_seen(next: u32) { SERIAL_SEEN.with(|s| if next > s.get() { s.set(next) }); }

#[no_mangle]
pub extern "C" fn face_stand(k: u32, slot: usize) -> u32 {
    super::hook();
    if slot >= sim::state::GAMES { return 0; }
    let made: Option<Box<game::game::Game>> = KEPT.with(|v| match v.borrow().get((k as usize).wrapping_sub(1)).and_then(|x| x.as_ref()) {
        Some(Kept::Own(g, block)) => {
            unsafe { core::ptr::copy_nonoverlapping(block.as_ptr(), sim::state::addr(slot) as *mut u8, block.len().min(block_len())); }
            let mut g = g.clone();
            game::io::set_slot(&mut g, slot);
            g.col.mirror.put_backs = -1.0;
            Some(g)
        }
        Some(Kept::Bytes(state, block)) => Some(game_run::start(state, block, slot).0),
        None => None,
    });
    let Some(mut g) = made else { return 0 };
    g.serial_next = g.serial_next.max(SERIAL_SEEN.with(|s| s.get()));
    let h = super::put(super::Held { g, cmd: None, cmds: None, after: Vec::new() });
    crate::memo::forget(h);
    h
}
#[no_mangle]
pub extern "C" fn face_debug_panic(h: u32, how: u32) {
    super::hook();
    if how != 0 { panic!("a planted panic, outside any game (face_debug_panic)"); }
    super::with(h, |x| {
        // (half a write, as a rule that panics leaves one: the clock moved and nothing else)
        x.g.time += 1.0 / 60.0;
        let none: Option<u32> = None;
        let at = x.g.time;
        if none.is_none() { panic!("a planted panic, inside the game at {at:.3} s (face_debug_panic)"); }
    });
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_saves_in(h: u32, p: *const u8, n: usize) {
    use game::savestate::{MemSave, SlotInfo, SlotInfoKind, SlotInfoSlot};
    let b = super::bytes(p, n);
    let mut at = 0usize;
    let mut u32_ = |at: &mut usize| { let v = u32::from_le_bytes([b[*at], b[*at + 1], b[*at + 2], b[*at + 3]]) as usize; *at += 4; v };
    let f64_ = |at: &mut usize| { let mut w = [0u8; 8]; w.copy_from_slice(&b[*at..*at + 8]); *at += 8; f64::from_le_bytes(w) };
    let mut text = |at: &mut usize| { let len = u32_(at); let s = String::from_utf8_lossy(&b[*at..*at + len]).into_owned(); *at += len; s };
    let run = text(&mut at);
    let rows = { let v = u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize; at += 4; v };
    let mut list: Vec<SlotInfo> = Vec::with_capacity(rows);
    for _ in 0..rows {
        let key = text(&mut at);
        let manual = b[at] != 0; at += 1;
        let nn = f64_(&mut at);
        let label = text(&mut at);
        let saved_at = f64_(&mut at);
        let mission_t = f64_(&mut at);
        let stage = text(&mut at);
        let run_id = text(&mut at);
        let regret = { let has = b[at] != 0; at += 1; if has { Some(text(&mut at)) } else { None } };
        list.push(SlotInfo { key, slot: if manual { SlotInfoSlot::Manual } else { SlotInfoSlot::Auto }, n: nn, label, kind: if manual { SlotInfoKind::Manual } else { SlotInfoKind::Auto },
            saved_at, mission_t, stage, run_id, bytes: 0.0, regret, mem: Some(true) });
    }
    super::with(h, |x| {
        let s = &mut x.g.saves;
        s.run_id = run;
        s.shelf.m.clear();
        for info in list { s.shelf.m.set(info.key.clone(), MemSave { info, pic: None, take: 0.0 }); }
    });
}
#[no_mangle]
pub extern "C" fn face_saves_asked(h: u32) -> u32 {
    let (bits, key) = super::with(h, |x| {
        let g = &mut x.g;
        let due = g.saves.due.take();
        let cut = core::mem::replace(&mut g.restart_cut, false);
        ((due.is_some() as u32) | ((cut as u32) << 1), due.unwrap_or_default())
    });
    super::out(key.into_bytes());
    bits
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_loaded(h: u32, p: *const u8, n: usize, loads: f64, saves: f64) {
    let label = super::text(p, n);
    super::with(h, |x| {
        let g = &mut x.g;
        let pl = g.player;
        // (handring.ts: a ring that merely stood at the save is nothing of the saved moment)
        if g.pl[pl].sel.ring.as_ref().is_some_and(|r| r.free == Some(true)) {
            let s = &mut g.pl[pl].sel;
            s.open = false; s.ring = None; s.down_t = -1.0; s.moved = false;
            g.pl[pl].near_sel = 0;
        }
        g.pl[pl].setup = None;   // (slotsetup.ts: a gun's setup that was up at the save is not up after the load)
        game::orders::squad_orders_loaded(g);
        g.chronicle.counts.loads = loads;
        g.chronicle.counts.saves = saves;
        let _ = game::chronicle::chron_checkpoint(g, game::chronicle::ChronCheckpointWhat::Load, Some(&label));
        game::rails::rails_put_back(g);
    });
}
