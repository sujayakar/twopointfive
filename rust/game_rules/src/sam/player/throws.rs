//! HIS THROWS: the arc while a can is in his hand, the throw, and the can leaving his fist.
use super::*;
use crate::game::{Game, SlotKind, ThrowKind};
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};

/// His cans this tick. `arc`: the launcher's, shown when no can's is.
pub(super) fn throws(g: &mut Game, t: Tick, a: Aim, arc: Option<crate::throwables::ThrowSolution>) {
    let pl = t.pl;
    let left = |g: &Game, kind| throw_count(g, &crate::seats::here(g), kind) > 0.0;
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

/// The kind of can his slot has up, whether or not one is left.
pub fn can_in_hand(g: &crate::game::Game, pl: PlayerId) -> Option<crate::game::ThrowKind> {
    if g.pl[pl].holstered { None } else { crate::game::player_canister_sel(g, pl) }
}

/// a thrown thing not yet out of the hand goes back on the belt
pub fn can_back(g: &mut crate::game::Game, pl: PlayerId, kind: crate::game::ThrowKind) {
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

/// Throw origin, right hand-ish.
pub(super) fn throw_origin(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> V3 {
    let pl = seat.player;
    let c = g.pl[pl].ch;
    let f = crate::character::character_forward(g, c);
    let pos = g.ch[c].pos;
    [pos[0] + f[0] * 0.32, pos[1] + (if g.pl[pl].crouch { 1.05 } else { 1.5 }), pos[2] + f[2] * 0.32]
}

/// The wind-up of a throw: the can counted now, it leaves the hand at THROW_RELEASE along the arc previewed.
pub(super) fn do_throw(g: &mut crate::game::Game, sol: &crate::throwables::ThrowSolution, kind: crate::game::ThrowKind) {
    let pl = g.player;
    if crate::game::game_throw_count(g, &crate::seats::here(g), kind) <= 0.0 {
        return;
    }
    if g.pl[pl].pending_throw.is_some() {
        return;
    }
    match kind {
        crate::game::ThrowKind::Frag => g.pl[pl].frags -= 1.0,
        crate::game::ThrowKind::Flash => g.pl[pl].flashbangs -= 1.0,
        _ => g.pl[pl].canisters -= 1.0,
    }
    let c = g.pl[pl].ch;
    let dist = v3::dist_xz(g.ch[c].pos, sol.land);
    crate::character::anim_throw_item(g, c, Some(dist), Some(m::hyp2(sol.v0[0], sol.v0[2])));
    g.pl[pl].pending_throw = Some(crate::game::PlayerPendingThrow { t: crate::character::THROW_RELEASE, sol: sol.clone(), kind });
    let at = throw_origin(g, &crate::seats::here(g));
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Throw, Some(at), Some(0.7), None);
}
