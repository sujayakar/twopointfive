//! THE BRING-UP'S OWN PART of irlaser: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/irlaser.ts:92 samLaserBoxes)
pub fn sam_laser_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {}

/// BOUNDARY NO-OP (irlaser.ts:78): the mark "this box is drawn in IR" is the picture's set (read by irMade as a view is drawn); the box is handed back
pub fn ir_note(g: &mut crate::game::Game, b: BoxId) -> BoxId { b }

/// The picture is the tube's: an infrared laser shows in it; a replay viewer's own tube, while one is set, answers instead.   (irlaser.ts irViewer)
pub fn ir_viewer(g: &mut crate::game::Game) -> bool {
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

/// BOUNDARY: the host's draw of a viewer's picture asks whether this box is an infrared one, and at the swap `ir_note` above fills the set; until then the set is empty and this is false.   (irlaser.ts irMade)
pub fn ir_made(g: &mut crate::game::Game, b: BoxId) -> bool {
    g.m.irlaser.ir_made.has(b)
}

/// The band of a man's weapon laser this frame. IR.on is settled on.   (irlaser.ts laserBand)
pub fn laser_band(g: &mut crate::game::Game, gd: GuardId) -> crate::irlaser::LaserBand {
    if crate::nvg::nvg_worn(g, gd) && crate::nvg::nvg_down(g, gd) { LaserBand::Ir } else { LaserBand::Visible }
}

/// A teammate's laser is on this frame. IR.on and IR.carry are settled on, IR.teamOff settled on, BREAK false.   (irlaser.ts laserOn)
pub fn laser_on(g: &mut crate::game::Game, gd: GuardId) -> bool {
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
