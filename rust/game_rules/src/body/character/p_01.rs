//! src/game/character.ts lines 1 to 325
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// When key k (0..3) of a code is pressed, on a verb `dur` long.   (character.ts workKeyBeat)
pub fn work_key_beat(k: f64, dur: f64) -> f64 {
    let work = work();
    work.key_lead + (k + 0.5) * (dur - work.key_lead) / 4.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The TypeScript's own bits: workKeyBeat under bun with EXACT on.
    #[test]
    fn work_key_beat_has_the_typescripts_bits() {
        let rows: [(f64, f64, u64); 9] = [
            (0.0, 1.3, 0x3fd599999999999a),
            (1.0, 1.3, 0x3fe399999999999a),
            (2.0, 1.3, 0x3fec666666666666),
            (3.0, 1.3, 0x3ff299999999999a),
            (0.0, 0.7, 0x3fd0cccccccccccd),
            (3.0, 2.45, 0x400159999999999a),
            (1.0, 0.2, 0x3fc999999999999a),
            (2.0, 0.1, 0x3fc199999999999a),
            (3.0, 1.0 / 3.0, 0x3fd4444444444444),
        ];
        for (k, dur, bits) in rows {
            assert_eq!(work_key_beat(k, dur).to_bits(), bits, "k {k} dur {dur}");
        }
    }
}
