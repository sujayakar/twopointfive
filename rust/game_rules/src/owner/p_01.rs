//! WHOSE MAN HE IS: one word a man, a bit a tasker (`Taskers`, types.rs). A tasker that takes him sets its bit and lets go by clearing it; the strongest
//! bit set is who has him, since when and for what reason. A rule asks whether any of some taskers has a hand on him (`has`).
//! The list, its names and its order are the types file's `taskers!`; the word lies on the man's row of the block (sim_men_store).
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::sim_men_store as men;

/// the word changed: if who has him changed, he has him from now, for its first reason or the one given
fn settle(g: &mut Game, gd: GuardId, word: By, t: f64, why: Option<u32>) {
    let c = g.gd[gd].ch;
    men::man_set_owner(g, c, word.0);
    let rank = word.rank();
    if rank == men::man_rank(g, c) {
        if let Some(w) = why { if rank > 0 { men::man_set_reason(g, c, w); } }
        return;
    }
    men::man_set_rank(g, c, rank);
    men::man_set_reason(g, c, why.unwrap_or(0));
    men::man_set_since(g, c, t);
}

/// Tasker `by` takes him; nothing changes if it has him already, but for the reason it gives. `t` the game's clock; `why` a place in its list of reasons.
pub fn claim(g: &mut Game, gd: GuardId, by: By, t: f64, why: Option<f64>) {
    let why = js::to_u32(why.unwrap_or(0.0));
    let c = g.gd[gd].ch;
    let was = By(men::man_owner(g, c));
    let word = was | by;
    let top = by.rank() == word.rank();
    if was == word {
        if top { men::man_set_reason(g, c, why); }
        return;
    }
    settle(g, gd, word, t, if top { Some(why) } else { None });
}

/// Tasker `by` lets go. The one below it, if any, has him again from now.
pub fn release(g: &mut Game, gd: GuardId, by: By, t: f64) {
    let was = By(men::man_owner(g, g.gd[gd].ch));
    if !was.any(by) { return; }
    settle(g, gd, By(was.0 & !by.0), t, None);
}

/// has any of these a hand on him
pub fn has(g: &Game, gd: GuardId, by: By) -> bool { By(men::man_owner(g, g.gd[gd].ch)).any(by) }

/// who has him, for what reason and since when; None: nobody
pub fn who(g: &Game, gd: GuardId) -> Option<(&'static str, &'static str, f64)> {
    let c = g.gd[gd].ch;
    let top = By(men::man_owner(g, c)).rank();
    let t = TASKERS.iter().find(|t| t.by.rank() == top)?;
    Some((t.name, t.why.get(men::man_reason(g, c) as usize).copied().unwrap_or(t.why[0]), men::man_since(g, c)))
}

// ---- by number, for the page's checks (the call table's rows)
/// `claim`, the tasker by its bit
pub fn claim_man(g: &mut Game, gd: GuardId, rule: f64, t: f64, why: Option<f64>) { claim(g, gd, By(js::to_u32(rule)), t, why); }
/// `release`, the tasker by its bit
pub fn release_man(g: &mut Game, gd: GuardId, rule: f64, t: f64) { release(g, gd, By(js::to_u32(rule)), t); }

#[cfg(test)]
mod tests {
    use super::*;

    /// the bits the page's table has for them, and the rank each comes to
    #[test]
    fn the_taskers_bits_are_their_places_in_the_list() {
        for (i, t) in TASKERS.iter().enumerate() {
            assert_eq!((t.by.0, t.by.rank()), (1 << (14 - i), 15 - i as u32), "{}", t.name);
            assert!(!t.why.is_empty() && TASKERS[..i].iter().all(|o| o.name != t.name), "{}", t.name);
        }
        assert_eq!((By(0).rank(), By(3).rank(), By(32767).rank()), (0, 2, 15));
    }

    /// (owner, rank, reason, since) after each step, as the TypeScript's claimMan and releaseMan left them
    #[test]
    fn claims_and_releases_as_the_typescript_leaves_them() {
        let mut game = Game::blank();
        let g = &mut game;
        // a man with no row on the block keeps his numbers on his Character, so this needs no block
        let c = g.ch.push(crate::character::Character::blank());
        g.ch[c].man_kept = Some([0.0; 4]);
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let m = g.gd.push(man);
        let row = |g: &Game| (crate::sim_men_store::man_owner(g, c), crate::sim_men_store::man_rank(g, c), crate::sim_men_store::man_reason(g, c), crate::sim_men_store::man_since(g, c));

        claim(g, m, by::SQUAD, 1.0, None);
        assert_eq!(row(g), (1, 1, 0, 1.0));
        claim(g, m, by::LANE, 2.0, None);
        assert_eq!(row(g), (65, 7, 0, 2.0));
        claim(g, m, by::SQUAD, 3.0, None);
        assert_eq!(row(g), (65, 7, 0, 2.0), "a lower rule it already has: nothing");
        claim(g, m, by::RUNNER, 4.0, Some(1.0));
        assert_eq!(row(g), (8257, 14, 1, 4.0));
        claim(g, m, by::RUNNER, 5.0, None);
        assert_eq!(row(g), (8257, 14, 0, 4.0), "the top rule again: its reason, not its time");
        claim(g, m, by::RUNNER, 6.0, Some(1.0));
        assert_eq!(row(g), (8257, 14, 1, 4.0));
        claim(g, m, by::PRESS, 7.0, Some(0.0));
        assert_eq!(row(g), (8289, 14, 1, 4.0), "a new rule below the top: the top's reason stands");
        release(g, m, by::RUNNER, 8.0);
        assert_eq!(row(g), (97, 7, 0, 8.0), "the rule below has him from now, at its first reason");
        claim(g, m, by::PRESS | by::FRAG, 9.0, Some(0.0));
        assert_eq!(row(g), (16481, 15, 0, 9.0));
        release(g, m, by::FRAG, 10.0);
        assert_eq!(row(g), (97, 7, 0, 10.0));
        claim(g, m, by::STAND, 11.0, Some(0.0));
        assert_eq!(row(g), (1121, 11, 0, 11.0));
        claim(g, m, by::WALK, 12.0, Some(1.0));
        assert_eq!(row(g), (1137, 11, 0, 11.0), "a reason given below the top is not written");
        release(g, m, by::STAND | by::LANE | by::PRESS | by::WALK, 13.0);
        assert_eq!(row(g), (1, 1, 0, 13.0));
        assert!(has(g, m, by::SQUAD));
        assert!(!has(g, m, by::FRAG));
        assert!(has(g, m, By(!0)));
        release(g, m, by::SQUAD, 14.0);
        assert_eq!(row(g), (0, 0, 0, 14.0));
        release(g, m, by::SQUAD, 15.0);
        assert_eq!(row(g), (0, 0, 0, 14.0), "a rule with no hand on him: nothing");
        assert!(!has(g, m, By(!0)));
    }
}
