//! src/game/pressure.ts lines 434 to 931
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the table of spots outside a door, (depth, lateral) (pressure.ts 872)
const SPOTS: [(f64, f64); 9] = [(3.0, 1.6), (3.0, -1.6), (2.4, 1.2), (2.4, -1.2), (1.2, 2.4), (1.2, -2.4), (1.2, 1.7), (1.2, -1.7), (2.6, 0.0)];

// ---- small reads
#[inline]
fn df(g: &Game, d: RoomfightDefId) -> &crate::roomfight::Def { &g.tab.roomfight_def[d] }
#[inline]
fn dfm(g: &mut Game, d: RoomfightDefId) -> &mut crate::roomfight::Def { &mut g.tab.roomfight_def[d] }
#[inline]
fn gd_of(g: &Game, d: RoomfightDefId) -> GuardId { g.tab.roomfight_def[d].gd }
#[inline]
fn op_gd(g: &Game, i: usize) -> GuardId { g.tab.roomfight_op[g.fight.ops[i]].gd }
#[inline]
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
#[inline]
fn player_pos(g: &Game) -> V3 { g.ch[g.pl[g.player].ch].pos }
#[inline]
fn door_centre(g: &Game, door: DoorId) -> V3 { g.dr[door].frame_centre }
#[inline]
fn door_name(g: &Game, door: DoorId) -> String { g.dr[door].def.name.clone() }
#[inline]
fn in_cells(g: &Game, t: RoomfightRoomId, cell: f64) -> bool { g.tab.roomfight_room[t].cells.has(crate::Num(cell)) }
#[inline]
fn is_blocked(g: &mut Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
/// a live man of the fight other than `d` stands within 0.9 m of p
fn someone_at(g: &mut Game, d: RoomfightDefId, p: V3) -> bool {
    for i in 0..g.fight.defs.len() {
        let x = g.fight.defs[i];
        if x != d && alive(g, x) && flat(pos_of(g, gd_of(g, x)), p) < 0.9 { return true; }
    }
    false
}
/// `D.ev[i] = Math.max(D.ev[i], v)`: a Def's ev is not always as long as its room's entries (roomfight.ts 898), and a read past its end is undefined
fn bump_ev(g: &mut Game, d: RoomfightDefId, i: usize, v: f64) {
    let ev = &mut g.tab.roomfight_def[d].ev;
    if ev.len() <= i { ev.resize(i + 1, m::NAN); }
    ev[i] = m::max(ev[i], v);
}

// ---- standOffSpot
struct Cand { p: V3, d: f64, ok: Option<bool>, lit: Option<bool>, way: Option<Option<f64>> }
/// p's distance from the line a → mouth, where it stands beside that line
fn off_line(p: V3, a: V3, mouth: V3) -> f64 {
    let dx = mouth[0] - a[0];
    let dz = mouth[2] - a[2];
    let l2 = js::or(dx * dx + dz * dz, 1.0);
    let t = ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2;
    if t <= 0.0 || t >= 1.0 { m::INF } else { m::hyp2(p[0] - (a[0] + dx * t), p[2] - (a[2] + dz * t)) }
}
/// the place's own steady light at q is enough to be seen by
fn lit_at(g: &mut Game, q: V3) -> bool {
    let e = crate::guards::light_estimate_at(g, [q[0], 1.2, q[2]], Some(&mut |_g: &mut Game, _o: f64| true), Some(true), None, None);
    crate::game::game_visibility_of(g, e) >= g.k.lights.lit_at
}
/// the way on foot to q that does not cross T: its length, or none
fn way_to(g: &mut Game, from: V3, q: V3, t: RoomfightRoomId, dear: &mut dyn FnMut(&mut Game, f64, f64) -> f64) -> Option<f64> {
    let path = crate::scene_collision::nav_grid_find_path(g, from, q, Some(6000.0), Some(&mut *dear))?;
    if path.is_empty() { return None; }
    let mut l = 0.0;
    let mut w0 = from;
    for w in path {
        l += flat(w0, w);
        w0 = w;
        let c = cell_of(g, w);
        if in_cells(g, t, c) { return None; }
    }
    if flat(w0, q) > 0.8 { None } else { Some(l) }
}
/// PRESSURE.wayOut: a spot to hold entry `e` of T from that a can through that door does not reach, nearest on foot of the first few that pass. (pressure.ts standOffSpot)
pub fn stand_off_spot(g: &mut crate::game::Game, d: RoomfightDefId, t: RoomfightRoomId, e: crate::roomfight::Entry, dear: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) -> Option<crate::pressure::StandOffSpot> {
    let gd = gd_of(g, d);
    let c_pos = pos_of(g, gd);
    let cs = g.col.nav.cell;
    let m0 = crate::effects::bang().dazzle_radius + g.k.pressure.way_out_m;
    let m1 = m0 + STAND_OFF_BAND;
    let air_in = g.k.doortools.air_in;
    let burst: V3 = [e.at[0] + e.out[0] * air_in, 0.0, e.at[2] + e.out[1] * air_in];
    let mouth: V3 = [e.at[0] + e.out[0] * 0.7, 1.2, e.at[2] + e.out[1] * 0.7];
    let mut taken: Vec<V3> = Vec::new();
    let men: Vec<PressureManId> = state_of(g).of.values().copied().collect();
    for man in men {
        let x = g.tab.pressure_man[man];
        if x.d != d && alive(g, x.d) { taken.push(x.spot); }
    }
    // the cheap tests on the band, every other cell each way; lines, light and ways only on the nearest, once each
    let mut cand: Vec<Cand> = Vec::new();
    let x0 = js::to_i32(m::floor((burst[0] - m1) / cs));
    let x1 = js::to_i32(m::floor((burst[0] + m1) / cs));
    let z0 = js::to_i32(m::floor((burst[2] - m1) / cs));
    let z1 = js::to_i32(m::floor((burst[2] + m1) / cs));
    let mut iz = z0;
    while iz <= z1 {
        let mut ix = x0;
        while ix <= x1 {
            let (cx, cz) = (ix, iz);
            ix += 1;
            if (cx | cz) & 1 != 0 { continue; }
            let p: V3 = [(cx as f64 + 0.5) * cs, 0.0, (cz as f64 + 0.5) * cs];
            let dist = flat(p, burst);
            if dist < m0 || dist > m1 || (p[0] - e.at[0]) * e.out[0] + (p[2] - e.at[2]) * e.out[1] < m::max(1.2, 0.2 * dist) { continue; }
            if is_blocked(g, p[0], p[2]) { continue; }
            let cell = crate::scene_collision::nav_grid_idx(g, cx as f64, cz as f64);
            if in_cells(g, t, cell) { continue; }
            if is_blocked(g, p[0] + cs, p[2]) || is_blocked(g, p[0] - cs, p[2]) || is_blocked(g, p[0], p[2] + cs) || is_blocked(g, p[0], p[2] - cs) { continue; }
            if someone_at(g, d, p) { continue; }
            cand.push(Cand { p, d: flat(p, c_pos), ok: None, lit: None, way: None });
        }
        iz += 1;
    }
    js::sort_by(&mut cand, |a, b| a.d - b.d);
    let team = g.gd[gd].team;
    // dark and well clear of his mates and their lines · any light, a body's width clear · a floor too small for lanes: behind a mate
    for (apart, lane, dark) in [(1.4, 0.8, true), (1.0, 0.45, false), (1.0, 0.0, false)] {
        let mut best: Option<(V3, f64)> = None;
        let mut tries = 0.0;
        for k in cand.iter_mut() {
            if tries >= STAND_OFF_TRIES || best.map_or(false, |b| k.d >= b.1) { break; }
            if k.ok == Some(false) || k.way == Some(None) || taken.iter().any(|q| flat(*q, k.p) < apart || off_line(k.p, *q, mouth) < lane || off_line(*q, k.p, mouth) < lane) { continue; }
            if k.ok.is_none() {
                let energy = crate::glasspen::side_shot_energy(g, team);
                k.ok = Some(!crate::scene_collision::shot_blocked(g, [k.p[0], 1.35, k.p[2]], mouth, Some(0.02), energy));
            }
            if k.ok != Some(true) { continue; }
            if dark {
                let lit = match k.lit { Some(l) => l, None => { let l = lit_at(g, k.p); k.lit = Some(l); l } };
                if lit { continue; }
            }
            if k.way.is_none() { tries += 1.0; k.way = Some(way_to(g, c_pos, k.p, t, &mut *dear)); }
            if let Some(Some(l)) = k.way {
                if best.map_or(true, |b| l < b.1) { best = Some((k.p, l)); }
            }
        }
        if let Some((spot, l)) = best { return Some(crate::pressure::StandOffSpot { spot, l }); }
    }
    None
}

/// A leg this module orders: he runs to `spot`, off the fight AI, a fallback's move. (pressure.ts run)
pub fn run(g: &mut crate::game::Game, d: RoomfightDefId, spot: V3, door: Option<DoorId>, fast: Option<f64>) {
    let fast = fast.unwrap_or(0.0);
    let gd = gd_of(g, d);
    let here = pos_of(g, gd);
    let de = dfm(g, d);
    de.mode = crate::roomfight::DefMode::Move; de.move_kind = crate::roomfight::DefMoveKind::Fallback; de.came_by = door; de.move_t = 0.0; de.stall_at = here; de.stall_t = 0.0; de.rally_at = m::NAN;
    g.gd[gd].pinned = false;
    let speed = g.tune.guard_chase + fast;
    crate::squad::order(g, gd, crate::squad::OrderPatch {
        goal: Some(Some(spot)), speed: Some(Some(speed)), face: Some(None), strafe: Some(Some(false)), aim_at: Some(None),
        upper: Some(Some(crate::character::UpperMode::Aim)), stance: Some(Some(crate::character::Stance::None)), crouch: Some(Some(false)),
        ..Default::default()
    });
}

/// PRESSURE.stuns: a man with a charge on him, let go at the fight, takes the door he was put down for as the drill, if their side has something that puts the team in that room. (pressure.ts charge)
pub fn charge(g: &mut crate::game::Game, d: RoomfightDefId, door: DoorId, t: Option<RoomfightRoomId>) -> bool {
    let gd = gd_of(g, d);
    if !(g.k.pressure.stuns > 0.0 && crate::tactics::stuns_of(g, gd) > 0.0) { return false; }
    let cand: Vec<RoomfightRoomId> = match t { Some(x) => vec![x], None => rooms_by(g, d, door) };
    let mut r: Option<RoomfightRoomId> = None;
    for &x in &cand {
        if team_placed(g, x).known { r = Some(x); break; }
    }
    if r.is_none() {
        state_of(g).stats.stun_held += 1.0;
        let mut any = false;
        for &x in &cand { if team_stands(g, x) { any = true; break; } }
        if any { state_of(g).stats.stun_held_in += 1.0; }
        let what = format!("no charge at {}: nothing puts them in that room", door_name(g, door));
        note(g, gd, &what);
        return false;
    }
    let team = g.gd[gd].team;
    let prev = crate::tactics::danger_at(g, door, team);
    let had = state_of(g).marks.get(door).copied();
    crate::tactics::mark_danger(g, door, team, crate::tactics::DangerWhy::Press, None);
    state_of(g).stats.stun_marked += 1.0;
    // (own: as the TypeScript reads it: prev is the one row mark_danger has just written `why` on)
    let own = match had { Some(h) => h.own, None => prev.map_or(true, |p| g.tab.tactics_danger_rec[p].why == crate::tactics::DangerWhy::Press) };
    state_of(g).marks.set(door, crate::pressure::StateMarksValue { r, team, own });
    true
}
/// the rooms a door gives on to, the one he stands in and the one he holds aside. (pressure.ts roomsBy)
pub fn rooms_by(g: &mut crate::game::Game, d: RoomfightDefId, door: DoorId) -> Vec<RoomfightRoomId> {
    let fr = crate::guards::frame_of(g, door);
    let c = door_centre(g, door);
    let here = pos_of(g, gd_of(g, d));
    let mine = crate::roomfight::room_fight_room_at(g, here);
    let held = df(g, d).room;
    let mut out: Vec<RoomfightRoomId> = Vec::new();
    for k in [1.0, -1.0] {
        let r = crate::roomfight::room_fight_room_at(g, [c[0] + fr.nx * k, 0.0, c[2] + fr.nz * k]);
        if let Some(x) = r {
            if r != mine && r != held && !out.contains(&x) { out.push(x); }
        }
    }
    out
}
/// p is in room T or in one of its doorways (a doorway's cells are no room's). (pressure.ts inRoom)
pub fn in_room(g: &mut crate::game::Game, t: RoomfightRoomId, p: V3) -> bool {
    let c = cell_of(g, p);
    if in_cells(g, t, c) { return true; }
    let n = g.tab.roomfight_room[t].entries.len();
    for i in 0..n {
        let door = g.tab.roomfight_room[t].entries[i].door;
        if flat(door_centre(g, door), p) < 1.2 { return true; }
    }
    false
}
/// a man of the team stands in R: the floor's truth, for the counts only. (pressure.ts teamStands)
pub fn team_stands(g: &mut crate::game::Game, r: RoomfightRoomId) -> bool {
    for i in 0..g.fight.ops.len() {
        let o = op_gd(g, i);
        if g.gd[o].state != crate::game::GuardState::Dead && in_room(g, r, pos_of(g, o)) { return true; }
    }
    let p = player_pos(g);
    in_room(g, r, p)
}
/// the last of his side alive: nobody left to go to. (pressure.ts lastAlone)
pub fn last_alone(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    for i in 0..g.fight.defs.len() {
        let x = g.fight.defs[i];
        if x != d && alive(g, x) { return false; }
    }
    true
}
/// PRESSURE.stunKnown: what their side has to go on that the team is in room T. (pressure.ts teamPlaced)
pub fn team_placed(g: &mut crate::game::Game, t: RoomfightRoomId) -> crate::pressure::TeamPlaced {
    let mut in_t = -m::INF;
    let mut other = -m::INF;
    let heard: Vec<crate::pressure::StateHeardItem> = state_of(g).heard.clone();   // (put changes nothing of it)
    for h in heard.iter() { put(g, t, h.t, h.p, &mut in_t, &mut other); }
    for i in 0..g.fight.defs.len() {
        let x = g.fight.defs[i];
        let o = gd_of(g, x);
        if alive(g, x) && g.gd[o].held.is_none() && (g.gd[o].state == crate::game::GuardState::Alert || g.gd[o].state == crate::game::GuardState::Search) {
            let (when, at) = (g.gd[o].sight_t, g.gd[o].sight_pos);
            put(g, t, when, at, &mut in_t, &mut other);
        }
    }
    let moved = other > in_t + g.k.pressure.stun_moved_secs;
    crate::pressure::TeamPlaced { known: in_t > -m::INF && !moved, moved }
}
fn put(g: &mut Game, t: RoomfightRoomId, when: f64, p: V3, in_t: &mut f64, other: &mut f64) {
    if g.time - when > g.k.pressure.stun_know_secs { return; }
    if in_room(g, t, p) { *in_t = m::max(*in_t, when); } else { *other = m::max(*other, when); }
}
/// one more thing heard of the team at p (a spot heard from again is the one entry, its time moved on). (pressure.ts heardAt)
pub fn heard_at(st: &mut crate::pressure::State, t: f64, p: V3) {
    if let Some(h) = st.heard.iter_mut().find(|x| flat(x.p, p) < 2.0) {
        h.t = m::max(h.t, t);
        h.p = p;
    } else {
        st.heard.push(crate::pressure::StateHeardItem { t, p });
    }
}
/// is p where the bang at a marked door (room R, else within 6 m of its centre c) could have taken a man
fn bang_by(g: &mut Game, r: Option<RoomfightRoomId>, c: V3, p: V3) -> bool {
    match r { Some(x) => in_room(g, x, p), None => flat(p, c) < 6.0 }
}
/// once a frame while anybody carries a charge: the team's loud doings read off the floor's events; marked doors the team has left let be; the counts of charges gone off. (pressure.ts learn)
pub fn learn(g: &mut crate::game::Game) {
    let pid = g.ch[g.pl[g.player].ch].id;
    let now = g.time;
    for i in 0..g.events.len() {
        let e = g.events[i];
        let kick = e.kind == crate::director::EventKind::Kick;
        if e.kind != crate::director::EventKind::Shot && e.kind != crate::director::EventKind::GuardShot && !kick { continue; }
        let who = match e.who { Some(w) => w, None => continue };
        let st = state_of(g);
        if st.ev_seen.has(e.serial) { continue; }
        st.ev_seen.add(e.serial);
        if !kick && e.level.unwrap_or(0.0) < 1.0 { continue; }
        if who != pid && !g.fight.ops.iter().any(|&o| g.ch[g.gd[g.tab.roomfight_op[o].gd].ch].id == who) { continue; }
        let mut p = e.pos;
        if kick {
            let mut bd = 2.5;
            for j in 0..g.doors.list.len() {
                let fc = door_centre(g, g.doors.list[j]);
                let q = flat(fc, e.pos);
                if q < bd { bd = q; p = fc; }
            }
        }
        heard_at(state_of(g), e.time, p);
    }
    let know = g.k.pressure.stun_know_secs;
    let st = state_of(g);
    if st.heard.len() > 24 || (!st.heard.is_empty() && now - st.heard[0].t > know) {
        let mut kept: Vec<crate::pressure::StateHeardItem> = st.heard.iter().filter(|h| now - h.t <= know).copied().collect();
        if kept.len() > 24 { kept.drain(..kept.len() - 24); }
        st.heard = kept;
    }
    // (only the entry walked on is ever deleted: a copy of the keys walks the same)
    for door in state_of(g).marks.keys() {
        let k = *state_of(g).marks.get(door).unwrap();
        let r = match crate::tactics::danger_at(g, door, k.team) { Some(r) => r, None => { state_of(g).marks.delete(door); continue; } };
        let rec = &g.tab.tactics_danger_rec[r];
        let (why, banged, entered) = (rec.why, rec.banged, rec.entered);
        let own = if why != crate::tactics::DangerWhy::Press { state_of(g).marks.get_mut(door).unwrap().own = false; false } else { k.own };
        let room = match k.r { Some(x) if own && !banged && !entered => x, _ => continue };
        if !team_placed(g, room).moved { continue; }
        // their newest word puts the team somewhere else: nobody bangs an empty room on an old one
        let rec = &mut g.tab.tactics_danger_rec[r];
        rec.until = m::min(rec.until, now);
        let st = state_of(g);
        st.marks.delete(door);
        st.stats.stun_off += 1.0;
        let men: Vec<PressureManId> = st.of.values().copied().collect();
        for man in men {
            let x = &mut g.tab.pressure_man[man];
            if x.door == door && x.charged == Some(true) { x.charged = Some(false); }
        }
        let mut by = None;
        for i in 0..g.fight.defs.len() { let x = g.fight.defs[i]; if alive(g, x) { by = Some(x); break; } }
        if let Some(x) = by {
            let what = format!("the charge at {} is off: the team is known to be somewhere else", door_name(g, door));
            let who = gd_of(g, x);
            note(g, who, &what);
        }
    }
    for r in crate::tactics::danger_doors(g, None) {
        let rec = &g.tab.tactics_danger_rec[r];
        let (door, team, banged) = (rec.door, rec.team, rec.banged);
        let key = (door, team);
        if state_of(g).bangs.get(key).copied() == Some(banged) || !g.fight.defs.iter().any(|&x| g.gd[g.tab.roomfight_def[x].gd].team == team) { continue; }
        let st = state_of(g);
        st.bangs.set(key, banged);
        if !banged { continue; }
        let room = st.marks.get(door).and_then(|k| k.r);
        st.stats.stun_bangs += 1.0;
        let c = door_centre(g, door);
        let mut any = false;
        for i in 0..g.fight.ops.len() {
            let o = op_gd(g, i);
            if g.gd[o].state != crate::game::GuardState::Dead { let p = pos_of(g, o); if bang_by(g, room, c, p) { any = true; break; } }
        }
        let pp = player_pos(g);
        if !any && !bang_by(g, room, c, pp) { state_of(g).stats.stun_empty += 1.0; }
    }
}
/// handed to the fight AI as the stock rally hands a man over: off his post, roused, the fight's place his fix. (pressure.ts loose)
pub fn loose(g: &mut crate::game::Game, d: RoomfightDefId, at: V3) {
    let gd = gd_of(g, d);
    if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert)); }
    let now = g.time;
    let de = dfm(g, d);
    de.mode = crate::roomfight::DefMode::Free; de.free_at = now; de.rally_at = m::NAN;
    let man = &mut g.gd[gd];
    man.hold = false; man.pinned = false;
    man.state = crate::game::GuardState::Alert;
    man.awareness = m::max(man.awareness, 0.92);
    if now - man.last_seen_t > 1.0 { man.last_known = Some(at); }
    man.path = Vec::new();
    man.path_goal = None;
}
/// a man on a scripted leg files his own boots, so the team's ears get them. (pressure.ts boots)
pub fn boots(g: &mut crate::game::Game, gd: GuardId, last: f64) -> f64 {
    let speed = g.gd[gd].speed;
    if speed < 0.4 || g.time - last < (if speed > 2.0 { 0.33 } else { 0.5 }) { return last; }
    let c = g.gd[gd].ch;
    let serial = g.serial();
    let ev = crate::game::GameEvent {
        kind: crate::director::EventKind::Step, pos: g.ch[c].pos, time: g.time, loud: false,
        level: Some(if speed > 2.0 { 0.8 } else { 0.4 }), who: Some(g.ch[c].id), serial,
        ..crate::game::GameEvent::zeroed()
    };
    g.events.push(ev);
    g.time
}
/// a teammate's boots in his ears, and where they come to him from, by the rule every man's ears use on an enemy's step. (pressure.ts teamBoots)
pub fn team_boots(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> {
    let here = pos_of(g, gd);
    let ear = v3::add(here, [0.0, 1.5, 0.0]);
    let mut best: Option<V3> = None;
    let mut bl = 0.12;
    for i in 0..g.fight.ops.len() {
        let o = op_gd(g, i);
        let op = pos_of(g, o);
        let speed = g.gd[o].speed;
        if g.gd[o].state == crate::game::GuardState::Dead || speed < 0.5 || v3::dist(op, here) > 9.0 { continue; }
        let pr = crate::weather::hear_path(g, v3::add(op, [0.0, 1.0, 0.0]), ear, None);
        let l = crate::teampace::team_tread(g, o, speed, 0.4) * v3::clamp(1.0 - pr.path_len / 9.0, 0.0, 1.0) * pr.carry;
        if l > bl { bl = l; best = Some(pr.via); }   // where it comes to him FROM: the doorway it spills through, the man himself only with nothing between
    }
    best
}
/// (pressure.ts hearsTeam)
pub fn hears_team(g: &mut crate::game::Game, gd: GuardId) -> bool { team_boots(g, gd).is_some() }
/// planted outside a door: that door is what he watches. (pressure.ts watchDoor)
pub fn watch_door(g: &mut crate::game::Game, d: RoomfightDefId, door: DoorId) {
    let room = match df(g, d).room { Some(r) => r, None => return };
    let i = match g.tab.roomfight_room[room].entries.iter().position(|e| e.door == door) { Some(i) => i, None => return };
    let at = g.tab.roomfight_room[room].entries[i].at;
    bump_ev(g, d, i, 1.5);
    let de = dfm(g, d);
    de.want = i as f64; de.watch = i as f64;
    de.home = at;
    de.pt[0] = at[0]; de.pt[1] = 0.0; de.pt[2] = at[2];
}

/// a far fight he placed or was told of, while it is still worth going to. (pressure.ts toldOf)
pub fn told_of(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::pressure::StateToldValue> {
    let (now, memory) = (g.time, g.k.pressure.memory_secs);
    match state_of(g).told.get(d) { Some(t) if now - t.t_2 < memory => Some(*t), _ => None }
}
/// PRESSURE.byWay: his room and the room being fought over share a door. (pressure.ts nextDoor)
pub fn next_door(g: &mut crate::game::Game, r: Option<RoomfightRoomId>, t: RoomfightRoomId) -> bool {
    let r = match r { Some(r) => r, None => return false };
    let (a, b) = (&g.tab.roomfight_room[r].entries, &g.tab.roomfight_room[t].entries);
    a.iter().any(|e| b.iter().any(|x| x.door == e.door))
}
#[derive(Clone, Copy)]
struct Way { e: crate::roomfight::Entry, spot: V3, l: f64 }
/// a holding man with a fight at a mate's post in his ears: after his own beat he joins the press on that room (or starts it). (pressure.ts consider)
pub fn consider(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = gd_of(g, d);
    let again = g.k.pressure.again_secs;
    let hold = crate::roomfight::DefMode::Hold;
    // (rallyAt Infinity = somebody pinned him to his room: not ours either)
    let ok0 = df(g, d).mode == hold && !state_of(g).of.has(d) && df(g, d).noise_out && g.gd[gd].held.is_none() && df(g, d).rally_at != m::INF
        && g.time >= g.gd[gd].dazzled_until && g.time >= g.gd[gd].stagger_until
        && g.time - df(g, d).noise_t < g.k.pressure.memory_secs && g.time - g.gd[gd].last_seen_t > 3.0
        && g.time - state_of(g).done_at.get(d).copied().unwrap_or(-100.0) > again && !state_of(g).outs.iter().any(|o| o.def == d)
        && !crate::laststand::last_stand_man(g, gd)
        && !{ let n = df(g, d).noise_at; crate::lanes::lane_holds(g, gd, n) }
        && !crate::enemysquads::squad_owns(g, gd)
        && !(g.k.digin.on && crate::digin::dig_in_stays(g, gd));
    // the last of them alive has no others to go to, and nobody on the net to call: he keeps his room
    let lone = last_alone(g, d);
    let ok = ok0 && !lone;
    if lone { state_of(g).stay.delete(d); }
    if ok0 && !ok && !state_of(g).last_held.has(d) && aim(g, d).is_some() {
        let st = state_of(g);
        st.last_held.add(d);
        st.stats.last_held += 1.0;
        note(g, gd, "the last of them alive: keeps his room, nobody to go to");
    }
    let told = told_of(g, d);
    let heard = if ok { aim(g, d) } else { None };
    if let (Some(h), Some(t)) = (heard, told) {
        let room = df(g, d).room;
        if h.t != t.t && next_door(g, room, h.t) { state_of(g).told.delete(d); }
    }
    let tg: Option<(RoomfightRoomId, V3)> = match heard {
        Some(h) => Some((h.t, h.at)),
        None => match told { Some(t) if ok && !report_stands(g, d) && state_of(g).told.has(d) => Some((t.t, t.at)), _ => None },
    };
    if let Some((tt, _)) = tg {
        if state_of(g).stay.get(d).copied() == Some(tt) && !stays_home(g, d) { state_of(g).stay.delete(d); }
    }
    let (tt, tat) = match tg {
        Some((tt, tat)) if state_of(g).stay.get(d).copied() != Some(tt) => (tt, tat),
        _ => { state_of(g).think.delete(d); return; }
    };
    let my_room = df(g, d).room;
    let defs: Vec<RoomfightDefId> = g.fight.defs.clone();
    let mut mates: Vec<RoomfightDefId> = Vec::new();
    for &x in &defs {
        if x != d && alive(g, x) && df(g, x).room == my_room && df(g, x).mode == hold && !state_of(g).of.has(x) { mates.push(x); }
    }
    let th = state_of(g).think.get(d).copied();
    let th = match th {
        Some(th) if th.room == tt => th,
        _ => {
            let beat = crate::roomfight::fight_skill().get(String::from("hostile")).unwrap().rally;
            let at = g.time + pick(g, beat) * g.k.pressure.organise_k;
            state_of(g).think.set(d, crate::pressure::StateThinkValue { room: tt, at, held: None });
            // PRESSURE.byWay, a fight that is not next door: he says what he has placed, and the men of his room who heard it and could not place it know it from him
            if g.k.pressure.by_way && heard.is_some() && told.map(|t| t.t) != heard.map(|h| h.t) && my_room.is_some() && !next_door(g, my_room, tt) {
                let now = g.time;
                state_of(g).told.set(d, crate::pressure::StateToldValue { t: tt, at: tat, t_2: now });
                let mut n = 0;
                for &x in &defs {
                    if x != d && alive(g, x) && df(g, x).room == my_room && df(g, x).mode == hold && !state_of(g).of.has(x) && !state_of(g).think.has(x)
                        && told_of(g, x).map(|t| t.t) != Some(tt) && df(g, x).noise_out && now - df(g, x).noise_t < 1.5 {
                        state_of(g).told.set(x, crate::pressure::StateToldValue { t: tt, at: tat, t_2: now });
                        n += 1;
                    }
                }
                if n > 0 {
                    state_of(g).stats.pairs_told += 1.0;
                    let line = crate::words::wg(g, &"they've hit the others — get ready, we go together!", &"that's the others they've hit — get ready, we go together!");
                    crate::game::game_say(g, gd, line, Some(false), None, None);
                    note(g, gd, &format!("by way: tells {} of his room where the fight is", n));
                }
            }
            return;
        }
    };
    if g.time < th.at { return; }
    // PRESSURE.byWay: a fight that is not next door: the men of one room go together, on the slower man's beat, and nobody is kept
    let by_way = g.k.pressure.by_way;
    let mut others: Vec<RoomfightDefId> = Vec::new();
    if by_way {
        for &x in &defs {
            if x != d && alive(g, x) && df(g, x).room == my_room && df(g, x).mode != crate::roomfight::DefMode::Free && !state_of(g).of.has(x) && g.gd[gd_of(g, x)].held.is_none() && df(g, x).rally_at != m::INF { others.push(x); }
        }
    }
    let far_room = by_way && my_room.is_some() && !next_door(g, my_room, tt);
    let far = far_room && !others.is_empty();
    if far {
        let mut wait = false;
        for &x in &others {
            let o = gd_of(g, x);
            // a mate on his beat toward the same fight, or told of it and free to come
            let w = match state_of(g).think.get(x).copied() {
                Some(tx) => tx.room == tt && tx.at > g.time,
                None => told_of(g, x).map(|t| t.t) == Some(tt) && df(g, x).mode == hold && g.time >= g.gd[o].dazzled_until && g.time >= g.gd[o].stagger_until
                    && g.time - g.gd[o].last_seen_t > 3.0 && g.time - state_of(g).done_at.get(x).copied().unwrap_or(-100.0) > again,
            };
            if w { wait = true; }
        }
        if wait && g.time - th.held.unwrap_or(g.time) < PAIR_WAIT_SECS * m::max(1.0, g.k.pressure.organise_k) {
            if th.held.is_none() {
                let now = g.time;
                let st = state_of(g);
                st.think.get_mut(d).unwrap().held = Some(now);
                st.stats.pairs_waited += 1.0;
                let what = format!("by way: room {} is not next door, waits for his mate", g.tab.roomfight_room[tt].id);
                note(g, gd, &what);
            }
            return;
        }
        state_of(g).stats.pairs_went += 1.0;
    }
    state_of(g).think.delete(d);
    // somebody keeps the room: of several holding it, the one farthest from the fight stays at home
    if !far && !stood_to(g) && !mates.is_empty() {
        let st = state_of(g);
        if st.stay.get(d).copied() != Some(tt) && !mates.iter().any(|&x| st.stay.get(x).copied() == Some(tt)) {
            let mut keeper = d;
            let mut fd = flat(df(g, d).station, tat);
            for &x in &mates {
                let q = flat(df(g, x).station, tat);
                if q > fd { fd = q; keeper = x; }
            }
            state_of(g).stay.set(keeper, tt);
            if keeper == d { note(g, gd, "stays to keep his room"); return; }
        }
    }
    let way_out_now = way_out(g);
    let forms_now = forms(g);
    let now = g.time;
    // (the way out: a hold the team has already come out into is not joined, and no second one is formed)
    if way_out_now && state_of(g).presses.iter().any(|p| p.room == tt && p.sprung == Some(true)) { state_of(g).done_at.set(d, now); return; }
    let found = state_of(g).presses.iter().position(|p| p.room == tt && (if forms_now { p.phase == PressPhase::Gather } else { now - p.t0 < 25.0 }));
    // (a read of the men at this moment: nothing below writes a row of theirs)
    let mut up: Vec<crate::pressure::Man> = Vec::new();
    if let Some(pi) = found {
        let men: Vec<PressureManId> = state_of(g).presses[pi].men.clone();
        for man in men { let x = g.tab.pressure_man[man]; if !x.out && alive(g, x.d) { up.push(x); } }
    }
    // (PRESSURE.byWay: a far room that goes together is not cut in two by the count)
    if !stood_to(g) && up.len() as f64 >= g.k.pressure.max_men && !(far_room && (!others.is_empty() || up.iter().any(|x| x.from == my_room))) { state_of(g).done_at.set(d, now); return; }
    let held = if way_out_now && !up.is_empty() { Some(up[0].door) } else { None };   // (the way out: one door, the first man's)
    let entries: Vec<crate::roomfight::Entry> = g.tab.roomfight_room[tt].entries.clone();
    let mut ways: Vec<Way> = Vec::new();
    for &e in &entries {
        if g.dr[e.door].def.steel.unwrap_or(false) || held.map_or(false, |h| e.door != h) { continue; }
        if let Some(w) = gather_spot(g, d, tt, e) { ways.push(Way { e, spot: w.spot, l: w.l }); }
    }
    if ways.is_empty() {   // no spot outside any door of it that he can reach: forming up is off; going at it is not
        if forms_now { state_of(g).done_at.set(d, now); note(g, gd, "no way to a door of that room"); return; }
        ways.push(Way { e: entries[0], spot: pos_of(g, gd), l: 0.0 });
    }
    js::sort_by(&mut ways, |a, b| a.l - b.l);
    let mut way = ways[0];
    let mut role = ManRole::Press;
    // PRESSURE.byWay: the room goes by ONE door: the second man of a far room takes the door and the role the first took
    let lead = if far_room { up.iter().find(|x| x.from == my_room).copied() } else { None };
    if let Some(l) = lead {
        if let Some(x) = ways.iter().find(|x| x.e.door == l.door) { way = *x; }
        role = l.role;
    } else if !up.is_empty() && !way_out_now {   // (the way out: nobody goes round)
        if up.iter().any(|x| x.door == way.e.door) {
            let k = &g.k.pressure;
            let reach = m::min(k.flank_max_m, m::max(k.flank_k * ways[0].l, ways[0].l + k.flank_slack_m));
            if let Some(x) = ways.iter().find(|x| !up.iter().any(|u| u.door == x.e.door) && x.l <= reach) { way = *x; role = ManRole::Flank; }
        }
    }
    let st = state_of(g);
    let pi = match found {
        None => {
            st.serial += 1.0;
            st.presses.push(crate::pressure::Press {
                id: st.serial, room: tt, at: tat, t0: now, phase: if forms_now { PressPhase::Gather } else { PressPhase::Go }, team_in: false, set_at: -1.0,
                lone_at: None, sprung: None, listen_at: 0.0, moving: false, word_at: 0.0, go_at: now, n0: 0.0, lone: false, men: Vec::new(),
            });
            st.stats.press_formed += 1.0;
            st.presses.len() - 1
        }
        Some(pi) => {
            let p = &mut st.presses[pi];
            if p.phase == PressPhase::Gather && (up.is_empty() || p.lone) {   // a press that had come down to one man holding a door: the clock starts again for the two of them
                p.t0 = now; p.set_at = -1.0; p.moving = false;
                if p.lone_at.is_some() { p.lone_at = Some(now); }
            }
            pi
        }
    };
    let (spot, door) = (way.spot, way.e.door);
    let man = g.tab.pressure_man.push(crate::pressure::Man {
        d, role, door, spot, from: my_room, moving: true, set: false, set_at: 0.0, tries: 0.0, step_t: -100.0, small0: 0.0, cue_t: now, free_at: 0.0, land_t: -1.0,
        arrived: false, heard: false, fell: false, replant_t: -1.0, fought: false, out: false, ask_t: None, charged: None, ear_t: None,
    });
    let st = state_of(g);
    st.presses[pi].men.push(man);
    st.of.set(d, man);
    st.called.set(d, crate::pressure::StateCalledValue { t: tt, t_2: now });
    if let Some(r) = my_room { st.sent.add(r); }
    let (p_id, p_room, p_at, p_phase) = (st.presses[pi].id, st.presses[pi].room, st.presses[pi].at, st.presses[pi].phase);
    if p_phase == PressPhase::Go {   // no forming up: a presser goes at the fight now, a flanker runs round to his door first
        st.presses[pi].n0 += 1.0;
        g.tab.pressure_man[man].free_at = now;
        if role == ManRole::Press {
            let x = &mut g.tab.pressure_man[man];
            x.moving = false;
            x.set = true;
            let c = charge(g, d, door, Some(p_room));
            g.tab.pressure_man[man].charged = Some(c);
            loose(g, d, p_at);
        } else {
            run(g, d, spot, Some(door), None);
        }
    } else {
        run(g, d, spot, Some(door), None);
    }
    if role == ManRole::Flank {
        state_of(g).stats.flank_sent += 1.0;
        let line = crate::words::wg(g, &"going around — keep them there!", &"going round — keep them there!");
        crate::game::game_say(g, gd, line, Some(true), None, None);
    } else {
        state_of(g).stats.press_men += 1.0;
        let line = if forms_now {
            if !up.is_empty() { "with you — on the door!" } else { "they've hit the others — on that door!" }
        } else if !up.is_empty() { "with you — moving!" } else { "they've hit the others — get in there!" };
        crate::game::game_say(g, gd, line, Some(true), None, None);
    }
    let what = format!("press {} on room {}: {} to {} ({:.1} m)", p_id, g.tab.roomfight_room[tt].id, role.as_str(), door_name(g, door), way.l);
    note(g, gd, &what);
}

/// `m` out of the press. (pressure.ts drop)
pub fn drop(g: &mut crate::game::Game, m: PressureManId) {
    let x = &mut g.tab.pressure_man[m];
    x.out = true;
    let d = x.d;
    let now = g.time;
    let st = state_of(g);
    if st.of.get(d).copied() == Some(m) { st.of.delete(d); }
    st.done_at.set(d, now);
}
/// one frame of a press: its men, then its phase. True = it is over. `p` is lent by pressTick, out of st.presses for the call: nothing under this reads st.presses. (pressure.ts tickPress)
pub fn tick_press(g: &mut crate::game::Game, p: &mut crate::pressure::Press) -> bool {
    let mut team_in = false;
    for i in 0..g.fight.ops.len() {
        let o = op_gd(g, i);
        if g.gd[o].state != crate::game::GuardState::Dead {
            let c = cell_of(g, pos_of(g, o));
            if in_cells(g, p.room, c) { team_in = true; break; }
        }
    }
    if team_in { p.team_in = true; }
    for i in 0..p.men.len() {
        let man = p.men[i];
        if g.tab.pressure_man[man].out { continue; }
        tick_man(g, p, man, team_in);
    }
    let up: Vec<PressureManId> = p.men.iter().copied().filter(|&x| !g.tab.pressure_man[x].out).collect();
    if p.phase == PressPhase::Gather { return tick_gather(g, p, &up); }
    if p.phase == PressPhase::Word {
        if g.time - p.word_at < g.k.pressure.word_secs { return false; }
        p.phase = PressPhase::Go; p.go_at = g.time; p.n0 = up.len() as f64;
        for &x in &up {
            let now = g.time;
            let row = &mut g.tab.pressure_man[x];
            row.moving = false;
            row.free_at = now;
            let d = row.d;
            loose(g, d, p.at);
        }
        return false;
    }
    // 'go': over when every man of it is down, back at a post, or it is old
    if up.iter().any(|&x| { let r = &g.tab.pressure_man[x]; r.moving && df(g, r.d).mode == crate::roomfight::DefMode::Move }) && g.time - p.go_at < 90.0 { return false; }
    let now = g.time;
    if up.is_empty() || now - p.go_at > 45.0 || up.iter().all(|&i| {
        let x = &g.tab.pressure_man[i];
        df(g, x.d).mode == crate::roomfight::DefMode::Hold && !x.moving && (if x.fell { x.fought || now - x.replant_t > 30.0 } else { now - p.go_at > 30.0 })
    }) {
        for &x in &up { drop(g, x); }
        return true;
    }
    false
}
/// one man of a press, this frame (the body of tickPress's walk)
fn tick_man(g: &mut Game, p: &mut Press, mi: PressureManId, team_in: bool) {
    let d = g.tab.pressure_man[mi].d;
    let gd = gd_of(g, d);
    if !alive(g, d) || !g.fight.defs.contains(&d) { drop(g, mi); return; }
    let here = pos_of(g, gd);
    let cell = cell_of(g, here);
    let (door, role) = (g.tab.pressure_man[mi].door, g.tab.pressure_man[mi].role);
    if !g.tab.pressure_man[mi].arrived && (flat(here, door_centre(g, door)) < 2.5 || in_cells(g, p.room, cell)) {   // (a count only: he is at the room, and LATE when the team has been in it and gone)
        g.tab.pressure_man[mi].arrived = true;
        let late = p.team_in && !team_in;
        let s = &mut state_of(g).stats;
        if role == ManRole::Flank { s.flank_arrived += 1.0; if late { s.flank_late += 1.0; } } else { s.press_arrived += 1.0; if late { s.press_late += 1.0; } }
        if late { let what = format!("press {}: at {}, the team has moved on", p.id, door_name(g, door)); note(g, gd, &what); }
    }
    if g.tab.pressure_man[mi].land_t < 0.0 && !g.tab.pressure_man[mi].fell {   // the man himself arrives: he has eyes on the enemy, or the enemy on him
        let cue_t = g.tab.pressure_man[mi].cue_t;
        if !g.tab.pressure_man[mi].heard && g.fight.ops.iter().any(|&o| g.tab.roomfight_op[o].step_by.as_ref().map_or(false, |s| s.by == gd && s.t > cue_t)) { g.tab.pressure_man[mi].heard = true; }
        if g.time - g.gd[gd].last_seen_t < 0.1 || crate::roomfight::room_fight_team_sees(g, gd) {
            let now = g.time;
            g.tab.pressure_man[mi].land_t = now;
            let lead = now - cue_t;
            let first = lead >= CUE_FIRST_SECS;
            let heard = g.tab.pressure_man[mi].heard;
            let s = &mut state_of(g).stats;
            if role == ManRole::Flank { s.flank_landed += 1.0; s.flank_lead += lead; if first { s.flank_cue_first += 1.0; } if heard { s.flank_heard += 1.0; } }
            else { s.press_landed += 1.0; s.press_lead += lead; if first { s.press_cue_first += 1.0; } if heard { s.press_heard += 1.0; } }
            let what = format!("press {}: lands {:.1} s after his shout{}", p.id, lead, if heard { ", boots heard" } else { "" });
            note(g, gd, &what);
        }
    }
    if df(g, d).mode == crate::roomfight::DefMode::Move {
        if !g.tab.pressure_man[mi].moving { drop(g, mi); return; }   // (not moving = the stock fall-back took him: he is its man now)
        let step = boots(g, gd, g.tab.pressure_man[mi].step_t);
        g.tab.pressure_man[mi].step_t = step;
        return;
    }
    if p.phase != PressPhase::Go {
        if df(g, d).mode == crate::roomfight::DefMode::Free { drop(g, mi); return; }
        if g.tab.pressure_man[mi].moving {   // he has planted (roomfight.ts tickMove): there, in contact on the way, or stopped short
            g.tab.pressure_man[mi].moving = false;
            let contact = g.time - g.gd[gd].last_seen_t < 2.0;
            let spot = g.tab.pressure_man[mi].spot;
            let near = flat(pos_of(g, gd), spot) < 1.5;
            if contact || near || { let x = &mut g.tab.pressure_man[mi]; x.tries += 1.0; x.tries >= 3.0 } {
                let (now, small) = (g.time, g.gd[gd].witness.heard.small);
                let x = &mut g.tab.pressure_man[mi];
                x.set = true; x.set_at = now; x.small0 = small;
                if !contact { watch_door(g, d, door); }
                if role == ManRole::Flank {
                    state_of(g).stats.flank_set += 1.0;
                    if !contact { crate::game::game_say(g, gd, "in position!", Some(true), None, None); }
                }
                let what = format!("press {}: set at {}{}", p.id, door_name(g, door), if contact { " (in contact)" } else { "" });
                note(g, gd, &what);
            } else {
                g.tab.pressure_man[mi].moving = true;
                run(g, d, spot, Some(door), None);
            }
        }
        return;
    }
    // ---- let go at the fight
    if g.tab.pressure_man[mi].fell {
        let now = g.time;
        if g.tab.pressure_man[mi].moving && df(g, d).mode == crate::roomfight::DefMode::Hold {
            let x = &mut g.tab.pressure_man[mi];
            x.moving = false; x.replant_t = now;
            watch_door(g, d, door);
            state_of(g).stats.fb_planted += 1.0;
            let what = format!("press {}: back outside {}", p.id, door_name(g, door));
            note(g, gd, &what);
        }
        let x = g.tab.pressure_man[mi];
        if x.replant_t >= 0.0 && !x.fought && g.gd[gd].last_seen_t > x.replant_t + 0.2 { g.tab.pressure_man[mi].fought = true; state_of(g).stats.fb_fought += 1.0; }
        return;
    }
    if g.tab.pressure_man[mi].moving {   // a flanker off his leg: round at his door, or met on the way, or stopped short and he sets off again
        let contact = g.time - g.gd[gd].last_seen_t < 0.5;
        let fc: Option<CharId> = match g.gd[gd].foe { Some(crate::game::Foe::Player) => Some(g.pl[g.player].ch), Some(crate::game::Foe::Guard(o)) => Some(g.gd[o].ch), None => None };
        let spot = g.tab.pressure_man[mi].spot;
        let off = flat(pos_of(g, gd), spot);
        if !contact && off > 1.5 && { let x = &mut g.tab.pressure_man[mi]; x.tries += 1.0; x.tries < 3.0 } { run(g, d, spot, Some(door), None); return; }
        let there = off <= 1.5;
        let now = g.time;
        let x = &mut g.tab.pressure_man[mi];
        x.moving = false; x.set = true; x.set_at = now;
        if role == ManRole::Flank && there && !contact { state_of(g).stats.flank_set += 1.0; crate::game::game_say(g, gd, "in position — on them!", Some(true), None, None); }
        if there && !contact { let c = charge(g, d, door, Some(p.room)); g.tab.pressure_man[mi].charged = Some(c); }
        let to = match fc { Some(c) if contact => g.ch[c].pos, _ => p.at };
        loose(g, d, to);   // (met on the way: the mover's own whole beat, roomfight.ts tickMove's: nothing here shortens it)
        let what = format!("press {}: {}{}", p.id, if there { format!("round at {}", door_name(g, door)) } else { String::from("let go short of his door") }, if contact { ", in contact" } else { "" });
        note(g, gd, &what);
    }
    // PRESSURE.stunKnown: asked for the charge and turned down: he listens at the door as he comes to it, and the moment his side has them in that room the door is the drill, charge first.
    // Only a man who was turned down asks again (charged false: one never asked is not)
    let x = g.tab.pressure_man[mi];
    if g.k.pressure.stuns > 0.0 && x.charged == Some(false) && x.set && !x.moving && g.time - p.go_at < 30.0 && crate::tactics::stuns_of(g, gd) > 0.0 && g.time - g.gd[gd].last_seen_t > 0.5 && {
        let c = cell_of(g, pos_of(g, gd));
        !in_cells(g, p.room, c)
    } {
        let fc = door_centre(g, door);
        if flat(pos_of(g, gd), fc) < 5.0 && g.time - x.ear_t.unwrap_or(-1.0) > 0.3 {
            let now = g.time;
            g.tab.pressure_man[mi].ear_t = Some(now);
            if let Some(via) = team_boots(g, gd) {
                let inside = in_room(g, p.room, via);
                if inside || flat(via, fc) < 1.5 { heard_at(state_of(g), now, if inside { via } else { fc }); }
            }
        }
        if team_placed(g, p.room).known {
            let c = charge(g, d, door, Some(p.room));
            g.tab.pressure_man[mi].charged = Some(c);
            if c {
                state_of(g).stats.stun_late += 1.0;
                let what = format!("press {}: they are in there — {} charge first after all", p.id, door_name(g, door));
                note(g, gd, &what);
            }
        }
    }
    if g.time - g.tab.pressure_man[mi].free_at > 0.5 && g.time >= g.gd[gd].dazzled_until && g.time - p.go_at < 30.0 {
        // the push has failed for him when enough of it lies where he can see it and the enemy is on him: back out to the door he came by
        let eye = eye_of(g, gd);
        let here = pos_of(g, gd);
        let (mut dead, mut up) = (0.0, 0.0);
        for j in 0..p.men.len() {
            if p.men[j] == mi { continue; }
            let x = g.tab.pressure_man[p.men[j]];
            if alive(g, x.d) { if !x.fell && !x.out { up += 1.0; } continue; }
            let xg = gd_of(g, x.d);
            let xc = g.gd[xg].ch;
            let b = g.ch[xc].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[xc].pos);
            if flat(b, here) < 6.0 && !crate::scene_collision::segment_blocked(g, eye, [b[0], 0.3, b[2]], None, None, None) { dead += 1.0; } else { up += 1.0; }   // (a man down where he cannot see him is, to him, a man still up)
        }
        let pressed = g.time - g.gd[gd].last_seen_t < 4.0 || g.time - g.gd[gd].witness.heard.last_shot_t < 2.5;
        if dead > 0.0 && pressed && (up == 0.0 || dead / p.n0 >= df(g, d).nerve) && g.time - g.tab.pressure_man[mi].ask_t.unwrap_or(-1.0) > 0.5 {
            let now = g.time;
            g.tab.pressure_man[mi].ask_t = Some(now);   // (with no way back at all he asks again in half a second, not every frame)
            let threat = if now - g.gd[gd].last_seen_t < 4.0 { g.gd[gd].sight_pos } else { p.at };
            let to = hooked(g, d, HookedWhy::Push, threat);
            let spot = g.tab.pressure_man[mi].spot;
            if to.is_some() || crate::scene_collision::nav_grid_find_path(g, here, spot, None, None).map_or(false, |w| !w.is_empty()) {
                let x = &mut g.tab.pressure_man[mi];
                x.fell = true; x.moving = true;
                let who = &mut g.gd[gd];
                who.hold = true; who.path = Vec::new(); who.path_goal = None;
                if let Some(t) = to {
                    let x = &mut g.tab.pressure_man[mi];
                    x.spot = t.spot;
                    if let Some(door) = t.door { x.door = door; }
                    run(g, d, t.spot, t.door, Some(0.3));
                } else {
                    run(g, d, spot, Some(door), Some(0.3));
                }
                state_of(g).stats.fb_push += 1.0;
                crate::game::game_say(g, gd, "back — get back! hold the door!", Some(true), None, None);
                let what = format!("press {}: the push has failed, back to {}", p.id, door_name(g, g.tab.pressure_man[mi].door));
                note(g, gd, &what);
            }
        }
    }
}
/// how many doors the men `up` of a press are put down for
fn doors_of(g: &Game, up: &[PressureManId]) -> usize {
    let mut doors: Vec<DoorId> = Vec::new();
    for &i in up { let door = g.tab.pressure_man[i].door; if !doors.contains(&door) { doors.push(door); } }
    doors.len()
}
/// the press forming up on the doors (tickPress's 'gather')
fn tick_gather(g: &mut Game, p: &mut Press, up: &[PressureManId]) -> bool {
    let hold = way_out(g);
    if p.sprung != Some(true) && hold {
        let mut sprung = false;
        for j in 0..p.men.len() {
            let x = g.tab.pressure_man[p.men[j]].d;
            if !alive(g, x) || g.time - g.gd[gd_of(g, x)].last_seen_t < 1.0 { sprung = true; break; }
        }
        if sprung { p.sprung = Some(true); }   // (the team has come out into the hold: consider() sends nobody else to it)
    }
    if up.is_empty() { return g.time - p.t0 > g.k.pressure.memory_secs; }
    // the word: everybody set and a breath taken, or nobody still on his way has any of his own time left
    let now = g.time;
    let waited = now - p.t0;
    let all_set = up.iter().all(|&i| g.tab.pressure_man[i].set);
    let (gs, fs) = (g.k.pressure.gather_secs, g.k.pressure.flank_secs);
    let due = up.iter().all(|&i| { let x = &g.tab.pressure_man[i]; x.set || now - x.cue_t > gs + (if x.role == ManRole::Flank { fs } else { 0.0 }) });
    if up.len() >= 2 && waited > 1.0 && (all_set || due) {
        let lead_i = up.iter().copied().find(|&i| g.tab.pressure_man[i].set).unwrap_or(up[0]);
        let lead_gd = gd_of(g, g.tab.pressure_man[lead_i].d);
        // set on the doors. A room gone quiet is four rifles waiting: they HOLD its doors from outside, and go in only when `siegeSecs` have brought nobody
        let siege = if hold { g.k.pressure.way_out_secs } else { g.k.pressure.siege_secs };
        if p.set_at < 0.0 {
            p.set_at = now;
            if siege > 0.0 {
                state_of(g).stats.press_sieges += 1.0;
                crate::game::game_say(g, lead_gd, if hold { "back off the doors — out of their flash! let them come out to us!" } else { "hold the doors — let them come to us!" }, Some(true), None, None);
                let what = format!("press {}: set, holds the doors ({} men, {} doors)", p.id, up.len(), doors_of(g, up));
                note(g, lead_gd, &what);
            }
        }
        // the trap sprung at one door is heard at the others: the men with nobody in front of them go THROUGH, at the noise, by their own ears
        if !hold && up.iter().any(|&i| { let x = &g.tab.pressure_man[i]; x.set && now - g.gd[gd_of(g, x.d)].last_seen_t < 2.0 }) {
            let go: Vec<PressureManId> = up.iter().copied().filter(|&i| {
                let x = &g.tab.pressure_man[i];
                let o = gd_of(g, x.d);
                let de = df(g, x.d);
                x.set && now - g.gd[o].last_seen_t > 3.0 && now >= g.gd[o].dazzled_until && de.noise_at.is_some() && now - de.noise_t < 2.0 && de.noise_t > de.moved_at + 0.5
            }).collect();
            if !go.is_empty() {
                p.phase = PressPhase::Go; p.go_at = now; p.n0 = up.len() as f64;
                state_of(g).stats.press_converge += 1.0;
                let first = gd_of(g, g.tab.pressure_man[go[0]].d);
                let line = crate::words::wg(g, &"they're coming out — get around them, go!", &"they're coming out — get round them, go!");
                crate::game::game_say(g, first, line, Some(true), None, None);
                let what = format!("press {}: the trap is sprung, {} go round", p.id, go.len());
                note(g, first, &what);
                for &i in up {
                    g.tab.pressure_man[i].free_at = now;
                    if go.contains(&i) {
                        g.tab.pressure_man[i].moving = false;
                        let x = g.tab.pressure_man[i].d;
                        let at = df(g, x).noise_at.unwrap();
                        loose(g, x, at);
                    }
                }
                return false;
            }
        }
        let calm = up.iter().all(|&i| now - g.gd[gd_of(g, g.tab.pressure_man[i].d)].last_seen_t > 3.0);
        let mut fight = false;
        if !hold && calm {
            for &i in up {
                let de = df(g, g.tab.pressure_man[i].d);
                let (noise, nt, moved) = (de.noise_at, de.noise_t, de.moved_at);
                if let Some(at) = noise {
                    if now - nt < 2.0 && nt > moved + 0.5 {
                        let c = cell_of(g, at);
                        if in_cells(g, p.room, c) { fight = true; break; }
                    }
                }
            }
        }
        if now - p.set_at < 0.6 || !(fight || (calm && (p.moving || now - p.set_at > siege))) { return false; }
        p.phase = PressPhase::Word; p.word_at = now; p.lone = false;
        state_of(g).stats.press_words += 1.0;
        crate::game::game_say(g, lead_gd, if p.moving && !fight { "they're moving — go, go!" } else { "go — go — go!" }, Some(true), None, None);
        let what = format!("press {}: the word{} ({} men, {} doors)", p.id, if fight { ", a fight inside" } else if p.moving { ", boots inside" } else { "" }, up.len(), doors_of(g, up));
        note(g, lead_gd, &what);
    } else if p.lone && (match p.lone_at { Some(t) => now - t > 40.0, None => waited > gs + 40.0 }) {
        for &i in up { drop(g, i); }   // (he stays where he planted, the stock room fight's holder again)
        return true;
    } else if up.len() == 1 && g.tab.pressure_man[up[0]].set && !p.lone && waited > gs {
        p.lone = true;
        state_of(g).stats.press_lone += 1.0;
        if hold { p.lone_at = Some(now); }   // (the way out holds long: his forty seconds run from now, not from the forming up)
        let who = gd_of(g, g.tab.pressure_man[up[0]].d);
        crate::game::game_say(g, who, "I've got this door — they're not coming out!", Some(true), None, None);
        let what = format!("press {}: alone, holds {} from outside", p.id, door_name(g, g.tab.pressure_man[up[0]].door));
        note(g, who, &what);
    }
    false
}
/// the last man up in a room does not die at his post: two of the enemy in view, or the team's boots at one door on a floor gone loud, and after his own beat he gives the room up through another. (pressure.ts lastMan)
pub fn last_man(g: &mut crate::game::Game, d: RoomfightDefId) {
    let gd = gd_of(g, d);
    let c_ch = g.gd[gd].ch;
    let mode_free = crate::roomfight::DefMode::Free;
    // (lostAt ahead of the clock = somebody holds him to his room)
    let room = df(g, d).room;
    let ok = df(g, d).mode == crate::roomfight::DefMode::Hold && !state_of(g).of.has(d) && room.map_or(false, |r| g.tab.roomfight_room[r].cells.len() as f64 <= ROOM_CELLS) && g.gd[gd].held.is_none() && !(df(g, d).lost_at > g.time)
        && g.time >= g.gd[gd].dazzled_until && g.time >= g.gd[gd].stagger_until
        && g.time - df(g, d).moved_at > 6.0 && g.time - state_of(g).tried.get(d).copied().unwrap_or(-100.0) > 6.0 && !state_of(g).outs.iter().any(|o| o.def == d)
        && {
            let mut other = false;
            for i in 0..g.fight.defs.len() {
                let x = g.fight.defs[i];
                if x != d && alive(g, x) && df(g, x).room == room && df(g, x).mode != mode_free { other = true; break; }
            }
            !other
        }
        && !crate::enemysquads::squad_stays_in(g, gd)
        && !crate::laststand::last_stand_man(g, gd);
    if !ok { state_of(g).last.delete(d); return; }
    let r = room.unwrap();
    let mut why: Option<HookedWhy> = None;
    let mut threat: V3 = g.gd[gd].sight_pos;
    let mut shut: Option<usize> = None;
    if g.gd[gd].state == crate::game::GuardState::Alert && g.time - g.gd[gd].last_seen_t < 1.5 {
        let mut seen = 0;   // (his own eyes: the game's sight test, and some light on the man)
        let team = g.gd[gd].team;
        for i in 0..g.fight.ops.len() {
            let o = op_gd(g, i);
            let (ot, oc) = (g.gd[o].team, g.gd[o].ch);
            if g.gd[o].state == crate::game::GuardState::Dead || !crate::teams::is_hostile(g, team, ot) || flat(g.ch[oc].pos, g.ch[c_ch].pos) > 14.0 { continue; }
            let vis = crate::guards::can_see_char(g, gd, oc, false);
            if vis.visible && crate::guards::sight_gain_on(g, gd, o, vis.dist) > 0.0 { seen += 1; }
        }
        if seen >= 2 { why = Some(HookedWhy::Seen); }
    } else if g.tab.roomfight_room[r].entries.len() > 1 && df(g, d).noise_at.is_some() && g.time - df(g, d).noise_t < g.k.pressure.loud_secs && g.time - g.gd[gd].last_seen_t > 3.0
        && !(g.k.digin.on && crate::digin::dig_in_set(g, gd)) {   // (a man SET on his door is waiting for exactly those boots: he holds)
        if !state_of(g).ear.has(d) { state_of(g).ear.set(d, crate::pressure::StateEarValue { t: -1.0, i: -1.0, at: -100.0 }); }
        let mut e = *state_of(g).ear.get(d).unwrap();
        if g.time - e.t > 0.25 {
            e.t = g.time;
            if let Some(p) = team_boots(g, gd) {
                let mut bi: i32 = -1;
                let mut bd = 5.0;
                let n = g.tab.roomfight_room[r].entries.len();
                for i in 0..n {
                    let q = flat(g.tab.roomfight_room[r].entries[i].at, p);
                    if q < bd {
                        let c = cell_of(g, p);
                        if !in_cells(g, r, c) { bd = q; bi = i as i32; }
                    }
                }
                if bi >= 0 { e.i = bi as f64; e.at = g.time; }
            }
            *state_of(g).ear.get_mut(d).unwrap() = e;
        }
        if e.i >= 0.0 && (e.i as usize) < g.tab.roomfight_room[r].entries.len() && g.time - e.at < 3.0 {
            why = Some(HookedWhy::Door);
            shut = Some(e.i as usize);
            threat = g.tab.roomfight_room[r].entries[e.i as usize].at;
        }
    }
    let why = match why { Some(w) => w, None => { state_of(g).last.delete(d); return; } };
    let at = match state_of(g).last.get(d).copied() {
        Some(at) => at,
        None => {
            let range = if why == HookedWhy::Seen { g.k.pressure.last_man_secs } else { g.k.pressure.out_secs };
            let at = g.time + pick(g, range);
            state_of(g).last.set(d, at);
            return;
        }
    };
    if g.time < at { return; }
    state_of(g).last.delete(d);
    let c_pos = g.ch[c_ch].pos;
    let entries: Vec<crate::roomfight::Entry> = g.tab.roomfight_room[r].entries.clone();
    let mut best: Option<(V3, DoorId, f64)> = None;
    for (i, e) in entries.iter().enumerate() {
        if shut == Some(i) || df(g, d).ev.get(i).map_or(false, |v| *v > 0.3) || flat(e.at, threat) < 3.0 || g.dr[e.door].def.steel.unwrap_or(false) || crate::doors::doors_locked_out(g, e.door, c_pos, None) { continue; }
        let fr = crate::guards::frame_of(g, e.door);
        let mut spot: Option<V3> = None;
        for (depth, lat) in SPOTS {
            let p: V3 = [e.at[0] + e.out[0] * depth + fr.ux * lat * (1.0 + fr.w / 3.2), 0.0, e.at[2] + e.out[1] * depth + fr.uz * lat * (1.0 + fr.w / 3.2)];
            if is_blocked(g, p[0], p[2]) { continue; }
            let cell = cell_of(g, p);
            if in_cells(g, r, cell) { continue; }
            if someone_at(g, d, p) { continue; }
            spot = Some(p);
            break;
        }
        let spot = match spot { Some(s) => s, None => continue };
        let path = match crate::scene_collision::nav_grid_find_path(g, c_pos, spot, None, None) { Some(w) if !w.is_empty() => w, _ => continue };
        let (mut l, mut q, mut toward) = (0.0, c_pos, false);
        for w in path {
            l += flat(q, w);
            q = w;
            if flat(w, threat) < 2.0 { toward = true; }
        }
        if l > 18.0 || toward || flat(q, spot) > 0.8 { continue; }
        if best.map_or(true, |b| l < b.2) { best = Some((spot, e.door, l)); }
    }
    let chosen: Option<(V3, Option<DoorId>)> = match hooked(g, d, why, threat) {
        Some(h) => Some((h.spot, h.door)),
        None => best.map(|b| (b.0, Some(b.1))),
    };
    let now = g.time;
    state_of(g).tried.set(d, now);   // (whether or not there was a way out: he does not ask again for a while)
    let cause = if why == HookedWhy::Seen { String::from("two of them in view") } else { format!("boots at {}", door_name(g, entries[shut.unwrap()].door)) };
    let (spot, door) = match chosen {
        Some(x) => x,
        None => {
            let what = format!("last man: {}, no way out of room {}", cause, g.tab.roomfight_room[r].id);
            note(g, gd, &what);
            return;
        }
    };
    run(g, d, spot, door, Some(0.3));
    state_of(g).outs.push(crate::pressure::Out { door, def: d, moving: true, plant_t: -1.0, fought: false, step_t: -100.0 });
    if why == HookedWhy::Seen {
        state_of(g).stats.fb_last += 1.0;
        crate::game::game_say(g, gd, "too many — I'm out the back!", Some(true), None, None);
    } else {
        state_of(g).stats.fb_early += 1.0;
        crate::game::game_say(g, gd, "they're at the door — I'm out the back!", Some(true), None, None);
    }
    let through = match door { Some(x) => format!(" through {}", door_name(g, x)), None => String::new() };
    let what = format!("last man: {}, gives room {} up{}", cause, g.tab.roomfight_room[r].id, through);
    note(g, gd, &what);
}
/// a last man out of his room, this frame. True = done with him. `o` is lent by pressTick, out of st.outs for the call. (pressure.ts tickOut)
pub fn tick_out(g: &mut crate::game::Game, o: &mut crate::pressure::Out) -> bool {
    let d = o.def;
    let gd = gd_of(g, d);
    if !alive(g, d) || !g.fight.defs.contains(&d) { return true; }
    if o.moving {
        if df(g, d).mode == crate::roomfight::DefMode::Move { o.step_t = boots(g, gd, o.step_t); return false; }
        o.moving = false; o.plant_t = g.time;
        state_of(g).stats.fb_planted += 1.0;
        // whoever of his side holds the room he has come into hears it from him: the door he came by is the one to watch
        let here = pos_of(g, gd);
        let room = df(g, d).room;
        let mut told = 0;
        let defs: Vec<RoomfightDefId> = g.fight.defs.clone();
        for &x in &defs {
            if !(x != d && alive(g, x) && df(g, x).mode == crate::roomfight::DefMode::Hold && df(g, x).room == room && flat(pos_of(g, gd_of(g, x)), here) < 12.0) { continue; }
            let i = match (df(g, x).room, o.door) {
                (Some(r), Some(door)) => g.tab.roomfight_room[r].entries.iter().position(|e| e.door == door),
                _ => None,
            };
            if let Some(i) = i { bump_ev(g, x, i, 1.2); told += 1; }
        }
        if told > 0 {
            state_of(g).stats.fb_told += 1.0;
            crate::game::game_say(g, gd, "they're right behind me — watch that door!", Some(true), None, None);
        }
        let beyond = match o.door { Some(x) => format!(" beyond {}", door_name(g, x)), None => String::new() };
        let tells = if told > 0 { format!(", tells {}", told) } else { String::new() };
        let what = format!("last man: plants{}{}", beyond, tells);
        note(g, gd, &what);
        return false;
    }
    let now = g.time;
    if !o.fought && g.gd[gd].last_seen_t > o.plant_t + 0.2 {
        o.fought = true;
        state_of(g).stats.fb_fought += 1.0;
        note(g, gd, "last man: fights again");
        state_of(g).done_at.set(d, now);
        return true;
    }
    if now - o.plant_t > 90.0 { state_of(g).done_at.set(d, now); return true; }
    false
}

/// boots on doors and charges gone off, as their ears have counted them. (pressure.ts charges)
pub fn charges(g: &mut crate::game::Game) -> f64 {
    let mut n = 0.0;
    for i in 0..g.fight.defs.len() {
        let h = &g.gd[gd_of(g, g.fight.defs[i])].witness.heard;
        n = n + h.kick + h.bang;
    }
    n
}
/// the fight is on, by what their side can know of it. (pressure.ts fightOn)
pub fn fight_on(g: &mut crate::game::Game) -> bool {
    if charges(g) > 0.0 { return true; }
    for i in 0..g.fight.defs.len() {
        let x = g.fight.defs[i];
        let o = gd_of(g, x);
        if !alive(g, x) || df(g, x).mode == crate::roomfight::DefMode::Free || g.time < g.gd[o].dazzled_until || g.time - g.gd[o].last_seen_t < 5.0 { return true; }
        if df(g, x).noise_at.is_some() && g.time - df(g, x).noise_t < 5.0 && !df(g, x).noise_out { return true; }
    }
    false
}
/// the report is heard or called and no room is hit yet. (pressure.ts awaiting)
pub fn awaiting(g: &mut crate::game::Game) -> bool {
    g.k.pressure.warned && state_of(g).warn.map_or(false, |w| w.phase != WarnPhase::Over)
}
/// a report was called and a room has since been hit: all hands, for the rest of the fight. (pressure.ts stoodTo)
pub fn stood_to(g: &mut crate::game::Game) -> bool {
    g.k.pressure.warned && state_of(g).warn.map_or(false, |w| w.call_t >= 0.0 && w.phase == WarnPhase::Over)
}
/// a man of the reaction force: a report stands (or its room was hit only just now) and the room it points at is not his. (pressure.ts readyMan)
pub fn ready_man(g: &mut crate::game::Game, d: RoomfightDefId, now: f64) -> bool {
    if !g.k.pressure.warned { return false; }
    let w = match state_of(g).warn { Some(w) => w, None => return false };
    if w.room.is_none() || df(g, d).room == w.room { return false; }
    w.phase != WarnPhase::Over || (w.call_t >= 0.0 && now - w.hit_t.unwrap_or(m::NAN) < g.k.pressure.stand_grace)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn off_line_is_the_distance_beside_the_segment_only() {
        let (a, mouth) = ([0.0, 0.0, 0.0], [2.0, 1.2, 0.0]);
        assert_eq!(off_line([1.0, 0.0, 0.75], a, mouth), 0.75);
        assert_eq!(off_line([-1.0, 0.0, 0.5], a, mouth), m::INF);   // behind a
        assert_eq!(off_line([3.0, 0.0, 0.5], a, mouth), m::INF);    // past the mouth
        assert_eq!(off_line([1.0, 0.0, 1.0], mouth, mouth), m::INF); // a at the mouth: L2 falls to 1, t = 0
    }
}
