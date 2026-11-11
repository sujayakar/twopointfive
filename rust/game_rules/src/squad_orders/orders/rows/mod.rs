//! AN ORDER IS ONE MODULE AND ONE LINE.
//!
//! An order of the squad's menu is a `Row`: its id and the functions the menu and the tick call. An order is one file
//! here (`orders/rows/<name>.rs`, with a `pub const ORDER: Row`) and one line of `rows!` below. `OrderDef.row` is the
//! row the menu holds it by.
//!
//!   is      what a rule that does not know it by name needs to know of it (`Is`: is it loud, does it lift the held fire, ...)
//!   label   the words of its row on the menu, for this target
//!   can     this man can do his part (None), or the word for why not
//!   only    the men, of those addressed, the order is for
//!   valid   these men together can carry it out here (Ok), or not (Err(None)), or the word for why (Err(Some))
//!   gone    the ring does not offer it on this target at all
//!   when    it stands in now for the row its data names (`OrderDef.stands_in`)
//!   issue   hand each man his assignment; Some is a refusal, in those words
//!   tick    once a frame for each man under the order, before he is posed
//!   frame   once a frame for the order itself, after the menu's input: put its row on the menu or take it off
//!           (`def` makes the row), end what has gone stale
//!
//!   notes   its lines in the squad's HUD block, for this seat (the block comes in the list's order)
//!
//! Only `id`, `label` and `issue` must be written: `Row { frame: Some(frame), ..Row::new(ID, label, issue) }`.
//!
//! What an order's functions call, so that its id is written once, in its `Row`:
//!   `list(g, &ORDER, data)`    its row on the menu (`data`: its page, its targets, its words of assent). Listing again is the same row.
//!   `unlist(g, &ORDER)`        off the menu
//!   `listed(g, seat, &ORDER)`  is it on this seat's menu
//!   `assign(g, &ORDER, man, a)`  hand a man his task under this order: where he stands, which way he faces, how he carries his gun, the two
//!                              lines he says (`squad_orders_assign`, with the order's own id)
//!
pub use super::{Is, Row};
use crate::game::Game;
use crate::ids::*;
use crate::orders::{GiveOpts, OrderDef, OrderTarget};
use alloc::string::String;
use alloc::vec::Vec;

macro_rules! rows {
    ($($m:ident;)*) => {
        $(pub mod $m;)*
        /// THE LIST: every order written as a module, in the order their `frame` runs and their lines come in the squad's HUD block.
        pub const ROWS: &[&Row] = &[$(&$m::ORDER),*];
    };
}
rows! {
    movethere;
    stack;
    cover;
    fallin;
    hold;
    tools;
    held;
    openclear;
    bangclear;
    charge;
    fragclear;
    threshold;
    go;
    bigclear;
    thrword;
    pairmark;
    pairwatch;
    pairtake;
    pairdark;
    pairgo;
    silent;
    loud;
    shutdoor;
    wand;
    work;
    smokethere;
    wallrow;
    walltools;
    shell40;
    throwopen;
    fragthere;
    stunthere;
}

/// The row of the menu for an order: `data` is what only it knows (its page, its targets), and the functions
/// are the order's own.
pub fn def(r: &'static Row, data: OrderDef) -> OrderDef { OrderDef { id: String::from(r.id), row: r, ..data } }

/// Put the order's row on the menu. Listing it again is the same row (`squad_orders_row`), in the place the menu deals it.
pub fn list(g: &mut Game, r: &'static Row, data: OrderDef) -> OrdersOrderDefId {
    let d = crate::orders::squad_orders_row(g, def(r, data));
    crate::orders::squad_orders_register(g, d, None);
    d
}

/// take the order's row off the menu
pub(in crate::squad_orders) fn unlist(g: &mut Game, r: &Row) { crate::orders::squad_orders_unregister(g, r.id) }

/// Hand a man his task under this order: `a` with the order's own id (whatever `a.order` held).
pub(in crate::squad_orders) fn assign(g: &mut Game, r: &Row, gd: GuardId, a: crate::orders::SquadOrdersAssignA) {
    crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA { order: String::from(r.id), ..a });
}

/// Once a frame for each seat, after the menu's own input: every order's `frame`, in the list's order.
pub fn frame(g: &mut Game) {
    for r in ROWS { if let Some(f) = r.frame { f(g); } }
}

/// `issue` of a row that only opens a page: `give` refuses such a row before it comes to this
fn opens_a_page(_g: &mut Game, _men: &[GuardId], _t: &OrderTarget, _o: Option<&GiveOpts>) -> Option<String> { None }

/// `valid` from a word for why not: none is valid
fn plain_valid(w: Option<String>) -> Result<(), Option<String>> {
    match w { Some(w) => Err(Some(w)), None => Ok(()) }
}

/// the order of that id
pub fn by_id(id: &str) -> Option<&'static Row> { ROWS.iter().copied().find(|r| r.id == id) }

/// Is the order of that id such an order? An id on no line of the list is nothing.
pub fn id_is(id: &str, what: Is) -> bool { by_id(id).map_or(false, |r| r.is.contains(&what)) }

/// Is the man under this assignment under such an order?
pub fn is(g: &Game, a: OrdersAssignmentId, what: Is) -> bool { id_is(&g.tab.orders_assignment[a].order, what) }
