//! A stack on a door that stands open: out of the view of the floor beyond it, and off the far side of the opening.
use super::*;
use crate::orders::StackPost;
use crate::rearwatch::RearWaysItem;

/// Is `p` (on face `side`) in view of the floor beyond the door through its opening
fn in_view(g: &Game, d: DoorId, far: &[V3], p: V3) -> bool {
    let (eye, need) = (g.k.stackroom.view_eye, g.k.stackroom.view_cells);
    let pp = [p[0], eye, p[2]];
    let mut n = 0.0;
    for q in far {
        if crate::guards::crosses_door(g, *q, p, d).is_some() && !crate::scene_collision::segment_blocked(g, [q[0], eye, q[2]], pp, None, None, None) {
            n += 1.0;
            if n >= need { return true; }
        }
    }
    false
}

/// The nav cells within `viewFar` of the doorway on the face the men are NOT on
fn far_cells(g: &Game, d: DoorId, side: f64) -> Vec<V3> {
    let view_far = g.k.stackroom.view_far;
    let cell = g.col.nav.cell;
    let c = g.dr[d].frame_centre;
    let n = m::ceil(view_far / cell);
    let (cx, cz) = (m::floor(c[0] / cell), m::floor(c[2] / cell));
    let mut l = Vec::new();
    let mut dz = -n;
    while dz <= n {
        let mut dx = -n;
        while dx <= n {
            let q = [(cx + dx + 0.5) * cell, 0.0, (cz + dz + 0.5) * cell];
            if v3::dist_xz(q, c) <= view_far && !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) {
                if crate::orders::side_of(g, d, q) != side { l.push(q); }
            }
            dx += 1.0;
        }
        dz += 1.0;
    }
    l
}

/// The leaf stands open: off its hinges, or unlatched and swung
pub fn stack_view_open(g: &Game, d: DoorId) -> bool {
    if g.dr[d].leaf.off { return true; }
    if g.dr[d].leaf.latched { return false; }
    m::abs(g.dr[d].leaf.angle) >= g.k.stackroom.view_ajar
}

/// How far from the frame's centre a plain stack's mark on this door can be
pub(in crate::entries) fn stack_view_reach(g: &Game, d: DoorId) -> f64 {
    if stack_view_open(g, d) { m::max(2.6, g.dr[d].def.width / 2.0 + g.k.stackroom.view_past + 0.7) } else { 2.6 }
}

/// The door, the floor before it, the floor beyond that sees through it, and the marks as they were dealt.
struct View<'a> { d: DoorId, side: f64, f: &'a Floor, w: f64, c: V3, jamb_out: f64, jamb_past: f64, front: V3, far: Vec<V3>, taken: &'a [V3], posts: &'a [StackPost] }
impl View<'_> {
    fn hand(&self, g: &Game, q: V3) -> f64 { hand_of(g, self.d, self.w, q) }
}
/// What is worked out of them: the marks a man already stands on, those in view, those across the opening from the men, and each hand's places (with
/// whether they are behind glass), made once.
struct Deal { kept: JsMap<usize, V3>, seen: Vec<bool>, across: Vec<bool>, places: [Option<Vec<(V3, bool)>>; 2], ways: Vec<Vec<RearWaysItem>> }
impl Deal {
    /// Where a mark is as it stands.
    fn at(&self, x: &View, i: usize) -> V3 { self.kept.get(i).copied().unwrap_or(x.posts[i].pos) }
    fn moved(&self, i: usize) -> bool { self.seen[i] || self.across[i] }
}
/// A place dealt: where, on which hand, and whether that is the other hand from the mark's own.
#[derive(Clone, Copy)]
struct Dealt { q: V3, h: f64, over: bool }

/// A plain 'stack up' on a door standing open: the marks in view, and those across the opening from `men`, dealt again along the door's wall.
pub fn stack_view_fix(g: &mut Game, d: DoorId, side: f64, posts: Option<&[StackPost]>, taken: &[V3], men: &[GuardId]) -> Option<Vec<StackPost>> {
    let posts = posts?;
    if !stack_view_open(g, d) { return Some(posts.to_vec()); }
    let Some(f) = floor_of(g, d, side) else { return Some(posts.to_vec()) };
    let (jamb_out, jamb_past) = (g.groups[g.group].orders.knobs.jamb_out, g.groups[g.group].orders.knobs.jamb_past);
    let w = g.dr[d].def.width;
    let front = crate::doortools::door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    let x = View { d, side, f: &f, w, c: g.dr[d].frame_centre, jamb_out, jamb_past, front, far: far_cells(g, d, side), taken, posts };
    let mut k = Deal { kept: kept_marks(g, &x, men), seen: Vec::new(), across: vec![false; posts.len()], places: [None, None], ways: Vec::new() };
    for i in 0..posts.len() { k.seen.push(!k.kept.has(i) && in_view(g, d, &x.far, posts[i].pos)); }
    across_marks(g, &x, &mut k, men);
    g.stats.stackroom.view_in += k.seen.iter().filter(|b| **b).count() as i32;
    g.stats.stackroom.view_across += k.across.iter().filter(|b| **b).count() as i32;
    if !(0..posts.len()).any(|i| k.moved(i)) && k.kept.is_empty() { return Some(posts.to_vec()); }
    k.ways = ways_kept(g, &x);
    let got = deal(g, &x, &mut k);
    Some(as_posts(g, &x, &got))
}

/// A stack that stands does not creep: a place a man of this order already stands on, no mark as dealt now, is his while it still stands as a place.
fn kept_marks(g: &Game, x: &View, men: &[GuardId]) -> JsMap<usize, V3> {
    let apart = g.k.stackroom.apart;
    let mut kept: JsMap<usize, V3> = JsMap::new();
    let stood = stood_marks(g, x, men);
    let loose: Vec<usize> = (0..x.posts.len()).filter(|i| !stood.iter().any(|q| v3::dist_xz(*q, x.posts[*i].pos) < SAME_M)).collect();
    // (only a place this rule could have dealt: by the door's own wall, no further along it than the rule reaches)
    let mut mine: Vec<V3> = stood.into_iter().filter(|&q| crate::doortools::off_of(g, x.d, q, x.side) < x.jamb_out + 0.5
        && from_middle(g, x, q) <= x.w / 2.0 + g.k.stackroom.view_past + 0.05 && !x.posts.iter().any(|p| v3::dist_xz(p.pos, q) < SAME_M)).collect();
    js::sort_by(&mut mine, |p, q| v3::dist_xz(*p, x.c) - v3::dist_xz(*q, x.c));
    for q in mine {
        if loose.iter().all(|i| kept.has(*i)) { break; }
        if !unseen_place(g, x, q) { continue; }
        let stands = (0..x.posts.len()).any(|i| !loose.contains(&i) && v3::dist_xz(x.posts[i].pos, q) < apart);
        if stands || kept.values().any(|o| v3::dist_xz(*o, q) < apart) { continue; }
        let h = x.hand(g, q);
        let mut free: Vec<(usize, f64)> = loose.iter().filter(|i| !kept.has(**i)).map(|&i| (i, if x.hand(g, x.posts[i].pos) == h { 0.0 } else { 1.0 })).collect();
        js::sort_by(&mut free, |a, b| js::or(a.1 - b.1, v3::dist_xz(x.posts[a.0].pos, q) - v3::dist_xz(x.posts[b.0].pos, q)));
        let u = free[0].0;
        // (nor a body's width from a dealt mark that will stand where it is)
        if loose.iter().any(|&j| j != u && !kept.has(j) && v3::dist_xz(x.posts[j].pos, q) < apart && !in_view(g, x.d, &x.far, x.posts[j].pos)) { continue; }
        kept.set(u, q);
    }
    kept
}
/// The marks at this door the men stand on.
fn stood_marks(g: &Game, x: &View, men: &[GuardId]) -> Vec<V3> {
    let mut stood: Vec<V3> = Vec::new();
    for &man in men {
        let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), man) else { continue };
        if !crate::orders::rows::is(g, a, crate::orders::Is::Stacks) || g.tab.task[a].door != Some(x.d) { continue; }
        let Some(post) = g.tab.task[a].post else { continue };
        if v3::dist_xz(g.pos_of(man), post) < 0.6 && crate::doortools::off_of(g, x.d, post, x.side) > 0.0 { stood.push(post); }
    }
    stood
}
/// How far a place is from the middle of the doorway, along the wall.
fn from_middle(g: &Game, x: &View, q: V3) -> f64 { m::abs(crate::doortools::along_of(g, x.d, q) - x.w / 2.0) }
/// A place the floor beyond does not see, a body's width off what is taken.
fn unseen_place(g: &Game, x: &View, q: V3) -> bool {
    !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && on_floor(g, x.f, q) && !crate::scene_collision::low_blocked(g, x.front, q) && !crate::doorway::doorway_stand(g, q)
        && !x.taken.iter().any(|o| v3::dist_xz(*o, q) < g.k.stackroom.apart) && !in_view(g, x.d, &x.far, q)
}

/// A hand dealt more places than it has men to take them without crossing gives up the ones nearest the frame.
fn across_marks(g: &Game, x: &View, k: &mut Deal, men: &[GuardId]) {
    let s = g.k.stackroom;
    let mut on: Vec<f64> = Vec::new();
    for &man in men {
        let p = g.pos_of(man);
        if !(crate::doortools::off_of(g, x.d, p, x.side) > 0.0) || v3::dist_xz(p, x.c) > s.view_men { continue; }
        let a = crate::doortools::along_of(g, x.d, p);
        on.push(if a >= x.w + s.view_out { 1.0 } else if a <= -s.view_out { -1.0 } else { 0.0 });
    }
    for h in [1.0, -1.0] {
        let room = men.len() as i32 - on.iter().filter(|o| **o == -h).count() as i32;
        let mut mine: Vec<usize> = (0..x.posts.len()).filter(|&i| x.hand(g, k.at(x, i)) == h && from_middle(g, x, k.at(x, i)) < x.w / 2.0 + s.view_past + 1.0).collect();
        js::sort_by(&mut mine, |a, b| v3::dist_xz(k.at(x, *a), x.c) - v3::dist_xz(k.at(x, *b), x.c));
        let over = mine.len() as i32 - room;
        let give: Vec<usize> = mine.into_iter().filter(|i| !k.kept.has(*i)).take(m::max(over as f64, 0.0) as usize).collect();
        for i in give { k.across[i] = true; }
    }
}

/// The ways at the stack's back as the marks stand: a mark that has a line to one keeps a line to it.
fn ways_kept(g: &mut Game, x: &View) -> Vec<Vec<RearWaysItem>> {
    let marks: Vec<V3> = x.posts.iter().map(|p| p.pos).collect();
    let ways = crate::rearwatch::rear_ways(g, x.d, x.side, &marks);
    marks.iter().map(|&p| ways.iter().copied().filter(|way| crate::rearwatch::rear_sees(g, p, way.o, way.front).is_some()).collect()).collect()
}

/// Dealt nearest the frame first. A place with nowhere to go stays and stands in the others' way: the deal is run again with it standing, and at the end
/// every one that stayed is asked again with all the others where they ended up, until none of them moves.
fn deal(g: &mut Game, x: &View, k: &mut Deal) -> JsMap<usize, Dealt> {
    let n = x.posts.len();
    let mut order: Vec<usize> = (0..n).filter(|i| k.moved(*i)).collect();
    js::sort_by(&mut order, |a, b| v3::dist_xz(x.posts[*a].pos, x.c) - v3::dist_xz(x.posts[*b].pos, x.c));
    let mut stay: JsSet<usize> = JsSet::new();
    let mut got = loop {
        match deal_round(g, x, k, &order, &stay) { Ok(got) => break got, Err(i) => { stay.add(i); } }
    };
    loop {
        let mut gone: Option<(usize, Dealt)> = None;
        for &i in &order {
            if !stay.has(i) { continue; }
            let others: Vec<V3> = (0..n).filter(|j| *j != i).map(|j| got.get(j).map_or(k.at(x, j), |x| x.q)).collect();
            if let Some(dealt) = place_for(g, x, k, i, &others) { gone = Some((i, dealt)); break; }
        }
        let Some((i, dealt)) = gone else { return got };
        stay.delete(i);
        got.set(i, dealt);
    }
}
/// One round of it: every mark's place, or the first that has nowhere to go.
fn deal_round(g: &mut Game, x: &View, k: &mut Deal, order: &[usize], stay: &JsSet<usize>) -> Result<JsMap<usize, Dealt>, usize> {
    let mut fixed: Vec<V3> = (0..x.posts.len()).filter(|i| !k.moved(*i) || stay.has(*i)).map(|i| k.at(x, i)).collect();
    let mut got: JsMap<usize, Dealt> = JsMap::new();
    for (i, q) in k.kept.iter() {
        let h = x.hand(g, *q);
        got.set(i, Dealt { q: *q, h, over: h != x.hand(g, x.posts[i].pos) });
    }
    for &i in order {
        if stay.has(i) { continue; }
        let dealt = place_for(g, x, k, i, &fixed).ok_or(i)?;
        fixed.push(dealt.q);
        got.set(i, dealt);
    }
    Ok(got)
}
/// A place for a mark: across the opening first if it is across, else on its own hand if it is in view.
fn place_for(g: &mut Game, x: &View, k: &mut Deal, i: usize, fixed: &[V3]) -> Option<Dealt> {
    let own = x.hand(g, x.posts[i].pos);
    let over = if k.across[i] { place_on(g, x, k, i, -own, fixed) } else { None };
    if let Some(q) = over { return Some(Dealt { q, h: -own, over: true }); }
    if !k.seen[i] { return None; }
    place_on(g, x, k, i, own, fixed).map(|q| Dealt { q, h: own, over: false })
}
/// The first such place on a hand a body clear of every place that stands, not behind glass while another will do.
fn place_on(g: &mut Game, x: &View, k: &mut Deal, i: usize, h: f64, fixed: &[V3]) -> Option<V3> {
    let apart = g.k.stackroom.view_apart;
    let places = k.places[if h > 0.0 { 0 } else { 1 }].get_or_insert_with(|| places_on(g, x, h));
    for shy in [true, false] {
        for &(q, glass) in places.iter() {
            if (shy && glass) || fixed.iter().any(|o| v3::dist_xz(*o, q) < apart) { continue; }
            if k.ways[i].iter().all(|way| crate::rearwatch::rear_sees(g, q, way.o, way.front).is_some()) { return Some(q); }
        }
    }
    None
}
/// The places along a hand of the door's wall the floor beyond does not see, from the jamb out, as far as the door's own wall stands behind them: where it
/// ends, or another doorway opens in it, the file ends too.
fn places_on(g: &mut Game, x: &View, h: f64) -> Vec<(V3, bool)> {
    let s = g.k.stackroom;
    let mut l = Vec::new();
    let mut past = x.jamb_past;
    while past <= s.view_past + 1e-6 {
        let a = if h > 0.0 { x.w + past } else { -past };
        let before = crate::doortools::door_pt(g, x.d, a, x.jamb_out, x.side, Some(0.45));
        let behind = crate::doortools::door_pt(g, x.d, a, -s.view_wall, x.side, Some(0.45));
        if !crate::scene_collision::solid_blocked(g, before, behind, None) { break; }
        for off in [x.jamb_out, s.view_off] {
            let q = crate::doortools::door_pt(g, x.d, a, off, x.side, None);
            if !unseen_place(g, x, q) || furnished(g, x.d, q) { continue; }
            l.push((q, crate::orders::squad_orders_behind_glass(g, x.d, x.side, q)));
        }
        past += s.view_step;
    }
    l
}

/// The marks with the places dealt: each faced down the wall at the doorway, one that crossed in its new hand's file, and the door's one opener the man
/// nearest its frame.
fn as_posts(g: &Game, x: &View, got: &JsMap<usize, Dealt>) -> Vec<StackPost> {
    let th = g.dr[x.d].def.closed_dir;
    let mut out = x.posts.to_vec();
    for (i, dealt) in got.iter() {
        let was = x.posts[i];
        let (face, stack_side) = on_wall(th, x.side, dealt.h);
        // (0 past the far jamb, 1 before the hinge)
        let group = if dealt.over && was.group.is_some() && was.group != Some(2.0) { Some(if dealt.h > 0.0 { 0.0 } else { 1.0 }) } else { was.group };
        out[i] = StackPost { pos: dealt.q, face, stack_side, group, ..was };
    }
    if !got.keys().into_iter().any(|i| x.posts[i].lead) { return out; }
    let mut best = 0;
    for i in 1..out.len() { if v3::dist_xz(out[i].pos, x.c) < v3::dist_xz(out[best].pos, x.c) { best = i; } }
    for (i, p) in out.iter_mut().enumerate() { p.lead = i == best; }
    out
}
