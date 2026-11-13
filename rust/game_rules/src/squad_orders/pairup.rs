pub use ::game_types::pairup::types;
pub use types::*;

/// A rule is in force: the switch, and its own row (settled on, on the page).
pub fn pair_up(g: &crate::game::Game) -> bool {
    g.k.pairup.on
}

/// The switch reaches the three parts: on, all three on unless this visit put one back; `off` (the row turned off), all three off and out of their stores.
pub fn pair_up_apply(g: &mut crate::game::Game, off: Option<bool>) {
    let off = off.unwrap_or(false);
    if g.k.pairup.on {
        g.k.pair.on = true;
        g.k.sneak.on = true;
        g.k.spotted.on = true;
    } else if off {
        g.k.spotted.on = false;
        g.k.sneak.on = false;
        g.k.pair.on = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use super::*;

    #[test]
    fn a_rule_is_in_force_only_under_the_switch() {
        let g = &mut Game::blank();
        g.k.pairup.on = false;
        assert!(!pair_up(g));
        g.k.pairup.on = true;
        assert!(pair_up(g));
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
