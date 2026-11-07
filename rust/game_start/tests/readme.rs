//! The example of README.md ("Write a behaviour test"), word for word: `the_readme_shows_this_file` holds the two equal, so the README's
//! example compiles and passes for as long as this file does.
use game::game::GuardState;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

#[test]
fn a_guard_who_hears_a_shot_behind_a_shut_door_goes_to_it() {
    // two lit rooms with a shut door between them; a sentry with his back to it, Sam next door
    let mut s = Scene::new("shot-next-door");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 12.4).door("east-yard", Axis::X, 16.0, 21.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0);
    s.sentry("post", [12.25, 13.25], [10.0, 13.25]).sam(23.75, 13.25);

    let mut p = Play::scene(&s, &Opts { seed: 1, ..Default::default() });
    let post = p.guard("post");
    p.run(1.0);
    p.sam_fires_at([25.9, 13.25], 0.6);                               // one rifle round into the far wall
    p.until(1.0, |p| p.state(post) == GuardState::Alert).expect("he hears it");
    p.until(5.0, |p| p.at(post)[0] > 15.0).expect("and goes to the door");
}

#[test]
fn the_readme_shows_this_file() {
    let readme = include_str!("../README.md");
    let block = readme.split("```rust\n").nth(1).and_then(|b| b.split("```").next()).expect("README.md has a rust block");
    let me = include_str!("readme.rs");
    assert!(me.contains(block), "the first rust block of README.md is not this file's example, line for line:\n{}", block);
}
