//! THE TRAFFIC MAP the look's floors read (render/shaders/detail.wgsl; render/wear.ts says why each number is as it is): two bytes a texel, eight texels a metre over the floor. r = where
//! the level is walked: every way it has, traced along the walk grid's own paths, each kind by its weight, every waypoint stood at and every threshold crossed. g = the band along a wall where
//! stones gather: each standing box lays its own × how long the straight line of faces is that the texel's nearest face is a piece of. A pure function of the level: made once, as the level
//! stands when it is first asked for.
use crate::boxes::Word;
use crate::Row;
use alloc::vec;
use alloc::vec::Vec;
use game::game::Game;
use game::m;
use game::scene_boxes::Box;
use game::scene_level::LevelDoorDef;
use game::scene_scenarios::Rect;

type V3 = [f64; 3];
const RES: f64 = 8.0;
const W: usize = 40 * 8;
const H: usize = 28 * 8;
const LANE_SIGMA: f64 = 0.34;
const DOOR_SIGMA: f64 = 0.55;
const DOOR_AMP: f64 = 1.4;
const TRAFFIC_K: f64 = 1.1;
const GRIME_REACH: f64 = 0.16;
/// what one way counts as, by its kind
const PATROL: f64 = 1.0;
const TEAM: f64 = 0.45;
const DOORS: f64 = 0.3;
const HINT: f64 = 1.0;
const RUN_LO: f64 = 1.25;
const RUN_HI: f64 = 2.5;
const RUN_MEET: f64 = 0.03;
const RUN_STEP: f64 = 0.12;
const RUN_TURN: f64 = 0.9986;
const PATH_ITER: f64 = 6000.0;

fn door_mid(d: &LevelDoorDef) -> V3 { [d.hinge[0] + m::cos(d.closed_dir) * d.width * 0.5, 0.0, d.hinge[1] + m::sin(d.closed_dir) * d.width * 0.5] }
fn mid(r: &Rect) -> V3 { [(r.x0 + r.x1) / 2.0, 0.0, (r.z0 + r.z1) / 2.0] }

/// the map of a level, made once a level
#[derive(Default)]
pub struct Traffic { pub words: Vec<Word>, of: Option<alloc::string::String> }
impl Traffic {
    pub fn take(&mut self, g: &Game) {
        let name = game::scene_level_load::level_name_of(g);
        if self.of == name && !self.words.is_empty() { return; }
        self.of = name;
        fill(g, &mut self.words);
    }
}

/// the rows of the map: a word holds two texels
fn fill(g: &Game, out: &mut Vec<Word>) {
    let mut t = vec![0f32; W * H];
    {
        let mut stamp = |x: f64, z: f64, sigma: f64, amp: f64| {
            let (r, s2) = (sigma * 2.5, 2.0 * sigma * sigma);
            let x0 = m::max(0.0, m::floor((x - r) * RES)) as i64;
            let x1 = m::min(W as f64 - 1.0, m::ceil((x + r) * RES)) as i64;
            let z0 = m::max(0.0, m::floor((z - r) * RES)) as i64;
            let z1 = m::min(H as f64 - 1.0, m::ceil((z + r) * RES)) as i64;
            for tz in z0..=z1 {
                let dz = (tz as f64 + 0.5) / RES - z;
                for tx in x0..=x1 {
                    let dx = (tx as f64 + 0.5) / RES - x;
                    let d2 = dx * dx + dz * dz;
                    if d2 <= r * r { let i = tz as usize * W + tx as usize; t[i] += (amp * m::exp(-d2 / s2)) as f32; }
                }
            }
        };
        // every way the level is walked, each traced along its own line, by its kind's weight
        for (amp, pts) in ways(g) { for k in 0..pts.len().saturating_sub(1) {
            let (a, b) = (pts[k], pts[k + 1]);
            let len = m::hyp2(b[0] - a[0], b[2] - a[2]);
            let n = m::max(1.0, m::ceil(len / 0.1));
            let per = amp * (len / n) / (LANE_SIGMA * 2.5);
            let mut i = 0.0;
            while i <= n { let s = i / n; stamp(a[0] + (b[0] - a[0]) * s, a[2] + (b[2] - a[2]) * s, LANE_SIGMA, per); i += 1.0; }
        } }
        // a man stands and turns at a waypoint for his wait
        for &r in &g.level.routes { let route = &g.tab.route[r]; if route.points.len() < 2 { continue; }
            for (i, p) in route.points.iter().enumerate() { stamp(p[0], p[2], LANE_SIGMA * 1.3, 0.35 + m::min(1.0, route.wait.get(i).copied().unwrap_or(0.0) / 20.0)); } }
        // thresholds: everybody funnels through a doorway
        for d in &g.level.doors { let c = door_mid(d); stamp(c[0], c[2], DOOR_SIGMA, DOOR_AMP); }
    }
    let boxes: Vec<&Box> = g.col.boxes.iter().map(|b| &g.tab.boxes[*b]).collect();
    let band = band_by_faces(&boxes);
    let byte = |v: f64| m::min(255.0, m::max(0.0, m::round(v * 255.0))) as u32;
    out.clear();
    out.resize(W * H / 2, Word::ZERO);
    for i in 0..W * H {
        let traffic = 1.0 - m::exp(-TRAFFIC_K * t[i] as f64);
        let two = byte(traffic) | byte(band[i]) << 8;
        out[i / 2].w |= two << (i % 2 * 16);
    }
}

/// every way the level is walked: what a metre of it counts as, and its line on the floor. Each is the walk grid's own path, asked of a copy of the game: the search's tables are the module's, and this takes the game shared.
fn ways(g: &Game) -> Vec<(f64, Vec<V3>)> {
    let mut asked = g.clone();
    let mut out: Vec<(f64, Vec<V3>)> = Vec::new();
    let mut walk = |amp: f64, a: V3, b: V3, out: &mut Vec<(f64, Vec<V3>)>| {
        let pts = match game::scene_collision::nav_grid_find_path(&mut asked, a, b, Some(PATH_ITER), None) {
            Some(p) if !p.is_empty() => { let mut l = Vec::with_capacity(p.len() + 1); l.push(a); l.extend(p); l }
            _ => vec![a, b],
        };
        out.push((amp, pts));
    };
    let level = &g.level;
    let door_at = |id: &str| level.doors.iter().find(|d| d.name == id).map(door_mid);
    // the shift's routes
    for &r in &level.routes { let p = &g.tab.route[r].points; if p.len() < 2 { continue; } for i in 0..p.len() { walk(PATROL, p[i], p[(i + 1) % p.len()], &mut out); } }
    // every start to everything a team goes for
    let sc = level.scenarios.as_ref();
    let (mut starts, mut goals): (Vec<V3>, Vec<V3>) = (Vec::new(), Vec::new());
    if let Some(s) = sc {
        if let Some(c) = &s.room_clear {
            if let Some(t) = &c.team { starts.push(t.sam); }
            for t in c.starts.iter().flatten() { starts.push(t.sam); }
        }
        if let Some(r) = &s.rescue { starts.push(r.team.sam); }
        if let Some(n) = &s.night_breach { starts.push(n.team.sam); }
        if let Some(q) = &s.quiet { for t in &q.starts { starts.push(t.sam); } }
        if let Some(c) = &s.room_clear { for r in &c.rooms { goals.push(mid(&r.bounds)); } }
        if let Some(r) = &s.rescue { if let Some(p) = r.room.bounds.as_ref().map(mid).or_else(|| door_at(&r.room.door)) { goals.push(p); } }
        if let Some(q) = &s.quiet {
            for a in &q.spots { goals.push(mid(&a.area)); }
            for e in &q.exits { goals.push([e.x, 0.0, e.z]); }
        }
    }
    if starts.is_empty() { starts.push(level.player_spawn); }
    for s in &starts { for t in &goals { walk(TEAM, *s, *t, &mut out); } }
    // the yard's own ways: every pair of the level's outer doors
    let outer: Vec<&LevelDoorDef> = level.doors.iter().filter(|d| d.exterior == Some(true)).collect();
    for i in 0..outer.len() { for k in i + 1..outer.len() { walk(DOORS, door_mid(outer[i]), door_mid(outer[k]), &mut out); } }
    // the author's own, by name: a door's, a mark's, a route's first point, a room clear room's, a place's
    let named = |n: &str| -> Option<V3> {
        if n == "spawn" { return Some(level.player_spawn); }
        if let Some(d) = door_at(n) { return Some(d); }
        if let Some(k) = level.marks.iter().flatten().find(|x| x.name == n) { return Some(k.pos); }
        if let Some(r) = level.routes.iter().map(|r| &g.tab.route[*r]).find(|r| r.name == n) { if let Some(p) = r.points.first() { return Some(*p); } }
        let s = sc?;
        if let Some(r) = s.room_clear.as_ref().and_then(|c| c.rooms.iter().find(|x| x.id == n || x.name == n)) { return Some(mid(&r.bounds)); }
        s.places.as_ref().and_then(|p| p.areas.iter().find(|x| x.name == n)).map(|a| mid(&a.bounds))
    };
    for h in sc.and_then(|s| s.walked.as_ref()).into_iter().flatten() {
        if let (Some(a), Some(b)) = (named(&h.from), named(&h.to)) { walk(HINT * h.weight.unwrap_or(1.0), a, b, &mut out); }
    }
    out
}

/// a static whose foot is on the floor: not a glow card or a decal, not a slab or a rug, not something up on a wall
fn stands_on(b: &Box) -> bool { (b.flags as i64 & 2) == 0 && b.c[1] - b.h[1] <= 0.06 && b.c[1] + b.h[1] >= 0.25 }

struct Face { x: f64, z: f64, nx: f64, nz: f64, tx: f64, tz: f64, a: f64, at: usize }
struct Facing { deg: f64, key: i64, d: Vec<f64>, f: Vec<Face> }

/// HOW LONG THE STRAIGHT LINE OF FACES IS THAT EACH FACE OF EACH BOX IS A PIECE OF (m; four a box: its +x, −x, +z, −z sides; 0 where the box does not stand on the floor). Asked of one face: every
/// standing box's face that looks the same way and lies within RUN_STEP of this face's own plane is on its line; laid end to end along it, the stretch this face is in is its run.
fn face_runs(boxes: &[&Box]) -> Vec<f32> {
    let mut out = vec![0f32; boxes.len() * 4];
    let mut lists: Vec<Facing> = Vec::new();
    for (i, b) in boxes.iter().enumerate() {
        if !stands_on(b) { continue; }
        let (c, s) = (m::cos(b.yaw), m::sin(b.yaw));
        for k in 0..4 {
            let (sg, on_x) = (if k & 1 != 0 { -1.0 } else { 1.0 }, k < 2);
            let (nx, nz, d) = (sg * if on_x { c } else { s }, sg * if on_x { -s } else { c }, if on_x { b.h[0] } else { b.h[2] });
            let f = Face { x: b.c[0] + nx * d, z: b.c[2] + nz * d, nx, nz, tx: if on_x { s } else { c }, tz: if on_x { c } else { -s }, a: if on_x { b.h[2] } else { b.h[0] }, at: i * 4 + k };
            let deg = m::atan2(nz, nx) * 180.0 / m::PI;
            let key = m::round(deg * 1000.0) as i64;
            match lists.iter_mut().find(|l| l.key == key) { Some(l) => l.f.push(f), None => lists.push(Facing { deg, key, d: Vec::new(), f: vec![f] }) }
        }
    }
    let turn = m::acos(RUN_TURN) * 180.0 / m::PI + 0.002;
    for l in lists.iter_mut() {
        let (nx, nz) = (l.f[0].nx, l.f[0].nz);
        l.f.sort_by(|a, b| (a.x * nx + a.z * nz).total_cmp(&(b.x * nx + b.z * nz)));
        l.d = l.f.iter().map(|f| f.x * nx + f.z * nz).collect();
    }
    let mut span: Vec<f64> = Vec::new();
    for oi in 0..lists.len() {
        for fi in 0..lists[oi].f.len() {
            let own = &lists[oi];
            let f = &own.f[fi];
            span.clear();
            let take = |o: &Face, span: &mut Vec<f64>| {
                if core::ptr::eq(o, f) || f.nx * o.nx + f.nz * o.nz < RUN_TURN { return; }
                let (dx, dz) = (o.x - f.x, o.z - f.z);
                let (off, lean) = (dx * f.nx + dz * f.nz, (o.tx * f.nx + o.tz * f.nz) * o.a);
                if m::abs(off) + m::abs(lean) > RUN_STEP { return; }
                let (mid, len) = (dx * f.tx + dz * f.tz, m::abs(o.tx * f.tx + o.tz * f.tz) * o.a);
                span.push(mid - len);
                span.push(mid + len);
            };
            let mut k = fi;
            while k > 0 { k -= 1; if own.d[fi] - own.d[k] > RUN_STEP + 0.01 { break; } take(&own.f[k], &mut span); }
            let mut k = fi + 1;
            while k < own.f.len() { if own.d[k] - own.d[fi] > RUN_STEP + 0.01 { break; } take(&own.f[k], &mut span); k += 1; }
            for (li, l) in lists.iter().enumerate() {
                if li == oi { continue; }
                let dd = m::abs(l.deg - own.deg);
                if m::min(dd, 360.0 - dd) <= turn { for o in &l.f { take(o, &mut span); } }
            }
            // grow this face's own stretch by every stretch that touches it, until none does
            let (mut lo, mut hi) = (-f.a, f.a);
            loop {
                let mut grew = false;
                for s in span.chunks_exact(2) {
                    if s[0] > hi + RUN_MEET || s[1] < lo - RUN_MEET || (s[0] >= lo && s[1] <= hi) { continue; }
                    lo = m::min(lo, s[0]);
                    hi = m::max(hi, s[1]);
                    grew = true;
                }
                if !grew { break; }
            }
            out[f.at] = (hi - lo) as f32;
        }
    }
    out
}

/// THE BAND: each standing box lays its own, exact from its own rectangle, × how long the face is that the texel is nearest to; a texel keeps the most any box gives it
fn band_by_faces(boxes: &[&Box]) -> Vec<f64> {
    let mut prox = vec![0f64; W * H];
    let run = face_runs(boxes);
    let (half, pad) = (0.5 / RES, GRIME_REACH + 0.5 / RES);
    let weigh = |v: f32| { let t = m::min(1.0, m::max(0.0, (v as f64 - RUN_LO) / (RUN_HI - RUN_LO))); t * t * (3.0 - 2.0 * t) };
    for (bi, b) in boxes.iter().enumerate() {
        if !stands_on(b) { continue; }
        let k = [weigh(run[bi * 4]), weigh(run[bi * 4 + 1]), weigh(run[bi * 4 + 2]), weigh(run[bi * 4 + 3])];
        if k.iter().all(|v| *v <= 0.0) { continue; }
        let (c, s) = (m::cos(b.yaw), m::sin(b.yaw));
        let (ex, ez) = (m::abs(c) * b.h[0] + m::abs(s) * b.h[2] + pad, m::abs(s) * b.h[0] + m::abs(c) * b.h[2] + pad);
        let x0 = m::max(0.0, m::floor((b.c[0] - ex) * RES)) as i64;
        let x1 = m::min(W as f64 - 1.0, m::ceil((b.c[0] + ex) * RES)) as i64;
        let z0 = m::max(0.0, m::floor((b.c[2] - ez) * RES)) as i64;
        let z1 = m::min(H as f64 - 1.0, m::ceil((b.c[2] + ez) * RES)) as i64;
        for tz in z0..=z1 {
            let pz = (tz as f64 + 0.5) / RES - b.c[2];
            for tx in x0..=x1 {
                let px = (tx as f64 + 0.5) / RES - b.c[0];
                let (lx, lz) = (c * px - s * pz, s * px + c * pz);
                let (dx, dz) = (m::abs(lx) - b.h[0], m::abs(lz) - b.h[2]);
                let face = if dx >= dz { if lx >= 0.0 { 0 } else { 1 } } else if lz >= 0.0 { 2 } else { 3 };
                let v = k[face] * m::max(0.0, 1.0 - m::max(0.0, m::hyp2(m::max(dx, 0.0), m::max(dz, 0.0)) - half) / GRIME_REACH);
                let i = tz as usize * W + tx as usize;
                if v > prox[i] { prox[i] = v; }
            }
        }
    }
    prox
}
