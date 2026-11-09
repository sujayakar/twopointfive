use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The one gate: the switch on AND glass breaking live.
pub(in crate::breakage) fn shards_live(g: &crate::game::Game) -> bool {
    g.tune.glass_shards && crate::glassbreak::glass_break_live(g)
}

/// `x >>> 0`, as the number it is in the TypeScript.
pub fn u32(x: f64) -> f64 {
    js::to_u32(x) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bits printed by the TypeScript's own two lines under bun.
    #[test]
    fn u32_matches_the_typescript_bits() {
        let cases: [(f64, u64); 6] = [
            (0.0, 0x0000000000000000),
            (-1.0, 0x41efffffffe00000),
            (3.9, 0x4008000000000000),
            (-3.9, 0x41efffffffa00000),
            (4294967301.0, 0x4014000000000000),
            (f64::NAN, 0x0000000000000000),
        ];
        for (x, bits) in cases {
            assert_eq!(u32(x).to_bits(), bits, "u32({x})");
        }
    }

}
