//! The stand-off's marks: where the men wait out a charge on a door.
use super::*;
use crate::orders::StackPost;

/// Marks for `n`: in file along the wall past a jamb, nearest the door first, the file the men's picture of the place calls safest; with no file of the
/// floor for one, whatever else of the floor will do.
pub fn door_tools_standoff_posts(g: &mut Game, d: DoorId, side: f64, n: f64, men: &[GuardId]) -> Option<Vec<StackPost>> {
    let idea = crate::picture::any_idea(g, men);
    let files = whole_files(g, d, side, n, if idea { usize::MAX } else { 1 });
    if !idea || files.is_empty() {
        let was = match files.into_iter().next() { Some(f) => Some(f), None => no_file(g, d, side, n, men) };
        return fixed(g, d, side, was.as_deref(), men);
    }
    let mut whole: Vec<Vec<StackPost>> = Vec::new();
    for f in &files { if let Some(f) = fixed(g, d, side, Some(f), men) { whole.push(f); } }
    if whole.is_empty() { return fixed(g, d, side, Some(&files[0]), men); }
    let ranked = crate::picture::safest_sets(g, men, &whole);
    let pick = ranked.into_iter().find(|&i| walked_to(g, men, d, n, &whole[i])).unwrap_or(0);
    if pick != 0 { g.stats.picture.moved += 1; }
    Some(whole.swap_remove(pick))
}
/// Can the men get to these.
fn walked_to(g: &mut Game, men: &[GuardId], d: DoorId, n: f64, posts: &[StackPost]) -> bool {
    if men.len() as f64 == n { return door_tools_standoff_walk(g, men, d, posts).is_none(); }
    posts.iter().all(|p| men.iter().any(|&m_| crate::orders::squad_orders_reach(g, m_, p.pos).is_none()))
}
/// The marks, moved off anything that stands on them.
fn fixed(g: &mut Game, d: DoorId, side: f64, posts: Option<&[StackPost]>, men: &[GuardId]) -> Option<Vec<StackPost>> {
    let (taken, min_m) = (taken_on(g, d, men), g.k.doortools.backblast_m + 0.2);
    crate::stackroom::stack_room_fix(g, d, side, posts, &taken, Some(min_m))
}
/// The floor at the door that Sam and the men on other orders have.
fn taken_on(g: &mut Game, d: DoorId, men: &[GuardId]) -> Vec<V3> {
    let mut taken = vec![sam_pos(g, &crate::seats::here(g))];
    taken.extend(crate::orders::squad_orders_held_on(g, &crate::seats::here(g), d, men, None));
    taken
}

/// The side of the file a man stands on, from his facing.
fn stack_side_of(ux: f64, uz: f64, side: f64, fx: f64, fz: f64) -> f64 { if (uz * side) * (-fz) + (-ux * side) * fx >= 0.0 { 1.0 } else { -1.0 } }
fn along_wall(g: &Game, d: DoorId) -> (f64, f64) { let th = g.dr[d].def.closed_dir; (m::cos(th), m::sin(th)) }
/// A file out past one jamb, as far as the floor goes.
fn file_of(g: &mut Game, d: DoorId, side: f64, n: f64, off: f64, dir: f64) -> Vec<StackPost> {
    let w = g.dr[d].def.width;
    let (ux, uz) = along_wall(g, d);
    let (jamb_out, gap) = (g.k.orders.jamb_out, g.k.orders.file_gap);
    let front = door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    let mut out = Vec::new();
    let mut k = 0.0;
    while k < n {
        let a = if dir > 0.0 { w + off + k * gap } else { -off - k * gap };
        let pos = door_pt(g, d, a, jamb_out, side, None);
        if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) || crate::scene_collision::solid_blocked(g, front, [pos[0], 0.45, pos[2]], None) { break; }
        let (fx, fz) = (-ux * dir, -uz * dir);
        out.push(StackPost { pos, face: m::atan2(fx, fz), stack_side: stack_side_of(ux, uz, side, fx, fz), lead: false, group: Some(if dir > 0.0 { 0.0 } else { 1.0 }) });
        k += 1.0;
    }
    out
}
/// The files with room for them all, the nearest the door first; `most` of them.
fn whole_files(g: &mut Game, d: DoorId, side: f64, n: f64, most: usize) -> Vec<Vec<StackPost>> {
    let mut files = Vec::new();
    for off in g.k.doortools.standoff.clone() {
        for dir in [1.0, -1.0] {
            let mut file = file_of(g, d, side, n, off, dir);
            if file.len() as f64 != n { continue; }
            for (k, p) in file.iter_mut().enumerate() { (p.lead, p.group) = (k == 0, None); }
            files.push(file);
            if files.len() == most { return files; }
        }
    }
    files
}

/// No one file will take them: half out past each jamb and the rest behind, or the level's own marks, or the floor around the door, or down the hall.
fn no_file(g: &mut Game, d: DoorId, side: f64, n: f64, men: &[GuardId]) -> Option<Vec<StackPost>> {
    if n > 2.0 { if let Some(posts) = both_jambs(g, d, side, n) { return Some(posts); } }
    if let Some(marks) = marks_for(g, d, side, n, men) {
        let (fc, (ux, uz)) = (g.dr[d].frame_centre, along_wall(g, d));
        return Some(marks.iter().enumerate().map(|(k, &p)| {
            let face = v3::yaw_to(p, fc);
            StackPost { pos: v3::copy(p), face, stack_side: stack_side_of(ux, uz, side, m::sin(face), m::cos(face)), lead: k == 0, group: None }
        }).collect());
    }
    let taken = taken_on(g, d, men);
    let from = g.k.doortools.backblast_m + 0.4;
    let hall = crate::orders::squad_orders_hall_places(g, &crate::ask::Over::OWN, d, side, n, from, &mut |_g: &mut Game, q: V3| taken.iter().any(|&h| v3::dist_xz(h, q) < 0.7));
    if hall.len() as f64 != n { return None; }
    Some(hall.iter().enumerate().map(|(k, p)| StackPost { pos: p.pos, face: p.face, stack_side: p.stack_side, lead: k == 0, group: None }).collect())
}
fn both_jambs(g: &mut Game, d: DoorId, side: f64, n: f64) -> Option<Vec<StackPost>> {
    for off in g.k.doortools.standoff.clone() {
        let (near, far) = (file_of(g, d, side, n, off, 1.0), file_of(g, d, side, n, off, -1.0));
        let half = m::min(near.len() as f64, m::ceil(n / 2.0));
        let b = m::min(far.len() as f64, n - half);
        let a = m::min(near.len() as f64, n - b);
        let mut out: Vec<StackPost> = Vec::new();
        for k in 0..m::max(a, b) as usize {
            if (k as f64) < a { out.push(near[k]); }
            if (k as f64) < b { out.push(far[k]); }
        }
        if (out.len() as f64) < n {
            let taken: Vec<V3> = out.iter().map(|o| o.pos).collect();
            let more = crate::orders::squad_orders_rear_posts(g, d, side, n - out.len() as f64, &taken);
            out.extend(more);
        }
        if (out.len() as f64) < n { continue; }
        out[0].lead = true;
        return Some(out);
    }
    None
}
fn marks_for(g: &mut Game, d: DoorId, side: f64, n: f64, men: &[GuardId]) -> Option<Vec<V3>> {
    if let Some(x) = g.sets[g.seat].orders.door_marks.iter().find(|x| x.door == d && x.side == side) {
        return x.standoff.get(..n as usize).map(|s| s.to_vec());
    }
    let (taken, min_m) = (taken_on(g, d, men), g.k.doortools.backblast_m + 0.2);
    crate::orders::squad_orders_worked_pick(g, d, side, n, &taken, Some(min_m))
}

#[cfg(test)]
mod tests {
    use super::stack_side_of;
    /// Which shoulder is to the wall: it turns over with the way he faces along the door. A failure means a man on a standoff post leans out on the wrong side.
    #[test]
    fn a_man_s_stack_side_turns_over_with_the_way_he_faces_along_the_door() {
        // a door along +x, its +1 face
        assert_eq!(stack_side_of(1.0, 0.0, 1.0, -1.0, 0.0), 1.0);
        assert_eq!(stack_side_of(1.0, 0.0, 1.0, 1.0, 0.0), -1.0);
        assert_eq!(stack_side_of(1.0, 0.0, -1.0, 1.0, 0.0), 1.0);    // the other face
        assert_eq!(stack_side_of(1.0, 0.0, 1.0, 0.0, 1.0), 1.0);
    }
}
