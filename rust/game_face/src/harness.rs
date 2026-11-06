//! WHAT A HOST THAT PLAYS HEADLESS ASKS OF THE FACE (tools/blitz/rust-host.ts, tools/blitz/PLAY.md): the filtered state a tick, the tick's out events, and
//! the few things the host's TypeScript does between two ticks that the Rust game has to do too. Exports only: every rule is the crate `game`'s, every
//! host step `game_run`'s.
//!
//!   face_puppet(h, start)        the FILTERED export (`puppet::export_puppet`) at face_out: `start` = 1 the long list with every row, which also fills
//!                                the memo the every-tick export is held to; 0 = what changed since the last call. The tick's out events ride on its tail
//!                                and are cleared here (the list is a step's output: a second export must not hand them again).
//!   face_puppet_warm(h)          the every-tick export with every row and field sent as if it had changed (what the memo holds is spoiled first): a
//!                                host applies it to a puppet that already holds that state, to run the every-tick path once over everything
//!   face_follow(h)               the host's camera asked the game where to look (`game_run::follow`): after a tick, for every seat
//!   face_cuts(h)                 for each seat four f64 at face_out: its lens's `desired` and the count of whole writes of it (`desired_serial`), which is
//!                                how a host knows a cut was made (cmd.ts `takeCut` knows it by the array's identity, which no export carries)
//!   face_room_at(h, x, y, z) -> 1 | 0   the driver's question that floods a room and keeps it (`roomfight.roomAt` between two ticks): 1 where it
//!                                flooded one (the list of rooms grew)
//!   face_stop_program(h) -> 1 | 0   a level on rails whose program runs: the program is stopped and the rails told to start it again on the next tick
//!                                (savestate.ts:991, what the save controller does before it takes an authored checkpoint); 1 where it did. A host
//!                                whose own save controller takes the checkpoint calls this first, so that what it keeps is the floor with no program
//!   face_reserve(n)              room asked for and given back, so that no step has to grow the memory to find it
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// what each game's host was last sent, by handle: the every-tick export sends only what differs from it
    static MEMO: RefCell<HashMap<u32, game::puppet::PuppetMemo>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub extern "C" fn face_puppet(h: u32, start: u32) {
    let mut memo = MEMO.with(|m| m.borrow_mut().remove(&h)).unwrap_or_default();
    if start != 0 { memo = Default::default(); }
    let b = super::with(h, |x| {
        let b = game::puppet::export_puppet(&x.g, &mut memo, start != 0);
        x.g.out.clear();
        b
    });
    MEMO.with(|m| m.borrow_mut().insert(h, memo));
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_puppet_warm(h: u32) {
    let mut memo = MEMO.with(|m| m.borrow_mut().remove(&h)).unwrap_or_default();
    // (a row or a field with no bytes on the every-tick list is never sent, and stays so)
    for v in memo.top.iter_mut().chain(memo.rows.iter_mut().flatten()) { if !v.is_empty() { v.push(0xff); } }
    let b = super::with(h, |x| game::puppet::export_puppet(&x.g, &mut memo, false));
    MEMO.with(|m| m.borrow_mut().insert(h, memo));
    super::out(b);
}
/// the memo of a game that is gone (face_drop does not know of it)
#[no_mangle]
pub extern "C" fn face_puppet_drop(h: u32) {
    MEMO.with(|m| m.borrow_mut().remove(&h));
}
#[no_mangle]
pub extern "C" fn face_follow(h: u32) {
    super::with(h, |x| game_run::follow(&mut x.g));
}
#[no_mangle]
pub extern "C" fn face_cuts(h: u32) {
    let b = super::with(h, |x| {
        let g = &x.g;
        let mut b = Vec::with_capacity(g.seats.len() * 32);
        for s in &g.seats {
            let c = &g.tab.camera[g.tab.cmd_seat[*s].cam];
            for v in c.desired { b.extend_from_slice(&v.to_le_bytes()); }
            b.extend_from_slice(&(c.desired_serial as f64).to_le_bytes());
        }
        b
    });
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_room_at(h: u32, x: f64, y: f64, z: f64) -> u32 {
    super::with(h, |held| {
        let n0 = held.g.fight.rooms.len();
        let _ = game::roomfight::room_fight_room_at(&mut held.g, [x, y, z]);
        (held.g.fight.rooms.len() != n0) as u32
    })
}
#[no_mangle]
pub extern "C" fn face_stop_program(h: u32) -> u32 {
    super::with(h, |x| {
        let g = &mut x.g;
        if !game::rails::rails_live(g) { return 0; }
        let Some(r) = g.director.run else { return 0 };
        if g.tab.director_run[r].ended { return 0; }
        game::director::director_stop(g, Some(false));
        game::rails::rails_resume(g);
        1
    })
}
#[no_mangle]
pub extern "C" fn face_reserve(n: usize) {
    drop(Vec::<u8>::with_capacity(n.max(1)));
}
