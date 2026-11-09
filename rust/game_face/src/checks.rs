//! FOR A CHECK OR A TOOL, and no host that plays.
//!
//!   face_debug_panic(h, how)           A PLANTED PANIC, for the check of the road back from one: how 0 inside the borrow of game `h`, as a rule of the tick would (the game is lost);
//!                                      how 1 outside any game (nothing is lost but the call)
//!   face_kit_plan(p, n, way) -> 1 | 0  of no game: `p`, `n` are a level file's text, `way` a way of its kit picture's block; 1: the plan as JSON at face_out
//!; 0: why not, in words
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
pub unsafe extern "C" fn face_kit_plan(p: *const u8, n: usize, way: u32) -> u32 {
    super::hook();
    let answer = core::str::from_utf8(super::bytes(p, n)).map_err(|e| format!("{}", e)).and_then(game_start::json::parse).map(|file| game_start::kit_stage::plan_of_file(&file, way as usize));
    match answer {
        Ok(j) if !j.is_null() => { super::out(game_start::json::to_line(&j).into_bytes()); 1 }
        Ok(_) => { super::out(format!("way {}: no block, or no such way", way).into_bytes()); 0 }
        Err(why) => { super::out(why.into_bytes()); 0 }
    }
}
