//! src/game/backkey.ts lines 1 to 22
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The back key went down this tick: read, not taken. Tab backs out as Esc does (settled: `BACKKEY.tab`). (backkey.ts backHit)
pub fn back_hit(g: &mut crate::game::Game, input: &crate::backkey::Keys) -> bool {
    hit(input, "Escape") || hit(input, "Tab")
}

/// The back key, taken out of the frame: whoever answers to it, nobody after does. Both codes go, whichever was there. (backkey.ts backTake)
pub fn back_take(g: &mut crate::game::Game, input: &mut crate::backkey::Keys) -> bool {
    let esc = input.pressed.delete(String::from("Escape"));
    let tab = input.pressed.delete(String::from("Tab"));
    esc || tab
}

/// The name a hint row shows. (backkey.ts backName)
pub fn back_name(g: &mut crate::game::Game) -> String {
    String::from("Tab")
}

/// `Keys.hit`: the interface's one holder is `Input`, whose `hit` is `pressed.has` (input.ts:100).
fn hit(input: &crate::backkey::Keys, code: &str) -> bool {
    input.pressed.has(String::from(code))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(codes: &[&str]) -> Keys {
        let mut k = Keys::zeroed();
        for c in codes { k.pressed.add(String::from(*c)); }
        k
    }

    #[test]
    fn take_empties_both_codes_whichever_was_there() {
        let mut g = Game::blank();
        let mut both = keys(&["Escape", "Tab", "KeyE"]);
        assert!(back_take(&mut g, &mut both));
        assert!(!both.pressed.has(String::from("Escape")) && !both.pressed.has(String::from("Tab")));
        assert!(both.pressed.has(String::from("KeyE")));
        assert!(!back_take(&mut g, &mut both));
    }

    #[test]
    fn hit_reads_either_code_and_takes_nothing() {
        let mut g = Game::blank();
        assert!(back_hit(&mut g, &keys(&["Tab"])));
        assert!(back_hit(&mut g, &keys(&["Escape"])));
        assert!(!back_hit(&mut g, &keys(&["KeyE"])));
        let k = keys(&["Tab"]);
        back_hit(&mut g, &k);
        assert!(k.pressed.has(String::from("Tab")));
    }
}
