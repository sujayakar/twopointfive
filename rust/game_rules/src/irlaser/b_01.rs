//! THE BRING-UP'S OWN PART of irlaser: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Sam's laser is on: a scenario's team on the floor, his goggles down, a gun in his hands and up at the aim, hands not on a lock or round a man, not
/// down.   (irlaser.ts samLaserOn)
pub fn sam_laser_on(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    let pl = seat.player;
    if !crate::roomfight::fight_live(g) || !g.pl[pl].nv { return false; }
    if !(crate::game::player_pistol_up(g, pl) || crate::game::player_rifle_up(g, pl)) { return false; }
    let c = g.pl[pl].ch;
    let p = &g.pl[pl];
    let a = &g.ch[c].anim;
    // (kitHold: the gun is still on its way to his hands; `undefined` alone is "in his hands", null is bare hands and holds the laser off too)
    a.upper == crate::character::UpperMode::Aim && !p.down && !p.sprinting && p.holding.is_none() && !a.hide_held_item && a.kit_hold.is_none()
}

/// Sam's own infrared laser, muzzle to whatever it meets, in the tube's picture only (combat.ts handProps asks, right after his rig).
/// NOT a boundary no-op, though it draws: its line and its dot are two boxes of the FRAME'S LIST, which the fog of war, the listing of the box world and
/// every table numbered by reach count. Left empty, a goggled player's frame was two boxes short from the moment his gun came up.   (irlaser.ts samLaserBoxes)
pub fn sam_laser_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    if !sam_laser_on(g, &crate::seats::here(g)) || !ir_viewer(g) { return; }
    let c = g.pl[g.player].ch;
    if let Some(r) = crate::combat::laser_reach(g, c, None, Some(true)) { crate::combat::beam_boxes(g, r.from, r.dir, r.l, out, r.dot, Some(true), Some(true)); }
}

/// The mark "this box is an infrared line the step drew for its own tube": the set beside the boxes that a picture with the tube off leaves out by
/// (`ir_made`). It goes when the box's row is freed (`ir_forget`).   (irlaser.ts:78 irNote)
pub fn ir_note(g: &mut crate::game::Game, b: BoxId) -> BoxId { g.m.irlaser.ir_made.add(b); b }
/// A box's row is free again: the mark of the box that stood in it goes with it.
pub fn ir_forget(g: &mut crate::game::Game, b: BoxId) { if !g.m.irlaser.ir_made.is_empty() { g.m.irlaser.ir_made.delete(b); } }

/// The picture is the tube's: an infrared laser shows in it; a replay viewer's own tube, while one is set, answers instead.   (irlaser.ts irViewer)
pub fn ir_viewer(g: &crate::game::Game) -> bool {
    if let Some(t) = g.m.irlaser.view_tube { return t; }
    let man = crate::seats::host_man(g);
    g.pl[man].nv_amount > 0.5 && !(g.engine.settings.debug_view > 0.0)
}

/// `fn_` run under a viewer's own tube, the old one put back after. Between steps only.   (irlaser.ts irViewAs)
pub fn ir_view_as<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, tube: bool, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let was = g.m.irlaser.view_tube;
    g.m.irlaser.view_tube = Some(tube);
    let r = fn_(g);
    g.m.irlaser.view_tube = was;
    r
}

/// Is this box an infrared line the step drew for its own tube (`ir_note`)?   (irlaser.ts irMade)
pub fn ir_made(g: &mut crate::game::Game, b: BoxId) -> bool {
    g.m.irlaser.ir_made.has(b)
}

/// The band of a man's weapon laser this frame. IR.on is settled on.   (irlaser.ts laserBand)
pub fn laser_band(g: &crate::game::Game, gd: GuardId) -> crate::irlaser::LaserBand {
    if crate::nvg::nvg_worn(g, gd) && crate::nvg::nvg_down(g, gd) { LaserBand::Ir } else { LaserBand::Visible }
}

/// A teammate's laser is on this frame. IR.on and IR.carry are settled on, IR.teamOff settled on, BREAK false.   (irlaser.ts laserOn)
pub fn laser_on(g: &crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].muzzle_down || g.gd[gd].held.is_some() || crate::rails::rails_no_lasers(g) { return false; }
    let team = g.gd[gd].team;
    if !crate::teams::is_hostile(g, team, crate::teams::Team::Player) { return false; }
    if g.gd[gd].state == crate::game::GuardState::Alert || g.gd[gd].state == crate::game::GuardState::Search { return true; }
    let c = g.gd[gd].ch;
    g.ch[c].anim.upper == crate::character::UpperMode::Aim && laser_band(g, gd) == LaserBand::Ir
}

/// An infrared line would end on one of his own: a player, or a man of a team not hostile to his.   (irlaser.ts paintsOwn)
pub fn paints_own(g: &mut crate::game::Game, gd: GuardId, on: CharId) -> bool {
    let team = g.gd[gd].team;
    if crate::seats::man_of(g, Some(on)).is_some() { return !crate::teams::is_hostile(g, team, crate::teams::Team::Player); }
    let o = g.guards.iter().copied().find(|&x| g.gd[x].ch == on);
    match o {
        Some(o) => { let other = g.gd[o].team; !crate::teams::is_hostile(g, team, other) }
        None => false,
    }
}
