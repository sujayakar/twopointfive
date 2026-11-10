//! The desk's two calls (game_start `host::desk`): what the cards before a game show and press, with no game started.
//!
//!   face_desk_machine(p, n) -> 1 | 0   what this machine keeps and what the address says (two texts, each its length and its bytes), laid on the desk as a start lays them. 0 = a
//!                                  record that is none (said at face_out)
//!   face_desk_ask(p, n) -> 1 | 0   one ask, its answer's bytes at face_out
use game_start::host::command::{Bytes, Field};
use game_start::host::desk::Desk;
use std::cell::RefCell;

thread_local! { static DESK: RefCell<Option<Box<Desk>>> = const { RefCell::new(None) }; }
fn desk<T>(f: impl FnOnce(&mut Desk) -> T) -> T { DESK.with(|d| f(d.borrow_mut().get_or_insert_with(|| Box::new(Desk::new())))) }

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_desk_machine(p: *const u8, n: usize) -> u32 {
    super::hook();
    let mut r = Bytes::of(super::bytes(p, n));
    let (kept, address) = (<&str as Field>::read(&mut r), <&str as Field>::read(&mut r));
    match desk(|d| d.machine(kept, address)) { Ok(()) => 1, Err(why) => { super::out(why.into_bytes()); 0 } }
}

/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_desk_ask(p: *const u8, n: usize) -> u32 {
    super::hook();
    let b = super::bytes(p, n);
    match desk(|d| d.ask(b)) { Ok(a) => { super::out(a); 1 } Err(why) => { super::out(why.into_bytes()); 0 } }
}
