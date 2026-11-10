//! What his hands or feet are in the middle of: one thing at a time, and how it is put down.
use crate::game::{Deed, Game};
use crate::ids::PlayerId;

/// His hands go to a new thing, and whatever they were in the middle of is put down.
pub fn begin(g: &mut Game, deed: Deed) {
    put_down(g, g.player);
    g.pl[g.player].deed = Some(deed);
}

/// What he is in the middle of, left where it is: a man held is let go of, the wand comes out, his hands come off the rest.
pub fn put_down(g: &mut Game, pl: PlayerId) {
    match g.pl[pl].deed {
        Some(Deed::Hold(_)) => crate::player::free_held(g, crate::player::FreeHeldWhy::Cancel),
        Some(Deed::Wand(_)) => crate::wand::wand_drop(g, pl),
        Some(Deed::Pick(_)) => { let c = g.pl[pl].ch; g.ch[c].anim.lockpick = false; }
        _ => {}
    }
    g.pl[pl].deed = None;
}

/// His hands are wanted for something: off the wall, the body, the reload, the throw and whatever else they were at.
pub fn hands_free(g: &mut Game) {
    let pl = g.player;
    crate::player::release_wall(g);
    crate::player::release_body(g, None);
    put_down(g, pl);
    crate::player::cancel_reloads(g, pl);
    crate::player::throw_off(g, pl);
    g.sets[g.seat].hand.throw_held = false;
    g.sets[g.seat].hand.throw_preview = None;
    g.pl[pl].sprinting = false;
}

/// The hurt man's hands come off what he does on his knees at a door: the lock, the look under it.
pub fn shaken_off(g: &mut Game, pl: PlayerId) {
    if !matches!(g.pl[pl].deed, Some(Deed::Pick(_) | Deed::Wand(_))) { return; }
    put_down(g, pl);
    let his = crate::seats::seat_of_man(g, pl);
    g.sets[his].hand.f_held = true;
}
