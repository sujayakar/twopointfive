//! GO SILENT: the whole squad.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    only: Some(crate::silent::row_whole_squad),
    ..Row::new("silent", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("go silent") }

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    crate::silent::set_silent(g, true, Some(crate::silent::By::Order));
    None
}
