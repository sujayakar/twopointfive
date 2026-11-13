//! THE THROWS' PAGE: the row that opens it, with what is left of each can.
use crate::game::Game;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String};

pub const ORDER: Row = Row {
    notes: Some(crate::squadthrow::notes),
    targets: &[TargetKind::Floor],
    opens: Some(crate::squadthrow::THROW_PAGE),
    free: true,
    ..Row::new("throwThere", label, rows::opens_a_page)
};

fn label(g: &mut Game, _: &OrderTarget) -> String {
    let frag = crate::squadthrow::left(g, crate::squadthrow::SquadThrowKind::Frag);
    let stun = crate::squadthrow::left(g, crate::squadthrow::SquadThrowKind::Stun);
    format!("throw · frag {} · bang {}{}", frag, stun, crate::squad40::squad40_word(g, &crate::seats::here(g)))
}
