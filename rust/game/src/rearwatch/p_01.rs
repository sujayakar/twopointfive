//! src/game/rearwatch.ts lines 1 to 368
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A row of the books, by its key, for a write: the book stays in the table because `rear_ways` reads it through `g`.
macro_rules! book {
    ($g:expr, $k:expr) => {
        $g.m.rearwatch.books.get_mut($k).unwrap()
    };
}

fn flat(a: V3, b: V3) -> f64 {
    v3::dist_xz(a, b)
}

/// `n.toFixed(1)` for a time: JavaScript takes the larger on an exact tie, the formatter the even digit, and they differ only on a fraction of exactly .25.
fn to_fixed1(x: f64) -> String {
    if x - m::floor(x) == 0.25 {
        return format!("{:.1}", x + 0.01);
    }
    format!("{:.1}", x)
}

// ---- small reads

fn of(g: &mut Game, m: GuardId) -> Option<OrdersAssignmentId> {
    crate::orders::squad_orders_of(g, m)
}
fn post_of(g: &mut Game, m: GuardId) -> Option<V3> {
    let a = of(g, m)?;
    g.tab.orders_assignment[a].post
}
fn spare_of(g: &mut Game, a: OrdersAssignmentId) -> bool {
    g.tab.orders_assignment[a].spare.unwrap_or(false)
}
fn pos_of(g: &Game, m: GuardId) -> V3 {
    g.ch[g.gd[m].ch].pos
}
fn is_dead(g: &Game, m: GuardId) -> bool {
    g.gd[m].state == crate::game::GuardState::Dead
}
fn pin_row(g: &Game, key: PlayerId, m: GuardId) -> Option<RearPin> {
    g.m.rearwatch.books.get(key).unwrap().pins.get(m).copied()
}
fn is_pre(p: crate::doortools::Phase) -> bool {
    use crate::doortools::Phase;
    matches!(p, Phase::Stack | Phase::Ready | Phase::Place | Phase::Standoff | Phase::Armed)
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
/// three or more living men on an entry that is not a threshold's hold (rearwatch.ts carries, and the head of rearWatcher)
fn carries(g: &mut Game, op: DoorOpId) -> bool {
    let r = &g.tab.doortools_door_op[op];
    if r.thr.is_some() || !is_entry(r.kind) {
        return false;
    }
    r.men.iter().filter(|&&m| !is_dead(g, m)).count() >= 3
}
/// does an order of the door tools work this door
fn door_worked(g: &Game, door: DoorId) -> bool {
    g.door_tools.ops.iter().any(|&x| g.tab.doortools_door_op[x].door == door)
}
/// the room as fight sees it, or none
fn room_of(g: &mut Game, p: V3) -> Option<RoomfightRoomId> {
    crate::roomfight::room_fight_room_at(g, p)
}

/// This view's book, made if it is not yet. (rearwatch.ts bookOf)
pub fn book_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::rearwatch::Book {
    let key = crate::seats::view_key(g);
    // (the literal in the TypeScript is all empty: the same as zeroed)
    g.m.rearwatch.books.get_or_insert_with(key, Book::zeroed)
}

/// A door off its hinges, or unlatched and far enough open. (rearwatch.ts isOpenWay)
pub fn is_open_way(g: &mut crate::game::Game, d: DoorId) -> bool {
    if crate::doors::door_off(g, d) {
        return true;
    }
    if crate::doors::door_latched(g, d) {
        return false;
    }
    m::abs(crate::doors::door_angle(g, d)) >= g.k.rearwatch.ajar
}

/// The ways behind a stack on face `side` of `door`, best first. (rearwatch.ts rearWays)
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
    let key = crate::seats::view_key(g);
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
        let dists: Vec<f64> = marks.iter().map(|q| flat(at, *q)).collect();
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
        let far_is_a = if f.floor && has_a != has_b { !has_a } else { flat(a, c) >= flat(b, c) };
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

/// SHORT OF HIS MARK: between a stride and a step from it for too long, off the watch until he stands on it. (rearwatch.ts rearWatchTick shortOf)
fn short_of(g: &mut Game, key: PlayerId, m: GuardId) -> bool {
    let a = match of(g, m) {
        Some(a) => a,
        None => return false,
    };
    let q = match g.tab.orders_assignment[a].post {
        Some(q) => q,
        None => return false,
    };
    let d = flat(pos_of(g, m), q);
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

/// The man of this order already pinned behind this stack, who is not its lead nor the lane's man. (rearwatch.ts rearWatchTick hadOf)
fn had_of(g: &mut Game, key: PlayerId, op: DoorOpId, men: &[GuardId], lane: Option<GuardId>) -> Option<GuardId> {
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

/// `A.from ??= op.from.get(m) ?? where he stands`, and his eyes spare. (rearwatch.ts rearWatchTick)
fn keep_eyes_spare(g: &mut Game, a: OrdersAssignmentId, op: DoorOpId, m: GuardId) {
    g.tab.orders_assignment[a].spare = Some(true);
    if g.tab.orders_assignment[a].from.is_none() {
        let p = pos_of(g, m);
        let from = g.tab.doortools_door_op[op].from.get(m).copied().unwrap_or([p[0], 0.0, p[2]]);
        g.tab.orders_assignment[a].from = Some(from);
    }
}

/// Who of a stack should have this way: a line from where he is, then into the doorway, then the man already on it. (rearwatch.ts rearWatchTick rank)
fn rank(g: &mut Game, key: PlayerId, op: Option<DoorOpId>, m: GuardId, o: CoverOpeningId, front: V3) -> f64 {
    let mut r = 0.0;
    let p = pos_of(g, m);
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

/// Game.update, after the door tools' tick and before the cover's: who has the way behind each stack. (rearwatch.ts rearWatchTick)
pub fn rear_watch_tick(g: &mut crate::game::Game) {
    let key = crate::seats::view_key(g);
    book_of(g);
    room_keep_tick(g, key);
    if !crate::orders::squad_orders_active(g) {
        if !g.m.rearwatch.books.get(key).unwrap().pins.is_empty() {
            let pins: Vec<RearPin> = g.m.rearwatch.books.get(key).unwrap().pins.values().copied().collect();
            for p in pins {
                if p.raised == Some(true) && p.out == Some(true) {
                    g.tab.orders_assignment[p.a].spare = Some(false);
                }
            }
            book!(g, key).pins.clear();
        }
        return;
    }
    let grace = g.k.rearwatch.grace;
    let squad: Vec<GuardId> = crate::orders::squad_orders_men(g, None, None).into_iter().filter(|&m| !is_dead(g, m)).collect();
    let mut groups: Vec<Group> = Vec::new();
    let mut taken: JsSet<GuardId> = JsSet::new();
    for op in g.door_tools.ops.clone() {
        let op_men = g.tab.doortools_door_op[op].men.clone();
        for &m in &op_men {
            taken.add(m);
        }
        let mut men: Vec<GuardId> = Vec::new();
        for &m in &op_men {
            if is_dead(g, m) {
                continue;
            }
            let a = of(g, m);
            if a != g.tab.doortools_door_op[op].mine.get(m).copied() || post_of(g, m).is_none() || g.tab.doortools_door_op[op].pod.has(m) {
                continue;
            }
            men.push(m);
        }
        // (the whole file's marks, the men already sent in included: the stack is where it stood)
        let mut marks: Vec<V3> = Vec::new();
        for &m in &op_men {
            if is_dead(g, m) {
                continue;
            }
            let q = match g.tab.doortools_door_op[op].post.get(m) {
                Some(s) => Some(s.pos),
                None => post_of(g, m),
            };
            if let Some(q) = q {
                marks.push(q);
            }
        }
        let (door, side, phase, on_word) = {
            let r = &g.tab.doortools_door_op[op];
            (r.door, r.side, r.phase, r.on_word)
        };
        if is_pre(phase) {
            let lane = g.tab.doortools_door_op[op].lane.as_ref().map(|l| l.gd);
            let mut cands: Vec<GuardId> = Vec::new();
            for &m in &men {
                let a = of(g, m).unwrap();
                if spare_of(g, a) && Some(m) != lane {
                    cands.push(m);
                }
            }
            // (the man the doorway's manners have on the open door HAD the watch if he was pinned when the order came)
            if let Some(l) = lane {
                if pin_row(g, key, l).is_some() && men.len() >= 3 {
                    book!(g, key).by_op.set(op, l);
                }
            }
            // (NO NEW WATCH FOR A MOMENT: an entry given to a stack already on its marks files the men again and goes at once)
            if phase == crate::doortools::Phase::Stack && !on_word {
                let mut formed = true;
                for &m in &op_men {
                    if is_dead(g, m) {
                        continue;
                    }
                    let q = match g.tab.doortools_door_op[op].post.get(m) {
                        Some(s) => Some(s.pos),
                        None => post_of(g, m),
                    };
                    if let Some(q) = q {
                        if !(flat(pos_of(g, m), q) < FORMED_M) {
                            formed = false;
                            break;
                        }
                    }
                }
                if formed {
                    let bk = g.m.rearwatch.books.get(key).unwrap();
                    let by = bk.by_op.get(op).copied();
                    cands.retain(|&m| bk.pins.has(m) || by == Some(m));
                }
            }
            // (… and THE MAN WHO HAS IT KEEPS IT over an entry order)
            let had = had_of(g, key, op, &men, lane);
            if let Some(had) = had {
                if !cands.contains(&had) && carries(g, op) {
                    let a = of(g, had).unwrap();
                    keep_eyes_spare(g, a, op, had);
                    cands.push(had);
                }
            }
            // (a man let off to step onto his mark has the watch again once he is on it, unless the door then goes at once with no carrying it through)
            let brief = phase == crate::doortools::Phase::Stack && !on_word && !carries(g, op);
            let mut kept: Vec<GuardId> = Vec::new();
            for &m in &cands {
                if short_of(g, key, m) {
                    continue;
                }
                if brief {
                    let a = of(g, m).unwrap();
                    if g.m.rearwatch.books.get(key).unwrap().let_off.has(a) {
                        continue;
                    }
                }
                kept.push(m);
            }
            groups.push(Group { op: Some(op), door, side, men, marks, cands: kept, going: false });
            continue;
        }
        if !carries(g, op) || !is_going(phase) {
            continue;
        }
        // (the door is going: the one man who had the watch keeps it, or a way come open since the last man of the file; in the entry only the man at the END of the turn)
        let mut out: Vec<GuardId> = Vec::new();
        {
            let r = &g.tab.doortools_door_op[op];
            let last = r.queue.last().map(|q| q.gd);
            for &m in &men {
                if !r.crossed.has(m) && m != r.lead && Some(m) != r.first && (phase != crate::doortools::Phase::Enter || last == Some(m)) {
                    out.push(m);
                }
            }
        }
        let by = g.m.rearwatch.books.get(key).unwrap().by_op.get(op).copied();
        let kept = match by {
            Some(k) => Some(k),
            None => had_of(g, key, op, &men, None),
        };
        let centre = g.dr[door].frame_centre;
        // (a NEW man takes it only behind a can: behind a shove or a boot the first man is through before a gun turned round has settled)
        let cands: Vec<GuardId> = match kept {
            Some(k) if out.contains(&k) => vec![k],
            _ if is_slow(phase) => {
                js::sort_by(&mut out, |a, b| {
                    let db = flat(post_of(g, *b).unwrap(), centre);
                    let da = flat(post_of(g, *a).unwrap(), centre);
                    db - da
                });
                out.iter().copied().take(1).collect()
            }
            _ => Vec::new(),
        };
        groups.push(Group { op: Some(op), door, side, men, marks, cands, going: true });
    }
    // (a plain 'stack' on a door, no tool on it yet: its men by door)
    let mut plain: Vec<(DoorId, f64, Vec<GuardId>)> = Vec::new();
    for &m in &squad {
        if taken.has(m) {
            continue;
        }
        let a = match of(g, m) {
            Some(a) => a,
            None => continue,
        };
        let (is_stack, door, post) = {
            let r = &g.tab.orders_assignment[a];
            (r.order == "stack", r.door, r.post)
        };
        let (door, post) = match (is_stack, door, post) {
            (true, Some(d), Some(q)) => (d, q),
            _ => continue,
        };
        let side = crate::orders::side_of(g, door, post);
        match plain.iter_mut().find(|x| x.0 == door && x.1 == side) {
            Some(l) => l.2.push(m),
            None => plain.push((door, side, vec![m])),
        }
    }
    for (door, side, men) in plain {
        let marks: Vec<V3> = men.iter().map(|&m| post_of(g, m).unwrap()).collect();
        let mut cands: Vec<GuardId> = Vec::new();
        for &m in &men {
            let a = of(g, m).unwrap();
            if spare_of(g, a) && !short_of(g, key, m) {
                cands.push(m);
            }
        }
        groups.push(Group { op: None, door, side, men, marks, cands, going: false });
    }

    let mut next: JsMap<GuardId, RearPin> = JsMap::new();
    let mut held_out: JsSet<CoverOpeningId> = JsSet::new();
    // LEFT OUTSIDE: the entry had no point inside for him, or the order ended with him still on his mark. Dealt BEFORE the stacks'.
    let mut grouped: JsSet<GuardId> = JsSet::new();
    for gr in &groups {
        for &m in &gr.men {
            grouped.add(m);
        }
    }
    for m in g.m.rearwatch.books.get(key).unwrap().pins.keys() {
        if next.has(m) || grouped.has(m) || taken.has(m) || is_dead(g, m) {
            continue;
        }
        let p = pin_row(g, key, m).unwrap();
        let a = match of(g, m) {
            Some(a) if a == p.a => a,
            _ => continue,
        };
        let post = match g.tab.orders_assignment[a].post {
            Some(q) => q,
            None => continue,
        };
        let od = match g.tab.cover_opening[p.o].door {
            Some(d) => d,
            None => continue,
        };
        if !(is_open_way(g, od) || g.m.rearwatch.books.get(key).unwrap().shut.has(od)) {
            continue;
        }
        // (he had it through a door already going: doortools.ts turned every gun in, so his eyes are made spare again here)
        if p.going == Some(true) && !spare_of(g, a) {
            g.tab.orders_assignment[a].spare = Some(true);
            book!(g, key).pins.get_mut(m).unwrap().raised = Some(true);
        }
        if !spare_of(g, a) {
            continue;
        }
        if rear_sees(g, post, p.o, p.front).is_some() {
            let t = g.time;
            book!(g, key).pins.get_mut(m).unwrap().seen_t = t;
        } else if g.time - p.seen_t >= grace {
            continue;
        }
        let lone = Group { op: p.op, door: p.door, side: p.side, men: vec![m], marks: vec![post], cands: vec![m], going: false };
        if held_by_other(g, &lone, p.o, &squad) {
            continue;
        }
        if p.out != Some(true) {
            book!(g, key).pins.get_mut(m).unwrap().out = Some(true);
            let s = format!("{} {} keeps {} from outside", to_fixed1(g.time), g.gd[m].callsign, g.tab.cover_opening[p.o].id);
            note(book!(g, key), &s);
        }
        let row = pin_row(g, key, m).unwrap();
        next.set(m, row);
        held_out.add(p.o);
    }
    for gr in &groups {
        if gr.cands.is_empty() || (gr.men.len() < 2 && !gr.going) {
            continue;
        }
        let mut ways: Vec<RearWaysItem> = Vec::new();
        for w in rear_ways(g, gr.door, gr.side, &gr.marks) {
            if !held_out.has(w.o) && !held_by_other(g, gr, w.o, &squad) {
                ways.push(w);
            }
        }
        if ways.is_empty() {
            continue;
        }
        g.stats.rearwatch.ways += 1;
        let mut pick: Option<(GuardId, CoverOpeningId, V3, f64, bool)> = None;
        for w in &ways {
            let mut see: Vec<GuardId> = Vec::new();
            let mut blind = false;
            for &m in &gr.cands {
                if next.has(m) {
                    continue;
                }
                let q = post_of(g, m).unwrap();
                if rear_sees(g, q, w.o, w.front).is_some() {
                    see.push(m);
                }
            }
            if see.is_empty() {
                // (his line cut for a moment, the worked door's leaf swinging across it: still his)
                let time = g.time;
                let k = gr.cands.iter().copied().find(|&m| !next.has(m) && pin_row(g, key, m).map_or(false, |p| p.o == w.o && time - p.seen_t < grace));
                let k = match k {
                    Some(k) => k,
                    None => continue,
                };
                see = vec![k];
                blind = true;
            }
            let (op, centre, o, front) = (gr.op, g.dr[gr.door].frame_centre, w.o, w.front);
            js::sort_by(&mut see, |a, b| {
                let c = rank(g, key, op, *b, o, front) - rank(g, key, op, *a, o, front);
                if js::truthy(c) {
                    return c;
                }
                flat(post_of(g, *b).unwrap(), centre) - flat(post_of(g, *a).unwrap(), centre)
            });
            pick = Some((see[0], w.o, w.front, w.cls, blind));
            break;
        }
        let (pm, po, pfront, pcls, pblind) = match pick {
            Some(x) => x,
            None => {
                g.stats.rearwatch.no_line += 1;
                continue;
            }
        };
        g.stats.rearwatch.pinned += 1;
        let was = pin_row(g, key, pm);
        let a = of(g, pm).unwrap();
        let time = g.time;
        let same = was.map_or(false, |w| w.o == po && w.op == gr.op);
        let mut p = match was {
            Some(w) if same => w,
            _ => RearPin {
                o: po,
                front: pfront,
                door: gr.door,
                side: gr.side,
                op: gr.op,
                cls: pcls,
                t0: time,
                seen_t: time,
                a,
                going: None,
                raised: None,
                held: None,
                late: None,
                out: None,
                to: None,
            },
        };
        p.cls = pcls;
        p.front = pfront;
        p.a = a;
        p.out = Some(false);
        p.going = Some(gr.going);
        if !pblind {
            p.seen_t = time;
        }
        // (the same object stays in the old map as in the new in the TypeScript: the old row sees the writes too)
        if same {
            *book!(g, key).pins.get_mut(pm).unwrap() = p;
        }
        held_out.add(po); // (no second stack pins a man to the same way this tick)
        next.set(pm, p);
        if let Some(op) = gr.op {
            book!(g, key).by_op.set(op, pm);
        }
        if !same {
            g.stats.rearwatch.pins += 1;
            let s = format!(
                "{} {} on {} ({} beyond) behind the stack on {}{}",
                to_fixed1(time),
                g.gd[pm].callsign,
                g.tab.cover_opening[po].id,
                if js::truthy(pcls) { "known" } else { "uncleared" },
                g.dr[gr.door].def.name,
                if gr.going { " · through the entry" } else { "" }
            );
            note(book!(g, key), &s);
        }
        // (the door is going: doortools.ts has just turned every gun in; his eyes stay spare, as on the wait)
        if gr.going {
            keep_eyes_spare(g, a, gr.op.unwrap(), pm);
        }
    }
    for m in g.m.rearwatch.books.get(key).unwrap().pins.keys() {
        if next.has(m) {
            continue;
        }
        let p = pin_row(g, key, m).unwrap();
        let gr = groups.iter().find(|x| x.men.contains(&m));
        let w_all = match gr {
            Some(x) => rear_ways(g, x.door, x.side, &x.marks),
            None => Vec::new(),
        };
        let w = w_all.iter().find(|x| x.o == p.o).copied();
        // (why, for the log: read by the checks and whoever asks why a back door went unwatched)
        let why = if is_dead(g, m) {
            "down"
        } else if let Some(x) = gr {
            if !x.cands.contains(&m) {
                if !next.is_empty() {
                    "another man has the rear"
                } else {
                    "not the rear man now"
                }
            } else if let Some(w) = w {
                if held_by_other(g, x, p.o, &squad) {
                    "another holds it"
                } else {
                    let q = post_of(g, m).unwrap();
                    if rear_sees(g, q, w.o, w.front).is_none() {
                        "no line from his mark"
                    } else {
                        "a way before it"
                    }
                }
            } else {
                "shut, or out of reach"
            }
        } else if p.op.map_or(false, |op| g.tab.doortools_door_op[op].crossed.has(m)) {
            "through the door"
        } else {
            "off the stack"
        };
        let s = format!("{} {} off {} · {}", to_fixed1(g.time), g.gd[m].callsign, g.tab.cover_opening[p.o].id, why);
        note(book!(g, key), &s);
    }
    book!(g, key).pins = next;
}

/// One row of the log, which keeps the newest 200. (rearwatch.ts note)
pub fn note(b: &mut crate::rearwatch::Book, s: &str) {
    b.log.push(String::from(s));
    b.rows += 1.0;
    if b.log.len() > 200 {
        b.log.remove(0);
    }
}

/// The point just inside the door on `p`'s side, `k` from its frame. (rearwatch.ts roomKeepTick inner)
fn inner(g: &Game, o: CoverOpeningId, p: V3, k: f64) -> V3 {
    let (at, n) = (g.tab.cover_opening[o].at, g.tab.cover_opening[o].n);
    let s = if (p[0] - at[0]) * n[0] + (p[2] - at[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 };
    [at[0] + n[0] * s * k, CHEST, at[2] + n[1] * s * k]
}
/// The squad but `m` that stands in `room`. (rearwatch.ts roomKeepTick others)
fn others(g: &mut Game, squad: &[GuardId], m: GuardId, room: RoomfightRoomId) -> Vec<GuardId> {
    let mut out = Vec::new();
    for &x in squad {
        if x != m && room_of(g, pos_of(g, x)) == Some(room) {
            out.push(x);
        }
    }
    out
}
/// A keeper let go: he follows, last. (rearwatch.ts roomKeepTick let_)
fn let_go(g: &mut Game, pl: PlayerId, m: GuardId, why: &str) {
    let r = match book!(g, pl).keeps.delete(m) {
        Some(r) => r,
        None => return,
    };
    if let Some(a) = of(g, m) {
        if a == r.pin.a && g.tab.orders_assignment[a].post.is_none() {
            g.tab.orders_assignment[a].going = String::from(FALLING);
        }
    }
    let s = format!("{} {} off {} · {}", to_fixed1(g.time), g.gd[m].callsign, g.tab.cover_opening[r.pin.o].id, why);
    note(book!(g, pl), &s);
}
/// The walk to Sam, once a judging for each man. (rearwatch.ts roomKeepTick walk)
fn walk(g: &mut Game, far: &mut JsMap<GuardId, f64>, x: GuardId, sam_at: V3) -> f64 {
    if let Some(l) = far.get(x) {
        return *l;
    }
    let l = walk_to(g, pos_of(g, x), sam_at);
    far.set(x, l);
    l
}

/// A TAKEN ROOM'S OTHER DOOR, KEPT WHILE THE SQUAD LEAVES. (rearwatch.ts roomKeepTick)
pub fn room_keep_tick(g: &mut crate::game::Game, pl: PlayerId) {
    if !crate::orders::squad_orders_active(g) {
        for m in book!(g, pl).keeps.keys() {
            let_go(g, pl, m, "switched off");
        }
        let b = book!(g, pl);
        b.was = Vec::new();
        b.shut.clear();
        return;
    }
    let now = crate::doortools::door_tools_door_keepers(g);
    let sam = g.player;
    let sam_up = !g.pl[sam].down;
    let sam_ch = g.pl[sam].ch;
    let squad: Vec<GuardId> = crate::orders::squad_orders_men(g, None, None).into_iter().filter(|&m| !is_dead(g, m)).collect();
    // ---- the hand-over: he had a door last frame, the entry has let go of him since, and what he has now is 'fall in'
    for (m, door, a0) in book!(g, pl).was.clone() {
        if book!(g, pl).keeps.has(m) || is_dead(g, m) || now.iter().any(|x| x.0 == m) {
            continue;
        }
        let a = match of(g, m) {
            Some(a) => a,
            None => continue,
        };
        if a == a0 || g.tab.orders_assignment[a].post.is_some() {
            continue;
        }
        let o = crate::cover::cover_ready(g).into_iter().find(|&x| g.tab.cover_opening[x].door == Some(door));
        let o = match o {
            Some(o) => o,
            None => continue,
        };
        let p = pos_of(g, m);
        let front = inner(g, o, p, 0.7);
        let room = match room_of(g, p) {
            Some(r) => Some(r),
            None => room_of(g, front),
        };
        let room = match room {
            Some(r) => r,
            None => continue,
        };
        if crate::entrypoints::entry_points_far_known(g, o, inner(g, o, p, 0.4)) || door_worked(g, door) {
            continue;
        }
        // (nobody left in it to keep it for)
        if others(g, &squad, m, room).is_empty() && !(sam_up && room_of(g, g.ch[sam_ch].pos) == Some(room)) {
            continue;
        }
        // (… and Sam is already too far off to stay behind for)
        if g.k.rearwatch.room_m > 0.0 && flat(p, g.ch[sam_ch].pos) > g.k.rearwatch.room_m {
            continue;
        }
        let to: Option<V3> = g.tab.orders_assignment[a0].watch.map(|w| [w[0], w[1], w[2]]);
        let time = g.time;
        book!(g, pl).keeps.set(
            m,
            RoomKeep {
                pin: RearPin {
                    o,
                    front,
                    door,
                    side: 1.0,
                    op: None,
                    cls: 0.0,
                    t0: time,
                    seen_t: time,
                    a,
                    going: None,
                    raised: None,
                    held: None,
                    late: None,
                    out: None,
                    to,
                },
                room,
                out_t: f64::NAN,
                behind: None,
                at: 0.0,
            },
        );
        g.stats.rearwatch.rooms += 1;
        let s = format!("{} {} keeps {} while the squad leaves the room", to_fixed1(time), g.gd[m].callsign, g.tab.cover_opening[o].id);
        note(book!(g, pl), &s);
    }
    book!(g, pl).was = now;
    // ---- a kept door that became a stack's way behind: until the squad has been beyond it, an order works it, or the squad is out of the room
    for door in book!(g, pl).shut.keys() {
        let w = *book!(g, pl).shut.get(door).unwrap();
        let w_room = w.room;
        let mut gone = crate::entrypoints::entry_points_far_known(g, w.o, w.at) || door_worked(g, door);
        if !gone {
            let mut still = false;
            for &x in &squad {
                if room_of(g, pos_of(g, x)) == Some(w_room) {
                    still = true;
                    break;
                }
            }
            gone = !(still || (sam_up && room_of(g, g.ch[sam_ch].pos) == Some(w_room)));
        }
        if gone {
            book!(g, pl).shut.delete(door);
        }
    }
    // ---- each kept door: his still?
    for m in book!(g, pl).keeps.keys() {
        let r = book!(g, pl).keeps.get(m).unwrap().clone();
        let p = r.pin;
        let o = p.o;
        if is_dead(g, m) {
            book!(g, pl).keeps.delete(m);
            continue;
        }
        let a = of(g, m);
        let a = match a {
            Some(a) if g.tab.orders_assignment[a].post.is_none() => a,
            _ => {
                book!(g, pl).keeps.delete(m);
                // (sent to a mark, the stack on the room's next door: shut, the door he was keeping is still a way behind it)
                let at_him = inner(g, o, pos_of(g, m), 0.4);
                let stays = a.is_some()
                    && !is_open_way(g, p.door)
                    && !crate::entrypoints::entry_points_far_known(g, o, at_him)
                    && !door_worked(g, p.door)
                    && g.tab.orders_assignment[a.unwrap()].door != Some(p.door);
                if stays {
                    book!(g, pl).shut.set(p.door, BookShutValue { o, room: r.room, at: at_him });
                    g.stats.rearwatch.room_ways += 1;
                }
                let s = format!(
                    "{} {} off {} · another order{}",
                    to_fixed1(g.time),
                    g.gd[m].callsign,
                    g.tab.cover_opening[o].id,
                    if stays { " (still a way behind the stack)" } else { "" }
                );
                note(book!(g, pl), &s);
                continue;
            }
        };
        // ('fall in' given again: the same keep, on the new order)
        if a != p.a {
            book!(g, pl).keeps.get_mut(m).unwrap().pin.a = a;
        }
        if !sam_up {
            let_go(g, pl, m, "the player is down");
            continue;
        }
        let at_him = inner(g, o, pos_of(g, m), 0.4);
        if crate::entrypoints::entry_points_far_known(g, o, at_him) {
            let_go(g, pl, m, "the squad has been beyond it");
            continue;
        }
        let sam_at = g.ch[sam_ch].pos;
        if g.k.rearwatch.room_m > 0.0 && flat(pos_of(g, m), sam_at) > g.k.rearwatch.room_m {
            g.stats.rearwatch.room_far += 1;
            let_go(g, pl, m, "the player has gone on");
            continue;
        }
        let o_at = g.tab.cover_opening[o].at;
        let mut other_has = door_worked(g, p.door);
        if !other_has {
            for &x in &squad {
                if x == m {
                    continue;
                }
                if let Some(xa) = of(g, x) {
                    let row = &g.tab.orders_assignment[xa];
                    if row.post.is_some() && (row.door == Some(p.door) || row.watch.map_or(false, |w| flat(w, o_at) < 1.5)) {
                        other_has = true;
                        break;
                    }
                }
            }
        }
        if other_has {
            let_go(g, pl, m, "another order has it");
            continue;
        }
        // (several men keeping doors of one room do not wait on each other: DEEPER and PASSED are by the walk to Sam. Asked four times a second)
        let room = r.room;
        if room_of(g, sam_at) == Some(room) {
            let row = book!(g, pl).keeps.get_mut(m).unwrap();
            row.out_t = f64::NAN;
            row.behind = None;
        } else if g.time >= r.at || r.at - g.time > 1.0 {
            let time = g.time;
            let row = book!(g, pl).keeps.get_mut(m).unwrap();
            row.at = time + 0.25;
            if row.out_t.is_nan() || time < row.out_t {
                row.out_t = time;
            }
            let out_t = row.out_t;
            let mut far: JsMap<GuardId, f64> = JsMap::new();
            let dm = walk(g, &mut far, m, sam_at);
            let mut rest: Vec<GuardId> = Vec::new();
            for x in others(g, &squad, m, room) {
                if !book!(g, pl).keeps.has(x) || walk(g, &mut far, x, sam_at) > dm {
                    rest.push(x);
                }
            }
            if rest.is_empty() {
                let_go(g, pl, m, "the squad is out of the room");
                continue;
            }
            // (THE LAST MAN PASSES HIM: the men who had further to walk to Sam than he did when Sam left have all come by)
            if book!(g, pl).keeps.get(m).unwrap().behind.is_none() {
                let mut behind: Vec<GuardId> = Vec::new();
                for &x in &rest {
                    if walk(g, &mut far, x, sam_at) > dm + 0.3 {
                        behind.push(x);
                    }
                }
                book!(g, pl).keeps.get_mut(m).unwrap().behind = Some(behind);
            }
            let behind = book!(g, pl).keeps.get(m).unwrap().behind.clone().unwrap();
            // (… and he does not crowd them at the way out: a mate still in the room whose place is beyond it goes out first)
            let mut queue = false;
            for &x in &rest {
                if let Some(xa) = of(g, x) {
                    let (xpost, xgoal) = (g.tab.orders_assignment[xa].post, g.tab.orders_assignment[xa].goal);
                    if xpost.is_none() && room_of(g, xgoal) != Some(room) {
                        queue = true;
                        break;
                    }
                }
            }
            if !behind.is_empty() && !queue {
                let mut passed = true;
                for &x in &behind {
                    if !(is_dead(g, x) || !rest.contains(&x) || walk(g, &mut far, x, sam_at) < dm - 0.3) {
                        passed = false;
                        break;
                    }
                }
                if passed {
                    let_go(g, pl, m, "the last man has passed him");
                    continue;
                }
            }
            if g.time - out_t >= g.k.rearwatch.room_secs {
                g.stats.rearwatch.room_late += 1;
                let_go(g, pl, m, "on the clock");
                continue;
            }
        }
        // (the last place in the file; the door's one name, as the squad's rows say it)
        let name = match g.tab.cover_opening[o].door {
            Some(d) => crate::orders::door_words(g, d),
            None => g.tab.cover_opening[o].id.clone(),
        };
        let row = &mut g.tab.orders_assignment[a];
        row.follow = 1e3;
        row.going = format!("holding · {}", name);
    }
}

/// How far from `a` to `b` on foot: the straight line when nothing stands in it, else the nav grid's path. (rearwatch.ts walkTo)
pub fn walk_to(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let d = flat(a, b);
    if !crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [b[0], 0.45, b[2]], None) {
        return d;
    }
    let path = match crate::scene_collision::nav_grid_find_path(g, a, b, Some(3000.0), None) {
        Some(p) => p,
        None => return d + 20.0,
    };
    let mut l = 0.0;
    let mut q = a;
    for r in path {
        l += flat(q, r);
        q = r;
    }
    l + flat(q, b)
}

/// orders.ts, the fall-in leg: this man's feet stay where they are, in his seat's book. (rearwatch.ts roomKept)
pub fn room_kept(g: &mut crate::game::Game, gd: GuardId) -> bool {
    // (viewOf: his seat's book; a cast is one book, its own)
    let key = if g.in_cast { crate::seats::view_key(g) } else { crate::seats::view_of(g, gd) };
    g.m.rearwatch.books.get(key).map_or(false, |b| b.keeps.has(gd))
}

/// Somebody not of this stack already has that way. (rearwatch.ts heldByOther)
pub fn held_by_other(g: &mut crate::game::Game, g_2: &crate::rearwatch::Group, o: CoverOpeningId, squad: &[GuardId]) -> bool {
    let (od, at) = (g.tab.cover_opening[o].door, g.tab.cover_opening[o].at);
    if let Some(op) = g_2.op {
        if let Some(l) = g.tab.doortools_door_op[op].lane.as_ref() {
            if od.is_some() && Some(l.door) == od && !is_dead(g, l.gd) {
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
            let row = &g.tab.orders_assignment[a];
            if let Some(post) = row.post {
                if flat(pos_of(g, m), post) <= 1.5 && ((od.is_some() && row.door == od) || row.watch.map_or(false, |w| flat(w, at) < 1.5)) {
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

/// The point of a way behind a man stood at `p` can watch: the doorway, its leaf's edge, or the floor just before it. (rearwatch.ts rearSees)
pub fn rear_sees(g: &mut crate::game::Game, p: V3, o: CoverOpeningId, front: V3) -> Option<V3> {
    let s = crate::cover::cover_sight_from(g, p, o);
    if s.is_some() {
        return s;
    }
    let d = flat(p, front);
    if d <= g.k.rearwatch.reach && d > 0.9 && !crate::scene_collision::segment_blocked(g, [p[0], EYE, p[2]], front, None, None, None) {
        Some(front)
    } else {
        None
    }
}

/// cover.ts deal(): the way this man is pinned to. (rearwatch.ts rearPinOf)
pub fn rear_pin_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::rearwatch::RearPin> {
    let key = crate::seats::view_key(g);
    let b = g.m.rearwatch.books.get(key)?;
    match b.pins.get(gd) {
        Some(p) => Some(*p),
        None => b.keeps.get(gd).map(|r| r.pin),
    }
}

/// The pin, for the debug read-out. (rearwatch.ts rearPin)
pub fn rear_pin(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::rearwatch::RearPin> {
    let key = crate::seats::view_key(g);
    g.m.rearwatch.books.get(key)?.pins.get(gd).copied()
}

/// doortools.ts, the entry's queue: nobody is held back, REARWATCH.hold being off on the page. (rearwatch.ts rearHolds)
pub fn rear_holds(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool {
    false
}

/// doortools.ts enter(): the turn the men go in by, with the rear man put last. (rearwatch.ts rearLast)
pub fn rear_last(g: &mut crate::game::Game, op: DoorOpId, turn: &[GuardId], first: GuardId) -> Vec<GuardId> {
    if turn.len() < 3 {
        return turn.to_vec();
    }
    let w = match rear_watcher(g, op) {
        Some(w) => w,
        None => return turn.to_vec(),
    };
    if w == first || !turn.contains(&w) || turn[turn.len() - 1] == w {
        return turn.to_vec();
    }
    g.stats.rearwatch.last += 1;
    let mut out: Vec<GuardId> = turn.iter().copied().filter(|&m| m != w).collect();
    out.push(w);
    out
}

/// doortools.ts firstIn(): the man of this order pinned on a way behind and held on it through the entry. (rearwatch.ts rearWatcher)
pub fn rear_watcher(g: &mut crate::game::Game, op: DoorOpId) -> Option<GuardId> {
    if !carries(g, op) {
        return None;
    }
    let key = crate::seats::view_key(g);
    let b = g.m.rearwatch.books.get(key)?;
    let w = *b.by_op.get(op)?;
    if b.pins.get(w)?.op == Some(op) {
        Some(w)
    } else {
        None
    }
}

/// The log's newest rows. (rearwatch.ts rearWatchLog)
pub fn rear_watch_log(g: &mut crate::game::Game) -> Vec<String> {
    let key = crate::seats::view_key(g);
    match g.m.rearwatch.books.get(key) {
        Some(b) => b.log.clone(),
        None => Vec::new(),
    }
}

/// How many rows that log has ever been handed. (rearwatch.ts rearWatchRows)
pub fn rear_watch_rows(g: &mut crate::game::Game) -> f64 {
    let key = crate::seats::view_key(g);
    g.m.rearwatch.books.get(key).map_or(0.0, |b| b.rows)
}

/// The rule's line for the checks and the debug read-out. (rearwatch.ts rearWatchSummary)
pub fn rear_watch_summary(g: &mut crate::game::Game) -> String {
    let mut now: Vec<String> = Vec::new();
    let key = crate::seats::view_key(g);
    if let Some(b) = g.m.rearwatch.books.get(key) {
        for (m, p) in b.pins.iter() {
            now.push(format!("{} on {}", g.gd[m].callsign, g.tab.cover_opening[p.o].id));
        }
        for (m, r) in b.keeps.iter() {
            now.push(format!("{} keeps {}", g.gd[m].callsign, g.tab.cover_opening[r.pin.o].id));
        }
    }
    let now = if now.is_empty() { String::from("nobody pinned") } else { now.join(", ") };
    let t = &g.stats.rearwatch;
    format!(
        "rear watch on: {} · {} pins · held back in {} entries ({} let go on the clock) · put last in {} · frames with a way behind {}, pinned {}, no rear mark sees it {} · rooms' doors kept over fall in {} ({} let go on the clock)",
        now, t.pins, t.held, t.late, t.last, t.ways, t.pinned, t.no_line, t.rooms, t.room_late
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fixed_takes_the_larger_on_a_tie_as_javascript_does() {
        // (each right-hand side is what (x).toFixed(1) prints under bun)
        for (x, s) in [(0.25, "0.3"), (1.25, "1.3"), (0.75, "0.8"), (1.75, "1.8"), (0.35, "0.3"), (0.05, "0.1"), (2.5, "2.5"), (0.0, "0.0"), (12.34, "12.3")] {
            assert_eq!(to_fixed1(x), s);
        }
    }

    #[test]
    fn the_log_keeps_the_newest_200_and_counts_them_all() {
        let mut b = Book::zeroed();
        for i in 0..205 {
            note(&mut b, &format!("{}", i));
        }
        assert_eq!(b.rows, 205.0);
        assert_eq!(b.log.len(), 200);
        assert_eq!(b.log[0], "5");
        assert_eq!(b.log[199], "204");
    }
}
