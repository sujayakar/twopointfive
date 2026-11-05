//! src/game/pairup.ts lines 1 to 98
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// What a part's own store holds for its `on`: the browser's storage is the host's, so here it is what the TypeScript answers with no store.   (pairup.ts keptOn)
pub fn kept_on(ls: &str) -> Option<bool> {
    None
}

/// A rule is in force: the switch, and its own row (settled on, on the page).   (pairup.ts pairUp)
pub fn pair_up(g: &mut crate::game::Game, k: crate::pairup::PairUpRule) -> bool {
    g.k.pairup.on
}

/// Keeps the pair's rows in the browser's storage: the host's, so here it does what the TypeScript does with no store, nothing.   (pairup.ts savePairUp)
pub fn save_pair_up(g: &mut crate::game::Game) {}

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
        crate::teamfeel::save_team_feel(g, "PAIR.on");
        crate::sneak::save_sneak(g);
        crate::spotted::save_spotted(g);
    }
}

/// This visit's own word against one part: the address and the environment are the host's, so here it is what the TypeScript answers with neither a page nor a process.   (pairup.ts putBack)
pub fn put_back(name: &str) -> bool {
    false
}

/// The panel's line for the pair.   (pairup.ts pairUpSummary)
pub fn pair_up_summary(g: &mut crate::game::Game) -> String {
    let s = &g.stats.pairup;
    summary_text(g.k.pairup.on, g.k.pair.on, g.k.sneak.on, g.k.spotted.on, s.rejoins, s.caught_off, s.finishes)
}

/// The line itself, from the switches and the counters it shows.
fn summary_text(on: bool, pair: bool, sneak: bool, spotted: bool, rejoins: i32, caught_off: i32, finishes: i32) -> String {
    let mut parts = Vec::new();
    if pair { parts.push("orders"); }
    if sneak { parts.push("sneak"); }
    if spotted { parts.push("spotted"); }
    if !on {
        return if parts.is_empty() { String::from("off") } else { alloc::format!("off · parts on by their own switches: {}", parts.join(", ")) };
    }
    let parts = if parts.is_empty() { String::from("every part put back off") } else { parts.join(" + ") };
    alloc::format!("on · {} · rules oneEye, hunted, rejoin, stay, word, hands · rejoins {} · jobs ended by caught {} · finished by hand {}", parts, rejoins, caught_off, finishes)
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

    /// The expected lines are what pairUpSummary printed under bun for the same switches and counters.
    #[test]
    fn the_summary_reads_as_the_typescript_prints_it() {
        assert_eq!(summary_text(false, false, false, false, 0, 0, 0), "off");
        assert_eq!(summary_text(false, true, false, true, 0, 0, 0), "off · parts on by their own switches: orders, spotted");
        assert_eq!(summary_text(true, true, false, true, 2, 3, 4), "on · orders + spotted · rules oneEye, hunted, rejoin, stay, word, hands · rejoins 2 · jobs ended by caught 3 · finished by hand 4");
        assert_eq!(summary_text(true, false, false, false, 2, 3, 4), "on · every part put back off · rules oneEye, hunted, rejoin, stay, word, hands · rejoins 2 · jobs ended by caught 3 · finished by hand 4");
    }
}
