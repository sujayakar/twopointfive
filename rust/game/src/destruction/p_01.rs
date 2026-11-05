//! src/game/destruction.ts lines 1 to 386
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `x.toFixed(2)`: JavaScript takes the larger of two equally near hundredths, Rust's `{:.2}` the even one; a tie is an odd multiple of 1/8.
fn to_fixed2(x: f64) -> String {
    if x.is_nan() {
        return String::from("NaN");
    }
    let a = m::abs(x);
    let t = a * 8.0;
    let tie = t == m::floor(t) && js::rem(t, 2.0) == 1.0;
    let s = if tie { format!("{:.2}", a + 0.005) } else { format!("{:.2}", a) };
    if x < 0.0 { format!("-{}", s) } else { s }
}

/// A word of a box's flags against a mask.
fn has_flag(flags: f64, mask: i32) -> bool {
    (js::to_i32(flags) & mask) != 0
}

/// What a box does, by its word.
fn does_of_box(g: &mut Game, b: BoxId) -> crate::scene_materials::Does {
    let word = crate::scene_materials::DoesOfB { does: g.col.boxes[b].does.map(|w| String::from(w.as_str())) };
    crate::scene_materials::does_of(g, Some(&word))
}

/// The knob as the systems read it: a showcase program's own, 0 under the tour, else tune.chaos clamped.   (destruction.ts chaosOf)
pub fn chaos_of(g: &mut crate::game::Game) -> f64 {
    if let Some(s) = g.destr.show_chaos {
        return m::max(0.0, m::min(1.0, s));
    }
    if g.touring {
        return 0.0;
    }
    let c = g.tune.chaos;
    m::max(0.0, m::min(1.0, c))
}

/// The loud levels' rules are live: a scenario is up.   (destruction.ts loudNow)
pub fn loud_now(g: &mut crate::game::Game) -> bool {
    crate::roomclear::room_clear_scenario_active(g) || g.night_breach.active || g.rescue.active
}

/// Impulses at all: chaos above 0 and the switch.   (destruction.ts impulsesOn)
pub fn impulses_on(g: &mut crate::game::Game) -> bool {
    chaos_of(g) > 0.0 && g.tune.debris_impulses
}

/// What of the kept knob is the game's: the first value. What a machine kept is the browser's storage, the host's.   (destruction.ts loadChaos)
pub fn load_chaos(g: &mut crate::game::Game, tune: &mut crate::destruction::ChaosTune) {
    tune.chaos = Some(g.k.chaos.def);
}

/// Nothing of the game's: the TypeScript only writes the browser's storage, the host's.   (destruction.ts saveChaos)
pub fn save_chaos(g: &mut crate::game::Game) {}

/// The knob and the caps back to the shipped values; the pall's and the mess's switches stay as the page opened. The kept copy's removal is the host's.   (destruction.ts resetChaosKnobs)
pub fn reset_chaos_knobs(g: &mut crate::game::Game) {
    let (on, solver, mess) = (g.k.chaos.pall.on, g.k.chaos.pall.solver, g.k.chaos.mess.on);
    g.k.chaos = chaos_defaults();
    g.k.chaos.pall.on = on;
    g.k.chaos.pall.solver = solver;
    g.k.chaos.mess.on = mess;
    g.tune.chaos = g.k.chaos.def;
    g.tune.debris_impulses = true;
}

/// The impulses' strength scale off the knob.   (destruction.ts impK)
pub fn imp_k(g: &mut crate::game::Game, chaos: f64) -> f64 {
    if chaos <= 0.5 { 1.0 } else { 1.0 + (g.k.chaos.imp.chaos_k - 1.0) * (chaos - 0.5) * 2.0 }
}

/// Under the ceiling slab: the office's footprint less a tile's width.   (destruction.ts indoors)
pub fn indoors(p: V3) -> bool {
    p[0] > 4.3 && p[0] < 35.7 && p[2] > 4.3 && p[2] < 23.7
}

/// The point just over a box's top, where line-of-sight targets aim.   (destruction.ts topOf)
pub fn top_of(g: &mut crate::game::Game, b: BoxId) -> V3 {
    let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
    [c[0], c[1] + h[1] + 0.05, c[2]]
}

/// The lane's capped log, for the panel and the probes.   (destruction.ts dlog)
pub fn dlog(g: &mut crate::game::Game, s: &str) {
    let line = format!("{} {}", to_fixed2(g.time), s);
    let log = &mut g.m.destruction.destr_log;
    log.push(line);
    if log.len() > 200 {
        log.drain(0..100);
    }
}

/// A cap of n() events in any one-second window.   (destruction.ts RateCap.constructor)
pub fn rate_cap_new(n: &crate::destruction::RateCapN) -> crate::destruction::RateCap {
    RateCap { n: n.clone(), ..RateCap::blank() }
}

/// One more event at time t, if the window has room.   (destruction.ts RateCap.take)
pub fn rate_cap_take(this: &mut crate::destruction::RateCap, t: f64, k: &crate::knobs::ChaosK) -> bool {
    let n = match this.n {
        RateCapN::Puff => k.puffs_per_sec,
        RateCapN::Paper => k.papers_per_sec,
        RateCapN::Knock => k.knocks_per_sec,
        RateCapN::Decal => k.decals_per_sec,
    };
    rate_cap_take_n(this, t, n)
}

/// RateCap.take with the knob's value read.
fn rate_cap_take_n(this: &mut RateCap, t: f64, n: f64) -> bool {
    let n = m::max(0.0, m::floor(n));
    if n == 0.0 {
        return false;
    }
    if this.ts.len() as f64 != n {
        this.ts = vec![-1e9; n as usize];
        this.head = 0.0;
    }
    let oldest = this.ts[this.head as usize];
    if t - oldest < 1.0 {
        return false;
    }
    this.ts[this.head as usize] = t;
    this.head = js::rem(this.head + 1.0, n);
    true
}

/// The window afresh.   (destruction.ts RateCap.reset)
pub fn rate_cap_reset(this: &mut crate::destruction::RateCap) {
    for x in this.ts.iter_mut() {
        *x = -1e9;
    }
    this.head = 0.0;
}

/// Stands the lane up. Built in `g.destr`, since everything it calls reaches it by that path, and handed back for the caller's `g.destr = …`.   (destruction.ts Destruction.constructor)
pub fn destruction_new(g: &mut crate::game::Game) -> crate::destruction::Destruction {
    let world = crate::scene_world::world();
    g.destr = Destruction::blank();
    g.destr.w = world.grid_w;
    g.destr.cs = world.grid_cell;
    g.destr.debris = crate::debris::debris_new(g);
    g.destr.breaks = crate::break_table::breaks_new(g);
    g.destr.hw = m::ceil((world.max[0] - world.min[0]) / g.k.chaos.cell);
    g.destr.hh = m::ceil((world.max[2] - world.min[2]) / g.k.chaos.cell);
    g.destr.heat = vec![0.0f32; (g.destr.hw * g.destr.hh) as usize];
    g.destr.cells = vec![Vec::new(); (world.grid_w * world.grid_h) as usize];
    destruction_index(g);
    g.destr.walls = Box::new(crate::wallbreak::wall_break_new(g));
    g.destr.edge_look = crate::edge_look::edge_look_new(g);
    g.destr.breaks.look = Some(crate::break_table::BreaksLook { on: g.m.edge_look.edge.on });
    g.destr.furn = crate::furnblast::furniture_new(g);
    core::mem::replace(&mut g.destr, Destruction::blank())
}

/// Walks the statics once: what can break, by shape and by its word.   (destruction.ts Destruction.index)
pub fn destruction_index(g: &mut crate::game::Game) {
    let world = crate::scene_world::world();
    let boxes_len = g.col.boxes.len();
    let bf = crate::scene_boxes::box_flag();
    let skip = js::to_i32(bf.glass) | js::to_i32(bf.no_primary);
    let no_shadow = js::to_i32(bf.no_shadow);
    g.destr.target_of_box.clear();
    for t in g.targets.clone() {
        for bi in g.tab.sim_lightshome_fixture[t].boxes.clone() {
            g.destr.target_of_box.set(crate::Num(bi), t);
        }
    }
    let mut fixture_boxes: JsSet<crate::Num> = JsSet::new();
    for k in g.destr.target_of_box.keys() {
        fixture_boxes.add(k);
    }
    for l in g.level.lights.iter() {
        if let Some(fb) = l.fixture_box {
            if fb >= 0.0 {
                fixture_boxes.add(crate::Num(fb));
            }
        }
    }
    let mut screens: Vec<usize> = Vec::new();
    for i in 0..boxes_len {
        let b = BoxId::from_ix(i);
        let h = g.col.boxes[b].h;
        let max_h = m::max_n(&h);
        let min_h = m::min_n(&h);
        let flags = g.col.boxes[b].flags;
        if has_flag(flags, skip) || g.col.boxes[b].piece == Some(true) {
            continue;
        }
        let bottom = g.col.boxes[b].c[1] - h[1];
        let e = g.col.boxes[b].emissive;
        let dark = e[0] + e[1] + e[2] == 0.0;
        let does = does_of_box(g, b);
        let kind = if has_flag(flags, no_shadow) && h[1] <= 0.03 && does.paper.unwrap_or(false) && max_h >= 0.08 && max_h <= 0.2 && bottom > 0.3 && dark {
            Some(DKind::Paper)
        } else if does.screen.unwrap_or(false) && min_h <= 0.012 {
            Some(if max_h > 0.35 || (bottom > 1.2 && max_h > 0.3) { DKind::Tv } else { DKind::Screen })
        } else if !has_flag(flags, no_shadow) && !fixture_boxes.has(crate::Num(i as f64)) && max_h <= 0.17 && h[0] * h[1] * h[2] * 8.0 <= 0.02 && bottom >= 0.7 && bottom <= 1.6 && dark {
            Some(if h[1] >= 0.1 && h[1] > 2.5 * m::min(h[0], h[2]) { DKind::Binder } else { DKind::Clutter })
        } else {
            None
        };
        let Some(kind) = kind else { continue };
        let mut d = Destructible { i: i as f64, kind, box_: b, flags0: flags, mate: None, target: None, group: None };
        if kind == DKind::Screen || kind == DKind::Tv {
            if let Some(&t) = g.destr.target_of_box.get(crate::Num(i as f64)) {
                d.target = Some(t);
            }
            if kind == DKind::Screen {
                screens.push(i);
            }
        }
        g.destr.items.set(crate::Num(i as f64), d);
    }
    // each desk screen with the housing slab just behind it
    for si in screens {
        let s = BoxId::from_ix(si);
        let (s_yaw, s_h, s_c) = (g.col.boxes[s].yaw, g.col.boxes[s].h, g.col.boxes[s].c);
        let mut best: Option<usize> = None;
        let mut bd = 0.08;
        for i in si.saturating_sub(6)..core::cmp::min(boxes_len, si + 6) {
            if i == si {
                continue;
            }
            let b = BoxId::from_ix(i);
            let h = g.col.boxes[b].h;
            if m::abs(g.col.boxes[b].yaw - s_yaw) > 1e-3 || h[0] < s_h[0] || h[1] < s_h[1] || h[2] > 0.05 || !does_of_box(g, b).casing.unwrap_or(false) {
                continue;
            }
            let dd = v3::dist(g.col.boxes[b].c, s_c);
            if dd < bd {
                bd = dd;
                best = Some(i);
            }
        }
        if let Some(best) = best {
            let best_key = crate::Num(best as f64);
            let target = {
                let sd = g.destr.items.get_mut(crate::Num(si as f64)).unwrap();
                sd.mate = Some(best as f64);
                sd.target
            };
            let had = g.destr.items.get(best_key).cloned();
            let mut hd = match had {
                Some(d) => d,
                None => Destructible { i: best as f64, kind: DKind::Housing, box_: BoxId::from_ix(best), flags0: g.col.boxes[BoxId::from_ix(best)].flags, mate: None, target: None, group: None },
            };
            hd.kind = DKind::Housing;
            hd.mate = Some(si as f64);
            hd.target = target;
            g.destr.items.set(best_key, hd);
        }
    }
    // a monitor's foot and neck, a fixture's own parts and whatever has no roof over it are not destructibles
    let mut fixtures: Vec<(V3, V3, f64)> = Vec::new();
    for fi in fixture_boxes.iter() {
        if (fi.0 as usize) < boxes_len {
            let f = &g.col.boxes[BoxId::from_ix(fi.0 as usize)];
            fixtures.push((f.c, f.h, f.yaw));
        }
    }
    crate::scene_collision::window(g, &mut |g: &mut Game| index_prune(g, &fixtures));
    // things standing on or against each other go together
    let mut smalls: Vec<(f64, V3, V3)> = Vec::new();
    let small_rows: Vec<(f64, BoxId)> = g.destr.items.values().filter(|d| d.kind == DKind::Clutter || d.kind == DKind::Binder).map(|d| (d.i, d.box_)).collect();
    for (i, b) in small_rows {
        smalls.push((i, g.col.boxes[b].c, g.col.boxes[b].h));
    }
    for d in 0..smalls.len() {
        for o in 0..smalls.len() {
            if o == d {
                continue;
            }
            let (di, ac, ah) = smalls[d];
            let (oi, bc, bh) = smalls[o];
            if m::abs(ac[0] - bc[0]) > 0.07 + m::min(ah[0], bh[0]) || m::abs(ac[2] - bc[2]) > 0.07 + m::min(ah[2], bh[2]) {
                continue;
            }
            if m::abs(ac[1] - bc[1]) > ah[1] + bh[1] + 0.03 {
                continue;
            }
            g.destr.items.get_mut(crate::Num(di)).unwrap().group.get_or_insert_with(Vec::new).push(oi);
        }
    }
    // the 1 m cells
    let rows: Vec<(f64, BoxId)> = g.destr.items.iter().map(|(i, d)| (i.0, d.box_)).collect();
    for (i, b) in rows {
        let c = g.col.boxes[b].c;
        let cx = m::floor((c[0] - world.min[0]) / g.destr.cs);
        let cz = m::floor((c[2] - world.min[2]) / g.destr.cs);
        if cx >= 0.0 && cz >= 0.0 && cx < g.destr.w && cz < world.grid_h {
            let at = (cz * g.destr.w + cx) as usize;
            g.destr.cells[at].push(i);
        }
    }
}

/// The walk over a copy of the items that takes out what is no destructible; only the entry it stands on is ever deleted.   (destruction.ts Destruction.index, the window's body)
fn index_prune(g: &mut Game, fixtures: &[(V3, V3, f64)]) {
    for i in g.destr.items.keys() {
        let (b, kind) = {
            let d = g.destr.items.get(i).unwrap();
            (d.box_, d.kind)
        };
        let c = g.col.boxes[b].c;
        if destruction_roof_y(g, c[0], c[2]).is_none() {
            g.destr.items.delete(i);
            continue;
        }
        if kind != DKind::Clutter && kind != DKind::Binder {
            continue;
        }
        let by_fixture = fixtures.iter().any(|&(fc, fh, fyaw)| {
            let ac = m::abs(m::cos(fyaw));
            let as_ = m::abs(m::sin(fyaw));
            m::abs(fc[0] - c[0]) < ac * fh[0] + as_ * fh[2] + 0.12 && m::abs(fc[2] - c[2]) < as_ * fh[0] + ac * fh[2] + 0.12 && m::abs(fc[1] - c[1]) < 0.7
        });
        if by_fixture {
            g.destr.items.delete(i);
            continue;
        }
        if !does_of_box(g, b).casing.unwrap_or(false) {
            continue;
        }
        let housings: Vec<BoxId> = g.destr.items.values().filter(|o| o.kind == DKind::Housing).map(|o| o.box_).collect();
        for hb in housings {
            let oc = g.col.boxes[hb].c;
            if m::hyp2(oc[0] - c[0], oc[2] - c[2]) < 0.12 && oc[1] > c[1] {
                g.destr.items.delete(i);
                break;
            }
        }
    }
}

/// The underside of the roof over (x, z), or none under the open sky: five rays a 0.6 m square, remembered per square.   (destruction.ts Destruction.roofY)
pub fn destruction_roof_y(g: &mut crate::game::Game, x: f64, z: f64) -> Option<f64> {
    let key = crate::Num(m::floor(x / 0.6) * 4096.0 + m::floor(z / 0.6) + 0.0);
    if let Some(&c) = g.destr.roofs.get(key) {
        return if c < 0.0 { None } else { Some(c) };
    }
    let cx = (m::floor(x / 0.6) + 0.5) * 0.6;
    let cz = (m::floor(z / 0.6) + 0.5) * 0.6;
    let mut n = 0;
    let mut top = -1.0;
    let tmax = crate::scene_world::world().max[1] - 2.3;
    for [dx, dz] in roof_rays() {
        if let Some(hit) = crate::scene_collision::raycast(g, [cx + dx, 2.3, cz + dz], [0.0, 1.0, 0.0], tmax) {
            n += 1;
            top = m::max(top, 2.3 + hit.t);
        }
    }
    let y = if n >= 4 { top } else { -1.0 };
    g.destr.roofs.set(key, y);
    if y < 0.0 { None } else { Some(y) }
}

/// Destructibles registered in the 3×3 cells round p, hidden ones left out.   (destruction.ts Destruction.near)
pub fn destruction_near(g: &mut crate::game::Game, p: V3, out: &mut Vec<crate::destruction::Destructible>) -> Vec<crate::destruction::Destructible> {
    out.clear();
    let world = crate::scene_world::world();
    let cx = m::floor((p[0] - world.min[0]) / g.destr.cs);
    let cz = m::floor((p[2] - world.min[2]) / g.destr.cs);
    for dz in -1..=1 {
        for dx in -1..=1 {
            let x = cx + dx as f64;
            let z = cz + dz as f64;
            if x < 0.0 || z < 0.0 || x >= g.destr.w || z >= world.grid_h {
                continue;
            }
            for &i in g.destr.cells[(z * g.destr.w + x) as usize].iter() {
                if let Some(d) = g.destr.items.get(crate::Num(i)) {
                    if !g.destr.hidden.has(crate::Num(i)) {
                        out.push(d.clone());
                    }
                }
            }
        }
    }
    out.clone()
}

/// Counts by kind, for the panel and the probes.   (destruction.ts Destruction.inventory)
pub fn destruction_inventory(g: &mut crate::game::Game) -> JsMap<String, f64> {
    let mut out: JsMap<String, f64> = JsMap::new();
    for d in g.destr.items.values() {
        let key = String::from(d.kind.as_str());
        let n = out.get(key.clone()).copied().unwrap_or(0.0);
        out.set(key, n + 1.0);
    }
    out.set(String::from("hidden"), g.destr.hidden.len() as f64);
    out
}

/// The destructible at a static's index.   (destruction.ts Destruction.itemOf)
pub fn destruction_item_of(g: &mut crate::game::Game, i: f64) -> Option<crate::destruction::Destructible> {
    g.destr.items.get(crate::Num(i)).cloned()
}

/// The heat cell of a point, clamped to the grid.   (destruction.ts Destruction.hidx)
pub fn destruction_hidx(g: &mut crate::game::Game, p: V3) -> f64 {
    let world = crate::scene_world::world();
    let cell = g.k.chaos.cell;
    let x = m::min(g.destr.hw - 1.0, m::max(0.0, m::floor((p[0] - world.min[0]) / cell)));
    let z = m::min(g.destr.hh - 1.0, m::max(0.0, m::floor((p[2] - world.min[2]) / cell)));
    z * g.destr.hw + x
}

/// Adds heat at p and gives the escalation there after it.   (destruction.ts Destruction.stoke)
pub fn destruction_stoke(g: &mut crate::game::Game, p: V3, amount: f64, chaos: f64) -> f64 {
    let i = destruction_hidx(g, p) as usize;
    g.destr.heat[i] = (g.destr.heat[i] as f64 + amount) as f32;
    g.destr.heat[i] as f64 * 2.0 * chaos
}

/// The heat at p.   (destruction.ts Destruction.heatAt)
pub fn destruction_heat_at(g: &mut crate::game::Game, p: V3) -> f64 {
    let i = destruction_hidx(g, p) as usize;
    g.destr.heat[i] as f64
}

/// The escalation at p, as things stand.   (destruction.ts Destruction.escalationAt)
pub fn destruction_escalation_at(g: &mut crate::game::Game, p: V3) -> f64 {
    let heat = destruction_heat_at(g, p);
    heat * 2.0 * chaos_of(g)
}

/// The module's own hashed choice, counting off its serial.   (destruction.ts Destruction.hash)
pub fn destruction_hash(g: &mut crate::game::Game) -> f64 {
    let s = g.destr.serial;
    g.destr.serial = s + 1.0;
    crate::debris::hash01(s * 2654435761.0)
}

/// One event out of a cap; a knock-off riding on its group leader's slot is free.   (destruction.ts Destruction.take)
pub fn destruction_take(g: &mut crate::game::Game, k: crate::destruction::DestructionTakeK) -> bool {
    if k == DestructionTakeK::Knock && g.destr.free_knocks > 0.0 {
        g.destr.free_knocks -= 1.0;
        return true;
    }
    let t = g.time;
    let (caps, knobs) = (&mut g.destr.caps, &g.k.chaos);
    match k {
        DestructionTakeK::Puff => rate_cap_take(&mut caps.puff, t, knobs),
        DestructionTakeK::Paper => rate_cap_take(&mut caps.paper, t, knobs),
        DestructionTakeK::Knock => rate_cap_take(&mut caps.knock, t, knobs),
        DestructionTakeK::Decal => rate_cap_take(&mut caps.decal, t, knobs),
    }
}

/// The RUBBLE table as Debris.feet's knobs: the TypeScript hands the table itself.
fn foot_knobs(g: &Game) -> crate::debris::FootKnobs {
    let r = &g.k.rubble;
    crate::debris::FootKnobs {
        radius: r.radius,
        reach: r.reach,
        lead: r.lead,
        slide: r.slide,
        inside: r.inside,
        floor: r.floor,
        heavy: r.heavy,
        walk: r.walk,
        base: r.base,
        gain: r.gain,
        aside: r.aside,
        v_max: r.v_max,
        off: r.off,
        spin: r.spin,
        per_frame: r.per_frame,
        cooldown: r.cooldown,
        give_up: r.give_up,
        give_up_secs: r.give_up_secs,
        scuff: r.scuff,
    }
}

/// Per frame: rubble at men's feet, the pools, the heat's decay, the queued dust.   (destruction.ts Destruction.update)
pub fn destruction_update(g: &mut crate::game::Game, dt: f64) {
    if !g.touring || g.destr.show_chaos.is_some() {
        let men: Vec<crate::debris::FootMan> = destruction_on_foot(g).into_iter().map(|c| crate::debris::FootMan { pos: g.ch[c].pos, vel: g.ch[c].vel }).collect();
        let knobs = foot_knobs(g);
        if !g.destr.debris.list.is_empty() {
            crate::debris::debris_feet(g, &men, knobs, None, None);
        }
        crate::edge_look::edge_look_feet(g, &men, knobs, dt);
    }
    if js::truthy(crate::sim_fallshome::fall_store_waiting_in(g, crate::sim_fallshome::FALL_PIECES)) {
        crate::debris::debris_landings(g);
    }
    if !g.destr.debris.list.is_empty() {
        crate::debris::debris_update(g, dt);
    }
    crate::break_table::breaks_tick(g);
    crate::edge_look::edge_look_update(g, dt);
    g.destr.thuds_this_frame = 0.0;
    if chaos_of(g) <= 0.0 && !g.destr.any_heat {
        return;
    }
    let k = m::exp(-dt / g.k.chaos.heat_tau);
    let mut any = false;
    for x in g.destr.heat.iter_mut() {
        let h = *x as f64 * k;
        *x = if h < 0.02 { 0.0 } else { h as f32 };
        if h >= 0.02 {
            any = true;
        }
    }
    g.destr.puffs_this_frame = 0.0;
    g.destr.free_knocks = 0.0;
    if chaos_of(g) <= 0.0 {
        g.destr.dust_queue.clear();
    }
    g.destr.any_heat = any || !g.destr.dust_queue.is_empty();
    // queued dust, oldest first: a puff the caps hold back this frame waits, and is counted refused only when it is let go
    while !g.destr.dust_queue.is_empty() {
        let q = g.destr.dust_queue[0];
        if g.time - q.t > g.k.chaos.loud.burst_secs {
            g.destr.dust_queue.remove(0);
            g.stats.destruction.destr_puffs_refused += 1;
            continue;
        }
        if g.destr.puffs_this_frame >= g.k.chaos.puffs_per_frame {
            break;
        }
        let r0 = g.stats.destruction.destr_puffs_refused;
        if !destruction_dust_puff(g, q.p, q.n, q.scale, q.surf) {
            g.stats.destruction.destr_puffs_refused = r0;
            break;
        }
        g.destr.dust_queue.remove(0);
    }
}

/// The men on their feet this frame, bar a seated civilian.   (destruction.ts Destruction.onFoot)
pub fn destruction_on_foot(g: &mut crate::game::Game) -> Vec<CharId> {
    let mut men: Vec<CharId> = crate::seats::men_up(g).into_iter().map(|pl| g.pl[pl].ch).collect();
    for gd in g.guards.iter() {
        if g.gd[*gd].state != crate::game::GuardState::Dead {
            men.push(g.gd[*gd].ch);
        }
    }
    for cv in crate::civilians::civilians_shown(g) {
        if g.gd[cv].state != crate::game::GuardState::Dead && !crate::civilians::civilian_seated(g, cv) {
            men.push(g.gd[cv].ch);
        }
    }
    men
}

/// The rigid debris' and the blown walls' shadow-casting boxes, for the picture.   (destruction.ts Destruction.boxes)
pub fn destruction_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    crate::debris::debris_boxes(g, out);
    crate::edge_look::edge_look_boxes(g, out);
}

/// Level reset: every hidden static back as authored, pools emptied, heat, caps and the counter back to the start.   (destruction.ts Destruction.reset)
pub fn destruction_reset(g: &mut crate::game::Game) {
    for i in g.destr.hidden.keys() {
        let f = *g.destr.hidden.get(i).unwrap();
        g.col.boxes[BoxId::from_ix(i.0 as usize)].flags = f;
    }
    crate::scene_collision::wrote(g);
    g.destr.hidden.clear();
    crate::debris::debris_clear(g);
    for x in g.destr.heat.iter_mut() {
        *x = 0.0;
    }
    g.destr.any_heat = false;
    g.destr.tiles_down.clear();
    g.destr.last_tile_t = -100.0;
    g.destr.tile_count = 0.0;
    g.destr.serial = 1.0;
    g.destr.knocks = 0.0;
    rate_cap_reset(&mut g.destr.caps.puff);
    rate_cap_reset(&mut g.destr.caps.paper);
    rate_cap_reset(&mut g.destr.caps.knock);
    rate_cap_reset(&mut g.destr.caps.decal);
    g.destr.free_knocks = 0.0;
    for t in g.destr.broke.keys() {
        crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().mend, Some(t), None, None, None, None);
    }
    g.destr.broke.clear();
    g.destr.fixture_count = 0.0;
    g.destr.last_fixture_t = -100.0;
    g.destr.dust_queue.clear();
    g.destr.pall_at.clear();
    crate::break_table::breaks_reset(g);
    crate::edge_look::edge_look_clear(g);
    crate::wallbreak::wall_break_reset(g, None);
    crate::furnblast::furniture_reset(g);
}

/// The tour's cuts: nothing tumbling carries over; hidden statics stay hidden.   (destruction.ts Destruction.clearMoving)
pub fn destruction_clear_moving(g: &mut crate::game::Game) {
    crate::debris::debris_clear(g);
    g.destr.dust_queue.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fixed2_matches_node() {
        // node: x.toFixed(2)
        let rows: [(f64, &str); 11] = [(0.125, "0.13"), (0.375, "0.38"), (0.625, "0.63"), (0.875, "0.88"), (1.125, "1.13"), (12.345, "12.35"), (0.0, "0.00"), (-0.125, "-0.13"), (-0.001, "-0.00"), (2.5, "2.50"), (100.005, "100.00")];
        for (x, want) in rows {
            assert_eq!(to_fixed2(x), want, "{}", x);
        }
    }

    #[test]
    fn rate_cap_holds_n_in_any_second() {
        let mut c = RateCap::blank();
        assert!(rate_cap_take_n(&mut c, 0.0, 2.0));
        assert!(rate_cap_take_n(&mut c, 0.5, 2.0));
        assert!(!rate_cap_take_n(&mut c, 0.9, 2.0));
        assert!(rate_cap_take_n(&mut c, 1.0, 2.0));
        assert!(!rate_cap_take_n(&mut c, 1.4, 2.0));
        assert!(rate_cap_take_n(&mut c, 1.5, 2.0));
        // a floored knob of nothing refuses; a changed cap starts the window afresh
        assert!(!rate_cap_take_n(&mut c, 9.0, 0.5));
        assert!(rate_cap_take_n(&mut c, 1.6, 3.0));
        assert_eq!(c.ts.len(), 3);
        assert_eq!(c.head, 1.0);
        rate_cap_reset(&mut c);
        assert_eq!(c.head, 0.0);
        assert!(c.ts.iter().all(|&t| t == -1e9));
    }

    #[test]
    fn rate_cap_reads_its_own_knob_live() {
        let mut k = crate::knobs::ChaosK::zeroed();
        k.papers_per_sec = 1.0;
        k.puffs_per_sec = 5.0;
        let mut c = rate_cap_new(&RateCapN::Paper);
        assert!(rate_cap_take(&mut c, 0.0, &k));
        assert!(!rate_cap_take(&mut c, 0.5, &k));
        // the panel raised the cap: the window starts afresh
        k.papers_per_sec = 2.0;
        assert!(rate_cap_take(&mut c, 0.6, &k));
        assert_eq!(c.ts.len(), 2);
    }

    #[test]
    fn indoors_is_strict() {
        assert!(indoors([4.31, 0.0, 4.31]));
        assert!(!indoors([4.3, 0.0, 10.0]));
        assert!(!indoors([10.0, 0.0, 23.7]));
    }
}
