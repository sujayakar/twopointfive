//! THE HELD THRESHOLD'S PAGE: the row that opens it. It has no `when`: its data says it stands in for the tools, and it never does (from outside, a held door's word is the key's).
use crate::game::Game;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::string::String;

/// it opens the rows of its own name
const ID: &str = "held";
pub const ORDER: Row = Row {
    targets: &[TargetKind::Door],
    opens: Some(ID),
    stands_in: Some(crate::doortools::TOOLS_PAGE),
    ..Row::new(ID, label, rows::opens_a_page)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("threshold held") }
