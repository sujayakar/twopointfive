//! THE WALL TOOLS' PAGE: the row that opens it.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

/// it opens the rows of its own name
const ID: &str = "walltools";
pub const ORDER: Row = Row {
    targets: &[TargetKind::Floor],
    listed: Some(&[]),
    opens: Some(ID),
    ..Row::new(ID, label, rows::opens_a_page)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("wall tools") }
