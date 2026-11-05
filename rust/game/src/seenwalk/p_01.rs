//! src/game/seenwalk.ts lines 1 to 91
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A number as the stamp's text. Only ever compared, so all it must keep is JavaScript's classes of equal text: -0 reads as 0.
fn stamp_num(x: f64) -> String {
    alloc::format!("{}", if x == 0.0 { 0.0 } else { x })
}

/// The memo's key of a block: two nav cells each way.   (seenwalk.ts seenPrice)
fn block_of(ix: f64, iz: f64) -> (f64, f64, f64) {
    let (bx, bz) = ((js::to_i32(ix) >> 1) as f64, (js::to_i32(iz) >> 1) as f64);
    (bx, bz, bz * 8192.0 + bx)
}

/// What the game holds of this tick's floor, thrown away with the clock, a room, the floor, a door or a number changed. A copy: the memo is written through `g.m.seenwalk.shared`.   (seenwalk.ts sharedOf)
pub fn shared_of(g: &mut crate::game::Game) -> crate::seenwalk::Shared {
    let mut doors = String::new();
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        doors.push_str(&stamp_num(crate::doors::door_angle(g, d)));
        if crate::doors::door_latched(g, d) { doors.push('l'); }
        if crate::doors::door_locked(g, d) { doors.push('k'); }
        if crate::doors::door_off(g, d) { doors.push('o'); }
        doors.push(',');
        i += 1;
    }
    let stamp = alloc::format!(
        "{}|{}|{}|{}|{}|{}",
        doors,
        stamp_num(g.engine.world.geom_serial),
        stamp_num(g.k.seenwalk.lit_at),
        stamp_num(g.k.seenwalk.dear),
        stamp_num(g.k.seenwalk.eye_step),
        stamp_num(g.k.seenwalk.eye_y),
    );
    let stale = match &g.m.seenwalk.shared {
        None => true,
        Some(s) => s.t != g.time || s.known != g.cleared.known || s.stamp != stamp,
    };
    if stale {
        // (known is read before the eyes: unknownEyes can rebuild the rooms, and that sets it back to 0)
        let (t, known, step) = (g.time, g.cleared.known, g.k.seenwalk.eye_step);
        let eyes = crate::cleared::cleared_rooms_unknown_eyes(g, step)
            .iter()
            .map(|e| SharedEyesItem { x: e.x, z: e.z })
            .collect();
        g.m.seenwalk.shared = Some(Shared { t, known, stamp, eyes, memo: JsMap::new() });
    }
    g.m.seenwalk.shared.clone().unwrap()
}

/// Nothing kept: a restart, a checkpoint loaded.   (seenwalk.ts seenReset)
pub fn seen_reset(g: &mut crate::game::Game) {
    g.m.seenwalk.shared = None;
}

/// The price of the floor to a walk from `from` by `base`: `dear` on lit floor in view of an uncleared room, 1 elsewhere; None when no uncleared room is within an eye's reach of that walk.   (seenwalk.ts seenPrice)
pub fn seen_price(g: &mut crate::game::Game, from: V3, base: &[V3]) -> Option<crate::seenwalk::SeenPrice> {
    let s = shared_of(g);
    g.stats.seenwalk.asks += 1;
    if s.eyes.is_empty() { return None; }
    let (mut x0, mut x1, mut z0, mut z1) = (from[0], from[0], from[2], from[2]);
    for b in base {
        x0 = m::min(x0, b[0]);
        x1 = m::max(x1, b[0]);
        z0 = m::min(z0, b[2]);
        z1 = m::max(z1, b[2]);
    }
    let margin = g.k.seenwalk.margin;
    x0 -= margin;
    x1 += margin;
    z0 -= margin;
    z1 += margin;
    let far = crate::guards::eye_reach(g, 1.0);
    let eyes: Vec<SharedEyesItem> = s
        .eyes
        .iter()
        .filter(|e| e.x >= x0 - far && e.x <= x1 + far && e.z >= z0 - far && e.z <= z1 + far)
        .copied()
        .collect();
    if eyes.is_empty() { return None; }
    Some(SeenPrice::Floor { x0, x1, z0, z1, eyes })
}

/// What a price `seen_price` handed back asks of a cell.   (seenwalk.ts seenPrice, the returned closure)
fn seen_price_at(g: &mut Game, price: &SeenPrice, ix: f64, iz: f64) -> f64 {
    let SeenPrice::Floor { x0, x1, z0, z1, eyes } = price;
    let cs = g.col.nav.cell;
    let (x, z) = ((ix + 0.5) * cs, (iz + 0.5) * cs);
    if x < *x0 || x > *x1 || z < *z0 || z > *z1 { return g.k.seenwalk.dear; }   // (unread floor is not safe floor)
    let (bx, bz, key) = block_of(ix, iz);
    if let Some(v) = g.m.seenwalk.shared.as_ref().and_then(|s| s.memo.get(crate::Num(key)).copied()) {
        return v;
    }
    let mut v = 1.0;
    g.stats.seenwalk.blocks += 1;
    // (the block's own point: its first free cell in a fixed order, so that its price does not hang on which cell was asked first)
    let (mut px, mut pz) = (f64::NAN, f64::NAN);
    let mut k: i32 = 0;
    while k < 4 && px.is_nan() {
        let (cx, cz) = (bx * 2.0 + (k & 1) as f64, bz * 2.0 + (k >> 1) as f64);
        if cx < g.col.nav.w && cz < g.col.nav.h {
            // (off the grid's low edge the TypeScript reads undefined from the typed array, which is not 1)
            let at = crate::scene_collision::nav_grid_idx(g, cx, cz);
            let cell = if at >= 0.0 { g.col.nav.blocked.get(at as usize).copied() } else { None };
            if cell != Some(1) {
                px = (cx + 0.5) * cs;
                pz = (cz + 0.5) * cs;
            }
        }
        k += 1;
    }
    if !px.is_nan() && crate::cleared::cleared_rooms_known_at(g, [px, 0.0, pz]) {   // (floor inside an unknown room is cleared.ts's own price)
        let lit = crate::calmtorch::place_light(g, px, pz);
        if lit >= g.k.seenwalk.lit_at {
            let (reach, head): (f64, V3) = (crate::guards::eye_reach(g, lit), [px, 1.5, pz]);
            for e in eyes {
                if m::hyp2(e.x - px, e.z - pz) > reach { continue; }
                g.stats.seenwalk.lines += 1;
                let eye_y = g.k.seenwalk.eye_y;
                if !crate::scene_collision::segment_blocked(g, [e.x, eye_y, e.z], head, None, None, None) {
                    v = g.k.seenwalk.dear;
                    break;
                }
            }
        }
    }
    if let Some(s) = g.m.seenwalk.shared.as_mut() {
        s.memo.set(crate::Num(key), v);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stamp_reads_minus_zero_as_javascript_does() {
        assert_eq!(stamp_num(-0.0), stamp_num(0.0));
        assert_ne!(stamp_num(0.1), stamp_num(0.1 + 1e-16));
    }

    #[test]
    fn a_block_key_shifts_to_the_low_side() {
        assert_eq!(block_of(5.0, 3.0), (2.0, 1.0, 8194.0));
        assert_eq!(block_of(-1.0, 0.0), (-1.0, 0.0, -1.0));
        assert_eq!(block_of(0.0, -1.0), (0.0, -1.0, -8192.0));
    }
}
