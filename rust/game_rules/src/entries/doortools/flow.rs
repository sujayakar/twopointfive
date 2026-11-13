use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap};
use crate::m;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec, vec::Vec};

use super::flow_in::door_tools_flow_in;
use crate::v3::dist_xz;

// ---- small reads

/// a point as this file deals one: `{ pos, face }`
pub(super) fn pod_at(pos: V3, face: f64) -> crate::entrypoints::Pod { crate::entrypoints::Pod { pos, face, watch: None } }

/// where his order has him, or where he stands
pub(super) fn where_of(g: &Game, m: GuardId) -> V3 {
    let post = crate::orders::squad_orders_of(&crate::seats::here(g), m).and_then(|a| g.tab.task[a].post);
    post.unwrap_or_else(|| g.pos_of(m))
}
/// where every order has `m` going, in the orders' order
pub(super) fn pods_of(g: &Game, seat: &crate::seats::SeatRef, m: GuardId, out: &mut Vec<V3>) {
    for &o in seat.door_tools.ops.iter() {
        if let Some(q) = g.tab.doortools_door_op[o].pod.get(m) { out.push(q.pos); }
    }
}

/// has `gd` a line from where he stands to cell `i`, asked once per cell per place he stands
fn door_tools_line_to(g: &mut crate::game::Game, sr: DoortoolsRoomSearchId, gd: GuardId, i: f64) -> bool {
    let i = i as usize;
    let c = g.gd[gd].ch;
    let leans = g.gd[gd].script.as_ref().and_then(|s| s.lean).map_or(false, js::truthy);
    let hd: Option<V3> = if leans { g.ch[c].bone(crate::character::Bone::Head) } else { None };
    let p = hd.unwrap_or(g.ch[c].pos);
    let s = &mut g.tab.doortools_room_search[sr];
    let stale = match s.los.get(gd) {
        None => true,
        Some(l) => dist_xz(l.at, p) > 0.35 || match hd { Some(h) => m::abs(l.at[1] - h[1]) > 0.2, None => l.at[1] != p[1] },
    };
    if stale {
        let n = s.pts.len();
        s.los.set(gd, RoomSearchLosValue { at: v3::copy(p), ok: vec![-1; n] });
    }
    let q = s.pts[i];
    let (at, known) = { let l = s.los.get(gd).unwrap(); (l.at, l.ok[i]) };
    if known < 0 {
        let blocked = crate::scene_collision::segment_blocked(g, [at[0], if hd.is_some() { at[1] } else { 1.55 }, at[2]], q, None, None, None);
        g.tab.doortools_room_search[sr].los.get_mut(gd).unwrap().ok[i] = if blocked { 0 } else { 1 };
    }
    g.tab.doortools_room_search[sr].los.get(gd).unwrap().ok[i] == 1
}

/// every cell a man's muzzle is on this frame is looked at
pub(super) fn door_tools_look(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, out: Option<&[GuardId]>, ok: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId, f64) -> bool>) {
    let mut ok = ok;
    let outside = out.is_some();
    let men: Vec<GuardId> = match out { Some(l) => l.to_vec(), None => g.tab.doortools_door_op[op].men.clone() };
    let (look_m, look_rad) = (g.k.doortools.look_m, g.k.doortools.look_rad);
    for gd in men {
        if (!outside && door_tools_in_at(g, op, gd).is_nan()) || crate::control::man::blind(g, gd) { continue; }
        let c = g.gd[gd].ch;
        let p = g.ch[c].pos;
        let n = g.tab.doortools_room_search[sr].pts.len();
        for i in 0..n {
            let (seen, q) = { let s = &g.tab.doortools_room_search[sr]; (s.looked[i] != 0, s.pts[i]) };
            if seen { continue; }
            let (dx, dz) = (q[0] - p[0], q[2] - p[2]);
            let dd = dx * dx + dz * dz;
            if dd > look_m * look_m { continue; }
            if (outside || dd > 1.44)
                && (m::abs(v3::wrap_angle(m::atan2(dx, dz) - g.ch[c].aim_yaw)) > look_rad || !door_tools_line_to(g, sr, gd, i as f64)) { continue; }
            if let Some(f) = ok.as_mut() { if !f(g, gd, i as f64) { continue; } }
            let s = &mut g.tab.doortools_room_search[sr];
            s.looked[i] = 1;
            s.left -= 1.0;
        }
    }
}

/// a man on his point: his muzzle travels to the unseen floor he has a line to that needs the least turn; false when he has none left
pub(super) fn door_tools_scan(g: &mut crate::game::Game, sr: DoortoolsRoomSearchId, gd: GuardId, a: TaskId) -> bool {
    let (look_m, scan_rate, now) = (g.k.doortools.look_m, g.k.doortools.scan_rate, g.time);
    let c = g.gd[gd].ch;
    let (p, aim) = (g.ch[c].pos, g.ch[c].aim_yaw);
    let s = &mut g.tab.doortools_room_search[sr];
    let was = s.at.get(gd).copied().unwrap_or(now);
    let dt = v3::clamp(now - was, 0.0, 0.05);
    s.at.set(gd, now);
    let y0 = s.yaw.get(gd).copied().unwrap_or(aim);
    let n = s.pts.len();
    let (mut best, mut bd, mut by) = (-1i64, f64::INFINITY, 0.0);
    for i in 0..n {
        let (seen, q) = { let s = &g.tab.doortools_room_search[sr]; (s.looked[i] != 0, s.pts[i]) };
        if seen { continue; }
        let (dx, dz) = (q[0] - p[0], q[2] - p[2]);
        if dx * dx + dz * dz > look_m * look_m || !door_tools_line_to(g, sr, gd, i as f64) { continue; }
        let y = m::atan2(dx, dz);
        let turn = m::abs(v3::wrap_angle(y - y0));
        if turn < bd { bd = turn; best = i as i64; by = y; }
    }
    if best < 0 { g.tab.doortools_room_search[sr].yaw.delete(gd); return false; }
    if crate::roomfight::has_over(g, gd) || crate::condition::is_blind(g, g.gd[gd].ch) { return true; }
    let yaw = y0 + v3::clamp(v3::wrap_angle(by - y0), -scan_rate * dt, scan_rate * dt);
    g.tab.doortools_room_search[sr].yaw.set(gd, yaw);
    let row = &mut g.tab.task[a];
    row.watch = Some([p[0] + m::sin(yaw) * 5.0, 1.1, p[2] + m::cos(yaw) * 5.0]);
    row.face = Some(yaw);
    row.there = "clearing the room".into();
    true
}

/// how far it is from `a` to `b` on foot; Infinity past `lim` or with no way
fn door_tools_walk_len(g: &crate::game::Game, a: V3, b: V3, lim: f64) -> f64 {
    let d = dist_xz(a, b);
    if d > lim { return f64::INFINITY; }
    if !crate::scene_collision::low_blocked(g, a, b) { return d; }
    let path = match crate::scene_collision::nav_grid_find_path(g, a, b, Some(3000.0), None) { Some(p) => p, None => return f64::INFINITY };
    let l = crate::places::walk_len(a, &path) + dist_xz(path.last().copied().unwrap_or(a), b);
    if l > lim { f64::INFINITY } else { l }
}

/// How far it is on foot between two places, each pair measured once a deal, to the longest reach a partner is asked for.
pub(super) struct Walks { memo: JsMap<String, f64>, lim: f64 }
impl Walks {
    pub fn of(g: &Game) -> Walks { let k = &g.k.doortools; Walks { memo: JsMap::new(), lim: m::max_n(&[k.buddy_m, k.buddy_pair_m, k.buddy_left_m]) } }
    pub fn len(&mut self, g: &Game, a: V3, b: V3) -> f64 {
        let key = walk_key(a, b);
        if let Some(l) = self.memo.get(key.clone()) { return *l; }
        let l = door_tools_walk_len(g, a, b, self.lim);
        self.memo.set(key, l);
        l
    }
}

/// the memo's key: the two places to the centimetre
fn walk_key(a: V3, b: V3) -> String {
    format!("{},{}>{},{}", js::to_fixed(a[0], 2), js::to_fixed(a[2], 2), js::to_fixed(b[0], 2), js::to_fixed(b[2], 2))
}

/// the man of `m`'s pair among the squad and `gd`
fn mate_of(g: &Game, squad: &[GuardId], gd: GuardId, m: GuardId) -> Option<GuardId> {
    let id = crate::orders::squad_orders_pair_of(g, m)?;
    for &x in squad.iter().chain(core::iter::once(&gd)) {
        if x != m && crate::orders::squad_orders_pair_of(g, x) == Some(id) { return Some(x); }
    }
    None
}

/// one of `who` (or Sam) within `lim` on foot of P
fn reach(g: &Game, who: &[GuardId], lim: f64, sam: &[V3], p: V3, walks: &mut Walks) -> bool {
    who.iter().any(|&m| { let w = where_of(g, m); walks.len(g, w, p) <= lim }) || sam.iter().any(|&q| walks.len(g, q, p) <= lim)
}

/// would it leave a man by himself who has company now
fn strands(g: &Game, squad: &[GuardId], sam: &[V3], gd: GuardId, p: V3, go: &[GuardId], walks: &mut Walks) -> bool {
    let left = g.k.doortools.buddy_left_m;
    for &c in squad {
        if go.contains(&c) { continue; }
        let at = where_of(g, c);
        let mut stay: Vec<V3> = squad.iter().filter(|&&m| m != c && !go.contains(&m)).map(|&m| where_of(g, m)).collect();
        stay.extend_from_slice(sam);
        if stay.iter().any(|&q| walks.len(g, q, at) <= left) || walks.len(g, p, at) <= left { continue; }
        let from = g.pos_of(gd);
        if walks.len(g, from, at) <= left { return true; }
        for &m in go {
            let q = if m == gd { from } else { where_of(g, m) };
            if walks.len(g, q, at) <= left { return true; }
        }
    }
    false
}

/// his own partner has a door of the order's room to hold, so he is spare
fn lent(g: &Game, squad: &[GuardId], gd: GuardId, free: &[GuardId], m: GuardId, op: Option<DoorOpId>) -> bool {
    let (Some(o), Some(op)) = (mate_of(g, squad, gd, m), op) else { return false };
    !free.contains(&o) && !crate::condition::down(g, g.gd[o].ch) && g.tab.doortools_door_op[op].men.contains(&o) && !crate::roomfight::has_over(g, o) && door_tools_on_other(g, op, o)
}

/// the nearest of `list` to `gd`, the first on a tie
fn nearest_of(g: &Game, list: &[GuardId], gd: GuardId) -> Option<GuardId> {
    let at = g.pos_of(gd);
    let mut l: Vec<(GuardId, f64)> = list.iter().map(|&m| (m, dist_xz(g.pos_of(m), at))).collect();
    js::sort_by(&mut l, |a, b| a.1 - b.1);
    l.first().map(|x| x.0)
}

/// `gd` would be dealt a walk to P: alone, with this partner, or no (and why, kept on the door tools). `pair`: the order he would take a partner for wherever one is free.
pub(super) fn door_tools_away(g: &mut crate::game::Game, gd: GuardId, p: V3, pool: &[GuardId], walks: &mut Walks, pair: Option<DoorOpId>) -> crate::doortools::DoorToolsAway {
    let (buddy_m, buddy_pair_m) = (g.k.doortools.buddy_m, g.k.doortools.buddy_pair_m);
    away_said(g, "", false);
    let squad: Vec<GuardId> = crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().filter(|&m| m != gd && !crate::condition::down(g, g.gd[m].ch)).collect();
    let sam: Vec<V3> = crate::group::player(g).map(|_| crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g))).into_iter().collect();
    let own = mate_of(g, &squad, gd, gd);
    let mut mate: Option<GuardId> = None;
    if pair.is_some() || !reach(g, &squad, buddy_m, &sam, p, walks) {
        mate = partner_for(g, &squad, gd, own, pool, pair);
        if mate.is_none() {
            let why = match own { Some(_) => format!("{}'s partner is busy", g.gd[gd].callsign), None => String::from("no partner free") };
            away_said(g, &why, false);
            return if pair.is_some() && reach(g, &squad, buddy_m, &sam, p, walks) { DoorToolsAway::Alone } else { DoorToolsAway::No };
        }
    }
    let borrowed = mate.is_some() && own.is_some() && mate != own;
    let others: Vec<GuardId> = squad.iter().copied().filter(|&m| (borrowed || Some(m) != own) && Some(m) != mate).collect();
    if mate.is_some() && others.is_empty() { away_said(g, "nobody would stay behind the two", false); return DoorToolsAway::No; }
    if (!others.is_empty() || !sam.is_empty()) && !reach(g, &others, buddy_pair_m, &sam, p, walks) {
        away_said(g, &format!("past {} m from the rest", js::num_str(buddy_pair_m)), true);
        return DoorToolsAway::No;
    }
    let go: Vec<GuardId> = mate.into_iter().collect();
    if strands(g, &squad, &sam, gd, p, &go, walks) { away_said(g, "it would leave a man by himself", true); return DoorToolsAway::No; }
    match mate { Some(m) => DoorToolsAway::Guard(m), None => DoorToolsAway::Alone }
}
fn away_said(g: &mut Game, why: &str, far: bool) {
    let d = &mut g.groups[g.group].tools;
    d.away_why = String::from(why);
    d.away_far = far;
}
/// Who would go with him: his own partner, or a man whose partner is kept at a door as his is; a man of no pair, another of none.
fn partner_for(g: &Game, squad: &[GuardId], gd: GuardId, own: Option<GuardId>, pool: &[GuardId], pair: Option<DoorOpId>) -> Option<GuardId> {
    let free: Vec<GuardId> = pool.iter().copied().filter(|&m| m != gd && !crate::condition::down(g, g.gd[m].ch)).collect();
    let spare: Vec<GuardId> = match own {
        Some(o) if free.contains(&o) => return Some(o),
        Some(_) if !lent(g, squad, gd, &free, gd, pair) => return None,
        Some(_) => free.iter().copied().filter(|&m| lent(g, squad, gd, &free, m, pair)).collect(),
        None => free.iter().copied().filter(|&m| mate_of(g, squad, gd, m).is_none()).collect(),
    };
    nearest_of(g, &spare, gd)
}

/// the partner's point beside P, and the longest look he has there off his partner's sector
pub(super) fn door_tools_beside_pod(g: &mut crate::game::Game, p: V3, face: f64, mate: GuardId, taken: &[V3], in_room: &dyn Fn(&crate::game::Game, V3) -> bool) -> Option<crate::entrypoints::Pod> {
    let apart = m::max(1.0, g.k.doortools.pod_apart);
    let mate_at = g.pos_of(mate);
    let (mut best, mut bc): (Option<V3>, f64) = (None, f64::INFINITY);
    for r in [1.5, 2.0, 2.5] {
        for k in 0..12 {
            let a = k as f64 * m::PI / 6.0;
            let q: V3 = [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || !in_room(g, q)
                || crate::scene_collision::low_blocked(g, p, q)
                || taken.iter().any(|&t| dist_xz(t, q) < apart)
                || dist_xz(crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g)), q) < 1.0 { continue; }
            if crate::entrypoints::entry_points_risk_at(g, q, None, None).is_some() { continue; }
            let c = dist_xz(mate_at, q) + (r - 1.5) * 0.5;
            if c < bc { bc = c; best = Some(q); }
        }
    }
    let best = best?;
    let mut home: V3 = [0.0, 0.0, 0.0];
    let n = taken.len() as f64;
    for t in taken { home[0] += t[0] / n; home[2] += t[2] / n; }
    let mut yaw = v3::wrap_angle(face + m::PI);
    let mut bl = f64::NEG_INFINITY;
    for k in 0..8 {
        let a = k as f64 * m::PI / 4.0;
        if m::abs(v3::wrap_angle(a - face)) < m::PI / 3.0 || m::abs(v3::wrap_angle(a - v3::yaw_to(best, p))) < m::PI / 6.0 { continue; }
        let mut l = 0.0;
        for d in [2.0, 4.0, 6.0, 8.0, 10.0] {
            if crate::scene_collision::segment_blocked(g, [best[0], 1.5, best[2]], [best[0] + m::sin(a) * d, 1.5, best[2] + m::cos(a) * d], None, None, None) { break; }
            l = d;
        }
        if !taken.is_empty() { l -= 3.0 * m::max(0.0, m::cos(v3::wrap_angle(a - v3::yaw_to(best, home)))); }
        if l > bl { bl = l; yaw = a; }
    }
    Some(pod_at(best, yaw))
}

/// an entry's man sent on to a point of the room it is still looking over
fn door_tools_send_on(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, pod: &mut crate::entrypoints::Pod, going: &str) {
    let door = g.tab.doortools_door_op[op].door;
    g.tab.doortools_door_op[op].pod.set(gd, *pod);
    g.tab.doortools_door_op[op].on_pod.delete(gd);
    let a = crate::doortools::DoorToolsAssignA {
        post: Some(pod.pos),
        face: Some(pod.face),
        watch: None,
        shy: Some(None),
        thru: Some(door),
        pace: Some(g.k.orders.entry),
        stance: Some(crate::character::Stance::None),
        upper: Some(crate::character::UpperMode::Aim),
        light: Some(g.groups[g.group].tools.lights.unwrap_or(true)),
        search: Some(true),
        going: String::from(going).into(),
        there: "holding the room".into(),
        ..Default::default()
    };
    door_tools_assign(g, op, gd, &a);
}

/// is this room's floor one the whole-room rule works? A room past the size is said once an order
pub(super) fn door_tools_flows(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId) -> bool {
    let flow_max = g.k.doortools.flow_max;
    let cells = g.tab.doortools_room_search[sr].pts.len();
    if !(flow_max > 0.0) || cells as f64 <= flow_max { return true; }
    if !g.groups[g.group].tools.big_said.has(op) {
        g.groups[g.group].tools.big_said.add(op);
        door_tools_beat(g, op, &format!("flow: no · a room of {} cells is looked over, not gone through (flowMax {})", cells, js::num_str(flow_max)));
    }
    false
}

/// the point of the room's list on this cell of the nav grid
pub(super) fn cell_of(g: &Game, at: &JsMap<crate::Num, f64>, x: f64, z: f64) -> Option<usize> {
    let cs = g.col.nav.cell;
    let (ix, iz) = (m::floor(x / cs), m::floor(z / cs));
    if ix < 0.0 || iz < 0.0 || ix >= g.col.nav.w { return None; }
    let k = crate::scene_collision::nav_grid_idx(g, ix, iz);
    at.get(crate::Num(k)).map(|&i| i as usize)
}

/// A point dealt to a man, and he is sent there.
pub(super) fn search_moves(g: &mut Game, op: DoorOpId, sr: DoortoolsRoomSearchId, gd: GuardId, pod: &mut crate::entrypoints::Pod, going: &str) {
    g.tab.doortools_room_search[sr].yaw.delete(gd);
    door_tools_send_on(g, op, gd, pod, going);
}

/// what `door_tools_away` ruled for a point of a search
#[derive(Clone, Copy)]
enum PointMate { Refused, Alone, With(GuardId, crate::entrypoints::Pod) }

#[derive(Clone, Copy)]
struct MoveCand { i: usize, p: V3, sc: f64 }

/// The floor of a search nobody can see from where he stands, its middle, and how much of it each point looked at so far shows.
struct Dead { sr: DoortoolsRoomSearchId, pts: Vec<V3>, dead: Vec<V3>, mid: V3, seen: JsMap<usize, f64> }
impl Dead {
    fn of(g: &Game, sr: DoortoolsRoomSearchId) -> Dead {
        let s = &g.tab.doortools_room_search[sr];
        let dead: Vec<V3> = s.pts.iter().zip(&s.looked).filter(|x| *x.1 == 0).map(|x| *x.0).collect();
        let (n, mut mid) = (dead.len() as f64, [0.0, 0.0, 0.0]);
        for q in &dead { mid[0] += q[0] / n; mid[2] += q[2] / n; }
        Dead { sr, pts: s.pts.clone(), dead, mid, seen: JsMap::new() }
    }
    /// one in this many of the unseen cells is asked about
    fn step(&self) -> usize { core::cmp::max(1, self.dead.len() / 40) }
    /// how many of those are in plain view of point `i`, asked once a point
    fn shown(&mut self, g: &Game, i: usize) -> f64 {
        if let Some(&s) = self.seen.get(i) { return s; }
        let (p, far) = (self.pts[i], g.k.doortools.look_m * 0.8);
        let s = self.dead.iter().step_by(self.step()).filter(|&&q| dist_xz(p, q) <= far && !crate::scene_collision::segment_blocked(g, [p[0], 1.55, p[2]], q, None, None, None)).count() as f64;
        self.seen.set(i, s);
        s
    }
}

/// floor left that no man can see from where he stands: the man nearest it goes on to the point it is best seen from. True when one was sent
pub(super) fn door_tools_move_on(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId) -> bool {
    let (dead_cells, most) = (g.k.doortools.dead_cells, g.k.doortools.search_moves);
    let flowing = door_tools_flows(g, op, sr);
    let s = &g.tab.doortools_room_search[sr];
    if s.left < dead_cells || (!flowing && s.moves >= most) || s.left > s.left_at_move - 3.0 { return false; }
    let mut dead = Dead::of(g, sr);
    let mut by: Vec<(GuardId, f64)> = Vec::new();
    for mm in g.tab.doortools_door_op[op].men.clone() {
        if !door_tools_in_at(g, op, mm).is_nan() && !crate::control::man::blind(g, mm) && !crate::roomfight::has_over(g, mm) && !door_tools_on_other(g, op, mm) { by.push((mm, dist_xz(g.pos_of(mm), dead.mid))); }
    }
    if by.is_empty() { return false; }
    js::sort_by(&mut by, |a, b| a.1 - b.1);
    let men: Vec<GuardId> = by.iter().map(|x| x.0).collect();
    if flowing {
        if door_tools_flow_in(g, op, sr, &men) { return true; }
        if g.tab.doortools_room_search[sr].moves >= most { return false; }
    }
    let mut walks = Walks::of(g);
    for &gd in &men {
        let points = points_for(g, &mut dead, gd);
        let mut ask = Asked { gd, men: &men, mates: JsMap::new(), refused: false };
        match best_point(g, &dead, &points, &mut ask, &mut walks) {
            Some(c) => { sent_on(g, op, &dead, gd, c, ask.mates.get(c.i).copied()); return true; }
            None if ask.refused => continue,
            None => return false,
        }
    }
    false
}

/// Every point that shows him enough of the unseen floor, the best first: what it shows, less the walk.
fn points_for(g: &Game, dead: &mut Dead, gd: GuardId) -> Vec<MoveCand> {
    let (apart, least) = (m::max(1.0, g.k.doortools.pod_apart), g.k.doortools.dead_cells / 2.0);
    let from = g.pos_of(gd);
    let mut others: Vec<V3> = Vec::new();
    for who in crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None) {
        if who != gd { others.push(g.pos_of(who)); pods_of(g, &crate::seats::here(g), who, &mut others); }
    }
    let of = dead.step() as f64;
    let mut out: Vec<MoveCand> = Vec::new();
    for i in (0..dead.pts.len()).step_by(core::cmp::max(1, dead.pts.len() / 80)) {
        let p = dead.pts[i];
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || dist_xz(from, p) < 1.5 || others.iter().any(|&q| dist_xz(q, p) < apart) { continue; }
        let shown = dead.shown(g, i) * of;
        let sc = shown - 0.8 * dist_xz(from, p);
        if shown >= least && sc > 0.0 { out.push(MoveCand { i, p, sc }); }
    }
    js::sort_by(&mut out, |a, b| b.sc - a.sc);
    out
}

/// May a man be dealt a point: each asked once.
struct Asked<'a> { gd: GuardId, men: &'a [GuardId], mates: JsMap<usize, PointMate>, refused: bool }
impl Asked<'_> {
    fn may(&mut self, g: &mut Game, dead: &Dead, c: &MoveCand, walks: &mut Walks) -> bool {
        if let Some(known) = self.mates.get(c.i) { return !matches!(known, PointMate::Refused); }
        let answer = match door_tools_away(g, self.gd, c.p, self.men, walks, None) {
            DoorToolsAway::No => PointMate::Refused,
            DoorToolsAway::Alone => PointMate::Alone,
            DoorToolsAway::Guard(x) => {
                let taken: Vec<V3> = crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().filter(|&who| who != self.gd && who != x).map(|who| where_of(g, who)).collect();
                let room = g.tab.doortools_room_search[dead.sr].room;
                match door_tools_beside_pod(g, c.p, v3::yaw_to(c.p, dead.mid), x, &taken, &|g, q| in_room(g, room, q)) { Some(pod) => PointMate::With(x, pod), None => PointMate::Refused }
            }
        };
        self.mates.set(c.i, answer);
        let ok = !matches!(answer, PointMate::Refused);
        self.refused = self.refused || !ok;
        ok
    }
}
fn in_room(g: &Game, room: RoomfightRoomId, q: V3) -> bool {
    let cell = g.col.nav.cell;
    let c = crate::scene_collision::nav_grid_idx(g, m::floor(q[0] / cell), m::floor(q[2] / cell));
    g.tab.roomfight_room[room].cells.has(crate::Num(c))
}

/// The best point he may have; or one not much worse that is out of the line of an opening nobody has been beyond.
fn best_point(g: &mut Game, dead: &Dead, points: &[MoveCand], ask: &mut Asked, walks: &mut Walks) -> Option<MoveCand> {
    let top = *points.iter().find(|c| ask.may(g, dead, c, walks))?;
    let clear = points.iter().find(|c| crate::entrypoints::entry_points_risk_at(g, c.p, None, None).is_none() && ask.may(g, dead, c, walks));
    Some(match clear { Some(c) if c.sc >= 0.6 * top.sc => *c, _ => top })
}

/// He goes on to it, and his partner beside him.
fn sent_on(g: &mut Game, op: DoorOpId, dead: &Dead, gd: GuardId, c: MoveCand, mate: Option<PointMate>) {
    let mut pod = pod_at([c.p[0], 0.0, c.p[2]], v3::yaw_to(c.p, dead.mid));
    let s = &mut g.tab.doortools_room_search[dead.sr];
    s.moves += 1.0;
    s.left_at_move = s.left;
    search_moves(g, op, dead.sr, gd, &mut pod, "clearing the room");
    let mut said = format!("on:{}", g.gd[gd].callsign);
    if let Some(PointMate::With(x, mut his)) = mate {
        search_moves(g, op, dead.sr, x, &mut his, "clearing the room");
        said = format!("{said} with {}", g.gd[x].callsign);
    }
    door_tools_beat(g, op, &said);
}

/// the points of domination for a door taken from `side`, as `entry_points_clear_of` leaves them
pub(super) fn door_tools_pods(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> {
    let w = g.dr[d].def.width;
    let were = door_tools_pods_as_were(g, d, side, n);
    crate::entrypoints::entry_points_clear_of(g, d, &were, &mut |g: &mut Game, p: V3| [along_of(g, d, p) - w / 2.0, -off_of(g, d, p, side)])
}

/// the points before `entry_points_clear_of` looks at them
pub fn door_tools_pods_as_were(g: &crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> {
    let n = n.unwrap_or(2.0);
    let mut out = door_tools_pods2(g, d, side);
    if n <= 2.0 || out.is_empty() { return out; }
    let w = g.dr[d].def.width;
    let into = -side;
    let mid = door_pt(g, d, w / 2.0, 0.9, into, Some(0.45));
    for s in [1.0, -1.0] {
        if out.len() as f64 >= n { break; }
        'find: for dd in [1.0, 1.4, 0.8, 1.9] {
            for l in [w / 2.0 + 0.9, w / 2.0 + 1.4, w / 2.0 + 0.55, w / 2.0 + 2.0] {
                let p = door_pt(g, d, w / 2.0 + s * l, dd, into, None);
                if !pods_fit(g, d, mid, p, &out) { continue; }
                let toward = door_pt(g, d, w / 2.0 - s * 0.6, 4.5, into, None);
                out.push(pod_at(p, v3::yaw_to(p, toward)));
                break 'find;
            }
        }
    }
    for b in [3.0, 2.2, 3.8] {
        if out.len() as f64 >= n { break; }
        let p = door_pt(g, d, w / 2.0, b, into, None);
        if pods_fit(g, d, mid, p, &out) {
            let (a0, a1) = (door_pt(g, d, w / 2.0, 0.0, into, None), door_pt(g, d, w / 2.0, 1.0, into, None));
            out.push(pod_at(p, v3::yaw_to(a0, a1)));
        }
    }
    out
}

/// podsAsWere: fits
fn pods_fit(g: &Game, d: DoorId, mid: V3, p: V3, out: &[crate::entrypoints::Pod]) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2])
        && !crate::scene_collision::solid_blocked(g, mid, [p[0], 0.45, p[2]], None)
        && !out.iter().any(|o| dist_xz(o.pos, p) < 0.9)
        && !door_tools_in_frame(g, d, mid, p)
}

/// DOORTOOLS.podFrames: `p` stands in the opening of a door other than `d`, or the line to it from `mid` passes through one
pub(super) fn door_tools_in_frame(g: &crate::game::Game, d: DoorId, mid: V3, p: V3) -> bool {
    let frame_m = g.k.doortools.pod_frame_m;
    for &o in g.doors.list.iter() {
        if o == d { continue; }
        let def = &g.dr[o].def;
        let ([hx, hz], w) = (def.hinge, def.width);
        let (ux, uz) = (m::cos(def.closed_dir), m::sin(def.closed_dir));
        let a = (p[0] - hx) * ux + (p[2] - hz) * uz;
        let b = -(p[0] - hx) * uz + (p[2] - hz) * ux;
        if a > -0.1 && a < w + 0.1 && m::abs(b) < frame_m { return true; }
        let mm = -(mid[0] - hx) * uz + (mid[2] - hz) * ux;
        if mm * b >= 0.0 { continue; }
        let t = mm / (mm - b);
        let x = (mid[0] + (p[0] - mid[0]) * t - hx) * ux + (mid[2] + (p[2] - mid[2]) * t - hz) * uz;
        if x > 0.0 && x < w { return true; }
    }
    false
}

/// the two deep points: the level's clear plan when authored from that face, else the first free floor to either hand
fn door_tools_pods2(g: &crate::game::Game, d: DoorId, side: f64) -> Vec<crate::entrypoints::Pod> {
    let w = g.dr[d].def.width;
    let into = -side;
    let name = g.dr[d].def.name.clone();
    if let Some(plan) = g.level.door_plan(&name) {
        if crate::orders::side_of(g, d, plan.stack[0]) == side {
            let mut out: Vec<crate::entrypoints::Pod> = Vec::new();
            for k in 0..2 {
                let pos = v3::copy(plan.entry[k][plan.entry[k].len() - 1]);
                if !crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) && off_of(g, d, pos, side) < -0.3 {
                    out.push(pod_at(pos, plan.pod_face[k]));
                }
            }
            if out.len() == 2 { return out; }
        }
    }
    let mid = door_pt(g, d, w / 2.0, 0.9, into, Some(0.45));
    let mut out: Vec<crate::entrypoints::Pod> = Vec::new();
    for s in [1.0, -1.0] {
        'find: for dd in [1.7, 2.2, 1.2] {
            for l in [1.7, 1.3, 0.9] {
                let p = door_pt(g, d, w / 2.0 + s * l, dd, into, None);
                if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || crate::scene_collision::solid_blocked(g, mid, [p[0], 0.45, p[2]], None) || door_tools_in_frame(g, d, mid, p) { continue; }
                let toward = door_pt(g, d, w / 2.0 - s * 1.2, 4.0, into, None);
                out.push(pod_at(p, v3::yaw_to(p, toward)));
                break 'find;
            }
        }
    }
    let (a0, a1) = (door_pt(g, d, w / 2.0, 0.0, into, None), door_pt(g, d, w / 2.0, 1.0, into, None));
    let inward = v3::yaw_to(a0, a1);
    for b in [1.5, 2.4] {
        if out.len() >= 2 { break; }
        let q = door_pt(g, d, w / 2.0, b, into, None);
        let f = if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { crate::scene_collision::nav_grid_nearest_free_point(g, q[0], q[2]) } else { Some(q) };
        if let Some(f) = f {
            if !out.iter().any(|o| dist_xz(o.pos, f) < 0.6) { out.push(pod_at([f[0], 0.0, f[2]], inward)); }
        }
    }
    out
}

/// The queue is written, and the word goes.
fn enter_go(g: &mut Game, op: DoorOpId, t: f64, queue: Vec<DoorToolsDoneItemQueueItem>) {
    let o = &mut g.tab.doortools_door_op[op];
    (o.queue, o.phase, o.t0) = (queue, Phase::Enter, t);
    let lead = o.lead;
    door_tools_beat_is(g, op, crate::doortools::Beat::Go, "go");
    crate::game::game_say(g, lead, "going in.", None, None);
}

/// Of two men of one entry by `d`, is one named to go in before the other: `Some(true)` the first of them, `Some(false)` the second, until the man named is over. None: neither is named, or it is not the one entry.
pub fn named_before(g: &crate::game::Game, d: DoorId, a: GuardId, b: GuardId) -> Option<bool> {
    for &op in &crate::seats::here(g).door_tools.ops {
        let o = &g.tab.doortools_door_op[op];
        if o.door != d || o.phase != Phase::Enter || !o.men.contains(&a) || !o.men.contains(&b) { continue; }
        let first = o.first?;
        if o.crossed.has(first) || crate::condition::down(g, g.gd[first].ch) { return None; }
        return if a == first { Some(true) } else if b == first { Some(false) } else { None };
    }
    None
}

/// the men go in: the lead first, the next `second` later, each to his point of domination
pub(super) fn door_tools_enter(g: &mut crate::game::Game, op: DoorOpId) {
    if g.tab.doortools_door_op[op].thr.is_some() { door_tools_step_in(g, op); return; }
    let t = g.time;
    let (door, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let n_men = g.tab.doortools_door_op[op].men.len();
    let big = crate::orders::squad_orders_big(g, &crate::seats::here(g));
    let pods = door_tools_pods(g, door, side, Some(if n_men > 2 || big { 4.0 } else { n_men as f64 }));
    if pods.is_empty() {
        crate::game::game_say(g, lead, "no way in.", None, None);
        door_tools_end(g, op, crate::doortools::Ended::NoFloorInside);
        return;
    }
    let first = door_tools_first_in(g, op, Some(pods[0].pos), None);
    if first != lead && crate::gunroles::gun_before(g, first, lead) && door_tools_first_in(g, op, Some(pods[0].pos), Some(false)) != first {
        let said = format!("first:{} · his gun is the room's", g.gd[first].callsign);
        door_tools_beat(g, op, &said);
    } else if first != lead {
        let said = format!("first:{} · has seen most of the room", g.gd[first].callsign);
        door_tools_beat(g, op, &said);
    }
    g.tab.doortools_door_op[op].first = Some(first);
    let mut men: Vec<GuardId> = vec![first];
    men.extend(g.tab.doortools_door_op[op].men.iter().copied().filter(|&m| m != first));
    if men.len() > 2 {
        // THREE OR MORE: in turn off alternate jambs while both files have men; a man with no point keeps his place on the stack, and is not this order's any more
        let w = g.dr[door].def.width;
        let centre = g.dr[door].frame_centre;
        let mut files: Vec<(GuardId, bool, f64)> = Vec::with_capacity(men.len());
        for &m in &men {
            let at = g.tab.doortools_door_op[op].post.get(m).map(|p| p.pos).unwrap_or_else(|| g.pos_of(m));
            files.push((m, along_of(g, door, at) > w / 2.0, dist_xz(at, centre)));
        }
        let hi_first = files[0].1;
        let mut mine: Vec<(GuardId, bool, f64)> = files.iter().copied().filter(|f| f.1 == hi_first).collect();
        js::sort_by(&mut mine, |a, b| if a.0 == first { -1.0 } else if b.0 == first { 1.0 } else { a.2 - b.2 });
        let mut theirs: Vec<(GuardId, bool, f64)> = files.iter().copied().filter(|f| f.1 != hi_first).collect();
        js::sort_by(&mut theirs, |a, b| a.2 - b.2);
        let mut turn: Vec<GuardId> = Vec::new();
        for k in 0..men.len() {
            if let Some(f) = mine.get(k) { turn.push(f.0); }
            if let Some(f) = theirs.get(k) { turn.push(f.0); }
        }
        let turn = crate::gunroles::gun_turn(g, &turn, first);
        let turn = crate::rearwatch::rear_last(g, op, &turn, first);
        let cut = core::cmp::min(pods.len(), turn.len());
        let (in_men, stay) = (&turn[..cut], &turn[cut..]);
        let mut deal: JsMap<GuardId, crate::entrypoints::Pod> = JsMap::new();
        for k in [0usize, 2] {
            let m = &in_men[core::cmp::min(k, in_men.len())..core::cmp::min(k + 2, in_men.len())];
            let sp = &pods[core::cmp::min(k, pods.len())..core::cmp::min(k + 2, pods.len())];
            if m.is_empty() { continue; }
            let spots: Vec<V3> = sp.iter().map(|x| x.pos).collect();
            for (gd, pos) in crate::orders::squad_orders_pair(g, m, &spots, None, None) {
                deal.set(gd, *sp.iter().find(|x| x.pos == pos).unwrap());
            }
        }
        if !stay.is_empty() {
            g.tab.doortools_door_op[op].men.retain(|m| !stay.contains(m));
            let names: Vec<String> = stay.iter().map(|&m| g.gd[m].callsign.clone()).collect();
            door_tools_beat(g, op, &format!("outside · {}", names.join(", ")));
        }
        let queue: Vec<DoorToolsDoneItemQueueItem> = in_men.iter().map(|&gd| {
            DoorToolsDoneItemQueueItem { gd, pod: deal.get(gd).copied().unwrap_or(pods[0]) }
        }).collect();
        enter_go(g, op, t, queue);
        return;
    }
    let own: Vec<crate::entrypoints::Pod> = if big { door_tools_pods_apart(g, op, &pods, men.len() as f64) } else { pods };
    let spots: Vec<V3> = own.iter().take(men.len()).map(|p| p.pos).collect();
    let pairs = crate::orders::squad_orders_pair(g, &men, &spots, None, None);
    let queue: Vec<DoorToolsDoneItemQueueItem> = men.iter().map(|&gd| {
        let to = pairs.iter().find(|x| x.0 == gd).unwrap().1;
        DoorToolsDoneItemQueueItem { gd, pod: own.iter().find(|p| p.pos == to).copied().unwrap_or(own[0]) }
    }).collect();
    enter_go(g, op, t, queue);
}

/// THE SQUAD OF FOUR, a pair going in: its points kept `podApart` clear of the other order's men and of any teammate posted inside
pub(super) fn door_tools_pods_apart(g: &crate::game::Game, op: DoorOpId, pods: &[crate::entrypoints::Pod], n: f64) -> Vec<crate::entrypoints::Pod> {
    let pod_apart = g.k.doortools.pod_apart;
    let (door, side) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side) };
    let mut taken: Vec<V3> = Vec::new();
    for &o in g.groups[g.group].tools.ops.iter() {
        if o == op { continue; }
        for p in g.tab.doortools_door_op[o].pod.values() { taken.push(p.pos); }
        for q in g.tab.doortools_door_op[o].queue.iter() { taken.push(q.pod.pos); }
    }
    for gd in crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None) {
        if g.tab.doortools_door_op[op].men.contains(&gd) { continue; }
        if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), gd) {
            let (post, arrived) = (g.tab.task[a].post, g.tab.task[a].arrived);
            if let Some(post) = post { if arrived && off_of(g, door, post, side) < 0.0 { taken.push(post); } }
        }
    }
    // (by index: two points may hold the same numbers)
    let clear: Vec<usize> = (0..pods.len()).filter(|&i| !taken.iter().any(|&q| dist_xz(q, pods[i].pos) < pod_apart)).collect();
    if clear.len() as f64 >= n { return clear.iter().map(|&i| pods[i]).collect(); }
    let mut out: Vec<crate::entrypoints::Pod> = clear.iter().map(|&i| pods[i]).collect();
    for i in 0..pods.len() { if !clear.contains(&i) { out.push(pods[i]); } }
    out
}

/// Sam's trigger: the set charge nearest him (or the one given) goes; what it did, null with none set
pub(super) fn door_tools_blow(g: &mut crate::game::Game, c_2: Option<ChargeId>) {
    let c = match c_2 { Some(c) => Some(c), None => door_tools_nearest(g, None) };
    let who = g.player_id(g.player);
    crate::fogofwar::fog_of_war_doing(g, Some(who), &mut |g: &mut Game| door_tools_blow_raw(g, c));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A deal measures a walk once. A failure means two men on one spot ask the floor twice, or a walk back is answered with the walk there.
    #[test]
    fn a_walk_is_kept_by_its_two_places_to_the_centimetre_and_its_way() {
        let (a, b): (V3, V3) = ([1.0, 0.0, 2.0], [3.0, 0.0, 4.0]);
        assert_eq!(walk_key([1.001, 9.0, 2.0], b), walk_key(a, b));   // the same place at any height, to the centimetre
        assert_ne!(walk_key([1.02, 0.0, 2.0], b), walk_key(a, b));
        assert_ne!(walk_key(b, a), walk_key(a, b));
    }
}
