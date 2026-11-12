//! GO LOUD: the whole squad. It stands in silent's place while the squad is silent.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::string::String;

pub const ORDER: Row = Row {
    only: Some(crate::silent::row_whole_squad),
    when: Some(when),
    ..Row::new("loud", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("go loud") }

fn issue(g: &mut Game, _: &[GuardId], _: &OrderTarget, _: Option<&GiveOpts>) -> Option<String> {
    crate::silent::set_silent(g, false);
    None
}

/// in silent's place while the squad is silent
fn when(g: &Game, seat: &crate::seats::SeatRef, _t: &OrderTarget, _men: &[GuardId]) -> bool { crate::silent::squad_silent(g, seat) }
