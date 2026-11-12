//! THEY GO IN TOGETHER: men of a drilled squad sent to a room that is hit go to one of its doors by a way the guns they know of do not look down, wait short of it for each other, and go in
//! by the squad's own door tools, as a player's men do. Only a drilled squad does (`group::drilled`): green men by themselves go each on his own.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::roomfight::{def_of, DefMode};
use crate::v3::V3;
use alloc::{format, vec::Vec};

/// how long the first of them there waits for the others
const FORM_SECS: f64 = 6.0;
/// longer than this at the door, and each goes on by himself
const ENTRY_CAP: f64 = 20.0;

/// These men set out for one door of that room, each by a way of his own to it. False, and nobody is sent: no way there but past guns they know of.
pub(super) fn set_out(g: &mut Game, q: EnemysquadsSquadId, due: &[SquadDueItem], room: &crate::scene_scenarios::StructureRoom, at: V3) -> bool {
    let middle: V3 = [(room.bounds.x0 + room.bounds.x1) / 2.0, 0.0, (room.bounds.z0 + room.bounds.z1) / 2.0];
    let known = super::decide::team_known_how(g);
    let now = g.time;
    let theirs: Vec<DoorId> = g.m.enemysquads.states.as_ref().map_or(Vec::new(), |st| st.net.iter().filter(|c| c.squad == q && now - c.t < 30.0).filter_map(|c| c.door).collect());
    let mut ways: Vec<(RoomfightDefId, Way)> = Vec::new();
    for x in due {
        let Some(d) = def_of(g, x.gd) else { continue };
        let used: Vec<V3> = ways.iter().map(|w| w.1.spot).collect();
        let Some(w) = super::decide::way_round2(g, d, middle, &known, &theirs, &[], &used, None) else { continue };
        if w.door.is_none() || ways.first().is_some_and(|o| o.1.door != w.door) { continue; }   // (by the one door: a man with no way to it goes by himself)
        ways.push((d, w));
    }
    if ways.len() < 2 { return false; }
    for (d, w) in ways {
        let (gd, from) = (g.tab.roomfight_def[d].gd, crate::roomfight::station(g, d));
        let way = w.path.clone().unwrap_or_default();
        g.tab.enemysquads_squad[q].due.retain(|x| x.gd != gd);
        g.tab.enemysquads_squad[q].sent.push(Sent {
            gd, from, role: SentRole::Reinforce, phase: SentPhase::Run, spot: Some(w.spot), door: w.door, fix_at: at, t0: now, way: way.clone(), aim: w.aim, ..Sent::default()
        });
        super::decide::send_round(g, d, w.spot, w.door, way);
        event(g, q, SquadEventWhat::Reinforce, Some(gd));
    }
    true
}

/// A frame of the squad's men on their way in together: those who wait for each other short of the door, and those who go in.
pub(super) fn together_tick(g: &mut Game, q: EnemysquadsSquadId) {
    let mut doors: Vec<DoorId> = Vec::new();
    for s in &g.tab.enemysquads_squad[q].sent { if let (SentPhase::Form, Some(d)) = (s.phase, s.door) { if !doors.contains(&d) { doors.push(d); } } }
    for d in doors { formed(g, q, d); }
    entries_tick(g, q);
}

/// Those short of that door go in when nobody else is on his way to it, or the first has waited long enough: two or more by the squad's door tools, one by himself.
fn formed(g: &mut Game, q: EnemysquadsSquadId, door: DoorId) {
    let rows: Vec<Sent> = g.tab.enemysquads_squad[q].sent.iter().filter(|s| s.role == SentRole::Reinforce && s.door == Some(door)).cloned().collect();
    let here: Vec<&Sent> = rows.iter().filter(|s| s.phase == SentPhase::Form).collect();
    let first = here.iter().map(|s| s.there.unwrap_or(g.time)).fold(f64::INFINITY, m::min);
    let coming = rows.iter().any(|s| s.phase == SentPhase::Run && alive(g, s.gd));
    if coming && g.time - first < FORM_SECS { return; }
    let men: Vec<GuardId> = here.iter().map(|s| s.gd).filter(|&x| fit(g, x)).collect();
    let at = here[0].fix_at;
    let id = g.tab.enemysquads_squad[q].group;
    let busy = men.iter().any(|&x| g.gd[x].saw_foe_this_frame);
    if men.len() >= 2 && !busy && crate::group::drilled(g, id) && go_in(g, q, &men, door, at) { return; }
    if busy && g.time - first < FORM_SECS { return; }
    for s in here.iter().map(|s| s.gd).collect::<Vec<_>>() {
        for r in g.tab.enemysquads_squad[q].sent.iter_mut() { if r.gd == s { r.phase = SentPhase::Loose; } }
        if let Some(d) = def_of(g, s).filter(|_| alive(g, s)) { crate::roomfight::loose_at(g, d, at); }
    }
}

/// These men go in by that door. False: the tools refuse it.
fn go_in(g: &mut Game, q: EnemysquadsSquadId, men: &[GuardId], door: DoorId, at: V3) -> bool {
    let id = g.tab.enemysquads_squad[q].group;
    let side = crate::orders::side_of(g, door, g.pos_of(men[0]));
    let stuns: f64 = men.iter().map(|&x| crate::tactics::stuns_of(g, x)).sum();
    let kind = if stuns > 0.0 { crate::doortools::DoorToolKind::BangClear } else { crate::doortools::DoorToolKind::OpenClear };
    let got = crate::group::with(g, id, |g| {
        crate::doortools::door_tools_sync(g);
        let work = &mut g.groups[id].tools;
        (work.bangs, work.charges, work.frags) = (stuns, 0.0, 0.0);
        crate::doortools::door_tools_order(g, kind, men, door, side, None, None)
    });
    let crate::doortools::DoorToolsOrder::DoorOp(op) = got else { return false };
    for &x in men {
        if let Some(d) = def_of(g, x) { crate::roomfight::told(g, d); }
        for r in g.tab.enemysquads_squad[q].sent.iter_mut() { if r.gd == x { r.phase = SentPhase::Told; } }
    }
    let t0 = g.time;
    g.tab.enemysquads_squad[q].entries.push(Entry { op, men: men.to_vec(), at, t0, stuns });
    let name = g.tab.enemysquads_squad[q].name.clone();
    crate::game::game_say(g, men[0], &format!("{}: on the door, {} of us \u{2014} going in!", name, super::radio_check::count_word(men.len())), Some(true), None);
    true
}

/// A frame of the squad's entries: over when they have the room, or cannot get in, or there are not two of them left who are not through it.
fn entries_tick(g: &mut Game, q: EnemysquadsSquadId) {
    let mut i = g.tab.enemysquads_squad[q].entries.len();
    while i > 0 {
        i -= 1;
        let id = g.tab.enemysquads_squad[q].group;
        taken_off(g, q, i);
        let e = g.tab.enemysquads_squad[q].entries[i].clone();
        let running = g.groups[id].tools.ops.contains(&e.op);
        if running && g.time - e.t0 < ENTRY_CAP && (e.men.len() >= 2 || going_in(g, &e)) && !e.men.is_empty() { continue; }
        g.tab.enemysquads_squad[q].entries.remove(i);
        let thrown = e.stuns - g.groups[id].tools.bangs;
        if thrown > 0.0 { spend(g, &e.men, thrown); }
        let there = !running && g.tab.doortools_door_op[e.op].ended == Some(crate::doortools::Ended::In);
        crate::group::with(g, id, |g| crate::orders::squad_orders_clear(g, Some(&e.men)));
        for &x in &e.men {
            g.tab.enemysquads_squad[q].sent.retain(|s| s.gd != x || s.phase != SentPhase::Told);
            let Some(d) = def_of(g, x).filter(|_| alive(g, x)) else { continue };
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

/// A man another rule has taken, one in somebody's arm and one who is down are off it.
fn taken_off(g: &mut Game, q: EnemysquadsSquadId, i: usize) {
    let id = g.tab.enemysquads_squad[q].group;
    let told = |g: &Game, x: GuardId| def_of(g, x).is_some_and(|d| g.tab.roomfight_def[d].mode == DefMode::Told);
    let men = g.tab.enemysquads_squad[q].entries[i].men.clone();
    let off: Vec<GuardId> = men.into_iter().filter(|&x| !alive(g, x) || g.gd[x].held.is_some() || !told(g, x)).collect();
    if off.is_empty() { return; }
    crate::group::with(g, id, |g| for &x in &off { crate::orders::squad_orders_drop(g, x); });
    let s = &mut g.tab.enemysquads_squad[q];
    s.entries[i].men.retain(|x| !off.contains(x));
    s.sent.retain(|r| !off.contains(&r.gd) || r.phase != SentPhase::Told);
    for x in off { if g.gd[x].held.is_some() && told(g, x) { crate::roomfight::loose(g, x, "squads"); } }
}

/// the stuns they threw are off whoever carried one
fn spend(g: &mut Game, men: &[GuardId], mut n: f64) {
    for &x in men {
        let has = crate::tactics::stuns_of(g, x);
        let off = m::min(has, n);
        if off > 0.0 { crate::tactics::give_stuns(g, x, has - off); n -= off; }
    }
}
