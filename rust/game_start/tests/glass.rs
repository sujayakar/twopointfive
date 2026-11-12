//! A PANE THAT BREAKS BEFORE THE FIRST FRAME'S UPDATE GOES THE WAY EVERY OTHER PANE GOES.
//!
//! The glass's fragments reach the shards model by a hook the model's update used to set, so a blast before the first update found no hook and
//! took a road of its own: a thin box made for the piece, read back as a fragment, and an answer naming "the piece" that was a row kept for
//! the purpose. A game handed over by the page's stand has no such row, and the first question of it panicked (the pass, a blast by a window).
use game_start::play::{Opts, Play};

/// a window of the pass's outer wall, and the same stand as the page's: nothing has stepped yet
const WINDOW: [f64; 3] = [13.65, 1.375, 11.0];

/// THE RULE: a blast by a window before anything has stepped breaks the window, and its pieces go to the shards like any others.
/// A FAILURE MEANS: there is a second road out of a broken pane, one that asks for a row of the piece table nobody promised.
#[test]
fn a_pane_broken_before_the_first_frame_goes_as_any_other() {
    let mut p = Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed: 1, ..Default::default() });
    p.g.tab.debris_chunk = Default::default();   // (what the page's stand hands over: the table holds the pieces, and no piece has come off yet)
    (p.g.k.chaos.def, p.g.k.tune.wall_break) = (0.5, false);
    let was = p.g.m.glassbreak.gb_host.hooked;
    game::destruction::destruction_blast(&mut p.g, [WINDOW[0], WINDOW[1], WINDOW[2] - 0.5], Some(game::wallbreak::BlastOpts { radius: Some(2.4), who: Some(game::consts::PLAYER_ID), ..Default::default() }));
    p.run(0.5);
    let hooked = p.g.m.glassbreak.gb_host.hooked - was;
    assert!(hooked > 0.0, "the staging is wrong: no fragment of the window was let go");
    assert_eq!(p.g.m.glassbreak.gb_host.shards, 0.0, "a fragment went by the stand-in with the shards model live");
}
