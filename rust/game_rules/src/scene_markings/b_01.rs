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
