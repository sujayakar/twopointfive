//! AN ORDER IS ONE MODULE AND ONE LINE.
//!
//! An order of the squad's menu is a `Row`: its id and the functions the menu and the tick call. An order written this
//! way is one file here (`orders/rows/<name>.rs`, with a `pub const ORDER: Row`) and one line of `rows!` below. Nothing
//! else names it.
//!
//!   label   the words of its row on the menu, for this target
//!   can     this man can do his part (None), or the word for why not
//!   only    the men, of those addressed, the order is for
//!   valid   these men together can carry it out here (Ok), or not (Err(None)), or the word for why (Err(Some))
//!   issue   hand each man his assignment; Some is a refusal, in those words
//!   tick    once a frame for each man under the order, before he is posed
//!   frame   once a frame for the order itself, after the menu's input: put its row on the menu or take it off
//!           (`def` makes the row), end what has gone stale
//!
//! The older orders are arms of the enums in orders/types.rs (OrderLabel, OrderCan, OrderOnly, OrderValid, OrderIssue,
//! OrderTick) with their bodies in the matches of orders/p_03.rs and p_05.rs. Each of those enums has one arm, `Row`,
//! through which every order written this way is called, so moving an older order here deletes its arms and adds none.
//! The first moved is the 40 mm shell (`shell40`), whose own code and state are still the module `squad40`.
pub use super::Row;
use crate::game::Game;
use crate::ids::*;
use crate::orders::{GiveOpts, OrderCan, OrderDef, OrderIssue, OrderLabel, OrderOnly, OrderTarget, OrderTick, OrderValid};
use alloc::string::String;
use alloc::vec::Vec;

macro_rules! rows {
    ($($m:ident;)*) => {
        $(pub mod $m;)*
        /// THE LIST: every order written as a module, in the order their `frame` runs.
        pub const ROWS: &[&Row] = &[$(&$m::ORDER),*];
    };
}
rows! {
    shell40;
}

/// The row of the menu for an order: `data` is what only it knows (its page, its targets, its key), and the functions
/// are the order's own.
pub fn def(r: &'static Row, data: OrderDef) -> OrderDef {
    OrderDef {
        id: String::from(r.id),
        label: OrderLabel::Row(r),
        can: r.can.map(|_| OrderCan::Row(r)),
        only: r.only.map(|_| OrderOnly::Row(r)),
        valid: r.valid.map(|_| OrderValid::Row(r)),
        issue: OrderIssue::Row(r),
        tick: r.tick.map(|_| OrderTick::Row(r)),
        ..data
    }
}

/// Once a frame for each seat, after the menu's own input: every order's `frame`, in the list's order.
pub fn frame(g: &mut Game) {
    for r in ROWS { if let Some(f) = r.frame { f(g); } }
}

/// the point of the floor an order's target names
pub fn at(t: &OrderTarget) -> crate::v3::V3 {
    match t { OrderTarget::Floor(x) => x.at, OrderTarget::Door(x) => x.at, OrderTarget::None(x) => x.at, OrderTarget::Foe(x) => x.at }
}

/// the order of that id, if it is written as a module
pub fn by_id(id: &str) -> Option<&'static Row> { ROWS.iter().copied().find(|r| r.id == id) }
