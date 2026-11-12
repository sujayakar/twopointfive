//! TWO GAMES THAT STAND AT ONCE DO NOT TOUCH EACH OTHER: each is the game it is alone. A host of several games (a server, a viewer beside a game) has them side by side.
//! (A file of its own: these stands hold no `Play`, so they must not run beside the tests that do.)
use game::bytes::{ToBytes, W};
use game_start::stand::{self, StartOpts, Stood};
use game_start::{files, json};

fn step(s: &mut Stood, frames: usize) { for _ in 0..frames { stand::frame(&mut s.game, &s.host, 1.0 / 60.0); } }
/// the game's own bytes, whole
fn read(s: &Stood) -> Vec<u8> {
    let mut o = W::new();
    s.game.w(&mut o);
    o.b
}

/// A FAILURE MEANS: something of a game is kept outside the game (a static of rust/sim or of the game crate that another game reads or writes).
#[test]
fn a_game_is_the_same_alone_and_beside_another() {
    let ch = files::CharacterFiles::read();
    let levels = [("kit-yard.json", "?levelFile=kit-yard.json&roomclear&squad4"), ("killhouse.json", "?levelFile=killhouse.json&roomclear&squad4")];
    let at = |i: usize| { let (level, link) = levels[i % 2]; let data = json::parse(&files::level_text(level)).expect(level); stand::stand(&data, ch.assets(), &StartOpts { seed: 7 + i as u64 as u32 % 3, link: Some(link.into()), ..Default::default() }) };
    let alone = |i: usize| { let mut s = at(i); step(&mut s, 300); read(&s) };
    let want: Vec<Vec<u8>> = (0..6).map(alone).collect();
    // six at once, on two levels and three seeds, stepped turn about
    let mut games: Vec<Stood> = (0..6).map(at).collect();
    for _ in 0..300 { for g in &mut games { step(g, 1); } }
    for (i, g) in games.iter().enumerate() { assert!(read(g) == want[i], "game {i} of six, stepped turn about, is not the game it is alone"); }
}
