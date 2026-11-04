//! AN EYE ON A MAN — src/game/guards.ts eyeContact: is his chest inside the eye's cone and reach (in plan; and in elevation where the eye is a lens), and is
//! there then a line from the eye to his chest or to his head. The rule is the TypeScript's, statement for statement; what moves here is the whole question,
//! so a pair of men is one call to the module where it was a cone's arithmetic and two lines.
//!
//! The lengths and angles are the game's own (maths.rs), so this answers only where the game's own maths stand in the host's `Math` (math/exact.ts EXACT.on).
//! A line that stops at glass may have a pane to ask the host about: an eye's line always passes glass, and the host sends no other here.

use crate::lines;
use crate::maths::{asin, cos, hypot3};
use crate::world::{ASK_HOST, clamp, js_abs, js_max, or_one, DEG};


/// what the host says of the question: the eye is a lens (it has a half-angle in elevation) · nothing is seen whatever the lines say (the target is hidden,
/// down or dead, or the eye is out) · the smoke between them hides him (the host marched it)
pub const Q_LENS: u32 = 1;
pub const Q_UNSEEN: u32 = 2;
pub const Q_SMOKED: u32 = 4;
/// the answer's bits
pub const A_VISIBLE: u32 = 1;
pub const A_IN_CONE: u32 = 2;

/// eyeContact. `e` the eye, `l` the unit direction it looks along, `c` his chest, `h` his head. The distance goes to `dist`.
#[allow(clippy::too_many_arguments)]
pub fn contact(w: usize, set: i32, through: u32, q: u32, e: [f64; 3], l: [f64; 3], range: f64, half_deg: f64, near_m: f64, near_deg: f64, v_half_deg: f64, c: [f64; 3], h: [f64; 3], dist: &mut f64) -> u32 {
    let to = [c[0] - e[0], c[1] - e[1], c[2] - e[2]];
    let d = hypot3(to[0], to[1], to[2]);
    *dist = d;
    let k = 1.0 / js_max(d, 1e-4);
    let dir = [to[0] * k, to[1] * k, to[2] * k];
    // v3.dot([look[0], 0, look[2]], v3.normalize([dirTo[0], 0, dirTo[2]]))
    let n = or_one(hypot3(dir[0], 0.0, dir[2]));
    let cos_a = l[0] * (dir[0] / n) + 0.0 * (0.0 / n) + l[2] * (dir[2] / n);
    let fov = if d < near_m { cos(near_deg * DEG) } else { cos(half_deg * DEG) };
    let mut in_cone = cos_a > fov && d < range;
    if in_cone && q & Q_LENS != 0 {
        in_cone = js_abs(asin(clamp(dir[1], -1.0, 1.0)) - asin(clamp(l[1], -1.0, 1.0))) < v_half_deg * DEG;
    }
    if !in_cone {
        return 0;
    }
    if q & Q_UNSEEN != 0 {
        return A_IN_CONE;
    }
    // (segmentBlocked(eye, chest) && segmentBlocked(eye, head)) || smoked: the head is asked only behind a blocked chest, the smoke was marched before either
    let blocked = match lines::man_hidden(w, set, through, e[0], e[1], e[2], c[0], c[1], c[2], h[0], h[1], h[2]) {
        None => return ASK_HOST,
        Some(b) => b,
    };
    if blocked || q & Q_SMOKED != 0 {
        A_IN_CONE
    } else {
        A_IN_CONE | A_VISIBLE
    }
}
