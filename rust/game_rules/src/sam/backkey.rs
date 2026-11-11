use crate::game::Game;
use crate::ids::*;
use alloc::string::String;

/// THE BACK KEY backs out of what is up: the squad's ring, his own, a verb that is running, the launcher's arc, a gun's setup. Esc is the pause and only the
/// pause: the page keeps it, and the first tick after a card is handed the one-tick code 'Paused' (as a lost window hands it 'Blur'), which lets go of what a
/// key or a button held, unsent. A running verb goes on after a pause.
pub const BACK_KEY: &str = "Tab";

/// It went down this tick: read, not taken.
pub(super) fn back_hit(g: &crate::game::Game, input: InputId) -> bool {
    crate::input::input_hit(g, input, BACK_KEY)
}

/// …and taken out of the frame: whoever answers to it, nobody after does.
pub fn back_take(g: &mut crate::game::Game, input: InputId) -> bool {
    g.tab.input[input].pressed.delete(String::from(BACK_KEY))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_press_is_taken_once_and_no_other_with_it() {
        let mut g = Game::blank();
        let mut row = crate::input::Input::blank();
        for c in [BACK_KEY, "KeyE"] { row.pressed.add(String::from(c)); }
        let id: InputId = g.tab.input.push(row);
        assert!(back_hit(&g, id) && back_take(&mut g, id) && !back_take(&mut g, id));
        assert!(g.tab.input[id].pressed.holds("KeyE"));
    }
}
