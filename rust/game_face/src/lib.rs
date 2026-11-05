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
}
