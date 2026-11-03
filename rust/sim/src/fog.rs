//! THE FOG OF WAR'S SIGHT PASS — src/game/fogofwar.ts: which of the team's eyes has a line to a man (his chest, and his head behind a blocked chest) or to a
//! point (a body where it lies). The rule is the TypeScript's, statement for statement (inField, lineToMan, eyeOnBody, sightBlocked); what moves here is the
//! loop over the eyes and the lines it asks, so a man is one question to the module where he was two lines an eye.
//!
//! The eyes are written by the host as the pass opens (a row an eye: where it is, where it looks, and its field as the host worked it out) and are scratch
//! of that pass: nothing here is kept from one update to the next.
//!
//! The lengths are the game's own (maths.rs), so this answers only where the game's own maths stand in the host's `Math` (math/exact.ts EXACT.on); under an
//! engine's maths the host asks the TypeScript, whose lengths are that engine's.

use crate::lines;
use crate::maths::{hypot2, hypot3};
use crate::world::WORLDS;

/// eyes a pass can hold (the game's own cap is seven)
pub const MAX_EYES: usize = 15;
/// DOUBLES AN EYE: where it is (x y z), where it looks in plan (x z), that look's length (or 1), the cosine of half its field, its all-round near field, and
/// 1 if it sees all round
pub const EYE: usize = 9;
const E_POS: usize = 0;
const E_LOOK: usize = 3;
const E_LEN: usize = 5;
const E_COS: usize = 6;
const E_NEAR: usize = 7;
const E_FULL: usize = 8;

static mut EYES: [[f64; MAX_EYES * EYE]; WORLDS] = [[0.0; MAX_EYES * EYE]; WORLDS];

/// where world `w`'s eyes are written (a byte address)
pub fn eyes_addr(w: usize) -> usize {
    (unsafe { &(*core::ptr::addr_of!(EYES))[if w < WORLDS { w } else { 0 }] }).as_ptr() as usize
}

/// the answer when a line is none this module can walk (a length that is no length of this world's): the host asks the TypeScript
pub const ASK_HOST: u32 = 0xffff_ffff;

/// fogofwar.ts inField: is the point inside the eye's field (in plan)
#[inline]
fn in_field(e: &[f64], px: f64, pz: f64) -> bool {
    if e[E_FULL] != 0.0 {
        return true;
    }
    let (dx, dz) = (px - e[E_POS], pz - e[E_POS + 2]);
    let d = hypot2(dx, dz);
    if d < e[E_NEAR] || d < 1e-4 {
        return true;
    }
    (dx * e[E_LOOK] + dz * e[E_LOOK + 1]) / (d * e[E_LEN]) > e[E_COS]
}

/// fogofwar.ts sightBlocked (collision.ts segmentBlocked at 0.02, through `through`): Some(blocked), or None = not a line this module walks
#[inline]
fn sight_blocked(w: usize, set: i32, through: u32, e: &[f64], b0: f64, b1: f64, b2: f64) -> Option<bool> {
    let (a0, a1, a2) = (e[E_POS], e[E_POS + 1], e[E_POS + 2]);
    let len = hypot3(b0 - a0, b1 - a1, b2 - a2);
    if len < 1e-4 {
        return Some(false);
    }
    if !(len < 1e6) {
        return None;
    }
    Some(lines::blocked(w, a0, a1, a2, b0, b1, b2, len, 0.02, through, set, 0) == -1)
}

/// What a question is about
pub const KIND_MAN: u32 = 1;
pub const KIND_ALIVE: u32 = 2;

/// THE EYES WITH A LINE. `order` names `n` eyes four bits each, in the order to try them; the trying stops at the `want`-th eye with a line. A LIVING MAN
/// (KIND_MAN; KIND_ALIVE if he is alive): `a` his chest, `b` his head (lineToMan). Otherwise `a` is a point and `b` is not read (eyeOnBody). `through` must
/// carry the glass flag (the fog's own lines pass glass: nothing is listed). The answer: how many have a line in the top four bits, and those eyes four bits
/// each in the order found; or ASK_HOST.
#[allow(clippy::too_many_arguments)]
pub fn see(w: usize, set: i32, order: u32, n: u32, want: u32, kind: u32, through: u32, a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> u32 {
    let eyes = unsafe { &(*core::ptr::addr_of!(EYES))[if w < WORLDS { w } else { 0 }] };
    let n = if n > 7 { 7 } else { n };
    let (mut out, mut found) = (0u32, 0u32);
    if want == 0 {
        return 0;
    }
    for k in 0..n {
        let ei = ((order >> (4 * k)) & 15) as usize;
        if ei >= MAX_EYES {
            return ASK_HOST;
        }
        let e = &eyes[ei * EYE..ei * EYE + EYE];
        if !in_field(e, a0, a2) {
            continue;
        }
        let clear = if kind & KIND_MAN != 0 {
            // (he is under the eye itself)
            if hypot2(a0 - e[E_POS], a2 - e[E_POS + 2]) <= 1e-3 {
                true
            } else if kind & KIND_ALIVE == 0 {
                false
            } else {
                // the chest first, and the head only behind a blocked chest
                match sight_blocked(w, set, through, e, a0, a1, a2) {
                    None => return ASK_HOST,
                    Some(false) => true,
                    Some(true) => match sight_blocked(w, set, through, e, b0, b1, b2) {
                        None => return ASK_HOST,
                        Some(blocked) => !blocked,
                    },
                }
            }
        } else {
            match sight_blocked(w, set, through, e, a0, a1, a2) {
                None => return ASK_HOST,
                Some(blocked) => !blocked,
            }
        };
        if clear {
            out |= (ei as u32) << (4 * found);
            found += 1;
            if found >= want {
                break;
            }
        }
    }
    (found << 28) | out
}
