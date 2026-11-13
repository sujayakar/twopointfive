pub use ::game_types::input::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;

/// A new Input with every field initialised and no listeners: a seat's, a host's.
pub fn input_detached(g: &mut Game) -> InputId {
    g.tab.input.push(Input::blank())
}

/// Is this key held?
pub fn input_down(g: &Game, id: InputId, key: Key) -> bool {
    g.tab.input[id].keys.holds(key.as_str())
}

/// Did this key go down this frame?
pub fn input_hit(g: &Game, id: InputId, key: Key) -> bool {
    g.tab.input[id].pressed.holds(key.as_str())
}

/// Left button held.
pub fn input_lmb(g: &Game, id: InputId) -> bool {
    js::has_bits(g.tab.input[id].buttons, 1)
}

/// Right button held.
pub fn input_rmb(g: &Game, id: InputId) -> bool {
    js::has_bits(g.tab.input[id].buttons, 4)
}

/// Left button went down this frame.
pub fn input_lmb_hit(g: &Game, id: InputId) -> bool {
    js::has_bits(g.tab.input[id].clicked, 1)
}

/// Right button went down this frame.
pub fn input_rmb_hit(g: &Game, id: InputId) -> bool {
    js::has_bits(g.tab.input[id].clicked, 4)
}

/// The drawn frame is over: every one-frame edge and travel goes.
pub fn input_end_frame(g: &mut Game, id: InputId) {
    end_frame(&mut g.tab.input[id])
}

fn end_frame(i: &mut Input) {
    i.pressed.clear();
    i.clicked = 0.0;
    i.wheel = 0.0;
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use super::*;

    fn s(x: &str) -> String {
        String::from(x)
    }

    /// A press lasts a frame; what is held is held.
    #[test]
    fn a_frames_presses_and_travel_go_with_it_and_what_is_held_stays() {
        let mut i = Input::blank();
        i.keys.add(s("KeyW"));
        i.pressed.add(s("KeyW"));
        i.buttons = 1.0;
        i.clicked = 1.0;
        i.wheel = 3.0;
        end_frame(&mut i);
        assert!(i.pressed.is_empty() && i.clicked == 0.0 && i.wheel == 0.0);
        assert!(i.keys.has(s("KeyW")) && i.buttons == 1.0);
    }

    #[test]
    fn each_input_is_its_own_row_and_the_methods_read_that_row() {
        let mut g = Game::blank();
        let a = input_detached(&mut g);
        let b = input_detached(&mut g);
        assert_ne!(a, b);
        g.tab.input[a].keys.add(s("KeyW"));
        g.tab.input[a].pressed.add(s("KeyE"));
        g.tab.input[a].buttons = 5.0;   // left and right
        g.tab.input[a].clicked = 4.0;   // right
        assert!(input_down(&mut g, a, crate::input::Key::W) && input_hit(&mut g, a, crate::input::Key::E));
        assert!(input_lmb(&mut g, a) && input_rmb(&mut g, a));
        assert!(input_rmb_hit(&mut g, a) && !input_lmb_hit(&mut g, a));
        assert!(!input_down(&mut g, b, crate::input::Key::W) && !input_hit(&mut g, b, crate::input::Key::E) && !input_lmb(&mut g, b));
    }
}
