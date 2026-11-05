//! src/game/glassbreak.ts lines 1 to 456
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small helpers

/// `g.level.boxes[bi]`: undefined off either end of the list. A level's box is the row of its place in the list, in `g.col.boxes`
fn box_at(g: &Game, bi: f64) -> Option<BoxId> {
    if !(bi >= 0.0) || bi != m::floor(bi) || bi as usize >= g.level.boxes.len() { return None; }
    Some(BoxId(bi as u32))
}
/// `this.panes.get(bi)`, as a working copy: the holder writes it back by key (put_back)
fn pane_copy(g: &Game, bi: f64) -> Option<PaneState> { g.glass_break.panes.get(crate::Num(bi)).cloned() }
/// the row `st` was copied from takes what was done to the copy; a row deleted meanwhile stays deleted, as writes to an orphan object were lost
fn put_back(g: &mut Game, st: PaneState) { if let Some(row) = g.glass_break.panes.get_mut(crate::Num(st.bi)) { *row = st; } }
/// the pane rule (the first half: the view's `also` is part 2's). glass_rule keeps only that a rule is hung, so nothing calls this through the hook
fn pane_rule(g: &mut Game, bi: f64, p: V3) -> bool { glass_break_open_at(g, bi, p) }
/// `() => this`: what the copy judges panes by
fn pane_rows(g: &mut Game) -> crate::scene_box_mirror::PaneSource { crate::scene_box_mirror::PaneSource { view: glass_break_view(g) } }
/// `(st.pending).sort((x, y) => x.at - y.at || x.fid - y.fid)`
fn sort_pending(st: &mut PaneState) { js::sort_by(&mut st.pending, |x, y| js::or(x.at - y.at, x.fid - y.fid)); }
/// `st.pending.some(p => p.fid === fid)`
fn is_pending(st: &PaneState, fid: f64) -> bool { st.pending.iter().any(|p| p.fid == fid) }
/// one step of a blast's star seed: `(Math.imul(s ^ (s >>> 15), 0x2C1B3C6D) + 0x297A2D39) >>> 0`
fn star_step(s: f64) -> f64 {
    let x = js::to_i32(s) ^ ((js::to_u32(s) >> 15) as i32);
    js::to_u32(m::imul(x, 0x2C1B3C6D) as f64 + 0x297A2D39u32 as f64) as f64
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
    crate::scene_collision::glass_rule(g, Some(&mut pane_rule), Some(&mut pane_rows));
    g.engine.world.glass_open = Some(crate::FnField);
    g.col.glass_walkable = Some(crate::scene_collision::GlassWalkable::GlassBreak);
    crate::destruction::destruction_on_blast(g, &crate::destruction::BlastListener::Glass);
    GlassBreak::blank()
}

/// a line of the module's log, last 60 kept (GlassBreak.note)
pub fn glass_break_note(g: &mut crate::game::Game, s: &str) {
    let line = format!("{:.1}s {}", g.time, s);
    g.glass_break.log.push(line);
    if g.glass_break.log.len() > 60 { g.glass_break.log.remove(0); }
}
/// a line of the pane's log, last 16 kept (GlassBreak.plog)
pub fn glass_break_plog(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, s: &str) {
    st.log.push(String::from(s));
    if st.log.len() > 16 { st.log.remove(0); }
}

/// which face of pane bi has the open sky: +1 its normal's side, −1 the other, 0 neither or both; cached per pane (GlassBreak.skySide)
pub fn glass_break_sky_side(g: &mut crate::game::Game, bi: f64) -> f64 {
    if let Some(e) = g.glass_break.ext.get(crate::Num(bi)) { return *e; }
    let Some(b) = box_at(g, bi) else { return 0.0; };
    let (yaw, bc, bh) = (g.col.boxes[b].yaw, g.col.boxes[b].c, g.col.boxes[b].h);
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
        None => if (b.0 as usize) < g.level.boxes.len() { b.0 as f64 } else { -1.0 },   // indexOf: a level's box is the row of its place; a leaf or a piece of furniture is no box of the list
    };
    let outside = bi >= 0.0 && glass_break_exterior_of(g, bi);
    let (bc, bh) = (g.col.boxes[b].c, g.col.boxes[b].h);
    if if outside { bh[1] < 0.4 } else { m::max(bh[0], bh[2]) < 0.35 } { return GlassType::Laminated; }
    let foot = bc[1] - bh[1];
    if foot < g.k.glassreal.low_m || (foot < g.k.glassreal.door_foot_m && glass_break_door_gap(g, b) <= g.k.glassreal.door_m) { GlassType::Tempered } else { GlassType::Annealed }
}

/// metres on the plan from pane b to the nearest door's opening; Infinity with no door; kept nowhere (GlassBreak.doorGap)
pub fn glass_break_door_gap(g: &mut crate::game::Game, b: BoxId) -> f64 {
    let (yaw, bc, bh) = (g.col.boxes[b].yaw, g.col.boxes[b].c, g.col.boxes[b].h);
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
        let bh = g.col.boxes[b].h;
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
pub fn glass_break_to_uv(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, x: V3) -> [f64; 2] {
    let (yaw, bc) = (g.col.boxes[st.box_].yaw, g.col.boxes[st.box_].c);
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    let px = x[0] - bc[0]; let pz = x[2] - bc[2];
    let lx = cs * px - sn * pz; let lz = sn * px + cs * pz;
    [if st.u_axis == 0.0 { lx } else { lz }, x[1] - bc[1]]
}
/// …and held a hair inside the frame (GlassBreak.toUVIn)
pub fn glass_break_to_uv_in(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, x: V3) -> [f64; 2] {
    let [u, v] = glass_break_to_uv(g, st, x);
    [m::max(-st.hu * 0.995, m::min(st.hu * 0.995, u)), m::max(-st.hv * 0.995, m::min(st.hv * 0.995, v))]
}
/// pane-local (u, v, w across the thickness) → world (GlassBreak.toWorld)
pub fn glass_break_to_world(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, u: f64, v: f64, w: Option<f64>) -> V3 {
    let w = w.unwrap_or(0.0);
    let (yaw, bc) = (g.col.boxes[st.box_].yaw, g.col.boxes[st.box_].c);
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    let lx = if st.u_axis == 0.0 { u } else { w }; let lz = if st.u_axis == 0.0 { w } else { u };
    [bc[0] + cs * lx + sn * lz, bc[1] + v, bc[2] - sn * lx + cs * lz]
}
/// the pane's unit normal = the direction of pane-local +w (GlassBreak.normalOf)
pub fn glass_break_normal_of(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState) -> V3 {
    let yaw = g.col.boxes[st.box_].yaw;
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    if st.u_axis == 0.0 { [sn, 0.0, cs] } else { [cs, 0.0, -sn] }
}
/// the pane's pieces, cut when first asked and kept on `st` until its sites change (GlassBreak.fragsOf)
pub fn glass_break_frags_of(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) -> Vec<crate::glass_pattern::GpFrag> {
    if let Some(f) = &st.frags { return f.clone(); }
    let f = if st.type_ == GlassType::Annealed { crate::glass_pattern::gp_fragments(&st.sites, st.run, st.hu, st.hv) } else { Vec::new() };
    st.frags = Some(f.clone());
    f
}

/// a new site at (u, v) for a blow of energy e, its ring zone fitted to the room the others leave; −1 = no room (GlassBreak.addSite)
pub fn glass_break_add_site(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, u: f64, v: f64, e: f64, seed32: f64) -> f64 {
    if st.sites.len() as f64 >= m::min(crate::glass_pattern::GP_MAX_SITES, g.k.glassbreak.max_sites) || st.run { return -1.0; }
    let web = st.type_ == GlassType::Laminated;
    let base = *g.k.glassbreak.rho.get(if web { crate::knobs::GlassbreakKRhoKey::Laminated } else { crate::knobs::GlassbreakKRhoKey::Annealed }).unwrap();
    let k = m::max(0.8, m::min(1.4, 0.75 + 0.25 * e));
    let top = crate::glass_pattern::GP_RHO_LO + crate::glass_pattern::GP_RHO_SPAN;
    let mut rho: [f64; 4] = [base[0] * k, base[1] * k, base[2] * k, base[3] * k];
    let mut rz = rho[2] * top * 1.001;
    let mut room = m::INF;
    for q in st.sites.iter() { room = m::min(room, m::hyp2(q.u - u, q.v - v) - q.rz); }
    if room < (if web { g.k.glassbreak.rz_min * 0.4 } else { g.k.glassbreak.rz_min }) { return -1.0; }
    if rz > room { let f = room / rz; rho[0] *= f; rho[1] *= f; rho[2] *= f; rz = room; }
    rho[3] = m::max(rho[3], rho[2] * 2.0);
    let seed = js::to_u32(seed32) >> 8;
    let r = *g.k.glassbreak.radials.get(if web { crate::knobs::GlassbreakKRadialsKey::Laminated } else { crate::knobs::GlassbreakKRadialsKey::Annealed }).unwrap();
    st.sites.push(crate::glass_pattern::GpSite {
        u, v,
        rot: (seed & 0xfff) as f64 / 4096.0 * m::PI * 2.0,
        n: m::min(crate::glass_pattern::GP_MAX_SECTORS, r[0] + js::rem((seed >> 12) as f64, m::max(1.0, r[1]))),
        seed: seed as f64, rho, rz, e,
        merge: if web { 0.0 } else { g.k.glassbreak.merge },
        hits: 1.0,
    });
    st.frags = None;
    g.stats.glassbreak.sites += 1;
    st.sites.len() as f64 - 1.0
}
/// whose territory the point is in (GlassBreak.nearestSite)
pub fn glass_break_nearest_site(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, u: f64, v: f64) -> f64 { crate::glass_pattern::gp_nearest(&st.sites, u, v) }

/// THE IMPACT ENTRY POINT: a blow of `energy` met pane `bi` at world x going `dir`: file it and answer what the glass did (GlassBreak.impact)
pub fn glass_break_impact(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact {
    crate::scene_collision::glass_begin(g);
    let res = glass_break_impact_in(g, bi, x, dir, energy, who, through);
    crate::scene_collision::glass_end(g);
    res
}
/// impact's body (GlassBreak.impactIn)
pub fn glass_break_impact_in(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact {
    let b = box_at(g, bi);
    let is_glass = match b { Some(b) => (js::to_i32(g.col.boxes[b].flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) != 0, None => false };
    let thickness = match (is_glass, b) { (true, Some(b)) => { let h = g.col.boxes[b].h; 2.0 * m::min(h[0], h[2]) } _ => 0.0 };
    let mut res = GlassImpact { pane: is_glass, live: false, type_: GlassType::Annealed, thickness, was_open: false, punched: false, diced: false, released: 0.0, cracked: false };
    if !is_glass { return res; }
    res.type_ = glass_break_type_of_pane(g, bi).unwrap_or(GlassType::Laminated);
    if !glass_break_live(g) { return res; }
    // (the working copy: every step below takes g and the pane together; put_back gives it to the row by key)
    let Some(mut st) = glass_break_state_of(g, bi).map(|row| row.clone()) else { return res; };
    res.live = true;
    let [u, v] = glass_break_to_uv_in(g, &st, x);
    if glass_break_open_uv(g, &st, u, v) { res.was_open = true; res.punched = true; return res; }
    st.hits += 1.0; st.energy += energy; st.damage = m::min(1.0, st.damage + 0.12 * energy); st.last_who = who; st.last_uv = [u, v];
    if let Some(d) = dir { if v3::len(d) > 1e-6 { st.last_dir = v3::normalize(d); } }
    g.stats.glassbreak.impacts += 1;
    let seed = crate::glass::point_seed(st.key, x);
    impact_on(g, &mut st, &mut res, u, v, energy, through, seed);
    put_back(g, st);
    res
}
/// the part of impactIn that decides by type, on the working copy (every return of the TypeScript's after the first write comes through here, so the row is written back once)
fn impact_on(g: &mut Game, st: &mut PaneState, res: &mut GlassImpact, u: f64, v: f64, energy: f64, through: Option<bool>, seed: f64) {
    use crate::glass_pattern::{gp_classify, gp_unfid};
    if st.type_ == GlassType::Tempered {
        if st.energy >= g.k.glassbreak.break_e.tempered || through == Some(true) {
            let started = st.dice.is_none();
            glass_break_start_dice(g, st, u, v, seed, "hit");
            res.diced = true; res.punched = glass_break_open_uv(g, st, u, v); res.released = if started { 1.0 } else { 0.0 }; res.cracked = true;
        }
        return;
    }
    if st.type_ == GlassType::Laminated {
        if gp_classify(&st.sites, false, u, v) < 0.0 && glass_break_add_site(g, st, u, v, energy, seed) >= 0.0 { res.cracked = true; return; }
        let i = glass_break_nearest_site(g, st, u, v);
        if i >= 0.0 { let s = &mut st.sites[i as usize]; s.e += energy; s.hits += 1.0; }
        return;
    }
    // annealed
    let hard = through.unwrap_or(energy >= g.k.glassbreak.punch_e);
    let mut fid = gp_classify(&st.sites, st.run, u, v);
    let mut no_room = false;
    if fid >= 0.0 {
        let s = &mut st.sites[gp_unfid(fid).i as usize]; s.e += energy; s.hits += 1.0;
        if hard { res.released += glass_break_knock_out(g, st, fid); }
    } else {
        let i = glass_break_add_site(g, st, u, v, energy, seed);
        if i >= 0.0 {
            res.cracked = true;
            if hard { res.released += glass_break_release_crater(g, st, i, GlassBreakReleaseCraterCause::Hit); }
        } else {
            let j = glass_break_nearest_site(g, st, u, v);
            if j >= 0.0 { let s = &mut st.sites[j as usize]; s.e += energy; s.hits += 1.0; no_room = true; }
        }
    }
    if !st.run && (st.energy >= g.k.glassbreak.break_e.annealed || energy >= g.k.glassbreak.run_single_e || (through == Some(true) && no_room)) {
        glass_break_set_run(g, st, "the cracks run to the frame");
        if hard && !glass_break_open_uv(g, st, u, v) {
            fid = gp_classify(&st.sites, true, u, v);
            if fid >= 0.0 { res.released += glass_break_knock_out(g, st, fid); }
        }
        let j = if hard { glass_break_nearest_site(g, st, u, v) } else { -1.0 };
        if j >= 0.0 && g.k.glassbreak.run_star > 0.0 {
            let frags = glass_break_frags_of(g, st);
            for f in frags.iter() {
                if f.i == j && f.b >= 1.0 && f.b <= m::min(2.0, g.k.glassbreak.run_star) && st.open[f.fid as usize] == 0 && !is_pending(st, f.fid) {
                    let at = g.time + g.k.glassbreak.run_ring_secs * (f.b + crate::debris::hash01(st.key * 53.0 + f.fid));
                    st.pending.push(Pending { fid: f.fid, at, cause: PendingCause::Hit, from: None, k: 1.0 });
                }
            }
            sort_pending(st);
        }
    }
    res.punched = glass_break_open_uv(g, st, u, v);
    if res.released > 0.0 { res.cracked = true; }
    if st.next_think <= 0.0 { st.next_think = g.time + g.k.glassbreak.think; }
}

/// the base's name for a round's impact at energy 1 (GlassBreak.hit)
pub fn glass_break_hit(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, who: f64, energy: Option<f64>) {
    let energy = energy.unwrap_or(1.0);
    glass_break_impact(g, bi, x, dir, energy, who, None);
}

/// a pellet through glass its own shell has just broken: no blow filed, but a standing piece of an annealed pane goes with it; fragments let go (GlassBreak.widen)
pub fn glass_break_widen(g: &mut crate::game::Game, bi: f64, x: V3) -> f64 {
    if !glass_break_live(g) { return 0.0; }
    let Some(mut st) = pane_copy(g, bi) else { return 0.0; };
    if st.type_ != GlassType::Annealed { return 0.0; }
    let [u, v] = glass_break_to_uv_in(g, &st, x);
    if glass_break_open_uv(g, &st, u, v) { return 0.0; }
    let fid = crate::glass_pattern::gp_classify(&st.sites, st.run, u, v);
    if fid < 0.0 { return 0.0; }
    crate::scene_collision::glass_begin(g);
    let n = glass_break_knock_out(g, &mut st, fid);
    if n > 0.0 && st.next_think <= 0.0 { st.next_think = g.time + g.k.glassbreak.think; }
    put_back(g, st);
    crate::scene_collision::glass_end(g);
    n
}

/// a hard blow on piece fid: it goes now (a crater wedge takes the whole crater); fragments let go (GlassBreak.knockOut)
pub fn glass_break_knock_out(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, fid: f64) -> f64 {
    if st.open[fid as usize] != 0 { return 0.0; }
    let at = crate::glass_pattern::gp_unfid(fid);
    if at.b == 0.0 { return glass_break_release_crater(g, st, at.i, GlassBreakReleaseCraterCause::Hit); }
    let frags = glass_break_frags_of(g, st);
    let Some(f) = frags.iter().find(|x| x.fid == fid) else { return 0.0; };
    glass_break_release_frag(g, st, f, GlassBreakReleaseFragCause::Hit, None, 1.0);
    1.0
}

/// the cracks run to the frame: the pane is daggers now (GlassBreak.setRun)
pub fn glass_break_set_run(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, why: &str) {
    if st.run || st.type_ != GlassType::Annealed { return; }
    if st.sites.is_empty() { return; }
    st.run = true; st.frags = None; g.stats.glassbreak.runs += 1;
    glass_break_plog(g, st, &format!("RUN: {}", why));
    glass_break_note(g, &format!("pane {} runs ({})", st.bi, why));
    st.next_think = g.time + g.k.glassbreak.think;
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
        let Some(mut st) = glass_break_state_of(g, bi).map(|row| row.clone()) else { continue; };   // (the working copy, as in impact_in)
        if st.gone { continue; }
        g.glass_break.blasted[twin].k.set(crate::Num(bi), k);
        changed += blast_pane(g, &mut st, at, twin_at, q, d, k, prev, who);
        put_back(g, st);
    }
    changed
}
/// what one blast does to one pane (the loop's body of blastIn, on the working copy): 1 where it counts as changed
fn blast_pane(g: &mut Game, st: &mut PaneState, at: V3, twin_at: V3, q: V3, d: f64, k: f64, prev: f64, who: f64) -> f64 {
    use crate::glass_pattern::gp_classify;
    let away: V3 = if d > 1e-6 { v3::normalize(v3::sub(q, at)) } else { glass_break_normal_of(g, st) };
    st.last_dir = away; st.last_who = who;
    let [qu, qv] = glass_break_to_uv(g, st, q);
    st.last_uv = [qu, qv];
    st.damage = m::min(1.0, st.damage + 0.3 * (k - prev));
    if st.type_ == GlassType::Tempered {
        if k > g.k.glassbreak.blast_dice_k && st.dice.is_none() {
            st.energy += 1.0 + 2.0 * k;
            let seed = crate::glass::blast_seed(st.key, twin_at);
            glass_break_start_dice(g, st, qu, qv, seed, "blast");
            return 1.0;
        }
        return 0.0;
    }
    // stars toward the blast: glass.ts crackNear's construction, so the first two sit on its decals
    let laminated = st.type_ == GlassType::Laminated;
    let stars_of = |kk: f64| if laminated { 3.0 } else { 1.0 + m::round(2.0 * m::min(1.0, kk)) };
    let mut s = crate::glass::blast_seed(st.key, twin_at);
    let stars = stars_of(k); let had = if prev > 0.0 { stars_of(prev) } else { 0.0 };
    let mut n = 0.0;
    while n < stars {
        s = star_step(s);
        if n < had { n += 1.0; continue; }
        n += 1.0;
        let bits = js::to_u32(s);
        let fu = ((bits & 0xffff) as f64 / 65535.0) * 2.0 - 1.0; let fv = ((bits >> 16) as f64 / 65535.0) * 2.0 - 1.0;
        let on = glass_break_to_world(g, st, fu * 0.9 * st.hu, fv * 0.9 * st.hv, None);
        let [pu, pv] = glass_break_to_uv(g, st, v3::lerp3(on, q, 0.35));
        if glass_break_open_uv(g, st, pu, pv) || gp_classify(&st.sites, st.run, pu, pv) >= 0.0 { continue; }
        glass_break_add_site(g, st, pu, pv, 0.6 + k, s);
    }
    st.energy += if prev > 0.0 { 2.0 * (k - prev) } else { 1.0 + 2.0 * k };
    if laminated {
        if k > g.k.glassbreak.blast_sheet_k { glass_break_release_sheet(g, st, away, k); }
        return 1.0;
    }
    if k > g.k.glassbreak.blast_run_k { glass_break_set_run(g, st, "blast"); }
    let reach = blast_reach(k);
    let mut pieces = 0;
    let frags = glass_break_frags_of(g, st);
    for f in frags.iter() {
        if st.open[f.fid as usize] != 0 || is_pending(st, f.fid) { continue; }
        let dd = m::hyp2(f.cen[0] - qu, f.cen[1] - qv);
        if dd > reach { continue; }
        if k < g.k.glassbreak.tooth_k && f.area < g.k.glassbreak.tooth_area && f.frame.bottom + f.frame.left + f.frame.right + f.frame.top > 1e-4 { continue; }
        st.pending.push(Pending { fid: f.fid, at: g.time + dd * 0.03, cause: PendingCause::Blast, from: Some(at), k });
        pieces += 1;
    }
    if pieces != 0 { sort_pending(st); }
    glass_break_plog(g, st, &format!("blast k {:.2}: {} pieces within {:.2} m", k, pieces, reach));
    if st.next_think <= 0.0 { st.next_think = g.time + g.k.glassbreak.think; }
    1.0
}

#[cfg(test)]
mod tests {
    use super::*;
    /// the TypeScript's own: `(Math.imul(s ^ (s >>> 15), 0x2C1B3C6D) + 0x297A2D39) >>> 0`, run under node, non-integers and a negative as a double can hold them
    fn pane(u_axis: f64, type_: GlassType) -> PaneState {
        let mut st = PaneState::zeroed();
        st.bi = 0.0; st.box_ = BoxId(0); st.key = 1.0; st.type_ = type_; st.u_axis = u_axis; st.hu = 1.2; st.hv = 0.9; st.ht = 0.006;
        st
    }
    fn game_with_a_box() -> alloc::boxed::Box<Game> {
        let mut g = Game::blank();
        let mut b = crate::scene_boxes::Box::zeroed();
        b.c = [3.25, 1.4, -7.5]; b.h = [1.2, 0.9, 0.006]; b.yaw = 0.7;
        g.col.boxes.push(b);
        alloc::boxed::Box::new(g)
    }
    fn bits(v: &[f64]) -> Vec<u64> { v.iter().map(|x| x.to_bits()).collect() }
    /// from the TypeScript's own toUV, toUVIn, toWorld, normalOf under bun with EXACT.on
    #[test]
    fn a_pane_s_frame_is_the_typescript_s_bits() {
        let mut g = game_with_a_box();
        let (a, c) = (pane(0.0, GlassType::Annealed), pane(2.0, GlassType::Annealed));
        let x = [4.1, 2.0, -6.9];
        assert_eq!(bits(&glass_break_to_uv(&mut g, &a, x)), [0x3fd0de94a7bb39cd, 0x3fe3333333333334]);
        assert_eq!(bits(&glass_break_to_uv(&mut g, &c, x)), [0x3ff01a959f20e098, 0x3fe3333333333334]);
        assert_eq!(bits(&glass_break_to_uv_in(&mut g, &a, [9.0, 9.0, 9.0])), [0xbff31a9fbe76c8b4, 0x3feca7ef9db22d0e]);
        assert_eq!(bits(&glass_break_to_world(&mut g, &a, 0.3, -0.2, Some(0.01))), [0x400be31cd48a39f6, 0x3ff3333333333333, 0xc01ebe125a3e96d9]);
        assert_eq!(bits(&glass_break_to_world(&mut g, &c, 0.3, -0.2, None)), [0x400b8bceae4ba854, 0x3ff3333333333333, 0xc01d150a5ce6d07e]);
        assert_eq!(bits(&glass_break_normal_of(&mut g, &a)), [0x3fe49d6e694619b8, 0, 0x3fe87996529f9d92]);
        assert_eq!(bits(&glass_break_normal_of(&mut g, &c)), [0x3fe87996529f9d92, 0, 0xbfe49d6e694619b8]);
    }
    /// from the TypeScript's own addSite under bun with EXACT.on: a site, a second fitted to the room the first leaves, one with no room, a web's
    #[test]
    fn a_site_is_the_typescript_s_bits() {
        let mut g = game_with_a_box();
        let mut st = pane(0.0, GlassType::Annealed);
        assert_eq!(glass_break_add_site(&mut g, &mut st, 0.1, 0.2, 1.3, 3735928559.0), 0.0);
        let s = &st.sites[0];
        assert_eq!((s.n, s.seed, s.hits), (11.0, 14593470.0, 1.0));
        assert_eq!(bits(&[s.u, s.v, s.rot, s.rz, s.e, s.merge]), [0x3fb999999999999a, 0x3fc999999999999a, 0x4015960fbd000e3b, 0x3fc7868c8c14f6b1, 0x3ff4cccccccccccd, 0x3fdccccccccccccd]);
        assert_eq!(bits(&s.rho), [0x3fa083126e978d50, 0x3fb34395810624dd, 0x3fc34395810624dd, 0x3fe1333333333333]);
        assert_eq!(glass_break_add_site(&mut g, &mut st, 0.45, 0.2, 0.2, 305419896.0), 1.0);
        let s = &st.sites[1];
        assert_eq!((s.n, s.seed), (12.0, 1193046.0));
        assert_eq!(bits(&[s.rot, s.rz]), [0x3ffb3e55efd7e6e2, 0x3fc181e59dd410e4]);
        assert_eq!(bits(&s.rho), [0x3f989374bc6a7efa, 0x3facac083126e97a, 0x3fbcac083126e97a, 0x3fd999999999999a]);
        assert_eq!(glass_break_add_site(&mut g, &mut st, 0.12, 0.21, 1.0, 77.0), -1.0);
        let mut web = pane(0.0, GlassType::Laminated);
        assert_eq!(glass_break_add_site(&mut g, &mut web, -0.3, 0.1, 5.0, 3405705229.0), 0.0);
        let s = &web.sites[0];
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
