//! WHAT A VIEWER'S OWN PICTURE SHOWS THAT NO LIST OF THE GAME'S HOLDS: what the men the fog leaves out carry, and the infrared lines of the players where the run's own picture was no tube. They are made by the
//! rules a step makes the others by, on a copy of the game that is thrown away: the game is read, and nothing of it is written. Only a picture that asks pays for it.
use alloc::vec::Vec;
use game::game::Game;
use game::ids::AnyBoxId;
use game::scene_boxes::Box;

/// the boxes, each with whether it is an infrared line. The world's copy is asked of (a line's end): whoever calls holds the game, and has it proven again
pub fn boxes(g: &Game, bare: bool, tube: bool) -> Vec<(Box, bool)> {
    let tube = tube && !game::irlaser::ir_viewer(g);
    if !bare && !tube { return Vec::new(); }
    let mut c = g.clone();
    let (mut hidden, mut lines): (Vec<AnyBoxId>, Vec<AnyBoxId>) = (Vec::new(), Vec::new());
    if bare { game::combat::hidden_props(&mut c, &mut hidden); }
    if tube { game::irlaser::ir_beams(&mut c, &mut lines); }
    let row = |id: &AnyBoxId| c.tab[*id].clone();
    hidden.iter().map(|b| (row(b), false)).chain(lines.iter().map(|b| (row(b), true))).collect()
}
