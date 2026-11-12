//! THE WALL TOOLS' PAGE: the row that opens it.
use crate::game::Game;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::string::String;

pub const ORDER: Row = Row::new("walltools", label, rows::opens_a_page);

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("wall tools") }
