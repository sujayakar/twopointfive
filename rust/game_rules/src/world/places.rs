//! A PLACE TO STAND, AND THE WALK THERE: what any rule asks of floor before it sends a man to it, said here once. What suits a rule itself (hidden from whom, in
//! sight of what) is its own business; these are the same for all of them.
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::vec::Vec;

/// The metres of a walk from `from` by its corners.
pub fn walk_len(from: V3, path: &[V3]) -> f64 {
    let (mut l, mut q) = (0.0, from);
    for &w in path {
        l += v3::dist_xz(q, w);
        q = w;
    }
    l
}

/// The metres on foot from `a` to `b`, or to the free floor nearest it; infinite with no way there.
pub fn on_foot(g: &Game, a: V3, b: V3) -> f64 {
    crate::scene_collision::nav_grid_find_path(g, a, b, None, None).map_or(m::INF, |path| walk_len(a, &path))
}

/// a walk: its corners, its metres, where it ends
pub struct Walk {
    pub path: Vec<V3>,
    pub l: f64,
    pub end: V3,
}

/// His walk from `from` to `p`, if there is one and it ends within `near` of it.
pub fn walk(g: &Game, from: V3, p: V3, near: f64) -> Option<Walk> {
    let path = crate::scene_collision::nav_grid_find_path(g, from, p, None, None)?;
    let end = *path.last()?;
    if v3::dist_xz(end, p) > near { return None; }
    Some(Walk { l: walk_len(from, &path), path, end })
}

/// Room for a man's shoulders at `p`: the cell and the four beside it are free floor.
pub fn roomy(g: &Game, p: V3) -> bool {
    let cs = g.col.nav.cell;
    [[0.0, 0.0], [cs, 0.0], [-cs, 0.0], [0.0, cs], [0.0, -cs]].iter().all(|o| !crate::scene_collision::nav_grid_is_blocked(g, p[0] + o[0], p[2] + o[1]))
}

/// SOMEBODY HAS IT: within `apart` of `p` another man stands, or is on his way there, or means to, whatever rule or order sent him; or a player does.
pub fn taken(g: &mut Game, gd: GuardId, p: V3, apart: f64) -> bool {
    let at = |q: Option<V3>| q.map_or(false, |q| v3::dist_xz(q, p) < apart);
    if crate::seats::men_up(g).into_iter().any(|pl| at(Some(g.player_pos(pl)))) { return true; }
    for o in g.guards.clone() {
        if o == gd || crate::condition::down(g, g.gd[o].ch) { continue; }
        let m = &g.gd[o];
        if at(Some(g.ch[m.ch].pos))
            || at(m.script.as_ref().and_then(|s| s.goal))
            || (m.path_i < m.path.len() && at(m.path_goal))
            || at(g.cond[m.ch].sup.cover)
            || at(m.body_duty.and_then(|d| d.spot))
            || at(m.standoff.and_then(|s| g.tab.guards_standoff[s].orbit_goal)) { return true; }
        if g.gd[o].unit.is_some() && crate::seats::order_of(g, o).is_some_and(|a| at(g.tab.orders_assignment[a].post)) { return true; }
    }
    false
}
