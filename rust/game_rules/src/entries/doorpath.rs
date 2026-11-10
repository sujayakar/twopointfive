pub use ::game_types::doorpath::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec, vec::Vec};

/// A door blown off its hinges: no leaf to walk round.
fn gone(g: &mut crate::game::Game, d: DoorId) -> bool {
    g.dr[d].leaf.off
}

pub(crate) fn cross(ox: f64, oz: f64, px: f64, pz: f64, qx: f64, qz: f64) -> f64 {
    (px - ox) * (qz - oz) - (pz - oz) * (qx - ox)
}

/// The least distance between segments a→b and c→d on the floor.
fn seg_seg(a: V3, b: V3, c_2: V3, d: V3) -> f64 {
    let c = c_2;
    let d1 = cross(c[0], c[2], d[0], d[2], a[0], a[2]);
    let d2 = cross(c[0], c[2], d[0], d[2], b[0], b[2]);
    let d3 = cross(a[0], a[2], b[0], b[2], c[0], c[2]);
    let d4 = cross(a[0], a[2], b[0], b[2], d[0], d[2]);
    if d1 * d2 < 0.0 && d3 * d4 < 0.0 {
        return 0.0;
    }
    m::min_n(&[pt_seg(a, c, d), pt_seg(b, c, d), pt_seg(c, a, b), pt_seg(d, a, b)])
}

/// The least distance from p to segment a→b on the floor.
fn pt_seg(p: V3, a: V3, b: V3) -> f64 {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let l2 = dx * dx + dz * dz;
    let t = if l2 > 1e-9 { m::max(0.0, m::min(1.0, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) } else { 0.0 };
    m::hyp2(p[0] - a[0] - dx * t, p[2] - a[2] - dz * t)
}

/// The leaf as it stands: hinge → free tip.
fn door_path_leaf(g: &crate::game::Game, d: DoorId) -> [V3; 2] {
    let u = crate::doors::door_dir(g, d);
    let h = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    [[h[0], 0.0, h[1]], [h[0] + u[0] * w, 0.0, h[1] + u[1] * w]]
}

/// Every leaf standing within 2.5 m of the segment near→far.
fn door_path_leaves(g: &mut crate::game::Game, near: V3, far: V3) -> Vec<[V3; 2]> {
    let mut out: Vec<[V3; 2]> = Vec::new();
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        if gone(g, d) {
            continue;
        }
        let l = door_path_leaf(g, d);
        if seg_seg(near, far, l[0], l[1]) < 2.5 {
            out.push(l);
        }
    }
    out
}

/// A man can stand here: clear of every static at his own width.
fn door_path_free(g: &crate::game::Game, p: V3) -> bool {
    let mut q: V3 = [p[0], 0.0, p[2]];
    crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, Some(3.0));
    v3::dist_xz(q, p) < 0.03
}

/// A man's width of open floor from a to b.
fn door_path_open(g: &crate::game::Game, a: V3, b: V3) -> bool {
    if crate::scene_collision::low_blocked(g, a, b) {
        return false;
    }
    let d = v3::dist_xz(a, b);
    let n = m::floor(d / 0.3);
    let mut i = 1.0;
    while i < n {
        let k = i / n;
        if !door_path_free(g, [a[0] + (b[0] - a[0]) * k, 0.0, a[2] + (b[2] - a[2]) * k]) {
            return false;
        }
        i += 1.0;
    }
    true
}

/// How far this point stands off the nearest of these leaves.
fn off_leaves(p: V3, l: &[[V3; 2]]) -> f64 {
    let mut least = m::INF;
    for x in l {
        least = m::min(least, pt_seg(p, x[0], x[1]));
    }
    least
}

/// This leg keeps the clearance, or as much of it as its ends have.
fn leg_ok(p: V3, q: V3, dp: f64, dq: f64, c: f64, l: &[[V3; 2]]) -> bool {
    let need = m::max(0.05, m::min_n(&[c, dp, dq]) - 0.02);
    for x in l {
        if seg_seg(p, q, x[0], x[1]) < need {
            return false;
        }
    }
    true
}

/// From a to b keeping `leafClear` off every leaf nearby: the corners to walk (b last), or None when no way round was found.
fn door_path_round_leaves(g: &mut crate::game::Game, a: V3, b: V3) -> Option<crate::doorpath::DoorPathRoundLeaves> {
    let k = g.k.doorpath;
    let c = k.leaf_clear;
    let l = door_path_leaves(g, a, b);
    let da = off_leaves(a, &l);
    let db = off_leaves(b, &l);
    if leg_ok(a, b, da, db, c, &l) {
        return Some(crate::doorpath::DoorPathRoundLeaves { pts: vec![b], round: false });
    }
    // the corners: an arc round each leaf's free tip
    let rr = k.tip_m;
    let mut nodes: Vec<V3> = vec![a, b];
    let mut dist: Vec<f64> = vec![da, db];
    for x in l.iter() {
        let th = m::atan2(x[1][2] - x[0][2], x[1][0] - x[0][0]);
        let mut kk = -3i32;
        while kk <= 3 {
            let ang = th + (kk as f64) * m::PI / 4.0;
            let p: V3 = [x[1][0] + m::cos(ang) * rr, 0.0, x[1][2] + m::sin(ang) * rr];
            if door_path_free(g, p) {
                nodes.push(p);
                dist.push(off_leaves(p, &l));
            }
            kk += 1;
        }
    }
    // the shortest way over them (a dozen points: the plain search)
    let n = nodes.len();
    let mut best = vec![m::INF; n];
    let mut from = vec![-1isize; n];
    let mut done = vec![false; n];
    best[0] = 0.0;
    loop {
        let mut i: isize = -1;
        for kk in 0..n {
            if !done[kk] && best[kk] < m::INF && (i < 0 || best[kk] < best[i as usize]) {
                i = kk as isize;
            }
        }
        if i < 0 || i == 1 {
            break;
        }
        let iu = i as usize;
        done[iu] = true;
        for j in 1..n {
            if done[j] {
                continue;
            }
            let w = best[iu] + v3::dist_xz(nodes[iu], nodes[j]);
            if w >= best[j] {
                continue;
            }
            if !leg_ok(nodes[iu], nodes[j], dist[iu], dist[j], c, &l) || !door_path_open(g, nodes[iu], nodes[j]) {
                continue;
            }
            best[j] = w;
            from[j] = i;
        }
    }
    if best[1] == m::INF {
        return None;
    }
    let mut pts: Vec<V3> = Vec::new();
    let mut i: isize = 1;
    while i > 0 {
        pts.insert(0, nodes[i as usize]);
        i = from[i as usize];
    }
    Some(crate::doorpath::DoorPathRoundLeaves { pts, round: true })
}

/// From a to b for a man's feet: clear of the leaves and of the level, by the nav grid's path where the straight leg is not open.
fn door_path_way(g: &mut crate::game::Game, a: V3, b: V3) -> Option<crate::doorpath::DoorPathWay> {
    let r = door_path_round_leaves(g, a, b)?;
    let mut prev = a;
    let mut clear = true;
    for &q in r.pts.iter() {
        if !door_path_open(g, prev, q) {
            clear = false;
            break;
        }
        prev = q;
    }
    let plain = crate::doorpath::DoorPathWay { pts: r.pts, round: r.round };
    if clear {
        return Some(plain);
    }
    let mut nav = match crate::scene_collision::nav_grid_find_path(g, a, b, Some(3000.0), None) {
        Some(nav) if !nav.is_empty() => nav,
        _ => return Some(plain), // (no plan either: the straight leg, as the runner would have walked it)
    };
    if v3::dist_xz(nav[nav.len() - 1], b) > 0.05 {
        nav.push(b);
    }
    let mut pts: Vec<V3> = Vec::new();
    let mut round = false;
    prev = a;
    for &w in nav.iter() {
        if v3::dist_xz(prev, w) < 0.05 {
            continue;
        }
        let r = door_path_round_leaves(g, prev, w)?;
        pts.extend_from_slice(&r.pts);
        round = round || r.round;
        prev = w;
    }
    if pts.is_empty() {
        Some(plain)
    } else {
        Some(crate::doorpath::DoorPathWay { pts, round })
    }
}

/// His line from where he stands: the lane through the opening while he is outside it, then round the leaves to his point.
fn door_path_lay(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk) {
    let k = g.k.doorpath;
    let d = w.door;
    let p = g.pos_of(w.gd);
    let side = g.tab.doortools_door_op[w.op].side;
    let h = g.dr[d].def.hinge;
    let ux = m::cos(g.dr[d].def.closed_dir);
    let uz = m::sin(g.dr[d].def.closed_dir);
    let wide = g.dr[d].def.width;
    let vx = uz * side; // into the room: off the side the men stacked on
    let vz = -ux * side;
    let at = |s: f64| -> V3 { [h[0] + ux * wide * k.at + vx * s, 0.0, h[1] + uz * wide * k.at + vz * s] };
    let s0 = (p[0] - h[0]) * vx + (p[2] - h[1]) * vz;
    let a0 = (p[0] - h[0]) * ux + (p[2] - h[1]) * uz;
    let mut pts: Vec<V3> = vec![[p[0], 0.0, p[2]]];
    w.lane = 0.0;
    if s0 < k.in_m - 0.25 && v3::dist_xz(p, w.pod) > 0.6 {
        let mut in_m = k.in_m;
        let far = at(in_m);
        if !door_path_free(g, far) {
            in_m = 0.45;
        }
        w.in_m = in_m;
        let pa = at(-k.out_m);
        let pi = at(in_m);
        if door_path_free(g, pi) && (s0 > -0.15 || door_path_free(g, pa)) {
            if s0 < -k.out_m - 0.2 || (s0 < -0.15 && m::abs(a0 - wide * k.at) > 0.15) {
                // (a leaf standing open on HIS side of the wall is gone round as well)
                match door_path_way(g, pts[0], pa) {
                    Some(r0) => pts.extend_from_slice(&r0.pts),
                    None => pts.push(pa),
                }
            }
            pts.push(pi);
            w.lane = (pts.len() - 1) as f64;
        } else {
        }
    }
    let from = pts[pts.len() - 1];
    let mut r = door_path_way(g, from, w.pod);
    w.wait = false;
    if r.is_none() && w.waited < k.cap && leaf_swinging(g, from, w.pod) {
        w.wait = true; // (a leaf still swinging: no verdict on it yet)
    } else if r.is_none() {
        if let Some(q) = door_path_stand(g, w, from, None) {
            door_path_move(g, w, q);
            r = door_path_way(g, from, q);
        }
    }
    if let Some(r) = r {
        pts.extend_from_slice(&r.pts);
        if r.round && !w.round {
            w.round = true;
        }
    } else if !w.wait {
        pts.push(w.pod);
    }
    let mut len: Vec<f64> = vec![0.0];
    for i in 1..pts.len() {
        let next = len[i - 1] + v3::dist_xz(pts[i - 1], pts[i]);
        len.push(next);
    }
    w.pts = pts;
    w.len = len;
    w.s = 0.0;
    w.laid_t = g.time;
    w.arr = None;
    w.near = Vec::new();
    let mut i = 0;
    while i < g.doors.list.len() {
        let x = g.doors.list[i];
        i += 1;
        if gone(g, x) {
            continue;
        }
        let l = door_path_leaf(g, x);
        let mut least = m::INF;
        for j in 1..w.pts.len() {
            least = m::min(least, seg_seg(w.pts[j - 1], w.pts[j], l[0], l[1]));
        }
        if least < 2.5 {
            let angle = g.dr[x].leaf.angle;
            w.near.push(crate::doorpath::WalkNearItem { d: x, angle });
        }
    }
}

/// A leaf still swinging within 2.5 m of this leg.
fn leaf_swinging(g: &mut Game, from: V3, pod: V3) -> bool {
    let mut i = 0;
    while i < g.doors.list.len() {
        let x = g.doors.list[i];
        i += 1;
        if !gone(g, x) && m::abs(g.dr[x].leaf.vel) > 0.15 {
            let l = door_path_leaf(g, x);
            if seg_seg(from, pod, l[0], l[1]) < 2.5 {
                return true;
            }
        }
    }
    false
}

/// His point stands where no walk reaches it clear of a leaf: the nearest floor of the same room that one does, or None.
fn door_path_stand(g: &mut crate::game::Game, w: &crate::doorpath::Walk, from: V3, reach: Option<f64>) -> Option<V3> {
    let k = g.k.doorpath;
    let reach = reach.unwrap_or(k.move_m);
    let pod = w.pod0;
    let l = door_path_leaves(g, pod, pod);
    let mut others: Vec<V3> = Vec::new();
    let room = crate::roomfight::room_fight_room_at(g, pod);
    let down: Vec<V3> = if k.body_m > 0.0 { door_path_fallen(g, pod, reach + k.body_off + 0.5) } else { Vec::new() };
    for (man, q) in g.tab.doortools_door_op[w.op].pod.iter() {
        if man != w.gd {
            others.push(q.pos);
        }
    }
    for q in g.tab.doortools_door_op[w.op].queue.iter() {
        if q.gd != w.gd {
            others.push(q.pod.pos);
        }
    }
    let mut r = 0.3;
    while r <= reach + 1e-6 {
        let mut best: Option<V3> = None;
        let mut bd = m::INF;
        let mut kk = 0i32;
        while kk < 16 {
            let ang = (kk as f64) * m::PI / 8.0;
            kk += 1;
            let c: V3 = [pod[0] + m::cos(ang) * r, 0.0, pod[2] + m::sin(ang) * r];
            if !door_path_free(g, c)
                || l.iter().any(|x| pt_seg(c, x[0], x[1]) < k.leaf_clear + 0.05)
                || others.iter().any(|o| v3::dist_xz(*o, c) < k.apart_m)
                || down.iter().any(|o| v3::dist_xz(*o, c) < k.body_off)
                || crate::roomfight::room_fight_room_at(g, c) != room
            {
                continue;
            }
            let dd = v3::dist_xz(c, from);
            if dd >= bd || door_path_way(g, from, c).is_none() {
                continue;
            }
            best = Some(c);
            bd = dd;
        }
        if best.is_some() {
            return best;
        }
        r += 0.3;
    }
    None
}

/// Where the men down lie within `r` of a point: hips, chest and head of each.
fn door_path_fallen(g: &crate::game::Game, at: V3, r: f64) -> Vec<V3> {
    let mut out: Vec<V3> = Vec::new();
    for &x in g.guards.iter().chain(g.civilians.iter()) {
        let c = g.gd[x].ch;
        if !crate::condition::down(g, g.gd[x].ch) && g.ch[c].ragdoll.is_none() {
            continue;
        }
        for bone in [crate::character::Bone::Hips, crate::character::Bone::Chest, crate::character::Bone::Head] {
            if let Some(q) = g.ch[c].bones.get(bone) {
                if v3::dist_xz(*q, at) < r {
                    out.push([q[0], 0.0, q[2]]);
                }
            }
        }
    }
    out
}

/// His point is `q` now: this walk's, the order's, the runner's, and the door order's record of where he was sent.
fn door_path_move(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk, q: V3) {
    let gd = w.gd;
    let old = w.pod;
    let a = crate::orders::squad_orders_of(&crate::seats::here(g), gd);
    let pod = g.tab.doortools_door_op[w.op].pod.get(gd).copied();
    w.pod = q;
    if let Some(a) = a {
        if let Some(post) = g.tab.orders_assignment[a].post {
            if v3::dist_xz(post, old) < 0.1 {
                let goal: V3 = [q[0], post[1], q[2]];
                g.tab.orders_assignment[a].post = Some(goal); if g.tab.orders_assignment[a].had == Some(goal) { g.tab.orders_assignment[a].had = None; }   // (a NEW ARRAY in `post`'s place: `A.had === A.post`, orders.ts:1472, is an identity: a mark he had that holds THESE numbers is not this one, and `had` is asked by value, so that one goes; any other stays, as the TypeScript's stale one stays. STAND-IN by value)
                g.tab.orders_assignment[a].goal = goal;
                if let Some(s) = g.gd[gd].script.as_mut() {
                    if let Some(sg) = s.goal {
                        if v3::dist_xz(sg, old) < 0.1 {
                            s.goal = Some(goal);
                            // (`gd.script.goal = A.goal`: his goal IS the order's goal object now, so the order's next hand-over is no new leg,
                            //  whatever was pending on the assignment)
                            g.tab.orders_assignment[a].new_leg = false;
                        }
                    }
                }
            }
        }
    }
    if let Some(pod) = pod {
        if v3::dist_xz(pod.pos, old) < 0.1 {
            g.tab.doortools_door_op[w.op].pod.set(gd, crate::entrypoints::Pod { pos: [q[0], pod.pos[1], q[2]], moved: false, ..pod });
        }
    }
}

/// The whole length of his line.
fn door_path_total(w: &crate::doorpath::Walk) -> f64 {
    w.len[w.len.len() - 1]
}

/// The point `s` metres along his line.
fn door_path_point_at(w: &crate::doorpath::Walk, s: f64) -> V3 {
    let n = w.pts.len();
    if s >= door_path_total(w) {
        return w.pts[n - 1];
    }
    let mut i = 1;
    while i < n - 1 && w.len[i] < s {
        i += 1;
    }
    let a = w.pts[i - 1];
    let b = w.pts[i];
    let l = w.len[i] - w.len[i - 1];
    let k = if l > 1e-6 { (s - w.len[i - 1]) / l } else { 1.0 };
    [a[0] + (b[0] - a[0]) * k, 0.0, a[2] + (b[2] - a[2]) * k]
}

/// How far along his line he is now: the nearest point of it from where he was to a metre on, never back; returns how far off it he stands.
fn door_path_advance(g: &crate::game::Game, w: &mut crate::doorpath::Walk) -> f64 {
    let p = g.pos_of(w.gd);
    let mut best_s = w.s;
    let mut best_d = m::INF;
    for i in 1..w.pts.len() {
        if w.len[i] < w.s {
            continue;
        }
        let a = w.pts[i - 1];
        let b = w.pts[i];
        let l = w.len[i] - w.len[i - 1];
        if l < 1e-6 {
            continue;
        }
        if w.len[i - 1] > w.s + 1.0 {
            break;
        }
        let k = m::max(0.0, m::min(1.0, ((p[0] - a[0]) * (b[0] - a[0]) + (p[2] - a[2]) * (b[2] - a[2])) / (l * l)));
        let s = m::max(w.s, m::min(w.s + 1.0, w.len[i - 1] + k * l));
        let q = door_path_point_at(w, s);
        let dd = v3::dist_xz(p, q);
        if dd < best_d - 1e-6 {
            best_d = dd;
            best_s = s;
        }
    }
    w.s = best_s;
    best_d
}

/// Metres of his line still to walk to the frame line of his door (negative: that far past it).
fn door_path_to_line(g: &crate::game::Game, w: &crate::doorpath::Walk) -> f64 {
    if w.lane > 0.0 {
        w.len[w.lane as usize] - w.in_m - w.s
    } else {
        let side = g.tab.doortools_door_op[w.op].side;
        let p = g.pos_of(w.gd);
        -door_path_depth(g, w.door, side, &p)
    }
}

/// How far past the frame line a point stands, on the room's side.
fn door_path_depth(g: &crate::game::Game, d: DoorId, side: f64, p: &[f64]) -> f64 {
    let h = g.dr[d].def.hinge;
    let ux = m::cos(g.dr[d].def.closed_dir);
    let uz = m::sin(g.dr[d].def.closed_dir);
    (p[0] - h[0]) * uz * side - (p[2] - h[1]) * ux * side
}

/// May the queue's i-th man be let go now? One at a time, in the order dealt, each once the man before him has his start.
pub(super) fn door_path_may_go(g: &crate::game::Game, op: DoorOpId, q: crate::doorpath::DoorPathMayGoQ, i: f64) -> bool {
    if i != 0.0 {
        return false;
    }
    let k = g.k.doorpath;
    let t = g.time;
    let (v_gd, v_serial, l_t) = match g.door_path.last_of.get(op) {
        Some(l) => match l.w {
            Some((v_gd, v_serial)) => (v_gd, v_serial, l.t),
            None => return true,
        },
        None => return true,
    };
    if crate::condition::down(g, g.gd[v_gd].ch) {
        return true;
    }
    // (`walks.get(V.gd) !== V`: the last-of row holds the man and his walk's serial. toLine reads the live row)
    let v = match g.door_path.walks.get(v_gd) {
        Some(live) if live.serial == v_serial => live.clone(),
        _ => return true,
    };
    if t - l_t >= k.cap {
        return true;
    }
    let d = g.tab.doortools_door_op[op].door;
    let side = g.tab.doortools_door_op[op].side;
    let p = g.pos_of(q.gd);
    let h = g.dr[d].def.hinge;
    let ux = m::cos(g.dr[d].def.closed_dir);
    let uz = m::sin(g.dr[d].def.closed_dir);
    let out = -door_path_depth(g, d, side, &p);
    let width = g.dr[d].def.width;
    let lx = h[0] + ux * width * k.at - uz * side * k.out_m; // (the lane's outer end, as lay has it)
    let lz = h[1] + uz * width * k.at + ux * side * k.out_m;
    let mine = if out > 0.0 { m::hyp2(p[0] - lx, p[2] - lz) + k.out_m } else { 0.0 }; // his own walk to the line
    mine - door_path_to_line(g, &v) >= k.gap_m
}

/// This man has just been handed his point: his walk is laid and he is last in his order's file.
pub(super) fn door_path_sent(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, pod: crate::doorpath::DoorPathSentPod) {
    let l_n = g.door_path.last_of.get(op).map_or(0.0, |l| l.n);
    let t = g.time;
    let in_m = g.k.doorpath.in_m;
    let door = g.tab.doortools_door_op[op].door;
    let serial = g.serial();
    let mut w = crate::doorpath::Walk {
        gd,
        op,
        door,
        pod: [pod.pos[0], 0.0, pod.pos[2]],
        n: l_n,
        t0: t,
        pts: Vec::new(),
        len: vec![0.0],
        s: 0.0,
        laid_t: t,
        near: Vec::new(),
        lane: 0.0,
        via: Vec::new(),
        arr: None,
        arr_n: 0.0,
        yield_t: -1.0,
        yields: 0.0,
        held: 0.0,
        off_t: m::NAN,
        round: false,
        wait: false,
        waited: 0.0,
        in_m,
        pod0: [pod.pos[0], 0.0, pod.pos[2]],
        body_t: -1.0,
        serial,
    };
    door_path_own(g, &mut w);
    door_path_lay(g, &mut w);
    g.door_path.walks.set(gd, w);
    g.door_path.last_of.set(op, crate::doorpath::DoorPathLastOfValue { w: Some((gd, serial)), n: l_n + 1.0, t });
}

/// The order's own way round is this walk's while it lasts: taken off the order and kept.
fn door_path_own(g: &mut crate::game::Game, w: &mut crate::doorpath::Walk) -> bool {
    let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), w.gd) else { return false };
    let row = &mut g.tab.orders_assignment[a];
    let Some(post) = row.post else { return false };
    if v3::dist_xz(post, w.pod) > 0.1 || row.via.is_empty() {
        return false;
    }
    w.via = row.via.clone();
    row.via.clear();
    row.goal = [post[0], post[1], post[2]];
    true
}

/// The walk is over early: handed back to the nav, and the order's way round to the order if he is not yet through.
fn door_path_drop(g: &mut crate::game::Game, w: &crate::doorpath::Walk, why: crate::doorpath::DoorPathDropWhy) {
    g.door_path.walks.delete(w.gd);
    match why {
        crate::doorpath::DoorPathDropWhy::Taken => {},
        crate::doorpath::DoorPathDropWhy::GaveUp => {},
    }
    if why == crate::doorpath::DoorPathDropWhy::GaveUp {
        // (the line under him was this walk's own: left there, goTo sees its goal met and never plans)
        g.gd[w.gd].drop_path();
    }
    let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), w.gd) else { return };
    let Some(post) = g.tab.orders_assignment[a].post else { return };
    let p = g.pos_of(w.gd);
    let side = g.tab.doortools_door_op[w.op].side;
    if v3::dist_xz(post, w.pod) < 0.1 && g.tab.orders_assignment[a].via.is_empty() && !w.via.is_empty() && door_path_depth(g, w.door, side, &p) < 0.3 {
        // (from the corner nearest him on: never back to one he has passed)
        let mut k = 0;
        for i in 1..w.via.len() {
            if v3::dist_xz(w.via[i], p) < v3::dist_xz(w.via[k], p) {
                k = i;
            }
        }
        g.tab.orders_assignment[a].via = w.via[k..].to_vec();
    }
}

/// His order's stuck clocks wait with him: a man waiting his turn is not a man who cannot get there.
fn hold_clocks(g: &mut Game, gd: GuardId, t: f64, p: V3) {
    if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), gd) {
        let row = &mut g.tab.orders_assignment[a];
        row.mark_t = t;
        row.moved_t = t;
        row.mark[0] = p[0];
        row.mark[2] = p[2];
    }
}

/// `gd.path === W.arr`: W.arr holds the man's path_serial as this walk laid it.
fn path_is_arr(g: &Game, gd: GuardId, w: &crate::doorpath::Walk) -> bool {
    w.arr == Some(g.gd[gd].path_serial)
}

/// The leaves by his line have moved since it was laid.
fn near_moved(g: &mut Game, w: &crate::doorpath::Walk) -> bool {
    for x in w.near.iter() {
        if gone(g, x.d) || m::abs(g.dr[x.d].leaf.angle - x.angle) > 0.05 {
            return true;
        }
    }
    false
}

/// One walk's frame; false when the walk has left the table.
fn tick_walk(g: &mut Game, gd: GuardId, w: &mut crate::doorpath::Walk, t: f64, dt: f64) -> bool {
    let k = g.k.doorpath;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    if crate::condition::down(g, g.gd[gd].ch) || t < w.t0 || !g.guards.contains(&gd) {
        g.door_path.walks.delete(gd); // (dead · the clock has gone back · not on the floor any more)
        return false;
    }
    if g.gd[gd].script.is_some() && door_path_own(g, w) {
        // (back from the fight, the order laid its own way round again)
        let a = crate::orders::squad_orders_of(&crate::seats::here(g), gd).unwrap();
        let goal = g.tab.orders_assignment[a].goal;
        g.tab.orders_assignment[a].new_leg = false;   // (`s.goal = A.goal`: the same object, as in `move` above)
        let s = g.gd[gd].script.as_mut().unwrap();
        s.goal = Some(goal);
        s.arrived = Some(false);
    }
    let on_order = match g.gd[gd].script {
        Some(crate::game::GuardScript { goal: Some(goal), .. }) => !(v3::dist_xz(goal, w.pod) > 0.1),
        _ => false,
    };
    if !on_order {
        // off this order: the walk waits a moment for him, no longer
        if w.off_t.is_nan() {
            w.off_t = t;
        } else if t - w.off_t > 1.5 {
            door_path_drop(g, w, crate::doorpath::DoorPathDropWhy::Taken);
            return false;
        }
        w.arr = None;
        return true;
    }
    if !w.off_t.is_nan() {
        w.off_t = m::NAN;
        door_path_lay(g, w);
    }
    if v3::dist_xz(p, w.pod) < m::max(0.3, k.look * 0.6) && door_path_total(w) - w.s < k.look + 0.3 {
        g.door_path.walks.delete(gd); 
        return false;
    }
    if t < w.yield_t {
        return true;
    }
    if !w.wait && path_is_arr(g, gd, w) && g.gd[gd].path.len() == 1 && g.gd[gd].path_i >= 1 {
        g.door_path.walks.delete(gd); 
        return false;
    }
    if w.arr.is_some() && !g.gd[gd].path.is_empty() && (!path_is_arr(g, gd, w) || g.gd[gd].path.len() as f64 != w.arr_n) {
        // somebody else has laid a path under him: the watchdog's side-step, or its re-plan
        w.yields += 1.0;
        if w.yields > 2.0 {
            door_path_drop(g, w, crate::doorpath::DoorPathDropWhy::GaveUp);
            return false;
        }
        w.yield_t = t + 0.8;
        w.arr = None;
        return true;
    }
    if w.arr.is_none() && js::truthy(w.yields) && t >= w.yield_t {
        door_path_lay(g, w);
    }
    // the leaves have moved under the line: laid afresh from here
    if t - w.laid_t > 0.2 && (w.wait || near_moved(g, w)) {
        door_path_lay(g, w);
    }
    // a man has dropped on his point: it is moved a body's width off him
    if k.body_m > 0.0 && t - w.body_t > 0.25 && v3::dist_xz(p, w.pod) > 0.6 {
        w.body_t = t;
        if !door_path_fallen(g, w.pod, k.body_m).is_empty() {
            if let Some(q) = door_path_stand(g, w, [p[0], 0.0, p[2]], Some(k.body_move)) {
                door_path_move(g, w, q);
                door_path_lay(g, w);
            }
        }
    }
    let off_line = if w.pts.len() > 1 { door_path_advance(g, w) } else { 0.0 };
    if off_line > 0.6 {
        door_path_lay(g, w);
    }
    let end = w.pts[w.pts.len() - 1];
    let tot = door_path_total(w);
    let near = tot - w.s <= k.look;
    let mut cs = w.s + k.look;
    let mut carrot = if near { end } else { door_path_point_at(w, cs) };
    // (round a corner the point ahead is kept that far off him as the crow flies: followPath takes a point for reached at 0.45)
    while !near && cs < tot && v3::dist_xz(p, carrot) < k.look - 1e-3 {
        cs += 0.05;
        carrot = door_path_point_at(w, cs);
    }
    let arr: Vec<V3> = if near || cs >= tot { vec![[end[0], 0.0, end[2]]] } else { vec![carrot, [end[0], 0.0, end[2]]] };
    let goal = g.gd[gd].script.unwrap().goal.unwrap();
    let first = arr[0];
    let arr_n = arr.len() as f64;
    g.gd[gd].set_path(arr);
    let man = &mut g.gd[gd];
    man.path_i = 0;
    man.set_path_goal(Some([goal[0], goal[1], goal[2]]));
    w.arr = Some(man.path_serial);
    w.arr_n = arr_n;
    // his feet stay on the line: until his chest is round he side-steps along it, as the fight had him
    let yaw = v3::yaw_to(p, first);
    if v3::dist_xz(p, first) > 0.05 && m::cos(yaw - crate::character::character_want_body(g, c)) < 0.7 {
        let s = g.gd[gd].script.as_mut().unwrap();
        s.strafe = Some(true);
        s.face = Some(yaw);
    }
    // a leaf still swinging by his line: he stands at the lane's end until it stands
    if w.wait {
        w.waited += dt;
        if tot - w.s < 0.3 {
            g.gd[gd].script.as_mut().unwrap().speed = Some(0.0);
            hold_clocks(g, gd, t, p);
        }
    }
    // the file: room for the man sent before him
    if w.held < k.cap {
        let mut f = 1.0;
        for (key, v) in g.door_path.walks.iter() {
            if key == gd || v.door != w.door || v.t0 > w.t0 || (v.t0 == w.t0 && v.n >= w.n) {
                continue;
            }
            let q = g.pos_of(v.gd);
            let dd = v3::dist_xz(p, q);
            if dd > k.short_m || dd < 1e-3 {
                continue;
            }
            let hx = carrot[0] - p[0];
            let hz = carrot[2] - p[2];
            let hl = js::or(m::hyp2(hx, hz), 1.0);
            if ((q[0] - p[0]) * hx + (q[2] - p[2]) * hz) / (hl * dd) < 0.2 {
                continue; // (beside or behind him: not in his way)
            }
            f = m::min(f, m::max(0.0, (dd - (k.short_m - 0.35)) / 0.35));
        }
        if f < 1.0 {
            let walk = g.tune.guard_walk;
            let s = g.gd[gd].script.as_mut().unwrap();
            s.speed = Some(s.speed.unwrap_or(walk) * f);
            if f < 0.5 {
                w.held += dt;
                hold_clocks(g, gd, t, p);
            }
        }
    }
    true
}

/// Once a frame, after the orders have posed their men and before anybody moves.
pub fn door_path_tick(g: &mut crate::game::Game, dt: f64) {
    let t = g.time;
    // (a copy of the list, as there; a walk's frame removes only its own row, and writes the row back when it stays: set keeps the Map's order)
    for gd in g.door_path.walks.keys() {
        let Some(row) = g.door_path.walks.get(gd) else { continue };
        let mut w = row.clone();
        // (his walk goes by his order, in his own seat's book)
        if crate::seats::standing_for(g, gd, &mut |g| tick_walk(g, gd, &mut w, t, dt)) {
            g.door_path.walks.set(gd, w);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How near a walk passes a door's leaf: a failure means a man walks through a leaf, or round one that is nowhere near his way.
    #[test]
    fn two_stretches_of_floor_are_as_far_apart_as_their_nearest_points() {
        let along = |x0: f64, z0: f64, x1: f64, z1: f64| -> (V3, V3) { ([x0, 0.0, z0], [x1, 0.0, z1]) };
        let apart = |p: (V3, V3), q: (V3, V3)| {
            let d = seg_seg(p.0, p.1, q.0, q.1);
            assert_eq!(d, seg_seg(q.0, q.1, p.0, p.1), "it does not matter which is asked of which");
            d
        };
        let walk = along(0.0, 0.0, 4.0, 0.0);
        assert_eq!(apart(walk, along(2.0, -1.0, 2.0, 1.0)), 0.0);   // they cross
        assert_eq!(apart(walk, along(0.0, 2.0, 4.0, 2.0)), 2.0);    // side by side
        assert_eq!(apart(walk, along(2.0, 1.0, 2.0, 3.0)), 1.0);    // the end of one over the middle of the other
        assert_eq!(apart(walk, along(7.0, 4.0, 9.0, 4.0)), 5.0);    // end to end: not the lines they lie on, which are 4 apart
        assert_eq!(apart(walk, along(-3.0, 4.0, -3.0, 4.0)), 5.0);  // a stretch of no length: its one point
    }
}
