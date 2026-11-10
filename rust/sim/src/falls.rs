//! WHAT FALLS, AND WHAT IT LEAVES: the landings scheduled for things that let go (when and where each reaches the floor, whether the floor hears it, the glass it leaves there), and the
//! litter: square metres of broken glass in each half-metre cell of the floor, which makes a footfall louder and is ground down by it. This is the gameplay half of what breaks: the plates,
//! the chunks and the spray a player sees read none of it.
pub use game_types::falls::types::*;
use crate::world::js_min;
use alloc::{vec, vec::Vec};
use prim::floor;

/// the floor's cells, the world's floor rectangle
const CELL: f64 = 0.5;
const CELLS_X: usize = 80;
const CELLS_Z: usize = 56;
const CELLS: usize = CELLS_X * CELLS_Z;
/// a cell holds no more glass than its own floor (m²)
const CELL_MAX: f64 = 0.25;
/// landings that can wait at once in one list (a rule's own cap is under it)
pub const MAX_WAITING: usize = 96;

fn list_of(f: &mut Falls, kind: FallKind) -> &mut FallList {
    match kind { FallKind::Glass => &mut f.glass, FallKind::Pieces => &mut f.pieces }
}
/// the cell (x, z) is in; none where the world has no floor under it
fn cell(x: f64, z: f64) -> Option<usize> {
    let (ix, iz) = (floor(x / CELL), floor(z / CELL));
    if ix >= 0.0 && ix < CELLS_X as f64 && iz >= 0.0 && iz < CELLS_Z as f64 { Some(iz as usize * CELLS_X + ix as usize) } else { None }
}
/// `amount` m² of glass laid at (x, z)
fn lay(litter: &mut Vec<f64>, x: f64, z: f64, amount: f64) {
    if amount <= 0.0 { return; }
    let Some(k) = cell(x, z) else { return };
    if litter.is_empty() { *litter = vec![0.0; CELLS]; }
    litter[k] = js_min(CELL_MAX, litter[k] + amount);
}

/// has any glass been laid since the floor was cleared
pub fn any_glass(f: &Falls) -> bool { !f.litter.is_empty() }

/// A landing put on its list; the list at `cap`, its glass is laid now, its sound thickens the last landing that has one, and the answer is false.
pub fn schedule(f: &mut Falls, kind: FallKind, l: Landing, cap: f64) -> bool {
    let Falls { glass, pieces, litter } = f;
    let list = match kind { FallKind::Glass => glass, FallKind::Pieces => pieces };
    let n = list.waiting.len();
    if n as f64 >= cap || n >= MAX_WAITING {
        lay(litter, l.pos[0], l.pos[2], l.amount);
        if l.gain > 0.0 {
            if let Some(last) = list.waiting.iter_mut().rev().find(|w| w.gain > 0.0) { last.gain = js_min(1.0, last.gain + l.gain * 0.2); }
        }
        return false;
    }
    list.waiting.push(l);
    true
}

/// Every landing of its list whose time has come, in order: its glass laid, the floor told (one a `gap`, floor-wide), each said at the rule's own `level` unless it has one.
pub fn due(f: &mut Falls, kind: FallKind, time: f64, level: f64, gap: f64) -> Vec<FallDue> {
    let Falls { glass, pieces, litter } = f;
    let list = match kind { FallKind::Glass => glass, FallKind::Pieces => pieces };
    let mut out = Vec::new();
    let mut last_told = list.last_told;
    list.waiting.retain(|l| {
        if l.t > time { return true; }
        lay(litter, l.pos[0], l.pos[2], l.amount);
        let tells = l.told == FallTold::Always || (l.told == FallTold::IfItSounds && l.gain > 0.0);
        let heard = tells && !(time - last_told < gap);
        if heard { last_told = time; }
        out.push(FallDue { sound: l.sound, pos: l.pos, gain: l.gain, rate: l.rate, heard, level: if l.level != 0.0 { l.level } else { level }, who: l.who, dust: l.dust });
        false
    });
    list.last_told = last_told;
    out
}

/// A footfall at (x, z) of hearing level `level`: on glass it is louder and grinds the glass down.
pub fn step(f: &mut Falls, x: f64, z: f64, level: f64, crunch_full: f64, grind: f64, gain: f64, add: f64) -> FallStep {
    let quiet = FallStep { level, k: 0.0 };
    let (Some(c), false) = (cell(x, z), f.litter.is_empty()) else { return quiet };
    let a = f.litter[c];
    if a == 0.0 || a != a { return quiet; }
    let k = js_min(1.0, a / crunch_full);
    if k < 0.08 { return quiet; }
    f.litter[c] = a * grind;
    FallStep { level: level * (1.0 + gain * k) + add * k, k }
}

/// Nothing waits on its list.
pub fn clear_waiting(f: &mut Falls, kind: FallKind) { list_of(f, kind).waiting.clear(); }

/// Nothing waits on its list and its memory is new, and with the glass's list no glass lies.
pub fn clear(f: &mut Falls, kind: FallKind) {
    let list = list_of(f, kind);
    list.waiting.clear();
    list.last_told = FALL_NEVER;
    if kind == FallKind::Glass { f.litter.clear(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn landing(t: f64, sound: FallSound, gain: f64, amount: f64, told: FallTold) -> Landing {
        Landing { t, pos: [2.0, 0.5, 3.0], sound, gain, rate: 1.5, amount, who: 7.0, told, level: 4.0, dust: 0.0 }
    }
    fn lying(f: &Falls, x: f64, z: f64) -> f64 { cell(x, z).and_then(|k| f.litter.get(k).copied()).unwrap_or(0.0) }

    #[test]
    fn cell_is_the_floors_half_metre_grid() {
        assert_eq!(cell(0.0, 0.0), Some(0));
        assert_eq!(cell(0.49, 0.5), Some(80));
        assert_eq!(cell(39.99, 27.99), Some(4479));
        for (x, z) in [(40.0, 0.0), (0.0, 28.0), (-0.01, 0.0), (0.0, -0.01), (f64::NAN, 0.0)] { assert_eq!(cell(x, z), None); }
    }

    #[test]
    fn a_landing_lays_its_glass_tells_the_floor_once_a_gap_and_is_gone() {
        let mut f = Falls::blank();
        assert!(schedule(&mut f, FallKind::Glass, landing(1.0, FallSound::GlassShatter, 0.5, 0.1, FallTold::Always), 8.0));
        assert!(schedule(&mut f, FallKind::Pieces, landing(1.0, FallSound::Empty, 0.0, 0.0, FallTold::Never), 8.0));
        assert_eq!((f.glass.waiting.len(), f.pieces.waiting.len()), (1, 1));
        assert!(due(&mut f, FallKind::Glass, 0.5, 1.0, 0.0).is_empty());
        let d = due(&mut f, FallKind::Glass, 1.0, 9.0, 0.25);
        assert_eq!(d.len(), 1);
        assert_eq!((d[0].sound, d[0].pos, d[0].gain, d[0].rate, d[0].heard, d[0].level, d[0].who), (FallSound::GlassShatter, [2.0, 0.5, 3.0], 0.5, 1.5, true, 4.0, 7.0));
        assert!(f.glass.waiting.is_empty() && any_glass(&f));
        assert_eq!((lying(&f, 2.1, 3.1), lying(&f, -1.0, 3.1)), (0.1, 0.0));
        assert!(schedule(&mut f, FallKind::Glass, landing(1.1, FallSound::GlassTinkle, 0.5, 0.0, FallTold::IfItSounds), 8.0));
        assert!(!due(&mut f, FallKind::Glass, 1.1, 9.0, 0.25)[0].heard, "inside the gap");
        assert!(schedule(&mut f, FallKind::Glass, landing(1.3, FallSound::GlassTinkle, 0.0, 0.0, FallTold::IfItSounds), 8.0));
        assert!(!due(&mut f, FallKind::Glass, 1.3, 9.0, 0.25)[0].heard, "no sound of its own");
    }

    #[test]
    fn a_full_list_lays_the_glass_now_and_thickens_the_last_sound() {
        let mut f = Falls::blank();
        assert!(schedule(&mut f, FallKind::Glass, landing(1.0, FallSound::GlassTinkle, 0.5, 0.0, FallTold::Always), 1.0));
        assert!(!schedule(&mut f, FallKind::Glass, landing(1.0, FallSound::GlassTinkle, 0.5, 0.1, FallTold::Always), 1.0));
        assert_eq!((f.glass.waiting.len(), f.glass.waiting[0].gain, lying(&f, 2.0, 3.0)), (1, 0.6, 0.1));
    }

    #[test]
    fn a_footfall_on_glass_is_louder_and_grinds_it() {
        let mut f = Falls::blank();
        let bare = step(&mut f, 2.1, 3.1, 1.0, 0.1, 0.5, 2.0, 1.0);
        assert_eq!((bare.level, bare.k), (1.0, 0.0));
        lay(&mut f.litter, 2.1, 3.1, 0.1);
        let s = step(&mut f, 2.1, 3.1, 1.0, 0.1, 0.5, 2.0, 1.0);
        assert_eq!((s.level, s.k, lying(&f, 2.1, 3.1)), (4.0, 1.0, 0.05));
        let s = step(&mut f, 0.1, 0.1, 1.0, 0.1, 0.5, 2.0, 1.0);
        assert_eq!((s.level, s.k), (1.0, 0.0));
    }

    #[test]
    fn clearing_reaches_as_far_as_it_says() {
        let mut f = Falls::blank();
        lay(&mut f.litter, 2.1, 3.1, 0.1);
        f.glass.last_told = 1.0;
        assert!(schedule(&mut f, FallKind::Glass, landing(1.0, FallSound::Empty, 0.0, 0.0, FallTold::Never), 8.0));
        clear_waiting(&mut f, FallKind::Glass);
        assert!(f.glass.waiting.is_empty() && any_glass(&f) && f.glass.last_told == 1.0, "the landings alone");
        clear(&mut f, FallKind::Pieces);
        assert!(any_glass(&f), "the pieces' list leaves the glass");
        clear(&mut f, FallKind::Glass);
        assert!(!any_glass(&f) && f.glass.last_told == FALL_NEVER);
    }
}
