#![allow(unused_variables)]
//! Every function and method of `netquietfeed`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/netquietfeed.ts:15 netFeedAsked)
pub fn net_feed_asked(g: &mut crate::game::Game, asker: Option<GuardId>, who: GuardId, men: &[GuardId]) { todo!() }

/// (src/game/netquietfeed.ts:20 netFeedBody)
pub fn net_feed_body(g: &mut crate::game::Game, finder: Option<GuardId>, men: &[GuardId]) { todo!() }

/// (src/game/netquietfeed.ts:26 netFeedHold)
pub fn net_feed_hold(g: &mut crate::game::Game, gd: GuardId, on: bool) { todo!() }

/// (src/game/netquietfeed.ts:28 netFeedHolds)
pub fn net_feed_holds(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquietfeed.ts:30 netFeedTake)
pub fn net_feed_take(g: &mut crate::game::Game) -> Vec<crate::netquietfeed::NetFeed> { todo!() }
