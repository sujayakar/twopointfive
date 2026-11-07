//! Bodies for skeleton stubs of scene_markings that translated code calls and no part had (the stub round).
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// The public helpers of this module (texelOf, faceIndex, faceHalf, resolveAlbedo, faceAreas) are still the skeleton's empty stubs and panic, so the bodies below
// use these private copies. MarkRes's Rust order is not MARK_RES's: its index is written out here (markings.ts:32).

/// RES_K[m.res ?? 'base'] × base. (markings.ts texelOf)
fn texel_for(res: Option<MarkRes>, base: f64) -> f64 {
    let k = match res.unwrap_or(MarkRes::Base) { MarkRes::Fine => 0.5, MarkRes::Base => 1.0, MarkRes::Coarse => 2.0 };
    base * k
}

/// MARK_FACES.indexOf(f). (markings.ts faceIndex)
fn face_at(f: MarkFace) -> usize {
    match f { MarkFace::X => 0, MarkFace::X2 => 1, MarkFace::Y => 2, MarkFace::Y2 => 3, MarkFace::Z => 4, MarkFace::Z2 => 5 }
}

/// MARK_RES.indexOf(r), the order the shader reads: fine, base, coarse. (markings.ts:32)
fn res_at(r: MarkRes) -> f64 {
    match r { MarkRes::Fine => 0.0, MarkRes::Base => 1.0, MarkRes::Coarse => 2.0 }
}

/// CELL_CHARS.indexOf of one cell's character; −1 for anything else.
fn cell_at(ch: Option<&u8>) -> i32 {
    match ch {
        Some(b) => CELL_CHARS.as_bytes().iter().position(|c| c == b).map_or(-1, |i| i as i32),
        None => -1,
    }
}

/// (markings.ts faceHalf's pick)
fn pick(a: V3, h: V3) -> f64 {
    m::abs(a[0]) * h[0] + m::abs(a[1]) * h[1] + m::abs(a[2]) * h[2]
}

/// (markings.ts faceHalf)
fn half_of(h: V3, f: usize) -> FaceHalf {
    let ax = face_axes()[f];
    FaceHalf { hu: pick(ax.u, h), hv: pick(ax.v, h), hn: pick(ax.n, h) }
}

/// (markings.ts resolveAlbedo)
fn albedo_of(p: Option<&MarkPal>, base: V3) -> V3 {
    let Some(p) = p else { return [base[0], base[1], base[2]] };
    if let Some(a) = p.albedo { return [a[0], a[1], a[2]]; }
    let k = p.mul.unwrap_or(1.0);
    [m::min(base[0] * k, 1.0), m::min(base[1] * k, 1.0), m::min(base[2] * k, 1.0)]
}

/// The total of faceAreas: 4·hu·hv over the six faces, in their order. (markings.ts faceAreas)
fn total_area(h: V3) -> f64 {
    let mut total = 0.0;
    for f in 0..6 {
        let s = half_of(h, f);
        total += 4.0 * s.hu * s.hv;
    }
    total
}

/// A marking's coverage fractions, resolved albedos, mean radiance and area. (markings.ts markStats)
pub fn mark_stats(m: &crate::scene_markings::MarkingDef, base: V3, texel_base: Option<f64>) -> crate::scene_markings::MarkStats {
    let t = texel_for(m.res, texel_base.unwrap_or(MARKING_TEXEL));
    let (w, h) = (m.size[0], m.size[1]);
    let mut cnt = [0.0_f64; 4];
    for r in m.cells.iter() {
        for b in r.as_bytes() {
            let e = cell_at(Some(b));
            if e >= 0 { cnt[e as usize] += 1.0; }
        }
    }
    let n = w * h;
    let f = [cnt[1] / n, cnt[2] / n, cnt[3] / n];
    let alb = [albedo_of(m.pal.get(0), base), albedo_of(m.pal.get(1), base), albedo_of(m.pal.get(2), base)];
    let mut mean_l = [0.0_f64; 3];
    let mut d_albedo = [0.0_f64; 3];
    let mut emask = 0_i32;
    for k in 0..3 {
        let Some(p) = m.pal.get(k) else { continue };
        if let Some(e) = p.emit {
            emask |= 1 << k;
            for ch in 0..3 { mean_l[ch] += f[k] * e[ch]; }
        }
        if p.albedo.is_some() || p.mul.is_some_and(|x| x != 1.0) {
            for ch in 0..3 { d_albedo[ch] += f[k] * (alb[k][ch] - base[ch]); }
        }
    }
    MarkStats { texel: t, w, h, f, alb, mean_l, area: (w * t) * (h * t), d_albedo, emask: emask as f64 }
}

/// Rows the packed record takes. (markings.ts markRecordRows)
pub fn mark_record_rows(m: &crate::scene_markings::MarkingDef) -> f64 {
    MARK_HEAD_ROWS + m::ceil(m.size[0] * m.size[1] / CELLS_PER_ROW)
}

/// A Float32Array's store: the double rounded to single once; past the end, nothing.
fn put(out: &mut [f32], at: usize, v: f64) {
    if let Some(slot) = out.get_mut(at) { *slot = v as f32; }
}

/// The record at vec4 row `row` of the table that the host uploads to the GPU. Pure: it touches only `out`. (markings.ts packMarking)
pub fn pack_marking(out: &mut Vec<f32>, row: f64, m: &crate::scene_markings::MarkingDef, st: crate::scene_markings::MarkStats, last: bool, any_emit: bool, area_lit: bool) {
    let o = (row * 4.0) as usize;
    let end = core::cmp::min((row * 4.0 + mark_record_rows(m) * 4.0) as usize, out.len());
    if o < end { out[o..end].fill(0.0); }
    let mut pal_flags = 0_i32;
    for k in 0..3 {
        let Some(p) = m.pal.get(k) else { continue };
        if p.albedo.is_some() || p.mul.is_some_and(|x| x != 1.0) { pal_flags |= 1 << k; }
        if p.rough.is_some() { pal_flags |= 8 << k; }
    }
    let meta = face_at(m.face) as f64 + 8.0 * res_at(m.res.unwrap_or(MarkRes::Base)) + 32.0 * (if last { 1.0 } else { 0.0 }) + 64.0 * (if any_emit { 1.0 } else { 0.0 }) + 128.0 * (if area_lit { 1.0 } else { 0.0 }) + 256.0 * st.emask + 2048.0 * pal_flags as f64;
    put(out, o, m.at[0]); put(out, o + 1, m.at[1]); put(out, o + 2, m.size[0]); put(out, o + 3, m.size[1]);
    put(out, o + 4, meta); put(out, o + 5, st.f[0]); put(out, o + 6, st.f[1]); put(out, o + 7, st.f[2]);
    for k in 0..3 {
        let p = m.pal.get(k);
        let b = o + 8 + k * 8;
        let a = st.alb[k];
        put(out, b, a[0]); put(out, b + 1, a[1]); put(out, b + 2, a[2]);
        put(out, b + 3, p.and_then(|p| p.rough).unwrap_or(-1.0));
        let emit = p.and_then(|p| p.emit);
        put(out, b + 4, emit.map_or(0.0, |e| e[0])); put(out, b + 5, emit.map_or(0.0, |e| e[1])); put(out, b + 6, emit.map_or(0.0, |e| e[2]));
        put(out, b + 7, 0.0);
    }
    let (w, h) = (m.size[0] as usize, m.size[1] as usize);
    let cb = o + MARK_HEAD_ROWS as usize * 4;
    let per_float = CELLS_PER_FLOAT as usize;
    for j in 0..h {
        let row_str = m.cells[h - 1 - j].as_bytes();
        for i in 0..w {
            let e = cell_at(row_str.get(i));
            if e <= 0 { continue; }
            let c = j * w + i;
            let at = cb + c / per_float;
            if let Some(slot) = out.get_mut(at) { *slot = (*slot as f64 + (e << (2 * (c % per_float))) as f64) as f32; }
        }
    }
}

/// The albedo the secondary rays see for a marked box: base + each marking's mean change × its share of the box's surface. (markings.ts secondaryAlbedo)
pub fn secondary_albedo(base: V3, h: V3, marks: &[crate::scene_markings::MarkingDef], texel_base: Option<f64>) -> V3 {
    let texel_base = texel_base.unwrap_or(MARKING_TEXEL);
    let total = total_area(h);
    if total <= 0.0 { return [base[0], base[1], base[2]]; }
    let mut out = [base[0], base[1], base[2]];
    for mk in marks.iter() {
        let st = mark_stats(mk, base, Some(texel_base));
        let w = m::min(st.area / total, 1.0);
        out[0] += st.d_albedo[0] * w;
        out[1] += st.d_albedo[1] * w;
        out[2] += st.d_albedo[2] * w;
    }
    [m::min(m::max(out[0], 0.0), 1.0), m::min(m::max(out[1], 0.0), 1.0), m::min(m::max(out[2], 0.0), 1.0)]
}

/// The world centre, half sizes and outward normal of a marking's bitmap on box (c, h, yaw): where the game stands its light. (markings.ts markWorldRect)
pub fn mark_world_rect(m: &crate::scene_markings::MarkingDef, c_2: V3, h: V3, yaw: f64, texel_base: Option<f64>) -> crate::scene_markings::MarkWorldRect {
    let f = face_at(m.face);
    let ax = face_axes()[f];
    let t = texel_for(m.res, texel_base.unwrap_or(MARKING_TEXEL));
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let to_world = |l: V3| -> V3 { [cs * l[0] + sn * l[2], l[1], -sn * l[0] + cs * l[2]] };
    let (n, u_w, v_w) = (to_world(ax.n), to_world(ax.u), to_world(ax.v));
    let hn = half_of(h, f).hn;
    let uc = (m.at[0] + m.size[0] / 2.0) * t;
    let vc = (m.at[1] + m.size[1] / 2.0) * t;
    MarkWorldRect {
        centre: [c_2[0] + n[0] * hn + u_w[0] * uc + v_w[0] * vc, c_2[1] + n[1] * hn + u_w[1] * uc + v_w[1] * vc, c_2[2] + n[2] * hn + u_w[2] * uc + v_w[2] * vc],
        n,
        hu: m.size[0] * t / 2.0,
        hv: m.size[1] * t / 2.0,
    }
}
