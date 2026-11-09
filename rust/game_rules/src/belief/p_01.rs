use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The tick's first word: what a man covers is said anew each tick by the duty that holds him to it.
pub fn duties_anew(g: &mut crate::game::Game) {
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        g.gd[gd].covers = None;
    }
}

/// The room fight (and the rules that hand a man back to it): his duty has his gun on `pt`.
pub fn cover(g: &mut crate::game::Game, gd: GuardId, pt: V3) {
    g.gd[gd].covers = Some(pt);
}

/// His duty has nothing for his gun after all, this tick.
pub fn uncover(g: &mut crate::game::Game, gd: GuardId) {
    g.gd[gd].covers = None;
}

/// His fix is the spot his eyes last had the man.
pub fn fix_at_sight(g: &crate::game::Game, gd: GuardId) -> bool {
    match g.gd[gd].last_known {
        Some(lk) => v3::dist_xz(lk, g.gd[gd].sight_pos) < 0.01,
        None => false,
    }
}

/// His fix came from his eyes: the spot they last had the man, or where he worked out that man had gone.
pub fn fix_is_sight(g: &crate::game::Game, gd: GuardId) -> bool {
    if fix_at_sight(g, gd) { return true; }
    match (g.gd[gd].last_known, g.gd[gd].led_at) {
        (Some(lk), Some(led)) => v3::dist_xz(lk, led) < 0.01,
        _ => false,
    }
}

/// Where his own eyes have the man, for the rules that ask only that of a man on a post.
pub fn eyes_fix(g: &crate::game::Game, gd: GuardId, pt: V3) -> Option<V3> {
    let lk = g.gd[gd].last_known?;
    if fix_is_sight(g, gd) { Some(lk) } else { None }
}

/// The same, asked of a defender by his row of the room fight.
pub fn eyes_fix_def(g: &crate::game::Game, gd: GuardId, d: RoomfightDefId) -> Option<V3> {
    let pt = g.tab.roomfight_def[d].pt;
    eyes_fix(g, gd, pt)
}

/// Where his gun goes with no man to shoot at: his eyes this very tick, else his duty, else his belief.
pub fn gun_point(g: &crate::game::Game, gd: GuardId) -> Option<V3> {
    let (lk, cv) = (g.gd[gd].last_known, g.gd[gd].covers);
    if cv.is_none() || !g.gd[gd].hold || !g.gd[gd].pinned { return lk; }
    if lk.is_some() && g.time - g.gd[gd].sight_t < 1e-6 && fix_at_sight(g, gd) { lk } else { cv }
}

/// A report that reaches him outside his ears and eyes places his belief.
pub fn word(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    let now = g.time;
    crate::mind::fix(g, gd, Some(at), "belief"); // a fresh object in the TypeScript: always a new fix
    g.gd[gd].word_t = now;
}

/// What he hears of `at` or before does not write over a word.
pub fn word_stands(g: &crate::game::Game, gd: GuardId, at: f64) -> bool {
    at <= g.gd[gd].word_t
}

/// The same, of this moment.
pub fn word_now(g: &crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    word_stands(g, gd, now)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stand() -> (Box<Game>, GuardId) {
        let mut g = Box::new(Game::blank());
        let gd = g.gd.push(crate::game::Guard::blank());
        g.guards.push(gd);
        (g, gd)
    }

    #[test]
    fn rule_on_his_duty_is_not_his_belief() {
        let (mut g, gd) = stand();
        cover(&mut g, gd, [1.0, 0.0, 2.0]);
        assert_eq!((g.gd[gd].covers, g.gd[gd].last_known, g.gd[gd].fix_stamp), (Some([1.0, 0.0, 2.0]), None, 0));
        duties_anew(&mut g);
        assert_eq!(g.gd[gd].covers, None);
    }

    #[test]
    fn eyes_fix_asks_where_it_came_from() {
        let (mut g, gd) = stand();
        let pt = [5.0, 0.0, 5.0];
        assert_eq!(eyes_fix(&mut g, gd, pt), None);
        g.gd[gd].sight_pos = [1.0, 7.0, 1.0];
        g.gd[gd].last_known = Some([1.005, 0.0, 1.0]); // height does not count
        assert_eq!(eyes_fix(&mut g, gd, pt), Some([1.005, 0.0, 1.0]));
        g.gd[gd].last_known = Some([3.0, 0.0, 3.0]); // a noise or a word
        assert_eq!(eyes_fix(&mut g, gd, pt), None);
        g.gd[gd].led_at = Some([3.0, 0.0, 3.0]); // where he worked out the man had gone
        assert_eq!(eyes_fix(&mut g, gd, pt), Some([3.0, 0.0, 3.0]));
    }

    #[test]
    fn whose_gun() {
        let (mut g, gd) = stand();
        let (lk, cv) = ([1.0, 0.0, 1.0], [2.0, 0.0, 2.0]);
        g.time = 10.0;
        g.gd[gd].last_known = Some(lk);
        g.gd[gd].sight_pos = lk;
        g.gd[gd].sight_t = 10.0;
        g.gd[gd].covers = Some(cv);
        assert_eq!(gun_point(&mut g, gd), Some(lk)); // off his post: no duty
        g.gd[gd].hold = true;
        assert_eq!(gun_point(&mut g, gd), Some(lk)); // both are asked
        g.gd[gd].pinned = true;
        assert_eq!(gun_point(&mut g, gd), Some(lk)); // his eyes this tick beat his duty
        g.gd[gd].sight_t = 10.0 - 1.0 / 60.0;
        assert_eq!(gun_point(&mut g, gd), Some(cv)); // last tick's eyes do not
        g.gd[gd].sight_t = 10.0;
        g.gd[gd].last_known = Some([3.0, 0.0, 3.0]);
        assert_eq!(gun_point(&mut g, gd), Some(cv)); // a noise this tick does not
        g.gd[gd].covers = None;
        assert_eq!(gun_point(&mut g, gd), Some([3.0, 0.0, 3.0]));
    }

    #[test]
    fn a_word_stands_against_what_is_heard_of_then_or_before() {
        let (mut g, gd) = stand();
        g.time = 4.0;
        word(&mut g, gd, [1.0, 0.0, 1.0]);
        assert_eq!((g.gd[gd].last_known, g.gd[gd].word_t, g.gd[gd].fix_stamp), (Some([1.0, 0.0, 1.0]), 4.0, 1));
        assert!(word_now(&mut g, gd));
        assert!(word_stands(&mut g, gd, 3.0));
        assert!(!word_stands(&mut g, gd, 4.5));
    }
}
