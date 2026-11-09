use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// What rust/sim says of its blocks: length in words, slots, the layout's word, each part's start, length and pool flag.
pub fn layout() -> crate::sim_state::Layout {
    let parts = &::sim::state::PARTS;
    let (mut off, mut len, mut pool) = (Vec::new(), Vec::new(), Vec::new());
    for p in parts.iter() {
        off.push(p.off as f64);
        len.push(p.len as f64);
        pool.push(p.pool);
    }
    crate::sim_state::Layout {
        words: (::sim::state::LEN >> 2) as f64,
        games: ::sim::state::GAMES as f64,
        word: ::sim::state::LAYOUT as f64,
        off,
        len,
        pool,
    }
}

/// Where part `p` of a block starts, in bytes from the block's own start.
pub fn sim_part(p: f64) -> f64 {
    ::sim::state::PARTS[p as usize].off as f64
}

/// How long part `p` of a block is, in bytes.
pub fn sim_part_len(p: f64) -> f64 {
    ::sim::state::PARTS[p as usize].len as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The layout says what rust/sim's own exports say, which is what the TypeScript read.
    #[test]
    fn layout_is_the_modules_exports() {
        let l = layout();
        let n = ::sim::state_parts();
        assert_eq!(n, crate::sim_state::sim_parts().len());
        assert_eq!(l.off.len(), n);
        assert_eq!(l.words, (::sim::state_len() >> 2) as f64);
        assert_eq!(l.games, ::sim::state_games() as f64);
        assert_eq!(l.word, ::sim::state_layout() as f64);
        for i in 0..n {
            assert_eq!(l.off[i], ::sim::state_part(i, 0) as f64);
            assert_eq!(l.len[i], ::sim::state_part(i, 1) as f64);
            assert_eq!(l.pool[i], ::sim::state_part(i, 2) != 0);
            assert_eq!(sim_part(i as f64), l.off[i]);
            assert_eq!(sim_part_len(i as f64), l.len[i]);
        }
    }

}
