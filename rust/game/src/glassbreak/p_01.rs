//! src/game/glassbreak.ts lines 1 to 219
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the persisted switch alone: a machine with no storage answers on. The storage is the host's (glassbreak.ts glassBreakPersisted)
pub fn glass_break_persisted() -> bool { true }
/// the switch + knobs kept per machine: with no storage the defaults stand. The storage is the host's (glassbreak.ts loadGlassBreak)
pub fn load_glass_break(g: &mut crate::game::Game, tune: &mut crate::glassbreak::GbTune) {}
/// with no storage nothing is kept. The storage is the host's (glassbreak.ts saveGlassBreak)
pub fn save_glass_break(g: &mut crate::game::Game) {}

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

/// nothing of a pane's stands in this game's block: the panes part's first word, `used` (glassbreak.ts panesUntouched)
pub fn panes_untouched(g: &mut crate::game::Game) -> bool {
    ::sim::state::state(g.sim).panes.used == 0
}

/// a row's fields (panes_field), found once and kept (glassbreak.ts fields)
pub fn fields(g: &mut crate::game::Game) -> Vec<f64> {
    if g.m.glassbreak.f.is_none() {
        let mut f = Vec::new();
        let mut i = 0.0;
        while i < FIELDS { f.push(::sim::panes::field(i as u32) as f64); i += 1.0; }
        g.m.glassbreak.f = Some(f);
    }
    g.m.glassbreak.f.clone().unwrap()
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
    /// the list is the module's, in the module's order, and asked once
    #[test]
    fn fields_are_the_module_s_and_kept() {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let f = fields(&mut g);
        assert_eq!(f.len(), FIELDS as usize);
        for i in 0..f.len() { assert_eq!(f[i], ::sim::panes::field(i as u32) as f64, "field {}", i); }
        g.m.glassbreak.f.as_mut().unwrap()[0] = -1.0;
        assert_eq!(fields(&mut g)[0], -1.0);
    }
}
