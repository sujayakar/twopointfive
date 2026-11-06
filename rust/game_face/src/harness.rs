//! WHAT A HOST THAT PLAYS HEADLESS ASKS OF THE FACE (tools/blitz/rust-host.ts, tools/blitz/PLAY.md) beside the filtered export (memo.rs): the few
//! things the host's TypeScript does between two ticks that the Rust game has to do too. Exports only: every rule is the crate `game`'s, every
//! host step `game_run`'s.
//!
//!   face_follow(h)               the host's camera asked the game where to look (`game_run::follow`): after a tick, for every seat
//!   face_look(h) -> p                six numbers: where the seat's camera is to follow, and the seat's aim point (a host stepping without an apply)
//!   face_room_at(h, x, y, z) -> 1 | 0   the driver's question that floods a room and keeps it (`roomfight.roomAt` between two ticks): 1 where it
//!                                flooded one (the list of rooms grew)
//!   face_stop_program(h) -> 1 | 0   a level on rails whose program runs: the program is stopped and the rails told to start it again on the next tick
//!                                (savestate.ts:991, what the save controller does before it takes an authored checkpoint); 1 where it did. A host
//!                                whose own save controller takes the checkpoint calls this first, so that what it keeps is the floor with no program
#[no_mangle]
pub extern "C" fn face_follow(h: u32) {
    super::with(h, |x| game_run::follow(&mut x.g));
}
thread_local! { static LOOK: core::cell::Cell<[f64; 6]> = const { core::cell::Cell::new([0.0; 6]) }; }
/// WHERE THE HOST'S CAMERA IS TO LOOK, for a host that steps without applying the state (a quiet run of steps: tools/blitz/PLAY.md): the seat on the
/// Game's place to follow (`Game.followPos`) and its aim point, six numbers. The host's camera needs these two after every tick and nothing else of the
/// game; with them its next record is the one it would have made off an applied puppet.
#[no_mangle]
pub extern "C" fn face_look(h: u32) -> *const f64 {
    super::with(h, |x| {
        let g = &mut x.g;
        let p = game::game::game_follow_pos(g, None);
        let a = g.aim_point;
        LOOK.with(|c| { c.set([p[0], p[1], p[2], a[0], a[1], a[2]]); c.as_ptr() as *const f64 })
    })
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
