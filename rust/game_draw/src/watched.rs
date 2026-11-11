//! WHAT A VIEWER'S OWN PICTURE SHOWS THAT NO LIST OF THE GAME'S HOLDS: what the men the fog leaves out carry, and the infrared lines of the players where the run's own picture was no tube. They are made by the
//! rules a step makes the others by. Only a picture that asks pays for it.
use alloc::vec::Vec;
use game::game::Game;
use game::rig_props::{Carried, CarriedBox};

/// the boxes; an infrared line is marked as one
pub fn boxes(g: &Game, bare: bool, tube: bool) -> Vec<CarriedBox> {
    let mut list = Carried::default();
    if bare { game::combat::hidden_props(g, &mut list); }
    if tube && !game::irlaser::ir_viewer(g) { game::irlaser::ir_beams(g, &mut list); }
    list.boxes
}
