//! A GAME IS THE SAME GAME IN ANY OF rust/sim's SLOTS, and two that stand at once do not touch each other. The page's module stands a game
//! wherever a slot is free (`face_start`), and a host of several games (a server, the probe runner) has them side by side.
//! (A file of its own: these stands hold no `Play`, so they must not run beside the tests that do.)
use game_start::stand::{self, StartOpts, Stood};
use game_start::{block, files, json};

const LINK: &str = "?levelFile=kit-yard.json&roomclear&squad4";
fn opts(seed: u32, slot: usize) -> StartOpts { StartOpts { seed, slot, link: Some(LINK.into()), cpu_senses: true, ..Default::default() } }
fn step(s: &mut Stood, frames: usize) { for _ in 0..frames { stand::frame(&mut s.game, &s.host, 1.0 / 60.0); } }
/// the game as the bridge writes it, and a hash of each part of its block. The box mirror is left out of the game's bytes: it holds where its
/// tables stand in memory, which is the slot's own (its tables themselves are in the block, and the block is compared whole)
fn read(s: &Stood, slot: usize) -> (Vec<u8>, Vec<u32>) {
    let mut g = s.game.clone();
    g.col.mirror = game::scene_box_mirror::BoxMirror::blank();
    (game::io::export(&g), block::part_hashes(slot))
}

/// A FAILURE MEANS: something of a game is kept by the slot's number, or outside the game and its block (a static of rust/sim or of the game
/// crate that a game in another slot reads or writes).
#[test]
fn a_game_is_the_same_in_another_slot_and_alone_beside_another() {
    let ch = files::CharacterFiles::read();
    let data = json::parse(&files::level_text("kit-yard.json")).expect("kit-yard.json");
    let alone = |seed: u32, slot: usize| { let mut s = stand::stand(&data, ch.assets(), &opts(seed, slot)); let start = read(&s, slot); step(&mut s, 300); (start, read(&s, slot)) };
    let (start0, end0) = alone(7, 0);
    let (start1, end1) = alone(7, 1);
    assert!(start0 == start1, "the start in slot 1 is not the start in slot 0");
    assert!(end0 == end1, "five seconds on, the game in slot 1 is not the game in slot 0");
    // two at once, stepped turn about: each is the game it is alone
    let other = alone(8, 0).1;
    assert!(other != end0, "another seed is another game (or the die is not in play)");
    let mut a = stand::stand(&data, ch.assets(), &opts(7, 0));
    let mut b = stand::stand(&data, ch.assets(), &opts(8, 1));
    for _ in 0..300 { step(&mut a, 1); step(&mut b, 1); }
    assert!(read(&a, 0) == end0, "seed 7 in slot 0, stepped beside seed 8 in slot 1, is not the game it is alone");
    assert!(read(&b, 1) == other, "seed 8 in slot 1, stepped beside seed 7 in slot 0, is not the game it is alone");
}
