//! WHAT THE GAME SAID OF THE PICTURE'S SMOKE: all of it goes, or the cans stop. A restart is the picture's cut, and says nothing here.
use alloc::vec::Vec;
use game::game::Game;
use game::out_events::OutEvent;

game::bits! {
    /// what of the picture's smoke begins again
    pub mod reset_flag as ResetFlag {
        /// all of it goes
        SMOKE = 1,
        /// the cans venting stop, what they let out left in the air
        CANS = 2,
    }
}
event! {
    /// the tick began a part of the picture again
    pub struct Reset {
        /// ResetFlag
        pub what: u32,
    }
}

pub fn fill(g: &Game, out: &mut Vec<Reset>) {
    out.clear();
    let what = g.out.iter().fold(0, |w, e| w | match e {
        OutEvent::SmokeSystemLikeClearAll => reset_flag::SMOKE,
        OutEvent::SmokeCansShut => reset_flag::CANS,
        _ => 0,
    });
    if what != 0 { out.push(Reset { tick: crate::tick(g), serial: 0, what }); }
}
