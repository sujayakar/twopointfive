//! THE WALL TOOLS' PAGE: the row that opens it.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row::new(crate::wallcharge::WALL_PAGE, label, rows::opens_a_page);

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("wall tools") }
