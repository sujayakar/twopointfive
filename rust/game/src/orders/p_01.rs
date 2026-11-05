//! src/game/orders.ts lines 1 to 366
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Yaw 0 = +z = south, pi/2 = +x = east.   (orders.ts COMPASS)
static COMPASS: [&str; 8] = ["south", "south-east", "east", "north-east", "north", "north-west", "west", "south-west"];

/// A row's digit off its key code: `Digit1` to `Digit9`, else 0.   (orders.ts digitOf)
pub fn digit_of(code: &str) -> f64 {
    match code.strip_prefix("Digit").map(str::as_bytes) {
        Some(&[d @ b'1'..=b'9']) => (d - b'0') as f64,
        _ => 0.0,
    }
}

/// A key code as the HUD names it: one leading `Key` or `Digit` off.   (orders.ts keyName)
pub fn key_name(code: &str) -> String {
    String::from(code.strip_prefix("Key").or_else(|| code.strip_prefix("Digit")).unwrap_or(code))
}

/// The bearing's word. A yaw that is not finite has none: the array read gives `undefined`, and every reader puts it in a template string.   (orders.ts compass)
pub fn compass(yaw: f64) -> String {
    let at = js::rem(js::rem(m::round(yaw / (m::PI / 4.0)), 8.0) + 8.0, 8.0);
    if at.is_nan() {
        return String::from("undefined");
    }
    String::from(COMPASS[at as usize])
}

/// A store for one asker of `hover` (the page's, a check's).   (orders.ts hoverStore)
pub fn hover_store() -> crate::orders::HoverKept {
    crate::orders::HoverKept { hov: None, cam: Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every expected value below was printed by the TypeScript's own expressions under bun.
    #[test]
    fn digits_as_the_regexp_reads_them() {
        for (code, want) in [("Digit1", 1.0), ("Digit5", 5.0), ("Digit9", 9.0), ("Digit0", 0.0), ("Digit10", 0.0), ("Digit", 0.0), ("KeyX", 0.0), ("Digit5\n", 0.0), ("xDigit1", 0.0), ("", 0.0), ("Space", 0.0)] {
            assert_eq!(digit_of(code), want, "{code:?}");
        }
    }

    #[test]
    fn key_name_takes_one_prefix_at_the_start() {
        for (code, want) in [("Digit1", "1"), ("Digit0", "0"), ("Digit10", "10"), ("Digit", ""), ("KeyX", "X"), ("Digit5\n", "5\n"), ("xDigit1", "xDigit1"), ("", ""), ("Space", "Space"), ("KeyKeyX", "KeyX"), ("DigitKey1", "Key1"), ("KeyDigit1", "Digit1"), ("Keys", "s"), ("xKey", "xKey")] {
            assert_eq!(key_name(code), want, "{code:?}");
        }
    }

    #[test]
    fn compass_as_the_typescript_rounds_it() {
        let p = m::PI;
        for (yaw, want) in [
            (0.0, "south"), (-0.0, "south"), (0.1, "south"), (-0.1, "south"),
            (p / 8.0, "south-east"), (-p / 8.0, "south"), (p / 4.0, "south-east"), (p / 2.0, "east"),
            (p, "north"), (-p, "north"), (3.0 * p / 4.0, "north-east"), (-3.0 * p / 4.0, "north-west"),
            (3.0 * p / 8.0, "east"), (5.0 * p / 8.0, "north-east"), (-p / 2.0, "west"),
            (2.0 * p, "south"), (-2.0 * p, "south"), (7.0, "south-east"), (-7.0, "south-west"),
            (100.0, "south-west"), (-100.0, "south-east"), (1e9, "south-east"), (-1e9, "south-west"), (1e300, "south"),
            (0.39269908169872414, "south-east"), (0.3926990816987241, "south"),
            (-0.39269908169872414, "south"), (-0.3926990816987242, "south-west"),
            (f64::NAN, "undefined"), (f64::INFINITY, "undefined"), (f64::NEG_INFINITY, "undefined"),
        ] {
            assert_eq!(compass(yaw), want, "{yaw:?}");
        }
    }

    #[test]
    fn a_new_store_holds_nothing() {
        let s = hover_store();
        assert!(s.hov.is_none() && s.cam.is_empty());
    }
}
