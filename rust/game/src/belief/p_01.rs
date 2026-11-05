//! src/game/belief.ts lines 1 to 50
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The tick's first word: what a man covers is said anew each tick by the duty that holds him to it. (belief.ts dutiesAnew)
pub fn duties_anew(g: &mut crate::game::Game) {
    if !g.k.belief.on { return; }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        g.gd[gd].covers = None;
    }
}

/// The room fight (and the rules that hand a man back to it): his duty has his gun on `pt`. (belief.ts cover)
pub fn cover(g: &mut crate::game::Game, gd: GuardId, pt: V3) {
    if g.k.belief.on {
        g.gd[gd].covers = Some(pt);
        return;
    }
    // The same object written again is no new fix (doze.ts:244), and here equal bits stand for the same object.
    if g.gd[gd].last_known != Some(pt) {
        g.gd[gd].fix_stamp = g.gd[gd].fix_stamp.wrapping_add(1);
    }
    g.gd[gd].last_known = Some(pt);
}

/// His duty has nothing for his gun after all, this tick. (belief.ts uncover)
pub fn uncover(g: &mut crate::game::Game, gd: GuardId) {
    g.gd[gd].covers = None;
}

/// His fix is the spot his eyes last had the man. (belief.ts fixAtSight)
pub fn fix_at_sight(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match g.gd[gd].last_known {
        Some(lk) => v3::dist_xz(lk, g.gd[gd].sight_pos) < 0.01,
        None => false,
    }
}

/// His fix came from his eyes: the spot they last had the man, or where he worked out that man had gone. (belief.ts fixIsSight)
pub fn fix_is_sight(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if fix_at_sight(g, gd) { return true; }
    match (g.gd[gd].last_known, g.gd[gd].led_at) {
        (Some(lk), Some(led)) => v3::dist_xz(lk, led) < 0.01,
        _ => false,
    }
}

/// Where his own eyes have the man, for the rules that ask only that of a man on a post. (belief.ts eyesFix)
pub fn eyes_fix(g: &mut crate::game::Game, gd: GuardId, pt: V3) -> Option<V3> {
    let lk = g.gd[gd].last_known?;
    // Rule off, `lk !== pt` asks identity: equal bits stand for the same object (see notes, Traps).
    let his = if g.k.belief.on { fix_is_sight(g, gd) } else { lk != pt };
    if his { Some(lk) } else { None }
}

/// Where his gun goes with no man to shoot at: his eyes this very tick, else his duty, else his belief. (belief.ts gunPoint)
pub fn gun_point(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> {
    let (lk, cv) = (g.gd[gd].last_known, g.gd[gd].covers);
    if cv.is_none() || !g.gd[gd].hold || !g.gd[gd].pinned { return lk; }
    if lk.is_some() && g.time - g.gd[gd].sight_t < 1e-6 && fix_at_sight(g, gd) { lk } else { cv }
}

/// A report that reaches him outside his ears and eyes places his belief. (belief.ts word)
pub fn word(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    let now = g.time;
    let man = &mut g.gd[gd];
    man.last_known = Some(at);
    man.fix_stamp = man.fix_stamp.wrapping_add(1); // a fresh object in the TypeScript: always a new fix
    man.word_t = now;
}

/// What he hears of `at` or before does not write over a word. (belief.ts wordStands)
pub fn word_stands(g: &mut crate::game::Game, gd: GuardId, at: f64) -> bool {
    g.k.belief.on && at <= g.gd[gd].word_t
}

/// The same, of this moment. (belief.ts wordNow)
pub fn word_now(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    word_stands(g, gd, now)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stand(on: bool) -> (Box<Game>, GuardId) {
        let mut g = Box::new(Game::blank());
        g.k.belief.on = on;
        let gd = g.gd.push(crate::game::Guard::blank());
        g.guards.push(gd);
        (g, gd)
    }

    #[test]
    fn rule_on_his_duty_is_not_his_belief() {
        let (mut g, gd) = stand(true);
        cover(&mut g, gd, [1.0, 0.0, 2.0]);
        assert_eq!((g.gd[gd].covers, g.gd[gd].last_known, g.gd[gd].fix_stamp), (Some([1.0, 0.0, 2.0]), None, 0));
        duties_anew(&mut g);
        assert_eq!(g.gd[gd].covers, None);
    }

    #[test]
    fn rule_off_one_field_and_duties_stay() {
        let (mut g, gd) = stand(false);
        let pt = [1.0, 0.0, 2.0];
        cover(&mut g, gd, pt);
        assert_eq!((g.gd[gd].covers, g.gd[gd].last_known, g.gd[gd].fix_stamp), (None, Some(pt), 1));
        cover(&mut g, gd, pt); // the same point again is no new fix
        assert_eq!(g.gd[gd].fix_stamp, 1);
        assert_eq!(eyes_fix(&mut g, gd, pt), None);
        assert_eq!(eyes_fix(&mut g, gd, [9.0, 0.0, 9.0]), Some(pt));
        g.gd[gd].covers = Some(pt);
        duties_anew(&mut g); // only the rule's word clears the field
        assert_eq!(g.gd[gd].covers, Some(pt));
    }

    #[test]
    fn eyes_fix_asks_where_it_came_from() {
        let (mut g, gd) = stand(true);
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
        let (mut g, gd) = stand(true);
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
        let (mut g, gd) = stand(true);
        g.time = 4.0;
        word(&mut g, gd, [1.0, 0.0, 1.0]);
        assert_eq!((g.gd[gd].last_known, g.gd[gd].word_t, g.gd[gd].fix_stamp), (Some([1.0, 0.0, 1.0]), 4.0, 1));
        assert!(word_now(&mut g, gd));
        assert!(word_stands(&mut g, gd, 3.0));
        assert!(!word_stands(&mut g, gd, 4.5));
        g.k.belief.on = false;
        assert!(!word_now(&mut g, gd));
    }
}
