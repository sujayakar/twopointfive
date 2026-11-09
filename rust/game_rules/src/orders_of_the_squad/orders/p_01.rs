use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A row's digit off its key code: `Digit1` to `Digit9`, else 0.
pub fn digit_of(code: &str) -> f64 {
    match code.strip_prefix("Digit").map(str::as_bytes) {
        Some(&[d @ b'1'..=b'9']) => (d - b'0') as f64,
        _ => 0.0,
    }
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

}
