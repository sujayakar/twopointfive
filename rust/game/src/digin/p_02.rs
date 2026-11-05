//! src/game/digin.ts lines 462 to 921
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// THE LIVE STATE is g.m.digin.states, reached by state_of. A function with `m: &mut Man` or `r: &mut RoomSt` holds a copy of that row checked out:
// the top of the stack (dig_in_tick, dig_in_release; row_put for a room) writes it back. `drop` writes a room's bracer to the table: never call it with that row checked out.

fn row_of(g: &mut Game, k: RoomfightRoomId) -> RoomSt { state_of(g).rooms.get(k).unwrap().clone() }
fn row_put(g: &mut Game, k: RoomfightRoomId, r: RoomSt) { state_of(g).rooms.set(k, r); }
/// `R.room.entries[i]`: undefined off either end, -1 included
fn entry_at(g: &Game, room: RoomfightRoomId, i: f64) -> Option<crate::roomfight::Entry> {
    if i < 0.0 { return None; }
    g.tab.roomfight_room[room].entries.get(i as usize).copied()
}
fn door_name(g: &Game, d: DoorId) -> String { g.dr[d].def.name.clone() }
fn prop_name(g: &Game, p: PropId) -> String { g.tab.props_prop[p].def.name.clone() }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
/// `!isNaN(st.rooms.get(room)?.enteredAt ?? NaN)`
fn entered(g: &mut Game, room: RoomfightRoomId) -> bool {
    match state_of(g).rooms.get(room) { Some(r) => !r.entered_at.is_nan(), None => false }
}
/// `st.men.get(gd)?.phase ?? 'idle'`
fn phase_of(g: &mut Game, gd: GuardId) -> Phase {
    match state_of(g).men.get(gd) { Some(x) => x.phase, None => Phase::Idle }
}
fn prop_at(g: &mut Game, p: PropId) -> V3 { [crate::props::prop_x(g, p), 0.0, crate::props::prop_z(g, p)] }

/// braceInRoom: `to` is his room's floor and the way to it from `from` does not leave the room   (digin.ts wayInRoom)
pub fn way_in_room(g: &mut crate::game::Game, room: RoomfightRoomId, from: V3, to: V3) -> bool {
    if !crate::roomfight::room_fight_in_room(g, Some(room), to) { return false; }
    let a = crate::roomfight::room_fight_free_near(g, from).unwrap_or(from);
    let path = match crate::scene_collision::nav_grid_find_path(g, a, to, Some(6000.0), None) { Some(p) if !p.is_empty() => p, _ => return false };
    for w in path {
        if !(crate::roomfight::room_fight_in_room(g, Some(room), w) || flat(w, to) < 0.8 || flat(w, from) < 0.8) { return false; }
    }
    true
}

/// braceInRoom: he can push this prop to the door from `q` without leaving his room   (digin.ts pushable)
pub fn pushable(g: &mut crate::game::Game, d: RoomfightDefId, room: RoomfightRoomId, pp: V3, q: V3, stand: V3) -> bool {
    if !(flat(q, pp) < 1.6) { return false; }
    if crate::scene_collision::solid_blocked(g, [q[0], 0.45, q[2]], [pp[0], 0.45, pp[2]], None) { return false; }
    let from = pos_of(g, g.tab.roomfight_def[d].gd);
    way_in_room(g, room, from, q) && way_in_room(g, room, q, stand)
}

/// braceInRoom: the best free floor of his room to push this prop toward the door from; None = none   (digin.ts pushSpot)
pub fn push_spot(g: &mut crate::game::Game, d: RoomfightDefId, room: RoomfightRoomId, e: crate::roomfight::Entry, pp: V3, stand: V3) -> Option<V3> {
    let (dx, dz) = (pp[0] - e.at[0], pp[2] - e.at[2]);
    let dl = js::or(m::hyp2(dx, dz), 1.0);
    let mut cand: Vec<(V3, f64)> = Vec::new();
    for r in [0.7, 1.0, 1.3] {
        for a in 0..12 {
            let a = a as f64;
            let (ux, uz) = (m::cos(a * m::PI / 6.0), m::sin(a * m::PI / 6.0));
            let q: V3 = [pp[0] + ux * r, 0.0, pp[2] + uz * r];
            let behind = (ux * dx + uz * dz) / dl;
            if behind < 0.25 || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || !crate::roomfight::room_fight_in_room(g, Some(room), q) { continue; }
            cand.push((q, behind - 0.3 * (r - 0.7)));
        }
    }
    js::sort_by(&mut cand, |a, b| b.1 - a.1);
    for k in cand.iter().take(6) {
        if pushable(g, d, room, pp, k.0, stand) { return Some(k.0); }
    }
    None
}

/// furniture for entry `idx` of his room: off to the prop. False = nothing to haul, no time, not that kind of door   (digin.ts startBrace)
pub fn start_brace(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &mut crate::digin::RoomSt, idx: f64) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    if r.wedged >= g.k.digin.wedges { return false; }
    let e = match entry_at(g, r.room, idx) { Some(e) => e, None => return false };
    if !wedgeable(g, e.door) || state_of(g).wedges.has(e.door) { return false; }
    if !(crate::enemysquads::squad_may_wedge(g, gd, e.door) || crate::laststand::last_stand_man(g, gd)) { return false; }
    if flat(heard_at(g, d, *m), e.at) < g.k.digin.close_m { return false; }
    let first = nearest_prop(g, d, e, None);
    let stand = crate::roomfight::room_fight_free_near(g, [e.at[0] - e.out[0] * 1.4, 0.0, e.at[2] - e.out[1] * 1.4]);
    let (mut p, stand) = match (first, stand) { (Some(p), Some(q)) => (p, q), _ => return false };
    if !crate::roomfight::room_fight_in_room(g, Some(r.room), stand) { return false; }
    let mut pp = prop_at(g, p);
    let (dx, dz) = (pp[0] - e.at[0], pp[2] - e.at[2]);
    let dl = js::or(m::hyp2(dx, dz), 1.0);
    let mut goal = match crate::roomfight::room_fight_free_near(g, [pp[0] + dx / dl * 0.7, 0.0, pp[2] + dz / dl * 0.7]) {
        Some(q) => Some(q),
        None => crate::roomfight::room_fight_free_near(g, pp),
    };
    // THE GOAL RULE: he pushes it from inside his own room, gets there without leaving it, and hauls it to the door inside it
    let mut passed: JsSet<PropId> = JsSet::new();
    loop {
        if let Some(q) = goal { if pushable(g, d, r.room, pp, q, stand) { break; } }
        if let Some(behind) = push_spot(g, d, r.room, e, pp, stand) {
            goal = Some(behind);
            g.stats.digin.brace_beside += 1;
            let text = format!("takes {} from the nearest floor of room {} behind it", prop_name(g, p), g.tab.roomfight_room[r.room].id);
            log(g, Some(gd), &text);
            break;
        }
        passed.add(p);
        g.stats.digin.brace_passed += 1;
        let text = format!("leaves {}: no floor of room {} to push it from", prop_name(g, p), g.tab.roomfight_room[r.room].id);
        log(g, Some(gd), &text);
        let q = if passed.len() < 3 { nearest_prop(g, d, e, Some(&passed)) } else { None };
        p = match q { Some(q) => q, None => return false };
        pp = prop_at(g, p);
        let (qx, qz) = (pp[0] - e.at[0], pp[2] - e.at[2]);
        let ql = js::or(m::hyp2(qx, qz), 1.0);
        goal = match crate::roomfight::room_fight_free_near(g, [pp[0] + qx / ql * 0.7, 0.0, pp[2] + qz / ql * 0.7]) {
            Some(q) => Some(q),
            None => crate::roomfight::room_fight_free_near(g, pp),
        };
    }
    let goal = match goal { Some(q) => q, None => return false };
    if crate::laststand::last_stand_man(g, gd) && !crate::roomfight::room_fight_in_room(g, Some(r.room), goal) { return false; }
    r.bracer = Some(gd);
    m.brace = idx;
    m.prop = Some(p);
    state_of(g).used.add(p);
    m.stand = stand;
    m.rest = [e.at[0] - e.out[0] * 0.6, 0.0, e.at[2] - e.out[1] * 0.6];
    m.phase = Phase::ToProp;
    g.stats.digin.hauls += 1;
    let speed = g.tune.guard_chase;
    move_(g, d, goal, speed);
    true
}

/// DOUBLE UP: alone in his room, the room of his side's with the most men still holding it, away from the noise   (digin.ts gatherTo)
pub fn gather_to(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man) -> Option<crate::digin::GatherTo> {
    let pos = pos_of(g, g.tab.roomfight_def[d].gd);
    let my_room = g.tab.roomfight_def[d].room;
    let defs = g.fight.defs.clone();
    if defs.iter().any(|x| *x != d && g.tab.roomfight_def[*x].room == my_room && !dead(g, g.tab.roomfight_def[*x].gd)) { return None; }
    let mut rooms: JsMap<RoomfightRoomId, Vec<RoomfightDefId>> = JsMap::new();
    for x in defs.iter().copied() {
        if x == d { continue; }
        let (k, xg, xmode) = match g.tab.roomfight_def[x].room { Some(k) => (k, g.tab.roomfight_def[x].gd, g.tab.roomfight_def[x].mode), None => continue };
        if Some(k) == my_room || g.tab.roomfight_room[k].entries.is_empty() { continue; }
        if !(!dead(g, xg) && xmode == crate::roomfight::DefMode::Hold && g.time - g.gd[xg].last_seen_t > 3.0) { continue; }
        if crate::roomfight::room_fight_in_room(g, Some(k), m.know) || entered(g, k) { continue; }
        rooms.get_or_insert_with(k, Vec::new).push(x);
    }
    let mine = flat(pos, m.know);
    let mut best: Option<(GatherTo, f64)> = None;
    let cell = g.col.nav.cell;
    let (know, shy_m) = (m.know, g.k.digin.gather_shy_m);
    for k in rooms.keys() {
        let men = rooms.get(k).unwrap();
        let station = g.tab.roomfight_def[men[0]].station;
        let far = flat(station, know);
        if men.len() < 2 && far < mine + 3.0 { continue; }
        let aside = [station[0] + (if pos[0] > station[0] { 1.2 } else { -1.2 }), 0.0, station[2]];
        let spot = match crate::roomfight::room_fight_free_near(g, aside) {
            Some(q) => Some(q),
            None => crate::roomfight::room_fight_free_near(g, station),
        };
        let spot = match spot { Some(q) => q, None => continue };
        if !crate::roomfight::room_fight_in_room(g, Some(k), spot) { continue; }
        let mut shy = |_: &mut Game, ix: f64, iz: f64| -> f64 {
            if m::hyp2((ix + 0.5) * cell - know[0], (iz + 0.5) * cell - know[2]) < shy_m + 2.0 { 20.0 } else { 1.0 }
        };
        let path = match crate::scene_collision::nav_grid_find_path(g, pos, spot, Some(6000.0), Some(&mut shy)) { Some(p) if !p.is_empty() => p, _ => continue };
        let walk = path_len(pos, &path);
        if walk > g.k.digin.gather_m || path.iter().any(|w| flat(*w, know) < shy_m) { continue; }
        let s = men.len() as f64 * 10.0 + far - walk * 0.2;
        if best.as_ref().map_or(true, |b| s > b.1) { best = Some((GatherTo { room: k, spot }, s)); }
    }
    best.map(|b| b.0)
}

/// how many other men of this room are at it on entry `i`   (digin.ts begin's `on`)
fn on_entry(g: &mut Game, me: RoomfightDefId, room: RoomfightRoomId, i: f64) -> usize {
    let mut n = 0;
    for x in g.fight.defs.clone() {
        let xg = g.tab.roomfight_def[x].gd;
        if x == me || dead(g, xg) || g.tab.roomfight_def[x].room != Some(room) { continue; }
        if phase_of(g, xg) == Phase::Idle { continue; }
        if state_of(g).men.get(xg).unwrap().entry == i { n += 1; }
    }
    n
}

/// he has thought: the room's row, his door, the word, the furniture or the door, or cover   (digin.ts begin)
pub fn begin(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man) {
    let gd = g.tab.roomfight_def[d].gd;
    let room = g.tab.roomfight_def[d].room.unwrap();
    let now = g.time;
    let first = !state_of(g).rooms.has(room);
    let mut r;
    if first {
        let [a, b] = expect_entries(g, room, m.know);
        r = RoomSt { room, expected: a, second: b, at: now, t0: m.know_t, men: Vec::new(), bracer: None, wedged: 0.0, cue_at: m::NAN, cues: JsSet::new(), entered_at: m::NAN, ready_at: m::NAN, callout: false };
        g.stats.digin.rooms += 1;
        if let Some(fix) = state_of(g).fix { if now - fix.t < 20.0 { cue(g, Some(&mut r), Cue::Radio); } }
    } else {
        r = row_of(g, room);
        if now - r.at > 4.0 {
            let [a, b] = expect_entries(g, room, m.know);
            r.expected = a; r.second = b; r.at = now;
        }
    }
    row_put(g, room, r.clone());
    if r.expected < 0.0 { m.phase = Phase::Idle; return; }
    if !r.men.contains(&gd) { r.men.push(gd); }
    m.room = Some(room);
    m.entry = if r.second >= 0.0 && on_entry(g, d, room, r.expected) > on_entry(g, d, room, r.second) { r.second } else { r.expected };
    m.brace = -1.0;
    m.spot = None;
    fear(g, d, *m);
    let e = entry_at(g, room, m.entry).unwrap();
    let expected = entry_at(g, room, r.expected).unwrap();
    let or_second = match entry_at(g, room, r.second) { Some(s) if r.second >= 0.0 => format!(" (or {})", door_name(g, s.door)), _ => String::new() };
    let text = format!("digs in: room {}, expects {}{}, takes {}", g.tab.roomfight_room[room].id, door_name(g, expected.door), or_second, door_name(g, e.door));
    log(g, Some(gd), &text);
    if first && !m.ordered {
        let line = one_of(g, &["they're coming this way. get ready!", "they're working through the rooms. watch that door!", "that's next door. get set, get set!"]);
        if !shout(g, gd, line, Some(&mut r)) {
            // out of the team's earshot: the room says it on its side's net instead
            let guide = crate::words::wg(g, &"copy. we'll be ready for them.", &"heard. they'll find this room ready.");
            let line = one_of(g, &["we hear it. getting ready for them here.", "copy. we're digging in.", guide]);
            crate::game::game_say(g, gd, line, Some(true), None, None);
            g.stats.digin.radio += 1;
            cue(g, Some(&mut r), Cue::Radio);
        }
    }
    row_put(g, room, r.clone());
    // furniture against the door they will come by: one man of the room, if there is time and something to haul
    if r.bracer.is_none() && m.entry == r.expected {
        let mut braced = false;
        let ex = r.expected;
        if holds(g, expected) { braced = start_brace(g, d, m, &mut r, ex); }
        else if away(g, d, *m, &r, expected) {
            for i in next_doors(g, &r) {
                let next = entry_at(g, room, i).unwrap();
                if holds(g, next) && start_brace(g, d, m, &mut r, i) {
                    let text = format!("instead: {}", door_name(g, next.door));
                    log(g, Some(gd), &text);
                    braced = true;
                    break;
                }
            }
        }
        row_put(g, room, r.clone());
        if braced {
            let line = one_of(g, &["block that door. get something against it!", "get that against the door!"]);
            shout(g, gd, line, Some(&mut r));
            row_put(g, room, r);
            return;
        }
    }
    shut_or_cover(g, d, m, &r);
}

/// a wedge HOLDS this entry's door, asked from the inside of the entry   (digin.ts holds)
pub fn holds(g: &mut crate::game::Game, e: crate::roomfight::Entry) -> bool {
    crate::doors::doors_wedge_holds(g, e.door, [e.at[0] - e.out[0] * 0.8, 0.0, e.at[2] - e.out[1] * 0.8])
}

/// would he have gone to wedge this entry but for the way its door swings: startBrace's first tests   (digin.ts wouldBrace)
pub fn would_brace(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt, e: crate::roomfight::Entry) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    if r.wedged >= g.k.digin.wedges || !wedgeable(g, e.door) || state_of(g).wedges.has(e.door) { return false; }
    if !(crate::enemysquads::squad_may_wedge(g, gd, e.door) || crate::laststand::last_stand_man(g, gd)) { return false; }
    if flat(heard_at(g, d, m), e.at) < g.k.digin.close_m { return false; }
    if nearest_prop(g, d, e, None).is_none() { return false; }
    match crate::roomfight::room_fight_free_near(g, [e.at[0] - e.out[0] * 1.4, 0.0, e.at[2] - e.out[1] * 1.4]) {
        Some(stand) => crate::roomfight::room_fight_in_room(g, Some(r.room), stand),
        None => false,
    }
}

/// a door passed over for the way it swings: counted once a door, said in the log. True = he would have wedged it   (digin.ts away)
pub fn away(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt, e: crate::roomfight::Entry) -> bool {
    if !would_brace(g, d, m, r, e) { return false; }
    if !state_of(g).away.has(e.door) {
        state_of(g).away.add(e.door);
        g.stats.digin.opens_away += 1;
        let text = format!("{} opens away from him: nothing to wedge", door_name(g, e.door));
        let gd = g.tab.roomfight_def[d].gd;
        log(g, Some(gd), &text);
    }
    true
}

/// the room's doors after the one they are expected by: the second-likeliest first, then as authored   (digin.ts nextDoors)
pub fn next_doors(g: &mut crate::game::Game, r: &crate::digin::RoomSt) -> Vec<f64> {
    let n = g.tab.roomfight_room[r.room].entries.len();
    let mut v: Vec<f64> = (0..n).map(|i| i as f64).filter(|i| *i != r.expected).collect();
    let second = r.second;
    js::sort_by(&mut v, |a, b| (if *a == second { -1.0 } else { 0.0 }) - (if *b == second { -1.0 } else { 0.0 }));
    v
}

/// his door standing open toward them: he goes to it and pulls it to, else to cover   (digin.ts shutOrCover)
pub fn shut_or_cover(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &crate::digin::RoomSt) {
    let gd = g.tab.roomfight_def[d].gd;
    let e = entry_at(g, r.room, m.entry).unwrap();
    let door = e.door;
    if !crate::doors::door_latched(g, door) && m::abs(crate::doors::door_angle(g, door)) > 0.05 && !crate::doors::door_lock_broken(g, door) && flat(heard_at(g, d, *m), e.at) > g.k.digin.close_m {
        let stand = crate::roomfight::room_fight_free_near(g, [e.at[0] - e.out[0] * 1.2, 0.0, e.at[2] - e.out[1] * 1.2]);
        if let Some(stand) = stand {
            if crate::roomfight::room_fight_in_room(g, Some(r.room), stand) && flat(pos_of(g, gd), stand) < 12.0 {
                m.brace = -1.0; m.prop = None; m.also = Some(-1.0); m.stand = stand; m.phase = Phase::ToDoor;
                let speed = g.tune.guard_chase;
                move_(g, d, stand, speed);
                let text = format!("goes to shut {}", door_name(g, door));
                log(g, Some(gd), &text);
                return;
            }
        }
    }
    to_cover(g, d, m, r, None);
}

/// to the spot that sees his door, off its line if he is one to; `or_stay` false: only if there is a spot to walk to   (digin.ts toCover)
pub fn to_cover(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &crate::digin::RoomSt, or_stay: Option<bool>) -> bool {
    let or_stay = or_stay.unwrap_or(true);
    let gd = g.tab.roomfight_def[d].gd;
    let want_deep = wants_deep(g, d, r);
    let dp = if want_deep { Some(cover_spot(g, d, *m, r, true)) } else { None };
    let deep = match dp { Some(x) => x.spot.is_some() || x.here, None => false };
    let mut p = match dp { Some(x) => x.spot, None => None };
    if !deep { p = cover_spot(g, d, *m, r, false).spot; }
    if p.is_none() && !or_stay { return false; }
    m.tried_deep = want_deep;
    m.deep = deep;
    m.look = if deep { dp.unwrap().look } else { None };
    if deep {
        g.stats.digin.deep += 1;
        log(g, Some(gd), "keeps off the door's line");
    }
    m.brace = -1.0;
    m.prop = None;
    fear(g, d, *m);
    if let Some(p) = p {
        m.spot = Some(p);
        m.phase = Phase::ToCover;
        g.stats.digin.moved += 1;
        let speed = g.tune.guard_chase;
        move_(g, d, p, speed);
        let door = entry_at(g, r.room, m.entry).unwrap().door;
        let text = format!("to cover on {}", door_name(g, door));
        log(g, Some(gd), &text);
    } else {
        g.stats.digin.stayed += 1;
        settle(g, d, m);
    }
    true
}

/// he is where he will fight from, his eye on his door   (digin.ts settle)
pub fn settle(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man) {
    let gd = g.tab.roomfight_def[d].gd;
    m.phase = Phase::Set; m.set_at = g.time; m.also = Some(-1.0); m.also_n = Some(0.0); m.spot = Some(pos_of(g, gd));
    if !m.dug_once { m.dug_once = true; g.stats.digin.dug += 1; }
    g.tab.roomfight_def[d].want = m.entry;
    g.tab.roomfight_def[d].watch = m.entry;
    fear(g, d, *m);
    let on = match m.room.and_then(|k| entry_at(g, k, m.entry)) { Some(e) => door_name(g, e.door), None => String::from("?") };
    let text = format!("set on {}", on);
    log(g, Some(gd), &text);
}

/// a man the dig-in has not touched yet   (digin.ts newMan)
pub fn new_man() -> crate::digin::Man {
    Man { far: false, owe: 0.0, haul_by: 0.0, deep: false, look: None, tried_deep: false, gathered: false, gather_to: None, dug_once: false, ordered: false, phase: Phase::Idle, until: 0.0, know: [0.0, 0.0, 0.0], know_t: -100.0, room: None, entry: -1.0, brace: -1.0, prop: None, stand: [0.0, 0.0, 0.0], rest: [0.0, 0.0, 0.0], scrape_t: -100.0, set_at: -100.0, spot: None, kept: false, eyes_t: None, lane_t: None, also: None, also_n: None, home_t: None, home_d: None, home_at: None, hand_t: None, pull_t: None, pushes: None, pull_a: None, stalls: None }
}

/// he lets go of what he was at; furniture left where it stands is somebody else's to haul   (digin.ts drop)
pub fn drop(g: &mut crate::game::Game, m: &mut crate::digin::Man, gd: Option<GuardId>) {
    if let Some(p) = m.prop {
        crate::props::prop_set_vx(g, p, 0.0);
        crate::props::prop_set_vz(g, p, 0.0);
        state_of(g).used.delete(p);
    }
    if let (Some(k), Some(who)) = (m.room, gd) {
        if let Some(r) = state_of(g).rooms.get_mut(k) { if r.bracer == Some(who) { r.bracer = None; } }
    }
    m.prop = None; m.brace = -1.0; m.also = Some(-1.0); m.also_n = Some(0.0); m.phase = Phase::Idle;
}

/// the team in his eyes while his hands were on the furniture or the door: the gun has to come up first   (digin.ts caught)
pub fn caught(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, what: &str) {
    let range = g.k.digin.caught;
    m.owe = pick(g, range);
    g.stats.digin.caught += 1;
    g.stats.digin.aborted += 1;
    let text = format!("caught {}", what);
    let gd = g.tab.roomfight_def[d].gd;
    log(g, Some(gd), &text);
}

/// one door wedged: the next likeliest while the room has furniture, doors and its quota left, else to cover   (digin.ts afterWedge)
pub fn after_wedge(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &mut crate::digin::RoomSt) {
    let gd = g.tab.roomfight_def[d].gd;
    m.prop = None;
    m.brace = -1.0;
    if r.wedged < g.k.digin.wedges {
        let mut doors = vec![r.expected];
        doors.extend(next_doors(g, r));
        for i in doors {
            let e = entry_at(g, r.room, i).unwrap();
            if !holds(g, e) { away(g, d, *m, r, e); continue; }
            if start_brace(g, d, m, r, i) {
                let text = format!("next: {}", door_name(g, e.door));
                log(g, Some(gd), &text);
                return;
            }
        }
    }
    shut_or_cover(g, d, m, r);
}

/// HAULHOME: stopped at his stand with the thing a hand short of the door and his clock running, his hands bring it the rest of the way   (digin.ts bringsHome)
pub fn brings_home(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &mut crate::digin::RoomSt, e: Option<crate::roomfight::Entry>, p: PropId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    let (pos, pp) = (pos_of(g, gd), prop_at(g, p));
    let e = match e { Some(e) => e, None => return false };
    if g.time >= m.haul_by || flat(pos, m.stand) >= HAUL_NEAR || flat(pp, pos) > HOME_REACH || !wedgeable(g, e.door) || !holds(g, e) { return false; }
    let dist = flat(pp, m.rest);
    let now = g.time;
    if m.home_t != Some(m.haul_by) {
        m.home_t = Some(m.haul_by); m.home_d = Some(dist); m.home_at = Some(now);
        log(g, Some(gd), "brings it the last of the way");
    } else if now - m.home_at.unwrap_or(now) >= 0.5 {
        if dist > m.home_d.unwrap_or(dist) - 0.03 { return false; }
        m.home_d = Some(dist); m.home_at = Some(now);
    }
    haul(g, d, m, Some(r));
    true
}

/// hauling: the prop is kept a pace ahead of him on his way to the door, then set down against it   (digin.ts haul)
pub fn haul(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: Option<&mut crate::digin::RoomSt>) {
    let gd = g.tab.roomfight_def[d].gd;
    let p = match m.prop { Some(p) if g.props.props.contains(&p) => p, _ => return };
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    if g.time < m.haul_by {
        let w = &mut g.tab.roomfight_def[d];
        w.move_t = m::min(w.move_t, 5.0); w.stall_t = 0.0; w.stall_at[0] = pos[0]; w.stall_at[2] = pos[2];
    }
    let pp = prop_at(g, p);
    let (mut tx, mut tz) = (m.stand[0] - pos[0], m.stand[2] - pos[2]);
    let tl = m::hyp2(tx, tz);
    let want: V3;
    if tl < HAUL_NEAR || flat(pp, m.rest) < 0.8 { want = m.rest; }
    else { tx /= tl; tz /= tl; want = [pos[0] + tx * 0.75, 0.0, pos[2] + tz * 0.75]; }
    let (mut vx, mut vz) = ((want[0] - pp[0]) / 0.2, (want[2] - pp[2]) / 0.2);
    let vl = m::hyp2(vx, vz);
    let cap = g.k.digin.haul_speed / m::max(0.1, g.k.digin.pace) * 1.6;
    if vl > cap { vx *= cap / vl; vz *= cap / vl; }
    if flat(pp, pos) > 2.2 { vx = 0.0; vz = 0.0; }                     // it has caught on something behind him: left where it is
    crate::props::prop_set_vx(g, p, vx);
    crate::props::prop_set_vz(g, p, vz);
    crate::props::prop_wake(g, p);
    let id = g.ch[c].id;
    crate::props::prop_set_last_who(g, p, id);
    if m::hyp2(vx, vz) > 0.3 && g.time - m.scrape_t > g.k.digin.scrape_every {
        m.scrape_t = g.time;
        scrape(g, gd, *m, r, None);
    }
}

/// a cue to every room dug in with this door   (digin.ts:801, 813)
fn cue_door(g: &mut Game, door: DoorId, kind: Cue) {
    for k in state_of(g).rooms.keys() {
        if !g.tab.roomfight_room[k].entries.iter().any(|e| e.door == door) { continue; }
        let mut r = row_of(g, k);
        cue(g, Some(&mut r), kind);
        row_put(g, k, r);
    }
}
/// a cue to every room dug in   (digin.ts:685)
fn cue_all(g: &mut Game, kind: Cue) {
    for k in state_of(g).rooms.keys() {
        let mut r = row_of(g, k);
        cue(g, Some(&mut r), kind);
        row_put(g, k, r);
    }
}
/// a haul with its room's row checked out and written back
fn haul_on(g: &mut Game, d: RoomfightDefId, m: &mut Man, rk: Option<RoomfightRoomId>) {
    match rk {
        Some(k) => { let mut r = row_of(g, k); haul(g, d, m, Some(&mut r)); row_put(g, k, r); }
        None => haul(g, d, m, None),
    }
}

/// roomfight.ts tickDef, every frame for every defender it runs   (digin.ts digInTick)
pub fn dig_in_tick(g: &mut crate::game::Game, def: RoomfightDefId, _dt: f64) {
    let gd = g.tab.roomfight_def[def].gd;
    if dead(g, gd) || g.rescue.active { return; }                     // the hostage rescue has its own script for these men
    let now = g.time;
    if state_of(g).frame_t != now { state_of(g).frame_t = now; watch(g); }
    let mut m = *state_of(g).men.get_or_insert_with(gd, new_man);
    tick_man(g, def, &mut m);
    state_of(g).men.set(gd, m);                                       // his row, checked out for the frame: the one write back
}

/// digInTick's body, with his row in `m`
fn tick_man(g: &mut Game, d: RoomfightDefId, m: &mut Man) {
    use crate::game::GuardState;
    use crate::roomfight::DefMode;
    let gd = g.tab.roomfight_def[d].gd;
    let now = g.time;
    let rk = match m.room { Some(k) if state_of(g).rooms.has(k) => Some(k), _ => None };
    if m.owe > 0.0 { g.gd[gd].react_t += m.owe; m.owe = 0.0; }
    let last_seen = g.gd[gd].last_seen_t;
    if now - last_seen >= 0.25 { m.eyes_t = None; } else if m.eyes_t.is_none() { m.eyes_t = Some(now); }
    let mode = g.tab.roomfight_def[d].mode;
    let my_room = g.tab.roomfight_def[d].room;
    if mode == DefMode::Free { if m.phase != Phase::Idle { drop(g, m, Some(gd)); } return; }
    if mode == DefMode::Move {
        if m.phase == Phase::Haul {
            // HAULHOME: a man bringing the last hand in is planted: a leg another rule gives him then is not a haul
            if m.home_t == Some(m.haul_by) { log(g, Some(gd), "was moved off his furniture"); drop(g, m, Some(gd)); }
            else { haul_on(g, d, m, rk); }
        }
        return;
    }
    let dazzled = now < g.gd[gd].dazzled_until;
    let contact = now - last_seen < 3.0;
    // the word on the net: the first of a room to have eyes on them, once he is past his own start
    if !crate::enemysquads::squads_live(g) && !dazzled && !state_of(g).called.has(gd) && now - last_seen < 0.25 && g.gd[gd].react_t <= 0.0
        && (g.gd[gd].state == GuardState::Alert || g.gd[gd].state == GuardState::Search) && now - m.eyes_t.unwrap_or(now) >= g.k.digin.noticed_secs
        && state_of(g).fix.map_or(true, |x| now - x.t > 10.0) {
        let pos = pos_of(g, gd);
        let shout_m = g.k.digin.shout_m;
        let heard = heard_by(g, pos, shout_m).is_some();              // (he says it out loud into his radio: a cue only to a team within earshot of HIM)
        state_of(g).called.add(gd);
        state_of(g).fix = Some(StFix { at: pos, t: now, room: my_room, heard });
        g.stats.digin.radio += 1;
        let pool = crate::words::w(g, &["contact! they're here. hold your rooms!", "they're on us. watch your doors!"], &["contact! they're inside. hold your rooms!", "they're in here. watch your doors!"]);
        let line = one_of(g, &pool);
        crate::game::game_say(g, gd, line, Some(true), None, None);
        let text = format!("calls contact on the net{}", if heard { "" } else { " (out of the team's earshot)" });
        log(g, Some(gd), &text);
        cue_all(g, Cue::Radio);
    }
    if dazzled { return; }
    if m.phase != Phase::Idle && m.phase != Phase::Think && m.phase != Phase::ToRoom && (rk.is_none() || my_room != m.room) { drop(g, m, Some(gd)); return; }   // he is somewhere else now: the room fight's man again
    let reach = g.k.digin.reach_m;
    if m.phase != Phase::Idle && m.far && reach > 0.0 { g.tab.roomfight_def[d].rally_at = m::NAN; }   // held here: no rally clock left running to send him the frame he is let go
    match m.phase {
        Phase::Idle => {
            let room = match my_room { Some(k) if !contact && !g.tab.roomfight_room[k].entries.is_empty() => k, _ => return };
            if entered(g, room) { return; }                           // the team has been in his room: the room fight's man from there on
            let mut know: Option<V3> = None;
            let mut t = -100.0;
            let (noise_at, noise_out, noise_t) = { let x = &g.tab.roomfight_def[d]; (x.noise_at, x.noise_out, x.noise_t) };
            let noise = match noise_at { Some(at) if noise_out && now - noise_t < 6.0 && noise_t > m.know_t => Some(at), _ => None };
            if let Some(at) = noise { know = Some(at); t = noise_t; }
            else if let Some(fix) = state_of(g).fix {
                if fix.room != my_room && fix.t > m.know_t && now - fix.t > 0.8 && now - fix.t < 8.0 { know = Some(fix.at); t = fix.t; }
            }
            let know = match know { Some(k) => k, None => return };
            if crate::gunroles::gun_keeps_post(g, gd) { m.know_t = t; return; }   // the belt-fed gun stays on its line: heard, and let lie
            let think = g.k.digin.think;
            m.know = know; m.know_t = t; m.phase = Phase::Think; m.until = now + paced(g, think);
            g.stats.digin.heard += 1;
            m.far = if reach > 0.0 { let pos = pos_of(g, gd); way_to(g, pos, know) > reach } else { false };   // too far to join that fight in time: his room is his job
            if m.far { g.stats.digin.far += 1; }
        }
        Phase::Think => {
            if contact { m.phase = Phase::Idle; g.stats.digin.aborted += 1; return; }
            let room = match my_room { Some(k) => k, None => { m.phase = Phase::Idle; return } };
            let mut inside = entered(g, room);
            if !inside { for l in listeners(g) { if crate::roomfight::room_fight_in_room(g, my_room, l.head) { inside = true; break; } } }
            if inside { m.phase = Phase::Idle; return; }              // they are in his room already: the room fight's man
            if now < m.until { return; }
            if crate::enemysquads::squad_defers(g, gd) { return; }    // his squad has heard and not yet said what it does
            let far = m.far && reach > 0.0;
            if !far && crate::pressure::press_wants(g, d) { return; } // on his beat toward a press, or in one: the press has him first
            if !far && !g.tab.roomfight_def[d].rally_at.is_nan() && g.tab.roomfight_def[d].rally_go && now < g.tab.roomfight_def[d].rally_at + 0.5 { return; }   // he means to go to the fight next door: the rally has him first
            begin(g, d, m);
        }
        Phase::ToRoom => {                                            // planted: with the others, or stopped on the way
            if my_room.is_some() && my_room == m.gather_to && !contact {
                m.gather_to = None; m.phase = Phase::Think; m.until = now + 0.4;
                let text = format!("is in with room {}", g.tab.roomfight_room[my_room.unwrap()].id);
                log(g, Some(gd), &text);
            } else {
                m.gather_to = None; m.phase = Phase::Idle;
                g.stats.digin.gather_lost += 1;
                log(g, Some(gd), "did not make it to the others");
            }
        }
        Phase::ToProp => {                                            // planted: at the prop, or stopped short
            let k = rk.unwrap();                                      // (the test above the match: no row, no phase past 'toRoom')
            let p = match m.prop { Some(p) if !contact && g.props.props.contains(&p) => p, _ => { g.stats.digin.aborted += 1; drop(g, m, Some(gd)); return } };
            let pos = pos_of(g, gd);
            if flat(pos, prop_at(g, p)) < 1.7 {
                let sp = g.k.digin.haul_speed / m::max(0.1, g.k.digin.pace);
                m.phase = Phase::Haul; m.scrape_t = -100.0; m.haul_by = now + flat(pos, m.stand) / sp * 1.6 + 3.0;
                let stand = m.stand;
                move_(g, d, stand, sp);
                return;
            }
            let r = row_of(g, k);
            shut_or_cover(g, d, m, &r);
        }
        Phase::Haul => {                                              // planted: at the door with it, or stopped short
            let k = rk.unwrap();
            let p = match m.prop { Some(p) if g.props.props.contains(&p) => p, _ => { drop(g, m, Some(gd)); return } };
            crate::props::prop_set_vx(g, p, 0.0);
            crate::props::prop_set_vz(g, p, 0.0);
            let e = entry_at(g, k, m.brace);
            if contact { caught(g, d, m, "hauling"); drop(g, m, Some(gd)); return; }
            if let Some(e) = e {
                if flat(prop_at(g, p), m.rest) < 1.3 && wedgeable(g, e.door) && holds(g, e) {
                    if !crate::doors::door_latched(g, e.door) { hand_on(g, d, m, e.door); return; }
                    let secs = g.k.digin.wedge_secs;
                    m.phase = Phase::Wedge; m.until = now + paced(g, secs); m.scrape_t = now;
                    return;
                }
            }
            let mut r = row_of(g, k);
            let home = brings_home(g, d, m, &mut r, e, p);            // a hand short with his clock running: he brings it the rest of the way
            row_put(g, k, r.clone());
            if home { return; }
            log(g, Some(gd), "the furniture did not make it to the door");
            shut_or_cover(g, d, m, &r);
        }
        Phase::ToDoor => {                                            // planted: at his open door, or stopped short
            let k = rk.unwrap();
            let i = door_of(g, *m);
            let e = entry_at(g, k, i).unwrap();
            if contact { g.stats.digin.aborted += 1; settle(g, d, m); return; }
            if flat(pos_of(g, gd), e.at) < 2.2 && !crate::doors::door_latched(g, e.door) && !crate::doors::door_lock_broken(g, e.door) {
                let text = format!("pulls {} to", door_name(g, e.door));
                log(g, Some(gd), &text);
                hand_on(g, d, m, e.door);
                return;
            }
            let r = row_of(g, k);
            to_cover(g, d, m, &r, None);
        }
        Phase::Shut => {                                              // a hand on the leaf until it latches
            let k = rk.unwrap();
            let i = door_of(g, *m);
            let e = entry_at(g, k, i).unwrap();
            if contact { caught(g, d, m, "at the door"); drop(g, m, Some(gd)); return; }
            let r = row_of(g, k);
            if flat(pos_of(g, gd), e.at) > 3.0 { log(g, Some(gd), "was moved off the door"); to_cover(g, d, m, &r, None); return; }
            if hand_waits(g, d, m, e.door) { return; }
            if m.brace >= 0.0 && m.prop.is_some() && crate::doors::door_latched(g, e.door) && wedgeable(g, e.door) && holds(g, e) {
                let secs = g.k.digin.wedge_secs;
                m.phase = Phase::Wedge; m.until = now + paced(g, secs); m.scrape_t = now;
                return;
            }
            to_cover(g, d, m, &r, None);
        }
        Phase::Wedge => {                                             // working the furniture under the door: heard, and no gun in his shoulder
            let k = rk.unwrap();
            let e = entry_at(g, k, m.brace);
            if contact { caught(g, d, m, "wedging the door"); drop(g, m, Some(gd)); return; }
            let mut r = row_of(g, k);
            let good = match (e, m.prop) {
                (Some(e), Some(p)) => g.props.props.contains(&p) && crate::doors::door_latched(g, e.door) && wedgeable(g, e.door) && holds(g, e),
                _ => false,
            };
            if !good { log(g, Some(gd), "the door came open under his hands"); to_cover(g, d, m, &r, None); return; }
            let mut e = e.unwrap();
            if flat(pos_of(g, gd), e.at) > 3.0 { log(g, Some(gd), "was moved off the door"); to_cover(g, d, m, &r, None); return; }
            if now - m.scrape_t > g.k.digin.scrape_every * 1.6 {
                m.scrape_t = now;
                scrape(g, gd, *m, Some(&mut r), Some(true));
                row_put(g, k, r.clone());
            }
            if now < m.until { return; }
            wedge(g, d, *m, &mut r, &mut e);
            after_wedge(g, d, m, &mut r);
            row_put(g, k, r);
        }
        Phase::ToCover => {                                           // planted: there, or stopped by what he saw
            if contact { g.stats.digin.aborted += 1; }
            settle(g, d, m);
        }
        Phase::Set => set_tick(g, d, m, rk.unwrap(), contact),
    }
}

/// the phase 'set' of digInTick   (digin.ts:771)
fn set_tick(g: &mut Game, d: RoomfightDefId, m: &mut Man, k: RoomfightRoomId, contact: bool) {
    let (gd, now) = (g.tab.roomfight_def[d].gd, g.time);
    let r = row_of(g, k);
    if now - r.entered_at > 1.0 { m.phase = Phase::Idle; return; }    // they are in: his eye is his own again
    let last = m::max_n(&[g.tab.roomfight_def[d].noise_t, m.know_t, g.gd[gd].last_seen_t]);
    if now - last > g.k.digin.hold_secs && !crate::laststand::last_stand_man(g, gd) {   // (the last stand stays set)
        m.phase = Phase::Idle;
        log(g, Some(gd), "stands down");
        return;
    }
    fear(g, d, *m);
    if !contact && r.entered_at.is_nan() && now >= m.lane_t.unwrap_or(0.0) && now - m.set_at > g.k.digin.lane_secs {
        let pos = pos_of(g, gd);
        let secs = g.k.digin.lane_secs;
        if !crate::lanes::lane_covers(g, gd, pos) { m.lane_t = Some(now + 0.5); }   // (asked twice a second, not every frame)
        else if to_cover(g, d, m, &r, Some(false)) {                  // set in a lane that has cost them: another spot of his room, out of the line
            m.lane_t = Some(now + secs);
            g.stats.digin.lane_moved += 1;
            log(g, Some(gd), "moves out of their line");
            return;
        } else { m.lane_t = Some(now + 4.0 * secs); g.stats.digin.lane_kept += 1; }   // none: he stays as he is
    }
    if r.second < 0.0 || r.men.len() > 1 {                            // nothing else to watch for: his eye stays on his door
        let w = &mut g.tab.roomfight_def[d];
        w.glance_next = m::max(w.glance_next, now + 3.0);
    }
    // the noise has moved: another door is now the likely one
    let (noise_at, noise_out, noise_t) = { let x = &g.tab.roomfight_def[d]; (x.noise_at, x.noise_out, x.noise_t) };
    let at = match noise_at { Some(at) if !contact && noise_out && noise_t > m.know_t + 4.0 && now - noise_t < 2.0 && now - m.set_at > g.k.digin.redig_secs => at, _ => return };
    m.know = at;
    m.know_t = noise_t;
    let was = r.expected;
    let [a, b] = expect_entries(g, k, at);
    let live = state_of(g).rooms.get_mut(k).unwrap();
    live.expected = a; live.second = b; live.at = now;
    if a >= 0.0 && a != was && m.entry == was {
        let my_room = g.tab.roomfight_def[d].room;
        let mut taken = false;
        for x in g.fight.defs.clone() {
            let xg = g.tab.roomfight_def[x].gd;
            if x == d || dead(g, xg) || g.tab.roomfight_def[x].room != my_room { continue; }
            // (`?.entry === a && ?.phase !== 'idle'`: a man with no row has no entry to match)
            if let Some(o) = state_of(g).men.get(xg) { if o.entry == a && o.phase != Phase::Idle { taken = true; break; } }
        }
        if !taken {
            m.entry = a;
            let door = entry_at(g, k, a).unwrap().door;
            let text = format!("shifts to {}", door_name(g, door));
            log(g, Some(gd), &text);
            let r = row_of(g, k);
            to_cover(g, d, m, &r, None);
        }
    }
}

fn seam_add_know_to_ready(g: &mut Game, secs: f64) { todo!("needs: DiginStats.know_to_ready is i32, the TypeScript sums seconds") }
fn seam_add_know_to_entry(g: &mut Game, secs: f64) { todo!("needs: DiginStats.know_to_entry is i32, the TypeScript sums seconds") }

/// once a frame: a room all set, the team's first step into a room that dug in, and wedges that have been booted   (digin.ts watch)
pub fn watch(g: &mut crate::game::Game) {
    let now = g.time;
    for d in state_of(g).wedges.keys() {                              // the wedge found the hard way: the door that would not open IS the cue
        if state_of(g).wedges.get(d).unwrap().kicked || !crate::doors::door_lock_broken(g, d) { continue; }
        state_of(g).wedges.get_mut(d).unwrap().kicked = true;
        g.stats.digin.kicked += 1;
        cue_door(g, d, Cue::Kick);
    }
    for d in state_of(g).wedges.keys() {                              // a hand on the handle from outside: the wedge found the quiet way
        let w = state_of(g).wedges.get(d).unwrap();
        if w.tried || w.kicked || !crate::doors::door_locked(g, d) { continue; }
        let (c, key) = (g.dr[d].frame_centre, g.dr[d].key_side);
        let mut by: Option<ListenersItem> = None;
        for l in listeners(g) {
            if flat(l.head, c) < 1.7 && crate::doors::doors_side(g, d, l.head) == key && m::hyp2(l.vel[0], l.vel[2]) < 0.4 { by = Some(l); break; }
        }
        let by = match by { Some(l) => l, None => { state_of(g).wedges.get_mut(d).unwrap().by_t = m::NAN; continue } };
        let w = state_of(g).wedges.get_mut(d).unwrap();
        if w.by_t.is_nan() { w.by_t = now; }
        let by_t = w.by_t;
        if now - by_t < g.k.digin.try_secs { continue; }
        state_of(g).wedges.get_mut(d).unwrap().tried = true;
        g.stats.digin.tried += 1;
        let dn = crate::orders::door_words(g, d);                     // the line NAMES its door
        if m::abs(crate::doors::door_angle(g, d)) > 0.05 {            // a leaf standing open is no door to call wedged: the line is not said, his die is still drawn
            if by.gd.is_some() { one_of(g, &["", ""]); }
        } else if let Some(who) = by.gd {
            let lines = [format!("the {} is blocked. something heavy behind it.", dn), format!("the {} is wedged from inside. they know we're here.", dn)];
            let line = one_of(g, &lines);
            crate::game::game_say(g, who, &line, Some(true), None, None);
        } else {
            let text = format!("the {} is wedged from inside", dn);
            crate::game::game_msg(g, &text);
        }
        let text = format!("tries {}: wedged", door_name(g, d));
        log(g, by.gd, &text);
        cue_door(g, d, Cue::Tried);
    }
    for k in state_of(g).rooms.keys() {
        let mut r = row_of(g, k);
        if !r.entered_at.is_nan() { continue; }
        let mut home: Vec<GuardId> = Vec::new();
        for x in g.fight.defs.iter() {
            let (xg, xr) = (g.tab.roomfight_def[*x].gd, g.tab.roomfight_def[*x].room);
            if !dead(g, xg) && xr == Some(r.room) { home.push(xg); }
        }
        let mut sets = 0;
        for x in home.iter() { if phase_of(g, *x) == Phase::Set { sets += 1; } }
        if r.ready_at.is_nan() && !home.is_empty() && sets == home.len() {
            r.ready_at = now;
            g.stats.digin.ready_n += 1;
            seam_add_know_to_ready(g, now - r.t0);
            row_put(g, k, r.clone());
            let text = format!("room {} all set, {:.1} s after the first word of them", g.tab.roomfight_room[r.room].id, now - r.t0);
            log(g, None, &text);
        }
        let mut inside = false;
        for l in listeners(g) { if crate::roomfight::room_fight_in_room(g, Some(r.room), l.head) { inside = true; break; } }
        if !inside { continue; }
        r.entered_at = now;
        g.stats.digin.entered += 1;
        seam_add_know_to_entry(g, now - r.t0);
        if !r.cue_at.is_nan() { g.stats.digin.cue_first += 1; }
        if r.cues.has(Cue::Radio) { g.stats.digin.cue_radio += 1; }
        if r.cues.has(Cue::Shout) { g.stats.digin.cue_shout += 1; }
        if r.cues.has(Cue::Scrape) { g.stats.digin.cue_scrape += 1; }
        if r.cues.has(Cue::Tried) { g.stats.digin.cue_tried += 1; }
        if r.cues.has(Cue::Kick) { g.stats.digin.cue_kick += 1; }
        let mut busy = 0;
        for x in home.iter() {
            if matches!(phase_of(g, *x), Phase::ToProp | Phase::Haul | Phase::ToDoor | Phase::Shut | Phase::Wedge | Phase::Think | Phase::ToCover) { busy += 1; }
        }
        if !r.ready_at.is_nan() { g.stats.digin.ready_first += 1; } else if busy > 0 { g.stats.digin.half_ready += 1; }
        for e in g.tab.roomfight_room[r.room].entries.clone() {
            if state_of(g).wedges.has(e.door) && !crate::doors::door_lock_broken(g, e.door) && crate::doors::door_locked(g, e.door) { g.stats.digin.wedged_at_entry += 1; }
        }
        row_put(g, k, r.clone());
        let cues: Vec<&str> = r.cues.iter().map(|c| c.as_str()).collect();
        let cues = if cues.is_empty() { String::from("none") } else { cues.join("+") };
        let cue_first = if r.cue_at.is_nan() { String::new() } else { format!(" (first {:.1} s before)", now - r.cue_at) };
        let text = format!("team into room {} {:.1} s after its first word of them: cues {}{}, {}/{} set, {} still busy, {} wedged", g.tab.roomfight_room[r.room].id, now - r.t0, cues, cue_first, sets, home.len(), busy, r.wedged);
        log(g, None, &text);
    }
}

/// roomfight.ts evidence: a door of his room booted in; a man set on a door his room wedged was waiting for exactly that   (digin.ts digInBraced)
pub fn dig_in_braced(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool {
    if !dig_in_set_on(g, gd, d) { return false; }
    g.stats.digin.steady += 1;
    true
}

/// the same question with nothing counted: `gd` is set on `d`, a door his room wedged   (digin.ts digInSetOn)
pub fn dig_in_set_on(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool {
    let s = match g.m.digin.states.as_ref() { Some(s) => s, None => return false };
    if !s.wedges.has(d) { return false; }
    match s.men.get(gd) { Some(x) => x.phase == Phase::Set, None => false }
}

/// laststand.ts: this man is TOLD to dig in where he holds; false = already at it, no room with a door, or the team has been in it   (digin.ts digInOrder)
pub fn dig_in_order(g: &mut crate::game::Game, def: RoomfightDefId, know: V3, beat: f64) -> bool {
    let (gd, mode, my_room) = { let x = &g.tab.roomfight_def[def]; (x.gd, x.mode, x.room) };
    let phase = state_of(g).men.get_or_insert_with(gd, new_man).phase;
    let room = match my_room { Some(k) if phase == Phase::Idle && mode == crate::roomfight::DefMode::Hold && !g.tab.roomfight_room[k].entries.is_empty() => k, _ => return false };
    if entered(g, room) { return false; }
    let now = g.time;
    let w = state_of(g).men.get_mut(gd).unwrap();
    w.know = know; w.know_t = now; w.far = false; w.gathered = true; w.ordered = true; w.phase = Phase::Think; w.until = now + beat;
    g.stats.digin.heard += 1;
    true
}

/// laststand.ts: the stand is called off: a man it ORDERED to dig in lets go of whatever he was at   (digin.ts digInRelease)
pub fn dig_in_release(g: &mut crate::game::Game, gd: GuardId) {
    let mut m = match g.m.digin.states.as_ref().and_then(|s| s.men.get(gd)) { Some(x) if x.ordered => *x, _ => return };
    m.ordered = false;
    if m.phase != Phase::Idle { drop(g, &mut m, Some(gd)); }
    state_of(g).men.set(gd, m);
}

/// laststand.ts: where his dig-in stands ('idle' = not at it)   (digin.ts digInPhase)
pub fn dig_in_phase(g: &mut crate::game::Game, gd: GuardId) -> String {
    let phase = g.m.digin.states.as_ref().and_then(|s| s.men.get(gd)).map_or(Phase::Idle, |x| x.phase);
    String::from(phase.as_str())
}

/// laststand.ts: the team has been in this room since somebody dug in there   (digin.ts digInEntered)
pub fn dig_in_entered(g: &mut crate::game::Game, room: RoomfightRoomId) -> bool {
    match g.m.digin.states.as_ref().and_then(|s| s.rooms.get(room)) { Some(r) => !r.entered_at.is_nan(), None => false }
}

/// laststand.ts: this room's doors that stand wedged (not yet booted)   (digin.ts digInWedged)
pub fn dig_in_wedged(g: &mut crate::game::Game, room: RoomfightRoomId) -> Vec<crate::digin::DigInWedgedItem> {
    let s = match g.m.digin.states.as_ref() { Some(s) => s, None => return Vec::new() };
    let mut out = Vec::new();
    for e in g.tab.roomfight_room[room].entries.iter() {
        if let Some(w) = s.wedges.get(e.door) { if !w.kicked { out.push(DigInWedgedItem { door: e.door, at: e.at }); } }
    }
    out
}

/// roomfight.ts applyWatch: a man set off the door's line rests his gun on the floor they must cross (writes `pt`)   (digin.ts digInLook)
pub fn dig_in_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) {
    let m = match g.m.digin.states.as_ref().and_then(|s| s.men.get(gd)) { Some(x) => *x, None => return };
    if m.phase != Phase::Set || !m.deep { return; }
    let look = match m.look { Some(l) => l, None => return };
    pt[0] = look[0];
    pt[2] = look[2];
}

/// roomfight.ts rally: a man who is digging in holds his room, if the fight is out of his reach   (digin.ts digInStays)
pub fn dig_in_stays(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let reach = g.k.digin.reach_m;
    let w = match g.m.digin.states.as_mut().and_then(|s| s.men.get_mut(gd)) { Some(x) => x, None => return false };
    if w.phase == Phase::Idle || !(w.far && reach > 0.0) { return false; }
    if !w.kept { w.kept = true; g.stats.digin.no_rally += 1; }
    true
}

/// pressure.ts lastMan: he is SET on his door   (digin.ts digInSet)
pub fn dig_in_set(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.m.digin.states.as_ref().and_then(|s| s.men.get(gd)).map_or(false, |x| x.phase == Phase::Set)
}

/// pressure.ts's fall-back seam: a spot beside a man of a room dug in that the team has not reached, and the door he comes in by; None = none   (digin.ts digInRefuge)
pub fn dig_in_refuge(g: &mut crate::game::Game, def: RoomfightDefId, threat: V3) -> Option<crate::digin::DigInRefuge> {
    match g.m.digin.states.as_ref() { Some(s) if !s.rooms.is_empty() => {} _ => return None }
    let (gd, my_room) = (g.tab.roomfight_def[def].gd, g.tab.roomfight_def[def].room);
    let pos = pos_of(g, gd);
    let (cell, shy_m) = (g.col.nav.cell, g.k.digin.gather_shy_m);
    let mut best: Option<(V3, Option<DoorId>, f64)> = None;
    for k in state_of(g).rooms.keys() {
        let r = row_of(g, k);
        if Some(r.room) == my_room || !r.entered_at.is_nan() || crate::roomfight::room_fight_in_room(g, Some(r.room), threat) { continue; }
        let mut team_in = false;
        for l in listeners(g) { if crate::roomfight::room_fight_in_room(g, Some(r.room), l.head) { team_in = true; break; } }
        if team_in { continue; }
        let mut with: Option<GuardId> = None;
        for x in g.fight.defs.clone() {
            let (xg, xmode, xroom) = { let y = &g.tab.roomfight_def[x]; (y.gd, y.mode, y.room) };
            if x == def || dead(g, xg) || xmode != crate::roomfight::DefMode::Hold || xroom != Some(r.room) || !(g.time - g.gd[xg].last_seen_t > 3.0) { continue; }
            if matches!(phase_of(g, xg), Phase::Idle | Phase::Think | Phase::ToRoom) { continue; }
            with = Some(xg);
            break;
        }
        let xp = match with { Some(x) => pos_of(g, x), None => continue };
        let aside = [xp[0] + (if pos[0] > xp[0] { 1.2 } else { -1.2 }), 0.0, xp[2]];
        let spot = match crate::roomfight::room_fight_free_near(g, aside) {
            Some(q) => Some(q),
            None => crate::roomfight::room_fight_free_near(g, xp),
        };
        let spot = match spot { Some(q) => q, None => continue };
        if !crate::roomfight::room_fight_in_room(g, Some(r.room), spot) { continue; }
        let mut shy = |_: &mut Game, ix: f64, iz: f64| -> f64 {
            if m::hyp2((ix + 0.5) * cell - threat[0], (iz + 0.5) * cell - threat[2]) < shy_m { 20.0 } else { 1.0 }
        };
        let path = match crate::scene_collision::nav_grid_find_path(g, pos, spot, Some(6000.0), Some(&mut shy)) { Some(p) if !p.is_empty() => p, _ => continue };
        let walk = path_len(pos, &path);
        if walk > g.k.digin.gather_m || best.map_or(false, |b| walk >= b.2) { continue; }
        // (findPath hands back the corners of the way: the threat and the doors are measured against its LEGS)
        let (mut near, mut door, mut bd, mut q) = (m::INF, None, 1.3, pos);
        for w in path.iter() {
            near = m::min(near, leg_dist(threat, q, *w));
            for e in g.tab.roomfight_room[r.room].entries.iter() {
                let x = leg_dist(e.at, q, *w);
                if x < bd { bd = x; door = Some(e.door); }
            }
            q = *w;
        }
        if near < shy_m * 0.5 { continue; }
        best = Some((spot, door, walk));
    }
    let (spot, door, walk) = best?;
    g.stats.digin.refuge += 1;
    let by = match door { Some(x) => format!(" by {}", door_name(g, x)), None => String::new() };
    let text = format!("falls back to a dug-in room{} ({:.1} m)", by, walk);
    log(g, Some(gd), &text);
    Some(DigInRefuge { spot, door })
}

/// flat distance from p to the leg a→b   (digin.ts legDist)
pub fn leg_dist(p: V3, a: V3, b: V3) -> f64 {
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]);
    let l2 = dx * dx + dz * dz;
    let t = if l2 > 0.0 { m::min(1.0, m::max(0.0, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) } else { 0.0 };
    m::hyp2(p[0] - (a[0] + dx * t), p[2] - (a[2] + dz * t))
}

/// a fresh staging: wedged doors get their own lock back, everything remembered dropped   (digin.ts digInReset)
pub fn dig_in_reset(g: &mut crate::game::Game) {
    if g.m.digin.states.is_none() { return; }                         // nothing dug in since the last one: nothing to put back
    for d in state_of(g).wedges.keys() {
        let w = state_of(g).wedges.get(d).unwrap().clone();
        g.dr[d].key_side = w.key_side;
        g.dr[d].lock_id = w.lock_id;
        if !crate::doors::door_lock_broken(g, d) {
            let locked = g.dr[d].def.locked == Some(true);
            crate::doors::door_set_locked(g, d, locked);
        }
    }
    g.m.digin.states = None;
    g.stats.digin = DiginStats::default();
    g.m.digin.digin_log.clear();
    g.m.digin.digin_shut = DiginModDIGINSHUT { home: 0.0, held: 0.0, again: 0.0, more: 0.0, left: 0.0 };
}

/// the one line the checks print   (digin.ts digInSummary)
pub fn dig_in_summary(g: &mut crate::game::Game) -> String {
    if !g.k.digin.on && !crate::enemysquads::squads_live(g) { return String::from("off"); }   // a squads level's men dig in on their squad's word whatever the switch says
    let rooms = g.m.digin.states.as_ref().map_or(0, |s| s.rooms.len());
    let s = &g.stats.digin;
    format!("{} dug in ({} moved) · {} doors wedged ({} booted) · {} rooms · cue first {}/{}", s.dug, s.moved, s.braced, s.kicked, rooms, s.cue_first, s.entered)
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own bits, under bun with EXACT.on (digin.ts legDist)
    #[test]
    fn leg_dist_has_the_typescripts_bits() {
        assert_eq!(leg_dist([1.3, 0.0, 2.7], [0.1, 0.0, -3.3], [7.7, 0.0, 4.9]).to_bits(), 0x400996784bec9bcf);
        assert_eq!(leg_dist([-5.5, 0.0, 0.33], [2.0, 0.0, 2.0], [2.0, 0.0, 2.0]).to_bits(), 0x401ebc15f4b30045);
        assert_eq!(leg_dist([9.1, 0.0, 9.2], [0.0, 0.0, 0.0], [1.1, 0.0, 0.7]).to_bits(), 0x40275861532b8544);
        assert_eq!(leg_dist([0.37, 0.0, -1.9], [1.1, 0.0, 0.7], [-4.2, 0.0, 3.3]).to_bits(), 0x40059ab32252ee56);
    }
    /// a new man is idle with no door, not the generated blank's first word
    #[test]
    fn new_man_is_idle_with_no_door() {
        let m = new_man();
        assert!(m.phase == Phase::Idle);
        assert_eq!((m.entry, m.brace, m.know_t, m.scrape_t, m.set_at), (-1.0, -1.0, -100.0, -100.0, -100.0));
    }
}
