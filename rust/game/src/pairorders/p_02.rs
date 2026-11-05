//! src/game/pairorders.ts lines 613 to 636
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A teammate on a take job takes no man as a target who has not noticed anybody, nor the man his hands are on.   (pairorders.ts pairPasses)
pub fn pair_passes(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if !g.k.pair.on { return false; }
    let j = pair_orders_job_of(g, gd);
    let j = match j { Some(j) => j, None => return false };
    if crate::silent::urgent(g) { return false; }
    if j.foe == h && j.t0 >= 0.0 { return true; }
    if pair_orders_finishing(g, gd) { return true; }
    !pair_orders_noticed(g, h)
}

/// A man in the partner's hands finds no body.   (pairorders.ts pairGripBody)
pub fn pair_grip_body(g: &mut crate::game::Game, h: GuardId) -> bool {
    pair_gripped(g, h)
}

/// The partner's hands (or spotted.ts's, under pairup's 'hands') are on this man: he sees nobody until they are off.   (pairorders.ts pairGripped)
pub fn pair_gripped(g: &mut crate::game::Game, h: GuardId) -> bool {
    (g.k.pair.on && pair_orders_gripped(g, h))
        || (crate::pairup::pair_up(g, crate::pairup::PairUpRule::Hands) && crate::spotted::spotted_gripped(g, h))
}

/// One told to WATCH a man fires no round while that man has not noticed.   (pairorders.ts pairHolds)
pub fn pair_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if !g.k.pair.on { return false; }
    let a = match crate::orders::squad_orders_of(g, gd) { Some(a) => a, None => return false };
    if g.tab.orders_assignment[a].order != "pairWatch" || g.tab.orders_assignment[a].foe != Some(h) || pair_orders_noticed(g, h) || crate::silent::urgent(g) {
        return false;
    }
    g.stats.pairorders.held += 1;
    true
}
