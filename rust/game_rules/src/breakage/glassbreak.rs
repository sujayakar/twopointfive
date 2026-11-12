//! Glass breaking, thin over rust/sim's panes.rs.

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

use crate::ask::Over;
use ::sim::panes::{self, Ctx};
use crate::glasspen::PaneType;
use ::sim::world::{Sheet, VIEW_BLOWN, VIEW_GIVES, VIEW_NONE};
use crate::Num;

// ---------------------------------------------------------------- a glass's word and its number
/// a glass's number as panes.rs's word for it; a number off the list reads laminated (only impact can meet one)
fn type_word(t: u32) -> PaneType {
    match t { 0 => PaneType::Annealed, 1 => PaneType::Tempered, _ => PaneType::Laminated }
}
/// a glass's word as its number
fn type_index(t: PaneType) -> u32 {
    match t { PaneType::Annealed => 0, PaneType::Tempered => 1, PaneType::Laminated => 2 }
}
/// the panes that have been touched
pub fn touched(g: &Game) -> &panes::Panes { &g.tab.boxes.world().panes }
/// the row of pane bi. Only a pane with a row is asked (a fragment's, a tool's in hand)
fn row_at(g: &Game, bi: f64) -> &panes::Row {
    let p = touched(g);
    &p.rows[p.row_of(js::to_u32(bi)).unwrap_or_else(|| panic!("glass: pane {} has no row", bi))]
}
/// The class's fields as made; its hooks are called by name, so what is hung here is which they are
pub fn glass_break_new(g: &mut crate::game::Game) -> crate::glassbreak::GlassBreak {
    let sheets = level_sheets(g);
    g.tab.boxes.glass(sheets);
    crate::destruction::destruction_on_blast(g, crate::destruction::BlastListener::Glass);
    GlassBreak::blank()
}

// ---------------------------------------------------------------- the call
/// a rule of the panes, over this game's box world as it stands: `ask` asks `::sim::panes` under the context it is handed. What the rule did to the game is
/// carried over: what a walker goes through, what will come down, and what it let go, handed on in the order it went
pub fn glass_break_asking<T>(g: &mut Game, ask: impl FnOnce(&mut Ctx, &crate::knobs::Knobs) -> T) -> T {
    let (now, shards, gravity) = (g.time, crate::shards::shards_live(g), crate::shard_model::shard().gravity);
    let mut cx = Ctx { w: g.tab.boxes.lent(), now, shards, gravity, out: Vec::new(), landings: Vec::new(), wrote: false };
    let r = ask(&mut cx, &g.k);
    let Ctx { wrote, landings, out, .. } = cx;
    if wrote { glass_walked(g); }
    for l in landings { crate::falls::schedule(&mut g.falls, crate::falls::FallKind::Glass, l, g.k.shards.pending_max); }
    if !out.is_empty() { glass_break_drain(g, out); }
    r
}
/// the box world is told which panes a walker goes through
fn glass_walked(g: &mut Game) {
    for i in 0..g.tab.boxes.world().sheets.len() {
        let bi = g.tab.boxes.world().sheets[i].bi;
        let on = glass_break_walkable(g, bi as f64);
        g.tab.boxes.walk(bi as usize, g.col.boxes[bi as usize], on);
    }
}

/// the level's sheets of glass as the box world lists them: each its static, how far it stands on the plan from the nearest door's opening, which face has the sky (not known yet)
fn level_sheets(g: &mut Game) -> Vec<Sheet> {
    let panes = crate::glass::glass_panes(g);
    panes.into_iter().filter_map(|bi| {
        let b = g.box_at(bi).filter(|&b| crate::glass::is_sheet(g, Some(b)))?;
        Some(Sheet { bi: bi as u32, sky: SKY_UNKNOWN, gap: glass_break_door_gap(g, b) })
    }).collect()
}

/// the level has stood up: which glass each pane is, settled as the level stands
pub fn glass_break_stood(g: &mut crate::game::Game) {
    let mut sheets = level_sheets(g);
    g.tab.boxes.glass(sheets.clone());
    for s in &mut sheets { s.sky = panes::sky_known(g.tab.boxes.world(), s.bi, &g.k); }
    g.tab.boxes.glass(sheets);
}

/// metres on the plan from pane b to the nearest door's opening; Infinity with no door
fn glass_break_door_gap(g: &Game, b: BoxId) -> f64 {
    let (yaw, bc, bh) = { let r = &g.tab.boxes[b]; (r.yaw, r.c, r.h) };
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

/// what a call let go, handed on in the order it went; the list is taken whole first, since what is told of a fragment may ask the panes anything
fn glass_break_drain(g: &mut crate::game::Game, out: Vec<panes::LetGo>) {
    let list: Vec<crate::glassbreak::Fragment> = out.into_iter().map(|l| {
        let v = glass_break_geom(g, l.f.bi);
        let outline = l.pts.iter().map(|p| glass_break_world(g, v, p.0, p.1, None)).collect();
        let tint = g.tab.boxes[v.box_].albedo;
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
        crate::glass::glass_event(g, g.box_at(f.bi).unwrap(), f.centre, 1.0, f.who);
    }
    if crate::shards::shards_live(g) {
        g.m.glassbreak.gb_host.hooked += 1.0;
        crate::shards::glass_shards_release(g, f);
        return;
    }
    glass_break_stand_in(g, v, f);
}

/// a pane's frame, for what the host still makes of a fragment
fn glass_break_geom(g: &crate::game::Game, bi: f64) -> crate::glassbreak::PaneFrame {
    let box_ = g.box_at(bi).unwrap();
    let st = row_at(g, bi);
    PaneFrame { key: st.key as f64, u_axis: if st.u_axis == 0 { 0.0 } else { 2.0 }, ht: st.ht, box_ }
}

/// pane-local (u, v) → world
fn glass_break_world(g: &crate::game::Game, v: crate::glassbreak::PaneFrame, u: f64, v_2: f64, w: Option<f64>) -> V3 {
    let (yaw, c) = { let b = &g.tab.boxes[v.box_]; (b.yaw, b.c) };
    world_on(yaw, c, v.u_axis, u, v_2, w.unwrap_or(0.0))
}

/// world → pane-local (u, v)
fn glass_break_uv(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, x: V3) -> [f64; 2] {
    let (yaw, c) = { let b = &g.tab.boxes[v.box_]; (b.yaw, b.c) };
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
    let yaw = g.tab.boxes[v.box_].yaw;
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
            flags: Some(crate::scene_boxes::box_flag::DYNAMIC as f64),
            ..Default::default()
        };
        let bx = crate::scene_boxes::make_box(g, &p);
        let ch = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Shard, bx, v3::copy(f.velocity), v3::copy(f.spin), Some(f.who), None);
        if let Some(ch) = ch {
            let chunk_box = g.tab.debris_chunk[ch].box_;
            g.tab.edit(chunk_box).rough = Some(0.05);
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
pub fn glass_break_type_of_pane(g: &Game, bi: f64) -> Option<crate::glasspen::PaneType> {
    let t = panes::type_of_pane(g.tab.boxes.world(), js::to_u32(bi), &g.k);
    if t as f64 == NO_TYPE { None } else { Some(type_word(t)) }
}

/// make pane bi break as `type_` from here on, whole first; None = back to typeOf
pub fn glass_break_set_type(g: &mut crate::game::Game, bi: f64, type_: Option<crate::glasspen::PaneType>) {
    let ty = match type_ { Some(t) => type_index(t), None => NO_OVER as u32 };
    panes::set_type(g.tab.boxes.panes(), js::to_u32(bi), ty);
    glass_walked(g);
    if let Some(src) = g.glass_break.gusts.get(Num(bi)).copied() {
        if let Some(i) = g.wind.all.iter().position(|s| *s == src) { g.wind.all.remove(i); }
        g.glass_break.gusts.delete(Num(bi));
    }
}

// ---------------------------------------------------------------- stimuli
/// THE IMPACT ENTRY POINT: a blow of `energy` met pane `bi` at world x going `dir`: the module files it and answers what the glass did
pub fn glass_break_impact(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact {
    let b = g.box_at(bi);
    let shape = match b { Some(b) => { let r = &g.tab.boxes[b]; Some((r.flags, r.h)) } None => None };
    let is_glass = match shape { Some((flags, _)) => (js::to_i32(flags) & crate::scene_boxes::box_flag::GLASS as i32) != 0, None => false };
    let thickness = match (is_glass, shape) { (true, Some((_, h))) => 2.0 * m::min(h[0], h[2]), _ => 0.0 };
    let mut res = GlassImpact { pane: is_glass, live: false, type_: PaneType::Annealed, thickness, was_open: false, punched: false, diced: false, released: 0.0, cracked: false };
    if !is_glass { return res; }
    if !glass_break_live(g) {
        res.type_ = glass_break_type_of_pane(g, bi).unwrap_or(PaneType::Laminated);
        return res;
    }
    let thr = match through { None => -1, Some(true) => 1, Some(false) => 0 };
    let v = glass_break_asking(g, |cx, k| panes::impact(cx, js::to_u32(bi), x, dir, energy, who, thr, k));
    (res.live, res.was_open, res.punched, res.diced, res.cracked) = (v.live, v.was_open, v.punched, v.diced, v.cracked);
    res.type_ = type_word(v.ty);
    res.released = v.released as f64;
    res
}

/// a pellet through glass its own shell has just broken: no blow filed, but a standing piece of an annealed pane goes with it; fragments let go
pub fn glass_break_widen(g: &mut crate::game::Game, bi: f64, x: V3) -> f64 {
    if !glass_break_live(g) { return 0.0; }
    glass_break_asking(g, |cx, k| panes::widen(cx, js::to_u32(bi), x, k)) as f64
}

/// A BLAST at `at`, in the world as it takes it: every sheet in reach that it has a line to (panes.rs `blast_meets`, asked of them all before any breaks) takes it by type
/// and by k = power × (1 − d / radius); the panes it changed
pub fn glass_break_blast(g: &mut crate::game::Game, over: &Over, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>) -> f64 {
    let opts = opts.unwrap_or_default();
    if !glass_break_live(g) { return 0.0; }
    let radius = opts.radius.unwrap_or(g.k.glassbreak.blast_m);
    let power = opts.power.unwrap_or(1.0);
    let who = opts.who.unwrap_or(0.0);
    let mut changed = 0.0;
    if radius > 0.0 && power > 0.0 {
        let met = crate::scene_collision::panes_met(g, over, at, radius);
        let arrival = glass_break_asking(g, |cx, _| panes::blast_begin(cx, at));
        for pane in met {
            if glass_break_asking(g, |cx, k| panes::blast_sheet(cx, arrival, pane, at, radius, power, who, k)) { changed += 1.0; }
        }
    }
    changed
}

/// a stun charge went off at `at`: a blast at bangPower
pub fn glass_break_bang(g: &mut crate::game::Game, at: V3, who: f64) -> f64 {
    let opts = GlassBlastOpts { radius: Some(g.k.glassbreak.blast_m), power: Some(g.k.glassbreak.bang_power), who: Some(who) };
    glass_break_blast(g, &Over::OWN, at, Some(opts))
}

// ---------------------------------------------------------------- a burst's view of the glass
/// for the length of `fn_`, every solid walk and every ray also passes a pane where a burst at `from` of this size sees no glass, in the world as the question takes it
pub fn glass_break_see_through<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, over: &Over, view: crate::glassbreak::GlassBreakViewOfKind, from: V3, size: crate::glassbreak::GlassBlastOpts, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let kind = view;
    let was = g.glass_break.view_of.replace(GlassBreakViewOf { kind, from, size });
    let shown = glass_viewed(g, over);
    let out = fn_(g);
    g.glass_break.view_of = was;
    g.tab.boxes.view(shown);
    out
}

/// the box world is told the view in hand, as numbers: which view, the burst's point, reach and power, whether a man counts on it; and which panes such a burst meets.
/// → the view it had
fn glass_viewed(g: &mut Game, over: &Over) -> ::sim::world::View {
    let (kind, at) = match g.glass_break.view_of {
        None => (VIEW_NONE, [0.0; 6]),
        Some(v) => (
            if v.kind == GlassBreakViewOfKind::Blown { VIEW_BLOWN } else { VIEW_GIVES },
            [v.from[0], v.from[1], v.from[2], v.size.radius.unwrap_or(g.k.glassbreak.blast_m), v.size.power.unwrap_or(1.0), if v.kind == GlassBreakViewOfKind::Counted { 1.0 } else { 0.0 }],
        ),
    };
    let met = if kind == VIEW_GIVES { crate::scene_collision::panes_met(g, over, [at[0], at[1], at[2]], at[3]) } else { Vec::new() };
    g.tab.boxes.view((kind, at, met))
}

/// the rule's whole answer at a point: a piece out, or the view in hand counts the glass there out
pub fn glass_break_rule_open(g: &Game, bi: f64, p: V3) -> bool {
    if g.glass_break.view_of.is_none() { return glass_break_open_at(g, bi, p); }
    (js::to_u32(bi) as usize) < g.col.boxes.len() && ::sim::glass::open(g.tab.boxes.world(), js::to_u32(bi) as usize, p[0], p[1], p[2], &g.k)
}

// ---------------------------------------------------------------- per frame
/// per frame: the gusts, then every touched pane's step in the module, draining after each
pub fn glass_break_update(g: &mut crate::game::Game) {
    if core::mem::take(&mut g.tab.boxes.panes().gust) { g.glass_break.gust_dirty = true; }
    if g.glass_break.gusts.len() > 0 || g.glass_break.gust_dirty {
        glass_break_update_gusts(g);
        g.glass_break.gust_dirty = g.glass_break.gusts.len() > 0;
    }
    if !glass_break_live(g) { return; }
    let n = touched(g).rows.len();
    if n > 0 {
        panes::frame_begin(g.tab.boxes.panes());
        // who may walk into what is left of a pane: any player's man, on his feet with his hands free
        let men: Vec<panes::Walker> = crate::seats::men(g).into_iter().filter(|&pl| {
            let p = &g.pl[pl];
            !crate::condition::down(g, p.ch) && p.holding().is_none() && p.dragging().is_none() && p.wall.is_none()
        }).map(|pl| { let c = &g.ch[g.pl[pl].ch]; panes::Walker { pos: c.pos, vel: c.vel, id: c.id } }).collect();
        for r in 0..n { glass_break_asking(g, |cx, k| panes::step_row(cx, r, &men, k)); }
    }
}

/// a wind source through each shot-out exterior pane, its speed each frame the storm at that face times the open share
fn glass_break_update_gusts(g: &mut crate::game::Game) {
    let swv = crate::wind::wind_storm_wind(g);
    let sw = swv.map(|s| [s[0], s[2]]);
    let storm_speed = match sw { Some(s) => m::hyp2(s[0], s[1]), None => 0.0 };
    for r in 0..touched(g).rows.len() {
        let (exterior, bi, st_key, u_axis, hu) = { let st = &touched(g).rows[r]; (st.exterior, st.w[0] as f64, st.key, st.u_axis, st.hu) };
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
                let box_ = g.box_at(bi).unwrap();
                let (yaw, c) = { let b = &g.tab.boxes[box_]; (b.yaw, b.c) };
                let nrm = normal_on(yaw, if u_axis == 0 { 0.0 } else { 2.0 });
                let sky = panes::sky_known(g.tab.boxes.world(), js::to_u32(bi), &g.k);
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
/// is world point x on pane bi where a piece has fallen out
pub fn glass_break_open_at(g: &crate::game::Game, bi: f64, x: V3) -> bool { panes::open_at(touched(g), js::to_u32(bi), x[0], x[1], x[2]) }

/// the movement rule's answer for pane bi, false for a pane never hit
pub fn glass_break_walkable(g: &Game, bi: f64) -> bool { panes::walkable(touched(g), js::to_u32(bi)) }

/// a number that changes whenever what is open in any pane changes
pub fn glass_break_open_sig(g: &Game) -> f64 { panes::open_sig(touched(g)) as f64 }

/// open fraction of pane bi by area, 0 = whole
fn glass_break_open_fraction(g: &Game, bi: f64) -> f64 { panes::open_fraction(touched(g), js::to_u32(bi)) }

/// what a sibling needs to know about pane bi without touching it; None = not a sheet of glass
pub fn glass_break_pane_info(g: &Game, bi: f64) -> Option<crate::glassbreak::GlassBreakPaneInfo> {
    glass_break_type_of_pane(g, bi)?;
    let p = touched(g);
    let gone = p.row_of(js::to_u32(bi)).is_some_and(|r| p.rows[r].w[GW_FLAGS as usize] & G_GONE as u32 != 0);
    let open_fraction = glass_break_open_fraction(g, bi);
    Some(GlassBreakPaneInfo { open_fraction, gone })
}

/// every pane whole, the gust sources gone
pub fn glass_break_reset(g: &mut crate::game::Game) {
    panes::reset(g.tab.boxes.panes());
    glass_walked(g);
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
    // pinned: (0x61A55 ^ imul(key, 2654435761)) >>> 0
    #[test]
    fn the_gust_seed_is_as_pinned() {
        assert_eq!(seed_mix_gust(3735928559.0).to_bits(), 0x41e397dc0d400000);
    }
    // panes.rs's ANNEALED, TEMPERED, LAMINATED: `PaneType` lists its words in another order
    #[test]
    fn a_glass_word_is_the_modules() {
        for (t, want) in [(panes::ANNEALED, PaneType::Annealed), (panes::TEMPERED, PaneType::Tempered), (panes::LAMINATED, PaneType::Laminated)] {
            assert_eq!(type_word(t), want);
            assert_eq!(type_index(want), t);
        }
        assert_eq!(type_word(7), PaneType::Laminated);
    }
    // the view in hand as the numbers the box world takes, with a size's defaults; none in hand is none
    #[test]
    fn a_view_is_its_numbers() {
        let mut g = blank();
        glass_viewed(&mut g, &Over::OWN);
        assert_eq!(g.tab.boxes.world().view, VIEW_NONE);
        g.glass_break.view_of = Some(GlassBreakViewOf { kind: GlassBreakViewOfKind::Counted, from: [1.0, 2.0, 3.0], size: GlassBlastOpts { radius: Some(5.0), power: None, who: None } });
        glass_viewed(&mut g, &Over::OWN);
        let v = g.tab.boxes.world();
        assert_eq!((v.view, v.view_at), (VIEW_GIVES, [1.0, 2.0, 3.0, 5.0, 1.0, 1.0]));
        g.glass_break.view_of = Some(GlassBreakViewOf { kind: GlassBreakViewOfKind::Blown, from: [0.0, 0.0, 0.0], size: GlassBlastOpts::default() });
        glass_viewed(&mut g, &Over::OWN);
        let v = g.tab.boxes.world();
        assert_eq!((v.view, v.view_at[3], v.view_at[5]), (VIEW_BLOWN, g.k.glassbreak.blast_m, 0.0));
    }
    // a game that never touched glass: every question answers none and nothing stands up
    #[test]
    fn untouched_answers_none() {
        let mut g = blank();
        assert!(!glass_break_open_at(&mut g, 3.0, [0.0, 0.0, 0.0]));
        assert!(!glass_break_walkable(&mut g, 3.0));
        assert_eq!(glass_break_open_sig(&mut g), 0.0);
        assert_eq!(glass_break_open_fraction(&mut g, 3.0), 0.0);
        assert!(touched(&g).rows.is_empty());
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
    // off the list, negative, fractional, NaN: none
    #[test]
    fn a_box_off_the_list_is_none() {
        let g = blank();
        for bi in [-1.0, 0.5, f64::NAN, 1e9] { assert!(g.box_at(bi).is_none()); }
    }
}
