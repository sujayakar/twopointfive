//! src/game/stealthfeed.ts lines 1 to 16
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// One frame's word to Sam's animator, before any state poses him; under a show or a puppet, or down: nothing fed.   (stealthfeed.ts stealthFeed)
pub fn stealth_feed(g: &mut crate::game::Game) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if g.touring || g.puppet.is_some() || g.pl[pl].down {
        g.ch[c].anim.stealth = None;
        return;
    }
    let wanding = g.pl[pl].wanding.is_some();
    let cable = wanding;
    // the same test, in the same order: sam_work only when all the rest are false
    let p = &g.pl[pl];
    let taken = p.holding.is_some()
        || p.takedown.is_some()
        || p.dragging.is_some()
        || p.haul.is_some()
        || p.kick.is_some()
        || p.picking.is_some()
        || p.coding.is_some()
        || p.wall.is_some()
        || p.easing.is_some()
        || p.door_move.is_some()
        || p.breaker_move.is_some()
        || p.charging.is_some()
        || p.sprinting
        || crate::handwork::sam_work(g).is_some();
    let f = g.ch[c].anim.stealth.get_or_insert(crate::stealthpose::StealthFeed { free: false, cable: false });
    f.cable = cable;
    // kit_in_hand only when the tests before it pass, and after the row above is written, as the TypeScript's && has it
    let free = !taken
        && !wanding
        && crate::kitanim::kit_in_hand(g).is_none()
        && g.pl[pl].throw_preview.is_none()
        && !g.pl[pl].throw_held;
    g.ch[c].anim.stealth.as_mut().unwrap().free = free;
}
