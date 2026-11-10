use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// a box's row
fn seam_box(g: &mut Game, b: BoxId) -> &mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// A word of a box's flags against a mask.
fn has_flag(flags: f64, mask: i32) -> bool {
    (js::to_i32(flags) & mask) != 0
}

/// What a box does, by its word.
fn does_of_box(g: &mut Game, b: BoxId) -> crate::scene_materials::Does {
    let word = seam_box(g, b).does;
    crate::scene_materials::does_of(g, word)
}

/// The knob as the systems read it: tune.chaos, from 0 to 1.
pub fn chaos_of(g: &crate::game::Game) -> f64 {
    let c = g.tune.chaos;
    m::max(0.0, m::min(1.0, c))
}

/// The loud levels' rules are live: a scenario is up.
pub(super) fn loud_now(g: &crate::game::Game) -> bool {
    g.staged.facts.fight
}

/// Impulses at all: chaos above 0 and the switch.
pub(in crate::breakage) fn impulses_on(g: &crate::game::Game) -> bool {
    chaos_of(g) > 0.0 && g.tune.debris_impulses
}

/// What of the kept knob is the game's: the first value. What a machine kept is the browser's storage, the host's.
pub fn load_chaos(g: &mut crate::game::Game) {
    g.tune.chaos = g.k.chaos.def;
}

/// The impulses' strength scale off the knob.
pub(super) fn imp_k(g: &crate::game::Game, chaos: f64) -> f64 {
    if chaos <= 0.5 { 1.0 } else { 1.0 + (g.k.chaos.imp.chaos_k - 1.0) * (chaos - 0.5) * 2.0 }
}

/// Under the ceiling slab: the office's footprint less a tile's width.
fn indoors(p: V3) -> bool {
    p[0] > 4.3 && p[0] < 35.7 && p[2] > 4.3 && p[2] < 23.7
}

/// The point just over a box's top, where line-of-sight targets aim.
pub(super) fn top_of(g: &mut crate::game::Game, b: BoxId) -> V3 {
    let (c, h) = (seam_box(g, b).c, seam_box(g, b).h);
    [c[0], c[1] + h[1] + 0.05, c[2]]
}

/// The lane's capped log, for the panel and the probes.
pub(super) fn dlog(g: &mut crate::game::Game, s: &str) {
    let line = format!("{} {}", js::to_fixed(g.time, 2), s);
    let log = &mut g.m.destruction.destr_log;
    log.push(line);
    if log.len() > 200 {
        log.drain(0..100);
    }
}

/// A cap of n() events in any one-second window.
fn rate_cap_new(n: crate::destruction::RateCapN) -> crate::destruction::RateCap {
    RateCap { n, ..RateCap::blank() }
}

/// One more event at time t, if the window has room.
fn rate_cap_take(this: &mut crate::destruction::RateCap, t: f64, k: &crate::knobs::ChaosK) -> bool {
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

/// The window afresh.
fn rate_cap_reset(this: &mut crate::destruction::RateCap) {
    for x in this.ts.iter_mut() {
        *x = -1e9;
    }
    this.head = 0.0;
}

/// Stands the lane up. Built in `g.destr`, since everything it calls reaches it by that path, and handed back for the caller's `g.destr = …`.
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
    g.destr.breaks.look = Some(crate::break_table::BreaksLook {  });
    g.destr.furn = crate::furnblast::furniture_new(g);
    core::mem::replace(&mut g.destr, Destruction::blank())
}

/// Walks the statics once: what can break, by shape and by its word.
fn destruction_index(g: &mut crate::game::Game) {
    let world = crate::scene_world::world();
    let boxes_len = g.col.boxes.len();
    let bf = crate::scene_boxes::box_flag();
    let skip = js::to_i32(bf.glass) | js::to_i32(bf.no_primary);
    let no_shadow = js::to_i32(bf.no_shadow);
    g.destr.target_of_box.clear();
    for t in g.targets.clone() {
        for bi in g.tab.fixture[t].boxes.clone() {
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
        let h = seam_box(g, b).h;
        let max_h = m::max_n(&h);
        let min_h = m::min_n(&h);
        let flags = seam_box(g, b).flags;
        if has_flag(flags, skip) || seam_box(g, b).piece == Some(true) {
            continue;
        }
        let bottom = seam_box(g, b).c[1] - h[1];
        let e = seam_box(g, b).emissive;
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
        let mut d = Destructible { i: i as f64, kind, box_: b, mate: None, target: None, group: None };
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
        let (s_yaw, s_h, s_c) = (seam_box(g, s).yaw, seam_box(g, s).h, seam_box(g, s).c);
        let mut best: Option<usize> = None;
        let mut bd = 0.08;
        for i in si.saturating_sub(6)..core::cmp::min(boxes_len, si + 6) {
            if i == si {
                continue;
            }
            let b = BoxId::from_ix(i);
            let h = seam_box(g, b).h;
            if m::abs(seam_box(g, b).yaw - s_yaw) > 1e-3 || h[0] < s_h[0] || h[1] < s_h[1] || h[2] > 0.05 || !does_of_box(g, b).casing.unwrap_or(false) {
                continue;
            }
            let dd = v3::dist(seam_box(g, b).c, s_c);
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
                None => Destructible { i: best as f64, kind: DKind::Housing, box_: BoxId::from_ix(best), mate: None, target: None, group: None },
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
            let f = &seam_box(g, BoxId::from_ix(fi.0 as usize));
            fixtures.push((f.c, f.h, f.yaw));
        }
    }
    crate::scene_collision::window(g, &mut |g: &mut Game| index_prune(g, &fixtures));
    // things standing on or against each other go together
    let mut smalls: Vec<(f64, V3, V3)> = Vec::new();
    let small_rows: Vec<(f64, BoxId)> = g.destr.items.values().filter(|d| d.kind == DKind::Clutter || d.kind == DKind::Binder).map(|d| (d.i, d.box_)).collect();
    for (i, b) in small_rows {
        smalls.push((i, seam_box(g, b).c, seam_box(g, b).h));
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
        let c = seam_box(g, b).c;
        let cx = m::floor((c[0] - world.min[0]) / g.destr.cs);
        let cz = m::floor((c[2] - world.min[2]) / g.destr.cs);
        if cx >= 0.0 && cz >= 0.0 && cx < g.destr.w && cz < world.grid_h {
            let at = (cz * g.destr.w + cx) as usize;
            g.destr.cells[at].push(i);
        }
    }
}

/// The walk over a copy of the items that takes out what is no destructible; only the entry it stands on is ever deleted.
fn index_prune(g: &mut Game, fixtures: &[(V3, V3, f64)]) {
    for i in g.destr.items.keys() {
        let (b, kind) = {
            let d = g.destr.items.get(i).unwrap();
            (d.box_, d.kind)
        };
        let c = seam_box(g, b).c;
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
            let oc = seam_box(g, hb).c;
            if m::hyp2(oc[0] - c[0], oc[2] - c[2]) < 0.12 && oc[1] > c[1] {
                g.destr.items.delete(i);
                break;
            }
        }
    }
}

/// The underside of the roof over (x, z), or none under the open sky: five rays a 0.6 m square, remembered per square.
pub(super) fn destruction_roof_y(g: &mut crate::game::Game, x: f64, z: f64) -> Option<f64> {
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

/// Destructibles registered in the 3×3 cells round p, hidden ones left out.
pub(super) fn destruction_near(g: &crate::game::Game, p: V3, out: &mut Vec<crate::destruction::Destructible>) -> Vec<crate::destruction::Destructible> {
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

/// The destructible at a static's index.
pub fn destruction_item_of(g: &crate::game::Game, i: f64) -> Option<crate::destruction::Destructible> {
    g.destr.items.get(crate::Num(i)).cloned()
}

/// The heat cell of a point, clamped to the grid.
fn destruction_hidx(g: &crate::game::Game, p: V3) -> f64 {
    let world = crate::scene_world::world();
    let cell = g.k.chaos.cell;
    let x = m::min(g.destr.hw - 1.0, m::max(0.0, m::floor((p[0] - world.min[0]) / cell)));
    let z = m::min(g.destr.hh - 1.0, m::max(0.0, m::floor((p[2] - world.min[2]) / cell)));
    z * g.destr.hw + x
}

/// Adds heat at p and gives the escalation there after it.
pub(super) fn destruction_stoke(g: &mut crate::game::Game, p: V3, amount: f64, chaos: f64) -> f64 {
    let i = destruction_hidx(g, p) as usize;
    g.destr.heat[i] = (g.destr.heat[i] as f64 + amount) as f32;
    g.destr.heat[i] as f64 * 2.0 * chaos
}

/// The module's own hashed choice, counting off its serial.
pub(super) fn destruction_hash(g: &mut crate::game::Game) -> f64 {
    let s = g.destr.serial;
    g.destr.serial = s + 1.0;
    crate::debris::hash01(s * 2654435761.0)
}

/// One event out of a cap; a knock-off riding on its group leader's slot is free.
pub(super) fn destruction_take(g: &mut crate::game::Game, k: crate::destruction::DestructionTakeK) -> bool {
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

/// Per frame: rubble at men's feet, the pools, the heat's decay, the queued dust.
pub fn destruction_update(g: &mut crate::game::Game, dt: f64) {
    let mut men: Vec<crate::debris::FootMan> = destruction_on_foot(g).into_iter().map(|c| crate::debris::FootMan { pos: g.ch[c].pos, vel: g.ch[c].vel, body: false }).collect();
    if !g.destr.debris.list.is_empty() || !g.destr.edge_look.rubble.is_empty() { destruction_bodies(g, &mut men); }
    let knobs = foot_knobs(g);
    if !g.destr.debris.list.is_empty() {
        crate::debris::debris_feet(g, &men, knobs, None, None);
    }
    crate::edge_look::edge_look_feet(g, &men, knobs, dt);
    if js::truthy(crate::sim_fallshome::fall_store_waiting_in(g, crate::sim_fallshome::FALL_PIECES)) {
        crate::debris::debris_landings(g);
    }
    if !g.destr.debris.list.is_empty() {
        crate::debris::debris_update(g, dt);
    }
    crate::break_table::breaks_tick(g);
    crate::edge_look::edge_look_update(g, dt);
    crate::wallskin::wall_skin_update(g);
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

/// THE MEN DOWN, as the places a piece may not lie: a place under each of a lying man's main bones, at his floor's height (below him: `debris_feet` asks a place's height of what lies there).
fn destruction_bodies(g: &Game, out: &mut Vec<crate::debris::FootMan>) {
    use crate::character::Bone;
    const BONES: [Bone; 9] = [Bone::Head, Bone::Chest, Bone::Hips, Bone::ThighL, Bone::ThighR, Bone::ShinL, Bone::ShinR, Bone::FootL, Bone::FootR];
    let mut men: Vec<CharId> = g.guards.iter().filter(|&&gd| g.ch[g.gd[gd].ch].ragdoll.is_some()).map(|&gd| g.gd[gd].ch).collect();
    for cv in crate::civilians::civilians_shown(g) {
        if g.ch[g.gd[cv].ch].ragdoll.is_some() { men.push(g.gd[cv].ch); }
    }
    for s in g.seats.clone() {
        let c = g.pl[crate::cmd::seat_man(g, s)].ch;
        if g.ch[c].ragdoll.is_some() { men.push(c); }
    }
    for c in men {
        // (his floor is under his lowest bone: a ragdoll's own place says nothing of the height it lies at. A bone well over that is a limb propped up, with nothing under it)
        let bones: Vec<V3> = BONES.iter().filter_map(|&k| g.ch[c].bones.get(k).copied()).collect();
        let low = bones.iter().fold(m::INF, |l, p| m::min(l, p[1]));
        for p in bones {
            if p[1] < low + 0.6 { out.push(crate::debris::FootMan { pos: [p[0], low - 0.1, p[2]], vel: [0.0; 3], body: true }); }
        }
    }
}


/// The men on their feet this frame, bar a seated civilian.
fn destruction_on_foot(g: &crate::game::Game) -> Vec<CharId> {
    let mut men: Vec<CharId> = crate::seats::men_up(g).into_iter().map(|pl| g.pl[pl].ch).collect();
    for gd in g.guards.iter() {
        if !crate::condition::down(g, g.gd[*gd].ch) {
            men.push(g.gd[*gd].ch);
        }
    }
    for cv in crate::civilians::civilians_shown(g) {
        if !crate::condition::down(g, g.gd[cv].ch) && !crate::civilians::civilian_seated(g, cv) {
            men.push(g.gd[cv].ch);
        }
    }
    men
}

/// The rigid debris' and the blown walls' shadow-casting boxes, for the picture.
pub fn destruction_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    crate::debris::debris_boxes(g, out);
    crate::edge_look::edge_look_boxes(g, out);
    crate::wallskin::wall_skin_boxes(g, out);   // (…and what stands of a holed wall's paint: wallskin.ts)
}

/// Level reset: every hidden static back as authored, pools emptied, heat, caps and the counter back to the start.
pub fn destruction_reset(g: &mut crate::game::Game) {
    for i in g.destr.hidden.keys() {
        let f = *g.destr.hidden.get(i).unwrap();
        seam_box(g, BoxId::from_ix(i.0 as usize)).flags = f;
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
        g.tab.fixture[t].broken = false;
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

/// Nothing tumbling carries over; hidden statics stay hidden.
pub fn destruction_clear_moving(g: &mut crate::game::Game) {
    crate::debris::debris_clear(g);
    g.destr.dust_queue.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut k = crate::knobs::ChaosK::default();
        k.papers_per_sec = 1.0;
        k.puffs_per_sec = 5.0;
        let mut c = rate_cap_new(RateCapN::Paper);
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
