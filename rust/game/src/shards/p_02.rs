//! GlassShards: broken glass after it lets go (shards.ts 154-670).
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::debris::hash01;

const AXES: [V3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// `u32(a ^ b)`: both sides to int32, the word back as an unsigned number
fn xor_u32(a: f64, b: f64) -> f64 { js::to_u32((js::to_i32(a) ^ js::to_i32(b)) as f64) as f64 }
/// `a | b` of two flag words
fn flags_or(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }
/// `[0, 1, 2].sort((p, q) => h[p] - h[q])`: the box's axes, thinnest first (stable)
fn sort_axes(h: V3) -> [usize; 3] {
    let mut ax = [0usize, 1, 2];
    js::sort_by(&mut ax, |p, q| h[*p] - h[*q]);
    ax
}
/// shed(): a point `s` along a rectangle's outline as (along the long side, along the short side)
fn outline_walk(s0: f64, hl: f64, hs: f64) -> (f64, f64) {
    let mut s = s0;
    if s < 2.0 * hl { return (s - hl, -hs); }
    s -= 2.0 * hl;
    if s < 2.0 * hs { return (hl, s - hs); }
    s -= 2.0 * hs;
    if s < 2.0 * hl { return (hl - s, hs); }
    (-hl, hs - (s - 2.0 * hl))
}
#[allow(clippy::too_many_arguments)]
fn spray(speed: f64, speed_jit: f64, spread: f64, life: f64, life_jit: f64, size: f64, size_jit: f64, aspect: f64, tumble: f64, nrm_jit: f64, pos_jit: f64, vadd: Option<V3>, color: V3) -> SprayOpts {
    SprayOpts { speed, speed_jit, spread, life, life_jit, size, size_jit, aspect, tumble, nrm_jit, pos_jit, vadd, color }
}
fn to_sched(f: GlassShardsFallOf) -> GlassShardsScheduleFall { GlassShardsScheduleFall { t: f.t, pos: f.pos } }
fn to_landing(f: GlassShardsFallOf) -> GlassShardsLandingFall { GlassShardsLandingFall { t: f.t, pos: f.pos } }

// ---- what the skeleton still cannot do: one seam a step
fn seam_audio_play(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate (JsMap<String, Opaque> holds no number)") }
fn seam_particles_emit_shard(g: &mut Game, pos: V3, dir: V3, count: f64, o: SprayOpts, seed: f64) { todo!("needs: EmitOpts with the spray's fields, and a Particles reached in place, not gpu_fx's copy") }

/// A litter cell's name.   (shards.ts cellName)
pub fn cell_name(ix: f64, iz: f64) -> f64 { (ix + 512.0) * 4096.0 + (iz + 512.0) }

/// The landings waiting, for a tool.   (shards.ts GlassShards.pending)
pub fn glass_shards_pending(g: &mut crate::game::Game) -> Vec<crate::shards::Pending> {
    use crate::sim_fallshome::{FallSound, FallStoreLandingKey as K};
    let n = crate::sim_fallshome::fall_store_waiting(g);
    let mut out = Vec::new();
    let mut i = 0.0;
    while i < n {
        let r = crate::sim_fallshome::fall_store_landing(g, i, None);
        let v = |k: K| r.get(k).copied().unwrap_or(0.0);
        out.push(Pending { t: v(K::T), pos: [v(K::X), v(K::Y), v(K::Z)], name: if js::truthy(v(K::Name)) { FallSound::GlassShatter } else { FallSound::GlassTinkle }, gain: v(K::Gain), rate: v(K::Rate), amount: v(K::Amount), who: v(K::Who), event: v(K::Event) });
        i += 1.0;
    }
    out
}

/// m² of glass in each ½ m cell, a copy.   (shards.ts GlassShards.litter)
pub fn glass_shards_litter(g: &mut crate::game::Game) -> JsMap<crate::Num, f64> {
    let mut out = JsMap::new();
    for [ix, iz, a] in crate::sim_fallshome::fall_store_cells(g) { out.set(crate::Num(cell_name(ix, iz)), a); }
    out
}

/// The shards' books; the ghost is a chunk in no pool (the pool's glass hook is calls by name).   (shards.ts GlassShards.constructor)
pub fn glass_shards_new(g: &mut crate::game::Game) -> crate::shards::GlassShards {
    let bf = crate::scene_boxes::box_flag();
    let ghost_box = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: [0.0, -100.0, 0.0], h: [0.01, 0.01, 0.01], yaw: Some(0.0), albedo: Some([0.0, 0.0, 0.0]), flags: Some(flags_or(bf.no_primary, bf.no_shadow)),
        ..Default::default()
    });
    let ghost = g.tab.debris_chunk.push(crate::debris::Chunk { box_: ghost_box, ..crate::debris::Chunk::zeroed() });
    let mut s = crate::shards::GlassShards::blank();
    s.ghost = ghost;
    s
}

/// One fragment lets go: its plates, its spray, its landing. False = the model is not live.   (shards.ts GlassShards.release)
pub fn glass_shards_release(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike) -> bool {
    use crate::glassbreak::GlassType;
    use crate::shards::{GlassFragmentLikeCause as Cause, GlassFragmentLikeKind as Kind};
    use crate::sim_fallshome::FallSound;
    if !shards_live(g) { return false; }
    g.stats.shards.fragments += 1;
    let e = match f.cause { Cause::Hit => 1.0, Cause::Blast => 1.5, Cause::Dice | Cause::Rake => 0.5, Cause::Support => 0.25 };
    let n = v3::normalize(f.normal);
    let a = m::max(1e-4, f.area);
    let mut plated = 0.0;
    let dice = f.kind == Kind::Clump || f.type_ == GlassType::Tempered;
    let powder = !dice && f.kind != Kind::Sheet && f.grain > 0.0;
    let plateable = !dice && !powder && a >= g.k.shards.plate_min_area;
    if plateable { plated = glass_shards_plates(g, f, n); }
    let sp = g.k.shards.speed;
    let tint = f.tint;
    let spread = m::sqrt(a) * 0.45;
    let fall = glass_shards_fall_of(g, f.centre, f.velocity);
    let hit_or_blast = f.cause == Cause::Hit || f.cause == Cause::Blast;
    let by_fall = g.k.shards.by_fall;
    let fine_life = g.k.shards.fine_life;
    let fine_size = g.k.shards.fine_size;
    let fine_max = g.k.shards.fine_max;
    if powder {
        let want = clamp_i(m::round(a * m::min(g.k.shards.crater_per_m2, 0.5 / (f.grain * f.grain)) * (0.5 + 0.5 * e)), g.k.shards.fine_min, g.k.shards.crater_max);
        let ng = glass_shards_budget(g, want);
        glass_shards_emit(g, f.centre, n, ng, spray((2.2 + 2.6 * e) * sp, 0.85, 0.7, fine_life, 0.4, f.grain, 0.6, 1.3, 45.0, 1.0, spread, Some(v3::scale(f.velocity, 0.3)), tint), xor_u32(f.seed, 0xC8A7 as f64));
        if hit_or_blast {
            let want = m::round(ng * g.k.shards.back_share);
            let nb = glass_shards_budget(g, want);
            glass_shards_emit(g, v3::mad(f.centre, n, -0.03), v3::scale(n, -1.0), nb, spray((1.0 + 0.8 * e) * sp, 0.8, 1.0, fine_life * 0.8, 0.4, f.grain, 0.6, 1.3, 45.0, 1.0, spread * 0.6, None, tint), xor_u32(f.seed, 0xBACC as f64));
        }
        glass_shards_schedule(g, GlassShardsScheduleFall { t: fall.t + 0.05, pos: fall.pos }, FallSound::GlassTinkle, 0.0, 1.0, a, f.who, Some(false));
    } else if dice {
        let grain = if f.grain > 0.0 { f.grain } else { 0.009 };
        let want = clamp_i(m::round(a * m::min(g.k.shards.dice_per_m2, 0.5 / (grain * grain))), g.k.shards.dice_min, g.k.shards.dice_max);
        let nd = glass_shards_budget(g, want);
        let dice_life = g.k.shards.dice_life;
        glass_shards_emit(g, f.centre, n, nd, spray(0.9 * sp * (0.6 + 0.4 * e), 0.8, 1.1, dice_life, 0.35, grain, 0.35, 1.0, 30.0, 1.0, spread, Some(f.velocity), tint), xor_u32(f.seed, 0xD1CE as f64));
        let want = clamp_i(m::round(a * g.k.shards.fine_per_m2 * g.k.shards.dice_fine_share), 4.0, fine_max);
        let nf = glass_shards_budget(g, want);
        glass_shards_emit(g, f.centre, n, nf, spray(1.6 * sp, 0.85, 1.2, fine_life, 0.4, fine_size * 0.8, 0.5, 1.2, 45.0, 1.0, spread, Some(v3::scale(f.velocity, 0.5)), tint), xor_u32(f.seed, 0xF17E as f64));
        glass_shards_schedule(g, to_sched(fall), FallSound::GlassShatter, m::min(1.0, 0.35 + a * 4.0), 1.15 + 0.3 * hash01(f.seed), a * 0.8, f.who, None);
    } else if f.kind == Kind::Sheet {
        let nf = glass_shards_budget(g, clamp_i(m::round(a * 60.0), 4.0, 40.0));
        glass_shards_emit(g, f.centre, n, nf, spray(0.8 * sp, 0.8, 1.2, fine_life, 0.4, fine_size, 0.5, 1.2, 40.0, 1.0, spread, Some(f.velocity), tint), xor_u32(f.seed, 0xF17E as f64));
        if by_fall { glass_shards_landing(g, to_landing(fall), if plateable { 0.0 } else { 0.5 }, 0.8, a * 0.3, f.who); }
        else if !js::truthy(plated) { glass_shards_schedule(g, to_sched(fall), FallSound::GlassTinkle, 0.5, 0.8, a * 0.3, f.who, None); }
    } else {
        let k = (0.5 + 0.5 * e) * (if js::truthy(plated) { 1.0 } else { 2.0 });
        let want = clamp_i(m::round(a * g.k.shards.mid_per_m2 * k), g.k.shards.mid_min, g.k.shards.mid_max);
        let nm = glass_shards_budget(g, want);
        let (mid_life, mid_size) = (g.k.shards.mid_life, g.k.shards.mid_size);
        glass_shards_emit(g, f.centre, n, nm, spray((1.2 + 1.6 * e) * sp, 0.75, 0.55, mid_life, 0.3, mid_size, 0.55, 1.7, 22.0, 0.5, spread, Some(v3::scale(f.velocity, 0.6)), tint), xor_u32(f.seed, 0x51D5 as f64));
        let lo = if f.cause == Cause::Hit { m::max(g.k.shards.fine_min, g.k.shards.hit_fine_min) } else { g.k.shards.fine_min };
        let want = clamp_i(m::round(a * g.k.shards.fine_per_m2 * (0.3 + 0.7 * e)), lo, fine_max);
        let nf = glass_shards_budget(g, want);
        glass_shards_emit(g, f.centre, n, nf, spray((2.2 + 2.6 * e) * sp, 0.85, 0.95, fine_life, 0.4, fine_size, 0.6, 1.4, 45.0, 1.0, spread, Some(v3::scale(f.velocity, 0.3)), tint), xor_u32(f.seed, 0xF17E as f64));
        if hit_or_blast {
            let share = if f.cause == Cause::Hit { m::max(g.k.shards.back_share, g.k.shards.hit_back_share) } else { g.k.shards.back_share };
            let nb = glass_shards_budget(g, m::round(nf * share));
            glass_shards_emit(g, v3::mad(f.centre, n, -0.03), v3::scale(n, -1.0), nb, spray((1.0 + 0.8 * e) * sp, 0.8, 1.0, fine_life * 0.8, 0.4, fine_size * 0.9, 0.6, 1.3, 45.0, 1.0, spread * 0.6, None, tint), xor_u32(f.seed, 0xBACC as f64));
        }
        if by_fall { glass_shards_landing(g, to_landing(fall), if plateable { 0.0 } else { m::min(0.9, 0.3 + a * 8.0) }, 0.9 + 0.5 * hash01(f.seed), a, f.who); }
        else if !js::truthy(plated) { glass_shards_schedule(g, to_sched(fall), FallSound::GlassTinkle, m::min(0.9, 0.3 + a * 8.0), 0.9 + 0.5 * hash01(f.seed), a, f.who, None); }
        else { glass_shards_schedule(g, GlassShardsScheduleFall { t: fall.t + 0.05, pos: fall.pos }, FallSound::GlassTinkle, 0.0, 1.0, a * 0.35, f.who, Some(false)); }
    }
    true
}

/// A blow that knocked nothing out still spits fines off both faces. Particles only.   (shards.ts GlassShards.spall)
pub fn glass_shards_spall(g: &mut crate::game::Game, at: V3, n: V3, dir: Option<V3>, thickness: f64, tint: V3, seed: f64, far: Option<bool>) {
    let far = far.unwrap_or(true);
    if !shards_live(g) || g.k.shards.spall_out + g.k.shards.spall_back <= 0.0 { return; }
    g.stats.shards.spalls += 1;
    let sp = g.k.shards.speed;
    let out = match dir { Some(d) => v3::normalize(v3::add(v3::scale(d, 0.6), v3::scale(n, -0.6))), None => v3::scale(n, -1.0) };
    let no = if far { let w = g.k.shards.spall_out; glass_shards_budget(g, w) } else { 0.0 };
    let w = g.k.shards.spall_back;
    let nb = glass_shards_budget(g, w);
    let (fine_life, fine_size) = (g.k.shards.fine_life, g.k.shards.fine_size);
    if no > 0.0 { glass_shards_emit(g, v3::mad(at, n, -(thickness * 0.5 + 0.01)), out, no, spray(3.2 * sp, 0.85, 0.7, fine_life, 0.4, fine_size, 0.6, 1.4, 45.0, 1.0, 0.012, None, tint), xor_u32(seed, 0x5BA1 as f64)); }
    glass_shards_emit(g, v3::mad(at, n, thickness * 0.5 + 0.01), n, nb, spray(1.6 * sp, 0.8, 1.0, fine_life * 0.8, 0.4, fine_size * 0.9, 0.6, 1.3, 45.0, 1.0, 0.012, None, tint), xor_u32(seed, 0xBACC as f64));
}

/// The interim hookup: a shard box the pool was about to spawn, read back as the fragment it stands for. None = not ours.   (shards.ts GlassShards.take)
pub fn glass_shards_take(g: &mut crate::game::Game, box_: BoxId, v: V3, w: V3, who: f64) -> Option<DebrisChunkId> {
    use crate::glassbreak::GlassType;
    use crate::shards::{GlassFragmentLikeCause as Cause, GlassShardsPaneMemValueCause as MemCause};
    if g.shards.hooked || !shards_live(g) { return None; }
    g.shards.serial += 1.0;
    let serial = g.shards.serial;
    let c = g.col.boxes[box_].c;
    let word = m::imul(js::to_i32(serial), 0x9E3779B1u32 as i32)
        ^ m::imul(js::to_i32(m::round(c[0] * 1000.0)), 73856093)
        ^ m::imul(js::to_i32(m::round(c[1] * 1000.0)), 19349663)
        ^ m::imul(js::to_i32(m::round(c[2] * 1000.0)), 83492791);
    let mut f = fragment_of_box(g, box_, v, w, who, u32(word as f64));
    if let Some(st) = glass_shards_pane_at(g, c) {
        let now = g.time;
        let key = crate::Num(st.bi);
        let row = g.shards.pane_mem.get_or_insert_with(key, || GlassShardsPaneMemValue { hits: 0.0, sites: 0.0, t: -1e9, cause: MemCause::Hit, diced_t: -1e9 });
        // (`impacts` is no field of PaneState any more: the TypeScript's loose read is always 0, so the blast arm never ran)
        if st.hits > row.hits { row.cause = MemCause::Hit; row.t = now; }
        row.hits = st.hits;
        row.sites = 0.0;
        let (mem_t, mem_cause) = (row.t, row.cause);
        f.pane = st.bi;
        f.type_ = st.type_;
        f.cause = if now - mem_t < 0.35 { match mem_cause { MemCause::Hit => Cause::Hit, MemCause::Blast => Cause::Blast } } else { Cause::Support };
        if st.type_ == GlassType::Tempered {
            let diced = g.shards.pane_mem.get(key).unwrap().diced_t;
            if now - diced > 2.0 {
                g.shards.pane_mem.get_mut(key).unwrap().diced_t = now;
                glass_shards_dice_pane(g, &st, &f);
            }
            glass_shards_note(g, &alloc::format!("take pane {} tempered → dice", st.bi));
            return Some(g.shards.ghost);
        }
        glass_shards_note(g, &alloc::format!("take pane {} {} {} {} {:.0} cm²", st.bi, st.type_.as_str(), f.kind.as_str(), f.cause.as_str(), f.area * 1e4));
    }
    g.shards.last_plate = None;
    glass_shards_release(g, &f);
    Some(g.shards.last_plate.unwrap_or(g.shards.ghost))
}

/// The pane whose face a dropped cell's box lies in.   (shards.ts GlassShards.paneAt)
pub fn glass_shards_pane_at(g: &mut crate::game::Game, c_2: V3) -> Option<crate::glassbreak::PaneState> {
    let rows: Vec<(crate::Num, BoxId)> = g.glass_break.panes.iter().map(|(k, st)| (k, st.box_)).collect();
    let mut best = None;
    let mut best_d = 0.12;
    for (key, bx) in rows {
        let (bc, bh, yaw) = { let b = &g.col.boxes[bx]; (b.c, b.h, b.yaw) };
        let (dx, dz) = (c_2[0] - bc[0], c_2[2] - bc[2]);
        let (cy, sy) = (m::cos(yaw), m::sin(yaw));
        let l = [dx * cy - dz * sy, c_2[1] - bc[1], dx * sy + dz * cy];
        let thin = if bh[0] < bh[2] { 0 } else { 2 };
        if m::abs(l[thin]) > bh[thin] + 0.08 { continue; }
        let out = m::max_n(&[0.0, m::abs(l[2 - thin]) - bh[2 - thin], m::abs(l[1]) - bh[1]]);
        if out < best_d { best_d = out; best = Some(key); }
    }
    best.and_then(|k| g.glass_break.panes.get(k).cloned())
}

/// A tempered pane gone to dice, whole: its face as a grid of clumps, each a release.   (shards.ts GlassShards.dicePane)
pub fn glass_shards_dice_pane(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, f0: &crate::shards::GlassFragmentLike) {
    use crate::glassbreak::GlassType;
    use crate::shards::{GlassFragmentLikeCause as Cause, GlassFragmentLikeKind as Kind};
    let (bc, bh, yaw) = { let b = &g.col.boxes[st.box_]; (b.c, b.h, b.yaw) };
    let thin = if bh[0] < bh[2] { 0 } else { 2 };
    let (hu, hv) = (bh[2 - thin], bh[1]);
    let nu = clamp_i(m::round(hu * 2.0 / 0.5), 1.0, 4.0);
    let nv = clamp_i(m::round(hv * 2.0 / 0.5), 1.0, 4.0);
    let (cy, sy) = (m::cos(yaw), m::sin(yaw));
    let u_ax: V3 = if thin == 0 { [sy, 0.0, cy] } else { [cy, 0.0, -sy] };
    let mut i = 0.0;
    while i < nu {
        let mut j = 0.0;
        while j < nv {
            let du = ((i + 0.5) / nu - 0.5) * 2.0 * hu;
            let dv = ((j + 0.5) / nv - 0.5) * 2.0 * hv;
            let (su, sv) = (hu / nu, hv / nv);
            let c: V3 = [bc[0] + u_ax[0] * du, bc[1] + dv, bc[2] + u_ax[2] * du];
            let corner = |a: f64, e: f64| -> V3 { [c[0] + u_ax[0] * su * a, c[1] + sv * e, c[2] + u_ax[2] * su * a] };
            let f = GlassFragmentLike {
                pane: st.bi, type_: GlassType::Tempered, kind: Kind::Clump, grain: 0.009, cause: Cause::Dice,
                outline: vec![corner(-1.0, -1.0), corner(1.0, -1.0), corner(1.0, 1.0), corner(-1.0, 1.0)],
                centre: c, area: 4.0 * su * sv, thickness: 2.0 * bh[thin],
                velocity: [f0.velocity[0], f0.velocity[1] - 0.2 * hash01(f0.seed + i * 7.0 + j), f0.velocity[2]],
                seed: u32(f0.seed + m::imul(js::to_i32(i * 4.0 + j + 1.0), 0x9E3779B1u32 as i32) as f64),
                ..f0.clone()
            };
            glass_shards_release(g, &f);
            j += 1.0;
        }
        i += 1.0;
    }
}

/// The fragment's rigid part: 1‥4 thin boxes over its outline; returns how many.   (shards.ts GlassShards.plates)
pub fn glass_shards_plates(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike, n: V3) -> f64 {
    use crate::shards::GlassFragmentLikeKind as Kind;
    let fr = frame_of(&f.outline, f.centre, n);
    if f.kind != Kind::Sheet && f.outline.len() >= 3 { return glass_shards_cut_plates(g, f, n, fr); }
    let fill = m::min(1.0, f.area / m::max(1e-6, fr.lu * fr.lv));
    let sc = m::sqrt(fill) * 0.96;
    let k = if f.kind == Kind::Sheet { clamp_i(m::round(f.area / 0.25), 1.0, 4.0) }
        else if f.kind == Kind::Dagger { clamp_i(m::round(fr.lu / m::max(fr.lv, 0.08) / 1.6), 1.0, 3.0) }
        else if f.area > 0.09 { 2.0 } else { 1.0 };
    let rot = v3::quat::from_basis(fr.u, fr.v, n);
    let under = glass_shards_under_albedo(g, f.centre);
    let mut made = 0.0;
    let mut j = 0.0;
    while j < k {
        let h = |i: f64| hash01(u32(f.seed + j * 7919.0 + i * 104729.0));
        let hu = fr.lu * sc / (2.0 * k);
        let hv = fr.lv * sc / 2.0;
        if 4.0 * hu * hv < g.k.shards.child_min_area { j += 1.0; continue; }
        let off = ((j + 0.5) / k - 0.5) * fr.lu * sc;
        let c = v3::mad(v3::mad(fr.mid, fr.u, off), n, 0.02);
        let bx = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c, h: [hu, hv, m::max(0.004, f.thickness / 2.0)], yaw: Some(0.0), rot: Some(rot), albedo: Some(plate_albedo(f.tint, under, f.type_)),
            flags: Some(crate::scene_boxes::box_flag().dynamic), ..Default::default()
        });
        let vel = v3::add(f.velocity, v3::add(v3::scale(fr.u, (j - (k - 1.0) / 2.0) * 0.25), [(h(1.0) - 0.5) * 0.3, -0.2 * h(2.0), (h(3.0) - 0.5) * 0.3]));
        let w: V3 = [f.spin[0] * (0.6 + 0.8 * h(4.0)) + (h(5.0) - 0.5) * 2.0, f.spin[1] + (h(6.0) - 0.5) * 2.0, f.spin[2] * (0.6 + 0.8 * h(7.0)) + (h(8.0) - 0.5) * 2.0];
        let meta = PlateMeta { gen_: 0.0, area: 4.0 * hu * hv, tint: v3::copy(f.tint), type_: f.type_, seed: u32(f.seed + j * 31.0), littered: None, poly: None };
        if glass_shards_plate(g, bx, vel, w, f.who, &meta, None).is_some() { made += 1.0; }
        j += 1.0;
    }
    made
}

/// The fragment's own cell as ONE box cut along its edges (two halves if a record cannot hold it); returns how many.   (shards.ts GlassShards.cutPlates)
pub fn glass_shards_cut_plates(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike, n: V3, fr: crate::shards::GlassShardsCutPlatesFr) -> f64 {
    let flat: Vec<[f64; 2]> = f.outline.iter().map(|p| { let d = v3::sub(*p, fr.mid); [v3::dot(d, fr.u), v3::dot(d, fr.v)] }).collect();
    let poly = tidy_poly(&flat);
    if poly.len() < 3 { return 0.0; }
    let half_t = m::max(0.004, f.thickness / 2.0);
    let parts = cell_parts(&poly, half_t, f.seed);
    let k = parts.len();
    if k > 1 { g.stats.shards.cut_split += 1; }
    let rot = v3::quat::from_basis(fr.u, fr.v, n);
    let under = glass_shards_under_albedo(g, f.centre);
    let albedo = plate_albedo(f.tint, under, f.type_);
    let mut made = 0.0;
    for jx in 0..k {
        let j = jx as f64;
        let h = |i: f64| hash01(u32(f.seed + j * 7919.0 + i * 104729.0));
        let area = poly_area(&parts[jx]);
        if area < g.k.shards.child_min_area { continue; }
        let pb = match cut_plate_box(g, v3::mad(fr.mid, n, 0.02), rot, &parts[jx], half_t, albedo) { Some(pb) => pb, None => continue };
        let pc = poly_centroid(&parts[jx]);
        let pl = m::hyp2(pc[0], pc[1]);
        let apart: V3 = if k > 1 && pl > 1e-6 { v3::scale(v3::add(v3::scale(fr.u, pc[0] / pl), v3::scale(fr.v, pc[1] / pl)), 0.125) } else { [0.0, 0.0, 0.0] };
        let vel = v3::add(f.velocity, v3::add(apart, [(h(1.0) - 0.5) * 0.3, -0.2 * h(2.0), (h(3.0) - 0.5) * 0.3]));
        let w: V3 = [f.spin[0] * (0.6 + 0.8 * h(4.0)) + (h(5.0) - 0.5) * 2.0, f.spin[1] + (h(6.0) - 0.5) * 2.0, f.spin[2] * (0.6 + 0.8 * h(7.0)) + (h(8.0) - 0.5) * 2.0];
        let meta = PlateMeta { gen_: 0.0, area, tint: v3::copy(f.tint), type_: f.type_, seed: u32(f.seed + j * 31.0), littered: None, poly: Some(pb.poly.clone()) };
        let cut = GlassShardsPlateCut { cut: pb.cut.clone(), dropped: pb.dropped };
        if glass_shards_plate(g, pb.box_, vel, w, f.who, &meta, Some(&cut)).is_some() { made += 1.0; }
    }
    made
}

/// One plate into the debris pool, inside glass's share: over it the oldest resting plate crumbles; none resting = refused.   (shards.ts GlassShards.plate)
pub fn glass_shards_plate(g: &mut crate::game::Game, box_: BoxId, v: V3, w: V3, who: f64, meta: &crate::shards::PlateMeta, cut: Option<&crate::shards::GlassShardsPlateCut>) -> Option<DebrisChunkId> {
    let pool_cap = crate::debris::debris_cap(g);
    let cap = m::max(2.0, m::floor(pool_cap * g.k.shards.plate_share));
    let mut mine = 0.0;
    for c in g.destr.debris.list.iter() { if g.tab.debris_chunk[*c].glass.is_some() { mine += 1.0; } }
    while mine >= cap {
        let old = g.destr.debris.list.iter().copied().find(|c| g.tab.debris_chunk[*c].glass.is_some() && g.tab.debris_chunk[*c].rest);
        let old = match old { Some(o) => o, None => { g.stats.shards.plates_refused += 1; return None; } };
        glass_shards_crumble(g, old);
        mine -= 1.0;
    }
    let ch = match crate::debris::debris_spawn_raw(g, crate::debris::ChunkKind::Shard, box_, v, w, Some(who), None) {
        Some(c) => c,
        None => { g.stats.shards.plates_refused += 1; return None; }
    };
    let contact = if meta.type_ == crate::glassbreak::GlassType::Laminated { None } else {
        let k = g.k.shards.contact;
        Some(crate::debris::ChunkContact { keep_floor: k.keep_floor, keep_wall: k.keep_wall, rest: k.rest })
    };
    let by_fall = g.k.shards.by_fall;
    {
        let row = &mut g.tab.debris_chunk[ch];
        row.glass = Some(meta.clone());
        row.contact = contact;
        if by_fall { row.dress = Some(true); }
    }
    let bx = g.tab.debris_chunk[ch].box_;
    g.col.boxes[bx].rough = Some(0.05);
    g.col.boxes[bx].flags = flags_or(g.col.boxes[bx].flags, crate::scene_boxes::box_flag().no_shadow);
    if let Some(cut) = cut {
        if g.k.shards.cut_body > 0.0 {
            let at = g.col.boxes[bx].c;
            let under = glass_shards_under_albedo(g, at);
            let albedo = if cut.cut.is_some() { cut_face_albedo(g, meta.tint, under, meta.type_, false) } else { plate_albedo(meta.tint, under, meta.type_) };
            g.col.boxes[bx].albedo = albedo;
        }
        g.col.boxes[bx].cut = cut.cut.clone();
        if cut.cut.is_some() {
            if g.k.shards.cut_edge > 0.0 { let e = edge_colour(meta.tint, g.k.shards.cut_edge); g.col.boxes[bx].cut_core = Some(e); }
            if g.k.shards.cut_see > 0.0 || g.k.shards.cut_rim > 0.0 {
                g.col.boxes[bx].cut_glass = Some(crate::scene_cut::CutGlass { see: g.k.shards.cut_see, rim: g.k.shards.cut_rim, anim: Some(true) });
            }
        }
        g.stats.shards.cut_plates += 1;
        g.stats.shards.cut_dropped += cut.dropped as i32;
    }
    g.stats.shards.plates += 1;
    g.shards.last_plate = Some(ch);
    g.stats.shards.peak_plates = g.stats.shards.peak_plates.max(mine as i32 + 1);
    Some(ch)
}

/// A resting plate retired to make room: gone from the pool, a few particles where it was.   (shards.ts GlassShards.crumble)
pub fn glass_shards_crumble(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    if let Some(i) = g.destr.debris.list.iter().position(|c| *c == c_2) { g.destr.debris.list.remove(i); }
    glass_shards_crumbs(g, c_2);
}

/// A few grains of a plate's own glass where it lay.   (shards.ts GlassShards.crumbs)
pub fn glass_shards_crumbs(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    g.stats.shards.crumbled += 1;
    let meta = g.tab.debris_chunk[c_2].glass.clone().unwrap();
    let n = glass_shards_budget(g, clamp_i(m::round(meta.area * 220.0), 3.0, 10.0));
    let bx = g.tab.debris_chunk[c_2].box_;
    let at = g.col.boxes[bx].c;
    let (mid_life, mid_size) = (g.k.shards.mid_life, g.k.shards.mid_size);
    glass_shards_emit(g, v3::add(at, [0.0, 0.01, 0.0]), [0.0, 1.0, 0.0], n, spray(0.25, 0.8, 1.3, mid_life * 0.7, 0.3, mid_size, 0.5, 1.6, 6.0, 0.2, m::sqrt(meta.area) * 0.4, None, meta.tint), xor_u32(meta.seed, 0xC2B2 as f64));
}

/// The same as gone, for a caller outside the pool's own hook.   (shards.ts GlassShards.evicted)
pub fn glass_shards_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) { glass_shards_gone(g, c_2); }

/// A glass plate left the pool by the pool's own hand: it goes as a few grains.   (shards.ts GlassShards.gone)
pub fn glass_shards_gone(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    if !shards_live(g) || g.tab.debris_chunk[c_2].glass.is_none() { return; }
    let bx = g.tab.debris_chunk[c_2].box_;
    if g.col.boxes[bx].c[1] < -0.5 { return; }
    glass_shards_crumbs(g, c_2);
}

/// A plate's first hard touch: its sound and hearing now, whether it breaks decided here and done in update. Always true.   (shards.ts GlassShards.landed)
pub fn glass_shards_landed(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) -> bool {
    let meta = g.tab.debris_chunk[c_2].glass.clone().unwrap();
    let (serial, who, bx) = { let c = &g.tab.debris_chunk[c_2]; (c.serial, c.who, c.box_) };
    let (v0, v1) = (g.k.shards.shatter_v0, g.k.shards.shatter_v1);
    let p = if v1 > v0 { m::max(0.0, m::min(1.0, (speed - v0) / (v1 - v0))) } else { 1.0 };
    let laminated = meta.type_ == crate::glassbreak::GlassType::Laminated;
    let breaks = shards_live(g) && !laminated && meta.gen_ < g.k.shards.max_gen && meta.area >= g.k.shards.child_min_area * 2.0
        && hash01(serial * 61.0 + 7.0) < p * (if js::truthy(meta.gen_) { 0.6 } else { 1.0 });
    if breaks { g.shards.breaks.push(GlassShardsBreaksItem { c: c_2, speed }); }
    let big = m::min(1.0, meta.area / 0.08);
    let at = g.col.boxes[bx].c;
    if breaks { glass_shards_sound(g, "glassShatter", at, m::min(1.0, 0.4 + 0.5 * big + speed * 0.05), 1.25 - 0.4 * big + 0.15 * hash01(serial * 31.0)); }
    else { glass_shards_sound(g, "glassTinkle", at, m::min(1.0, 0.3 + 0.3 * big + speed * 0.08), (if laminated { 0.55 } else { 1.1 - 0.35 * big }) + 0.2 * hash01(serial * 31.0)); }
    if !g.k.shards.by_fall {
        let level = if breaks { g.k.shards.level + 0.05 } else { g.k.shards.level };
        glass_shards_heard(g, at, level, who);
    }
    true
}

/// A plate came to rest: it takes the colour of what it lies on, and its glass is litter there.   (shards.ts GlassShards.rested)
pub fn glass_shards_rested(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let meta = g.tab.debris_chunk[c_2].glass.clone().unwrap();
    let bx = g.tab.debris_chunk[c_2].box_;
    let at = g.col.boxes[bx].c;
    let under = glass_shards_under_albedo(g, at);
    let has_cut = g.col.boxes[bx].cut.is_some();
    let albedo = if has_cut { cut_face_albedo(g, meta.tint, under, meta.type_, true) } else { plate_albedo(meta.tint, under, meta.type_) };
    g.col.boxes[bx].albedo = albedo;
    if has_cut {
        if let Some(glass) = g.col.boxes[bx].cut_glass {
            if glass.see > 0.0 && g.k.shards.cut_see_rest > 0.0 {
                let see = g.k.shards.cut_see_rest;
                g.col.boxes[bx].cut_glass = Some(crate::scene_cut::CutGlass { see, ..glass });
            }
        }
    }
    if meta.littered != Some(true) {
        g.tab.debris_chunk[c_2].glass.as_mut().unwrap().littered = Some(true);
        if !g.k.shards.by_fall { glass_shards_deposit(g, at, meta.area * 0.5); }
        if shards_live(g) { glass_shards_shed(g, c_2); }
    }
}

/// The edges a plate broke along shed their crumbs where it stops, two grains to a point round its outline. Once per plate.   (shards.ts GlassShards.shed)
pub fn glass_shards_shed(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let meta = g.tab.debris_chunk[c_2].glass.clone().unwrap();
    let bx = g.tab.debris_chunk[c_2].box_;
    let want = clamp_i(m::round(m::sqrt(meta.area) * 28.0), 0.0, g.k.shards.rest_crumbs);
    let n = glass_shards_budget(g, want);
    if n <= 0.0 { return; }
    if meta.poly.is_some() { glass_shards_shed_cut(g, c_2, n); return; }
    let (bc, bh, yaw, brot) = { let b = &g.col.boxes[bx]; (b.c, b.h, b.yaw, b.rot) };
    let rot = brot.unwrap_or_else(|| v3::quat::yaw(yaw));
    let ax = sort_axes(bh);
    let (long, short) = (ax[2], ax[1]);
    let l = v3::quat::rotate(rot, AXES[long]);
    let s_ax = v3::quat::rotate(rot, AXES[short]);
    let (hl, hs) = (bh[long] * 1.08, bh[short] * 1.08);
    let per = 4.0 * (hl + hs);
    let pts = m::ceil(n / 2.0);
    let (mid_life, crumb) = (g.k.shards.mid_life, g.k.shards.crumb_size);
    let mut j = 0.0;
    while j < pts {
        let s = (j + hash01(u32(meta.seed + 0xC4B5 as f64 + j * 977.0))) / pts * per;
        let (a, d) = outline_walk(s, hl, hs);
        let p = v3::mad(v3::mad(bc, l, a), s_ax, d);
        glass_shards_emit(g, [p[0], m::max(p[1], bc[1]) + 0.012, p[2]], [0.0, 1.0, 0.0], m::min(2.0, n - 2.0 * j), spray(0.15, 0.8, 1.3, mid_life, 0.3, crumb, 0.4, 1.3, 8.0, 0.3, 0.03, None, meta.tint), xor_u32(meta.seed, 0xC4B5 as f64 + j * 131.0));
        j += 1.0;
    }
}

/// shed() for a cut plate: round its real outline.   (shards.ts GlassShards.shedCut)
pub fn glass_shards_shed_cut(g: &mut crate::game::Game, c_2: DebrisChunkId, n: f64) {
    let meta = g.tab.debris_chunk[c_2].glass.clone().unwrap();
    let bx = g.tab.debris_chunk[c_2].box_;
    let p_ = meta.poly.clone().unwrap();
    let (bc, yaw, brot) = { let b = &g.col.boxes[bx]; (b.c, b.yaw, b.rot) };
    let rot = brot.unwrap_or_else(|| v3::quat::yaw(yaw));
    let mut len: Vec<f64> = Vec::new();
    let mut per = 0.0;
    for i in 0..p_.len() {
        let q = p_[(i + 1) % p_.len()];
        let l = m::hyp2(q[0] - p_[i][0], q[1] - p_[i][1]);
        len.push(l);
        per += l;
    }
    if !(per > 1e-6) { return; }
    let cc = poly_centroid(&p_);
    let pts = m::ceil(n / 2.0);
    let (mid_life, crumb) = (g.k.shards.mid_life, g.k.shards.crumb_size);
    let mut j = 0.0;
    while j < pts {
        let mut s = (j + hash01(u32(meta.seed + 0xC4B5 as f64 + j * 977.0))) / pts * per;
        let mut i = 0;
        while i < p_.len() - 1 && s > len[i] { s -= len[i]; i += 1; }
        let (a, q) = (p_[i], p_[(i + 1) % p_.len()]);
        let t = if len[i] > 1e-9 { m::min(1.0, s / len[i]) } else { 0.0 };
        let (ex, ey) = (a[0] + (q[0] - a[0]) * t, a[1] + (q[1] - a[1]) * t);
        let p = v3::add(bc, v3::quat::rotate(rot, [cc[0] + (ex - cc[0]) * 1.08, cc[1] + (ey - cc[1]) * 1.08, 0.0]));
        glass_shards_emit(g, [p[0], m::max(p[1], bc[1]) + 0.012, p[2]], [0.0, 1.0, 0.0], m::min(2.0, n - 2.0 * j), spray(0.15, 0.8, 1.3, mid_life, 0.3, crumb, 0.4, 1.3, 8.0, 0.3, 0.03, None, meta.tint), xor_u32(meta.seed, 0xC4B5 as f64 + j * 131.0));
        j += 1.0;
    }
}

/// A plate that landed hard comes apart: 2‥3 smaller plates along its long side, a burst where it struck.   (shards.ts GlassShards.shatter)
pub fn glass_shards_shatter(g: &mut crate::game::Game, c_2: DebrisChunkId, speed: f64) {
    let i = match g.destr.debris.list.iter().position(|c| *c == c_2) { Some(i) => i, None => return };
    let meta = g.tab.debris_chunk[c_2].glass.clone().unwrap();
    let (bx, serial, cv, who) = { let c = &g.tab.debris_chunk[c_2]; (c.box_, c.serial, c.v, c.who) };
    let (bc, bh, yaw, balbedo, brot) = { let b = &g.col.boxes[bx]; (b.c, b.h, b.yaw, b.albedo, b.rot) };
    let rot = brot.unwrap_or_else(|| v3::quat::yaw(yaw));
    g.destr.debris.list.remove(i);
    g.stats.shards.breaks += 1;
    let cut_plate = meta.poly.is_some();
    let ax = if cut_plate { if bh[0] >= bh[1] { [2, 1, 0] } else { [2, 0, 1] } } else { sort_axes(bh) };
    let (long, short) = (ax[2], ax[1]);
    let l = v3::quat::rotate(rot, AXES[long]);
    let seed = meta.seed;
    let h = |k: f64| hash01(u32(seed + serial * 131.0 + k * 977.0));
    let k = 2.0 + (if h(0.0) > 0.6 { 1.0 } else { 0.0 });
    let n_parts = k as usize;
    let mut cuts = vec![0.0];
    for jx in 1..n_parts { let j = jx as f64; cuts.push((j + (h(j) - 0.5) * 0.5) / k); }
    cuts.push(1.0);
    let mut lost = 0.0;
    let child_min = g.k.shards.child_min_area;
    if cut_plate {
        let mut rem: Vec<[f64; 2]> = meta.poly.clone().unwrap();
        for jx in 0..n_parts {
            let j = jx as f64;
            let mut piece = rem.clone();
            if jx < n_parts - 1 {
                let ang = (h(90.0 + j) - 0.5) * 0.9;
                let (ca, sa) = (m::cos(ang), m::sin(ang));
                let (nx, ny) = if long == 0 { (ca, sa) } else { (-sa, ca) };
                let d = (cuts[jx + 1] * 2.0 - 1.0) * bh[long] * (if long == 0 { nx } else { ny });
                piece = clip_half(&rem, nx, ny, d);
                rem = clip_half(&rem, -nx, -ny, -d);
            }
            let full = if piece.len() >= 3 { poly_area(&piece) } else { 0.0 };
            let sc = 0.88 + 0.1 * h(10.0 + j);
            let cen = poly_centroid(&piece);
            let small: Vec<[f64; 2]> = piece.iter().map(|p| [cen[0] + (p[0] - cen[0]) * sc, cen[1] + (p[1] - cen[1]) * sc]).collect();
            let area = full * sc * sc;
            if area < child_min { lost += full; continue; }
            let pb = match cut_plate_box(g, [bc[0], bc[1] + 0.004, bc[2]], rot, &small, bh[2], v3::copy(balbedo)) { Some(pb) => pb, None => { lost += full; continue; } };
            let side = j - (k - 1.0) / 2.0;
            let vel = v3::add(cv, v3::add(v3::scale(l, side * (0.5 + 0.5 * h(20.0 + j))), [(h(30.0 + j) - 0.5) * 0.5, 0.3 + 0.5 * h(40.0 + j), (h(50.0 + j) - 0.5) * 0.5]));
            let w: V3 = [(h(60.0 + j) - 0.5) * 12.0, (h(70.0 + j) - 0.5) * 6.0, (h(80.0 + j) - 0.5) * 12.0];
            let child = PlateMeta { gen_: meta.gen_ + 1.0, area, tint: meta.tint, type_: meta.type_, seed: u32(meta.seed + 7.0 + j * 131.0), littered: meta.littered, poly: Some(pb.poly.clone()) };
            let cut = GlassShardsPlateCut { cut: pb.cut.clone(), dropped: pb.dropped };
            if glass_shards_plate(g, pb.box_, vel, w, who, &child, Some(&cut)).is_none() { lost += full; }
        }
    } else {
        let word = crate::scene_boxes::WordOfB { stuff: g.col.boxes[bx].stuff, does: g.col.boxes[bx].does };
        for jx in 0..n_parts {
            let j = jx as f64;
            let (a0, a1) = (cuts[jx], cuts[jx + 1]);
            let share = a1 - a0;
            let mut hh: V3 = [bh[0], bh[1], bh[2]];
            hh[long] = bh[long] * share;
            hh[short] = bh[short] * (0.7 + 0.3 * h(10.0 + j));
            let area = 4.0 * hh[long] * hh[short];
            if area < child_min { lost += area; continue; }
            let cc = v3::mad(bc, l, ((a0 + a1) / 2.0 - 0.5) * 2.0 * bh[long]);
            let w_of = crate::scene_boxes::word_of(word);
            let child_box = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
                c: [cc[0], cc[1] + 0.004, cc[2]], h: hh, yaw: Some(yaw), rot: Some(rot), albedo: Some(v3::copy(balbedo)), stuff: w_of.stuff, does: w_of.does,
                flags: Some(crate::scene_boxes::box_flag().dynamic), ..Default::default()
            });
            let side = j - (k - 1.0) / 2.0;
            let vel = v3::add(cv, v3::add(v3::scale(l, side * (0.5 + 0.5 * h(20.0 + j))), [(h(30.0 + j) - 0.5) * 0.5, 0.3 + 0.5 * h(40.0 + j), (h(50.0 + j) - 0.5) * 0.5]));
            let w: V3 = [(h(60.0 + j) - 0.5) * 12.0, (h(70.0 + j) - 0.5) * 6.0, (h(80.0 + j) - 0.5) * 12.0];
            let child = PlateMeta { gen_: meta.gen_ + 1.0, area, tint: meta.tint, type_: meta.type_, seed: u32(meta.seed + 7.0 + j * 131.0), littered: meta.littered, poly: None };
            if glass_shards_plate(g, child_box, vel, w, who, &child, None).is_none() { lost += area; }
        }
    }
    let a = meta.area * 0.35 + lost;
    let want = clamp_i(m::round(a * g.k.shards.mid_per_m2 * 1.5), 3.0, g.k.shards.mid_max);
    let nm = glass_shards_budget(g, want);
    let (mid_life, mid_size, fine_life, fine_size) = (g.k.shards.mid_life, g.k.shards.mid_size, g.k.shards.fine_life, g.k.shards.fine_size);
    glass_shards_emit(g, v3::add(bc, [0.0, 0.02, 0.0]), [0.0, 1.0, 0.0], nm, spray(0.6 + 0.25 * speed, 0.7, 1.25, mid_life, 0.3, mid_size, 0.55, 1.7, 25.0, 1.0, bh[long] * 0.7, Some([cv[0] * 0.5, 0.0, cv[2] * 0.5]), meta.tint), xor_u32(meta.seed, 0x5A77 as f64));
    let want = clamp_i(m::round(a * g.k.shards.fine_per_m2), 8.0, g.k.shards.fine_max);
    let nf = glass_shards_budget(g, want);
    glass_shards_emit(g, v3::add(bc, [0.0, 0.02, 0.0]), [0.0, 1.0, 0.0], nf, spray(0.9 + 0.35 * speed, 0.85, 1.35, fine_life, 0.4, fine_size, 0.6, 1.4, 45.0, 1.0, bh[long] * 0.7, None, meta.tint), xor_u32(meta.seed, 0x5A78 as f64));
    if meta.littered != Some(true) && !g.k.shards.by_fall { glass_shards_deposit(g, bc, a); }
    glass_shards_note(g, &alloc::format!("plate {} (gen {}, {:.0} cm²) broke at {:.1} m/s into {} (+{} particles)", serial, meta.gen_, meta.area * 1e4, speed, k, nm + nf));
}

/// `g.level.boxes[bi]?.albedo ?? the pale default`
fn level_box_albedo(g: &mut Game, bi: f64) -> V3 {
    let row = if bi >= 0.0 { g.level.boxes.get(bi as usize) } else { None };
    match row { Some(b) => b.albedo, None => [0.72, 0.8, 0.82] }
}

/// A round went through a sheet: the exit spall, and a little litter where the jet comes down.   (shards.ts GlassShards.crossed)
pub fn glass_shards_crossed(g: &mut crate::game::Game, x: crate::shards::PaneCrossLike, who: f64) -> bool {
    if !shards_live(g) { return false; }
    g.stats.shards.crossings += 1;
    let k = v3::clamp(x.take / 0.1, 0.5, 2.0) * (if x.hole { 0.2 } else { 1.0 }) * (if x.type_ == crate::glassbreak::GlassType::Laminated { 0.35 } else { 1.0 });
    let want = m::round(g.k.shards.exit_fines * k);
    let n = glass_shards_budget(g, want);
    let (fine_life, fine_size) = (g.k.shards.fine_life, g.k.shards.fine_size);
    let sound = crate::sim_fallshome::FallSound::GlassTinkle;
    if g.k.shards.by_fall {
        if want <= 0.0 { return true; }
        let d = v3::normalize(x.dir_out);
        let sp = (g.k.shards.exit_speed + 5.0 * v3::clamp(x.e_out, 0.0, 1.0)) * g.k.shards.speed;
        let fall = glass_shards_fall_of(g, x.exit, v3::scale(d, sp * 0.35));
        glass_shards_schedule(g, to_sched(fall), sound, 0.0, 1.0, want * fine_size * fine_size, who, Some(false));
        if n > 0.0 {
            let tint = level_box_albedo(g, x.bi);
            g.shards.serial += 1.0;
            let seed = xor_u32(m::imul(js::to_i32(g.shards.serial), js::to_i32(2654435761.0)) as f64, 0xE817 as f64);
            glass_shards_emit(g, v3::mad(x.exit, d, 0.01), d, n, spray(sp, 0.7, 0.3, fine_life, 0.4, fine_size * 0.8, 0.6, 1.3, 50.0, 1.0, 0.01, None, tint), seed);
        }
        return true;
    }
    if n <= 0.0 { return true; }
    let d = v3::normalize(x.dir_out);
    let sp = (g.k.shards.exit_speed + 5.0 * v3::clamp(x.e_out, 0.0, 1.0)) * g.k.shards.speed;
    let tint = level_box_albedo(g, x.bi);
    g.shards.serial += 1.0;
    let seed = xor_u32(m::imul(js::to_i32(g.shards.serial), js::to_i32(2654435761.0)) as f64, 0xE817 as f64);
    glass_shards_emit(g, v3::mad(x.exit, d, 0.01), d, n, spray(sp, 0.7, 0.3, fine_life, 0.4, fine_size * 0.8, 0.6, 1.3, 50.0, 1.0, 0.01, None, tint), seed);
    let fall = glass_shards_fall_of(g, x.exit, v3::scale(d, sp * 0.35));
    glass_shards_schedule(g, to_sched(fall), sound, 0.0, 1.0, n * fine_size * fine_size, who, Some(false));
    true
}

/// `want` births inside the live budget.   (shards.ts GlassShards.budget)
pub fn glass_shards_budget(g: &mut crate::game::Game, want: f64) -> f64 {
    if want <= 0.0 { return 0.0; }
    let cap = g.k.shards.live_cap;
    let have = g.shards.live_est + g.shards.deferred_n;
    let fill = have / cap;
    let k = m::min(1.0, m::max(0.0, 1.0 - fill) / 0.35);
    let mut n = m::max(0.0, m::min(m::round(want * k), cap - have));
    if n < g.k.shards.cap_floor && have < cap * g.k.shards.over_cap { n = m::min(want, g.k.shards.cap_floor); }
    g.stats.shards.trimmed += (want - n) as i32;
    n
}

/// One emit record: out now, or waiting its turn, or trimmed.   (shards.ts GlassShards.emit)
pub fn glass_shards_emit(g: &mut crate::game::Game, pos: V3, dir: V3, count: f64, o: crate::shards::SprayOpts, seed: f64) {
    if count <= 0.0 { return; }
    if g.shards.recs_this_frame < g.k.shards.recs_per_frame { glass_shards_send(g, pos, dir, count, o, seed); return; }
    if g.shards.deferred.len() as f64 >= g.k.shards.defer_max { g.stats.shards.trimmed += count as i32; return; }
    g.stats.shards.recs_deferred += 1;
    g.shards.deferred_n += count;
    g.shards.deferred.push(GlassShardsDeferredItem { pos: v3::copy(pos), dir: v3::copy(dir), count, o, seed });
}

/// One record out: the books first, the same with or without a GPU.   (shards.ts GlassShards.send)
pub fn glass_shards_send(g: &mut crate::game::Game, pos: V3, dir: V3, count: f64, o: crate::shards::SprayOpts, seed: f64) {
    let mut o = o;
    g.shards.recs_this_frame += 1.0;
    let fill = g.shards.live_est / g.k.shards.live_cap;
    let (press_life, press_from) = (g.k.shards.press_life, g.k.shards.press_from);
    if press_life < 1.0 && fill > press_from {
        o.life = o.life * m::max(press_life, 1.0 - (fill - press_from) / (1.0 - press_from) * (1.0 - press_life));
    }
    g.shards.live_est += count;
    let t = g.time + o.life * (1.0 + o.life_jit);
    g.shards.lives.push(Life { t, n: count });
    g.stats.shards.emits += 1;
    g.stats.shards.born += count as i32;
    g.stats.shards.peak_live = g.stats.shards.peak_live.max(g.shards.live_est as i32);
    seam_particles_emit_shard(g, pos, dir, count, o, seed);
}

/// Where and when a fragment let go at `c` with velocity `v` reaches what is under it.   (shards.ts GlassShards.fallOf)
pub fn glass_shards_fall_of(g: &mut crate::game::Game, c_2: V3, v: V3) -> crate::shards::GlassShardsFallOf {
    let gravity = crate::shard_model::shard().gravity;
    let r = crate::falling::fall_of(g, c_2, v, gravity, 0.8, crate::falling::FallWorld::Statics);
    GlassShardsFallOf { t: r.t, pos: r.pos }
}

/// A landing filed in the game's fall store.   (shards.ts GlassShards.schedule)
pub fn glass_shards_schedule(g: &mut crate::game::Game, fall: crate::shards::GlassShardsScheduleFall, name: crate::sim_fallshome::FallSound, gain: f64, rate: f64, amount: f64, who: f64, event: Option<bool>) {
    let event = event.unwrap_or(true);
    let t = g.time + fall.t;
    let cap = g.k.shards.pending_max;
    let word = if event { crate::sim_fallshome::FALL_IF_SOUNDED } else { crate::sim_fallshome::FALL_UNHEARD };
    crate::sim_fallshome::fall_store_schedule(g, t, fall.pos[0], fall.pos[1], fall.pos[2], name, gain, rate, amount, who, word, 0.0, cap, None, None);
}

/// By the fall: a fragment's one landing, the floor told whatever the ear gets.   (shards.ts GlassShards.landing)
pub fn glass_shards_landing(g: &mut crate::game::Game, fall: crate::shards::GlassShardsLandingFall, gain: f64, rate: f64, amount: f64, who: f64) {
    let t = g.time + fall.t;
    let cap = g.k.shards.pending_max;
    crate::sim_fallshome::fall_store_schedule(g, t, fall.pos[0], fall.pos[1], fall.pos[2], crate::sim_fallshome::FallSound::GlassTinkle, gain, rate, amount, who, crate::sim_fallshome::FALL_HEARD, 0.0, cap, None, None);
}

/// A voice, inside the frame's cap and the voice's gap; a merged landing thickens the next.   (shards.ts GlassShards.sound)
pub fn glass_shards_sound(g: &mut crate::game::Game, name: &str, pos: V3, gain: f64, rate: f64) {
    if gain <= 0.0 { return; }
    let key = String::from(name);
    let gap = g.k.shards.sound_gap.get(key.clone()).copied().unwrap_or(0.0);
    let since = g.time - g.shards.sound_at.get(key.clone()).copied().unwrap_or(-1e9);
    if g.shards.sounds_this_frame >= g.k.shards.sounds_per_frame || (gap > 0.0 && since >= 0.0 && since < gap) {
        if gap > 0.0 {
            let owed = m::min(g.k.shards.sound_thicken, g.shards.sound_owed.get(key.clone()).copied().unwrap_or(0.0) + gain * 0.25);
            g.shards.sound_owed.set(key, owed);
            g.stats.shards.sounds_merged += 1;
        }
        return;
    }
    let owed = if gap > 0.0 && since >= 0.0 && since < gap + g.k.shards.sound_owed_secs { g.shards.sound_owed.get(key.clone()).copied().unwrap_or(0.0) } else { 0.0 };
    g.shards.sound_owed.set(key.clone(), 0.0);
    let now = g.time;
    g.shards.sound_at.set(key, now);
    g.shards.sounds_this_frame += 1.0;
    g.stats.shards.sounds += 1;
    seam_audio_play(g, name, v3::copy(pos), gain + owed, rate);
}

/// A landing the floor can hear: one 'prop' event a gap, floor-wide.   (shards.ts GlassShards.heard)
pub fn glass_shards_heard(g: &mut crate::game::Game, p: V3, level: f64, who: f64) {
    let (time, gap) = (g.time, g.k.shards.event_gap);
    if !crate::sim_fallshome::fall_store_heard(g, time, gap, None) { return; }
    g.stats.shards.events += 1;
    let serial = g.serial();
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: [p[0], 0.0, p[2]], time, loud: false, level: Some(level), who: Some(who), serial, ..crate::game::GameEvent::zeroed() });
}

/// Glass laid in a ½ m cell.   (shards.ts GlassShards.deposit)
pub fn glass_shards_deposit(g: &mut crate::game::Game, p: V3, amount: f64) { crate::sim_fallshome::fall_store_deposit(g, p[0], p[2], amount); }

/// 0‥1: how much glass lies under `pos`.   (shards.ts GlassShards.underfoot)
pub fn glass_shards_underfoot(g: &mut crate::game::Game, pos: V3) -> f64 {
    let a = crate::sim_fallshome::fall_store_at(g, pos[0], pos[2]);
    if js::truthy(a) { m::min(1.0, a / g.k.shards.crunch_full) } else { 0.0 }
}

/// A footfall on litter crunches, is louder, and grinds the glass down. Elsewhere `level` back untouched.   (shards.ts GlassShards.step)
pub fn glass_shards_step(g: &mut crate::game::Game, pos: V3, level: f64) -> f64 {
    if !crate::sim_fallshome::fall_store_any(g) || !shards_live(g) { return level; }
    let (full, grind, gain, add) = (g.k.shards.crunch_full, g.k.shards.grind, g.k.shards.crunch_gain, g.k.shards.crunch_add);
    let r = crate::sim_fallshome::fall_store_step(g, pos[0], pos[2], level, full, grind, gain, add);
    let k = r.k;
    if k == 0.0 { return level; }
    g.stats.shards.crunches += 1;
    g.shards.crunch_n += 1.0;
    let rate = 0.85 + 0.3 * hash01(g.shards.crunch_n * 97.0);
    seam_audio_play(g, "glassCrunch", [pos[0], 0.02, pos[2]], m::min(1.0, (0.35 + 0.65 * k) * m::min(1.5, 0.5 + level * 2.0)), rate);
    r.level
}

/// Per frame: the release hook's edges, waiting records, the live estimate, breaks, landings due.   (shards.ts GlassShards.update)
pub fn glass_shards_update(g: &mut crate::game::Game, _dt: f64) {
    let live = shards_live(g);
    use crate::glassbreak::GlassBreakOnRelease as Hook;
    let ours = matches!(g.glass_break.on_release, Some(Hook::Shards));
    if live && !g.shards.was_live && g.glass_break.on_release.is_none() { g.glass_break.on_release = Some(Hook::Shards); }
    else if !live && g.shards.was_live && ours { g.glass_break.on_release = None; }
    g.shards.hooked = matches!(g.glass_break.on_release, Some(Hook::Shards));
    g.shards.was_live = live;
    if !live {
        if js::truthy(crate::sim_fallshome::fall_store_waiting(g)) { crate::sim_fallshome::fall_store_clear(g, Some(true), None); }
        g.shards.deferred.clear();
        g.shards.deferred_n = 0.0;
        g.shards.breaks.clear();
    }
    while !g.shards.deferred.is_empty() && g.shards.recs_this_frame < g.k.shards.recs_per_frame {
        let e = g.shards.deferred.remove(0);
        g.shards.deferred_n -= e.count;
        glass_shards_send(g, e.pos, e.dir, e.count, e.o, e.seed);
    }
    if !js::truthy(crate::sim_fallshome::fall_store_waiting(g)) && g.shards.lives.is_empty() && g.shards.breaks.is_empty() {
        g.shards.recs_this_frame = 0.0;
        g.shards.sounds_this_frame = 0.0;
        return;
    }
    let now = g.time;
    let mut w = 0;
    for r in 0..g.shards.lives.len() {
        let l = g.shards.lives[r];
        if l.t > now { g.shards.lives[w] = l; w += 1; } else { g.shards.live_est -= l.n; }
    }
    g.shards.lives.truncate(w);
    if !g.shards.breaks.is_empty() {
        let b = core::mem::take(&mut g.shards.breaks);
        for it in b { glass_shards_shatter(g, it.c, it.speed); }
    }
    let (level, gap) = (g.k.shards.level, g.k.shards.event_gap);
    for p in crate::sim_fallshome::fall_store_due(g, now, level, gap, None) {
        glass_shards_sound(g, p.name.as_str(), p.pos, p.gain, p.rate);
        if p.heard {
            g.stats.shards.events += 1;
            let serial = g.serial();
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: [p.pos[0], 0.0, p.pos[2]], time: now, loud: false, level: Some(p.level), who: Some(p.who), serial, ..crate::game::GameEvent::zeroed() });
        }
    }
    g.shards.recs_this_frame = 0.0;
    g.shards.sounds_this_frame = 0.0;
}

/// The fracture part's release hook.   (shards.ts GlassShards.onRelease)
pub fn glass_shards_on_release(g: &mut crate::game::Game, f: &crate::shards::GlassFragmentLike) { glass_shards_release(g, f); }

/// A restart: nothing pending, no litter.   (shards.ts GlassShards.reset)
pub fn glass_shards_reset(g: &mut crate::game::Game) {
    glass_shards_clear(g);
    g.shards.log.clear();
    g.shards.serial = 0.0;
    g.shards.crunch_n = 0.0;
}

/// The particles and moving debris were wiped: nothing of ours is still owed.   (shards.ts GlassShards.clear)
pub fn glass_shards_clear(g: &mut crate::game::Game) {
    g.shards.sound_at = JsMap::new();
    g.shards.sound_owed = JsMap::new();
    crate::sim_fallshome::fall_store_clear(g, None, None);
    g.shards.deferred.clear();
    g.shards.deferred_n = 0.0;
    g.shards.pane_mem.clear();
    g.shards.lives.clear();
    g.shards.live_est = 0.0;
    g.shards.breaks.clear();
    g.shards.last_plate = None;
    g.shards.recs_this_frame = 0.0;
    g.shards.sounds_this_frame = 0.0;
}

/// Births alive or waiting.   (shards.ts GlassShards.live)
pub fn glass_shards_live(g: &mut crate::game::Game) -> f64 { g.shards.live_est + g.shards.deferred_n }

/// Glass plates in the pool now.   (shards.ts GlassShards.plateCount)
pub fn glass_shards_plate_count(g: &mut crate::game::Game) -> f64 {
    let mut n = 0.0;
    for c in g.destr.debris.list.iter() { if g.tab.debris_chunk[*c].glass.is_some() { n += 1.0; } }
    n
}

/// The albedo of the static under `p`, or a neutral floor.   (shards.ts GlassShards.underAlbedo)
pub fn glass_shards_under_albedo(g: &mut crate::game::Game, p: V3) -> V3 {
    let hit = crate::scene_collision::raycast(g, [p[0], p[1] + 0.05, p[2]], [0.0, -1.0, 0.0], p[1] + 0.6);
    let b = match hit { Some(h) if h.index >= 0.0 => g.engine.world.statics.get(h.index as usize).copied(), _ => None };
    match b { Some(b) => g.col.boxes[b].albedo, None => [0.4, 0.4, 0.4] }
}

/// A line in the tool's log, the last sixty kept.   (shards.ts GlassShards.note)
pub fn glass_shards_note(g: &mut crate::game::Game, s: &str) {
    let line = alloc::format!("{:.2} {}", g.time, s);
    g.shards.log.push(line);
    if g.shards.log.len() > 60 { g.shards.log.remove(0); }
}

/// One line for the panel.   (shards.ts GlassShards.summary)
pub fn glass_shards_summary(g: &mut crate::game::Game) -> String {
    let cells = crate::sim_fallshome::fall_store_cells(g);
    let mut lit = 0.0;
    for c in cells.iter() { lit += c[2]; }
    let (live, plates) = (shards_live(g), glass_shards_plate_count(g));
    let s = &g.stats.shards;
    alloc::format!("{}{} · {} fragments → {} plates ({} now, {} broke, {} crumbled, {} refused) + {} particles in {} emits (~{} live of {}, {} trimmed) · {} sounds, {} heard, {} crunches · litter {:.0} cm² in {} cells",
        if live { "live" } else { "inert" }, if g.shards.hooked { " (release hook)" } else { "" }, s.fragments, s.plates, plates, s.breaks, s.crumbled, s.plates_refused, s.born, s.emits, g.shards.live_est, g.k.shards.live_cap, s.trimmed, s.sounds, s.events, s.crunches, lit * 1e4, cells.len())
}

/// A hand test: a fragment of `area` m² let go at `at`, thrown along `dir`.   (shards.ts GlassShards.test)
pub fn glass_shards_test(g: &mut crate::game::Game, at: V3, dir: Option<V3>, area: Option<f64>, kind: Option<crate::shards::GlassShardsTestKind>, type_: Option<crate::glassbreak::GlassType>) -> bool {
    use crate::shards::{GlassFragmentLikeCause as Cause, GlassFragmentLikeKind as Kind, GlassShardsTestKind as TestKind};
    let dir = dir.unwrap_or([0.0, 0.0, 1.0]);
    let area = area.unwrap_or(0.06);
    let kind = match kind.unwrap_or(TestKind::Piece) { TestKind::Piece => Kind::Piece, TestKind::Sheet => Kind::Sheet, TestKind::Dagger => Kind::Dagger, TestKind::Clump => Kind::Clump };
    let type_ = type_.unwrap_or(crate::glassbreak::GlassType::Annealed);
    let n: V3 = if m::hyp2(dir[0], dir[2]) > 1e-6 { v3::normalize([dir[0], 0.0, dir[2]]) } else { [0.0, 0.0, 1.0] };
    let s = m::sqrt(area) / 2.0;
    let u_: V3 = [n[2], 0.0, -n[0]];
    let corner = |a: f64, b: f64| -> V3 { [at[0] + u_[0] * a * s, at[1] + b * s, at[2] + u_[2] * a * s] };
    let outline = vec![corner(-1.0, -1.0), corner(1.0, -1.0), corner(1.0, 1.0), corner(-1.0, 1.0)];
    g.shards.serial += 1.0;
    let seed = u32(m::imul(js::to_i32(g.shards.serial), js::to_i32(2654435761.0)) as f64);
    let f = GlassFragmentLike {
        pane: -1.0, type_, kind, outline, centre: v3::copy(at), normal: n, thickness: 0.006, area, velocity: [n[0] * 0.8, -0.3, n[2] * 0.8], spin: [2.0, 0.5, -1.5], tint: [0.72, 0.8, 0.82],
        grain: if kind == Kind::Clump { 0.009 } else { 0.0 }, cause: Cause::Hit, who: crate::consts::NOBODY_ID, seed,
    };
    glass_shards_release(g, &f)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_cells_name() { assert_eq!(cell_name(-3.0, 7.0), 2085383.0); }
    #[test]
    fn seed_words_are_unsigned() {
        // the TypeScript's own: (3000000000 ^ 0xC8A7) >>> 0, and (1.5 ^ 0xBACC) >>> 0
        assert_eq!(xor_u32(3000000000.0, 0xC8A7 as f64), XOR_A);
        assert_eq!(xor_u32(1.5, 0xBACC as f64), XOR_B);
    }
    #[test]
    fn axes_sort_stably_thinnest_first() {
        assert_eq!(sort_axes([0.2, 0.004, 0.1]), [1, 2, 0]);
        assert_eq!(sort_axes([0.1, 0.1, 0.004]), [2, 0, 1]);
    }
    #[test]
    fn the_walk_goes_round_the_rectangle() {
        assert_eq!(outline_walk(0.5, 1.0, 0.5), (-0.5, -0.5));
        assert_eq!(outline_walk(2.5, 1.0, 0.5), (1.0, 0.0));
        assert_eq!(outline_walk(3.5, 1.0, 0.5), (0.5, 0.5));
        assert_eq!(outline_walk(5.5, 1.0, 0.5), (-1.0, 0.0));
    }
    const XOR_A: f64 = 3000014503.0;
    const XOR_B: f64 = 47821.0;
}
