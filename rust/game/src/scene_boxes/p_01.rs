//! src/scene/boxes.ts lines 1 to 1209
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the game's Box (the header's `Box` is alloc's)
type Bx = crate::scene_boxes::Box;

// BoxWorld's static numbers: the skeleton has no entry for a class's statics.
const MAX_CAST: f64 = 16.0;
/// `BoxWorld.castCap`: only `?allproxies=0` of the address moves it, so its value on the page
const CAST_CAP: f64 = MAX_CAST;
const CAPS_PER_CHAR: f64 = 15.0;
const INFO_BOUNDS: f64 = 24.0;
const RS: usize = 12;
const RAY_PAD: f64 = 1e-3;
// the browser's GPUBufferUsage words
const USAGE_COPY_SRC: f64 = 4.0;
const USAGE_COPY_DST: f64 = 8.0;
const USAGE_UNIFORM: f64 = 64.0;
const USAGE_STORAGE: f64 = 128.0;

/// the box an ID names: the one place that says which table holds it
#[inline]
fn bx(g: &Game, id: BoxId) -> &Bx {
    &g.col.boxes[id]
}

/// a new box into the game's table (boxes.ts:171 and :182 hand back the object itself)
fn seam_box_push(g: &mut Game, b: Bx) -> BoxId {
    todo!("needs: the table every Box lives in (ids.txt: Box has no table; g.col.boxes is the statics' list, where a drawn or dynamic box would shift the indices)")
}
/// the same for `box_of`, whose stub takes no `g`
fn seam_box_push_no_g(b: Bx) -> BoxId {
    todo!("needs: `g` on box_of, and the table every Box lives in")
}
/// `device.queue.writeBuffer(matBuf, offset, data, 0, bytes)` (boxes.ts:312, :317)
fn seam_write_mat_buf(g: &mut Game, offset: f64, data: &[f32], bytes: f64) {
    todo!("needs: a boundary call for the GPU queue's writeBuffer on matBuf")
}
/// `own.w` as the TypeScript casts it, `{ stamp, ray }`, taken out (boxes.ts:405)
fn seam_own_take(own: &mut crate::scene_boxes::BoxWorldRayAsideOwn) -> Option<(Vec<u32>, f64)> {
    todo!("needs: BoxWorldRayAsideOwnW to hold stamp: Vec<u32> and ray: f64 (it came out an empty struct)")
}
/// `own.w = { stamp, ray }` (boxes.ts:409)
fn seam_own_put(own: &mut crate::scene_boxes::BoxWorldRayAsideOwn, stamp: Vec<u32>, ray: f64) {
    todo!("needs: BoxWorldRayAsideOwnW to hold stamp: Vec<u32> and ray: f64 (it came out an empty struct)")
}
/// `this.glassOpen(i, p)` (boxes.ts:1047), the hook glassbreak.ts:214 sets
fn seam_glass_open(g: &mut Game, i: f64, p: V3) -> bool {
    todo!("needs: BoxWorld.glassOpen as an enum or a direct call, not FnField (glassbreak.ts:214 sets it to glass_break_live(g) && glass_break_open_at(g, i, p))")
}
/// `AREA_LIT_MARKS.has(m)`, a set of MarkingDef objects by identity (boxes.ts:505)
fn seam_area_lit(g: &mut Game, m: &crate::scene_markings::MarkingDef) -> bool {
    todo!("needs: a serial on MarkingDef (AREA_LIT_MARKS is a JsSet<u32> and a MarkingDef has no number to ask it with)")
}

/// A piece of the picture, which no game ray meets. (boxes.ts lookKeyed)
pub fn look_keyed(g: &mut crate::game::Game, b: BoxId) -> bool {
    let (dress, mkey) = { let x = bx(g, b); (x.dress, x.mkey) };
    dress == Some(true) || (match mkey { Some(k) => js::to_u32(k) >> 24 == js::to_u32(crate::scene_boxes::mkey().look) >> 24, None => false })
}

/// boxOf's value, before it has a place in the table
fn box_value(c: V3, h: V3, yaw: f64, albedo: V3, emissive: V3, flags: f64, owner: f64, rot: Option<Quat>, name: Option<String>) -> Bx {
    Bx { c, h, yaw, albedo, emissive, flags, owner, cut_height: 1e6, rot, name, ..Bx::zeroed() }
}

/// A box with every field in place. (boxes.ts boxOf)
pub fn box_of(c_2: V3, h: V3, yaw: f64, albedo: V3, emissive: V3, flags: f64, owner: f64, rot: Option<Quat>, name: Option<&str>) -> BoxId {
    seam_box_push_no_g(box_value(c_2, h, yaw, albedo, emissive, flags, owner, rot, name.map(String::from)))
}

/// A box from what the caller gives; absent = boxOf's. (boxes.ts makeBox)
pub fn make_box(g: &mut crate::game::Game, p: &crate::scene_boxes::MakeBoxP) -> BoxId {
    let mut b = box_value(p.c, p.h, 0.0, [0.5, 0.5, 0.5], [0.0, 0.0, 0.0], 0.0, 0.0, None, None);
    if let Some(x) = p.yaw { b.yaw = x; }
    if let Some(x) = p.albedo { b.albedo = x; }
    if let Some(x) = p.emissive { b.emissive = x; }
    if let Some(x) = p.flags { b.flags = x; }
    if let Some(x) = p.owner { b.owner = x; }
    if let Some(x) = p.cut_height { b.cut_height = x; }
    b.wet_k = p.wet_k;
    b.rot = p.rot;
    b.dc = p.dc;
    b.dh = p.dh;
    b.name = p.name.clone();
    b.rough = p.rough;
    b.metal = p.metal;
    b.stuff = p.stuff;
    b.does = p.does;
    b.aniso = p.aniso;
    b.marks = p.marks.clone();
    b.mark_gain = p.mark_gain;
    b.fog_emit = p.fog_emit;
    b.fog_mark = p.fog_mark;
    b.mkey = p.mkey;
    b.cut = p.cut.clone();
    b.cut_core = p.cut_core;
    b.cut_glass = p.cut_glass;
    b.cut_round = p.cut_round;
    b.cut_arc = p.cut_arc;
    b.cut_old = p.cut_old;
    b.cut_core_stuff = p.cut_core_stuff;
    b.round = p.round;
    b.was = p.was.clone();
    b.shape = p.shape.clone();
    b.shaped = p.shaped;
    b.piece = p.piece;
    b.of = p.of;
    b.dress = p.dress;
    seam_box_push(g, b)
}

/// A copy of a box to change. (boxes.ts copyBox)
pub fn copy_box(g: &mut crate::game::Game, b: BoxId) -> BoxId {
    let copy = bx(g, b).clone();
    seam_box_push(g, copy)
}

/// A box's two words, to go on the box made from its paint. (boxes.ts wordOf)
pub fn word_of(b: crate::scene_boxes::WordOfB) -> crate::scene_boxes::WordOf {
    crate::scene_boxes::WordOf { stuff: b.stuff, does: b.does }
}

/// A thing of solid glass: says 'glass' and is no pane. (boxes.ts solidGlassBox)
pub fn solid_glass_box(b: crate::scene_boxes::SolidGlassBoxB) -> bool {
    b.stuff == Some(crate::scene_stuff::BoxStuff::Glass) && js::to_i32(b.flags) & js::to_i32(box_flag().glass) == 0
}

/// The air grid's rows, in matBuf's tail. (boxes.ts BoxWorld.setAirGrid)
pub fn box_world_set_air_grid(g: &mut crate::game::Game, cells: &[f32]) {
    let max_boxes = g.engine.world.max_boxes;
    let offset = max_boxes * MAT_STRIDE * 4.0 + crate::scene_markings::MARK_TABLE_ROWS * 16.0 + GB_ROWS * 16.0;
    let bytes = m::min((cells.len() * 4) as f64, crate::scene_air_grid::air_rows() * 16.0);
    seam_write_mat_buf(g, offset, cells, bytes);
}

/// The hills' rows, in matBuf's last region; nothing without the switch. (boxes.ts BoxWorld.setGroundField)
pub fn box_world_set_ground_field(g: &mut crate::game::Game, rows: &[f32]) {
    if !g.engine.world.ground_on { return; }
    let max_boxes = g.engine.world.max_boxes;
    let offset = max_boxes * MAT_STRIDE * 4.0 + crate::scene_markings::MARK_TABLE_ROWS * 16.0 + GB_ROWS * 16.0 + crate::scene_air_grid::air_rows() * 16.0 + max_boxes * crate::scene_cut::CUT_STRIDE * 16.0;
    let bytes = m::min((rows.len() * 4) as f64, crate::scene_ground_field::gh_rows() * 16.0);
    seam_write_mat_buf(g, offset, rows, bytes);
}

/// A ray asked by the picture: the ray grid's stamps are the asker's, the last ray's scratch put back after. (boxes.ts BoxWorld.rayAside)
pub fn box_world_ray_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_boxes::BoxWorldRayAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let need = g.engine.world.rg_stamp.len();
    let (q_stamp, q_ray) = match seam_own_take(own) {
        Some((stamp, ray)) if stamp.len() >= need => (stamp, ray),
        _ => (vec![0u32; need], 0.0),
    };
    let w = &mut g.engine.world;
    let (best, bi, bn, ray) = (w.rc_best, w.rc_bi, w.rc_bn, w.rg_ray);
    let stamp = core::mem::replace(&mut w.rg_stamp, q_stamp);
    w.rg_ray = q_ray;
    own.w = None;
    let out = fn_(g);
    let w = &mut g.engine.world;
    let own_stamp = core::mem::replace(&mut w.rg_stamp, stamp);
    let own_ray = w.rg_ray;
    w.rg_ray = ray;
    w.rc_best = best;
    w.rc_bi = bi;
    w.rc_bn = bn;
    seam_own_put(own, own_stamp, own_ray);
    out
}

/// The world's buffers and tables. (boxes.ts BoxWorld.constructor)
pub fn box_world_new(g: &mut crate::game::Game, device: crate::Opaque, max_boxes: Option<f64>) -> crate::scene_boxes::BoxWorld {
    let wd = crate::scene_world::world();
    let max_boxes = max_boxes.unwrap_or(wd.max_boxes);
    let mut w = crate::scene_boxes::BoxWorld::blank();
    w.device = device;
    w.max_boxes = max_boxes;
    if max_boxes != wd.max_boxes { panic!("BoxWorld: maxBoxes must be WORLD.maxBoxes (the shaders place the marking table after that many mat rows)"); }
    let n = max_boxes as usize;
    w.geo_f32 = vec![0.0; n * GEO_STRIDE as usize];
    w.geo_u32 = vec![0; n * GEO_STRIDE as usize];
    w.mat_data = vec![0.0; n * MAT_STRIDE as usize];
    let s = USAGE_STORAGE + USAGE_COPY_DST;
    w.geo_buf = crate::gpu_device::make_buffer(device, max_boxes * GEO_STRIDE * 4.0, s + USAGE_COPY_SRC, Some("boxGeo"));
    w.cut_on = true;
    w.round_on = g.m.scene_round.round_config.gpu;
    w.ground_on = g.m.scene_ground_field.ground_ray_config.on;
    w.cut_data = vec![0.0; n * crate::scene_cut::CUT_STRIDE as usize * 4];
    let mat_bytes = max_boxes * MAT_STRIDE * 4.0 + crate::scene_markings::MARK_TABLE_ROWS * 16.0 + GB_ROWS * 16.0 + crate::scene_air_grid::air_rows() * 16.0 + max_boxes * crate::scene_cut::CUT_STRIDE * 16.0 + (if w.ground_on { crate::scene_ground_field::gh_rows() * 16.0 } else { 0.0 });
    w.mat_buf = crate::gpu_device::make_buffer(device, mat_bytes, s + USAGE_COPY_SRC, Some("boxMat"));
    if wd.grid_w + wd.grid_h > 96.0 { panic!("trace grid: the DDA in common.wgsl caps a ray at 96 cell visits; W + H must stay <= 96 for that cap to be unreachable"); }
    if max_boxes > 65536.0 || max_boxes * wd.grid_w * wd.grid_h >= 16777216.0 { panic!("trace grid: packed items keep the box index in 16 bits and cell starts in 24"); }
    w.cell_start = vec![0; (wd.grid_w * wd.grid_h + 1.0 + 2.0 * wd.grid_w * wd.grid_h) as usize];
    w.items = vec![0; 65536];
    w.cell_buf = crate::gpu_device::make_buffer(device, (w.cell_start.len() * 4) as f64, s, Some("gridCells"));
    w.item_buf = crate::gpu_device::make_buffer(device, (w.items.len() * 4) as f64, s, Some("gridItems"));
    w.info_buf = crate::gpu_device::make_buffer(device, (w.info_data.len() * 4) as f64, USAGE_UNIFORM + USAGE_COPY_DST, Some("sceneInfo"));
    w.capsule_buf = crate::gpu_device::make_buffer(device, (w.capsule_data.len() * 4) as f64, s, Some("capsules"));
    w.visible = vec![0; n];
    w.visible_sent = vec![0; n];
    w.visible_buf = crate::gpu_device::make_buffer(device, (w.visible.len() * 4) as f64, s, Some("visibleBoxes"));
    w
}

/// (boxes.ts BoxWorld.addStatic)
pub fn box_world_add_static(g: &mut crate::game::Game, b: BoxId) -> f64 {
    let w = &mut g.engine.world;
    w.statics.push(b);
    w.statics_dirty = true;
    (w.statics.len() - 1) as f64
}

/// A marking list into the table at its slot, and its secondary albedo. (boxes.ts BoxWorld.packList)
pub fn box_world_pack_list(g: &mut crate::game::Game, ent: &mut crate::scene_boxes::BoxWorldPackListEnt) {
    let mut row = ent.slot - 1.0;
    let n = ent.n as usize;
    let texel = g.engine.world.marking_texel;
    let cap = core::cmp::min(n, ent.marks.len());
    let mut stats: Vec<crate::scene_markings::MarkStats> = Vec::with_capacity(cap);
    for m_ in &ent.marks[..cap] { stats.push(crate::scene_markings::mark_stats(m_, ent.base, Some(texel))); }
    let any_emit = stats.iter().any(|st| st.emask != 0.0);
    for k in 0..n {
        let m_ = &ent.marks[k];
        let lit = seam_area_lit(g, m_);
        crate::scene_markings::pack_marking(&mut g.engine.world.mark_data, row, m_, stats[k], k == n - 1, any_emit, lit);
        row += crate::scene_markings::mark_record_rows(m_);
    }
    let sec = crate::scene_markings::secondary_albedo(ent.base, ent.h, &ent.marks[..cap], Some(texel));
    ent.sec[0] = sec[0];
    ent.sec[1] = sec[1];
    ent.sec[2] = sec[2];
    ent.any_emit = any_emit;
    g.engine.world.mark_dirty = true;
}

/// Every registered list again at a new base texel. (boxes.ts BoxWorld.repackMarkings)
pub fn box_world_repack_markings(g: &mut crate::game::Game, texel: Option<f64>) {
    let texel = texel.unwrap_or(g.engine.world.marking_texel);
    g.engine.world.marking_texel = texel;
    for key in g.engine.world.mark_by_key.keys() {
        let Some(row) = g.engine.world.mark_by_key.get(key.clone()) else { continue };
        if !js::truthy(row.slot) { continue; }
        let mut ent = crate::scene_boxes::BoxWorldPackListEnt { slot: row.slot, n: row.n, sec: row.sec, any_emit: row.any_emit, marks: row.marks.clone(), base: row.base, h: row.h };
        box_world_pack_list(g, &mut ent);
        let w = &mut g.engine.world;
        if let Some(row) = w.mark_by_key.get_mut(key.clone()) {
            row.sec = ent.sec;
            row.any_emit = ent.any_emit;
        }
        // (the TypeScript's markOfBox rows hold the same `sec` array)
        for (_, kept) in w.mark_of_box.iter_mut() {
            if kept.key == key { kept.sec = ent.sec; }
        }
    }
    g.engine.world.pack_epoch += 1.0;
}

/// Begin a frame's proxy list. (boxes.ts BoxWorld.resetCapsules)
pub fn box_world_reset_capsules(g: &mut crate::game::Game) {
    g.engine.world.capsule_count = 0.0;
    g.engine.world.char_count = 0.0;
}

/// rgb565 of a colour, clamped to 0..1
fn pack_clamped(a: V3) -> i32 {
    ((js::to_i32(m::round(m::min(1.0, m::max(0.0, a[0])) * 31.0)) & 31) << 11) | ((js::to_i32(m::round(m::min(1.0, m::max(0.0, a[1])) * 63.0)) & 63) << 5) | (js::to_i32(m::round(m::min(1.0, m::max(0.0, a[2])) * 31.0)) & 31)
}

/// One man's proxies, CAPS_PER_CHAR records. (boxes.ts BoxWorld.addCharacterCapsules)
pub fn box_world_add_character_capsules(g: &mut crate::game::Game, owner: f64, albedo: &crate::scene_boxes::BoxWorldAddCharacterCapsulesAlbedo, caps: &[Vec<f64>], bound: [f64; 4]) {
    use crate::scene_boxes::BoxWorldAddCharacterCapsulesAlbedo as Albedo;
    let w = &mut g.engine.world;
    if w.char_count >= CAST_CAP { return; }
    let own = js::to_i32(owner) & 0xff;
    let misc = match albedo {
        Albedo::Array(_) => 0u32,
        Albedo::List(one) => {
            let rgb565 = ((js::to_i32(m::round(one[0] * 31.0)) & 31) << 11) | ((js::to_i32(m::round(one[1] * 63.0)) & 63) << 5) | (js::to_i32(m::round(one[2] * 31.0)) & 31);
            (own | (rgb565 << 8)) as u32
        }
    };
    let base = w.char_count * CAPS_PER_CHAR;
    const NONE: [f64; 7] = [0.0, -100.0, 0.0, 0.0, 0.0, -100.0, 0.0];
    for i in 0..CAPS_PER_CHAR as usize {
        let c: &[f64] = match caps.get(i) { Some(c) => c, None => &NONE };
        let o = (base as usize + i) * 8;
        w.capsule_data[o] = c[0] as f32;
        w.capsule_data[o + 1] = c[1] as f32;
        w.capsule_data[o + 2] = c[2] as f32;
        w.capsule_data[o + 3] = c[3] as f32;
        w.capsule_data[o + 4] = c[4] as f32;
        w.capsule_data[o + 5] = c[5] as f32;
        w.capsule_data[o + 6] = c[6] as f32;
        let word = match albedo {
            Albedo::Array(per) => (own | (pack_clamped(*per.get(i).unwrap_or(&per[0])) << 8)) as u32,
            Albedo::List(_) => misc,
        };
        w.capsule_data[o + 7] = f32::from_bits(word);
    }
    let bo = (INFO_BOUNDS + w.char_count * 4.0) as usize;
    w.info_data[bo] = (bound[0] as f32).to_bits();
    w.info_data[bo + 1] = (bound[1] as f32).to_bits();
    w.info_data[bo + 2] = (bound[2] as f32).to_bits();
    w.info_data[bo + 3] = (bound[3] as f32).to_bits();
    w.char_count += 1.0;
    w.capsule_count = w.char_count * CAPS_PER_CHAR;
}

/// Half extents in world XZ of a yawed or rotated box. (boxes.ts BoxWorld.footprint)
pub fn box_world_footprint(g: &mut crate::game::Game, b: BoxId) -> [f64; 2] {
    footprint_of(bx(g, b))
}

fn footprint_of(b: &Bx) -> [f64; 2] {
    if let Some(rot) = b.rot {
        let ax = v3::quat::rotate(rot, [1.0, 0.0, 0.0]);
        let ay = v3::quat::rotate(rot, [0.0, 1.0, 0.0]);
        let az = v3::quat::rotate(rot, [0.0, 0.0, 1.0]);
        return [m::abs(ax[0]) * b.h[0] + m::abs(ay[0]) * b.h[1] + m::abs(az[0]) * b.h[2], m::abs(ax[2]) * b.h[0] + m::abs(ay[2]) * b.h[1] + m::abs(az[2]) * b.h[2]];
    }
    let cs = m::abs(m::cos(b.yaw));
    let sn = m::abs(m::sin(b.yaw));
    [cs * b.h[0] + sn * b.h[2], sn * b.h[0] + cs * b.h[2]]
}

/// What the game reads of the world: `all`, the glass statics, and the ray grid when a static's shape moved. Returns the statics' count. (boxes.ts BoxWorld.list)
pub fn box_world_list(g: &mut crate::game::Game) -> f64 {
    let w = &mut g.engine.world;
    w.all.clear();
    w.all.extend_from_slice(&w.statics);
    w.all.extend_from_slice(&w.dynamics);
    if w.all.len() as f64 > w.max_boxes { panic!("too many boxes: {}", w.all.len()); }
    let n_stat = w.statics.len();
    if w.statics_dirty || w.glass_statics_n != n_stat as f64 {
        g.engine.world.glass_statics.clear();
        let glass = js::to_i32(box_flag().glass);
        for i in 0..n_stat {
            let id = g.engine.world.statics[i];
            if js::to_i32(bx(g, id).flags) & glass != 0 { g.engine.world.glass_statics.push(i as f64); }
        }
        g.engine.world.statics_dirty = false;
        g.engine.world.glass_statics_n = n_stat as f64;
    }
    let w = &mut g.engine.world;
    if w.rg_shape.len() < n_stat * RS {
        w.rg_shape = vec![0.0; core::cmp::max(n_stat, w.max_boxes as usize) * RS];
        w.rg_n = -1.0;
    }
    let mut moved = w.rg_n != n_stat as f64;
    for i in 0..n_stat {
        let id = g.engine.world.all[i];
        let (c, h, yaw, rot) = { let b = bx(g, id); (b.c, b.h, b.yaw, b.rot) };
        let o = i * RS;
        let s = &mut g.engine.world.rg_shape;
        if s[o] == c[0] && s[o + 1] == c[1] && s[o + 2] == c[2] && s[o + 3] == yaw && s[o + 4] == h[0] && s[o + 5] == h[1] && s[o + 6] == h[2]
            && (match rot { Some(r) => s[o + 7] == 1.0 && s[o + 8] == r[0] && s[o + 9] == r[1] && s[o + 10] == r[2] && s[o + 11] == r[3], None => s[o + 7] == 0.0 }) { continue; }
        moved = true;
        s[o] = c[0]; s[o + 1] = c[1]; s[o + 2] = c[2]; s[o + 3] = yaw; s[o + 4] = h[0]; s[o + 5] = h[1]; s[o + 6] = h[2];
        match rot {
            Some(r) => { s[o + 7] = 1.0; s[o + 8] = r[0]; s[o + 9] = r[1]; s[o + 10] = r[2]; s[o + 11] = r[3]; }
            None => { s[o + 7] = 0.0; s[o + 8] = 0.0; s[o + 9] = 0.0; s[o + 10] = 0.0; s[o + 11] = 0.0; }
        }
    }
    if moved { box_world_build_ray_grid(g, n_stat as f64); }
    n_stat as f64
}

/// The list without the pack, for a tick nothing is drawn after. (boxes.ts BoxWorld.relist)
pub fn box_world_relist(g: &mut crate::game::Game) {
    box_world_list(g);
    g.engine.world.pack_behind = true;
}

/// Ray against all boxes, closest hit. (boxes.ts BoxWorld.raycast)
pub fn box_world_raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: Option<f64>, skip_flags: Option<f64>, skip_owner: Option<f64>, with_look: Option<bool>) -> Option<crate::scene_boxes::BoxWorldRaycast> {
    let tmax = tmax.unwrap_or(1e9);
    let skip_flags = skip_flags.unwrap_or_else(|| { let f = box_flag(); (js::to_i32(f.no_shadow) | js::to_i32(f.no_primary)) as f64 });
    let skip_owner = skip_owner.unwrap_or(0.0);
    let with_look = with_look.unwrap_or(false);
    let w = &mut g.engine.world;
    w.rc_best = tmax;
    w.rc_bi = -1.0;
    w.rc_bn = [0.0, 1.0, 0.0];
    let n_s = w.rg_n;
    if n_s < 0.0 || n_s > w.all.len() as f64 {
        let mut i = 0;
        while i < g.engine.world.all.len() {
            let id = g.engine.world.all[i];
            if with_look || !look_keyed(g, id) { box_world_ray_test(g, i as f64, ro, rd, skip_flags, skip_owner); }
            i += 1;
        }
    } else {
        let mut i = n_s as usize;
        while i < g.engine.world.all.len() {
            let id = g.engine.world.all[i];
            if with_look || !look_keyed(g, id) { box_world_ray_test(g, i as f64, ro, rd, skip_flags, skip_owner); }
            i += 1;
        }
        let mut k = 0;
        while k < g.engine.world.rg_globals.len() {
            let gi = g.engine.world.rg_globals[k];
            box_world_ray_test(g, gi, ro, rd, skip_flags, skip_owner);
            k += 1;
        }
        box_world_ray_walk(g, ro, rd, skip_flags, skip_owner);
    }
    let w = &g.engine.world;
    if w.rc_bi >= 0.0 { Some(crate::scene_boxes::BoxWorldRaycast { t: w.rc_best, index: w.rc_bi, n: w.rc_bn }) } else { None }
}

/// One candidate of the raycast, with the tie rule the index order gave it. (boxes.ts BoxWorld.rayTest)
pub fn box_world_ray_test(g: &mut crate::game::Game, i: f64, ro: V3, rd: V3, skip_flags: f64, skip_owner: f64) {
    let id = g.engine.world.all[i as usize];
    let (flags, owner) = { let b = bx(g, id); (b.flags, b.owner) };
    if js::to_i32(flags) & js::to_i32(skip_flags) != 0 { return; }
    if js::truthy(skip_owner) && owner == skip_owner { return; }
    let best = g.engine.world.rc_best;
    let Some(r) = ray_box(g, ro, rd, id, best) else { return };
    if r.t < best || (r.t == best && i < g.engine.world.rc_bi) {
        if g.engine.world.glass_open.is_some() && js::to_i32(flags) & js::to_i32(box_flag().glass) != 0 && i < g.engine.world.statics.len() as f64
            && seam_glass_open(g, i, [ro[0] + rd[0] * r.t, ro[1] + rd[1] * r.t, ro[2] + rd[2] * r.t]) { return; }
        let w = &mut g.engine.world;
        w.rc_best = r.t;
        w.rc_bi = i;
        w.rc_bn = r.n;
    }
}

/// The statics along the ray: a 2D DDA over the XZ cells, each cell's statics tested once. (boxes.ts BoxWorld.rayWalk)
pub fn box_world_ray_walk(g: &mut crate::game::Game, ro: V3, rd: V3, skip_flags: f64, skip_owner: f64) {
    let wd = crate::scene_world::world();
    let (w_, h_, cs) = (wd.grid_w, wd.grid_h, wd.grid_cell);
    let (gx, gz) = (w_ * cs, h_ * cs);
    let (ox, oz, dx, dz) = (ro[0], ro[2], rd[0], rd[2]);
    let (mut t0, mut t1) = (0.0, f64::INFINITY);
    if dx != 0.0 {
        let (ta, tb) = ((0.0 - ox) / dx, (gx - ox) / dx);
        if ta < tb { if ta > t0 { t0 = ta; } if tb < t1 { t1 = tb; } } else { if tb > t0 { t0 = tb; } if ta < t1 { t1 = ta; } }
    } else if ox < 0.0 || ox > gx { return; }
    if dz != 0.0 {
        let (ta, tb) = ((0.0 - oz) / dz, (gz - oz) / dz);
        if ta < tb { if ta > t0 { t0 = ta; } if tb < t1 { t1 = tb; } } else { if tb > t0 { t0 = tb; } if ta < t1 { t1 = ta; } }
    } else if oz < 0.0 || oz > gz { return; }
    if !(t0 <= t1) { return; }
    let mut cx = m::floor((ox + dx * t0) / cs);
    let mut cz = m::floor((oz + dz * t0) / cs);
    if cx < 0.0 { cx = 0.0; } else if cx > w_ - 1.0 { cx = w_ - 1.0; }
    if cz < 0.0 { cz = 0.0; } else if cz > h_ - 1.0 { cz = h_ - 1.0; }
    let step_x = if dx > 0.0 { 1.0 } else if dx < 0.0 { -1.0 } else { 0.0 };
    let step_z = if dz > 0.0 { 1.0 } else if dz < 0.0 { -1.0 } else { 0.0 };
    let dtx = if step_x != 0.0 { cs / m::abs(dx) } else { f64::INFINITY };
    let dtz = if step_z != 0.0 { cs / m::abs(dz) } else { f64::INFINITY };
    let mut tmx = if dx > 0.0 { ((cx + 1.0) * cs - ox) / dx } else if dx < 0.0 { (cx * cs - ox) / dx } else { f64::INFINITY };
    let mut tmz = if dz > 0.0 { ((cz + 1.0) * cs - oz) / dz } else if dz < 0.0 { (cz * cs - oz) / dz } else { f64::INFINITY };
    let w = &mut g.engine.world;
    w.rg_ray += 1.0;
    if w.rg_ray == 4294967295.0 {
        w.rg_stamp.fill(0);
        w.rg_ray = 1.0;
    }
    let ray = w.rg_ray as u32;
    let mut t_enter = t0;
    loop {
        if t_enter > g.engine.world.rc_best { return; }
        let c = cz * w_ + cx;
        // (a NaN origin leaves the cell NaN: the TypeScript's array reads give undefined and no candidate)
        if c == c {
            let c = c as usize;
            let (s, e) = (g.engine.world.rg_start[c] as usize, g.engine.world.rg_start[c + 1] as usize);
            for k in s..e {
                let i = g.engine.world.rg_items[k] as usize;
                if g.engine.world.rg_stamp[i] == ray { continue; }
                g.engine.world.rg_stamp[i] = ray;
                box_world_ray_test(g, i as f64, ro, rd, skip_flags, skip_owner);
            }
        }
        if tmx < tmz {
            t_enter = tmx;
            cx += step_x;
            if cx < 0.0 || cx >= w_ { return; }
            tmx += dtx;
        } else if tmz < f64::INFINITY {
            t_enter = tmz;
            cz += step_z;
            if cz < 0.0 || cz >= h_ { return; }
            tmz += dtz;
        } else {
            return;
        }
    }
}

/// The static prefix of `all` into the ray grid: padded footprints into cells, the oversize and outside ones as globals. (boxes.ts BoxWorld.buildRayGrid)
pub fn box_world_build_ray_grid(g: &mut crate::game::Game, n: f64) {
    let wd = crate::scene_world::world();
    let (w_, h_, cs) = (wd.grid_w, wd.grid_h, wd.grid_cell);
    let (wi, cells) = (w_ as usize, (w_ * h_) as usize);
    let count = n as usize;
    let w = &mut g.engine.world;
    let max_boxes = w.max_boxes as usize;
    if w.rg_ranges.len() < count * 4 { w.rg_ranges = vec![0; core::cmp::max(count, max_boxes) * 4]; }
    if w.rg_stamp.len() < max_boxes {
        w.rg_stamp = vec![0; max_boxes];
        w.rg_ray = 0.0;
    }
    w.rg_start.fill(0);
    w.rg_globals.clear();
    for i in 0..count {
        let id = g.engine.world.all[i];
        let r = i * 4;
        let b = bx(g, id);
        let [ex, ez] = footprint_of(b);
        let c = b.c;
        let x0 = m::floor((c[0] - ex - RAY_PAD) / cs);
        let x1 = m::floor((c[0] + ex + RAY_PAD) / cs);
        let z0 = m::floor((c[2] - ez - RAY_PAD) / cs);
        let z1 = m::floor((c[2] + ez + RAY_PAD) / cs);
        let w = &mut g.engine.world;
        if !(x0 >= 0.0 && z0 >= 0.0 && x1 < w_ && z1 < h_) || (x1 - x0 + 1.0) * (z1 - z0 + 1.0) > 96.0 {
            w.rg_ranges[r] = -1;
            w.rg_globals.push(i as f64);
            continue;
        }
        let (xa, xb, za, zb) = (x0 as i32, x1 as i32, z0 as i32, z1 as i32);
        w.rg_ranges[r] = xa;
        w.rg_ranges[r + 1] = xb;
        w.rg_ranges[r + 2] = za;
        w.rg_ranges[r + 3] = zb;
        for z in za as usize..=zb as usize { for x in xa as usize..=xb as usize { w.rg_start[z * wi + x] += 1; } }
    }
    let w = &mut g.engine.world;
    let mut sum = 0u32;
    for c in 0..cells {
        let k = w.rg_start[c];
        w.rg_start[c] = sum;
        sum += k;
    }
    w.rg_start[cells] = sum;
    if sum as usize > w.rg_items.len() { w.rg_items = vec![0; core::cmp::max(sum as usize, w.rg_items.len() * 2)]; }
    w.rg_fill.fill(0);
    for i in 0..count {
        let r = i * 4;
        let x0 = w.rg_ranges[r];
        if x0 < 0 { continue; }
        let (x1, z0, z1) = (w.rg_ranges[r + 1], w.rg_ranges[r + 2], w.rg_ranges[r + 3]);
        for z in z0 as usize..=z1 as usize {
            for x in x0 as usize..=x1 as usize {
                let c = z * wi + x;
                let at = w.rg_start[c] + w.rg_fill[c];
                w.rg_fill[c] += 1;
                w.rg_items[at as usize] = i as u32;
            }
        }
    }
    w.rg_n = n;
    w.ray_grid_stats.statics = n;
    w.ray_grid_stats.globals = w.rg_globals.len() as f64;
    w.ray_grid_stats.items = sum as f64;
}

/// Is this box's record a shape's and no break's? (boxes.ts shapeCut)
pub fn shape_cut(g: &mut crate::game::Game, b: BoxId) -> bool {
    let x = bx(g, b);
    x.shaped == Some(true) && x.cut.is_some()
}

/// Is this box's cut record met by the game's rays? Ask it only of a box that has one. (boxes.ts cutMet; CUT.on is settled on)
pub fn cut_met(g: &mut crate::game::Game, b: BoxId) -> bool {
    bx(g, b).cut.as_ref().unwrap().len() > 0
}

/// One slab of the test: false = the ray has left the box
#[inline]
fn slab(h: f64, o: f64, inv: f64, ax: i32, t0: &mut f64, t1: &mut f64, axis: &mut i32, sgn: &mut f64) -> bool {
    let (mut ta, mut tb) = ((-h - o) * inv, (h - o) * inv);
    let mut s = -1.0;
    if ta > tb {
        core::mem::swap(&mut ta, &mut tb);
        s = 1.0;
    }
    if ta > *t0 {
        *t0 = ta;
        *axis = ax;
        *sgn = s;
    }
    if tb < *t1 { *t1 = tb; }
    !(*t0 > *t1)
}

/// `quat.rotate(quat.conj(rot), v)` over scalars
#[inline]
fn unturn(rot: Quat, v: V3) -> V3 {
    let (qx, qy, qz, qw) = (-rot[0], -rot[1], -rot[2], rot[3]);
    let (tx, ty, tz) = (2.0 * (qy * v[2] - qz * v[1]), 2.0 * (qz * v[0] - qx * v[2]), 2.0 * (qx * v[1] - qy * v[0]));
    [v[0] + qw * tx + (qy * tz - qz * ty), v[1] + qw * ty + (qz * tx - qx * tz), v[2] + qw * tz + (qx * ty - qy * tx)]
}

/// rayBox's own lines: a box with neither record nor rotation
fn ray_plain(ro: V3, rd: V3, b: &Bx, tmax: f64) -> Option<(f64, V3)> {
    let cs = m::cos(b.yaw);
    let sn = m::sin(b.yaw);
    let (px, py, pz) = (ro[0] - b.c[0], ro[1] - b.c[1], ro[2] - b.c[2]);
    let (lx, lz, ly) = (cs * px - sn * pz, sn * px + cs * pz, py);
    let (dx, dz, dy) = (cs * rd[0] - sn * rd[2], sn * rd[0] + cs * rd[2], rd[1]);
    let (ix, iy, iz) = (1.0 / js::or(dx, 1e-20), 1.0 / js::or(dy, 1e-20), 1.0 / js::or(dz, 1e-20));
    let h = b.h;
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1, 0.0);
    if !slab(h[0], lx, ix, 0, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[1], ly, iy, 1, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[2], lz, iz, 2, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if axis < 0 { return None; }
    let (ln0, ln1, ln2) = (if axis == 0 { sgn } else { 0.0 }, if axis == 1 { sgn } else { 0.0 }, if axis == 2 { sgn } else { 0.0 });
    Some((t0, [cs * ln0 + sn * ln2, ln1, -sn * ln0 + cs * ln2]))
}

/// The slab test in the frame of a fully rotated box
fn ray_rot(ro: V3, rd: V3, b: &Bx, rot: Quat, tmax: f64) -> Option<(f64, V3)> {
    let o = unturn(rot, [ro[0] - b.c[0], ro[1] - b.c[1], ro[2] - b.c[2]]);
    let d = unturn(rot, [rd[0], rd[1], rd[2]]);
    let h = b.h;
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1, 0.0);
    if !slab(h[0], o[0], 1.0 / js::or(d[0], 1e-20), 0, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[1], o[1], 1.0 / js::or(d[1], 1e-20), 1, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[2], o[2], 1.0 / js::or(d[2], 1e-20), 2, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if axis < 0 { return None; }
    let mut ln = [0.0, 0.0, 0.0];
    ln[axis as usize] = sgn;
    Some((t0, v3::quat::rotate(rot, ln)))
}

/// The cylinder about the box's own y, radius h[0], between its end faces
fn ray_round(ro: V3, rd: V3, b: &Bx, tmax: f64) -> Option<(f64, V3)> {
    let (mut cs, mut sn) = (1.0, 0.0);
    let (px, py, pz) = (ro[0] - b.c[0], ro[1] - b.c[1], ro[2] - b.c[2]);
    let (ox, oy, oz, dx, dy, dz);
    if let Some(rot) = b.rot {
        let o = unturn(rot, [px, py, pz]);
        let d = unturn(rot, [rd[0], rd[1], rd[2]]);
        (ox, oy, oz, dx, dy, dz) = (o[0], o[1], o[2], d[0], d[1], d[2]);
    } else {
        cs = m::cos(b.yaw);
        sn = m::sin(b.yaw);
        (ox, oy, oz, dx, dy, dz) = (cs * px - sn * pz, py, sn * px + cs * pz, cs * rd[0] - sn * rd[2], rd[1], sn * rd[0] + cs * rd[2]);
    }
    let (r, hy) = (b.h[0], b.h[1]);
    let (mut t0, mut t1, mut face, mut sgn) = (0.0, tmax, 0, 0.0);
    {
        let inv = 1.0 / js::or(dy, 1e-20);
        let (mut ta, mut tb) = ((-hy - oy) * inv, (hy - oy) * inv);
        let mut s = -1.0;
        if ta > tb {
            core::mem::swap(&mut ta, &mut tb);
            s = 1.0;
        }
        if ta > t0 {
            t0 = ta;
            face = 1;
            sgn = s;
        }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return None; }
    }
    let (a, hb, c) = (dx * dx + dz * dz, ox * dx + oz * dz, ox * ox + oz * oz - r * r);
    if a < 1e-18 {
        if c > 0.0 { return None; }
    } else {
        let disc = hb * hb - a * c;
        if disc < 0.0 { return None; }
        let s = m::sqrt(disc);
        let (ta, tb) = ((-hb - s) / a, (-hb + s) / a);
        if ta > t0 {
            t0 = ta;
            face = 2;
        }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return None; }
    }
    if face == 0 { return None; }
    let (mut l0, mut l1, mut l2) = (0.0, sgn, 0.0);
    if face == 2 {
        let (hx, hz) = (ox + dx * t0, oz + dz * t0);
        let il = 1.0 / js::or(m::sqrt(hx * hx + hz * hz), 1.0);
        l0 = hx * il;
        l1 = 0.0;
        l2 = hz * il;
    }
    if let Some(rot) = b.rot { return Some((t0, v3::quat::rotate(rot, [l0, l1, l2]))); }
    Some((t0, [cs * l0 + sn * l2, l1, -sn * l0 + cs * l2]))
}

/// The closest hit of a ray on one box; origin inside = no hit. (boxes.ts rayBox)
pub fn ray_box(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBox> {
    let (round, has_cut) = { let x = bx(g, b); (x.round == Some(true), x.cut.is_some()) };
    if round { return ray_box_round(g, ro, rd, b, tmax).map(|r| crate::scene_boxes::RayBox { t: r.t, n: r.n }); }
    if has_cut && cut_met(g, b) { return ray_box_cut(g, ro, rd, b, tmax).map(|r| crate::scene_boxes::RayBox { t: r.t, n: r.n }); }
    let x = bx(g, b);
    let hit = match x.rot {
        Some(rot) => ray_rot(ro, rd, x, rot, tmax),
        None => ray_plain(ro, rd, x, tmax),
    };
    hit.map(|(t, n)| crate::scene_boxes::RayBox { t, n })
}

/// (boxes.ts rayBoxRot)
pub fn ray_box_rot(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxRot> {
    let x = bx(g, b);
    ray_rot(ro, rd, x, x.rot.unwrap(), tmax).map(|(t, n)| crate::scene_boxes::RayBoxRot { t, n })
}

/// The test against a box with a cut record: the slab interval, cut down by the record's planes. (boxes.ts rayBoxCut)
pub fn ray_box_cut(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxCut> {
    let (c, h, yaw, rot) = { let x = bx(g, b); (x.c, x.h, x.yaw, x.rot) };
    let (mut cs, mut sn) = (1.0, 0.0);
    let (px, py, pz) = (ro[0] - c[0], ro[1] - c[1], ro[2] - c[2]);
    let (ox, oy, oz, dx, dy, dz);
    if let Some(r) = rot {
        let o = unturn(r, [px, py, pz]);
        let d = unturn(r, [rd[0], rd[1], rd[2]]);
        (ox, oy, oz, dx, dy, dz) = (o[0], o[1], o[2], d[0], d[1], d[2]);
    } else {
        cs = m::cos(yaw);
        sn = m::sin(yaw);
        (ox, oy, oz, dx, dy, dz) = (cs * px - sn * pz, py, sn * px + cs * pz, cs * rd[0] - sn * rd[2], rd[1], sn * rd[0] + cs * rd[2]);
    }
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1, 0.0);
    if !slab(h[0], ox, 1.0 / js::or(dx, 1e-20), 0, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[1], oy, 1.0 / js::or(dy, 1e-20), 1, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[2], oz, 1.0 / js::or(dz, 1e-20), 2, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    // (the record is lent out of the box for the call: cut_clip takes `g` and reads no box)
    let cut = g.col.boxes[b].cut.take().unwrap();
    let met = crate::scene_cut::cut_clip(g, &cut, ox, oy, oz, dx, dy, dz, t0, t1);
    let (clip_t0, k) = (g.m.scene_cut.cut_clip.t0, g.m.scene_cut.cut_clip.k);
    let plane = if met && k >= 0.0 { Some(cut[k as usize].n) } else { None };
    g.col.boxes[b].cut = Some(cut);
    if !met { return None; }
    if k < 0.0 && axis < 0 { return None; }
    let ln: V3 = match plane {
        Some(n) => [n[0], n[1], n[2]],
        None => { let mut l = [0.0, 0.0, 0.0]; l[axis as usize] = sgn; l }
    };
    if let Some(r) = rot { return Some(crate::scene_boxes::RayBoxCut { t: clip_t0, n: v3::quat::rotate(r, ln) }); }
    Some(crate::scene_boxes::RayBoxCut { t: clip_t0, n: [cs * ln[0] + sn * ln[2], ln[1], -sn * ln[0] + cs * ln[2]] })
}

/// (boxes.ts rayBoxRound)
pub fn ray_box_round(g: &mut crate::game::Game, ro: V3, rd: V3, b: BoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxRound> {
    ray_round(ro, rd, bx(g, b), tmax).map(|(t, n)| crate::scene_boxes::RayBoxRound { t, n })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u32);
    impl Lcg {
        fn r(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
            self.0 as f64 / 4294967296.0
        }
    }
    fn fnv(h: u32, w: u32) -> u32 { (h ^ w).wrapping_mul(16777619) }
    fn fnv_f(h: u32, x: f64) -> u32 {
        let b = x.to_bits();
        fnv(fnv(h, b as u32), (b >> 32) as u32)
    }

    /// the same world as /tmp/p_scene_boxes_01_bits.ts builds, to the draw
    fn world(g: &mut Game, x: &mut Lcg) {
        g.engine.world = crate::scene_boxes::BoxWorld::blank();
        g.engine.world.max_boxes = 4096.0;
        for i in 0..220 {
            let c = [x.r() * 40.0, x.r() * 3.0, x.r() * 28.0];
            let mut h = [0.1 + x.r() * 3.0, 0.1 + x.r() * 1.5, 0.1 + x.r() * 3.0];
            let kind = m::floor(x.r() * 5.0) as i32;
            let (a, b_, c_, d) = (x.r() - 0.5, x.r() - 0.5, x.r() - 0.5, x.r() - 0.5);
            let yaw_draw = x.r() * 6.28;
            let (f0, f1, f2, f3) = (x.r(), x.r(), x.r(), x.r());
            let mut b = box_value(c, h, 0.0, [0.5, 0.5, 0.5], [0.0, 0.0, 0.0], 0.0, 0.0, None, None);
            if kind == 1 { b.yaw = yaw_draw; }
            if kind == 2 {
                let l = m::sqrt(a * a + b_ * b_ + c_ * c_ + d * d);
                b.rot = Some([a / l, b_ / l, c_ / l, d / l]);
            }
            if kind == 3 { h[2] = h[0]; b.h = h; b.round = Some(true); b.yaw = yaw_draw; }
            if kind == 4 { b.h = [60.0, 0.1, 60.0]; }
            let mut flags = 0;
            if f0 < 0.1 { flags |= 2; }
            if f1 < 0.1 { flags |= 1; }
            if f2 < 0.1 { flags |= 131072; }
            b.flags = flags as f64;
            let dynamic = i >= 200;
            if dynamic {
                b.owner = 1.0 + m::floor(f3 * 3.0);
                if f3 < 0.2 { b.mkey = Some(crate::scene_boxes::mkey().look + 5.0); }
                if f3 > 0.9 { b.dress = Some(true); }
            }
            let id = g.col.boxes.push(b);
            if dynamic { g.engine.world.dynamics.push(id); } else { box_world_add_static(g, id); }
        }
    }

    fn rays(g: &mut Game, x: &mut Lcg) -> u32 {
        let mut h = 2166136261u32;
        for j in 0..4000 {
            let ro = [x.r() * 50.0 - 5.0, x.r() * 3.0, x.r() * 38.0 - 5.0];
            let mut rd = [x.r() * 2.0 - 1.0, x.r() * 2.0 - 1.0, x.r() * 2.0 - 1.0];
            if j % 7 == 0 { rd[1] = 0.0; }
            if j % 11 == 0 { rd[0] = 0.0; }
            if j % 13 == 0 { rd = [0.0, -1.0, 0.0]; }
            let tmax = if j % 5 == 0 { Some(10.0) } else { None };
            let skip = if j % 3 == 0 { Some(2.0) } else { None };
            let owner = if j % 4 == 0 { Some(1.0) } else { None };
            let look = if j % 6 == 0 { Some(true) } else { None };
            match box_world_raycast(g, ro, rd, tmax, skip, owner, look) {
                None => h = fnv(h, 0xdead),
                Some(r) => { h = fnv_f(h, r.index); h = fnv_f(h, r.t); h = fnv_f(h, r.n[0]); h = fnv_f(h, r.n[1]); h = fnv_f(h, r.n[2]); }
            }
        }
        h
    }

    /// raycast through the grid, the brute force and the tests of every shape, against the TypeScript's own bits (bun, EXACT on)
    #[test]
    fn raycast_matches_the_typescripts_bits() {
        let mut g = Game::blank();
        let mut x = Lcg(12345);
        world(&mut g, &mut x);
        // no list yet: rg_n is −1, `all` empty, as in the TypeScript before an upload
        assert_eq!(box_world_list(&mut g), 200.0);
        assert_eq!(g.engine.world.all.len(), 220);
        let mut seen = 2166136261u32;
        seen = fnv_f(seen, g.engine.world.ray_grid_stats.globals);
        seen = fnv_f(seen, g.engine.world.ray_grid_stats.items);
        for i in 0..200 { let id = g.engine.world.all[i]; let f = box_world_footprint(&mut g, id); seen = fnv_f(seen, f[0]); seen = fnv_f(seen, f[1]); }
        assert_eq!(seen, TS_GRID, "grid and footprints");
        let hits = rays(&mut g, &mut x);
        assert_eq!(hits, TS_RAYS, "the rays through the grid");
        // the same rays by the brute force: the grid changes no answer
        let mut x2 = Lcg(777);
        g.engine.world.rg_n = -1.0;
        let brute = rays(&mut g, &mut x2);
        box_world_list(&mut g);
        g.engine.world.rg_n = 200.0;
        let mut x3 = Lcg(777);
        assert_eq!(rays(&mut g, &mut x3), brute, "grid against brute force");
    }

    #[test]
    fn a_moved_static_rebuilds_the_grid_and_a_repaint_does_not() {
        let mut g = Game::blank();
        let mut x = Lcg(99);
        world(&mut g, &mut x);
        box_world_list(&mut g);
        g.engine.world.rg_n = 200.0;
        let id = g.engine.world.statics[3];
        g.col.boxes[id].albedo = [1.0, 0.0, 0.0];
        g.engine.world.ray_grid_stats.items = -1.0;
        box_world_list(&mut g);
        assert_eq!(g.engine.world.ray_grid_stats.items, -1.0, "colour leaves it standing");
        g.col.boxes[id].c[0] += 1.0;
        box_world_list(&mut g);
        assert!(g.engine.world.ray_grid_stats.items > 0.0, "a move rebuilds it");
    }

    #[test]
    fn capsules_pack_as_the_typescript_does() {
        let mut g = Game::blank();
        g.engine.world = crate::scene_boxes::BoxWorld::blank();
        let caps = vec![vec![1.0, 2.0, 3.0, 0.5, 4.0, 5.0, 6.0]];
        box_world_add_character_capsules(&mut g, 300.0, &crate::scene_boxes::BoxWorldAddCharacterCapsulesAlbedo::List([1.0, 0.5, 0.0]), &caps, [1.0, 2.0, 3.0, 1.5]);
        let w = &g.engine.world;
        assert_eq!(w.char_count, 1.0);
        assert_eq!(w.capsule_count, 15.0);
        assert_eq!(w.capsule_data[3], 0.5);
        // owner 300 & 0xff = 44; rgb565 of (1, .5, 0): 31<<11 | 32<<5 | 0 = 0xFC00
        assert_eq!(w.capsule_data[7].to_bits(), 44 | (0xFC00 << 8));
        assert_eq!(w.capsule_data[8 + 1], -100.0);
        assert_eq!(w.info_data[24 + 3], 1.5f32.to_bits());
    }

    const TS_GRID: u32 = 0x92b81bdb;
    const TS_RAYS: u32 = 0x72cbf515;
}
