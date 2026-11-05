//! src/game/doorswing.ts lines 1 to 185
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A drill plan as the deal reads it: door, fling, room, bounds (Level.breach and Level.clearPlans are one list in the TypeScript, two Rust structs)
type Plan = (String, f64, Option<String>, Option<crate::doorswing::Rect>);

/// What the deal reads of the level, copied out once so a rule can call what takes `g` (nothing writes the level during the deal).
struct Given {
    rooms: Vec<crate::scene_scenarios::StructureRoom>,
    plans: Vec<Plan>,
    /// the rescue's room door, side door, room name and bounds (its own, else the first plan's for that door)
    rescue: Option<(String, Option<String>, String, Option<crate::doorswing::Rect>)>,
    entry_door: String,
    server_room: crate::doorswing::Rect,
}

/// The deal as the doors stand now.   (doorswing.ts swingsNow)
pub fn swings_now(g: &mut crate::game::Game, deals: &[crate::doorswing::SwingDeal]) -> Vec<crate::doorswing::SwingDeal> {
    deals
        .iter()
        .map(|s| match &s.clear {
            Some(c) => (**c).clone(),
            None => s.clone(),
        })
        .collect()
}

/// The side (Doors.side's sign) of the closed leaf a floor point is on, 0 on the leaf's own line.   (doorswing.ts sideOf)
pub fn side_of(g: &mut crate::game::Game, d: DoorId, x: f64, z: f64) -> f64 {
    let c = g.dr[d].def.closed_dir;
    let ux = m::cos(c);
    let uz = m::sin(c);
    let w = g.dr[d].def.width;
    let hinge = g.dr[d].def.hinge;
    let mx = hinge[0] + ux * w / 2.0;
    let mz = hinge[1] + uz * w / 2.0;
    let s = (x - mx) * -uz + (z - mz) * ux;
    if m::abs(s) < 0.05 {
        0.0
    } else if s > 0.0 {
        1.0
    } else {
        -1.0
    }
}

/// A rectangle's middle.   (doorswing.ts mid)
pub fn mid(r: crate::doorswing::Rect) -> [f64; 2] {
    [(r.x0 + r.x1) / 2.0, (r.z0 + r.z1) / 2.0]
}

/// A rectangle's area.   (doorswing.ts area)
pub fn area(r: crate::doorswing::Rect) -> f64 {
    m::abs((r.x1 - r.x0) * (r.z1 - r.z0))
}

/// How many of ROOF_PTS points just off face `s` of a door have a roof over them.   (doorswing.ts roofOver)
pub fn roof_over(g: &mut crate::game::Game, d: DoorId, s: f64) -> f64 {
    let c = g.dr[d].def.closed_dir;
    let ux = m::cos(c);
    let uz = m::sin(c);
    let hinge = g.dr[d].def.hinge;
    let width = g.dr[d].def.width;
    let mx = hinge[0] + ux * width / 2.0;
    let mz = hinge[1] + uz * width / 2.0;
    let no_shadow = js::to_i32(crate::scene_boxes::box_flag().no_shadow);
    let mut n = 0.0;
    for off in [0.8, 1.5] {
        for al in [-0.4, 0.0, 0.4] {
            let x = mx - uz * off * s + ux * al;
            let z = mz + ux * off * s + uz * al;
            let some = g.level.boxes.iter().any(|b| {
                if js::to_i32(b.flags) & no_shadow != 0 || b.c[1] - b.h[1] < ROOF_UNDER {
                    return false;
                }
                let dx = x - b.c[0];
                let dz = z - b.c[2];
                let cy = m::cos(b.yaw);
                let sy = m::sin(b.yaw);
                m::abs(dx * cy - dz * sy) <= b.h[0] && m::abs(dx * sy + dz * cy) <= b.h[2]
            });
            if some {
                n += 1.0;
            }
        }
    }
    n
}

/// The open-air side of an outside door, 0 for one not marked `exterior`, or roofed on both faces or neither.   (doorswing.ts outsideOf)
pub fn outside_of(g: &mut crate::game::Game, d: DoorId) -> f64 {
    if !g.dr[d].def.exterior.unwrap_or(false) {
        return 0.0;
    }
    let p = roof_over(g, d, 1.0);
    let mi = roof_over(g, d, -1.0);
    let inn = ROOF_PTS - 2.0;
    let out = 2.0;
    if p >= inn && mi <= out {
        -1.0
    } else if mi >= inn && p <= out {
        1.0
    } else {
        0.0
    }
}

/// floors' `flood`: the component of `start`, flooded and its size kept on the first asking.   (doorswing.ts floors)
fn floors_flood(comp: &mut [i32], sizes: &mut Vec<f64>, wall: &[u8], w: usize, h: usize, start: usize) -> i32 {
    if comp[start] >= 0 {
        return comp[start];
    }
    let id = sizes.len() as i32;
    let mut n = 0.0;
    let mut q = vec![start];
    comp[start] = id;
    while let Some(i) = q.pop() {
        n += 1.0;
        let x = i % w;
        let z = i / w;
        if x > 0 && wall[i - 1] == 0 && comp[i - 1] < 0 {
            comp[i - 1] = id;
            q.push(i - 1);
        }
        if x + 1 < w && wall[i + 1] == 0 && comp[i + 1] < 0 {
            comp[i + 1] = id;
            q.push(i + 1);
        }
        if z > 0 && wall[i - w] == 0 && comp[i - w] < 0 {
            comp[i - w] = id;
            q.push(i - w);
        }
        if z + 1 < h && wall[i + w] == 0 && comp[i + w] < 0 {
            comp[i + w] = id;
            q.push(i + w);
        }
    }
    sizes.push(n);
    id
}

/// The nav cells either face of every door opens on with every doorway shut: per door, the cells reachable off each face.   (doorswing.ts floors)
pub fn floors(g: &mut crate::game::Game) -> JsMap<DoorId, crate::doorswing::FloorsValue> {
    let (wf, hf, cs) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let (w, h) = (wf as usize, hf as usize);
    let mut wall = g.col.nav.blocked.clone();
    let list = g.doors.list.clone();
    for &d in list.iter() {
        let c = g.dr[d].def.closed_dir;
        let ux = m::cos(c);
        let uz = m::sin(c);
        let hinge = g.dr[d].def.hinge;
        let width = g.dr[d].def.width;
        let mut t = -0.3;
        while t <= width + 0.3 {
            for o in [-cs * 0.3, 0.0, cs * 0.3] {
                let x = m::floor((hinge[0] + ux * t - uz * o) / cs);
                let z = m::floor((hinge[1] + uz * t + ux * o) / cs);
                if x >= 0.0 && z >= 0.0 && x < wf && z < hf {
                    wall[(z * wf + x) as usize] = 1;
                }
            }
            t += cs / 4.0;
        }
    }
    let mut comp = vec![-1i32; w * h];
    let mut sizes: Vec<f64> = Vec::new();
    let mut out = JsMap::new();
    for &d in list.iter() {
        let c = g.dr[d].def.closed_dir;
        let ux = m::cos(c);
        let uz = m::sin(c);
        let hinge = g.dr[d].def.hinge;
        let width = g.dr[d].def.width;
        let mx = hinge[0] + ux * width / 2.0;
        let mz = hinge[1] + uz * width / 2.0;
        let mut from = |s: f64| -> i32 {
            for off in [0.75, 1.0, 1.25, 0.5, 1.5] {
                for al in [0.0, 0.3, -0.3] {
                    let x = m::floor((mx - uz * off * s + ux * al) / cs);
                    let z = m::floor((mz + ux * off * s + uz * al) / cs);
                    if x >= 0.0 && z >= 0.0 && x < wf && z < hf && wall[(z * wf + x) as usize] == 0 {
                        return floors_flood(&mut comp, &mut sizes, &wall, w, h, (z * wf + x) as usize);
                    }
                }
            }
            -1
        };
        let a = from(1.0);
        let b = from(-1.0);
        let row = if a >= 0 && b >= 0 && a == b {
            crate::doorswing::FloorsValue { plus: 0.0, minus: 0.0 }
        } else {
            crate::doorswing::FloorsValue {
                plus: if a < 0 { 0.0 } else { sizes[a as usize] },
                minus: if b < 0 { 0.0 } else { sizes[b as usize] },
            }
        };
        out.set(d, row);
    }
    out
}

/// leafCut's alOf: along the leaf's line.
fn al_of(k: usize, na: usize) -> f64 {
    -LEAF_WIN + (k % na) as f64 * LEAF_STEP
}
/// leafCut's peOf: square off the wall.
fn pe_of(k: usize, na: usize) -> f64 {
    ((k / na) as f64 + 0.5) * LEAF_STEP
}
/// leafCut's `free`: a man can stand at grid spot k (asked once). `at` is [cx, ux, nx, cz, uz, nz].
fn leaf_free(g: &mut Game, known: &mut [i8], k: usize, na: usize, at: [f64; 6]) -> bool {
    if known[k] < 0 {
        let al = al_of(k, na);
        let pe = pe_of(k, na);
        let x = at[0] + at[1] * al + at[2] * pe;
        let z = at[3] + at[4] * al + at[5] * pe;
        let mut q: V3 = [x, 0.0, z];
        known[k] = if crate::scene_collision::collide_circle(g, &mut q, LEAF_MAN, 0.25, 1.5, Some(1.0)) && m::hyp2(q[0] - x, q[2] - z) > 0.005 { 0 } else { 1 };
    }
    known[k] == 1
}
/// leafCut's `flood`: what a man reaches from the doorway, past the leaf if one is given; None when the leaf stands on the doorway itself.
fn leaf_flood(g: &mut Game, known: &mut [i8], n_all: usize, na: usize, at: [f64; 6], start: usize, r: f64, leaf: Option<&[f32]>) -> Option<Vec<u8>> {
    if let Some(l) = leaf {
        if (l[start] as f64) < r {
            return None;
        }
    }
    let mut seen = vec![0u8; n_all];
    let mut st = vec![start];
    seen[start] = 1;
    let (n_i, na_i) = (n_all as i64, na as i64);
    while let Some(k) = st.pop() {
        let i = k % na;
        let ki = k as i64;
        for n in [if i > 0 { ki - 1 } else { -1 }, if i < na - 1 { ki + 1 } else { -1 }, ki - na_i, ki + na_i] {
            if n >= 0 && n < n_i {
                let nu = n as usize;
                if seen[nu] == 0 && !leaf.map_or(false, |l| (l[nu] as f64) < r) && leaf_free(g, known, nu, na, at) {
                    seen[nu] = 1;
                    st.push(nu);
                }
            }
        }
    }
    Some(seen)
}

/// How much floor on face `s` of a door (m²) a man reaches the doorway from with no leaf there and not with the leaf open on that face; −1 = nowhere to stand at the doorway.   (doorswing.ts leafCut)
pub fn leaf_cut(g: &mut crate::game::Game, d: DoorId, s: f64) -> f64 {
    let stop = if s > 0.0 { g.dr[d].max_auth } else { -g.dr[d].min_auth };
    if stop < 0.9 {
        return 0.0;
    }
    let c = g.dr[d].def.closed_dir;
    let w = g.dr[d].def.width;
    let ux = m::cos(c);
    let uz = m::sin(c);
    let nx = -uz * s;
    let nz = ux * s;
    let [hx, hz] = g.dr[d].def.hinge;
    let cx = hx + ux * w / 2.0;
    let cz = hz + uz * w / 2.0;
    let thick = g.dr[d].thick;
    let na = m::round(2.0 * LEAF_WIN / LEAF_STEP) as usize + 1;
    let np = m::round(LEAF_WIN / LEAF_STEP) as usize;
    let n_all = na * np;
    let mut known = vec![-1i8; n_all];
    let at = [cx, ux, nx, cz, uz, nz];
    let mut start: Option<usize> = None;
    let mut near = 1e9;
    for k in 0..na * core::cmp::min(np, 9) {
        let al = m::abs(al_of(k, na));
        let pe = pe_of(k, na);
        if al < w / 2.0 - LEAF_MAN + 0.06 && al + pe < near && leaf_free(g, &mut known, k, na, at) {
            near = al + pe;
            start = Some(k);
        }
    }
    let start = match start {
        Some(k) => k,
        None => return -1.0,
    };
    let r = LEAF_MAN + thick / 2.0;
    let mut open: Option<Vec<u8>> = None;
    let mut best = f64::INFINITY;
    let right = m::PI / 2.0;
    let angles: Vec<f64> = if stop < right { vec![stop] } else { vec![right, m::min(stop, LEAF_REST)] };
    for &th in angles.iter() {
        let lx = m::cos(th) * ux + m::sin(th) * nx;
        let lz = m::cos(th) * uz + m::sin(th) * nz;
        let mut leaf = vec![0f32; n_all];
        let mut round = true;
        for k in 0..n_all {
            let al = al_of(k, na);
            let pe = pe_of(k, na);
            let px = cx + ux * al + nx * pe - hx;
            let pz = cz + uz * al + nz * pe - hz;
            let t = m::max(0.0, m::min(w, px * lx + pz * lz));
            leaf[k] = m::hyp2(px - lx * t, pz - lz * t) as f32;
            let lk = leaf[k] as f64;
            if round && lk >= r && lk < r + LEAF_RING && pe >= LEAF_WALL && !leaf_free(g, &mut known, k, na, at) {
                round = false;
            }
        }
        if round {
            return 0.0;
        }
        if open.is_none() {
            open = leaf_flood(g, &mut known, n_all, na, at, start, r, None);
        }
        let shut = leaf_flood(g, &mut known, n_all, na, at, start, r, Some(&leaf));
        let opened = open.as_ref().unwrap();
        let mut n = 0.0;
        for k in 0..n_all {
            if opened[k] != 0 && (leaf[k] as f64) >= r && !shut.as_ref().map_or(false, |sh| sh[k] != 0) {
                n += 1.0;
            }
        }
        best = m::min(best, n * LEAF_STEP * LEAF_STEP);
        if best == 0.0 {
            return 0.0;
        }
    }
    best
}

/// Whether a leaf swung open on face `s`, as far as it rests, meets nothing of the level on its way.   (doorswing.ts leafSwings)
pub fn leaf_swings(g: &mut crate::game::Game, d: DoorId, s: f64) -> bool {
    let stop = m::min(if s > 0.0 { g.dr[d].max_auth } else { -g.dr[d].min_auth }, LEAF_REST);
    let c = g.dr[d].def.closed_dir;
    let w = g.dr[d].def.width;
    let [hx, hz] = g.dr[d].def.hinge;
    let mut a = 0.5;
    while a <= stop + 1e-9 {
        for t in [0.35, 0.7, 1.0] {
            let x = hx + m::cos(c + s * a) * w * t;
            let z = hz + m::sin(c + s * a) * w * t;
            let mut q: V3 = [x, 0.0, z];
            if crate::scene_collision::collide_circle(g, &mut q, 0.03, 0.25, 1.5, Some(1.0)) && m::hyp2(q[0] - x, q[2] - z) > 0.005 {
                return false;
            }
        }
        a += 0.1;
    }
    true
}

/// dealSwings' `opens`: its authored stop lets it open at least MIN_OPEN on that side.
fn opens(g: &Game, d: DoorId, s: f64) -> bool {
    (if s > 0.0 { g.dr[d].max_auth } else { -g.dr[d].min_auth }) >= MIN_OPEN
}
/// dealSwings' `deal`: never a side its stop will not let it open to while the other will. An empty `into` is none.
fn deal(g: &Game, d: DoorId, swing: f64, why: crate::doorswing::SwingWhy, into: Option<&str>) -> Option<crate::doorswing::SwingDeal> {
    if !js::truthy(swing) || !(why == crate::doorswing::SwingWhy::Authored || opens(g, d, swing) || !opens(g, d, -swing)) {
        return None;
    }
    Some(crate::doorswing::SwingDeal {
        door: g.dr[d].def.name.clone(),
        swing,
        why,
        into: into.filter(|i| !i.is_empty()).map(String::from),
        out: None,
        clear: None,
    })
}
/// dealSwings' `toward`: the side of the leaf a rectangle's middle is on.
fn toward(g: &mut Game, d: DoorId, r: Option<crate::doorswing::Rect>) -> f64 {
    match r {
        Some(r) => {
            let c = mid(r);
            side_of(g, d, c[0], c[1])
        }
        None => 0.0,
    }
}
fn rect_of(r: &crate::scene_scenarios::Rect) -> crate::doorswing::Rect {
    crate::doorswing::Rect { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }
}
fn plan_rect(r: &crate::scene_level::BreachPlanBounds) -> crate::doorswing::Rect {
    crate::doorswing::Rect { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }
}
/// `n.toFixed(1)` of a number >= 0. Only a multiple of a quarter that is odd is an exact tie; JavaScript takes the upper of the two, Rust the even.
fn to_fixed_1(x: f64) -> String {
    let t = x * 4.0;
    let tie = m::floor(t) == t && js::rem(t, 2.0) != 0.0;
    alloc::format!("{:.1}", if tie { x + 0.01 } else { x })
}

/// dealSwings' `tries`, in order: the first rule that speaks for this door.
fn first_rule(g: &mut Game, given: &Given, fl: &mut Option<JsMap<DoorId, crate::doorswing::FloorsValue>>, d: DoorId, name: &str) -> Option<crate::doorswing::SwingDeal> {
    use crate::doorswing::SwingWhy;
    // authored
    if let Some(s) = deal(g, d, g.dr[d].def.swing.unwrap_or(0.0), SwingWhy::Authored, None) {
        return Some(s);
    }
    // folds
    let (max_auth, min_auth) = (g.dr[d].max_auth, g.dr[d].min_auth);
    let folds = if max_auth > 2.2 && -min_auth <= 2.2 {
        1.0
    } else if -min_auth > 2.2 && max_auth <= 2.2 {
        -1.0
    } else {
        0.0
    };
    if let Some(s) = deal(g, d, folds, SwingWhy::Folds, None) {
        return Some(s);
    }
    // plan: plans from both faces that throw it opposite ways say nothing
    let mine: Vec<&Plan> = given.plans.iter().filter(|p| p.0 == name && js::truthy(p.1)).collect();
    if !mine.is_empty() && mine.iter().all(|p| m::sign(p.1) == m::sign(mine[0].1)) {
        if let Some(s) = deal(g, d, m::sign(mine[0].1), SwingWhy::Plan, mine[0].2.as_deref()) {
            return Some(s);
        }
    }
    // room
    let mut mine: Vec<&crate::scene_scenarios::StructureRoom> = given.rooms.iter().filter(|r| r.doors.iter().any(|x| x.id == name)).collect();
    js::sort_by(&mut mine, |a, b| js::or(area(rect_of(&a.bounds)) - area(rect_of(&b.bounds)), if a.id < b.id { -1.0 } else { 1.0 }));
    for r in mine {
        let side = toward(g, d, Some(rect_of(&r.bounds)));
        if let Some(s) = deal(g, d, side, SwingWhy::Room, Some(&r.name)) {
            return Some(s);
        }
    }
    // hostages
    if let Some((room_door, side_door, room_name, bounds)) = &given.rescue {
        if room_door == name || side_door.as_deref() == Some(name) {
            let side = toward(g, d, *bounds);
            if let Some(s) = deal(g, d, side, SwingWhy::Hostages, Some(room_name)) {
                return Some(s);
            }
        }
    }
    // mission
    if given.entry_door == name {
        let side = toward(g, d, Some(given.server_room));
        if let Some(s) = deal(g, d, side, SwingWhy::Mission, Some("server room")) {
            return Some(s);
        }
    }
    // lock
    if g.dr[d].def.locked.unwrap_or(false) {
        if let Some(s) = deal(g, d, -g.dr[d].key_side, SwingWhy::Lock, None) {
            return Some(s);
        }
    }
    // stands open
    if let Some(s) = deal(g, d, m::sign(g.dr[d].def.angle.unwrap_or(0.0)), SwingWhy::StandsOpen, None) {
        return Some(s);
    }
    // floor: flooded on the first door that gets as far as asking
    if fl.is_none() {
        *fl = Some(floors(g));
    }
    let f = fl.as_ref().and_then(|t| t.get(d)).copied();
    if let Some(f) = f {
        if f.plus > 0.0 && f.minus > 0.0 && f.plus != f.minus {
            let into = alloc::format!("{} cells against {}", m::min(f.plus, f.minus), m::max(f.plus, f.minus));
            return deal(g, d, if f.plus < f.minus { 1.0 } else { -1.0 }, SwingWhy::Floor, Some(&into));
        }
    }
    None
}

/// What the deal reads of the level: `S?.roomClear?.rooms ?? []`, breach then clearPlans, the rescue's room, the mission's door and room.
fn given_of(g: &Game) -> Given {
    let level = &g.level;
    let mut plans: Vec<Plan> = Vec::new();
    if let Some(p) = &level.breach {
        plans.push((p.door.clone(), p.fling, p.room.clone(), p.bounds.as_ref().map(plan_rect)));
    }
    if let Some(ps) = &level.clear_plans {
        for p in ps.iter() {
            plans.push((p.door.clone(), p.fling, p.room.clone(), p.bounds.as_ref().map(plan_rect)));
        }
    }
    let scenarios = level.scenarios.as_ref();
    let rooms = scenarios.and_then(|s| s.room_clear.as_ref()).map_or_else(Vec::new, |r| r.rooms.clone());
    let rescue = scenarios.and_then(|s| s.rescue.as_ref()).map(|rs| {
        let bounds = match &rs.room.bounds {
            Some(b) => Some(rect_of(b)),
            None => plans.iter().find(|p| p.0 == rs.room.door).and_then(|p| p.3),
        };
        (rs.room.door.clone(), rs.side.as_ref().map(|x| x.door.clone()), rs.room.name.clone(), bounds)
    });
    let sr = &level.mission.server_room;
    let server_room = crate::doorswing::Rect { x0: sr.x0, x1: sr.x1, z0: sr.z0, z1: sr.z1 };
    Given { rooms, plans, rescue, entry_door: level.mission.entry_door.clone(), server_room }
}

/// dealSwings' `all`: every door dealt in the list's order, each set on the door before the next is asked.
fn deal_all(g: &mut Game) -> Vec<crate::doorswing::SwingDeal> {
    use crate::doorswing::SwingWhy;
    let given = given_of(g);
    let mut fl: Option<JsMap<DoorId, crate::doorswing::FloorsValue>> = None;
    let list = g.doors.list.clone();
    let mut dealt = Vec::with_capacity(list.len());
    for &d in list.iter() {
        let name = g.dr[d].def.name.clone();
        let mut got = match first_rule(g, &given, &mut fl, d, &name) {
            Some(s) => s,
            None => crate::doorswing::SwingDeal {
                door: name.clone(),
                swing: if g.dr[d].max_auth >= -g.dr[d].min_auth { 1.0 } else { -1.0 },
                why: SwingWhy::Wider,
                into: None,
                out: None,
                clear: None,
            },
        };
        let out = if got.why != SwingWhy::Authored && got.why != SwingWhy::Folds {
            let side = outside_of(g, d);
            deal(g, d, side, SwingWhy::Outside, Some("the open air"))
        } else {
            None
        };
        let out_swing = out.as_ref().map_or(0.0, |o| o.swing);
        if let Some(o) = out {
            got.out = Some(Box::new(o));
        }
        let mut clear: Option<crate::doorswing::SwingDeal> = None;
        if got.why != SwingWhy::Authored && got.why != SwingWhy::Folds && got.why != SwingWhy::Plan && got.why != SwingWhy::StandsOpen
            && !js::truthy(g.dr[d].def.angle.unwrap_or(0.0)) && opens(g, d, -got.swing)
        {
            let cut = leaf_cut(g, d, got.swing);
            let other = if cut >= LEAF_CUT { leaf_cut(g, d, -got.swing) } else { -1.0 };
            if cut >= LEAF_CUT && other >= 0.0 && other < LEAF_CUT && other * 2.0 <= cut && leaf_swings(g, d, -got.swing) {
                clear = Some(crate::doorswing::SwingDeal {
                    door: name.clone(),
                    swing: -got.swing,
                    why: SwingWhy::LeafRoom,
                    into: Some(alloc::format!("{} m² of floor shut off by its leaf the other way", to_fixed_1(cut))),
                    out: None,
                    clear: None,
                });
            }
        }
        let clear_swing = clear.as_ref().map_or(0.0, |c| c.swing);
        if let Some(c) = clear {
            got.clear = Some(Box::new(c));
        }
        crate::doors::door_set_swing(g, d, got.swing, Some(out_swing), Some(clear_swing));
        dealt.push(got);
    }
    dealt
}

/// Deal every door of the level its side (Door.setSwing) and say why.   (doorswing.ts dealSwings)
pub fn deal_swings(g: &mut crate::game::Game) -> Vec<crate::doorswing::SwingDeal> {
    if !g.doors.list.is_empty() {
        crate::scene_collision::window(g, &mut |g| deal_all(g))
    } else {
        deal_all(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two floors of a 5 by 2 grid split by a wall column: each cell's component kept, a second asking floods nothing.
    #[test]
    fn floors_flood_keeps_each_floor_once() {
        let wall = [0, 0, 1, 0, 0, 0, 0, 1, 0, 0];
        let mut comp = [-1i32; 10];
        let mut sizes: Vec<f64> = Vec::new();
        assert_eq!(floors_flood(&mut comp, &mut sizes, &wall, 5, 2, 0), 0);
        assert_eq!(floors_flood(&mut comp, &mut sizes, &wall, 5, 2, 9), 1);
        assert_eq!(floors_flood(&mut comp, &mut sizes, &wall, 5, 2, 6), 0);
        assert_eq!(sizes, [4.0, 4.0]);
        assert_eq!(comp, [0, 0, -1, 1, 1, 0, 0, -1, 1, 1]);
    }

    /// The same strings as bun's `toFixed(1)`, its ties among them.
    #[test]
    fn to_fixed_1_matches_javascript() {
        for (x, s) in [(0.25, "0.3"), (0.75, "0.8"), (1.25, "1.3"), (1.75, "1.8"), (2.25, "2.3"), (1.0, "1.0"), (1.2000000000000002, "1.2"), (3.1400000000000006, "3.1"), (1.35, "1.4"), (12.5, "12.5")] {
            assert_eq!(to_fixed_1(x), s, "{}", x);
        }
    }

    /// Spots of leafCut's grid, as the TypeScript's alOf and peOf.
    #[test]
    fn leaf_grid_spots() {
        assert_eq!((-LEAF_WIN + 3.0 * LEAF_STEP).to_bits(), al_of(81 * 2 + 3, 81).to_bits());
        assert_eq!(((2.0 + 0.5) * LEAF_STEP).to_bits(), pe_of(81 * 2 + 3, 81).to_bits());
    }

    /// One door of the oracle's scene.
    fn door(g: &mut Game, name: &str, hinge: [f64; 2], max_auth: f64, min_auth: f64, set: fn(&mut crate::doors::DoorDef)) {
        let mut row = crate::doors::Door::blank();
        row.def = crate::doors::DoorDef::zeroed();
        row.def.name = String::from(name);
        row.def.hinge = hinge;
        row.def.closed_dir = m::PI / 2.0;
        row.def.width = 1.0;
        set(&mut row.def);
        row.max_auth = max_auth;
        row.min_auth = min_auth;
        row.key_side = 1.0;
        let id = g.dr.push(row);
        g.doors.list.push(id);
    }
    fn roof(c: V3, h: V3, yaw: f64, flags: f64) -> crate::scene_boxes::Box {
        let mut b = crate::scene_boxes::Box::zeroed();
        (b.c, b.h, b.yaw, b.flags) = (c, h, yaw, flags);
        b
    }
    fn room(id: &str, name: &str, x0: f64, x1: f64, z0: f64, z1: f64, door: &str) -> crate::scene_scenarios::StructureRoom {
        let mut r = crate::scene_scenarios::StructureRoom::zeroed();
        r.id = String::from(id);
        r.name = String::from(name);
        r.bounds = crate::scene_scenarios::Rect { x0, x1, z0, z1 };
        let mut x = crate::scene_scenarios::StructureDoor::zeroed();
        x.id = String::from(door);
        r.doors.push(x);
        r
    }
    fn clear_plan(door: &str, fling: f64, room: Option<&str>, bounds: Option<crate::scene_level::BreachPlanBounds>) -> crate::scene_level::RoomClearPlan {
        let mut p = crate::scene_level::RoomClearPlan::zeroed();
        p.door = String::from(door);
        p.fling = fling;
        p.room = room.map(String::from);
        p.bounds = bounds;
        p
    }

    /// Every rule of `tries` and the outside rule against the TypeScript's own dealSwings on one made-up level (scene and answers from bun, doorswing.ts with `col` null; the box world and setSwing are stubs here, so the leaf room rule is not reached).
    #[test]
    fn rules_answer_as_the_typescript() {
        let mut g = Game::blank();
        door(&mut g, "floor", [10.0, 4.0], 1.6, -1.6, |d| d.exterior = Some(true));
        door(&mut g, "authored", [10.0, 6.0], 1.6, -1.6, |d| d.swing = Some(-1.0));
        door(&mut g, "folds", [10.0, 8.0], 2.5, -1.0, |_| {});
        door(&mut g, "plan", [10.0, 10.0], 1.6, -1.6, |_| {});
        door(&mut g, "room", [10.0, 12.0], 1.6, -1.6, |_| {});
        door(&mut g, "hostages", [10.0, 14.0], 1.6, -1.6, |_| {});
        door(&mut g, "entry", [10.0, 16.0], 1.6, -1.6, |_| {});
        door(&mut g, "locked", [10.0, 18.0], 1.6, -1.6, |d| d.locked = Some(true));
        door(&mut g, "ajar", [5.0, 2.0], 1.6, -1.6, |d| d.angle = Some(0.4));
        door(&mut g, "wider", [5.0, 6.0], 1.0, -1.5, |_| {});
        door(&mut g, "stopped", [10.0, 0.5], 0.5, -1.6, |_| {});
        door(&mut g, "split", [5.0, 10.0], 1.6, -1.6, |_| {});
        let (w, h) = (40usize, 40usize);
        let mut blocked = vec![0u8; w * h];
        for z in 0..h {
            blocked[z * w + 20] = 1;
            for x in 30..w {
                blocked[z * w + x] = 1;
            }
        }
        (g.col.nav.w, g.col.nav.h, g.col.nav.cell, g.col.nav.blocked) = (40.0, 40.0, 0.5, blocked);
        g.level.boxes = vec![roof([8.0, 3.0, 4.5], [2.0, 0.2, 3.0], 0.0, 0.0), roof([12.0, 3.0, 4.5], [2.0, 0.2, 3.0], 0.0, 2.0), roof([12.0, 1.0, 4.5], [2.0, 0.2, 3.0], 0.3, 0.0)];
        let mut breach = crate::scene_level::BreachPlan::zeroed();
        breach.door = String::from("plan");
        breach.fling = -3.0;
        breach.room = Some(String::from("vault"));
        g.level.breach = Some(breach);
        g.level.clear_plans = Some(vec![
            clear_plan("hostages", 0.0, None, Some(crate::scene_level::BreachPlanBounds { x0: 11.0, x1: 13.0, z0: 13.0, z1: 16.0 })),
            clear_plan("stopped", 2.0, Some("nope"), None),
            clear_plan("split", 2.0, None, None),
            clear_plan("split", -2.0, None, None),
        ]);
        g.level.mission.entry_door = String::from("entry");
        (g.level.mission.server_room.x0, g.level.mission.server_room.x1, g.level.mission.server_room.z0, g.level.mission.server_room.z1) = (2.0, 4.0, 15.0, 18.0);
        let mut sc = crate::scene_scenarios::LevelScenarios::zeroed();
        let mut rc = crate::scene_scenarios::StructureData::zeroed();
        rc.rooms = vec![room("b", "big", 11.0, 19.0, 8.0, 16.0, "room"), room("a", "small", 7.0, 9.0, 11.0, 14.0, "room")];
        sc.room_clear = Some(rc);
        let mut rs = crate::scene_scenarios::RescueData::zeroed();
        rs.room.name = String::from("cell");
        rs.room.door = String::from("hostages");
        let mut side = crate::scene_scenarios::RescueDataSide::zeroed();
        side.door = String::from("nobody");
        rs.side = Some(side);
        sc.rescue = Some(rs);
        g.level.scenarios = Some(sc);

        let want: [(&str, f64, &str, &str, f64); 12] = [
            ( "floor" , -1.0 , "floor" , "360 cells against 740" , -1.0 ),
            ( "authored" , -1.0 , "authored" , "" , 0.0 ),
            ( "folds" , 1.0 , "folds" , "" , 0.0 ),
            ( "plan" , -1.0 , "plan" , "vault" , 0.0 ),
            ( "room" , 1.0 , "room" , "small" , 0.0 ),
            ( "hostages" , -1.0 , "hostages" , "cell" , 0.0 ),
            ( "entry" , 1.0 , "mission" , "server room" , 0.0 ),
            ( "locked" , -1.0 , "lock" , "" , 0.0 ),
            ( "ajar" , 1.0 , "stands open" , "" , 0.0 ),
            ( "wider" , -1.0 , "wider" , "" , 0.0 ),
            ( "stopped" , -1.0 , "floor" , "360 cells against 740" , 0.0 ),
            ( "split" , 1.0 , "wider" , "" , 0.0 ),
        ];
        let given = given_of(&g);
        let mut fl = None;
        let list = g.doors.list.clone();
        for (&d, w) in list.iter().zip(want.iter()) {
            let name = g.dr[d].def.name.clone();
            let got = first_rule(&mut g, &given, &mut fl, d, &name);
            let (swing, why, into) = match &got {
                Some(s) => (s.swing, s.why.as_str(), s.into.clone().unwrap_or_default()),
                None => (if g.dr[d].max_auth >= -g.dr[d].min_auth { 1.0 } else { -1.0 }, "wider", String::new()),
            };
            let out = if why != "authored" && why != "folds" {
                let side = outside_of(&mut g, d);
                deal(&g, d, side, crate::doorswing::SwingWhy::Outside, Some("the open air")).map_or(0.0, |o| o.swing)
            } else {
                0.0
            };
            assert_eq!((name.as_str(), swing, why, into.as_str(), out), *w);
        }
    }
}
