//! THE FACE OF THE RUST GAME (the bring-up's first cut). A host (bun, the page) loads this module, hands a game's state in as bytes, asks
//! for a tick, and reads the state back. One game a handle; nothing here decides anything.
//!   face_alloc / face_free        memory for the host to write bytes into
//!   face_blank                    a game as its field initialisers leave it (no level): proves the module stands
//!   face_import / face_export     the state bridge (game::io, generated): bytes ↔ Game
//!   face_update                   one tick as the page drives it
//!   face_sim_addr / _len          where rust/sim's block of a slot is in THIS module's memory: the host copies the TypeScript game's block
//!                                 over it (same layout: both are the same Rust built to wasm)
//!   face_panic_ptr / _len         what the last panic said (a `todo!("needs: …")` names what is missing), before the trap
use std::cell::RefCell;

thread_local! {
    static GAMES: RefCell<Vec<Option<Box<game::game::Game>>>> = const { RefCell::new(Vec::new()) };
    static PANIC: RefCell<String> = const { RefCell::new(String::new()) };
    static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn hook() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            let s = info.to_string();
            PANIC.with(|p| *p.borrow_mut() = s);
        }));
    });
}
fn put(g: game::game::Game) -> u32 {
    GAMES.with(|v| {
        let mut v = v.borrow_mut();
        v.push(Some(Box::new(g)));
        v.len() as u32
    })
}
fn with<R>(h: u32, f: impl FnOnce(&mut game::game::Game) -> R) -> R {
    // the game is taken out for the call, so a panic inside leaves no borrow behind
    let mut g = GAMES.with(|v| v.borrow_mut()[h as usize - 1].take()).expect("no such game");
    let r = f(&mut g);
    GAMES.with(|v| v.borrow_mut()[h as usize - 1] = Some(g));
    r
}

#[no_mangle]
pub extern "C" fn face_alloc(n: usize) -> *mut u8 {
    hook();
    let mut v = Vec::<u8>::with_capacity(n.max(1));
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}
/// # Safety
/// `p` and `n` are what face_alloc gave.
#[no_mangle]
pub unsafe extern "C" fn face_free(p: *mut u8, n: usize) {
    drop(Vec::from_raw_parts(p, 0, n.max(1)));
}
#[no_mangle]
pub extern "C" fn face_blank() -> u32 {
    hook();
    put(game::game::Game::blank())
}
#[no_mangle]
pub extern "C" fn face_drop(h: u32) {
    GAMES.with(|v| v.borrow_mut()[h as usize - 1] = None);
}
#[no_mangle]
pub extern "C" fn face_time(h: u32) -> f64 {
    with(h, |g| g.time)
}
#[no_mangle]
pub extern "C" fn face_sim_addr(slot: usize) -> usize {
    sim::state::addr(slot)
}
#[no_mangle]
pub extern "C" fn face_sim_len() -> usize {
    core::mem::size_of::<sim::state::State>()
}
/// the word of the block's layout (rust/sim state.rs LAYOUT): the host holds it to the TypeScript module's before it copies a block over
#[no_mangle]
pub extern "C" fn face_sim_layout() -> u32 {
    sim::state::LAYOUT
}
/// which of rust/sim's slots a game stands in (`Game.sim`: the bridge does not write it)
#[no_mangle]
pub extern "C" fn face_sim_slot(h: u32) -> usize {
    with(h, |g| g.sim)
}
#[no_mangle]
pub extern "C" fn face_set_sim_slot(h: u32, slot: usize) {
    with(h, |g| g.sim = slot)
}
/// forget the last panic's message (the host asks before a call it wants the message of)
#[no_mangle]
pub extern "C" fn face_panic_clear() {
    PANIC.with(|p| p.borrow_mut().clear());
}
#[no_mangle]
pub extern "C" fn face_panic_ptr() -> *const u8 {
    PANIC.with(|p| p.borrow().as_ptr())
}
#[no_mangle]
pub extern "C" fn face_panic_len() -> usize {
    PANIC.with(|p| p.borrow().len())
}
#[no_mangle]
pub extern "C" fn face_out_ptr() -> *const u8 {
    OUT.with(|o| o.borrow().as_ptr())
}
#[no_mangle]
pub extern "C" fn face_out_len() -> usize {
    OUT.with(|o| o.borrow().len())
}

// THE BRIDGE AND THE TICK: on when `game::io` exists (the skeleton's program writes it) — `--features bridge`.
#[cfg(feature = "bridge")]
mod bridge {
    use super::*;
    /// # Safety
    /// `p`, `n` name bytes the host wrote into memory from face_alloc.
    #[no_mangle]
    pub unsafe extern "C" fn face_import(p: *const u8, n: usize) -> u32 {
        hook();
        put(game::io::import(std::slice::from_raw_parts(p, n)))
    }
    /// the state as bytes: read them at face_out_ptr / face_out_len before the next call
    #[no_mangle]
    pub extern "C" fn face_export(h: u32) {
        let b = with(h, |g| game::io::export(g));
        OUT.with(|o| *o.borrow_mut() = b);
    }
    /// the bridge's bytes over a game that already stands: what the walk does not carry stays as that game has it
    /// # Safety
    /// `p`, `n` name bytes the host wrote into memory from face_alloc.
    #[no_mangle]
    pub unsafe extern "C" fn face_import_into(h: u32, p: *const u8, n: usize) {
        let b = std::slice::from_raw_parts(p, n);
        with(h, |g| game::io::import_into(g, b));
    }
    /// ONE TICK OFF THE RECORD THE STATE CARRIES (`Game.cmd_buf`: what the TypeScript's `update` sampled before it called `step`): game.ts `update`'s own lines
    /// before the step that are the game's and not the host's (bit 0 of `pre`: the die taken for the frame; bit 1: the quiet mission's switches), then
    /// `game_step`. The host's half (`hostBefore` / `hostAfter`, the director's input, the journal) is not run: the TypeScript side is exported round its own
    /// `stepSeats`, not round `update`. A panic leaves its message at face_panic_ptr and traps; the game of this handle is gone after one.
    /// TEMP-BRINGUP (bringup-log.md stops 7, 14, 29): what a game's state and its block do not hold and a tick still reads. `knobs`: table 0 of the
    /// TypeScript module's rust/sim knobs; the clips, the bench's rig and the furniture's facts are written again from the state. `draws`: what the
    /// TypeScript tick drew off Math.random (a game with no die of its own). `pending`: asks in the probe's table as the tick opens.
    /// # Safety
    /// the pointers name bytes the host wrote into memory from face_alloc.
    #[no_mangle]
    pub unsafe extern "C" fn face_bringup(h: u32, knobs: *const u8, kn: usize, draws: *const u8, dn: usize, pending: usize) {
        let k = std::slice::from_raw_parts(knobs, kn);
        let d: Vec<f64> = std::slice::from_raw_parts(draws, dn * 8).chunks(8).map(|c| f64::from_le_bytes(c.try_into().unwrap())).collect();
        with(h, |g| game::anim_skeleton::b_01::bringup_module_state(g, k));
        game::math_dice::b_01::host_draws_set(&d);
        game::sensors::b_01::host_probe_pending_set(pending);
    }
    /// how many of the host's draws the Rust tick has taken
    #[no_mangle]
    pub extern "C" fn face_draws_taken() -> usize {
        game::math_dice::b_01::host_draws_count().0
    }
    #[no_mangle]
    pub extern "C" fn face_step(h: u32, dt: f64, pre: u32) {
        with(h, |g| {
            if pre & 1 != 0 {
                let d = g.dice.clone();
                game::math_dice::use_dice(g, d);
            }
            if pre & 2 != 0 {
                game::quiet::quiet_mission_current(g);
            }
            let cmd = g.cmd_buf.clone().expect("face_step: the state carries no record (Game.cmd_buf is None): export the TypeScript game after `update` has sampled one");
            game::game::game_step(g, dt, &cmd);
        });
    }
}
