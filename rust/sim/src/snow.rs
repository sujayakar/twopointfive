//! LYING SNOW UNDERFOOT: how boggy the ground is, read off the one table of the snow (world.rs `snow`, a byte a static, each box's place and turn being its own row's)
//! under the roofs in force. A bounce's own numbers (`dead`, `grip`, `stick`) are no part of how boggy the ground is.
use crate::world::{js_abs, js_max, js_min, REC, R_C, R_COS, R_H, R_SIN, SNOW_BANK, SNOW_OLD, SNOW_PACKED, World};
use game_types::knobs::Knobs;

/// how many roofs a level has in force, at most
pub const MAX_ROOFS: usize = 4;

/// lying snow as the rule's numbers stand: a landing higher than `top` is not on the ground · snow whose top is within `bank` of the landing is what it landed in · how boggy trodden snow is, and old snow · the share of its bounce and of its slide a thing loses in deep snow, and how much faster than on a floor it may be going and still stop dead · the roofs in force, each one's footprint (x0, z0, x1, z1)
pub struct Snow {
    pub top: f64,
    pub bank: f64,
    pub packed: f64,
    pub old: f64,
    pub dead: f64,
    pub grip: f64,
    pub stick: f64,
    pub roofs: usize,
    pub roof: [f64; MAX_ROOFS * 4],
}

/// is (x, z) over the static whose row starts at `q`, in plan
#[inline(always)]
fn over(rec: &[f64], q: usize, x: f64, z: f64) -> bool {
    let (cs, sn) = (rec[q + R_COS], rec[q + R_SIN]);
    let (px, pz) = (x - rec[q + R_C], z - rec[q + R_C + 2]);
    js_abs(cs * px - sn * pz) <= rec[q + R_H] && js_abs(sn * px + cs * pz) <= rec[q + R_H + 2]
}
/// HOW BOGGY THE GROUND IS where a thing touched down at (x, y, z), 0‥1 (0 = a floor). Up on something, only snow that stands or caps as high bogs; at the ground, nothing under a roof does, and a thin lay over the spot says how much: trodden, old, or bare
pub fn bog_of(w: &World, s: &Snow, x: f64, y: f64, z: f64) -> f64 {
    let n = w.boxes();
    if !(y < s.top) {
        for bi in 0..n {
            if w.snow[bi] == SNOW_BANK {
                let q = bi * REC;
                if js_abs(w.rec[q + R_C + 1] + w.rec[q + R_H + 1] - y) <= s.bank && over(&w.rec, q, x, z) {
                    return 1.0;
                }
            }
        }
        return 0.0;
    }
    if crate::weather::under(&s.roof, s.roofs, x, z) {
        return 0.0;
    }
    let mut k = 1.0;
    for bi in 0..n {
        let kind = w.snow[bi];
        if kind == 0 || kind == SNOW_BANK || !over(&w.rec, bi * REC, x, z) {
            continue;
        }
        let v = if kind == SNOW_PACKED { s.packed } else if kind == SNOW_OLD { s.old } else { 0.0 };
        if v < k {
            k = v;
        }
    }
    js_max(0.0, js_min(1.0, k))
}

/// How boggy the ground of world `w` is under these roofs where a thing touched down at (x, y, z), 0‥1
pub fn bog_at(roofs: &[[f64; 4]], w: &World, x: f64, y: f64, z: f64, tune: &Knobs) -> f64 {
    let mut roof = [0.0; MAX_ROOFS * 4];
    for (to, r) in roof.chunks_exact_mut(4).zip(roofs) {
        to.copy_from_slice(r);
    }
    let s = Snow { top: tune.snowbog.top, bank: tune.snowbog.bank_m, packed: tune.snowbog.packed, old: tune.snowbog.old, dead: 0.0, grip: 0.0, stick: 0.0, roofs: roofs.len(), roof };
    bog_of(w, &s, x, y, z)
}
