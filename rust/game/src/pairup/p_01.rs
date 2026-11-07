//! src/game/pairup.ts lines 1 to 98
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A rule is in force: the switch, and its own row (settled on, on the page).   (pairup.ts pairUp)
pub fn pair_up(g: &mut crate::game::Game, k: crate::pairup::PairUpRule) -> bool {
    g.k.pairup.on
}

/// The switch reaches the three parts: on, all three on unless this visit put one back; `off` (the row turned off), all three off and out of their stores.   (pairup.ts pairUpApply)
pub fn pair_up_apply(g: &mut crate::game::Game, off: Option<bool>) {
    let off = off.unwrap_or(false);
    if g.k.pairup.on {
        if !put_back("pairorders") { g.k.pair.on = true; }
        if !put_back("sneak") { g.k.sneak.on = true; }
        if !put_back("spotted") { g.k.spotted.on = true; }
    } else if off {
        g.k.spotted.on = false;
        g.k.sneak.on = false;
        g.k.pair.on = false;
        crate::sneak::save_sneak(g);
        crate::spotted::save_spotted(g);
    }
}

/// This visit's own word against one part: the address and the environment are the host's, so here it is what the TypeScript answers with neither a page nor a process.   (pairup.ts putBack)
pub fn put_back(name: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The expected values in these two are what pairUp and pairUpApply gave under bun for the same switches.
    #[test]
    fn a_rule_is_in_force_only_under_the_switch() {
        let g = &mut Game::blank();
        g.k.pairup.on = false;
        assert!(!pair_up(g, PairUpRule::OneEye));
        g.k.pairup.on = true;
        assert!(pair_up(g, PairUpRule::Hands));
    }

    #[test]
    fn the_switch_on_turns_the_three_parts_on_and_off_with_no_word_touches_nothing() {
        let g = &mut Game::blank();
        g.k.pairup.on = false;
        (g.k.pair.on, g.k.sneak.on, g.k.spotted.on) = (true, false, true);
        pair_up_apply(g, None);
        assert_eq!((g.k.pair.on, g.k.sneak.on, g.k.spotted.on), (true, false, true));
        g.k.pairup.on = true;
        (g.k.pair.on, g.k.sneak.on, g.k.spotted.on) = (false, false, false);
        pair_up_apply(g, None);
        assert_eq!((g.k.pair.on, g.k.sneak.on, g.k.spotted.on), (true, true, true));
    }

}
