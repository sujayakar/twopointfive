//! src/game/glassbreak.ts lines 456 to 867
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::glass_pattern::{
    gp_chord_pt, gp_classify, gp_clip_rect, gp_dice_centre, gp_dice_clump, gp_dice_key, gp_dice_poly, gp_hull, gp_poly_area, gp_unfid, GpDice, GpFrag,
    GP_BANDS, GP_FRAGS_PER_SITE, GP_MAX_SECTORS, GP_REMAINDER,
};
use crate::scene_box_mirror::{PaneDice, PaneDiceClumpsItem, PaneRow, PaneRowPendingItem, PaneSite, PaneView, VIEW_BLOWN, VIEW_GIVES};
use crate::Num;

// ---------------------------------------------------------------- seams: what skeleton 5 still lacks
/// a box's row. Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of rows; when it has one this body is one line and no other line moves
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { todo!("needs: the table of Box rows: skeleton 5 has g.col.boxes as Vec<BoxId> and no Tab of Box") }

// ---------------------------------------------------------------- a pane's row, by its key
/// the row of pane bi, for a few lines. Nothing a release reaches deletes a row (reset and setType are a restart's and the panel's), so a row in hand stays
fn row<'g>(g: &'g mut Game, bi: f64) -> &'g mut PaneState { g.glass_break.panes.get_mut(Num(bi)).unwrap() }
/// `g.level.boxes[bi]`: undefined off the list (a negative, fractional or NaN index included). That list and `g.col.boxes` are one array in the TypeScript (game.ts:1219): the box at place bi
fn box_at(g: &Game, bi: f64) -> Option<BoxId> {
    let ix = bi as usize;
    if ix as f64 != bi { return None; }
    g.col.boxes.get(ix).copied()
}

fn gp_of(d: &Dice) -> GpDice { GpDice { u: d.u, v: d.v, cols: d.cols, rows: d.rows, seed: d.seed, hu: d.hu, hv: d.hv } }
/// `(seed ^ Math.imul(n, 0x9E3779B1)) >>> 0`
fn seed_mix(seed: f64, n: f64) -> f64 { (js::to_i32(seed) ^ m::imul(js::to_i32(n), js::to_i32(2654435761.0))) as u32 as f64 }
/// `(a ^ b) >>> 0`
fn xor_u32(a: f64, b: f64) -> f64 { (js::to_i32(a) ^ js::to_i32(b)) as u32 as f64 }
fn by_time(a: &Pending, b: &Pending) -> f64 { js::or(a.at - b.at, a.fid - b.fid) }
fn is_tooth(f: &GpFrag, tooth_area: f64) -> bool { f.area < tooth_area && f.frame.bottom + f.frame.left + f.frame.right + f.frame.top > 1e-4 }

fn crater_cause(c: PendingCause) -> GlassBreakReleaseCraterCause {
    match c {
        PendingCause::Blast => GlassBreakReleaseCraterCause::Blast,
        PendingCause::Hit => GlassBreakReleaseCraterCause::Hit,
        PendingCause::Dice => GlassBreakReleaseCraterCause::Dice,
        PendingCause::Support => GlassBreakReleaseCraterCause::Support,
        PendingCause::Rake => GlassBreakReleaseCraterCause::Rake,
    }
}
fn frag_cause(c: PendingCause) -> GlassBreakReleaseFragCause {
    match c {
        PendingCause::Blast => GlassBreakReleaseFragCause::Blast,
        PendingCause::Hit => GlassBreakReleaseFragCause::Hit,
        PendingCause::Dice => GlassBreakReleaseFragCause::Dice,
        PendingCause::Support => GlassBreakReleaseFragCause::Support,
        PendingCause::Rake => GlassBreakReleaseFragCause::Rake,
    }
}
fn crater_fragment_cause(c: GlassBreakReleaseCraterCause) -> GlassFragmentCause {
    match c {
        GlassBreakReleaseCraterCause::Blast => GlassFragmentCause::Blast,
        GlassBreakReleaseCraterCause::Hit => GlassFragmentCause::Hit,
        GlassBreakReleaseCraterCause::Dice => GlassFragmentCause::Dice,
        GlassBreakReleaseCraterCause::Support => GlassFragmentCause::Support,
        GlassBreakReleaseCraterCause::Rake => GlassFragmentCause::Rake,
    }
}
fn frag_fragment_cause(c: GlassBreakReleaseFragCause) -> GlassFragmentCause {
    match c {
        GlassBreakReleaseFragCause::Blast => GlassFragmentCause::Blast,
        GlassBreakReleaseFragCause::Hit => GlassFragmentCause::Hit,
        GlassBreakReleaseFragCause::Dice => GlassFragmentCause::Dice,
        GlassBreakReleaseFragCause::Support => GlassFragmentCause::Support,
        GlassBreakReleaseFragCause::Rake => GlassFragmentCause::Rake,
    }
}
/// what the shards part takes (shards.ts GlassFragmentLike): the same record
fn fragment_like(f: &GlassFragment) -> crate::shards::GlassFragmentLike {
    use crate::shards::{GlassFragmentLikeCause as C, GlassFragmentLikeKind as K};
    crate::shards::GlassFragmentLike {
        pane: f.pane,
        type_: f.type_,
        kind: match f.kind {
            GlassFragmentKind::Piece => K::Piece,
            GlassFragmentKind::Sheet => K::Sheet,
            GlassFragmentKind::Dagger => K::Dagger,
            GlassFragmentKind::Clump => K::Clump,
        },
        outline: f.outline.clone(),
        centre: f.centre,
        normal: f.normal,
        thickness: f.thickness,
        area: f.area,
        velocity: f.velocity,
        spin: f.spin,
        tint: f.tint,
        grain: f.grain,
        cause: match f.cause {
            GlassFragmentCause::Blast => C::Blast,
            GlassFragmentCause::Hit => C::Hit,
            GlassFragmentCause::Dice => C::Dice,
            GlassFragmentCause::Support => C::Support,
            GlassFragmentCause::Rake => C::Rake,
        },
        who: f.who,
        seed: f.seed,
    }
}
/// a pane's state as the box world's copy reads it (scene/boxMirror.ts PaneRow): in the TypeScript the state itself, handed over
fn pane_row(st: &PaneState) -> PaneRow {
    PaneRow {
        bi: st.bi,
        gone: st.gone,
        run: st.run,
        sites: st.sites.iter().map(|s| PaneSite { u: s.u, v: s.v, rot: s.rot, n: s.n, seed: s.seed, rho: s.rho.to_vec(), rz: s.rz, merge: s.merge }).collect(),
        open: st.open.clone(),
        pending: st.pending.iter().map(|p| PaneRowPendingItem { fid: p.fid }).collect(),
        dice: st.dice.as_ref().map(|d| PaneDice {
            u: d.u,
            v: d.v,
            cols: d.cols,
            rows: d.rows,
            seed: d.seed,
            hu: d.hu,
            hv: d.hv,
            next: d.next,
            out: d.out.clone(),
            clumps: d.clumps.iter().map(|c| PaneDiceClumpsItem { k: c.k, key: c.key }).collect(),
        }),
    }
}

// ---------------------------------------------------------------- a burst's view of the glass
/// the panes no longer whole, for the box world's copy (glassbreak.ts GlassBreak.each)
pub fn glass_break_each(g: &mut crate::game::Game, to: &mut dyn FnMut(&mut crate::game::Game, crate::scene_box_mirror::PaneRow)) {
    let mut at = 0usize;
    while let Some(key) = g.glass_break.panes.next_key(&mut at) {
        let row = match g.glass_break.panes.get(key) {
            Some(st) if st.gone || st.dice.is_some() || st.n_open > 0.0 || !st.pending.is_empty() => pane_row(st),
            _ => continue,
        };
        to(g, row);
    }
}

/// for the length of `fn_`, every solid walk and ray also passes glass this burst sees none of (glassbreak.ts GlassBreak.seeThrough)
pub fn glass_break_see_through<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, view: crate::glassbreak::GlassBreakSeeThroughView, from: V3, size: crate::glassbreak::GlassBlastOpts, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let statics = g.engine.world.glass_statics.clone();
    for bi in statics { glass_break_sky_side(g, bi); }
    let kind = match view {
        GlassBreakSeeThroughView::Blown => GlassBreakViewOfKind::Blown,
        GlassBreakSeeThroughView::Wary => GlassBreakViewOfKind::Wary,
        GlassBreakSeeThroughView::Counted => GlassBreakViewOfKind::Counted,
    };
    let was_of = g.glass_break.view_of.take();
    g.glass_break.view_of = Some(GlassBreakViewOf { kind, from, size, rows: None });
    crate::scene_collision::glass_viewed(g);
    let out = fn_(g);
    g.glass_break.view_of = was_of;
    crate::scene_collision::glass_viewed(g);
    out
}
/// the rule the TypeScript's `also` closure held (glassbreak.ts:466), from the view in hand, which holds the same three things: what it says of pane bi at x
fn glass_view_also(g: &mut Game, bi: f64, x: V3) -> bool {
    let (kind, from, size) = match g.glass_break.view_of.as_ref() { Some(v) => (v.kind, v.from, v.size), None => return false };
    match kind {
        GlassBreakViewOfKind::Blown => glass_break_going_at(g, bi, x),
        GlassBreakViewOfKind::Wary => glass_break_would_give(g, bi, from, Some(size), None),
        GlassBreakViewOfKind::Counted => glass_break_would_give(g, bi, from, Some(size), Some(x)),
    }
}

/// the view in hand as rows, worked out at the first read and kept for the judgment (glassbreak.ts GlassBreak.view)
pub fn glass_break_view(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneView> {
    let (kind, from, size, rows) = match g.glass_break.view_of.as_ref() { Some(v) => (v.kind, v.from, v.size, v.rows.clone()), None => return None };
    if rows.is_some() { return rows; }
    let rows = if kind == GlassBreakViewOfKind::Blown {
        Some(PaneView { kind: VIEW_BLOWN, bi: Vec::new(), at: Vec::new() })
    } else {
        glass_break_gives(g, from, size, kind == GlassBreakViewOfKind::Counted)
    };
    if let Some(v) = g.glass_break.view_of.as_mut() { v.rows = rows.clone(); }
    rows
}

/// wouldGive for every sheet of the level at once, as the rows of a view (glassbreak.ts GlassBreak.gives)
pub fn glass_break_gives(g: &mut crate::game::Game, at: V3, opts: crate::glassbreak::GlassBlastOpts, sure: bool) -> Option<crate::scene_box_mirror::PaneView> {
    if !glass_break_live(g) { return None; }
    let mut rows = PaneView { kind: VIEW_GIVES, bi: Vec::new(), at: Vec::new() };
    let radius = opts.radius.unwrap_or(g.k.glassbreak.blast_m);
    let power = opts.power.unwrap_or(1.0);
    if !(radius > 0.0) || !(power > 0.0) { return Some(rows); }
    let mut q: V3 = [0.0, 0.0, 0.0];
    for bi in crate::glass::glass_panes(g) {
        let b = box_at(g, bi);
        if !crate::glass::is_sheet(g, b) { continue; }
        let reach = glass_break_give(g, bi, b.unwrap(), at, radius, power, sure, &mut q);
        if !reach.is_nan() {
            rows.bi.push(bi);
            rows.at.push(q[0]);
            rows.at.push(q[1]);
            rows.at.push(q[2]);
            rows.at.push(reach);
        }
    }
    Some(rows)
}

/// how a sheet gives to a blast of this size: the reach round the nearest point, below nought = the whole pane, NaN = it does not give (glassbreak.ts GlassBreak.give)
pub fn glass_break_give(g: &mut crate::game::Game, bi: f64, b: BoxId, at: V3, radius: f64, power: f64, sure: bool, q: &mut V3) -> f64 {
    let p = nearest_on(g, b, at);
    let k = power * (1.0 - v3::dist(p, at) / radius);
    q[0] = p[0];
    q[1] = p[1];
    q[2] = p[2];
    let (dice_k, sheet_k, tooth_k) = (g.k.glassbreak.blast_dice_k, g.k.glassbreak.blast_sheet_k, g.k.glassbreak.tooth_k);
    if k < m::min_n(&[dice_k, sheet_k, if sure { tooth_k } else { BLAST_MIN_K }]) { return f64::NAN; }
    let type_ = match g.glass_break.types.get(Num(bi)) { Some(t) => *t, None => glass_break_type_of(g, b, Some(bi)) };
    if type_ == GlassType::Tempered { return if k > dice_k { -1.0 } else { f64::NAN }; }
    if type_ == GlassType::Laminated { return if k > sheet_k { -1.0 } else { f64::NAN }; }
    if !sure { return if k > BLAST_MIN_K { -1.0 } else { f64::NAN }; }
    let reach = blast_reach(k);
    if k >= tooth_k && reach >= 0.0 { reach } else { f64::NAN }
}

/// is the glass of pane bi at x on its way out (glassbreak.ts GlassBreak.goingAt)
pub fn glass_break_going_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool {
    match g.glass_break.panes.get(Num(bi)) {
        None => return false,
        Some(st) => {
            if st.gone || st.dice.is_some() { return true; }
            if st.pending.is_empty() { return false; }
        }
    }
    let [u, v] = glass_break_to_uv(g, x, bi);
    let st = row(g, bi);
    let fid = gp_classify(&st.sites, st.run, u, v);
    if fid < 0.0 { return false; }
    let at = gp_unfid(fid);
    st.pending.iter().any(|p| p.fid == fid || (at.b == 0.0 && gp_unfid(p.fid).b == 0.0 && gp_unfid(p.fid).i == at.i))
}

/// would a blast of this size at `at` take pane bi out of the way (glassbreak.ts GlassBreak.wouldGive)
pub fn glass_break_would_give(g: &mut crate::game::Game, bi: f64, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>, x: Option<V3>) -> bool {
    if !glass_break_live(g) { return false; }
    let opts = opts.unwrap_or_default();
    if let Some(st) = g.glass_break.panes.get(Num(bi)) { if st.gone || st.dice.is_some() { return true; } }
    let b = box_at(g, bi);
    if !crate::glass::is_sheet(g, b) { return false; }
    let radius = opts.radius.unwrap_or(g.k.glassbreak.blast_m);
    let power = opts.power.unwrap_or(1.0);
    if !(radius > 0.0) || !(power > 0.0) { return false; }
    let mut q: V3 = [0.0, 0.0, 0.0];
    let reach = glass_break_give(g, bi, b.unwrap(), at, radius, power, x.is_some(), &mut q);
    reach < 0.0 || (x.is_some() && v3::dist(x.unwrap(), q) <= reach)
}

/// a stun charge went off at `at`: a blast at bangPower (glassbreak.ts GlassBreak.bang)
pub fn glass_break_bang(g: &mut crate::game::Game, at: V3, who: f64) -> f64 {
    let opts = GlassBlastOpts { radius: Some(g.k.glassbreak.blast_m), power: Some(g.k.glassbreak.bang_power), who: Some(who) };
    glass_break_blast(g, at, Some(opts))
}

// ---------------------------------------------------------------- tempered: the dice
/// toughened glass starts to come down; the clump under the blow goes with this call (glassbreak.ts GlassBreak.startDice)
pub fn glass_break_start_dice(g: &mut crate::game::Game, u: f64, v: f64, seed: f64, why: &str, bi: f64) {
    let (hu, hv) = {
        let st = row(g, bi);
        if st.dice.is_some() || st.gone { return; }
        (st.hu, st.hv)
    };
    let clump_m = g.k.glassbreak.clump_m;
    let cols = m::max(1.0, m::round(2.0 * hu / clump_m));
    let rows = m::max(1.0, m::round(2.0 * hv / clump_m));
    let mut d = Dice { u, v, cols, rows, seed: (js::to_u32(seed) >> 8) as f64, hu, hv, t0: g.time, clumps: Vec::new(), next: 0.0, out: vec![0u8; (cols * rows) as usize] };
    let gp = gp_of(&d);
    let (mut r, mut clumps) = (0.0, Vec::new());
    while r < rows {
        let mut c = 0.0;
        while c < cols {
            clumps.push(Clump { k: r * cols + c, c, r, key: gp_dice_key(gp, c, r) });
            c += 1.0;
        }
        r += 1.0;
    }
    js::sort_by(&mut clumps, |a, b| js::or(a.key - b.key, a.k - b.k));
    d.clumps = clumps;
    let n_clumps = d.clumps.len();
    let k0 = gp_dice_clump(gp, u, v);
    let key0 = d.clumps.iter().find(|c| c.k == k0).unwrap().key;
    row(g, bi).dice = Some(d);
    g.stats.glassbreak.diced += 1;
    glass_break_note(g, &format!("pane {} DICES ({})", bi, why));
    glass_break_plog(g, &format!("dices ({}): {} clumps", why, n_clumps), bi);
    loop {
        let d = row(g, bi).dice.as_ref().unwrap();
        let at = d.next as usize;
        if at >= d.clumps.len() || !(d.clumps[at].key <= key0 + DICE_TIE) { break; }
        let c = d.clumps[at];
        glass_break_release_clump(g, c, bi);
        row(g, bi).dice.as_mut().unwrap().next += 1.0;
    }
    let d = row(g, bi).dice.as_ref().unwrap();
    if (d.next as usize) >= d.clumps.len() { glass_break_dice_done(g, bi); }
}

/// every clump is out (glassbreak.ts GlassBreak.diceDone)
pub fn glass_break_dice_done(g: &mut crate::game::Game, bi: f64) {
    let st = row(g, bi);
    st.gone = true;
    st.walkable = true;
    let exterior = st.exterior;
    crate::scene_collision::wrote(g);
    if exterior { g.glass_break.gust_dirty = true; }
}

/// the crazed sheet stands `hang` seconds, then every clump leaves on the one step: GLASSREAL.drop, settled (glassbreak.ts GlassBreak.stepDice)
pub fn glass_break_step_dice(g: &mut crate::game::Game, bi: f64) {
    let (t0, walkable) = { let st = row(g, bi); (st.dice.as_ref().unwrap().t0, st.walkable) };
    if g.time - t0 >= g.k.glassreal.hang {
        glass_break_drop_all(g, bi);
    } else if walkable {
        row(g, bi).walkable = false;
        crate::scene_collision::wrote(g);
    }
}

/// a clump lets go (glassbreak.ts GlassBreak.releaseClump). `D` is the row's own dice: the skeleton dropped the parameter, one holder now
pub fn glass_break_release_clump(g: &mut crate::game::Game, c_2: crate::glassbreak::Clump, bi: f64) {
    let gp = {
        let d = row(g, bi).dice.as_mut().unwrap();
        d.out[c_2.k as usize] = 1;
        gp_of(d)
    };
    g.stats.glassbreak.clumps += 1;
    g.stats.glassbreak.dropped += 1;
    let pts = gp_dice_poly(gp, c_2.c, c_2.r);
    if pts.len() < 3 { return; }
    let pa = gp_poly_area(&pts);
    let (area, cen) = (pa.area, pa.cen);
    if area < 1e-8 { return; }
    let n = glass_break_normal_of(g, bi);
    let (st_key, last_dir, type_, ht, box_, last_who) = { let st = row(g, bi); (st.key, st.last_dir, st.type_, st.ht, st.box_, st.last_who) };
    let out_sign = if v3::dot(n, last_dir) >= 0.0 { 1.0 } else { -1.0 };
    let nn = v3::scale(n, out_sign);
    let h1 = crate::debris::hash01(st_key * 7919.0 + c_2.k * 131.0);
    let h2 = crate::debris::hash01(st_key * 104729.0 + c_2.k * 193.0);
    let h3 = crate::debris::hash01(c_2.k * 7.0 + st_key);
    let rad = m::max(1e-3, m::hyp2(cen[0] - gp.u, cen[1] - gp.v));
    let spread = glass_break_dir_uv(g, (cen[0] - gp.u) / rad, (cen[1] - gp.v) / rad, bi);
    let (out, spin, grain) = (g.k.glassbreak.out, g.k.glassbreak.spin, g.k.glassbreak.dice_grain);
    let velocity = v3::add(v3::add(v3::scale(nn, out * (0.3 + 0.9 * h1)), v3::scale(spread, 0.5 * h2)), [0.0, -0.3 - 0.5 * h3, 0.0]);
    let f = GlassFragment {
        pane: bi,
        type_,
        kind: GlassFragmentKind::Clump,
        outline: glass_break_outline(g, &pts, out_sign, bi),
        centre: glass_break_to_world(g, cen[0], cen[1], None, bi),
        normal: nn,
        thickness: 2.0 * ht,
        area,
        velocity,
        spin: [(h1 - 0.5) * spin, (h2 - 0.5) * 2.0, (h3 - 0.5) * spin],
        tint: seam_box(g, box_).albedo,
        grain,
        cause: GlassFragmentCause::Dice,
        who: last_who,
        seed: seed_mix(gp.seed, c_2.k + 1.0),
    };
    glass_break_emit(g, &f, c_2.k, bi);
}

/// GLASSREAL.drop: every clump still standing leaves on this step, handed on in square blocks (glassbreak.ts GlassBreak.dropAll)
pub fn glass_break_drop_all(g: &mut crate::game::Game, bi: f64) {
    let (gp, n_clumps, next0) = { let d = row(g, bi).dice.as_ref().unwrap(); (gp_of(d), d.clumps.len() as f64, d.next) };
    let left = n_clumps - next0;
    let mut mm = 1.0;
    let blocks = m::max(1.0, g.k.glassreal.blocks);
    while m::ceil(gp.cols / mm) * m::ceil(gp.rows / mm) > blocks { mm += 1.0; }
    let du = 2.0 * gp.hu / gp.cols;
    let dv = 2.0 * gp.hv / gp.rows;
    let bc = m::ceil(gp.cols / mm);
    let br = m::ceil(gp.rows / mm);
    let size = (bc * br) as usize;
    let (mut n, mut su, mut sv) = (vec![0u16; size], vec![0.0f64; size], vec![0.0f64; size]);
    {
        let d = g.glass_break.panes.get_mut(Num(bi)).unwrap().dice.as_mut().unwrap();
        while (d.next as usize) < d.clumps.len() {
            let c = d.clumps[d.next as usize];
            d.out[c.k as usize] = 1;
            g.stats.glassbreak.clumps += 1;
            g.stats.glassbreak.dropped += 1;
            let b = (m::floor(c.r / mm) * bc + m::floor(c.c / mm)) as usize;
            let p = gp_dice_centre(gp, c.c, c.r);
            n[b] = n[b].wrapping_add(1);
            su[b] += p[0];
            sv[b] += p[1];
            d.next += 1.0;
        }
    }
    let nrm = glass_break_normal_of(g, bi);
    let (st_key, last_dir, type_, ht, box_, last_who) = { let st = row(g, bi); (st.key, st.last_dir, st.type_, st.ht, st.box_, st.last_who) };
    let out_sign = if v3::dot(nrm, last_dir) >= 0.0 { 1.0 } else { -1.0 };
    let nn = v3::scale(nrm, out_sign);
    let (out, spin, grain) = (g.k.glassbreak.out, g.k.glassbreak.spin, g.k.glassbreak.dice_grain);
    for bi_ in 0..n.len() {
        if n[bi_] == 0 { continue; }
        let b = bi_ as f64;
        let count = n[bi_] as f64;
        let c0 = js::rem(b, bc) * mm;
        let r0 = m::floor(b / bc) * mm;
        let u0 = -gp.hu + c0 * du;
        let u1 = m::min(gp.hu, u0 + mm * du);
        let v0 = -gp.hv + r0 * dv;
        let v1 = m::min(gp.hv, v0 + mm * dv);
        let cu = su[bi_] / count;
        let cv = sv[bi_] / count;
        let h1 = crate::debris::hash01(st_key * 7919.0 + b * 131.0);
        let h3 = crate::debris::hash01(b * 7.0 + st_key);
        let near = m::max(0.0, 1.0 - m::hyp2(cu - gp.u, cv - gp.v) / 0.6);
        let f = GlassFragment {
            pane: bi,
            type_,
            kind: GlassFragmentKind::Clump,
            outline: glass_break_outline(g, &[[u0, v0], [u1, v0], [u1, v1], [u0, v1]], out_sign, bi),
            centre: glass_break_to_world(g, cu, cv, None, bi),
            normal: nn,
            thickness: 2.0 * ht,
            area: count * du * dv,
            velocity: v3::add(v3::scale(nn, out * (0.1 + 0.25 * h1 + 0.8 * near)), [0.0, -0.3 - 0.5 * h3, 0.0]),
            spin: [(h1 - 0.5) * spin, 0.0, (h3 - 0.5) * spin],
            tint: seam_box(g, box_).albedo,
            grain,
            cause: GlassFragmentCause::Dice,
            who: last_who,
            seed: seed_mix(gp.seed, b + 1.0),
        };
        glass_break_emit(g, &f, b, bi);
    }
    glass_break_plog(g, &format!("drops: {} clumps as blocks of {} × {}", left, mm, mm), bi);
    glass_break_dice_done(g, bi);
}

/// a laminated sheet leaves its frame whole (glassbreak.ts GlassBreak.releaseSheet)
pub fn glass_break_release_sheet(g: &mut crate::game::Game, away: V3, k: f64, bi: f64) {
    {
        let st = row(g, bi);
        if st.gone { return; }
        st.gone = true;
        st.walkable = true;
    }
    crate::scene_collision::wrote(g);
    let (exterior, st_key, type_, ht, hu, hv, box_, last_who) = {
        let st = row(g, bi);
        st.pending.clear();
        (st.exterior, st.key, st.type_, st.ht, st.hu, st.hv, st.box_, st.last_who)
    };
    g.stats.glassbreak.sheets += 1;
    g.stats.glassbreak.dropped += 1;
    if exterior { g.glass_break.gust_dirty = true; }
    let n = glass_break_normal_of(g, bi);
    let out_sign = if v3::dot(n, away) >= 0.0 { 1.0 } else { -1.0 };
    let nn = v3::scale(n, out_sign);
    let pts = [[-hu, -hv], [hu, -hv], [hu, hv], [-hu, hv]];
    glass_break_note(g, &format!("pane {}: the laminated sheet leaves its frame", bi));
    glass_break_plog(g, "sheet blown out", bi);
    let (c, albedo) = { let b = seam_box(g, box_); (b.c, b.albedo) };
    let blast_out = g.k.glassbreak.blast_out;
    let f = GlassFragment {
        pane: bi,
        type_,
        kind: GlassFragmentKind::Sheet,
        outline: glass_break_outline(g, &pts, out_sign, bi),
        centre: v3::copy(c),
        normal: nn,
        thickness: 2.0 * ht,
        area: 4.0 * hu * hv,
        velocity: v3::add(v3::scale(nn, blast_out * k * 0.6), [0.0, 0.4, 0.0]),
        spin: [1.5 * k, 0.0, 0.4],
        tint: albedo,
        grain: 0.0,
        cause: GlassFragmentCause::Blast,
        who: last_who,
        seed: crate::glass::point_seed(st_key, c),
    };
    glass_break_emit(g, &f, 0.0, bi);
}

// ---------------------------------------------------------------- pieces letting go
/// a pane-local direction in the world (glassbreak.ts GlassBreak.dirUV)
pub fn glass_break_dir_uv(g: &mut crate::game::Game, du: f64, dv: f64, bi: f64) -> V3 {
    let a = glass_break_to_world(g, 0.0, 0.0, None, bi);
    let b = glass_break_to_world(g, du, dv, None, bi);
    v3::sub(b, a)
}

/// pane-local polygon to the world outline, anticlockwise seen from the side the fragment leaves by (glassbreak.ts GlassBreak.outline)
pub fn glass_break_outline(g: &mut crate::game::Game, pts: &[[f64; 2]], out_sign: f64, bi: f64) -> Vec<V3> {
    let ccw_from_plus_w = row(g, bi).u_axis == 0.0;
    let keep = (out_sign > 0.0) == ccw_from_plus_w;
    let mut out = Vec::with_capacity(pts.len());
    if keep {
        for p in pts { out.push(glass_break_to_world(g, p[0], p[1], None, bi)); }
    } else {
        for p in pts.iter().rev() { out.push(glass_break_to_world(g, p[0], p[1], None, bi)); }
    }
    out
}

/// a site's crater goes, as one fragment (glassbreak.ts GlassBreak.releaseCrater)
pub fn glass_break_release_crater(g: &mut crate::game::Game, i: f64, cause: crate::glassbreak::GlassBreakReleaseCraterCause, bi: f64) -> f64 {
    let (s_, hu, hv) = {
        let st = row(g, bi);
        let s_ = st.sites[i as usize];
        let mut any = false;
        let mut s = 0.0;
        while s < s_.n {
            let fid = (i * GP_FRAGS_PER_SITE + s) as usize;
            if st.open[fid] == 0 {
                st.open[fid] = 1;
                st.n_open += 1.0;
                any = true;
            }
            s += 1.0;
        }
        if !any { return 0.0; }
        (s_, st.hu, st.hv)
    };
    g.stats.glassbreak.dropped += 1;
    let mut chords = Vec::new();
    let mut s = 0.0;
    while s < s_.n {
        chords.push(gp_chord_pt(s_, s, 0.0));
        s += 1.0;
    }
    let hull = gp_clip_rect(&gp_hull(&chords), hu, hv);
    if hull.len() < 3 { return 0.0; }
    let pa = gp_poly_area(&hull);
    let (area, cen) = (pa.area, pa.cen);
    let n = glass_break_normal_of(g, bi);
    let (last_dir, type_, ht, box_, last_who) = { let st = row(g, bi); (st.last_dir, st.type_, st.ht, st.box_, st.last_who) };
    let out_sign = if v3::dot(n, last_dir) >= 0.0 { 1.0 } else { -1.0 };
    let nn = v3::scale(n, out_sign);
    let h = crate::debris::hash01(s_.seed);
    glass_break_plog(g, &format!("crater {} out", i), bi);
    let f = GlassFragment {
        pane: bi,
        type_,
        kind: GlassFragmentKind::Piece,
        outline: glass_break_outline(g, &hull, out_sign, bi),
        centre: glass_break_to_world(g, cen[0], cen[1], None, bi),
        normal: nn,
        thickness: 2.0 * ht,
        area,
        velocity: v3::add(v3::scale(nn, 2.5 + 2.0 * h), [0.0, -0.2, 0.0]),
        spin: [0.0, 0.0, 0.0],
        tint: seam_box(g, box_).albedo,
        grain: 0.004,
        cause: crater_fragment_cause(cause),
        who: last_who,
        seed: xor_u32(s_.seed, 0xC2A7E5 as f64),
    };
    glass_break_emit(g, &f, i * GP_FRAGS_PER_SITE, bi);
    glass_break_after_release(g, bi);
    1.0
}

/// a piece lets go (glassbreak.ts GlassBreak.releaseFrag)
pub fn glass_break_release_frag(g: &mut crate::game::Game, f: &crate::glass_pattern::GpFrag, cause: crate::glassbreak::GlassBreakReleaseFragCause, from: Option<V3>, k: f64, bi: f64) {
    let fid = f.fid as usize;
    let (st_key, last_dir, type_, ht, box_, last_who) = {
        let st = row(g, bi);
        if st.open[fid] != 0 { return; }
        st.open[fid] = 1;
        st.n_open += 1.0;
        (st.key, st.last_dir, st.type_, st.ht, st.box_, st.last_who)
    };
    g.stats.glassbreak.dropped += 1;
    let n = glass_break_normal_of(g, bi);
    let cw = glass_break_to_world(g, f.cen[0], f.cen[1], None, bi);
    let mut dir_out = last_dir;
    if let Some(from) = from {
        let d = v3::sub(cw, from);
        if v3::len(d) > 1e-6 { dir_out = v3::normalize(d); }
    }
    let out_sign = if v3::dot(n, dir_out) >= 0.0 { 1.0 } else { -1.0 };
    let nn = v3::scale(n, out_sign);
    let h1 = crate::debris::hash01(st_key * 7919.0 + f.fid * 131.0);
    let h2 = crate::debris::hash01(st_key * 104729.0 + f.fid * 193.0);
    let h3 = crate::debris::hash01(f.fid * 7.0 + st_key);
    let (out, spin, blast_out) = (g.k.glassbreak.out, g.k.glassbreak.spin, g.k.glassbreak.blast_out);
    let speed = match cause {
        GlassBreakReleaseFragCause::Blast => blast_out * k * (0.6 + 0.8 * h3),
        GlassBreakReleaseFragCause::Hit => out * (0.8 + 1.2 * h3),
        GlassBreakReleaseFragCause::Rake => out * (1.2 + 0.8 * h3),
        _ => out * (0.15 + 0.5 * h3),
    };
    let aimed = (cause == GlassBreakReleaseFragCause::Blast || cause == GlassBreakReleaseFragCause::Rake) && from.is_some();
    let base = if aimed { v3::scale(v3::normalize(v3::add(v3::scale(dir_out, 0.7), v3::scale(nn, 0.5))), speed) } else { v3::scale(nn, speed) };
    let velocity = v3::add(base, [0.0, -0.2 - 0.6 * h1, 0.0]);
    glass_break_plog(g, &format!("piece {}:{}:{} out ({})", f.i, f.b, f.s, cause.as_str()), bi);
    let frag = GlassFragment {
        pane: bi,
        type_,
        kind: if f.b >= 3.0 { GlassFragmentKind::Dagger } else { GlassFragmentKind::Piece },
        outline: glass_break_outline(g, &f.pts, out_sign, bi),
        centre: cw,
        normal: nn,
        thickness: 2.0 * ht,
        area: f.area,
        velocity,
        spin: [(h1 - 0.5) * spin, (h2 - 0.5) * 2.0, (h3 - 0.5) * spin],
        tint: seam_box(g, box_).albedo,
        grain: 0.0,
        cause: frag_fragment_cause(cause),
        who: last_who,
        seed: xor_u32(crate::glass::point_seed(st_key, cw), f.fid),
    };
    glass_break_emit(g, &frag, f.fid, bi);
    glass_break_after_release(g, bi);
}

/// the walk rule asked again after a piece went (glassbreak.ts GlassBreak.afterRelease)
pub fn glass_break_after_release(g: &mut crate::game::Game, bi: f64) {
    let w = glass_break_walkable_now(g, bi);
    if w != row(g, bi).walkable {
        row(g, bi).walkable = w;
        crate::scene_collision::wrote(g);
    }
    if row(g, bi).exterior { g.glass_break.gust_dirty = true; }
}

/// a fragment has let go: the break heard, then the consumer or the stand-in (glassbreak.ts GlassBreak.emit)
pub fn glass_break_emit(g: &mut crate::game::Game, f: &crate::glassbreak::GlassFragment, key: f64, bi: f64) {
    let now = g.time;
    let crack_gap = g.k.glassbreak.crack_gap;
    let (sound_at, st_key, box_, last_who) = { let st = row(g, bi); (st.sound_at, st.key, st.box_, st.last_who) };
    if f.cause != GlassFragmentCause::Hit && now - sound_at >= crack_gap {
        row(g, bi).sound_at = now;
        let dicing = f.kind == GlassFragmentKind::Clump || f.kind == GlassFragmentKind::Sheet;
        let rate = if dicing { 0.7 } else { 1.25 + 0.3 * crate::debris::hash01(key + st_key) };
        crate::audio_iface::game_audio_like_play(g, "glassCrack", Some(f.centre), Some(if dicing { 1.0 } else { 0.75 }), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
        crate::glass::glass_event(g, box_, f.centre, 1.0, last_who);
    }
    if g.glass_break.on_release.is_some() {
        g.stats.glassbreak.hooked += 1;
        crate::shards::glass_shards_on_release(g, &fragment_like(f));
        return;
    }
    glass_break_stand_in(g, f, key, bi);
}

/// no shards consumer attached: thin Debris boxes over the fragment and a burst of pale chips (glassbreak.ts GlassBreak.standIn)
pub fn glass_break_stand_in(g: &mut crate::game::Game, f: &crate::glassbreak::GlassFragment, key: f64, bi: f64) {
    let mut uv = Vec::with_capacity(f.outline.len());
    for p in &f.outline { uv.push(glass_break_to_uv(g, *p, bi)); }
    let cen = gp_poly_area(&uv).cen;
    let grains = f.grain > 0.0;
    let sheet = f.kind == GlassFragmentKind::Sheet;
    let n_sh = if grains {
        if f.kind == GlassFragmentKind::Clump && js::rem(key, g.k.glassbreak.crumb_every) == 0.0 { 1.0 } else { 0.0 }
    } else if sheet {
        3.0
    } else {
        m::max(1.0, m::min(3.0, m::round(f.area / 0.07)))
    };
    let normal = glass_break_normal_of(g, bi);
    let (st_key, u_axis, ht, box_) = { let st = row(g, bi); (st.key, st.u_axis, st.ht, st.box_) };
    let out_sign = if v3::dot(normal, f.normal) >= 0.0 { 1.0 } else { -1.0 };
    let n_uv = uv.len() as f64;
    let mut k = 0.0;
    while k < n_sh {
        let h1 = crate::debris::hash01(st_key * 7919.0 + key * 131.0 + k * 17.0);
        let h2 = crate::debris::hash01(st_key * 104729.0 + key * 193.0 + k * 29.0);
        let vtx = uv.get(js::rem(k * m::max(1.0, m::floor(n_uv / n_sh)), n_uv) as usize).copied().unwrap_or(cen);
        let pu = cen[0] * (if n_sh == 1.0 { 1.0 } else { 0.6 }) + vtx[0] * (if n_sh == 1.0 { 0.0 } else { 0.4 });
        let pv = cen[1] * (if n_sh == 1.0 { 1.0 } else { 0.6 }) + vtx[1] * (if n_sh == 1.0 { 0.0 } else { 0.4 });
        let side = m::sqrt(f.area / n_sh) * (if grains { 0.35 } else { 0.92 });
        let su = side * (0.8 + 0.4 * h1);
        let sv = side * (0.8 + 0.4 * h2);
        let hh: V3 = if u_axis == 0.0 { [su / 2.0, sv / 2.0, m::max(0.006, ht)] } else { [m::max(0.006, ht), sv / 2.0, su / 2.0] };
        let p = crate::scene_boxes::MakeBoxP {
            c: glass_break_to_world(g, pu, pv, Some(out_sign * 0.02), bi),
            h: hh,
            yaw: Some(seam_box(g, box_).yaw),
            albedo: Some(v3::copy(g.k.glassbreak.shard_albedo)),
            flags: Some(crate::scene_boxes::box_flag().dynamic),
            ..Default::default()
        };
        let bx = crate::scene_boxes::make_box(g, &p);
        let ch = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Shard, bx, v3::copy(f.velocity), v3::copy(f.spin), Some(f.who), None);
        if let Some(ch) = ch {
            let chunk_box = g.tab.debris_chunk[ch].box_;
            seam_box(g, chunk_box).rough = Some(0.05);
            g.stats.glassbreak.shards += 1;
        } else {
            g.stats.glassbreak.refused += 1;
        }
        k += 1.0;
    }
    if crate::particle_fx::gpu_fx(g) {
        let n_chips = m::round(m::max(if grains { 3.0 } else { g.k.glassbreak.chips_min }, f.area * (if grains { g.k.glassbreak.dice_chips_per_m2 } else { g.k.glassbreak.chips_per_m2 })));
        let color = v3::copy(g.k.glassbreak.chip_color);
        let o = crate::render_particles::EmitOpts {
            shape: Some(crate::render_particles::EmitShape::Cone),
            speed: 1.1,
            speed_jit: Some(0.8),
            spread: Some(1.2),
            life: 30.0,
            life_jit: Some(0.4),
            size: if grains { 0.012 } else { 0.008 },
            size_jit: Some(0.6),
            aspect: Some(1.4),
            pos_jit: Some(m::sqrt(f.area) * 0.45),
            color: Some(color),
            ..Default::default()
        };
        crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, f.centre, f.normal, n_chips, o);
        g.stats.glassbreak.chips += n_chips as i32;
    }
}

// ---------------------------------------------------------------- support
/// which intact pieces of an annealed pane are held, grown outward from the frame and the uncut sheet (glassbreak.ts GlassBreak.heldSet)
pub fn glass_break_held_set(g: &mut crate::game::Game, bi: f64) -> JsSet<crate::Num> {
    let frags = glass_break_frags_of(g, bi);
    let (side, top, wedged) = (g.k.glassbreak.grip.side, g.k.glassbreak.grip.top, g.k.glassbreak.grip.wedged);
    let (stand_ny, balance_tol, stand_foot, hang_area) = (g.k.glassbreak.stand_ny, g.k.glassbreak.balance_tol, g.k.glassbreak.stand_foot, g.k.glassbreak.hang_area);
    let st = row(g, bi);
    let mut by: JsMap<Num, usize> = JsMap::new();
    for (i, f) in frags.iter().enumerate() {
        if st.open[f.fid as usize] == 0 { by.set(Num(f.fid), i); }
    }
    let order: Vec<usize> = by.values().copied().collect();
    let mut held: JsSet<Num> = JsSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for &i in &order {
            let f = &frags[i];
            if held.has(Num(f.fid)) { continue; }
            let mut ok = false;
            let (mut u0, mut u1) = (f64::INFINITY, f64::NEG_INFINITY);
            let mut grip = (f.frame.left + f.frame.right + f.frame.bottom) * side + f.frame.top * top;
            if let Some(foot) = f.foot {
                u0 = foot[0];
                u1 = foot[1];
            }
            for n in &f.nb {
                if n.fid == GP_REMAINDER {
                    ok = true;
                    break;
                }
                if !by.has(Num(n.fid)) || !held.has(Num(n.fid)) { continue; }
                grip += n.len * wedged;
                if n.ny < -stand_ny {
                    u0 = m::min(u0, n.u0);
                    u1 = m::max(u1, n.u1);
                }
            }
            if !ok && u1 > u0 { ok = f.cen[0] >= u0 - balance_tol && f.cen[0] <= u1 + balance_tol && u1 - u0 >= stand_foot * (f.u_max - f.u_min); }
            if !ok { ok = grip > 0.0 && f.area <= hang_area * grip; }
            if ok {
                held.add(Num(f.fid));
                changed = true;
            }
        }
    }
    held
}

/// queues what the last change left unheld, staggered (glassbreak.ts GlassBreak.supportPass)
pub fn glass_break_support_pass(g: &mut crate::game::Game, bi: f64) {
    { let st = row(g, bi); if st.type_ != GlassType::Annealed || st.n_open == 0.0 { return; } }
    let held = glass_break_held_set(g, bi);
    let mut n = 0;
    let (stagger_per_m, stagger_jit, now) = (g.k.glassbreak.stagger_per_m, g.k.glassbreak.stagger_jit, g.time);
    for f in glass_break_frags_of(g, bi) {
        let st = row(g, bi);
        if st.open[f.fid as usize] != 0 || held.has(Num(f.fid)) || st.pending.iter().any(|p| p.fid == f.fid) { continue; }
        let d = m::hyp2(f.cen[0] - st.last_uv[0], f.cen[1] - st.last_uv[1]);
        let at = now + d * stagger_per_m + stagger_jit * crate::debris::hash01(st.key * 31.0 + f.fid);
        st.pending.push(Pending { fid: f.fid, at, cause: PendingCause::Support, from: None, k: 1.0 });
        n += 1;
    }
    if n != 0 {
        glass_break_plog(g, &format!("{} pieces unheld", n), bi);
        js::sort_by(&mut row(g, bi).pending, by_time);
    }
}

// ---------------------------------------------------------------- per frame
/// per frame: the gusts, then every pane's due pieces, its support pass and its raking (glassbreak.ts GlassBreak.update)
pub fn glass_break_update(g: &mut crate::game::Game, _dt: f64) {
    g.glass_break.released_this_frame = 0.0;
    if g.glass_break.gusts.len() > 0 || g.glass_break.gust_dirty {
        glass_break_update_gusts(g);
        g.glass_break.gust_dirty = g.glass_break.gusts.len() > 0;
    }
    if !glass_break_live(g) { return; }
    let now = g.time;
    if g.glass_break.panes.len() == 0 { return; }
    crate::scene_collision::glass_begin(g);
    let mut at = 0usize;
    while let Some(key) = g.glass_break.panes.next_key(&mut at) {
        if g.glass_break.panes.get(key).map_or(true, |s| s.gone) { continue; }
        update_pane(g, key.0, now);
    }
    crate::scene_collision::glass_end(g);
}
fn update_pane(g: &mut Game, bi: f64, now: f64) {
    if row(g, bi).dice.is_some() {
        glass_break_step_dice(g, bi);
        return;
    }
    if !row(g, bi).pending.is_empty() {
        let mut went = false;
        loop {
            let budget = g.k.glassbreak.releases_per_frame;
            let released = g.glass_break.released_this_frame;
            let st = row(g, bi);
            if st.pending.is_empty() || !(st.pending[0].at <= now) || !(released < budget) { break; }
            let p = st.pending.remove(0);
            let f = glass_break_frags_of(g, bi).into_iter().find(|x| x.fid == p.fid);
            if let Some(f) = f {
                if row(g, bi).open[p.fid as usize] == 0 {
                    if f.b == 0.0 {
                        glass_break_release_crater(g, f.i, crater_cause(p.cause), bi);
                    } else {
                        glass_break_release_frag(g, &f, frag_cause(p.cause), p.from, p.k, bi);
                    }
                    g.glass_break.released_this_frame += 1.0;
                    went = true;
                }
            }
        }
        let think = g.k.glassbreak.think;
        let st = row(g, bi);
        if went && st.next_think <= 0.0 { st.next_think = now + think; }
    }
    {
        let st = row(g, bi);
        if st.next_think > 0.0 && now >= st.next_think {
            st.next_think = 0.0;
            g.stats.glassbreak.thinks += 1;
            glass_break_support_pass(g, bi);
        }
    }
    let rake_open = g.k.glassbreak.rake_open;
    let st = row(g, bi);
    if st.run && !st.walkable && rake_open > 0.0 && st.type_ == GlassType::Annealed { glass_break_rake(g, bi); }
}

/// any player's man walking into what is left of a run pane (glassbreak.ts GlassBreak.rake)
pub fn glass_break_rake(g: &mut crate::game::Game, bi: f64) {
    for pl in crate::seats::men(g) { glass_break_rake_by(g, pl, bi); }
}

/// one man's raking: the pieces in his way go, away from him, nearest first (glassbreak.ts GlassBreak.rakeBy)
pub fn glass_break_rake_by(g: &mut crate::game::Game, pl: PlayerId, bi: f64) {
    if g.pl[pl].down || g.pl[pl].holding.is_some() || g.pl[pl].dragging.is_some() || g.pl[pl].wall.is_some() { return; }
    let c = g.pl[pl].ch;
    let (pos, vel) = (g.ch[c].pos, g.ch[c].vel);
    let (rake_m, rake_open, walk_clear_h, tooth_area) = (g.k.glassbreak.rake_m, g.k.glassbreak.rake_open, g.k.glassbreak.walk_clear_h, g.k.glassbreak.tooth_area);
    let n = glass_break_normal_of(g, bi);
    let (box_, hu, hv, st_key) = { let st = row(g, bi); (st.box_, st.hu, st.hv, st.key) };
    let bc = seam_box(g, box_).c;
    let w = v3::dot(v3::sub(pos, bc), n);
    if m::abs(w) > rake_m || pos[1] >= bc[1] + hv { return; }
    let mu = glass_break_to_uv(g, pos, bi)[0];
    if m::abs(mu) > hu + 0.1 { return; }
    let vn = -v3::dot(vel, n) * m::sign(js::or(w, 1.0));
    let sp = m::hyp2(vel[0], vel[2]);
    if vn <= 0.3 || vn < 0.6 * sp { return; }
    if row(g, bi).pending.iter().any(|p| p.cause == PendingCause::Rake) || glass_break_open_fraction(g, bi) < rake_open { return; }
    let v_clear = walk_clear_h - bc[1];
    let from: V3 = [pos[0], 1.0, pos[2]];
    let mut took = 0;
    let mut waiting: JsSet<Num> = JsSet::new();
    for p in &row(g, bi).pending { waiting.add(Num(p.fid)); }
    let now = g.time;
    for f in glass_break_frags_of(g, bi) {
        let st = row(g, bi);
        if st.open[f.fid as usize] != 0 || f.v_min >= v_clear || waiting.has(Num(f.fid)) { continue; }
        if is_tooth(&f, tooth_area) { continue; }
        st.pending.push(Pending { fid: f.fid, at: now + 0.04 + 0.12 * m::abs(f.cen[0] - mu) + 0.05 * crate::debris::hash01(st_key * 71.0 + f.fid), cause: PendingCause::Rake, from: Some(from), k: 1.0 });
        took += 1;
    }
    if took != 0 {
        let who = g.ch[c].id;
        let st = row(g, bi);
        st.last_who = who;
        js::sort_by(&mut st.pending, by_time);
        g.stats.glassbreak.raked += took;
        glass_break_plog(g, &format!("raked: {} pieces", took), bi);
        glass_break_note(g, &format!("pane {}: {} pieces raked out", bi, took));
    }
}

/// the movement rule: every piece reaching below walkClearH has fallen, but for teeth (glassbreak.ts GlassBreak.walkableNow)
pub fn glass_break_walkable_now(g: &mut crate::game::Game, bi: f64) -> bool {
    let box_ = {
        let st = row(g, bi);
        if st.gone { return true; }
        if st.dice.is_some() || st.type_ != GlassType::Annealed || !st.run { return false; }
        st.box_
    };
    let v_clear = g.k.glassbreak.walk_clear_h - seam_box(g, box_).c[1];
    let tooth_area = g.k.glassbreak.tooth_area;
    for f in glass_break_frags_of(g, bi) {
        if f.v_min < v_clear && row(g, bi).open[f.fid as usize] == 0 && !is_tooth(&f, tooth_area) { return false; }
    }
    true
}

/// a wind source through each shot-out exterior pane, its speed each frame the storm at that face times the open share (glassbreak.ts GlassBreak.updateGusts)
pub fn glass_break_update_gusts(g: &mut crate::game::Game) {
    let swv = crate::wind::wind_storm_wind(g);
    let sw = swv.map(|s| [s[0], s[2]]);
    let storm_speed = match sw { Some(s) => m::hyp2(s[0], s[1]), None => 0.0 };
    let mut at = 0usize;
    while let Some(key) = g.glass_break.panes.next_key(&mut at) {
        let bi = key.0;
        if !g.glass_break.panes.get(key).map_or(false, |s| s.exterior) { continue; }
        let open = glass_break_open_fraction(g, bi);
        let src = g.glass_break.gusts.get(key).copied();
        if open < 0.25 || storm_speed < 0.05 {
            if let Some(src) = src { g.tab.wind_wind_source[src].base = 0.0; }
            continue;
        }
        let src = match src {
            Some(src) => src,
            None => {
                let n = glass_break_normal_of(g, bi);
                let (box_, hu, st_key) = { let st = row(g, bi); (st.box_, st.hu, st.key) };
                let c = seam_box(g, box_).c;
                let inward: V3 = if glass_break_sky_side(g, bi) > 0.0 { v3::scale(n, -1.0) } else { n };
                let reach = g.k.wind.door_reach;
                let d_max = crate::wind::wind_field().cut_k * reach;
                let tan_spread = m::tan(g.k.wind.door_spread_deg * core::f64::consts::PI / 180.0);
                let r0 = hu * 0.7;
                let pos: V3 = [c[0] - inward[0] * 0.15, c[1], c[2] - inward[2] * 0.15];
                let made = crate::wind::WindSource {
                    kind: crate::wind::WindSourceKind::Jet,
                    why: crate::wind::WindWhy::Glass,
                    name: format!("glass:{}", bi),
                    pos,
                    dir: inward,
                    speed: 0.0,
                    base: 0.0,
                    on: 1.0,
                    gust: 1.0,
                    reach,
                    tan_spread,
                    r0,
                    d_max,
                    bound: crate::smoke_types::WindGpuSourceBound { c: v3::mad(pos, inward, d_max / 2.0), r: m::hyp2(d_max / 2.0, r0 + d_max * tan_spread) + 0.3 },
                    seed: seed_mix_gust(st_key),
                    ..crate::wind::WindSource::zeroed()
                };
                let id = g.tab.wind_wind_source.push(made);
                g.wind.all.push(id);
                g.glass_break.gusts.set(key, id);
                glass_break_note(g, &format!("gust source through pane {}", bi));
                id
            }
        };
        let dir = g.tab.wind_wind_source[src].dir;
        let s = sw.unwrap();
        let facing = m::max(0.0, (s[0] * dir[0] + s[1] * dir[2]) / m::max(1e-3, storm_speed));
        let base = storm_speed * g.k.wind.door_k * open * (0.35 + 0.65 * facing) * g.k.glassbreak.gust_speed_k / 0.6;
        g.tab.wind_wind_source[src].base = base;
    }
}
/// `(0x61A55 ^ Math.imul(key, 2654435761)) >>> 0`
fn seed_mix_gust(key: f64) -> f64 { seed_mix(0x61A55 as f64, key) }

// ---------------------------------------------------------------- queries
/// has the glass at (u, v) of this pane fallen out (glassbreak.ts GlassBreak.openUV)
pub fn glass_break_open_uv(g: &mut crate::game::Game, u: f64, v: f64, bi: f64) -> bool {
    let st = row(g, bi);
    if st.gone { return true; }
    if let Some(d) = st.dice.as_ref() { return d.out[gp_dice_clump(gp_of(d), u, v) as usize] == 1; }
    if st.n_open == 0.0 { return false; }
    let fid = gp_classify(&st.sites, st.run, u, v);
    fid >= 0.0 && st.open[fid as usize] == 1
}

/// is world point x on pane bi where a piece has fallen out (glassbreak.ts GlassBreak.openAt)
pub fn glass_break_open_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool {
    if !g.glass_break.panes.has(Num(bi)) { return false; }
    let [u, v] = glass_break_to_uv(g, x, bi);
    glass_break_open_uv(g, u, v, bi)
}

/// the movement rule's answer for pane bi, false for a pane never hit (glassbreak.ts GlassBreak.walkable)
pub fn glass_break_walkable(g: &mut crate::game::Game, bi: f64) -> bool {
    g.glass_break.panes.get(Num(bi)).map_or(false, |st| st.walkable)
}

/// a number that changes whenever what is open in any pane changes (glassbreak.ts GlassBreak.openSig)
pub fn glass_break_open_sig(g: &mut crate::game::Game) -> f64 {
    let mut s = g.glass_break.panes.len() as f64;
    for st in g.glass_break.panes.values() {
        let next = match st.dice.as_ref() { Some(d) => d.next, None => 0.0 };
        s = js::to_i32(m::imul(js::to_i32(s), 31) as f64 + st.bi * 7.0 + st.n_open + next + (if st.gone { 4096.0 } else { 0.0 })) as f64;
    }
    s
}

/// open fraction of pane bi by area, 0 = whole (glassbreak.ts GlassBreak.openFraction)
pub fn glass_break_open_fraction(g: &mut crate::game::Game, bi: f64) -> f64 {
    let (hu, hv) = match g.glass_break.panes.get(Num(bi)) {
        None => return 0.0,
        Some(st) => {
            if st.gone { return 1.0; }
            if let Some(d) = st.dice.as_ref() { return d.next / d.clumps.len() as f64; }
            if st.n_open == 0.0 { return 0.0; }
            (st.hu, st.hv)
        }
    };
    let mut a = 0.0;
    for f in glass_break_frags_of(g, bi) {
        if row(g, bi).open[f.fid as usize] != 0 { a += f.area; }
    }
    m::min(1.0, a / (4.0 * hu * hv))
}

/// what a sibling needs to know about pane bi without touching it; None = not a sheet of glass (glassbreak.ts GlassBreak.paneInfo)
pub fn glass_break_pane_info(g: &mut crate::game::Game, bi: f64) -> Option<crate::glassbreak::GlassBreakPaneInfo> {
    let type_ = glass_break_type_of_pane(g, bi)?;
    let b = box_at(g, bi).unwrap();
    let h = seam_box(g, b).h;
    let exterior = glass_break_exterior_of(g, bi);
    let open_fraction = glass_break_open_fraction(g, bi);
    let walkable = glass_break_walkable(g, bi);
    let (gone, run, sites, energy) = match g.glass_break.panes.get(Num(bi)) {
        Some(st) => (st.gone, st.run, st.sites.len() as f64, st.energy),
        None => (false, false, 0.0, 0.0),
    };
    Some(GlassBreakPaneInfo { type_, thickness: 2.0 * m::min(h[0], h[2]), exterior, open_fraction, walkable, gone, run, sites, energy })
}

/// the record for the renderer, GB_RECORD_ROWS vec4 rows (glassbreak.ts GlassBreak.fill)
pub fn glass_break_fill(g: &mut crate::game::Game, bi: f64, dst: &mut Vec<f32>, off: f64) -> bool {
    let grain = g.k.glassbreak.dice_grain;
    let st = match g.glass_break.panes.get(Num(bi)) { Some(st) => st, None => return false };
    if st.sites.is_empty() && st.dice.is_none() && !st.gone { return false; }
    let off = off as usize;
    let any_open = st.gone || st.n_open > 0.0 || st.dice.as_ref().map_or(false, |d| d.next > 0.0);
    dst[off] = st.sites.len() as f32;
    dst[off + 1] = ((if any_open { 1 } else { 0 }) | (if st.run { 2 } else { 0 }) | (if st.dice.is_some() && !st.gone { 4 } else { 0 }) | (if st.gone { 8 } else { 0 }) | (if st.type_ == GlassType::Laminated { 16 } else { 0 })) as f32;
    dst[off + 2] = if st.u_axis == 0.0 { 0.0 } else { 1.0 };
    dst[off + 3] = st.damage as f32;
    dst[off + 4] = st.hu as f32;
    dst[off + 5] = st.hv as f32;
    dst[off + 6] = match st.type_ { GlassType::Laminated => 2.0, GlassType::Tempered => 1.0, GlassType::Annealed => 0.0 };
    dst[off + 7] = 0.0;
    if let Some(d) = st.dice.as_ref() {
        let next = d.next as usize;
        dst[off + 8] = d.u as f32;
        dst[off + 9] = d.v as f32;
        dst[off + 10] = (if next >= d.clumps.len() { 1e9 } else if next == 0 { -1.0 } else { (d.clumps[next - 1].key + d.clumps[next].key) / 2.0 }) as f32;
        dst[off + 11] = grain as f32;
        dst[off + 12] = d.cols as f32;
        dst[off + 13] = d.rows as f32;
        dst[off + 14] = d.seed as f32;
    }
    for (i, s_) in st.sites.iter().enumerate() {
        let o = off + (GB_HEADER_ROWS as usize + i * GB_SITE_ROWS as usize) * 4;
        let mut mk = [0i32; 7];
        for b in 0..(GP_BANDS as usize) {
            for s in 0..(s_.n as usize) {
                if st.open[i * GP_FRAGS_PER_SITE as usize + b * GP_MAX_SECTORS as usize + s] != 0 { mk[b] |= 1 << s; }
            }
        }
        dst[o] = s_.u as f32;
        dst[o + 1] = s_.v as f32;
        dst[o + 2] = s_.rot as f32;
        dst[o + 3] = s_.rz as f32;
        dst[o + 4] = s_.n as f32;
        dst[o + 5] = s_.seed as f32;
        dst[o + 6] = m::min(8.0, s_.e) as f32;
        dst[o + 7] = s_.merge as f32;
        dst[o + 8] = s_.rho[0] as f32;
        dst[o + 9] = s_.rho[1] as f32;
        dst[o + 10] = s_.rho[2] as f32;
        dst[o + 11] = s_.rho[3] as f32;
        dst[o + 12] = (mk[0] | (mk[1] << 12)) as f32;
        dst[o + 13] = (mk[2] | (mk[3] << 12)) as f32;
        dst[o + 14] = (mk[4] | (mk[5] << 12)) as f32;
        dst[o + 15] = mk[6] as f32;
    }
    true
}

/// the panel's break test: three rounds' worth on the pane on the man's line to the cursor; the pane, or -1 (glassbreak.ts GlassBreak.breakTest)
pub fn glass_break_break_test(g: &mut crate::game::Game) -> f64 {
    let aim = g.aim_point;
    let from = v3::add(g.ch[g.pl[g.player].ch].pos, [0.0, 1.3, 0.0]);
    let d = v3::sub(aim, from);
    let l = v3::len(d);
    if l < 1e-3 { return -1.0; }
    let rd = v3::scale(d, 1.0 / l);
    let glass = js::to_i32(crate::scene_boxes::box_flag().glass);
    let mut bi = -1.0;
    let mut k = 0.0;
    while k < 3.0 {
        let o = v3::add(from, [0.0, 0.07 * k, 0.0]);
        let h = match crate::scene_collision::raycast(g, o, rd, l + 6.0) { Some(h) if !(h.index >= 1e6) => h, _ => return bi };
        let b = match box_at(g, h.index) { Some(b) => b, None => return bi };
        let (flags, hh) = { let r = seam_box(g, b); (r.flags, r.h) };
        if js::to_i32(flags) & glass == 0 || m::min(hh[0], hh[2]) > 0.05 { return bi; }
        bi = h.index;
        crate::glass::glass_impact(g, bi, v3::mad(o, rd, h.t), Some(rd), 1.0, 0.0, None, None, None);
        if k == 2.0 { return bi; }
        k += 1.0;
    }
    -1.0
}

/// the panel's line (glassbreak.ts GlassBreak.summary)
pub fn glass_break_summary(g: &mut crate::game::Game) -> String {
    let state = if g.tune.glass_break { if glass_break_live(g) { "ON" } else { "on — but inert (chaos 0 / the tour)" } } else { "off" };
    let t = &g.stats.glassbreak;
    let mut s = format!("glass breaking {} ▸ {} panes touched · {} impacts · {} stars · {} run · {} fragments out ({} clumps) · diced {}", state, g.glass_break.panes.len(), t.impacts, t.sites, t.runs, t.dropped, t.clumps, t.diced);
    if g.glass_break.on_release.is_some() {
        s.push_str(&format!(" · {} to the shards hook", t.hooked));
    } else {
        s.push_str(&format!(" · stand-in: {} shards ({} refused), {} chips", t.shards, t.refused, t.chips));
    }
    let mut at = 0usize;
    while let Some(key) = g.glass_break.panes.next_key(&mut at) {
        let (bi, type_, run, walkable) = match g.glass_break.panes.get(key) {
            Some(st) if st.n_open != 0.0 || st.dice.is_some() || st.gone => (st.bi, st.type_, st.run, st.walkable),
            _ => continue,
        };
        let open = m::round(100.0 * glass_break_open_fraction(g, bi));
        s.push_str(&format!(" | pane {} {} {}% open{}{}", bi, type_.as_str(), open, if run { " run" } else { "" }, if walkable { " (walkable)" } else { "" }));
    }
    s
}

/// every pane whole, the gust sources gone (glassbreak.ts GlassBreak.reset)
pub fn glass_break_reset(g: &mut crate::game::Game) {
    g.glass_break.panes.clear();
    crate::scene_collision::wrote(g);
    crate::scene_collision::glass_wrote(g);
    g.glass_break.blasted.clear();
    let gusts: Vec<WindWindSourceId> = g.glass_break.gusts.values().copied().collect();
    for src in gusts {
        if let Some(i) = g.wind.all.iter().position(|s| *s == src) { g.wind.all.remove(i); }
    }
    g.glass_break.gusts.clear();
    g.glass_break.gust_dirty = false;
}

/// nearest point of a yawed box to p (glassbreak.ts nearestOn)
pub fn nearest_on(g: &mut crate::game::Game, b: BoxId, p: V3) -> V3 {
    let (c, h, yaw) = { let r = seam_box(g, b); (r.c, r.h, r.yaw) };
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let (px, pz) = (p[0] - c[0], p[2] - c[2]);
    let lx = m::max(-h[0], m::min(h[0], cs * px - sn * pz));
    let lz = m::max(-h[2], m::min(h[2], sn * px + cs * pz));
    let ly = m::max(-h[1], m::min(h[1], p[1] - c[1]));
    [c[0] + cs * lx + sn * lz, c[1] + ly, c[2] - sn * lx + cs * lz]
}

#[cfg(test)]
mod tests {
    use super::*;
    // the TypeScript's own results, from bun: (s ^ Math.imul(n, 0x9E3779B1)) >>> 0 and (0x61A55 ^ Math.imul(key, 2654435761)) >>> 0
    #[test]
    fn seeds_are_the_typescripts() {
        assert_eq!(seed_mix(16711935.0, 7.0).to_bits(), 0x41d4ded4ca000000);
        assert_eq!(seed_mix(4294901760.0, 4000000123.0).to_bits(), 0x41ecab2401600000);
        assert_eq!(seed_mix_gust(3735928559.0).to_bits(), 0x41e397dc0d400000);
        assert_eq!(xor_u32(4294901760.0, 0xC2A7E5 as f64).to_bits(), 0x41efe7b4fca00000);
    }
    // the TypeScript's openSig over the same two rows, from bun: s = (Math.imul(s, 31) + bi * 7 + nOpen + next + gone) | 0, in the map's order
    const OPEN_SIG: f64 = 1115106832.0;
    #[test]
    fn open_sig_is_the_typescripts_and_goes_by_the_order_set() {
        let mut g = crate::game::Game::blank();
        g.glass_break.panes.set(Num(12.0), PaneState { bi: 12.0, n_open: 3.0, ..PaneState::zeroed() });
        g.glass_break.panes.set(Num(2000000000.0), PaneState { bi: 2000000000.0, gone: true, dice: Some(Dice { next: 5.0, ..Dice::zeroed() }), ..PaneState::zeroed() });
        assert_eq!(glass_break_open_sig(&mut g).to_bits(), f64::to_bits(OPEN_SIG));
    }
    // each hands on a pane with a piece out, a piece queued, coming down or gone, and no whole or merely cracked one
    #[test]
    fn each_passes_over_a_whole_pane() {
        let mut g = crate::game::Game::blank();
        let queued = Pending { fid: 8.0, ..Pending::zeroed() };
        g.glass_break.panes.set(Num(1.0), PaneState { bi: 1.0, ..PaneState::zeroed() });
        g.glass_break.panes.set(Num(2.0), PaneState { bi: 2.0, n_open: 1.0, ..PaneState::zeroed() });
        g.glass_break.panes.set(Num(3.0), PaneState { bi: 3.0, pending: vec![queued], ..PaneState::zeroed() });
        g.glass_break.panes.set(Num(4.0), PaneState { bi: 4.0, gone: true, ..PaneState::zeroed() });
        g.glass_break.panes.set(Num(5.0), PaneState { bi: 5.0, dice: Some(Dice::zeroed()), ..PaneState::zeroed() });
        let mut seen = Vec::new();
        glass_break_each(&mut g, &mut |_, r| seen.push(r.bi));
        assert_eq!(seen, vec![2.0, 3.0, 4.0, 5.0]);
    }
    // the rule of the burst's view off the row's own word: a pane gone or coming down answers without asking a box
    #[test]
    fn a_view_reads_the_view_of_and_none_answers_false() {
        let mut g = crate::game::Game::blank();
        g.glass_break.panes.set(Num(4.0), PaneState { bi: 4.0, gone: true, ..PaneState::zeroed() });
        assert!(!glass_view_also(&mut g, 4.0, [0.0, 0.0, 0.0]));
        g.glass_break.view_of = Some(GlassBreakViewOf { kind: GlassBreakViewOfKind::Blown, from: [0.0, 0.0, 0.0], size: GlassBlastOpts::zeroed(), rows: None });
        assert!(glass_view_also(&mut g, 4.0, [0.0, 0.0, 0.0]));
        assert!(!glass_view_also(&mut g, 5.0, [0.0, 0.0, 0.0]));
    }
    // the queries by key: a pane never hit, a whole one, one gone, one half down by its clumps
    #[test]
    fn queries_go_by_the_key() {
        let mut g = crate::game::Game::blank();
        assert!(!glass_break_walkable(&mut g, 9.0));
        assert_eq!(glass_break_open_fraction(&mut g, 9.0), 0.0);
        assert!(!glass_break_going_at(&mut g, 9.0, [0.0, 0.0, 0.0]));
        assert!(!glass_break_open_at(&mut g, 9.0, [0.0, 0.0, 0.0]));
        g.glass_break.panes.set(Num(1.0), PaneState { bi: 1.0, ..PaneState::zeroed() });
        assert_eq!(glass_break_open_fraction(&mut g, 1.0), 0.0);
        assert!(!glass_break_open_uv(&mut g, 0.0, 0.0, 1.0));
        g.glass_break.panes.set(Num(2.0), PaneState { bi: 2.0, gone: true, walkable: true, ..PaneState::zeroed() });
        assert_eq!(glass_break_open_fraction(&mut g, 2.0), 1.0);
        assert!(glass_break_walkable(&mut g, 2.0) && glass_break_open_uv(&mut g, 0.0, 0.0, 2.0) && glass_break_going_at(&mut g, 2.0, [0.0, 0.0, 0.0]));
        let c = Clump::zeroed();
        g.glass_break.panes.set(Num(3.0), PaneState { bi: 3.0, dice: Some(Dice { next: 1.0, clumps: vec![c, c, c, c], ..Dice::zeroed() }), ..PaneState::zeroed() });
        assert_eq!(glass_break_open_fraction(&mut g, 3.0), 0.25);
    }
    // off the list, negative, fractional, NaN: undefined in the TypeScript
    #[test]
    fn box_at_goes_by_place() {
        let mut g = crate::game::Game::blank();
        g.col.boxes = vec![BoxId::from_ix(7), BoxId::from_ix(3)];
        assert!(box_at(&g, 1.0) == Some(BoxId::from_ix(3)));
        for i in [2.0, -1.0, 0.5, f64::NAN] { assert!(box_at(&g, i).is_none()); }
    }
}
