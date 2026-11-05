//! src/game/netquietfeed.ts lines 1 to 31
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Files a report in the book, making the book if this game has none yet.
fn file(g: &mut Game, f: NetFeed) {
    g.m.netquietfeed.filed.get_or_insert_with(Vec::new).push(f);
}

/// `asker` asks for `who` by name: filed for netquiet's tick while `live`.   (netquietfeed.ts netFeedAsked)
pub fn net_feed_asked(g: &mut crate::game::Game, asker: Option<GuardId>, who: GuardId, men: &[GuardId]) {
    if !g.m.netquietfeed.netfeed.live { return; }
    file(g, NetFeed { what: NetFeedWhat::Asked, by: asker, who: Some(who), men: men.to_vec() });
}

/// `finder` has a man of the squad down: filed for netquiet's tick while `live`.   (netquietfeed.ts netFeedBody)
pub fn net_feed_body(g: &mut crate::game::Game, finder: Option<GuardId>, men: &[GuardId]) {
    if !g.m.netquietfeed.netfeed.live { return; }
    file(g, NetFeed { what: NetFeedWhat::Body, by: finder, who: None, men: men.to_vec() });
}

/// Marks or clears the last man at his act.   (netquietfeed.ts netFeedHold)
pub fn net_feed_hold(g: &mut crate::game::Game, gd: GuardId, on: bool) {
    if on { g.m.netquietfeed.holding.add(gd); } else { g.m.netquietfeed.holding.delete(gd); }
}

/// Is he the last man at his act; false while the feed is not `live`.   (netquietfeed.ts netFeedHolds)
pub fn net_feed_holds(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.netquietfeed.netfeed.live && g.m.netquietfeed.holding.has(gd)
}

/// What was filed since the last take; the book is left empty.   (netquietfeed.ts netFeedTake)
pub fn net_feed_take(g: &mut crate::game::Game) -> Vec<crate::netquietfeed::NetFeed> {
    let filed = &mut g.m.netquietfeed.filed;
    match filed {
        Some(f) if !f.is_empty() => core::mem::take(f),
        _ => Vec::new(),
    }
}
