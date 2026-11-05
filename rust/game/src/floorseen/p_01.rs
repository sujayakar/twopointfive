//! src/game/floorseen.ts lines 1 to 43
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A new record: empty maps, frame 0; its one argument was the game.   (floorseen.ts FloorSeen.constructor)
pub fn floor_seen_new(g: &mut crate::game::Game) -> crate::floorseen::FloorSeen {
    crate::floorseen::FloorSeen::blank()
}

/// Once a frame: bring the staggered living men's record of seen floor up to date; `reach` in metres.   (floorseen.ts FloorSeen.tick)
pub fn floor_seen_tick(g: &mut crate::game::Game, men: &mut Vec<GuardId>, reach: f64) {
    let (cs, w, h, t) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h, g.time);
    let frame = {
        let s = g.door_tools.seen.as_mut().unwrap();
        s.frame = s.frame + 1.0;
        s.frame
    };
    for j in 0..men.len() {
        if js::rem(frame + (j as f64) * 3.0, EVERY) != 0.0 { continue; }
        let gd = men[j];
        if g.gd[gd].state == crate::game::GuardState::Dead || t < g.gd[gd].dazzled_until { continue; }
        {
            let s = g.door_tools.seen.as_mut().unwrap();
            if !s.at.has(gd) || !s.asked.has(gd) {
                let cells = (w * h) as usize;
                s.at.set(gd, vec![NEVER as f32; cells]);
                s.asked.set(gd, vec![NEVER as f32; cells]);
            }
        }
        let c = g.gd[gd].ch;
        let eye: V3 = match g.ch[c].bones.get(crate::character::Bone::Head) {
            Some(head) => *head,
            None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
        };
        let chest = g.ch[c].chest_dir;
        let look = v3::lerp3(chest, crate::character::character_head_dir(g, c), 0.5);
        let ll = js::or(m::hyp2(look[0], look[2]), 1.0);
        let (lx, lz) = (look[0] / ll, look[2] / ll);
        let (half_deg, near_deg, near_m) = (g.k.guard_eye.half_deg, g.k.guard_eye.near_deg, g.k.guard_eye.near_m);
        let cos_far = m::cos(half_deg * v3::DEG);
        let cos_near = m::cos(near_deg * v3::DEG);
        let n = m::ceil(reach / cs);
        let x0 = m::floor(eye[0] / cs);
        let z0 = m::floor(eye[2] / cs);
        // The man's two arrays are held out of the record for the walk: the line test takes `g`, and nothing it reaches reads this record.
        let (mut a, mut q) = {
            let s = g.door_tools.seen.as_mut().unwrap();
            (core::mem::take(s.at.get_mut(gd).unwrap()), core::mem::take(s.asked.get_mut(gd).unwrap()))
        };
        let (x_lo, x_hi) = (m::max(0.0, x0 - n), m::min(w - 1.0, x0 + n));
        let z_hi = m::min(h - 1.0, z0 + n);
        let mut z = m::max(0.0, z0 - n);
        while z <= z_hi {
            let cell_z = z;
            z += 1.0;
            let mut x = x_lo;
            while x <= x_hi {
                let cell_x = x;
                x += 1.0;
                let i = crate::scene_collision::nav_grid_idx(g, cell_x, cell_z) as usize;
                if g.col.nav.blocked[i] == 1 || t - (q[i] as f64) < REFRESH { continue; }
                let px = (cell_x + 0.5) * cs;
                let pz = (cell_z + 0.5) * cs;
                let dx = px - eye[0];
                let dz = pz - eye[2];
                let dd = m::hyp2(dx, dz);
                if dd > reach { continue; }
                // The floor he stands on is seen; past that, the wedge.
                if dd > 0.6 && (dx * lx + dz * lz) / dd < (if dd < near_m { cos_near } else { cos_far }) { continue; }
                q[i] = t as f32;
                if !crate::scene_collision::segment_blocked(g, eye, [px, 1.0, pz], None, None, None) { a[i] = t as f32; }
            }
        }
        let s = g.door_tools.seen.as_mut().unwrap();
        *s.at.get_mut(gd).unwrap() = a;
        *s.asked.get_mut(gd).unwrap() = q;
    }
}

/// How many of `cells` (nav cell indices) `gd` has seen since game time `since`.   (floorseen.ts FloorSeen.count)
pub fn floor_seen_count(g: &mut crate::game::Game, gd: GuardId, cells: &[f64], since: f64) -> f64 {
    let s = g.door_tools.seen.as_ref().unwrap();
    let Some(a) = s.at.get(gd) else { return 0.0 };
    let mut n = 0.0;
    for &i in cells {
        // An index outside the array reads undefined in the TypeScript, and that is never `>= since`.
        if let Some(&seen) = a.get(i as usize) {
            if (seen as f64) >= since { n += 1.0; }
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A store is a single: the rounded value decides `since`, as a Float32Array does (bits from bun: Math.fround).
    #[test]
    fn count_reads_the_rounded_single() {
        assert_eq!((0.1f64 as f32 as f64).to_bits(), 0x3FB99999A0000000);
        assert_eq!((0.7f64 as f32 as f64).to_bits(), 0x3FE6666660000000);
        let mut g = Game::blank();
        let gd = GuardId(0);
        let mut seen = floor_seen_new(&mut g);
        seen.at.set(gd, vec![0.1f64 as f32, 0.7f64 as f32, NEVER as f32]);
        g.door_tools.seen = Some(seen);
        // 0.1 rounds up and counts at 0.1; 0.7 rounds down and does not count at 0.7; cell 2 was never seen; cell 9 is past the array.
        assert_eq!(floor_seen_count(&mut g, gd, &[0.0, 1.0, 2.0, 9.0], 0.1), 2.0);
        assert_eq!(floor_seen_count(&mut g, gd, &[0.0, 1.0, 2.0, 9.0], 0.7), 0.0);
        // A man with no record has seen nothing.
        assert_eq!(floor_seen_count(&mut g, GuardId(1), &[0.0], -2e9), 0.0);
    }
}
