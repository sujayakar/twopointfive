//! A game stands with the light's lines made: its first step, on any host, follows none.
use game_start::play::{Opts, Play};

#[test]
fn the_first_step_follows_no_lines() {
    let mut p = Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed: 1, ..Default::default() });
    let mut most = 0;
    for _ in 0..12 { p.frame(); most = most.max(sim::glow::swept()[1]); }
    assert_eq!(most, 0, "the first steps followed the lines of {most} metres the stand had not made");
}
