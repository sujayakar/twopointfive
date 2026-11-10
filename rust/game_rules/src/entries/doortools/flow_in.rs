//! THE BIGGEST PIECE OF A ROOM'S FLOOR NOBODY HAS LOOKED AT IS GONE INTO, by a pair where one can go, the man nearest it first; and where it would be too far from the rest, they close up on him.
use super::*;
use super::flow::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap};
use crate::m;
use crate::v3::{self, dist_xz as flat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// The search it is of, its floor's points and which point is on a cell of the nav grid, and the piece: its points and their middle.
struct Piece { sr: DoortoolsRoomSearchId, pts: Vec<V3>, at: JsMap<crate::Num, f64>, part: Vec<usize>, mid: V3 }
/// Where the piece is gone in by: a place on the way there, the way on from it, how far on foot it is, and whether it is at the piece itself (`into`) or only on the way to it.
#[derive(Clone, Copy)]
struct Mouth { at: V3, way: [f64; 2], far: f64, into: bool }
/// A point to stand at: what it is worth, and how far from the mouth.
#[derive(Clone, Copy)]
struct Point { p: V3, sc: f64, d: f64 }

impl Piece {
    /// the point of the list on this cell of the nav grid
    fn cell(&self, g: &Game, x: f64, z: f64) -> Option<usize> { cell_of(g, &self.at, x, z) }
    /// each man's sector: the middle of the floor of the piece nearer his point than his mate's
    fn face(&self, p: V3, other: Option<V3>) -> f64 {
        let (mut x, mut z, mut n) = (0.0, 0.0, 0.0);
        for &i in &self.part {
            let q = self.pts[i];
            if other.is_some_and(|o| flat(q, o) < flat(q, p)) { continue; }
            x += q[0]; z += q[2]; n += 1.0;
        }
        v3::yaw_to(p, if n > 0.0 { [x / n, 0.0, z / n] } else { self.mid })
    }
    fn pod(&self, p: V3, other: Option<V3>) -> crate::entrypoints::Pod { pod_at([p[0], 0.0, p[2]], self.face(p, other)) }
}

/// The biggest unseen piece is gone into. True when men were sent.
pub(super) fn door_tools_flow_in(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, men: &[GuardId]) -> bool {
    if search(g, sr).flows.unwrap_or(0.0) >= g.k.doortools.flow_moves { return false; }
    let mut f = Piece { sr, pts: search(g, sr).pts.clone(), at: cells_of(g, sr), part: Vec::new(), mid: [0.0; 3] };
    let sent = piece_gone_into(g, op, &mut f, men);
    search(g, sr).cell_at = Some(f.at);
    sent
}
fn piece_gone_into(g: &mut Game, op: DoorOpId, f: &mut Piece, men: &[GuardId]) -> bool {
    f.part = biggest_unseen(g, f);
    if (f.part.len() as f64) < g.k.doortools.flow_cells { return false; }
    let n = f.part.len() as f64;
    for &i in &f.part { f.mid[0] += f.pts[i][0] / n; f.mid[2] += f.pts[i][2] / n; }
    let men = nearest_to(g, f, men);
    let sent = match mouth_of(g, f, men[0]) {
        None => Err(String::from("no way to it")),
        Some(mouth) => go_in(g, op, f, mouth, &men),
    };
    match sent {
        Ok(said) => { door_tools_beat(g, op, &said); true }
        Err(why) => { door_tools_beat(g, op, &format!("flow: no · {} cells · {}", f.part.len(), why)); false }
    }
}

/// Which point of the search's list is on a cell, worked out once a search and lent out of it for the question.
fn cells_of(g: &mut Game, sr: DoortoolsRoomSearchId) -> JsMap<crate::Num, f64> {
    if let Some(at) = search(g, sr).cell_at.take() { return at; }
    let cs = g.col.nav.cell;
    let mut at: JsMap<crate::Num, f64> = JsMap::new();
    for (i, p) in g.tab.doortools_room_search[sr].pts.iter().enumerate() {
        at.set(crate::Num(crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cs), m::floor(p[2] / cs))), i as f64);
    }
    at
}

/// The unseen floor in pieces, cells that touch by a side: the biggest.
fn biggest_unseen(g: &mut Game, f: &Piece) -> Vec<usize> {
    let cs = g.col.nav.cell;
    let n4 = crate::doortools::n4();
    let looked = search(g, f.sr).looked.clone();
    let mut mark = vec![false; f.pts.len()];
    let mut part: Vec<usize> = Vec::new();
    for i in 0..f.pts.len() {
        if looked[i] != 0 || mark[i] { continue; }
        let mut c = vec![i];
        mark[i] = true;
        let mut h = 0;
        while h < c.len() {
            let p = f.pts[c[h]];
            for &[dx, dz] in n4.iter() {
                if let Some(j) = f.cell(g, p[0] + dx * cs, p[2] + dz * cs) { if looked[j] == 0 && !mark[j] { mark[j] = true; c.push(j); } }
            }
            h += 1;
        }
        if c.len() > part.len() { part = c; }
    }
    part
}

/// The men by how near they are to the piece.
fn nearest_to(g: &Game, f: &Piece, men: &[GuardId]) -> Vec<GuardId> {
    let step = core::cmp::max(1, f.part.len() / 24);
    let mut by: Vec<(GuardId, f64)> = men.iter().map(|&who| { let at = pos_of(g, who); (who, f.part.iter().step_by(step).fold(f64::INFINITY, |b, &i| m::min(b, flat(at, f.pts[i])))) }).collect();
    js::sort_by(&mut by, |a, b| a.1 - b.1);
    by.into_iter().map(|x| x.0).collect()
}

/// Its MOUTH: where the way there of the man nearest it first crosses onto floor of the piece; where that is too far for one bound, as far along it as a bound goes.
fn mouth_of(g: &mut Game, f: &Piece, lead: GuardId) -> Option<Mouth> {
    let bound = g.k.doortools.flow_bound;
    let mut goal = f.pts[f.part[0]];
    for &i in &f.part { if flat(f.pts[i], f.mid) < flat(goal, f.mid) { goal = f.pts[i]; } }
    let from = pos_of(g, lead);
    let way = crate::scene_collision::nav_grid_find_path(g, from, goal, Some(4000.0), None)?;
    let walk = quarter_steps(from, &way, goal);
    let mut inside = vec![false; f.pts.len()];
    for &i in &f.part { inside[i] = true; }
    let on = walk.iter().position(|w| f.cell(g, w.0[0], w.0[2]).is_some_and(|j| inside[j]))?;
    let into = walk[on].1 <= bound - 1.5;
    let mut k = on;
    if !into { k = 0; while k + 4 < on && walk[k].1 < bound - 1.5 { k += 1; } }
    let (at, far) = walk[k];
    let mut way = [f.mid[0] - at[0], f.mid[2] - at[2]];
    if !into || m::hyp2(way[0], way[1]) < 0.75 || crate::scene_collision::segment_blocked(g, [at[0], 1.5, at[2]], [f.mid[0], 1.5, f.mid[2]], None, None, None) {
        let q = walk[core::cmp::min(walk.len() - 1, k + 6)].0;
        way = [q[0] - at[0], q[2] - at[2]];
    }
    let l = m::hyp2(way[0], way[1]);
    if l < 1e-3 { return None; }
    Some(Mouth { at, way: [way[0] / l, way[1] / l], far, into })
}
/// A walk as places a quarter of a metre apart, each with how far along it is.
fn quarter_steps(from: V3, way: &[V3], to: V3) -> Vec<(V3, f64)> {
    let (mut out, mut run, mut a) = (Vec::new(), 0.0, from);
    for &b in way.iter().chain([&to]) {
        let l = flat(a, b);
        let n = m::max(1.0, m::ceil(l / 0.25));
        let mut j = 1.0;
        while j <= n {
            out.push(([a[0] + (b[0] - a[0]) * j / n, 0.0, a[2] + (b[2] - a[2]) * j / n], run + l * j / n));
            j += 1.0;
        }
        run += l;
        a = b;
    }
    out
}

/// The men on their feet and where each is, and the places the orders have those going who are not of this move.
struct Squad { here: Vec<(GuardId, V3)>, stay: Vec<V3> }

/// Men go in by the mouth. What is said of it, or why none did.
fn go_in(g: &mut Game, op: DoorOpId, f: &Piece, mouth: Mouth, men: &[GuardId]) -> Result<String, String> {
    let mut squad = Squad { here: Vec::new(), stay: Vec::new() };
    for who in crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None) {
        if crate::condition::down(g, g.gd[who].ch) { continue; }
        squad.here.push((who, pos_of(g, who)));
        if !men.contains(&who) { pods_of(g, &crate::seats::here(g), who, &mut squad.stay); }
    }
    let points = points_past(g, f, mouth, &squad);
    let Some(first) = points.first() else { return Err(String::from(if mouth.into { "no point past its mouth" } else { "no point on the way to it" })) };
    let two = pair_of_points(g, &points[..points.len().min(24)], mouth);
    let mut walks = Walks::of(g);
    let mut whys: Vec<String> = Vec::new();
    let mut far = false;
    for &gd in men {
        match dealt(g, op, f, &squad, gd, men, first.p, two, &mut walks) {
            Ok(pods) => return Ok(sent_in(g, op, f, mouth, pods, points.len())),
            Err(refused) => {
                for w in refused.whys { if !w.is_empty() && !whys.contains(&w) { whys.push(w); } }
                far = far || refused.far;
            }
        }
    }
    if far && search(g, f.sr).closes.unwrap_or(0.0) < g.k.doortools.flow_moves {
        if let Some(said) = close_up(g, op, f, men[0], &mut walks) { return Ok(said); }
    }
    Err(format!("stay together: {}", if whys.is_empty() { String::from("nobody free") } else { whys.join(", ") }))
}

/// The points, the best first: free floor in plain view of the mouth, by a wall where there is one, out of the line of an opening nobody has been beyond.
fn points_past(g: &mut Game, f: &Piece, mouth: Mouth, squad: &Squad) -> Vec<Point> {
    let k = &g.k.doortools;
    let (bound, reach, deep, look_m, apart) = (k.flow_bound, k.flow_reach, k.flow_deep, k.look_m, m::max(1.0, k.pod_apart));
    let step = core::cmp::max(1, f.part.len() / 40);
    let sam = crate::orders::squad_orders_sam_stood(g, &crate::seats::here(g));
    let eye: V3 = [mouth.at[0], 1.5, mouth.at[2]];
    let into = mouth.into;
    let mut out: Vec<Point> = Vec::new();
    for &p in &f.pts {
        let (x, z) = (p[0] - mouth.at[0], p[2] - mouth.at[2]);
        let (d, fwd) = (m::hyp2(x, z), x * mouth.way[0] + z * mouth.way[1]);
        if if into { d > reach || d < 1.2 || fwd < 0.5 || mouth.far + d > bound + 1.5 } else { d > 2.75 || fwd < -1.0 } { continue; }
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || flat(sam, p) < 1.0
            || squad.here.iter().any(|h| flat(h.1, p) < 1.5) || squad.stay.iter().any(|&q| flat(q, p) < apart)
            || crate::scene_collision::segment_blocked(g, eye, [p[0], 1.5, p[2]], None, None, None) { continue; }
        let (mut seen, mut of) = (0.0, 0.0);
        for &i in f.part.iter().step_by(step) {
            of += 1.0;
            if flat(p, f.pts[i]) <= look_m && !crate::scene_collision::segment_blocked(g, [p[0], 1.55, p[2]], f.pts[i], None, None, None) { seen += 1.0; }
        }
        if into && seen * 4.0 < of { continue; }
        let walls = crate::doortools::n4().iter().filter(|&&[dx, dz]| crate::scene_collision::nav_grid_is_blocked(g, p[0] + dx * 0.75, p[2] + dz * 0.75)).count();
        let risk = crate::entrypoints::entry_points_risk_at(g, p, None, None).is_some();
        let sc = (if into { 10.0 } else { 4.0 }) * seen / of + (if walls >= 1 { 2.0 } else { 0.0 }) + (if walls >= 2 { 1.0 } else { 0.0 }) - (if into { m::abs(d - deep) } else { d * 0.5 }) - (if risk { 4.0 } else { 0.0 });
        out.push(Point { p, sc, d });
    }
    js::sort_by(&mut out, |a, b| b.sc - a.sc);
    out
}

/// The pair of them: apart, to either hand of the way in, each in the other's view.
fn pair_of_points(g: &Game, points: &[Point], mouth: Mouth) -> Option<(V3, V3)> {
    let least = m::max(2.0, g.k.doortools.pod_apart);
    let (mut two, mut best) = (None, f64::NEG_INFINITY);
    for (k, a) in points.iter().enumerate() {
        for b in &points[k + 1..] {
            let l = flat(a.p, b.p);
            if l < least { continue; }
            let cos = ((a.p[0] - mouth.at[0]) * (b.p[0] - mouth.at[0]) + (a.p[2] - mouth.at[2]) * (b.p[2] - mouth.at[2])) / m::max(1e-3, a.d * b.d);
            if mouth.into && cos > 0.8 { continue; }
            let sc = a.sc + b.sc + m::min(l, 4.0) - (if mouth.into { cos } else { 0.0 });
            if sc <= best || crate::scene_collision::segment_blocked(g, [a.p[0], 1.5, a.p[2]], [b.p[0], 1.5, b.p[2]], None, None, None) { continue; }
            best = sc;
            two = Some((a.p, b.p));
        }
    }
    two
}

/// Why a man could not be dealt it, and whether it was for the walk it would be from the men who stay.
struct Refused { whys: Vec<String>, far: bool }
/// The points dealt to `gd`, and to the man who goes with him: by himself only where nobody would have to go along.
fn dealt(g: &mut Game, op: DoorOpId, f: &Piece, squad: &Squad, gd: GuardId, men: &[GuardId], first: V3, two: Option<(V3, V3)>, walks: &mut Walks) -> Result<Vec<(GuardId, crate::entrypoints::Pod)>, Refused> {
    let p = two.map_or(first, |t| t.0);
    let said = |g: &Game| { let d = &g.sets[g.seat].door_tools; (d.away_why.clone(), d.away_far) };
    let mate = match door_tools_away(g, gd, p, men, walks, Some(op)) {
        DoorToolsAway::Guard(x) => x,
        _ => {
            let (why, far) = said(g);
            if matches!(door_tools_away(g, gd, p, men, walks, None), DoorToolsAway::Alone) { return Ok(vec![(gd, f.pod(p, None))]); }
            let again = said(g);
            return Err(Refused { whys: vec![why, again.0], far: far || again.1 });
        }
    };
    let (mut a, mut b) = match two {
        Some(t) => t,
        None => {
            let taken: Vec<V3> = squad.stay.iter().copied().chain(squad.here.iter().filter(|h| h.0 != gd && h.0 != mate).map(|h| h.1)).collect();
            match door_tools_beside_pod(g, p, v3::yaw_to(p, f.mid), mate, &taken, &|g, q| f.cell(g, q[0], q[2]).is_some()) {
                Some(q) => (p, q.pos),
                None => return Err(Refused { whys: vec![String::from("no floor beside the point")], far: false }),
            }
        }
    };
    let (his, other) = (pos_of(g, gd), pos_of(g, mate));
    if flat(his, a) + flat(other, b) > flat(his, b) + flat(other, a) { core::mem::swap(&mut a, &mut b); }
    Ok(vec![(gd, f.pod(a, Some(b))), (mate, f.pod(b, Some(a)))])
}

/// They go. What is said of it.
fn sent_in(g: &mut Game, op: DoorOpId, f: &Piece, mouth: Mouth, pods: Vec<(GuardId, crate::entrypoints::Pod)>, points: usize) -> String {
    let s = search(g, f.sr);
    s.flows = Some(s.flows.unwrap_or(0.0) + 1.0);
    s.left_at_move = if mouth.into { s.left } else { f64::INFINITY };
    let names: Vec<String> = pods.iter().map(|x| g.gd[x.0].callsign.clone()).collect();
    let gd = pods[0].0;
    for (who, mut pod) in pods { search_moves(g, op, f.sr, who, &mut pod, "clearing the room"); }
    if search(g, f.sr).flow_said != Some(true) {
        search(g, f.sr).flow_said = Some(true);
        crate::game::game_say(g, gd, "pushing through.", None, None);
    }
    format!("{}:{} · {} cells · {} points", if mouth.into { "flow" } else { "bound" }, names.join(" with "), f.part.len(), points)
}

/// Refused for the walk it would be from the men who stay: THEY CLOSE UP on the man at the front, and the walk is asked again from there. What is said of it; None: nobody came.
fn close_up(g: &mut Game, op: DoorOpId, f: &Piece, lead: GuardId, walks: &mut Walks) -> Option<String> {
    let own = crate::orders::squad_orders_pair_of(g, lead);
    let mut taken: Vec<V3> = crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().filter(|&who| !crate::condition::down(g, g.gd[who].ch)).map(|who| where_of(g, who)).collect();
    let mut came: Vec<String> = Vec::new();
    for mm in g.tab.doortools_door_op[op].men.clone() {
        if mm == lead || crate::condition::down(g, g.gd[mm].ch) || door_tools_in_at(g, op, mm).is_nan() || crate::control::man::blind(g, mm) || has_over(g, mm) { continue; }
        let (to, at) = (pos_of(g, lead), where_of(g, mm));
        let his_pair = own.is_some() && crate::orders::squad_orders_pair_of(g, mm) == own;
        if (his_pair && walks.len(g, at, to) <= g.k.doortools.buddy_m) || walks.len(g, at, to) <= 3.5 { continue; }
        let face = g.ch[g.gd[lead].ch].aim_yaw;
        let Some(mut pod) = door_tools_beside_pod(g, to, face, mm, &taken, &|g, q| f.cell(g, q[0], q[2]).is_some()) else { continue };
        if let Some(other) = g.tab.doortools_door_op[op].other.as_mut() {
            for o in other.keys() { if other.get(o).is_some_and(|r| r.gd == mm) { other.delete(o); } }
        }
        taken.push(pod.pos);
        search_moves(g, op, f.sr, mm, &mut pod, "closing up");
        came.push(g.gd[mm].callsign.clone());
    }
    if came.is_empty() { return None; }
    let s = search(g, f.sr);
    s.closes = Some(s.closes.unwrap_or(0.0) + 1.0);
    s.left_at_move = f64::INFINITY;
    Some(format!("close up:{} · on {}", came.join(", "), g.gd[lead].callsign))
}
