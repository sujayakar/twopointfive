//! src/game/shards.ts lines 1 to 154
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A machine's kept switch and knobs: the browser's storage is the host's, which writes `g.tune` and `g.k.shards` itself; with no storage the TypeScript's own `try` does nothing too.   (shards.ts loadShards)
pub fn load_shards(g: &mut crate::game::Game, tune: &mut crate::shards::ShardsTune) {}

/// Nothing of the game's: the TypeScript only writes the browser's storage, the host's.   (shards.ts saveShards)
pub fn save_shards(g: &mut crate::game::Game) {}

/// The one gate: the switch on AND glass breaking live.   (shards.ts shardsLive)
pub fn shards_live(g: &mut crate::game::Game) -> bool {
    g.tune.glass_shards && crate::glassbreak::glass_break_live(g)
}

/// `x >>> 0`, as the number it is in the TypeScript.   (shards.ts u32)
pub fn u32(x: f64) -> f64 {
    js::to_u32(x) as f64
}

/// A litter cell's name in `GlassShards.litter`: cell (ix, iz) of half a metre.   (shards.ts cellName)
pub fn cell_name(ix: f64, iz: f64) -> f64 {
    (ix + 512.0) * 4096.0 + (iz + 512.0)
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

    #[test]
    fn cell_name_matches_the_typescript_bits() {
        let cases: [(f64, f64, u64); 5] = [
            (0.0, 0.0, 0x4140010000000000),
            (-3.0, 7.0, 0x413fd20700000000),
            (-512.0, -512.0, 0x0000000000000000),
            (511.0, 511.0, 0x414ff9ff80000000),
            (-3.5, 7.25, 0x413fca0740000000),
        ];
        for (ix, iz, bits) in cases {
            assert_eq!(cell_name(ix, iz).to_bits(), bits, "cell_name({ix}, {iz})");
        }
    }
}
