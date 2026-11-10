//! WHAT THE GAME SAID OF THE PICTURE'S SMOKE: all of it goes, or the cans stop. A restart is the picture's cut, and says nothing here.
use alloc::vec::Vec;
use game::game::Game;
use game::out_events::OutEvent;

event! {
    /// the tick began a part of the picture again
    pub struct Reset {
        /// 1 the smoke · 2 the cans venting stop, what they let out left in the air
        pub what: u32,
    }
}

pub fn fill(g: &Game, out: &mut Vec<Reset>) {
    out.clear();
    let what = g.out.iter().fold(0, |w, e| w | match e {
        OutEvent::SmokeSystemLikeClearAll => 1,
        OutEvent::SmokeCansShut => 2,
        _ => 0,
    });
    if what != 0 { out.push(Reset { tick: crate::tick(g), serial: 0, what }); }
}
