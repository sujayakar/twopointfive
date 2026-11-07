//! src/game/doors.ts lines 1 to 296
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One double of the slab's row, as it stands now.   (doors.ts SlabRow.g)
pub fn slab_row_g(g: &mut crate::game::Game, id: SlabRowId, k: crate::doors::SlabRowGK) -> f64 {
    let r = row_now(g, id);
    slab_col(&::sim::state::state(g.sim).slabs, k, r)
}

/// The slab row's word of truths.   (doors.ts SlabRow.f)
pub fn slab_row_f(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    let r = row_now(g, id);
    ::sim::state::state(g.sim).slabs.flags[r] as f64
}

/// Its centre.   (doors.ts SlabRow.p)
pub fn slab_row_p(g: &mut crate::game::Game, id: SlabRowId) -> [f64; 3] {
    let x = slab_row_g(g, id, SlabRowGK::Px);
    let y = slab_row_g(g, id, SlabRowGK::Py);
    let z = slab_row_g(g, id, SlabRowGK::Pz);
    [x, y, z]
}

/// Its velocity.   (doors.ts SlabRow.v)
pub fn slab_row_v(g: &mut crate::game::Game, id: SlabRowId) -> [f64; 3] {
    let x = slab_row_g(g, id, SlabRowGK::Vx);
    let y = slab_row_g(g, id, SlabRowGK::Vy);
    let z = slab_row_g(g, id, SlabRowGK::Vz);
    [x, y, z]
}

/// The world angle of its width axis.   (doors.ts SlabRow.a)
pub fn slab_row_a(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::A)
}

/// Its turn.   (doors.ts SlabRow.spin)
pub fn slab_row_spin(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Spin)
}

/// 0 upright to π/2 flat.   (doors.ts SlabRow.tip)
pub fn slab_row_tip(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Tip)
}

/// How fast it is tipping.   (doors.ts SlabRow.tipV)
pub fn slab_row_tip_v(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::TipV)
}

/// The slab's own clock.   (doors.ts SlabRow.t)
pub fn slab_row_t(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::T)
}

/// The angle the door reads as while off.   (doors.ts SlabRow.ang)
pub fn slab_row_ang(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Ang)
}

/// The clock before which a slab with no clear spot is not searched for again.   (doors.ts SlabRow.wedged)
pub fn slab_row_wedged(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Wedged)
}

/// The way its top fell: +1 the CCW normal of its width axis, −1 the other.   (doors.ts SlabRow.side)
pub fn slab_row_side(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    let f = slab_row_f(g, id);
    if flag(f, crate::sim_slabshome::slf().back) { -1.0 } else { 1.0 }
}

/// Lying at rest.   (doors.ts SlabRow.rest)
pub fn slab_row_rest(g: &mut crate::game::Game, id: SlabRowId) -> bool {
    let f = slab_row_f(g, id);
    flag(f, crate::sim_slabshome::slf().rest)
}

/// Slammed flat.   (doors.ts SlabRow.landed)
pub fn slab_row_landed(g: &mut crate::game::Game, id: SlabRowId) -> bool {
    let f = slab_row_f(g, id);
    flag(f, crate::sim_slabshome::slf().landed)
}

/// Whose doing it was.   (doors.ts SlabRow.who)
pub fn slab_row_who(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    let r = row_now(g, id);
    ::sim::state::state(g.sim).slabs.who[r] as f64
}

/// The lock rail of a leaf gunfire took apart: half width, half length, where its handles are.   (doors.ts SlabRow.bits)
pub fn slab_row_bits(g: &mut crate::game::Game, id: SlabRowId) -> Option<[f64; 4]> {
    let f = slab_row_f(g, id);
    if !flag(f, crate::sim_slabshome::slf().bits) {
        return None;
    }
    let w = slab_row_g(g, id, SlabRowGK::Bw);
    let l = slab_row_g(g, id, SlabRowGK::Bl);
    let x = slab_row_g(g, id, SlabRowGK::Bx);
    let y = slab_row_g(g, id, SlabRowGK::By);
    Some([w, l, x, y])
}

/// `T.rowOf(this.door)`, for a read of the row: a SlabRow is good only while its door has a slab (Door.slab answers null otherwise).
fn row_now(g: &mut crate::game::Game, id: SlabRowId) -> usize {
    let r = crate::sim_slabshome::slab_store_row_of(g, id.0 as f64);
    debug_assert!(r >= 0.0, "a SlabRow read with no slab on its door");
    r as usize
}

/// `d[b8 + O[k] + r]`: the word names the column of rust/sim's Slabs.
fn slab_col(s: &::sim::slabs::Slabs, k: crate::doors::SlabRowGK, r: usize) -> f64 {
    match k {
        SlabRowGK::Px => s.px[r],
        SlabRowGK::Py => s.py[r],
        SlabRowGK::Pz => s.pz[r],
        SlabRowGK::Vx => s.vx[r],
        SlabRowGK::Vy => s.vy[r],
        SlabRowGK::Vz => s.vz[r],
        SlabRowGK::A => s.a[r],
        SlabRowGK::Spin => s.spin[r],
        SlabRowGK::Tip => s.tip[r],
        SlabRowGK::TipV => s.tip_v[r],
        SlabRowGK::T => s.t[r],
        SlabRowGK::Ang => s.ang[r],
        SlabRowGK::Wedged => s.wedged[r],
        SlabRowGK::Bw => s.bw[r],
        SlabRowGK::Bl => s.bl[r],
        SlabRowGK::Bx => s.bx[r],
        SlabRowGK::By => s.by[r],
    }
}

/// `flags & bit` as a condition.
fn flag(f: f64, bit: f64) -> bool {
    (js::to_i32(f) & js::to_i32(bit)) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A word is a 32-bit AND, whatever else is set in it, and SLF's bits are rust/sim's.
    #[test]
    fn flags_are_bits_of_the_word() {
        assert!(flag(12.0, 4.0));
        assert!(!flag(11.0, 4.0));
        assert!(flag(4294967295.0, 8.0));
        let l = crate::sim_slabshome::slf();
        assert_eq!([l.rest, l.landed, l.back, l.bits], [::sim::slabs::F_REST as f64, ::sim::slabs::F_LANDED as f64, ::sim::slabs::F_BACK as f64, ::sim::slabs::F_BITS as f64]);
    }

    /// Each word reads its own column: rust/sim's fields written in SLAB_F64's order, each word found at its place in that list.
    #[test]
    fn a_word_reads_its_own_column() {
        let mut s: ::sim::slabs::Slabs = unsafe { core::mem::zeroed() };
        let cols: [&mut [f64; ::sim::slabs::CAP]; 17] = [&mut s.px, &mut s.py, &mut s.pz, &mut s.vx, &mut s.vy, &mut s.vz, &mut s.a, &mut s.spin, &mut s.tip, &mut s.tip_v, &mut s.t, &mut s.ang, &mut s.wedged, &mut s.bw, &mut s.bl, &mut s.bx, &mut s.by];
        for (i, c) in cols.into_iter().enumerate() {
            c[3] = (i + 1) as f64;
        }
        let names = crate::sim_slabshome::slab_f64();
        for k in [SlabRowGK::Px, SlabRowGK::Py, SlabRowGK::Pz, SlabRowGK::Vx, SlabRowGK::Vy, SlabRowGK::Vz, SlabRowGK::A, SlabRowGK::Spin, SlabRowGK::Tip, SlabRowGK::TipV, SlabRowGK::T, SlabRowGK::Ang, SlabRowGK::Wedged, SlabRowGK::Bw, SlabRowGK::Bl, SlabRowGK::Bx, SlabRowGK::By] {
            let at = names.iter().position(|n| n.as_str() == k.as_str()).unwrap();
            assert_eq!(slab_col(&s, k, 3), (at + 1) as f64, "{}", k.as_str());
            assert_eq!(slab_col(&s, k, 2), 0.0);
        }
    }

}
