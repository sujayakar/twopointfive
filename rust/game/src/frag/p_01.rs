//! src/game/frag.ts lines 1 to 296
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// QA: the burst log, the reasons and the counters back to nothing. (frag.ts fragStatsReset)
pub fn frag_stats_reset(g: &mut crate::game::Game) {
    g.m.frag.frag_log.clear();
    g.m.frag.frag_why.clear();
    g.stats.frag = crate::frag::FragStats::default();
}

/// The burst's one roll: a hash of its serial and the man's id, 0‥1. (frag.ts hash01)
pub fn hash01(a: f64, b: f64) -> f64 {
    let mut x = (m::imul(js::to_i32(a), 0x9E3779B1u32 as i32) ^ m::imul(js::to_i32(b).wrapping_add(0x7F4A7C15), 0x85EBCA6Bu32 as i32)) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, 0xC2B2AE35u32 as i32) as u32;
    x ^= x >> 13;
    (x >> 8) as f64 / 16777216.0
}

/// `fn_` asked with glass as `view` reckons it for a burst at `from`. (frag.ts throughGlass)
pub fn through_glass<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, from: V3, view: crate::frag::GlassView, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let see = match view {
        GlassView::Whole => return fn_(g),
        GlassView::Blown => crate::glassbreak::GlassBreakSeeThroughView::Blown,
        GlassView::Wary => crate::glassbreak::GlassBreakSeeThroughView::Wary,
        GlassView::Counted => crate::glassbreak::GlassBreakSeeThroughView::Counted,
    };
    if !crate::glassbreak::glass_break_live(g) {
        return fn_(g);
    }
    let size = crate::glassbreak::GlassBlastOpts { radius: Some(g.k.frag.glass_m), power: Some(g.k.frag.glass_power), who: None };
    crate::glassbreak::glass_break_see_through(g, see, from, size, fn_)
}

/// Is a burst's line from a to b stopped: by anything solid, and by glass as glass is to a burst. (frag.ts burstBlocked)
pub fn burst_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    if !crate::glassbreak::glass_break_live(g) {
        return crate::scene_collision::shot_blocked(g, a, b, None, None);
    }
    if g.k.frag.pane_energy > 0.0 {
        let energy = g.k.frag.pane_energy;
        crate::scene_collision::shot_blocked(g, a, b, Some(0.02), Some(energy))
    } else {
        crate::scene_collision::solid_blocked(g, a, b, Some(0.02))
    }
}

/// Has a burst at `from` a line to `to`, glass by `view` (wary if not said). (frag.ts burstLine)
pub fn burst_line(g: &mut crate::game::Game, from: V3, to: V3, view: Option<crate::frag::GlassView>) -> bool {
    let view = view.unwrap_or(GlassView::Wary);
    through_glass(g, from, view, &mut |g: &mut Game| !burst_blocked(g, from, to))
}

/// Share (0‥1) of a man the burst point has a clear line to: four points from shin to head. (frag.ts exposure)
pub fn exposure(g: &mut crate::game::Game, from: V3, feet: V3, head: V3, view: Option<crate::frag::GlassView>) -> f64 {
    let view = view.unwrap_or(GlassView::Wary);
    let h = m::max(0.5, head[1] - feet[1]);
    let pts: [V3; 4] = [
        [feet[0], feet[1] + 0.2 * h, feet[2]],
        [feet[0], feet[1] + 0.55 * h, feet[2]],
        [v3::lerp(feet[0], head[0], 0.8), feet[1] + 0.78 * h, v3::lerp(feet[2], head[2], 0.8)],
        [head[0], head[1], head[2]],
    ];
    through_glass(g, from, view, &mut |g: &mut Game| share_clear(g, from, &pts))
}

/// The closure `exposure` and `lyingShows` both hand the glass: the share of `pts` with a clear line. (frag.ts:241, 295)
fn share_clear(g: &mut Game, from: V3, pts: &[V3]) -> f64 {
    let mut n = 0.0;
    for &p in pts {
        if !burst_blocked(g, from, p) {
            n += 1.0;
        }
    }
    n / pts.len() as f64
}

/// A question asked with these door leaves out of the collision layer's list, the list put back after. (frag.ts withoutLeaves)
pub fn without_leaves<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, leaves: &[BoxId], fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    if leaves.is_empty() {
        return fn_(g);
    }
    let dyn_ = g.col.dynamic_boxes.clone();
    g.col.dynamic_boxes.retain(|b| !leaves.contains(b));
    let r = fn_(g);
    g.col.dynamic_boxes = dyn_;
    r
}

/// The leaves a hand grenade bursting at `from` would take off its hinges. (frag.ts leavesFor)
pub fn leaves_for(g: &mut crate::game::Game, from: V3) -> Vec<BoxId> {
    let opts = crate::destruction::DestructionDoorsForOpts { radius: Some(g.k.frag.destr_m), power: Some(1.0) };
    let doors = crate::destruction::destruction_doors_for(g, from, Some(opts));
    doors.into_iter().map(|d| g.dr[d].box_).collect()
}

/// exposure() as a man reckons his cover from a grenade that has not gone off: what the burst would take is no cover. (frag.ts exposedTo)
pub fn exposed_to(g: &mut crate::game::Game, from: V3, feet: V3, head: V3, view: Option<crate::frag::GlassView>) -> f64 {
    let e = exposure(g, from, feet, head, view);
    if e > 0.0 {
        return e;
    }
    let walls = walls_for(g, from);
    let leaves = leaves_for(g, from);
    if !walls.is_empty() {
        return crate::destruction::destruction_with_walls_open(g, &walls, &mut |g: &mut Game| {
            without_leaves(g, &leaves, &mut |g: &mut Game| exposure(g, from, feet, head, view))
        });
    }
    if !leaves.is_empty() {
        without_leaves(g, &leaves, &mut |g: &mut Game| exposure(g, from, feet, head, view))
    } else {
        e
    }
}

/// The walls and cover a hand grenade bursting at `from` would open; the plan asked once a tick for a point. (frag.ts wallsFor)
pub fn walls_for(g: &mut crate::game::Game, from: V3) -> Vec<crate::wallbreak::WallOpening> {
    let serial = g.destr.walls.serial;
    let time = g.time;
    let keep = match &g.m.frag.walls_for {
        Some(c) => c.time == time && c.serial == serial,
        None => false,
    };
    if !keep {
        g.m.frag.walls_for = Some(crate::frag::FragModWALLSFORValue { time, serial, at: JsMap::new() });
    }
    // (`+ 0.0`: the language's own text of −0 is "0", and this text is only ever a key)
    let key = alloc::format!("{},{},{}", from[0] + 0.0, from[1] + 0.0, from[2] + 0.0);
    if let Some(c) = &mut g.m.frag.walls_for {
        if let Some(w) = c.at.get(key.clone()) {
            return w.clone();
        }
        if c.at.len() >= 64 {
            c.at.clear();
        }
    }
    let opts = crate::destruction::DestructionWallsForOpts { radius: Some(g.k.frag.destr_m), power: Some(1.0) };
    let w = crate::destruction::destruction_walls_for(g, from, Some(opts));
    if let Some(c) = &mut g.m.frag.walls_for {
        c.at.set(key, w.clone());
    }
    w
}

/// Distance on the floor. (frag.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// One side: the same team, or the player and an operative. (frag.ts sameSide)
pub fn same_side(a: crate::teams::Team, b: crate::teams::Team) -> bool {
    use crate::teams::Team;
    a == b || (a == Team::Player && b == Team::Operative) || (a == Team::Operative && b == Team::Player)
}

/// A man a burst can still kill where he lies: down and breathing, the player's enemy, never a civilian. (frag.ts downAsked)
pub fn down_asked(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !crate::game::is_breathing(g, gd) || g.gd[gd].god || g.gd[gd].civ.is_some() {
        return false;
    }
    let team = g.gd[gd].team;
    crate::teams::is_hostile(g, crate::teams::Team::Player, team)
}

/// The points of a lying man a burst is asked for: hips, chest, head, a shin, none under the floor's skin. (frag.ts lying)
pub fn lying(g: &mut crate::game::Game, gd: GuardId) -> Vec<V3> {
    use crate::character::Bone;
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let b = &g.ch[c].bones;
    let hips = lying_up(b.get(Bone::Hips).copied().unwrap_or(pos));
    vec![
        hips,
        lying_up(b.get(Bone::Spine2).copied().unwrap_or(hips)),
        lying_up(b.get(Bone::Head).copied().unwrap_or(hips)),
        lying_up(b.get(Bone::ShinL).copied().unwrap_or(hips)),
    ]
}

/// (frag.ts:291 `up`)
fn lying_up(p: V3) -> V3 {
    [p[0], m::max(p[1], 0.12), p[2]]
}

/// How much of those points a burst at `from` has a line to (0‥1), through the glass as `view` reckons it. (frag.ts lyingShows)
pub fn lying_shows(g: &mut crate::game::Game, from: V3, pts: &[V3], view: crate::frag::GlassView) -> f64 {
    through_glass(g, from, view, &mut |g: &mut Game| share_clear(g, from, pts))
}

/// THE BLAST: its sparks and dust are its thrower's doing. (frag.ts blast)
pub fn blast(g: &mut crate::game::Game, at: V3, spec: &crate::frag::BlastSpec) -> crate::frag::BlastReport {
    let was = crate::fogofwar::fog_of_war_begin(g, Some(spec.who));
    let r = blast_raw(g, at, spec);
    crate::fogofwar::fog_of_war_done(g, was);
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    // bits from the TypeScript under bun with EXACT.on
    #[test]
    fn hash01_has_the_typescript_s_bits() {
        let rows: [(f64, f64, u64); 9] = [
            (0.0, 0.0, 0x3fe2255ea0000000),
            (1.0, 2.0, 0x3fea137a20000000),
            (12345.0, 7.0, 0x3fd3933f80000000),
            (-1.0, -1.0, 0x3fc89cf180000000),
            (2147483647.0, 2147483647.0, 0x3fc89cf180000000),
            (3000000000.7, -5.9, 0x3fbd80b900000000),
            (1234567.5, 99.0, 0x3fea623280000000),
            (f64::NAN, 4.0, 0x3fd898a300000000),
            (4242000.0, 17.0, 0x3fc9db4c80000000),
        ];
        for (a, b, bits) in rows {
            assert_eq!(hash01(a, b).to_bits(), bits, "{a} {b}");
        }
    }
    #[test]
    fn without_leaves_filters_the_list_and_puts_it_back() {
        let mut g = Game::blank();
        let all = vec![BoxId(3), BoxId(1), BoxId(7), BoxId(1)];
        g.col.dynamic_boxes = all.clone();
        // every copy of a leaf goes, the rest keep their order; a nested view sees the outer one's list
        let (once, twice) = without_leaves(&mut g, &[BoxId(1)], &mut |g: &mut Game| {
            let once = g.col.dynamic_boxes.clone();
            let twice = without_leaves(g, &[BoxId(7)], &mut |g: &mut Game| g.col.dynamic_boxes.clone());
            (once, twice)
        });
        assert_eq!(once, vec![BoxId(3), BoxId(7)]);
        assert_eq!(twice, vec![BoxId(3)]);
        assert_eq!(g.col.dynamic_boxes, all);
        // what the question itself left in the list is not kept: the put-back is of what was found
        without_leaves(&mut g, &[BoxId(3)], &mut |g: &mut Game| g.col.dynamic_boxes.push(BoxId(9)));
        assert_eq!(g.col.dynamic_boxes, all);
        // no leaves: asked as it stands
        assert_eq!(without_leaves(&mut g, &[], &mut |g: &mut Game| g.col.dynamic_boxes.clone()), all);
    }
    #[test]
    fn flat_has_the_typescript_s_bits() {
        assert_eq!(flat([0.1, 5.0, 0.2], [3.3, 9.0, -4.7]).to_bits(), 0x401768ce6d3c11e0);
        assert_eq!(flat([1e-3, 0.0, 7.25], [1e-3, 0.0, 7.25]).to_bits(), 0);
        assert_eq!(flat([-12.75, 1.0, 33.125], [8.5, 2.0, -0.0625]).to_bits(), 0x4043b431f0bad27a);
    }
}
