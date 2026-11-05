//! src/sim/state.ts lines 1 to 284
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// What rust/sim says of its blocks: length in words, slots, the layout's word, each part's start, length and pool flag.   (state.ts layout)
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

/// Where part `p` of a block starts, in bytes from the block's own start.   (state.ts simPart)
pub fn sim_part(p: f64) -> f64 {
    ::sim::state::PARTS[p as usize].off as f64
}

/// How long part `p` of a block is, in bytes.   (state.ts simPartLen)
pub fn sim_part_len(p: f64) -> f64 {
    ::sim::state::PARTS[p as usize].len as f64
}

/// Register a system that keeps state in the block.   (state.ts simTenant)
pub fn sim_tenant(t: &crate::sim_state::SimTenant) {
    seam_tenants(t);
}

/// The blocked step: TENANTS is `g.m.sim_state.tenants` and the stub has no `g`. Once it has: `add_tenant(&mut g.m.sim_state.tenants, t)`.
fn seam_tenants(t: &crate::sim_state::SimTenant) {
    todo!("needs: `g` in sim_tenant's signature, for g.m.sim_state.tenants (state.ts:102)")
}

/// The list's own rule: a bit or a name registered twice is a fault of the program.   (state.ts simTenant)
fn add_tenant(tenants: &mut Vec<crate::sim_state::SimTenant>, t: &crate::sim_state::SimTenant) {
    if tenants.iter().any(|o| o.bit == t.bit || o.name == t.name) {
        panic!("a tenant of the module's state is registered twice: {} (bit {})", t.name, t.bit);
    }
    tenants.push(t.clone());
}

/// How many words of part `i` are state, given the part's first word (a pool's first word is the bytes of it in use).   (state.ts liveOf)
pub fn live_of(l: &crate::sim_state::Layout, i: f64, first: f64) -> f64 {
    let i = i as usize;
    let all = js::to_u32(l.len[i]) >> 2;
    if !l.pool[i] {
        return all as f64;
    }
    let n = js::to_u32(first + 3.0) >> 2;
    (if n < 2 { 2 } else if n > all { all } else { n }) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lay() -> crate::sim_state::Layout {
        crate::sim_state::Layout {
            words: 0.0,
            games: 0.0,
            word: 0.0,
            off: vec![0.0, 16.0, 32.0],
            len: vec![16.0, 4096.0, 7.0],
            pool: vec![false, true, true],
        }
    }

    /// The bits of the TypeScript's own liveOf (state.ts:273, lines copied as they stand, run under bun) on this same layout.
    #[test]
    fn live_of_has_the_typescripts_bits() {
        let l = lay();
        let cases: [(f64, f64, u64); 16] = [
            (0.0, 123.0, 0x4010000000000000),
            (1.0, 0.0, 0x4000000000000000),
            (1.0, 1.0, 0x4000000000000000),
            (1.0, 4.0, 0x4000000000000000),
            (1.0, 5.0, 0x4000000000000000),
            (1.0, 8.0, 0x4000000000000000),
            (1.0, 9.0, 0x4008000000000000),
            (1.0, 4095.0, 0x4090000000000000),
            (1.0, 4096.0, 0x4090000000000000),
            (1.0, 4097.0, 0x4090000000000000),
            (1.0, 8192.0, 0x4090000000000000),
            (1.0, 4294967292.0, 0x4090000000000000),
            (1.0, 4294967293.0, 0x4000000000000000),
            (1.0, 4294967295.0, 0x4000000000000000),
            (2.0, 0.0, 0x4000000000000000),
            (2.0, 100.0, 0x3ff0000000000000),
        ];
        for (i, first, bits) in cases {
            assert_eq!(live_of(&l, i, first).to_bits(), bits, "part {} first {}", i, first);
        }
    }

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

    #[test]
    #[should_panic(expected = "registered twice")]
    fn a_tenant_registered_twice_is_a_fault() {
        let mut list = Vec::new();
        let t = crate::sim_state::SimTenant { name: String::from("dice"), bit: 1.0 };
        add_tenant(&mut list, &t);
        add_tenant(&mut list, &crate::sim_state::SimTenant { name: String::from("other"), bit: 1.0 });
    }
}
