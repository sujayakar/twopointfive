pub use ::game_types::samstun::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};

/// Can a stun show on the player at all: up.
fn live(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool { live_of(g, seat.player) }
/// …of one man
fn live_of(g: &crate::game::Game, p: PlayerId) -> bool {
    !crate::condition::down(g, g.pl[p].ch)
}

/// Fold the player for `secs`; `dir` = the way the blow came from → him.
pub fn stagger_player(g: &mut crate::game::Game, secs: f64, dir: V3) {
    let (p, c) = (g.player, g.pl[g.player].ch);
    if !(secs > 0.0) || crate::condition::down(g, g.pl[p].ch) { return; }
    crate::condition::fold(g, c, secs);
    crate::condition::rattle(g, c);
    crate::character::anim_hit_from(g, c, Some(dir));
    g.pl[p].hit_flash = 1.0;
    g.pl[p].fire_cd = m::max(g.pl[p].fire_cd, secs);
    fold_cancels(g);
    g.stats.samstun.staggered += 1;
}

/// What a fold takes out of his hands: a can wound up goes back on the belt, a quick throw is dropped, every sustained verb stops.
pub fn fold_cancels(g: &mut crate::game::Game) {
    let p = g.player;
    if let Some(k) = g.pl[p].pending_throw.as_ref().map(|t| t.kind) {
        match k {
            crate::game::ThrowKind::Frag => g.pl[p].frags += 1.0,
            crate::game::ThrowKind::Flash => g.pl[p].flashbangs += 1.0,
            _ => g.pl[p].canisters += 1.0,
        }
        g.pl[p].pending_throw = None;
    }
    g.pl[p].throw_held = false;
    if g.pl[p].picking.is_some() || g.pl[p].wanding.is_some() { g.pl[p].f_held = true; }
    let mut i = 0;
    while i < 6 && crate::interact::cancel_sustained(g) { i += 1; } // one verb a call
}

/// Blind the player for `secs` by a flash at `at`.
pub fn dazzle_player(g: &mut crate::game::Game, secs: f64, at: V3) {
    let (p, c) = (g.player, g.pl[g.player].ch);
    if !(secs > 0.0) || crate::condition::down(g, g.pl[p].ch) { return; }
    crate::condition::blind(g, c, secs, Some(at));
    let along = v3::sub(g.ch[c].pos, at);
    crate::character::anim_hit_from(g, c, Some(along));
    g.cond[c].sup.v = m::max(g.cond[c].sup.v, 1.0);
    g.stats.samstun.dazzled += 1;
}

/// Is he folded right now (rooted, gun down, no round)? `now`: the clock asked of.
pub fn sam_staggered(g: &crate::game::Game, seat: &crate::seats::SeatRef, now: Option<f64>) -> bool {
    let now = now.unwrap_or(g.time);
    if !live(g, &crate::seats::here(g)) { return false; }
    let p = seat.player;
    now < g.cond[g.pl[p].ch].fold_until
}

/// How blind he is right now: 1 for the body of a dazzle, easing to 0 over its last `fade` seconds.
pub(super) fn sam_blind(g: &crate::game::Game, seat: &crate::seats::SeatRef, now: Option<f64>) -> f64 { man_blind(g, seat.player, now) }
/// …of one man
pub fn man_blind(g: &crate::game::Game, p: PlayerId, now: Option<f64>) -> f64 {
    let now = now.unwrap_or(g.time);
    if !live_of(g, p) { return 0.0; }
    let left = g.cond[g.pl[p].ch].blind_until - now;
    if left <= 0.0 { 0.0 } else { v3::clamp(left / m::max(0.05, g.k.samstun.fade), 0.0, 1.0) }
}

/// May his feet move? A dazzle does not root him on the page (SAMSTUN.blindRoots).
pub fn sam_rooted(g: &crate::game::Game, seat: &crate::seats::SeatRef, now: Option<f64>) -> bool {
    sam_staggered(g, seat, now)
}

/// Once a frame right after the down test: what the two states show on him and keep true while they run.
pub(super) fn sam_stun_pose(g: &mut crate::game::Game) {
    let (p, c) = (g.player, g.pl[g.player].ch);
    let (live, at) = (live_of(g, p), g.ch[c].pos);
    let (blind, folded) = crate::condition::stun_pose(g, c, live, at);
    g.pl[p].stun_posed = folded || blind;
    if blind { g.cond[c].sup.v = m::max(g.cond[c].sup.v, man_blind(g, p, None)); }
    if folded { g.pl[p].fire_cd = m::max(g.pl[p].fire_cd, g.cond[c].fold_until - g.time); }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A game with the page's knobs, its clock at 11, and a player with a body.
    fn game() -> Game {
        let mut g = Game::blank();
        g.time = 11.0;
        let c = crate::character::man_new(&mut g, crate::character::Character::blank());
        let mut pl = crate::game::Player::blank();
        pl.ch = c;
        g.player = g.pl.push(pl);
        g
    }

    /// A failure means the blind screen drops off at once or never clears: he is wholly blind until the last of it, then it eases to nothing at the end.
    #[test]
    fn a_dazzle_is_whole_then_fades_over_its_last_moments() {
        let mut g = game();
        let p = g.player;
        g.cond[g.pl[p].ch].blind_until = 11.0;
        let fade = g.k.samstun.fade;
        let blind = |g: &Game, now| sam_blind(g, &crate::seats::here(g), Some(now));
        assert_eq!(blind(&g, 1.0), 1.0);
        assert_eq!(blind(&g, 11.0 - fade - 0.01), 1.0);
        assert!((blind(&g, 11.0 - fade / 2.0) - 0.5).abs() < 1e-9);
        assert!(blind(&g, 10.97) > 0.0 && blind(&g, 10.97) < 0.1);
        assert_eq!(blind(&g, 11.0), 0.0);
    }

    #[test]
    fn a_stun_shows_only_on_a_man_who_is_live() {
        let mut g = game();
        let p = g.player;
        g.cond[g.pl[p].ch].fold_until = 12.0;
        g.cond[g.pl[p].ch].blind_until = 12.0;
        assert!(sam_staggered(&g, &crate::seats::here(&g), None));
        assert_eq!(sam_blind(&g, &crate::seats::here(&g), None), 1.0);
        assert!(!sam_staggered(&g, &crate::seats::here(&g), Some(12.0)));   // the clock asked of, not the game's
        g.cond[g.pl[p].ch].down = true;
        assert!(!sam_staggered(&g, &crate::seats::here(&g), None));
        assert_eq!(sam_blind(&g, &crate::seats::here(&g), None), 0.0);
    }

    #[test]
    fn a_dazzle_does_not_root_him() {
        let mut g = game();
        let p = g.player;
        g.cond[g.pl[p].ch].blind_until = 12.0;
        assert_eq!(sam_blind(&g, &crate::seats::here(&g), None), 1.0);
        assert!(!sam_rooted(&g, &crate::seats::here(&g), None));
        g.cond[g.pl[p].ch].fold_until = 12.0;
        assert!(sam_rooted(&g, &crate::seats::here(&g), None));
    }

    #[test]
    fn no_seconds_and_a_downed_man_take_no_stamp() {
        // both return before the stub of the animator is reached
        let mut g = game();
        let p = g.player;
        stagger_player(&mut g, 0.0, [1.0, 0.0, 0.0]);
        dazzle_player(&mut g, f64::NAN, [1.0, 0.0, 0.0]);
        assert_eq!((g.cond[g.pl[p].ch].fold_until, g.cond[g.pl[p].ch].blind_until), (-1.0, -1.0));
        g.cond[g.pl[p].ch].down = true;
        stagger_player(&mut g, 2.0, [1.0, 0.0, 0.0]);
        dazzle_player(&mut g, 2.0, [1.0, 0.0, 0.0]);
        assert_eq!((g.cond[g.pl[p].ch].fold_until, g.cond[g.pl[p].ch].blind_until), (-1.0, -1.0));
        assert_eq!((g.stats.samstun.staggered, g.stats.samstun.dazzled), (0, 0));
    }

    #[test]
    fn the_pose_follows_the_two_states_and_comes_off() {
        let mut g = game();
        let (p, c) = (g.player, g.pl[g.player].ch);
        // folded, not blind: the clutch, the gun kept down
        g.cond[g.pl[p].ch].fold_until = 12.5;
        sam_stun_pose(&mut g);
        assert!(g.pl[p].stun_posed);
        assert_eq!(g.ch[c].anim.clutch, 1.0);
        assert!(g.ch[c].anim.dazzle.is_none());
        assert_eq!(g.pl[p].fire_cd, 1.5);
        // blind as well: no clutch, turned away from the flash, and his nerve goes with the blindness
        g.time = 10.7;
        g.cond[g.pl[p].ch].blind_until = 11.0;
        g.cond[g.pl[p].ch].blind_from = Some([1.0, 2.0, 3.0]);
        sam_stun_pose(&mut g);
        assert_eq!(g.ch[c].anim.clutch, 0.0);
        let dz = g.ch[c].anim.dazzle.expect("a dazzle");
        assert_eq!(dz.from, [1.0, 2.0, 3.0]);
        assert!((dz.k - 0.3 / 0.5).abs() < 1e-9, "the turn eases off over the last half second: {}", dz.k);
        assert!((g.cond[g.pl[p].ch].sup.v - sam_blind(&g, &crate::seats::here(&g), None)).abs() < 1e-12);
        // both run out: all of it comes off
        g.time = 20.0;
        sam_stun_pose(&mut g);
        assert!(!g.pl[p].stun_posed);
        assert_eq!(g.ch[c].anim.clutch, 0.0);
        assert!(g.ch[c].anim.dazzle.is_none());
    }
}
