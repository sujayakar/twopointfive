//! src/game/owner.ts lines 1 to 120
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A rule's bit: higher is stronger. `1 << (TOP - i)` as an int32. (owner.ts bit)
pub fn bit(i: f64) -> f64 {
    (1i32 << (js::to_u32(TOP - i) & 31)) as f64
}

/// The rank a word of claims comes to: 1 the weakest rule, 0 nobody. (owner.ts rankOf)
pub fn rank_of(word: f64) -> f64 {
    32.0 - m::clz32(js::to_u32(word)) as f64
}

/// The word changed: if the rule on top changed, it has him from now, for its first reason. (owner.ts settle)
pub fn settle(g: &mut crate::game::Game, gd: GuardId, word: f64, t: f64, why: f64) {
    g.gd[gd].owner = js::to_u32(word);
    let rank = rank_of(word);
    if rank == g.gd[gd].rank as f64 {
        if why >= 0.0 && rank > 0.0 { g.gd[gd].reason = js::to_u32(why); }
        return;
    }
    g.gd[gd].rank = js::to_u32(rank);
    g.gd[gd].reason = if why >= 0.0 { js::to_u32(why) } else { 0 };
    g.gd[gd].since = t;
}

/// Rule `rule` takes him; nothing changes if it has him already, but for the reason it gives. `t` the game's clock; `why` an index into its row's `why`. (owner.ts claimMan)
pub fn claim_man(g: &mut crate::game::Game, gd: GuardId, rule: f64, t: f64, why: Option<f64>) {
    let why = why.unwrap_or(0.0);
    let was = g.gd[gd].owner;
    let word = was | js::to_u32(rule);
    if was == word {
        if rank_of(word as f64) == rank_of(rule) { g.gd[gd].reason = js::to_u32(why); }
        return;
    }
    settle(g, gd, word as f64, t, if rank_of(rule) == rank_of(word as f64) { why } else { -1.0 });
}

/// Rule `rule` lets go. The rule below it, if any, has him again from now. (owner.ts releaseMan)
pub fn release_man(g: &mut crate::game::Game, gd: GuardId, rule: f64, t: f64) {
    let was = g.gd[gd].owner;
    let rule = js::to_u32(rule);
    if was & rule == 0 { return; }
    settle(g, gd, (was & !rule) as f64, t, -1.0);
}

/// Does any of these rules have a hand on him? (owner.ts hasHim)
pub fn has_him(g: &mut crate::game::Game, gd: GuardId, rules: f64) -> bool {
    g.gd[gd].owner & js::to_u32(rules) != 0
}

/// Does nothing: the TypeScript check's registration (tools/qa/owner-check.ts), which nothing in the game reads and no Rust part calls (blitz/enemies.md). (owner.ts ownerWitness)
pub fn owner_witness<G: Clone + core::fmt::Debug>(g: &mut crate::game::Game, name: crate::owner::OwnerRule, says: &mut dyn FnMut(&mut crate::game::Game, GuardId, G) -> bool, at_read: Option<bool>) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// every answer below was printed by the TypeScript's own functions under bun on the snapshot
    #[test]
    fn bit_and_rank_as_the_typescript_answers() {
        for (i, want) in [(0.0, 16384.0), (1.0, 8192.0), (14.0, 1.0), (15.0, -2147483648.0), (16.0, 1073741824.0), (-17.0, -2147483648.0), (0.5, 8192.0), (f64::NAN, 1.0), (46.0, 1.0)] {
            assert_eq!(bit(i), want, "bit({i})");
        }
        for (w, want) in [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 2.0), (16384.0, 15.0), (32767.0, 15.0), (-1.0, 32.0), (-2147483648.0, 32.0), (0.5, 0.0), (f64::NAN, 0.0), (4294967296.0, 0.0), (4294967297.0, 1.0), (f64::INFINITY, 0.0)] {
            assert_eq!(rank_of(w), want, "rank_of({w})");
        }
    }

    #[test]
    fn the_table_of_bits_is_the_ranks_order() {
        let o = own();
        let in_rank_order = [o.frag, o.runner, o.morale, o.startle, o.stand, o.squad_leg, o.squad_word, o.last_man, o.lane, o.press, o.walk, o.breaker, o.buddy, o.needs, o.squad];
        for (i, b) in in_rank_order.iter().enumerate() {
            assert_eq!(*b, bit(i as f64), "row {i}");
        }
    }

    /// (owner, rank, reason, since) after each step, as the TypeScript's claimMan and releaseMan left them
    #[test]
    fn claims_and_releases_as_the_typescript_leaves_them() {
        let mut game = Game::blank();
        let g = &mut game;
        let m = g.gd.push(crate::game::Guard::blank());
        let o = own();
        let row = |g: &Game| (g.gd[m].owner, g.gd[m].rank, g.gd[m].reason, g.gd[m].since);

        claim_man(g, m, o.squad, 1.0, None);
        assert_eq!(row(g), (1, 1, 0, 1.0));
        claim_man(g, m, o.lane, 2.0, None);
        assert_eq!(row(g), (65, 7, 0, 2.0));
        claim_man(g, m, o.squad, 3.0, None);
        assert_eq!(row(g), (65, 7, 0, 2.0), "a lower rule it already has: nothing");
        claim_man(g, m, o.runner, 4.0, Some(1.0));
        assert_eq!(row(g), (8257, 14, 1, 4.0));
        claim_man(g, m, o.runner, 5.0, None);
        assert_eq!(row(g), (8257, 14, 0, 4.0), "the top rule again: its reason, not its time");
        claim_man(g, m, o.runner, 6.0, Some(1.0));
        assert_eq!(row(g), (8257, 14, 1, 4.0));
        claim_man(g, m, o.press, 7.0, Some(0.0));
        assert_eq!(row(g), (8289, 14, 1, 4.0), "a new rule below the top: the top's reason stands");
        release_man(g, m, o.runner, 8.0);
        assert_eq!(row(g), (97, 7, 0, 8.0), "the rule below has him from now, at its first reason");
        claim_man(g, m, o.press + o.frag, 9.0, Some(0.0));
        assert_eq!(row(g), (16481, 15, 0, 9.0));
        release_man(g, m, o.frag, 10.0);
        assert_eq!(row(g), (97, 7, 0, 10.0));
        claim_man(g, m, o.stand, 11.0, Some(0.0));
        assert_eq!(row(g), (1121, 11, 0, 11.0));
        claim_man(g, m, o.walk, 12.0, Some(1.0));
        assert_eq!(row(g), (1137, 11, 0, 11.0), "a reason given below the top is not written");
        release_man(g, m, o.stand + o.lane + o.press + o.walk, 13.0);
        assert_eq!(row(g), (1, 1, 0, 13.0));
        assert!(has_him(g, m, o.squad));
        assert!(!has_him(g, m, o.frag));
        assert!(has_him(g, m, -1.0), "~0");
        release_man(g, m, o.squad, 14.0);
        assert_eq!(row(g), (0, 0, 0, 14.0));
        release_man(g, m, o.squad, 15.0);
        assert_eq!(row(g), (0, 0, 0, 14.0), "a rule with no hand on him: nothing");
        assert!(!has_him(g, m, -1.0));
    }
}
