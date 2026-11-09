use crate::game::Game;
use crate::ids::*;
use alloc::{string::String};

/// The back key went down this tick: read, not taken. Tab backs out as Esc does (settled: `BACKKEY.tab`).
pub(in crate::hands_of_the_player) fn back_hit(g: &crate::game::Game, input: InputId) -> bool {
    crate::input::input_hit(g, input, "Escape") || crate::input::input_hit(g, input, "Tab")
}

/// The back key, taken out of the frame: whoever answers to it, nobody after does. Both codes go, whichever was there.
pub fn back_take(g: &mut crate::game::Game, input: InputId) -> bool {
    let esc = g.tab.input[input].pressed.delete(String::from("Escape"));
    let tab = g.tab.input[input].pressed.delete(String::from("Tab"));
    esc || tab
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_empties_both_codes_whichever_was_there() {
        let mut g = Game::blank();
        let mut row = crate::input::Input::blank();
        for c in ["Escape", "Tab", "KeyE"] { row.pressed.add(String::from(c)); }
        let id: InputId = g.tab.input.push(row);
        assert!(back_take(&mut g, id));
        assert!(!g.tab.input[id].pressed.has(String::from("Escape")) && !g.tab.input[id].pressed.has(String::from("Tab")));
        assert!(g.tab.input[id].pressed.has(String::from("KeyE")));
        assert!(!back_take(&mut g, id));
    }

    #[test]
    fn take_answers_for_tab_alone() {
        let mut g = Game::blank();
        let mut row = crate::input::Input::blank();
        row.pressed.add(String::from("Tab"));
        let id: InputId = g.tab.input.push(row);
        assert!(back_take(&mut g, id));
    }
}
