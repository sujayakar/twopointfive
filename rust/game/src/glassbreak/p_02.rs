//! src/game/glassbreak.ts lines 219 to 584: the class GlassBreak, thin over rust/sim's panes.rs
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::glass_pattern::{gp_dice_key, gp_poly_area, GpDice, GpSite, GP_BANDS, GP_FRAGS_PER_SITE, GP_MAX_SECTORS, GP_MAX_SITES};
use crate::glasspen::PaneType;
use crate::scene_box_mirror::{PaneView, VIEW_BLOWN, VIEW_GIVES};
use crate::Num;

// ---------------------------------------------------------------- what the TypeScript reads off an array by a number
/// a box's row. `g.col.boxes: Vec<BoxId>` names it, `g.tab.boxes` holds it
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }
/// `g.level.boxes[bi]`: undefined off the list (a negative, fractional or NaN index included). That list and `g.col.boxes` are one array in the TypeScript (game.ts:1219): the box at place bi
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
fn row_at(g: &Game, bi: f64) -> &'static mut ::sim::panes::Row {
    let r = ::sim::panes::row_index(g.sim, js::to_u32(bi));
    assert!(r >= 0, "glass: pane {} has no row", bi);
    ::sim::panes::row_ref(g.sim, r as usize)
}
/// a queued piece, where `panes_q_addr` says it stands (the first of the run at place i of the queue)
fn q_at(s: usize, i: usize) -> ::sim::panes::Q {
    unsafe { *((::sim::state::addr(s) + ::sim::panes::q_addr(s) + i * core::mem::size_of::<::sim::panes::Q>()) as *const ::sim::panes::Q) }
}
/// `m[o]`, the doubles of what the last call let go
fn out_at(i: f64) -> f64 { unsafe { *(::sim::panes::out_addr() as *const f64).add(i as usize) } }
/// `m[at]`, their outlines' points
fn out_pt(i: f64) -> f64 { unsafe { *(::sim::panes::out_pts_addr() as *const f64).add(i as usize) } }
/// GB_STATS.x: what the host counts of what a fragment becomes, and what the module has counted at its place of panes.rs Tally
fn stat(host: f64, at: Option<f64>) -> f64 { host + tally(at) }
/// what `letGo` hands the shards part: the fragment with `key`, `heard` and `bi` left behind
fn fragment_like(f: &GlassBreakLetGoF) -> crate::shards::GlassFragmentLike {
    use crate::shards::{GlassFragmentLikeCause as C, GlassFragmentLikeKind as K};
    crate::shards::GlassFragmentLike {
        pane: f.pane,
        type_: f.type_,
        kind: match f.kind {
            GlassBreakLetGoFKind::Piece => K::Piece,
            GlassBreakLetGoFKind::Sheet => K::Sheet,
            GlassBreakLetGoFKind::Dagger => K::Dagger,
            GlassBreakLetGoFKind::Clump => K::Clump,
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
            GlassBreakLetGoFCause::Blast => C::Blast,
            GlassBreakLetGoFCause::Hit => C::Hit,
            GlassBreakLetGoFCause::Dice => C::Dice,
            GlassBreakLetGoFCause::Support => C::Support,
            GlassBreakLetGoFCause::Rake => C::Rake,
        },
        who: f.who,
        seed: f.seed,
        fall: f.fall.map(|x| crate::shards::GlassFragmentLikeFall { t: x.t, pos: x.pos }),
        landed: f.landed,
    }
}
/// …and what the stand-in takes of it
fn fragment_of(f: &GlassBreakLetGoF) -> GlassFragment {
    GlassFragment {
        pane: f.pane,
        type_: f.type_,
        kind: match f.kind {
            GlassBreakLetGoFKind::Piece => GlassFragmentKind::Piece,
            GlassBreakLetGoFKind::Sheet => GlassFragmentKind::Sheet,
            GlassBreakLetGoFKind::Dagger => GlassFragmentKind::Dagger,
            GlassBreakLetGoFKind::Clump => GlassFragmentKind::Clump,
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
            GlassBreakLetGoFCause::Blast => GlassFragmentCause::Blast,
            GlassBreakLetGoFCause::Hit => GlassFragmentCause::Hit,
            GlassBreakLetGoFCause::Dice => GlassFragmentCause::Dice,
            GlassBreakLetGoFCause::Support => GlassFragmentCause::Support,
            GlassBreakLetGoFCause::Rake => GlassFragmentCause::Rake,
        },
        who: f.who,
        seed: f.seed,
        fall: f.fall,
        landed: f.landed,
    }
}

/// The class's fields as made; its hooks are called by name, so what is hung here is which they are (glassbreak.ts GlassBreak.constructor)
pub fn glass_break_new(g: &mut crate::game::Game) -> crate::glassbreak::GlassBreak {
    crate::scene_collision::glass_rule(g, Some(crate::scene_collision::PaneRule::GlassBreak), Some(crate::scene_collision::PaneRows::GlassBreak));
    g.engine.world.glass_open = Some(crate::FnField);
    g.col.glass_walkable = Some(crate::scene_collision::GlassWalkable::GlassBreak);
    crate::destruction::destruction_on_blast(g, crate::destruction::BlastListener::Glass);
    GlassBreak::blank()
}

// ---------------------------------------------------------------- the call
/// a question or a rule of the module's, for a neighbour (glassbreak.ts GlassBreak.asking)
pub fn glass_break_asking<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> T) -> T {
    glass_break_ask(g, fn_)
}

/// a question or a rule of the module's, over this game's block and the copy of its box world as it stands: `fn_` gets the block's slot and the world's, and asks `::sim::panes_*` under the context set here (glassbreak.ts GlassBreak.ask)
pub fn glass_break_ask<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> T) -> T {
    crate::scene_collision::window(g, &mut |g| {
        crate::scene_collision::rules_statics(g);
        if !g.col.mirror.live { panic!("glass: the box world's copy will not stand for the rule: rust/sim/src/panes.rs has nothing to ask"); }
        if !g.m.rustknobs.rust_knobs.tick { crate::rustknobs::knobs_to_rust(g); }
        let dynamic = g.col.dynamic_boxes.clone();
        let set = crate::scene_box_mirror::box_mirror_leaves(g, &dynamic);
        if set == -2.0 { panic!("glass: more leaves than a list of the module holds"); }
        let (s, w) = (g.sim, g.col.mirror.slot);
        let shards = g.glass_break.on_release.is_some() && g.tune.glass_shards && glass_break_live(g);
        ::sim::panes_ctx(s, w as usize, set as i32, g.time, if shards { 1 } else { 0 }, crate::shard_model::shard().gravity);
        let r = fn_(g, s as f64, w);
        if ::sim::panes::take_wrote() { crate::scene_collision::wrote(g); }
        r
    })
}

/// the level's sheets of glass for the copy of its box world: each its static, how far it stands on the plan from the nearest door's opening, which face has the sky (glassbreak.ts GlassBreak.sheets)
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

/// the level has stood up: which glass each pane is, settled as the level stands (glassbreak.ts GlassBreak.stood)
pub fn glass_break_stood(g: &mut crate::game::Game) {
    g.glass_break.sheet_list = None;
    let mut l = glass_break_sheets(g);
    crate::scene_collision::glass_wrote(g);
    glass_break_ask(g, &mut |_g, _s, _w| {
        let mut i = 0;
        while i < l.len() {
            l[i + 2] = ::sim::panes_sky(js::to_u32(l[i])) as f64;
            i += 3;
        }
    });
    g.glass_break.sheet_list = Some(l);
    crate::scene_collision::glass_wrote(g);
}

/// metres on the plan from pane b to the nearest door's opening; Infinity with no door (glassbreak.ts GlassBreak.doorGap)
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

/// what the last call let go, handed on in the order it went; the list is read whole first, since what is told of a fragment may ask the module anything (glassbreak.ts GlassBreak.drain)
pub fn glass_break_drain(g: &mut crate::game::Game) {
    let n = ::sim::panes::out_n();
    if n == 0 { return; }
    let mut list: Vec<GlassBreakLetGoF> = Vec::with_capacity(n);
    for i in 0..n {
        let o = i as f64 * OUT_LEN;
        let bi = out_at(o + O_PANE);
        let at = 2.0 * out_at(o + O_PTS);
        let np = out_at(o + O_NPTS);
        let b = box_at(g, bi).unwrap();
        let v = glass_break_geom(g, bi);
        let mut outline = Vec::with_capacity(np as usize);
        let mut k = 0.0;
        while k < np {
            outline.push(glass_break_world(g, v, out_pt(at + 2.0 * k), out_pt(at + 2.0 * k + 1.0), None));
            k += 1.0;
        }
        let tint = v3::copy(seam_box(g, b).albedo);
        list.push(GlassBreakLetGoF {
            pane: bi,
            bi,
            type_: type_word(out_at(o + O_TYPE) as u32),
            kind: match out_at(o + O_KIND) as u32 { 0 => GlassBreakLetGoFKind::Piece, 1 => GlassBreakLetGoFKind::Dagger, 2 => GlassBreakLetGoFKind::Clump, _ => GlassBreakLetGoFKind::Sheet },
            outline,
            centre: [out_at(o + O_CX), out_at(o + O_CX + 1.0), out_at(o + O_CX + 2.0)],
            normal: [out_at(o + O_NX), out_at(o + O_NX + 1.0), out_at(o + O_NX + 2.0)],
            thickness: out_at(o + O_THICK),
            area: out_at(o + O_AREA),
            velocity: [out_at(o + O_VX), out_at(o + O_VX + 1.0), out_at(o + O_VX + 2.0)],
            spin: [out_at(o + O_SX), out_at(o + O_SX + 1.0), out_at(o + O_SX + 2.0)],
            tint,
            grain: out_at(o + O_GRAIN),
            cause: match out_at(o + O_CAUSE) as u32 { 0 => GlassBreakLetGoFCause::Hit, 1 => GlassBreakLetGoFCause::Support, 2 => GlassBreakLetGoFCause::Blast, 3 => GlassBreakLetGoFCause::Dice, _ => GlassBreakLetGoFCause::Rake },
            who: out_at(o + O_WHO),
            seed: out_at(o + O_SEED),
            key: out_at(o + O_KEY),
            heard: out_at(o + O_HEARD) != 0.0,
            landed: Some(out_at(o + O_LANDED) != 0.0),
            fall: if out_at(o + O_FALL_T) < 0.0 { None } else { Some(GlassFragmentFall { t: out_at(o + O_FALL_T), pos: [out_at(o + O_FALL_X), out_at(o + O_FALL_X + 1.0), out_at(o + O_FALL_X + 2.0)] }) },
        });
    }
    ::sim::panes::out_clear();
    for f in &list { glass_break_let_go(g, f); }
}

/// a fragment the rule let go, told to the game: its one way out (glassbreak.ts GlassBreak.letGo)
pub fn glass_break_let_go(g: &mut crate::game::Game, f: &crate::glassbreak::GlassBreakLetGoF) {
    let key = f.key;
    let v = glass_break_geom(g, f.bi);
    if f.heard {
        let dicing = f.kind == GlassBreakLetGoFKind::Clump || f.kind == GlassBreakLetGoFKind::Sheet;
        let rate = if dicing { 0.7 } else { 1.25 + 0.3 * crate::debris::hash01(key + v.key) };
        crate::audio_iface::game_audio_like_play(g, "glassCrack", Some(f.centre), Some(if dicing { 1.0 } else { 0.75 }), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
        crate::glass::glass_event(g, box_at(g, f.bi).unwrap(), f.centre, 1.0, f.who);
    }
    if g.glass_break.on_release.is_some() {
        g.m.glassbreak.gb_host.hooked += 1.0;
        crate::shards::glass_shards_on_release(g, &fragment_like(f));
        return;
    }
    glass_break_stand_in(g, v, &fragment_of(f), key, f.bi);
}

/// a pane's frame, for what the host still makes of a fragment (glassbreak.ts GlassBreak.geom)
pub fn glass_break_geom(g: &mut crate::game::Game, bi: f64) -> crate::glassbreak::PaneFrame {
    let box_ = box_at(g, bi).unwrap();
    let st = row_at(g, bi);
    PaneFrame { key: st.key as f64, u_axis: if st.u_axis == 0 { 0.0 } else { 2.0 }, hu: st.hu, hv: st.hv, ht: st.ht, box_ }
}

/// pane-local (u, v) → world (glassbreak.ts GlassBreak.world)
pub fn glass_break_world(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, u: f64, v_2: f64, w: Option<f64>) -> V3 {
    let (yaw, c) = { let b = seam_box(g, v.box_); (b.yaw, b.c) };
    world_on(yaw, c, v.u_axis, u, v_2, w.unwrap_or(0.0))
}

/// world → pane-local (u, v) (glassbreak.ts GlassBreak.uv)
pub fn glass_break_uv(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, x: V3) -> [f64; 2] {
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

/// no shards consumer attached: thin Debris boxes over the fragment and a burst of pale chips (glassbreak.ts GlassBreak.standIn)
pub fn glass_break_stand_in(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, f: &crate::glassbreak::GlassFragment, key: f64, bi: f64) {
    let mut uv = Vec::with_capacity(f.outline.len());
    for p in &f.outline { uv.push(glass_break_uv(g, v, *p)); }
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
            g.m.glassbreak.gb_host.refused += 1.0;
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
        g.m.glassbreak.gb_host.chips += n_chips;
    }
}

// ---------------------------------------------------------------- which glass
/// the type a pane breaks as when nobody has said: its wall's or its place's word (glassbreak.ts GlassBreak.typeOf)
pub fn glass_break_type_of(g: &mut crate::game::Game, b: BoxId, bi: Option<f64>) -> crate::glasspen::PaneType {
    let bi = match bi {
        Some(bi) => bi,
        None => match g.col.boxes.iter().position(|x| *x == b) { Some(i) => i as f64, None => -1.0 },   // indexOf
    };
    let t = glass_break_ask(g, &mut |g, _s, _w| {
        if bi >= 0.0 { return ::sim::panes_type_place(js::to_u32(bi)); }
        let (h, cy) = { let r = seam_box(g, b); (r.h, r.c[1]) };   // (a box the level does not hold, a tool's question: indoors, by its place)
        let gap = glass_break_door_gap(g, b);
        ::sim::panes_type_box(h[0], h[1], h[2], cy, gap)
    });
    type_word(t)
}

/// the type of pane bi as it will break; None for anything not a sheet (glassbreak.ts GlassBreak.typeOfPane)
pub fn glass_break_type_of_pane(g: &mut crate::game::Game, bi: f64) -> Option<crate::glasspen::PaneType> {
    let t = glass_break_ask(g, &mut |_g, _s, _w| ::sim::panes_type(js::to_u32(bi)));
    if t as f64 == NO_TYPE { None } else { Some(type_word(t)) }
}

/// make pane bi break as `type_` from here on, whole first; None = back to typeOf (glassbreak.ts GlassBreak.setType)
pub fn glass_break_set_type(g: &mut crate::game::Game, bi: f64, type_: Option<crate::glasspen::PaneType>) {
    let ty = match type_ { Some(t) => type_index(t), None => NO_OVER as u32 };
    glass_break_ask(g, &mut |_g, s, _w| { ::sim::panes::set_type(s as usize, js::to_u32(bi), ty); });
    crate::scene_collision::wrote(g);
    if let Some(src) = g.glass_break.gusts.get(Num(bi)).copied() {
        if let Some(i) = g.wind.all.iter().position(|s| *s == src) { g.wind.all.remove(i); }
        g.glass_break.gusts.delete(Num(bi));
    }
}

// ---------------------------------------------------------------- stimuli
/// THE IMPACT ENTRY POINT: a blow of `energy` met pane `bi` at world x going `dir`: the module files it and answers what the glass did (glassbreak.ts GlassBreak.impact)
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
    let v = glass_break_ask(g, &mut |g, _s, _w| {
        let r = match dir {
            Some(d) => ::sim::panes_impact(js::to_u32(bi), x[0], x[1], x[2], 1, d[0], d[1], d[2], energy, who, thr),
            None => ::sim::panes_impact(js::to_u32(bi), x[0], x[1], x[2], 0, 0.0, 0.0, 0.0, energy, who, thr),
        };
        glass_break_drain(g);
        r
    });
    res.live = v & 1 != 0;
    res.was_open = v & 2 != 0;
    res.punched = v & 4 != 0;
    res.diced = v & 8 != 0;
    res.cracked = v & 16 != 0;
    res.type_ = type_word((v >> 5) & 7);
    res.released = (v >> 8) as f64;
    res
}

/// the base's name for a round's impact at energy 1 (glassbreak.ts GlassBreak.hit)
pub fn glass_break_hit(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, who: f64, energy: Option<f64>) {
    let energy = energy.unwrap_or(1.0);
    glass_break_impact(g, bi, x, dir, energy, who, None);
}

/// a pellet through glass its own shell has just broken: no blow filed, but a standing piece of an annealed pane goes with it; fragments let go (glassbreak.ts GlassBreak.widen)
pub fn glass_break_widen(g: &mut crate::game::Game, bi: f64, x: V3) -> f64 {
    if !glass_break_live(g) { return 0.0; }
    let n = glass_break_ask(g, &mut |g, _s, _w| {
        let r = ::sim::panes_widen(js::to_u32(bi), x[0], x[1], x[2]);
        glass_break_drain(g);
        r
    });
    n as f64
}

/// A BLAST at `at`: every sheet in reach takes it by type and by k = power × (1 − d / radius); the panes it changed (glassbreak.ts GlassBreak.blast)
pub fn glass_break_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>) -> f64 {
    let opts = opts.unwrap_or_default();
    if !glass_break_live(g) { return 0.0; }
    let radius = opts.radius.unwrap_or(g.k.glassbreak.blast_m);
    let power = opts.power.unwrap_or(1.0);
    let who = opts.who.unwrap_or(0.0);
    let mut changed = 0.0;
    if radius > 0.0 && power > 0.0 {
        glass_break_ask(g, &mut |g, _s, _w| {
            ::sim::panes_blast_begin(at[0], at[1], at[2]);
            let n = ::sim::panes_sheets() as usize;
            for i in 0..n {
                if ::sim::panes_blast_sheet(i, at[0], at[1], at[2], radius, power, who) != 0 { changed += 1.0; }
                if ::sim::panes::take_wrote() { crate::scene_collision::wrote(g); }
                glass_break_drain(g);
            }
        });
    }
    changed
}

/// a stun charge went off at `at`: a blast at bangPower (glassbreak.ts GlassBreak.bang)
pub fn glass_break_bang(g: &mut crate::game::Game, at: V3, who: f64) -> f64 {
    let opts = GlassBlastOpts { radius: Some(g.k.glassbreak.blast_m), power: Some(g.k.glassbreak.bang_power), who: Some(who) };
    glass_break_blast(g, at, Some(opts))
}

// ---------------------------------------------------------------- a burst's view of the glass
/// where the rule's state stands: the slot is `g.sim`, so what comes back says only that it is the module's (glassbreak.ts GlassBreak.home)
pub fn glass_break_home(g: &mut crate::game::Game) -> Option<crate::Opaque> {
    Some(crate::Opaque)
}

/// for the length of `fn_`, every solid walk and every ray also passes a pane where a burst at `from` of this size sees no glass (glassbreak.ts GlassBreak.seeThrough)
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

/// the view in hand as numbers: which view, the burst's point, reach and power, whether a man counts on it (glassbreak.ts GlassBreak.view)
pub fn glass_break_view(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneView> {
    let v = g.glass_break.view_of?;
    Some(PaneView {
        kind: if v.kind == GlassBreakViewOfKind::Blown { VIEW_BLOWN } else { VIEW_GIVES },
        bi: Vec::new(),
        at: vec![v.from[0], v.from[1], v.from[2], v.size.radius.unwrap_or(g.k.glassbreak.blast_m), v.size.power.unwrap_or(1.0), if v.kind == GlassBreakViewOfKind::Counted { 1.0 } else { 0.0 }],
    })
}

/// the rule's whole answer at a point: a piece out, or the view in hand counts the glass there out (glassbreak.ts GlassBreak.ruleOpen)
pub fn glass_break_rule_open(g: &mut crate::game::Game, bi: f64, p: V3) -> bool {
    if g.glass_break.view_of.is_none() { return glass_break_open_at(g, bi, p); }
    glass_break_ask(g, &mut |_g, _s, w| ::sim::glass_open(w as usize, js::to_u32(bi), p[0], p[1], p[2]) == 1)
}

// ---------------------------------------------------------------- per frame
/// per frame: the gusts, then every touched pane's step in the module, draining after each (glassbreak.ts GlassBreak.update)
pub fn glass_break_update(g: &mut crate::game::Game, dt: f64) {
    let touched = !glass_break_untouched(g);
    let s = g.sim;
    if touched && ::sim::panes::take_gust(s) { g.glass_break.gust_dirty = true; }
    if g.glass_break.gusts.len() > 0 || g.glass_break.gust_dirty {
        glass_break_update_gusts(g);
        g.glass_break.gust_dirty = g.glass_break.gusts.len() > 0;
    }
    if !glass_break_live(g) { return; }
    let n = if touched { ::sim::panes::row_n(s) as usize } else { 0 };
    if n > 0 {
        glass_break_ask(g, &mut |g, gs, _w| {
            ::sim::panes::frame_begin(gs as usize);
            // who may walk into what is left of a pane: any player's man, on his feet with his hands free (seats.ts)
            let men = ::sim::panes::men_addr() as *mut f64;
            let mut k = 0usize;
            for pl in crate::seats::men(g) {
                if g.pl[pl].down || g.pl[pl].holding.is_some() || g.pl[pl].dragging.is_some() || g.pl[pl].wall.is_some() { continue; }
                let c = g.pl[pl].ch;
                let (pos, vel, id) = (g.ch[c].pos, g.ch[c].vel, g.ch[c].id);
                if k < ::sim::panes::MAX_MEN {
                    let row = [pos[0], pos[1], pos[2], vel[0], vel[1], vel[2], id];
                    for (i, x) in row.iter().enumerate() {
                        unsafe { *men.add(::sim::panes::MAN * k + i) = *x; }
                    }
                }
                k += 1;
            }
            ::sim::panes::men_set(k);
            for r in 0..n {
                ::sim::panes_step(r);
                if ::sim::panes::take_wrote() { crate::scene_collision::wrote(g); }
                glass_break_drain(g);
            }
        });
    }
}

/// a wind source through each shot-out exterior pane, its speed each frame the storm at that face times the open share (glassbreak.ts GlassBreak.updateGusts)
pub fn glass_break_update_gusts(g: &mut crate::game::Game) {
    let swv = crate::wind::wind_storm_wind(g);
    let sw = swv.map(|s| [s[0], s[2]]);
    let storm_speed = match sw { Some(s) => m::hyp2(s[0], s[1]), None => 0.0 };
    let s = g.sim;
    let n = ::sim::panes::row_n(s) as usize;
    for r in 0..n {
        let (exterior, bi, st_key, u_axis, hu) = { let st = ::sim::panes::row_ref(s, r); (st.exterior, st.w[0] as f64, st.key, st.u_axis, st.hu) };
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
                let sky = glass_break_ask(g, &mut |_g, _s, _w| ::sim::panes_sky(js::to_u32(bi)));
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
                    ..crate::wind::WindSource::zeroed()
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
pub fn glass_break_untouched(g: &mut crate::game::Game) -> bool {
    ::sim::state::state(g.sim).panes.used == 0
}

/// is world point x on pane bi where a piece has fallen out (glassbreak.ts GlassBreak.openAt)
pub fn glass_break_open_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool {
    !glass_break_untouched(g) && ::sim::panes::open_at(g.sim, js::to_u32(bi), x[0], x[1], x[2])
}

/// the movement rule's answer for pane bi, false for a pane never hit (glassbreak.ts GlassBreak.walkable)
pub fn glass_break_walkable(g: &mut crate::game::Game, bi: f64) -> bool {
    !glass_break_untouched(g) && ::sim::panes::walkable(g.sim, js::to_u32(bi))
}

/// a number that changes whenever what is open in any pane changes (glassbreak.ts GlassBreak.openSig)
pub fn glass_break_open_sig(g: &mut crate::game::Game) -> f64 {
    if glass_break_untouched(g) { 0.0 } else { ::sim::panes::open_sig(g.sim) as f64 }
}

/// open fraction of pane bi by area, 0 = whole (glassbreak.ts GlassBreak.openFraction)
pub fn glass_break_open_fraction(g: &mut crate::game::Game, bi: f64) -> f64 {
    if glass_break_untouched(g) { 0.0 } else { ::sim::panes::open_fraction(g.sim, js::to_u32(bi)) }
}

/// what a sibling needs to know about pane bi without touching it; None = not a sheet of glass (glassbreak.ts GlassBreak.paneInfo)
pub fn glass_break_pane_info(g: &mut crate::game::Game, bi: f64) -> Option<crate::glassbreak::GlassBreakPaneInfo> {
    let type_ = glass_break_type_of_pane(g, bi)?;
    let b = box_at(g, bi).unwrap();
    let h = seam_box(g, b).h;
    // (`this.panes.get(bi)`: only the row's own few words are read of it)
    let touched = !glass_break_untouched(g);
    let r = if touched { ::sim::panes::row_index(g.sim, js::to_u32(bi)) } else { -1 };
    let (gone, run, sites, energy) = if r >= 0 {
        let st = ::sim::panes::row_ref(g.sim, r as usize);
        let flags = st.w[GW_FLAGS as usize];
        (flags & G_GONE as u32 != 0, flags & G_RUN as u32 != 0, (flags >> 8) as f64, st.energy)
    } else {
        (false, false, 0.0, 0.0)
    };
    let exterior = glass_break_ask(g, &mut |_g, _s, _w| ::sim::panes_sky(js::to_u32(bi))) != 0;
    let open_fraction = glass_break_open_fraction(g, bi);
    let walkable = glass_break_walkable(g, bi);
    Some(GlassBreakPaneInfo { type_, thickness: 2.0 * m::min(h[0], h[2]), exterior, open_fraction, walkable, gone, run, sites, energy })
}

/// the panes that have been touched, as a tool or the picture reads them (a copy, made when asked; nothing here is kept) (glassbreak.ts GlassBreak.panes)
pub fn glass_break_panes(g: &mut crate::game::Game) -> JsMap<crate::Num, crate::glassbreak::PaneState> {
    let mut out = JsMap::new();
    if glass_break_untouched(g) { return out; }
    let s = g.sim as f64;
    let n = ::sim::panes::row_n(g.sim);
    let mut r = 0;
    while r < n {
        let bi = s0_bi(g, s, r as f64);
        out.set(Num(bi), glass_break_handle(g, bi));
        r += 1;
    }
    out
}

/// a pane as a tool holds it: the pane as it stands when asked (glassbreak.ts GlassBreak.handle; a value here, no Proxy: see the notes)
pub fn glass_break_handle(g: &mut crate::game::Game, bi: f64) -> crate::glassbreak::PaneState {
    let s = g.sim;
    let r = ::sim::panes::row_index(s, js::to_u32(bi));
    assert!(r >= 0, "glass: pane {} has no row", bi);
    glass_break_row_state(g, s as f64, r as f64)
}

/// row r of the module, read out as the TypeScript's PaneState (glassbreak.ts GlassBreak.rowState)
pub fn glass_break_row_state(g: &mut crate::game::Game, s: f64, r: f64) -> crate::glassbreak::PaneState {
    let s = s as usize;
    let st = *::sim::panes::row_ref(s, r as usize);   // (a read: the row's words as they stand, and no reference held over what comes next)
    let flags = st.w[GW_FLAGS as usize];
    let ns = (flags >> 8) as usize;
    let bi = st.w[0] as f64;
    let mut sites: Vec<GpSite> = Vec::with_capacity(ns);
    for i in 0..ns {
        let q = i * G_SITE as usize;
        sites.push(GpSite {
            u: st.d[q], v: st.d[q + 1], rot: st.d[q + 2],
            rho: [st.d[q + 3], st.d[q + 4], st.d[q + 5], st.d[q + 6]],
            rz: st.d[q + 7], merge: st.d[q + 8],
            n: st.w[GW_N as usize + i] as f64, seed: st.w[GW_SEED as usize + i] as f64,
            e: st.e[i], hits: st.hits[i],
        });
    }
    let n_bits = (GP_MAX_SITES * GP_FRAGS_PER_SITE) as usize;
    let mut open: Vec<u8> = vec![0; n_bits];
    for k in 0..n_bits {
        if st.w[GW_OPEN as usize + (k >> 5)] >> (k & 31) & 1 != 0 { open[k] = 1; }
    }
    let mut pending = Vec::with_capacity(st.qn as usize);
    for i in 0..st.qn as usize {
        let e = q_at(s, st.q0 as usize + i);
        let cause = match e.cause { 0 => PendingCause::Hit, 1 => PendingCause::Support, 2 => PendingCause::Blast, 3 => PendingCause::Dice, _ => PendingCause::Rake };
        pending.push(Pending { fid: e.fid as f64, at: e.at, cause, from: if cause == PendingCause::Blast || cause == PendingCause::Rake { Some(e.from) } else { None }, k: e.k });
    }
    let mut dice = None;
    if flags & G_DICE as u32 != 0 {
        let at = G_DICE_AT as usize;
        let d = GpDice { u: st.d[at], v: st.d[at + 1], cols: st.w[GW_COLS as usize] as f64, rows: st.w[GW_ROWS as usize] as f64, seed: st.w[GW_DSEED as usize] as f64, hu: st.d[at + 2], hv: st.d[at + 3] };
        let mut clumps: Vec<Clump> = Vec::new();
        let mut rr = 0.0;
        while rr < d.rows {
            let mut c = 0.0;
            while c < d.cols {
                clumps.push(Clump { k: rr * d.cols + c, c, r: rr, key: gp_dice_key(d, c, rr) });
                c += 1.0;
            }
            rr += 1.0;
        }
        js::sort_by(&mut clumps, |a, b| js::or(a.key - b.key, a.k - b.k));
        let next = st.d_next as usize;
        let mut out: Vec<u8> = vec![0; (d.cols * d.rows) as usize];
        for i in 0..next { out[clumps[i].k as usize] = 1; }
        dice = Some(Dice { t0: st.d[at + 6], clumps, next: next as f64, out, u: d.u, v: d.v, cols: d.cols, rows: d.rows, seed: d.seed, hu: d.hu, hv: d.hv });
    }
    let box_ = box_at(g, bi).unwrap();
    PaneState {
        bi,
        box_,
        key: st.key as f64,
        type_: type_word(st.ty),
        u_axis: if st.u_axis == 0 { 0.0 } else { 2.0 },
        hu: st.hu,
        hv: st.hv,
        ht: st.ht,
        exterior: st.exterior != 0,
        sites,
        run: flags & G_RUN as u32 != 0,
        open,
        n_open: st.n_open as f64,
        hits: st.hit_n as f64,
        energy: st.energy,
        damage: st.damage,
        frags: None,   // (`get frags()`, made when read, and nothing of the game's reads it: see the notes)
        pending,
        next_think: st.next_think,
        dice,
        gone: flags & G_GONE as u32 != 0,
        last_dir: st.last_dir,
        last_who: st.who,
        last_uv: st.last_uv,
        sound_at: st.sound_at,
        walkable: st.walkable != 0,
        log: Vec::new(),
    }
}

/// a tool's question: which pieces of this pane are held, a set of piece ids (glassbreak.ts GlassBreak.heldSet)
pub fn glass_break_held_set(g: &mut crate::game::Game, without: Option<f64>, bi: f64) -> JsSet<crate::Num> {
    let without = without.unwrap_or(-1.0);
    let w = ::sim::panes::held_bits(g.sim, js::to_u32(bi), js::to_i32(without));
    let mut out = JsSet::new();
    for k in 0..(GP_MAX_SITES * GP_FRAGS_PER_SITE) as usize {
        if w[k >> 5] >> (k & 31) & 1 != 0 { out.add(Num(k as f64)); }
    }
    out
}

/// world point → pane-local (u, v), the frame of a pane in hand (glassbreak.ts GlassBreak.toUV)
pub fn glass_break_to_uv(g: &mut crate::game::Game, x: V3, bi: f64) -> [f64; 2] {
    let v = glass_break_geom(g, bi);
    glass_break_uv(g, v, x)
}

/// pane-local (u, v, w across the thickness) → world (glassbreak.ts GlassBreak.toWorld)
pub fn glass_break_to_world(g: &mut crate::game::Game, u: f64, v: f64, w: Option<f64>, bi: f64) -> V3 {
    let f = glass_break_geom(g, bi);
    glass_break_world(g, f, u, v, w)
}

/// the pane's unit normal = the direction of pane-local +w (glassbreak.ts GlassBreak.normalOf)
pub fn glass_break_normal_of(g: &mut crate::game::Game, bi: f64) -> V3 {
    let f = glass_break_geom(g, bi);
    let yaw = seam_box(g, f.box_).yaw;
    normal_on(yaw, f.u_axis)
}

/// a burst's reckoning, for a tool: is the glass of pane bi at x on its way out (glassbreak.ts GlassBreak.goingAt)
pub fn glass_break_going_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool {
    glass_break_see_through(g, GlassBreakSeeThroughView::Blown, [0.0, 0.0, 0.0], GlassBlastOpts::zeroed(), &mut |g| glass_break_rule_open(g, bi, x))
}

/// a burst's reckoning, for a tool: would a blast of this size at `at` take pane bi out of the way (glassbreak.ts GlassBreak.wouldGive)
pub fn glass_break_would_give(g: &mut crate::game::Game, bi: f64, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>, x: Option<V3>) -> bool {
    if !glass_break_live(g) { return false; }
    let opts = opts.unwrap_or_default();
    let view = if x.is_some() { GlassBreakSeeThroughView::Counted } else { GlassBreakSeeThroughView::Wary };
    glass_break_see_through(g, view, at, opts, &mut |g| {
        let p = match x { Some(x) => x, None => { let b = box_at(g, bi).unwrap(); seam_box(g, b).c } };
        glass_break_rule_open(g, bi, p)
    })
}

/// the record for the renderer, GB_RECORD_ROWS vec4 rows: reads the pane, writes nothing of it (glassbreak.ts GlassBreak.fill)
pub fn glass_break_fill(g: &mut crate::game::Game, bi: f64, dst: &mut Vec<f32>, off: f64) -> bool {
    let s = g.sim;
    let r = ::sim::panes::row_index(s, js::to_u32(bi));
    if r < 0 { return false; }
    let st = *::sim::panes::row_ref(s, r as usize);
    let flags = st.w[GW_FLAGS as usize];
    let ns = (flags >> 8) as usize;
    let gone = flags & G_GONE as u32 != 0;
    let dice = flags & G_DICE as u32 != 0;
    if ns == 0 && !dice && !gone { return false; }
    let off = off as usize;
    let any_open = gone || st.n_open > 0 || (dice && st.d_next > 0);
    dst[off] = ns as f32;
    dst[off + 1] = ((if any_open { 1 } else { 0 }) | (if flags & G_RUN as u32 != 0 { 2 } else { 0 }) | (if dice && !gone { 4 } else { 0 }) | (if gone { 8 } else { 0 }) | (if st.ty == 2 { 16 } else { 0 })) as f32;
    dst[off + 2] = if st.u_axis == 0 { 0.0 } else { 1.0 };
    dst[off + 3] = st.damage as f32;
    dst[off + 4] = st.hu as f32;
    dst[off + 5] = st.hv as f32;
    dst[off + 6] = st.ty as f32;
    dst[off + 7] = 0.0;
    if dice {
        let at = G_DICE_AT as usize;
        dst[off + 8] = st.d[at] as f32;
        dst[off + 9] = st.d[at + 1] as f32;
        dst[off + 10] = ::sim::panes::dice_threshold(s, js::to_u32(bi)) as f32;
        dst[off + 11] = g.k.glassbreak.dice_grain as f32;
        dst[off + 12] = st.w[GW_COLS as usize] as f32;
        dst[off + 13] = st.w[GW_ROWS as usize] as f32;
        dst[off + 14] = st.w[GW_DSEED as usize] as f32;
    }
    let (per_site, bands, sectors) = (GP_FRAGS_PER_SITE as usize, GP_BANDS as usize, GP_MAX_SECTORS as usize);
    for i in 0..ns {
        let q = i * G_SITE as usize;
        let n = st.w[GW_N as usize + i] as usize;
        let o = off + (GB_HEADER_ROWS as usize + i * GB_SITE_ROWS as usize) * 4;
        let mut mk = [0i32; 7];
        for b in 0..bands {
            for k in 0..n {
                let fid = i * per_site + b * sectors + k;
                if st.w[GW_OPEN as usize + (fid >> 5)] >> (fid & 31) & 1 != 0 { mk[b] |= 1 << k; }
            }
        }
        dst[o] = st.d[q] as f32;
        dst[o + 1] = st.d[q + 1] as f32;
        dst[o + 2] = st.d[q + 2] as f32;
        dst[o + 3] = st.d[q + 7] as f32;
        dst[o + 4] = n as f32;
        dst[o + 5] = st.w[GW_SEED as usize + i] as f32;
        dst[o + 6] = m::min(8.0, st.e[i]) as f32;
        dst[o + 7] = st.d[q + 8] as f32;
        dst[o + 8] = st.d[q + 3] as f32;
        dst[o + 9] = st.d[q + 4] as f32;
        dst[o + 10] = st.d[q + 5] as f32;
        dst[o + 11] = st.d[q + 6] as f32;
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
    let panes = glass_break_panes(g);
    let t = g.m.glassbreak.gb_host;
    let at = tally_at();
    let mut s = format!("glass breaking {} ▸ {} panes touched · {} impacts · {} stars · {} run · {} fragments out ({} clumps) · diced {}", state, panes.len(), stat(t.impacts, at.impacts), stat(t.sites, at.sites), stat(t.runs, at.runs), stat(t.dropped, at.dropped), stat(t.clumps, at.clumps), stat(t.diced, at.diced));
    if g.glass_break.on_release.is_some() {
        s.push_str(&format!(" · {} to the shards hook", stat(t.hooked, at.hooked)));
    } else {
        s.push_str(&format!(" · stand-in: {} shards ({} refused), {} chips", stat(t.shards, at.shards), stat(t.refused, at.refused), stat(t.chips, at.chips)));
    }
    for st in panes.values() {
        if st.n_open != 0.0 || st.dice.is_some() || st.gone {
            let open = m::round(100.0 * glass_break_open_fraction(g, st.bi));
            s.push_str(&format!(" | pane {} {} {}% open{}{}", st.bi, st.type_.as_str(), open, if st.run { " run" } else { "" }, if st.walkable { " (walkable)" } else { "" }));
        }
    }
    s
}

/// every pane whole, the gust sources gone (glassbreak.ts GlassBreak.reset)
pub fn glass_break_reset(g: &mut crate::game::Game) {
    ::sim::panes::reset(g.sim);
    crate::scene_collision::wrote(g);
    let gusts: Vec<WindWindSourceId> = g.glass_break.gusts.values().copied().collect();
    for src in gusts {
        if let Some(i) = g.wind.all.iter().position(|s| *s == src) { g.wind.all.remove(i); }
    }
    g.glass_break.gusts.clear();
    g.glass_break.gust_dirty = false;
}

/// row r's static (glassbreak.ts s0Bi)
pub fn s0_bi(g: &mut crate::game::Game, s: f64, r: f64) -> f64 {
    ::sim::panes::row_ref(s as usize, r as usize).w[0] as f64
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
        for (t, want) in [(::sim::panes::ANNEALED, PaneType::Annealed), (::sim::panes::TEMPERED, PaneType::Tempered), (::sim::panes::LAMINATED, PaneType::Laminated)] {
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
        g.glass_break.view_of = Some(GlassBreakViewOf { kind: GlassBreakViewOfKind::Blown, from: [0.0, 0.0, 0.0], size: GlassBlastOpts::zeroed() });
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
        assert_eq!(glass_break_panes(&mut g).len(), 0);
        assert!(!glass_break_fill(&mut g, 3.0, &mut vec![0.0f32; 4], 0.0));
        assert_eq!(::sim::state::state(g.sim).panes.used, 0);
    }
    // the queue's first place, by panes_q_addr from the block's start, stands where the module's rows begin when nothing stands before it
    #[test]
    fn a_queue_place_is_where_the_module_says() {
        assert_eq!(::sim::panes::row_n(1), 0);
        assert_eq!(::sim::state::addr(1) + ::sim::panes::q_addr(1), ::sim::panes::row_ref(1, 0) as *mut ::sim::panes::Row as usize);
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
