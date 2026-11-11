pub use ::game_types::planmode::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// Shift + the key adds a goal: on Sam's own squad, and on a lesson only from the step that teaches it.
pub(super) fn goals_on(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    crate::orders::squad_orders_sams(seat) && !crate::rails::rails_locks(g, "goal")
}

/// The squad as the goals it is on now: the men with nothing to do, and the rest by what they were sent to.
fn goal_parts(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> crate::planmode::GoalParts {
    let mut idle: Vec<GuardId> = Vec::new();
    let mut by: JsMap<String, Vec<GuardId>> = JsMap::new();
    for m in crate::orders::squad_orders_men(g, seat, Some(crate::orders::SquadOrdersMenSel::All), None) {
        let a = match crate::orders::squad_orders_of(seat, m) {
            Some(a) if g.tab.task[a].post.is_some() => a,
            _ => {
                idle.push(m);
                continue;
            }
        };
        let row = &g.tab.task[a];
        let k = match row.door {
            Some(d) => alloc::format!("d{}", g.doors.list.iter().position(|x| *x == d).map_or(-1, |i| i as i64)),
            None => alloc::format!("g{}", row.group),
        };
        match by.get_mut(k.clone()) {
            Some(l) => l.push(m),
            None => {
                by.set(k, vec![m]);
            }
        }
    }
    crate::planmode::GoalParts { idle, groups: by.values().cloned().collect() }
}

/// Who takes a goal added on `t`, and how they come to; None: every group is one man and nobody is free. Orders and counts nothing.
pub fn goal_men(g: &crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::planmode::GoalMen> {
    use crate::orders::OrderTarget;
    use crate::planmode::{GoalMen, GoalMenHow};

    let all = crate::orders::squad_orders_men(g, &crate::seats::here(g), Some(crate::orders::SquadOrdersMenSel::All), None);
    let crate::planmode::GoalParts { idle, groups } = goal_parts(g, &crate::seats::here(g));
    if let OrderTarget::None(n) = t {
        if n.man.is_some() {
            return Some(GoalMen { men: crate::orders::squad_orders_recipients(g, &crate::seats::here(g), Some(t), None), how: GoalMenHow::His });
        }
    }
    let half = groups.is_empty() && g.k.cmdring.goal_half && all.len() > 1;
    if matches!(t, OrderTarget::None(_)) || (groups.is_empty() && !half) {
        return Some(GoalMen { men: all, how: GoalMenHow::All });
    }
    if let OrderTarget::Door(d) = t {
        for men in &groups {
            if every_on_door(g, &crate::seats::here(g), men, d.door) {
                return Some(GoalMen { men: men.clone(), how: GoalMenHow::Same });
            }
        }
    }
    if !idle.is_empty() && !half {
        return Some(GoalMen { men: idle, how: GoalMenHow::Idle });
    }
    let pool: Vec<Vec<GuardId>> = if half { vec![all] } else { groups };
    let size = pool.iter().map(|l| l.len()).max().unwrap_or(0);
    if size < 2 {
        return None;
    }
    let biggest: Vec<Vec<GuardId>> = pool.into_iter().filter(|x| x.len() == size).collect();
    let group = best(g, &biggest, t).clone();
    let mut parts: Vec<Vec<GuardId>> = Vec::new();
    for id in crate::orders::squad_orders_pairs(g, &crate::seats::here(g)) {
        let mut l: Vec<GuardId> = Vec::new();
        for &m in &group {
            if crate::orders::squad_orders_pair_of(g, m) == Some(id) {
                l.push(m);
            }
        }
        if l.len() * 2 <= group.len() + (if half { 1 } else { 0 }) && !l.is_empty() {
            parts.push(l);
        }
    }
    let men = if !parts.is_empty() {
        best(g, &parts, t).clone()
    } else {
        let mut sorted = group.clone();
        js::sort_by(&mut sorted, |a, b| far(g, &[*a], t) - far(g, &[*b], t));
        sorted.truncate(group.len() >> 1);
        sorted
    };
    Some(GoalMen { men, how: GoalMenHow::Split })
}

/// Shift + the key kept down is the ring for those men.
pub(super) fn goal_ring_on(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    goals_on(g, seat)
}

/// Shift + the key on `t`: a part of the squad takes what the key alone gives there, and the rest go on as they were.
pub fn plan_goal(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) {
    if crate::orders::squad_orders_men(g, &crate::seats::here(g), Some(crate::orders::SquadOrdersMenSel::All), None).is_empty() {
        return;
    }
    let Some(goal) = goal_men(g, t) else {
        crate::interact::say_why(g, None, crate::view::ThingKind::Wall, crate::planmode::GOAL_NONE, crate::planmode::GOAL_NONE.trim_end_matches('.'));
        return;
    };
    crate::autopilot::quick(g, t, Some(&goal.men));
}

/// Shift + the ring's key is the goal's own this frame, only where the ring has no hold to wait out.
pub(super) fn goal_shift(g: &crate::game::Game, seat: &crate::seats::SeatRef, input: InputId) -> bool {
    (crate::input::input_down(g, input, "ShiftLeft") || crate::input::input_down(g, input, "ShiftRight")) && goals_on(g, seat) && !(g.k.cmdring.hold_secs > 0.0)
}

/// Once a tick with a squad on the floor and Sam up: where the ring has no hold, the pair adds the goal as the key goes down.
pub(super) fn goal_input(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    if !(goal_shift(g, &crate::seats::here(g), input) && g.sets[g.seat].orders.cmd_ring.is_none() && crate::orders::squad_orders_back_free(g, &crate::seats::here(g))) {
        return;
    }
    let key = g.k.cmdring.key.clone();
    if g.tab.input[input].pressed.delete(key) {
        let t = crate::orders::squad_orders_target_under(g, input, cam, canvas);
        plan_goal(g, &t);
    }
}

/// `G.every(m => S.of(m)?.door === door)`.
fn every_on_door(g: &Game, seat: &crate::seats::SeatRef, men: &[GuardId], door: DoorId) -> bool {
    for &m in men {
        let on = match crate::orders::squad_orders_of(seat, m) {
            Some(a) => g.tab.task[a].door == Some(door),
            None => false,
        };
        if !on {
            return false;
        }
    }
    true
}

/// The men's mean distance to `t` on the floor.
fn far(g: &Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> f64 {
    let at = t.at();
    let mut s = 0.0;
    for &m in men {
        let p = g.pos_of(m);
        s = s + v3::dist_xz(p, at);
    }
    s / men.len() as f64
}

/// The nearest list; ties keep the earlier. `ls` is never empty here.
fn best<'a>(g: &Game, ls: &'a [Vec<GuardId>], t: &crate::orders::OrderTarget) -> &'a Vec<GuardId> {
    let mut a = &ls[0];
    for b in &ls[1..] {
        if far(g, b, t) < far(g, a, t) - 1e-6 {
            a = b;
        }
    }
    a
}
