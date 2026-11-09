//! src/game/enemysquads.ts lines 1536 to 1572
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `r3`: to three places; what is not a finite number is not given   (squadWaysRound)
fn r3(x: Option<f64>) -> Option<f64> {
    match x { Some(v) if v.is_finite() => Some(m::round(v * 1000.0) / 1000.0), _ => None }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own bits, under bun with EXACT on: `Math.round(x * 1000) / 1000`
    #[test]
    fn r3_rounds_to_the_typescripts_bits() {
        for &(x, want) in &[
            (0x0000000000000000u64, 0x0000000000000000u64),
            (0x3ff3c083126e978d, 0x3ff3c28f5c28f5c3),
            (0x3ff0020c49ba5e35, 0x3ff004189374bc6a),
            (0x4004000000000000, 0x4004000000000000),
            (0x40300020c2ee26ba, 0x4030000000000000),
            (0xbff0020c49ba5e35, 0xbff0000000000000),
            (0xbf3a36e2eb1c432d, 0x8000000000000000),
            (0x40934a456d5cfaad, 0x40934a45a1cac083),
            (0x3fd3333333333334, 0x3fd3333333333333),
            (0x3e112e0be826d695, 0x0000000000000000),
        ] {
            assert_eq!(r3(Some(f64::from_bits(x))).unwrap().to_bits(), want);
        }
    }
    #[test]
    fn r3_gives_nothing_for_what_is_not_a_finite_number() {
        assert_eq!(r3(None), None);
        assert_eq!(r3(Some(f64::INFINITY)), None);
        assert_eq!(r3(Some(f64::NAN)), None);
    }
}
