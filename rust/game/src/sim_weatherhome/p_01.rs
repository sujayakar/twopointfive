//! src/sim/weatherhome.ts lines 1 to 41
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The block of a slot as doubles: `st.d`. Take it for one read or write and let it go.   (state.ts SimState.d)
fn block_d(slot: usize) -> &'static mut [f64] {
    unsafe { core::slice::from_raw_parts_mut(::sim::state::addr(slot) as *mut f64, ::sim::state::LEN >> 3) }
}
/// The same doubles, to read.
fn block_d_read(slot: usize) -> &'static [f64] {
    unsafe { core::slice::from_raw_parts(::sim::state::addr(slot) as *const f64, ::sim::state::LEN >> 3) }
}

/// The module's scratch, as doubles from its first: what `f64()` reads at `wxScratch()`.   (weatherhome.ts wxOut's `m[a + i]`)
fn scratch_d() -> &'static [f64] {
    unsafe { core::slice::from_raw_parts(::sim::sim_scratch() as *const f64, ::sim::sim_scratch_len()) }
}

/// The module's scratch, as an index of doubles over its memory.   (weatherhome.ts wxScratch)
pub fn wx_scratch() -> f64 {
    (::sim::sim_scratch() as usize >> 3) as f64
}

/// How many of `boxes` are roofs at all, by the rule roofsOf makes the table with.   (weatherhome.ts roofsCounted)
pub fn roofs_counted(boxes: &[crate::sim_weatherhome::RoofsCountedBoxesItem], ceiling: f64) -> f64 {
    ::sim::weather::roofs_begin();
    for b in boxes {
        ::sim::weather::roofs_box(b.c[0], b.c[1], b.c[2], b.h[0], b.h[1], b.h[2], ceiling);
    }
    ::sim::weather::roofs_seen() as f64
}

/// Copy `n` doubles from the scratch.   (weatherhome.ts wxOut)
pub fn wx_out(n: f64) -> Vec<f64> {
    let m = scratch_d();
    let mut out = Vec::new();
    let mut i = 0usize;
    while (i as f64) < n {
        out.push(m[i]);
        i += 1;
    }
    out
}

pub mod wx_knobs {
    /// The body this line of the TypeScript has: nothing. rustknobs.ts:59 replaces it, and that needs `g` (notes, Skeleton).   (weatherhome.ts WX_KNOBS.here)
    pub fn here() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `st.d[at >>> 3]` is the field the column names, for the weather's part of a real block (wx_at's `sim_part` is a stub in this crate, so its sum is written out here).
    #[test]
    fn a_column_lands_on_its_field() {
        let at = |c: u32| (::sim::state::PARTS[crate::sim_state::P_WEATHER as usize].off + ::sim::weather::col(c)) >> 3;
        let w = crate::sim_weatherhome::wx();
        block_d(1)[at(w.storm as u32)] = 2.5;
        block_d(1)[at(w.air_acc as u32)] = 3.5;
        block_d(1)[at(w.roofs as u32) + 2] = 4.5;
        let s = &::sim::state::state(1).weather;
        assert_eq!(s.storm.to_bits(), 2.5f64.to_bits());
        assert_eq!(s.air.acc.to_bits(), 3.5f64.to_bits());
        assert_eq!(s.roofs[2].to_bits(), 4.5f64.to_bits());
    }

    #[test]
    fn out_copies_the_first_doubles_of_the_scratch() {
        let n = ::sim::sim_scratch_len();
        let m = unsafe { core::slice::from_raw_parts_mut(::sim::sim_scratch(), n) };
        m[0] = 1.25;
        m[1] = -0.0;
        m[2] = 7.0;
        let got = wx_out(2.0);
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].to_bits(), 1.25f64.to_bits());
        assert_eq!(got[1].to_bits(), (-0.0f64).to_bits());
        assert!(wx_out(0.0).is_empty());
        assert_eq!((wx_scratch() as usize) << 3, ::sim::sim_scratch() as usize);
    }

    /// The centre and the half size reach the rule in their places, and the ceiling with them.
    #[test]
    fn roofs_counted_hands_each_box_over_in_order() {
        let boxy = |c: [f64; 3], h: [f64; 3]| RoofsCountedBoxesItem { c: c.to_vec(), h: h.to_vec() };
        let boxes = [
            boxy([1.0, 3.0, 2.0], [0.5, 0.1, 0.5]),   // a slab at the ceiling
            boxy([1.0, 3.0, 2.0], [0.1, 0.5, 0.5]),   // its x and y half sizes crossed: thin and tall
            boxy([3.0, 1.0, 2.0], [0.5, 0.1, 0.5]),   // its x and y centre crossed: far below
        ];
        assert_eq!(roofs_counted(&boxes, 3.0), 1.0);
        assert_eq!(roofs_counted(&boxes, 1.0), 1.0);
        assert_eq!(roofs_counted(&boxes, 9.0), 0.0);
        assert_eq!(roofs_counted(&[], 3.0), 0.0);
    }
}
