//! src/game/samstun.ts lines 1 to 78
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Can a stun show on the player at all: up, no puppet's strings, no tour.   (samstun.ts live)
pub fn live(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    !g.pl[p].down && g.puppet.is_none() && !g.touring
}

/// Fold the player for `secs`; `dir` = the way the blow came from → him.   (samstun.ts staggerPlayer)
pub fn stagger_player(g: &mut crate::game::Game, secs: f64, dir: V3) {
    let p = g.player;
    if !(secs > 0.0) || g.pl[p].down { return; }
    g.pl[p].stagger_until = m::max(g.pl[p].stagger_until, g.time + secs);
    let c = g.pl[p].ch;
    crate::character::anim_hit_from(g, c, Some(dir));
    g.pl[p].hit_flash = 1.0;
    g.pl[p].fire_cd = m::max(g.pl[p].fire_cd, secs);
    g.pl[p].sup.v = m::max(g.pl[p].sup.v, 1.0);
    g.pl[p].sup.kick = m::max(g.pl[p].sup.kick, 1.0);
    fold_cancels(g);
    g.stats.samstun.staggered += 1;
}

/// What a fold takes out of his hands: a can wound up goes back on the belt, a quick throw is dropped, every sustained verb stops.   (samstun.ts foldCancels)
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

/// Blind the player for `secs` by a flash at `at`.   (samstun.ts dazzlePlayer)
pub fn dazzle_player(g: &mut crate::game::Game, secs: f64, at: V3) {
    let p = g.player;
    if !(secs > 0.0) || g.pl[p].down { return; }
    g.pl[p].dazzled_until = m::max(g.pl[p].dazzled_until, g.time + secs);
    g.pl[p].dazzled_at = Some(v3::copy(at));
    let c = g.pl[p].ch;
    let along = v3::sub(g.ch[c].pos, at);
    crate::character::anim_hit_from(g, c, Some(along));
    g.pl[p].sup.v = m::max(g.pl[p].sup.v, 1.0);
    g.stats.samstun.dazzled += 1;
}

/// Is he folded right now (rooted, gun down, no round)? `now`: the clock asked of.   (samstun.ts samStaggered)
pub fn sam_staggered(g: &mut crate::game::Game, now: Option<f64>) -> bool {
    let now = now.unwrap_or(g.time);
    if !live(g) { return false; }
    let p = g.player;
    now < g.pl[p].stagger_until
}

/// How blind he is right now: 1 for the body of a dazzle, easing to 0 over its last `fade` seconds.   (samstun.ts samBlind)
pub fn sam_blind(g: &mut crate::game::Game, now: Option<f64>) -> f64 {
    let now = now.unwrap_or(g.time);
    if !live(g) { return 0.0; }
    let p = g.player;
    let left = g.pl[p].dazzled_until - now;
    if left <= 0.0 { 0.0 } else { v3::clamp(left / m::max(0.05, g.k.samstun.fade), 0.0, 1.0) }
}

/// May his feet move? A dazzle does not root him on the page (SAMSTUN.blindRoots).   (samstun.ts samRooted)
pub fn sam_rooted(g: &mut crate::game::Game, now: Option<f64>) -> bool {
    sam_staggered(g, now)
}

/// Once a frame right after the down test: what the two states show on him and keep true while they run.   (samstun.ts samStunPose)
pub fn sam_stun_pose(g: &mut crate::game::Game) {
    let p = g.player;
    let c = g.pl[p].ch;
    let blind = sam_blind(g, None);
    let stag = sam_staggered(g, None);
    g.pl[p].stun_posed = stag || blind > 0.0;
    g.ch[c].anim.clutch = if stag && blind <= 0.0 { 1.0 } else { 0.0 };
    if blind > 0.0 {
        // every field is written here each frame, so a new row and the kept one read the same
        let from = match g.pl[p].dazzled_at { Some(at) => at, None => g.ch[c].pos };
        let k = m::min(1.0, (g.pl[p].dazzled_until - g.time) / 0.5);
        g.ch[c].anim.dazzle = Some(crate::character::AnimDazzle { from, k });
        g.pl[p].sup.v = m::max(g.pl[p].sup.v, blind);
    } else {
        g.ch[c].anim.dazzle = None;
    }
    if stag {
        g.pl[p].fire_cd = m::max(g.pl[p].fire_cd, g.pl[p].stagger_until - g.time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A game with the page's knobs, its clock at 11, and a player with a body.
    fn game() -> Game {
        let mut g = Game::blank();
        g.time = 11.0;
        let c = g.ch.push(crate::character::Character::blank());
        let mut pl = crate::game::Player::blank();
        pl.ch = c;
        g.player = g.pl.push(pl);
        g
    }

    #[test]
    fn the_fade_keeps_the_typescripts_bits() {
        let mut g = game();
        let p = g.player;
        g.pl[p].dazzled_until = 11.0;
        // bun, EXACT on: clamp((11 - 10.7) / Math.max(0.05, 0.6), 0, 1)
        assert_eq!(sam_blind(&mut g, Some(10.7)).to_bits(), 0x3fe000000000000b);
        // (11 - 10.97) / Math.max(0.05, 0.6)
        assert_eq!(sam_blind(&mut g, Some(10.97)).to_bits(), 0x3fa9999999999900);
        assert_eq!(sam_blind(&mut g, Some(11.0)), 0.0);
        assert_eq!(sam_blind(&mut g, Some(1.0)), 1.0);
    }

    #[test]
    fn a_stun_shows_only_on_a_man_who_is_live() {
        let mut g = game();
        let p = g.player;
        g.pl[p].stagger_until = 12.0;
        g.pl[p].dazzled_until = 12.0;
        assert!(sam_staggered(&mut g, None));
        assert_eq!(sam_blind(&mut g, None), 1.0);
        assert!(!sam_staggered(&mut g, Some(12.0)));   // the clock asked of, not the game's
        g.pl[p].down = true;
        assert!(!sam_staggered(&mut g, None));
        assert_eq!(sam_blind(&mut g, None), 0.0);
        g.pl[p].down = false;
        g.touring = true;
        assert!(!sam_staggered(&mut g, None));
        assert_eq!(sam_blind(&mut g, None), 0.0);
    }

    #[test]
    fn a_dazzle_does_not_root_him() {
        let mut g = game();
        let p = g.player;
        g.pl[p].dazzled_until = 12.0;
        assert_eq!(sam_blind(&mut g, None), 1.0);
        assert!(!sam_rooted(&mut g, None));
        g.pl[p].stagger_until = 12.0;
        assert!(sam_rooted(&mut g, None));
    }

    #[test]
    fn no_seconds_and_a_downed_man_take_no_stamp() {
        // both return before the stub of the animator is reached
        let mut g = game();
        let p = g.player;
        stagger_player(&mut g, 0.0, [1.0, 0.0, 0.0]);
        dazzle_player(&mut g, f64::NAN, [1.0, 0.0, 0.0]);
        assert_eq!((g.pl[p].stagger_until, g.pl[p].dazzled_until), (-1.0, -1.0));
        g.pl[p].down = true;
        stagger_player(&mut g, 2.0, [1.0, 0.0, 0.0]);
        dazzle_player(&mut g, 2.0, [1.0, 0.0, 0.0]);
        assert_eq!((g.pl[p].stagger_until, g.pl[p].dazzled_until), (-1.0, -1.0));
        assert_eq!((g.stats.samstun.staggered, g.stats.samstun.dazzled), (0, 0));
    }

    #[test]
    fn the_pose_follows_the_two_states_and_comes_off() {
        let mut g = game();
        let (p, c) = (g.player, g.pl[g.player].ch);
        // folded, not blind: the clutch, the gun kept down
        g.pl[p].stagger_until = 12.5;
        sam_stun_pose(&mut g);
        assert!(g.pl[p].stun_posed);
        assert_eq!(g.ch[c].anim.clutch, 1.0);
        assert!(g.ch[c].anim.dazzle.is_none());
        assert_eq!(g.pl[p].fire_cd, 1.5);
        // blind as well: no clutch, turned away from the flash; bun, EXACT on: (11 - 10.7) / 0.5, and the fade above
        g.time = 10.7;
        g.pl[p].dazzled_until = 11.0;
        g.pl[p].dazzled_at = Some([1.0, 2.0, 3.0]);
        sam_stun_pose(&mut g);
        assert_eq!(g.ch[c].anim.clutch, 0.0);
        let dz = g.ch[c].anim.dazzle.expect("a dazzle");
        assert_eq!(dz.from, [1.0, 2.0, 3.0]);
        assert_eq!(dz.k.to_bits(), 0x3fe3333333333340);
        assert_eq!(g.pl[p].sup.v.to_bits(), 0x3fe000000000000b);
        // both run out: all of it comes off
        g.time = 20.0;
        sam_stun_pose(&mut g);
        assert!(!g.pl[p].stun_posed);
        assert_eq!(g.ch[c].anim.clutch, 0.0);
        assert!(g.ch[c].anim.dazzle.is_none());
    }
}
