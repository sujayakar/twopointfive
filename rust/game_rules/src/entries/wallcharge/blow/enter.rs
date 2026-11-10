//! Behind the blast: through the hole, each to a point of his own inside.
use super::*;

/// The hole they go in by: its wall, how thick that is, and the floor just past it.
struct Way { w: WallSpot, tk: f64, du: f64, half: f64, mouth: V3 }

/// The men nearest the hole first, each through its far mouth to a point of his own; one with no floor inside keeps the stand-off.
pub(super) fn wall_charge_enter(g: &mut Game, op: WallOpId) {
    let w = o!(g, op).wall.clone();
    let hole = o!(g, op).hole.unwrap();
    let tk = thick(g, &w);
    let mouth = wall_pt(g, &w, hole.du, -(tk + g.k.wallcharge.mouth_in), None);
    let x = Way { w, tk, du: hole.du, half: hole.half, mouth };
    let men = by_nearest(g, op, mouth);
    let pods = points_inside(g, &x, men.len());
    let lead = o!(g, op).lead;
    if pods.is_empty() {
        crate::game::game_say(g, lead, "no way in.", None, None);
        wall_charge_end(g, op);
        return;
    }
    let (go, stay) = men.split_at(pods.len().min(men.len()));
    if !stay.is_empty() { o!(g, op).men.retain(|m| !stay.contains(m)); }
    let mut queue = lined_up(g, &x, go, &pods);
    doors_held(g, &x, &mut queue);
    let t = g.time;
    let r = &mut o!(g, op);
    (r.queue, r.phase, r.t0) = (queue, Phase::Enter, t);
    crate::game::game_say(g, lead, "going in.", None, None);
}
/// The order's men, the nearest the hole first and the man on the big gun last.
fn by_nearest(g: &Game, op: WallOpId, mouth: V3) -> Vec<GuardId> {
    let mut men: Vec<(GuardId, f64)> = o!(g, op).men.iter().map(|&m| (m, v3::dist_xz(pos_of(g, m), mouth))).collect();
    js::sort_by(&mut men, |a, b| a.1 - b.1);
    let men: Vec<GuardId> = men.into_iter().map(|r| r.0).collect();
    crate::gunroles::gun_last(g, &men)
}
fn nav_free(g: &Game, q: V3) -> bool { !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) }

/// A point inside for each, as the table has them past the hole or as near to that as the floor allows.
fn points_inside(g: &Game, x: &Way, n: usize) -> Vec<Pod> {
    let into = v3::yaw_to(wall_pt(g, &x.w, 0.0, 0.0, None), wall_pt(g, &x.w, 0.0, -1.0, None));
    let table = g.k.wallcharge.pods.clone();
    let mut pods: Vec<Pod> = Vec::new();
    for i in 0..n {
        let Some(pos) = point_inside(g, x, table[i % table.len()], &pods) else { continue };
        let face = look_from(g, pos, into, v3::yaw_to(x.mouth, pos));
        pods.push(Pod { pos, face, watch: None, door: None, hold: None, took: None });
    }
    pods
}
/// (In one of the room's doorways only when the room has no other floor for him.)
fn point_inside(g: &Game, x: &Way, [lat, deep]: [f64; 2], pods: &[Pod]) -> Option<V3> {
    let (mouth, off_frame) = (x.mouth, g.k.wallcharge.off_frame_pod);
    for strict in [true, false] {
        for k in [1.0, 0.6, 0.3, -1.0, -0.6, 0.0] {
            for dd in [0.0, 0.6, -0.5, 1.2] {
                let q = wall_pt(g, &x.w, x.du + lat * k, -(x.tk + m::max(0.8, deep + dd)), None);
                if nav_free(g, q) && !crate::scene_collision::solid_blocked(g, [mouth[0], 0.45, mouth[2]], [q[0], 0.45, q[2]], None)
                    && !(strict && wall_charge_in_frame(g, q, Some(off_frame))) && !pods.iter().any(|p| v3::dist_xz(p.pos, q) < 0.7) { return Some(q); }
            }
        }
    }
    None
}
/// How far a line runs open from `p` at `yaw`, up to 6 m.
fn open_way(g: &Game, p: V3, yaw: f64) -> f64 {
    let (s, c) = (m::sin(yaw), m::cos(yaw));
    let mut d = 0.0;
    while d < 6.0 && !crate::scene_collision::solid_blocked(g, [p[0], 1.2, p[2]], [p[0] + s * (d + 1.0), 1.2, p[2] + c * (d + 1.0)], None) { d += 1.0; }
    d
}
/// The way a man on his point inside faces: the way he came unless that is a wall at his chest.
fn look_from(g: &Game, p: V3, into: f64, want: f64) -> f64 {
    if open_way(g, p, want) >= 2.0 { return want; }
    let (mut best, mut score) = (want, f64::NEG_INFINITY);
    let mut a = -90.0;
    while a <= 90.0 {
        let yaw = into + a * core::f64::consts::PI / 180.0;
        let s = open_way(g, p, yaw) - 0.4 * m::abs(m::atan2(m::sin(yaw - want), m::cos(yaw - want)));
        if s > score { score = s; best = yaw; }
        a += 15.0;
    }
    best
}

/// Two at a time, the points of each two dealt to the two they are nearest; each with his lane through the hole and when he may go.
fn lined_up(g: &Game, x: &Way, men: &[GuardId], pods: &[Pod]) -> Vec<WallChargeDoneItemQueueItem> {
    let mut deal: JsMap<GuardId, Pod> = JsMap::new();
    for (two, pods) in men.chunks(2).zip(pods.chunks(2)) {
        let spots: Vec<V3> = pods.iter().map(|p| p.pos).collect();
        for (gd, pos) in crate::orders::squad_orders_pair(g, two, &spots, None, None) { deal.set(gd, pods.iter().find(|p| p.pos == pos).unwrap().clone()); }
    }
    let half = m::max(0.0, m::min(g.k.wallcharge.lane, x.half - 0.4));
    let (t, second, mouth_in) = (g.time, g.k.doortools.second, g.k.wallcharge.mouth_in);
    men.iter().enumerate().map(|(i, &gd)| {
        let pod = deal.get(gd).unwrap_or(&pods[0]).clone();
        let hand = js::or(m::sign(wall_along(&x.w, pod.pos) - x.du), if i % 2 != 0 { 1.0 } else { -1.0 });
        let lane = wall_pt(g, &x.w, x.du + hand * half, -(x.tk + mouth_in), None);
        let lane = if nav_free(g, lane) { lane } else if nav_free(g, x.mouth) { x.mouth } else { pod.pos };
        WallChargeDoneItemQueueItem { gd, at: t + i as f64 * second, lane, pod }
    }).collect()
}

/// One of the room's own doors, and whether it is in this same wall.
struct Door { name: String, p: V3, back: bool }
/// Each, nearest the hole first, is one man's to hold from his point. A man with no door keeps the room ahead of him, and one man always does.
fn doors_held(g: &mut Game, x: &Way, queue: &mut [WallChargeDoneItemQueueItem]) {
    let has = |p: &Pod| p.watch.is_some() || p.hold.is_some();
    for e in doors_in(g, x) {
        if queue.iter().filter(|q| !has(&q.pod)).count() <= 1 { break; }
        let mut pick: Option<(f64, usize)> = None;
        for (i, q) in queue.iter().enumerate() {
            let (p, d) = (q.pod.pos, v3::dist_xz(q.pod.pos, e.p));
            if has(&q.pod) || d < 0.8 || pick.is_some_and(|b| d >= b.0) || crate::scene_collision::shot_blocked(g, [p[0], 1.2, p[2]], e.p, None, None) { continue; }
            // (a line of FIRE: a mate's point on it is a mate down his muzzle)
            if queue.iter().enumerate().any(|(j, o)| j != i && near_seg(o.pod.pos, p, e.p) < g.k.wallcharge.mate_line) { continue; }
            pick = Some((d, i));
        }
        let Some((_, i)) = pick else { continue };
        let face = v3::yaw_to(queue[i].pod.pos, e.p);
        queue[i].pod.hold = Some(PodHold { face, watch: e.p, door: Some(e.name) });
    }
}
fn doors_in(g: &mut Game, x: &Way) -> Vec<Door> {
    let mut doors: Vec<Door> = Vec::new();
    let Some(room) = crate::roomfight::room_fight_room_at(g, x.mouth) else { return doors };
    for e in &g.tab.roomfight_room[room].entries {
        let p = [e.at[0] - e.out[0] * 0.5, 1.2, e.at[2] - e.out[1] * 0.5];
        let off = wall_off(g, &x.w, p);
        if off >= -x.tk { continue; }
        let square = m::abs(if x.w.along_x { e.out[1] } else { e.out[0] }) > 0.7;
        doors.push(Door { name: g.dr[e.door].def.name.clone(), p, back: square && off > -(x.tk + 1.2) });
    }
    js::sort_by(&mut doors, |a, b| js::or((a.back as i32 - b.back as i32) as f64, v3::dist_xz(a.p, x.mouth) - v3::dist_xz(b.p, x.mouth)));
    doors
}
