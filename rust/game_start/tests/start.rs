//! THE START ITSELF: a level of `levels/` stands and runs with nothing made by TypeScript, and a game stood again in the same process is the
//! same game (rust/sim keeps blocks and tables in statics: a stand must leave nothing of the game before it).
use game_start::play::{Opts, Play};

/// a hash of each part of rust/sim's block in slot 0, and the die's count
fn block() -> (Vec<u32>, f64) { (game_start::block::part_hashes(0), sim::state::state(0).dice.n) }

/// A real level's room clear, run for ten seconds: the block after it.
fn kit_yard(seed: u32) -> (Vec<u32>, f64) {
    let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
    assert_eq!(p.mates().len(), 4, "the room clear stages four teammates");
    assert!(p.hostiles_by([20.0, 14.0]).len() >= 4, "…and the level's garrison: {}", p.roll());
    p.run(10.0);
    assert!((p.time() - 10.0).abs() < 1e-6);
    block()
}

/// A FAILURE MEANS: a stand left something of the last game behind (a static table of rust/sim or of the game crate that a new game does not
/// make anew), so what a test sees depends on the tests that ran before it.
#[test]
fn a_game_stood_again_is_the_same_game() {
    let a = kit_yard(1);
    let other = kit_yard(2);
    let again = kit_yard(1);
    assert_ne!(a, other, "another seed is another game (or the die is not in play)");
    assert_eq!(a, again, "seed 1 stood, then seed 2, then seed 1 again: the first and the third are one game");
}

/// A level as its file has it, on the page's own stand (no mission started, the page's senses): what the compare tool and a host stand.
/// A FAILURE MEANS: the plain start of a real level (the constructor, the solo night's card, the host's first frames) no longer runs.
#[test]
fn a_level_stands_as_its_file_has_it() {
    let mut p = Play::level("kit-yard.json", false, &Opts { cpu_senses: false, ..Default::default() });
    assert!(p.mates().is_empty(), "no mission was started, so nobody was staged: {}", p.roll());
    p.run(10.0);
    assert!((p.time() - 10.0).abs() < 1e-6 && p.frames == 600);
}

/// A FAILURE MEANS: the reader of level files lost a record it carried, or a level file gained a record it does not carry: the list names it.
#[test]
fn the_start_says_what_it_does_not_carry() {
    let lacks = |file: &str| game_start::level::lacks_of(&game_start::json::parse(&game_start::files::level_text(file)).unwrap());
    assert!(lacks("kit-yard.json").is_empty(), "kit-yard.json is carried whole: {:?}", lacks("kit-yard.json"));
    assert!(lacks("office.json").iter().any(|l| l.starts_with("civilians")), "the office's civilians are named as not carried: {:?}", lacks("office.json"));
}
