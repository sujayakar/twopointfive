//! src/game/doortools.ts lines 2963 to 3312
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::v3::dist_xz as flat;

// ---- small reads

/// `SR` of the TypeScript's methods: always the order's own (doortools.ts:2346, :2358, :2361, :2366, :2458). Let go before the next call that takes `g`
fn search(g: &mut Game, op: DoorOpId) -> &mut RoomSearch { g.tab.doortools_door_op[op].search.as_mut().unwrap() }
/// a point as this file deals one: `{ pos, face }`
fn pod_at(pos: V3, face: f64) -> crate::entrypoints::Pod { crate::entrypoints::Pod { pos, face, watch: None, moved: false } }

fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn alive(g: &Game, gd: GuardId) -> bool { g.gd[gd].state != crate::game::GuardState::Dead }
fn has_over(g: &Game, gd: GuardId) -> bool { g.gd[gd].script.as_ref().map_or(false, |s| s.over.is_some()) }
/// S.of(m)?.post ?? m.char.pos
fn where_of(g: &mut Game, m: GuardId) -> V3 {
    let post = crate::orders::squad_orders_of(g, m).and_then(|a| g.tab.orders_assignment[a].post);
    post.unwrap_or_else(|| pos_of(g, m))
}
/// where every order has `m` going, in the orders' order
fn pods_of(g: &Game, m: GuardId, out: &mut Vec<V3>) {
    for &o in g.door_tools.ops.iter() {
        if let Some(q) = g.tab.doortools_door_op[o].pod.get(m) { out.push(q.pos); }
    }
}

/// Number.prototype.toFixed(2): exact, a tie going away from zero, -0 without its sign
fn fixed2(x: f64) -> String {
    let a = m::abs(x);
    let e = a * 8.0;
    let tie = m::floor(e) == e && js::rem(e, 2.0) == 1.0;
    format!("{}{:.2}", if x < 0.0 { "-" } else { "" }, if tie { a + 0.001 } else { a })
}

/// the walks' memo of one deal (the closure DoorTools.walks answers)
struct Walks { memo: JsMap<String, f64>, lim: f64 }
impl Walks {
    fn new(g: &Game) -> Walks {
        let k = &g.k.doortools;
        Walks { memo: JsMap::new(), lim: m::max_n(&[k.buddy_m, k.buddy_pair_m, k.buddy_left_m]) }
    }
    fn len(&mut self, g: &mut Game, a: V3, b: V3) -> f64 {
        let key = format!("{},{}>{},{}", fixed2(a[0]), fixed2(a[2]), fixed2(b[0]), fixed2(b[2]));
        if let Some(l) = self.memo.get(key.clone()) { return *l; }
        let l = door_tools_walk_len(g, a, b, self.lim);
        self.memo.set(key, l);
        l
    }
}

/// has `gd` a line from where he stands to cell `i`, asked once per cell per place he stands (doortools.ts DoorTools.lineTo)
pub fn door_tools_line_to(g: &mut crate::game::Game, sr_key: DoorOpId, gd: GuardId, i: f64) -> bool {
    let i = i as usize;
    let c = g.gd[gd].ch;
    let leans = g.gd[gd].script.as_ref().and_then(|s| s.lean).map_or(false, js::truthy);
    let hd: Option<V3> = if leans { g.ch[c].bones.get(crate::character::Bone::Head).copied() } else { None };
    let p = hd.unwrap_or(g.ch[c].pos);
    let s = search(g, sr_key);
    let stale = match s.los.get(gd) {
        None => true,
        Some(l) => flat(l.at, p) > 0.35 || match hd { Some(h) => m::abs(l.at[1] - h[1]) > 0.2, None => l.at[1] != p[1] },
    };
    if stale {
        let n = s.pts.len();
        s.los.set(gd, RoomSearchLosValue { at: v3::copy(p), ok: vec![-1; n] });
    }
    let q = s.pts[i];
    let (at, known) = { let l = s.los.get(gd).unwrap(); (l.at, l.ok[i]) };
    if known < 0 {
        let blocked = crate::scene_collision::segment_blocked(g, [at[0], if hd.is_some() { at[1] } else { 1.55 }, at[2]], q, None, None, None);
        search(g, sr_key).los.get_mut(gd).unwrap().ok[i] = if blocked { 0 } else { 1 };
    }
    search(g, sr_key).los.get(gd).unwrap().ok[i] == 1
}

/// every cell a man's muzzle is on this frame is looked at (doortools.ts DoorTools.look)
pub fn door_tools_look(g: &mut crate::game::Game, op: DoorOpId, out: Option<&[GuardId]>, ok: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId, f64) -> bool>) {
    let mut ok = ok;
    let outside = out.is_some();
    let men: Vec<GuardId> = match out { Some(l) => l.to_vec(), None => g.tab.doortools_door_op[op].men.clone() };
    let (look_m, look_rad) = (g.k.doortools.look_m, g.k.doortools.look_rad);
    for gd in men {
        if (!outside && door_tools_in_at(g, op, gd).is_nan()) || g.time < g.gd[gd].dazzled_until { continue; }
        let c = g.gd[gd].ch;
        let p = g.ch[c].pos;
        let n = search(g, op).pts.len();
        for i in 0..n {
            let (seen, q) = { let s = search(g, op); (s.looked[i] != 0, s.pts[i]) };
            if seen { continue; }
            let (dx, dz) = (q[0] - p[0], q[2] - p[2]);
            let dd = dx * dx + dz * dz;
            if dd > look_m * look_m { continue; }
            if (outside || dd > 1.44)
                && (m::abs(v3::wrap_angle(m::atan2(dx, dz) - g.ch[c].aim_yaw)) > look_rad || !door_tools_line_to(g, op, gd, i as f64)) { continue; }
            if let Some(f) = ok.as_mut() { if !f(g, gd, i as f64) { continue; } }
            let s = search(g, op);
            s.looked[i] = 1;
            s.left -= 1.0;
        }
    }
}

/// a man on his point: his muzzle travels to the unseen floor he has a line to that needs the least turn; false when he has none left (doortools.ts DoorTools.scan)
pub fn door_tools_scan(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId) -> bool {
    let (look_m, scan_rate, now) = (g.k.doortools.look_m, g.k.doortools.scan_rate, g.time);
    let c = g.gd[gd].ch;
    let (p, aim) = (g.ch[c].pos, g.ch[c].aim_yaw);
    let s = search(g, op);
    let was = s.at.get(gd).copied().unwrap_or(now);
    let dt = v3::clamp(now - was, 0.0, 0.05);
    s.at.set(gd, now);
    let y0 = s.yaw.get(gd).copied().unwrap_or(aim);
    let n = s.pts.len();
    let (mut best, mut bd, mut by) = (-1i64, f64::INFINITY, 0.0);
    for i in 0..n {
        let (seen, q) = { let s = search(g, op); (s.looked[i] != 0, s.pts[i]) };
        if seen { continue; }
        let (dx, dz) = (q[0] - p[0], q[2] - p[2]);
        if dx * dx + dz * dz > look_m * look_m || !door_tools_line_to(g, op, gd, i as f64) { continue; }
        let y = m::atan2(dx, dz);
        let turn = m::abs(v3::wrap_angle(y - y0));
        if turn < bd { bd = turn; best = i as i64; by = y; }
    }
    if best < 0 { search(g, op).yaw.delete(gd); return false; }
    if has_over(g, gd) || now < g.gd[gd].dazzled_until { return true; }
    let yaw = y0 + v3::clamp(v3::wrap_angle(by - y0), -scan_rate * dt, scan_rate * dt);
    search(g, op).yaw.set(gd, yaw);
    let row = &mut g.tab.orders_assignment[a];
    row.watch = Some([p[0] + m::sin(yaw) * 5.0, 1.1, p[2] + m::cos(yaw) * 5.0]);
    row.face = Some(yaw);
    row.there = String::from("clearing the room");
    true
}

/// how far it is from `a` to `b` on foot; Infinity past `lim` or with no way (doortools.ts DoorTools.walkLen)
pub fn door_tools_walk_len(g: &mut crate::game::Game, a: V3, b: V3, lim: f64) -> f64 {
    let d = flat(a, b);
    if d > lim { return f64::INFINITY; }
    if !crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [b[0], 0.45, b[2]], None) { return d; }
    let path = match crate::scene_collision::nav_grid_find_path(g, a, b, Some(3000.0), None) { Some(p) => p, None => return f64::INFINITY };
    let (mut l, mut q) = (0.0, a);
    for r in path { l += flat(q, r); q = r; }
    l += flat(q, b);
    if l > lim { f64::INFINITY } else { l }
}

/// walkLen for one deal's questions, each pair measured once (doortools.ts DoorTools.walks). The answer is a closure: this file's `Walks` is it.
pub fn door_tools_walks(g: &mut crate::game::Game) -> crate::FnField { todo!("needs: a return type that holds the memo (FnField cannot be called)") }

/// away(): mateOf: the man of `m`'s pair among the squad and `gd`
fn mate_of(g: &mut Game, squad: &[GuardId], gd: GuardId, m: GuardId) -> Option<GuardId> {
    let id = crate::orders::squad_orders_pair_of(g, m)?;
    for &x in squad.iter().chain(core::iter::once(&gd)) {
        if x != m && crate::orders::squad_orders_pair_of(g, x) == Some(id) { return Some(x); }
    }
    None
}

/// away(): reach: one of `who` (or Sam) within `lim` on foot of P
fn reach(g: &mut Game, who: &[GuardId], lim: f64, sam: &[V3], p: V3, where_: &mut dyn FnMut(&mut Game, GuardId) -> V3, len: &mut dyn FnMut(&mut Game, V3, V3) -> f64) -> bool {
    for &m in who {
        let w = where_(g, m);
        if len(g, w, p) <= lim { return true; }
    }
    for &q in sam { if len(g, q, p) <= lim { return true; } }
    false
}

/// away(): strands: nobody who has company now is left without
fn strands(g: &mut Game, squad: &[GuardId], sam: &[V3], gd: GuardId, p: V3, go: &[GuardId], where_: &mut dyn FnMut(&mut Game, GuardId) -> V3, len: &mut dyn FnMut(&mut Game, V3, V3) -> f64) -> bool {
    let left = g.k.doortools.buddy_left_m;
    for &c in squad {
        if go.contains(&c) { continue; }
        let at = where_(g, c);
        let mut stay: Vec<V3> = Vec::new();
        for &m in squad { if m != c && !go.contains(&m) { stay.push(where_(g, m)); } }
        stay.extend_from_slice(sam);
        let mut kept = false;
        for &q in &stay { if len(g, q, at) <= left { kept = true; break; } }
        if kept || len(g, p, at) <= left { continue; }
        let from = pos_of(g, gd);
        if len(g, from, at) <= left { return true; }
        for &m in go {
            let q = if m == gd { pos_of(g, gd) } else { where_(g, m) };
            if len(g, q, at) <= left { return true; }
        }
    }
    false
}

/// away(): lent: his own partner has a door of this room to hold, so he is spare
fn lent(g: &mut Game, squad: &[GuardId], gd: GuardId, free: &[GuardId], m: GuardId, on_door: &mut Option<&mut dyn FnMut(&mut Game, GuardId) -> bool>) -> bool {
    let o = match mate_of(g, squad, gd, m) { Some(o) => o, None => return false };
    if free.contains(&o) { return false; }
    match on_door.as_mut() { Some(f) => f(g, o), None => false }
}

/// the nearest of `list` to `gd`, the first on a tie: `.sort(by flat to him)[0]`
fn nearest_of(g: &Game, list: &[GuardId], gd: GuardId) -> Option<GuardId> {
    let at = pos_of(g, gd);
    let mut l: Vec<(GuardId, f64)> = list.iter().map(|&m| (m, flat(pos_of(g, m), at))).collect();
    js::sort_by(&mut l, |a, b| a.1 - b.1);
    l.first().map(|x| x.0)
}

/// DOORTOOLS.buddy: `gd` would be dealt a walk to P: alone, with this partner, or no. A classifier only (doortools.ts DoorTools.away)
pub fn door_tools_away(g: &mut crate::game::Game, gd: GuardId, p: V3, pool: &[GuardId], where_: &mut dyn FnMut(&mut crate::game::Game, GuardId) -> V3, len: &mut dyn FnMut(&mut crate::game::Game, V3, V3) -> f64, pair: Option<bool>, on_door: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> crate::doortools::DoorToolsAway {
    let pair = pair.unwrap_or(false);
    let mut on_door = on_door;
    let (buddy_m, buddy_pair_m) = (g.k.doortools.buddy_m, g.k.doortools.buddy_pair_m);
    g.door_tools.away_far = false;
    g.door_tools.away_why = String::new();
    let squad: Vec<GuardId> = crate::orders::squad_orders_men(g, None, None).into_iter().filter(|&m| m != gd && alive(g, m)).collect();
    let sam: Vec<V3> = if g.pl[g.player].down { Vec::new() } else { vec![crate::orders::squad_orders_sam_stood(g)] };
    let own = mate_of(g, &squad, gd, gd);
    let rest: Vec<GuardId> = squad.iter().copied().filter(|&m| Some(m) != own).collect();
    let mut mate: Option<GuardId> = None;
    if pair || !reach(g, &squad, buddy_m, &sam, p, where_, len) {
        let free: Vec<GuardId> = pool.iter().copied().filter(|&m| m != gd && alive(g, m)).collect();
        mate = match own {
            Some(o) => {
                if free.contains(&o) { Some(o) }
                else if lent(g, &squad, gd, &free, gd, &mut on_door) {
                    let mut spare: Vec<GuardId> = Vec::new();
                    for &m in &free { if lent(g, &squad, gd, &free, m, &mut on_door) { spare.push(m); } }
                    nearest_of(g, &spare, gd)
                } else { None }
            }
            None => {
                let mut lone: Vec<GuardId> = Vec::new();
                for &m in &free { if mate_of(g, &squad, gd, m).is_none() { lone.push(m); } }
                nearest_of(g, &lone, gd)
            }
        };
        if mate.is_none() {
            g.door_tools.away_why = match own { Some(_) => format!("{}'s partner is busy", g.gd[gd].callsign), None => String::from("no partner free") };
            return if pair && reach(g, &squad, buddy_m, &sam, p, where_, len) { DoorToolsAway::Alone } else { DoorToolsAway::No };
        }
    }
    let base: &[GuardId] = if mate.is_some() && own.is_some() && mate != own { &squad } else { &rest };
    let others: Vec<GuardId> = base.iter().copied().filter(|&m| Some(m) != mate).collect();
    if mate.is_some() && others.is_empty() { g.door_tools.away_why = String::from("nobody would stay behind the two"); return DoorToolsAway::No; }
    if (!others.is_empty() || !sam.is_empty()) && !reach(g, &others, buddy_pair_m, &sam, p, where_, len) {
        g.door_tools.away_why = format!("past {} m from the rest", buddy_pair_m);
        g.door_tools.away_far = true;
        return DoorToolsAway::No;
    }
    let go: Vec<GuardId> = mate.into_iter().collect();
    if strands(g, &squad, &sam, gd, p, &go, where_, len) {
        g.door_tools.away_why = String::from("it would leave a man by himself");
        g.door_tools.away_far = true;
        return DoorToolsAway::No;
    }
    match mate { Some(m) => DoorToolsAway::Guard(m), None => DoorToolsAway::Alone }
}

/// the partner's point beside P, and the longest look he has there off his partner's sector (doortools.ts DoorTools.besidePod)
pub fn door_tools_beside_pod(g: &mut crate::game::Game, p: V3, face: f64, mate: GuardId, taken: &[V3], in_room: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> Option<crate::entrypoints::Pod> {
    let apart = m::max(1.0, g.k.doortools.pod_apart);
    let mate_at = pos_of(g, mate);
    let (mut best, mut bc): (Option<V3>, f64) = (None, f64::INFINITY);
    for r in [1.5, 2.0, 2.5] {
        for k in 0..12 {
            let a = k as f64 * m::PI / 6.0;
            let q: V3 = [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || !in_room(g, q)
                || crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None)
                || taken.iter().any(|&t| flat(t, q) < apart)
                || flat(crate::orders::squad_orders_sam_stood(g), q) < 1.0 { continue; }
            if crate::entrypoints::entry_points_risk_at(g, q, None, None).is_some() { continue; }
            let c = flat(mate_at, q) + (r - 1.5) * 0.5;
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

/// an entry's man sent on to a point of the room it is still looking over (doortools.ts DoorTools.sendOn). `this.assign` written out: `shy: null` over its `shy: op.door`, which DoorToolsAssignA cannot say
pub fn door_tools_send_on(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, pod: &mut crate::entrypoints::Pod, going: &str) {
    let (door, kind) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.kind) };
    g.tab.doortools_door_op[op].pod.set(gd, *pod);
    g.tab.doortools_door_op[op].on_pod.delete(gd);
    let a = crate::orders::SquadOrdersAssignA {
        order: String::from(kind.as_str()),
        door: Some(door),
        post: Some(pod.pos),
        face: Some(pod.face),
        watch: None,
        shy: None,
        thru: Some(door),
        pace: Some(g.orders.knobs.entry),
        stance: Some(crate::character::Stance::None),
        upper: Some(crate::character::UpperMode::Aim),
        light: Some(g.door_tools.lights.unwrap_or(true)),
        search: Some(true),
        going: String::from(going),
        there: String::from("holding the room"),
        ..Default::default()
    };
    crate::orders::squad_orders_assign(g, gd, &a);
    let mine = crate::orders::squad_orders_of(g, gd).unwrap();
    g.tab.doortools_door_op[op].mine.set(gd, mine);
}

/// is this room's floor one the whole-room rule works? A room past the size is said once an order (doortools.ts DoorTools.flows)
pub fn door_tools_flows(g: &mut crate::game::Game, op: DoorOpId, sr: &crate::doortools::RoomSearch) -> bool {
    let flow_max = g.k.doortools.flow_max;
    if !(flow_max > 0.0) || sr.pts.len() as f64 <= flow_max { return true; }
    if !g.door_tools.big_said.has(op) {
        g.door_tools.big_said.add(op);
        door_tools_beat(g, op, &format!("flow: no · a room of {} cells is looked over, not gone through (flowMax {})", sr.pts.len(), flow_max));
    }
    false
}

/// flowIn: cellOf: the point of the room's list on this cell of the nav grid
fn cell_of(g: &mut Game, at: &JsMap<crate::Num, f64>, x: f64, z: f64) -> Option<usize> {
    let cs = g.col.nav.cell;
    let (ix, iz) = (m::floor(x / cs), m::floor(z / cs));
    if ix < 0.0 || iz < 0.0 || ix >= g.col.nav.w { return None; }
    let k = crate::scene_collision::nav_grid_idx(g, ix, iz);
    at.get(crate::Num(k)).map(|&i| i as usize)
}

/// flowIn: faceOf: each man's sector, the middle of the floor of the piece nearer his point than his mate's
fn face_of(pts: &[V3], part: &[usize], mid: V3, p: V3, other: Option<V3>) -> f64 {
    let (mut x, mut z, mut n) = (0.0, 0.0, 0.0);
    for &i in part {
        let q = pts[i];
        if let Some(o) = other { if flat(q, o) < flat(q, p) { continue; } }
        x += q[0]; z += q[2]; n += 1.0;
    }
    if js::truthy(n) { v3::yaw_to(p, [x / n, 0.0, z / n]) } else { v3::yaw_to(p, mid) }
}

/// flowIn: a beat saying which rule said no; false
fn flow_no(g: &mut Game, op: DoorOpId, cells: usize, why: &str) -> bool {
    door_tools_beat(g, op, &format!("flow: no · {} cells · {}", cells, why));
    false
}

#[derive(Clone, Copy)]
struct FlowCand { p: V3, sc: f64, d: f64 }

/// DOORTOOLS.flowOn: the biggest piece of the floor nobody has looked at is gone INTO. True when men were sent (doortools.ts DoorTools.flowIn)
pub fn door_tools_flow_in(g: &mut crate::game::Game, op: DoorOpId, men: &[GuardId]) -> bool {
    let flow_moves = g.k.doortools.flow_moves;
    if search(g, op).flows.unwrap_or(0.0) >= flow_moves { return false; }
    if search(g, op).cell_at.is_none() {
        let cs = g.col.nav.cell;
        let pts = search(g, op).pts.clone();
        let mut at: JsMap<crate::Num, f64> = JsMap::new();
        for i in 0..pts.len() {
            let k = crate::scene_collision::nav_grid_idx(g, m::floor(pts[i][0] / cs), m::floor(pts[i][2] / cs));
            at.set(crate::Num(k), i as f64);
        }
        search(g, op).cell_at = Some(at);
    }
    // (lent out for the body: the closures hand it about while `g` is lent to their callees. Only flowIn ever reads it)
    let at = search(g, op).cell_at.take().unwrap();
    let r = flow_in_body(g, op, men, &at);
    search(g, op).cell_at = Some(at);
    r
}

fn flow_in_body(g: &mut Game, op: DoorOpId, men: &[GuardId], at: &JsMap<crate::Num, f64>) -> bool {
    let cs = g.col.nav.cell;
    let (flow_cells, flow_bound, flow_reach, flow_deep, flow_moves, look_m, pod_apart, buddy_m) = {
        let k = &g.k.doortools;
        (k.flow_cells, k.flow_bound, k.flow_reach, k.flow_deep, k.flow_moves, k.look_m, k.pod_apart, k.buddy_m)
    };
    // (`pts` never changes once made, and nothing in here looks at floor)
    let (pts, looked) = { let s = search(g, op); (s.pts.clone(), s.looked.clone()) };
    let n4 = crate::doortools::n4();
    // the unseen floor in pieces (cells that touch by a side): the biggest
    let mut mark = vec![0u8; pts.len()];
    let mut part: Vec<usize> = Vec::new();
    for i in 0..pts.len() {
        if looked[i] != 0 || mark[i] != 0 { continue; }
        let mut c = vec![i];
        mark[i] = 1;
        let mut h = 0;
        while h < c.len() {
            let p = pts[c[h]];
            for &[dx, dz] in n4.iter() {
                if let Some(j) = cell_of(g, at, p[0] + dx * cs, p[2] + dz * cs) {
                    if looked[j] == 0 && mark[j] == 0 { mark[j] = 1; c.push(j); }
                }
            }
            h += 1;
        }
        if c.len() > part.len() { part = c; }
    }
    if (part.len() as f64) < flow_cells { return false; }
    let of_part = part.len() as f64;
    let mut in_part = vec![0u8; pts.len()];
    let mut mid: V3 = [0.0, 0.0, 0.0];
    for &i in &part { in_part[i] = 1; mid[0] += pts[i][0] / of_part; mid[2] += pts[i][2] / of_part; }
    let (mut goal, mut gb) = (pts[part[0]], f64::INFINITY);
    for &i in &part { let d = flat(pts[i], mid); if d < gb { gb = d; goal = pts[i]; } }
    // its MOUTH: where the way there of the man nearest it first crosses onto floor of the piece
    let ps = core::cmp::max(1, part.len() / 24);
    let mut near: Vec<(GuardId, f64)> = Vec::with_capacity(men.len());
    for &who in men {
        let at_m = pos_of(g, who);
        let (mut b, mut k) = (f64::INFINITY, 0);
        while k < part.len() { b = m::min(b, flat(at_m, pts[part[k]])); k += ps; }
        near.push((who, b));
    }
    js::sort_by(&mut near, |a, b| a.1 - b.1);
    let men: Vec<GuardId> = near.iter().map(|x| x.0).collect();
    let lead = men[0];
    let lead_at = pos_of(g, lead);
    let way = match crate::scene_collision::nav_grid_find_path(g, lead_at, goal, Some(4000.0), None) { Some(w) => w, None => return flow_no(g, op, part.len(), "no way to it") };
    let mut line: Vec<V3> = vec![lead_at];
    line.extend_from_slice(&way);
    line.push(goal);
    let (mut walk, mut cum, mut run): (Vec<V3>, Vec<f64>, f64) = (Vec::new(), Vec::new(), 0.0);
    for k in 1..line.len() {
        let (a, b) = (line[k - 1], line[k]);
        let l = flat(a, b);
        let n = m::max(1.0, m::ceil(l / 0.25));
        let mut j = 1.0;
        while j <= n {
            walk.push([a[0] + (b[0] - a[0]) * j / n, 0.0, a[2] + (b[2] - a[2]) * j / n]);
            cum.push(run + l * j / n);
            j += 1.0;
        }
        run += l;
    }
    let mut mi: Option<usize> = None;
    for k in 0..walk.len() {
        if let Some(j) = cell_of(g, at, walk[k][0], walk[k][2]) { if in_part[j] == 1 { mi = Some(k); break; } }
    }
    let mi = match mi { Some(i) => i, None => return flow_no(g, op, part.len(), "no way to it") };
    let into = cum[mi] <= flow_bound - 1.5;
    let mut ai = mi;
    if !into { ai = 0; while (ai as i64) < mi as i64 - 4 && cum[ai] < flow_bound - 1.5 { ai += 1; } }
    let mouth = walk[ai];
    let (mut ux, mut uz) = (mid[0] - mouth[0], mid[2] - mouth[2]);
    if !into || m::hyp2(ux, uz) < 0.75 || crate::scene_collision::segment_blocked(g, [mouth[0], 1.5, mouth[2]], [mid[0], 1.5, mid[2]], None, None, None) {
        let q = walk[core::cmp::min(walk.len() - 1, ai + 6)];
        ux = q[0] - mouth[0];
        uz = q[2] - mouth[2];
    }
    let ul = m::hyp2(ux, uz);
    if ul < 1e-3 { return flow_no(g, op, part.len(), "no way to it"); }
    ux /= ul;
    uz /= ul;
    // the points: free floor in plain view of the mouth, by a wall where there is one, out of the line of an opening nobody has been beyond
    let sd = core::cmp::max(1, part.len() / 40);
    let sam = crate::orders::squad_orders_sam_stood(g);
    let apart = m::max(1.0, pod_apart);
    // (`here` by man: the TypeScript tells a place from another by the array, `q !== gd.char.pos`)
    let (mut stay, mut here): (Vec<V3>, Vec<(GuardId, V3)>) = (Vec::new(), Vec::new());
    for who in crate::orders::squad_orders_men(g, None, None) {
        if alive(g, who) {
            here.push((who, pos_of(g, who)));
            if !men.contains(&who) { pods_of(g, who, &mut stay); }
        }
    }
    let mut cands: Vec<FlowCand> = Vec::new();
    let mut n0 = 0;
    for i in 0..pts.len() {
        let p = pts[i];
        let (rx, rz) = (p[0] - mouth[0], p[2] - mouth[2]);
        let d = m::hyp2(rx, rz);
        let fwd = rx * ux + rz * uz;
        if if into { d > flow_reach || d < 1.2 || fwd < 0.5 || cum[ai] + d > flow_bound + 1.5 } else { d > 2.75 || fwd < -1.0 } { continue; }
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || flat(sam, p) < 1.0
            || here.iter().any(|h| flat(h.1, p) < 1.5) || stay.iter().any(|&q| flat(q, p) < apart)
            || crate::scene_collision::segment_blocked(g, [mouth[0], 1.5, mouth[2]], [p[0], 1.5, p[2]], None, None, None) { continue; }
        let (mut seen, mut of) = (0.0, 0.0);
        let mut k = 0;
        while k < part.len() {
            let q = pts[part[k]];
            of += 1.0;
            if flat(p, q) <= look_m && !crate::scene_collision::segment_blocked(g, [p[0], 1.55, p[2]], q, None, None, None) { seen += 1.0; }
            k += sd;
        }
        if into && seen * 4.0 < of { continue; }
        n0 += 1;
        let mut w = 0;
        for &[dx, dz] in n4.iter() { if crate::scene_collision::nav_grid_is_blocked(g, p[0] + dx * 0.75, p[2] + dz * 0.75) { w += 1; } }
        let risk = crate::entrypoints::entry_points_risk_at(g, p, None, None).is_some();
        cands.push(FlowCand { p, d, sc: (if into { 10.0 } else { 4.0 }) * seen / of + (if w >= 1 { 2.0 } else { 0.0 }) + (if w >= 2 { 1.0 } else { 0.0 }) - (if into { m::abs(d - flow_deep) } else { d * 0.5 }) - (if risk { 4.0 } else { 0.0 }) });
    }
    if cands.is_empty() { return flow_no(g, op, part.len(), if into { "no point past its mouth" } else { "no point on the way to it" }); }
    js::sort_by(&mut cands, |a, b| b.sc - a.sc);
    cands.truncate(24);
    // the pair of them: apart, to either hand of the way in, each in the other's view
    let (mut two, mut tb): (Option<(V3, V3)>, f64) = (None, f64::NEG_INFINITY);
    for a in 0..cands.len() {
        for b in (a + 1)..cands.len() {
            let (ca, cb) = (cands[a], cands[b]);
            let l = flat(ca.p, cb.p);
            if l < m::max(2.0, apart) { continue; }
            let cos = ((ca.p[0] - mouth[0]) * (cb.p[0] - mouth[0]) + (ca.p[2] - mouth[2]) * (cb.p[2] - mouth[2])) / m::max(1e-3, ca.d * cb.d);
            if into && cos > 0.8 { continue; }
            let sc = ca.sc + cb.sc + m::min(l, 4.0) - (if into { cos } else { 0.0 });
            if sc <= tb || crate::scene_collision::segment_blocked(g, [ca.p[0], 1.5, ca.p[2]], [cb.p[0], 1.5, cb.p[2]], None, None, None) { continue; }
            tb = sc;
            two = Some((ca.p, cb.p));
        }
    }
    let mut walks = Walks::new(g);
    let mut len = |g: &mut Game, a: V3, b: V3| walks.len(g, a, b);
    let mut where_ = |g: &mut Game, who: GuardId| where_of(g, who);
    let mut on_door = |g: &mut Game, who: GuardId| alive(g, who) && g.tab.doortools_door_op[op].men.contains(&who) && !has_over(g, who) && door_tools_on_other(g, op, who);
    let mut in_room = |g: &mut Game, p: V3| cell_of(g, at, p[0], p[2]).is_some();
    let mut whys: Vec<String> = Vec::new();
    let mut far = false;
    for &gd in &men {
        let p1 = match two { Some(t) => t.0, None => cands[0].p };
        let mut mate: Option<GuardId> = None;
        let h = door_tools_away(g, gd, p1, &men, &mut where_, &mut len, Some(true), Some(&mut on_door as &mut dyn FnMut(&mut Game, GuardId) -> bool));
        let why = g.door_tools.away_why.clone();
        let was = g.door_tools.away_far;
        if let DoorToolsAway::Guard(x) = h { mate = Some(x); }
        else if !matches!(door_tools_away(g, gd, p1, &men, &mut where_, &mut len, None, None), DoorToolsAway::Alone) {
            for w in [why, g.door_tools.away_why.clone()] { if !w.is_empty() && !whys.contains(&w) { whys.push(w); } }
            far = far || was || g.door_tools.away_far;
            continue;
        }
        let gd_at = pos_of(g, gd);
        let mut pods: Vec<(GuardId, crate::entrypoints::Pod)> = Vec::new();
        if let Some(mt) = mate {
            let mt_at = pos_of(g, mt);
            let (mut a, mut b): (V3, V3);
            if let Some(t) = two { a = t.0; b = t.1; }
            else {
                let mut taken = stay.clone();
                for h in &here { if h.0 != gd && h.0 != mt { taken.push(h.1); } }
                match door_tools_beside_pod(g, p1, v3::yaw_to(p1, mid), mt, &taken, &mut in_room) {
                    Some(q) => { a = p1; b = q.pos; }
                    None => { let w = String::from("no floor beside the point"); if !whys.contains(&w) { whys.push(w); } continue; }
                }
            }
            if flat(gd_at, a) + flat(mt_at, b) > flat(gd_at, b) + flat(mt_at, a) { core::mem::swap(&mut a, &mut b); }
            pods.push((gd, pod_at([a[0], 0.0, a[2]], face_of(&pts, &part, mid, a, Some(b)))));
            pods.push((mt, pod_at([b[0], 0.0, b[2]], face_of(&pts, &part, mid, b, Some(a)))));
        } else {
            pods.push((gd, pod_at([p1[0], 0.0, p1[2]], face_of(&pts, &part, mid, p1, None))));
        }
        {
            let s = search(g, op);
            s.flows = Some(s.flows.unwrap_or(0.0) + 1.0);
            s.left_at_move = if into { s.left } else { f64::INFINITY };
        }
        for (who, mut pod) in pods {
            crate::entrypoints::entry_points_dealt(g, pod, None);
            search(g, op).yaw.delete(who);
            door_tools_send_on(g, op, who, &mut pod, "clearing the room");
        }
        g.door_tools.stats.sent_on += 1.0;
        g.door_tools.stats.flowed += 1.0;
        if mate.is_some() { g.door_tools.stats.sent_pair += 1.0; }
        if search(g, op).flow_said != Some(true) {
            search(g, op).flow_said = Some(true);
            crate::game::game_say(g, gd, "pushing through.", None, None, None);
        }
        let with = match mate { Some(x) => format!(" with {}", g.gd[x].callsign), None => String::new() };
        let said = format!("{}:{}{} · {} cells · {} points", if into { "flow" } else { "bound" }, g.gd[gd].callsign, with, part.len(), n0);
        door_tools_beat(g, op, &said);
        return true;
    }
    // refused for the walk it would be from the men who stay: THEY CLOSE UP on the man at the front, and the walk is asked again from there
    if far && search(g, op).closes.unwrap_or(0.0) < flow_moves {
        let own = crate::orders::squad_orders_pair_of(g, lead);
        let mut taken: Vec<V3> = Vec::new();
        for who in crate::orders::squad_orders_men(g, None, None) { if alive(g, who) { taken.push(where_(g, who)); } }
        let mut n = 0;
        let mut who: Vec<String> = Vec::new();
        let mut k = 0;
        while k < g.tab.doortools_door_op[op].men.len() {
            let mm = g.tab.doortools_door_op[op].men[k];
            k += 1;
            if mm == lead || !alive(g, mm) || door_tools_in_at(g, op, mm).is_nan() || g.time < g.gd[mm].dazzled_until || has_over(g, mm) { continue; }
            let lead_now = pos_of(g, lead);
            if own.is_some() && crate::orders::squad_orders_pair_of(g, mm) == own {
                let w = where_(g, mm);
                if len(g, w, lead_now) <= buddy_m { continue; }
            }
            let w = where_(g, mm);
            if len(g, w, lead_now) <= 3.5 { continue; }
            let lead_yaw = g.ch[g.gd[lead].ch].aim_yaw;
            let mut pod = match door_tools_beside_pod(g, lead_now, lead_yaw, mm, &taken, &mut in_room) { Some(p) => p, None => continue };
            if let Some(other) = g.tab.doortools_door_op[op].other.as_mut() {
                for o in other.keys() { if other.get(o).map_or(false, |r| r.gd == mm) { other.delete(o); } }
            }
            taken.push(pod.pos);
            crate::entrypoints::entry_points_dealt(g, pod, None);
            search(g, op).yaw.delete(mm);
            door_tools_send_on(g, op, mm, &mut pod, "closing up");
            who.push(g.gd[mm].callsign.clone());
            n += 1;
        }
        if n > 0 {
            let s = search(g, op);
            s.closes = Some(s.closes.unwrap_or(0.0) + 1.0);
            s.left_at_move = f64::INFINITY;
            g.door_tools.stats.closed_up += n as f64;
            let said = format!("close up:{} · on {}", who.join(", "), g.gd[lead].callsign);
            door_tools_beat(g, op, &said);
            return true;
        }
    }
    let why = if whys.is_empty() { String::from("nobody free") } else { whys.join(", ") };
    flow_no(g, op, part.len(), &format!("stay together: {}", why))
}

/// moveOn: what away() ruled for a point (the TypeScript's `mates`, by the point's array: here by its index in SR.pts)
#[derive(Clone, Copy)]
enum PointMate { Refused, Alone, With(GuardId, crate::entrypoints::Pod) }

#[derive(Clone, Copy)]
struct MoveCand { i: usize, p: V3, sc: f64 }

/// floor left that no man can see from where he stands: the man nearest it goes on to the point it is best seen from. True when one was sent (doortools.ts DoorTools.moveOn)
pub fn door_tools_move_on(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let (dead_cells, search_moves, look_m, pod_apart) = { let k = &g.k.doortools; (k.dead_cells, k.search_moves, k.look_m, k.pod_apart) };
    // (flows reads the row while `g` is lent to it, and writes only the order's beats: the row is handed to it for the call)
    let row = g.tab.doortools_door_op[op].search.take().unwrap();
    let flowing = door_tools_flows(g, op, &row);
    g.tab.doortools_door_op[op].search = Some(row);
    let (left, moves, left_at_move) = { let s = search(g, op); (s.left, s.moves, s.left_at_move) };
    if left < dead_cells || (!flowing && moves >= search_moves) { return false; }
    if left > left_at_move - 3.0 { return false; }
    // (`pts` never changes once made)
    let (pts, looked, room) = { let s = search(g, op); (s.pts.clone(), s.looked.clone(), s.room) };
    let mut dead: Vec<V3> = Vec::new();
    for i in 0..pts.len() { if looked[i] == 0 { dead.push(pts[i]); } }
    let mut mid: V3 = [0.0, 0.0, 0.0];
    let of_dead = dead.len() as f64;
    for q in &dead { mid[0] += q[0] / of_dead; mid[2] += q[2] / of_dead; }
    let mut men: Vec<GuardId> = Vec::new();
    for mm in g.tab.doortools_door_op[op].men.clone() {
        if !door_tools_in_at(g, op, mm).is_nan() && g.time >= g.gd[mm].dazzled_until && !has_over(g, mm) && !door_tools_on_other(g, op, mm) { men.push(mm); }
    }
    if men.is_empty() { return false; }
    let mut by: Vec<(GuardId, f64)> = men.iter().map(|&x| (x, flat(pos_of(g, x), mid))).collect();
    js::sort_by(&mut by, |a, b| a.1 - b.1);
    let men: Vec<GuardId> = by.iter().map(|x| x.0).collect();
    if flowing {
        if door_tools_flow_in(g, op, &men) { return true; }
        if search(g, op).moves >= search_moves { return false; }
    }
    let sd = core::cmp::max(1, dead.len() / 40);
    let sp = core::cmp::max(1, pts.len() / 80);
    let cell = g.col.nav.cell;
    let mut in_room = |g: &mut Game, q: V3| {
        let c = crate::scene_collision::nav_grid_idx(g, m::floor(q[0] / cell), m::floor(q[2] / cell));
        g.tab.roomfight_room[room].cells.has(crate::Num(c))
    };
    let mut walks = Walks::new(g);
    let mut len = |g: &mut Game, a: V3, b: V3| walks.len(g, a, b);
    let mut where_ = |g: &mut Game, who: GuardId| where_of(g, who);
    let mut refused = false;
    let mut seen_at: JsMap<usize, f64> = JsMap::new();
    for &gd in &men {
        let from = pos_of(g, gd);
        let mut others: Vec<V3> = Vec::new();
        for who in crate::orders::squad_orders_men(g, None, None) {
            if who != gd { others.push(pos_of(g, who)); pods_of(g, who, &mut others); }
        }
        // every point that shows him enough of the unseen floor, the best first (seen, less the walk)
        let mut cands: Vec<MoveCand> = Vec::new();
        let mut i = 0;
        while i < pts.len() {
            let p = pts[i];
            let here = i;
            i += sp;
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || flat(from, p) < 1.5 || others.iter().any(|&q| flat(q, p) < m::max(1.0, pod_apart)) { continue; }
            let seen = match seen_at.get(here) {
                Some(&s) => s,
                None => {
                    let (mut s, mut k) = (0.0, 0);
                    while k < dead.len() {
                        let q = dead[k];
                        if flat(p, q) <= look_m * 0.8 && !crate::scene_collision::segment_blocked(g, [p[0], 1.55, p[2]], q, None, None, None) { s += 1.0; }
                        k += sd;
                    }
                    seen_at.set(here, s);
                    s
                }
            };
            let sc = seen * sd as f64 - 0.8 * flat(from, p);
            if seen * (sd as f64) < dead_cells / 2.0 || sc <= 0.0 { continue; }
            cands.push(MoveCand { i: here, p, sc });
        }
        js::sort_by(&mut cands, |a, b| b.sc - a.sc);
        // (`clear`: the best of them that is not in the line of an opening nobody has been beyond: entrypoints.ts)
        let mut mates: JsMap<usize, PointMate> = JsMap::new();
        let mut no = false;
        let mut top: Option<MoveCand> = None;
        for c in &cands {
            if move_ok(g, gd, c.i, c.p, mid, &men, &mut mates, &mut no, &mut refused, &mut where_, &mut len, &mut in_room) { top = Some(*c); break; }
        }
        let top = match top { Some(t) => t, None => { if no { continue; } break; } };
        let mut cl: Option<MoveCand> = None;
        for c in &cands {
            if crate::entrypoints::entry_points_risk_at(g, c.p, None, None).is_none()
                && move_ok(g, gd, c.i, c.p, mid, &men, &mut mates, &mut no, &mut refused, &mut where_, &mut len, &mut in_room) { cl = Some(*c); break; }
        }
        let swap = match cl { Some(c) => c.i != top.i && c.sc >= 0.6 * top.sc, None => false };
        let chosen = if swap { cl.unwrap() } else { top };
        let best = chosen.p;
        let mut pod = pod_at([best[0], 0.0, best[2]], v3::yaw_to(best, mid));
        // (swapped takes a copy: the mark it stands for is on the point, read by dealt)
        if swap { pod.moved = true; crate::entrypoints::entry_points_swapped(g, pod); }
        crate::entrypoints::entry_points_dealt(g, pod, None);
        {
            let s = search(g, op);
            s.yaw.delete(gd);
            s.moves += 1.0;
            s.left_at_move = s.left;
        }
        door_tools_send_on(g, op, gd, &mut pod, "clearing the room");
        g.door_tools.stats.sent_on += 1.0;
        if let Some(PointMate::With(x, mut xp)) = mates.get(chosen.i).copied() {
            g.door_tools.stats.sent_pair += 1.0;
            crate::entrypoints::entry_points_dealt(g, xp, None);
            search(g, op).yaw.delete(x);
            door_tools_send_on(g, op, x, &mut xp, "clearing the room");
            let said = format!("on:{} with {}", g.gd[gd].callsign, g.gd[x].callsign);
            door_tools_beat(g, op, &said);
        } else {
            let said = format!("on:{}", g.gd[gd].callsign);
            door_tools_beat(g, op, &said);
        }
        return true;
    }
    if refused { g.door_tools.stats.not_sent += 1.0; }
    false
}

/// moveOn: ok: may `gd` be dealt this point, by away(); each point asked once
fn move_ok(g: &mut Game, gd: GuardId, i: usize, p: V3, mid: V3, men: &[GuardId], mates: &mut JsMap<usize, PointMate>, no: &mut bool, refused: &mut bool, where_: &mut dyn FnMut(&mut Game, GuardId) -> V3, len: &mut dyn FnMut(&mut Game, V3, V3) -> f64, in_room: &mut dyn FnMut(&mut Game, V3) -> bool) -> bool {
    if let Some(known) = mates.get(i) { return !matches!(known, PointMate::Refused); }
    let h = door_tools_away(g, gd, p, men, &mut *where_, &mut *len, None, None);
    let (answer, ok) = match h {
        DoorToolsAway::No => (PointMate::Refused, false),
        DoorToolsAway::Alone => (PointMate::Alone, true),
        DoorToolsAway::Guard(x) => {
            let mut taken: Vec<V3> = Vec::new();
            for who in crate::orders::squad_orders_men(g, None, None) { if who != gd && who != x { taken.push(where_(g, who)); } }
            match door_tools_beside_pod(g, p, v3::yaw_to(p, mid), x, &taken, in_room) {
                Some(pod) => (PointMate::With(x, pod), true),
                None => (PointMate::Refused, false),
            }
        }
    };
    if !ok { *no = true; *refused = true; }
    mates.set(i, answer);
    ok
}

/// the points of domination for a door taken from `side`, with entrypoints.ts's say (doortools.ts DoorTools.pods)
pub fn door_tools_pods(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> {
    let w = g.dr[d].def.width;
    let were = door_tools_pods_as_were(g, d, side, n);
    crate::entrypoints::entry_points_clear_of(g, d, &were, &mut |g: &mut Game, p: V3| [along_of(g, d, p) - w / 2.0, -off_of(g, d, p, side)])
}

/// doorway.ts MARKS (DOORMANNERS.podClear): off on the page, so a point stays where it is (doortools.ts DoorTools.offSwing)
pub fn door_tools_off_swing(g: &mut crate::game::Game, p: &mut crate::doortools::DoorToolsOffSwingP, d: Option<DoorId>, side: f64, taken: &[V3], see: Option<V3>) {}

/// the points before entrypoints.ts looks at them (doortools.ts DoorTools.podsAsWere)
pub fn door_tools_pods_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> {
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
fn pods_fit(g: &mut Game, d: DoorId, mid: V3, p: V3, out: &[crate::entrypoints::Pod]) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2])
        && !crate::scene_collision::solid_blocked(g, mid, [p[0], 0.45, p[2]], None)
        && !out.iter().any(|o| flat(o.pos, p) < 0.9)
        && !door_tools_in_frame(g, d, mid, p)
}

/// DOORTOOLS.podFrames: `p` stands in the opening of a door other than `d`, or the line to it from `mid` passes through one (doortools.ts DoorTools.inFrame)
pub fn door_tools_in_frame(g: &mut crate::game::Game, d: DoorId, mid: V3, p: V3) -> bool {
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

/// the two deep points: the level's clear plan when authored from that face, else the first free floor to either hand (doortools.ts DoorTools.pods2)
pub fn door_tools_pods2(g: &mut crate::game::Game, d: DoorId, side: f64) -> Vec<crate::entrypoints::Pod> {
    let w = g.dr[d].def.width;
    let into = -side;
    let name = g.dr[d].def.name.clone();
    if let Some(plan) = crate::roomclear::door_plan(g, &name) {
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
            if !out.iter().any(|o| flat(o.pos, f) < 0.6) { out.push(pod_at([f[0], 0.0, f[2]], inward)); }
        }
    }
    out
}

/// enter: the queue is written, its points dealt, and the word goes
fn enter_go(g: &mut Game, op: DoorOpId, t: f64, queue: Vec<DoorToolsDoneItemQueueItem>) {
    let door = g.tab.doortools_door_op[op].door;
    g.tab.doortools_door_op[op].queue = queue.clone();
    for q in &queue {
        crate::entrypoints::entry_points_dealt(g, q.pod, Some(door));
    }
    g.tab.doortools_door_op[op].phase = Phase::Enter;
    g.tab.doortools_door_op[op].t0 = t;
    door_tools_beat(g, op, "go");
    let lead = g.tab.doortools_door_op[op].lead;
    crate::game::game_say(g, lead, "going in.", None, None, None);
}

/// the men go in: the lead first, the next `second` later, each to his point of domination (doortools.ts DoorTools.enter)
pub fn door_tools_enter(g: &mut crate::game::Game, op: DoorOpId) {
    if g.tab.doortools_door_op[op].thr.is_some() { door_tools_step_in(g, op); return; }
    let t = g.time;
    let second = g.k.doortools.second;
    let (door, side, lead) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side, o.lead) };
    let n_men = g.tab.doortools_door_op[op].men.len();
    let big = crate::orders::squad_orders_big(g);
    let pods = door_tools_pods(g, door, side, Some(if n_men > 2 || big { 4.0 } else { n_men as f64 }));
    if pods.is_empty() {
        crate::game::game_say(g, lead, "no way in.", None, None, None);
        door_tools_end(g, op, "no floor inside");
        return;
    }
    let first = door_tools_first_in(g, op, Some(pods[0].pos), None);
    g.door_tools.led.entries += 1.0;
    if first != lead && crate::gunroles::gun_before(g, first, lead) && door_tools_first_in(g, op, Some(pods[0].pos), Some(false)) != first {
        let said = format!("first:{} · his gun is the room's", g.gd[first].callsign);
        door_tools_beat(g, op, &said);
    } else if first != lead {
        g.door_tools.led.by_seen += 1.0;
        let said = format!("first:{} · has seen most of the room", g.gd[first].callsign);
        door_tools_beat(g, op, &said);
    }
    let mut men: Vec<GuardId> = vec![first];
    men.extend(g.tab.doortools_door_op[op].men.iter().copied().filter(|&m| m != first));
    if men.len() > 2 {
        // THREE OR MORE: in turn off alternate jambs while both files have men; a man with no point keeps his place on the stack, and is not this order's any more
        let w = g.dr[door].def.width;
        let centre = g.dr[door].frame_centre;
        let mut files: Vec<(GuardId, bool, f64)> = Vec::with_capacity(men.len());
        for &m in &men {
            let at = g.tab.doortools_door_op[op].post.get(m).map(|p| p.pos).unwrap_or_else(|| pos_of(g, m));
            files.push((m, along_of(g, door, at) > w / 2.0, flat(at, centre)));
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
        let queue: Vec<DoorToolsDoneItemQueueItem> = in_men.iter().enumerate().map(|(i, &gd)| {
            DoorToolsDoneItemQueueItem { gd, at: t + i as f64 * second, pod: deal.get(gd).copied().unwrap_or(pods[0]) }
        }).collect();
        enter_go(g, op, t, queue);
        return;
    }
    let own: Vec<crate::entrypoints::Pod> = if big { door_tools_pods_apart(g, op, &pods, men.len() as f64) } else { pods };
    let spots: Vec<V3> = own.iter().take(men.len()).map(|p| p.pos).collect();
    let pairs = crate::orders::squad_orders_pair(g, &men, &spots, None, None);
    let queue: Vec<DoorToolsDoneItemQueueItem> = men.iter().enumerate().map(|(i, &gd)| {
        let to = pairs.iter().find(|x| x.0 == gd).unwrap().1;
        DoorToolsDoneItemQueueItem { gd, at: t + i as f64 * second, pod: own.iter().find(|p| p.pos == to).copied().unwrap_or(own[0]) }
    }).collect();
    enter_go(g, op, t, queue);
}

/// THE SQUAD OF FOUR, a pair going in: its points kept `podApart` clear of the other order's men and of any teammate posted inside (doortools.ts DoorTools.podsApart)
pub fn door_tools_pods_apart(g: &mut crate::game::Game, op: DoorOpId, pods: &[crate::entrypoints::Pod], n: f64) -> Vec<crate::entrypoints::Pod> {
    let pod_apart = g.k.doortools.pod_apart;
    let (door, side) = { let o = &g.tab.doortools_door_op[op]; (o.door, o.side) };
    let mut taken: Vec<V3> = Vec::new();
    for &o in g.door_tools.ops.iter() {
        if o == op { continue; }
        for p in g.tab.doortools_door_op[o].pod.values() { taken.push(p.pos); }
        for q in g.tab.doortools_door_op[o].queue.iter() { taken.push(q.pod.pos); }
    }
    for gd in crate::orders::squad_orders_men(g, None, None) {
        if g.tab.doortools_door_op[op].men.contains(&gd) { continue; }
        if let Some(a) = crate::orders::squad_orders_of(g, gd) {
            let (post, arrived) = (g.tab.orders_assignment[a].post, g.tab.orders_assignment[a].arrived);
            if let Some(post) = post { if arrived && off_of(g, door, post, side) < 0.0 { taken.push(post); } }
        }
    }
    // (by index: the TypeScript tells a point from another by the object)
    let clear: Vec<usize> = (0..pods.len()).filter(|&i| !taken.iter().any(|&q| flat(q, pods[i].pos) < pod_apart)).collect();
    let first = if n < pods.len() as f64 { n as usize } else { pods.len() };
    if clear.len() < first || clear.iter().take(n as usize).enumerate().any(|(k, &i)| i != k) { g.door_tools.stats.pods_moved += 1.0; }
    if clear.len() as f64 >= n { return clear.iter().map(|&i| pods[i]).collect(); }
    let mut out: Vec<crate::entrypoints::Pod> = clear.iter().map(|&i| pods[i]).collect();
    for i in 0..pods.len() { if !clear.contains(&i) { out.push(pods[i]); } }
    out
}

/// Sam's trigger: the set charge nearest him (or the one given) goes; what it did, null with none set (doortools.ts DoorTools.blow)
pub fn door_tools_blow(g: &mut crate::game::Game, c_2: Option<ChargeId>) -> Option<crate::doortools::DoorToolsBlow> {
    let c = match c_2 { Some(c) => Some(c), None => door_tools_nearest(g, None) };
    let who = g.ch[g.pl[g.player].ch].id;
    let r = crate::fogofwar::fog_of_war_doing(g, Some(who), &mut |g: &mut Game| door_tools_blow_raw(g, c));
    r.map(|x| DoorToolsBlow { knocked: x.knocked, stunned: x.stunned, backblast: x.backblast })
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own strings: node, (x).toFixed(2)
    #[test]
    fn fixed2_is_to_fixed() {
        for (x, s) in [(0.125, "0.13"), (-0.125, "-0.13"), (0.375, "0.38"), (1.125, "1.13"), (1.005, "1.00"), (2.5, "2.50"), (-0.0, "0.00"), (-0.001, "-0.00"), (12.345, "12.35"), (0.0, "0.00"), (7.0625, "7.06")] {
            assert_eq!(fixed2(x), s);
        }
    }
}
