pub use ::game_types::rearwatch::types;
pub use types::*;
use crate::v3::dist_xz;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// A row of the books, by its key, for a write: the book stays in the table because `rear_ways` reads it through `g`.
macro_rules! book {
    ($g:expr, $k:expr) => {
        $g.m.rearwatch.books.get_mut($k).unwrap()
    };
}

// ---- small reads

fn of(g: &Game, m: GuardId) -> Option<TaskId> {
    crate::orders::squad_orders_of(&crate::seats::here(g), m)
}
fn post_of(g: &Game, m: GuardId) -> Option<V3> {
    let a = of(g, m)?;
    g.tab.task[a].post
}
fn spare_of(g: &Game, a: TaskId) -> bool {
    g.tab.task[a].spare.unwrap_or(false)
}
fn pin_row(g: &Game, key: PlayerId, m: GuardId) -> Option<RearPin> {
    g.m.rearwatch.books.get(key).unwrap().pins.get(m).copied()
}
fn is_going(p: crate::doortools::Phase) -> bool {
    use crate::doortools::Phase;
    matches!(p, Phase::Kick | Phase::Toss | Phase::Fuse | Phase::Wait | Phase::Enter)
}
fn is_slow(p: crate::doortools::Phase) -> bool {
    use crate::doortools::Phase;
    matches!(p, Phase::Toss | Phase::Fuse | Phase::Wait)
}
fn is_entry(k: crate::doortools::DoorToolKind) -> bool {
    use crate::doortools::DoorToolKind;
    matches!(k, DoorToolKind::OpenClear | DoorToolKind::BangClear | DoorToolKind::Charge)
}
/// three or more living men on an entry that is not a threshold's hold
fn carries(g: &Game, op: DoorOpId) -> bool {
    let r = &g.tab.doortools_door_op[op];
    if r.thr.is_some() || !is_entry(r.kind) {
        return false;
    }
    r.men.iter().filter(|&&m| !crate::condition::down(g, g.gd[m].ch)).count() >= 3
}
/// does an order of the door tools work this door
fn door_worked(g: &Game, seat: &crate::seats::SeatRef, door: DoorId) -> bool {
    seat.door_tools.ops.iter().any(|&x| g.tab.doortools_door_op[x].door == door)
}
/// the room as fight sees it, or none
fn room_of(g: &mut Game, p: V3) -> Option<RoomfightRoomId> {
    crate::roomfight::room_fight_room_at(g, p)
}

/// This view's book, made if it is not yet.
fn book_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::rearwatch::Book {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    g.m.rearwatch.books.get_or_insert_with(key, Book::default)
}

/// A door off its hinges, or unlatched and far enough open.
fn is_open_way(g: &crate::game::Game, d: DoorId) -> bool {
    if g.dr[d].leaf.off {
        return true;
    }
    if g.dr[d].leaf.latched {
        return false;
    }
    m::abs(g.dr[d].leaf.angle) >= g.k.rearwatch.ajar
}

/// The ways behind a stack on face `side` of `door`, best first.
pub fn rear_ways(g: &mut crate::game::Game, door: DoorId, side: f64, marks: &[V3]) -> Vec<crate::rearwatch::RearWaysItem> {
    if marks.is_empty() {
        return Vec::new();
    }
    let reach = g.k.rearwatch.reach;
    let f = crate::stackroom::stack_room_probe(g, door, side);
    let mut c: V3 = [0.0, 0.0, 0.0];
    let n = marks.len() as f64;
    for q in marks {
        c[0] += q[0] / n;
        c[2] += q[2] / n;
    }
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    let mut out: Vec<RearWaysItem> = Vec::new();
    for o in crate::cover::cover_ready(g) {
        let (od, at, on) = {
            let r = &g.tab.cover_opening[o];
            (r.door, r.at, r.n)
        };
        let od = match od {
            Some(d) if d != door => d,
            _ => continue,
        };
        // (… or a taken room's other door a man was keeping when this stack was ordered)
        let kept_shut = g.m.rearwatch.books.get(key).map_or(false, |b| b.shut.has(od));
        if !(is_open_way(g, od) || kept_shut) {
            continue;
        }
        let dists: Vec<f64> = marks.iter().map(|q| dist_xz(at, *q)).collect();
        let d = m::min_n(&dists);
        if d > reach {
            continue;
        }
        let a: V3 = [at[0] + on[0] * 0.7, 0.0, at[2] + on[1] * 0.7];
        let b: V3 = [at[0] - on[0] * 0.7, 0.0, at[2] - on[1] * 0.7];
        // (F.has of the probe, which is only asked with F.floor true: then it is stackRoomHas, the switch on)
        let (has_a, has_b) = if f.floor {
            (crate::stackroom::stack_room_has(g, door, side, a), crate::stackroom::stack_room_has(g, door, side, b))
        } else {
            (false, false)
        };
        if f.floor && !has_a && !has_b {
            continue;
        }
        let far_is_a = if f.floor && has_a != has_b { !has_a } else { dist_xz(a, c) >= dist_xz(b, c) };
        let far = if far_is_a { a } else { b };
        let near = if far_is_a { b } else { a };
        let far_side = crate::orders::side_of(g, od, far);
        let r = crate::cleared::cleared_rooms_room_by(g, od, far_side);
        let cls = if js::truthy(r) && crate::cleared::cleared_rooms_is_known(g, r) { 1.0 } else { 0.0 };
        out.push(RearWaysItem { o, front: [near[0], CHEST, near[2]], cls, d });
    }
    js::sort_by(&mut out, |x, y| {
        let c = x.cls - y.cls;
        if js::truthy(c) {
            return c;
        }
        let c = x.d - y.d;
        if js::truthy(c) {
            return c;
        }
        if g.tab.cover_opening[x.o].id < g.tab.cover_opening[y.o].id {
            -1.0
        } else {
            1.0
        }
    });
    out
}

/// SHORT OF HIS MARK: between a stride and a step from it for too long, off the watch until he stands on it.
fn short_of(g: &mut Game, key: PlayerId, m: GuardId) -> bool {
    let a = match of(g, m) {
        Some(a) => a,
        None => return false,
    };
    let q = match g.tab.task[a].post {
        Some(q) => q,
        None => return false,
    };
    let d = dist_xz(g.pos_of(m), q);
    if d < 0.35 || d >= 1.0 {
        book!(g, key).short.delete(a);
        return false;
    }
    let time = g.time;
    let t = g.m.rearwatch.books.get(key).unwrap().short.get(a).copied();
    let t = match t {
        Some(t) if time >= t => t,
        _ => {
            book!(g, key).short.set(a, time);
            return false;
        }
    };
    if time - t <= g.k.rearwatch.short {
        return false;
    }
    book!(g, key).let_off.add(a);
    true
}

/// The man of this order already pinned behind this stack, who is not its lead nor the lane's man.
fn had_of(g: &Game, key: PlayerId, op: DoorOpId, men: &[GuardId], lane: Option<GuardId>) -> Option<GuardId> {
    let (door, side, lead) = {
        let r = &g.tab.doortools_door_op[op];
        (r.door, r.side, r.lead)
    };
    for &m in men {
        if let Some(p) = pin_row(g, key, m) {
            if p.door == door && p.side == side && m != lead && Some(m) != lane {
                return Some(m);
            }
        }
    }
    None
}

/// `A.from ??= op.from.get(m) ?? where he stands`, and his eyes spare.
fn keep_eyes_spare(g: &mut Game, a: TaskId, op: DoorOpId, m: GuardId) {
    g.tab.task[a].spare = Some(true);
    if g.tab.task[a].from.is_none() {
        let p = g.pos_of(m);
        let from = g.tab.doortools_door_op[op].from.get(m).copied().unwrap_or([p[0], 0.0, p[2]]);
        g.tab.task[a].from = Some(from);
    }
}

/// Who of a stack should have this way: a line from where he is, then into the doorway, then the man already on it.
fn rank(g: &Game, key: PlayerId, op: Option<DoorOpId>, m: GuardId, o: CoverOpeningId, front: V3) -> f64 {
    let mut r = 0.0;
    let p = g.pos_of(m);
    if rear_sees(g, p, o, front).is_some() {
        r += 16.0;
    }
    let post = post_of(g, m).unwrap();
    let at = g.tab.cover_opening[o].at;
    if crate::cover::cover_sight_from(g, post, o) == Some(at) {
        r += 8.0;
    }
    if pin_row(g, key, m).map(|x| x.o) == Some(o) {
        r += 4.0;
    }
    if crate::cover::cover_of(g, m).map(|j| j.o) == Some(o) {
        r += 2.0;
    }
    if let Some(op) = op {
        if g.m.rearwatch.books.get(key).unwrap().by_op.get(op).copied() == Some(m) {
            r += 1.0;
        }
    }
    r
}

/// The stacks of this tick, and who is on them.
struct Stacks {
    /// the squad on its feet
    squad: Vec<GuardId>,
    groups: Vec<Group>,
    /// every man of an order of the door tools
    taken: JsSet<GuardId>,
}
/// What has been dealt this tick: the men pinned, and the ways no second man may be pinned to.
struct Dealt {
    next: JsMap<GuardId, RearPin>,
    held_out: JsSet<CoverOpeningId>,
}
/// The man a stack gives a way to; `blind`: his line to it is cut for a moment.
#[derive(Clone, Copy)]
struct Pick {
    man: GuardId,
    way: RearWaysItem,
    blind: bool,
}

/// Game.update, after the door tools' tick and before the cover's: who has the way behind each stack.
pub fn rear_watch_tick(g: &mut crate::game::Game) {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    book_of(g);
    room_keep_tick(g, key);
    if !crate::orders::squad_orders_active(g, &crate::seats::here(g)) { return drop_the_pins(g, key); }
    let squad = squad_up(g);
    let (mut groups, taken) = door_tool_stacks(g, key);
    groups.extend(plain_stacks(g, key, &squad, &taken));
    let st = Stacks { squad, groups, taken };
    let mut dealt = Dealt { next: JsMap::new(), held_out: JsSet::new() };
    left_outside(g, key, &st, &mut dealt);
    for gr in &st.groups { pin_the_rear_man(g, key, gr, &st.squad, &mut dealt); }
    log_the_men_off(g, key, &st, &dealt.next);
    book!(g, key).pins = dealt.next;
}

/// Orders are over: nobody is pinned, and a man whose eyes were made spare from outside has them back.
fn drop_the_pins(g: &mut Game, key: PlayerId) {
    let pins: Vec<RearPin> = g.m.rearwatch.books.get(key).unwrap().pins.values().copied().collect();
    for p in pins {
        if p.raised == Some(true) && p.out == Some(true) {
            g.tab.task[p.a].spare = Some(false);
        }
    }
    book!(g, key).pins.clear();
}

/// The men of the squad on their feet.
fn squad_up(g: &Game) -> Vec<GuardId> {
    crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().filter(|&m| !crate::condition::down(g, g.gd[m].ch)).collect()
}

/// The stack of each order of the door tools that has one now, and every man those orders take.
fn door_tool_stacks(g: &mut Game, key: PlayerId) -> (Vec<Group>, JsSet<GuardId>) {
    let mut groups: Vec<Group> = Vec::new();
    let mut taken: JsSet<GuardId> = JsSet::new();
    for op in g.sets[g.seat].door_tools.ops.clone() {
        let op_men = g.tab.doortools_door_op[op].men.clone();
        for &m in &op_men {
            taken.add(m);
        }
        let men = men_of_the_stack(g, op, &op_men);
        // (the whole file's marks, the men already sent in included: the stack is where it stood)
        let marks: Vec<V3> = op_men.iter().filter_map(|&m| if crate::condition::down(g, g.gd[m].ch) { None } else { mark_of(g, op, m) }).collect();
        let (door, side, phase) = {
            let r = &g.tab.doortools_door_op[op];
            (r.door, r.side, r.phase)
        };
        let (cands, going) = if crate::doortools::is_pre(phase) {
            (before_the_door(g, key, op, &men), false)
        } else if carries(g, op) && is_going(phase) {
            (as_the_door_goes(g, key, op, &men), true)
        } else {
            continue;
        };
        groups.push(Group { op: Some(op), door, side, men, marks, cands, going });
    }
    (groups, taken)
}

/// The men of an order still on their feet, on its assignment, with a post, and not of its pod.
fn men_of_the_stack(g: &Game, op: DoorOpId, op_men: &[GuardId]) -> Vec<GuardId> {
    op_men.iter().copied().filter(|&m| {
        if crate::condition::down(g, g.gd[m].ch) { return false; }
        let a = of(g, m);
        a == g.tab.doortools_door_op[op].mine.get(m).copied() && post_of(g, m).is_some() && !g.tab.doortools_door_op[op].pod.has(m)
    }).collect()
}

/// A man's mark in the file: the order's own post for him, or his post.
fn mark_of(g: &Game, op: DoorOpId, m: GuardId) -> Option<V3> {
    match g.tab.doortools_door_op[op].post.get(m) {
        Some(s) => Some(s.pos),
        None => post_of(g, m),
    }
}

/// Every man of the order on his feet stands on his mark.
fn stack_formed(g: &Game, op: DoorOpId) -> bool {
    let op_men = g.tab.doortools_door_op[op].men.clone();
    op_men.iter().all(|&m| {
        if crate::condition::down(g, g.gd[m].ch) { return true; }
        mark_of(g, op, m).map_or(true, |q| dist_xz(g.pos_of(m), q) < FORMED_M)
    })
}

/// Before the door goes: the men with spare eyes who are not the lane's, and the man who has the watch.
fn before_the_door(g: &mut Game, key: PlayerId, op: DoorOpId, men: &[GuardId]) -> Vec<GuardId> {
    let phase = g.tab.doortools_door_op[op].phase;
    let lane = g.tab.doortools_door_op[op].lane.as_ref().map(|l| l.gd);
    let mut cands: Vec<GuardId> = men.iter().copied().filter(|&m| {
        let a = of(g, m).unwrap();
        spare_of(g, a) && Some(m) != lane
    }).collect();
    // (the man the doorway's manners have on the open door HAD the watch if he was pinned when the order came)
    if let Some(l) = lane {
        if pin_row(g, key, l).is_some() && men.len() >= 3 {
            book!(g, key).by_op.set(op, l);
        }
    }
    // (NO NEW WATCH FOR A MOMENT: an entry given to a stack already on its marks files the men again and goes at once)
    if phase == crate::doortools::Phase::Stack && stack_formed(g, op) {
        let bk = g.m.rearwatch.books.get(key).unwrap();
        let by = bk.by_op.get(op).copied();
        cands.retain(|&m| bk.pins.has(m) || by == Some(m));
    }
    // (… and THE MAN WHO HAS IT KEEPS IT over an entry order)
    if let Some(had) = had_of(g, key, op, men, lane) {
        if !cands.contains(&had) && carries(g, op) {
            let a = of(g, had).unwrap();
            keep_eyes_spare(g, a, op, had);
            cands.push(had);
        }
    }
    // (a man let off to step onto his mark has the watch again once he is on it, unless the door then goes at once with no carrying it through)
    let brief = phase == crate::doortools::Phase::Stack && !carries(g, op);
    cands.into_iter().filter(|&m| {
        if short_of(g, key, m) { return false; }
        if !brief { return true; }
        let a = of(g, m).unwrap();
        !g.m.rearwatch.books.get(key).unwrap().let_off.has(a)
    }).collect()
}

/// The door is going: the one man who had the watch keeps it, or a way come open since the last man of the file; in the entry only the man at the END of the turn.
fn as_the_door_goes(g: &Game, key: PlayerId, op: DoorOpId, men: &[GuardId]) -> Vec<GuardId> {
    let r = &g.tab.doortools_door_op[op];
    let (phase, door) = (r.phase, r.door);
    let last = r.queue.last().map(|q| q.gd);
    let mut out: Vec<GuardId> = men.iter().copied().filter(|&m| {
        !r.crossed.has(m) && m != r.lead && Some(m) != r.first && (phase != crate::doortools::Phase::Enter || last == Some(m))
    }).collect();
    let kept = g.m.rearwatch.books.get(key).unwrap().by_op.get(op).copied().or_else(|| had_of(g, key, op, men, None));
    let centre = g.dr[door].frame_centre;
    // (a NEW man takes it only behind a can: behind a shove or a boot the first man is through before a gun turned round has settled)
    match kept {
        Some(k) if out.contains(&k) => vec![k],
        _ if is_slow(phase) => {
            js::sort_by(&mut out, |a, b| {
                let db = dist_xz(post_of(g, *b).unwrap(), centre);
                let da = dist_xz(post_of(g, *a).unwrap(), centre);
                db - da
            });
            out.iter().copied().take(1).collect()
        }
        _ => Vec::new(),
    }
}

/// A plain 'stack' on a door, no tool on it yet: its men by door.
fn plain_stacks(g: &mut Game, key: PlayerId, squad: &[GuardId], taken: &JsSet<GuardId>) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for (door, side, men) in plain_stackers(g, squad, taken) {
        let marks: Vec<V3> = men.iter().map(|&m| post_of(g, m).unwrap()).collect();
        let cands: Vec<GuardId> = men.iter().copied().filter(|&m| {
            let a = of(g, m).unwrap();
            spare_of(g, a) && !short_of(g, key, m)
        }).collect();
        groups.push(Group { op: None, door, side, men, marks, cands, going: false });
    }
    groups
}

/// The men on a plain stack order, by door and face.
fn plain_stackers(g: &Game, squad: &[GuardId], taken: &JsSet<GuardId>) -> Vec<(DoorId, f64, Vec<GuardId>)> {
    let mut plain: Vec<(DoorId, f64, Vec<GuardId>)> = Vec::new();
    for &m in squad {
        if taken.has(m) {
            continue;
        }
        let Some(a) = of(g, m) else { continue };
        let r = &g.tab.task[a];
        let (true, Some(door), Some(post)) = (crate::orders::rows::id_is(&r.order, crate::orders::Is::Stacks), r.door, r.post) else { continue };
        let side = crate::orders::side_of(g, door, post);
        match plain.iter_mut().find(|x| x.0 == door && x.1 == side) {
            Some(l) => l.2.push(m),
            None => plain.push((door, side, vec![m])),
        }
    }
    plain
}

/// LEFT OUTSIDE: the entry had no point inside for him, or the order ended with him still on his mark. Dealt BEFORE the stacks'.
fn left_outside(g: &mut Game, key: PlayerId, st: &Stacks, dealt: &mut Dealt) {
    for m in g.m.rearwatch.books.get(key).unwrap().pins.keys() {
        if dealt.next.has(m) || st.groups.iter().any(|gr| gr.men.contains(&m)) || st.taken.has(m) || crate::condition::down(g, g.gd[m].ch) {
            continue;
        }
        let Some(row) = keeps_it_from_outside(g, key, m, &st.squad) else { continue };
        dealt.next.set(m, row);
        dealt.held_out.add(row.o);
    }
}

/// His pin, while he is on the same order with a post, his way is open or kept, his eyes are spare, he still has a line to it and nobody else holds it.
fn keeps_it_from_outside(g: &mut Game, key: PlayerId, m: GuardId, squad: &[GuardId]) -> Option<RearPin> {
    let p = pin_row(g, key, m).unwrap();
    let a = of(g, m).filter(|&a| a == p.a)?;
    let post = g.tab.task[a].post?;
    let od = g.tab.cover_opening[p.o].door?;
    if !(is_open_way(g, od) || g.m.rearwatch.books.get(key).unwrap().shut.has(od)) {
        return None;
    }
    // (he had it through a door already going: the door tools turned every gun in, so his eyes are made spare again if he keeps it)
    let turned_in = p.going == Some(true) && !spare_of(g, a);
    if !turned_in && !spare_of(g, a) {
        return None;
    }
    if rear_sees(g, post, p.o, p.front).is_some() {
        let t = g.time;
        book!(g, key).pins.get_mut(m).unwrap().seen_t = t;
    } else if g.time - p.seen_t >= g.k.rearwatch.grace {
        return None;
    }
    let lone = Group { op: p.op, door: p.door, side: p.side, men: vec![m], marks: vec![post], cands: vec![m], going: false };
    if held_by_other(g, &lone, p.o, squad) {
        return None;
    }
    if turned_in {
        g.tab.task[a].spare = Some(true);
        book!(g, key).pins.get_mut(m).unwrap().raised = Some(true);
    }
    if p.out != Some(true) {
        book!(g, key).pins.get_mut(m).unwrap().out = Some(true);
        let s = format!("{} {} keeps {} from outside", js::to_fixed(g.time, 1), g.gd[m].callsign, g.tab.cover_opening[p.o].id);
        note(book!(g, key), &s);
    }
    pin_row(g, key, m)
}

/// A stack of two or more, or a door already going, gets its man on the best way behind it that nobody holds.
fn pin_the_rear_man(g: &mut Game, key: PlayerId, gr: &Group, squad: &[GuardId], dealt: &mut Dealt) {
    if gr.cands.is_empty() || (gr.men.len() < 2 && !gr.going) {
        return;
    }
    let ways: Vec<RearWaysItem> = rear_ways(g, gr.door, gr.side, &gr.marks).into_iter().filter(|w| !dealt.held_out.has(w.o) && !held_by_other(g, gr, w.o, squad)).collect();
    let Some(pick) = who_watches(g, key, gr, &ways, &dealt.next) else { return };
    pin_him(g, key, gr, pick, dealt);
}

/// The first way that a man of the stack can watch, and the best of those who can.
fn who_watches(g: &Game, key: PlayerId, gr: &Group, ways: &[RearWaysItem], next: &JsMap<GuardId, RearPin>) -> Option<Pick> {
    for w in ways {
        let mut see: Vec<GuardId> = gr.cands.iter().copied().filter(|&m| {
            if next.has(m) { return false; }
            let q = post_of(g, m).unwrap();
            rear_sees(g, q, w.o, w.front).is_some()
        }).collect();
        let blind = see.is_empty();
        if blind {
            // (his line cut for a moment, the worked door's leaf swinging across it: still his)
            let time = g.time;
            let grace = g.k.rearwatch.grace;
            let Some(k) = gr.cands.iter().copied().find(|&m| !next.has(m) && pin_row(g, key, m).map_or(false, |p| p.o == w.o && time - p.seen_t < grace)) else { continue };
            see = vec![k];
        }
        let (op, centre, o, front) = (gr.op, g.dr[gr.door].frame_centre, w.o, w.front);
        js::sort_by(&mut see, |a, b| {
            let c = rank(g, key, op, *b, o, front) - rank(g, key, op, *a, o, front);
            if js::truthy(c) {
                return c;
            }
            dist_xz(post_of(g, *b).unwrap(), centre) - dist_xz(post_of(g, *a).unwrap(), centre)
        });
        return Some(Pick { man: see[0], way: *w, blind });
    }
    None
}

/// He has that way: the pin written, the way taken for this tick, the log told of a new one.
fn pin_him(g: &mut Game, key: PlayerId, gr: &Group, pick: Pick, dealt: &mut Dealt) {
    let Pick { man: pm, way, blind } = pick;
    let po = way.o;
    let was = pin_row(g, key, pm);
    let a = of(g, pm).unwrap();
    let time = g.time;
    let same = was.map_or(false, |w| w.o == po && w.op == gr.op);
    let mut p = match was {
        Some(w) if same => w,
        _ => RearPin {
            o: po,
            front: way.front,
            door: gr.door,
            side: gr.side,
            op: gr.op,
            seen_t: time,
            a,
            going: None,
            raised: None,
            out: None,
            to: None,
        },
    };
    p.front = way.front;
    p.a = a;
    p.out = Some(false);
    p.going = Some(gr.going);
    if !blind {
        p.seen_t = time;
    }
    // (the old row sees the writes too: a later stack reads it this tick)
    if same {
        *book!(g, key).pins.get_mut(pm).unwrap() = p;
    }
    dealt.held_out.add(po); // (no second stack pins a man to the same way this tick)
    dealt.next.set(pm, p);
    if let Some(op) = gr.op {
        book!(g, key).by_op.set(op, pm);
    }
    if !same {
        let s = format!(
            "{} {} on {} ({} beyond) behind the stack on {}{}",
            js::to_fixed(time, 1),
            g.gd[pm].callsign,
            g.tab.cover_opening[po].id,
            if js::truthy(way.cls) { "known" } else { "uncleared" },
            g.dr[gr.door].def.name,
            if gr.going { " · through the entry" } else { "" }
        );
        note(book!(g, key), &s);
    }
    // (the door is going: the door tools have just turned every gun in; his eyes stay spare, as on the wait)
    if gr.going {
        keep_eyes_spare(g, a, gr.op.unwrap(), pm);
    }
}

/// Every man who had a pin and has none now goes in the log, with why: read by the checks and whoever asks why a back door went unwatched.
fn log_the_men_off(g: &mut Game, key: PlayerId, st: &Stacks, next: &JsMap<GuardId, RearPin>) {
    for m in g.m.rearwatch.books.get(key).unwrap().pins.keys() {
        if next.has(m) {
            continue;
        }
        let p = pin_row(g, key, m).unwrap();
        let gr = st.groups.iter().find(|x| x.men.contains(&m));
        let w = gr.and_then(|x| rear_ways(g, x.door, x.side, &x.marks).into_iter().find(|y| y.o == p.o));
        let why = if crate::condition::down(g, g.gd[m].ch) {
            "down"
        } else if let Some(x) = gr {
            if x.cands.contains(&m) {
                why_not_that_way(g, x, m, w, &st.squad)
            } else if !next.is_empty() {
                "another man has the rear"
            } else {
                "not the rear man now"
            }
        } else if p.op.map_or(false, |op| g.tab.doortools_door_op[op].crossed.has(m)) {
            "through the door"
        } else {
            "off the stack"
        };
        let s = format!("{} {} off {} · {}", js::to_fixed(g.time, 1), g.gd[m].callsign, g.tab.cover_opening[p.o].id, why);
        note(book!(g, key), &s);
    }
}

/// Why a man the stack could still have used lost his way `w` (none: it is no longer among the stack's ways).
fn why_not_that_way(g: &Game, gr: &Group, m: GuardId, w: Option<RearWaysItem>, squad: &[GuardId]) -> &'static str {
    let Some(w) = w else { return "shut, or out of reach" };
    if held_by_other(g, gr, w.o, squad) {
        return "another holds it";
    }
    let q = post_of(g, m).unwrap();
    if rear_sees(g, q, w.o, w.front).is_none() { "no line from his mark" } else { "a way before it" }
}

/// One row of the log, which keeps the newest 200.
fn note(b: &mut crate::rearwatch::Book, s: &str) {
    b.log.push(String::from(s));
    b.rows += 1.0;
    if b.log.len() > 200 {
        b.log.remove(0);
    }
}

/// The point just inside the door on `p`'s side, `k` from its frame.
fn inner(g: &Game, o: CoverOpeningId, p: V3, k: f64) -> V3 {
    let (at, n) = (g.tab.cover_opening[o].at, g.tab.cover_opening[o].n);
    let s = if (p[0] - at[0]) * n[0] + (p[2] - at[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 };
    [at[0] + n[0] * s * k, CHEST, at[2] + n[1] * s * k]
}
/// The squad but `m` that stands in `room`.
fn others(g: &mut Game, squad: &[GuardId], m: GuardId, room: RoomfightRoomId) -> Vec<GuardId> {
    let mut out = Vec::new();
    for &x in squad {
        if x != m && room_of(g, g.pos_of(x)) == Some(room) {
            out.push(x);
        }
    }
    out
}
/// A keeper let go: he follows, last.
fn let_go(g: &mut Game, pl: PlayerId, m: GuardId, why: &str) {
    let r = match book!(g, pl).keeps.delete(m) {
        Some(r) => r,
        None => return,
    };
    if let Some(a) = of(g, m) {
        if a == r.pin.a && g.tab.task[a].post.is_none() {
            g.tab.task[a].going = String::from(FALLING).into();
        }
    }
    let s = format!("{} {} off {} · {}", js::to_fixed(g.time, 1), g.gd[m].callsign, g.tab.cover_opening[r.pin.o].id, why);
    note(book!(g, pl), &s);
}
/// The walk to Sam, once a judging for each man.
fn walk(g: &Game, far: &mut JsMap<GuardId, f64>, x: GuardId, sam_at: V3) -> f64 {
    if let Some(l) = far.get(x) {
        return *l;
    }
    let l = walk_to(g, g.pos_of(x), sam_at);
    far.set(x, l);
    l
}

/// What a kept door is judged by: the squad on its feet, and Sam.
struct Leaving {
    sam_up: bool,
    sam_at: V3,
    squad: Vec<GuardId>,
}

/// A TAKEN ROOM'S OTHER DOOR, KEPT WHILE THE SQUAD LEAVES.
fn room_keep_tick(g: &mut crate::game::Game, pl: PlayerId) {
    if !crate::orders::squad_orders_active(g, &crate::seats::here(g)) { return nobody_keeps(g, pl); }
    let now = crate::doortools::door_tools_door_keepers(&crate::seats::here(g));
    let sam_ch = g.pl[g.player].ch;
    let l = Leaving { sam_up: !crate::condition::down(g, sam_ch), sam_at: g.ch[sam_ch].pos, squad: squad_up(g) };
    hand_overs(g, pl, &now, &l);
    book!(g, pl).was = now;
    forget_shut_doors(g, pl, &l);
    for m in book!(g, pl).keeps.keys() {
        keeps_his_door(g, pl, m, &l);
    }
}

/// Orders are over: every keeper is let go, and nothing is remembered.
fn nobody_keeps(g: &mut Game, pl: PlayerId) {
    for m in book!(g, pl).keeps.keys() {
        let_go(g, pl, m, "switched off");
    }
    let b = book!(g, pl);
    b.was = Vec::new();
    b.shut.clear();
}

/// The hand-over: he had a door last frame, the entry has let go of him since, and what he has now is 'fall in'.
fn hand_overs(g: &mut Game, pl: PlayerId, now: &[(GuardId, DoorId, TaskId)], l: &Leaving) {
    for (m, door, a0) in book!(g, pl).was.clone() {
        if book!(g, pl).keeps.has(m) || crate::condition::down(g, g.gd[m].ch) || now.iter().any(|x| x.0 == m) {
            continue;
        }
        let Some(keep) = stays_on_the_door(g, m, door, a0, l) else { continue };
        let s = format!("{} {} keeps {} while the squad leaves the room", js::to_fixed(g.time, 1), g.gd[m].callsign, g.tab.cover_opening[keep.pin.o].id);
        book!(g, pl).keeps.set(m, keep);
        note(book!(g, pl), &s);
    }
}

/// His keep of that door: on a new order with no post, the way not yet known beyond and not worked, somebody still in the room, Sam near enough to stay behind for.
fn stays_on_the_door(g: &mut Game, m: GuardId, door: DoorId, a0: TaskId, l: &Leaving) -> Option<RoomKeep> {
    let a = of(g, m)?;
    if a == a0 || g.tab.task[a].post.is_some() {
        return None;
    }
    let o = crate::cover::cover_ready(g).into_iter().find(|&x| g.tab.cover_opening[x].door == Some(door))?;
    let p = g.pos_of(m);
    let front = inner(g, o, p, 0.7);
    let room = room_of(g, p).or_else(|| room_of(g, front))?;
    if crate::entrypoints::entry_points_far_known(g, o, inner(g, o, p, 0.4)) || door_worked(g, &crate::seats::here(g), door) {
        return None;
    }
    // (nobody left in it to keep it for)
    if others(g, &l.squad, m, room).is_empty() && !sam_in_room(g, l, room) {
        return None;
    }
    // (… and Sam is already too far off to stay behind for)
    if g.k.rearwatch.room_m > 0.0 && dist_xz(p, l.sam_at) > g.k.rearwatch.room_m {
        return None;
    }
    let to: Option<V3> = g.tab.task[a0].watch.map(|w| [w[0], w[1], w[2]]);
    let pin = RearPin { o, front, door, side: 1.0, op: None, seen_t: g.time, a, going: None, raised: None, out: None, to };
    Some(RoomKeep { pin, room, out_t: f64::NAN, behind: None, at: 0.0 })
}

/// Sam on his feet in that room.
fn sam_in_room(g: &mut Game, l: &Leaving, room: RoomfightRoomId) -> bool {
    l.sam_up && room_of(g, l.sam_at) == Some(room)
}

/// A kept door that became a stack's way behind: until the squad has been beyond it, an order works it, or the squad is out of the room.
fn forget_shut_doors(g: &mut Game, pl: PlayerId, l: &Leaving) {
    for door in book!(g, pl).shut.keys() {
        let w = *book!(g, pl).shut.get(door).unwrap();
        let gone = crate::entrypoints::entry_points_far_known(g, w.o, w.at)
            || door_worked(g, &crate::seats::here(g), door)
            || !(l.squad.iter().any(|&x| room_of(g, g.pos_of(x)) == Some(w.room)) || sam_in_room(g, l, w.room));
        if gone {
            book!(g, pl).shut.delete(door);
        }
    }
}

/// Each kept door, each tick: his still, on 'fall in'? Then he holds it, last in the file, or he is let go.
fn keeps_his_door(g: &mut Game, pl: PlayerId, m: GuardId, l: &Leaving) {
    let r = book!(g, pl).keeps.get(m).unwrap().clone();
    if crate::condition::down(g, g.gd[m].ch) {
        book!(g, pl).keeps.delete(m);
        return;
    }
    let a = of(g, m);
    let Some(a) = a.filter(|&a| g.tab.task[a].post.is_none()) else { return another_order(g, pl, m, a, &r) };
    // ('fall in' given again: the same keep, on the new order)
    if a != r.pin.a {
        book!(g, pl).keeps.get_mut(m).unwrap().pin.a = a;
    }
    if let Some(why) = why_let_go(g, pl, m, &r, l) {
        return let_go(g, pl, m, why);
    }
    // (the last place in the file; the door's one name, as the squad's rows say it)
    let o = r.pin.o;
    let name = match g.tab.cover_opening[o].door {
        Some(d) => crate::orders::door_words(g, d),
        None => g.tab.cover_opening[o].id.clone(),
    };
    let row = &mut g.tab.task[a];
    row.follow = 1e3;
    row.going = format!("holding · {}", name).into();
}

/// Off the keep with an order that is not 'fall in'. Sent to a mark, the stack on the room's next door: shut, the door he was keeping is still a way behind it.
fn another_order(g: &mut Game, pl: PlayerId, m: GuardId, a: Option<TaskId>, r: &RoomKeep) {
    let (o, door) = (r.pin.o, r.pin.door);
    book!(g, pl).keeps.delete(m);
    let at_him = inner(g, o, g.pos_of(m), 0.4);
    let stays = a.is_some()
        && !is_open_way(g, door)
        && !crate::entrypoints::entry_points_far_known(g, o, at_him)
        && !door_worked(g, &crate::seats::here(g), door)
        && g.tab.task[a.unwrap()].door != Some(door);
    if stays {
        book!(g, pl).shut.set(door, BookShutValue { o, room: r.room, at: at_him });
    }
    let s = format!(
        "{} {} off {} · another order{}",
        js::to_fixed(g.time, 1),
        g.gd[m].callsign,
        g.tab.cover_opening[o].id,
        if stays { " (still a way behind the stack)" } else { "" }
    );
    note(book!(g, pl), &s);
}

/// Why he should leave the door now, if he should.
fn why_let_go(g: &mut Game, pl: PlayerId, m: GuardId, r: &RoomKeep, l: &Leaving) -> Option<&'static str> {
    let (o, door) = (r.pin.o, r.pin.door);
    if !l.sam_up {
        return Some("the player is down");
    }
    let at_him = inner(g, o, g.pos_of(m), 0.4);
    if crate::entrypoints::entry_points_far_known(g, o, at_him) {
        return Some("the squad has been beyond it");
    }
    if g.k.rearwatch.room_m > 0.0 && dist_xz(g.pos_of(m), l.sam_at) > g.k.rearwatch.room_m {
        return Some("the player has gone on");
    }
    if another_order_has_it(g, m, o, door, l) {
        return Some("another order has it");
    }
    // (several men keeping doors of one room do not wait on each other: DEEPER and PASSED are by the walk to Sam. Asked four times a second)
    if room_of(g, l.sam_at) == Some(r.room) {
        let row = book!(g, pl).keeps.get_mut(m).unwrap();
        row.out_t = f64::NAN;
        row.behind = None;
        return None;
    }
    if g.time >= r.at || r.at - g.time > 1.0 { why_done_with_the_room(g, pl, m, r.room, l) } else { None }
}

/// The door is worked, or another man on a mark has it or watches it.
fn another_order_has_it(g: &Game, m: GuardId, o: CoverOpeningId, door: DoorId, l: &Leaving) -> bool {
    if door_worked(g, &crate::seats::here(g), door) {
        return true;
    }
    let o_at = g.tab.cover_opening[o].at;
    l.squad.iter().any(|&x| {
        if x == m { return false; }
        let Some(xa) = of(g, x) else { return false };
        let row = &g.tab.task[xa];
        row.post.is_some() && (row.door == Some(door) || row.watch.map_or(false, |w| dist_xz(w, o_at) < 1.5))
    })
}

/// Sam has left the room: nobody left in it who is deeper than he is, the last man has passed him, or the clock has run out.
fn why_done_with_the_room(g: &mut Game, pl: PlayerId, m: GuardId, room: RoomfightRoomId, l: &Leaving) -> Option<&'static str> {
    let time = g.time;
    let row = book!(g, pl).keeps.get_mut(m).unwrap();
    row.at = time + 0.25;
    if row.out_t.is_nan() || time < row.out_t {
        row.out_t = time;
    }
    let out_t = row.out_t;
    let sam_at = l.sam_at;
    let mut far: JsMap<GuardId, f64> = JsMap::new();
    let dm = walk(g, &mut far, m, sam_at);
    let rest: Vec<GuardId> = others(g, &l.squad, m, room).into_iter().filter(|&x| !book!(g, pl).keeps.has(x) || walk(g, &mut far, x, sam_at) > dm).collect();
    if rest.is_empty() {
        return Some("the squad is out of the room");
    }
    // (THE LAST MAN PASSES HIM: the men who had further to walk to Sam than he did when Sam left have all come by)
    if book!(g, pl).keeps.get(m).unwrap().behind.is_none() {
        let behind: Vec<GuardId> = rest.iter().copied().filter(|&x| walk(g, &mut far, x, sam_at) > dm + 0.3).collect();
        book!(g, pl).keeps.get_mut(m).unwrap().behind = Some(behind);
    }
    let behind = book!(g, pl).keeps.get(m).unwrap().behind.clone().unwrap();
    // (… and he does not crowd them at the way out: a mate still in the room whose place is beyond it goes out first)
    let queue = rest.iter().any(|&x| bound_beyond(g, x, room));
    if !behind.is_empty() && !queue {
        let passed = behind.iter().all(|&x| crate::condition::down(g, g.gd[x].ch) || !rest.contains(&x) || walk(g, &mut far, x, sam_at) < dm - 0.3);
        if passed {
            return Some("the last man has passed him");
        }
    }
    if g.time - out_t >= g.k.rearwatch.room_secs { Some("on the clock") } else { None }
}

/// A mate following, whose place is not in that room.
fn bound_beyond(g: &mut Game, x: GuardId, room: RoomfightRoomId) -> bool {
    let Some(xa) = of(g, x) else { return false };
    let (xpost, xgoal) = (g.tab.task[xa].post, g.tab.task[xa].goal);
    xpost.is_none() && room_of(g, xgoal) != Some(room)
}

/// How far from `a` to `b` on foot: the straight line when nothing stands in it, else the nav grid's path.
fn walk_to(g: &crate::game::Game, a: V3, b: V3) -> f64 {
    let d = dist_xz(a, b);
    if !crate::scene_collision::low_blocked(g, a, b) {
        return d;
    }
    let path = match crate::scene_collision::nav_grid_find_path(g, a, b, Some(3000.0), None) {
        Some(p) => p,
        None => return d + 20.0,
    };
    crate::places::walk_len(a, &path) + dist_xz(path.last().copied().unwrap_or(a), b)
}

/// the fall-in leg: this man's feet stay where they are, in his seat's book.
pub fn room_kept(g: &crate::game::Game, seat_ref: &crate::seats::SeatRef, gd: GuardId) -> bool {
    // (view_of: his seat's book; a cast is one book, its own)
    let key = if g.in_cast {
        crate::seats::view_key(g, seat_ref)
    } else {
        let seat = crate::seats::view_of(g, gd);
        crate::cmd::seat_man(g, seat)
    };
    g.m.rearwatch.books.get(key).map_or(false, |b| b.keeps.has(gd))
}

/// Somebody not of this stack already has that way.
fn held_by_other(g: &crate::game::Game, g_2: &crate::rearwatch::Group, o: CoverOpeningId, squad: &[GuardId]) -> bool {
    let (od, at) = (g.tab.cover_opening[o].door, g.tab.cover_opening[o].at);
    if let Some(op) = g_2.op {
        if let Some(l) = g.tab.doortools_door_op[op].lane.as_ref() {
            if od.is_some() && Some(l.door) == od && !crate::condition::down(g, g.gd[l.gd].ch) {
                return true;
            }
        }
    }
    for &m in squad {
        // (… nor a man of this same entry already sent in: his sweep passing over the door from inside is not a watch on it)
        if g_2.men.contains(&m) || g_2.op.map_or(false, |op| g.tab.doortools_door_op[op].men.contains(&m)) {
            continue;
        }
        if let Some(a) = of(g, m) {
            let row = &g.tab.task[a];
            if let Some(post) = row.post {
                if dist_xz(g.pos_of(m), post) <= 1.5 && ((od.is_some() && row.door == od) || row.watch.map_or(false, |w| dist_xz(w, at) < 1.5)) {
                    return true;
                }
            }
        }
        if let Some(j) = crate::cover::cover_of(g, m) {
            if !j.off && j.o == o {
                return true;
            }
        }
    }
    false
}

/// The point of a way behind a man stood at `p` can watch: the doorway, its leaf's edge, or the floor just before it.
pub fn rear_sees(g: &crate::game::Game, p: V3, o: CoverOpeningId, front: V3) -> Option<V3> {
    let s = crate::cover::cover_sight_from(g, p, o);
    if s.is_some() {
        return s;
    }
    let d = dist_xz(p, front);
    if d <= g.k.rearwatch.reach && d > 0.9 && !crate::scene_collision::segment_blocked(g, [p[0], EYE, p[2]], front, None, None, None) {
        Some(front)
    } else {
        None
    }
}

/// the way this man is pinned to.
pub(super) fn rear_pin_of(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> Option<crate::rearwatch::RearPin> {
    let key = crate::seats::view_key(g, seat);
    let b = g.m.rearwatch.books.get(key)?;
    match b.pins.get(gd) {
        Some(p) => Some(*p),
        None => b.keeps.get(gd).map(|r| r.pin),
    }
}

/// the turn the men go in by, with the rear man put last.
pub fn rear_last(g: &crate::game::Game, op: DoorOpId, turn: &[GuardId], first: GuardId) -> Vec<GuardId> {
    if turn.len() < 3 {
        return turn.to_vec();
    }
    let w = match rear_watcher(g, &crate::seats::here(g), op) {
        Some(w) => w,
        None => return turn.to_vec(),
    };
    if w == first || !turn.contains(&w) || turn[turn.len() - 1] == w {
        return turn.to_vec();
    }
    let mut out: Vec<GuardId> = turn.iter().copied().filter(|&m| m != w).collect();
    out.push(w);
    out
}

/// the man of this order pinned on a way behind and held on it through the entry.
pub fn rear_watcher(g: &crate::game::Game, seat: &crate::seats::SeatRef, op: DoorOpId) -> Option<GuardId> {
    if !carries(g, op) {
        return None;
    }
    let key = crate::seats::view_key(g, seat);
    let b = g.m.rearwatch.books.get(key)?;
    let w = *b.by_op.get(op)?;
    if b.pins.get(w)?.op == Some(op) {
        Some(w)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_log_line_time_prints_as_to_fixed_does_under_bun() {
        // (the case that differs from Rust's own formatter is the exact tie at .25)
        for (x, s) in [(0.25, "0.3"), (1.25, "1.3"), (0.75, "0.8"), (1.75, "1.8"), (0.35, "0.3"), (0.05, "0.1"), (2.5, "2.5"), (0.0, "0.0"), (12.34, "12.3")] {
            assert_eq!(js::to_fixed(x, 1), s);
        }
    }

    #[test]
    fn the_log_keeps_the_newest_200_and_counts_them_all() {
        let mut b = Book::default();
        for i in 0..205 {
            note(&mut b, &format!("{}", i));
        }
        assert_eq!(b.rows, 205.0);
        assert_eq!(b.log.len(), 200);
        assert_eq!(b.log[0], "5");
        assert_eq!(b.log[199], "204");
    }
}
