//! Every mark of a stack that is not on the floor of its face, or is another door's, dealt again.
use super::*;
use crate::orders::StackPost;

/// The door the marks are for, and the floor of its face.
struct Face<'a> { d: DoorId, side: f64, f: &'a Floor, w: f64, c: V3, th: f64, front: V3, min_m: f64, jamb_out: f64, jamb_past: f64, gap: f64 }

/// Every post not on the floor of that face, or at another door, is dealt again, the rest stay; None: one has no room at all.
pub fn stack_room_fix(g: &mut Game, d: DoorId, side: f64, posts: Option<&[StackPost]>, taken: &[V3], min_m: Option<f64>) -> Option<Vec<StackPost>> {
    let posts = posts?;
    let Some(f) = floor_of(g, d, side) else { return Some(posts.to_vec()) };
    let mut bad = Vec::with_capacity(posts.len());
    for p in posts { bad.push(!on_floor(g, &f, p.pos) || anothers(g, d, p.pos)); }
    if !bad.iter().any(|b| *b) { return Some(posts.to_vec()); }
    let w = g.dr[d].def.width;
    let (jamb_out, jamb_past, gap) = { let k = &g.sets[g.seat].orders.knobs; (k.jamb_out, k.jamb_past, k.file_gap) };
    let front = crate::doortools::door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    let x = Face { d, side, f: &f, w, c: g.dr[d].frame_centre, th: g.dr[d].def.closed_dir, front, min_m: min_m.unwrap_or(0.0), jamb_out, jamb_past, gap };
    let mut out = posts.to_vec();
    for i in 0..out.len() {
        if !bad[i] { continue; }
        let was = out[i];
        let others: Vec<StackPost> = (0..out.len()).filter(|&j| j != i && !bad[j]).map(|j| out[j]).collect();
        let Some(got) = dealt_again(g, &x, &was, &others, taken) else {
            if on_floor(g, &f, was.pos) { continue; }
            return None;
        };
        // (the door has ONE opener: with a man still on the other jamb the handle is that man's)
        let lead = was.lead && !others.iter().any(|o| o.lead);
        out[i] = StackPost { pos: got.pos, face: got.face, stack_side: got.stack_side.unwrap_or(was.stack_side), lead, group: got.group };
        bad[i] = false;
    }
    Some(out)
}

/// What a mark dealt again takes of its new place; no shoulder: its own stays.
struct Got { pos: V3, face: f64, stack_side: Option<f64>, group: Option<f64> }
/// A place off the wall, faced at the doorway, in no file.
fn loose(x: &Face, was: &StackPost, pos: V3) -> Got { Got { pos, face: v3::yaw_to(pos, x.c), stack_side: None, group: was.group.map(|_| 2.0) } }

/// One deal's question: the places taken, whether a place a round reaches through a pane counts (glass is not cover), and whether one furniture stands on does.
struct Ask<'a> { x: &'a Face<'a>, was: &'a StackPost, others: &'a [StackPost], used: &'a [V3], hand: f64, glass: bool, shy: bool }

/// The first place that will have it, from the other jamb's wall out to any floor a body fits; all of it first with the panes shunned, then the furniture.
fn dealt_again(g: &mut Game, x: &Face, was: &StackPost, others: &[StackPost], taken: &[V3]) -> Option<Got> {
    let used: Vec<V3> = taken.iter().copied().chain(others.iter().map(|o| o.pos)).collect();
    let hand = hand_of(g, x.d, x.w, was.pos);
    for (glass, shy) in [(true, true), (true, false), (false, true), (false, false)] {
        let a = Ask { x, was, others, used: &used, hand, glass, shy };
        let got = other_jamb(g, &a)
            .or_else(|| around_corner(g, &a))
            .or_else(|| if was.lead { None } else { back_from_door(g, &a) })
            .or_else(|| within_walk(g, &a));
        if got.is_some() { return got; }
        if glass { continue; }
        let got = (if was.lead { None } else { next_room(g, &a) })
            .or_else(|| any_floor(g, &a))
            .or_else(|| down_passage(g, &a));
        if got.is_some() { return got; }
    }
    None
}
/// Is this place another door's: before its leaf, or within the blast a charge on it throws back.
fn anothers(g: &Game, d: DoorId, q: V3) -> bool { crate::doorway::before_another_door(g, q, d) || crate::doortools::in_anothers_blast(g, &crate::seats::here(g), d, q) }
/// A place a mark may be dealt.
fn fits(g: &mut Game, a: &Ask, q: V3, apart: f64) -> bool {
    let x = a.x;
    on_floor(g, x.f, q)
        && !anothers(g, x.d, q)
        && flat(q, x.c) >= x.min_m
        && !a.used.iter().any(|o| flat(*o, q) < apart)
        && !crate::doorway::doorway_stand(g, q)
        && !(a.glass && crate::orders::squad_orders_behind_glass(g, x.d, x.side, q))
        && !(a.shy && furnished(g, x.d, q))
}

/// The other jamb's wall, the next place past whoever stands there, as far as the wall and the floor run.
fn other_jamb(g: &mut Game, a: &Ask) -> Option<Got> {
    let x = a.x;
    for k in 0..4 {
        let along = if a.hand > 0.0 { -x.jamb_past - k as f64 * x.gap } else { x.w + x.jamb_past + k as f64 * x.gap };
        let pos = crate::doortools::door_pt(g, x.d, along, x.jamb_out, x.side, None);
        if nav_blocked(g, pos[0], pos[2]) || !sees(g, x.front, pos) { return None; }
        if !fits(g, a, pos, g.k.stackroom.apart) { continue; }
        let (face, stack_side) = on_wall(x.th, x.side, -a.hand);
        return Some(Got { pos, face, stack_side: Some(stack_side), group: a.was.group.map(|_| if a.hand > 0.0 { 1.0 } else { 0.0 }) });
    }
    None
}
/// His file turns the corner: behind its nearest man, straight off the door's wall.
fn around_corner(g: &mut Game, a: &Ask) -> Option<Got> {
    let x = a.x;
    let mut file: Vec<StackPost> = a.others.iter().copied().filter(|o| hand_of(g, x.d, x.w, o.pos) == a.hand).collect();
    js::sort_by(&mut file, |p, q| flat(p.pos, a.was.pos) - flat(q.pos, a.was.pos));
    for man in file {
        let (along, off) = (crate::doortools::along_of(g, x.d, man.pos), crate::doortools::off_of(g, x.d, man.pos, x.side));
        let mut prev = man.pos;
        for k in 1..=3 {
            let pos = crate::doortools::door_pt(g, x.d, along, off + k as f64 * x.gap, x.side, None);
            if nav_blocked(g, pos[0], pos[2]) || !sees(g, prev, pos) { break; }
            prev = pos;
            if fits(g, a, pos, g.k.stackroom.apart) && off_lane(g, x.d, x.w, pos) { return Some(Got { pos, face: v3::yaw_to(pos, x.c), stack_side: Some(man.stack_side), group: man.group }); }
        }
    }
    None
}
/// Back from the door, to a hand of it: not the man who has the door to open.
fn back_from_door(g: &mut Game, a: &Ask) -> Option<Got> {
    for r in crate::orders::squad_orders_rear_posts(g, a.x.d, a.x.side, 6.0, &[]) {
        if fits(g, a, r.pos, g.k.stackroom.apart) { return Some(Got { pos: r.pos, face: r.face, stack_side: Some(r.stack_side), group: r.group }); }
    }
    None
}
/// Any floor of the face within a walk, the nearest with a line to the doorway first.
fn within_walk(g: &mut Game, a: &Ask) -> Option<Got> {
    let (x, s, cell) = (a.x, g.k.stackroom, g.col.nav.cell);
    let mut best: Option<(f64, V3)> = None;
    for &c in &x.f.cells {
        let steps = x.f.dist[c as usize] as f64 * cell;
        if steps > s.walk_m || best.is_some_and(|b| steps >= b.0) { break; }
        let p = centre(g, c);
        if crate::doortools::off_of(g, x.d, p, x.side) < x.jamb_out - 0.1 || !off_lane(g, x.d, x.w, p) || !fits(g, a, p, s.apart) || !clear_of_props(g, p) { continue; }
        let score = steps + (if sees(g, x.front, p) { 0.0 } else { 3.0 });
        if best.map_or(true, |b| score < b.0) { best = Some((score, p)); }
    }
    best.map(|b| loose(x, a.was, b.1))
}
/// Anywhere on the face's floor, off the opening's lane, a body fits.
fn any_floor(g: &mut Game, a: &Ask) -> Option<Got> {
    let x = a.x;
    for &c in &x.f.cells {
        let p = centre(g, c);
        if off_lane(g, x.d, x.w, p) && fits(g, a, p, g.k.stackroom.squeeze) && clear_of_props(g, p) { return Some(loose(x, a.was, p)); }
    }
    None
}
/// A passage no wider than its door: down it in file, stood back past where a leaf swings, the nearest first.
fn down_passage(g: &mut Game, a: &Ask) -> Option<Got> {
    let (x, s, cell) = (a.x, g.k.stackroom, g.col.nav.cell);
    for apart in [s.apart, s.squeeze] {
        for &c in &x.f.cells {
            if x.f.dist[c as usize] as f64 * cell > s.walk_m { break; }
            let p = centre(g, c);
            if crate::doortools::off_of(g, x.d, p, x.side) >= s.lane_back && fits(g, a, p, apart) && clear_of_props(g, p) { return Some(loose(x, a.was, p)); }
        }
    }
    None
}
/// A cell's middle not up against the furniture.
fn clear_of_props(g: &Game, q: V3) -> bool {
    for n in 0..g.props.props.len() {
        let pr = g.props.props[n];
        let p = &g.tab.props_prop[pr];
        let gap = crate::props::prop_gap(p.home, p.hx, p.hz, q);
        if gap < 0.3 { return false; }
    }
    true
}

/// A way through to a held room next door.
struct Way { o: DoorId, far: f64, floor: Floor, cost: f64 }

/// The room is too small: by a doorway of a room next to it that the team already holds, on that doorway's far face; never the man who has the door to open.
fn next_room(g: &mut Game, a: &Ask) -> Option<Got> {
    let x = a.x;
    for way in ways_next(g, x) {
        let (o, far) = (way.o, way.far);
        let (w, c) = (g.dr[o].def.width, g.dr[o].frame_centre);
        let front = crate::doortools::door_pt(g, o, w / 2.0, x.jamb_out, far, Some(0.45));
        let mut places: Vec<(f64, f64)> = Vec::new();
        for k in 0..3 {
            let k = k as f64;
            for along in [w + x.jamb_past + k * x.gap, -x.jamb_past - k * x.gap] { places.push((along, x.jamb_out)); }
        }
        for back in [1.6, 2.5] {
            for lat in [w / 2.0 + 0.8, -w / 2.0 - 0.8, w / 2.0 + 1.5, -w / 2.0 - 1.5] { places.push((w / 2.0 + lat, back)); }
        }
        for (along, off) in places {
            let q = crate::doortools::door_pt(g, o, along, off, far, None);
            if next_fits(g, a, &way.floor, c, front, q) { return Some(Got { face: v3::yaw_to(q, c), ..loose(x, a.was, q) }); }
        }
    }
    None
}
/// The ways through to held rooms next door, the shortest first.
fn ways_next(g: &mut Game, x: &Face) -> Vec<Way> {
    let (d, f, s, cs) = (x.d, x.f, g.k.stackroom, g.col.nav.cell);
    let into = floor_of(g, d, -x.side);
    let mut ways: Vec<Way> = Vec::new();
    let pl = g.player;
    let ring = crate::orders::player_ring(g, pl);
    let mut ours: Vec<V3> = Vec::new();
    if !crate::condition::down(g, g.pl[pl].ch) { ours.push(g.ch[g.pl[pl].ch].pos); }
    for man in crate::orders::squad_orders_squad(g, &crate::seats::here(g)) {
        if !crate::condition::down(g, g.gd[man].ch) { ours.push(g.ch[g.gd[man].ch].pos); }
    }
    for n in 0..g.doors.list.len() {
        let o = g.doors.list[n];
        if o == d { continue; }
        for s_o in [1.0, -1.0] {
            let near = floor_of(g, o, s_o);
            let far = -s_o;
            let other = floor_of(g, o, far);
            let (Some(near), Some(other)) = (near, other) else { continue };
            if f.dist[near.seed as usize] < 0 { continue; }
            let walk = f.dist[near.seed as usize] as f64 * cs;
            if walk > s.next_walk_m { continue; }
            // its far face is this very floor, or the room about to be entered
            if other.dist[f.seed as usize] >= 0 || into.as_ref().map_or(false, |i| other.dist[i.seed as usize] >= 0) { continue; }
            let room = if crate::cleared::cleared_rooms_has_floor(g, o, far) { crate::cleared::cleared_rooms_room_by(g, o, far) } else { -1.0 };
            if room < 0.0 { continue; }
            let unheld = if js::truthy(room) { !crate::cleared::cleared_rooms_is_known(g, room) } else { !ours.iter().any(|p| on_floor(g, &other, *p)) };   // (held: one of ours has stood in it)
            if unheld { continue; }
            let latched = g.dr[o].leaf.latched;
            if latched {
                let width = g.dr[o].def.width;
                let at = crate::doortools::door_pt(g, o, width / 2.0, 0.6, s_o, None);
                if crate::doors::doors_locked_out(g, o, at, Some(&ring)) { continue; }
            }
            ways.push(Way { o, far, floor: other, cost: walk + (if latched { 6.0 } else { 0.0 }) });
        }
    }
    js::sort_by(&mut ways, |a, b| a.cost - b.cost);
    ways
}
/// A place by that doorway a mark may be dealt.
fn next_fits(g: &Game, a: &Ask, floor: &Floor, c: V3, front: V3, q: V3) -> bool {
    let s = g.k.stackroom;
    on_floor(g, floor, q)
        && flat(q, c) <= s.next_m
        && flat(q, a.x.c) >= a.x.min_m
        && !a.used.iter().any(|u| flat(*u, q) < s.apart)
        && !crate::doorway::doorway_stand(g, q)
        && sees(g, front, q)
        && !(a.shy && furnished(g, a.x.d, q))
}
