//! src/game/orders.ts lines 1726 to 2099
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `flat` of orders.ts: an alias of `v3.distXZ`
fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }

/// noRoomWords' `ask`: the skeleton's type takes g twice (`FnMut(&mut Game, &mut Game)`), and one g cannot be lent to both (orders.ts:1874)
fn seam_ask_no_room(g: &mut Game, ask: &mut dyn FnMut(&mut Game, &mut Game) -> bool) -> bool {
    todo!("needs: noRoomWords' ask to take g once")
}

/// the call of what `stackRoomBookKeep` returned, as `StackRoomBookKeep`'s doc line says (stackroom.ts:95-96): the floors and doorway cells read since are forgotten
fn book_put_back(g: &mut Game, keep: crate::stackroom::StackRoomBookKeep) {
    let (sig, doors, near, floors) = match keep {
        crate::stackroom::StackRoomBookKeep::Forget => { g.m.stackroom.books = None; return; }
        crate::stackroom::StackRoomBookKeep::Back { sig, doors, near, floors } => (sig, doors, near, floors),
    };
    let book = match &mut g.m.stackroom.books {
        Some(b) if b.sig == sig && b.doors == doors => b,
        _ => { g.m.stackroom.books = None; return; }
    };
    if !near { book.near = None; }
    for door in book.floors.keys() {
        let read = match floors.get(door) { Some(r) => *r, None => { book.floors.delete(door); continue; } };
        if let Some(pair) = book.floors.get_mut(door) {
            if !read[0] { pair[0] = None; }
            if !read[1] { pair[1] = None; }
        }
    }
}

/// the call marksKeep returned: puts back what it took (orders.ts:1805)
fn marks_put_back(g: &mut Game, keep: crate::orders::MarksKeep) {
    let crate::orders::MarksKeep::Kept { leaf, stack_own, doorway, stackroom, glasspen, book } = keep;
    g.orders.leaf_out_gave = leaf;
    g.m.doorway.stack_own = stack_own;
    g.stats.doorway = doorway;
    g.stats.stackroom = stackroom;
    g.stats.glasspen = glasspen;
    book_put_back(g, book);
}

/// the line from the doorway's front to a place, asked three times a hand apart when `seam` is set (orders.ts stackFileOn, stackPlaces `line`)
fn clear_line(g: &mut Game, front: V3, q: V3, seam: f64) -> bool {
    if crate::scene_collision::solid_blocked(g, front, [q[0], 0.45, q[2]], None) { return false; }
    if !(seam > 0.0) { return true; }
    let dx = q[0] - front[0];
    let dz = q[2] - front[2];
    let l = js::or(m::hyp2(dx, dz), 1.0);
    let ox = -dz / l * seam;
    let oz = dx / l * seam;
    !crate::scene_collision::solid_blocked(g, [front[0] + ox, 0.45, front[2] + oz], [q[0] + ox, 0.45, q[2] + oz], None)
        && !crate::scene_collision::solid_blocked(g, [front[0] - ox, 0.45, front[2] - oz], [q[0] - ox, 0.45, q[2] - oz], None)
}

/// n spots for a move order round `at`, abreast across the facing (orders.ts spotsAt)
pub fn squad_orders_spots_at(g: &mut crate::game::Game, at: V3, face: f64, n: f64) -> Vec<V3> {
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
fn spot_of(g: &mut Game, at: V3, tries: &[V3], out: &[V3]) -> V3 {
    for q in tries.iter().copied() {
        let ok = !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && !crate::scene_collision::solid_blocked(g, [at[0], 0.45, at[2]], [q[0], 0.45, q[2]], None);
        if ok && !out.iter().any(|o| flat(*o, q) < 0.6) { return q; }
    }
    v3::copy(at)
}

/// The stack's marks on `side` of a door for these men (orders.ts stackPosts)
pub fn squad_orders_stack_posts(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    own_mark_first_of(g, d, side, men, &mut |g: &mut Game| {
        squad_orders_stack_seen(g, d, side, men, ignore, &mut |g: &mut Game| squad_orders_stack_posts_room(g, d, side, men, Some(ignore)))
    }, |p: &crate::orders::StackPost| p.pos)
}

/// the plain 'stack up' on a door: the marks above, dealt again out of the floor's view where the leaf stands open (orders.ts stackPostsWait)
pub fn squad_orders_stack_posts_wait(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
    own_mark_first_of(g, d, side, men, &mut |g: &mut Game| {
        squad_orders_stack_seen(g, d, side, men, &[], &mut |g: &mut Game| {
            let posts = squad_orders_stack_posts_room(g, d, side, men, Some(&[]));
            let mut taken: Vec<V3> = vec![squad_orders_sam_place(g)];
            taken.extend(squad_orders_held_on(g, d, men, Some(&[])));
            let items: Vec<crate::stackroom::StackViewFixMenItem> = men.iter().map(|m_| crate::stackroom::StackViewFixMenItem {
                char: crate::stackroom::StackViewFixMenItemChar { pos: g.ch[g.gd[*m_].ch].pos },
            }).collect();
            crate::stackroom::stack_view_fix(g, d, side, posts.as_deref(), &taken, &items)
        })
    }, |p: &crate::orders::StackPost| p.pos)
}

/// stackSeen's `trial`: the chain asked with the whole order on hand `h`, nothing left behind but the glass rule's kept answers
fn seen_trial(g: &mut Game, h: f64, ask: &mut dyn FnMut(&mut Game) -> Option<Vec<crate::orders::StackPost>>) -> Option<Vec<crate::orders::StackPost>> {
    let keep = squad_orders_marks_keep(g);
    g.orders.file_hand = h;
    g.orders.file_gave = false;
    let p = ask(g);
    g.orders.file_hand = 0.0;
    marks_put_back(g, keep);
    if p.is_some() && g.orders.file_gave { p } else { None }
}

/// stackSeen's `sum`: the marks' exposure, ticking none of the picture's counts
fn seen_sum(g: &mut Game, men: &[GuardId], posts: &[crate::orders::StackPost]) -> f64 {
    crate::picture::picture_quiet(g, &mut |g: &mut Game| {
        let mut a = 0.0;
        for q in posts { a = a + crate::picture::exposed_for(g, men, q.pos, None); }
        a
    })
}

/// THE HAND OF THE DOOR NO KNOWN ENEMY SEES (orders.ts stackSeen, picture.ts PICTURE.stack)
pub fn squad_orders_stack_seen(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], ask: &mut dyn FnMut(&mut crate::game::Game) -> Option<Vec<crate::orders::StackPost>>) -> Option<Vec<crate::orders::StackPost>> {
    if g.orders.cast.is_some() || g.orders.file_hand != 0.0 || men.is_empty() || !crate::picture::any_idea(g, men) { return ask(g); }
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
    let own = squad_orders_own_marks(g, d, side, men);
    if !own.is_empty() {
        if own.iter().all(|q| dealt.iter().any(|p| flat(p.pos, *q) < OWN_EPS)) { return Some(dealt); }
        let hs: Vec<f64> = own.iter().map(|q| hand_of(*q, 0.0)).collect();
        let h = hs[0];
        if h == 0.0 || hs.iter().any(|x| *x != h) { return Some(dealt); }
        let p = seen_trial(g, h, ask);
        return match p {
            Some(p) if own.iter().all(|q| p.iter().any(|x| flat(x.pos, *q) < OWN_EPS)) => Some(p),
            _ => Some(dealt),
        };
    }
    let s0 = seen_sum(g, men, &dealt);
    if s0 < 0.5 { return Some(dealt); }
    let on: Vec<f64> = if crate::stackroom::stack_view_open(g, d) {
        let (view_men, view_out) = (g.k.stackroom.view_men, g.k.stackroom.view_out);
        men.iter().map(|m_| g.ch[g.gd[*m_].ch].pos)
            .filter(|p| (p[0] - c[0]) * n[0] + (p[2] - c[2]) * n[1] > 0.0 && flat(*p, c) <= view_men)
            .map(|p| hand_of(p, view_out))
            .collect()
    } else { Vec::new() };
    let mut best: Option<Vec<crate::orders::StackPost>> = None;
    let mut best_s = g.k.picture.stack_k * s0;
    let mut hinge = false;
    for h in [1.0, -1.0] {
        if on.contains(&(-h)) { continue; }
        let p = match seen_trial(g, h, ask) { Some(p) => p, None => continue };
        let s = seen_sum(g, men, &p);
        if if best.is_some() { s < best_s } else { s <= best_s } {
            best = Some(p);
            best_s = s;
            hinge = h < 0.0;
        }
    }
    let best = match best { Some(b) => b, None => return Some(dealt) };
    if !g.orders.sam_out {
        g.stats.picture.filed += 1;
        if hinge { g.stats.picture.filed_hinge += 1; }
    }
    Some(best)
}

/// what the makers of marks tick, and the room rule's book, as they stand: the returned call puts them back (orders.ts marksKeep)
pub fn squad_orders_marks_keep(g: &mut crate::game::Game) -> crate::orders::MarksKeep {
    let leaf = g.orders.leaf_out_gave;
    let (stack_own, doorway, stackroom, glasspen) = (g.m.doorway.stack_own.clone(), g.stats.doorway.clone(), g.stats.stackroom.clone(), g.stats.glasspen.clone());
    let book = crate::stackroom::stack_room_book_keep(g);
    crate::orders::MarksKeep::Kept { leaf, stack_own, doorway, stackroom, glasspen, book }
}

/// THE WHOLE ORDER FILED ON ONE HAND OF THE DOOR (orders.ts stackFileOn)
pub fn squad_orders_stack_file_on(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], latch: bool, replaced: &[crate::orders::StackPost]) -> Option<Vec<crate::orders::StackPost>> {
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
    let sam = squad_orders_sam_place(g);
    let held = squad_orders_held_on(g, d, men, Some(ignore));
    if !latch && !crate::doors::door_off(g, d) {
        let arc = if side > 0.0 { crate::doors::door_max_a(g, d) > 0.05 } else { crate::doors::door_min_a(g, d) < -0.05 };
        if arc { return None; }
    }
    let mut out: Vec<crate::orders::StackPost> = Vec::new();
    let mut k: usize = 0;
    while k < men.len() + 3 && out.len() < men.len() {
        let kf = k as f64;
        let pos = at(if latch { w + jamb_past + kf * file_gap } else { -jamb_past - kf * file_gap });
        k += 1;
        if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) || !clear_line(g, front, pos, seam) || !crate::stackroom::stack_room_has(g, d, side, pos) { break; }
        if flat(sam, pos) < 0.7 || held.iter().any(|q| flat(*q, pos) < 0.7) || squad_orders_on_furniture(g, pos) { continue; }
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

/// every mark on the floor of the room that face of the door is in (orders.ts stackPostsRoom, stackroom.ts)
pub fn squad_orders_stack_posts_room(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let posts = squad_orders_stack_posts_marks(g, d, side, men, Some(ignore));
    let mut taken: Vec<V3> = vec![squad_orders_sam_place(g)];
    taken.extend(squad_orders_held_on(g, d, men, Some(ignore)));
    crate::stackroom::stack_room_fix(g, d, side, posts.as_deref(), &taken, None)
}

/// ownMarkFirst for any kind of post: the signature in the skeleton lost the TypeScript's `P extends { pos: Vec3 }`
fn own_mark_first_of<P>(g: &mut Game, d: DoorId, side: f64, men: &[GuardId], ask: &mut dyn FnMut(&mut Game) -> Option<Vec<P>>, pos_of: fn(&P) -> V3) -> Option<Vec<P>> {
    if g.orders.sam_out { return ask(g); }
    let c = g.pl[g.player].ch;
    let sam = g.ch[c].pos;
    let own = squad_orders_own_marks(g, d, side, men);
    if !own.iter().any(|q| flat(sam, *q) < 1.3) { return ask(g); }
    fn stats(g: &Game) -> (f64, i32, f64) { (g.m.doorway.stack_own.leaf, g.stats.doorway.marks, g.m.doorway.stack_own.mark) }
    let c0 = stats(g);
    let plain = ask(g);
    let c1 = stats(g);
    if let Some(pl) = &plain {
        if own.iter().all(|q| pl.iter().any(|p| flat(pos_of(p), *q) < OWN_EPS)) { return plain; }
    }
    g.orders.sam_out = true;
    let blind = ask(g);
    g.orders.sam_out = false;
    let c2 = stats(g);
    let took = match &blind {
        Some(b) => b.iter().all(|p| flat(sam, pos_of(p)) >= 0.75 || own.iter().any(|q| flat(*q, pos_of(p)) < OWN_EPS)),
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
        Some(pl) => pl.len() != blind.len() || pl.iter().enumerate().any(|(i, p)| flat(pos_of(p), pos_of(&blind[i])) > 0.01),
    };
    g.m.doorway.stack_own.leaf = c0.0 + c2.0 - c1.0;
    g.stats.doorway.marks = c0.1 + c2.1 - c1.1;
    g.m.doorway.stack_own.mark = c0.2 + (if changed || c1.2 > c0.2 { 1.0 } else { 0.0 });
    Some(blind)
}

/// doorway.ts OWN MARK, asked where a stack's marks are handed out (orders.ts ownMarkFirst)
pub fn squad_orders_own_mark_first(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ask: &mut dyn FnMut(&mut crate::game::Game) -> Option<Vec<crate::orders::SquadOrdersOwnMarkFirstAskRetItem>>) -> Option<Vec<crate::orders::SquadOrdersOwnMarkFirstAskRetItem>> {
    own_mark_first_of(g, d, side, men, ask, |p: &crate::orders::SquadOrdersOwnMarkFirstAskRetItem| p.pos)
}

/// the marks men of an order STAND on, on this face of this door (orders.ts ownMarks)
pub fn squad_orders_own_marks(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Vec<V3> {
    let th = g.dr[d].def.closed_dir;
    let nx = -m::sin(th) * side;
    let nz = m::cos(th) * side;
    let c = g.dr[d].frame_centre;
    let mut out: Vec<V3> = Vec::new();
    for gd in men.iter().copied() {
        let id = match g.orders.asg.get(gd) { Some(a) => *a, None => continue };
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        let a = &g.tab.orders_assignment[id];
        if a.door != Some(d) { continue; }
        let post = match a.post { Some(p) => p, None => continue };
        let p = g.ch[g.gd[gd].ch].pos;
        if flat(p, post) < 0.6 && (post[0] - c[0]) * nx + (post[2] - c[2]) * nz > 0.0 { out.push(post); }
    }
    out
}

/// ORDERS.inWay: the words of a 'no room' refusal on a door; `ask` is asked once more as if Sam were not there, and leaves nothing behind (orders.ts noRoomWords)
pub fn squad_orders_no_room_words(g: &mut crate::game::Game, plain: &str, ask: &mut dyn FnMut(&mut crate::game::Game, &mut crate::game::Game) -> bool) -> String {
    if g.orders.sam_out { return String::from(plain); }
    let keep = squad_orders_marks_keep(g);
    let glass = g.orders.glass_at.clone();
    let glass_sig = g.orders.glass_sig;
    g.orders.sam_out = true;
    let free = seam_ask_no_room(g, ask);
    g.orders.sam_out = false;
    marks_put_back(g, keep);
    g.orders.glass_at = glass;
    g.orders.glass_sig = glass_sig;
    if free { String::from(IN_WAY_WORDS) } else { String::from(plain) }
}

/// where Sam stands, as the makers of marks ask it: nowhere during ownMarkFirst's second ask (orders.ts samPlace)
pub fn squad_orders_sam_place(g: &mut crate::game::Game) -> V3 {
    if g.orders.sam_out { nowhere() } else { squad_orders_sam_stood(g) }
}

/// where Sam stands, to the marks the door code makes; to another cast he is nowhere (orders.ts samStood)
pub fn squad_orders_sam_stood(g: &mut crate::game::Game) -> V3 {
    if g.orders.cast.is_some() { return nowhere(); }
    let c = g.pl[g.player].ch;
    g.ch[c].pos
}

/// doorway.ts MARKS: a mark of a man not at a jamb is moved off every frame's strip and leaf's swing (orders.ts stackPostsMarks)
pub fn squad_orders_stack_posts_marks(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let mut out = match squad_orders_stack_posts_as_were(g, d, side, men, Some(ignore)) { Some(o) => o, None => return None };
    let c = g.dr[d].frame_centre;
    let th = g.dr[d].def.closed_dir;
    let n = [-m::sin(th) * side, m::cos(th) * side];
    let jamb_out = g.k.orders.jamb_out;
    let front: V3 = [c[0] + n[0] * jamb_out, 0.45, c[2] + n[1] * jamb_out];
    let sam = squad_orders_sam_place(g);
    let held = squad_orders_held_on(g, d, men, Some(ignore));
    let all: Vec<BoxId> = g.col.dynamic_boxes.clone();
    let lines: Vec<BoxId> = if g.orders.leaf_out_gave {
        let leaf = g.dr[d].box_;
        all.iter().copied().filter(|b| *b != leaf).collect()
    } else { all.clone() };
    let mut i = 0;
    while i < out.len() {
        let p = out[i].clone();
        if p.lead || !crate::doorway::doorway_in_way(g, p.pos) { i += 1; continue; }
        let mut taken: Vec<V3> = vec![sam];
        taken.extend(held.iter().copied());
        for (j, o) in out.iter().enumerate() { if j != i { taken.push(o.pos); } }
        let q = crate::doorway::doorway_clear_near(g, p.pos, &mut |g: &mut Game, q: V3| {
            if crate::doorway::doorway_in_way(g, q) || (q[0] - c[0]) * n[0] + (q[2] - c[2]) * n[1] < 0.3 { return true; }
            g.col.dynamic_boxes = lines.clone();
            let cut = crate::scene_collision::solid_blocked(g, front, [q[0], 0.45, q[2]], None);
            g.col.dynamic_boxes = all.clone();
            cut
        }, Some(&taken), Some(0.75), None);
        if let Some(q) = q {
            g.stats.doorway.marks += 1;
            out[i].pos = q;
        }
        i += 1;
    }
    Some(out)
}

/// doorway.ts OWN LEAF: where the answer was 'no room', the door's own open leaf is left out of the walls (orders.ts stackPostsAsWere)
pub fn squad_orders_stack_posts_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let out = squad_orders_stack_posts_leaf_in(g, d, side, men, Some(ignore));
    g.orders.leaf_out_gave = false;
    if out.is_some() || m::abs(crate::doors::door_angle(g, d)) < 0.3 || crate::doors::door_off(g, d) { return out; }
    let all: Vec<BoxId> = g.col.dynamic_boxes.clone();
    let leaf = g.dr[d].box_;
    g.col.dynamic_boxes = all.iter().copied().filter(|b| *b != leaf).collect();
    let marks = squad_orders_stack_posts_leaf_in(g, d, side, men, Some(ignore));
    if marks.is_some() {
        g.m.doorway.stack_own.leaf += 1.0;
        g.orders.leaf_out_gave = true;
    }
    g.col.dynamic_boxes = all;
    marks
}

/// the passes for places: glass first, then the walls' own, then a passage's file, and stackSeen's trial on one hand (orders.ts stackPostsLeafIn)
pub fn squad_orders_stack_posts_leaf_in(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> {
    let ignore: &[GuardId] = ignore.unwrap_or(&[]);
    let mut as_ = None;
    if crate::glasspen::glass_pen_live(g) && g.k.glasspen.stack_eyes > 0.0 {
        as_ = squad_orders_stack_posts_on(g, d, side, men, ignore, true, None);
        if as_.is_none() { as_ = squad_orders_stack_least_glass(g, d, side, men, ignore); }
    }
    if as_.is_none() { as_ = squad_orders_stack_posts_on(g, d, side, men, ignore, false, None); }
    let out = match as_ { Some(x) => Some(x), None => squad_orders_stack_posts_on(g, d, side, men, ignore, false, Some(true)) };
    let file_hand = g.orders.file_hand;
    if file_hand == 0.0 { return out; }
    let out = match out { Some(o) => o, None => return None };
    let file = squad_orders_stack_file_on(g, d, side, men, ignore, file_hand > 0.0, &out);
    if file.is_some() { g.orders.file_gave = true; }
    match file { Some(f) => Some(f), None => Some(out) }
}

/// GLASS IS NOT COVER: is a man at `pos` on this face in a line of fire through glass from the floor beyond (orders.ts behindGlass)
pub fn squad_orders_behind_glass(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3) -> bool {
    let eyes = squad_orders_glass_eyes(g, d, side, pos, None);
    eyes >= g.k.glasspen.stack_eyes && g.k.glasspen.stack_eyes > 0.0
}

/// glassEyes' key: `${index}:${side}:${round(x*10)}:${round(z*10)}`. `+ 0.0`: JavaScript prints -0 as "0", Rust as "-0"
fn glass_key(index: i64, side: f64, pos: V3) -> String {
    alloc::format!("{}:{}:{}:{}", index, side, m::round(pos[0] * 10.0) + 0.0, m::round(pos[2] * 10.0) + 0.0)
}

/// how many of the lattice's points beyond the door have a line through glass onto a man at `pos` (orders.ts glassEyes)
pub fn squad_orders_glass_eyes(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3, kept: Option<bool>) -> f64 {
    let kept = kept.unwrap_or(true);
    if !crate::glasspen::glass_pen_live(g) || g.k.glasspen.stack_eyes <= 0.0 { return 0.0; }
    let sig = crate::glasspen::glass_toll_sig(g);
    if sig != g.orders.glass_sig {
        g.orders.glass_at.clear();
        g.orders.glass_sig = sig;
    }
    let index = g.doors.list.iter().position(|x| *x == d).map_or(-1, |i| i as i64);
    let key = glass_key(index, side, pos);
    if kept {
        if let Some(v) = g.orders.glass_at.get(key.clone()) { return *v; }
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
    if kept { g.orders.glass_at.set(key, v); }
    v
}

/// GLASS AT BOTH HANDS OF A DOOR: everybody on the one hand fewer of those lines reach (orders.ts stackLeastGlass, GLASSPEN.stackLeast)
pub fn squad_orders_stack_least_glass(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
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
    let base = match squad_orders_stack_posts_on(g, d, side, men, ignore, false, None) { Some(b) => b, None => return None };
    let latch = n_h > n_l;
    let toward = if latch { -1.0 } else { 1.0 };
    let f = [u[0] * toward, u[1] * toward];
    let face = m::atan2(f[0], f[1]);
    let stack_side = if (-n[0]) * (-f[1]) + (-n[1]) * f[0] >= 0.0 { 1.0 } else { -1.0 };
    let fc = g.dr[d].frame_centre;
    let front: V3 = [fc[0] + n[0] * jamb_out, 0.45, fc[2] + n[1] * jamb_out];
    let sam = squad_orders_sam_place(g);
    let held = squad_orders_held_on(g, d, men, Some(ignore));
    let mut out: Vec<crate::orders::StackPost> = Vec::new();
    let mut k: usize = 0;
    while k < men.len() + 3 && out.len() < men.len() {
        let kf = k as f64;
        let pos = at(if latch { w + jamb_past + kf * file_gap } else { -jamb_past - kf * file_gap });
        k += 1;
        if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) || crate::scene_collision::solid_blocked(g, front, [pos[0], 0.45, pos[2]], None) { break; }
        if flat(sam, pos) < 0.7 || held.iter().any(|q| flat(*q, pos) < 0.7) { continue; }
        out.push(crate::orders::StackPost { pos, face, stack_side, lead: kf == 0.0, group: if men.len() > 2 { Some((out.len() >> 1) as f64) } else { None } });
    }
    if out.len() < men.len() { return None; }
    let on_out = eyes_summed(g, d, side, &out);
    let on_base = eyes_summed(g, d, side, &base);
    if on_out > g.k.glasspen.stack_least_k * on_base { return None; }
    if held.is_empty() && !out[0].lead { out[0].lead = true; }
    Some(out)
}

/// A DOOR ACROSS A PASSAGE: may a man wait at `pos` on this face of it (orders.ts hallStands, ORDERS.hallFile)
pub fn squad_orders_hall_stands(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3) -> bool {
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
        if crate::scene_collision::solid_blocked(g, [fx + ox, 0.45, fz + oz], [pos[0] + ox, 0.45, pos[2] + oz], None) { return false; }
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

/// A DOOR ACROSS A PASSAGE: up to `k` places back down the passage on this face, behind the door's jamb marks (orders.ts hallPlaces)
pub fn squad_orders_hall_places(g: &mut crate::game::Game, d: DoorId, side: f64, k: f64, from: f64, taken: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> Vec<crate::orders::StackPost> {
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
            if crate::scene_collision::solid_blocked(g, [p[0] + n[0] * e, 0.45, p[2] + n[1] * e], [p[0] + n[0] * e + u[0] * wd * 0.8, 0.45, p[2] + n[1] * e + u[1] * wd * 0.8], None) { return true; }
        }
        false
    };
    let mut cols: Vec<(f64, crate::orders::StackPost)> = Vec::new();
    for s in [0, 1] {
        let j = at(s, jamb_out);
        if !squad_orders_hall_stands(g, d, side, j) || !walled(g, s, j) { continue; }
        let mut good = |g: &mut Game, off: f64| -> Option<V3> {
            let p = at(s, off);
            if squad_orders_hall_stands(g, d, side, p) && walled(g, s, p) && !taken(g, p) { Some(p) } else { None }
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
        if !out.iter().any(|o| flat(o.pos, p.pos) < 0.7) { out.push(p); }
    }
    out
}

/// doorway.ts FURNITURE: where keeping off the furniture leaves 'no room', the same places are asked with it left in (orders.ts stackPostsOn)
pub fn squad_orders_stack_posts_on(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], glass: bool, hall: Option<bool>) -> Option<Vec<crate::orders::StackPost>> {
    let hall = hall.unwrap_or(false);
    let out = squad_orders_stack_places(g, d, side, men, ignore, glass, hall);
    if out.is_some() || g.orders.fur_in { return out; }
    g.orders.fur_in = true;
    let again = squad_orders_stack_places(g, d, side, men, ignore, glass, hall);
    g.orders.fur_in = false;
    again
}

/// stackPlaces' `heldBy`: a man outside the order on that jamb under an order on this door
fn held_by(g: &Game, d: DoorId, men: &[GuardId], ignore: &[GuardId], p: V3) -> bool {
    g.orders.asg.iter().any(|(gd, id)| {
        if men.contains(&gd) || ignore.contains(&gd) { return false; }
        let a = &g.tab.orders_assignment[*id];
        if a.door != Some(d) { return false; }
        match a.post { Some(post) => flat(post, p) < (if a.order == "stack" { 0.4 } else { 0.9 }), None => false }
    })
}

/// stackPlaces' `taken`: the mark another of the squad stands on under an order on this door
fn taken_at(g: &Game, d: DoorId, men: &[GuardId], ignore: &[GuardId], p: V3) -> bool {
    g.orders.asg.iter().any(|(gd, id)| {
        if men.contains(&gd) || ignore.contains(&gd) { return false; }
        let a = &g.tab.orders_assignment[*id];
        if a.door != Some(d) { return false; }
        match a.post { Some(post) => flat(post, p) < 0.45, None => false }
    })
}

/// `g.props.props.some(pr => propGap(pr, p) < 0.3)`
fn prop_by(g: &Game, p: V3) -> bool {
    g.props.props.iter().any(|pr| {
        let row = &g.tab.props_prop[*pr];
        crate::roomclear::prop_gap(crate::roomclear::PropGapP { home: row.home, hx: row.hx, hz: row.hz }, p) < 0.3
    })
}

/// the places a stack can have on this face of a door (orders.ts stackPlaces)
pub fn squad_orders_stack_places(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], glass: bool, hall: bool) -> Option<Vec<crate::orders::StackPost>> {
    let (jamb_out, jamb_past, file_gap, seam) = (g.k.orders.jamb_out, g.k.orders.jamb_past, g.k.orders.file_gap, g.k.orders.seam_m);
    let th = g.dr[d].def.closed_dir;
    let u = [m::cos(th), m::sin(th)];
    let n = [-u[1] * side, u[0] * side];
    let hg = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    let fc = g.dr[d].frame_centre;
    let front: V3 = [fc[0] + n[0] * jamb_out, 0.45, fc[2] + n[1] * jamb_out];
    let mk = |along: f64, toward: f64, off: f64| -> crate::orders::StackPost {
        let pos: V3 = [hg[0] + u[0] * along + n[0] * off, 0.0, hg[1] + u[1] * along + n[1] * off];
        let f = [u[0] * toward, u[1] * toward];
        crate::orders::StackPost { pos, face: m::atan2(f[0], f[1]), stack_side: if (-n[0]) * (-f[1]) + (-n[1]) * f[0] >= 0.0 { 1.0 } else { -1.0 }, lead: true, group: None }
    };
    let sam = squad_orders_sam_place(g);
    let sam_on = |g: &mut Game, p: V3| -> bool {
        if flat(sam, p) >= 0.7 { return false; }
        if !squad_orders_stood_on(g, d, men, p) { return true; }
        g.m.doorway.stack_own.mark += 1.0;
        false
    };
    let floor = |g: &mut Game, p: V3| -> bool {
        !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) && clear_line(g, front, p, seam) && !(glass && squad_orders_behind_glass(g, d, side, p))
    };
    let fur = |g: &mut Game, p: V3| -> bool { squad_orders_on_furniture(g, p) && !squad_orders_stood_on(g, d, men, p) };
    let free = |g: &mut Game, p: V3| -> bool { floor(g, p) && !sam_on(g, p) && !fur(g, p) };
    let latch = mk(w + jamb_past, -1.0, jamb_out);
    let hinge = mk(-jamb_past, 1.0, jamb_out);
    let ok_l = free(g, latch.pos);
    let ok_h = free(g, hinge.pos);
    let mut held: Vec<V3> = Vec::new();
    if squad_orders_big(g) {
        for (gd, id) in g.orders.asg.iter() {
            if men.contains(&gd) || ignore.contains(&gd) || g.gd[gd].state == crate::game::GuardState::Dead { continue; }
            let a = &g.tab.orders_assignment[*id];
            if a.door != Some(d) { continue; }
            if let Some(post) = a.post { held.push(post); }
        }
    }
    let pick1 = if ok_l && !(held_by(g, d, men, ignore, latch.pos) && ok_h) { latch.clone() } else { hinge.clone() };
    let filed = if men.len() > 2 {
        floor(g, latch.pos) || floor(g, hinge.pos)
    } else {
        !held.is_empty() && (ok_l || ok_h) && (men.len() == 2 || held_by(g, d, men, ignore, pick1.pos))
    };
    if filed {
        let nn = men.len();
        let theirs = |g: &mut Game, p: V3| -> bool { sam_on(g, p) || fur(g, p) || held.iter().any(|q| flat(*q, p) < 0.7) };
        let file = |g: &mut Game, latch_side: bool, off: f64, k0: usize| -> Vec<crate::orders::StackPost> {
            let mut out: Vec<crate::orders::StackPost> = Vec::new();
            for k in k0..(nn + 3) {
                if out.len() >= nn { break; }
                let kf = k as f64;
                let mut p = if latch_side { mk(w + jamb_past + kf * file_gap, -1.0, off) } else { mk(-jamb_past - kf * file_gap, 1.0, off) };
                if !floor(g, p.pos) { break; }
                if theirs(g, p.pos) { continue; }
                p.lead = k == 0 && off == jamb_out;
                out.push(p);
            }
            out
        };
        let l_file = file(g, true, jamb_out, 0);
        let h_file = file(g, false, jamb_out, 0);
        if nn == 1 {
            return match (l_file.first(), h_file.first()) {
                (None, None) => None,
                (Some(a), None) => Some(vec![a.clone()]),
                (None, Some(b)) => Some(vec![b.clone()]),
                (Some(a), Some(b)) => Some(vec![if flat(a.pos, latch.pos) <= flat(b.pos, hinge.pos) { a.clone() } else { b.clone() }]),
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
        if out.len() < nn {
            let mut rank = file(g, true, jamb_out + file_gap, 1);
            rank.extend(file(g, false, jamb_out + file_gap, 1));
            for p in rank {
                if out.len() >= nn { break; }
                if !out.iter().any(|o| flat(o.pos, p.pos) < 0.7) {
                    let mut p = p;
                    p.lead = false;
                    p.group = Some(2.0);
                    out.push(p);
                }
            }
        }
        if !out.is_empty() && out.len() < nn {
            let mut taken: Vec<V3> = out.iter().map(|o| o.pos).collect();
            taken.extend(held.iter().copied());
            let rear = squad_orders_rear_posts(g, d, side, (nn - out.len()) as f64, &taken);
            out.extend(rear);
        }
        if hall && out.len() < nn {
            let mut kept: Vec<crate::orders::StackPost> = Vec::new();
            for p in out.iter() {
                if squad_orders_hall_stands(g, d, side, p.pos) && floor(g, p.pos) { kept.push(p.clone()); }
            }
            out = kept;
            let more = (nn - out.len()) as f64;
            let places = squad_orders_hall_places(g, d, side, more, jamb_out + file_gap, &mut |g: &mut Game, q: V3| {
                sam_on(g, q) || fur(g, q) || held.iter().any(|h| flat(*h, q) < 0.7) || out.iter().any(|o| flat(o.pos, q) < 0.7)
            });
            out.extend(places);
            let col = |p: &crate::orders::StackPost| -> i32 { if (p.pos[0] - hg[0]) * u[0] + (p.pos[2] - hg[1]) * u[1] > w / 2.0 { 0 } else { 1 } };
            let off_of = |p: &crate::orders::StackPost| -> f64 { (p.pos[0] - hg[0]) * n[0] + (p.pos[2] - hg[1]) * n[1] };
            let both = out.iter().any(|p| col(p) == 0) && out.iter().any(|p| col(p) == 1);
            js::sort_by(&mut out, |a, b| off_of(a) - off_of(b));
            for (k, p) in out.iter_mut().enumerate() {
                let group = if both { col(p) as f64 } else { (k >> 1) as f64 };
                p.group = Some(group);
            }
        }
        if out.len() < nn { return None; }
        if held.is_empty() && !out.iter().any(|p| p.lead) { out[0].lead = true; }
        return Some(out);
    }
    if !ok_l && !ok_h && ((floor(g, latch.pos) && fur(g, latch.pos)) || (floor(g, hinge.pos) && fur(g, hinge.pos))) { return None; }
    if !ok_l && !ok_h {
        let authored: Option<Vec<V3>> = g.orders.door_marks.iter().find(|m_| m_.door == d && m_.side == side).map(|m_| m_.stack.clone());
        let stack = match authored {
            Some(s) if !s.is_empty() => s,
            _ => {
                let mut taken: Vec<V3> = vec![sam];
                taken.extend(squad_orders_held_on(g, d, men, Some(ignore)));
                return squad_orders_worked_stack(g, d, side, men.len() as f64, &taken);
            }
        };
        let marks: Vec<V3> = if men.len() <= 1 {
            vec![match stack.iter().find(|p| !taken_at(g, d, men, ignore, **p)) { Some(p) => *p, None => stack[0] }]
        } else {
            stack[..core::cmp::min(stack.len(), men.len())].to_vec()
        };
        return Some(marks.iter().enumerate().map(|(i, p)| {
            let along = (p[0] - hg[0]) * u[0] + (p[2] - hg[1]) * u[1];
            let mut q = mk(along, if along > w / 2.0 { -1.0 } else { 1.0 }, jamb_out);
            q.pos = v3::copy(*p);
            q.lead = i == 0;
            q
        }).collect());
    }
    if men.len() <= 1 { return Some(vec![pick1]); }
    if ok_l && ok_h { return Some(vec![latch, hinge]); }
    let first = if ok_l { latch.clone() } else { hinge.clone() };
    let mut second = if ok_l { mk(w + jamb_past + file_gap, -1.0, jamb_out) } else { mk(-jamb_past - file_gap, 1.0, jamb_out) };
    second.lead = false;
    if !free(g, second.pos) {
        let mut rank = if ok_l { mk(w + jamb_past, -1.0, jamb_out + file_gap) } else { mk(-jamb_past, 1.0, jamb_out + file_gap) };
        rank.lead = false;
        if free(g, rank.pos) { return Some(vec![first, rank]); }
        let rear_all = squad_orders_rear_posts(g, d, side, 4.0, &[first.pos]);
        let mut rear: Vec<crate::orders::StackPost> = Vec::new();
        for p in rear_all { if floor(g, p.pos) { rear.push(p); } }
        if !rear.is_empty() { return Some(vec![first, rear.swap_remove(0)]); }
        let mut near: Option<crate::orders::StackPost> = None;
        let mut near_d = m::INF;
        for i in 0..=8 {
            let fi = i as f64;
            for j in 0..=6 {
                let fj = j as f64;
                let p = if ok_l { mk(w + jamb_past + fj * 0.3, -1.0, jamb_out + fi * 0.3) } else { mk(-jamb_past - fj * 0.3, 1.0, jamb_out + fi * 0.3) };
                let dd = flat(p.pos, first.pos);
                if dd < file_gap - 0.1 || dd >= near_d || !free(g, p.pos) || prop_by(g, p.pos) { continue; }
                near = Some(p);
                near_d = dd;
            }
        }
        if let Some(mut p) = near {
            p.lead = false;
            return Some(vec![first, p]);
        }
        if glass { return None; }
    }
    if fur(g, second.pos) { return None; }
    Some(vec![first, second])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// one cell whichever way a place rounds to nought: JavaScript's `${-0}` is "0"
    #[test]
    fn glass_key_reads_minus_nought_as_nought() {
        assert_eq!(glass_key(3, -1.0, [-0.04, 0.0, 0.04]), "3:-1:0:0");
        assert_eq!(glass_key(0, 1.0, [1.26, 0.0, -2.51]), "0:1:13:-25");
    }

    fn book(g: &mut Game, d0: DoorId, d1: DoorId) {
        let f = || Some(Some(crate::stackroom::Floor::zeroed()));
        let mut floors = JsMap::new();
        floors.set(d0, [f(), None]);
        floors.set(d1, [f(), f()]);
        g.m.stackroom.books = Some(crate::stackroom::Book { sig: 3.0, doors: 2.0, near: Some(vec![1u8]), floors });
    }

    /// stackroom.ts:95-96: what the ask read for the first time is forgotten; what had been read stays
    #[test]
    fn book_put_back_forgets_what_was_read_since() {
        let (d0, d1) = (DoorId(0), DoorId(1));
        let mut g = Game::blank();
        book(&mut g, d0, d1);
        let mut read = JsMap::new();
        read.set(d0, [true, false]);
        book_put_back(&mut g, crate::stackroom::StackRoomBookKeep::Back { sig: 3.0, doors: 2.0, near: false, floors: read });
        let b = g.m.stackroom.books.as_ref().unwrap();
        assert!(b.near.is_none());
        assert!(!b.floors.has(d1));
        let pair = b.floors.get(d0).unwrap();
        assert!(pair[0].is_some() && pair[1].is_none());
    }

    /// no book when kept, or one rebuilt during the ask: no book after
    #[test]
    fn book_put_back_drops_a_rebuilt_book() {
        let (d0, d1) = (DoorId(0), DoorId(1));
        let mut g = Game::blank();
        book(&mut g, d0, d1);
        book_put_back(&mut g, crate::stackroom::StackRoomBookKeep::Back { sig: 2.0, doors: 2.0, near: true, floors: JsMap::new() });
        assert!(g.m.stackroom.books.is_none());
        book(&mut g, d0, d1);
        book_put_back(&mut g, crate::stackroom::StackRoomBookKeep::Forget);
        assert!(g.m.stackroom.books.is_none());
    }
}
