//! THE ORDER THE STATICS ARE DRAWN IN: tops falling, so that what stands higher takes the depth before what lies under it is shaded, and the list's own order
//! between equal tops; except that a box waits for every earlier one it shares a face's plane with, where the depth test keeps the first drawn. Two faces
//! share a plane when they look the same way, lie within `PLANE` of one and their bounds lie over one another by more than `PATCH`.
//! The order is made again once the statics have stood still for `CALM` drawn frames; until then they are drawn in the list's own order, which every tie is
//! defined by: the same picture, and more of it shaded.
use super::pack;
use alloc::vec;
use alloc::vec::Vec;
use game::m;
use game::scene_boxes::Box;
use game::v3::{quat, V3};

const PLANE: f64 = 0.00025;
const PATCH: f64 = 0.0005;
const CALM: u32 = 10;

#[derive(Default)]
pub struct Rank { of: Vec<u32>, stale: bool, still: u32, made: bool }

struct Face { of: u32, along: f64, n: V3, lo: V3, hi: V3 }

impl Rank {
    pub fn stale(&mut self) { self.stale = true; self.still = 0; }
    /// the order is to be made in this frame
    pub fn due(&self) -> bool { self.stale && (!self.made || self.still >= CALM) }
    /// `list`, statics by number, put in the order they are drawn in; false = the order is stale, and the list is left in its own. `boxes` = every static, where it is due
    pub fn order(&mut self, boxes: &[&Box], list: &mut Vec<u32>) -> bool {
        if self.due() { self.of = ranked(boxes); self.stale = false; self.made = true; }
        else if self.stale { self.still += 1; return false; }
        list.sort_unstable_by_key(|&i| self.of[i as usize]);
        true
    }
}

fn axes(b: &Box) -> [V3; 3] {
    match b.rot {
        Some(r) => [quat::rotate(r, [1.0, 0.0, 0.0]), quat::rotate(r, [0.0, 1.0, 0.0]), quat::rotate(r, [0.0, 0.0, 1.0])],
        None => { let (c, s) = (m::cos(b.yaw), m::sin(b.yaw)); [[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]] }
    }
}
fn cut_away(b: &Box) -> bool { b.rot.is_none() && pack::flags(b) & pack::CUTAWAY != 0 && b.cut_height < b.c[1] + b.h[1] }
fn top(b: &Box) -> f32 {
    if cut_away(b) { return b.cut_height as f32; }
    let a = axes(b);
    let up = if b.rot.is_some() { m::abs(a[0][1]) * b.h[0] + m::abs(a[1][1]) * b.h[1] + m::abs(a[2][1]) * b.h[2] } else { b.h[1] };
    b.c[1] as f32 + up as f32
}

/// each static's place in the order
fn ranked(all: &[&Box]) -> Vec<u32> {
    let n = all.len();
    let tops: Vec<f32> = all.iter().map(|b| top(b)).collect();
    let mut ord: Vec<u32> = (0..n as u32).collect();
    ord.sort_by(|&a, &b| tops[b as usize].total_cmp(&tops[a as usize]).then(a.cmp(&b)));
    let mut turn = vec![0u32; n];
    for (r, &i) in ord.iter().enumerate() { turn[i as usize] = r as u32; }
    // every face that can be seen (one that looks down is not)
    let mut faces: Vec<Face> = Vec::new();
    for (i, b) in all.iter().enumerate() {
        if pack::flags(b) & (pack::NO_PRIMARY | pack::GLASS) != 0 { continue; }
        let a = axes(b);
        let cut = cut_away(b);
        let hy = if cut { m::max(0.0, (b.cut_height - (b.c[1] - b.h[1])) / 2.0) } else { b.h[1] };
        let c = [b.c[0], if cut { b.c[1] - b.h[1] + hy } else { b.c[1] }, b.c[2]];
        let h = [b.h[0], hy, b.h[2]];
        for u in 0..3 {
            for sg in [-1.0, 1.0] {
                let nu = a[u];
                if nu[1] * sg < -0.5 { continue; }
                let (v, w, hv, hw) = (a[(u + 1) % 3], a[(u + 2) % 3], h[(u + 1) % 3], h[(u + 2) % 3]);
                let mid: V3 = core::array::from_fn(|k| c[k] + sg * h[u] * nu[k]);
                let ext: V3 = core::array::from_fn(|k| m::abs(v[k]) * hv + m::abs(w[k]) * hw);
                faces.push(Face { of: i as u32, along: sg * (nu[0] * mid[0] + nu[1] * mid[1] + nu[2] * mid[2]), n: nu.map(|x| sg * x), lo: core::array::from_fn(|k| mid[k] - ext[k]), hi: core::array::from_fn(|k| mid[k] + ext[k]) });
            }
        }
    }
    faces.sort_by(|p, q| p.along.total_cmp(&q.along));
    // who waits for whom: a later box for each earlier one whose face it shares
    let mut after: Vec<(u32, u32)> = Vec::new();
    for i in 0..faces.len() {
        let p = &faces[i];
        for q in faces[i + 1..].iter().take_while(|q| q.along <= p.along + PLANE) {
            if p.of == q.of || p.n[0] * q.n[0] + p.n[1] * q.n[1] + p.n[2] * q.n[2] < 1.0 - 1e-9 { continue; }
            let (mut over, mut apart) = (0, false);
            for d in 0..3 { let o = m::min(p.hi[d], q.hi[d]) - m::max(p.lo[d], q.lo[d]); if o > PATCH { over += 1; } else if o < -PLANE { apart = true; } }
            if !apart && over >= 2 { after.push((p.of.min(q.of), p.of.max(q.of))); }
        }
    }
    if after.is_empty() { return turn; }
    // a box takes the turn of the latest it waits for: every wait is for an earlier one, so in the list's order each turn is settled before it is passed on
    after.sort_unstable();
    for &(first, then) in &after { turn[then as usize] = turn[then as usize].max(turn[first as usize]); }
    ord.sort_by(|&a, &b| turn[a as usize].cmp(&turn[b as usize]).then(a.cmp(&b)));
    let mut rank = vec![0u32; n];
    for (r, &i) in ord.iter().enumerate() { rank[i as usize] = r as u32; }
    rank
}
