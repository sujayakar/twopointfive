//! src/game/roomfit.ts lines 1 to 255
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `x.toFixed(1)`: the nearest tenth by the double's exact value, and of two equally near the larger (Rust's own `{:.1}` takes the even one); −0 has no sign.
fn to_fixed_1(x: f64) -> String {
    let a = m::abs(x);
    // a double is exactly half way between two tenths only as an odd number of quarters
    let q = a * 4.0;
    let tie = m::floor(q) == q && js::rem(q, 2.0) == 1.0;
    let s = if tie { alloc::format!("{:.1}", a + 0.01) } else { alloc::format!("{:.1}", a) };
    if x < 0.0 { alloc::format!("-{}", s) } else { s }
}

/// The bearing's name of a yaw. (roomfit.ts compass)
pub fn compass(yaw: f64) -> String {
    let i = js::rem(js::rem(m::round(yaw / (m::PI / 4.0)), 8.0) + 8.0, 8.0);
    crate::roomfit::compass_table().swap_remove(i as usize)
}

/// A row's words for a watch on a way that is no door's: the place by the level's own name, else what it is. (roomfit.ts wayWords)
pub fn way_words(g: &mut crate::game::Game, at: V3, to: V3, yaw: f64) -> String {
    match crate::dialogue::watched_place(g, at, to) {
        Some(p) if !p.is_empty() => alloc::format!("on the way to {}", p),
        _ => String::from("on the opening"),
    }
}

/// A row's words for a watch down a run of floor. (roomfit.ts runWords)
pub fn run_words(g: &mut crate::game::Game, at: V3, to: V3, yaw: f64) -> String {
    match crate::dialogue::watched_place(g, at, to) {
        Some(p) if !p.is_empty() => alloc::format!("watching {}", p),
        _ => String::from("watching the far end"),
    }
}

/// How far `p` is from the stretch a → b in plan, only between its ends. (roomfit.ts offLine)
pub fn off_line(p: V3, a: V3, b: V3) -> f64 {
    let bx = b[0] - a[0];
    let bz = b[2] - a[2];
    let l2 = bx * bx + bz * bz;
    if l2 < 1e-6 { return m::INF; }
    let k = ((p[0] - a[0]) * bx + (p[2] - a[2]) * bz) / l2;
    if k < 0.08 || k > 1.0 { return m::INF; }
    m::hyp2(a[0] + bx * k - p[0], a[2] + bz * k - p[2])
}

/// How much the way Sam pointed adds in front and takes off behind. (roomfit.ts threatsOf frontOf)
fn front_of(front_k: f64, yaw: f64, front: f64) -> f64 {
    1.0 + front_k * m::cos(v3::wrap_angle(yaw - front))
}

/// What there is to watch from the space round `at`, heaviest first. (roomfit.ts threatsOf)
pub fn threats_of(g: &mut crate::game::Game, r: &crate::roomread::RoomRead, at: V3, front: f64, reach: Option<f64>) -> Vec<crate::roomfit::Threat> {
    let k = g.k.roomfit;
    let reach = reach.unwrap_or(k.reach);
    let mut out: Vec<crate::roomfit::Threat> = Vec::new();
    for &way in r.ways.iter() {
        if way.dist > reach { continue; }
        let (o_at, o_door) = { let o = &g.tab.cover_opening[way.o]; (o.at, o.door) };
        let yaw = v3::yaw_to(at, o_at);
        let w = k.near_m / (k.near_m + way.dist)
            * (if way.far_known { k.known_k } else { 1.0 })
            * (if way.shut { k.shut_k } else { 1.0 })
            * (if way.locked { k.locked_k } else { 1.0 })
            * (if way.dist > 0.5 { front_of(k.front_k, yaw, front) } else { 1.0 });
        let key = g.tab.cover_opening[way.o].id.clone();
        let words = match o_door {
            Some(d) => alloc::format!("on the {}", crate::orders::door_words(g, d)),
            None => way_words(g, at, way.beyond, yaw),
        };
        out.push(crate::roomfit::Threat { at: [o_at[0], CHEST, o_at[2]], w, way: Some(way), key, words });
    }
    // floor that runs on: the bearings from the point a man sees furthest along, where no way's line already stands for them
    const B: usize = 24;
    let step = 2.0 * m::PI / (B as f64);
    let eye: V3 = [at[0], EYE, at[2]];
    let mut run: Vec<f64> = Vec::with_capacity(B);
    for b in 0..B {
        let hit = crate::scene_collision::raycast(g, eye, [m::sin((b as f64) * step), 0.0, m::cos((b as f64) * step)], 14.0);
        run.push(match hit { Some(h) => h.t, None => 14.0 });
    }
    let sm = |b: usize| (run[(b + B - 1) % B] + 2.0 * run[b] + run[(b + 1) % B]) / 4.0;
    for b in 0..B {
        let t = run[b];
        if t < k.sector_m || sm(b) < sm((b + 1) % B) || sm(b) <= sm((b + B - 1) % B) { continue; } // (a peak of the smoothed run)
        let yaw = v3::wrap_angle((b as f64) * step);
        // a way stands for the run only where it lies ACROSS it
        let stood_for = r.ways.iter().any(|w| {
            w.dist < t + 0.5
                && m::abs(v3::wrap_angle(v3::yaw_to(at, g.tab.cover_opening[w.o].at) - yaw)) < k.sector_deg * DEG
                && (!js::truthy(k.sector_across) || -(w.in_n[0] * m::sin(yaw) + w.in_n[1] * m::cos(yaw)) > k.sector_across)
        });
        if stood_for { continue; }
        if out.iter().any(|o| o.way.is_none() && m::abs(v3::wrap_angle(v3::yaw_to(at, o.at) - yaw)) < k.sector_deg * DEG) { continue; }
        let look = m::min(t - 0.6, k.sector_look);
        let p: V3 = [at[0] + m::sin(yaw) * look, CHEST, at[2] + m::cos(yaw) * look];
        let far: V3 = [at[0] + m::sin(yaw) * (t - 0.8), 0.0, at[2] + m::cos(yaw) * (t - 0.8)];
        let known = crate::cleared::cleared_rooms_known_at(g, far);
        let w = k.sector_k * m::min(1.0, (t - k.sector_m + 1.5) / 9.0)
            * (if known { if t > k.sector_far_m { k.sector_far_k } else { k.known_k } } else { 1.0 })
            * front_of(k.front_k, yaw, front);
        let key = alloc::format!("floor:{}@{},{}", b, to_fixed_1(at[0]), to_fixed_1(at[2]));
        let words = run_words(g, at, far, yaw);
        out.push(crate::roomfit::Threat { at: p, w, way: None, key, words });
    }
    js::sort_by(&mut out, |a, b| js::or(b.w - a.w, if a.key < b.key { -1.0 } else { 1.0 }));
    out
}

/// Has a man stood at `q` a line to it: a door, to just short of its frame and, its leaf open, on into the doorway; else to the point. (roomfit.ts sees)
pub fn sees(g: &mut crate::game::Game, q: V3, t: &crate::roomfit::Threat) -> bool {
    let eye: V3 = [q[0], EYE, q[2]];
    let d = v3::dist_xz(q, t.at);
    if d < 0.3 { return true; }
    let door: Option<DoorId> = match t.way { Some(w) => g.tab.cover_opening[w.o].door, None => None };
    let k = if door.is_some() { m::max(0.0, 1.0 - 0.7 / d) } else { m::max(0.0, 1.0 - 0.3 / d) };
    if crate::scene_collision::segment_blocked(g, eye, [q[0] + (t.at[0] - q[0]) * k, CHEST, q[2] + (t.at[2] - q[2]) * k], None, None, None) { return false; }
    let Some(door) = door else { return true; };
    !(!crate::doors::door_off(g, door) && m::abs(crate::doors::door_angle(g, door)) >= 0.5 && crate::scene_collision::segment_blocked(g, eye, t.at, None, None, None))
}

/// What a man stood at `p` with his eyes spare could watch, heaviest first. (roomfit.ts watchablesAt)
pub fn watchables_at(g: &mut crate::game::Game, p: V3, front: f64, reach: f64, bodies: &[V3]) -> Vec<crate::roomfit::Watchable> {
    let k = g.k.roomfit;
    let at: V3 = [p[0], 0.0, p[2]];
    let r = crate::roomread::room_read(g, at, Some(reach), Some(at), None);
    let mut out: Vec<crate::roomfit::Watchable> = Vec::new();
    for t in threats_of(g, &r, at, front, Some(reach)).iter() {
        let dist = v3::dist_xz(at, t.at);
        if dist <= 1.0 || !sees(g, at, t) || bodies.iter().any(|b| off_line(*b, at, t.at) < k.lane_m) { continue; }
        out.push(crate::roomfit::Watchable { at: t.at, w: t.w, key: t.key.clone(), yaw: v3::yaw_to(at, t.at), dist, way: t.way.is_some() });
    }
    out
}

/// Is `q` in the funnel of this way, on the space's own face of it. (roomfit.ts inFunnel)
pub fn in_funnel(g: &mut crate::game::Game, q: V3, w: crate::roomread::RoomWay) -> bool {
    let k = g.k.roomfit;
    let (o_at, o_half) = { let o = &g.tab.cover_opening[w.o]; (o.at, o.half) };
    let x = q[0] - o_at[0];
    let z = q[2] - o_at[2];
    let dn = x * w.in_n[0] + z * w.in_n[1];
    if dn < -0.2 || dn > k.funnel_m { return false; }
    m::abs(x * w.in_n[1] - z * w.in_n[0]) < o_half + k.funnel_pad + k.funnel_flare * m::max(0.0, dn)
}

/// A man's Fit for a place and what he watches there. (roomfit.ts fitSpots mk)
fn mk(at: V3, pos: V3, t: Option<&crate::roomfit::Threat>, front: f64) -> crate::roomfit::Fit {
    crate::roomfit::Fit {
        at: v3::copy(at),
        pos,
        watch: t.map(|t| v3::copy(t.at)),
        face: match t { Some(t) => v3::yaw_to(pos, t.at), None => front },
        words: match t { Some(t) => t.words.clone(), None => String::new() },
        key: t.map(|t| t.key.clone()),
        w: match t { Some(t) => t.w, None => 0.0 },
        front,
        on: false,
        next_t: 0.0,
        sig: String::new(),
    }
}

/// Counts what a deal gave each man. (roomfit.ts fitSpots count)
fn count(g: &mut Game, fits: Vec<crate::roomfit::Fit>) -> Vec<crate::roomfit::Fit> {
    for f in fits.iter() {
        match f.key.as_deref() {
            None | Some("") => g.stats.roomfit.plain += 1,
            Some(key) if key.starts_with("floor:") => g.stats.roomfit.sector += 1,
            Some(_) => g.stats.roomfit.way += 1,
        }
    }
    fits
}

/// `held.includes(key)`
fn held_has(held: &[String], key: &str) -> bool {
    held.iter().any(|h| h.as_str() == key)
}

/// Clear of the furniture that can be shoved, where it stands now. (roomfit.ts fitSpots clear)
fn clear(g: &mut Game, q: V3) -> bool {
    for i in 0..g.props.props.len() {
        let pr = g.props.props[i];
        let dx = q[0] - crate::props::prop_x(g, pr);
        let dz = q[2] - crate::props::prop_z(g, pr);
        let yaw = crate::props::prop_yaw(g, pr);
        let c = m::cos(yaw);
        let sn = m::sin(yaw);
        let (hx, hz) = { let row = &g.tab.props_prop[pr]; (row.hx, row.hz) };
        if m::hyp2(m::max(0.0, m::abs(c * dx - sn * dz) - hx), m::max(0.0, m::abs(sn * dx + c * dz) - hz)) < 0.3 { return false; }
    }
    true
}

/// A wall or furniture `d` metres off along `yaw`. (roomfit.ts fitSpots wallAt)
fn wall_at(g: &mut Game, q: V3, yaw: f64, d: f64) -> bool {
    crate::scene_collision::nav_grid_is_blocked(g, q[0] + m::sin(yaw) * d, q[2] + m::cos(yaw) * d)
}

/// How good `q` is for a man whose job is `t`; `out` without its row `me` is the others. (roomfit.ts fitSpots score)
fn score(g: &mut Game, r: &crate::roomread::RoomRead, at: V3, front: f64, bodies: &[V3], q: V3, t: Option<&crate::roomfit::Threat>, out: &[Option<crate::roomfit::Fit>], me: usize) -> f64 {
    let k = g.k.roomfit;
    let mut s0 = 0.0;
    for b in bodies.iter() {
        if v3::dist_xz(*b, q) < k.apart * 0.8 { return -m::INF; }
        if let Some(t) = t { if off_line(*b, q, t.at) < k.lane_m { s0 -= k.lane_k; } }
    }
    let mut s = s0 - k.near_k * m::max(0.0, v3::dist_xz(q, at) - k.near_free);
    for (j, o) in out.iter().enumerate() {
        if j == me { continue; }
        let Some(o) = o else { continue; };
        let d = v3::dist_xz(o.pos, q);
        if d < k.apart { return -m::INF; }
        if d < k.loose { s -= (k.loose - d) * k.loose_k; }
        if let Some(watch) = o.watch { if off_line(q, o.pos, watch) < k.lane_m { s -= k.lane_k; } }
        if let Some(t) = t { if off_line(o.pos, q, t.at) < k.lane_m { s -= k.lane_k; } }
    }
    for &w in r.ways.iter() {
        if in_funnel(g, q, w) {
            // (`t?.way === W`: one RoomWay a way in a read, so by its opening)
            let own = match t.and_then(|t| t.way) { Some(tw) => tw.o == w.o, None => false };
            s -= k.funnel_k * (if own { k.own_k } else { 1.0 }) * (if w.far_known { k.known_funnel_k } else { 1.0 }) * (if w.shut { k.shut_funnel_k } else { 1.0 });
        }
    }
    let yaw = match t { Some(t) => v3::yaw_to(q, t.at), None => front };
    let back = yaw + m::PI;
    if wall_at(g, q, back, 0.55) || wall_at(g, q, back, 0.9) {
        s += k.wall_k;
        if wall_at(g, q, yaw + m::PI / 2.0, 0.55) || wall_at(g, q, yaw - m::PI / 2.0, 0.55) { s += k.corner_k; }
    }
    if let Some(t) = t {
        let d = v3::dist_xz(q, t.at);
        if d < k.close_m { s -= (k.close_m - d) * k.close_k; }
        if let Some(w) = t.way {
            let o_at = g.tab.cover_opening[w.o].at;
            let x = q[0] - o_at[0];
            let z = q[2] - o_at[2];
            let dn = x * w.in_n[0] + z * w.in_n[1];
            let a = m::acos(m::max(-1.0, m::min(1.0, dn / m::max(1e-3, m::hyp2(x, z))))) / DEG;
            s += k.ang_k * (if a < k.ang_lo { a / k.ang_lo } else if a <= k.ang_hi { 1.0 } else if a < k.ang_max { (k.ang_max - a) / (k.ang_max - k.ang_hi) } else { 0.0 });
        }
    }
    s
}

/// The best place on the floor `c` for row `i`'s job `t`. (roomfit.ts fitSpots pick)
fn pick(g: &mut Game, r: &crate::roomread::RoomRead, at: V3, front: f64, bodies: &[V3], c: &[V3], t: Option<&crate::roomfit::Threat>, out: &[Option<crate::roomfit::Fit>], i: usize) -> Option<crate::roomfit::Fit> {
    let mut best: Option<V3> = None;
    let mut bs = -m::INF;
    for &q in c.iter() {
        if let Some(t) = t { if !sees(g, q, t) { continue; } }
        let s = score(g, r, at, front, bodies, q, t, out, i);
        if s > bs { bs = s; best = Some(q); }
    }
    best.map(|b| mk(at, b, t, front))
}

/// THE DEAL: a place and a watch for each of `n` men sent to `at`. None: no floor there to deal. (roomfit.ts fitSpots)
pub fn fit_spots(g: &mut crate::game::Game, at: V3, n: f64, front: f64, from: V3, bodies: Option<&[V3]>, held: Option<&[String]>) -> Option<Vec<crate::roomfit::Fit>> {
    let bodies: &[V3] = bodies.unwrap_or(&[]);
    let held: &[String] = held.unwrap_or(&[]);
    let k = g.k.roomfit;
    if n < 1.0 { return None; }
    let (cs, nav_w) = (g.col.nav.cell, g.col.nav.w);
    let r = crate::roomread::room_read(g, at, Some(k.reach), Some(from), None);
    if r.cells.is_empty() { return None; }
    let threats = threats_of(g, &r, at, front, None);
    g.stats.roomfit.deals += 1;
    if n == 1.0 { // (one man: onto the point itself, on the heaviest thing he sees from it that is not under his feet, nor a mate's already)
        let mut pickt: Option<&crate::roomfit::Threat> = None;
        for t in threats.iter() {
            if !held_has(held, &t.key) && v3::dist_xz(at, t.at) > 1.0 && sees(g, at, t) && !bodies.iter().any(|b| off_line(*b, at, t.at) < k.lane_m) {
                pickt = Some(t);
                break;
            }
        }
        let one = vec![mk(at, v3::copy(at), pickt, front)];
        return Some(count(g, one));
    }
    // the floor they may take: the space's cells a short walk and a short way from the point, in plain view of it, off every frame and leaf's swing, clear of furniture
    let spread = k.spread0 + k.spread_per * n;
    let mut c: Vec<V3> = Vec::new();
    let ok_at = !crate::scene_collision::nav_grid_is_blocked(g, at[0], at[2]) && !crate::doorway::doorway_in_way(g, at) && clear(g, at);
    if ok_at { c.push([at[0], 0.0, at[2]]); }
    for &cell in r.cells.iter() {
        let steps = *r.steps.get(crate::Num(cell)).unwrap();
        if steps * cs > spread * 1.5 + 1.0 { break; }
        let q: V3 = [(js::rem(cell, nav_w) + 0.5) * cs, 0.0, ((js::to_i32(cell / nav_w) as f64) + 0.5) * cs];
        if v3::dist_xz(q, at) > spread || (ok_at && v3::dist_xz(q, at) < 0.3) { continue; }
        if crate::doorway::doorway_in_way(g, q) || !clear(g, q) || crate::scene_collision::solid_blocked(g, [at[0], 1.3, at[2]], [q[0], 1.3, q[2]], None) { continue; }
        c.push(q);
    }
    if (c.len() as f64) < n { return None; }
    let men = n as usize;
    // the jobs: every thing to watch once, heaviest first; then a second gun on the uncleared ways; then nothing
    let mut jobs: Vec<Option<crate::roomfit::Threat>> = threats.iter().filter(|t| !held_has(held, &t.key)).map(|t| Some(t.clone())).collect();
    for t in threats.iter() {
        if let Some(w) = t.way {
            if !w.far_known && !w.locked {
                let mut second = t.clone();
                second.w = t.w * k.second_k;
                jobs.push(Some(second));
            }
        }
    }
    for t in threats.iter() {
        if held_has(held, &t.key) && !jobs.iter().any(|j| match j { Some(j) => j.key == t.key, None => false }) { jobs.push(Some(t.clone())); }
    }
    jobs.truncate(men);
    while jobs.len() < men { jobs.push(None); }
    let mut out: Vec<Option<crate::roomfit::Fit>> = jobs.iter().map(|_| None).collect();
    for _pass in 0..3 {
        for i in 0..men {
            let mut f = pick(g, &r, at, front, bodies, &c, jobs[i].as_ref(), &out, i);
            if f.is_none() && jobs[i].is_some() && out[i].is_none() { // (nowhere to see it from: he is a spare man; only a man with no place yet gives a job up)
                jobs[i] = None;
                f = pick(g, &r, at, front, bodies, &c, None, &out, i);
            }
            if f.is_none() && out[i].is_none() { return None; } // (no floor left for him)
            if f.is_some() { out[i] = f; }
        }
    }
    let mut fits: Vec<crate::roomfit::Fit> = out.into_iter().flatten().collect();
    // a man with nothing to watch: the longest clear run from his mark that no mate's gun is on, the way Sam pointed among equals, else that way itself
    for i in 0..fits.len() {
        if matches!(fits[i].key.as_deref(), Some(key) if !key.is_empty()) { continue; }
        let mates: Vec<crate::roomfit::Fit> = fits.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, o)| o.clone()).collect();
        let pos = fits[i].pos;
        if let Some(t) = spare_run(g, pos, front, &mates) {
            let f = &mut fits[i];
            f.watch = Some(t.at);
            f.face = v3::yaw_to(f.pos, t.at);
            f.words = t.words;
            f.key = Some(t.key);
            f.w = t.w;
        }
    }
    Some(count(g, fits))
}

/// The longest clear run from `q` at eye height that is `sectorDeg` clear of every mate's gun. (roomfit.ts spareRun)
pub fn spare_run(g: &mut crate::game::Game, q: V3, front: f64, mates: &[crate::roomfit::Fit]) -> Option<crate::roomfit::Threat> {
    let k = g.k.roomfit;
    const B: usize = 16;
    let step = 2.0 * m::PI / (B as f64);
    let eye: V3 = [q[0], EYE, q[2]];
    let mut best: Option<crate::roomfit::Threat> = None;
    for b in 0..B {
        let yaw = v3::wrap_angle(front + (b as f64) * step);
        let hit = crate::scene_collision::raycast(g, eye, [m::sin(yaw), 0.0, m::cos(yaw)], 12.0);
        let t = match hit { Some(h) => h.t, None => 12.0 };
        if t < 2.2 { continue; }
        let mut gaps: Vec<f64> = Vec::with_capacity(mates.len() + 1);
        gaps.push(m::PI);
        for o in mates.iter() {
            let on = match o.watch { Some(watch) => v3::yaw_to(o.pos, watch), None => o.face };
            gaps.push(m::abs(v3::wrap_angle(on - yaw)));
        }
        let gap = m::min_n(&gaps);
        let look = m::min(t - 0.6, k.sector_look);
        let p: V3 = [q[0] + m::sin(yaw) * look, CHEST, q[2] + m::cos(yaw) * look];
        if mates.iter().any(|o| off_line(o.pos, q, p) < k.lane_m) { continue; }
        let w = 0.1 * m::min(1.0, t / 10.0)
            * (if gap < k.sector_deg * DEG { 0.2 } else if gap < 2.0 * k.sector_deg * DEG { 0.7 } else { 1.0 })
            * (1.0 + k.front_k * m::cos((b as f64) * step));
        let better = match &best { None => true, Some(best) => w > best.w + 1e-9 };
        if better {
            let key = alloc::format!("run:{}@{},{}", b, to_fixed_1(q[0]), to_fixed_1(q[2]));
            let words = run_words(g, q, [q[0] + m::sin(yaw) * (t - 0.8), 0.0, q[2] + m::cos(yaw) * (t - 0.8)], yaw);
            best = Some(crate::roomfit::Threat { at: p, w, way: None, key, words });
        }
    }
    best
}

/// Which spots are one pair's: the two nearest one another, then the rest. (roomfit.ts fitGroups)
pub fn fit_groups(fits: &[crate::roomfit::Fit]) -> Vec<f64> {
    let n = fits.len();
    if n < 3 { return vec![0.0; n]; }
    if n == 4 {
        let d = |a: usize, b: usize| v3::dist_xz(fits[a].pos, fits[b].pos);
        let mut ways: Vec<([usize; 4], f64)> = [[0, 1, 2, 3], [0, 2, 1, 3], [0, 3, 1, 2]].iter().map(|l: &[usize; 4]| (*l, d(l[0], l[1]) + d(l[2], l[3]))).collect();
        js::sort_by(&mut ways, |x, y| x.1 - y.1);
        let l = ways[0].0;
        return (0..n).map(|i| if i == l[0] || i == l[1] { 0.0 } else { 1.0 }).collect();
    }
    let (mut a, mut b, mut bd) = (0, 1, m::INF);
    for i in 0..n {
        for j in (i + 1)..n {
            let d = v3::dist_xz(fits[i].pos, fits[j].pos);
            if d < bd { bd = d; a = i; b = j; }
        }
    }
    (0..n).map(|i| if i == a || i == b { 0.0 } else { 1.0 }).collect()
}

/// He takes the deal's watch into his order. (roomfit.ts take)
pub fn take(g: &mut crate::game::Game, a: OrdersAssignmentId, f: &crate::roomfit::Fit) {
    let row = &mut g.tab.orders_assignment[a];
    row.watch = f.watch.map(v3::copy);
    row.face = Some(match (f.watch, row.post) { (Some(watch), Some(post)) => v3::yaw_to(post, watch), _ => f.face });
    row.there = if f.words.is_empty() { String::from("in position") } else { alloc::format!("in position · {}", f.words) };
}

/// Once a frame for each man under 'move': he takes his watch up over the last steps onto his mark; standing, he looks again once a second. (roomfit.ts fitTick)
pub fn fit_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let k = g.k.roomfit;
    let (post, f_on, f_next_t, f_at, f_front) = {
        let row = &g.tab.orders_assignment[a];
        let (Some(f), Some(post)) = (row.fit.as_ref(), row.post) else { return; };
        (post, f.on, f.next_t, f.at, f.front)
    };
    if g.tab.orders_assignment[a].there == "stopped short" { // (he could not get there, and says so: the deal was for a mark he never reached)
        g.tab.orders_assignment[a].fit = None;
        return;
    }
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    if !f_on {
        if v3::dist_xz(p, post) > k.watch_in_m || crate::scene_collision::solid_blocked(g, [p[0], 1.3, p[2]], [post[0], 1.3, post[2]], None) { return; }
        let next_t = g.time + k.again_secs * (1.0 + js::rem(g.ch[c].id, 4.0) * 0.25);
        let f = g.tab.orders_assignment[a].fit.as_mut().unwrap();
        f.on = true;
        f.next_t = next_t;
        let f = f.clone();
        take(g, a, &f);
        return;
    }
    if g.time < f_next_t || !g.tab.orders_assignment[a].arrived { return; }
    let next_t = g.time + k.again_secs;
    g.tab.orders_assignment[a].fit.as_mut().unwrap().next_t = next_t;
    // what the weights hang on: the rooms known, the leaves, who else holds a deal: nothing of it changed, nothing to weigh
    let mut mates: Vec<(V3, Option<String>)> = Vec::new();
    for man in crate::orders::squad_orders_men(g, None, None) {
        if man == gd { continue; }
        let Some(b) = crate::orders::squad_orders_of(g, man) else { continue; };
        let row = &g.tab.orders_assignment[b];
        if let (Some(fit), Some(pos)) = (row.fit.as_ref(), row.post) { mates.push((pos, fit.key.clone())); }
    }
    let keys: Vec<&str> = mates.iter().map(|o| o.1.as_deref().unwrap_or("-")).collect();
    let mut sig = alloc::format!("{}|{}|", g.cleared.known, keys.join(","));
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if crate::doors::door_off(g, d) {
            sig.push('x');
        } else if crate::doors::door_latched(g, d) {
            sig.push(if crate::doors::door_locked(g, d) { 'l' } else { 's' });
        } else {
            // (`+ 0.0`: String(-0) is "0")
            sig.push_str(&alloc::format!("{}", m::round(crate::doors::door_angle(g, d) * 4.0) + 0.0));
        }
    }
    {
        let f = g.tab.orders_assignment[a].fit.as_mut().unwrap();
        if sig == f.sig { return; }
        f.sig = sig;
    }
    // from the point the order named: the same things, by the same names, as the deal weighed
    let r = crate::roomread::room_read(g, f_at, Some(k.reach), Some(post), None);
    let threats = threats_of(g, &r, f_at, f_front, None);
    let (f_key, f_w) = { let f = g.tab.orders_assignment[a].fit.as_ref().unwrap(); (f.key.clone(), f.w) };
    let mine = threats.iter().find(|t| f_key.as_deref() == Some(t.key.as_str()));
    // (a stretch of floor whose bearing a swinging leaf has shifted by one is still his, at what it weighed)
    let cur = match mine {
        Some(t) => t.w,
        None => match f_key.as_deref() { Some(key) if key.starts_with("floor:") || key.starts_with("run:") => f_w, _ => 0.0 },
    };
    if let Some(t) = mine { g.tab.orders_assignment[a].fit.as_mut().unwrap().w = t.w; }
    for t in threats.iter() { // (heaviest first)
        if t.w <= cur * k.sticky + 0.02 { break; }
        if f_key.as_deref() == Some(t.key.as_str()) || mates.iter().any(|o| o.1.as_deref() == Some(t.key.as_str())) || !sees(g, post, t) { continue; }
        if mates.iter().any(|o| off_line(o.0, post, t.at) < k.lane_m) { continue; }
        let f = g.tab.orders_assignment[a].fit.as_mut().unwrap();
        f.watch = Some(v3::copy(t.at));
        f.words = t.words.clone();
        f.key = Some(t.key.clone());
        f.w = t.w;
        let f = f.clone();
        take(g, a, &f);
        g.stats.roomfit.moved += 1;
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // the bits below are the TypeScript's own: bun with EXACT.on, roomfit.ts's offLine and compass, Number.toFixed, and roomfit.ts's exported fitGroups
    #[test]
    fn off_line_has_the_typescripts_bits() {
        let cases: [(V3, V3, V3, u64); 5] = [
            ([1.3, 0.0, 2.7], [0.1, 1.5, 0.2], [5.5, 1.1, 6.25], 0x3fe89f8d72349371),
            ([3.1, 0.0, -1.9], [-2.2, 0.0, 4.4], [7.75, 0.0, -6.6], 0x3fd2ebaee522c0a9),
            ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0x7ff0000000000000),
            ([9.0, 0.0, 9.0], [0.0, 0.0, 0.0], [1.0, 0.0, 1.0], 0x7ff0000000000000),
            ([-0.01, 0.0, 0.13], [-1.1, 0.0, -1.3], [0.7, 0.0, 1.9], 0x3fcfdd63a3dd9dfb),
        ];
        for (p, a, b, bits) in cases {
            assert_eq!(off_line(p, a, b).to_bits(), bits);
        }
    }

    #[test]
    fn compass_names_as_the_typescript_does() {
        let cases: [(f64, &str); 10] = [
            (0.0, "south"), (0.4, "south-east"), (-0.4, "south-west"), (m::PI, "north"), (-m::PI, "north"),
            (2.1, "north-east"), (-2.1, "north-west"), (7.5, "east"), (-7.5, "west"), (0.3926990816987241, "south"),
        ];
        for (yaw, name) in cases {
            assert_eq!(compass(yaw), name);
        }
    }

    #[test]
    fn to_fixed_rounds_as_javascript_does() {
        let cases: [(f64, &str); 16] = [
            (0.25, "0.3"), (0.75, "0.8"), (1.25, "1.3"), (-1.25, "-1.3"), (-0.25, "-0.3"), (0.35, "0.3"), (12.05, "12.1"), (-0.04, "-0.0"),
            (-0.0, "0.0"), (0.0, "0.0"), (3.15, "3.1"), (7.45, "7.5"), (100.75, "100.8"), (0.05, "0.1"), (0.15, "0.1"), (33.333, "33.3"),
        ];
        for (x, s) in cases {
            assert_eq!(to_fixed_1(x), s, "{}", x);
        }
    }

    #[test]
    fn fit_groups_pairs_as_the_typescript_does() {
        let fits = |l: &[[f64; 2]]| -> Vec<crate::roomfit::Fit> {
            l.iter().map(|p| { let mut f = crate::roomfit::Fit::zeroed(); f.pos = [p[0], 0.0, p[1]]; f }).collect()
        };
        assert_eq!(fit_groups(&fits(&[[0.0, 0.0], [1.0, 1.0]])), [0.0, 0.0]);
        assert_eq!(fit_groups(&fits(&[[0.0, 0.0], [5.0, 0.0], [1.0, 0.0]])), [0.0, 1.0, 0.0]);
        assert_eq!(fit_groups(&fits(&[[0.0, 0.0], [5.0, 0.0], [1.0, 0.0], [6.0, 0.0]])), [0.0, 1.0, 0.0, 1.0]);
        assert_eq!(fit_groups(&fits(&[[0.0, 0.0], [1.0, 0.0], [5.0, 0.0], [6.0, 0.0]])), [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(fit_groups(&fits(&[[0.0, 0.0], [5.0, 0.0], [6.0, 0.0], [1.0, 0.0], [9.0, 9.0]])), [0.0, 1.0, 1.0, 0.0, 1.0]);
        assert_eq!(fit_groups(&fits(&[[0.0, 0.0], [3.0, 0.0], [0.0, 4.0], [3.0, 4.0]])), [0.0, 0.0, 1.0, 1.0]);
    }
}
