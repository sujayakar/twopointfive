//! The types of `input`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, crosses::Crosses, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Blank)]
pub struct Input {
    pub keys: JsSet<String>,
    pub pressed: JsSet<String>,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub buttons: f64,
    pub clicked: f64,
    pub wheel: f64,
}

/// A KEY THE GAME ASKS FOR, by its browser code: the one list of them, and the page's (`GameKey`, src/draw/names.ts, written from `Key::WORDS`). A key's word is its code:
/// a hand, a tape, a record and the wire carry codes as text, and an `Input`'s sets hold whatever text a host hands them, so a rule asks by `as_str`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words, Crosses)]
pub enum Key {
    #[word("KeyW")] W,
    #[word("KeyA")] A,
    #[word("KeyS")] S,
    #[word("KeyD")] D,
    #[word("KeyQ")] Q,
    #[word("KeyE")] E,
    #[word("KeyR")] R,
    #[word("KeyF")] F,
    #[word("KeyC")] C,
    #[word("KeyL")] L,
    #[word("KeyZ")] Z,
    #[word("KeyV")] V,
    #[word("KeyN")] N,
    #[word("KeyB")] B,
    #[word("KeyT")] T,
    #[word("Digit0")] Digit0,
    #[word("Digit1")] Digit1,
    #[word("Digit2")] Digit2,
    #[word("Digit3")] Digit3,
    #[word("Digit4")] Digit4,
    #[word("Digit5")] Digit5,
    #[word("Digit6")] Digit6,
    #[word("Digit7")] Digit7,
    #[word("Digit8")] Digit8,
    #[word("Digit9")] Digit9,
    #[word("Space")] Space,
    #[word("ShiftLeft")] ShiftLeft,
    #[word("ShiftRight")] ShiftRight,
    #[word("Enter")] Enter,
    #[word("NumpadEnter")] NumpadEnter,
    #[word("Tab")] Tab,
    #[word("ArrowUp")] ArrowUp,
    #[word("ArrowDown")] ArrowDown,
    #[word("BracketLeft")] BracketLeft,
    #[word("BracketRight")] BracketRight,
    #[word("F5")] F5,
    // a host says these on its own account: no key on any board
    #[word("Blur")] Blur,
    #[word("Paused")] Paused,
    #[word("Thawed")] Thawed,
}
impl Key {
    /// the digit's key, 0 to 9; there is no other
    pub const fn digit(d: i32) -> Option<Key> {
        const DIGITS: [Key; 10] = [Key::Digit0, Key::Digit1, Key::Digit2, Key::Digit3, Key::Digit4, Key::Digit5, Key::Digit6, Key::Digit7, Key::Digit8, Key::Digit9];
        if d >= 0 && d < 10 { Some(DIGITS[d as usize]) } else { None }
    }
    /// its code, to go in a set
    pub fn text(self) -> String { String::from(self.as_str()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// a code is the text a hand, a tape and the wire carry
    #[test]
    fn a_keys_word_is_its_code() {
        assert_eq!((Key::W.as_str(), Key::Digit1.as_str(), Key::Space.as_str(), Key::Blur.as_str()), ("KeyW", "Digit1", "Space", "Blur"));
    }
    #[test]
    fn the_digits_are_nought_to_nine() {
        for d in 0..10 { assert_eq!(Key::digit(d).map(Key::as_str), Some(alloc::format!("Digit{d}")).as_deref()); }
        assert_eq!((Key::digit(-1), Key::digit(10)), (None, None));
    }
}
