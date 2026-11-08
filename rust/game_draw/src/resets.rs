//! WHAT THE GAME SAID OF THE PICTURE'S OWN HISTORIES: the level began again, and what the picture holds of it (its histories, its particles, its smoke, the snow's marks) is another moment's.
use alloc::vec::Vec;
use game::game::Game;
use game::out_events::OutEvent;

event! {
    /// the tick began a part of the picture again
    pub struct Reset {
        /// 1 the level: every history · 2 the smoke · 4 the snow's blasts · 8 the things' marks · 16 the prints · 32 the cans venting stop, what they let out left in the air
        pub what: u32,
    }
}

pub fn fill(g: &Game, out: &mut Vec<Reset>) {
    out.clear();
    let what = g.out.iter().fold(0, |w, e| w | match e {
        OutEvent::HostApiRestartPicture(_) => 1,
        OutEvent::SmokeSystemLikeClearAll => 2,
        OutEvent::ClearSnowBlast => 4,
        OutEvent::ClearSnowMarks => 8,
        OutEvent::ClearSnowTracks => 16,
        OutEvent::SmokeCansShut => 32,
        _ => 0,
    });
    if what != 0 { out.push(Reset { tick: crate::tick(g), serial: 0, what }); }
}
