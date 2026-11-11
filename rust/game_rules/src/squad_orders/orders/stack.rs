use super::*;                      // your own module: its types, its other parts' functions
use crate::ask::Over;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the call marksKeep returned: puts back what it took
fn marks_put_back(g: &mut Game, keep: crate::orders::MarksKeep) {
    let crate::orders::MarksKeep::Kept { leaf, stack_own, doorway, stackroom, glasspen, book } = keep;
    g.sets[g.seat].orders.leaf_out_gave = leaf;
    g.m.doorway.stack_own = stack_own;
    g.stats.doorway = doorway;
    g.stats.stackroom = stackroom;
    g.stats.glasspen = glasspen;
    crate::stackroom::stack_room_book_put_back(g, book);
}

/// the line from the doorway's front to a place, asked three times a hand apart when `seam` is set
fn clear_line(g: &Game, over: &Over, front: V3, q: V3, seam: f64) -> bool {
    if crate::scene_collision::solid_blocked_over(g, over, front, [q[0], 0.45, q[2]], None) { return false; }
    if !(seam > 0.0) { return true; }
    let dx = q[0] - front[0];
    let dz = q[2] - front[2];
    let l = js::or(m::hyp2(dx, dz), 1.0);
    let ox = -dz / l * seam;
    let oz = dx / l * seam;
    !crate::scene_collision::solid_blocked_over(g, over, [front[0] + ox, 0.45, front[2] + oz], [q[0] + ox, 0.45, q[2] + oz], None)
        && !crate::scene_collision::solid_blocked_over(g, over, [front[0] - ox, 0.45, front[2] - oz], [q[0] - ox, 0.45, q[2] - oz], None)
}

/// n spots for a move order round `at`, abreast across the facing
pub fn squad_orders_spots_at(g: &crate::game::Game, at: V3, face: f64, n: f64) -> Vec<V3> {
    if n <= 1.0 { return vec![v3::copy(at)]; }
    let f: V3 = [m::sin(face), 0.0, m::cos(face)];
    let r: V3 = [-f[2], 0.0, f[0]];
    let h = g.k.orders.move_spread / 2.0;
    let file_gap = g.k.orders.file_gap;
    let mut out: Vec<V3> = Vec::new();
    if n > 2.0 {
        let mut i: i32 = 0;
        while (i as f64) < n {
            let row = i >> 1;
            let lone = (i as f64) == n - 1.0 && (i & 1) == 0;
            let s = if lone { 0.0 } else if (i & 1) != 0 { h } else { -h };
            let b = (row as f64) * file_gap;
            let mut tries: Vec<V3> = vec![
                [at[0] + r[0] * s - f[0] * b, 0.0, at[2] + r[2] * s - f[2] * b],
                [at[0] - f[0] * (b + 0.45), 0.0, at[2] - f[2] * (b + 0.45)],
            ];
            for ring in 1..=4 {
                let ring = ring as f64;
                for k in 0..8 {
                    let a = face + m::PI + (k as f64) * m::PI / 4.0;
                    tries.push([at[0] + m::sin(a) * 0.7 * ring, 0.0, at[2] + m::cos(a) * 0.7 * ring]);
                }
            }
            let got = spot_of(g, at, &tries, &out);
            out.push(got);
            i += 1;
        }
        return out;
    }
    let mut i: i32 = 0;
    while (i as f64) < n {
        let fi = i as f64;
        let s = (fi - (n - 1.0) / 2.0) * 2.0 * h;
        let mut tries: Vec<V3> = vec![
            [at[0] + r[0] * s, 0.0, at[2] + r[2] * s],
            [at[0] - f[0] * h * 2.0 * (fi + 1.0), 0.0, at[2] - f[2] * h * 2.0 * (fi + 1.0)],
            [at[0] + f[0] * h * 2.0 * (fi + 1.0), 0.0, at[2] + f[2] * h * 2.0 * (fi + 1.0)],
        ];
        for ring in 1..=3 {
            let ring = ring as f64;
            for k in 0..8 {
                let a = face + (k as f64) * m::PI / 4.0;
                tries.push([at[0] + m::sin(a) * 0.7 * ring, 0.0, at[2] + m::cos(a) * 0.7 * ring]);
            }
        }
        let got = spot_of(g, at, &tries, &out);
        out.push(got);
        i += 1;
    }
    out
}

/// spotsAt's `tries.find(q => ok(q) && apart(q)) ?? v3.copy(at)`
fn spot_of(g: &Game, at: V3, tries: &[V3], out: &[V3]) -> V3 {
    for q in tries.iter().copied() {
        let ok = !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && !crate::scene_collision::low_blocked(g, at, q);
        if ok && !out.iter().any(|o| v3::dist_xz(*o, q) < 0.6) { return q; }
    }
    v3::copy(at)
}

/// The stack's marks on `side` of a door for these men
pub fn squad_orders_stack_posts(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    squad_orders_own_mark_first(g, d, side, men, &mut |g: &mut Game| {
        squad_orders_stack_seen(g, d, side, men, &mut |g: &mut Game| squad_orders_stack_posts_room(g, d, side, men, Some(ignore)))
    })
}

/// the plain 'stack up' on a door: the marks above, dealt again out of the floor's view where the leaf stands open
pub fn squad_orders_stack_posts_wait(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
    squad_orders_own_mark_first(g, d, side, men, &mut |g: &mut Game| {
        squad_orders_stack_seen(g, d, side, men, &mut |g: &mut Game| {
            let posts = squad_orders_stack_posts_room(g, d, side, men, Some(&[]));
            let mut taken: Vec<V3> = vec![squad_orders_sam_place(g, &crate::seats::here(g))];
            taken.extend(squad_orders_held_on(g, &crate::seats::here(g), d, men, Some(&[])));
            crate::stackroom::stack_view_fix(g, d, side, posts.as_deref(), &taken, men)
        })
    })
}

/// stackSeen's `trial`: the chain asked with the whole order on hand `h`, nothing left behind but the glass rule's kept answers
fn seen_trial(g: &mut Game, h: f64, ask: &mut dyn FnMut(&mut Game) -> Option<Vec<crate::orders::StackPost>>) -> Option<Vec<crate::orders::StackPost>> {
    let keep = squad_orders_marks_keep(g, &crate::seats::here(g));
    g.sets[g.seat].orders.file_hand = h;
    g.sets[g.seat].orders.file_gave = false;
    let p = ask(g);
    g.sets[g.seat].orders.file_hand = 0.0;
    marks_put_back(g, keep);
    if p.is_some() && g.sets[g.seat].orders.file_gave { p } else { None }
}

/// stackSeen's `sum`: the marks' exposure, ticking none of the picture's counts
fn seen_sum(g: &mut Game, men: &[GuardId], posts: &[crate::orders::StackPost]) -> f64 {
    crate::picture::picture_quiet(g, &mut |g: &mut Game| {
        let mut a = 0.0;
        for q in posts { a = a + crate::picture::exposed_for(g, men, q.pos, None); }
        a
    })
}

/// THE HAND OF THE DOOR NO KNOWN ENEMY SEES
fn squad_orders_stack_seen(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ask: &mut dyn FnMut(&mut crate::game::Game) -> Option<Vec<crate::orders::StackPost>>) -> Option<Vec<crate::orders::StackPost>> {
    if g.sets[g.seat].orders.cast.is_some() || g.sets[g.seat].orders.file_hand != 0.0 || men.is_empty() || !crate::picture::any_idea(g, men) { return ask(g); }
    let dealt = match ask(g) { Some(x) => x, None => return None };
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let hg = g.dr[d].def.hinge;
    let width = g.dr[d].def.width;
    let c = g.dr[d].frame_centre;
    let hand_of = |p: V3, out: f64| -> f64 {
        let a = (p[0] - hg[0]) * u[0] + (p[2] - hg[1]) * u[1];
        if a >= width + out { 1.0 } else if a <= -out { -1.0 } else { 0.0 }
    };
    let own = squad_orders_own_marks(g, &crate::seats::here(g), d, side, men);
    if !own.is_empty() {
        if own.iter().all(|q| dealt.iter().any(|p| v3::dist_xz(p.pos, *q) < OWN_EPS)) { return Some(dealt); }
        let hs: Vec<f64> = own.iter().map(|q| hand_of(*q, 0.0)).collect();
        let h = hs[0];
        if h == 0.0 || hs.iter().any(|x| *x != h) { return Some(dealt); }
        let p = seen_trial(g, h, ask);
        return match p {
            Some(p) if own.iter().all(|q| p.iter().any(|x| v3::dist_xz(x.pos, *q) < OWN_EPS)) => Some(p),
            _ => Some(dealt),
        };
    }
    let s0 = seen_sum(g, men, &dealt);
    if s0 < 0.5 { return Some(dealt); }
    let on: Vec<f64> = if crate::stackroom::stack_view_open(g, d) {
        let (view_men, view_out) = (g.k.stackroom.view_men, g.k.stackroom.view_out);
        men.iter().map(|m_| g.pos_of(*m_))
            .filter(|p| (p[0] - c[0]) * n[0] + (p[2] - c[2]) * n[1] > 0.0 && v3::dist_xz(*p, c) <= view_men)
            .map(|p| hand_of(p, view_out))
            .collect()
    } else { Vec::new() };
    let mut best: Option<Vec<crate::orders::StackPost>> = None;
    let mut best_s = g.k.picture.stack_k * s0;
    for h in [1.0, -1.0] {
        if on.contains(&(-h)) { continue; }
        let p = match seen_trial(g, h, ask) { Some(p) => p, None => continue };
        let s = seen_sum(g, men, &p);
        if if best.is_some() { s < best_s } else { s <= best_s } {
            best = Some(p);
            best_s = s;
        }
    }
    let best = match best { Some(b) => b, None => return Some(dealt) };
    Some(best)
}

/// what the makers of marks tick, and the room rule's book, as they stand: the returned call puts them back
fn squad_orders_marks_keep(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> crate::orders::MarksKeep {
    let leaf = seat.orders.leaf_out_gave;
    let (stack_own, doorway, stackroom, glasspen) = (g.m.doorway.stack_own.clone(), g.stats.doorway.clone(), g.stats.stackroom.clone(), g.stats.glasspen.clone());
    let book = crate::stackroom::stack_room_book_keep(g);
    crate::orders::MarksKeep::Kept { leaf, stack_own, doorway, stackroom, glasspen, book }
}

/// THE WHOLE ORDER FILED ON ONE HAND OF THE DOOR
fn squad_orders_stack_file_on(g: &mut crate::game::Game, over: &Over, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], latch: bool, replaced: &[crate::orders::StackPost]) -> Option<Vec<crate::orders::StackPost>> {
    let (jamb_out, jamb_past, file_gap, seam) = (g.k.orders.jamb_out, g.k.orders.jamb_past, g.k.orders.file_gap, g.k.orders.seam_m);
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let hg = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    let at = |along: f64| -> V3 { [hg[0] + u[0] * along + n[0] * jamb_out, 0.0, hg[1] + u[1] * along + n[1] * jamb_out] };
    let toward = if latch { -1.0 } else { 1.0 };
    let f = [u[0] * toward, u[1] * toward];
    let face = m::atan2(f[0], f[1]);
    let stack_side = if (-n[0]) * (-f[1]) + (-n[1]) * f[0] >= 0.0 { 1.0 } else { -1.0 };
    let fc = g.dr[d].frame_centre;
    let front: V3 = [fc[0] + n[0] * jamb_out, 0.45, fc[2] + n[1] * jamb_out];
    let sam = squad_orders_sam_place(g, &crate::seats::here(g));
    let held = squad_orders_held_on(g, &crate::seats::here(g), d, men, Some(ignore));
    if !latch && !g.dr[d].leaf.off {
        let arc = if side > 0.0 { crate::doors::door_max_a(g, d) > 0.05 } else { crate::doors::door_min_a(g, d) < -0.05 };
        if arc { return None; }
    }
    let mut out: Vec<crate::orders::StackPost> = Vec::new();
    let mut k: usize = 0;
    while k < men.len() + 3 && out.len() < men.len() {
        let kf = k as f64;
        let pos = at(if latch { w + jamb_past + kf * file_gap } else { -jamb_past - kf * file_gap });
        k += 1;
        if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) || !clear_line(g, over, front, pos, seam) || !crate::stackroom::stack_room_has_over(g, over, d, side, pos) { break; }
        if v3::dist_xz(sam, pos) < 0.7 || held.iter().any(|q| v3::dist_xz(*q, pos) < 0.7) || squad_orders_on_furniture(g, pos) { continue; }
        out.push(crate::orders::StackPost { pos, face, stack_side, lead: kf == 0.0, group: if men.len() > 2 { Some((out.len() >> 1) as f64) } else { None } });
    }
    if out.len() < men.len() { return None; }
    let mut replaced_behind = false;
    for q in replaced { if squad_orders_behind_glass(g, d, side, q.pos) { replaced_behind = true; break; } }
    if !replaced_behind {
        for q in &out { if squad_orders_behind_glass(g, d, side, q.pos) { return None; } }
    }
    if eyes_summed(g, d, side, &out) > eyes_summed(g, d, side, replaced) { return None; }
    if held.is_empty() && !out[0].lead { out[0].lead = true; }
    Some(out)
}

/// stackFileOn's and stackLeastGlass's `lines`: the lines through glass onto these places, summed
fn eyes_summed(g: &mut Game, d: DoorId, side: f64, posts: &[crate::orders::StackPost]) -> f64 {
    let mut a = 0.0;
    for q in posts { a = a + squad_orders_glass_eyes(g, d, side, q.pos, None); }
    a
}

/// every mark on the floor of the room that face of the door is in
fn squad_orders_stack_posts_room(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let posts = squad_orders_stack_posts_marks(g, d, side, men, Some(ignore));
    let mut taken: Vec<V3> = vec![squad_orders_sam_place(g, &crate::seats::here(g))];
    taken.extend(squad_orders_held_on(g, &crate::seats::here(g), d, men, Some(ignore)));
    crate::stackroom::stack_room_fix(g, d, side, posts.as_deref(), &taken, None)
}

/// OWN MARK: where Sam stands by a mark men of the order are on and the plain answer moves one of those marks, the marks are asked again with Sam left out, and taken if none lands on him but those very marks
pub fn squad_orders_own_mark_first(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ask: &mut dyn FnMut(&mut crate::game::Game) -> Option<Vec<crate::orders::StackPost>>) -> Option<Vec<crate::orders::StackPost>> {
    if g.sets[g.seat].orders.sam_out { return ask(g); }
    let c = g.pl[g.player].ch;
    let sam = g.ch[c].pos;
    let own = squad_orders_own_marks(g, &crate::seats::here(g), d, side, men);
    if !own.iter().any(|q| v3::dist_xz(sam, *q) < 1.3) { return ask(g); }
    fn stats(g: &Game) -> (f64, i32, f64) { (g.m.doorway.stack_own.leaf, g.stats.doorway.marks, g.m.doorway.stack_own.mark) }
    let c0 = stats(g);
    let plain = ask(g);
    let c1 = stats(g);
    if let Some(pl) = &plain {
        if own.iter().all(|q| pl.iter().any(|p| v3::dist_xz(p.pos, *q) < OWN_EPS)) { return plain; }
    }
    g.sets[g.seat].orders.sam_out = true;
    let blind = ask(g);
    g.sets[g.seat].orders.sam_out = false;
    let c2 = stats(g);
    let took = match &blind {
        Some(b) => b.iter().all(|p| v3::dist_xz(sam, p.pos) >= 0.75 || own.iter().any(|q| v3::dist_xz(*q, p.pos) < OWN_EPS)),
        None => false,
    };
    if !took {
        g.m.doorway.stack_own.leaf = c1.0;
        g.stats.doorway.marks = c1.1;
        g.m.doorway.stack_own.mark = c1.2;
        return plain;
    }
    let blind = blind.unwrap();
    let changed = match &plain {
        None => true,
        Some(pl) => pl.len() != blind.len() || pl.iter().enumerate().any(|(i, p)| v3::dist_xz(p.pos, blind[i].pos) > 0.01),
    };
    g.m.doorway.stack_own.leaf = c0.0 + c2.0 - c1.0;
    g.stats.doorway.marks = c0.1 + c2.1 - c1.1;
    g.m.doorway.stack_own.mark = c0.2 + (if changed || c1.2 > c0.2 { 1.0 } else { 0.0 });
    Some(blind)
}

/// the marks men of an order STAND on, on this face of this door
pub fn squad_orders_own_marks(g: &crate::game::Game, seat: &crate::seats::SeatRef, d: DoorId, side: f64, men: &[GuardId]) -> Vec<V3> {
    let th = g.dr[d].def.closed_dir;
    let nx = -m::sin(th) * side;
    let nz = m::cos(th) * side;
    let c = g.dr[d].frame_centre;
    let mut out: Vec<V3> = Vec::new();
    for gd in men.iter().copied() {
        let id = match seat.orders.asg.get(gd) { Some(a) => *a, None => continue };
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let a = &g.tab.orders_assignment[id];
        if a.door != Some(d) { continue; }
        let post = match a.post { Some(p) => p, None => continue };
        let p = g.pos_of(gd);
        if v3::dist_xz(p, post) < 0.6 && (post[0] - c[0]) * nx + (post[2] - c[2]) * nz > 0.0 { out.push(post); }
    }
    out
}

/// ORDERS.inWay: the words of a 'no room' refusal on a door; `ask` is asked once more as if Sam were not there, and leaves nothing behind
pub fn squad_orders_no_room_words(g: &mut crate::game::Game, plain: &str, ask: &mut dyn FnMut(&mut crate::game::Game) -> bool) -> String {
    if g.sets[g.seat].orders.sam_out { return String::from(plain); }
    let keep = squad_orders_marks_keep(g, &crate::seats::here(g));
    let glass = g.sets[g.seat].orders.glass_at.clone();
    let glass_sig = g.sets[g.seat].orders.glass_sig;
    g.sets[g.seat].orders.sam_out = true;
    let free = ask(g);
    g.sets[g.seat].orders.sam_out = false;
    marks_put_back(g, keep);
    g.sets[g.seat].orders.glass_at = glass;
    g.sets[g.seat].orders.glass_sig = glass_sig;
    if free { String::from(IN_WAY_WORDS) } else { String::from(plain) }
}

/// where Sam stands, as the makers of marks ask it: nowhere during ownMarkFirst's second ask
pub fn squad_orders_sam_place(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> V3 {
    if seat.orders.sam_out { nowhere() } else { squad_orders_sam_stood(g, seat) }
}

/// where Sam stands, to the marks the door code makes; to another cast he is nowhere
pub fn squad_orders_sam_stood(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> V3 {
    if seat.orders.cast.is_some() { return nowhere(); }
    let c = g.pl[seat.player].ch;
    g.ch[c].pos
}

/// A mark of a man not at a jamb is moved off every frame's strip and leaf's swing
fn squad_orders_stack_posts_marks(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let mut out = match squad_orders_stack_posts_as_were(g, d, side, men, Some(ignore)) { Some(o) => o, None => return None };
    let c = g.dr[d].frame_centre;
    let th = g.dr[d].def.closed_dir;
    let n = [-m::sin(th) * side, m::cos(th) * side];
    let jamb_out = g.k.orders.jamb_out;
    let front: V3 = [c[0] + n[0] * jamb_out, 0.45, c[2] + n[1] * jamb_out];
    let sam = squad_orders_sam_place(g, &crate::seats::here(g));
    let held = squad_orders_held_on(g, &crate::seats::here(g), d, men, Some(ignore));
    let over = if g.sets[g.seat].orders.leaf_out_gave { Over::but_door(g, d) } else { Over::OWN };
    let mut i = 0;
    while i < out.len() {
        let p = out[i].clone();
        if p.lead || !crate::doorway::doorway_stand(g, p.pos) { i += 1; continue; }
        let mut taken: Vec<V3> = vec![sam];
        taken.extend(held.iter().copied());
        for (j, o) in out.iter().enumerate() { if j != i { taken.push(o.pos); } }
        let q = crate::doorway::doorway_clear_near(g, p.pos, &mut |g: &mut Game, q: V3| {
            if crate::doorway::doorway_stand(g, q) || (q[0] - c[0]) * n[0] + (q[2] - c[2]) * n[1] < 0.3 { return true; }
            crate::scene_collision::solid_blocked_over(g, &over, front, [q[0], 0.45, q[2]], None)
        }, Some(&taken), Some(0.75), None);
        if let Some(q) = q {
            g.stats.doorway.marks += 1;
            out[i].pos = q;
        }
        i += 1;
    }
    Some(out)
}

/// OWN LEAF: where the answer was 'no room', the door's own open leaf is left out of the walls
fn squad_orders_stack_posts_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let out = squad_orders_stack_posts_leaf_in(g, &Over::OWN, d, side, men, Some(ignore));
    g.sets[g.seat].orders.leaf_out_gave = false;
    if out.is_some() || m::abs(g.dr[d].leaf.angle) < 0.3 || g.dr[d].leaf.off { return out; }
    let marks = squad_orders_stack_posts_leaf_in(g, &Over::but_door(g, d), d, side, men, Some(ignore));
    if marks.is_some() {
        g.m.doorway.stack_own.leaf += 1.0;
        g.sets[g.seat].orders.leaf_out_gave = true;
    }
    marks
}

/// the passes for places: glass first, then the walls' own, then a passage's file, and stackSeen's trial on one hand
fn squad_orders_stack_posts_leaf_in(g: &mut crate::game::Game, over: &Over, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let mut as_ = None;
    if g.k.glasspen.stack_eyes > 0.0 {
        as_ = squad_orders_stack_posts_on(g, over, d, side, men, ignore, true, None);
        if as_.is_none() { as_ = squad_orders_stack_least_glass(g, over, d, side, men, ignore); }
    }
    if as_.is_none() { as_ = squad_orders_stack_posts_on(g, over, d, side, men, ignore, false, None); }
    let out = match as_ { Some(x) => Some(x), None => squad_orders_stack_posts_on(g, over, d, side, men, ignore, false, Some(true)) };
    let file_hand = g.sets[g.seat].orders.file_hand;
    if file_hand == 0.0 { return out; }
    let out = match out { Some(o) => o, None => return None };
    let file = squad_orders_stack_file_on(g, over, d, side, men, ignore, file_hand > 0.0, &out);
    if file.is_some() { g.sets[g.seat].orders.file_gave = true; }
    match file { Some(f) => Some(f), None => Some(out) }
}

/// GLASS IS NOT COVER: is a man at `pos` on this face in a line of fire through glass from the floor beyond
pub fn squad_orders_behind_glass(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3) -> bool {
    let eyes = squad_orders_glass_eyes(g, d, side, pos, None);
    eyes >= g.k.glasspen.stack_eyes && g.k.glasspen.stack_eyes > 0.0
}

/// glassEyes' key: `${index}:${side}:${round(x*10)}:${round(z*10)}`. `+ 0.0`: JavaScript prints -0 as "0", Rust as "-0"
fn glass_key(index: i64, side: f64, pos: V3) -> String {
    alloc::format!("{}:{}:{}:{}", index, side, m::round(pos[0] * 10.0) + 0.0, m::round(pos[2] * 10.0) + 0.0)
}

/// how many of the lattice's points beyond the door have a line through glass onto a man at `pos`
pub fn squad_orders_glass_eyes(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3, kept: Option<bool>) -> f64 {
    let kept = kept.unwrap_or(true);
    if g.k.glasspen.stack_eyes <= 0.0 { return 0.0; }
    let sig = crate::glasspen::glass_toll_sig(g);
    if sig != g.sets[g.seat].orders.glass_sig {
        g.sets[g.seat].orders.glass_at.clear();
        g.sets[g.seat].orders.glass_sig = sig;
    }
    let index = g.doors.list.iter().position(|x| *x == d).map_or(-1, |i| i as i64);
    let key = glass_key(index, side, pos);
    if kept {
        if let Some(v) = g.sets[g.seat].orders.glass_at.get(key.clone()) { return *v; }
    }
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let c = g.dr[d].frame_centre;
    let mut eyes: Vec<V3> = Vec::new();
    let mut b = 0.75;
    while b <= 7.5 {
        let mut l = -6.0;
        while l <= 6.0 {
            let q: V3 = [c[0] - n[0] * b + u[0] * l, 1.3, c[2] - n[1] * b + u[1] * l];
            if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { eyes.push(q); }
            l += 0.75;
        }
        b += 0.75;
    }
    let v = crate::glasspen::shot_through_glass(g, &eyes, [pos[0], 1.3, pos[2]], None);
    if kept { g.sets[g.seat].orders.glass_at.set(key, v); }
    v
}

/// GLASS AT BOTH HANDS OF A DOOR: everybody on the one hand fewer of those lines reach
fn squad_orders_stack_least_glass(g: &mut crate::game::Game, over: &Over, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
    if men.is_empty() { return None; }
    let (jamb_out, jamb_past, file_gap) = (g.k.orders.jamb_out, g.k.orders.jamb_past, g.k.orders.file_gap);
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let hg = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    let at = |along: f64| -> V3 { [hg[0] + u[0] * along + n[0] * jamb_out, 0.0, hg[1] + u[1] * along + n[1] * jamb_out] };
    let n_l = squad_orders_glass_eyes(g, d, side, at(w + jamb_past), None);
    let n_h = squad_orders_glass_eyes(g, d, side, at(-jamb_past), None);
    let stack_eyes = g.k.glasspen.stack_eyes;
    if n_l < stack_eyes || n_h < stack_eyes || n_l == n_h { return None; }
    let base = match squad_orders_stack_posts_on(g, over, d, side, men, ignore, false, None) { Some(b) => b, None => return None };
    let latch = n_h > n_l;
    let toward = if latch { -1.0 } else { 1.0 };
    let f = [u[0] * toward, u[1] * toward];
    let face = m::atan2(f[0], f[1]);
    let stack_side = if (-n[0]) * (-f[1]) + (-n[1]) * f[0] >= 0.0 { 1.0 } else { -1.0 };
    let fc = g.dr[d].frame_centre;
    let front: V3 = [fc[0] + n[0] * jamb_out, 0.45, fc[2] + n[1] * jamb_out];
    let sam = squad_orders_sam_place(g, &crate::seats::here(g));
    let held = squad_orders_held_on(g, &crate::seats::here(g), d, men, Some(ignore));
    let mut out: Vec<crate::orders::StackPost> = Vec::new();
    let mut k: usize = 0;
    while k < men.len() + 3 && out.len() < men.len() {
        let kf = k as f64;
        let pos = at(if latch { w + jamb_past + kf * file_gap } else { -jamb_past - kf * file_gap });
        k += 1;
        if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) || crate::scene_collision::solid_blocked_over(g, over, front, [pos[0], 0.45, pos[2]], None) { break; }
        if v3::dist_xz(sam, pos) < 0.7 || held.iter().any(|q| v3::dist_xz(*q, pos) < 0.7) { continue; }
        out.push(crate::orders::StackPost { pos, face, stack_side, lead: kf == 0.0, group: if men.len() > 2 { Some((out.len() >> 1) as f64) } else { None } });
    }
    if out.len() < men.len() { return None; }
    let on_out = eyes_summed(g, d, side, &out);
    let on_base = eyes_summed(g, d, side, &base);
    if on_out > g.k.glasspen.stack_least_k * on_base { return None; }
    if held.is_empty() && !out[0].lead { out[0].lead = true; }
    Some(out)
}

/// A DOOR ACROSS A PASSAGE: may a man wait at `pos` on this face of it
fn squad_orders_hall_stands(g: &crate::game::Game, over: &Over, d: DoorId, side: f64, pos: V3) -> bool {
    let jamb_out = g.k.orders.jamb_out;
    let th = g.dr[d].def.closed_dir;
    let nx = -m::sin(th) * side;
    let nz = m::cos(th) * side;
    if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) { return false; }
    let c = g.dr[d].frame_centre;
    let fx = c[0] + nx * jamb_out;
    let fz = c[2] + nz * jamb_out;
    let dx = pos[0] - fx;
    let dz = pos[2] - fz;
    let len = js::or(m::hyp2(dx, dz), 1.0);
    for k in [0.0, 0.08, -0.08] {
        let ox = -dz / len * k;
        let oz = dx / len * k;
        if crate::scene_collision::solid_blocked_over(g, over, [fx + ox, 0.45, fz + oz], [pos[0] + ox, 0.45, pos[2] + oz], None) { return false; }
    }
    for i in 0..g.doors.list.len() {
        let o = g.doors.list[i];
        if o == d { continue; }
        let def = &g.dr[o].def;
        let t = def.closed_dir;
        let ux = m::cos(t);
        let uz = m::sin(t);
        let rx = pos[0] - def.hinge[0];
        let rz = pos[2] - def.hinge[1];
        if m::abs(rx * ux + rz * uz - def.width / 2.0) < def.width / 2.0 + 0.35 && m::abs(-rx * uz + rz * ux) < 1.1 { return false; }
    }
    true
}

/// A DOOR ACROSS A PASSAGE: up to `k` places back down the passage on this face, behind the door's jamb marks
pub fn squad_orders_hall_places(g: &mut crate::game::Game, over: &Over, d: DoorId, side: f64, k: f64, from: f64, taken: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> Vec<crate::orders::StackPost> {
    let (jamb_out, jamb_past, file_gap) = (g.k.orders.jamb_out, g.k.orders.jamb_past, g.k.orders.file_gap);
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let hg = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    let c = g.dr[d].frame_centre;
    let mut out: Vec<crate::orders::StackPost> = Vec::new();
    if k <= 0.0 { return out; }
    let at = |s: i32, off: f64| -> V3 {
        let along = if s == 0 { w + jamb_past } else { -jamb_past };
        [hg[0] + u[0] * along + n[0] * off, 0.0, hg[1] + u[1] * along + n[1] * off]
    };
    const HALL_GAP: f64 = 2.6;
    let walled = |g: &mut Game, s: i32, p: V3| -> bool {
        let wd = if s == 0 { 1.0 } else { -1.0 };
        for e in [0.0, 0.08, -0.08] {
            if crate::scene_collision::solid_blocked_over(g, over, [p[0] + n[0] * e, 0.45, p[2] + n[1] * e], [p[0] + n[0] * e + u[0] * wd * 0.8, 0.45, p[2] + n[1] * e + u[1] * wd * 0.8], None) { return true; }
        }
        false
    };
    let mut cols: Vec<(f64, crate::orders::StackPost)> = Vec::new();
    for s in [0, 1] {
        let j = at(s, jamb_out);
        if !squad_orders_hall_stands(g, over, d, side, j) || !walled(g, s, j) { continue; }
        let mut good = |g: &mut Game, off: f64| -> Option<V3> {
            let p = at(s, off);
            if squad_orders_hall_stands(g, over, d, side, p) && walled(g, s, p) && !taken(g, p) { Some(p) } else { None }
        };
        let mut last = -m::INF;
        let mut n0 = 0.0;
        while n0 < k {
            let o = if n0 > 0.0 { last + file_gap } else { from };
            let mut off = o;
            let mut p = good(g, o);
            let mut b = o - 0.05;
            while p.is_none() && n0 > 0.0 && b >= last + 0.7 - 1e-6 {
                p = good(g, b);
                if p.is_some() { off = b; }
                b -= 0.05;
            }
            let mut a = o + 0.05;
            while p.is_none() && a <= o + HALL_GAP && a <= from + 9.0 {
                p = good(g, a);
                if p.is_some() { off = a; }
                a += 0.05;
            }
            let p = match p { Some(p) => p, None => break };
            let f = [c[0] - p[0], c[2] - p[2]];
            let wd = if s == 0 { 1.0 } else { -1.0 };
            let stack_side = if u[0] * wd * -f[1] + u[1] * wd * f[0] >= 0.0 { 1.0 } else { -1.0 };
            cols.push((off, crate::orders::StackPost { pos: p, face: m::atan2(f[0], f[1]), stack_side, lead: false, group: Some(2.0) }));
            last = off;
            n0 += 1.0;
        }
    }
    js::sort_by(&mut cols, |a, b| a.0 - b.0);
    for (_, p) in cols {
        if (out.len() as f64) >= k { break; }
        if !out.iter().any(|o| v3::dist_xz(o.pos, p.pos) < 0.7) { out.push(p); }
    }
    out
}

/// Where keeping off the furniture leaves 'no room', the same places are asked with it left in
pub fn squad_orders_stack_posts_on(g: &mut crate::game::Game, over: &Over, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], glass: bool, hall: Option<bool>) -> Option<Vec<crate::orders::StackPost>> {
    let hall = hall.unwrap_or(false);
    let out = squad_orders_stack_places(g, over, d, side, men, ignore, glass, hall);
    if out.is_some() || g.sets[g.seat].orders.fur_in { return out; }
    g.sets[g.seat].orders.fur_in = true;
    let again = squad_orders_stack_places(g, over, d, side, men, ignore, glass, hall);
    g.sets[g.seat].orders.fur_in = false;
    again
}

/// stackPlaces' `heldBy`: a man outside the order on that jamb under an order on this door
fn held_by(g: &Game, seat: &crate::seats::SeatRef, d: DoorId, men: &[GuardId], ignore: &[GuardId], p: V3) -> bool {
    seat.orders.asg.iter().any(|(gd, id)| {
        if men.contains(&gd) || ignore.contains(&gd) { return false; }
        let a = &g.tab.orders_assignment[*id];
        if a.door != Some(d) { return false; }
        match a.post { Some(post) => v3::dist_xz(post, p) < (if crate::orders::rows::id_is(&a.order, crate::orders::Is::Stacks) { 0.4 } else { 0.9 }), None => false }
    })
}

/// stackPlaces' `taken`: the mark another of the squad stands on under an order on this door
fn taken_at(g: &Game, seat: &crate::seats::SeatRef, d: DoorId, men: &[GuardId], ignore: &[GuardId], p: V3) -> bool {
    seat.orders.asg.iter().any(|(gd, id)| {
        if men.contains(&gd) || ignore.contains(&gd) { return false; }
        let a = &g.tab.orders_assignment[*id];
        if a.door != Some(d) { return false; }
        match a.post { Some(post) => v3::dist_xz(post, p) < 0.45, None => false }
    })
}

/// `g.props.props.some(pr => propGap(pr, p) < 0.3)`
fn prop_by(g: &Game, p: V3) -> bool {
    g.props.props.iter().any(|pr| {
        let row = &g.tab.props_prop[*pr];
        crate::props::prop_gap(row.home, row.hx, row.hz, p) < 0.3
    })
}

/// One face of a door, and the order asking for places on it.
struct StackFace<'a> {
    over: &'a Over,
    d: DoorId,
    side: f64,
    men: &'a [GuardId],
    ignore: &'a [GuardId],
    /// keep off the lines through glass
    glass: bool,
    /// a door across a passage: the file may go back down it
    hall: bool,
    jamb_out: f64,
    jamb_past: f64,
    file_gap: f64,
    seam: f64,
    u: [f64; 2],
    n: [f64; 2],
    hg: [f64; 2],
    w: f64,
    front: V3,
    sam: V3,
    /// the marks others of a big squad have under an order on this door
    held: Vec<V3>,
}

impl StackFace<'_> {
    /// A place `along` the door from its hinge and `off` it, facing `toward` the doorway (1: from the hinge's hand).
    fn mark(&self, along: f64, toward: f64, off: f64) -> crate::orders::StackPost {
        let (u, n, hg) = (self.u, self.n, self.hg);
        let pos: V3 = [hg[0] + u[0] * along + n[0] * off, 0.0, hg[1] + u[1] * along + n[1] * off];
        let f = [u[0] * toward, u[1] * toward];
        crate::orders::StackPost { pos, face: m::atan2(f[0], f[1]), stack_side: if (-n[0]) * (-f[1]) + (-n[1]) * f[0] >= 0.0 { 1.0 } else { -1.0 }, lead: true, group: None }
    }
}

/// Sam stands there, and it is not a mark the order already stands on (that it was his is counted).
fn sam_on(g: &mut Game, f: &StackFace, p: V3) -> bool {
    if v3::dist_xz(f.sam, p) >= 0.7 { return false; }
    if !squad_orders_stood_on(g, &crate::seats::here(g), f.d, f.men, p) { return true; }
    g.m.doorway.stack_own.mark += 1.0;
    false
}

/// Open floor with a clear line from the doorway, and out of the lines through glass if that is asked.
fn floor_for_a_stack(g: &mut Game, f: &StackFace, p: V3) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) && clear_line(g, f.over, f.front, p, f.seam) && !(f.glass && squad_orders_behind_glass(g, f.d, f.side, p))
}

/// On furniture, unless the order already stands there.
fn fur_on(g: &Game, f: &StackFace, p: V3) -> bool {
    squad_orders_on_furniture(g, p) && !squad_orders_stood_on(g, &crate::seats::here(g), f.d, f.men, p)
}

fn free_for_a_stack(g: &mut Game, f: &StackFace, p: V3) -> bool {
    floor_for_a_stack(g, f, p) && !sam_on(g, f, p) && !fur_on(g, f, p)
}

/// The face of the door and what is known of it, for one asking.
fn stack_face<'a>(g: &Game, over: &'a Over, door: (DoorId, f64), order: (&'a [GuardId], &'a [GuardId]), glass: bool, hall: bool) -> StackFace<'a> {
    let (d, side) = door;
    let (men, ignore) = order;
    let (jamb_out, jamb_past, file_gap, seam) = (g.k.orders.jamb_out, g.k.orders.jamb_past, g.k.orders.file_gap, g.k.orders.seam_m);
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let fc = g.dr[d].frame_centre;
    let front: V3 = [fc[0] + n[0] * jamb_out, 0.45, fc[2] + n[1] * jamb_out];
    let mut held: Vec<V3> = Vec::new();
    if squad_orders_big(g, &crate::seats::here(g)) {
        for (gd, id) in g.sets[g.seat].orders.asg.iter() {
            if men.contains(&gd) || ignore.contains(&gd) || crate::condition::down(g, g.gd[gd].ch) { continue; }
            let a = &g.tab.orders_assignment[*id];
            if a.door != Some(d) { continue; }
            if let Some(post) = a.post { held.push(post); }
        }
    }
    let sam = squad_orders_sam_place(g, &crate::seats::here(g));
    StackFace { over, d, side, men, ignore, glass, hall, jamb_out, jamb_past, file_gap, seam, u, n, hg: g.dr[d].def.hinge, w: g.dr[d].def.width, front, sam, held }
}

/// the places a stack can have on this face of a door: a file where one is wanted, else the level's own marks where both jambs are shut to them, else the jambs.
fn squad_orders_stack_places(
    g: &mut crate::game::Game, over: &Over, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], glass: bool, hall: bool,
) -> Option<Vec<crate::orders::StackPost>> {
    let f = stack_face(g, over, (d, side), (men, ignore), glass, hall);
    let latch = f.mark(f.w + f.jamb_past, -1.0, f.jamb_out);
    let hinge = f.mark(-f.jamb_past, 1.0, f.jamb_out);
    let ok_l = free_for_a_stack(g, &f, latch.pos);
    let ok_h = free_for_a_stack(g, &f, hinge.pos);
    let pick1 = if ok_l && !(held_by(g, &crate::seats::here(g), d, men, ignore, latch.pos) && ok_h) { latch.clone() } else { hinge.clone() };
    let filed = if men.len() > 2 {
        floor_for_a_stack(g, &f, latch.pos) || floor_for_a_stack(g, &f, hinge.pos)
    } else {
        !f.held.is_empty() && (ok_l || ok_h) && (men.len() == 2 || held_by(g, &crate::seats::here(g), d, men, ignore, pick1.pos))
    };
    if filed { return file_on_both_jambs(g, &f, &latch, &hinge); }
    if !ok_l && !ok_h {
        if (floor_for_a_stack(g, &f, latch.pos) && fur_on(g, &f, latch.pos)) || (floor_for_a_stack(g, &f, hinge.pos) && fur_on(g, &f, hinge.pos)) { return None; }
        return level_or_worked_marks(g, &f);
    }
    if men.len() <= 1 { return Some(vec![pick1]); }
    if ok_l && ok_h { return Some(vec![latch, hinge]); }
    two_on_one_jamb(g, &f, ok_l, if ok_l { latch } else { hinge })
}

/// Files back from each jamb in turn, half on either side; what they cannot hold goes to a second rank, then the room's rear, then down the hall.
fn file_on_both_jambs(g: &mut Game, f: &StackFace, latch: &crate::orders::StackPost, hinge: &crate::orders::StackPost) -> Option<Vec<crate::orders::StackPost>> {
    let nn = f.men.len();
    let l_file = file_from_jamb(g, f, true, f.jamb_out, 0);
    let h_file = file_from_jamb(g, f, false, f.jamb_out, 0);
    if nn == 1 {
        return match (l_file.first(), h_file.first()) {
            (None, None) => None,
            (Some(a), None) => Some(vec![a.clone()]),
            (None, Some(b)) => Some(vec![b.clone()]),
            (Some(a), Some(b)) => Some(vec![if v3::dist_xz(a.pos, latch.pos) <= v3::dist_xz(b.pos, hinge.pos) { a.clone() } else { b.clone() }]),
        };
    }
    let mut nl = core::cmp::min(l_file.len(), (nn + 1) / 2);
    let nh = core::cmp::min(h_file.len(), nn - nl);
    nl = core::cmp::min(l_file.len(), nn - nh);
    let split = nl > 0 && nh > 0;
    let mut out: Vec<crate::orders::StackPost> = Vec::new();
    for (i, p) in l_file[..nl].iter().enumerate() {
        let mut p = p.clone();
        p.group = Some(if split { 0.0 } else { (i >> 1) as f64 });
        out.push(p);
    }
    for (i, p) in h_file[..nh].iter().enumerate() {
        let mut p = p.clone();
        p.group = Some(if split { 1.0 } else { (i >> 1) as f64 });
        out.push(p);
    }
    if out.len() < nn { second_rank(g, f, &mut out); }
    if !out.is_empty() && out.len() < nn {
        let mut taken: Vec<V3> = out.iter().map(|o| o.pos).collect();
        taken.extend(f.held.iter().copied());
        let rear = squad_orders_rear_posts_over(g, f.over, f.d, f.side, (nn - out.len()) as f64, &taken);
        out.extend(rear);
    }
    if f.hall && out.len() < nn { down_the_hall(g, f, &mut out); }
    if out.len() < nn { return None; }
    if f.held.is_empty() && !out.iter().any(|p| p.lead) { out[0].lead = true; }
    Some(out)
}

/// Sam, furniture or another's mark has the place: the file goes on past it.
fn taken_for_a_file(g: &mut Game, f: &StackFace, p: V3) -> bool {
    sam_on(g, f, p) || fur_on(g, f, p) || f.held.iter().any(|q| v3::dist_xz(*q, p) < 0.7)
}

/// A file straight back from one jamb, `off` the door, from the `k0`th place on, up to where the floor ends.
fn file_from_jamb(g: &mut Game, f: &StackFace, latch_side: bool, off: f64, k0: usize) -> Vec<crate::orders::StackPost> {
    let nn = f.men.len();
    let mut out: Vec<crate::orders::StackPost> = Vec::new();
    for k in k0..(nn + 3) {
        if out.len() >= nn { break; }
        let kf = k as f64;
        let mut p = if latch_side { f.mark(f.w + f.jamb_past + kf * f.file_gap, -1.0, off) } else { f.mark(-f.jamb_past - kf * f.file_gap, 1.0, off) };
        if !floor_for_a_stack(g, f, p.pos) { break; }
        if taken_for_a_file(g, f, p.pos) { continue; }
        p.lead = k == 0 && off == f.jamb_out;
        out.push(p);
    }
    out
}

/// A second rank a gap out from the door, for the men the files could not hold.
fn second_rank(g: &mut Game, f: &StackFace, out: &mut Vec<crate::orders::StackPost>) {
    let mut rank = file_from_jamb(g, f, true, f.jamb_out + f.file_gap, 1);
    rank.extend(file_from_jamb(g, f, false, f.jamb_out + f.file_gap, 1));
    for p in rank {
        if out.len() >= f.men.len() { break; }
        if !out.iter().any(|o| v3::dist_xz(o.pos, p.pos) < 0.7) {
            let mut p = p;
            p.lead = false;
            p.group = Some(2.0);
            out.push(p);
        }
    }
}

/// A door across a passage: those who can stand where they were dealt keep it, the rest go back down the passage, and they are grouped again by column or by twos.
fn down_the_hall(g: &mut Game, f: &StackFace, out: &mut Vec<crate::orders::StackPost>) {
    let nn = f.men.len();
    let (u, n, hg, w) = (f.u, f.n, f.hg, f.w);
    let mut kept: Vec<crate::orders::StackPost> = Vec::new();
    for p in out.iter() {
        if squad_orders_hall_stands(g, f.over, f.d, f.side, p.pos) && floor_for_a_stack(g, f, p.pos) { kept.push(p.clone()); }
    }
    *out = kept;
    let more = (nn - out.len()) as f64;
    let places = squad_orders_hall_places(g, f.over, f.d, f.side, more, f.jamb_out + f.file_gap, &mut |g: &mut Game, q: V3| {
        taken_for_a_file(g, f, q) || out.iter().any(|o| v3::dist_xz(o.pos, q) < 0.7)
    });
    out.extend(places);
    let col = |p: &crate::orders::StackPost| -> i32 { if (p.pos[0] - hg[0]) * u[0] + (p.pos[2] - hg[1]) * u[1] > w / 2.0 { 0 } else { 1 } };
    let off_of = |p: &crate::orders::StackPost| -> f64 { (p.pos[0] - hg[0]) * n[0] + (p.pos[2] - hg[1]) * n[1] };
    let both = out.iter().any(|p| col(p) == 0) && out.iter().any(|p| col(p) == 1);
    js::sort_by(out, |a, b| off_of(a) - off_of(b));
    for (k, p) in out.iter_mut().enumerate() {
        let group = if both { col(p) as f64 } else { (k >> 1) as f64 };
        p.group = Some(group);
    }
}

/// Both jambs are shut: the level's own marks for this face if it has any (a man alone takes the first nobody stands on), else ones worked out from the floor.
fn level_or_worked_marks(g: &mut Game, f: &StackFace) -> Option<Vec<crate::orders::StackPost>> {
    let (d, side, men, ignore) = (f.d, f.side, f.men, f.ignore);
    let authored: Option<Vec<V3>> = g.sets[g.seat].orders.door_marks.iter().find(|m_| m_.door == d && m_.side == side).map(|m_| m_.stack.clone());
    let Some(stack) = authored.filter(|s| !s.is_empty()) else {
        let mut taken: Vec<V3> = vec![f.sam];
        taken.extend(squad_orders_held_on(g, &crate::seats::here(g), d, men, Some(ignore)));
        return squad_orders_worked_stack(g, d, side, men.len() as f64, &taken);
    };
    let marks: Vec<V3> = if men.len() <= 1 {
        vec![match stack.iter().find(|p| !taken_at(g, &crate::seats::here(g), d, men, ignore, **p)) { Some(p) => *p, None => stack[0] }]
    } else {
        stack[..core::cmp::min(stack.len(), men.len())].to_vec()
    };
    Some(marks.iter().enumerate().map(|(i, p)| {
        let along = (p[0] - f.hg[0]) * f.u[0] + (p[2] - f.hg[1]) * f.u[1];
        let mut q = f.mark(along, if along > f.w / 2.0 { -1.0 } else { 1.0 }, f.jamb_out);
        q.pos = v3::copy(*p);
        q.lead = i == 0;
        q
    }).collect())
}

/// Only one jamb is free for two men: the second behind the first, or if that is shut wherever else he can be put.
fn two_on_one_jamb(g: &mut Game, f: &StackFace, ok_l: bool, first: crate::orders::StackPost) -> Option<Vec<crate::orders::StackPost>> {
    let mut second = if ok_l { f.mark(f.w + f.jamb_past + f.file_gap, -1.0, f.jamb_out) } else { f.mark(-f.jamb_past - f.file_gap, 1.0, f.jamb_out) };
    second.lead = false;
    if !free_for_a_stack(g, f, second.pos) {
        if let Some(p) = second_man_elsewhere(g, f, ok_l, &first) { return Some(vec![first, p]); }
        if f.glass { return None; }
    }
    if fur_on(g, f, second.pos) { return None; }
    Some(vec![first, second])
}

/// Where the second man goes with no room behind the first: a rank out from him, a rear post, else the nearest free floor a body's room from him.
fn second_man_elsewhere(g: &mut Game, f: &StackFace, ok_l: bool, first: &crate::orders::StackPost) -> Option<crate::orders::StackPost> {
    let mut rank = if ok_l { f.mark(f.w + f.jamb_past, -1.0, f.jamb_out + f.file_gap) } else { f.mark(-f.jamb_past, 1.0, f.jamb_out + f.file_gap) };
    rank.lead = false;
    if free_for_a_stack(g, f, rank.pos) { return Some(rank); }
    let rear_all = squad_orders_rear_posts_over(g, f.over, f.d, f.side, 4.0, &[first.pos]);
    let mut rear: Vec<crate::orders::StackPost> = Vec::new();
    for p in rear_all { if floor_for_a_stack(g, f, p.pos) { rear.push(p); } }
    if !rear.is_empty() { return Some(rear.swap_remove(0)); }
    let mut near: Option<crate::orders::StackPost> = None;
    let mut near_d = m::INF;
    for i in 0..=8 {
        let fi = i as f64;
        for j in 0..=6 {
            let fj = j as f64;
            let p = if ok_l { f.mark(f.w + f.jamb_past + fj * 0.3, -1.0, f.jamb_out + fi * 0.3) } else { f.mark(-f.jamb_past - fj * 0.3, 1.0, f.jamb_out + fi * 0.3) };
            let dd = v3::dist_xz(p.pos, first.pos);
            if dd < f.file_gap - 0.1 || dd >= near_d || !free_for_a_stack(g, f, p.pos) || prop_by(g, p.pos) { continue; }
            near = Some(p);
            near_d = dd;
        }
    }
    near.map(|mut p| { p.lead = false; p })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A place near a stack is kept by the tenth of a metre: one cell, whichever side of nought it rounds from.
    #[test]
    fn a_place_is_one_key_either_side_of_nought() {
        assert_eq!(glass_key(3, -1.0, [-0.04, 0.0, 0.04]), "3:-1:0:0");
        assert_eq!(glass_key(0, 1.0, [1.26, 0.0, -2.51]), "0:1:13:-25");
    }
}
