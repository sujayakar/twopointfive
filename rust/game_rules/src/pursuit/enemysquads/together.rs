//! THEY GO IN TOGETHER: men of a squad sent to a room that is hit come to one of its doors, stack on it and go in by the squad's own door tools, as a player's men do.
//! Only a drilled squad does (`group::drilled`): green men by themselves go each on his own.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::roomfight::def_of;
use crate::v3::{dist_xz, V3};
use alloc::{format, string::String, vec::Vec};

/// longer than this on the way, and each goes on by himself
const ENTRY_CAP: f64 = 40.0;
/// a man nearer its door than this goes in whatever he meets; further off, a fight breaks them up
const AT_DOOR_M: f64 = 4.5;

/// These men are sent in by the door of that room nearest them on foot, but one the team is known to be at. False, and nobody is sent: no such door, or the tools refuse it.
pub(super) fn sent_in(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId], room: &crate::scene_scenarios::StructureRoom, at: V3) -> bool {
    let id = g.tab.enemysquads_squad[q].group;
    let Some((door, side)) = way_in(g, q, men[0], room) else { return false };
    let stuns: f64 = men.iter().map(|&x| crate::tactics::stuns_of(g, x)).sum();
    let kind = if stuns > 0.0 { crate::doortools::DoorToolKind::BangClear } else { crate::doortools::DoorToolKind::Threshold };
    let got = crate::group::with(g, id, |g| {
        crate::doortools::door_tools_sync(g);
        let work = &mut g.groups[id].tools;
        (work.bangs, work.charges, work.frags) = (stuns, 0.0, 0.0);
        crate::doortools::door_tools_order(g, kind, men, door, side, None, None)
    });
    let crate::doortools::DoorToolsOrder::DoorOp(op) = got else { return false };
    for &x in men {
        let Some(d) = def_of(g, x) else { continue };
        crate::roomfight::told(g, d);
        crate::mind::set_state(g, x, crate::game::GuardState::Alert, "squads");
        crate::mind::raise(g, x, 0.92, "squads");
    }
    let t0 = g.time;
    g.tab.enemysquads_squad[q].entries.push(Entry { op, men: men.to_vec(), at, t0, stuns, said: false });
    true
}

/// The door of that room with the shortest walk to it for him, and the face he comes to it on. Not steel, not locked against him, and not one the net has put the team at, where there is another.
fn way_in(g: &mut Game, q: EnemysquadsSquadId, man: GuardId, room: &crate::scene_scenarios::StructureRoom) -> Option<(DoorId, f64)> {
    let middle: V3 = [(room.bounds.x0 + room.bounds.x1) / 2.0, 0.0, (room.bounds.z0 + room.bounds.z1) / 2.0];
    let t = crate::roomfight::room_fight_room_at(g, middle)?;
    let here = g.pos_of(man);
    let theirs: Vec<DoorId> = g.m.enemysquads.states.as_ref().map_or(Vec::new(), |st| st.net.iter().filter(|c| c.squad == q && g.time - c.t < 30.0).filter_map(|c| c.door).collect());
    let mut best: Option<(DoorId, f64, f64)> = None;
    for e in g.tab.roomfight_room[t].entries.clone() {
        let out: V3 = [e.at[0] + e.out[0], 0.0, e.at[2] + e.out[1]];
        if g.dr[e.door].def.steel == Some(true) || crate::doors::doors_locked_out(g, e.door, out, None) { continue; }
        let Some(path) = crate::scene_collision::nav_grid_find_path(g, here, out, Some(6000.0), None).filter(|p| !p.is_empty()) else { continue };
        let last = path[path.len() - 1];
        if dist_xz(last, out) > 0.8 { continue; }
        // (a walk that goes through the room itself is no way to its door)
        let mut through = false;
        let l = super::decide::along(here, &path, &mut |p, _| if super::decide::in_room(g, t, p) { through = true; });
        if through { continue; }
        let l = l + if theirs.contains(&e.door) { 30.0 } else { 0.0 };
        if best.is_none_or(|b| l < b.2) { best = Some((e.door, crate::orders::side_of(g, e.door, out), l)); }
    }
    best.map(|(d, s, _)| (d, s))
}

/// A frame of the squad's entries: each said when they are on the door, broken up by a fight on the way, and over when they have the room or cannot get in.
pub(super) fn entries_tick(g: &mut Game, q: EnemysquadsSquadId) {
    let mut i = g.tab.enemysquads_squad[q].entries.len();
    while i > 0 {
        i -= 1;
        let id = g.tab.enemysquads_squad[q].group;
        those_who_met_them(g, q, i);
        taken_off(g, q, i);
        let e = g.tab.enemysquads_squad[q].entries[i].clone();
        let (running, going) = (g.groups[id].tools.ops.contains(&e.op), going_in(g, &e));
        if running && g.time - e.t0 < ENTRY_CAP && (going || e.men.len() >= 2) {
            if going && !e.said { say_going(g, q, i, &e); }
            continue;
        }
        g.tab.enemysquads_squad[q].entries.remove(i);
        let thrown = e.stuns - g.groups[id].tools.bangs;
        if thrown > 0.0 { spend(g, &e.men, thrown); }
        let there = !running && g.tab.doortools_door_op[e.op].ended == Some(crate::doortools::Ended::In);
        crate::group::with(g, id, |g| crate::orders::squad_orders_clear(g, Some(&e.men)));
        for &x in &e.men {
            g.tab.enemysquads_squad[q].sent.retain(|s| s.gd != x || s.phase != SentPhase::Told);
            let Some(d) = def_of(g, x).filter(|_| alive(g, x)) else { continue };
            if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Told { continue; }
            if there { crate::roomfight::stand_here(g, d); } else { crate::roomfight::loose_at(g, d, e.at); }
        }
        if there { if let Some(&x) = e.men.iter().find(|&&x| alive(g, x)) { event(g, q, SquadEventWhat::InPosition, Some(x)); } }
    }
}

/// They are through its door, or about to be.
fn going_in(g: &Game, e: &Entry) -> bool {
    use crate::doortools::Phase as P;
    let op = &g.tab.doortools_door_op[e.op];
    op.crossed.len() > 0 || matches!(op.phase, P::Enter | P::Toss | P::Fuse | P::Wait)
}

/// A man who sees one of them on the way, short of the door, fights him from where he is, as a man sent by himself does; the others go on.
fn those_who_met_them(g: &mut Game, q: EnemysquadsSquadId, i: usize) {
    let e = g.tab.enemysquads_squad[q].entries[i].clone();
    if going_in(g, &e) { return; }
    let door = g.dr[g.tab.doortools_door_op[e.op].door].frame_centre;
    for &x in &e.men {
        if !alive(g, x) || !g.gd[x].saw_foe_this_frame || dist_xz(g.pos_of(x), door) < AT_DOOR_M { continue; }
        if let (Some(d), Some(f)) = (def_of(g, x), g.gd[x].foe) { let at = g.ch[f].pos; crate::roomfight::loose_at(g, d, at); }
    }
}

/// A man another rule has taken (a covered lane on his way, a door to brace), one in somebody's arm and one who is down are off it.
fn taken_off(g: &mut Game, q: EnemysquadsSquadId, i: usize) {
    let id = g.tab.enemysquads_squad[q].group;
    let told = |g: &Game, x: GuardId| def_of(g, x).is_some_and(|d| g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Told);
    let men = g.tab.enemysquads_squad[q].entries[i].men.clone();
    let off: Vec<GuardId> = men.into_iter().filter(|&x| !alive(g, x) || g.gd[x].held.is_some() || !told(g, x)).collect();
    if off.is_empty() { return; }
    crate::group::with(g, id, |g| for &x in &off { crate::orders::squad_orders_drop(g, x); });
    let s = &mut g.tab.enemysquads_squad[q];
    s.entries[i].men.retain(|x| !off.contains(x));
    s.sent.retain(|r| !off.contains(&r.gd) || r.phase != SentPhase::Told);
    for x in off { if g.gd[x].held.is_some() && told(g, x) { crate::roomfight::loose(g, x, "squads"); } }
}

/// Said once, by the first of them who can, as the door goes.
fn say_going(g: &mut Game, q: EnemysquadsSquadId, i: usize, e: &Entry) {
    g.tab.enemysquads_squad[q].entries[i].said = true;
    let name = g.tab.enemysquads_squad[q].name.clone();
    if let Some(&x) = e.men.iter().find(|&&x| fit(g, x)) { crate::game::game_say(g, x, &format!("{}: on the door \u{2014} going in!", name), Some(true), None); }
}

/// the stuns they threw are off whoever carried one
fn spend(g: &mut Game, men: &[GuardId], mut n: f64) {
    for &x in men {
        let has = crate::tactics::stuns_of(g, x);
        let off = m::min(has, n);
        if off > 0.0 { crate::tactics::give_stuns(g, x, has - off); n -= off; }
    }
}
