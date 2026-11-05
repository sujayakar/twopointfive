//! src/sim/fallshome.ts lines 1 to 162
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the time a list's memory of its last landing holds when nothing has landed yet (fallshome.ts, the constructor and clear)
const NEVER: f64 = -1e9;

/// the columns of a landing, in the order of FP's keys (fallshome.ts FP; falls.rs P_…)
const LANDING: [(FallStoreLandingKey, usize); 12] = [
    (FallStoreLandingKey::T, sim::falls::P_T),
    (FallStoreLandingKey::X, sim::falls::P_X),
    (FallStoreLandingKey::Y, sim::falls::P_Y),
    (FallStoreLandingKey::Z, sim::falls::P_Z),
    (FallStoreLandingKey::Name, sim::falls::P_NAME),
    (FallStoreLandingKey::Gain, sim::falls::P_GAIN),
    (FallStoreLandingKey::Rate, sim::falls::P_RATE),
    (FallStoreLandingKey::Amount, sim::falls::P_AMOUNT),
    (FallStoreLandingKey::Who, sim::falls::P_WHO),
    (FallStoreLandingKey::Event, sim::falls::P_EVENT),
    (FallStoreLandingKey::Level, sim::falls::P_LEVEL),
    (FallStoreLandingKey::Dust, sim::falls::P_DUST),
];

/// a list the module has, as an index; none where lib.rs's exports answer as nothing
fn list_of(l: f64) -> Option<usize> {
    if l >= 0.0 && l < FALL_LISTS { Some(l as usize) } else { None }
}
/// a sound as its column holds it: its place in FALL_SOUNDS, none = −1
fn sound_place(name: FallSound) -> f64 {
    match name {
        FallSound::Empty => -1.0,
        FallSound::GlassTinkle => 0.0,
        FallSound::GlassShatter => 1.0,
    }
}
/// a column's place back as a sound (`FALL_SOUNDS[x] ?? ''`)
fn sound_at(x: f64) -> FallSound {
    if x == 0.0 {
        FallSound::GlassTinkle
    } else if x == 1.0 {
        FallSound::GlassShatter
    } else {
        FallSound::Empty
    }
}
/// a rule was taken by the module (`FALLS.there++`: the host's count, no game reads it)
fn took_there(g: &mut Game) {
    g.m.sim_fallshome.falls.there += 1.0;
}

/// The store over the game's block: its bit stands in the head, and neither list has told the floor of anything yet. (fallshome.ts FallStore.constructor)
pub fn fall_store_new(g: &mut crate::game::Game, home: crate::Opaque) -> crate::sim_fallshome::FallStore {
    let st = sim::state::state(g.sim);
    let bit = FALL_BIT as u32;
    debug_assert!(st.head.homes & bit == 0, "two stores of the falls over one game's block");
    st.head.homes |= bit;
    let base = (js::to_u32(crate::sim_state::sim_part(crate::sim_state::P_FALLS)) >> 3) as f64;
    st.falls.last_event_t = [NEVER; sim::falls::NLIST];
    FallStore { base, home }
}

/// The block as doubles, which only the `here` bodies and the reads went by. (fallshome.ts FallStore.d)
pub fn fall_store_d(g: &mut crate::game::Game) -> Vec<f64> {
    todo!("needs: no such thing: the block is typed columns in Rust, not a Vec of doubles; drop this stub")
}

/// Landings waiting in the glass's list. (fallshome.ts FallStore.waiting)
pub fn fall_store_waiting(g: &mut crate::game::Game) -> f64 {
    sim::state::state(g.sim).falls.n[sim::falls::L_GLASS]
}

/// Landings waiting in list `l`. (fallshome.ts FallStore.waitingIn)
pub fn fall_store_waiting_in(g: &mut crate::game::Game, l: f64) -> f64 {
    sim::state::state(g.sim).falls.n[l as usize]
}

/// Has any glass been laid since the map was cleared. (fallshome.ts FallStore.any)
pub fn fall_store_any(g: &mut crate::game::Game) -> bool {
    sim::state::state(g.sim).falls.any != 0.0
}

/// M² of glass in the cell under (x, z). (fallshome.ts FallStore.at)
pub fn fall_store_at(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    let k = cell(x, z);
    if k == NCELLS { 0.0 } else { sim::state::state(g.sim).falls.litter[k as usize] }
}

/// The cells that hold glass: [cell x, cell z, m²], row by row. (fallshome.ts FallStore.cells)
pub fn fall_store_cells(g: &mut crate::game::Game) -> Vec<[f64; 3]> {
    let litter = &sim::state::state(g.sim).falls.litter;
    let mut out = Vec::new();
    for k in 0..sim::falls::NCELLS {
        if litter[k] != 0.0 {
            out.push([(k % sim::falls::CELLS_X) as f64, (k / sim::falls::CELLS_X) as f64, litter[k]]);
        }
    }
    out
}

/// Landing i that waits in list `l`, by its columns' names. (fallshome.ts FallStore.landing)
pub fn fall_store_landing(g: &mut crate::game::Game, i: f64, l: Option<f64>) -> JsMap<crate::sim_fallshome::FallStoreLandingKey, f64> {
    let l = l.unwrap_or(FALL_GLASS) as usize;
    let p = &sim::state::state(g.sim).falls.p[l];
    let mut r = JsMap::default();
    for (key, col) in LANDING {
        r.set(key, p[col][i as usize]);
    }
    r
}

/// A landing put on list `l`; the list at `cap`, its glass is laid now and its sound thickens the last landing that has one. (fallshome.ts FallStore.schedule)
pub fn fall_store_schedule(g: &mut crate::game::Game, t: f64, x: f64, y: f64, z: f64, name: crate::sim_fallshome::FallSound, gain: f64, rate: f64, amount: f64, who: f64, event: f64, level: f64, cap: f64, l: Option<f64>, dust: Option<f64>) -> bool {
    let (l, dust) = (l.unwrap_or(FALL_GLASS), dust.unwrap_or(0.0));
    let n = sound_place(name);
    took_there(g);
    let Some(l) = list_of(l) else { return false };
    sim::falls::schedule(g.sim, l, t, x, y, z, n, gain, rate, amount, who, event, level, dust, cap) != 0
}

/// `amount` m² of glass laid at (x, z). (fallshome.ts FallStore.deposit)
pub fn fall_store_deposit(g: &mut crate::game::Game, x: f64, z: f64, amount: f64) {
    took_there(g);
    sim::falls::deposit(&mut sim::state::state(g.sim).falls, x, z, amount);
}

/// The landings of list `l` whose time has come, in order: each one's glass laid and the floor told. (fallshome.ts FallStore.due)
pub fn fall_store_due(g: &mut crate::game::Game, time: f64, level: f64, gap: f64, l: Option<f64>) -> Vec<crate::sim_fallshome::FallDue> {
    let Some(l) = list_of(l.unwrap_or(FALL_GLASS)) else { return none() };
    if sim::state::state(g.sim).falls.n[l] == 0.0 {
        return none();
    }
    g.m.sim_fallshome.falls.dues += 1.0;
    took_there(g);
    let mut m = [0.0; sim::falls::MAXP * sim::falls::DUE];
    let n = sim::falls::due(g.sim, l, time, level, gap, &mut m) as usize;
    let mut out = Vec::with_capacity(n);
    for r in m.chunks_exact(sim::falls::DUE).take(n) {
        out.push(FallDue { name: sound_at(r[0]), pos: [r[1], r[2], r[3]], gain: r[4], rate: r[5], heard: r[6] != 0.0, level: r[7], who: r[8], dust: r[9] });
    }
    out
}

/// A footfall at (x, z) of hearing level `level`: the level to file, and how much glass was underfoot (0‥1; 0 = no crunch). (fallshome.ts FallStore.step)
pub fn fall_store_step(g: &mut crate::game::Game, x: f64, z: f64, level: f64, crunch_full: f64, grind: f64, gain: f64, add: f64) -> crate::sim_fallshome::FallStoreStep {
    g.m.sim_fallshome.falls.steps += 1.0;
    took_there(g);
    let mut k = 0.0;
    let level = sim::falls::step(g.sim, x, z, level, crunch_full, grind, gain, add, &mut k);
    if k > 0.0 {
        g.m.sim_fallshome.falls.crunches += 1.0;
    }
    FallStoreStep { level, k }
}

/// May the floor be told of a landing of list `l` at `time` (one a `gap`, floor-wide)? If so the telling is remembered. (fallshome.ts FallStore.heard)
pub fn fall_store_heard(g: &mut crate::game::Game, time: f64, gap: f64, l: Option<f64>) -> bool {
    let l = l.unwrap_or(FALL_GLASS);
    took_there(g);
    let Some(l) = list_of(l) else { return false };
    sim::falls::heard(g.sim, l, time, gap) != 0
}

/// Nothing waits in list `l` and its memory is new, and with the glass's list no glass lies; `list_only`: the landings alone go. (fallshome.ts FallStore.clear)
pub fn fall_store_clear(g: &mut crate::game::Game, list_only: Option<bool>, l: Option<f64>) {
    let (list_only, l) = (list_only.unwrap_or(false), l.unwrap_or(FALL_GLASS));
    took_there(g);
    let Some(l) = list_of(l) else { return };
    sim::falls::clear(g.sim, l, !list_only, NEVER);
}

/// One rule: in Rust always the module's, so `here` is never run and this file's rules call the module directly. (fallshome.ts FallStore.rule)
pub fn fall_store_rule<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, what: &str, there: &mut dyn FnMut(&mut crate::game::Game) -> T, here: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    took_there(g);
    there(g)
}

/// The cell (x, z) is in, or NCELLS where the world has no floor under it. (fallshome.ts cell)
pub fn cell(x: f64, z: f64) -> f64 {
    let (ix, iz) = (m::floor(x / FALL_CELL), m::floor(z / FALL_CELL));
    if ix >= 0.0 && ix < FALL_CELLS_X && iz >= 0.0 && iz < FALL_CELLS_Z { iz * FALL_CELLS_X + ix } else { NCELLS }
}

/// TWIN of sim::falls::deposit, whose call needs the slot. (fallshome.ts depositHere)
pub fn deposit_here(d: &mut Vec<f64>, o: f64, x: f64, z: f64, amount: f64) {
    todo!("needs: g: the twin's call takes the game's slot and this stub has none; nothing in Rust calls it, so drop the stub")
}

/// TWIN of sim::falls::schedule, whose call needs the slot. (fallshome.ts scheduleHere)
pub fn schedule_here(d: &mut Vec<f64>, o: f64, l: f64, t: f64, x: f64, y: f64, z: f64, name: f64, gain: f64, rate: f64, amount: f64, who: f64, event: f64, level: f64, dust: f64, cap: f64) -> bool {
    todo!("needs: g: the twin's call takes the game's slot and this stub has none; nothing in Rust calls it, so drop the stub")
}

/// TWIN of sim::falls::due, whose call needs the slot. (fallshome.ts dueHere)
pub fn due_here(d: &mut Vec<f64>, o: f64, l: f64, time: f64, level: f64, gap: f64) -> Vec<crate::sim_fallshome::FallDue> {
    todo!("needs: g: the twin's call takes the game's slot and this stub has none; nothing in Rust calls it, so drop the stub")
}

/// TWIN of sim::falls::step, whose call needs the slot. (fallshome.ts stepHere)
pub fn step_here(d: &mut Vec<f64>, o: f64, x: f64, z: f64, level: f64, crunch_full: f64, grind: f64, gain: f64, add: f64) -> crate::sim_fallshome::StepHere {
    todo!("needs: g: the twin's call takes the game's slot and this stub has none; nothing in Rust calls it, so drop the stub")
}

/// TWIN of sim::falls::clear, whose call needs the slot. (fallshome.ts clearHere)
pub fn clear_here(d: &mut Vec<f64>, o: f64, l: f64, list_only: bool) {
    todo!("needs: g: the twin's call takes the game's slot and this stub has none; nothing in Rust calls it, so drop the stub")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_is_the_floors_half_metre_grid() {
        assert_eq!(cell(0.0, 0.0), 0.0);
        assert_eq!(cell(0.49, 0.5), 80.0);
        assert_eq!(cell(39.99, 27.99), 4479.0);
        for (x, z) in [(40.0, 0.0), (0.0, 28.0), (-0.01, 0.0), (0.0, -0.01), (f64::NAN, 0.0)] {
            assert_eq!(cell(x, z), NCELLS);
        }
    }

    /// the face over the real rust/sim, in one test since a slot's block is shared by the whole process
    #[test]
    fn the_face_answers_as_the_typescript_reads_the_module() {
        let mut game = Game::blank();
        let g = &mut game;
        g.sim = 3;
        sim::state::state(g.sim).falls.last_event_t = [NEVER; sim::falls::NLIST];

        // the sounds' places, a list's own count, a landing by its names in FP's order
        assert!(fall_store_schedule(g, 1.0, 2.0, 0.5, 3.0, FallSound::GlassShatter, 0.5, 1.5, 0.1, 7.0, FALL_HEARD, 4.0, 8.0, None, None));
        assert!(fall_store_schedule(g, 1.0, 2.0, 0.5, 3.0, FallSound::Empty, 0.0, 1.0, 0.0, 8.0, FALL_UNHEARD, 0.0, 8.0, Some(FALL_PIECES), Some(2.5)));
        assert_eq!((fall_store_waiting(g), fall_store_waiting_in(g, FALL_PIECES)), (1.0, 1.0));
        let row = fall_store_landing(g, 0.0, Some(FALL_PIECES));
        assert_eq!(row.get(FallStoreLandingKey::Name), Some(&-1.0));
        assert_eq!(row.get(FallStoreLandingKey::Dust), Some(&2.5));
        assert_eq!(row.keys()[0], FallStoreLandingKey::T);
        assert_eq!(row.keys()[11], FallStoreLandingKey::Dust);

        // a list the module does not have is answered as nothing
        assert!(!fall_store_schedule(g, 1.0, 2.0, 0.5, 3.0, FallSound::Empty, 0.0, 1.0, 0.0, 8.0, FALL_UNHEARD, 0.0, 8.0, Some(FALL_LISTS), None));
        assert!(!fall_store_heard(g, 1.0, 0.0, Some(-1.0)));
        assert!(fall_store_due(g, 9.0, 1.0, 0.0, Some(FALL_LISTS)).is_empty());

        // nothing due yet: no landing, and the count of dues still moves as the list is not empty
        assert!(fall_store_due(g, 0.5, 1.0, 0.0, None).is_empty());
        assert_eq!(g.m.sim_fallshome.falls.dues, 1.0);

        // due: each field back in its place, the glass laid, the floor told
        let due = fall_store_due(g, 1.0, 9.0, 0.25, None);
        assert_eq!(due.len(), 1);
        let d = due[0];
        assert_eq!(d.name, FallSound::GlassShatter);
        assert_eq!(d.pos, [2.0, 0.5, 3.0]);
        assert_eq!((d.gain, d.rate, d.heard, d.level, d.who, d.dust), (0.5, 1.5, true, 4.0, 7.0, 0.0));
        assert_eq!(fall_store_waiting(g), 0.0);
        assert!(fall_store_any(g));
        assert_eq!(fall_store_at(g, 2.1, 3.1), 0.1);
        assert_eq!(fall_store_at(g, -1.0, 3.1), 0.0);
        assert_eq!(fall_store_cells(g), vec![[4.0, 6.0, 0.1]]);
        let (dues, empty) = (g.m.sim_fallshome.falls.dues, fall_store_due(g, 2.0, 9.0, 0.25, None));
        assert!(empty.is_empty());
        assert_eq!(g.m.sim_fallshome.falls.dues, dues, "an empty list returns before the count");

        // a footfall that crunches and one that does not
        let s = fall_store_step(g, 2.1, 3.1, 1.0, 0.1, 0.5, 2.0, 1.0);
        assert_eq!((s.level, s.k), (1.0 * (1.0 + 2.0 * 1.0) + 1.0 * 1.0, 1.0));
        assert_eq!(fall_store_at(g, 2.1, 3.1), 0.05);
        let s = fall_store_step(g, 0.1, 0.1, 1.0, 0.1, 0.5, 2.0, 1.0);
        assert_eq!((s.level, s.k), (1.0, 0.0));
        assert_eq!((g.m.sim_fallshome.falls.steps, g.m.sim_fallshome.falls.crunches), (2.0, 1.0));

        // heard: one a gap, and clear's three reaches
        assert!(!fall_store_heard(g, 1.1, 0.25, None));
        assert!(fall_store_heard(g, 1.25, 0.25, None));
        fall_store_clear(g, Some(true), None);
        assert!(fall_store_any(g), "list only: the glass stays");
        assert!(!fall_store_heard(g, 1.3, 0.25, None), "list only: the memory stays");
        fall_store_clear(g, None, Some(FALL_PIECES));
        assert_eq!(fall_store_waiting_in(g, FALL_PIECES), 0.0);
        assert!(fall_store_any(g), "the pieces' list leaves the glass");
        fall_store_clear(g, None, None);
        assert!(!fall_store_any(g));
        assert!(fall_store_cells(g).is_empty());
        assert!(fall_store_heard(g, 1.3, 0.25, None), "the glass's memory is new");

        // the one rule takes the module's road
        let there = g.m.sim_fallshome.falls.there;
        assert_eq!(fall_store_rule(g, "x", &mut |_| 1, &mut |_| 2), 1);
        assert_eq!(g.m.sim_fallshome.falls.there, there + 1.0);
    }
}
