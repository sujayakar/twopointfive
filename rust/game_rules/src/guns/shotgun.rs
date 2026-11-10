pub use ::game_types::shotgun::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec, vec::Vec};

/// A pellet has found a man `t` metres from the muzzle: does it put him down? Booked either way.
pub fn pellet_drops(g: &mut crate::game::Game, s: &mut crate::shotgun::Shell, c: CharId, t: f64, dir: V3) -> bool {
    let p = s.def.pellets.unwrap();
    let h = s.men.get_or_insert_with(c, || crate::shotgun::ShellMenValue { n: 0.0, near: t, dir, down: false });
    h.n += 1.0;
    h.near = m::min(h.near, t);
    g.stats.shotgun.in_men += 1;
    if !h.down && (t <= p.drop_m || h.n >= p.need) {
        h.down = true;
        g.stats.shotgun.dropped += 1;
        return true;
    }
    false
}

/// Where a pellet stopped, if it was not in a man.
pub(super) fn pellet_end(s: &mut crate::shotgun::Shell, index: f64, t: f64, at: V3) {
    s.ends.push(crate::shotgun::ShellEndsItem { index, t, at });
}

/// One pull of a shotgun's trigger: every pellet through fireWeapon, then what the pattern as a whole did.
pub(super) fn fire_shell(g: &mut crate::game::Game, shooter: CharId, target: V3, o: &crate::combat::FireOpts, def: &crate::weapons::WeaponDef) {
    let p = def.pellets.unwrap();
    let mut shell = crate::shotgun::Shell { def: def.clone(), i: 0.0, dir: None, men: JsMap::new(), ends: Vec::new(), glass: JsMap::new(), late: Vec::new() };
    g.stats.shotgun.shells += 1;
    g.stats.shotgun.pellets += p.n as i32;
    let muzzle = v3::copy(g.ch[shooter].muzzle);
    let first = crate::combat::FireOpts { spread: o.spread, weapon: Some(def.clone()), steady: o.steady, blind: o.blind };
    crate::combat::fire_pellet(g, shooter, target, &first, &mut shell);
    let d0 = match shell.dir { Some(d) => d, None => v3::normalize(v3::sub(target, muzzle)) };
    let ax = v3::normalize(v3::cross(d0, if m::abs(d0[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] }));
    let ay = v3::cross(ax, d0);
    let mut i = 1.0;
    while i < p.n {
        shell.i = i;
        let r = p.cone * m::sqrt(crate::dice::draw_body(g, "shotgun::fire_shell#1", shooter));
        let a = crate::dice::draw_body(g, "shotgun::fire_shell#2", shooter) * m::PI * 2.0;
        let x = r * m::cos(a);
        let y = r * m::sin(a);
        let dir = v3::normalize([d0[0] + ax[0] * x + ay[0] * y, d0[1] + ax[1] * x + ay[1] * y, d0[2] + ax[2] * x + ay[2] * y]);
        let rest = crate::combat::FireOpts { spread: Some(0.0), weapon: Some(def.clone()), steady: Some(true), blind: o.blind };
        crate::combat::fire_pellet(g, shooter, v3::mad(muzzle, dir, 30.0), &rest, &mut shell);
        i += 1.0;
    }
    for r in &shell.late { crate::combat::round_in(g, r); }
    for (c, h) in shell.men.iter() {
        if h.down { continue; }
        g.stats.shotgun.peppered += 1;
        crate::flight::struck(g, c, h.dir, shooter);
    }
    g.m.shotgun.shotgun_last.ends = shell.ends.clone();
    g.m.shotgun.shotgun_last.glass = shell.glass.clone();
    let b = match p.breach { Some(b) => b, None => return };
    let mut leaves: Option<JsMap<DoorId, f64>> = None;
    for e in shell.ends.iter() {
        if e.index < 1e6 || e.t > b.reach_m { continue; }
        let bx = g.col.dynamic_boxes.get((e.index - 1e6) as usize).copied();
        let d = match bx { Some(bx) => g.doors.list.iter().copied().find(|&x| g.dr[x].box_ == bx), None => None };
        let d = match d { Some(d) => d, None => continue };
        if !crate::doors::door_latched(g, d) || !crate::doors::door_locked(g, d) || g.dr[d].lock_id.starts_with("wedge:") { continue; }
        let u = crate::doors::door_dir(g, d);
        let along = (e.at[0] - g.dr[d].def.hinge[0]) * u[0] + (e.at[2] - g.dr[d].def.hinge[1]) * u[1];
        if along < g.dr[d].def.width - b.latch_m { continue; }
        let map = leaves.get_or_insert_with(JsMap::new);
        let n = match map.get(d) { Some(n) => *n, None => 0.0 };
        map.set(d, n + 1.0);
    }
    if let Some(leaves) = leaves {
        for (d, n) in leaves.iter() {
            if *n >= b.need {
                let id = g.ch[shooter].id;
                if !crate::doors::doors_kick_in(g, d, muzzle, id, Some(true)) { continue; }
                g.stats.shotgun.locks += 1;
                if crate::character::is_player(g, shooter) {
                    crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::DoorKicked, d);
                }
            }
        }
    }
}

/// He carries a gun that takes a lock, in hand or in his big slot.
pub fn carries_breacher(g: &crate::game::Game, gd: GuardId) -> bool {
    if crate::condition::down(g, g.gd[gd].ch) { return false; }
    let def = g.fa[g.gd[gd].weapon].def;
    let breach = match g.m.weapons.weapons.get(def).unwrap().pellets { Some(p) => p.breach.is_some(), None => false };
    breach || crate::kit::kit_big(g, gd) == crate::kit::BigKit::Shotgun
}

/// The lock as a man on face `side` shoots it: a hand in from the latch edge, lever high, just proud of the leaf.
pub fn lock_point(g: &crate::game::Game, d: DoorId, side: f64) -> V3 {
    let a = g.dr[d].def.width - g.k.shotgun.lock_in;
    let y = g.k.shotgun.lock_h;
    crate::doortools::door_pt(g, d, a, LOCK_PROUD, side, Some(y))
}

/// Where the man with the gun stands for d's lock on face `side`. None: no spot has floor and a line.
pub fn breach_spot(g: &mut crate::game::Game, d: DoorId, side: f64, avoid: Option<&[V3]>, mark: Option<V3>, close: Option<crate::shotgun::BreachSpotClose>) -> Option<crate::shotgun::BreachSpot> {
    let avoid: &[V3] = avoid.unwrap_or(&[]);
    let at = lock_point(g, d, side);
    let w = g.dr[d].def.width;
    let mark_along = match mark { Some(mk) => crate::doortools::along_of(g, d, mk), None => m::NAN };
    let mark_ok = match mark {
        Some(mk) => {
            (mark_along < 0.0 || mark_along > w)
                && v3::dist_xz(mk, at) <= g.k.shotgun.mark_reach
                && on_leaf(g, d, side, w, mk)
                && stands(g, at, mk)
                && !across(g, at, avoid, mk)
        }
        None => false,
    };
    let mut mine: Option<Option<crate::shotgun::BreachSpot>> = None;
    if let Some(close) = close {
        let tool = match close {
            crate::shotgun::BreachSpotClose::Shotgun => crate::bigkit::BigSpotsTool::Shotgun,
            crate::shotgun::BreachSpotClose::Hammer => crate::bigkit::BigSpotsTool::Hammer,
        };
        for s in crate::bigkit::big_spots(g, d, side, tool) {
            if crowds(g, avoid, s.pos)
                || crate::orders::squad_orders_behind_glass(g, d, side, s.pos)
                || (close == crate::shotgun::BreachSpotClose::Shotgun && across(g, at, avoid, s.pos))
            {
                continue;
            }
            return Some(crate::shotgun::BreachSpot { pos: s.pos, yaw: s.yaw, at: s.at, beside: true, own: false });
        }
    }
    let mut crowded: Option<crate::shotgun::BreachSpot> = None;
    let mut glazed: Option<crate::shotgun::BreachSpot> = None;
    let mut panes = false;
    let mut i = 0;
    while i < g.k.shotgun.spots.len() {
        let [past, off] = g.k.shotgun.spots[i];
        i += 1;
        let pos = crate::doortools::door_pt(g, d, w + past, off, side, None);
        if !stands(g, at, pos) { continue; }
        if crowds(g, avoid, pos) {
            if crowded.is_none() { crowded = Some(spot(at, pos, past > 0.0, false)); }
            panes = panes || crate::orders::squad_orders_behind_glass(g, d, side, pos);
            continue;
        }
        if crate::orders::squad_orders_behind_glass(g, d, side, pos) {
            if glazed.is_none() { glazed = Some(spot(at, pos, past > 0.0, false)); }
            panes = true;
            continue;
        }
        let mine_here = if panes { own_side(g, d, side, at, w, avoid, mark, mark_along, mark_ok, &mut mine) } else { None };
        return Some(match mine_here { Some(s) => s, None => spot(at, pos, past > 0.0, false) });
    }
    if !panes { return crowded; }
    if let Some(s) = own_side(g, d, side, at, w, avoid, mark, mark_along, mark_ok, &mut mine) { return Some(s); }
    if mark_ok { return Some(spot(at, mark.unwrap(), true, true)); }
    if glazed.is_some() { return glazed; }
    crowded
}

/// The answer for a man standing at `pos`, looking at the lock `at`. (breachSpot's `spot`)
fn spot(at: V3, pos: V3, beside: bool, own: bool) -> crate::shotgun::BreachSpot {
    crate::shotgun::BreachSpot { pos, yaw: v3::yaw_to(pos, at), at, beside, own }
}

/// Floor under `pos` and a clear line from chest height to the lock. (breachSpot's `stands`)
fn stands(g: &mut Game, at: V3, pos: V3) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) && !crate::scene_collision::solid_blocked(g, [pos[0], 1.2, pos[2]], at, None)
}

/// On top of one of the other men's marks. (breachSpot's `crowds`)
fn crowds(g: &Game, avoid: &[V3], pos: V3) -> bool {
    let clear = g.k.shotgun.clear;
    avoid.iter().any(|&p| v3::dist_xz(p, pos) < clear)
}

/// A mate's mark (or you) on the line from `pos` to the lock. (breachSpot's `across`)
fn across(g: &Game, at: V3, avoid: &[V3], pos: V3) -> bool {
    let line = g.k.shotgun.line;
    let ax = at[0] - pos[0];
    let az = at[2] - pos[2];
    let l2 = js::or(ax * ax + az * az, 1.0);
    avoid.iter().any(|&p| {
        let k = m::max(0.0, m::min(1.0, ((p[0] - pos[0]) * ax + (p[2] - pos[2]) * az) / l2));
        m::hyp2(p[0] - pos[0] - ax * k, p[2] - pos[2] - az * k) < line
    })
}

/// The bore's line from `pos` through the lock point lands on the leaf's face. (breachSpot's `onLeaf`)
fn on_leaf(g: &Game, d: DoorId, side: f64, w: f64, pos: V3) -> bool {
    let o = crate::doortools::off_of(g, d, pos, side) - LOCK_PROUD;
    let a = w - g.k.shotgun.lock_in;
    let hit = a + LOCK_PROUD * (a - crate::doortools::along_of(g, d, pos)) / o;
    o > 0.0 && hit >= 0.0 && hit <= w
}

/// His own side of the frame, clean of glass: asked once, `mine` keeps the answer. (breachSpot's `ownSide`)
fn own_side(g: &mut Game, d: DoorId, side: f64, at: V3, w: f64, avoid: &[V3], mark: Option<V3>, mark_along: f64, mark_ok: bool, mine: &mut Option<Option<crate::shotgun::BreachSpot>>) -> Option<crate::shotgun::BreachSpot> {
    if let Some(known) = *mine { return known; }
    let answer = own_side_asked(g, d, side, at, w, avoid, mark, mark_along, mark_ok);
    *mine = Some(answer);
    answer
}

/// His mark, else (a hinge-side man) the first clean hinge spot. (the body of breachSpot's `ownSide`)
fn own_side_asked(g: &mut Game, d: DoorId, side: f64, at: V3, w: f64, avoid: &[V3], mark: Option<V3>, mark_along: f64, mark_ok: bool) -> Option<crate::shotgun::BreachSpot> {
    if let Some(mk) = mark {
        if mark_ok && !crate::orders::squad_orders_behind_glass(g, d, side, mk) { return Some(spot(at, mk, true, true)); }
        if mark_along < w / 2.0 {
            let mut i = 0;
            while i < g.k.shotgun.hinge.len() {
                let [past, off] = g.k.shotgun.hinge[i];
                i += 1;
                let pos = crate::doortools::door_pt(g, d, -past, off, side, None);
                if stands(g, at, pos) && on_leaf(g, d, side, w, pos) && !crowds(g, avoid, pos) && !across(g, at, avoid, pos) && !crate::orders::squad_orders_behind_glass(g, d, side, pos) {
                    return Some(spot(at, pos, true, true));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m870_shell(g: &Game) -> crate::shotgun::Shell {
        let def = g.m.weapons.weapons.get(crate::weapons::WeaponId::M870).unwrap().clone();
        crate::shotgun::Shell { def, i: 0.0, dir: None, men: JsMap::new(), ends: Vec::new(), glass: JsMap::new(), late: Vec::new() }
    }

    /// Inside dropM the first pellet puts him down, and no later one does it again.
    #[test]
    fn inside_drop_range_the_first_pellet_drops_him_once() {
        let mut g = Game::blank();
        let mut s = m870_shell(&g);
        let p = s.def.pellets.unwrap();
        assert!(pellet_drops(&mut g, &mut s, CharId(1), p.drop_m, [1.0, 0.0, 0.0]));
        assert!(!pellet_drops(&mut g, &mut s, CharId(1), p.drop_m, [1.0, 0.0, 0.0]));
        assert_eq!(g.stats.shotgun.in_men, 2);
        assert_eq!(g.stats.shotgun.dropped, 1);
    }

    /// Past dropM only the need-th pellet of this shell does it; each man keeps his own count, the first dir and the nearest t.
    #[test]
    fn past_drop_range_the_need_th_pellet_drops_him() {
        let mut g = Game::blank();
        let mut s = m870_shell(&g);
        let p = s.def.pellets.unwrap();
        let far = p.drop_m + 5.0;
        let mut n = 1.0;
        while n < p.need {
            assert!(!pellet_drops(&mut g, &mut s, CharId(0), far + n, [n, 0.0, 0.0]));
            n += 1.0;
        }
        assert!(!pellet_drops(&mut g, &mut s, CharId(2), far, [0.0, 0.0, 1.0]));
        assert!(pellet_drops(&mut g, &mut s, CharId(0), far, [9.0, 0.0, 0.0]));
        let h = s.men.get(CharId(0)).unwrap();
        assert!(h.down);
        assert_eq!(h.n, p.need);
        assert_eq!(h.near, far);
        assert_eq!(h.dir, [1.0, 0.0, 0.0]);
        let keys: Vec<CharId> = s.men.iter().map(|(k, _)| k).collect();
        assert_eq!(keys, vec![CharId(0), CharId(2)]);
    }
}
