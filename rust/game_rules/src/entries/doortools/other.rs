//! The other doors of the room an entry goes into: watched while its men come in and while they hold it, and held by its spare men once a threshold's are in.
use super::threats::{door_tools_room_in, frame};
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap};
use crate::m;
use crate::roomfight::Entry;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec::Vec};

macro_rules! dop { ($g:ident, $op:expr) => { $g.tab.doortools_door_op[$op] }; }
macro_rules! asg { ($g:ident, $a:expr) => { $g.tab.orders_assignment[$a] }; }

/// When the other doors are asked of: as they come in; at its end, when the doors watched are kept and no more dealt; as a threshold's men end it, when
/// a man a leaf would hide his door from has none; and while they hold the room.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Watched { Entry, Kept, Seen, Held }

/// The room's other doors, watched. False: none are, or nobody is in to watch them.
pub(super) fn door_tools_other_doors(g: &mut Game, op: DoorOpId, when: Watched) -> bool {
    dop!(g, op).other.get_or_insert_with(JsMap::new);
    let inside = men_in(g, op, when);
    if !(g.k.doortools.other_door_m > 0.0) { all_back(g, op, &inside); return false; }
    for (o, r) in rows(g, op) { if !inside.contains(&r.gd) { dop!(g, op).other.as_mut().unwrap().delete(o); } }
    if inside.is_empty() { return false; }
    if let Some(room) = door_tools_room_in(g, op) {
        if !open_floor(g, room) { watch_dealt(g, op, room, inside, when); }
        else if g.tab.roomfight_room[room].cells.len() as f64 <= g.k.doortools.search_cells { all_back(g, op, &inside); }
    }
    for (o, r) in rows(g, op) {
        let a = *dop!(g, op).mine.get(r.gd).unwrap();
        let yaw = v3::yaw_to(g.pos_of(r.gd), r.w);
        let words = crate::orders::door_words(g, o);
        asg!(g, a).watch = Some(r.w);
        asg!(g, a).face = Some(yaw);
        asg!(g, a).there = format!("holding · {}", words).into();
        let Some(h) = g.sets[g.seat].door_tools.held.get_mut(r.gd) else { continue };
        (h.face, h.d, h.out, h.door) = (yaw, 0.0, Some(true), Some(o));
    }
    true
}

/// Is he on one of this entry's other doors.
pub(super) fn door_tools_on_other(g: &Game, op: DoorOpId, gd: GuardId) -> bool {
    g.k.doortools.other_door_m > 0.0 && dop!(g, op).other.as_ref().is_some_and(|x| x.values().any(|r| r.gd == gd))
}

/// The doors watched, and by whom.
fn rows(g: &Game, op: DoorOpId) -> Vec<(DoorId, DoorToolsDoneItemOtherValue)> { dop!(g, op).other.as_ref().unwrap().iter().map(|(o, r)| (o, *r)).collect() }
/// The entry's men who are up, in, and on no other order.
fn men_in(g: &Game, op: DoorOpId, when: Watched) -> Vec<GuardId> {
    dop!(g, op).men.clone().into_iter().filter(|&x| {
        let mine = dop!(g, op).mine.get(x).copied();
        !crate::condition::down(g, g.gd[x].ch) && dop!(g, op).pod.has(x) && on_this(g, op, x) && (when == Watched::Entry || g.sets[g.seat].door_tools.held.get(x).map(|h| h.a) == mine)
    }).collect()
}
fn on_this(g: &Game, op: DoorOpId, x: GuardId) -> bool {
    let of = crate::orders::squad_orders_of(&crate::seats::here(g), x);
    of.is_some() && of == dop!(g, op).mine.get(x).copied()
}
/// A door given back: the man who had it is on the sector his point gave him again.
fn back(g: &mut Game, op: DoorOpId, inside: &[GuardId], o: DoorId, gd: GuardId) {
    dop!(g, op).other.as_mut().unwrap().delete(o);
    let (Some(h), Some(a), Some(pod)) = (g.sets[g.seat].door_tools.held.get(gd).copied(), dop!(g, op).mine.get(gd).copied(), dop!(g, op).pod.get(gd).copied()) else { return };
    if h.a != a || h.door != Some(o) || !inside.contains(&gd) { return; }
    let face = door_tools_on_sector(g, gd, a, pod);
    let r = g.sets[g.seat].door_tools.held.get_mut(gd).unwrap();
    (r.face, r.d, r.out, r.door) = (face, 0.0, Some(false), None);
}
fn all_back(g: &mut Game, op: DoorOpId, inside: &[GuardId]) { for (o, r) in rows(g, op) { back(g, op, inside, o, r.gd); } }

/// What the dealing goes by: the men in, the rest of the squad, and when it is asked.
struct Deal { op: DoorOpId, inside: Vec<GuardId>, squad: Vec<GuardId>, when: Watched }

/// Who watches which: a door somebody else has, or whose far side is known, is given up; then those left are dealt to the men with nothing to watch.
fn watch_dealt(g: &mut Game, op: DoorOpId, room: RoomfightRoomId, inside: Vec<GuardId>, when: Watched) {
    let squad: Vec<GuardId> = g.guards.iter().copied().filter(|&x| g.team(x).ours() && !crate::condition::down(g, g.gd[x].ch) && !inside.contains(&x)).collect();
    let x = Deal { op, inside, squad, when };
    let entries = g.tab.roomfight_room[room].entries.clone();
    for (o, r) in rows(g, op) {
        if still_watched(g, &x, entries.iter().find(|e| e.door == o), o, r.gd) { continue; }
        back(g, op, &x.inside, o, r.gd);
        let line = format!("other door · {} off {}", g.gd[r.gd].callsign, g.dr[o].def.name);
        door_tools_beat(g, op, &line);
    }
    if when != Watched::Kept { dealt(g, &x, entries); }
}
fn still_watched(g: &mut Game, x: &Deal, e: Option<&Entry>, o: DoorId, by: GuardId) -> bool {
    let (Some(far), Some(e)) = (opening_of(g, o), e) else { return false };
    let at = inner_of(g, e);
    if crate::entrypoints::entry_points_far_known(g, far, at) || theirs(g, x, o, Some(by)) { return false; }
    x.when != Watched::Seen || { let p = g.pos_of(by); !(leaf_across(g, o, p, at) > 0.0) }
}

/// The nearest doors first and those to the outside last: as they come in, no more than half of them on doors, and none of two.
fn dealt(g: &mut Game, x: &Deal, entries: Vec<Entry>) {
    let op = x.op;
    let mut free: Vec<GuardId> = x.inside.iter().copied().filter(|&m_| is_free(g, x, m_)).collect();
    let still = dop!(g, op).men.iter().filter(|&&m_| !crate::condition::down(g, g.gd[m_].ch) && on_this(g, op, m_)).count();
    let spare = if still < 3 { 0.0 } else { m::floor(x.inside.len() as f64 / 2.0) };
    let mut ranked: Vec<(f64, Entry)> = entries.into_iter().map(|e| {
        let fc = frame(g, e.door);
        let dists: Vec<f64> = x.inside.iter().map(|m_| v3::dist_xz(g.pos_of(*m_), fc)).collect();
        ((if g.dr[e.door].def.exterior.unwrap_or(false) { 100.0 } else { 0.0 }) + m::min_n(&dists), e)
    }).collect();
    js::sort_by(&mut ranked, |a, b| a.0 - b.0);
    for (_, e) in ranked {
        let o = e.door;
        if free.is_empty() { break; }
        if o == dop!(g, op).door || dop!(g, op).other.as_ref().unwrap().has(o) { continue; }
        if x.when == Watched::Entry && dop!(g, op).other.as_ref().unwrap().len() as f64 >= spare { break; }
        if !worth_watching(g, x, &e) { continue; }
        let Some(gd) = watcher_for(g, x, &e, &free) else { continue };
        let w = watch_at(g, &e, gd);
        dop!(g, op).other.as_mut().unwrap().set(o, DoorToolsDoneItemOtherValue { gd, w });
        free.retain(|m_| *m_ != gd);
        let line = format!("other door · {} on {}", g.gd[gd].callsign, g.dr[o].def.name);
        door_tools_beat(g, op, &line);
    }
}
/// On his point, with no door and nothing else of the room's to keep, and not blind.
fn is_free(g: &Game, x: &Deal, gd: GuardId) -> bool {
    let arrived = dop!(g, x.op).mine.get(gd).is_some_and(|&a| asg!(g, a).arrived);
    let keeps = g.sets[g.seat].door_tools.held.get(gd).is_some_and(|h| h.out.unwrap_or(false) || h.door.is_some());
    (dop!(g, x.op).on_pod.has(gd) || (x.when != Watched::Entry && arrived)) && !door_tools_on_other(g, x.op, gd) && !keeps && !crate::control::man::blind(g, gd)
}
/// A door onto floor nobody knows, that nobody else has or stands by, with room to come through it.
fn worth_watching(g: &mut Game, x: &Deal, e: &Entry) -> bool {
    let Some(far) = opening_of(g, e.door) else { return false };
    let (fc, at) = (frame(g, e.door), inner_of(g, e));
    if crate::entrypoints::entry_points_far_known(g, far, at) || theirs(g, x, e.door, None) || beside(g, &x.squad, e.door) { return false; }
    !(crate::scene_collision::nav_grid_is_blocked(g, fc[0] + e.out[0] * 0.9, fc[2] + e.out[1] * 0.9) && crate::scene_collision::nav_grid_is_blocked(g, fc[0] + e.out[0] * 1.5, fc[2] + e.out[1] * 1.5))
}
/// Of those with a line to it, whom its leaf would hide least of it from, then the nearest.
fn watcher_for(g: &Game, x: &Deal, e: &Entry, free: &[GuardId]) -> Option<GuardId> {
    let (fc, at) = (frame(g, e.door), inner_of(g, e));
    let mut can: Vec<(GuardId, f64, f64)> = Vec::new();
    for &gd in free {
        let p = g.pos_of(gd);
        if v3::dist_xz(p, fc) > g.k.doortools.other_door_m || crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], at, None, None, None) { continue; }
        let leaf = leaf_across(g, e.door, p, at);
        if x.when == Watched::Seen && leaf > 0.0 { continue; }
        can.push((gd, leaf, v3::dist_xz(p, fc)));
    }
    js::sort_by(&mut can, |a, b| js::or(a.1 - b.1, a.2 - b.2));
    can.first().map(|c| c.0)
}
/// His muzzle: on the floor beyond it, or on the frame where he has no line to that.
fn watch_at(g: &Game, e: &Entry, gd: GuardId) -> V3 {
    let (p, fc) = (g.pos_of(gd), frame(g, e.door));
    let beyond: V3 = [fc[0] + e.out[0], 1.2, fc[2] + e.out[1]];
    if crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], beyond, None, None, None) { [fc[0], 1.2, fc[2]] } else { beyond }
}

fn opening_of(g: &mut Game, o: DoorId) -> Option<CoverOpeningId> {
    let ready = crate::cover::cover_ready(g);
    ready.into_iter().find(|x| g.tab.cover_opening[*x].door == Some(o))
}
fn inner_of(g: &Game, e: &Entry) -> V3 {
    let fc = frame(g, e.door);
    [fc[0] - e.out[0] * 0.4, 1.2, fc[2] - e.out[1] * 0.4]
}
/// Another order's door, one somebody else holds, or one a teammate was told to watch.
fn theirs(g: &mut Game, x: &Deal, o: DoorId, by: Option<GuardId>) -> bool {
    let tools = &g.sets[g.seat].door_tools;
    if tools.ops.iter().any(|y| *y != x.op && dop!(g, *y).door == o) || tools.held.iter().any(|(y, r)| r.door == Some(o) && Some(y) != by) { return true; }
    let fc = frame(g, o);
    x.squad.iter().any(|&y| crate::seats::order_of(g, y).map(|a| &asg!(g, a)).is_some_and(|a| a.door == Some(o) || a.watch.is_some_and(|w| v3::dist_xz(w, fc) < 1.5)))
}
/// A teammate stands by it: on his post, or with no order.
fn beside(g: &mut Game, squad: &[GuardId], o: DoorId) -> bool {
    let fc = frame(g, o);
    squad.iter().any(|&x| v3::dist_xz(g.pos_of(x), fc) < 3.0 && crate::seats::order_of(g, x).map_or(true, |a| asg!(g, a).post.is_some()))
}

/// How much of a door its own leaf, stood open, would hide from a man at `p`.
fn leaf_across(g: &Game, o: DoorId, p: V3, at: V3) -> f64 {
    if g.dr[o].leaf.off { return 0.0; }
    let stop = open_stop(g, o);
    let (sg, top) = (m::sign(stop), m::abs(stop));
    let def = &g.dr[o].def;
    let (w, hx, hz, closed) = (def.width + LEAF_EDGE, def.hinge[0], def.hinge[1], def.closed_dir);
    let (rx, rz) = (at[0] - p[0], at[2] - p[2]);
    let mut n = 1.0;
    let mut hid = leaf_cut(closed, stop, w, hx, hz, p, rx, rz);
    let mut a = m::min(LEAF_FROM, top);
    while a < top {
        n += 1.0;
        hid += leaf_cut(closed, sg * a, w, hx, hz, p, rx, rz);
        a += LEAF_STEP;
    }
    hid / n
}
fn open_stop(g: &Game, o: DoorId) -> f64 { if crate::doors::door_swing(g, o) > 0.0 { crate::doors::door_max_a(g, o) } else { crate::doors::door_min_a(g, o) } }
/// Does the leaf at angle `a` lie across the line from `p` to the point held.
fn leaf_cut(closed_dir: f64, a: f64, w: f64, hx: f64, hz: f64, p: V3, rx: f64, rz: f64) -> f64 {
    let th = closed_dir + a;
    let (lx, lz) = (m::cos(th) * w, m::sin(th) * w);
    let den = rx * lz - rz * lx;
    if m::abs(den) < 1e-9 { return 0.0; }
    let s = ((hx - p[0]) * lz - (hz - p[2]) * lx) / den;
    let u = ((hx - p[0]) * rz - (hz - p[2]) * rx) / den;
    if s > 0.0 && s < 1.0 && u > 0.0 && u < 1.0 { 1.0 } else { 0.0 }
}

/// Who may be put on a far door and where everybody stands: the men with no door yet, ours, and Sam.
struct Holders { room: RoomfightRoomId, men: Vec<GuardId>, stood: JsMap<GuardId, V3>, sam: V3 }

/// The room's other doors, held: a threshold's last act before it is in. The farthest first, by a man who has it from where he stands or a short walk off.
pub(super) fn door_tools_hold_doors(g: &mut Game, op: DoorOpId) {
    let Some(room) = door_tools_room_in(g, op) else { return };
    if open_floor(g, room) { return; }
    let men: Vec<GuardId> = held_by(g, op, &|h| !h.out.unwrap_or(false));
    if men.is_empty() { return; }
    let mut stood: JsMap<GuardId, V3> = JsMap::new();
    for x in ours_up(g) { stood.set(x, g.pos_of(x)); }
    let mut x = Holders { room, men, stood, sam: crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g)) };
    let doors = far_doors(g, op, room);
    let stay = if held_by(g, op, &|h| h.out.unwrap_or(false) && h.door.is_none()).is_empty() { g.k.doortools.thr_doors_stay } else { 0.0 };
    let mut said: Vec<String> = Vec::new();
    for e in doors {
        if x.men.len() as f64 <= stay { break; }
        let Some((gd, post)) = holder_for(g, &x, &e) else { continue };
        x.men.retain(|m_| *m_ != gd);
        if let Some(to) = post { x.stood.set(gd, to); }
        on_door(g, op, &e, gd, post);
        said.push(format!("{} {} {}", g.gd[gd].callsign, if post.is_some() { "up to" } else { "on" }, g.dr[e.door].def.name));
    }
    if !said.is_empty() { door_tools_beat(g, op, &format!("doors · {}", said.join(" · "))); }
}
fn ours_up(g: &Game) -> Vec<GuardId> { g.guards.iter().copied().filter(|&x| g.team(x).ours() && !crate::condition::down(g, g.gd[x].ch)).collect() }
/// The order's men who are up and hold the room so.
fn held_by(g: &Game, op: DoorOpId, so: &dyn Fn(&DoorToolsHeldValue) -> bool) -> Vec<GuardId> {
    dop!(g, op).men.iter().copied().filter(|&x| !crate::condition::down(g, g.gd[x].ch) && g.sets[g.seat].door_tools.held.get(x).is_some_and(so)).collect()
}
/// The room's doors but the one they came by, the farthest from it first: not another order's, nor one already held, nor one somebody of ours is on.
fn far_doors(g: &Game, op: DoorOpId, room: RoomfightRoomId) -> Vec<Entry> {
    let (d, tools) = (dop!(g, op).door, &g.sets[g.seat].door_tools);
    let rest: Vec<GuardId> = ours_up(g).into_iter().filter(|x| !dop!(g, op).men.contains(x)).collect();
    let taken = |o: DoorId| tools.ops.iter().any(|x| *x != op && dop!(g, *x).door == o) || tools.held.values().any(|r| r.door == Some(o)) || rest.iter().any(|&x| v3::dist_xz(g.pos_of(x), frame(g, o)) < 3.0);
    let mut doors: Vec<Entry> = g.tab.roomfight_room[room].entries.iter().filter(|e| e.door != d && !taken(e.door)).cloned().collect();
    let fc = frame(g, d);
    js::sort_by(&mut doors, |a, b| v3::dist_xz(b.at, fc) - v3::dist_xz(a.at, fc));
    doors
}
/// The nearest with a line to its frame from where he stands; else the one with the least way to a place he would have one from, and that place.
fn holder_for(g: &Game, x: &Holders, e: &Entry) -> Option<(GuardId, Option<V3>)> {
    let (o, fc, at) = (e.door, frame(g, e.door), inner_of(g, e));
    let mut near: Vec<(GuardId, f64)> = Vec::new();
    for &gd in &x.men {
        let p = g.pos_of(gd);
        if v3::dist_xz(p, fc) > g.k.doortools.thr_doors_m || crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], at, None, None, None) || js::truthy(leaf_across(g, o, p, at)) { continue; }
        near.push((gd, v3::dist_xz(p, fc)));
    }
    js::sort_by(&mut near, |a, b| a.1 - b.1);
    if let Some(n) = near.first() { return Some((n.0, None)); }
    let (gd, to) = place_by(g, x, e)?;
    let from = g.pos_of(gd);
    let path = crate::scene_collision::nav_grid_find_path(g, from, to, None, None)?;
    if crate::places::walk_len(from, &path) > g.k.doortools.thr_doors_walk { return None; }
    Some((gd, Some(to)))
}
/// The places a door is held from, beside its frame and back from it, the likeliest first.
const BY_FRAME: [[f64; 2]; 6] = [[1.5, 2.6], [2.0, 2.2], [1.1, 3.2], [2.4, 3.0], [1.8, 3.8], [0.9, 2.2]];
/// And where its leaf opens onto them, clear of the leaf.
const BY_LEAF: [[f64; 2]; 2] = [[0.2, 2.4], [0.5, 2.4]];
fn place_by(g: &Game, x: &Holders, e: &Entry) -> Option<(GuardId, V3)> {
    let (o, fc, at, w) = (e.door, frame(g, e.door), inner_of(g, e), g.dr[e.door].def.width);
    let side = crate::orders::side_of(g, o, [fc[0] - e.out[0], 0.0, fc[2] - e.out[1]]);
    let in_swing = !g.dr[o].leaf.off && m::sign(open_stop(g, o)) == side;
    let mut best: Option<(f64, GuardId, V3)> = None;
    let places = BY_FRAME.iter().chain(if in_swing { &BY_LEAF[..] } else { &[] });
    for (k, [along, off]) in places.enumerate() {
        for sgn in [1.0, -1.0] {
            let p = door_pt(g, o, w / 2.0 + sgn * (w / 2.0 + along), *off, side, None);
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { continue; }
            let cell = g.col.nav.cell;
            let key = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
            if !g.tab.roomfight_room[x.room].cells.has(crate::Num(key)) || crate::scene_collision::segment_blocked(g, [p[0], 1.5, p[2]], at, None, None, None) { continue; }
            let hid = leaf_across(g, o, p, at);
            let behind = if js::truthy(hid) { LEAF_COST * (1.0 + hid) } else { 0.0 };
            for &gd in &x.men {
                let c = v3::dist_xz(g.pos_of(gd), p) + 0.4 * (k + 1) as f64 + behind;
                let clear = v3::dist_xz(x.sam, p) >= 1.0 && !x.stood.iter().any(|(y, q)| y != gd && v3::dist_xz(*q, p) < 1.0);
                if clear && best.map_or(true, |b| c < b.0) { best = Some((c, gd, p)); }
            }
        }
    }
    best.map(|b| (b.1, b.2))
}
/// He has the door: sent to his place where he needs one, and held from there.
fn on_door(g: &mut Game, op: DoorOpId, e: &Entry, gd: GuardId, post: Option<V3>) {
    let (o, fc) = (e.door, frame(g, e.door));
    let w: V3 = [fc[0] + e.out[0], 1.2, fc[2] + e.out[1]];
    let yaw = v3::yaw_to(post.unwrap_or(g.pos_of(gd)), w);
    let name = crate::orders::door_words(g, o);
    if let Some(to) = post {
        let r = &mut dop!(g, op);
        r.pod.set(gd, crate::entrypoints::Pod { pos: to, face: yaw, watch: None });
        r.on_pod.delete(gd);
        let d = r.door;
        let (pace, light) = (g.sets[g.seat].orders.knobs.entry, g.sets[g.seat].door_tools.lights.unwrap_or(true));
        door_tools_assign(g, op, gd, &DoorToolsAssignA {
            post: Some(to), face: Some(yaw), watch: Some(w), shy: Some(None), thru: Some(d), pace: Some(pace), stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
            light: Some(light), search: Some(true), going: format!("to the far door · {}", name).into(), there: format!("holding · {}", name).into(), ..Default::default()
        });
    }
    let a = *dop!(g, op).mine.get(gd).unwrap();
    asg!(g, a).watch = Some(w);
    asg!(g, a).face = Some(yaw);
    asg!(g, a).there = format!("holding · {}", name).into();
    g.sets[g.seat].door_tools.held.set(gd, DoorToolsHeldValue { a, face: yaw, d: 0.0, hit_t: m::NAN, arr_t: m::NAN, out: Some(true), door: Some(o) });
}
