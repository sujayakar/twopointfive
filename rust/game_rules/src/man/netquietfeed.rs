pub use ::game_types::netquietfeed::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use alloc::{vec::Vec};

/// Files a report in the book, making the book if this game has none yet.
fn file(g: &mut Game, f: NetFeed) {
    g.m.netquietfeed.filed.get_or_insert_with(Vec::new).push(f);
}

/// `asker` asks for `who` by name: filed for netquiet's tick while `live`.
pub fn net_feed_asked(g: &mut crate::game::Game, asker: Option<GuardId>, who: GuardId, men: &[GuardId]) {
    if !g.m.netquietfeed.netfeed.live { return; }
    file(g, NetFeed { what: NetFeedWhat::Asked, by: asker, who: Some(who), men: men.to_vec() });
}

/// `finder` has a man of the squad down: filed for netquiet's tick while `live`.
pub fn net_feed_body(g: &mut crate::game::Game, finder: Option<GuardId>, men: &[GuardId]) {
    if !g.m.netquietfeed.netfeed.live { return; }
    file(g, NetFeed { what: NetFeedWhat::Body, by: finder, who: None, men: men.to_vec() });
}

/// What was filed since the last take; the book is left empty.
pub(in crate::man) fn net_feed_take(g: &mut crate::game::Game) -> Vec<crate::netquietfeed::NetFeed> {
    let filed = &mut g.m.netquietfeed.filed;
    match filed {
        Some(f) if !f.is_empty() => core::mem::take(f),
        _ => Vec::new(),
    }
}
