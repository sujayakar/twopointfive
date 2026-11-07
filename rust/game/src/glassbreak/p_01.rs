//! src/game/glassbreak.ts lines 1 to 219
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the switch + knobs kept per machine: with no storage the defaults stand. The storage is the host's (glassbreak.ts loadGlassBreak)
pub fn load_glass_break(g: &mut crate::game::Game, tune: &mut crate::glassbreak::GbTune) {}
/// the one gate every entry point takes first (glassbreak.ts glassBreakLive). GLASSFX.on is settled on.
pub fn glass_break_live(g: &mut crate::game::Game) -> bool {
    g.tune.glass_break && crate::destruction::chaos_of(g) > 0.0 && g.engine.glass_fracture != Some(false)
}

/// what the module has counted at place `i` of panes.rs Tally; no place (a count only the host keeps) is 0 (glassbreak.ts tally)
pub fn tally(i: Option<f64>) -> f64 {
    let Some(i) = i else { return 0.0; };
    debug_assert!(i >= 0.0 && (i as usize) < core::mem::size_of::<::sim::panes::Tally>() / 8);
    // `f64()[x.panes_tally() / 8 + i]`: Tally is repr(C), all doubles
    unsafe { *(core::ptr::addr_of!(::sim::panes::TALLY) as *const f64).add(i as usize) }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// TALLY_AT's places are panes.rs Tally's fields, name for name; a count with no place reads 0
    #[test]
    fn each_count_reads_its_own_field_of_the_module_s_tally() {
        unsafe {
            ::sim::panes::TALLY = ::sim::panes::Tally { panes: 1.0, impacts: 2.0, sites: 3.0, runs: 4.0, dropped: 5.0, diced: 6.0, clumps: 7.0, sheets: 8.0, blasts: 9.0, thinks: 10.0, raked: 11.0, queue_peak: 12.0, queue_full: 13.0 };
        }
        let at = tally_at();
        let read: [f64; 11] = [tally(at.panes), tally(at.impacts), tally(at.sites), tally(at.runs), tally(at.dropped), tally(at.diced), tally(at.clumps), tally(at.sheets), tally(at.blasts), tally(at.thinks), tally(at.raked)];
        assert_eq!(read, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
        assert_eq!(tally(at.shards), 0.0);
        assert_eq!(tally(None), 0.0);
    }
}
