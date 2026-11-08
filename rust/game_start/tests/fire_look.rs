//! A FIRE ON THE LOOK: what the look's table asks of an open fire reaches the page. Its heat parts the falling snow over it, and only where the
//! page has particles. A failure means the fire's hot air never leaves the game, and a snowfall falls through the flame.
use game::out_events::OutEvent;
use game_start::play::{Opts, Play};
use game_start::stand;

const DT: f64 = 1.0 / 60.0;

/// Every source of heat the game told the page of in `frames` frames.
fn heat_told(frames: usize) -> usize {
    let mut p = Play::level("pass.json", false, &Opts::default());
    assert!((0..p.g.targets.len()).any(|i| game::fire::is_fire(&p.g, p.g.targets[i])), "the pass has a brazier");
    let mut told = 0;
    for _ in 0..frames {
        for e in stand::frame(&mut p.g, &p.host, DT) {
            if let OutEvent::ParticlesSetHeat(h) = e { told += h.srcs.len(); }
        }
    }
    told
}

#[test]
fn a_fire_tells_the_page_where_its_hot_air_rises() {
    assert!(heat_told(30) > 0, "a fire in view told the page's particles of no hot air");
}
