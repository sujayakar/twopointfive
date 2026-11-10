//! WHAT HARD'S MEN DO THAT IS NO SQUAD'S: the kit they are given, and the last man up in a room, who does not die at his post: he gives it up by another door and fights on from beyond it.
pub use ::game_types::pressure::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec::Vec};

/// the spots outside a door, (depth, lateral)
const SPOTS: [(f64, f64); 9] = [(3.0, 1.6), (3.0, -1.6), (2.4, 1.2), (2.4, -1.2), (1.2, 2.4), (1.2, -2.4), (1.2, 1.7), (1.2, -1.7), (2.6, 0.0)];

/// where a man who gives ground goes: a room his side dug in, or nothing
fn hooked(g: &mut Game, d: RoomfightDefId, threat: V3) -> Option<(V3, Option<DoorId>)> {
    if !g.k.digin.on { return None; }
    crate::digin::dig_in_refuge(g, d, threat).map(|r| (r.spot, r.door))
}
fn state_of(g: &mut Game) -> &mut State { g.m.pressure.states.get_or_insert_with(State::default) }
fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }
fn pick(g: &mut Game, who: crate::dice::Who, r: [f64; 2]) -> f64 { r[0] + crate::dice::draw_for(g, "pressure::pick#1", who) * (r[1] - r[0]) }
fn alive(g: &Game, d: RoomfightDefId) -> bool { !crate::condition::down(g, g.gd[g.tab.roomfight_def[d].gd].ch) }
/// a line in the fight's log, which keeps its last 400 to 600
fn note(g: &mut Game, gd: GuardId, what: &str) {
    let note = crate::roomfight::FightNote { t: g.time, who: g.gd[gd].callsign.clone(), what: format!("[pressure] {}", what) };
    g.fight.log.push(note);
    if g.fight.log.len() > 600 { g.fight.log.drain(0..200); }
}
/// the nav grid's cell a point falls in
fn cell_of(g: &Game, p: V3) -> f64 { let cell = g.col.nav.cell; crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell)) }
// ---- small reads
fn df(g: &Game, d: RoomfightDefId) -> &crate::roomfight::Def { &g.tab.roomfight_def[d] }
fn gd_of(g: &Game, d: RoomfightDefId) -> GuardId { g.tab.roomfight_def[d].gd }
fn op_gd(g: &Game, i: usize) -> GuardId { g.tab.roomfight_op[g.fight.ops[i]].gd }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn door_name(g: &Game, door: DoorId) -> String { g.dr[door].def.name.clone() }
fn in_cells(g: &Game, t: RoomfightRoomId, cell: f64) -> bool { g.tab.roomfight_room[t].cells.has(crate::Num(cell)) }
fn is_blocked(g: &Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }

/// a live man of the fight other than `d` stands within 0.9 m of p
fn someone_at(g: &Game, d: RoomfightDefId, p: V3) -> bool {
    for i in 0..g.fight.defs.len() {
        let x = g.fight.defs[i];
        if x != d && alive(g, x) && flat(pos_of(g, gd_of(g, x)), p) < 0.9 { return true; }
    }
    false
}

/// `D.ev[i] = Math.max(D.ev[i], v)`: a Def's ev is not always as long as its room's entries, and a read past its end is undefined
fn bump_ev(g: &mut Game, d: RoomfightDefId, i: usize, v: f64) {
    let ev = &mut g.tab.roomfight_def[d].ev;
    if ev.len() <= i { ev.resize(i + 1, m::NAN); }
    ev[i] = m::max(ev[i], v);
}

/// hard: the last of his side alive keeps his room, with nobody left to go to
pub(in crate::defenders) fn last_holds(g: &Game, d: RoomfightDefId) -> bool { g.k.pressure.on && !g.fight.defs.iter().any(|&x| x != d && alive(g, x)) }

/// a teammate's boots in his ears, and where they come to him from, by the rule every man's ears use on an enemy's step.
fn team_boots(g: &mut Game, gd: GuardId) -> Option<V3> {
    let here = pos_of(g, gd);
    let ear = v3::add(here, [0.0, 1.5, 0.0]);
    let mut best: Option<V3> = None;
    let mut bl = 0.12;
    for i in 0..g.fight.ops.len() {
        let o = op_gd(g, i);
        let op = pos_of(g, o);
        let speed = g.gd[o].speed;
        if crate::condition::down(g, g.gd[o].ch) || speed < 0.5 || v3::dist(op, here) > 9.0 { continue; }
        let pr = crate::weather::hear_path(g, v3::add(op, [0.0, 1.0, 0.0]), ear, None);
        let l = crate::teampace::team_tread(g, o, speed, 0.4) * v3::clamp(1.0 - pr.path_len / 9.0, 0.0, 1.0) * pr.carry;
        if l > bl { bl = l; best = Some(pr.via); }   // where it comes to him FROM: the doorway it spills through, the man himself only with nothing between
    }
    best
}

/// the last man up in a room does not die at his post: two of the enemy in view, or the team's boots at one door on a floor gone loud, and after his own beat he gives the room up through another.
fn last_man(g: &mut Game, d: RoomfightDefId) {
    let gd = gd_of(g, d);
    let c_ch = g.gd[gd].ch;
    let mode_free = crate::roomfight::DefMode::Free;
    // (lostAt ahead of the clock = somebody holds him to his room)
    let room = df(g, d).room;
    let ok = crate::roomfight::at_post(g, d) && room.map_or(false, |r| g.tab.roomfight_room[r].cells.len() as f64 <= ROOM_CELLS) && g.gd[gd].held.is_none() && !(df(g, d).lost_at > g.time)
        && !crate::control::man::blind(g, gd) && !crate::control::man::staggered(g, gd)
        && g.time - df(g, d).moved_at > 6.0 && g.time - state_of(g).tried.get(d).copied().unwrap_or(-100.0) > 6.0 && !state_of(g).outs.iter().any(|o| o.def == d)
        && {
            let mut other = false;
            for i in 0..g.fight.defs.len() {
                let x = g.fight.defs[i];
                if x != d && alive(g, x) && df(g, x).room == room && df(g, x).mode != mode_free { other = true; break; }
            }
            !other
        }
        && !g.gd[gd].stays_in_rooms;
    if !ok { state_of(g).last.delete(d); return; }
    let r = room.unwrap();
    let mut why: Option<LastWhy> = None;
    let mut threat: V3 = g.gd[gd].sight_pos;
    let mut shut: Option<usize> = None;
    if g.gd[gd].state == crate::game::GuardState::Alert && g.time - g.gd[gd].last_seen_t < 1.5 {
        let mut seen = 0;   // (his own eyes: the game's sight test, and some light on the man)
        let team = g.gd[gd].team;
        for i in 0..g.fight.ops.len() {
            let o = op_gd(g, i);
            let (ot, oc) = (g.gd[o].team, g.gd[o].ch);
            if crate::condition::down(g, g.gd[o].ch) || !crate::teams::is_hostile(g, team, ot) || flat(g.ch[oc].pos, g.ch[c_ch].pos) > 14.0 { continue; }
            let vis = crate::guards::can_see_char(g, gd, oc);
            if vis.visible && crate::guards::sight_gain_on(g, gd, o, vis.dist) > 0.0 { seen += 1; }
        }
        if seen >= 2 { why = Some(LastWhy::Seen); }
    } else if g.tab.roomfight_room[r].entries.len() > 1 && df(g, d).noise_at.is_some() && g.time - df(g, d).noise_t < g.k.pressure.loud_secs && g.time - g.gd[gd].last_seen_t > 3.0
        && !(g.k.digin.on && crate::digin::dig_in_set(g, gd)) {   // (a man SET on his door is waiting for exactly those boots: he holds)
        if !state_of(g).ear.has(d) { state_of(g).ear.set(d, StateEarValue { t: -1.0, i: -1.0, at: -100.0 }); }
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
            why = Some(LastWhy::Door);
            shut = Some(e.i as usize);
            threat = g.tab.roomfight_room[r].entries[e.i as usize].at;
        }
    }
    let why = match why { Some(w) => w, None => { state_of(g).last.delete(d); return; } };
    let at = match state_of(g).last.get(d).copied() {
        Some(at) => at,
        None => {
            let range = if why == LastWhy::Seen { g.k.pressure.last_man_secs } else { g.k.pressure.out_secs };
            let at = g.time + pick(g, crate::dice::Who::Def(d), range);
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
    let chosen: Option<(V3, Option<DoorId>)> = match hooked(g, d, threat) {
        Some(h) => Some(h),
        None => best.map(|b| (b.0, Some(b.1))),
    };
    let now = g.time;
    state_of(g).tried.set(d, now);   // (whether or not there was a way out: he does not ask again for a while)
    let cause = if why == LastWhy::Seen { String::from("two of them in view") } else { format!("boots at {}", door_name(g, entries[shut.unwrap()].door)) };
    let (spot, door) = match chosen {
        Some(x) => x,
        None => {
            let what = format!("last man: {}, no way out of room {}", cause, g.tab.roomfight_room[r].id);
            note(g, gd, &what);
            return;
        }
    };
    crate::roomfight::run(g, d, spot, door, 0.3, crate::owner::by::OUT);
    state_of(g).outs.push(Out { door, def: d, moving: true, plant_t: -1.0, fought: false, step_t: -100.0 });
    if why == LastWhy::Seen {
        crate::game::game_say(g, gd, "too many — I'm out the back!", Some(true), None);
    } else {
        crate::game::game_say(g, gd, "they're at the door — I'm out the back!", Some(true), None);
    }
    let through = match door { Some(x) => format!(" through {}", door_name(g, x)), None => String::new() };
    let what = format!("last man: {}, gives room {} up{}", cause, g.tab.roomfight_room[r].id, through);
    note(g, gd, &what);
}

/// a last man out of his room, this frame. True = done with him. `o` is lent by pressTick, out of st.outs for the call.
fn tick_out(g: &mut Game, o: &mut Out) -> bool {
    let d = o.def;
    let gd = gd_of(g, d);
    if !alive(g, d) || !g.fight.defs.contains(&d) { return true; }
    if o.moving {
        if crate::roomfight::on_leg_of(g, d, crate::owner::by::OUT) { o.step_t = crate::roomfight::boots(g, gd, o.step_t); return false; }
        o.moving = false; o.plant_t = g.time;
        // whoever of his side holds the room he has come into hears it from him: the door he came by is the one to watch
        let here = pos_of(g, gd);
        let room = df(g, d).room;
        let mut told = 0;
        let defs: Vec<RoomfightDefId> = g.fight.defs.clone();
        for &x in &defs {
            if !(x != d && alive(g, x) && crate::roomfight::at_post(g, x) && df(g, x).room == room && flat(pos_of(g, gd_of(g, x)), here) < 12.0) { continue; }
            let i = match (df(g, x).room, o.door) {
                (Some(r), Some(door)) => g.tab.roomfight_room[r].entries.iter().position(|e| e.door == door),
                _ => None,
            };
            if let Some(i) = i { bump_ev(g, x, i, 1.2); told += 1; }
        }
        if told > 0 {
            crate::game::game_say(g, gd, "they're right behind me — watch that door!", Some(true), None);
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
        note(g, gd, "last man: fights again");
            return true;
    }
    if now - o.plant_t > 90.0 { return true; }
    false
}

/// once a frame from the room fight, after the defenders' and the teammates' own ticks: the kit, then the last men and those who have given their room up
pub(in crate::defenders) fn press_tick(g: &mut Game) {
    // the kit is its own lever: what a defender carries is handed out whatever the difficulty
    for d in g.fight.defs.clone() {
        let gd = g.tab.roomfight_def[d].gd;
        if g.k.pressure.stuns > 0.0 {
            if !g.m.pressure.kitted.has(gd) { g.m.pressure.kitted.add(gd); let n = g.k.pressure.stuns; crate::tactics::give_stuns(g, gd, n); }
        } else if g.m.pressure.kitted.delete(gd) {
            crate::tactics::give_stuns(g, gd, 0.0);   // (the row put back to 0 mid-fight: the charges go back with it)
        }
    }
    if !g.k.pressure.on { return; }
    for d in g.fight.defs.clone() { if alive(g, d) { last_man(g, d); } }
    let mut kept = Vec::new();
    for mut o in core::mem::take(&mut state_of(g).outs) { if !tick_out(g, &mut o) { kept.push(o); } }
    state_of(g).outs.extend(kept);
}

/// the fight is over
pub(in crate::defenders) fn press_stop(g: &mut Game) { g.m.pressure.states = None; }

/// the shipped values back; dropping the stored copy is the host's
pub fn reset_pressure(g: &mut Game) { g.k.pressure = pressure_defaults(); }
