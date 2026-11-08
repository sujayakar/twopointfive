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

/// where the team is, as far as the defenders know. Never the team's true positions. (buddy.ts teamKnown)
pub fn team_known(g: &crate::game::Game) -> Vec<V3> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        let gd = g.tab.roomfight_def[d].gd;
        if !alive(g, d) {
            continue;
        }
        let seen = if g.time - g.gd[gd].last_seen_t < 4.0 {
            let pt = g.tab.roomfight_def[d].pt;
            crate::belief::eyes_fix_def(g, gd, d)
        } else {
            None
        };
        if let Some(seen) = seen {
            out.push(seen);
        }
        if let Some(at) = g.tab.roomfight_def[d].noise_at {
            if g.time - g.tab.roomfight_def[d].noise_t < 4.0 {
                out.push(at);
            }
        }
    }
    out
}

/// a free spot of the caller's room a couple of strides from him, off its doorways. (buddy.ts spotBy)
pub fn spot_by(g: &mut crate::game::Game, c_2: RoomfightDefId) -> V3 {
    let gd = g.tab.roomfight_def[c_2].gd;
    let p0 = g.ch[g.gd[gd].ch].pos;
    let room = g.tab.roomfight_def[c_2].room;
    for r in [1.8, 2.6, 1.2] {
        for a in 0..8 {
            let a = a as f64;
            let p: V3 = [p0[0] + m::cos(a * m::PI / 4.0) * r, 0.0, p0[2] + m::sin(a * m::PI / 4.0) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) {
                continue;
            }
            if let Some(rm) = room {
                let cell = crate::Num(cell_of(g, p));
                if !g.tab.roomfight_room[rm].cells.has(cell) {
                    continue;
                }
                if g.tab.roomfight_room[rm].entries.iter().any(|e| flat(e.at, p) < 1.5) {
                    continue;
                }
            }
            let mut crowded = false;
            let mut i = 0;
            while i < g.fight.defs.len() {
                let x = g.fight.defs[i];
                i += 1;
                if alive(g, x) {
                    let c = g.gd[g.tab.roomfight_def[x].gd].ch;
                    if flat(g.ch[c].pos, p) < 0.9 {
                        crowded = true;
                        break;
                    }
                }
            }
            if crowded {
                continue;
            }
            if crate::scene_collision::segment_blocked(g, [p0[0], 1.2, p0[2]], [p[0], 1.2, p[2]], None, None, None) {
                continue; // (beside him: not the far side of a rack from him)
            }
            return p;
        }
    }
    [p0[0], 0.0, p0[2]]
}

/// his way to `spot`: its length, or None when none, too long, or past where the team is known to be. (buddy.ts wayTo)
pub fn way_to(g: &mut crate::game::Game, d: RoomfightDefId, spot: V3, known: &[V3]) -> Option<f64> {
    let pos = g.ch[g.gd[g.tab.roomfight_def[d].gd].ch].pos;
    let path = crate::scene_collision::nav_grid_find_path(g, pos, spot, None, None)?;
    if path.is_empty() || flat(path[path.len() - 1], spot) > 0.8 {
        return None;
    }
    let mut l = 0.0;
    let mut q: V3 = pos;
    let mut bad = false;
    let clear = g.k.buddy.clear_m;
    for &w in path.iter() {
        let dist = flat(q, w);
        let n = m::max(1.0, m::ceil(dist / 0.5));
        if clear > 0.0 {
            let mut i = 1.0;
            while i <= n && !bad {
                let p: V3 = [q[0] + (w[0] - q[0]) * i / n, 0.0, q[2] + (w[2] - q[2]) * i / n];
                if known.iter().any(|f| flat(*f, p) < clear) {
                    bad = true;
                }
                i += 1.0;
            }
        }
        l += dist;
        q = w;
    }
    if bad || l > g.k.buddy.max_m { None } else { Some(l) }
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

/// the nearest free colleague with a way to the caller. (buddy.ts answerer)
pub fn answerer(g: &mut crate::game::Game, c_2: RoomfightDefId) -> Option<crate::buddy::Answerer> {
    let spot = spot_by(g, c_2);
    let mut known: Option<Vec<V3>> = None;
    let mut best: Option<Answerer> = None;
    // (nearest by the crow first: a way is never shorter than that, so most men are ruled out without a path search)
    let c_room = g.tab.roomfight_def[c_2].room;
    let max_m = g.k.buddy.max_m;
    let mut near: Vec<(RoomfightDefId, f64)> = Vec::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if x != c_2 && g.tab.roomfight_def[x].room != c_room && alive(g, x) {
            let d = flat(g.ch[g.gd[g.tab.roomfight_def[x].gd].ch].pos, spot);
            if d <= max_m {
                near.push((x, d));
            }
        }
    }
    js::sort_by(&mut near, |a, b| a.1 - b.1);
    for &(x, d) in near.iter() {
        if let Some(b) = &best {
            if d >= b.l {
                break;
            }
        }
        if !free(g, x, None) {
            continue;
        }
        if known.is_none() {
            known = Some(team_known(g));
        }
        let l = match way_to(g, x, spot, known.as_ref().unwrap()) {
            Some(l) => l,
            None => continue,
        };
        if let Some(b) = &best {
            if l >= b.l {
                continue;
            }
        }
        best = Some(Answerer { x, spot, l });
    }
    best
}

/// a colleague is on his beat for this caller. (buddy.ts send)
pub fn send(g: &mut crate::game::Game, c_2: RoomfightDefId, a: crate::buddy::SendA, t0: f64) {
    if let Some(own0) = state_of(g).men.get_mut(a.x) {
        own0.open = false; // (a man alone himself who answers: his own call is dropped)
        own0.call_at = m::NAN;
    }
    let answer = g.k.buddy.answer;
    let go_at = g.time + pick(g, crate::dice::Who::Def(a.x), answer);
    let home = v3::copy(g.tab.roomfight_def[a.x].station);
    let serial = g.serial();
    state_of(g).errands.push(Errand { caller: c_2, buddy: a.x, phase: ErrandPhase::Beat, go_at, spot: a.spot, home, tries: 0.0, step_t: -100.0, t0, l: a.l, serial });
    let gd = g.tab.roomfight_def[a.x].gd;
    let who = g.gd[g.tab.roomfight_def[c_2].gd].callsign.clone();
    note(g, gd, &format!("will answer {} ({} m)", who, js::to_fixed(a.l, 1)));
}

fn send_a(a: Answerer) -> SendA {
    SendA { x: a.x, spot: a.spot, l: a.l }
}

/// the row of a man `consider` has just made sure of (nothing it calls clears the table)
fn man_of<'g>(g: &'g mut Game, d: RoomfightDefId) -> &'g mut Man {
    state_of(g).men.get_mut(d).unwrap()
}

/// a man holding a room by himself: after his beat, with nobody in his sight, he calls it. (buddy.ts consider)
pub fn consider(g: &mut crate::game::Game, d: RoomfightDefId) {
    let st = state_of(g);
    if !st.men.has(d) {
        st.men.set(d, Man { had: false, call_at: m::NAN, called_t: -100.0, calls: 0.0, open: false, look_t: 0.0, done_t: -100.0, keep_t: -100.0 });
    }
    let gd = g.tab.roomfight_def[d].gd;
    let again = g.k.buddy.again;
    let time = g.time;
    {
        let st = state_of(g);
        let w = st.men.get_mut(d).unwrap();
        if w.open && time - w.called_t > again {
            w.open = false;
            w.call_at = m::NAN;
            st.stats.no_one += 1.0;
            note(g, gd, "nobody could come");
        }
    }
    let room = g.tab.roomfight_def[d].room;
    let room = match room {
        Some(r) if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold => r,
        _ => {
            man_of(g, d).call_at = m::NAN; // (walking a leg of somebody's: a call he has made stands meanwhile)
            return;
        }
    };
    if crate::owner::has(g, gd, crate::owner::by::STAND) {
        let w = man_of(g, d);
        w.call_at = m::NAN;
        w.open = false;
        return;
    }
    if !mates(g, d).is_empty() {
        let w = man_of(g, d);
        w.had = true;
        w.call_at = m::NAN;
        w.open = false;
        return;
    }
    if state_of(g).errands.iter().any(|e| e.caller == d || e.buddy == d) {
        return;
    }
    if !own(g, d) {
        return;
    }
    if man_of(g, d).open {
        // his call stands: the first man to come free answers it
        if time < man_of(g, d).look_t {
            return;
        }
        man_of(g, d).look_t = time + 1.0;
        if let Some(a) = answerer(g, d) {
            man_of(g, d).open = false;
            let called_t = man_of(g, d).called_t;
            send(g, d, send_a(a), called_t);
        }
        return;
    }
    let w = *man_of(g, d);
    if !w.had || w.calls >= g.k.buddy.calls || time - w.called_t < again {
        man_of(g, d).call_at = m::NAN;
        return;
    }
    if w.call_at.is_nan() {
        let beat = g.k.buddy.beat;
        let at = time + pick(g, crate::dice::Who::Def(d), beat);
        man_of(g, d).call_at = at;
        return;
    }
    if time < w.call_at || time - g.gd[gd].last_seen_t < g.k.buddy.quiet {
        return;
    }
    if g.k.buddy.calm > 0.0 {
        if let Some(noise_at) = g.tab.roomfight_def[d].noise_at {
            let calm_m = g.k.buddy.calm_m;
            if time - g.tab.roomfight_def[d].noise_t < g.k.buddy.calm
                && (!g.tab.roomfight_def[d].noise_out || g.tab.roomfight_room[room].entries.iter().any(|e| flat(e.at, noise_at) < calm_m))
            {
                return;
            }
        }
    }
    let calls = {
        let w = man_of(g, d);
        w.call_at = m::NAN;
        w.called_t = time;
        w.calls += 1.0;
        w.look_t = time + 1.0;
        w.calls
    };
    state_of(g).stats.calls += 1.0;
    let line = if calls > 1.0 { "still on my own in here — anybody!" } else { "I'm on my own in here — somebody get to me!" };
    crate::game::game_say(g, gd, line, Some(true), None, None);
    let a = answerer(g, d);
    let id = g.tab.roomfight_room[room].id;
    note(g, gd, &format!("alone in room {}: calls for a buddy{}", id, if a.is_some() { "" } else { " (nobody free)" }));
    match a {
        Some(a) => {
            let now = g.time;
            send(g, d, send_a(a), now);
        }
        None => man_of(g, d).open = true,
    }
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

