//! EVERY PART OF A GAME'S STATE, said once (game::state_words). What holds two games to each other (hold, the gate, a test that parts two machines) names and reads the same list, so a
//! part that joins the state joins every one of them.
use alloc::string::String;
use alloc::vec::Vec;
use game::game::Game;
use game::bytes::W;

/// the parts' names, in the order `words` gives them
pub fn names() -> Vec<String> { game::state_words::state_names() }

/// a word a part, into `out`. `o` is the writer the game's parts go through, kept by a caller that asks every frame.
pub fn words(g: &mut Game, o: &mut W, out: &mut Vec<u32>) {
    o.b.clear();
    game::state_words::state_words(g, o, out);
}

/// the parts' words as one number
pub fn fold(words: &[u32]) -> u64 {
    let mut all = 0xcbf2_9ce4_8422_2325u64;
    for x in words { all = (all ^ *x as u64).wrapping_mul(0x0000_0100_0000_01b3); }
    all
}

/// the names of the parts in which two rows of words differ
pub fn differ<'a>(names: &'a [String], a: &[u32], b: &[u32]) -> Vec<&'a str> { names.iter().zip(a.iter().zip(b)).filter(|(_, (x, y))| x != y).map(|(n, _)| n.as_str()).collect() }
