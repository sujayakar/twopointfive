//! HOW A SOUND REACHES A LISTENER — src/scene/collision.ts SoundPaths.propagate: straight where nothing solid stands between them (a jamb grazing a side ray
//! dulls it a touch), over the top of what is only chest high, and otherwise round by the floor (nav.rs: the listener's way to the source), measured leg by
//! leg, with the last waypoint the listener still has a straight run to and whether a shut leaf lies across the way. The rule is the TypeScript's, statement
//! for statement. It is asked in two halves because the TypeScript keeps a short memory of routed answers between them, and that memory stays the host's.
//!
//! Sound does not pass glass. Where the host has a rule for an open pane, a line that meets glass and nothing solid is the host's to judge, and so is the
//! whole question then. The lengths are the game's own, so this answers only where EXACT.on.

use crate::lines;
use crate::maths::{hypot2, hypot3};
use crate::nav;
use crate::world::{ASK_HOST, js_max, or_one};

/// the first half's answers: a clear line (+ GRAZE_A / GRAZE_B for each side ray that is not) · clear over the top · walled off and too far for a search ·
/// walled off (ask the second half)
pub const DIRECT: u32 = 0;
pub const GRAZE_A: u32 = 1;
pub const GRAZE_B: u32 = 2;
pub const OVER: u32 = 4;
pub const FAR: u32 = 8;
pub const WALLED: u32 = 16;
/// the second half's: no way round (sealed) · a way (its length, the point it comes through and whether a leaf lies across it are in `out`)
pub const SEALED: u32 = 0;
pub const ROUTED: u32 = 1;
pub const ROUTED_DOOR: u32 = 3;

/// the first half. The straight distance goes to `straight`.
#[allow(clippy::too_many_arguments)]
pub fn direct(w: usize, set: i32, ask: u32, s: [f64; 3], d: [f64; 3], straight: &mut f64) -> u32 {
    let a = [s[0], js_max(s[1], 0.9), s[2]];
    let b = [d[0], js_max(d[1], 0.9), d[2]];
    let st = hypot3(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
    *straight = st;
    match lines::solid_line(w, set, ask, a[0], a[1], a[2], b[0], b[1], b[2]) {
        None => return ASK_HOST,
        Some(false) => {
            let mut r = DIRECT;
            if st > 1.0 {
                let dd = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
                // v3.normalize([-d[2], 0, d[0]])
                let l = or_one(hypot3(-dd[2], 0.0, dd[0]));
                let side = [-dd[2] / l, 0.0 / l, dd[0] / l];
                match lines::solid_line(w, set, ask, a[0] + side[0] * 0.5, a[1] + side[1] * 0.5, a[2] + side[2] * 0.5, b[0], b[1], b[2]) {
                    None => return ASK_HOST,
                    Some(true) => r |= GRAZE_A,
                    Some(false) => {}
                }
                match lines::solid_line(w, set, ask, a[0] + side[0] * -0.5, a[1] + side[1] * -0.5, a[2] + side[2] * -0.5, b[0], b[1], b[2]) {
                    None => return ASK_HOST,
                    Some(true) => r |= GRAZE_B,
                    Some(false) => {}
                }
            }
            return r;
        }
        Some(true) => {}
    }
    match lines::solid_line(w, set, ask, s[0], 2.0, s[2], d[0], 2.0, d[2]) {
        None => return ASK_HOST,
        Some(false) => return OVER,
        Some(true) => {}
    }
    if st > 28.0 {
        return FAR;
    }
    WALLED
}

/// the second half: the way round. `seal` = the flag of a leaf that shuts sound out. `out`: the path's length, then the point it comes through (x y z).
#[allow(clippy::too_many_arguments)]
pub fn route(w: usize, set: i32, ask: u32, seal: u32, gw: usize, gh: usize, cell: f64, s: [f64; 3], d: [f64; 3], straight: f64, out: &mut [f64]) -> u32 {
    let n = nav::find_path(w, set, ask, gw, gh, cell, d[0], d[2], s[0], s[2], 6000);
    if n == nav::ASK_HOST {
        return ASK_HOST;
    }
    if n <= 0 {
        return SEALED;
    }
    let way = unsafe { &*(nav::way_addr() as *const [f64; nav::MAX_WAY * 2]) };
    let b = [d[0], js_max(d[1], 0.9), d[2]];
    let mut len = 0.0;
    let (mut prev0, mut prev2) = (d[0], d[2]);
    let mut via = [way[0], 1.0, way[1]];
    let mut door = false;
    let mut pa = [d[0], 1.0, d[2]];
    let mut lost = false;
    for i in 0..n as usize {
        let (q0, q2) = (way[i * 2], way[i * 2 + 1]);
        len += hypot2(q0 - prev0, q2 - prev2);
        prev0 = q0;
        prev2 = q2;
        let pb = [q0, 1.0, q2];
        if !lost {
            match lines::solid_line(w, set, ask, b[0], b[1], b[2], pb[0], pb[1], pb[2]) {
                None => return ASK_HOST,
                Some(false) => via = pb,
                Some(true) => lost = true,
            }
        }
        if !door {
            match lines::leaf_line(w, set, seal, pa[0], pa[1], pa[2], pb[0], pb[1], pb[2]) {
                None => return ASK_HOST,
                Some(hit) => door = hit,
            }
        }
        pa = pb;
    }
    let last = hypot2(s[0] - prev0, s[2] - prev2);
    len += last;
    len = js_max(len, straight);
    let s1 = [s[0], 1.0, s[2]];
    let end = if last > 0.3 {
        let v = [pa[0] - s1[0], pa[1] - s1[1], pa[2] - s1[2]];
        let l = or_one(hypot3(v[0], v[1], v[2]));
        [s1[0] + v[0] / l * 0.25, s1[1] + v[1] / l * 0.25, s1[2] + v[2] / l * 0.25]
    } else {
        pa
    };
    if !door {
        match lines::leaf_line(w, set, seal, pa[0], pa[1], pa[2], end[0], end[1], end[2]) {
            None => return ASK_HOST,
            Some(hit) => door = hit,
        }
    }
    out[0] = len;
    out[1] = via[0];
    out[2] = via[1];
    out[3] = via[2];
    if door {
        ROUTED_DOOR
    } else {
        ROUTED
    }
}
