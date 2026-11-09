//! The desk's two calls (game_start `host::desk`): what the cards before a game show and press, with no game started.
//!
//!   face_desk_do(p, n) -> 1 | 0    the host's commands done to the desk: what this machine keeps and what the address says, as a start is done them. 0 = a
//!                                  number no command has (said at face_out)
//!   face_desk_ask(p, n) -> 1 | 0   one ask, its answer's bytes at face_out
use game_start::host::desk::Desk;
use std::cell::RefCell;

thread_local! { static DESK: RefCell<Option<Box<Desk>>> = const { RefCell::new(None) }; }
fn desk<T>(f: impl FnOnce(&mut Desk) -> T) -> T { DESK.with(|d| f(d.borrow_mut().get_or_insert_with(|| Box::new(Desk::new())))) }

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_desk_do(p: *const u8, n: usize) -> u32 {
    super::hook();
    let b = super::bytes(p, n);
    match desk(|d| d.machine(b)) { Ok(()) => 1, Err(why) => { super::out(why.into_bytes()); 0 } }
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_desk_ask(p: *const u8, n: usize) -> u32 {
    super::hook();
    let b = super::bytes(p, n);
    match desk(|d| d.ask(b)) { Ok(a) => { super::out(a); 1 } Err(why) => { super::out(why.into_bytes()); 0 } }
}
