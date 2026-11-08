//! THE DOOR TOOLS' PAGE: the row that opens it.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row { notes: Some(notes), ..Row::new("tools", label, rows::opens_a_page) };

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("door tools") }

/// the door tools' lines in the block
fn notes(g: &Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::Note> {
    crate::doortools::door_tools_hud_notes(g, seat).into_iter().map(|text| crate::view::Note { key: String::new(), text }).collect()
}
