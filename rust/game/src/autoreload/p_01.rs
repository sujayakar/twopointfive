//! src/game/autoreload.ts lines 1 to 37
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Would this player's gun (the one in his hands) start its reload by itself this frame? (autoreload.ts autoReloadDue)
pub fn auto_reload_due(g: &mut crate::game::Game, pl: PlayerId, gun: FirearmId) -> bool {
    if g.touring || g.puppet.is_some() || g.pl[pl].equip_t > 0.0 || g.pl[pl].dragging.is_some() { return false; }
    // The chamber, not the rounds: a full magazine under an empty chamber cannot fire.
    if g.fa[gun].chamber != 0.0 || crate::weapons::firearm_reloading(g, gun) || (g.fa[gun].spare.is_empty() && !(g.fa[gun].mag.unwrap_or(0.0) > 0.0)) || (g.fa[gun].infinite && g.fa[gun].mag.is_some()) { return false; }
    let since = g.time - g.pl[pl].last_fire_t; // a stamp ahead of the clock is no reason to wait
    !(since >= 0.0 && since < g.k.reload.delay)
}

/// The words for a gun just gone dry: the reload is coming, so R is not asked for. (autoreload.ts autoReloads)
pub fn auto_reloads(g: &mut crate::game::Game, gun: FirearmId) -> bool {
    !g.touring && !g.puppet.is_some() && !g.fa[gun].spare.is_empty()
}
