//! src/game/glassbreak.ts lines 219 to 584: the class GlassBreak, thin over rust/sim's panes.rs

pub use ::game_types::glassbreak::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, vec, vec::Vec};

/// the one gate every entry point takes first. GLASSFX.on is settled on.
pub fn glass_break_live(g: &crate::game::Game) -> bool {
    g.tune.glass_break && crate::destruction::chaos_of(g) > 0.0
}

use ::sim::panes::{self, Ctx};
use crate::glasspen::PaneType;
use crate::scene_box_mirror::{PaneView, VIEW_BLOWN, VIEW_GIVES};
use crate::Num;

// ---------------------------------------------------------------- what the TypeScript reads off an array by a number
/// a box's row. `g.col.boxes: Vec<BoxId>` names it, `g.tab.boxes` holds it
fn seam_box<'g>(g: &'g mut Game, b: impl Into<AnyBoxId>) -> &'g mut crate::scene_boxes::Box { &mut g.tab[b.into()] }
/// `g.level.boxes[bi]`: undefined off the list (a negative, fractional or NaN index included). That list and `g.col.boxes` are one array in the TypeScript: the box at place bi
fn box_at(g: &Game, bi: f64) -> Option<BoxId> {
    let ix = bi as usize;
    if ix as f64 != bi { return None; }
    g.col.boxes.get(ix).copied()
}
/// TYPES[t], panes.rs's word for a glass; a word off the list reads laminated (the `??` of impact: no other caller can meet one)
fn type_word(t: u32) -> PaneType {
    match t { 0 => PaneType::Annealed, 1 => PaneType::Tempered, _ => PaneType::Laminated }
}
/// TYPES.indexOf(t)
fn type_index(t: PaneType) -> u32 {
    match t { PaneType::Annealed => 0, PaneType::Tempered => 1, PaneType::Laminated => 2 }
}
/// the row of pane bi, as a read or a write for a few lines. Only a pane with a row is asked (a fragment's, a tool's in hand)
fn row_at(g: &Game, bi: f64) -> &'static mut panes::Row {
    let r = panes::row_index(g.sim, js::to_u32(bi));
    assert!(r >= 0, "glass: pane {} has no row", bi);
    panes::row_ref(g.sim, r as usize)
}
/// The class's fields as made; its hooks are called by name, so what is hung here is which they are
pub fn glass_break_new(g: &mut crate::game::Game) -> crate::glassbreak::GlassBreak {
    crate::scene_collision::glass_rule(g, Some(crate::scene_collision::PaneRule::GlassBreak), Some(crate::scene_collision::PaneRows::GlassBreak));
    g.engine.world.glass_open = Some(crate::scene_collision::PaneRule::GlassBreak);
    g.col.glass_walkable = Some(crate::scene_collision::GlassWalkable::GlassBreak);
    crate::destruction::destruction_on_blast(g, crate::destruction::BlastListener::Glass);
    GlassBreak::blank()
}

// ---------------------------------------------------------------- the call
/// a question or a rule of the panes, over this game's block and the copy of its box world as it stands: `fn_` asks `::sim::panes` under the context it is handed
pub fn glass_break_asking<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, &mut Ctx) -> T) -> T {
    crate::scene_collision::window(g, &mut |g| {
        crate::scene_collision::rules_statics(g);
        if !crate::scene_collision::standing(g) { panic!("glass: the box world's copy will not stand for the rule: rust/sim/src/panes.rs has nothing to ask"); }
        let dynamic = g.col.dynamic_boxes.clone();
        let set = crate::scene_box_mirror::box_mirror_leaves(g, &dynamic);
        if set == -2.0 { panic!("glass: more leaves than a list of the module holds"); }
        let mut cx = Ctx { g: g.sim, w: g.col.mirror.slot as usize, set: set as i32, now: g.time, shards: crate::shards::shards_live(g), gravity: crate::shard_model::shard().gravity, out: Vec::new(), landings: Vec::new(), wrote: false };
        let r = fn_(g, &mut cx);
        wrote(g, &mut cx);
        r
    })
}
/// what the rule did to the game, carried over: a static's kind moved, and the copy is told; what will come down is on the falls' list
fn wrote(g: &mut crate::game::Game, cx: &mut Ctx) {
    if core::mem::take(&mut cx.wrote) { crate::scene_collision::wrote(g); }
    for l in cx.landings.drain(..) { crate::falls::schedule(&mut g.falls, crate::falls::FallKind::Glass, l, g.k.shards.pending_max); }
}

/// the level's sheets of glass for the copy of its box world: each its static, how far it stands on the plan from the nearest door's opening, which face has the sky
pub fn glass_break_sheets(g: &mut crate::game::Game) -> Vec<f64> {
    if g.glass_break.sheet_list.is_none() {
        let mut out = Vec::new();
        for bi in crate::glass::glass_panes(g) {
            let b = box_at(g, bi);
            if crate::glass::is_sheet(g, b) {
                let gap = glass_break_door_gap(g, b.unwrap());
                out.push(bi);
                out.push(gap);
                out.push(SKY_UNKNOWN);
            }
        }
        g.glass_break.sheet_list = Some(out);
    }
    g.glass_break.sheet_list.clone().unwrap()
}

/// the level has stood up: which glass each pane is, settled as the level stands
pub fn glass_break_stood(g: &mut crate::game::Game) {
    g.glass_break.sheet_list = None;
    let mut l = glass_break_sheets(g);
    crate::scene_collision::glass_wrote(g);
    glass_break_asking(g, &mut |g, cx| {
        let mut i = 0;
        while i < l.len() {
            l[i + 2] = panes::sky_known(cx, js::to_u32(l[i]), &g.k) as f64;
            i += 3;
        }
    });
    g.glass_break.sheet_list = Some(l);
    crate::scene_collision::glass_wrote(g);
}

/// metres on the plan from pane b to the nearest door's opening; Infinity with no door
fn glass_break_door_gap(g: &mut crate::game::Game, b: BoxId) -> f64 {
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

/// what the last call let go, handed on in the order it went; the list is taken whole first, since what is told of a fragment may ask the panes anything
fn glass_break_drain(g: &mut crate::game::Game, cx: &mut Ctx) {
    wrote(g, cx);
    if cx.out.is_empty() { return; }
    let list: Vec<crate::glassbreak::Fragment> = core::mem::take(&mut cx.out).into_iter().map(|l| {
        let v = glass_break_geom(g, l.f.bi);
        let outline = l.pts.iter().map(|p| glass_break_world(g, v, p.0, p.1, None)).collect();
        let tint = seam_box(g, v.box_).albedo;
        crate::glassbreak::Fragment { outline, tint, ..l.f }
    }).collect();
    for f in &list { glass_break_let_go(g, f); }
}

/// a fragment the rule let go, told to the game: its one way out
fn glass_break_let_go(g: &mut crate::game::Game, f: &crate::glassbreak::Fragment) {
    let v = glass_break_geom(g, f.bi);
    if f.heard {
        let dicing = f.kind == FragmentKind::Clump || f.kind == FragmentKind::Sheet;
        let rate = if dicing { 0.7 } else { 1.25 + 0.3 * crate::debris::hash01(f.key + v.key) };
        crate::audio_iface::game_audio_like_play(g, "glassCrack", Some(f.centre), Some(if dicing { 1.0 } else { 0.75 }), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
        crate::glass::glass_event(g, box_at(g, f.bi).unwrap(), f.centre, 1.0, f.who);
    }
    if crate::shards::shards_live(g) {
        g.m.glassbreak.gb_host.hooked += 1.0;
        crate::shards::glass_shards_release(g, f);
        return;
    }
    glass_break_stand_in(g, v, f);
}

/// a pane's frame, for what the host still makes of a fragment
fn glass_break_geom(g: &mut crate::game::Game, bi: f64) -> crate::glassbreak::PaneFrame {
    let box_ = box_at(g, bi).unwrap();
    let st = row_at(g, bi);
    PaneFrame { key: st.key as f64, u_axis: if st.u_axis == 0 { 0.0 } else { 2.0 }, ht: st.ht, box_ }
}

/// pane-local (u, v) → world
fn glass_break_world(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, u: f64, v_2: f64, w: Option<f64>) -> V3 {
    let (yaw, c) = { let b = seam_box(g, v.box_); (b.yaw, b.c) };
    world_on(yaw, c, v.u_axis, u, v_2, w.unwrap_or(0.0))
}

/// world → pane-local (u, v)
fn glass_break_uv(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, x: V3) -> [f64; 2] {
    let (yaw, c) = { let b = seam_box(g, v.box_); (b.yaw, b.c) };
    uv_on(yaw, c, v.u_axis, x)
}

/// the arithmetic of world and toWorld
fn world_on(yaw: f64, c: V3, u_axis: f64, u: f64, v: f64, w: f64) -> V3 {
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    let lx = if u_axis == 0.0 { u } else { w }; let lz = if u_axis == 0.0 { w } else { u };
    [c[0] + cs * lx + sn * lz, c[1] + v, c[2] - sn * lx + cs * lz]
}
/// the arithmetic of uv and toUV
fn uv_on(yaw: f64, c: V3, u_axis: f64, x: V3) -> [f64; 2] {
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    let px = x[0] - c[0]; let pz = x[2] - c[2];
    let lx = cs * px - sn * pz; let lz = sn * px + cs * pz;
    [if u_axis == 0.0 { lx } else { lz }, x[1] - c[1]]
}
/// the arithmetic of normalOf and of the stand-in's and the gusts' normals
fn normal_on(yaw: f64, u_axis: f64) -> V3 {
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    if u_axis == 0.0 { [sn, 0.0, cs] } else { [cs, 0.0, -sn] }
}

/// no shards consumer attached: thin Debris boxes over the fragment and a burst of pale chips
fn glass_break_stand_in(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, f: &crate::glassbreak::Fragment) {
    let key = f.key;
    let uv: Vec<[f64; 2]> = f.outline.iter().map(|p| glass_break_uv(g, v, *p)).collect();
    let (_, cu, cv) = ::sim::pattern::poly_area(&uv.iter().map(|p| (p[0], p[1])).collect::<Vec<_>>());
    let cen = [cu, cv];
    let grains = f.grain > 0.0;
    let sheet = f.kind == FragmentKind::Sheet;
    let n_sh = if grains {
        if f.kind == FragmentKind::Clump && js::rem(key, g.k.glassbreak.crumb_every) == 0.0 { 1.0 } else { 0.0 }
    } else if sheet {
        3.0
    } else {
        m::max(1.0, m::min(3.0, m::round(f.area / 0.07)))
    };
    let yaw = seam_box(g, v.box_).yaw;
    let nrm = normal_on(yaw, v.u_axis);
    let out_sign = if v3::dot(nrm, f.normal) >= 0.0 { 1.0 } else { -1.0 };
    let n_uv = uv.len() as f64;
    let mut k = 0.0;
    while k < n_sh {
        let h1 = crate::debris::hash01(v.key * 7919.0 + key * 131.0 + k * 17.0);
        let h2 = crate::debris::hash01(v.key * 104729.0 + key * 193.0 + k * 29.0);
        let vtx = uv.get(js::rem(k * m::max(1.0, m::floor(n_uv / n_sh)), n_uv) as usize).copied().unwrap_or(cen);
        let pu = cen[0] * (if n_sh == 1.0 { 1.0 } else { 0.6 }) + vtx[0] * (if n_sh == 1.0 { 0.0 } else { 0.4 });
        let pv = cen[1] * (if n_sh == 1.0 { 1.0 } else { 0.6 }) + vtx[1] * (if n_sh == 1.0 { 0.0 } else { 0.4 });
        let side = m::sqrt(f.area / n_sh) * (if grains { 0.35 } else { 0.92 });
        let su = side * (0.8 + 0.4 * h1);
        let sv = side * (0.8 + 0.4 * h2);
        let hh: V3 = if v.u_axis == 0.0 { [su / 2.0, sv / 2.0, m::max(0.006, v.ht)] } else { [m::max(0.006, v.ht), sv / 2.0, su / 2.0] };
        let p = crate::scene_boxes::MakeBoxP {
            c: glass_break_world(g, v, pu, pv, Some(out_sign * 0.02)),
            h: hh,
            yaw: Some(yaw),
            albedo: Some(v3::copy(g.k.glassbreak.shard_albedo)),
            flags: Some(crate::scene_boxes::box_flag().dynamic),
            ..Default::default()
        };
        let bx = crate::scene_boxes::make_box(g, &p);
        let ch = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Shard, bx, v3::copy(f.velocity), v3::copy(f.spin), Some(f.who), None);
        if let Some(ch) = ch {
            let chunk_box = g.tab.debris_chunk[ch].box_;
            seam_box(g, chunk_box).rough = Some(0.05);
            g.m.glassbreak.gb_host.shards += 1.0;
        } else {
        }
        k += 1.0;
    }
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
}

// ---------------------------------------------------------------- which glass

/// the type of pane bi as it will break; None for anything not a sheet
pub fn glass_break_type_of_pane(g: &mut crate::game::Game, bi: f64) -> Option<crate::glasspen::PaneType> {
    let t = glass_break_asking(g, &mut |g, cx| panes::type_of_pane(cx, js::to_u32(bi), &g.k));
    if t as f64 == NO_TYPE { None } else { Some(type_word(t)) }
}

/// make pane bi break as `type_` from here on, whole first; None = back to typeOf
pub fn glass_break_set_type(g: &mut crate::game::Game, bi: f64, type_: Option<crate::glasspen::PaneType>) {
    let ty = match type_ { Some(t) => type_index(t), None => NO_OVER as u32 };
    glass_break_asking(g, &mut |g, cx| { panes::set_type(cx.g, js::to_u32(bi), ty); });
    crate::scene_collision::wrote(g);
    if let Some(src) = g.glass_break.gusts.get(Num(bi)).copied() {
        if let Some(i) = g.wind.all.iter().position(|s| *s == src) { g.wind.all.remove(i); }
        g.glass_break.gusts.delete(Num(bi));
    }
}

// ---------------------------------------------------------------- stimuli
/// THE IMPACT ENTRY POINT: a blow of `energy` met pane `bi` at world x going `dir`: the module files it and answers what the glass did
pub fn glass_break_impact(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact {
    let b = box_at(g, bi);
    let shape = match b { Some(b) => { let r = seam_box(g, b); Some((r.flags, r.h)) } None => None };
    let is_glass = match shape { Some((flags, _)) => (js::to_i32(flags) & js::to_i32(crate::scene_boxes::box_flag().glass)) != 0, None => false };
    let thickness = match (is_glass, shape) { (true, Some((_, h))) => 2.0 * m::min(h[0], h[2]), _ => 0.0 };
    let mut res = GlassImpact { pane: is_glass, live: false, type_: PaneType::Annealed, thickness, was_open: false, punched: false, diced: false, released: 0.0, cracked: false };
    if !is_glass { return res; }
    if !glass_break_live(g) {
        res.type_ = glass_break_type_of_pane(g, bi).unwrap_or(PaneType::Laminated);
        return res;
    }
    let thr = match through { None => -1, Some(true) => 1, Some(false) => 0 };
    let v = glass_break_asking(g, &mut |g, cx| {
        let r = panes::impact(cx, js::to_u32(bi), x, dir, energy, who, thr, &g.k);
        glass_break_drain(g, cx);
        r
    });
    (res.live, res.was_open, res.punched, res.diced, res.cracked) = (v.live, v.was_open, v.punched, v.diced, v.cracked);
    res.type_ = type_word(v.ty);
    res.released = v.released as f64;
    res
}

/// a pellet through glass its own shell has just broken: no blow filed, but a standing piece of an annealed pane goes with it; fragments let go
pub fn glass_break_widen(g: &mut crate::game::Game, bi: f64, x: V3) -> f64 {
    if !glass_break_live(g) { return 0.0; }
    let n = glass_break_asking(g, &mut |g, cx| {
        let r = panes::widen(cx, js::to_u32(bi), x, &g.k);
        glass_break_drain(g, cx);
        r
    });
    n as f64
}

/// A BLAST at `at`: every sheet in reach takes it by type and by k = power × (1 − d / radius); the panes it changed
pub fn glass_break_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>) -> f64 {
    let opts = opts.unwrap_or_default();
    if !glass_break_live(g) { return 0.0; }
    let radius = opts.radius.unwrap_or(g.k.glassbreak.blast_m);
    let power = opts.power.unwrap_or(1.0);
    let who = opts.who.unwrap_or(0.0);
    let mut changed = 0.0;
    if radius > 0.0 && power > 0.0 {
        glass_break_asking(g, &mut |g, cx| {
            let arrival = panes::blast_begin(cx, at);
            for i in 0..panes::sheet_count(cx) as usize {
                if panes::blast_sheet(cx, arrival, i, at, radius, power, who, &g.k) { changed += 1.0; }
                wrote(g, cx);
                glass_break_drain(g, cx);
            }
        });
    }
    changed
}

/// a stun charge went off at `at`: a blast at bangPower
pub fn glass_break_bang(g: &mut crate::game::Game, at: V3, who: f64) -> f64 {
    let opts = GlassBlastOpts { radius: Some(g.k.glassbreak.blast_m), power: Some(g.k.glassbreak.bang_power), who: Some(who) };
    glass_break_blast(g, at, Some(opts))
}

// ---------------------------------------------------------------- a burst's view of the glass
/// where the rule's state stands: the slot is `g.sim`, so what comes back says only that it is the module's
pub fn glass_break_home() -> Option<crate::Opaque> {
    Some(crate::Opaque)
}

/// for the length of `fn_`, every solid walk and every ray also passes a pane where a burst at `from` of this size sees no glass
pub fn glass_break_see_through<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, view: crate::glassbreak::GlassBreakSeeThroughView, from: V3, size: crate::glassbreak::GlassBlastOpts, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let kind = match view {
        GlassBreakSeeThroughView::Blown => GlassBreakViewOfKind::Blown,
        GlassBreakSeeThroughView::Wary => GlassBreakViewOfKind::Wary,
        GlassBreakSeeThroughView::Counted => GlassBreakViewOfKind::Counted,
    };
    let was = g.glass_break.view_of.take();
    g.glass_break.view_of = Some(GlassBreakViewOf { kind, from, size });
    crate::scene_collision::glass_viewed(g);
    let out = fn_(g);
    g.glass_break.view_of = was;
    crate::scene_collision::glass_viewed(g);
    out
}

/// the view in hand as numbers: which view, the burst's point, reach and power, whether a man counts on it
pub fn glass_break_view(g: &crate::game::Game) -> Option<crate::scene_box_mirror::PaneView> {
    let v = g.glass_break.view_of?;
    Some(PaneView {
        kind: if v.kind == GlassBreakViewOfKind::Blown { VIEW_BLOWN } else { VIEW_GIVES },
        at: vec![v.from[0], v.from[1], v.from[2], v.size.radius.unwrap_or(g.k.glassbreak.blast_m), v.size.power.unwrap_or(1.0), if v.kind == GlassBreakViewOfKind::Counted { 1.0 } else { 0.0 }],
    })
}

/// the rule's whole answer at a point: a piece out, or the view in hand counts the glass there out
pub fn glass_break_rule_open(g: &mut crate::game::Game, bi: f64, p: V3) -> bool {
    if g.glass_break.view_of.is_none() { return glass_break_open_at(g, bi, p); }
    glass_break_asking(g, &mut |g, cx| ::sim::glass_open(cx.w, js::to_u32(bi), p[0], p[1], p[2], &g.k) == 1)
}

// ---------------------------------------------------------------- per frame
/// per frame: the gusts, then every touched pane's step in the module, draining after each
pub fn glass_break_update(g: &mut crate::game::Game) {
    let touched = !glass_break_untouched(g);
    let s = g.sim;
    if touched && panes::take_gust(s) { g.glass_break.gust_dirty = true; }
    if g.glass_break.gusts.len() > 0 || g.glass_break.gust_dirty {
        glass_break_update_gusts(g);
        g.glass_break.gust_dirty = g.glass_break.gusts.len() > 0;
    }
    if !glass_break_live(g) { return; }
    let n = if touched { panes::row_n(s) as usize } else { 0 };
    if n > 0 {
        glass_break_asking(g, &mut |g: &mut Game, cx: &mut Ctx| {
            panes::frame_begin(cx.g);
            // who may walk into what is left of a pane: any player's man, on his feet with his hands free
            let men: Vec<panes::Walker> = crate::seats::men(g).into_iter().filter(|&pl| {
                let p = &g.pl[pl];
                !crate::condition::down(g, p.ch) && p.holding.is_none() && p.dragging.is_none() && p.wall.is_none()
            }).map(|pl| { let c = &g.ch[g.pl[pl].ch]; panes::Walker { pos: c.pos, vel: c.vel, id: c.id } }).collect();
            for r in 0..n {
                panes::step_row(cx, r, &men, &g.k);
                wrote(g, cx);
                glass_break_drain(g, cx);
            }
        });
    }
}

/// a wind source through each shot-out exterior pane, its speed each frame the storm at that face times the open share
fn glass_break_update_gusts(g: &mut crate::game::Game) {
    let swv = crate::wind::wind_storm_wind(g);
    let sw = swv.map(|s| [s[0], s[2]]);
    let storm_speed = match sw { Some(s) => m::hyp2(s[0], s[1]), None => 0.0 };
    let s = g.sim;
    let n = panes::row_n(s) as usize;
    for r in 0..n {
        let (exterior, bi, st_key, u_axis, hu) = { let st = panes::row_ref(s, r); (st.exterior, st.w[0] as f64, st.key, st.u_axis, st.hu) };
        if exterior == 0 { continue; }
        let key = Num(bi);
        let open = glass_break_open_fraction(g, bi);
        let src = g.glass_break.gusts.get(key).copied();
        if open < 0.25 || storm_speed < 0.05 {
            if let Some(src) = src { g.tab.wind_wind_source[src].base = 0.0; }
            continue;
        }
        let src = match src {
            Some(src) => src,
            None => {
                let box_ = box_at(g, bi).unwrap();
                let (yaw, c) = { let b = seam_box(g, box_); (b.yaw, b.c) };
                let nrm = normal_on(yaw, if u_axis == 0 { 0.0 } else { 2.0 });
                let sky = glass_break_asking(g, &mut |g, cx| panes::sky_known(cx, js::to_u32(bi), &g.k));
                let inward: V3 = if sky > 0 { v3::scale(nrm, -1.0) } else { nrm };   // away from the face the sky is over
                let reach = g.k.wind.door_reach;
                let d_max = ::sim::wind::CUT_K * reach;   // (WIND_FIELD.cutK: its one home is the module's SHAPE)
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
                    seed: seed_mix_gust(st_key as f64),
                    ..crate::wind::WindSource::default()
                };
                let id = g.tab.wind_wind_source.push(made);
                g.wind.all.push(id);
                g.glass_break.gusts.set(key, id);
                id
            }
        };
        let dir = g.tab.wind_wind_source[src].dir;
        let w = sw.unwrap();
        let facing = m::max(0.0, (w[0] * dir[0] + w[1] * dir[2]) / m::max(1e-3, storm_speed));
        let base = storm_speed * g.k.wind.door_k * open * (0.35 + 0.65 * facing) * g.k.glassbreak.gust_speed_k / 0.6;
        g.tab.wind_wind_source[src].base = base;
    }
}
/// `(0x61A55 ^ Math.imul(key, 2654435761)) >>> 0`
fn seed_mix_gust(key: f64) -> f64 { (js::to_i32(0x61A55 as f64) ^ m::imul(js::to_i32(key), js::to_i32(2654435761.0))) as u32 as f64 }

// ---------------------------------------------------------------- queries
/// nothing of a pane's is in the block: the panes part's first word, `used` (glassbreak.ts GlassBreak.untouched; `panes_untouched` takes no slot)
fn glass_break_untouched(g: &crate::game::Game) -> bool {
    ::sim::state::state_read(g.sim).panes.used == 0
}

/// is world point x on pane bi where a piece has fallen out
pub fn glass_break_open_at(g: &crate::game::Game, bi: f64, x: V3) -> bool {
    !glass_break_untouched(g) && panes::open_at(g.sim, js::to_u32(bi), x[0], x[1], x[2])
}

/// the movement rule's answer for pane bi, false for a pane never hit
pub fn glass_break_walkable(g: &mut crate::game::Game, bi: f64) -> bool {
    !glass_break_untouched(g) && panes::walkable(g.sim, js::to_u32(bi))
}

/// a number that changes whenever what is open in any pane changes
pub fn glass_break_open_sig(g: &mut crate::game::Game) -> f64 {
    if glass_break_untouched(g) { 0.0 } else { panes::open_sig(g.sim) as f64 }
}

/// open fraction of pane bi by area, 0 = whole
fn glass_break_open_fraction(g: &mut crate::game::Game, bi: f64) -> f64 {
    if glass_break_untouched(g) { 0.0 } else { panes::open_fraction(g.sim, js::to_u32(bi)) }
}

/// what a sibling needs to know about pane bi without touching it; None = not a sheet of glass
pub fn glass_break_pane_info(g: &mut crate::game::Game, bi: f64) -> Option<crate::glassbreak::GlassBreakPaneInfo> {
    glass_break_type_of_pane(g, bi)?;
    let r = if glass_break_untouched(g) { -1 } else { panes::row_index(g.sim, js::to_u32(bi)) };
    let gone = r >= 0 && panes::row_ref(g.sim, r as usize).w[GW_FLAGS as usize] & G_GONE as u32 != 0;
    let open_fraction = glass_break_open_fraction(g, bi);
    Some(GlassBreakPaneInfo { open_fraction, gone })
}

/// every pane whole, the gust sources gone
pub fn glass_break_reset(g: &mut crate::game::Game) {
    panes::reset(g.sim);
    crate::scene_collision::wrote(g);
    let gusts: Vec<WindWindSourceId> = g.glass_break.gusts.values().copied().collect();
    for src in gusts {
        if let Some(i) = g.wind.all.iter().position(|s| *s == src) { g.wind.all.remove(i); }
    }
    g.glass_break.gusts.clear();
    g.glass_break.gust_dirty = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn blank() -> Box<Game> { Box::new(Game::blank()) }
    // the TypeScript's own result, from bun: (0x61A55 ^ Math.imul(key, 2654435761)) >>> 0
    #[test]
    fn the_gust_seed_is_the_typescripts() {
        assert_eq!(seed_mix_gust(3735928559.0).to_bits(), 0x41e397dc0d400000);
    }
    // TYPES of the TypeScript, panes.rs ANNEALED, TEMPERED, LAMINATED: the skeleton's PaneType lists its words in another order
    #[test]
    fn a_glass_word_is_the_modules() {
        for (t, want) in [(panes::ANNEALED, PaneType::Annealed), (panes::TEMPERED, PaneType::Tempered), (panes::LAMINATED, PaneType::Laminated)] {
            assert_eq!(type_word(t), want);
            assert_eq!(type_index(want), t);
        }
        assert_eq!(type_word(7), PaneType::Laminated);
    }
    // the view in hand as the numbers the copy takes, with a size's defaults; none in hand is none
    #[test]
    fn a_view_is_its_numbers() {
        let mut g = blank();
        assert!(glass_break_view(&mut g).is_none());
        g.glass_break.view_of = Some(GlassBreakViewOf { kind: GlassBreakViewOfKind::Counted, from: [1.0, 2.0, 3.0], size: GlassBlastOpts { radius: Some(5.0), power: None, who: None } });
        let v = glass_break_view(&mut g).unwrap();
        assert_eq!((v.kind, v.at.as_slice()), (VIEW_GIVES, &[1.0, 2.0, 3.0, 5.0, 1.0, 1.0][..]));
        g.glass_break.view_of = Some(GlassBreakViewOf { kind: GlassBreakViewOfKind::Blown, from: [0.0, 0.0, 0.0], size: GlassBlastOpts::default() });
        let v = glass_break_view(&mut g).unwrap();
        assert_eq!(v.kind, VIEW_BLOWN);
        assert_eq!(v.at[3], g.k.glassbreak.blast_m);
        assert_eq!(v.at[5], 0.0);
    }
    // a game that never touched glass: every question answers none and nothing stands up
    #[test]
    fn untouched_answers_none() {
        let mut g = blank();
        assert!(glass_break_untouched(&mut g));
        assert!(!glass_break_open_at(&mut g, 3.0, [0.0, 0.0, 0.0]));
        assert!(!glass_break_walkable(&mut g, 3.0));
        assert_eq!(glass_break_open_sig(&mut g), 0.0);
        assert_eq!(glass_break_open_fraction(&mut g, 3.0), 0.0);
        assert_eq!(::sim::state::state(g.sim).panes.used, 0);
    }
    // toUV undoes toWorld on a pane's own frame, for both of its axes
    #[test]
    fn the_frame_goes_both_ways() {
        let (yaw, c): (f64, V3) = (0.0, [1.0, 2.0, 3.0]);
        assert_eq!(world_on(yaw, c, 0.0, 0.5, 0.25, 0.125), [1.5, 2.25, 3.125]);
        assert_eq!(uv_on(yaw, c, 0.0, [1.5, 2.25, 3.125]), [0.5, 0.25]);
        assert_eq!(world_on(yaw, c, 2.0, 0.5, 0.25, 0.125), [1.125, 2.25, 3.5]);
        assert_eq!(uv_on(yaw, c, 2.0, [1.125, 2.25, 3.5]), [0.5, 0.25]);
        assert_eq!(normal_on(yaw, 0.0), [0.0, 0.0, 1.0]);
        assert_eq!(normal_on(yaw, 2.0), [1.0, 0.0, -0.0]);
    }
    // off the list, negative, fractional, NaN: undefined in the TypeScript
    #[test]
    fn a_box_off_the_list_is_none() {
        let g = blank();
        for bi in [-1.0, 0.5, f64::NAN, 1e9] { assert!(box_at(&g, bi).is_none()); }
    }
}
