//! src/game/laststand.ts lines 410 to 618
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small private helpers

/// the module's state: every caller here runs with it made
fn st(g: &mut Game) -> &mut State { g.m.laststand.states.as_mut().unwrap() }
fn gd_of(g: &Game, d: RoomfightDefId) -> GuardId { g.tab.roomfight_def[d].gd }
fn pos_of(g: &Game, d: RoomfightDefId) -> V3 { let c = g.gd[g.tab.roomfight_def[d].gd].ch; g.ch[c].pos }
fn mode_of(g: &Game, d: RoomfightDefId) -> crate::roomfight::DefMode { g.tab.roomfight_def[d].mode }
fn room_of(g: &Game, d: RoomfightDefId) -> Option<RoomfightRoomId> { g.tab.roomfight_def[d].room }

/// a line's `{…}` values
fn vars(pairs: &[(&str, &str)]) -> JsMap<String, String> {
    let mut v = JsMap::new();
    for &(k, x) in pairs { v.set(String::from(k), String::from(x)); }
    v
}

/// `decide`'s `send`: he goes from out to a mover, and the net names the stand's room for him
fn send(g: &mut Game, s: &mut Stand, d: RoomfightDefId, spot: V3, at: f64, quiet: bool) {
    s.out.delete(d);
    s.movers.push(Mover { d, phase: MoverPhase::Beat, go_at: at, spot, tries: 0.0, step_t: g.time, t0: g.time, quiet });
    name(g, d, s.id.as_deref());
    let t = &mut st(g).stats;
    t.sent += 1.0;
    if quiet { t.quiet += 1.0; }
}

/// the die's seed off the room clear's seed and attempt
fn fresh_seed(seed: f64, attempt: f64) -> f64 {
    ((m::imul(js::to_i32(seed), 0x9E3779B1u32 as i32) ^ m::imul(js::to_i32(attempt), 0x85EBCA6Bu32 as i32) ^ 0x1A575A4D) as i32) as f64
}

// ---------------------------------------------------------------- the stand

/// The close of a check with `regroup` on: the first makes the stand; a later one takes in whoever has answered since, or can come now.   (laststand.ts decide)
pub fn decide(g: &mut crate::game::Game, c_2: RoomfightDefId, heard: &[RoomfightDefId]) {
    let mut stand = st(g).stand.take();
    decide_on(g, c_2, heard, &mut stand);
    st(g).stand = stand;
}

/// decide's body, the stand held apart so that it can be lent beside `g` (nothing it calls reads st.stand)
fn decide_on(g: &mut Game, c: RoomfightDefId, heard: &[RoomfightDefId], stand: &mut Option<Stand>) {
    let n = st(g).known.len();
    let cgd = gd_of(g, c);
    let cp = pos_of(g, c);
    let w = spoken(g, cp);
    let known = team_known(g, cp).pts;
    if n > 1 {
        st(g).lone_said = false;
    } else if let Some(s) = stand.as_ref() {
        if member(g, s, c) {
            if lone(st(g)) {
                net(g, cgd, LineKind::LastAt, Some(&vars(&[("at", w.at.as_str())])), None, None);
                note(g, Some(cgd), &alloc::format!("the last man of the stand, {}", w.at));
            }
            return;
        }
        let of_it: Vec<RoomfightDefId> = s.in_.iter().chain(s.out.iter()).chain(s.movers.iter().map(|x| x.d)).collect();
        if of_it.iter().any(|&d| alive(g, d)) {
            if lone(st(g)) {
                let line = vars(&[("at", w.at.as_str())]);
                net(g, cgd, LineKind::LastAt, Some(&line), Some(&LaterThen::NameAtUnlessFree { d: c, w }), None);
            }
            return;
        }
        *stand = None;
    }
    let go = g.k.laststand.go;
    if stand.as_ref().map_or(true, |s| s.room.is_none()) {
        let mut men: Vec<RoomfightDefId> = Vec::new();
        for &d in heard { if calm(g, d) && mode_of(g, d) == crate::roomfight::DefMode::Hold { men.push(d); } }
        if mode_of(g, c) != crate::roomfight::DefMode::Hold || men.is_empty() || (n > 1 && men.len() < 2) { tell_only(g, c); return; }
        let ns = alloc::format!("{}", n);
        if n <= 1 {
            let r = stand_room(g, c, &known);
            let p = if r.is_some() { None } else { choose(g, &[c], &known, Some(1.0)) };
            let room = r.as_ref().map(|x| x.room).or(p.as_ref().map(|x| x.room));
            let id: Option<&str> = r.as_ref().map(|x| x.id.as_str()).or(p.as_ref().map(|x| x.id.as_str()));
            let the = match &p { Some(x) => the_of(g, &x.id), None => w.the.clone() };
            let anchor = p.as_ref().map_or(cp, |x| x.anchor);
            let mut s = new_stand(g, room, id, &the, anchor);
            st(g).stats.stands += 1.0;
            st(g).stats.stand_t = g.time;
            let first = lone(st(g));
            let back = p.as_ref().and_then(|x| x.come.first().map(|y| y.spot));
            if let Some(spot) = back {
                net(g, cgd, LineKind::LastGo, Some(&vars(&[("the", s.the.as_str())])), None, None);
                let at = g.time + pick(g, go);
                send(g, &mut s, c, spot, at, true);
                note(g, Some(cgd), &alloc::format!("the last man: back to {}", s.the));
            } else {
                if r.is_some() { s.in_.add(c); } else { s.out.add(c); }
                let what = alloc::format!("the last man: digs in {}", w.at);
                if first {
                    let line = vars(&[("at", w.at.as_str())]);
                    net(g, cgd, LineKind::LastAt, Some(&line), Some(&LaterThen::NameAt { d: c, w: Some(w) }), None);
                } else {
                    name_at(g, c, Some(&w));
                }
                note(g, Some(cgd), &what);
            }
            *stand = Some(s);
            return;
        }
        let Some(p) = choose(g, &men, &known, None) else {
            if stand.is_none() {
                let s = new_stand(g, None, None, "", cp);
                st(g).stats.stands += 1.0;
                st(g).stats.stand_t = g.time;
                let mut shut = true;
                for &d in &men { let dg = gd_of(g, d); if dig_in_phase_is(g, dg, "idle") { shut = false; break; } }
                let mut indoors = true;
                for &d in core::iter::once(&c).chain(men.iter()) {
                    let room = room_of(g, d);
                    let p = pos_of(g, d);
                    if !(room.is_some() && in_room(g, room, p)) { indoors = false; break; }
                }
                let kind = if !indoors { LineKind::OrderOut } else if shut { LineKind::OrderShut } else { LineKind::OrderStay };
                net(g, cgd, kind, Some(&vars(&[("n", ns.as_str()), ("at", w.at.as_str())])), None, None);
                name_at(g, c, Some(&w));
                note(g, Some(cgd), if shut { "no way to each other, every man already dug in: they stay where they are" } else { "no room they can gather in: every man digs in where he is" });
                *stand = Some(s);
            }
            let s = stand.as_mut().unwrap();
            for &d in &men { if !member(g, s, d) { holds_own(g, s, d, None); } }
            return;
        };
        let the = the_of(g, &p.id);
        let was = stand.take();
        let mut s = new_stand(g, Some(p.room), Some(&p.id), &the, p.anchor);
        st(g).stats.gathered += 1.0;
        if was.is_none() {
            st(g).stats.stands += 1.0;
            st(g).stats.stand_t = g.time;
        }
        if let Some(was) = &was { for d in was.out.iter() { s.out.add(d); } }
        let mine = p.inside.contains(&c) || p.come.iter().any(|x| x.d == c);
        net(g, cgd, if mine { LineKind::Order } else { LineKind::OrderFrom }, Some(&vars(&[("n", ns.as_str()), ("the", the.as_str()), ("at", w.at.as_str())])), None, None);
        if !mine {
            st(g).told.add(c);
            s.out.add(c);
            name_at(g, c, Some(&w));
            st(g).stats.no_way += 1.0;
        }
        let what = alloc::format!("orders the stand in {} (room {}): {} there, {} to come, of {}", the, g.tab.roomfight_room[p.room].id, p.inside.len(), p.come.len(), men.len());
        note(g, Some(cgd), &what);
        for &d in &p.inside {
            s.out.delete(d);
            s.in_.add(d);
            name(g, d, Some(&p.id));
            s.homed.add(d);
            let dg = gd_of(g, d);
            crate::roomclear::room_clear_scenario_rehome(g, dg, &p.id);
        }
        let mut at = g.time;
        for x in &p.come {
            at += pick(g, go);
            send(g, &mut s, x.d, x.spot, at, false);
        }
        for &d in &men {
            if !member(g, &s, d) {
                st(g).stats.no_way += 1.0;
                holds_own(g, &mut s, d, Some(LineKind::Cant));
            }
        }
        *stand = Some(s);
        return;
    }
    // a stand is made in a room: a man who answers now and is not in it comes if he can, or holds where he is
    let s = stand.as_mut().unwrap();
    let room = s.room.unwrap();
    if crate::digin::dig_in_entered(g, room) || known.iter().any(|&p| in_room(g, Some(room), p)) { return; }
    for &d in heard {
        if s.in_.has(d) || s.movers.iter().any(|x| x.d == d) || !calm(g, d) || mode_of(g, d) != crate::roomfight::DefMode::Hold { continue; }
        let anchor = anchor_of(g, s);
        let taken: Vec<V3> = s.movers.iter().map(|x| x.spot).collect();
        let spot = spot_in(g, room, anchor, &taken);
        if can_come(g, d, spot, &known).is_some() {
            let at = g.time + pick(g, go);
            send(g, s, d, spot, at, false);
            continue;
        }
        if !s.out.has(d) {
            st(g).stats.no_way += 1.0;
            holds_own(g, s, d, Some(LineKind::Cant));
        }
    }
}

fn dig_in_phase_is(g: &Game, gd: GuardId, word: &str) -> bool { crate::digin::dig_in_phase(g, gd) == word }

/// One man on his way to the stand; true = he is off the movers' list. `m` is his row, lent out of `s` by the walk: `s.movers` here still holds the row as it was.   (laststand.ts tickMover)
pub fn tick_mover(g: &mut crate::game::Game, s: &mut crate::laststand::Stand, m: &mut crate::laststand::Mover) -> bool {
    let d = m.d;
    let gd = gd_of(g, d);
    let c = g.gd[gd].ch;
    if !alive(g, d) || !g.fight.defs.contains(&d) {
        if g.gd[gd].state == crate::game::GuardState::Dead || !g.fight.defs.contains(&d) {
            st(g).stats.died_on_way += 1.0;
            note(g, Some(gd), "down on the way");
        }
        return true;
    }
    if m.phase == MoverPhase::Beat {
        if g.time < m.go_at { return false; }
        if s.room.is_none() || !movable(g, d) {
            st(g).stats.dropped += 1.0;
            s.out.add(d);
            let w = spoken(g, g.ch[c].pos);
            name_at(g, d, Some(&w));
            return true;
        }
        m.phase = MoverPhase::Run;
        crate::pressure::run(g, d, m.spot, None, None);
        m.step_t = g.time;
        if !m.quiet { net(g, gd, LineKind::Go, Some(&vars(&[("the", s.the.as_str())])), None, None); }
        note(g, Some(gd), &alloc::format!("sets off for {}", s.the));
        return false;
    }
    if mode_of(g, d) == crate::roomfight::DefMode::Move {
        m.step_t = crate::pressure::boots(g, gd, m.step_t);
        return false;
    }
    if mode_of(g, d) == crate::roomfight::DefMode::Free {
        st(g).stats.met += 1.0;
        note(g, Some(gd), "met on the way: the fight has him");
        name(g, d, None);
        return true;
    }
    // planted (roomfight.ts tickMove): there, in contact on the way, or stopped short
    let contact = g.time - g.gd[gd].last_seen_t < 2.0;
    if s.room.is_some() && room_of(g, d) == s.room && in_room(g, s.room, g.ch[c].pos) {
        s.in_.add(d);
        st(g).stats.arrived += 1.0;
        if let Some(id) = s.id.clone() {
            s.homed.add(d);
            crate::roomclear::room_clear_scenario_rehome(g, gd, &id);
        }
        let mut n = 0;
        for x in s.in_.keys() { if alive(g, x) { n += 1; } }
        if !contact && n > 1 {
            let ns = alloc::format!("{}", n);
            net(g, gd, LineKind::In, Some(&vars(&[("n", ns.as_str()), ("the", s.the.as_str())])), None, None);
        }
        note(g, Some(gd), &alloc::format!("in {}, {} s after the order", s.the, js::to_fixed(g.time - m.t0, 1)));
        return true;
    }
    if !contact && s.room.is_some() && { m.tries += 1.0; m.tries } < g.k.laststand.legs && fit(g, d) {
        let room = s.room.unwrap();
        let anchor = anchor_of(g, s);
        let taken: Vec<V3> = s.movers.iter().filter(|x| x.d != d).map(|x| x.spot).collect();
        let spot = spot_in(g, room, anchor, &taken);
        let known = team_known(g, g.ch[c].pos).pts;
        if way_to(g, d, spot, &known).is_some() {
            m.spot = spot;
            crate::pressure::run(g, d, spot, None, None);
            return false;
        }
    }
    if contact {
        st(g).stats.met += 1.0;
        note(g, Some(gd), "met on the way: fights where he is");
    } else {
        st(g).stats.short += 1.0;
        note(g, Some(gd), "stopped short: holds where he is");
    }
    s.out.add(d);
    let w = spoken(g, g.ch[c].pos);
    name_at(g, d, Some(&w));
    if !contact && calm(g, d) { net(g, gd, LineKind::Short, Some(&vars(&[("at", w.at.as_str())])), None, None); }
    true
}

/// The door of his room they will likeliest come by, as a point some strides back along that way: what the dig-in is handed as 'where they were heard'.   (laststand.ts comeBy)
pub fn come_by(g: &mut crate::game::Game, d: RoomfightDefId, r: RoomfightRoomId) -> V3 {
    let last = team_known(g, pos_of(g, d)).last;
    let refp = crate::roomfight::room_fight_free_near(g, last).unwrap_or(last);
    let entries = g.tab.roomfight_room[r].entries.clone();
    let mut way: Option<Vec<V3>> = None;
    let mut wl = f64::INFINITY;
    let mut e = entries[0];
    // round the room, not through it
    let mut dear = |g: &mut Game, ix: f64, iz: f64| -> f64 {
        let i = crate::scene_collision::nav_grid_idx(g, ix, iz);
        if g.tab.roomfight_room[r].cells.has(crate::Num(i + 0.0)) { 30.0 } else { 1.0 }   // (+ 0.0: a JavaScript Set takes -0 for 0, Num compares bits)
    };
    for x in &entries {
        let p0 = [x.at[0] + x.out[0] * 1.2, 0.0, x.at[2] + x.out[1] * 1.2];
        let o = crate::roomfight::room_fight_free_near(g, p0).unwrap_or(p0);
        let Some(path) = crate::scene_collision::nav_grid_find_path(g, o, refp, Some(6000.0), Some(&mut dear)) else { continue };
        let mut l = 0.0;
        let mut q = o;
        for &w in &path {
            l += flat(q, w);
            q = w;
        }
        if l < wl {
            wl = l;
            let mut whole = vec![o];
            whole.extend(path);
            way = Some(whole);
            e = *x;
        }
    }
    if let Some(way) = way {
        let mut l = 0.0;
        for i in 1..way.len() {
            l += flat(way[i - 1], way[i]);
            if l >= 8.0 && flat(way[i], e.at) > 6.5 { return v3::copy(way[i]); }
        }
    }
    [e.at[0] + e.out[0] * 8.0, 0.0, e.at[2] + e.out[1] * 8.0]
}

/// The stand's frame: its dead dropped, its movers moved, once a second its men set to dig in; then 'we're set' once, and a thud at a wedged door now and then.   (laststand.ts tickStand)
pub fn tick_stand(g: &mut crate::game::Game, s: &mut crate::laststand::Stand) {
    for x in s.in_.keys() {
        if !alive(g, x) || !g.fight.defs.contains(&x) {
            s.in_.delete(x);
            name(g, x, None);
        }
    }
    for x in s.out.keys() {
        if !alive(g, x) || !g.fight.defs.contains(&x) {
            s.out.delete(x);
            name(g, x, None);
        }
    }
    if !s.movers.is_empty() {
        // Array.filter: the list stays whole for the walk (tickMover reads it) and is replaced after; a row is lent as the walk reaches it and written back
        let mut kept: Vec<Mover> = Vec::new();
        for i in 0..s.movers.len() {
            let mut row = s.movers[i];
            let gone = tick_mover(g, s, &mut row);
            s.movers[i] = row;
            if !gone { kept.push(row); }
        }
        s.movers = kept;
    }
    if g.time < s.look_t { return; }
    s.look_t = g.time + 1.0;
    // every man of the stand who holds a room nobody has entered digs in, asked again if something stopped him
    let dig = g.k.laststand.dig;
    let walk: Vec<RoomfightDefId> = s.in_.iter().chain(s.out.iter()).collect();
    for d in walk {
        let Some(droom) = room_of(g, d) else { continue };
        if mode_of(g, d) != crate::roomfight::DefMode::Hold || !calm(g, d) { continue; }
        let gd = gd_of(g, d);
        if let Some(room) = s.room {
            if s.in_.has(d) && droom != room {
                // planted outside the stand room: back in, or he holds where he is
                s.in_.delete(d);
                let anchor = anchor_of(g, s);
                let taken: Vec<V3> = s.movers.iter().map(|x| x.spot).collect();
                let spot = spot_in(g, room, anchor, &taken);
                let known = team_known(g, pos_of(g, d)).pts;
                if can_come(g, d, spot, &known).is_some() {
                    st(g).stats.sent += 1.0;
                    st(g).stats.quiet += 1.0;
                    s.movers.push(Mover { d, phase: MoverPhase::Beat, go_at: g.time, spot, tries: 0.0, step_t: g.time, t0: g.time, quiet: true });
                } else {
                    s.out.add(d);
                    let w = spoken(g, pos_of(g, d));
                    name_at(g, d, Some(&w));
                }
                continue;
            }
        }
        let ph = crate::digin::dig_in_phase(g, gd);
        if ph == "set" && !s.homed.has(d) {
            if let Some(id) = where_(g, pos_of(g, d)).id {
                s.homed.add(d);
                crate::roomclear::room_clear_scenario_rehome(g, gd, &id);
            }
        }
        if s.room.is_some() && !s.movers.is_empty() && g.time - s.t < 25.0 { continue; }
        // one man at the doors at a time
        let others: Vec<RoomfightDefId> = s.in_.iter().chain(s.out.iter()).collect();
        let mut busy = false;
        for x in others {
            if x != d && room_of(g, x) == Some(droom) {
                let p = crate::digin::dig_in_phase(g, gd_of(g, x));
                if p != "idle" && p != "set" { busy = true; break; }
            }
        }
        if busy { continue; }
        if ph != "idle" || g.tab.roomfight_room[droom].entries.is_empty() || crate::digin::dig_in_entered(g, droom) || s.orders.get(d).copied().unwrap_or(0.0) >= 4.0 { continue; }
        let know = come_by(g, d, droom);
        let beat = pick(g, dig);
        if crate::digin::dig_in_order(g, d, know, beat) {
            let was = s.orders.get(d).copied().unwrap_or(0.0);
            s.orders.set(d, was + 1.0);
            st(g).stats.dig_orders += 1.0;
            let what = alloc::format!("told to dig in (room {})", g.tab.roomfight_room[droom].id);
            note(g, Some(gd), &what);
        }
    }
    let here: Vec<RoomfightDefId> = match s.room {
        Some(room) => s.in_.keys().into_iter().filter(|&x| room_of(g, x) == Some(room)).collect(),
        None => s.out.keys(),
    };
    let thump = g.k.laststand.thump;
    if !s.set_said {
        let mut set: Vec<RoomfightDefId> = Vec::new();
        for &x in &here { if dig_in_phase_is(g, gd_of(g, x), "set") { set.push(x); } }
        let mut busy = !s.movers.is_empty();
        if !busy {
            for &x in &here {
                let p = crate::digin::dig_in_phase(g, gd_of(g, x));
                if p != "set" && p != "idle" { busy = true; break; }
            }
        }
        let mut who = None;
        for &x in &set { if calm(g, x) { who = Some(x); break; } }
        if let (Some(who), Some(room)) = (who, s.room) {
            if !busy || g.time - s.t > 45.0 {
                s.set_said = true;
                s.set_t = g.time;
                st(g).stats.set_t = g.time;
                st(g).stats.set_men = here.len() as f64;
                s.thump_at = g.time + pick(g, thump);
                let wedged = crate::digin::dig_in_wedged(g, room).len();
                let kind = if here.len() > 1 {
                    if wedged != 0 { LineKind::Set } else { LineKind::SetOpen }
                } else if wedged != 0 { LineKind::SetLone } else { LineKind::SetLoneOpen };
                let wg = gd_of(g, who);
                net(g, wg, kind, Some(&vars(&[("the", s.the.as_str())])), None, None);
                note(g, Some(wg), &alloc::format!("the stand is set: {} in {}, {} door(s) wedged", here.len(), s.the, wedged));
                let again = g.time + g.k.laststand.again;
                let row = st(g);
                row.next_at = m::min(row.next_at, again);
            }
        }
        return;
    }
    // set: one of them shores the wedge up now and then, a thud at the door heard like any shoved furniture
    if let Some(room) = s.room {
        if g.time >= s.thump_at {
            s.thump_at = g.time + pick(g, thump);
            if crate::digin::dig_in_entered(g, room) { return; }
            let doors = crate::digin::dig_in_wedged(g, room);
            let i = m::floor(crate::math_dice::roll(&mut st(g).die) * doors.len() as f64) as usize;
            let e = doors.get(i).copied();
            let mut who = None;
            if e.is_some() {
                for &x in &here {
                    if calm(g, x) && mode_of(g, x) == crate::roomfight::DefMode::Hold && dig_in_phase_is(g, gd_of(g, x), "set") { who = Some(x); break; }
                }
            }
            let (Some(e), Some(who)) = (e, who) else { return };
            crate::audio_iface::game_audio_like_play(g, "propThud", Some(e.at), Some(0.9), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.8 }));
            let wg = gd_of(g, who);
            let wc = g.gd[wg].ch;
            let serial = g.serial();
            let ev = crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: [e.at[0], 0.0, e.at[2]], time: g.time, loud: false, level: Some(g.k.laststand.thump_level), who: Some(g.ch[wc].id), serial, ..crate::game::GameEvent::zeroed() };
            g.events.push(ev);   // (blamed on him: his own side ignore it)
            st(g).stats.thumps += 1.0;
            let what = alloc::format!("shores up {}", g.dr[e.door].def.name);
            note(g, Some(wg), &what);
        }
    }
}

// ---------------------------------------------------------------- the seams with the rest of the game

/// A new state for the endgame: its own die off the room clear's seed and attempt, four draws spent; the posts as staged, kept if it is the same staging.   (laststand.ts fresh)
pub fn fresh(g: &mut crate::game::Game) -> crate::laststand::State {
    let (seed, attempt) = (g.room_clear.seed, g.room_clear.attempt);
    let mut die = crate::math_dice::die_of(fresh_seed(seed, attempt));
    for _ in 0..4 { crate::math_dice::roll(&mut die); }
    let key = alloc::format!("{}:{}", seed, attempt);
    let kept = match &g.m.laststand.states { Some(was) if was.key == key => Some(was.post.clone()), _ => None };
    let post = match kept {
        Some(p) => p,
        None => {
            let mut p: JsMap<GuardId, String> = JsMap::new();
            for r in &g.room_clear.rooms { for &gd in &r.defenders { p.set(gd, r.def.id.clone()); } }
            p
        }
    };
    let mut rot: JsMap<LineKind, f64> = JsMap::new();
    rot.set(LineKind::MovedAt, 0.0);
    rot.set(LineKind::MovedFree, 0.0);
    State {
        post, key, die, next_at: 0.0, check: None, known: JsSet::new(), named: JsMap::new(), spots: JsMap::new(), at: JsMap::new(), stand: None,
        lone_said: false, few_n: 0.0, told: JsSet::new(), said: Vec::new(), stats: new_stats(), stale: false, rot, cls_at: Vec::new(), later: Vec::new(),
    }
}

/// `regroup` switched off with a stand made: every man lets go of what the stand told him to do.   (laststand.ts dissolve)
pub fn dissolve(g: &mut crate::game::Game, s: &crate::laststand::Stand) {
    let men: Vec<RoomfightDefId> = s.in_.iter().chain(s.out.iter()).chain(s.movers.iter().map(|x| x.d)).collect();
    for d in men {
        let gd = gd_of(g, d);
        crate::digin::dig_in_release(g, gd);
    }
    st(g).stand = None;
    stand_say(g);
}

/// Who the stand has: in the room, on his way to it, or told to hold where he is.   (laststand.ts ofStand)
pub fn of_stand(g: &crate::game::Game, s: &crate::laststand::Stand, gd: GuardId) -> bool {
    for d in s.in_.iter() { if g.tab.roomfight_def[d].gd == gd { return true; } }
    for d in s.out.iter() { if g.tab.roomfight_def[d].gd == gd { return true; } }
    s.movers.iter().any(|x| g.tab.roomfight_def[x.d].gd == gd)
}

/// The owner's bit: the stand claims its men and lets go of every other.   (laststand.ts standSay)
pub fn stand_say(g: &mut crate::game::Game) {
    let s = g.m.laststand.states.as_ref().and_then(|st| st.stand.clone());
    let rule = crate::owner::by::STAND;
    let t = g.time;
    let guards = g.guards.clone();   // (neither call touches the list)
    for gd in guards {
        let has = match &s { Some(s) => of_stand(g, s, gd), None => false };
        if has { crate::owner::claim(g, gd, rule, t, None); } else { crate::owner::release(g, gd, rule, t); }
    }
}

/// Once a frame from RoomFight.tick, after the defenders', the press's and the buddy call's own ticks.   (laststand.ts lastStandTick)
pub fn last_stand_tick(g: &mut crate::game::Game) {
    step_stand(g);
}

/// The frame of the endgame: 'most are down' starts it; then the lines waiting, the check, who has moved, the stand.   (laststand.ts stepStand)
pub fn step_stand(g: &mut crate::game::Game) {
    // (the switch flipped mid-stand; callOut is settled on, so the frame goes on)
    if let Some(s) = g.m.laststand.states.as_ref().and_then(|st| st.stand.clone()) { dissolve(g, &s); }
    if !crate::roomclear::room_clear_scenario_active(g) || g.room_clear.end.is_some() || g.rescue.active { return; }
    if g.m.laststand.states.as_ref().map_or(true, |st| st.stale) {
        let all = g.night_breach.hostiles.clone();
        let few = m::min(g.k.laststand.few, m::floor(all.len() as f64 / 3.0));
        let mut up = 0;
        for &h in &all { if g.gd[h].state != crate::game::GuardState::Dead && !g.gd[h].fled { up += 1; } }
        if up as f64 > few || up == 0 { return; }
        let row = fresh(g);
        g.m.laststand.states = Some(row);
        let t = g.time;
        st(g).stats.t0 = t;
        st(g).stats.up0 = up as f64;
        let first = g.k.laststand.first;
        let at = t + pick(g, first);
        st(g).next_at = at;
        note(g, None, &alloc::format!("most are down: {} of {} up", up, all.len()));
    }
    let defs = g.fight.defs.clone();
    if !defs.iter().any(|&d| alive(g, d)) { return; }
    let mut names = st(g).named.keys();
    names.extend(st(g).spots.keys());
    for d in names { if !alive(g, d) { name(g, d, None); } }
    tick_later(g);
    if st(g).check.is_some() {
        tick_check(g);
    } else if g.time >= st(g).next_at {
        start_check(g);
    }
    if !st(g).at.is_empty() { tick_moved(g); }
    if let Some(mut s) = st(g).stand.take() {
        tick_stand(g, &mut s);
        st(g).stand = Some(s);
    }
}

/// The fight is over (RoomFight.stop): everything forgotten but the counts and the lines.   (laststand.ts lastStandStop)
pub fn last_stand_stop(g: &mut crate::game::Game) {
    let Some(st) = g.m.laststand.states.as_mut() else { return };
    st.check = None;
    st.stand = None;
    stand_say(g);
    let st = g.m.laststand.states.as_mut().unwrap();
    st.known.clear();
    st.named.clear();
    st.spots.clear();
    st.at.clear();
    st.later.clear();
    st.stale = true;
}

/// roomclear.ts objective(): the rooms their net has named and the named places that are no room of it, by the men still up who said so
pub fn last_stand_hint(g: &crate::game::Game) -> Option<crate::laststand::Hint> {
    let st = g.m.laststand.states.as_ref()?;
    if st.stale { return None; }
    let mut rooms: JsSet<String> = JsSet::new();
    for (d, id) in st.named.iter() { if alive(g, d) { rooms.add(id.clone()); } }
    let mut spots: Vec<Spot> = Vec::new();
    for (d, x) in st.spots.iter() { if alive(g, d) && !spots.iter().any(|y| y.name == x.name) { spots.push(x.clone()); } }
    if rooms.is_empty() && spots.is_empty() { None } else { Some(Hint { rooms, spots }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_die_is_seeded_as_the_language_multiplies() {
        for &(seed, attempt, want) in SEEDS { assert_eq!(fresh_seed(seed, attempt).to_bits(), want.to_bits(), "{} {}", seed, attempt); }
    }

    const SEEDS: &[(f64, f64, f64)] = &[(0.0, 0.0, 441932365.0), (1.0, 1.0, 25946519.0), (12345.0, 2.0, -1330936078.0), (4294967295.0, 3.0, -363035837.0), (-7.0, 9.0, 57076647.0), (2147483648.0, 1.0, 532451366.0), (3.9, 2.9, -886660216.0), (987654321.0, 1000.0, -1273962284.0)];
}
