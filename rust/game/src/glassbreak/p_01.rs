//! src/game/glassbreak.ts lines 1 to 456
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small helpers

/// a box's row (Box.* in the TypeScript). Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of Box rows; where the table comes back this is the one line `&mut g.<table>[b]`
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }
/// `g.level.boxes[bi]`: undefined off either end of the list. The level's list IS the collision's (game.ts:1100)
fn box_at(g: &Game, bi: f64) -> Option<BoxId> {
    if !(bi >= 0.0) || bi != m::floor(bi) || bi as usize >= g.col.boxes.len() { return None; }
    Some(g.col.boxes[bi as usize])
}
/// `st`, read: the pane's row, which an entry point has already made or found
fn pane(g: &Game, bi: f64) -> &PaneState { g.glass_break.panes.get(crate::Num(bi)).unwrap() }
/// `st`, written: the row itself
fn pane_mut(g: &mut Game, bi: f64) -> &mut PaneState { g.glass_break.panes.get_mut(crate::Num(bi)).unwrap() }
/// `(st.pending).sort((x, y) => x.at - y.at || x.fid - y.fid)`
fn sort_pending(st: &mut PaneState) { js::sort_by(&mut st.pending, |x, y| js::or(x.at - y.at, x.fid - y.fid)); }
/// `st.pending.some(p => p.fid === fid)`
fn is_pending(st: &PaneState, fid: f64) -> bool { st.pending.iter().any(|p| p.fid == fid) }
/// `S.e += energy; S.hits++` on site i
fn count_on_site(g: &mut Game, bi: f64, i: f64, energy: f64) { let s = &mut pane_mut(g, bi).sites[i as usize]; s.e += energy; s.hits += 1.0; }
/// one step of a blast's star seed: `(Math.imul(s ^ (s >>> 15), 0x2C1B3C6D) + 0x297A2D39) >>> 0`
fn star_step(s: f64) -> f64 {
    let x = js::to_i32(s) ^ ((js::to_u32(s) >> 15) as i32);
    js::to_u32(m::imul(x, 0x2C1B3C6D) as f64 + 0x297A2D39u32 as f64) as f64
}
/// toUV's arithmetic
fn uv_on(yaw: f64, c: V3, u_axis: f64, x: V3) -> [f64; 2] {
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    let px = x[0] - c[0]; let pz = x[2] - c[2];
    let lx = cs * px - sn * pz; let lz = sn * px + cs * pz;
    [if u_axis == 0.0 { lx } else { lz }, x[1] - c[1]]
}
/// toUVIn's clamp
fn uv_in(uv: [f64; 2], hu: f64, hv: f64) -> [f64; 2] {
    let [u, v] = uv;
    [m::max(-hu * 0.995, m::min(hu * 0.995, u)), m::max(-hv * 0.995, m::min(hv * 0.995, v))]
}
/// toWorld's arithmetic
fn world_on(yaw: f64, c: V3, u_axis: f64, u: f64, v: f64, w: f64) -> V3 {
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    let lx = if u_axis == 0.0 { u } else { w }; let lz = if u_axis == 0.0 { w } else { u };
    [c[0] + cs * lx + sn * lz, c[1] + v, c[2] - sn * lx + cs * lz]
}
/// normalOf's arithmetic
fn normal_on(yaw: f64, u_axis: f64) -> V3 {
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    if u_axis == 0.0 { [sn, 0.0, cs] } else { [cs, 0.0, -sn] }
}

// ---------------------------------------------------------------- the module's functions

/// the persisted switch alone: a machine with no storage answers on. The storage is the host's (glassbreak.ts glassBreakPersisted)
pub fn glass_break_persisted() -> bool { true }
/// the switch + knobs kept per machine: with no storage the defaults stand. The storage is the host's (glassbreak.ts loadGlassBreak)
pub fn load_glass_break(g: &mut crate::game::Game, tune: &mut crate::glassbreak::GbTune) {}
/// with no storage nothing is kept. The storage is the host's (glassbreak.ts saveGlassBreak)
pub fn save_glass_break(g: &mut crate::game::Game) {}

/// the one gate every entry point takes first (glassbreak.ts glassBreakLive). GLASSFX.on is settled on.
pub fn glass_break_live(g: &mut crate::game::Game) -> bool {
    g.tune.glass_break && crate::destruction::chaos_of(g) > 0.0 && g.engine.glass_fracture != Some(false)
}

/// metres round a pane's nearest point to a blast of strength k within which its pieces are taken (glassbreak.ts blastReach)
pub fn blast_reach(k: f64) -> f64 { 1.4 * k + 0.15 }

/// The class's fields as made; its hooks are called by name, so what is hung here is which they are (glassbreak.ts GlassBreak.constructor)
pub fn glass_break_new(g: &mut crate::game::Game) -> crate::glassbreak::GlassBreak {
    crate::scene_collision::glass_rule(g, Some(crate::scene_collision::PaneRule::GlassBreak), Some(crate::scene_collision::PaneRows::GlassBreak));
    g.engine.world.glass_open = Some(crate::FnField);
    g.col.glass_walkable = Some(crate::scene_collision::GlassWalkable::GlassBreak);
    crate::destruction::destruction_on_blast(g, crate::destruction::BlastListener::Glass);
    GlassBreak::blank()
}

/// a line of the module's log, last 60 kept (GlassBreak.note)
pub fn glass_break_note(g: &mut crate::game::Game, s: &str) {
    let line = format!("{}s {}", js::to_fixed(g.time, 1), s);
    g.glass_break.log.push(line);
    if g.glass_break.log.len() > 60 { g.glass_break.log.remove(0); }
}
/// a line of the pane's log, last 16 kept (GlassBreak.plog)
pub fn glass_break_plog(g: &mut crate::game::Game, s: &str, bi: f64) {
    let st = pane_mut(g, bi);
    st.log.push(String::from(s));
    if st.log.len() > 16 { st.log.remove(0); }
}

/// which face of pane bi has the open sky: +1 its normal's side, −1 the other, 0 neither or both; cached per pane (GlassBreak.skySide)
pub fn glass_break_sky_side(g: &mut crate::game::Game, bi: f64) -> f64 {
    if let Some(e) = g.glass_break.ext.get(crate::Num(bi)) { return *e; }
    let Some(b) = box_at(g, bi) else { return 0.0; };
    let (yaw, bc, bh) = { let r = seam_box(g, b); (r.yaw, r.c, r.h) };
    let cs = m::cos(yaw); let sn = m::sin(yaw); let wide = bh[0] >= bh[2];
    let n: V3 = if wide { [sn, 0.0, cs] } else { [cs, 0.0, -sn] };
    let a: V3 = if wide { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
    let hu = m::max(bh[0], bh[2]);
    let mut sky = [0i32; 2];
    for k in 0..2usize { for off in [0.5, 1.0, 1.6] { for t in [-0.5, 0.0, 0.5] {
        let d = (if k != 0 { 1.0 } else { -1.0 }) * off;
        let p: V3 = [bc[0] + n[0] * d + a[0] * t * hu, bc[1], bc[2] + n[2] * d + a[2] * t * hu];
        if !crate::scene_collision::solid_blocked(g, p, [p[0], EXT_SKY_TOP, p[2]], Some(0.02)) { sky[k] += 1; }
    } } }
    let e = if (sky[0] >= 5) == (sky[1] >= 5) { 0.0 } else if sky[1] >= 5 { 1.0 } else { -1.0 };
    g.glass_break.ext.set(crate::Num(bi), e);
    e
}
/// is pane bi on an outside wall (GlassBreak.exteriorOf)
pub fn glass_break_exterior_of(g: &mut crate::game::Game, bi: f64) -> bool { glass_break_sky_side(g, bi) != 0.0 }

/// the type a pane breaks as when nobody has said; GLASSREAL.place is settled on: where it sits decides (GlassBreak.typeOf)
pub fn glass_break_type_of(g: &mut crate::game::Game, b: BoxId, bi: Option<f64>) -> crate::glassbreak::GlassType {
    let bi = match bi {
        Some(bi) => bi,
        None => match g.col.boxes.iter().position(|x| *x == b) { Some(i) => i as f64, None => -1.0 },   // indexOf
    };
    let outside = bi >= 0.0 && glass_break_exterior_of(g, bi);
    let (bc, bh) = { let r = seam_box(g, b); (r.c, r.h) };
    if if outside { bh[1] < 0.4 } else { m::max(bh[0], bh[2]) < 0.35 } { return GlassType::Laminated; }
    let foot = bc[1] - bh[1];
    if foot < g.k.glassreal.low_m || (foot < g.k.glassreal.door_foot_m && glass_break_door_gap(g, b) <= g.k.glassreal.door_m) { GlassType::Tempered } else { GlassType::Annealed }
}

/// metres on the plan from pane b to the nearest door's opening; Infinity with no door; kept nowhere (GlassBreak.doorGap)
pub fn glass_break_door_gap(g: &mut crate::game::Game, b: BoxId) -> f64 {
    let (yaw, bc, bh) = { let r = seam_box(g, b); (r.yaw, r.c, r.h) };
    let cs = m::cos(yaw); let sn = m::sin(yaw); let wide = bh[0] >= bh[2]; let hu = m::max(bh[0], bh[2]);
    let ax = if wide { cs } else { sn }; let az = if wide { -sn } else { cs };
    let a: V3 = [bc[0] - ax * hu, 0.0, bc[2] - az * hu];
    let c: V3 = [bc[0] + ax * hu, 0.0, bc[2] + az * hu];
    let mut d = m::INF;
    for ix in 0..g.doors.list.len() {
        let door = g.doors.list[ix];
        let (hinge, closed_dir, width) = { let def = &g.dr[door].def; (def.hinge, def.closed_dir, def.width) };
        d = m::min(d, crate::devices::seg_seg_closest(a, c, [hinge[0], 0.0, hinge[1]], [hinge[0] + m::cos(closed_dir) * width, 0.0, hinge[1] + m::sin(closed_dir) * width]).dist);
    }
    d
}

/// the type of pane bi as it will break; None for anything not a sheet (GlassBreak.typeOfPane)
pub fn glass_break_type_of_pane(g: &mut crate::game::Game, bi: f64) -> Option<crate::glassbreak::GlassType> {
    let b = box_at(g, bi)?;
    if !crate::glass::is_sheet(g, Some(b)) { return None; }
    if let Some(t) = g.glass_break.types.get(crate::Num(bi)) { return Some(*t); }
    Some(glass_break_type_of(g, b, Some(bi)))
}

/// make pane bi break as `type_` from here on, whole first; None = back to typeOf (GlassBreak.setType)
pub fn glass_break_set_type(g: &mut crate::game::Game, bi: f64, type_: Option<crate::glassbreak::GlassType>) {
    match type_ {
        Some(t) => { g.glass_break.types.set(crate::Num(bi), t); }
        None => { g.glass_break.types.delete(crate::Num(bi)); }
    }
    g.glass_break.panes.delete(crate::Num(bi));
    crate::scene_collision::wrote(g);
    crate::scene_collision::glass_wrote(g);
    g.glass_break.blasted.clear();
    if let Some(src) = g.glass_break.gusts.get(crate::Num(bi)).copied() {
        if let Some(i) = g.wind.all.iter().position(|s| *s == src) { g.wind.all.remove(i); }
        g.glass_break.gusts.delete(crate::Num(bi));
    }
}

/// pane bi's state, made the first time it is asked: the row itself; None = not a sheet (GlassBreak.stateOf)
pub fn glass_break_state_of<'g>(g: &'g mut crate::game::Game, bi: f64) -> Option<&'g mut crate::glassbreak::PaneState> {
    if !g.glass_break.panes.has(crate::Num(bi)) {
        let type_ = glass_break_type_of_pane(g, bi)?;
        let b = box_at(g, bi).unwrap();
        let bh = seam_box(g, b).h;
        let u_axis = if bh[0] >= bh[2] { 0.0 } else { 2.0 };
        let key = crate::glass::box_seed(g, b);
        let exterior = glass_break_exterior_of(g, bi);
        let st = PaneState {
            bi, box_: b, key, type_, u_axis, hu: bh[u_axis as usize], hv: bh[1], ht: m::min(bh[0], bh[2]), exterior,
            sites: Vec::new(), run: false, open: vec![0u8; (crate::glass_pattern::GP_MAX_SITES * crate::glass_pattern::GP_FRAGS_PER_SITE) as usize], n_open: 0.0,
            hits: 0.0, energy: 0.0, damage: 0.0, frags: None, pending: Vec::new(), next_think: 0.0, sound_at: -1e9,
            dice: None, gone: false, last_dir: [0.0, 0.0, 1.0], last_who: 0.0, last_uv: [0.0, 0.0], walkable: false, log: Vec::new(),
        };
        g.glass_break.panes.set(crate::Num(bi), st);
        g.stats.glassbreak.panes += 1;
    }
    g.glass_break.panes.get_mut(crate::Num(bi))
}

/// world point → pane-local (u, v) (GlassBreak.toUV)
pub fn glass_break_to_uv(g: &mut crate::game::Game, x: V3, bi: f64) -> [f64; 2] {
    let (b, u_axis) = { let st = pane(g, bi); (st.box_, st.u_axis) };
    let (yaw, c) = { let r = seam_box(g, b); (r.yaw, r.c) };
    uv_on(yaw, c, u_axis, x)
}
/// …and held a hair inside the frame (GlassBreak.toUVIn)
pub fn glass_break_to_uv_in(g: &mut crate::game::Game, x: V3, bi: f64) -> [f64; 2] {
    let uv = glass_break_to_uv(g, x, bi);
    let st = pane(g, bi);
    uv_in(uv, st.hu, st.hv)
}
/// pane-local (u, v, w across the thickness) → world (GlassBreak.toWorld)
pub fn glass_break_to_world(g: &mut crate::game::Game, u: f64, v: f64, w: Option<f64>, bi: f64) -> V3 {
    let w = w.unwrap_or(0.0);
    let (b, u_axis) = { let st = pane(g, bi); (st.box_, st.u_axis) };
    let (yaw, c) = { let r = seam_box(g, b); (r.yaw, r.c) };
    world_on(yaw, c, u_axis, u, v, w)
}
/// the pane's unit normal = the direction of pane-local +w (GlassBreak.normalOf)
pub fn glass_break_normal_of(g: &mut crate::game::Game, bi: f64) -> V3 {
    let (b, u_axis) = { let st = pane(g, bi); (st.box_, st.u_axis) };
    let yaw = seam_box(g, b).yaw;
    normal_on(yaw, u_axis)
}
/// the pane's pieces, cut when first asked and kept on the row until its sites change (GlassBreak.fragsOf)
pub fn glass_break_frags_of(g: &mut crate::game::Game, bi: f64) -> Vec<crate::glass_pattern::GpFrag> {
    let st = pane_mut(g, bi);
    if let Some(f) = &st.frags { return f.clone(); }
    let f = if st.type_ == GlassType::Annealed { crate::glass_pattern::gp_fragments(&st.sites, st.run, st.hu, st.hv) } else { Vec::new() };
    st.frags = Some(f.clone());
    f
}

/// a new site at (u, v) for a blow of energy e, its ring zone fitted to the room the others leave; −1 = no room (GlassBreak.addSite)
pub fn glass_break_add_site(g: &mut crate::game::Game, u: f64, v: f64, e: f64, seed32: f64, bi: f64) -> f64 {
    let (n_sites, run, web) = { let st = pane(g, bi); (st.sites.len() as f64, st.run, st.type_ == GlassType::Laminated) };
    if n_sites >= m::min(crate::glass_pattern::GP_MAX_SITES, g.k.glassbreak.max_sites) || run { return -1.0; }
    let base = *g.k.glassbreak.rho.get(if web { crate::knobs::GlassbreakKRhoKey::Laminated } else { crate::knobs::GlassbreakKRhoKey::Annealed }).unwrap();
    let k = m::max(0.8, m::min(1.4, 0.75 + 0.25 * e));
    let top = crate::glass_pattern::GP_RHO_LO + crate::glass_pattern::GP_RHO_SPAN;
    let mut rho: [f64; 4] = [base[0] * k, base[1] * k, base[2] * k, base[3] * k];
    let mut rz = rho[2] * top * 1.001;
    let mut room = m::INF;
    for q in pane(g, bi).sites.iter() { room = m::min(room, m::hyp2(q.u - u, q.v - v) - q.rz); }
    if room < (if web { g.k.glassbreak.rz_min * 0.4 } else { g.k.glassbreak.rz_min }) { return -1.0; }
    if rz > room { let f = room / rz; rho[0] *= f; rho[1] *= f; rho[2] *= f; rz = room; }
    rho[3] = m::max(rho[3], rho[2] * 2.0);
    let seed = js::to_u32(seed32) >> 8;
    let r = *g.k.glassbreak.radials.get(if web { crate::knobs::GlassbreakKRadialsKey::Laminated } else { crate::knobs::GlassbreakKRadialsKey::Annealed }).unwrap();
    let merge = if web { 0.0 } else { g.k.glassbreak.merge };
    let st = pane_mut(g, bi);
    st.sites.push(crate::glass_pattern::GpSite {
        u, v,
        rot: (seed & 0xfff) as f64 / 4096.0 * m::PI * 2.0,
        n: m::min(crate::glass_pattern::GP_MAX_SECTORS, r[0] + js::rem((seed >> 12) as f64, m::max(1.0, r[1]))),
        seed: seed as f64, rho, rz, e,
        merge,
        hits: 1.0,
    });
    st.frags = None;
    let last = st.sites.len() as f64 - 1.0;
    g.stats.glassbreak.sites += 1;
    last
}
/// whose territory the point is in (GlassBreak.nearestSite)
pub fn glass_break_nearest_site(g: &mut crate::game::Game, u: f64, v: f64, bi: f64) -> f64 { crate::glass_pattern::gp_nearest(&pane(g, bi).sites, u, v) }

/// THE IMPACT ENTRY POINT: a blow of `energy` met pane `bi` at world x going `dir`: file it and answer what the glass did (GlassBreak.impact)
pub fn glass_break_impact(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact {
    crate::scene_collision::glass_begin(g);
    let res = glass_break_impact_in(g, bi, x, dir, energy, who, through);
    crate::scene_collision::glass_end(g);
    res
}
/// impact's body (GlassBreak.impactIn)
pub fn glass_break_impact_in(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact {
    use crate::glass_pattern::{gp_classify, gp_unfid};
    let b = box_at(g, bi);
    let shape = match b { Some(b) => { let r = seam_box(g, b); Some((r.flags, r.h)) } None => None };
    let is_glass = match shape { Some((flags, _)) => (js::to_i32(flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) != 0, None => false };
    let thickness = match (is_glass, shape) { (true, Some((_, h))) => 2.0 * m::min(h[0], h[2]), _ => 0.0 };
    let mut res = GlassImpact { pane: is_glass, live: false, type_: GlassType::Annealed, thickness, was_open: false, punched: false, diced: false, released: 0.0, cracked: false };
    if !is_glass { return res; }
    res.type_ = glass_break_type_of_pane(g, bi).unwrap_or(GlassType::Laminated);
    if !glass_break_live(g) { return res; }
    if glass_break_state_of(g, bi).is_none() { return res; }
    res.live = true;
    let [u, v] = glass_break_to_uv_in(g, x, bi);
    if glass_break_open_uv(g, u, v, bi) { res.was_open = true; res.punched = true; return res; }
    {
        let st = pane_mut(g, bi);
        st.hits += 1.0; st.energy += energy; st.damage = m::min(1.0, st.damage + 0.12 * energy); st.last_who = who; st.last_uv = [u, v];
        if let Some(d) = dir { if v3::len(d) > 1e-6 { st.last_dir = v3::normalize(d); } }
    }
    g.stats.glassbreak.impacts += 1;
    let (key, type_) = { let st = pane(g, bi); (st.key, st.type_) };
    let seed = crate::glass::point_seed(key, x);
    if type_ == GlassType::Tempered {
        if pane(g, bi).energy >= g.k.glassbreak.break_e.tempered || through == Some(true) {
            let started = pane(g, bi).dice.is_none();
            glass_break_start_dice(g, u, v, seed, "hit", bi);
            res.diced = true; res.punched = glass_break_open_uv(g, u, v, bi); res.released = if started { 1.0 } else { 0.0 }; res.cracked = true;
        }
        return res;
    }
    if type_ == GlassType::Laminated {
        if gp_classify(&pane(g, bi).sites, false, u, v) < 0.0 && glass_break_add_site(g, u, v, energy, seed, bi) >= 0.0 { res.cracked = true; return res; }
        let i = glass_break_nearest_site(g, u, v, bi);
        if i >= 0.0 { count_on_site(g, bi, i, energy); }
        return res;
    }
    // annealed
    let hard = through.unwrap_or(energy >= g.k.glassbreak.punch_e);
    let mut fid = { let st = pane(g, bi); gp_classify(&st.sites, st.run, u, v) };
    let mut no_room = false;
    if fid >= 0.0 {
        count_on_site(g, bi, gp_unfid(fid).i, energy);
        if hard { res.released += glass_break_knock_out(g, fid, bi); }
    } else {
        let i = glass_break_add_site(g, u, v, energy, seed, bi);
        if i >= 0.0 {
            res.cracked = true;
            if hard { res.released += glass_break_release_crater(g, i, GlassBreakReleaseCraterCause::Hit, bi); }
        } else {
            let j = glass_break_nearest_site(g, u, v, bi);
            if j >= 0.0 { count_on_site(g, bi, j, energy); no_room = true; }
        }
    }
    let (run, total) = { let st = pane(g, bi); (st.run, st.energy) };
    if !run && (total >= g.k.glassbreak.break_e.annealed || energy >= g.k.glassbreak.run_single_e || (through == Some(true) && no_room)) {
        glass_break_set_run(g, "the cracks run to the frame", bi);
        if hard && !glass_break_open_uv(g, u, v, bi) {
            fid = gp_classify(&pane(g, bi).sites, true, u, v);
            if fid >= 0.0 { res.released += glass_break_knock_out(g, fid, bi); }
        }
        let j = if hard { glass_break_nearest_site(g, u, v, bi) } else { -1.0 };
        if j >= 0.0 && g.k.glassbreak.run_star > 0.0 {
            let frags = glass_break_frags_of(g, bi);
            for f in frags.iter() {
                let st = pane(g, bi);
                if f.i == j && f.b >= 1.0 && f.b <= m::min(2.0, g.k.glassbreak.run_star) && st.open[f.fid as usize] == 0 && !is_pending(st, f.fid) {
                    let at = g.time + g.k.glassbreak.run_ring_secs * (f.b + crate::debris::hash01(key * 53.0 + f.fid));
                    pane_mut(g, bi).pending.push(Pending { fid: f.fid, at, cause: PendingCause::Hit, from: None, k: 1.0 });
                }
            }
            sort_pending(pane_mut(g, bi));
        }
    }
    res.punched = glass_break_open_uv(g, u, v, bi);
    if res.released > 0.0 { res.cracked = true; }
    let (now, think) = (g.time, g.k.glassbreak.think);
    let st = pane_mut(g, bi);
    if st.next_think <= 0.0 { st.next_think = now + think; }
    res
}

/// the base's name for a round's impact at energy 1 (GlassBreak.hit)
pub fn glass_break_hit(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, who: f64, energy: Option<f64>) {
    let energy = energy.unwrap_or(1.0);
    glass_break_impact(g, bi, x, dir, energy, who, None);
}

/// a pellet through glass its own shell has just broken: no blow filed, but a standing piece of an annealed pane goes with it; fragments let go (GlassBreak.widen)
pub fn glass_break_widen(g: &mut crate::game::Game, bi: f64, x: V3) -> f64 {
    if !glass_break_live(g) { return 0.0; }
    match g.glass_break.panes.get(crate::Num(bi)) { Some(st) if st.type_ == GlassType::Annealed => {} _ => return 0.0 }
    let [u, v] = glass_break_to_uv_in(g, x, bi);
    if glass_break_open_uv(g, u, v, bi) { return 0.0; }
    let fid = { let st = pane(g, bi); crate::glass_pattern::gp_classify(&st.sites, st.run, u, v) };
    if fid < 0.0 { return 0.0; }
    crate::scene_collision::glass_begin(g);
    let n = glass_break_knock_out(g, fid, bi);
    if n > 0.0 {
        let (now, think) = (g.time, g.k.glassbreak.think);
        let st = pane_mut(g, bi);
        if st.next_think <= 0.0 { st.next_think = now + think; }
    }
    crate::scene_collision::glass_end(g);
    n
}

/// a hard blow on piece fid: it goes now (a crater wedge takes the whole crater); fragments let go (GlassBreak.knockOut)
pub fn glass_break_knock_out(g: &mut crate::game::Game, fid: f64, bi: f64) -> f64 {
    if pane(g, bi).open[fid as usize] != 0 { return 0.0; }
    let at = crate::glass_pattern::gp_unfid(fid);
    if at.b == 0.0 { return glass_break_release_crater(g, at.i, GlassBreakReleaseCraterCause::Hit, bi); }
    let frags = glass_break_frags_of(g, bi);
    let Some(f) = frags.iter().find(|x| x.fid == fid) else { return 0.0; };
    glass_break_release_frag(g, f, GlassBreakReleaseFragCause::Hit, None, 1.0, bi);
    1.0
}

/// the cracks run to the frame: the pane is daggers now (GlassBreak.setRun)
pub fn glass_break_set_run(g: &mut crate::game::Game, why: &str, bi: f64) {
    { let st = pane(g, bi); if st.run || st.type_ != GlassType::Annealed { return; } if st.sites.is_empty() { return; } }
    { let st = pane_mut(g, bi); st.run = true; st.frags = None; }
    g.stats.glassbreak.runs += 1;
    glass_break_plog(g, &format!("RUN: {}", why), bi);
    glass_break_note(g, &format!("pane {} runs ({})", js::num_str(bi), why));
    let (now, think) = (g.time, g.k.glassbreak.think);
    pane_mut(g, bi).next_think = now + think;
}

/// A BLAST at `at`: every sheet in reach takes it by type and by k = power × (1 − d / radius); the panes it changed (GlassBreak.blast)
pub fn glass_break_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>) -> f64 {
    let opts = opts.unwrap_or_default();
    if !glass_break_live(g) { return 0.0; }
    crate::scene_collision::glass_begin(g);
    let n = glass_break_blast_in(g, at, opts);
    crate::scene_collision::glass_end(g);
    n
}
/// blast's body: ONE charge, one blow, the same blast arriving twice counts where it is the stronger there (GlassBreak.blastIn)
pub fn glass_break_blast_in(g: &mut crate::game::Game, at: V3, opts: crate::glassbreak::GlassBlastOpts) -> f64 {
    let radius = opts.radius.unwrap_or(g.k.glassbreak.blast_m); let power = opts.power.unwrap_or(1.0); let who = opts.who.unwrap_or(0.0);
    if !(radius > 0.0) || !(power > 0.0) { return 0.0; }
    let now = g.time;
    g.glass_break.blasted.retain(|x| now >= x.t && now - x.t < 0.25);
    // (`twin`, an object of the list, is its place in the list: nothing below this adds to or takes from the list)
    let found = g.glass_break.blasted.iter().position(|x| m::hyp2(x.at[0] - at[0], x.at[2] - at[2]) < 0.05 && m::abs(x.at[1] - at[1]) < 0.6);
    let twin = match found {
        Some(i) => i,
        None => {
            g.glass_break.blasted.push(GlassBreakBlastedItem { t: now, at: v3::copy(at), k: JsMap::new() });
            g.stats.glassbreak.blasts += 1;
            g.glass_break.blasted.len() - 1
        }
    };
    let twin_at = g.glass_break.blasted[twin].at;
    let mut changed = 0.0;
    let mut ix = 0;
    while ix < g.engine.world.glass_statics.len() {
        let bi = g.engine.world.glass_statics[ix]; ix += 1;
        let Some(b) = box_at(g, bi) else { continue; };
        let q = nearest_on(g, b, at);
        let d = v3::dist(q, at);
        if d > radius { continue; }
        let k = power * (1.0 - d / radius);
        if k <= BLAST_MIN_K { continue; }
        let prev = g.glass_break.blasted[twin].k.get(crate::Num(bi)).copied().unwrap_or(0.0);
        if prev >= k { continue; }
        let Some(type_) = glass_break_type_of_pane(g, bi) else { continue; };
        if type_ == GlassType::Tempered && k <= g.k.glassbreak.blast_dice_k && !g.glass_break.panes.has(crate::Num(bi)) { continue; }
        match glass_break_state_of(g, bi) { Some(st) if !st.gone => {} _ => continue }
        g.glass_break.blasted[twin].k.set(crate::Num(bi), k);
        changed += blast_pane(g, bi, at, twin_at, q, d, k, prev, who);
    }
    changed
}
/// what one blast does to one pane (the loop's body of blastIn): 1 where it counts as changed
fn blast_pane(g: &mut Game, bi: f64, at: V3, twin_at: V3, q: V3, d: f64, k: f64, prev: f64, who: f64) -> f64 {
    use crate::glass_pattern::gp_classify;
    let away: V3 = if d > 1e-6 { v3::normalize(v3::sub(q, at)) } else { glass_break_normal_of(g, bi) };
    { let st = pane_mut(g, bi); st.last_dir = away; st.last_who = who; }
    let [qu, qv] = glass_break_to_uv(g, q, bi);
    { let st = pane_mut(g, bi); st.last_uv = [qu, qv]; st.damage = m::min(1.0, st.damage + 0.3 * (k - prev)); }
    let (key, type_) = { let st = pane(g, bi); (st.key, st.type_) };
    if type_ == GlassType::Tempered {
        if k > g.k.glassbreak.blast_dice_k && pane(g, bi).dice.is_none() {
            pane_mut(g, bi).energy += 1.0 + 2.0 * k;
            let seed = crate::glass::blast_seed(key, twin_at);
            glass_break_start_dice(g, qu, qv, seed, "blast", bi);
            return 1.0;
        }
        return 0.0;
    }
    // stars toward the blast: glass.ts crackNear's construction, so the first two sit on its decals
    let laminated = type_ == GlassType::Laminated;
    let stars_of = |kk: f64| if laminated { 3.0 } else { 1.0 + m::round(2.0 * m::min(1.0, kk)) };
    let mut s = crate::glass::blast_seed(key, twin_at);
    let stars = stars_of(k); let had = if prev > 0.0 { stars_of(prev) } else { 0.0 };
    let (hu, hv) = { let st = pane(g, bi); (st.hu, st.hv) };
    let mut n = 0.0;
    while n < stars {
        s = star_step(s);
        if n < had { n += 1.0; continue; }
        n += 1.0;
        let bits = js::to_u32(s);
        let fu = ((bits & 0xffff) as f64 / 65535.0) * 2.0 - 1.0; let fv = ((bits >> 16) as f64 / 65535.0) * 2.0 - 1.0;
        let on = glass_break_to_world(g, fu * 0.9 * hu, fv * 0.9 * hv, None, bi);
        let [pu, pv] = glass_break_to_uv(g, v3::lerp3(on, q, 0.35), bi);
        if glass_break_open_uv(g, pu, pv, bi) { continue; }
        let on_piece = { let st = pane(g, bi); gp_classify(&st.sites, st.run, pu, pv) >= 0.0 };
        if on_piece { continue; }
        glass_break_add_site(g, pu, pv, 0.6 + k, s, bi);
    }
    pane_mut(g, bi).energy += if prev > 0.0 { 2.0 * (k - prev) } else { 1.0 + 2.0 * k };
    if laminated {
        if k > g.k.glassbreak.blast_sheet_k { glass_break_release_sheet(g, away, k, bi); }
        return 1.0;
    }
    if k > g.k.glassbreak.blast_run_k { glass_break_set_run(g, "blast", bi); }
    let reach = blast_reach(k);
    let mut pieces = 0;
    let frags = glass_break_frags_of(g, bi);
    for f in frags.iter() {
        { let st = pane(g, bi); if st.open[f.fid as usize] != 0 || is_pending(st, f.fid) { continue; } }
        let dd = m::hyp2(f.cen[0] - qu, f.cen[1] - qv);
        if dd > reach { continue; }
        if k < g.k.glassbreak.tooth_k && f.area < g.k.glassbreak.tooth_area && f.frame.bottom + f.frame.left + f.frame.right + f.frame.top > 1e-4 { continue; }
        let due = g.time + dd * 0.03;
        pane_mut(g, bi).pending.push(Pending { fid: f.fid, at: due, cause: PendingCause::Blast, from: Some(at), k });
        pieces += 1;
    }
    if pieces != 0 { sort_pending(pane_mut(g, bi)); }
    glass_break_plog(g, &format!("blast k {}: {} pieces within {} m", js::to_fixed(k, 2), pieces, js::to_fixed(reach, 2)), bi);
    let (now, think) = (g.time, g.k.glassbreak.think);
    let st = pane_mut(g, bi);
    if st.next_think <= 0.0 { st.next_think = now + think; }
    1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pane_row(u_axis: f64, type_: GlassType) -> PaneState {
        let mut st = PaneState::zeroed();
        st.bi = 0.0; st.box_ = BoxId(0); st.key = 1.0; st.type_ = type_; st.u_axis = u_axis; st.hu = 1.2; st.hv = 0.9; st.ht = 0.006;
        st
    }
    fn game_with_a_pane(type_: GlassType) -> alloc::boxed::Box<Game> {
        let mut g = alloc::boxed::Box::new(Game::blank());
        g.glass_break.panes.set(crate::Num(0.0), pane_row(0.0, type_));
        g
    }
    fn bits(v: &[f64]) -> Vec<u64> { v.iter().map(|x| x.to_bits()).collect() }
    const C: V3 = [3.25, 1.4, -7.5];
    const YAW: f64 = 0.7;
    /// from the TypeScript's own toUV, toUVIn, toWorld, normalOf under bun with EXACT.on
    #[test]
    fn a_pane_s_frame_is_the_typescript_s_bits() {
        let x = [4.1, 2.0, -6.9];
        assert_eq!(bits(&uv_on(YAW, C, 0.0, x)), [0x3fd0de94a7bb39cd, 0x3fe3333333333334]);
        assert_eq!(bits(&uv_on(YAW, C, 2.0, x)), [0x3ff01a959f20e098, 0x3fe3333333333334]);
        assert_eq!(bits(&uv_in(uv_on(YAW, C, 0.0, [9.0, 9.0, 9.0]), 1.2, 0.9)), [0xbff31a9fbe76c8b4, 0x3feca7ef9db22d0e]);
        assert_eq!(bits(&world_on(YAW, C, 0.0, 0.3, -0.2, 0.01)), [0x400be31cd48a39f6, 0x3ff3333333333333, 0xc01ebe125a3e96d9]);
        assert_eq!(bits(&world_on(YAW, C, 2.0, 0.3, -0.2, 0.0)), [0x400b8bceae4ba854, 0x3ff3333333333333, 0xc01d150a5ce6d07e]);
        assert_eq!(bits(&normal_on(YAW, 0.0)), [0x3fe49d6e694619b8, 0, 0x3fe87996529f9d92]);
        assert_eq!(bits(&normal_on(YAW, 2.0)), [0x3fe87996529f9d92, 0, 0xbfe49d6e694619b8]);
    }
    /// from the TypeScript's own addSite under bun with EXACT.on: a site, a second fitted to the room the first leaves, one with no room, a web's
    #[test]
    fn a_site_is_the_typescript_s_bits() {
        let mut g = game_with_a_pane(GlassType::Annealed);
        assert_eq!(glass_break_add_site(&mut g, 0.1, 0.2, 1.3, 3735928559.0, 0.0), 0.0);
        let s = &pane(&g, 0.0).sites[0];
        assert_eq!((s.n, s.seed, s.hits), (11.0, 14593470.0, 1.0));
        assert_eq!(bits(&[s.u, s.v, s.rot, s.rz, s.e, s.merge]), [0x3fb999999999999a, 0x3fc999999999999a, 0x4015960fbd000e3b, 0x3fc7868c8c14f6b1, 0x3ff4cccccccccccd, 0x3fdccccccccccccd]);
        assert_eq!(bits(&s.rho), [0x3fa083126e978d50, 0x3fb34395810624dd, 0x3fc34395810624dd, 0x3fe1333333333333]);
        assert_eq!(glass_break_add_site(&mut g, 0.45, 0.2, 0.2, 305419896.0, 0.0), 1.0);
        let s = &pane(&g, 0.0).sites[1];
        assert_eq!((s.n, s.seed), (12.0, 1193046.0));
        assert_eq!(bits(&[s.rot, s.rz]), [0x3ffb3e55efd7e6e2, 0x3fc181e59dd410e4]);
        assert_eq!(bits(&s.rho), [0x3f989374bc6a7efa, 0x3facac083126e97a, 0x3fbcac083126e97a, 0x3fd999999999999a]);
        assert_eq!(glass_break_add_site(&mut g, 0.12, 0.21, 1.0, 77.0, 0.0), -1.0);
        let mut web = game_with_a_pane(GlassType::Laminated);
        assert_eq!(glass_break_add_site(&mut web, -0.3, 0.1, 5.0, 3405705229.0, 0.0), 0.0);
        let s = &pane(&web, 0.0).sites[0];
        assert_eq!((s.n, s.seed, s.merge), (11.0, 13303536.0, 0.0));
        assert_eq!(bits(&[s.rot, s.rz]), [0x401776b9a3aba619, 0x3fc8129bb9039738]);
        assert_eq!(bits(&s.rho), [0x3f99ce075f6fd21f, 0x3fb1eb851eb851eb, 0x3fc3b645a1cac083, 0x3fd3b645a1cac083]);
    }
    #[test]
    fn a_blast_s_star_seed_steps_as_the_language_s_does() {
        let cases: [(f64, f64); 7] = [
            (0.0, 695872825.0),
            (1.0, 1435855270.0),
            (3735928559.0, 2643587293.0),
            (4294967295.0, 2963287353.0),
            (123456789.0, 1199337779.0),
            (-5.5, 1628249837.0),
            (4294967301.25, 100817754.0),
        ];
        for (s, want) in cases { assert_eq!(star_step(s).to_bits(), want.to_bits(), "from {}", s); }
    }
}
