//! src/game/input.ts lines 1 to 111
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A new Input with every field initialised and no listeners: the harness's, the replayer's, a seat's, a native host's.   (input.ts Input.detached)
pub fn input_detached(g: &mut Game) -> InputId {
    input_new(g, None)
}

/// The field initialisers. A real `el`'s sink and listeners are the host's: the row holds neither.   (input.ts Input.constructor)
pub fn input_new(g: &mut Game, el: Option<crate::Opaque>) -> InputId {
    g.tab.input.push(Input::blank())
}

/// A modifier the event says is up is let go, whatever it was held as.   (input.ts Input.modsUp)
pub fn input_mods_up(g: &mut Game, id: InputId, e: crate::input::InputModsUpE) {
    mods_up(&mut g.tab.input[id], e)
}

/// Let go of one physical key the board holds.   (input.ts Input.modUp)
pub fn input_mod_up(g: &mut Game, id: InputId, phys: &str) {
    mod_up(&mut g.tab.input[id], phys)
}

/// Focus is the page's: an Input here has no `el`, and the TypeScript's detached path does nothing.   (input.ts Input.focusGame)
pub fn input_focus_game(g: &mut Game, id: InputId) {}

/// Is this key held?   (input.ts Input.down)
pub fn input_down(g: &mut Game, id: InputId, code: &str) -> bool {
    g.tab.input[id].keys.has(String::from(code))
}

/// Did this key go down this frame?   (input.ts Input.hit)
pub fn input_hit(g: &mut Game, id: InputId, code: &str) -> bool {
    g.tab.input[id].pressed.has(String::from(code))
}

/// Left button held.   (input.ts Input.lmb)
pub fn input_lmb(g: &mut Game, id: InputId) -> bool {
    bit(g.tab.input[id].buttons, 1)
}

/// Right button held.   (input.ts Input.rmb)
pub fn input_rmb(g: &mut Game, id: InputId) -> bool {
    bit(g.tab.input[id].buttons, 4)
}

/// Left button went down this frame.   (input.ts Input.lmbHit)
pub fn input_lmb_hit(g: &mut Game, id: InputId) -> bool {
    bit(g.tab.input[id].clicked, 1)
}

/// Right button went down this frame.   (input.ts Input.rmbHit)
pub fn input_rmb_hit(g: &mut Game, id: InputId) -> bool {
    bit(g.tab.input[id].clicked, 4)
}

/// Middle button held.   (input.ts Input.mmb)
pub fn input_mmb(g: &mut Game, id: InputId) -> bool {
    bit(g.tab.input[id].buttons, 2)
}

/// Middle button went down this frame.   (input.ts Input.mmbHit)
pub fn input_mmb_hit(g: &mut Game, id: InputId) -> bool {
    bit(g.tab.input[id].clicked, 2)
}

/// The drawn frame is over: every one-frame edge and travel goes.   (input.ts Input.endFrame)
pub fn input_end_frame(g: &mut Game, id: InputId) {
    end_frame(&mut g.tab.input[id])
}

/// Between two game ticks of one drawn frame: the first tick's edges go; what is held stays held, and the camera's travel stays.   (input.ts Input.endTick)
pub fn input_end_tick(g: &mut Game, id: InputId) {
    end_tick(&mut g.tab.input[id])
}

/// `(word & mask) !== 0` on a JavaScript number
fn bit(word: f64, mask: i32) -> bool {
    (js::to_i32(word) & mask) != 0
}

fn mods_up(i: &mut Input, e: InputModsUpE) {
    if i.held_as.is_empty() {
        return;
    }
    if !e.shift_key {
        mod_up(i, "ShiftLeft");
        mod_up(i, "ShiftRight");
    }
    if !e.ctrl_key {
        mod_up(i, "ControlLeft");
        mod_up(i, "ControlRight");
    }
    if !e.alt_key {
        mod_up(i, "AltLeft");
        mod_up(i, "AltRight");
    }
    if !e.meta_key {
        mod_up(i, "MetaLeft");
        mod_up(i, "MetaRight");
    }
}

fn mod_up(i: &mut Input, phys: &str) {
    let held = match i.held_as.get(String::from(phys)) {
        Some(a) => a.clone(),
        None => return,
    };
    i.held_as.delete(String::from(phys));
    if !held.is_empty() {
        i.keys.delete(held);
    }
    if i.cam_held == phys {
        i.cam_held = String::new();
    }
}

fn end_frame(i: &mut Input) {
    i.pressed.clear();
    i.clicked = 0.0;
    i.wheel = 0.0;
    i.wheel_ctrl = 0.0;
    i.orbit_dx = 0.0;
}

fn end_tick(i: &mut Input) {
    i.pressed.clear();
    i.clicked = 0.0;
    i.wheel = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(x: &str) -> String {
        String::from(x)
    }

    fn up() -> InputModsUpE {
        InputModsUpE::zeroed()
    }

    #[test]
    fn a_button_word_reads_as_int32() {
        assert!(bit(5.0, 1) && bit(5.0, 4) && !bit(5.0, 2));
        assert!(!bit(0.0, 1) && !bit(f64::NAN, 1));
        assert!(bit(4294967297.0, 1)); // 2^32 + 1: ToInt32 wraps it to 1
        assert!(bit(-1.0, 4));
        assert!(bit(1.5, 1)); // truncates
    }

    #[test]
    fn mods_up_lets_go_of_what_it_was_held_as() {
        let mut i = Input::blank();
        i.keys.add(s("Sprint"));
        i.keys.add(s("KeyW"));
        i.held_as.set(s("ShiftLeft"), s("Sprint"));
        i.held_as.set(s("KeyW"), s("KeyW"));
        mods_up(&mut i, up());
        assert!(!i.keys.has(s("Sprint")));
        assert!(!i.held_as.has(s("ShiftLeft")));
        assert!(i.keys.has(s("KeyW")), "not a modifier");
        assert!(i.held_as.has(s("KeyW")));
    }

    #[test]
    fn mods_up_keeps_what_the_event_says_is_down() {
        let mut i = Input::blank();
        i.keys.add(s("Sprint"));
        i.held_as.set(s("ShiftRight"), s("Sprint"));
        let mut e = up();
        e.shift_key = true;
        mods_up(&mut i, e);
        assert!(i.keys.has(s("Sprint")));
        assert!(i.held_as.has(s("ShiftRight")));
    }

    #[test]
    fn nothing_held_leaves_the_cameras_key_alone() {
        let mut i = Input::blank();
        i.cam_held = s("ShiftLeft");
        mods_up(&mut i, up());
        assert_eq!(i.cam_held, "ShiftLeft");
    }

    #[test]
    fn a_key_whose_row_moved_away_lets_go_of_nothing() {
        let mut i = Input::blank();
        i.keys.add(s("ShiftLeft")); // the key his row moved TO may be down
        i.held_as.set(s("ShiftLeft"), s(""));
        i.cam_held = s("ShiftLeft");
        mod_up(&mut i, "ShiftLeft");
        assert!(i.keys.has(s("ShiftLeft")));
        assert!(!i.held_as.has(s("ShiftLeft")));
        assert_eq!(i.cam_held, "");
    }

    #[test]
    fn a_key_not_held_changes_nothing() {
        let mut i = Input::blank();
        i.cam_held = s("AltLeft");
        mod_up(&mut i, "AltLeft");
        assert_eq!(i.cam_held, "AltLeft");
    }

    #[test]
    fn a_tick_keeps_the_frames_camera_travel_and_a_frame_does_not() {
        let mut i = Input::blank();
        let dirty = |i: &mut Input| {
            i.keys.add(s("KeyW"));
            i.pressed.add(s("KeyW"));
            i.buttons = 1.0;
            i.clicked = 1.0;
            i.wheel = 3.0;
            i.wheel_ctrl = 4.0;
            i.orbit_dx = 5.0;
        };
        dirty(&mut i);
        end_tick(&mut i);
        assert!(i.pressed.is_empty() && i.clicked == 0.0 && i.wheel == 0.0);
        assert!(i.wheel_ctrl == 4.0 && i.orbit_dx == 5.0);
        assert!(i.keys.has(s("KeyW")) && i.buttons == 1.0);
        dirty(&mut i);
        end_frame(&mut i);
        assert!(i.pressed.is_empty() && i.clicked == 0.0 && i.wheel == 0.0);
        assert!(i.wheel_ctrl == 0.0 && i.orbit_dx == 0.0);
        assert!(i.keys.has(s("KeyW")) && i.buttons == 1.0);
    }

    #[test]
    fn each_input_is_its_own_row_and_the_methods_read_that_row() {
        let mut g = Game::blank();
        let a = input_detached(&mut g);
        let b = input_new(&mut g, None);
        assert_ne!(a, b);
        g.tab.input[a].keys.add(s("KeyW"));
        g.tab.input[a].pressed.add(s("KeyE"));
        g.tab.input[a].buttons = 5.0;
        g.tab.input[a].clicked = 2.0;
        assert!(input_down(&mut g, a, "KeyW") && input_hit(&mut g, a, "KeyE"));
        assert!(input_lmb(&mut g, a) && input_rmb(&mut g, a) && !input_mmb(&mut g, a));
        assert!(input_mmb_hit(&mut g, a) && !input_lmb_hit(&mut g, a) && !input_rmb_hit(&mut g, a));
        assert!(!input_down(&mut g, b, "KeyW") && !input_hit(&mut g, b, "KeyE") && !input_lmb(&mut g, b));
        input_end_tick(&mut g, a);
        assert!(!input_hit(&mut g, a, "KeyE") && input_down(&mut g, a, "KeyW"));
    }
}
