//! WHAT FALLS, AND WHAT IT LEAVES (src/sim/fallshome.ts; the rules that call it are src/game/shards.ts and src/game/debris.ts) — the landings the
//! simulation has scheduled for things that let go (when and where each reaches the floor, whether the floor hears it, the glass it leaves there), and the
//! LITTER map: square metres of broken glass lying in each half-metre cell of the floor, which makes a footfall louder and is ground down by it.
//!
//! The landings wait in LISTS, one for each kind of thing that falls (L_GLASS: fragments of a pane · L_PIECES: the pieces a thing that breaks throws), each
//! with its own count and its own memory of when the floor was last told, so that one kind's rule (its cap, its gap, its going inert) never moves another's.
//!
//! This is the GAMEPLAY half of what breaks. The plates, the chunks and the spray a player sees are dressing: they read none of this and write none of it.
//!
//! STATEMENT FOR STATEMENT with the TypeScript in fallshome.ts (tools/qa/falls-ident.ts holds the two together, column for column). No length and no
//! angle is worked out here, so the row answers under either maths.
use crate::state::state;
use crate::world::js_min;
use core::mem::size_of;
use prim::floor;

/// the floor's cells: the world's floor rectangle (src/scene/world.ts WORLD: 40 m × 28 m from the origin) in cells of CELL metres
pub const CELL: f64 = 0.5;
pub const CELLS_X: usize = 80;
pub const CELLS_Z: usize = 56;
pub const NCELLS: usize = CELLS_X * CELLS_Z;
/// a cell holds no more glass than its own floor (m²)
pub const CELL_MAX: f64 = 0.25;
/// landings that can wait at once in one list (a rule's own cap — SHARDS.pendingMax — is under it; and a whole list coming due in one tick fits the
/// scratch the host reads it from: MAXP × DUE doubles of lib.rs SCRATCH_LEN)
pub const MAXP: usize = 96;
/// the lists: fragments of glass · pieces thrown by a thing that breaks
pub const L_GLASS: usize = 0;
pub const L_PIECES: usize = 1;
pub const NLIST: usize = 2;

// a landing's columns (fallshome.ts FP names them by the same numbers)
pub const P_T: usize = 0;
pub const P_X: usize = 1;
pub const P_Y: usize = 2;
pub const P_Z: usize = 3;
/// the sound it lands with, by its place in fallshome.ts FALL_SOUNDS
pub const P_NAME: usize = 4;
pub const P_GAIN: usize = 5;
pub const P_RATE: usize = 6;
/// m² of glass it leaves
pub const P_AMOUNT: usize = 7;
pub const P_WHO: usize = 8;
/// is the floor told of it: 0 no · 1 if it has a sound of its own (gain > 0) · 2 yes
pub const P_EVENT: usize = 9;
/// the hearing level it is told at (0: the rule's own, handed in when it lands)
pub const P_LEVEL: usize = 10;
/// the dust it raises where it lands (a puff's scale; 0 = none): the host's to let out, by its own caps
pub const P_DUST: usize = 11;
pub const NP: usize = 12;

/// The falls' part of a game's block.
#[repr(C)]
pub struct Falls {
    /// m² of glass in each cell, row by row (z × CELLS_X + x)
    pub litter: [f64; NCELLS],
    /// the landings waiting in each list, a column a number, in the order they were scheduled
    pub p: [[[f64; MAXP]; NP]; NLIST],
    /// how many wait in each list
    pub n: [f64; NLIST],
    /// glass has been laid since the map was last cleared (the TypeScript's `litter.size`)
    pub any: f64,
    /// the game time the floor was last told of a landing of each list (one a gap, floor-wide)
    pub last_event_t: [f64; NLIST],
    pub spare: f64,
}
const _: () = assert!(size_of::<Falls>() == (NCELLS + NLIST * NP * MAXP + 2 * NLIST + 2) * 8);

/// a landing that is due, as the host takes it back (lib.rs scratch, DUE doubles each): its sound, where, how loud and at what rate, whether the floor is
/// told of it, at what level, and whose it was
pub const DUE: usize = 10;
const _: () = assert!(MAXP * DUE <= crate::SCRATCH_LEN);

/// the cell (x, z) is in, or NCELLS where there is no floor of the world's under it
#[inline]
fn cell(x: f64, z: f64) -> usize {
    let (ix, iz) = (floor(x / CELL), floor(z / CELL));
    if ix >= 0.0 && ix < CELLS_X as f64 && iz >= 0.0 && iz < CELLS_Z as f64 {
        iz as usize * CELLS_X + ix as usize
    } else {
        NCELLS
    }
}
/// shards.ts deposit: `amount` m² of glass laid at (x, z)
pub fn deposit(f: &mut Falls, x: f64, z: f64, amount: f64) {
    if amount <= 0.0 {
        return;
    }
    let k = cell(x, z);
    if k == NCELLS {
        return;
    }
    f.litter[k] = js_min(CELL_MAX, f.litter[k] + amount);
    f.any = 1.0;
}
/// a landing put on list `l` — or, the list at the rule's cap, its glass laid now and its sound thickening the last landing of the list that has one.
/// 1 = it waits.
#[allow(clippy::too_many_arguments)]
pub fn schedule(g: usize, l: usize, t: f64, x: f64, y: f64, z: f64, name: f64, gain: f64, rate: f64, amount: f64, who: f64, event: f64, level: f64, dust: f64, cap: f64) -> u32 {
    let f = &mut state(g).falls;
    let n = f.n[l] as usize;
    if f.n[l] >= cap || n >= MAXP {
        deposit(f, x, z, amount);
        if gain > 0.0 {
            let mut i = n;
            while i > 0 {
                i -= 1;
                if f.p[l][P_GAIN][i] > 0.0 {
                    f.p[l][P_GAIN][i] = js_min(1.0, f.p[l][P_GAIN][i] + gain * 0.2);
                    break;
                }
            }
        }
        return 0;
    }
    let row = [t, x, y, z, name, gain, rate, amount, who, event, level, dust];
    for k in 0..NP {
        f.p[l][k][n] = row[k];
    }
    f.n[l] += 1.0;
    1
}
/// every landing of list `l` whose time has come, in order — its glass laid, the floor told (one event a gap) — written to `out` for the host to play and
/// to file. Returns how many.
pub fn due(g: usize, l: usize, time: f64, level: f64, gap: f64, out: &mut [f64]) -> u32 {
    let f = &mut state(g).falls;
    let n = f.n[l] as usize;
    let (mut w, mut made) = (0, 0);
    for i in 0..n {
        if f.p[l][P_T][i] > time {
            if w != i {
                for k in 0..NP {
                    f.p[l][k][w] = f.p[l][k][i];
                }
            }
            w += 1;
            continue;
        }
        let (x, z, gain, ev) = (f.p[l][P_X][i], f.p[l][P_Z][i], f.p[l][P_GAIN][i], f.p[l][P_EVENT][i]);
        deposit(f, x, z, f.p[l][P_AMOUNT][i]);
        let mut heard = 0.0;
        if (ev == 2.0 || (ev == 1.0 && gain > 0.0)) && !(time - f.last_event_t[l] < gap) {
            f.last_event_t[l] = time;
            heard = 1.0;
        }
        let o = made * DUE;
        if o + DUE <= out.len() {
            let lv = f.p[l][P_LEVEL][i];
            out[o] = f.p[l][P_NAME][i];
            out[o + 1] = x;
            out[o + 2] = f.p[l][P_Y][i];
            out[o + 3] = z;
            out[o + 4] = gain;
            out[o + 5] = f.p[l][P_RATE][i];
            out[o + 6] = heard;
            out[o + 7] = if lv != 0.0 { lv } else { level };
            out[o + 8] = f.p[l][P_WHO][i];
            out[o + 9] = f.p[l][P_DUST][i];
        }
        made += 1;
    }
    for i in w..n {
        for k in 0..NP {
            f.p[l][k][i] = 0.0;
        }
    }
    f.n[l] = w as f64;
    made as u32
}
/// shards.ts step: a footfall at (x, z) of hearing level `level` — on glass it is louder and grinds the glass down. Returns the level to file; how much
/// glass was underfoot (0‥1; 0 = no crunch) → `k`.
#[allow(clippy::too_many_arguments)]
pub fn step(g: usize, x: f64, z: f64, level: f64, crunch_full: f64, grind: f64, gain: f64, add: f64, k_out: &mut f64) -> f64 {
    let f = &mut state(g).falls;
    *k_out = 0.0;
    let c = cell(x, z);
    if c == NCELLS {
        return level;
    }
    let a = f.litter[c];
    if a == 0.0 || a != a {
        return level;
    }
    let k = js_min(1.0, a / crunch_full);
    if k < 0.08 {
        return level;
    }
    f.litter[c] = a * grind;
    *k_out = k;
    level * (1.0 + gain * k) + add * k
}
/// nothing waits in list `l`; `all`: the floor's memory of the last landing of that list it was told of is forgotten too (`events`), and with the glass's
/// list the glass that lies
pub fn clear(g: usize, l: usize, all: bool, events: f64) {
    let f = &mut state(g).falls;
    f.p[l] = [[0.0; MAXP]; NP];
    f.n[l] = 0.0;
    if all {
        f.last_event_t[l] = events;
        if l == L_GLASS {
            f.litter = [0.0; NCELLS];
            f.any = 0.0;
        }
    }
}
/// may the floor be told of a landing of list `l` now (one a gap, floor-wide)? If so the telling is remembered.
pub fn heard(g: usize, l: usize, time: f64, gap: f64) -> u32 {
    let f = &mut state(g).falls;
    if time - f.last_event_t[l] < gap {
        return 0;
    }
    f.last_event_t[l] = time;
    1
}
