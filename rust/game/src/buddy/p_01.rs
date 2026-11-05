//! src/game/buddy.ts lines 1 to 261
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the panel's rows kept in the browser's storage. Host only (conventions 6): where the TypeScript finds no store it does nothing, and the game has none. (buddy.ts saveBuddy)
pub fn save_buddy(g: &mut crate::game::Game) {}

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
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 {
    r[0] + crate::math_dice::sim_random(g) * (r[1] - r[0])
}

/// (buddy.ts alive)
pub fn alive(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
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
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 {
    let cell = g.col.nav.cell;
    crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell))
}

/// his own wits about him, and nobody else's man. (buddy.ts own)
pub fn own(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    alive(g, d)
        && g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold
        && g.gd[gd].held.is_none()
        && g.time >= g.gd[gd].dazzled_until
        && g.time >= g.gd[gd].stagger_until
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

/// `gd.lastKnown === D.pt`: one object in the TypeScript, two copies here. The flag says the fight last wrote pt as the fix; a fix without pt's bits is somebody else's whole write. (the fight's own test, roomfight.ts)
fn fix_is_pt(g: &mut Game, d: RoomfightDefId) -> bool {
    let def = &g.tab.roomfight_def[d];
    def.fix_is_pt && g.gd[def.gd].last_known == Some(def.pt)
}

/// where the team is, as far as the defenders know. Never the team's true positions. (buddy.ts teamKnown)
pub fn team_known(g: &mut crate::game::Game) -> Vec<V3> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        let gd = g.tab.roomfight_def[d].gd;
        if !alive(g, d) {
            continue;
        }
        if g.time - g.gd[gd].last_seen_t < 4.0 && g.gd[gd].last_known.is_some() && !fix_is_pt(g, d) {
            out.push(g.gd[gd].last_known.unwrap());
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
    if crate::pressure::press_owns(g, x) || crate::enemysquads::squad_stays_in(g, gd) || crate::enemysquads::squad_defers(g, gd) || crate::laststand::last_stand_man(g, gd) {
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
    let go_at = g.time + pick(g, answer);
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
    if crate::laststand::last_stand_man(g, gd) {
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
        let at = time + pick(g, beat);
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

/// the call stands for the next man (inside its own `again` seconds). (buddy.ts tickErrand reopen)
fn reopen(g: &mut Game, c: RoomfightDefId, wanted: bool) {
    let has = state_of(g).men.has(c);
    if has && wanted && mates(g, c).is_empty() {
        let look_t = g.time + 1.0;
        let w = man_of(g, c);
        w.open = true;
        w.look_t = look_t;
    }
}

/// the colleague's errand, a frame of it; true = over. (buddy.ts tickErrand)
pub fn tick_errand(g: &mut crate::game::Game, e: &mut crate::buddy::Errand) -> bool {
    let (b, c) = (e.buddy, e.caller);
    let gd = g.tab.roomfight_def[b].gd;
    let c_call = g.gd[g.tab.roomfight_def[c].gd].callsign.clone();
    // the caller is still a man holding somewhere (he may have given ground: he is found where he now is)
    let wanted = alive(g, c) && g.fight.defs.contains(&c) && g.tab.roomfight_def[c].mode != crate::roomfight::DefMode::Free;
    if !alive(g, b) || !g.fight.defs.contains(&b) {
        if e.phase == ErrandPhase::Run {
            state_of(g).stats.died_on_way += 1.0;
            note(g, gd, "down on the way");
        }
        if e.phase != ErrandPhase::Back {
            reopen(g, c, wanted);
        }
        return true;
    }
    if e.phase == ErrandPhase::Beat {
        if g.time < e.go_at {
            return false;
        }
        if !wanted || !mates(g, c).is_empty() || !free(g, b, Some(e.serial)) {
            reopen(g, c, wanted); // (he has his own trouble now, or somebody else's word)
            return true;
        }
        e.phase = ErrandPhase::Run;
        state_of(g).stats.answered += 1.0;
        crate::pressure::run(g, b, e.spot, None, None);
        crate::game::game_say(g, gd, "on my way to you — hold on!", Some(true), None, None);
        note(g, gd, &format!("sets off for {}", c_call));
        return false;
    }
    if g.tab.roomfight_def[b].mode == crate::roomfight::DefMode::Move {
        e.step_t = crate::pressure::boots(g, gd, e.step_t);
        if e.phase == ErrandPhase::Run && !wanted {
            // the man he was going to is down, or gone: back to the post he left
            e.phase = ErrandPhase::Back;
            e.tries = 0.0;
            state_of(g).stats.turned_back += 1.0;
            crate::pressure::run(g, b, e.home, None, None);
            crate::game::game_say(g, gd, "he's not answering — going back!", Some(true), None, None);
            note(g, gd, &format!("{} is down or gone: back to his post", c_call));
        }
        return false;
    }
    if g.tab.roomfight_def[b].mode == crate::roomfight::DefMode::Free {
        return true; // (somebody else has let him off on the fight AI's hunt: theirs)
    }
    // planted (roomfight.ts tickMove): there, in contact on the way, or stopped short
    let contact = g.time - g.gd[gd].last_seen_t < 2.0;
    if e.phase == ErrandPhase::Back {
        if !contact && flat(g.ch[g.gd[gd].ch].pos, e.home) > 2.0 {
            e.tries += 1.0;
            if e.tries < 3.0 && own(g, b) {
                crate::pressure::run(g, b, e.home, None, None);
                return false;
            }
        }
        return true;
    }
    if !wanted {
        state_of(g).stats.turned_back += 1.0;
        note(g, gd, &format!("{} is down or gone: holds where he stopped", c_call));
        return true;
    }
    if g.tab.roomfight_def[c].mode == crate::roomfight::DefMode::Move {
        return false; // (the caller is on a leg of somebody's: where he stops is where he is found)
    }
    let b_room = g.tab.roomfight_def[b].room;
    let there = b_room.is_some() && b_room == g.tab.roomfight_def[c].room && g.tab.roomfight_def[c].mode == crate::roomfight::DefMode::Hold;
    if there {
        let dt = g.time - e.t0;
        let s = &mut state_of(g).stats;
        s.arrived += 1.0;
        s.secs += dt;
        s.metres += e.l;
        // what the caller knows of the room's doors is told him, and he takes one the caller is not on
        let r = b_room.unwrap();
        let c_watch = g.tab.roomfight_def[c].watch;
        let c_pt = g.tab.roomfight_def[c].pt;
        let mine = if c_watch >= 0.0 { c_watch } else { nearest_entry(g, r, c_pt) };
        let n = g.tab.roomfight_room[r].entries.len();
        let mut ev: Vec<f64> = Vec::with_capacity(n);
        for i in 0..n {
            ev.push(g.tab.roomfight_def[c].ev.get(i).copied().unwrap_or(0.0));
        }
        let mut other = -1.0;
        let mut bv = -1.0;
        for i in 0..n {
            if i as f64 != mine && ev[i] > bv {
                bv = ev[i];
                other = i as f64;
            }
        }
        g.tab.roomfight_def[b].ev = ev;
        g.tab.roomfight_def[b].watch = -1.0;
        g.tab.roomfight_def[b].want = -1.0;
        let pick_i = if other >= 0.0 { other } else { mine };
        if pick_i >= 0.0 && (pick_i as usize) < n {
            let at = g.tab.roomfight_room[r].entries[pick_i as usize].at;
            let aliased = fix_is_pt(g, b); // (asked before pt moves)
            let def = &mut g.tab.roomfight_def[b];
            def.home = v3::copy(at);
            def.pt[0] = at[0];
            def.pt[1] = 0.0;
            def.pt[2] = at[2];
            let pt = def.pt;
            if aliased {
                g.gd[gd].last_known = Some(pt); // (line 225 moves D.pt in place: a lastKnown that IS it moves with it)
            }
        }
        let keep_t = g.time + g.k.buddy.again;
        for x in [b, c] {
            if let Some(w) = state_of(g).men.get_mut(x) {
                w.keep_t = keep_t;
            }
        }
        crate::game::game_say(g, gd, "with you — two on this room!", Some(true), None, None);
        let id = g.tab.roomfight_room[r].id;
        let dt = g.time - e.t0;
        note(g, gd, &format!("with {} in room {}, {} s after the call", c_call, id, js::to_fixed(dt, 1)));
        return true;
    }
    if !contact {
        e.tries += 1.0;
        if e.tries < 3.0 && own(g, b) {
            let spot = spot_by(g, c);
            let known = team_known(g);
            if way_to(g, b, spot, &known).is_some() {
                e.spot = spot;
                crate::pressure::run(g, b, spot, None, None); // (the room fight's move is ten seconds long: a longer way is walked in legs)
                return false;
            }
        }
    }
    if contact {
        state_of(g).stats.met_on_way += 1.0;
        note(g, gd, "met on the way: fights where he is");
    } else {
        state_of(g).stats.short += 1.0;
        note(g, gd, "stopped short of him");
    }
    true
}

/// the door of a room nearest a point, as an index, -1 for none. (buddy.ts nearestEntry)
pub fn nearest_entry(g: &mut crate::game::Game, r: RoomfightRoomId, p: V3) -> f64 {
    let mut best = -1.0;
    let mut bd = m::INF;
    for (i, e) in g.tab.roomfight_room[r].entries.iter().enumerate() {
        let d = flat(e.at, p);
        if d < bd {
            bd = d;
            best = i as f64;
        }
    }
    best
}

/// roomfight.ts rally() asks: a colleague is on his way to him, or the two have only just paired up. (buddy.ts buddyOwns)
pub fn buddy_owns(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    if !g.k.buddy.on {
        return false;
    }
    let st = match &g.m.buddy.states {
        Some(st) => st,
        None => return false,
    };
    g.time < st.men.get(d).map_or(-100.0, |w| w.keep_t) || st.errands.iter().any(|e| e.caller == d)
}

/// once a frame from RoomFight.tick, after the defenders', the teammates' and the press's own ticks. (buddy.ts buddyTick)
pub fn buddy_tick(g: &mut crate::game::Game) {
    if !g.k.buddy.on {
        return;
    }
    let st = state_of(g);
    if st.stale {
        st.stale = false;
        st.stats = new_stats();
    }
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        if alive(g, d) {
            consider(g, d);
        }
    }
    if state_of(g).errands.is_empty() {
        return;
    }
    // the filter walks a copy: nothing adds to the list while it runs, and what the walk reads of the list (caller, buddy) no errand changes
    let rows = state_of(g).errands.clone();
    let mut kept: Vec<Errand> = Vec::with_capacity(rows.len());
    for mut e in rows {
        let over = tick_errand(g, &mut e);
        if over && e.phase != ErrandPhase::Beat {
            let now = g.time;
            if let Some(w) = state_of(g).men.get_mut(e.buddy) {
                w.done_t = now;
            }
        }
        if !over {
            kept.push(e);
        }
    }
    state_of(g).errands = kept;
}

/// the fight is over: every call forgotten; the counts stay until a new fight starts them again. (buddy.ts buddyStop)
pub fn buddy_stop(g: &mut crate::game::Game) {
    if let Some(st) = g.m.buddy.states.as_mut() {
        st.men.clear();
        st.errands = Vec::new();
        st.stale = true;
    }
}

/// the counts of this run, by the TypeScript's names and in its order. (buddy.ts buddyStats)
pub fn buddy_stats(g: &mut crate::game::Game) -> JsMap<String, f64> {
    let s = g.m.buddy.states.as_ref().map_or_else(new_stats, |st| st.stats);
    let rows: [(&str, f64); 10] = [
        ("calls", s.calls),
        ("answered", s.answered),
        ("arrived", s.arrived),
        ("metOnWay", s.met_on_way),
        ("turnedBack", s.turned_back),
        ("diedOnWay", s.died_on_way),
        ("short", s.short),
        ("noOne", s.no_one),
        ("secs", js::to_fixed(s.secs, 1).parse::<f64>().unwrap()),
        ("metres", js::to_fixed(s.metres, 1).parse::<f64>().unwrap()),
    ];
    let mut out = JsMap::new();
    for (k, v) in rows {
        out.set(String::from(k), v);
    }
    out
}

/// the line of the debugger's panel. (buddy.ts buddySummary)
pub fn buddy_summary(g: &mut crate::game::Game) -> String {
    if !g.k.buddy.on {
        return String::from("off");
    }
    let st = match &g.m.buddy.states {
        Some(st) => st,
        None => return String::from("on · nothing yet"),
    };
    let s = &st.stats;
    format!("{} call{} · {} answered · {} arrived · {} on the way", s.calls, if s.calls == 1.0 { "" } else { "s" }, s.answered, s.arrived, st.errands.len())
}
