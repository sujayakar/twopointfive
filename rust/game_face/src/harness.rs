//! WHAT A HOST THAT PLAYS HEADLESS ASKS OF THE FACE (tools/blitz/rust-host.ts, tools/blitz/PLAY.md) beside the filtered export (memo.rs): the few
//! things the host's TypeScript does between two ticks that the Rust game has to do too. Exports only: every rule is the crate `game`'s, every
//! host step `game_run`'s.
//!
//!   face_follow(h)               the host's camera asked the game where to look (`game_run::follow`): after a tick, for every seat
//!   face_view_live(h)            after the host's half is taken: the view's matrix is the host camera's as the host left it (`game_run::view_live`), so a
//!                                call made between two ticks (an order: on-screen-or-not) reads the camera of now, not the record of the tick before
//!   face_look(h) -> p                six numbers: where the seat's camera is to follow, and the seat's aim point (a host stepping without an apply)
//!   face_room_at(h, x, y, z) -> 1 | 0   the driver's question that floods a room and keeps it (`roomfight.roomAt` between two ticks): 1 where it
//!                                flooded one (the list of rooms grew)
//!   face_kit_plan(p, n, way) -> 1 | 0   a read for tools, of no game: `p`, `n` are a level file's text, `way` a way of its kit picture's block; 1: the plan as JSON at face_out
//!                                (`game_start::kit_stage::plan_of_file`, tools/qa/kitview-plan.ts); 0: why not, in words, at face_out
#[no_mangle]
pub extern "C" fn face_follow(h: u32) {
    super::with(h, |x| game_run::follow(&mut x.g));
}
#[no_mangle]
pub extern "C" fn face_view_live(h: u32) {
    super::with(h, |x| game_run::view_live(&mut x.g));
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
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_kit_plan(p: *const u8, n: usize, way: u32) -> u32 {
    super::hook();
    let answer = core::str::from_utf8(super::bytes(p, n)).map_err(|e| format!("{}", e)).and_then(game_start::json::parse).map(|file| game_start::kit_stage::plan_of_file(&file, way as usize));
    match answer {
        Ok(j) if !j.is_null() => { super::out(game_start::json::to_line(&j).into_bytes()); 1 }
        Ok(_) => { super::out(format!("way {}: no block, or no such way", way).into_bytes()); 0 }
        Err(why) => { super::out(why.into_bytes()); 0 }
    }
}
