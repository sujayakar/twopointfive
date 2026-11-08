//! src/game/buddy.ts lines 1 to 265
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the counts of one run, all nought. (buddy.ts newStats)
pub fn new_stats() -> crate::buddy::StateStats {
    StateStats { calls: 0.0, answered: 0.0, arrived: 0.0, met_on_way: 0.0, turned_back: 0.0, died_on_way: 0.0, short: 0.0, no_one: 0.0, secs: 0.0, metres: 0.0 }
}

/// this game's state, made if it has none yet: the row itself. (buddy.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::buddy::State {
    g.m.buddy.states.get_or_insert_with(|| State { men: JsMap::new(), errands: Vec::new(), stats: new_stats(), stale: false })
}

/// distance on the floor. (buddy.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// a roll of the game's die inside a range. (buddy.ts pick)
pub fn pick(g: &mut crate::game::Game, who: crate::dice::Who, r: [f64; 2]) -> f64 {
    r[0] + crate::dice::draw_for(g, "buddy::pick#1", who) * (r[1] - r[0])
}

/// (buddy.ts alive)
pub fn alive(g: &crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    g.gd[gd].state != crate::game::GuardState::Dead
}

/// a row of the fight's log, which is capped. (buddy.ts note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: &str) {
    let row = crate::roomfight::FightNote { t: g.time, who: g.gd[gd].callsign.clone(), what: format!("[buddy] {}", what) };
    g.fight.log.push(row);
    if g.fight.log.len() > 600 {
        g.fight.log.drain(0..200);
    }
}

/// the nav grid's cell of a point. (buddy.ts cellOf)
pub fn cell_of(g: &crate::game::Game, p: V3) -> f64 {
    let cell = g.col.nav.cell;
    crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell))
}

/// his own wits about him, and nobody else's man. (buddy.ts own)
pub fn own(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    alive(g, d)
        && g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold
        && g.gd[gd].held.is_none()
        && !crate::control::man::blind(g, gd)
        && !crate::control::man::staggered(g, gd)
        && g.tab.roomfight_def[d].rally_at != m::INF
}

/// the men of his side holding the room he holds, himself not counted. (buddy.ts mates)
pub fn mates(g: &mut crate::game::Game, d: RoomfightDefId) -> Vec<RoomfightDefId> {
    let room = g.tab.roomfight_def[d].room;
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if x != d && alive(g, x) && g.tab.roomfight_def[x].mode != crate::roomfight::DefMode::Free && room.is_some() && g.tab.roomfight_def[x].room == room {
            out.push(x);
        }
    }
    out
}

/// may this man answer a call. (buddy.ts free)
pub fn free(g: &mut crate::game::Game, x: RoomfightDefId, mine: Option<u32>) -> bool {
    let gd = g.tab.roomfight_def[x].gd;
    if !own(g, x)
        || g.time - g.gd[gd].last_seen_t < g.k.buddy.quiet
        || state_of(g).errands.iter().any(|e| Some(e.serial) != mine && (e.buddy == x || e.caller == x))
    {
        return false;
    }
    let done_t = state_of(g).men.get(x).map_or(-100.0, |w| w.done_t);
    if g.time - done_t < g.k.buddy.again {
        return false; // (he has only just come to somebody: not off again on the next call)
    }
    if crate::owner::has(g, gd, crate::owner::by::PRESS)
        || crate::pressure::press_keeps_home(g, x, None)
        || g.gd[gd].stays_in_rooms
        || crate::owner::has(g, gd, crate::owner::by::SQUAD_WORD)
        || crate::owner::has(g, gd, crate::owner::by::STAND)
    {
        return false;
    }
    let set = crate::digin::dig_in_set(g, gd);
    if set && g.k.digin.reach_m > 0.0 {
        return false; // (digin.ts's own rules for who holds his room keep him)
    }
    let all = mates(g, x);
    let mut firm = 0;
    for &w in all.iter() {
        if g.tab.roomfight_def[w].mode == crate::roomfight::DefMode::Hold && !state_of(g).errands.iter().any(|e| e.buddy == w) {
            firm += 1;
        }
    }
    if firm >= 2 {
        return true;
    }
    all.is_empty() // (one of a pair stays; alone himself he may come, and the two pair up in the caller's room)
}

/// for a moment after, the two have just paired up: he waits. (buddy.ts buddyPaired)
pub fn buddy_paired(g: &crate::game::Game, d: RoomfightDefId) -> bool {
    let keep_t = g.m.buddy.states.as_ref().and_then(|st| st.men.get(d)).map_or(-100.0, |w| w.keep_t);
    false
}

/// the fight is over: every call forgotten; the counts stay until a new fight starts them again. (buddy.ts buddyStop)
pub fn buddy_stop(g: &mut crate::game::Game) {
    let st = match g.m.buddy.states.as_mut() {
        Some(st) => st,
        None => return,
    };
    st.men.clear();
    st.errands = Vec::new();
    st.stale = true;
    let rule = crate::owner::by::BUDDY;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let t = g.time;
        crate::owner::release(g, gd, rule, t);
    }
}

