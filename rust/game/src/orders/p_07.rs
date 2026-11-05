//! src/game/orders.ts lines 2099 to 2475
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }

// ---- seams: what the skeleton cannot do yet ----

/// orders.ts 2198 and on: asking a price `dearFor` handed back for a cell (cleared has no public way to answer one).
fn seam_dear_price(g: &mut Game, price: &crate::cleared::DearPrice, ix: f64, iz: f64) -> f64 { todo!("needs: cleared::dear_price(g, price, ix, iz) in sig.rs") }
/// orders.ts 2270 and on: asking a price `doorwayLanePrice` handed back for a cell; `&mut`, for its memo.
fn seam_lane_price(g: &mut Game, price: &mut crate::doorway::DoorwayLanePrice, ix: f64, iz: f64) -> f64 { todo!("needs: doorway::doorway_lane_price_at(g, price, ix, iz) in sig.rs") }
/// orders.ts 2283 and on: asking a price `seenPrice` handed back for a cell.
fn seam_seen_price(g: &mut Game, price: &crate::seenwalk::SeenPrice, ix: f64, iz: f64) -> f64 { todo!("needs: seenwalk::seen_price_at(g, price, ix, iz) in sig.rs") }

// ---- a walk's price list, as the closures of the TypeScript held it ----

/// a `penalty` a walk is priced by: `dearFor`'s, or `route`'s and `sideFor`'s own (a doorway `dear` is `wall`, else `unk`, else 1)
#[derive(Clone, Copy)]
enum Pen<'a> {
    Dear(&'a crate::cleared::DearPrice),
    Walk { dear: &'a JsSet<crate::Num>, wall: f64, unk: Option<&'a crate::cleared::DearPrice> },
}
fn price(g: &mut Game, pen: Pen<'_>, ix: f64, iz: f64) -> f64 {
    match pen {
        Pen::Dear(u) => seam_dear_price(g, u, ix, iz),
        Pen::Walk { dear, wall, unk } => {
            let i = crate::scene_collision::nav_grid_idx(g, ix, iz);
            if dear.has(crate::Num(i)) { return wall; }
            match unk { Some(u) => seam_dear_price(g, u, ix, iz), None => 1.0 }
        }
    }
}
fn find_path(g: &mut Game, a: V3, b: V3, max_iter: Option<f64>, pen: Option<Pen<'_>>) -> Option<Vec<V3>> {
    match pen {
        Some(p) => crate::scene_collision::nav_grid_find_path(g, a, b, max_iter, Some(&mut |g: &mut Game, x: f64, z: f64| price(g, p, x, z))),
        None => crate::scene_collision::nav_grid_find_path(g, a, b, max_iter, None),
    }
}
fn along(g: &mut Game, from: V3, path: &[V3], pen: Pen<'_>) -> f64 {
    crate::cleared::cleared_rooms_dear_along(g, from, path, &mut |g: &mut Game, x: f64, z: f64| price(g, pen, x, z))
}
fn along_lanes(g: &mut Game, from: V3, path: &[V3], lanes: &mut crate::doorway::DoorwayLanePrice) -> f64 {
    crate::cleared::cleared_rooms_dear_along(g, from, path, &mut |g: &mut Game, x: f64, z: f64| seam_lane_price(g, lanes, x, z))
}
fn along_seen(g: &mut Game, from: V3, path: &[V3], seen: &crate::seenwalk::SeenPrice) -> f64 {
    crate::cleared::cleared_rooms_dear_along(g, from, path, &mut |g: &mut Game, x: f64, z: f64| seam_seen_price(g, seen, x, z))
}
fn is_blocked(g: &mut Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
/// `nav.isBlocked(p) ? this.freeSpot(p) ?? p : p`
fn free_from(g: &mut Game, p: V3) -> V3 {
    if is_blocked(g, p[0], p[2]) { squad_orders_free_spot(g, p).unwrap_or(p) } else { p }
}
/// the cells of a doorway's width, made all but a wall (sideFor's and route's `dear`)
fn doorway_cells(g: &mut Game, o: DoorId, dear: &mut JsSet<crate::Num>) {
    let (h, a, width) = (g.dr[o].def.hinge, g.dr[o].def.closed_dir, g.dr[o].def.width);
    let (x, z) = (m::cos(a), m::sin(a));
    let mut s = 0.0;
    while s <= width + 1e-6 {
        for n in [-0.3, 0.0, 0.3] {
            let cell = g.col.nav.cell;
            let i = crate::scene_collision::nav_grid_idx(g, m::floor((h[0] + x * s - z * n) / cell), m::floor((h[1] + z * s + x * n) / cell));
            dear.add(crate::Num(i));
        }
        s += 0.2;
    }
}
/// a walk from `from` by corners: does it cross `d`
fn through(g: &mut Game, from: V3, p: &[V3], d: DoorId) -> bool {
    let mut a = from;
    for &b in p { if crate::guards::crosses_door(g, a, b, d).is_some() { return true; } a = b; }
    false
}
/// metres of a walk from `from` by corners
fn len_from(from: V3, q: &[V3]) -> f64 {
    let (mut l, mut a) = (0.0, from);
    for &b in q { l += flat(a, b); a = b; }
    l
}
/// metres of a walk that starts at its first point
fn len_of(w: &[V3]) -> f64 {
    let mut l = 0.0;
    for i in 1..w.len() { l += flat(w[i - 1], w[i]); }
    l
}
fn on_floor(q: V3) -> V3 { [q[0], 0.0, q[2]] }
/// `path.slice(0, -1)` on the floor
fn all_but_last(path: &[V3]) -> Vec<V3> {
    path[..path.len().saturating_sub(1)].iter().map(|&q| on_floor(q)).collect()
}

// ---- workedMarks ----

/// `b` metres out from the wall on this face (negative: beyond the door), `l` along it from the doorway's middle
fn worked_at(c: V3, n: [f64; 2], u: [f64; 2], b: f64, l: f64, y: f64) -> V3 {
    [c[0] + n[0] * b + u[0] * l, y, c[2] + n[1] * b + u[1] * l]
}
fn worked_walk(g: &mut Game, front: V3, p: V3) -> bool {
    !is_blocked(g, p[0], p[2]) && !crate::scene_collision::solid_blocked(g, front, [p[0], 0.45, p[2]], None)
}
fn worked_out(v: &SquadOrdersWorkedValueItem) -> SquadOrdersWorkedMarks {
    SquadOrdersWorkedMarks {
        jambless: v.jambless,
        marks: v.marks.iter().map(|x| SquadOrdersWorkedMarksMarksItem { pos: x.pos, walk: x.walk, seen: x.seen }).collect(),
    }
}

/// A door face with no floor at either jamb: every place the men could stand for it instead, worked out once per face.   (orders.ts SquadOrders.workedMarks)
pub fn squad_orders_worked_marks(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::orders::SquadOrdersWorkedMarks {
    let key = if side > 0.0 { 0 } else { 1 };
    if !g.orders.worked.has(d) { g.orders.worked.set(d, vec![None, None]); }
    if let Some(had) = g.orders.worked.get(d).and_then(|both| both[key].as_ref()) { return worked_out(had); }
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let (c, w) = (g.dr[d].frame_centre, g.dr[d].def.width);
    let h = w / 2.0;
    let (jamb_out, jamb_past) = (g.k.orders.jamb_out, g.k.orders.jamb_past);
    let front = worked_at(c, n, u, jamb_out, 0.0, 0.45);
    let mut res = SquadOrdersWorkedValueItem { jambless: false, marks: Vec::new() };
    let leaves = core::mem::take(&mut g.col.dynamic_boxes);   // (statics alone: a leaf standing open across the floor is not the level's shape)
    res.jambless = !worked_walk(g, front, worked_at(c, n, u, jamb_out, h + jamb_past, 0.0)) && !worked_walk(g, front, worked_at(c, n, u, jamb_out, -h - jamb_past, 0.0));
    if res.jambless {
        let mut eyes: Vec<[f64; 2]> = Vec::new();   // the floor beyond the door that looks at the opening
        let mut b = 0.75;
        while b <= 7.5 {
            let mut l = -6.0;
            while l <= 6.0 {
                let q = worked_at(c, n, u, -b, l, 1.3);
                if !is_blocked(g, q[0], q[2]) && !crate::scene_collision::segment_blocked(g, q, [c[0], 1.3, c[2]], None, None, None) { eyes.push([b, l]); }
                l += 0.75;
            }
            b += 0.75;
        }
        let mut b = jamb_out;
        while b <= 3.9 {
            for k in 0..7 {
                for sg in [1.0, -1.0] {
                    let l = sg * (h + 0.25 + (k as f64) * 0.3);
                    let pos = worked_at(c, n, u, b, l, 0.0);
                    if !worked_walk(g, front, pos) { continue; }
                    let mut near_prop = false;
                    for i in 0..g.props.props.len() {
                        let pr = &g.tab.props_prop[g.props.props[i]];
                        if crate::roomclear::prop_gap(crate::roomclear::PropGapP { home: pr.home, hx: pr.hx, hz: pr.hz }, pos) < 0.3 { near_prop = true; break; }
                    }
                    if near_prop { continue; }
                    let mut saw = 0.0;
                    for &[eb, el] in &eyes {   // (a line that crosses the wall's plane inside this doorway, and nothing else in it)
                        if m::abs(el + (l - el) * eb / (eb + b)) < h && !crate::scene_collision::segment_blocked(g, worked_at(c, n, u, -eb, el, 1.3), [pos[0], 1.3, pos[2]], None, None, None) { saw += 1.0; }
                    }
                    res.marks.push(SquadOrdersWorkedValueItemMarksItem { pos, walk: flat(front, pos), seen: if !eyes.is_empty() { saw / (eyes.len() as f64) } else { 0.0 } });
                }
            }
            b += 0.3;
        }
    }
    g.col.dynamic_boxes = leaves;
    let out = worked_out(&res);
    if let Some(both) = g.orders.worked.get_mut(d) { both[key] = Some(res); }
    out
}

/// The marks teammates outside `men` hold on this door under an order.   (orders.ts SquadOrders.heldOn)
pub fn squad_orders_held_on(g: &mut crate::game::Game, d: DoorId, men: &[GuardId], ignore: Option<&[GuardId]>) -> Vec<V3> {
    let ignore = ignore.unwrap_or(&[]);
    let mut out = Vec::new();
    for (gd, &a) in g.orders.asg.iter() {
        let row = &g.tab.orders_assignment[a];
        if !men.contains(&gd) && !ignore.contains(&gd) && g.gd[gd].state != crate::game::GuardState::Dead && row.door == Some(d) {
            if let Some(post) = row.post { out.push(post); }
        }
    }
    out
}

/// `n` worked marks for such a face, best first by walk + workSeenM × seen, returned nearest the door first; None: the face has floor at a jamb, or not that many marks.   (orders.ts SquadOrders.workedPick)
pub fn squad_orders_worked_pick(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, taken: &[V3], min_m: Option<f64>) -> Option<Vec<V3>> {
    let min_m = min_m.unwrap_or(0.0);
    if n < 1.0 { return None; }
    let w = squad_orders_worked_marks(g, d, side);
    if !w.jambless { return None; }
    let (seen_m, gap, fc) = (g.k.orders.work_seen_m, g.k.orders.file_gap, g.dr[d].frame_centre);
    let mut sorted = w.marks.clone();
    js::sort_by(&mut sorted, |a, b| (a.walk + seen_m * a.seen) - (b.walk + seen_m * b.seen));
    let mut out: Vec<V3> = Vec::new();
    for mk in &sorted {
        if (out.len() as f64) >= n { break; }
        if flat(mk.pos, fc) < min_m || taken.iter().any(|&q| flat(q, mk.pos) < 0.7) || out.iter().any(|&q| flat(q, mk.pos) < gap - 0.1) { continue; }
        out.push(v3::copy(mk.pos));
    }
    if (out.len() as f64) < n { return None; }
    js::sort_by(&mut out, |a, b| flat(*a, fc) - flat(*b, fc));
    Some(out)
}

/// Worked marks as a stack: the first man has it to open, everybody faces it; the men to one hand of the opening are one file.   (orders.ts SquadOrders.workedStack)
pub fn squad_orders_worked_stack(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, taken: &[V3]) -> Option<Vec<crate::orders::StackPost>> {
    let marks = squad_orders_worked_pick(g, d, side, n, taken, None)?;
    let (th, hg, w, c) = (g.dr[d].def.closed_dir, g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].frame_centre);
    let mut out = Vec::with_capacity(marks.len());
    for (i, &p) in marks.iter().enumerate() {
        let hi = (p[0] - hg[0]) * m::cos(th) + (p[2] - hg[1]) * m::sin(th) > w / 2.0;
        out.push(StackPost {
            pos: p,
            face: v3::yaw_to(p, c),
            stack_side: (if hi { 1.0 } else { -1.0 }) * side,
            lead: i == 0,
            group: if n > 2.0 { Some(if hi { 0.0 } else { 1.0 }) } else { None },
        });
    }
    Some(out)
}

/// Is this place the mark a man of the order already stands on, on this door?   (orders.ts SquadOrders.stoodOn)
pub fn squad_orders_stood_on(g: &mut crate::game::Game, d: DoorId, men: &[GuardId], p: V3) -> bool {
    for (gd, &a) in g.orders.asg.iter() {
        let row = &g.tab.orders_assignment[a];
        if men.contains(&gd) && g.gd[gd].state != crate::game::GuardState::Dead && row.door == Some(d) {
            if let Some(post) = row.post {
                if flat(post, p) < 0.3 && flat(g.ch[g.gd[gd].ch].pos, post) < 0.6 { return true; }
            }
        }
    }
    false
}

/// Does a piece of furniture stand on this place, where it is now (its footprint, and a hand's breadth round it)?   (orders.ts SquadOrders.onFurniture)
pub fn squad_orders_on_furniture(g: &mut crate::game::Game, p: V3) -> bool {
    if g.orders.fur_in { return false; }
    for i in 0..g.props.props.len() {
        let pr = g.props.props[i];
        let (dx, dz) = (p[0] - crate::props::prop_x(g, pr), p[2] - crate::props::prop_z(g, pr));
        let radius = g.tab.props_prop[pr].radius;
        if m::abs(dx) > radius + FUR_M || m::abs(dz) > radius + FUR_M { continue; }
        let (cs, sn) = (crate::props::prop_cs(g, pr), crate::props::prop_sn(g, pr));
        let (lx, lz) = (cs * dx - sn * dz, sn * dx + cs * dz);   // (the place in the prop's own frame)
        let (hx, hz) = (g.tab.props_prop[pr].hx, g.tab.props_prop[pr].hz);
        if m::hyp2(m::max(0.0, m::abs(lx) - hx), m::max(0.0, m::abs(lz) - hz)) < FUR_M { g.m.doorway.stack_own.fur += 1.0; return true; }
    }
    false
}

/// `k` marks back from a door on face `side` for men its walls have no room for; as many as there are.   (orders.ts SquadOrders.rearPosts)
pub fn squad_orders_rear_posts(g: &mut crate::game::Game, d: DoorId, side: f64, k: f64, taken: &[V3]) -> Vec<crate::orders::StackPost> {
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let c = g.dr[d].frame_centre;
    let jamb_out = g.k.orders.jamb_out;
    let front: V3 = [c[0] + n[0] * jamb_out, 0.45, c[2] + n[1] * jamb_out];
    let sam = squad_orders_sam_place(g);
    let mut out: Vec<StackPost> = Vec::new();
    let h = g.dr[d].def.width / 2.0;
    let clear = !g.orders.fur_in;
    let anys: &[bool] = if clear { &[false, true] } else { &[true] };
    for &any in anys {
        for back in [2.2, 3.1, 4.0] {
            for lat in [h + 1.3, -h - 1.3, h + 0.8, -h - 0.8, h + 1.9, -h - 1.9] {
                if (out.len() as f64) >= k { return out; }   // (never in front of the opening)
                let pos: V3 = [c[0] + n[0] * back + u[0] * lat, 0.0, c[2] + n[1] * back + u[1] * lat];
                if is_blocked(g, pos[0], pos[2]) || crate::scene_collision::solid_blocked(g, front, [pos[0], 0.45, pos[2]], None) || flat(sam, pos) < 0.7 || (!any && squad_orders_on_furniture(g, pos))
                    || taken.iter().chain(out.iter().map(|o| &o.pos)).any(|&q| flat(q, pos) < 0.85) { continue; }
                out.push(StackPost { pos, face: v3::yaw_to(pos, c), stack_side: 1.0, lead: false, group: Some(2.0) });
            }
        }
    }
    out
}

/// The target give() acts on: a door order is carried out from the side of the door the men it goes to are on.   (orders.ts SquadOrders.forMen)
pub fn squad_orders_for_men(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::orders::OrderTarget {
    if let OrderTarget::Door(td) = t {
        if squad_orders_big(g) && !men.is_empty() {
            let side = squad_orders_side_for(g, td.door, men, td.side);
            return OrderTarget::Door(OrderTargetDoor { at: td.at, door: td.door, side });
        }
    }
    t.clone()
}

/// One face's cost for the men: every man's walk to the floor before it.   (orders.ts SquadOrders.sideFor's cost)
fn side_cost(g: &mut Game, d: DoorId, men: &[GuardId], dear: &JsSet<crate::Num>, side: f64) -> f64 {
    let (hg, w, th) = (g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].def.closed_dir);
    let (ux, uz) = (m::cos(th), m::sin(th));
    let f: V3 = [hg[0] + ux * w / 2.0 - uz * side * 0.9, 0.0, hg[1] + uz * w / 2.0 + ux * side * 0.9];
    let to = if is_blocked(g, f[0], f[2]) { squad_orders_free_spot(g, f) } else { Some(f) };
    let to = match to { Some(to) if side_of(g, d, to) == side => to, _ => return f64::INFINITY };
    let mut c = 0.0;
    for &gd in men {
        let p = g.ch[g.gd[gd].ch].pos;
        let from = free_from(g, p);
        if flat(from, to) < 0.5 { continue; }
        let tr = crate::cleared::cleared_rooms_room_at(g, to);
        let cleared_dear = g.k.orders.cleared_dear;
        let also_room = [tr];
        let also: &[f64] = if crate::cleared::cleared_rooms_is_known(g, tr) { &[] } else { &also_room };
        let unk = crate::cleared::cleared_rooms_dear_for(g, from, to, cleared_dear, Some(also));   // (a walk through a room nobody has been in is all but barred as well)
        let path = find_path(g, from, to, Some(if unk.is_some() { ROUND_ITER } else { 6000.0 }), Some(Pen::Walk { dear, wall: 20.0, unk: unk.as_ref() }));
        let path = match path { Some(path) => path, None => { c += 200.0; continue; } };
        if let Some(u) = unk.as_ref() { c += along(g, from, &path, Pen::Dear(u)) * 10.0; }
        let mut a = from;
        for &b in &path {
            c += flat(a, b);
            for i in 0..g.doors.list.len() {
                let o = g.doors.list[i];
                if (o == d || crate::doors::door_latched(g, o)) && crate::guards::crosses_door(g, a, b, o).is_some() { c += if o == d { 60.0 } else { 25.0 }; }
            }
            a = b;
        }
    }
    c
}

/// Which face of the door the men stack on; Sam's on a near tie, and the known face where one is.   (orders.ts SquadOrders.sideFor)
pub fn squad_orders_side_for(g: &mut crate::game::Game, d: DoorId, men: &[GuardId], sam_side: f64) -> f64 {
    let mut dear: JsSet<crate::Num> = JsSet::new();   // this doorway, and every doorway whose leaf is shut
    for i in 0..g.doors.list.len() {
        let o = g.doors.list[i];
        if o != d && !crate::doors::door_latched(g, o) { continue; }
        doorway_cells(g, o, &mut dear);
    }
    let mine = side_cost(g, d, men, &dear, sam_side);
    let other = side_cost(g, d, men, &dear, -sam_side);
    let pick = if other + 1.5 < mine { -sam_side } else { sam_side };
    // (one face on floor the team has been on, the other in a room nobody has: the stack is the known face's)
    let room_a = crate::cleared::cleared_rooms_room_by(g, d, sam_side);
    let a = crate::cleared::cleared_rooms_is_known(g, room_a);
    let room_b = crate::cleared::cleared_rooms_room_by(g, d, -sam_side);
    let b = crate::cleared::cleared_rooms_is_known(g, room_b);
    if a != b && crate::cleared::cleared_rooms_has_floor(g, d, sam_side) && crate::cleared::cleared_rooms_has_floor(g, d, -sam_side) {
        let face = if a { sam_side } else { -sam_side };
        if (if face == sam_side { mine } else { other }).is_finite() { g.orders.face_flip = face != pick; return face; }
    }
    g.orders.face_flip = false;
    pick
}

// ---- holdArcs ----

/// `(b mod 36)` for a bearing index that may be negative
fn bearing(b: i32) -> usize { ((b % 36 + 36) % 36) as usize }
fn arc_score(w: &[f64], b: i32) -> f64 {
    let mut s = 0.0;
    for j in -5..=5i32 { s += w[bearing(b + j)] * (1.0 - 0.06 * m::abs(j as f64)); }
    s
}
/// every dealing of the bearings to the men tried (holdArcs's `rec`)
struct ArcDeal<'a> { at: &'a [V3], men: &'a [usize], c: V3, yaws: &'a [f64], used: Vec<bool>, cur: Vec<usize>, best: Vec<usize>, bd: f64 }
fn arc_rec(x: &mut ArcDeal<'_>, i: usize, s: f64) {
    if i == x.men.len() {
        if s > x.bd { x.bd = s; x.best = x.cur.clone(); }
        return;
    }
    for j in 0..x.yaws.len() {
        if x.used[j] { continue; }
        x.used[j] = true;
        x.cur[i] = j;
        let p = x.at[x.men[i]];
        let next = s + (p[0] - x.c[0]) * m::sin(x.yaws[j]) + (p[2] - x.c[2]) * m::cos(x.yaws[j]);
        arc_rec(x, i + 1, next);
        x.used[j] = false;
    }
}

/// Left holding: which way each man of a group faces, an approach each; a man alone keeps the way he faces (None).   (orders.ts SquadOrders.holdArcs)
pub fn squad_orders_hold_arcs(g: &mut crate::game::Game, at: &[V3]) -> Vec<Option<crate::orders::SquadOrdersHoldArcsItem>> {
    let n = at.len();
    let mut out: Vec<Option<SquadOrdersHoldArcsItem>> = vec![None; n];
    let mut grp: Vec<i32> = vec![-1; n];
    const B: i32 = 36;
    let step = 2.0 * m::PI / (B as f64);
    let hold_group_m = g.k.orders.hold_group_m;
    let mut ng = 0;
    for i in 0..n {
        if grp[i] >= 0 { continue; }
        let mut q = vec![i];
        grp[i] = ng;
        while let Some(a) = q.pop() {
            for j in 0..n { if grp[j] < 0 && flat(at[a], at[j]) < hold_group_m { grp[j] = ng; q.push(j); } }
        }
        ng += 1;
    }
    for k in 0..ng {
        let ii: Vec<usize> = (0..n).filter(|&i| grp[i] == k).collect();
        if ii.len() < 2 { continue; }
        let mut c: V3 = [0.0, 1.5, 0.0];
        for &i in &ii { c[0] += at[i][0] / (ii.len() as f64); c[2] += at[i][2] / (ii.len() as f64); }
        if crate::scene_collision::segment_blocked(g, [at[ii[0]][0], 1.5, at[ii[0]][2]], c, None, None, None) { c[0] = at[ii[0]][0]; c[2] = at[ii[0]][2]; }   // (a wall between them: measured from the first man)
        let mut w = vec![0.0; B as usize];
        let mut seen: Vec<DoorId> = Vec::new();
        for b in 0..B as usize {
            let bf = b as f64;
            let hit = crate::scene_collision::raycast(g, c, [m::sin(bf * step), 0.0, m::cos(bf * step)], 14.0);
            let t = match hit { Some(h) => h.t, None => 14.0 };
            w[b] = if t < 1.6 { 0.0 } else { t };
        }
        for i in 0..g.doors.list.len() {
            let d = g.doors.list[i];
            let fc = g.dr[d].frame_centre;
            let dist = flat(c, fc);
            if dist > 10.0 || dist < 0.3 { continue; }
            let k2 = m::max(0.0, 1.0 - 0.7 / dist);
            if crate::scene_collision::segment_blocked(g, c, [c[0] + (fc[0] - c[0]) * k2, 1.5, c[2] + (fc[2] - c[2]) * k2], None, None, None) { continue; }
            let bb = (js::rem(js::rem(m::round(v3::yaw_to(c, fc) / step), B as f64) + (B as f64), B as f64)) as usize;
            w[bb] += 14.0 * (1.0 - dist / 12.0);
            seen.push(d);
        }
        let mut yaws: Vec<f64> = Vec::new();
        for _ in 0..ii.len() {
            let (mut best, mut bs) = (0i32, -1.0);
            for b in 0..B { let s = arc_score(&w, b); if s > bs { bs = s; best = b; } }
            if bs < 2.0 && !yaws.is_empty() {   // (nothing left to watch: the bearing farthest from the others')
                let (mut fb, mut fd) = (0i32, -1.0);
                for b in 0..B {
                    let gaps: Vec<f64> = yaws.iter().map(|&y| m::abs(v3::wrap_angle((b as f64) * step - y))).collect();
                    let gap = m::min_n(&gaps);
                    if gap > fd { fd = gap; fb = b; }
                }
                yaws.push(v3::wrap_angle((fb as f64) * step));
                continue;
            }
            yaws.push(v3::wrap_angle((best as f64) * step));
            for j in -6..=6i32 { w[bearing(best + j)] = 0.0; }
        }
        // each man the bearing that lies out past his own side of the group (every dealing tried)
        let best_deal: Vec<usize> = if ii.len() <= 6 {
            let mut x = ArcDeal { at, men: &ii, c, yaws: &yaws, used: vec![false; yaws.len()], cur: vec![0; ii.len()], best: Vec::new(), bd: f64::NEG_INFINITY };
            arc_rec(&mut x, 0, 0.0);
            x.best
        } else { (0..ii.len()).collect() };
        let hold_door_m = g.k.orders.hold_door_m;
        let mut mine: JsSet<DoorId> = JsSet::new();
        for (mm, &i) in ii.iter().enumerate() {
            let yaw = yaws[best_deal[mm]];
            let p = at[i];
            let (mut door, mut dd): (Option<DoorId>, f64) = (None, hold_door_m);
            for &d in &seen {
                let fc = g.dr[d].frame_centre;
                let dist = flat(p, fc);
                if dist < dd && !mine.has(d) && m::abs(v3::wrap_angle(v3::yaw_to(p, fc) - yaw)) < 0.96 { dd = dist; door = Some(d); }
            }
            if let Some(d) = door { mine.add(d); }
            let hit = crate::scene_collision::raycast(g, [p[0], 1.5, p[2]], [m::sin(yaw), 0.0, m::cos(yaw)], 8.0);
            out[i] = Some(SquadOrdersHoldArcsItem {
                yaw: match door { Some(d) => v3::yaw_to(p, g.dr[d].frame_centre), None => yaw },
                door,
                reach: m::max(1.5, (match hit { Some(h) => h.t, None => 8.0 }) - 0.5),
            });
        }
    }
    out
}

// ---- route ----

/// what route's closures held of the walk asked about
#[derive(Clone, Copy)]
struct Way<'a> { from: V3, to: V3, why_only: bool, ring: &'a JsSet<String>, unk: Option<&'a crate::cleared::DearPrice> }

/// The first door of a walk locked against him.   (route's `shut`)
fn shut(g: &mut Game, from: V3, ring: &JsSet<String>, path: &[V3]) -> Option<DoorId> {
    let mut a = from;
    for &b in path {
        for i in 0..g.doors.list.len() {
            let d = g.doors.list[i];
            if crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d) && crate::guards::crosses_door(g, a, b, d).is_some() && crate::doors::doors_locked_out(g, d, a, Some(ring)) { return Some(d); }
        }
        a = b;
    }
    None
}
/// (route's `doorsOn`)
fn doors_on(g: &mut Game, from: V3, q: &[V3]) -> Vec<DoorId> {
    let mut out = Vec::new();
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if through(g, from, q, d) { out.push(d); }
    }
    out
}
/// Nor down the lane of a door standing open onto unknown floor, where there is a way round its edge.   (route's `laneOnly`)
fn lane_only(g: &mut Game, w: Way<'_>, base: &[V3], pen: Option<Pen<'_>>) -> Option<Vec<V3>> {
    if w.why_only { return None; }   // (a menu asking whether he can get there: a lane never changes that, and its search is the dear one)
    let mine = doors_on(g, w.from, base);
    let mut lanes = crate::doorway::doorway_lane_price(g, Some(&mut |_g: &mut Game, d: DoorId| mine.contains(&d)))?;
    let lane_m = along_lanes(g, w.from, base, &mut lanes);
    if lane_m < 1.0 { return None; }
    let way = crate::scene_collision::nav_grid_find_path(g, w.from, w.to, Some(ROUND_ITER), Some(&mut |g: &mut Game, x: f64, z: f64| match pen {
        Some(p) => { let a = price(g, p, x, z); a * seam_lane_price(g, &mut lanes, x, z) }
        None => seam_lane_price(g, &mut lanes, x, z),
    }))?;
    if shut(g, w.from, w.ring, &way).is_some() || doors_on(g, w.from, &way).iter().any(|d| !mine.contains(d)) || along_lanes(g, w.from, &way, &mut lanes) > lane_m - 1.0
        || len_from(w.from, &way) > len_from(w.from, base) * g.k.doormanners.lane_long + 1.0 { return None; }
    if let Some(u) = w.unk {
        let (a, b) = (along(g, w.from, &way, Pen::Dear(u)), along(g, w.from, base, Pen::Dear(u)));
        if a > m::max(OFF_M, b) { return None; }
    }
    Some(way)
}
/// Nor over lit floor a room nobody has been in has a sight line to.   (route's `seenWay`)
fn seen_way(g: &mut Game, w: Way<'_>, now: &[V3], base: &[V3], pen: Option<Pen<'_>>) -> Option<Vec<V3>> {
    let seen = crate::seenwalk::seen_price(g, w.from, now)?;
    let m_s = along_seen(g, w.from, now, &seen);
    let (less, long) = (g.k.seenwalk.less, g.k.seenwalk.long);
    if m_s < less { return None; }
    g.stats.seenwalk.in_view += 1;
    let mine = doors_on(g, w.from, base);
    let mut lanes = crate::doorway::doorway_lane_price(g, Some(&mut |_g: &mut Game, d: DoorId| mine.contains(&d)));
    let way = crate::scene_collision::nav_grid_find_path(g, w.from, w.to, Some(ROUND_ITER), Some(&mut |g: &mut Game, x: f64, z: f64| {
        let a = match pen { Some(p) => Some(price(g, p, x, z)), None => None };
        let both = match lanes.as_mut() {
            Some(l) => { let v = seam_lane_price(g, l, x, z); m::max(v, seam_seen_price(g, &seen, x, z)) }
            None => seam_seen_price(g, &seen, x, z),
        };
        match a { Some(a) => a * both, None => both }
    }))?;
    if shut(g, w.from, w.ring, &way).is_some() || doors_on(g, w.from, &way).iter().any(|d| !mine.contains(d)) || along_seen(g, w.from, &way, &seen) > m_s - less { return None; }
    if let Some(l) = lanes.as_mut() {
        let a = along_lanes(g, w.from, &way, l);
        let b = along_lanes(g, w.from, now, l);
        if a > b + 0.5 { return None; }
    }
    if len_from(w.from, &way) > len_from(w.from, now) * long + 1.0 { return None; }
    if let Some(u) = w.unk {
        let (a, b) = (along(g, w.from, &way, Pen::Dear(u)), along(g, w.from, now, Pen::Dear(u)));
        if a > m::max(OFF_M, b) { return None; }
    }
    g.stats.seenwalk.round += 1;
    Some(way)
}
/// The way the lanes leave him, then asked of the seen floor.   (route's `laneWay`)
fn lane_way(g: &mut Game, w: Way<'_>, base: &[V3], pen: Option<Pen<'_>>) -> Option<Vec<V3>> {
    let via_lane = lane_only(g, w, base, pen);
    if w.why_only { return via_lane; }
    let now: &[V3] = match via_lane.as_ref() { Some(v) => v, None => base };
    match seen_way(g, w, now, base, pen) { Some(v) => Some(v), None => via_lane }
}
fn route_none() -> SquadOrdersRoute { SquadOrdersRoute { why: None, via: Vec::new(), how: None, lock: None } }

/// How he gets there: `why` he cannot, or `via`, waypoints round a door locked against Sam, a room nobody has been in, a lane, a lit floor.   (orders.ts SquadOrders.route)
pub fn squad_orders_route(g: &mut crate::game::Game, gd: GuardId, to: V3, shy: Option<DoorId>, thru: Option<DoorId>, why_only: Option<bool>) -> crate::orders::SquadOrdersRoute {
    let why_only = why_only.unwrap_or(false);
    let at = g.ch[g.gd[gd].ch].pos;
    if flat(at, to) < 0.5 { return route_none(); }
    let from = free_from(g, at);
    let ring = squad_orders_ring(g);
    let path = match crate::scene_collision::nav_grid_find_path(g, from, to, None, None) {
        Some(p) => p,
        None => return SquadOrdersRoute { why: Some(String::from("can't get there.")), ..route_none() },
    };
    if let Some(thru) = thru {
        if let Some(mm) = squad_orders_by_door(g, from, to, thru, &path) {
            if shut(g, from, &ring, &mm).is_none() { return SquadOrdersRoute { via: mm, how: Some(SquadOrdersRouteHow::Mouth), ..route_none() }; }   // (never at a door locked against him)
        }
    }
    let own = match shy { Some(s) => through(g, from, &path, s), None => false };
    let round = own && squad_orders_big(g);   // (the short way to his mark runs through the very door he is to stack on)
    // (cleared.ts: the short way runs through a room nobody of the team has been in: `off_m` metres of it)
    let cleared_dear = g.k.orders.cleared_dear;
    let unk = crate::cleared::cleared_rooms_dear_for(g, from, to, cleared_dear, None);
    let off_m = match unk.as_ref() { Some(u) => along(g, from, &path, Pen::Dear(u)), None => 0.0 };
    let off = off_m > OFF_M;
    let w = Way { from, to, why_only, ring: &ring, unk: unk.as_ref() };
    if shut(g, from, &ring, &path).is_none() && !round && !off {
        let way = lane_way(g, w, &path, unk.as_ref().map(Pen::Dear));
        return SquadOrdersRoute { via: match way { Some(way) => all_but_last(&way), None => Vec::new() }, ..route_none() };
    }
    let mut dear: JsSet<crate::Num> = JsSet::new();   // every doorway locked against him (and the one he is shy of), made all but a wall
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if !(round && Some(d) == shy) && (!crate::doors::door_locked(g, d) || !crate::doors::door_latched(g, d) || ring.has(g.dr[d].lock_id.clone())) { continue; }
        doorway_cells(g, d, &mut dear);
    }
    let plain = path;
    let wall = if unk.is_some() { 1000.0 } else { 20.0 };   // (a doorway locked against him stays the dearer: he is never sent at a lock to keep him out of a room)
    let pen = Pen::Walk { dear: &dear, wall, unk: unk.as_ref() };
    let found = find_path(g, from, to, Some(if unk.is_some() { ROUND_ITER } else { 6000.0 }), Some(pen));
    let shut_plain = |g: &mut Game| shut(g, from, &ring, &plain).is_some();
    if round {
        let through_it = match found.as_ref() { None => true, Some(p) => through(g, from, p, shy.unwrap()) };
        if through_it && !shut_plain(g) && (!off || found.is_none()) { return route_none(); }   // (no way round it: through it, as ever)
    }
    if found.is_none() && off && !shut_plain(g) { return route_none(); }   // (the search gave out: the short way, as it was)
    let still = match found.as_ref() { Some(p) => shut(g, from, &ring, p), None => None };
    if still.is_some() && off && !shut_plain(g) { return route_none(); }   // (an order is never refused for a room)
    let path = match (found, still) {
        (Some(p), None) => p,
        (found, still) => {
            let mut lock = None;   // (the door that refusal is about and the face the way comes to it by: walkTry asks what the team knows of it)
            if let (Some(p), Some(s)) = (found.as_ref(), still) {
                let mut a = from;
                for &b in p { if crate::guards::crosses_door(g, a, b, s).is_some() { break; } a = b; }
                lock = Some(SquadOrdersRouteLock { door: s, side: side_of(g, s, a) });
            }
            let why = match still {
                Some(s) => { let words = door_words(g, s); let name = if words == "door" { String::from("that door") } else { words }; format!("{} is locked.", name) }
                None => String::from("can't get there."),
            };
            return SquadOrdersRoute { why: Some(why), lock, ..route_none() };
        }
    };
    let mut how = None;
    if off {
        let left = along(g, from, &path, Pen::Dear(unk.as_ref().unwrap()));
        if left > OFF_M {
            how = Some(SquadOrdersRouteHow::Through);
            if left > off_m - OFF_M && !shut_plain(g) && !round { return SquadOrdersRoute { how, ..route_none() }; }   // (no way round that room: the short way through it)
        } else {
            how = Some(SquadOrdersRouteHow::Round);
        }
        // (a way round far longer than the short way is not worth it, nor one still through unknown floor unless it crosses clearly less of it)
        let round_cap = g.k.orders.round_cap;
        if round_cap > 0.0 && !shut_plain(g) && !round && !(own && !through(g, from, &path, shy.unwrap()))
            && ((left > OFF_M && left > off_m * ROUND_LESS) || len_from(from, &path) > len_from(from, &plain) * round_cap + ROUND_PLUS_M) {
            return SquadOrdersRoute { how: Some(SquadOrdersRouteHow::Through), ..route_none() };
        }
    }
    let path = match lane_way(g, w, &path, Some(pen)) { Some(p) => p, None => path };   // (the way round a room or a lock, and off the lanes on it where that costs it nothing of either)
    SquadOrdersRoute { via: all_but_last(&path), how, ..route_none() }
}

// ---- sameWays ----

fn pair_of(g: &Game, gd: GuardId) -> Option<crate::game::PairId> { g.gd[gd].unit.as_ref().and_then(|u| u.pair) }
/// (sameWays's `of`)
fn sw_of(g: &Game, gd: GuardId, t: f64) -> Option<OrdersAssignmentId> {
    let id = *g.orders.asg.get(gd)?;
    let a = &g.tab.orders_assignment[id];
    if a.group == g.orders.group && a.t0 == t && a.post.is_some() && a.thru.is_none() && !js::truthy(a.follow) && g.ch[g.gd[gd].ch].alive { Some(id) } else { None }
}
/// (sameWays's `same`)
fn sw_same(g: &Game, a: OrdersAssignmentId, b: OrdersAssignmentId) -> bool {
    let (a, b) = (&g.tab.orders_assignment[a], &g.tab.orders_assignment[b]);
    a.order == b.order && a.door == b.door
}
/// (sameWays's `paired`)
fn sw_paired(g: &Game, men: &[GuardId], gd: GuardId, a: OrdersAssignmentId, t: f64) -> bool {
    let pair = pair_of(g, gd);
    pair.is_some() && men.iter().any(|&m| {
        if m == gd || pair_of(g, m) != pair { return false; }
        match sw_of(g, m, t) { Some(b) => sw_same(g, a, b), None => false }
    })
}
/// (sameWays's `wayOf`)
fn way_of(g: &mut Game, s: V3, mid: &[V3], to: V3) -> Option<Vec<V3>> {
    let mut w = vec![s];
    let mut q = s;
    for &v in mid.iter().chain(core::iter::once(&to)) {
        if flat(q, v) < 0.05 { continue; }
        let leg = crate::scene_collision::nav_grid_find_path(g, q, v, None, None)?;
        w.extend(leg);
        q = v;
    }
    Some(w)
}
/// `w.slice(1, -1)` on the floor (sameWays's `corners`)
fn corners(w: &[V3]) -> Vec<V3> {
    if w.len() <= 2 { return Vec::new(); }
    w[1..w.len() - 1].iter().map(|&q| on_floor(q)).collect()
}
/// B by A's way: A's own corners and then his own mark: the walk and what it costs him over his own, or None.   (sameWays's `join`)
fn sw_join(g: &mut Game, ring: &JsSet<String>, b_id: OrdersAssignmentId, mine: &[V3], his: &[V3]) -> Option<(Vec<V3>, f64)> {
    let from = mine[0];
    let post = g.tab.orders_assignment[b_id].post.unwrap();
    let w = way_of(g, from, &corners(his), post)?;
    if far_off(&w, his).d > g.k.orders.same_apart_m || len_of(&w) > len_of(mine) * g.k.orders.same_long + SAME_PLUS_M { return None; }
    let b_shy = g.tab.orders_assignment[b_id].shy;
    let mut a = from;
    for &b in &w {
        for i in 0..g.doors.list.len() {
            let d = g.doors.list[i];
            if crate::guards::crosses_door(g, a, b, d).is_some() && (Some(d) == b_shy || (crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d) && crate::doors::doors_locked_out(g, d, a, Some(ring)))) { return None; }
        }
        a = b;
    }
    let cleared_dear = g.k.orders.cleared_dear;
    if let Some(u) = crate::cleared::cleared_rooms_dear_for(g, from, post, cleared_dear, None).as_ref() {
        let (x, y) = (along(g, from, &w, Pen::Dear(u)), along(g, from, mine, Pen::Dear(u)));
        if x > m::max(OFF_M, y) { return None; }
    }
    let more = len_of(&w) - len_of(mine);
    Some((w, more))
}
/// (sameWays's `take`)
fn sw_take(g: &mut Game, b: OrdersAssignmentId, w: &[V3]) {
    let via = corners(w);
    let row = &mut g.tab.orders_assignment[b];
    row.goal = if !via.is_empty() { via[0] } else { v3::copy(row.post.unwrap()) };
    row.via = via;
}

/// A pair goes round one way: at the end of a giving, men of one order and door nearly together who would split round a building take one of their two ways.   (orders.ts SquadOrders.sameWays)
pub fn squad_orders_same_ways(g: &mut crate::game::Game, men: &[GuardId]) {
    let t = g.time;
    let ring = squad_orders_ring(g);
    let mut led: JsSet<OrdersAssignmentId> = JsSet::new();   // (the men whose way somebody has taken, or who took one: theirs stands from then on)
    for j in 1..men.len() {
        let gd = men[j];
        let b = match sw_of(g, gd, t) { Some(b) => b, None => continue };
        let mine2 = sw_paired(g, men, gd, b, t);
        let mut a_of = None;
        for i in 0..j {
            if a_of.is_some() { break; }
            let mm = men[i];
            if let Some(x) = sw_of(g, mm, t) {
                if sw_same(g, x, b) && (if mine2 { pair_of(g, mm) == pair_of(g, gd) } else { !sw_paired(g, men, mm, x, t) }) { a_of = Some(x); }
            }
        }
        let a = match a_of { Some(a) => a, None => continue };
        let (near, apart) = (g.k.orders.same_near_m, g.k.orders.same_apart_m);
        let (b_row_anchor, b_post, b_via) = { let r = &g.tab.orders_assignment[b]; (r.anchor, r.post.unwrap(), r.via.clone()) };
        let (a_row_anchor, a_post, a_via) = { let r = &g.tab.orders_assignment[a]; (r.anchor, r.post.unwrap(), r.via.clone()) };
        if flat(b_row_anchor, a_row_anchor) > near || flat(b_post, a_post) > near { continue; }
        let fb = free_from(g, b_row_anchor);
        let mine = way_of(g, fb, &b_via, b_post);
        let fa = free_from(g, a_row_anchor);
        let his = way_of(g, fa, &a_via, a_post);
        let (mine, his) = match (mine, his) { (Some(x), Some(y)) => (x, y), _ => continue };
        if m::max(far_off(&his, &mine).d, far_off(&mine, &his).d) <= apart { continue; }   // (one way already)
        // (either man's way will do: the one that costs the other the fewer metres more)
        let jb = sw_join(g, &ring, b, &mine, &his);
        let ja = if led.has(a) { None } else { sw_join(g, &ring, a, &his, &mine) };
        let b_takes = match (&jb, &ja) { (Some(x), Some(y)) => x.1 <= y.1 + 0.5, (Some(_), None) => true, _ => false };
        if b_takes {
            sw_take(g, b, &jb.as_ref().unwrap().0);
            if g.tab.orders_assignment[a].via.is_empty() { sw_take(g, a, &his); }   // (the man led walks the corners the two were measured by)
        } else if let Some(y) = ja.as_ref() {
            sw_take(g, a, &y.0);
            if g.tab.orders_assignment[b].via.is_empty() { sw_take(g, b, &mine); }
        } else { continue; }
        led.add(a);
        led.add(b);
    }
}

/// The ordered door: the way to a point inside for a man an entry sends in by `d`, by the door's mouth on the face away from the point; None when the short way is right.   (orders.ts SquadOrders.byDoor)
pub fn squad_orders_by_door(g: &mut crate::game::Game, from: V3, to: V3, d: DoorId, plain: &[V3]) -> Option<Vec<V3>> {
    let room = crate::cleared::cleared_rooms_room_at(g, to);
    let here = crate::cleared::cleared_rooms_room_at(g, from);
    let same_side = if js::truthy(room) { here == room } else { side_of(g, d, from) == side_of(g, d, to) };
    if same_side { return None; }
    // (a man standing in a doorway of that room, the door he was sent to hold, is in it)
    if js::truthy(room) && !js::truthy(here) {
        for i in 0..g.doors.list.len() {
            let o = g.doors.list[i];
            if flat(g.dr[o].frame_centre, from) < g.dr[o].def.width / 2.0 + 0.3 && m::abs(off_line(g, o, from)) < 0.45
                && (crate::cleared::cleared_rooms_room_by(g, o, 1.0) == room || crate::cleared::cleared_rooms_room_by(g, o, -1.0) == room) { return None; }
        }
    }
    let cleared_dear = g.k.orders.cleared_dear;
    let unk = crate::cleared::cleared_rooms_dear_for(g, from, to, cleared_dear, None);
    if through(g, from, plain, d) {
        let far = match unk.as_ref() { Some(u) => along(g, from, plain, Pen::Dear(u)) > OFF_M, None => false };
        if !far { return None; }
    }
    let side = side_of(g, d, to);
    let (hg, th, w) = (g.dr[d].def.hinge, g.dr[d].def.closed_dir, g.dr[d].def.width);
    let mouth = |off: f64| -> V3 { [hg[0] + m::cos(th) * w / 2.0 - m::sin(th) * off, 0.0, hg[1] + m::sin(th) * w / 2.0 + m::cos(th) * off] };
    let out0 = mouth(-side * MOUTH_M);
    let out = if is_blocked(g, out0[0], out0[2]) { squad_orders_free_spot(g, out0) } else { Some(out0) };
    let inn = mouth(side * MOUTH_M);
    let out = match out { Some(o) if side_of(g, d, o) != side => o, _ => return None };
    // (the walk to the mouth: the room the entry is for is dear whatever is known of it; so is every room nobody has been in)
    let also_room = [room];
    let also: &[f64] = if js::truthy(room) { &also_room } else { &[] };
    let pen = crate::cleared::cleared_rooms_dear_for(g, from, out, cleared_dear, Some(also));
    let way = find_path(g, from, out, Some(ROUND_ITER), pen.as_ref().map(Pen::Dear))?;
    if through(g, from, &way, d) { return None; }   // (no way to its mouth but out through the door itself: he is as good as inside)
    let mut via: Vec<V3> = way.iter().map(|&q| on_floor(q)).collect();
    if flat(via[via.len() - 1], out) > 0.3 { via.push(out); }
    if !is_blocked(g, inn[0], inn[2]) { via.push(inn); }
    Some(via)
}

/// Fall in: the corners of the way to his place in the file that keeps out of rooms nobody has been in; [] when the short way does, or there is no such way.   (orders.ts SquadOrders.roundRooms)
pub fn squad_orders_round_rooms(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Vec<V3> {
    let at = g.ch[g.gd[gd].ch].pos;
    let from = free_from(g, at);
    let cleared_dear = g.k.orders.cleared_dear;
    let unk = match crate::cleared::cleared_rooms_dear_for(g, from, to, cleared_dear, None) { Some(u) => u, None => return Vec::new() };
    let plain = match crate::scene_collision::nav_grid_find_path(g, from, to, None, None) { Some(p) => p, None => return Vec::new() };
    let off_m = along(g, from, &plain, Pen::Dear(&unk));
    if off_m <= OFF_M { return Vec::new(); }
    let path = match find_path(g, from, to, Some(ROUND_ITER), Some(Pen::Dear(&unk))) { Some(p) => p, None => return Vec::new() };
    if along(g, from, &path, Pen::Dear(&unk)) > OFF_M { return Vec::new(); }
    let round_cap = g.k.orders.round_cap;
    if round_cap > 0.0 && len_from(from, &path) > len_from(from, &plain) * round_cap + ROUND_PLUS_M { return Vec::new(); }   // (nor a way round far longer than the short way)
    all_but_last(&path)
}

/// The crumb of the trail that a place `back` metres behind the player lies just past: the index of the older end of its stretch; -1 off the trail's end.   (orders.ts SquadOrders.slotCrumb)
pub fn squad_orders_slot_crumb(g: &mut crate::game::Game, back: f64) -> f64 {
    let sam = g.ch[g.pl[g.player].ch].pos;
    let t = &g.orders.trail;
    let (mut prev, mut left): (V3, f64) = ([sam[0], 0.0, sam[2]], back);
    for i in (0..t.len()).rev() {
        let seg = flat(prev, t[i]);
        if seg < 1e-4 { continue; }
        if seg >= left { return i as f64; }
        left -= seg;
        prev = t[i];
    }
    -1.0
}

/// The doors a walk ends on the far side of (through one and back is no door; a corner on a door's line is stepped over).   (trailWay's `doorsOn`)
fn trail_doors_on(g: &mut Game, w: &[V3]) -> JsSet<DoorId> {
    let mut out: JsSet<DoorId> = JsSet::new();
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        let (h, th) = (g.dr[d].def.hinge, g.dr[d].def.closed_dir);
        let (nx, nz) = (-m::sin(th), m::cos(th));
        let mut a: Option<V3> = None;
        for &b in w {
            if m::abs((b[0] - h[0]) * nx + (b[2] - h[1]) * nz) < 1e-6 { continue; }
            if let Some(a) = a {
                if crate::guards::crosses_door(g, a, b, d).is_some() && !out.delete(d) { out.add(d); }
            }
            a = Some(b);
        }
    }
    out
}

/// A follower goes by the doors Sam took: the trail's corners where his own short way to his place ends through other doors; [] otherwise.   (orders.ts SquadOrders.trailWay)
pub fn squad_orders_trail_way(g: &mut crate::game::Game, gd: GuardId, slot: V3, on_trail: V3, j: f64) -> Vec<V3> {
    let t = g.orders.trail.clone();
    if t.len() < 4 { return Vec::new(); }
    let at = g.ch[g.gd[gd].ch].pos;
    let from = free_from(g, at);
    if crate::scene_collision::nav_grid_walkable(g, from, slot) { return Vec::new(); }   // (his place is a straight walk off: that is his way)
    let j = j as i64;
    let reach_m = g.k.orders.trail_reach_m;
    let mut k: i64 = -1;
    let mut i = j - 1;
    while i >= 0 {   // (the newest crumb he can step straight onto)
        if flat(from, t[i as usize]) < reach_m && crate::scene_collision::nav_grid_walkable(g, from, t[i as usize]) { k = i; break; }
        i -= 1;
    }
    if j < 0 || k < 0 || j - k < 3 { return Vec::new(); }
    let own = match crate::scene_collision::nav_grid_find_path(g, from, slot, None, None) { Some(o) => o, None => return Vec::new() };
    let mut way: Vec<V3> = vec![from];
    way.extend_from_slice(&t[k as usize..core::cmp::min(j as usize + 1, t.len())]);
    way.push(on_trail);
    way.push(slot);
    let mut mine: Vec<V3> = vec![from];   // (a path ends on a cell's centre, which by a leaf can be the far side of a frame from the place itself)
    mine.extend(own);
    mine.push(slot);
    let (a, b) = (trail_doors_on(g, &mine), trail_doors_on(g, &way));
    if a.len() == b.len() && a.iter().all(|d| b.has(d)) { return Vec::new(); }   // (the same doors: his own way is the trail's)
    if len_of(&way) > len_of(&mine) * g.k.orders.trail_long + SAME_PLUS_M { return Vec::new(); }
    let ring = squad_orders_ring(g);
    for i in 1..way.len() {
        for n in 0..g.doors.list.len() {
            let d = g.doors.list[n];
            if crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d) && crate::guards::crosses_door(g, way[i - 1], way[i], d).is_some() && crate::doors::doors_locked_out(g, d, way[i - 1], Some(&ring)) { return Vec::new(); }
        }
    }
    let mut mid: Vec<V3> = Vec::new();   // (the trail's corners: a crumb is kept where the next cannot be walked to straight, and every few metres)
    let mut a = from;
    let mut i = k;
    while i < j - 1 {
        let (now, next) = (t[i as usize], t[i as usize + 1]);
        if !crate::scene_collision::nav_grid_walkable(g, a, next) || flat(a, next) > 3.0 { mid.push(on_floor(now)); a = now; }
        i += 1;
    }
    mid
}

/// The word he says why he cannot get there, or None.   (orders.ts SquadOrders.reach)
pub fn squad_orders_reach(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<String> {
    squad_orders_asked(g, gd, to).why
}

/// The door his walk to `to` would be refused for, where nobody of the team has tried it on the face he comes to it by. Sam's squad only.   (orders.ts SquadOrders.walkTry)
pub fn squad_orders_walk_try(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::orders::SquadOrdersWalkTry> {
    if g.orders.cast.is_some() { return None; }
    let l = squad_orders_asked(g, gd, to).lock?;
    if crate::doortools::door_tools_untried(g, l.door, l.side) { Some(SquadOrdersWalkTry { door: l.door, side: l.side }) } else { None }
}

/// The men of a 'move' whose way runs at such a door, each dealt a stack mark on the face he comes to it by; Text: that door has no room for them.   (orders.ts SquadOrders.walkStacks)
pub fn squad_orders_walk_stacks(g: &mut crate::game::Game, men: &[GuardId], to: V3) -> Option<crate::orders::SquadOrdersWalkStacks> {
    if g.orders.cast.is_some() { return None; }
    let mut out: Option<JsMap<GuardId, SquadOrdersWalkStacksMapValue>> = None;
    let mut at: Vec<(DoorId, f64, Vec<GuardId>)> = Vec::new();
    for &gd in men {
        let l = match squad_orders_walk_try(g, gd, to) { Some(l) => l, None => continue };
        match at.iter_mut().find(|x| x.0 == l.door && x.1 == l.side) {
            Some(k) => k.2.push(gd),
            None => at.push((l.door, l.side, vec![gd])),
        }
    }
    for (door, side, men) in &at {
        let (door, side) = (*door, *side);
        let posts = match squad_orders_stack_posts_wait(g, door, side, men) {
            Some(p) => p,
            None => return Some(SquadOrdersWalkStacks::Text(squad_orders_no_room_words(g, "no room on that door.", &mut |g: &mut Game, _: &mut Game| squad_orders_stack_posts_wait(g, door, side, men).is_some()))),
        };
        let spots: Vec<V3> = posts.iter().map(|x| x.pos).collect();
        let groups: Vec<f64> = posts.iter().map(|x| x.group.unwrap_or(0.0)).collect();
        let fc = g.dr[door].frame_centre;
        for (gd, p) in squad_orders_pair(g, men, &spots, Some(&groups), Some(fc)) {
            let post = *posts.iter().find(|x| x.pos == p).unwrap();   // (the TypeScript finds the same array by identity: the marks are a body apart, so the place names it)
            out.get_or_insert_with(JsMap::new).set(gd, SquadOrdersWalkStacksMapValue { door, side, post });
        }
    }
    out.map(SquadOrdersWalkStacks::Map)
}

/// the all-optional order as the three struct types the skeleton made of the one TypeScript shape
macro_rules! order_as {
    ($to:ident, $s:expr, $tries:expr) => {{
        let s = $s;
        $to {
            post: s.post, going: s.going.clone(), order: s.order.clone(), face: s.face, there: s.there.clone(), t0: s.t0, follow: s.follow, watch: s.watch, door: s.door,
            strafe: s.strafe, stance: s.stance, upper: s.upper, stack_side: s.stack_side, move_: s.move_.clone(), set_line: s.set_line.clone(), group: s.group, set_said: s.set_said,
            goal: s.goal, arrived: s.arrived, held_set: s.held_set, held_fire: s.held_fire, arr_t: s.arr_t, moved_t: s.moved_t, light: s.light, low: s.low, yield_t: s.yield_t,
            face_now: s.face_now, search: s.search, search_t: s.search_t, via: s.via.clone(), anchor: s.anchor, mark: s.mark, mark_t: s.mark_t, had: s.had, back_n: s.back_n,
            shy: s.shy, thru: s.thru, via_for: s.via_for, via_t: s.via_t, pace: s.pace, spare: s.spare, from: s.from, foe: s.foe, crouch: s.crouch, lean: s.lean, fit: s.fit.clone(),
            tries: $tries(s.tries.clone()),
        }
    }};
}
type TriesOn = crate::doortools::DoorToolsDoneItemLaneBackTriesOn;
type Tries = crate::doortools::DoorToolsDoneItemLaneBackTries;
fn on_to_assign(s: &TriesOn) -> SquadOrdersAssignA { order_as!(SquadOrdersAssignA, s, |t: Option<Box<Tries>>| t.map(|b| *b)) }
fn on_to_send(s: &TriesOn) -> SquadOrdersWalkSendOn { order_as!(SquadOrdersWalkSendOn, s, |t: Option<Box<Tries>>| t.map(|b| *b)) }
fn send_to_on(s: &SquadOrdersWalkSendOn) -> TriesOn { order_as!(TriesOn, s, |t: Option<Tries>| t.map(Box::new)) }

/// A man of them sent: to his stack mark on that door, in the stack's carriage, still on his 'move'.   (orders.ts SquadOrders.walkSend)
pub fn squad_orders_walk_send(g: &mut crate::game::Game, gd: GuardId, t: crate::orders::SquadOrdersWalkSendT, at: V3, on: &crate::orders::SquadOrdersWalkSendOn) {
    let fc = g.dr[t.door].frame_centre;
    let a = SquadOrdersAssignA {
        order: String::from("move"),
        post: Some(t.post.pos),
        face: Some(t.post.face),
        watch: Some([fc[0], 1.05, fc[2]]),
        door: Some(t.door),
        shy: Some(t.door),
        stance: Some(if t.post.lead { crate::character::Stance::Stack } else { crate::character::Stance::HighReady }),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        stack_side: Some(t.post.stack_side),
        going: String::from("moving"),
        there: String::from("moving"),
        tries: Some(Tries { door: t.door, side: t.side, post: t.post, at: v3::copy(at), on: send_to_on(on) }),
        ..SquadOrdersAssignA::zeroed()
    };
    squad_orders_assign(g, gd, &a);
}

/// `this.asg.get(m)?.tries`, a copy
fn tries_of(g: &Game, m: GuardId) -> Option<Tries> {
    g.orders.asg.get(m).and_then(|&a| g.tab.orders_assignment[a].tries.clone())
}
/// (walkOn's `stop`)
fn walk_stop(g: &mut Game, mm: GuardId) {
    let p = g.ch[g.gd[mm].ch].pos;
    let on = tries_of(g, mm).unwrap().on;
    let a = SquadOrdersAssignA { post: Some([p[0], 0.0, p[2]]), there: String::from("stopped short"), set_line: None, fit: None, ..on_to_assign(&on) };
    squad_orders_assign(g, mm, &a);
}

/// The door they came to turned out free, or stands open: the walk is asked again for each of them, as the order would be asked fresh.   (orders.ts SquadOrders.walkOn)
pub fn squad_orders_walk_on(g: &mut crate::game::Game, men: &[GuardId]) {
    let all: Vec<GuardId> = men.iter().copied().filter(|&mm| tries_of(g, mm).is_some()).collect();
    if all.is_empty() { return; }
    let first = tries_of(g, all[0]).unwrap().at;
    let held: Vec<GuardId> = all.iter().copied().filter(|&mm| flat(tries_of(g, mm).unwrap().at, first) < 1e-6).collect();   // (the men of one order; another order's walkers on the same door: asked after them)
    if held.len() < all.len() {
        let rest: Vec<GuardId> = all.iter().copied().filter(|mm| !held.contains(mm)).collect();
        squad_orders_walk_on(g, &rest);
    }
    squad_orders_new_group(g);   // (a group of their own from here, each order's men theirs)
    let at = tries_of(g, held[0]).unwrap().at;
    let next = squad_orders_walk_stacks(g, &held, at);
    if let Some(SquadOrdersWalkStacks::Text(words)) = &next {
        let i = js::rem(g.orders.acks, held.len() as f64) as usize;
        g.orders.acks += 1.0;
        crate::game::game_say(g, held[i], words, None, None, None);
        for &mm in &held { walk_stop(g, mm); }
        return;
    }
    let mut said = false;
    for &mm in &held {
        let t = tries_of(g, mm).unwrap();
        let n = match &next { Some(SquadOrdersWalkStacks::Map(map)) => map.get(mm).copied(), _ => None };
        let why = if n.is_some() { None } else { squad_orders_reach(g, mm, at) };
        if let Some(n) = n {
            squad_orders_walk_send(g, mm, SquadOrdersWalkSendT { door: n.door, side: n.side, post: n.post }, at, &on_to_send(&t.on));
        } else if let Some(why) = why {
            if !said { crate::game::game_say(g, mm, &why, None, None, None); }
            said = true;
            walk_stop(g, mm);
        } else {
            squad_orders_assign(g, mm, &on_to_assign(&t.on));
        }
    }
    if held.len() > 1 { squad_orders_same_ways(g, &held); }   // (the walk's second leg is a giving of its own: a pair goes on by one way)
}
