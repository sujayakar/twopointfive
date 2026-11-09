#![allow(unused_imports, unused_variables)]
pub use ::game_types::irlaser::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use alloc::vec::Vec;

/// Sam's laser is on: a scenario's team on the floor, his goggles down, a gun in his hands and up at the aim, hands not on a lock or round a man, not
/// down.   (irlaser.ts samLaserOn)
fn sam_laser_on(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
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
pub fn sam_laser_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    if !sam_laser_on(g, &crate::seats::here(g)) || !ir_viewer(g) { return; }
    let c = g.pl[g.player].ch;
    if let Some(r) = crate::combat::laser_reach(g, c, None, Some(true)) { crate::combat::beam_boxes(g, r.from, r.dir, r.l, out, r.dot, Some(true), Some(true)); }
}

/// The infrared lines the players really have on, for a viewer's picture through a tube the run's own picture did not have: the very rule a step draws them by, asked as
/// if the picture were the tube's.   (combat.ts irBeams)
pub fn ir_beams(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let was = g.m.irlaser.view_tube.replace(true);
    for s in g.seats.clone() { crate::seats::with_seat(g, s, &mut |g| sam_laser_boxes(g, out)); }
    g.m.irlaser.view_tube = was;
}

/// The mark "this box is an infrared line the step drew for its own tube": the set beside the boxes that a picture with the tube off leaves out by
/// (`ir_made`). It goes when the box's row is freed (`ir_forget`).   (irlaser.ts:78 irNote)
pub fn ir_note(g: &mut crate::game::Game, b: AnyBoxId) -> AnyBoxId { g.m.irlaser.ir_made.add(b); b }
/// A box's row is free again: the mark of the box that stood in it goes with it.
pub fn ir_forget(g: &mut crate::game::Game, b: AnyBoxId) { if !g.m.irlaser.ir_made.is_empty() { g.m.irlaser.ir_made.delete(b); } }

/// SOME PLAYER'S PICTURE IS A TUBE'S, so the infrared lines are made this step; a replay viewer's own tube, while one is set, answers instead.
/// It asks EVERY seat's man, never the one this machine shows: the lines are boxes of the frame's list, so whether they are made is the game's and
/// every machine of it must answer alike. A machine whose own picture is bare-eyed leaves them out of what it draws (`ir_made` marks them).
/// (irlaser.ts irViewer, which asked the shown man)
pub fn ir_viewer(g: &crate::game::Game) -> bool {
    if let Some(t) = g.m.irlaser.view_tube { return t; }
    g.seats.iter().any(|&s| g.pl[crate::seats::seat(g, s).player].nv_amount > 0.5)
}
