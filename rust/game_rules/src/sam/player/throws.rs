//! HIS THROWS: the arc while a can is in his hand, the throw, and the can leaving his fist.
use super::*;
use crate::game::{Game, SlotKind, ThrowKind};
use crate::ids::*;
use crate::m;

/// His cans this tick. `arc`: the launcher's, shown when no can's is.
pub(super) fn throws(g: &mut Game, t: Tick, a: Aim, arc: Option<crate::throwables::ThrowSolution>) {
    let pl = t.pl;
    let left = |g: &Game, kind| throw_count(g, &crate::seats::here(g), kind) > 0.0;
    let first = g.pl[pl].throw_kind;
    let other = if first == ThrowKind::Flash { ThrowKind::Smoke } else { ThrowKind::Flash };
    if !left(g, first) && left(g, other) { g.pl[pl].throw_kind = other; }
    g.sets[g.seat].hand.throw_preview = match can_in_hand(g, pl) {
        Some(kind) if !g.pl[pl].sprinting && left(g, kind) => Some(can_up(g, t, a, kind)),
        _ => arc,
    };
    let Some(p) = g.pl[pl].pending_throw.as_mut() else { return };
    p.t -= t.dt;
    if p.t <= 0.0 { can_leaves(g, t); }
}

/// A can in his hand: the arc to the cursor, and the throw when the trigger is pressed.
fn can_up(g: &mut Game, t: Tick, a: Aim, kind: ThrowKind) -> crate::throwables::ThrowSolution {
    let origin = throw_origin(g, &crate::seats::here(g));
    let sol = crate::throwables::throwables_solve(g, origin, [a.at[0], m::max(0.05, m::min(a.at[1], 2.6)), a.at[2]], None);
    let in_fist = match g.ch[t.c].anim.kit_hold {
        None => true,
        Some(Some(crate::character::KitItem::Can)) => g.pl[t.pl].kit_can == Some(kind),
        Some(_) => false,
    };
    if t.lmb_hit(g) && g.arms[t.c].fire_cd <= 0.0 && in_fist {
        do_throw(g, &sol, kind);
        g.arms[t.c].fire_cd = 0.6;
    }
    sol
}

/// The can he wound up with is out of his hand.
fn can_leaves(g: &mut Game, t: Tick) {
    use crate::throwables::ItemKind;
    let pl = t.pl;
    let Some(p) = g.pl[pl].pending_throw.take() else { return };
    let origin = throw_origin(g, &crate::seats::here(g));
    let (item, fuse) = match p.kind {
        ThrowKind::Flash => (ItemKind::Flash, Some(crate::combat::flash_fuse(g))),
        ThrowKind::Frag => (ItemKind::Frag, Some(g.k.frag.fuse)),
        ThrowKind::Smoke => (ItemKind::Smoke, None),
    };
    let who = g.ch[t.c].id;
    let it = crate::throwables::throwables_spawn(g, item, origin, p.sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse, who: Some(who), ..Default::default() }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::CanHeard);
    crate::chronicle::chron_thrown(g, p.kind.as_str());
    if g.infinite_ammo { can_back(g, pl, p.kind); }
    let out = crate::game::player_canister_sel(g, pl).is_some_and(|kind| throw_count(g, &crate::seats::here(g), kind) == 0.0);
    if out { g.pl[pl].slot = SlotKind::Pistol; }
}

/// how many of a kind are on the belt
pub fn throw_count(g: &crate::game::Game, seat: &crate::seats::SeatRef, kind: crate::game::ThrowKind) -> f64 {
    let pl = seat.player;
    match kind {
        crate::game::ThrowKind::Frag => g.pl[pl].frags,
        crate::game::ThrowKind::Flash => g.pl[pl].flashbangs,
        _ => g.pl[pl].canisters,
    }
}

/// the can he has in his hand
pub fn can_in_hand(g: &crate::game::Game, pl: PlayerId) -> Option<crate::game::ThrowKind> {
    if g.pl[pl].holstered { None } else { crate::game::player_canister_sel(g, pl) }
}

/// a thrown thing not yet out of the hand goes back on the belt
pub(super) fn can_back(g: &mut crate::game::Game, pl: PlayerId, kind: crate::game::ThrowKind) {
    match kind {
        crate::game::ThrowKind::Frag => g.pl[pl].frags += 1.0,
        crate::game::ThrowKind::Flash => g.pl[pl].flashbangs += 1.0,
        _ => g.pl[pl].canisters += 1.0,
    }
}

/// a wind-up called off with the can still in his fist: back on the belt
pub(in crate::sam) fn throw_off(g: &mut crate::game::Game, pl: PlayerId) {
    if let Some(kind) = g.pl[pl].pending_throw.as_ref().map(|t| t.kind) {
        can_back(g, pl, kind);
        g.pl[pl].pending_throw = None;
    }
}
