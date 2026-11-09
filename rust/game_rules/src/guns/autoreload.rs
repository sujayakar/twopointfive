use crate::game::Game;
use crate::ids::*;

/// Would this player's gun (the one in his hands) start its reload by itself this frame? (autoreload.ts autoReloadDue)
pub fn auto_reload_due(g: &crate::game::Game, pl: PlayerId, gun: FirearmId) -> bool {
    if g.puppet.is_some() || g.pl[pl].equip_t > 0.0 || g.pl[pl].dragging.is_some() { return false; }
    // The chamber, not the rounds: a full magazine under an empty chamber cannot fire.
    if g.fa[gun].chamber != 0.0 || crate::weapons::firearm_reloading(g, gun) || (g.fa[gun].spare.is_empty() && !(g.fa[gun].mag.unwrap_or(0.0) > 0.0)) || (g.fa[gun].infinite && g.fa[gun].mag.is_some()) { return false; }
    let since = g.time - g.pl[pl].last_fire_t; // a stamp ahead of the clock is no reason to wait
    !(since >= 0.0 && since < g.k.reload.delay)
}

/// The words for a gun just gone dry: the reload is coming, so R is not asked for. (autoreload.ts autoReloads)
pub fn auto_reloads(g: &crate::game::Game, gun: FirearmId) -> bool {
    !g.puppet.is_some() && !g.fa[gun].spare.is_empty()
}
