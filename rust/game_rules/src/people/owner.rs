//! WHOSE MAN HE IS: one word a man, a bit a tasker (`Taskers`, types.rs). A tasker that takes him sets its bit and lets go by clearing it; the strongest
//! bit set is who has him, since when and for what reason. A rule asks whether any of some taskers has a hand on him (`has`).
//! The list, its names and its order are the types file's `taskers!`; the word lies on his body (`Character.hold`).

pub use ::game_types::owner::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};

/// the word changed: if who has him changed, he has him from now, for its first reason or the one given
fn settle(g: &mut Game, gd: GuardId, word: By, t: f64, why: Option<u32>) {
    let h = &mut g.ch[g.gd[gd].ch].hold;
    h.by = word;
    let rank = word.rank();
    if rank == h.rank {
        if let Some(w) = why { if rank > 0 { h.reason = w; } }
        return;
    }
    (h.rank, h.reason, h.since) = (rank, why.unwrap_or(0), t);
}

/// Tasker `by` takes him; nothing changes if it has him already, but for the reason it gives. `t` the game's clock; `why` a place in its list of reasons.
pub fn claim(g: &mut Game, gd: GuardId, by: By, t: f64, why: Option<f64>) {
    let why = js::to_u32(why.unwrap_or(0.0));
    let c = g.gd[gd].ch;
    let was = g.ch[c].hold.by;
    let word = was | by;
    let top = by.rank() == word.rank();
    if was == word {
        if top { g.ch[c].hold.reason = why; }
        return;
    }
    settle(g, gd, word, t, if top { Some(why) } else { None });
}

/// Tasker `by` lets go. The one below it, if any, has him again from now.
pub fn release(g: &mut Game, gd: GuardId, by: By, t: f64) {
    let was = g.ch[g.gd[gd].ch].hold.by;
    if !was.any(by) { return; }
    settle(g, gd, By(was.0 & !by.0), t, None);
}

/// has any of these a hand on him
pub fn has(g: &Game, gd: GuardId, by: By) -> bool { g.ch[g.gd[gd].ch].hold.by.any(by) }

/// MAY `by` TAKE HIM? Who keeps him from it, in words, or None. His body first: dead, in somebody's arm, blind, reeling, a hostage in his own. Then whoever above `by` in the list has
/// him. (`By::NOBODY` asks for the weakest of all: is he anybody's)
pub fn kept(g: &Game, gd: GuardId, by: By) -> Option<&'static str> {
    use crate::control::man;
    if man::dead(g, gd) { return Some("dead"); }
    if man::held(g, gd) { return Some("in somebody's arm"); }
    if man::blind(g, gd) { return Some("blind"); }
    if man::staggered(g, gd) { return Some("reeling"); }
    if man::in_a_standoff(g, gd) { return Some("a hostage in his arms"); }
    TASKERS.iter().find(|t| t.by.rank() > by.rank() && has(g, gd, t.by)).map(|t| t.name)
}

/// a tasker's name
pub fn name_of(by: By) -> Option<&'static str> { TASKERS.iter().find(|t| t.by == by).map(|t| t.name) }

/// who has him, for what reason and since when; None: nobody
pub(super) fn who(g: &Game, gd: GuardId) -> Option<(&'static str, &'static str, f64)> {
    let h = g.ch[g.gd[gd].ch].hold;
    let top = h.by.rank();
    let t = TASKERS.iter().find(|t| t.by.rank() == top)?;
    Some((t.name, t.why.get(h.reason as usize).copied().unwrap_or(t.why[0]), h.since))
}

// ---- by number, for the page's checks (the call table's rows)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_taskers_bits_are_their_places_in_the_list() {
        let n = TASKERS.len() as u32;
        for (i, t) in TASKERS.iter().enumerate() {
            assert_eq!((t.by.0, t.by.rank()), (1 << (n - 1 - i as u32), n - i as u32), "{}", t.name);
            assert!(!t.why.is_empty() && TASKERS[..i].iter().all(|o| o.name != t.name), "{}", t.name);
        }
        assert_eq!((By::NOBODY.rank(), By(3).rank()), (0, 2));
    }

    fn stand() -> (alloc::boxed::Box<Game>, GuardId) {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let c = crate::character::man_new(&mut g, crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let m = g.gd.push(man);
        (g, m)
    }

    /// who has him, for which reason, since when, after each step
    #[test]
    fn claims_and_releases() {
        let (mut g, m) = stand();
        let g = &mut *g;
        let c = g.gd[m].ch;
        let row = |g: &Game| { let h = g.ch[c].hold; (h.by.0, h.rank, h.reason, h.since) };
        let is = |word: By, reason: u32, since: f64| (word.0, word.rank(), reason, since);
        let (at_rest, lane) = (by::SQUAD, by::SQUAD | by::LANE);
        claim(g, m, by::SQUAD, 1.0, None);
        assert_eq!(row(g), is(at_rest, 0, 1.0));
        claim(g, m, by::LANE, 2.0, None);
        assert_eq!(row(g), is(lane, 0, 2.0));
        claim(g, m, by::SQUAD, 3.0, None);
        assert_eq!(row(g), is(lane, 0, 2.0), "a lower rule it already has: nothing");
        claim(g, m, by::RUNNER, 4.0, Some(1.0));
        assert_eq!(row(g), is(lane | by::RUNNER, 1, 4.0));
        claim(g, m, by::RUNNER, 5.0, None);
        assert_eq!(row(g), is(lane | by::RUNNER, 0, 4.0), "the top rule again: its reason, not its time");
        claim(g, m, by::RUNNER, 6.0, Some(1.0));
        claim(g, m, by::OUT, 7.0, Some(0.0));
        assert_eq!(row(g), is(lane | by::RUNNER | by::OUT, 1, 4.0), "a new rule below the top: the top's reason stands");
        release(g, m, by::RUNNER, 8.0);
        assert_eq!(row(g), is(lane | by::OUT, 0, 8.0), "the rule below has him from now, at its first reason");
        claim(g, m, by::NEEDS, 12.0, Some(1.0));
        assert_eq!(row(g), is(lane | by::OUT | by::NEEDS, 0, 8.0), "a reason given below the top is not written");
        release(g, m, by::LANE | by::OUT | by::NEEDS, 13.0);
        assert_eq!(row(g), is(at_rest, 0, 13.0));
        assert!(has(g, m, by::SQUAD) && !has(g, m, by::FRAG) && has(g, m, By(!0)));
        release(g, m, by::SQUAD, 14.0);
        assert_eq!(row(g), is(By::NOBODY, 0, 14.0));
        release(g, m, by::SQUAD, 15.0);
        assert_eq!(row(g), is(By::NOBODY, 0, 14.0), "a rule with no hand on him: nothing");
    }

    /// every pair of the list: the one above keeps him from the one below, never the other way round
    #[test]
    fn a_tasker_is_kept_off_by_those_above_it_and_no_others() {
        let (mut g, m) = stand();
        let g = &mut *g;
        for t in TASKERS {
            assert_eq!(kept(g, m, t.by), None, "nobody has him and {} may not take him", t.name);
            claim(g, m, t.by, 1.0, None);
            assert_eq!(kept(g, m, By::NOBODY), Some(t.name));
            for o in TASKERS { assert_eq!(kept(g, m, o.by).is_some(), t.by.rank() > o.by.rank(), "{} has him and {} asks", t.name, o.name); }
            release(g, m, t.by, 2.0);
        }
        g.time = 5.0;
        g.cond[g.gd[m].ch].blind_until = 6.0;
        assert!(TASKERS.iter().all(|t| kept(g, m, t.by) == Some("blind")), "nobody takes a man who cannot see");
    }
}
