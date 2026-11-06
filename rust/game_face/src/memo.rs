//! THE FILTERED EXPORT, ONE WAY FOR EVERY HOST (the page: src/host/rustgame.ts; the headless host: tools/blitz/rust-host.ts). A host that draws or
//! reads a puppet takes the state as `puppet::export_puppet` filters it: the long list once, then only what changed. What "changed" is held to is the
//! MEMO, one a game, kept here by the game's handle.
//!
//!   face_puppet(h, start)        the FILTERED export at face_out: `start` = 1 the long list with every row, which also fills the memo the every-tick
//!                                export is held to; 0 = what changed since the last call. The tick's out events ride on its tail and are cleared
//!                                here (the list is a step's output: a second export must not hand them again).
//!   face_puppet_warm(h)          the every-tick export with every row and field sent as if it had changed (what the memo holds is spoiled first): a
//!                                host applies it to a puppet that already holds that state, to run the every-tick path once over everything
//!   face_puppet_drop(h)          the memo of a game that is gone
//!   face_reserve(n)              room asked for and given back, so that no step has to grow the memory to find it
use std::cell::RefCell;
use std::collections::HashMap;
use game::puppet::PuppetMemo;

thread_local! {
    /// what each game's host was last sent, by handle: the every-tick export sends only what differs from it
    static MEMO: RefCell<HashMap<u32, PuppetMemo>> = RefCell::new(HashMap::new());
}

/// a game's host has been sent nothing yet (a new game under a handle; a game stood again under its old one)
pub fn forget(h: u32) {
    MEMO.with(|m| m.borrow_mut().remove(&h));
}

/// one export against a game's memo
fn export(h: u32, start: bool, spoil: bool) -> Vec<u8> {
    let mut memo = MEMO.with(|m| m.borrow_mut().remove(&h)).unwrap_or_default();
    if start { memo = Default::default(); }
    // (a row or a field with no bytes on the every-tick list is never sent, and stays so)
    if spoil { memo.all = true; }
    let b = super::with(h, |x| game::puppet::export_puppet(&x.g, &mut memo, start));
    MEMO.with(|m| m.borrow_mut().insert(h, memo));
    b
}

#[no_mangle]
pub extern "C" fn face_puppet(h: u32, start: u32) {
    let b = export(h, start != 0, false);
    super::with(h, |x| x.g.out.clear());
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_puppet_warm(h: u32) {
    let b = export(h, false, true);
    super::out(b);
}

#[no_mangle]
pub extern "C" fn face_puppet_drop(h: u32) {
    forget(h);
}

#[no_mangle]
pub extern "C" fn face_reserve(n: usize) {
    drop(Vec::<u8>::with_capacity(n.max(1)));
}
