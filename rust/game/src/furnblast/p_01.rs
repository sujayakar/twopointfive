//! src/game/furnblast.ts lines 1 to 682
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::Num;
use crate::scene_materials::{BreakClass, Does, DoesOfB, Surf};
use crate::wallbreak::{BKind, LevelChange, LevelChangeCause, LevelChangeKind, WallBreakNoteC, WallBreakNoteCCause, WallBreakRazeBudget, WallBreakRazeInBudget};

// ---------------------------------------------------------------- the one seam left (notes.md, ## Skeleton)
/// P.emit('debris', …) on gpuFx(g): gpu_fx hands back a copy, and EmitOpts lacks speedJit, spread, lifeJit, sizeJit, aspect, color, owner
#[allow(clippy::too_many_arguments)]
fn seam_emit_debris(g: &mut Game, pos: V3, dir: V3, count: f64, speed: f64, speed_jit: f64, spread: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, aspect: f64, color: V3, owner: f64) { todo!("needs: particles' emit options and the live Particles") }

// ---------------------------------------------------------------- small readers
/// the box at index `i` of the level's list, if there is one
fn box_at(g: &Game, i: f64) -> Option<BoxId> { if i < 0.0 || i as usize >= g.col.boxes.len() { None } else { Some(BoxId::from_ix(i as usize)) } }
fn bx(i: f64) -> BoxId { BoxId::from_ix(i as usize) }
fn is_piece(g: &Game, b: BoxId) -> bool { g.col.boxes[b].piece == Some(true) }
fn does_word(g: &Game, b: BoxId) -> Option<String> { g.col.boxes[b].does.map(|w| String::from(w.as_str())) }
fn box_does(g: &mut Game, b: BoxId) -> Does { let does = does_word(g, b); crate::scene_materials::does_of(g, Some(&DoesOfB { does })) }
fn snow_paint(g: &mut Game, b: BoxId) -> bool { let does = does_word(g, b); crate::render_snow_field::snow_paint_of(g, &crate::render_snow_field::SnowPaintOfB { does }).is_some() }
fn box_class(g: &mut Game, b: BoxId) -> BreakClass { let does = does_word(g, b); break_class_of(g, &BreakClassOfB { does }) }
fn flags_of(g: &Game, b: BoxId) -> i32 { js::to_i32(g.col.boxes[b].flags) }
/// BoxFlag.NoPrimary, BoxFlag.NoShadow, BoxFlag.Glass, BoxFlag.Dynamic as bits
fn bits() -> (i32, i32, i32, i32) { let f = crate::scene_boxes::box_flag(); (js::to_i32(f.no_primary), js::to_i32(f.no_shadow), js::to_i32(f.glass), js::to_i32(f.dynamic)) }
fn truly(x: Option<bool>) -> bool { x == Some(true) }
fn plan_a(a: &AABB) -> OverPlanA { OverPlanA { x0: a.x0, z0: a.z0, x1: a.x1, z1: a.z1 } }
fn plan_q(a: &AABB) -> OverPlanQ { OverPlanQ { x0: a.x0, z0: a.z0, x1: a.x1, z1: a.z1 } }
fn over(a: &AABB, q: &AABB) -> bool { over_plan(plan_a(a), plan_q(q)) }
/// makeBox({ c, h, yaw: b.yaw, albedo: copy of b's, ...wordOf(b) })
fn make_piece(g: &mut Game, b: BoxId, c: V3, h: V3) -> BoxId {
    let (yaw, albedo, stuff, does) = { let r = &g.col.boxes[b]; (r.yaw, r.albedo, r.stuff, r.does) };
    let w = crate::scene_boxes::word_of(crate::scene_boxes::WordOfB { stuff, does });
    crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(albedo), stuff: w.stuff, does: w.does, ..Default::default() })
}
/// walls.raze (wallbreak.ts:422), spending `budget`. True = it returned a row.
fn raze(g: &mut Game, id: f64, who: f64, dir: V3, budget: &mut f64) -> bool {
    let mut b = WallBreakRazeBudget { chunks: *budget };
    let row = crate::wallbreak::wall_break_raze(g, id, who, Some(dir), Some(&mut b));
    *budget = b.chunks; row.is_some()
}
/// walls.razeIn (wallbreak.ts:425), spending `budget`
fn raze_in(g: &mut Game, id: f64, r: [f64; 4], who: f64, dir: V3, budget: &mut f64) -> Option<LevelChange> {
    let mut b = WallBreakRazeInBudget { chunks: *budget };
    let row = crate::wallbreak::wall_break_raze_in(g, id, Some(r), who, Some(dir), Some(&mut b));
    *budget = b.chunks; row
}
/// FURN_BREAK[cls]: the table of how a part comes apart (furnblast.ts:104); nothing writes it
fn furn_break_row(cls: BreakClass) -> BreakRow {
    fn sfx(name: &str, rate: f64, gain: f64) -> BreakRowLandSfxItem { BreakRowLandSfxItem { name: String::from(name), rate, gain } }
    fn row(s: (f64, f64, f64), a: (f64, f64, f64), uneven: f64, fly: f64, spin: f64, level: f64, sfx: Vec<BreakRowLandSfxItem>) -> BreakRow {
        BreakRow { shatter: BreakRowShatter { len: s.0, max: s.1, strips: s.2 }, apart: BreakRowApart { len: a.0, max: a.1, strips: a.2 }, uneven, fly, spin, land: BreakRowLand { level, sfx } }
    }
    match cls {
        BreakClass::Wood => row((0.9, 2.0, 3.0), (1.2, 2.0, 2.0), 0.6, 1.0, 1.0, 0.25, vec![sfx("propThud", 1.15, 0.5)]),
        BreakClass::Board => row((0.45, 3.0, 2.0), (9.0, 1.0, 1.0), 0.4, 1.1, 1.2, 0.15, vec![sfx("propThud", 1.5, 0.3)]),
        BreakClass::Metal => row((0.7, 2.0, 1.0), (9.0, 1.0, 1.0), 0.5, 0.8, 0.7, 0.45, vec![sfx("propThud", 0.7, 0.9), sfx("magDrop", 0.5, 0.4)]),
        BreakClass::Hard => row((0.4, 3.0, 2.0), (0.8, 2.0, 1.0), 0.5, 1.0, 1.0, 0.2, vec![sfx("propThud", 1.3, 0.4)]),
        BreakClass::Soft => row((9.0, 1.0, 1.0), (9.0, 1.0, 1.0), 0.0, 1.2, 1.3, 0.1, vec![sfx("propThud", 0.9, 0.3)]),
    }
}
fn land_sfx(row: &BreakRow) -> Vec<crate::debris::ChunkSfxItem> { row.land.sfx.iter().map(|s| crate::debris::ChunkSfxItem { name: s.name.clone(), rate: s.rate, gain: s.gain }).collect() }

// ---------------------------------------------------------------- the file's free functions
/// what a breaching charge is to the furniture, by its breaking radius (furnblast.ts chargeSource)
pub fn charge_source(radius: f64) -> crate::furnblast::BlastSource { if radius >= 1.5 { BlastSource::Wall } else { BlastSource::Door } }
/// which row of the break table a box is: its word's (furnblast.ts breakClassOf)
pub fn break_class_of(g: &mut crate::game::Game, b: &crate::furnblast::BreakClassOfB) -> crate::scene_materials::BreakClass { crate::scene_materials::does_of(g, Some(&DoesOfB { does: b.does.clone() })).breaks }
/// a box's bounds, its yaw taken in (furnblast.ts aabbOf)
pub fn aabb_of(g: &mut crate::game::Game, b: BoxId) -> crate::furnblast::AABB {
    let (c, h, yaw) = { let r = &g.col.boxes[b]; (r.c, r.h, r.yaw) };
    let cs = m::abs(m::cos(yaw)); let sn = m::abs(m::sin(yaw)); let ex = cs * h[0] + sn * h[2]; let ez = sn * h[0] + cs * h[2];
    AABB { x0: c[0] - ex, x1: c[0] + ex, y0: c[1] - h[1], y1: c[1] + h[1], z0: c[2] - ez, z1: c[2] + ez }
}
/// plan distance from a point to a box (furnblast.ts distXZ)
pub fn dist_xz(p: V3, a: crate::furnblast::AABB) -> f64 { m::hyp2(m::max_n(&[a.x0 - p[0], 0.0, p[0] - a.x1]), m::max_n(&[a.z0 - p[2], 0.0, p[2] - a.z1])) }
/// a box's volume (furnblast.ts volOf)
pub fn vol_of(g: &mut crate::game::Game, b: BoxId) -> f64 { let h = g.col.boxes[b].h; h[0] * h[1] * h[2] * 8.0 }
/// two plan rectangles overlap by more than a centimetre (furnblast.ts overPlan)
pub fn over_plan(a: crate::furnblast::OverPlanA, q: crate::furnblast::OverPlanQ) -> bool { a.x0 < q.x1 - 0.01 && a.x1 > q.x0 + 0.01 && a.z0 < q.z1 - 0.01 && a.z1 > q.z0 + 0.01 }
pub mod none_done {
    /// (furnblast.ts NONE_DONE.has)
    pub fn has(_j: f64) -> bool { false }
}

// ---------------------------------------------------------------- class Furniture: g.destr.furn
/// (furnblast.ts Furniture.constructor)
pub fn furniture_new(g: &mut crate::game::Game) -> crate::furnblast::Furniture { Furniture::blank() }
/// its own deterministic stream (furnblast.ts Furniture.hash)
pub fn furniture_hash(g: &mut crate::game::Game) -> f64 {
    let c = g.destr.furn.counter; g.destr.furn.counter = c + 1.0;
    crate::debris::hash01(c * 2654435761.0)
}
fn find(parent: &mut [usize], mut k: usize) -> usize {
    while parent[k] != k { parent[k] = parent[parent[k]]; k = parent[k]; }
    k
}
/// the level's furniture, found once by shape: `groups` is filled and nothing handed back (furnblast.ts Furniture.index)
fn ensure_index(g: &mut Game) {
    if g.destr.furn.groups.is_some() { return; }
    let (r_min_top, r_max_top, r_max_long, r_max_vol) = (g.k.furn.root.min_top, g.k.furn.root.max_top, g.k.furn.root.max_long, g.k.furn.root.max_vol);
    let (rd_max_top, rd_max_vol) = (g.k.furn.rider.max_top, g.k.furn.rider.max_vol);
    let mut fixture: JsSet<Num> = JsSet::new();
    for ti in 0..g.targets.len() { let t = g.targets[ti]; for &bi in &g.tab.sim_lightshome_fixture[t].boxes { fixture.add(Num(bi)); } }
    for l in &g.level.lights { if let Some(fb) = l.fixture_box { if fb >= 0.0 { fixture.add(Num(fb)); } } }
    let mut kept = fixture.clone();
    for &bi in &g.level.breaker.boxes { kept.add(Num(bi)); }
    g.destr.furn.kept = kept;
    // (Level.mission is always there: every level is built with one, so its front is always a truthy Vec3)
    let mut fronts: Vec<V3> = vec![g.level.mission.rack.front];
    if let Some(q) = g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()) { for sp in &q.spots { fronts.push(sp.front); } }
    g.destr.furn.fronts = fronts;
    let w = crate::scene_world::world();
    let (np, ns, gl, dy) = bits();
    let mut cand: Vec<FurnblastMemberId> = Vec::new();
    let n = m::min(g.col.boxes.len() as f64, g.destr.walls.authored) as usize;
    let mut others: Vec<AABB> = Vec::new();   // every authored static that is NOT furniture: what may be resting on a piece of it
    for ix in 0..n {
        let i = ix as f64; let b = BoxId::from_ix(ix);
        if is_piece(g, b) { continue; }
        // by the flags the level was AUTHORED with, not as it stands: the index must not depend on when it was made
        let f0 = match g.destr.furn.hidden.get(Num(i)).copied() {
            Some(f) => f,
            None => match crate::destruction::destruction_flags0(g, i) { Some(f) => f, None => g.col.boxes[b].flags },
        };
        let f0 = js::to_i32(f0);
        if f0 & (gl | np | dy) != 0 { continue; }
        let a = aabb_of(g, b); let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
        if c[0] < w.min[0] || c[0] > w.max[0] || c[2] < w.min[2] || c[2] > w.max[2] || !furniture_roofed(g, c[0], c[2]) { others.push(a); continue; }
        let v = vol_of(g, b); let long = m::max(h[0], h[2]) * 2.0; let ghost = (f0 & ns) != 0;
        let br = crate::wallbreak::wall_break_breakable_at(g, i);
        if let Some(r) = &br { if r.kind == BKind::Wall { others.push(a); continue; } }
        let does = box_does(g, b); let e = g.col.boxes[b].emissive; let lit = e[0] + e[1] + e[2] > 0.0;
        let mut root = false;
        if !ghost && g.col.boxes[b].rot.is_none() && !lit && !fixture.has(Num(i)) && crate::destruction::destruction_item_of(g, i).is_none() && a.y0 <= 0.25 && a.y1 >= r_min_top && a.y1 <= r_max_top && long <= r_max_long && v <= r_max_vol {
            if br.is_some() { root = does.cover.is_some() && (m::min(h[0], h[2]) * 2.0 <= 0.12 || v <= 0.25); }   // (a desk's panel, a small carton: a crate of real cover stays the wall engine's)
            else { root = !truly(does.struct_); }
        }
        if !root && !(a.y0 > 0.06 && a.y1 <= rd_max_top && v <= rd_max_vol && long <= r_max_long && br.is_none()) { if !ghost { others.push(a); } continue; }
        cand.push(g.tab.furnblast_member.push(Member { i, a, root, ghost, nb: Vec::new(), sup: Vec::new(), group: None }));
    }
    // joined = touching (a sweep along x keeps the pair test small)
    { let tab = &g.tab.furnblast_member; js::sort_by(&mut cand, |p, q| js::or(tab[*p].a.x0 - tab[*q].a.x0, tab[*p].i - tab[*q].i)); }
    let e = g.k.furn.touch;
    let mut parent: Vec<usize> = (0..cand.len()).collect();
    for p in 0..cand.len() {
        let aa = g.tab.furnblast_member[cand[p]].a;
        let mut q = p + 1;
        while q < cand.len() && g.tab.furnblast_member[cand[q]].a.x0 <= aa.x1 + e {
            let bb = g.tab.furnblast_member[cand[q]].a;
            let (mp, mq) = (cand[p], cand[q]);
            if !(bb.z0 > aa.z1 + e || aa.z0 > bb.z1 + e || bb.y0 > aa.y1 + e || aa.y0 > bb.y1 + e) {
                g.tab.furnblast_member[mp].nb.push(mq); g.tab.furnblast_member[mq].nb.push(mp);
                if aa.y0 < bb.y0 - 0.005 { g.tab.furnblast_member[mq].sup.push(mp); } else if bb.y0 < aa.y0 - 0.005 { g.tab.furnblast_member[mp].sup.push(mq); }
                let (rp, rq) = (find(&mut parent, p), find(&mut parent, q));
                if rp != rq { parent[rp.max(rq)] = rp.min(rq); }
            }
            q += 1;
        }
    }
    let mut by_root: JsMap<usize, Vec<FurnblastMemberId>> = JsMap::new();
    for k in 0..cand.len() { let r = find(&mut parent, k); by_root.get_or_insert_with(r, Vec::new).push(cand[k]); }
    let mut out: Vec<Group> = Vec::new();
    let lists: Vec<Vec<FurnblastMemberId>> = by_root.values().cloned().collect();
    for mut l in lists {
        if !l.iter().any(|&m| g.tab.furnblast_member[m].root) { continue; }   // (nothing of it stands on the floor: a shelf on a wall, a sign)
        { let tab = &g.tab.furnblast_member; js::sort_by(&mut l, |p, q| tab[*p].i - tab[*q].i); }
        let mut a = AABB { x0: 1e9, x1: -1e9, y0: 1e9, y1: -1e9, z0: 1e9, z1: -1e9 }; let mut heavy = 0.0; let mut big: Option<FurnblastMemberId> = None;
        for &mi in &l {
            let (ma, root, i) = { let r = &g.tab.furnblast_member[mi]; (r.a, r.root, r.i) };
            a.x0 = m::min(a.x0, ma.x0); a.x1 = m::max(a.x1, ma.x1); a.y0 = m::min(a.y0, ma.y0); a.y1 = m::max(a.y1, ma.y1); a.z0 = m::min(a.z0, ma.z0); a.z1 = m::max(a.z1, ma.z1);
            if root { let b = bx(i); let v = vol_of(g, b); if v > heavy { heavy = v; big = Some(mi); } }
        }
        // THE LOAD RULE: something that is not furniture stands ON it: the whole thing is left to stand
        let loaded = { let tab = &g.tab.furnblast_member; others.iter().any(|o| o.x0 < a.x1 - 0.01 && o.x1 > a.x0 + 0.01 && o.z0 < a.z1 - 0.01 && o.z1 > a.z0 + 0.01 && l.iter().any(|&mi| { let ma = &tab[mi].a; m::abs(o.y0 - ma.y1) <= e && o.x0 < ma.x1 - 0.01 && o.x1 > ma.x0 + 0.01 && o.z0 < ma.z1 - 0.01 && o.z1 > ma.z0 + 0.01 })) };
        if loaded { continue; }
        let id0 = g.tab.furnblast_member[l[0]].i;
        let bi = g.tab.furnblast_member[big.unwrap_or(l[0])].i; let bb = bx(bi);
        let surf = box_does(g, bb).surf;
        let grp = Group { id: id0, members: l.clone(), a, heavy, surf };
        for &mi in &l {
            g.tab.furnblast_member[mi].group = Some(grp.clone());
            let i = g.tab.furnblast_member[mi].i;
            g.destr.furn.member_at.set(Num(i), mi);
        }
        out.push(grp);
    }
    js::sort_by(&mut out, |p, q| p.id - q.id);
    g.destr.furn.groups = Some(out);
}
/// the level's furniture, found once by shape on the first blast (furnblast.ts Furniture.index)
pub fn furniture_index(g: &mut crate::game::Game) -> Vec<crate::furnblast::Group> {
    ensure_index(g);
    g.destr.furn.groups.as_ref().unwrap().clone()
}
/// the index made ahead of the blast, off the frame the wall goes on (furnblast.ts Furniture.warm)
pub fn furniture_warm(g: &mut crate::game::Game) { if g.m.destruction_switch.blast_pace.on && g.k.destruction.on { ensure_index(g); } }
/// counts for the panel and probes (furnblast.ts Furniture.inventory)
pub fn furniture_inventory(g: &mut crate::game::Game) -> crate::furnblast::FurnitureInventory {
    ensure_index(g);
    let groups = g.destr.furn.groups.as_ref().unwrap();
    let tab = &g.tab.furnblast_member;
    FurnitureInventory {
        groups: groups.len() as f64,
        members: groups.iter().map(|x| x.members.len()).sum::<usize>() as f64,
        hidden: g.destr.furn.hidden.len() as f64,
        gone_props: g.destr.furn.gone_props,
        list: groups.iter().map(|x| FurnitureInventoryListItem { id: x.id, members: x.members.iter().map(|&mi| tab[mi].i).collect(), a: x.a, heavy: x.heavy, surf: x.surf }).collect(),
    }
}
/// does this part still stand; a cover panel, while any cell of it does (furnblast.ts Furniture.stands)
pub fn furniture_stands(g: &mut crate::game::Game, m: FurnblastMemberId) -> bool {
    let i = g.tab.furnblast_member[m].i;
    match crate::wallbreak::wall_break_breakable_at(g, i) {
        Some(br) => br.open < br.cells,
        None => { let b = bx(i); flags_of(g, b) & bits().0 == 0 }
    }
}
fn any_stands(g: &mut Game, members: &[FurnblastMemberId]) -> bool { for &mi in members { if furniture_stands(g, mi) { return true; } } false }
/// what a blast would take first, asked before it goes off and reading only (furnblast.ts Furniture.firstToGo)
pub fn furniture_first_to_go(g: &mut crate::game::Game, at: V3, src: crate::furnblast::BlastSource) -> Option<crate::furnblast::BlastFirst> {
    if !g.k.destruction.on || g.destr.show_chaos.is_some() { return None; }
    let k = g.k.furn.k.get(src).copied().unwrap_or(1.0); if !(k > 0.0) { return None; }
    let apart_m = g.k.furn.apart_m * k; let shatter_m = g.k.furn.shatter_m * k;
    let apart_max_vol = g.k.furn.root.apart_max_vol;
    ensure_index(g);
    let mut best: Option<f64> = None; let mut bd = f64::INFINITY;
    let n = g.destr.furn.groups.as_ref().unwrap().len();
    for gi in 0..n {
        let (a, heavy, id) = { let x = &g.destr.furn.groups.as_ref().unwrap()[gi]; (x.a, x.heavy, x.id) };
        let d = dist_xz(at, a); if d > apart_m || (d > shatter_m && heavy > apart_max_vol) { continue; }
        if d < bd {   // (the index is in id order: of two as near, the lower id, as fixed()'s sort has it)
            let members = g.destr.furn.groups.as_ref().unwrap()[gi].members.clone();
            if any_stands(g, &members) { best = Some(id); bd = d; }
        }
    }
    best.map(|group| BlastFirst { group, at: v3::copy(at), shatter_m, serial: g.serial() })
}
/// is static `i` part of what that first piece takes off the floor (furnblast.ts Furniture.wouldTake)
pub fn furniture_would_take(g: &mut crate::game::Game, first: crate::furnblast::BlastFirst, i: f64, origins: &[V3]) -> bool {
    let mi = match g.destr.furn.member_at.get(Num(i)).copied() { Some(x) => x, None => return false };
    let grp = match g.tab.furnblast_member[mi].group.clone() { Some(x) => x, None => return false };
    if grp.id != first.group || !furniture_stands(g, mi) || crate::wallbreak::wall_break_breakable_at(g, i).is_some() { return false; }
    if m::max(grp.a.x1 - grp.a.x0, grp.a.z1 - grp.a.z0) > g.k.furn.row_m { return furniture_row_take(g, first, &grp, origins).has(mi); }
    dist_xz(first.at, grp.a) <= first.shatter_m && furniture_exposed_box(g, origins, grp.a)
}
/// of a long run, the parts wouldTake can promise; one answer kept for the frame (furnblast.ts Furniture.rowTake)
pub fn furniture_row_take(g: &mut crate::game::Game, first: crate::furnblast::BlastFirst, g_2: &crate::furnblast::Group, origins: &[V3]) -> JsSet<FurnblastMemberId> {
    // (line 259: `first` by its serial, and G follows from it; `origins` by value, as the row holds them)
    if let Some(kp) = &g.destr.furn.row_kept {
        if kp.t == g.time && kp.first == first.serial && kp.origins.as_slice() == origins { return kp.gone.clone(); }
    }
    let mut standing: Vec<FurnblastMemberId> = Vec::new();
    for &mi in &g_2.members {
        if furniture_stands(g, mi) { let i = g.tab.furnblast_member[mi].i; if crate::wallbreak::wall_break_breakable_at(g, i).is_none() { standing.push(mi); } }
    }
    let gone = furniture_row_fall(g, &standing, first.at, first.shatter_m, origins);
    g.destr.furn.row_kept = Some(FurnitureRowKept { t: g.time, first: first.serial, origins: origins.to_vec(), gone: gone.clone() });
    gone
}
/// a long run under a blast: what is in reach with a line, then what those held up (furnblast.ts Furniture.rowFall)
pub fn furniture_row_fall(g: &mut crate::game::Game, standing: &[FurnblastMemberId], at: V3, reach: f64, origins: &[V3]) -> JsSet<FurnblastMemberId> {
    let mut gone: JsSet<FurnblastMemberId> = JsSet::new();
    for &mi in standing {
        let (root, a) = { let r = &g.tab.furnblast_member[mi]; (r.root, r.a) };
        if root && dist_xz(at, a) <= reach && furniture_exposed_box(g, origins, a) { gone.add(mi); }
    }
    let mut more = gone.len() > 0; let mut pass = 0;
    while more && pass < 8 {
        more = false;
        for &mi in standing {
            if gone.has(mi) || g.tab.furnblast_member[mi].root { continue; }
            let (sup, nb) = { let r = &g.tab.furnblast_member[mi]; (r.sup.clone(), r.nb.clone()) };
            let held = if !sup.is_empty() {
                let mut h = false; for &s in &sup { if !gone.has(s) && furniture_stands(g, s) { h = true; break; } } h
            } else { !nb.iter().any(|&s| gone.has(s)) };
            if !held { gone.add(mi); more = true; }
        }
        pass += 1;
    }
    gone
}
/// is static `i` one this module has broken (furnblast.ts Furniture.broken)
pub fn furniture_broken(g: &mut crate::game::Game, i: f64) -> bool { g.destr.furn.hidden.has(Num(i)) }
/// a clear line of blast: furniture is no shield, anything else is (furnblast.ts Furniture.clear)
pub fn furniture_clear(g: &mut crate::game::Game, o: V3, p: V3) -> bool {
    let d = v3::sub(p, o); let l = v3::len(d); if l < 0.05 { return true; } let rd = v3::scale(d, 1.0 / l);
    let mut t0 = 0.0;
    for _step in 0..12 {
        let ro: V3 = [o[0] + rd[0] * t0, o[1] + rd[1] * t0, o[2] + rd[2] * t0];
        let hit = match crate::scene_collision::raycast(g, ro, rd, l - t0) { Some(h) => h, None => return true };
        if hit.t >= l - t0 - 0.03 { return true; }
        if hit.index >= 1e6 { return false; }   // a door leaf
        let id = if g.destr.furn.member_at.has(Num(hit.index)) { hit.index } else { crate::wallbreak::wall_break_breakable_at(g, hit.index).map(|b| b.id).unwrap_or(-1.0) };
        if !g.destr.furn.member_at.has(Num(id)) { return false; }
        t0 += hit.t + 0.005;   // (just inside it: a thin part flush to a wall must not carry the ray into the wall)
    }
    false
}
/// a line of blast from any origin to a box: its middle, over its top, or its nearest point (furnblast.ts Furniture.exposedBox)
pub fn furniture_exposed_box(g: &mut crate::game::Game, origins: &[V3], a: crate::furnblast::AABB) -> bool {
    let cx = (a.x0 + a.x1) / 2.0; let cz = (a.z0 + a.z1) / 2.0;
    for &o in origins {
        let near: V3 = [m::max(a.x0, m::min(o[0], a.x1)), m::max(a.y0 + 0.05, m::min(o[1], a.y1 - 0.02)), m::max(a.z0, m::min(o[2], a.z1))];
        if furniture_clear(g, o, [cx, (a.y0 + a.y1) / 2.0, cz]) || furniture_clear(g, o, [cx, a.y1 + 0.05, cz]) || furniture_clear(g, o, near) { return true; }
    }
    false
}

// ---------------------------------------------------------------- the blast
/// a charge went off at `at`: throws and breaks what it reaches (furnblast.ts Furniture.blast)
pub fn furniture_blast(g: &mut crate::game::Game, at: V3, src: crate::furnblast::BlastSource, who: f64, chaos: f64) {
    if !(chaos > 0.0) || g.destr.show_chaos.is_some() { return; }
    let k = g.k.furn.k.get(src).copied().unwrap_or(1.0); if !(k > 0.0) { return; }
    if let Some(l) = g.destr.furn.last { if l.t == g.time && m::hyp2(at[0] - l.at[0], at[2] - l.at[2]) <= g.k.furn.merge_m && k <= l.k { g.stats.furnblast.merged += 1; return; } }
    g.destr.furn.last = Some(FurnitureLast { t: g.time, at: v3::copy(at), k }); g.stats.furnblast.blasts += 1;
    g.destr.furn.fling = g.k.furn.fling.get(src).copied().unwrap_or(1.0);
    let who = js::or(who, crate::consts::NOBODY_ID);
    // where it blows FROM: the charge, and the far mouth of every hole this blast has just opened
    let mut mouths: Vec<[f64; 4]> = Vec::new();
    let ceiling = crate::scene_world::world().ceiling_y;
    let mut origins: Vec<V3> = vec![[at[0], m::max(0.3, m::min(at[1] + 0.3, ceiling - 0.3)), at[2]]];
    let mouth_m = g.k.furn.mouth_m; let depth = g.k.furn.mouth.depth;
    let mut ci = 0;
    while ci < g.destr.walls.changes.len() {
        let (t, kind, cause, pos, rect, id) = { let c = &g.destr.walls.changes[ci]; (c.t, c.kind, c.cause, c.pos, c.rect, c.id) };
        ci += 1;
        if t != g.time || kind != LevelChangeKind::Hole || cause != LevelChangeCause::Blast || m::hyp2(pos[0] - at[0], pos[2] - at[2]) > 3.0 { continue; }
        let ax: usize = if rect[2] - rect[0] < rect[3] - rect[1] { 0 } else { 2 }; let s = if pos[ax] - at[ax] >= 0.0 { 1.0 } else { -1.0 };
        let mut mo: V3 = [pos[0], 1.0, pos[2]];
        mo[ax] = (if s > 0.0 { m::max(rect[if ax == 0 { 2 } else { 3 }], pos[ax]) } else { m::min(rect[if ax == 0 { 0 } else { 1 }], pos[ax]) }) + s * mouth_m;
        origins.push(mo);
        // both mouths of the hole: the stretch open from the floor up, `depth` out from either face
        if let Some(mut r) = crate::wallbreak::wall_break_sill_open_in(g, id, rect) { let lo = if ax == 0 { 0 } else { 1 }; r[lo] -= depth; r[lo + 2] += depth; mouths.push(r); }
    }
    let mut budget = g.k.furn.chunks_per_blast;   // (a FIXED count: what breaks never depends on the quality tier's pool)
    fixed_in(g, at, k, who, &origins, &mut budget);
    pushable_in(g, at, k, who, &origins, &mut budget);
    let mut mb = g.k.furn.mouth.chunks;
    for r in mouths { mouth_in(g, at, who, r, &mut mb); }
    let s = g.stats.furnblast.clone();
    let line = format!("{:.2} {} at {:.1},{:.1} origins {} · groups {} statics {} props {}/{} chunks {} (totals)", g.time, src.as_str(), at[0], at[2], origins.len(), s.groups, s.statics, s.props_thrown, s.props_broken, s.chunks);
    g.m.furnblast.furn_log.push(line);
    if g.m.furnblast.furn_log.len() > 200 { g.m.furnblast.furn_log.drain(0..100); }
}
/// (furnblast.ts Furniture.fixed) the budget's one number goes down the private functions: see fixed_in
pub fn furniture_fixed(g: &mut crate::game::Game, at: V3, k: f64, who: f64, origins: &[V3], budget: &mut crate::furnblast::FurnitureFixedBudget) { fixed_in(g, at, k, who, origins, &mut budget.chunks); }
/// the level's fixed furniture: judged whole against the level as it stands, then broken nearest first (furnblast.ts Furniture.fixed)
fn fixed_in(g: &mut Game, at: V3, k: f64, who: f64, origins: &[V3], budget: &mut f64) {
    let apart_m = g.k.furn.apart_m * k; let shatter_m = g.k.furn.shatter_m * k;
    let (apart_max_vol, row_m) = (g.k.furn.root.apart_max_vol, g.k.furn.row_m);
    ensure_index(g);
    let mut plan: Vec<(Group, f64, Vec<FurnblastMemberId>)> = Vec::new();
    let n = g.destr.furn.groups.as_ref().unwrap().len();
    for gi in 0..n {
        let (a, heavy) = { let x = &g.destr.furn.groups.as_ref().unwrap()[gi]; (x.a, x.heavy) };
        let d = dist_xz(at, a); if d > apart_m { continue; }
        if d > shatter_m && heavy > apart_max_vol { continue; }
        let row = m::max(a.x1 - a.x0, a.z1 - a.z0) > row_m;
        let grp = g.destr.furn.groups.as_ref().unwrap()[gi].clone();
        let mut standing: Vec<FurnblastMemberId> = Vec::new();
        for &mi in &grp.members { if furniture_stands(g, mi) { standing.push(mi); } }
        if standing.is_empty() { continue; }
        let fall: Vec<FurnblastMemberId>;
        if !row {
            if !furniture_exposed_box(g, origins, a) { g.stats.furnblast.shielded += 1; continue; }
            fall = standing;
        } else {
            // a long run: the parts in reach with a line to them, then whatever those were holding up
            let gone = furniture_row_fall(g, &standing, at, apart_m, origins);
            if gone.len() == 0 { g.stats.furnblast.shielded += 1; continue; }
            fall = standing.iter().copied().filter(|&mi| gone.has(mi)).collect();
        }
        plan.push((grp, d, fall));
    }
    js::sort_by(&mut plan, |p, q| js::or(p.1 - q.1, p.0.id - q.0.id));
    for (grp, _d, mut fall) in plan {
        if *budget <= 0.0 { g.stats.furnblast.left += 1; continue; }   // (out of pieces: it stands, never a thing that just vanishes)
        g.stats.furnblast.groups += 1;
        let (mut rx0, mut rx1, mut rz0, mut rz1) = (1e9, -1e9, 1e9, -1e9); let mut solid = 0;
        { let tab = &g.tab.furnblast_member; js::sort_by(&mut fall, |p, q| js::or(tab[*q].a.y0 - tab[*p].a.y0, tab[*p].i - tab[*q].i)); }   // what stands on it first, top down
        for &mi in &fall {
            if !furniture_stands(g, mi) { continue; }   // (went with something broken a moment ago)
            let (ma, i, ghost) = { let r = &g.tab.furnblast_member[mi]; (r.a, r.i, r.ghost) };
            let dm = dist_xz(at, ma); let shatter = dm <= shatter_m; let kk = m::max(0.0, 1.0 - dm / apart_m);
            if break_static_in(g, i, ghost, at, shatter, kk, who, budget) { solid += 1; rx0 = m::min(rx0, ma.x0); rx1 = m::max(rx1, ma.x1); rz0 = m::min(rz0, ma.z0); rz1 = m::max(rz1, ma.z1); }
        }
        // what a checkpoint files of it: every part of it that no longer stands
        for &mi in &grp.members {
            let i = g.tab.furnblast_member[mi].i;
            if !furniture_stands(g, mi) && crate::wallbreak::wall_break_breakable_at(g, i).is_none() { g.destr.furn.gone.add(Num(i)); }
        }
        let c: V3 = [(grp.a.x0 + grp.a.x1) / 2.0, (grp.a.y0 + grp.a.y1) / 2.0, (grp.a.z0 + grp.a.z1) / 2.0];
        crate::destruction::destruction_cell_dust(g, c, [0.0, 1.0, 0.0], 1.2, grp.surf);
        if solid == 0 { continue; }
        // one model: sound's routes, the smoke's obstacles, the nav cells, and the AI's feed
        crate::scene_collision::sound_paths_invalidate(g);
        crate::smoke_types::smoke_system_like_obstacles_changed(g, c, m::hyp2(rx1 - rx0, rz1 - rz0) / 2.0 + 1.5);
        let ch = crate::scene_collision::nav_grid_rebake(g, rx0, rz0, rx1, rz1);
        let mut opened: Vec<f64> = Vec::new(); let mut x = 0; while x < ch.len() { if ch[x + 1] == 0.0 { opened.push(ch[x]); } x += 2; }
        g.stats.furnblast.nav_opened += opened.len() as i32; furniture_bump(g);
        let dead = !any_stands(g, &grp.members);
        let walkable = !opened.is_empty();
        crate::wallbreak::wall_break_note(g, &WallBreakNoteC { kind: LevelChangeKind::Cover, cause: WallBreakNoteCCause::Blast, name: None, id: grp.id, pos: c, rect: [rx0, rz0, rx1, rz1], opened, walkable, sight: true, dead, who });
    }
}
/// (furnblast.ts Furniture.breakStatic) see break_static_in
pub fn furniture_break_static(g: &mut crate::game::Game, m: FurnblastMemberId, at: V3, mode: crate::furnblast::FurnitureBreakStaticMode, kk: f64, who: f64, budget: &mut crate::furnblast::FurnitureBreakStaticBudget) -> bool {
    let (i, ghost) = { let r = &g.tab.furnblast_member[m]; (r.i, r.ghost) };
    break_static_in(g, i, ghost, at, mode == FurnitureBreakStaticMode::Shatter, kk, who, &mut budget.chunks)
}
/// one static of a piece goes; true = this module hid a SOLID box itself (furnblast.ts Furniture.breakStatic). A member is read for `i` and `ghost` only, so mouth()'s stand-in member needs no row.
fn break_static_in(g: &mut Game, i: f64, ghost: bool, at: V3, shatter: bool, kk: f64, who: f64, budget: &mut f64) -> bool {
    let b = bx(i); let c = g.col.boxes[b].c; let dir = furniture_away(g, at, c);
    if crate::destruction::destruction_item_of(g, i).is_some() {   // the lane's own destructible: its road, plus the blast's shove
        let sp = (1.0 + 3.0 * kk) * g.destr.furn.fling; g.stats.furnblast.items += 1;
        if g.k.debris.dress {   // (the shove is said BEFORE the thing is knocked off; the budget is spent by what was KNOCKED OFF)
            let knocks = g.destr.knocks; g.destr.debris.fling = Some([dir[0] * sp, 0.6 + 1.2 * kk, dir[2] * sp]);
            crate::destruction::destruction_drop_item(g, i, dir, Some(who), None);
            g.destr.debris.fling = None;
            *budget -= g.destr.knocks - knocks; return false;
        }
        let had = g.destr.debris.list.clone(); crate::destruction::destruction_drop_item(g, i, dir, Some(who), None);
        let now = g.destr.debris.list.clone();
        for ci in now { if !had.contains(&ci) {
            let r = &mut g.tab.debris_chunk[ci]; r.v[0] += dir[0] * sp; r.v[1] += 0.6 + 1.2 * kk; r.v[2] += dir[2] * sp; *budget -= 1.0;
        } }
        return false;
    }
    if let Some(br) = crate::wallbreak::wall_break_breakable_at(g, i) {   // a cover panel: the wall engine's road
        if raze(g, br.id, who, dir, budget) { g.stats.furnblast.razed += 1; }
        return false;
    }
    let f = g.col.boxes[b].flags; let (np, ns, _, _) = bits();
    g.destr.furn.hidden.set(Num(i), f); g.col.boxes[b].flags = (js::to_i32(f) | (np | ns)) as f64; crate::scene_collision::wrote(g); g.stats.furnblast.statics += 1;
    crate::destruction::destruction_break_fixture_box(g, i);
    if ghost { return false; }   // (a drawn-only sheet or placard: it just goes)
    if snow_paint(g, b) {   // lying snow: a spray from where the blast met it, never rigid pieces
        let a = aabb_of(g, b); let near: V3 = [m::min(a.x1, m::max(a.x0, at[0])), c[1], m::min(a.z1, m::max(a.z0, at[2]))];
        furniture_chips(g, near, [dir[0], 0.9, dir[2]], b, who, 36.0); return true;
    }
    let cls = box_class(g, b); let row = furn_break_row(cls);
    let steel = if cls == BreakClass::Metal && crate::scene_cut::cut_boxes_on(g) {
        let md = if shatter && !(*budget < STEEL_SHEETS_ROOM) { FurnitureSteelMode::Shatter } else { FurnitureSteelMode::Apart };
        furniture_steel(g, b, md, dir, i)
    } else { None };
    if let Some(parts) = steel { for p in parts { throw_piece_in(g, p.box_, at, shatter, kk, &row, who, budget, b, Some(p)); } }
    else {
        let frag_mode = if shatter { FurnitureFragmentsMode::Shatter } else { FurnitureFragmentsMode::Apart };
        for f in furniture_fragments(g, b, &row, frag_mode) { throw_piece_in(g, f, at, shatter, kk, &row, who, budget, b, None); }
    }
    furniture_chips(g, c, [dir[0], 0.6, dir[2]], b, who, if shatter { 12.0 } else { 5.0 });
    true
}
/// (furnblast.ts Furniture.throwPiece) see throw_piece_in
pub fn furniture_throw_piece(g: &mut crate::game::Game, f: BoxId, at: V3, mode: crate::furnblast::FurnitureThrowPieceMode, kk: f64, row: &crate::furnblast::BreakRow, who: f64, budget: &mut crate::furnblast::FurnitureThrowPieceBudget, of: BoxId, steel: Option<crate::furnblast::SteelPart>) {
    throw_piece_in(g, f, at, mode == FurnitureThrowPieceMode::Shatter, kk, row, who, &mut budget.chunks, of, steel);
}
/// one piece on its way: a chunk while budget and pool allow, else a burst of chips (furnblast.ts Furniture.throwPiece)
fn throw_piece_in(g: &mut Game, f: BoxId, at: V3, shatter: bool, kk: f64, row: &BreakRow, who: f64, budget: &mut f64, of: BoxId, steel: Option<SteelPart>) {
    let fc = g.col.boxes[f].c; let fd = furniture_away(g, at, fc);
    let h1 = furniture_hash(g); let h2 = furniture_hash(g); let h3 = furniture_hash(g);
    let fling = g.destr.furn.fling;
    let v: V3; let w: V3;
    if shatter {
        let sp = (2.4 + 3.2 * kk) * (0.7 + 0.6 * h1) * row.fly * fling; let sn = row.spin;
        v = [fd[0] * sp + (h2 - 0.5) * 1.2, (1.0 + 2.6 * kk * h3) * (0.5 + 0.5 * fling), fd[2] * sp + (h3 - 0.5) * 1.2];
        w = [(h1 - 0.5) * 14.0 * sn, (h2 - 0.5) * 8.0 * sn, (h3 - 0.5) * 14.0 * sn];
    } else {   // (goes over AWAY from the blast: the turn is about the floor axis across its line)
        let sp = (0.7 + 1.5 * kk) * (0.7 + 0.6 * h1) * row.fly * (0.5 + 0.5 * fling); let tip = (1.6 + 2.6 * kk) * row.spin;
        v = [fd[0] * sp, 0.3 + 0.8 * h2, fd[2] * sp];
        w = [fd[2] * tip + (h3 - 0.5), (h1 - 0.5) * 1.5, -fd[0] * tip + (h2 - 0.5)];
        // a TALL piece starts already past its balance point: one that starts upright on the floor would be stood straight back up
        let (yaw, fh) = (g.col.boxes[f].yaw, g.col.boxes[f].h);
        let cs = m::cos(yaw); let sn = m::sin(yaw); let along = fh[0] * m::abs(fd[0] * cs - fd[2] * sn) + fh[2] * m::abs(fd[0] * sn + fd[2] * cs);
        if fh[1] >= 0.3 && fh[1] > 1.2 * along {
            let th = 0.95; let s2 = m::sin(th / 2.0);
            g.col.boxes[f].rot = Some(v3::quat::mul([fd[2] * s2, 0.0, -fd[0] * s2, m::cos(th / 2.0)], v3::quat::yaw(yaw)));
            let mut c = fc;
            c[1] = m::max(c[1] - fh[1] + along * m::sin(th) + fh[1] * m::cos(th) + 0.01, 0.02); c[0] += fd[0] * fh[1] * 0.5; c[2] += fd[2] * fh[1] * 0.5;
            g.col.boxes[f].c = c;
        }
    }
    let got = if *budget > 0.0 { crate::debris::debris_spawn(g, crate::debris::ChunkKind::Chunk, f, v, w, Some(who), Some(row.land.level)) } else { None };
    *budget -= 1.0;
    if let Some(c) = got {
        if steel.is_none() { g.tab.debris_chunk[c].level = row.land.level; g.tab.debris_chunk[c].sfx = land_sfx(row); }
        g.stats.furnblast.chunks += 1;
        match steel {
            Some(SteelPart { open: Some(o), seed, .. }) => crate::break_table::breaks_carcass(g, c, o.axis, o.sign, seed),
            Some(_) => crate::break_table::breaks_sheet(g, c),
            None => crate::break_table::breaks_dress(g, c),
        }
        if steel.is_some() { g.tab.debris_chunk[c].level = row.land.level; g.tab.debris_chunk[c].sfx = land_sfx(row); }   // (after the table took it for steel: one landing, whatever its paint read as)
    } else {
        g.stats.furnblast.demoted += 1;
        let fc = g.col.boxes[f].c;
        furniture_chips(g, fc, v, of, who, 14.0);
    }
}
/// what hung on the wall under a charge goes: cut into slabs, driven through the hole or torn off into the room (furnblast.ts Furniture.hungGoes)
pub fn furniture_hung_goes(g: &mut crate::game::Game, b: BoxId, at: V3, n: V3, who: f64) {
    let k = g.k.furn.hung.clone();
    let base = furn_break_row(if box_class(g, b) == BreakClass::Wood { BreakClass::Wood } else { BreakClass::Board });
    let mut row = base.clone(); row.shatter = BreakRowShatter { len: k.len, max: k.max, strips: k.strips };
    let t: V3 = [-n[2], 0.0, n[0]]; let mut left = k.chunks;
    for f in furniture_fragments(g, b, &row, FurnitureFragmentsMode::Shatter) {
        let fc = g.col.boxes[f].c;
        let s = (fc[0] - at[0]) * t[0] + (fc[2] - at[2]) * t[2];
        let h1 = furniture_hash(g); let h2 = furniture_hash(g); let h3 = furniture_hash(g);
        let (yaw, fh) = (g.col.boxes[f].yaw, g.col.boxes[f].h);
        let cs = m::cos(yaw); let sn = m::sin(yaw); let half = fh[0] * m::abs(t[0] * cs - t[2] * sn) + fh[2] * m::abs(t[0] * sn + t[2] * cs);
        let v: V3;
        if m::abs(s) <= m::max(k.under_m, m::min(half, k.hole_m)) {   // (under the charge: within underM of it, or the slab it lies on while that fits the hole)
            let sp = v3::lerp(k.through[0], k.through[1], h1); let j = (h2 - 0.5) * 1.2;
            v = [-n[0] * sp + t[0] * j, 0.3 + 0.9 * h3, -n[2] * sp + t[2] * j];
        } else {
            let out = v3::lerp(k.back[0], k.back[1], h1); let along = m::sign(s) * k.side * (0.35 + 0.65 * h2);
            v = [n[0] * out + t[0] * along, v3::lerp(k.up[0], k.up[1], h3), n[2] * out + t[2] * along];
        }
        let w: V3 = [(h1 - 0.5) * k.spin * base.spin, (h2 - 0.5) * 0.6 * k.spin * base.spin, (h3 - 0.5) * k.spin * base.spin];
        let l = left; left -= 1.0;
        let got = if l > 0.0 { crate::debris::debris_spawn(g, crate::debris::ChunkKind::Chunk, f, v, w, Some(who), Some(base.land.level)) } else { None };
        if let Some(c) = got {
            g.stats.furnblast.chunks += 1; g.tab.debris_chunk[c].level = base.land.level; g.tab.debris_chunk[c].sfx = land_sfx(&base);
            crate::break_table::breaks_dress(g, c);
        } else { g.stats.furnblast.demoted += 1; furniture_chips(g, fc, v, b, who, 14.0); }
    }
    if crate::particle_fx::gpu_fx(g).is_none() { return; }
    let (c, albedo) = (g.col.boxes[b].c, g.col.boxes[b].albedo);
    let bursts: [(V3, usize); 2] = [([n[0], 0.5, n[2]], 0), ([-n[0], 0.3, -n[2]], 1)];   // (splinters back into the room, and through)
    for (d, i) in bursts { seam_emit_debris(g, c, v3::normalize(d), k.chips[i], k.chip_v[i], 0.7, 1.1, 18.0, 0.3, 0.006, 0.6, 3.0, v3::copy(albedo), who); }
}
/// `on` of Furniture.steel: the sheet of a carcass on face `a`, side `s`
fn steel_on(g: &mut Game, b: BoxId, a: usize, s: f64, shy: f64, t: f64) -> SteelPart {
    let bh = g.col.boxes[b].h;
    let mut l: V3 = [0.0, 0.0, 0.0]; let mut h: V3 = [bh[0] - shy, bh[1] - shy, bh[2] - shy];
    l[a] = s * (bh[a] - t); h[a] = t;
    SteelPart { box_: steel_part(g, b, l, h), open: None, seed: None }
}
/// `part` of Furniture.steel: a box of the carcass at local place `l`
fn steel_part(g: &mut Game, b: BoxId, l: V3, h: V3) -> BoxId {
    let (c, yaw) = (g.col.boxes[b].c, g.col.boxes[b].yaw); let cs = m::cos(yaw); let sn = m::sin(yaw);
    make_piece(g, b, [c[0] + cs * l[0] + sn * l[2], c[1] + l[1], c[2] - sn * l[0] + cs * l[2]], h)
}
/// a steel carcass comes apart as sheet on a frame; None = not a carcass (furnblast.ts Furniture.steel). Draws nothing from the stream.
pub fn furniture_steel(g: &mut crate::game::Game, b: BoxId, mode: crate::furnblast::FurnitureSteelMode, dir: V3, seed: f64) -> Option<Vec<crate::furnblast::SteelPart>> {
    let h = g.col.boxes[b].h;
    if g.col.boxes[b].rot.is_some() || m::min_n(&[h[0], h[1], h[2]]) * 2.0 < STEEL_CARCASS_M || m::max_n(&[h[0], h[1], h[2]]) * 2.0 > STEEL_CARCASS_MOST { return None; }
    if !truly(box_does(g, b).steel) { return None; }   // (and painted as one: the class 'metal' is also where every unnamed paint lands)
    let yaw = g.col.boxes[b].yaw; let cs = m::cos(yaw); let sn = m::sin(yaw); let t = g.k.breaks.steel.sheet;
    let deep: usize = if h[0] <= h[2] { 0 } else { 2 }; let wide: usize = if deep == 0 { 2 } else { 0 };   // the door is on a broad upright face
    let ax: V3 = if deep == 0 { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
    let sign = if ax[0] * dir[0] + ax[2] * dir[2] > 0.0 { -1.0 } else { 1.0 };   // (the broad face that LOOKS AT the blast is the sheet torn off)
    if mode == FurnitureSteelMode::Apart {
        let door = steel_on(g, b, deep, sign, 0.012, t);
        let mut dc = g.col.boxes[door.box_].c; dc[0] += ax[0] * sign * 0.03; dc[2] += ax[2] * sign * 0.03; g.col.boxes[door.box_].c = dc;
        let whole = steel_part(g, b, [0.0, 0.0, 0.0], [h[0], h[1], h[2]]);
        return Some(vec![SteelPart { box_: whole, open: Some(SteelPartOpen { axis: deep as f64, sign }), seed: Some(seed) }, door]);
    }
    let p0 = steel_on(g, b, deep, sign, 0.012, t); let p1 = steel_on(g, b, deep, -sign, 0.0, t);
    let p2 = steel_on(g, b, wide, 1.0, 2.0 * t, t); let p3 = steel_on(g, b, wide, -1.0, 2.0 * t, t); let p4 = steel_on(g, b, 1, 1.0, 2.0 * t, t);
    Some(vec![p0, p1, p2, p3, p4])
}
/// the cut places of `n` pieces (fragments' `cuts`)
fn cuts(g: &mut Game, n: usize, uneven: f64) -> Vec<f64> {
    let mut c = vec![-1.0];
    let nf = n as f64;
    for j in 1..n { let h = furniture_hash(g); c.push(-1.0 + 2.0 * (j as f64 + (h - 0.5) * uneven) / nf); }
    c.push(1.0);
    c
}
/// `b` cut into the boxes it flies as, by its material's row (furnblast.ts Furniture.fragments)
pub fn furniture_fragments(g: &mut crate::game::Game, b: BoxId, row: &crate::furnblast::BreakRow, mode: crate::furnblast::FurnitureFragmentsMode) -> Vec<BoxId> {
    let (bh, bc, yaw) = (g.col.boxes[b].h, g.col.boxes[b].c, g.col.boxes[b].yaw);
    let mut ax: [usize; 3] = [0, 1, 2];
    js::sort_by(&mut ax, |p, q| js::or(bh[*q] - bh[*p], *p as f64 - *q as f64));
    let l1 = bh[ax[0]] * 2.0; let l2 = bh[ax[1]] * 2.0;
    let (len, max, strips) = match mode { FurnitureFragmentsMode::Shatter => (row.shatter.len, row.shatter.max, row.shatter.strips), FurnitureFragmentsMode::Apart => (row.apart.len, row.apart.max, row.apart.strips) };
    let n1 = m::max(1.0, m::min(max, m::round(l1 / len))) as usize;
    let n2 = if l2 >= 0.3 { m::max(1.0, strips) as usize } else { 1 };
    let c1 = cuts(g, n1, row.uneven); let c2 = cuts(g, n2, row.uneven);
    let cs = m::cos(yaw); let sn = m::sin(yaw); let mut out: Vec<BoxId> = Vec::new();
    for p in 0..n1 { for q in 0..n2 {
        let mut l: V3 = [0.0, 0.0, 0.0]; let mut h: V3 = [bh[0], bh[1], bh[2]];
        l[ax[0]] = bh[ax[0]] * (c1[p] + c1[p + 1]) / 2.0; h[ax[0]] = bh[ax[0]] * (c1[p + 1] - c1[p]) / 2.0;
        l[ax[1]] = bh[ax[1]] * (c2[q] + c2[q + 1]) / 2.0; h[ax[1]] = bh[ax[1]] * (c2[q + 1] - c2[q]) / 2.0;
        out.push(make_piece(g, b, [bc[0] + cs * l[0] + sn * l[2], bc[1] + l[1], bc[2] - sn * l[0] + cs * l[2]], h));
    } }
    out
}
/// (furnblast.ts Furniture.pushable) see pushable_in
pub fn furniture_pushable(g: &mut crate::game::Game, at: V3, k: f64, who: f64, origins: &[V3], budget: &mut crate::furnblast::FurniturePushableBudget) { pushable_in(g, at, k, who, origins, &mut budget.chunks); }
/// the props: thrown and spun through their own solver; one beside the charge comes apart (furnblast.ts Furniture.pushable)
fn pushable_in(g: &mut Game, at: V3, k: f64, who: f64, origins: &[V3], budget: &mut f64) {
    let throw_m = g.k.furn.throw_m * k; let break_m = g.k.furn.prop_break_m * k;
    let list = g.props.props.clone();   // (the TypeScript walks a copy too: breakProp takes one off the list)
    for p in list {
        let (px, pz) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p));
        let (radius, height, mass) = { let r = &g.tab.props_prop[p]; (r.radius, r.height, r.mass) };
        let d = m::max(0.0, m::hyp2(px - at[0], pz - at[2]) - radius * 0.5); if d > throw_m { continue; }
        let mid: V3 = [px, m::min(0.6, height * 0.6), pz]; let top: V3 = [px, height + 0.05, pz];
        let mut seen = false;
        for &o in origins { if furniture_clear(g, o, mid) || furniture_clear(g, o, top) { seen = true; break; } }
        if !seen { g.stats.furnblast.props_shielded += 1; continue; }
        let dir = furniture_away(g, at, [px, 0.0, pz]); let fall = 1.0 - d / throw_m;
        if d <= break_m && *budget > 0.0 { break_prop_in(g, p, at, fall, who, budget); continue; }
        let dv = g.k.furn.throw_v * fall * m::max(0.6, m::min(1.4, m::sqrt(12.0 / mass)));
        let side = (furniture_hash(g) - 0.5) * 1.6 * radius;   // off-centre: it spins as it goes
        crate::props::prop_system_apply_impulse(g, p, [px - dir[2] * side, 0.0, pz + dir[0] * side], [dir[0] * dv * mass, 0.0, dir[2] * dv * mass], who, None);
        g.stats.furnblast.props_thrown += 1;
    }
}
/// (furnblast.ts Furniture.breakProp) see break_prop_in
pub fn furniture_break_prop(g: &mut crate::game::Game, p: PropId, at: V3, fall: f64, who: f64, budget: &mut crate::furnblast::FurnitureBreakPropBudget) { break_prop_in(g, p, at, fall, who, &mut budget.chunks); }
/// a prop comes apart: its parts thrown whole, what lay on it falls, it leaves the level (furnblast.ts Furniture.breakProp)
fn break_prop_in(g: &mut Game, p: PropId, at: V3, fall: f64, who: f64, budget: &mut f64) {
    let boxes = g.tab.props_prop[p].boxes.clone();
    for b in boxes {
        let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
        let piece = make_piece(g, b, v3::copy(c), v3::copy(h));
        let row = furn_break_row(box_class(g, b));
        throw_piece_in(g, piece, at, true, fall, &row, who, budget, b, None);
    }
    let chunks = g.destr.debris.list.clone();
    for c in chunks { let r = &mut g.tab.debris_chunk[c]; if r.on == Some(p) { r.on = None; r.rest = false; r.still_t = 0.0; } }
    let items = g.items.items.clone();
    for it in items { if crate::throwables::item_on(g, it) == Some(p) {   // (a can, a dropped magazine: its own rule only looks again when the prop MOVES)
        crate::throwables::item_set_on(g, it, None); crate::throwables::item_set_settled(g, it, false); crate::throwables::item_set_vel(g, it, [0.0, 0.0, 0.0]);
    } }
    crate::props::prop_system_remove(g, p); g.destr.furn.gone_props += 1.0; g.stats.furnblast.props_broken += 1;
    crate::scene_collision::sound_paths_invalidate(g);
}
/// under a roof? (furnblast.ts Furniture.roofed; index()'s own closure is the same test)
pub fn furniture_roofed(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    let i = match &g.reach_interior { Some(i) => i, None => return true };
    let w = crate::scene_world::world();
    let cx = m::floor(x - w.min[0]); let cz = m::floor(z - w.min[2]);
    cx >= 0.0 && cz >= 0.0 && cx < w.grid_w && cz < w.grid_h && i[(cz * w.grid_w + cx) as usize] == 1
}
/// may the mouth rule take static `i` at all, by what it was AUTHORED as (furnblast.ts Furniture.mouthOk). `saved` was a checkpoint's way past the snow switch; with the switch settled on it changes nothing.
pub fn furniture_mouth_ok(g: &mut crate::game::Game, i: f64, saved: Option<bool>) -> bool {
    let b = match box_at(g, i) { Some(b) => b, None => return false };
    if is_piece(g, b) || g.col.boxes[b].rot.is_some() || g.destr.furn.kept.has(Num(i)) || crate::wallbreak::wall_break_breakable_at(g, i).is_some() { return false; }   // (trim is no thing)
    let (np, _, gl, dy) = bits();
    let hidden = g.destr.furn.hidden.get(Num(i)).copied();
    let flags = g.col.boxes[b].flags;
    if js::to_i32(hidden.unwrap_or(flags)) & (gl | dy) != 0 { return false; }
    if hidden.is_none() && crate::destruction::destruction_flags0(g, i).is_none() && js::to_i32(flags) & np != 0 && crate::destruction::destruction_item_of(g, i).is_none() { return false; }
    let a = aabb_of(g, b);
    if !(a.y1 <= g.k.furn.mouth.top) { return false; }
    if !(vol_of(g, b) <= g.k.furn.mouth.vol || snow_paint(g, b)) { return false; }   // (a long bank of lying snow is no heavy thing)
    !g.destr.furn.fronts.iter().any(|f| f[0] > a.x0 - 0.3 && f[0] < a.x1 + 0.3 && f[2] > a.z0 - 0.3 && f[2] < a.z1 + 0.3)
}
/// everything resting on `seed`, layer by layer; None = something up there is not the rule's to take (furnblast.ts Furniture.mouthStack)
pub fn furniture_mouth_stack(g: &mut crate::game::Game, seed: &[crate::furnblast::AABB], done: &JsSet<crate::Num>, n: f64) -> Option<Vec<f64>> {
    let n = n as usize;
    let touch = g.k.furn.touch; let (np, ns, _, _) = bits();
    let mut out: Vec<f64> = Vec::new(); let mut layer: Vec<AABB> = seed.to_vec();
    let mut d = 0;
    while d < 16 && !layer.is_empty() {
        let mut next: Vec<AABB> = Vec::new();
        for ix in 0..n {
            let j = ix as f64; let o = BoxId::from_ix(ix);
            let (oc, oh, ofl) = (g.col.boxes[o].c, g.col.boxes[o].h, flags_of(g, o));
            let y0 = oc[1] - oh[1];
            if ofl & np != 0 || done.has(Num(j)) || out.contains(&j) || !layer.iter().any(|t| m::abs(y0 - t.y1) <= touch) { continue; }
            let q = aabb_of(g, o);
            if !layer.iter().any(|t| m::abs(q.y0 - t.y1) <= touch && over(&q, t)) || crate::wallbreak::wall_break_breakable_at(g, j).is_some() { continue; }
            if !furniture_mouth_ok(g, j, None) { if ofl & ns != 0 && !g.destr.furn.kept.has(Num(j)) { continue; } return None; }   // (a drawn-only thing too tall to take stays where it hangs; a lamp's own card keeps what carries it)
            out.push(j); next.push(q);
        }
        layer = next;
        d += 1;
    }
    Some(out)
}
/// is the standing static `i` the mouth rule's to take as a THING (furnblast.ts Furniture.mouthMine)
pub fn furniture_mouth_mine(g: &mut crate::game::Game, i: f64, b: BoxId, a: crate::furnblast::AABB) -> bool {
    if g.destr.furn.member_at.has(Num(i)) { return true; }
    if snow_paint(g, b) { return true; }   // (lying snow banked at the wall's foot, under a roof or not)
    let does = box_does(g, b); let (h, c) = (g.col.boxes[b].h, g.col.boxes[b].c);
    let post = m::min(h[0], h[2]) < 0.1 && a.y1 > 1.5 && a.y1 - a.y0 > 1.0;   // (tall in itself: a binder on a top shelf is thin and high up, and no post)
    !post && furniture_roofed(g, c[0], c[2]) && (!truly(does.struct_) || (truly(does.counter) && a.y1 <= 2.0))
}
/// would mouth() take static `i` from `r`, a hole not blown yet? Reading only (furnblast.ts Furniture.mouthWouldTake)
pub fn furniture_mouth_would_take(g: &mut crate::game::Game, i: f64, r: [f64; 4], depth: Option<f64>) -> bool {
    let depth = depth.unwrap_or(0.0);
    if !g.k.destruction.on || g.destr.show_chaos.is_some() { return false; }
    let n = m::min(g.col.boxes.len() as f64, g.destr.walls.authored) as usize;
    let b = match box_at(g, i) { Some(b) => b, None => return false };
    if i >= n as f64 || is_piece(g, b) { return false; }
    let serial = g.engine.world.geom_serial;
    if g.destr.furn.mouth_memo.t != g.time || g.destr.furn.mouth_memo.serial != serial {
        let t = g.time; let mm = &mut g.destr.furn.mouth_memo; mm.t = t; mm.serial = serial; mm.can.clear(); mm.under.clear();
    }
    let a = aabb_of(g, b);
    let zone = OverPlanQ { x0: r[0], z0: r[1], x1: r[2], z1: r[3] };
    if over_plan(plan_a(&a), zone) {   // as a thing in the zone itself
        let can = match g.destr.furn.mouth_memo.can.get(Num(i)).copied() {
            Some(c) => c,
            None => {
                ensure_index(g);
                let (np, ns, _, _) = bits();
                let c = !(a.y0 >= 1.7 || a.y1 <= 0.2 || crate::wallbreak::wall_break_breakable_at(g, i).is_some() || flags_of(g, b) & (np | ns) != 0)
                    && furniture_mouth_mine(g, i, b, a) && furniture_mouth_ok(g, i, None) && furniture_mouth_stack(g, &[a], &JsSet::new(), n as f64).is_some();
                g.destr.furn.mouth_memo.can.set(Num(i), c);
                c
            }
        };
        if can { return true; }
    }
    // …or with the thing it rests on (mouthStack's own test of 'rests on')
    if depth >= 4.0 || crate::wallbreak::wall_break_breakable_at(g, i).is_some() || !furniture_mouth_ok(g, i, None) { return false; }
    let under = match g.destr.furn.mouth_memo.under.get(Num(i)).cloned() {
        Some(u) => u,
        None => {
            let (np, _, _, _) = bits(); let touch = g.k.furn.touch;
            let mut u: Vec<f64> = Vec::new();
            for jx in 0..n {
                let j = jx as f64; if j == i { continue; }
                let o = BoxId::from_ix(jx);
                let (oc, oh, ofl) = (g.col.boxes[o].c, g.col.boxes[o].h, flags_of(g, o));
                if ofl & np != 0 || m::abs(a.y0 - (oc[1] + oh[1])) > touch { continue; }
                let oa = aabb_of(g, o);
                if over(&a, &oa) { u.push(j); }
            }
            g.destr.furn.mouth_memo.under.set(Num(i), u.clone());
            u
        }
    };
    for j in under { if furniture_mouth_would_take(g, j, r, Some(depth + 1.0)) { return true; } }
    false
}
/// (furnblast.ts Furniture.mouth) see mouth_in
pub fn furniture_mouth(g: &mut crate::game::Game, at: V3, who: f64, r: [f64; 4], budget: &mut crate::furnblast::FurnitureMouthBudget) { mouth_in(g, at, who, r, &mut budget.chunks); }
/// `take` of Furniture.mouth: static `i` goes, the rule's own stand-in member (root, no neighbours) being read only for its `ghost`
fn mouth_take(g: &mut Game, i: f64, at: V3, who: f64, budget: &mut f64, done: &mut JsSet<Num>, hid: &mut Vec<f64>, snowy: &mut JsSet<Num>) {
    let b = bx(i); done.add(Num(i));
    let member = g.destr.furn.member_at.get(Num(i)).copied();
    let ghost = match member { Some(mi) => g.tab.furnblast_member[mi].ghost, None => flags_of(g, b) & bits().1 != 0 };
    if break_static_in(g, i, ghost, at, true, 0.6, who, budget) { hid.push(i); if snow_paint(g, b) { snowy.add(Num(i)); } }
    if member.is_some() { g.destr.furn.gone.add(Num(i)); } else { g.destr.furn.gone_mouth.add(Num(i)); }
    g.stats.furnblast.mouth += 1;
}
/// a hole a blast opens can be walked through: whatever still stands in a body's way over the two mouths `r` goes (furnblast.ts Furniture.mouth)
fn mouth_in(g: &mut Game, at: V3, who: f64, r: [f64; 4], budget: &mut f64) {
    ensure_index(g);
    let n = m::min(g.col.boxes.len() as f64, g.destr.walls.authored) as usize;
    let zone = AABB { x0: r[0], z0: r[1], x1: r[2], z1: r[3], y0: 0.0, y1: 0.0 };
    let mut done: JsSet<Num> = JsSet::new(); let mut hid: Vec<f64> = Vec::new();
    let mut snowy: JsSet<Num> = JsSet::new();   // (lying snow among what was hidden: the feed's row counts it by its stretch at the hole)
    let cx = (r[0] + r[2]) / 2.0; let cz = (r[1] + r[3]) / 2.0; let hx = (r[2] - r[0]) / 2.0; let hz = (r[3] - r[1]) / 2.0;
    let mut razed: Vec<AABB> = Vec::new();
    let (np, ns, _, _) = bits();
    for ix in 0..n {
        let i = ix as f64; let b = BoxId::from_ix(ix);
        let (bc, bh) = (g.col.boxes[b].c, g.col.boxes[b].h);
        let reach = m::max(bh[0], bh[2]) * 1.42;
        if is_piece(g, b) || m::abs(bc[0] - cx) > hx + reach || m::abs(bc[2] - cz) > hz + reach || done.has(Num(i)) { continue; }
        let a = aabb_of(g, b);
        if !over(&a, &zone) || a.y0 >= 1.7 || a.y1 <= 0.2 { continue; }   // (in a walking body's way: on the floor, on legs or on a wall)
        if let Some(br) = crate::wallbreak::wall_break_breakable_at(g, i) {   // the wall engine's cover: the stretch of it in the way
            if br.kind != BKind::Cover || br.id != i || br.open >= br.cells { continue; }
            let dir = furniture_away(g, at, bc);
            if let Some(row) = raze_in(g, i, r, who, dir, budget) {
                g.stats.furnblast.mouth += 1; g.stats.furnblast.razed += 1;
                razed.push(AABB { x0: m::max(a.x0, row.rect[0]), z0: m::max(a.z0, row.rect[1]), x1: m::min(a.x1, row.rect[2]), z1: m::min(a.z1, row.rect[3]), y0: a.y0, y1: a.y1 });
            }
            continue;
        }
        let fl = flags_of(g, b);
        if fl & np != 0 && (g.destr.furn.hidden.has(Num(i)) || crate::destruction::destruction_flags0(g, i).is_some()) {   // gone already: what it carried hangs in the hole, and goes
            match furniture_mouth_stack(g, &[a], &done, n as f64) {
                Some(up) => { for j in up { mouth_take(g, j, at, who, budget, &mut done, &mut hid, &mut snowy); } }
                None => g.stats.furnblast.mouth_left += 1,
            }
            continue;
        }
        if fl & (np | ns) != 0 { continue; }   // (drawn only, or authored unseen: nothing a body meets)
        let up = if furniture_mouth_mine(g, i, b, a) && furniture_mouth_ok(g, i, None) { furniture_mouth_stack(g, &[a], &done, n as f64) } else { None };
        let up = match up { Some(u) => u, None => { g.stats.furnblast.mouth_left += 1; continue; } };
        mouth_take(g, i, at, who, budget, &mut done, &mut hid, &mut snowy);
        for j in up { mouth_take(g, j, at, who, budget, &mut done, &mut hid, &mut snowy); }
    }
    if !razed.is_empty() {   // (what stood on the razed stretch: a chest's top)
        match furniture_mouth_stack(g, &razed, &done, n as f64) {
            Some(up) => { for j in up { mouth_take(g, j, at, who, budget, &mut done, &mut hid, &mut snowy); } }
            None => g.stats.furnblast.mouth_left += 1,
        }
    }
    // THE BOTTOM LAYER GOES WITH THE PILE: a low thing with a body that CARRIED something now gone goes too, with what still rests on it
    let (base_thick, touch) = (g.k.furn.mouth.base_thick, g.k.furn.touch);
    for ix in 0..n {
        let i = ix as f64; let b = BoxId::from_ix(ix);
        let (bc, bh) = (g.col.boxes[b].c, g.col.boxes[b].h);
        let reach = m::max(bh[0], bh[2]) * 1.42;
        if m::abs(bc[0] - cx) > hx + reach || m::abs(bc[2] - cz) > hz + reach || done.has(Num(i)) || flags_of(g, b) & (np | ns) != 0 { continue; }
        let a = aabb_of(g, b);
        if !over(&a, &zone) || a.y1 > 0.2 || a.y1 - a.y0 < base_thick || crate::wallbreak::wall_break_breakable_at(g, i).is_some() || !furniture_mouth_mine(g, i, b, a) || !furniture_mouth_ok(g, i, None) { continue; }
        let mut carried = false;
        for jx in 0..n {
            if carried { break; }
            let j = jx as f64;
            if j == i || !(done.has(Num(j)) || g.destr.furn.hidden.has(Num(j))) { continue; }
            let o = BoxId::from_ix(jx);
            let (oc, oh) = (g.col.boxes[o].c, g.col.boxes[o].h);
            if m::abs(oc[1] - oh[1] - a.y1) <= touch { let oa = aabb_of(g, o); carried = over(&oa, &a); }
        }
        if !carried { continue; }
        let up = match furniture_mouth_stack(g, &[a], &done, n as f64) { Some(u) => u, None => { g.stats.furnblast.mouth_left += 1; continue; } };
        mouth_take(g, i, at, who, budget, &mut done, &mut hid, &mut snowy);
        for j in up { mouth_take(g, j, at, who, budget, &mut done, &mut hid, &mut snowy); }
    }
    if hid.is_empty() { return; }
    let (mut qx0, mut qx1, mut qz0, mut qz1) = (1e9, -1e9, 1e9, -1e9);
    for &i in &hid { let b = bx(i); let a = aabb_of(g, b); qx0 = m::min(qx0, a.x0); qx1 = m::max(qx1, a.x1); qz0 = m::min(qz0, a.z0); qz1 = m::max(qz1, a.z1); }
    let c: V3 = [(qx0 + qx1) / 2.0, 0.6, (qz0 + qz1) / 2.0];
    crate::scene_collision::sound_paths_invalidate(g);
    crate::smoke_types::smoke_system_like_obstacles_changed(g, c, m::hyp2(qx1 - qx0, qz1 - qz0) / 2.0 + 1.5);
    let ch = crate::scene_collision::nav_grid_rebake(g, qx0, qz0, qx1, qz1);
    let mut opened: Vec<f64> = Vec::new(); let mut k = 0; while k < ch.len() { if ch[k + 1] == 0.0 { opened.push(ch[k]); } k += 2; }
    g.stats.furnblast.nav_opened += opened.len() as i32; furniture_bump(g);
    let mut rect: [f64; 4] = [qx0, qz0, qx1, qz1]; let mut pos = c;
    if snowy.len() > 0 {
        let (mut nx0, mut nx1, mut nz0, mut nz1) = (1e9, -1e9, 1e9, -1e9);
        for &i in &hid {
            let b = bx(i); let a = aabb_of(g, b); let sn = snowy.has(Num(i));
            nx0 = m::min(nx0, if sn { m::max(a.x0, zone.x0) } else { a.x0 }); nx1 = m::max(nx1, if sn { m::min(a.x1, zone.x1) } else { a.x1 });
            nz0 = m::min(nz0, if sn { m::max(a.z0, zone.z0) } else { a.z0 }); nz1 = m::max(nz1, if sn { m::min(a.z1, zone.z1) } else { a.z1 });
        }
        if nx1 >= nx0 && nz1 >= nz0 { rect = [nx0, nz0, nx1, nz1]; pos = [(nx0 + nx1) / 2.0, 0.6, (nz0 + nz1) / 2.0]; }
    }
    let walkable = !opened.is_empty();
    crate::wallbreak::wall_break_note(g, &WallBreakNoteC { kind: LevelChangeKind::Cover, cause: WallBreakNoteCCause::Blast, name: None, id: hid[0], pos, rect, opened, walkable, sight: true, dead: true, who });
}
/// the way off the blast on the floor; right on it, a drawn direction (furnblast.ts Furniture.away)
pub fn furniture_away(g: &mut crate::game::Game, at: V3, p: V3) -> V3 {
    let dx = p[0] - at[0]; let dz = p[2] - at[2]; let l = m::hyp2(dx, dz);
    if l > 0.05 { return [dx / l, 0.0, dz / l]; }
    let a = furniture_hash(g) * m::PI * 2.0;
    [m::cos(a), 0.0, m::sin(a)]
}
/// a burst of chips (furnblast.ts Furniture.chips)
pub fn furniture_chips(g: &mut crate::game::Game, c_2: V3, v: V3, b: BoxId, who: f64, n: f64) {
    if crate::particle_fx::gpu_fx(g).is_none() { return; }
    let wood = box_does(g, b).surf == Surf::Wood;
    let albedo = g.col.boxes[b].albedo;
    seam_emit_debris(g, c_2, v3::normalize([v[0], m::max(0.3, v[1]), v[2]]), n, 2.4, 0.7, 1.1, 18.0, 0.3, if wood { 0.006 } else { 0.01 }, 0.6, if wood { 3.0 } else { 1.0 }, v3::copy(albedo), who);
}
/// the world's geometry moved (furnblast.ts Furniture.bump)
pub fn furniture_bump(g: &mut crate::game::Game) { g.engine.world.geom_serial += 1.0; crate::scene_collision::wrote(g); }
/// level reset: every static this module hid back as authored, nav, sound and smoke told (furnblast.ts Furniture.reset)
pub fn furniture_reset(g: &mut crate::game::Game) {
    let f = &mut g.destr.furn; f.counter = 1.0; f.last = None; f.gone_props = 0.0; f.gone.clear(); f.gone_mouth.clear();
    if g.destr.furn.hidden.is_empty() { return; }
    let hidden: Vec<(Num, f64)> = g.destr.furn.hidden.iter().map(|(k, v)| (k, *v)).collect();
    for (i, fl) in hidden { let b = bx(i.0); g.col.boxes[b].flags = fl; }
    g.destr.furn.hidden.clear(); furniture_bump(g);
    let w = crate::scene_world::world();
    crate::scene_collision::nav_grid_rebake(g, w.min[0], w.min[2], w.max[0], w.max[2]); crate::scene_collision::sound_paths_invalidate(g);
    crate::smoke_types::smoke_system_like_obstacles_changed(g, [(w.min[0] + w.max[0]) / 2.0, 1.0, (w.min[2] + w.max[2]) / 2.0], 60.0);
    // the AI was told of every piece that went: it is told the level is back too, unless the walls' own reset has just said so
    furniture_note_reset(g, FurnitureNoteResetCause::Reset);
}
/// (furnblast.ts Furniture.snapshot) not translated: a checkpoint is a clone of Game (conventions 6)
pub fn furniture_snapshot(g: &mut crate::game::Game) -> Option<crate::furnblast::FurnSave> { todo!("not translated: checkpoints are a clone of Game") }
/// (furnblast.ts Furniture.check) not translated: a checkpoint is a clone of Game
pub fn furniture_check(g: &mut crate::game::Game, s: Option<&crate::furnblast::FurnSave>) -> Option<String> { todo!("not translated: checkpoints are a clone of Game") }
/// (furnblast.ts Furniture.restore) not translated: a checkpoint is a clone of Game
pub fn furniture_restore(g: &mut crate::game::Game, s: Option<&crate::furnblast::FurnSave>) { todo!("not translated: checkpoints are a clone of Game") }
/// one 'reset' row in the walls' change feed, unless its last row already says exactly that at this instant (furnblast.ts Furniture.noteReset)
pub fn furniture_note_reset(g: &mut crate::game::Game, cause: crate::furnblast::FurnitureNoteResetCause) {
    let say = match g.destr.walls.changes.last() {
        None => true,
        Some(l) => l.kind != LevelChangeKind::Reset || l.t != g.time || (cause == FurnitureNoteResetCause::Load && l.cause != LevelChangeCause::Load),
    };
    if !say { return; }
    let w = crate::scene_world::world();
    let c = match cause { FurnitureNoteResetCause::Load => WallBreakNoteCCause::Load, FurnitureNoteResetCause::Reset => WallBreakNoteCCause::Reset };
    crate::wallbreak::wall_break_note(g, &WallBreakNoteC { kind: LevelChangeKind::Reset, cause: c, name: None, id: -1.0, pos: [0.0, 0.0, 0.0], rect: [w.min[0], w.min[2], w.max[0], w.max[2]], opened: Vec::new(), walkable: false, sight: false, dead: false, who: crate::consts::NOBODY_ID });
}
/// one line for the panel (furnblast.ts Furniture.summary)
pub fn furniture_summary(g: &mut crate::game::Game) -> String {
    let s = &g.stats.furnblast;
    format!("furniture on · {} blasts · {} broken ({} statics, {} panels, {} things on them) · props {} thrown {} broken · chunks {} (+{} as chips) · shielded {}", s.blasts, s.groups, s.statics, s.razed, s.items, s.props_thrown, s.props_broken, s.chunks, s.demoted, s.shielded + s.props_shielded)
}
