//! ONE BOX, AS THE GPU READS IT: its place and flags (32 bytes), its material (64 bytes) and, where it is cut or round, its planes (common.wgsl's layouts;
//! src/scene/boxes.ts says each word). A pure function of the box and of what the page says once of its look.
use super::marks::Marks;
use super::{Cut, Geo, Look, Mat};
use game::m;
use game::scene_boxes::Box;
use game::scene_stuff::BoxStuff;

pub const NO_PRIMARY: u32 = 1;
pub const NO_SHADOW: u32 = 2;
pub const CUTAWAY: u32 = 4;
const EMISSIVE: u32 = 16;
const ROT: u32 = 64;
pub const GLASS: u32 = 1 << 17;
pub const SOUND_SEAL: u32 = 1 << 16;
const CUT_BIT: u32 = 1 << 16;
/// a round thing's bit: bit 3, which no shader reads of a box
const ROUND_BIT: u32 = 8;
/// a key's system, in its top byte: a door's leaf, and what stands of a holed wall's paint
pub const DOOR: u32 = 1;
const SKIN: u32 = 10;

pub fn flags(b: &Box) -> u32 { b.flags as i64 as u32 }
pub fn key(b: &Box) -> u32 { b.mkey.map_or(super::NO_KEY, |k| k as i64 as u32) }
pub fn round(b: &Box) -> bool { b.round == Some(true) }
pub fn cut(b: &Box) -> bool { b.cut.as_ref().map_or(false, |c| !c.is_empty()) }
/// the picture's emission, where it is not the box's own (a lamp's face under the fog as the team last saw it)
pub fn emission(b: &Box) -> [f64; 3] { b.fog_emit.unwrap_or(b.emissive) }
fn gain(b: &Box) -> f64 { b.fog_mark.or(b.mark_gain).unwrap_or(1.0) }
fn solid_glass(b: &Box) -> bool { b.stuff == Some(BoxStuff::Glass) && flags(b) & GLASS == 0 }
fn lum(a: [f64; 3]) -> f64 { 0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2] }

/// the colour a box is sent in: its own by its wetness, and a thing of solid glass near black
pub fn sent(b: &Box, look: &Look) -> [f64; 3] {
    let wet = b.wet_k.unwrap_or(1.0);
    let dark = look.glass_lum >= 0.0 && b.marks.is_none() && solid_glass(b) && lum(b.albedo) > look.glass_lum;
    if dark { [look.glass_lum * wet; 3] } else { b.albedo.map(|a| a * wet) }
}
/// its finish as one number: roughness, and 2 for a metal, 4 for an anisotropic one
pub fn finish(b: &Box, look: &Look) -> f64 {
    let own = b.rough.unwrap_or(1.0);
    let coat = match b.stuff { Some(s) if look.coat[s as usize] >= 0.0 => Some(look.coat[s as usize]), _ if look.glass_lum >= 0.0 && solid_glass(b) => Some(look.glass_rough), _ => None };
    let r = match coat { Some(c) if c < own => c, _ => own };
    m::min(1.0, m::max(0.0, r)) + if b.metal == Some(true) { 2.0 } else { 0.0 } + if b.aniso == Some(true) { 4.0 } else { 0.0 }
}
/// what it is made of, whether it moves, and a number of its own that shifts its map (render/gbuffer.ts lookWordOf)
fn word(b: &Box, moves: bool, look: &Look) -> f64 {
    if !look.on { return 0.0; }
    let n = |v: f64, k: f64| m::round(v * k) as i64;
    let seed = if !moves { n(b.c[0], 100.0) + 7919 * n(b.c[1], 100.0) + 104729 * n(b.c[2], 100.0) }
        else if let Some(k) = b.mkey { k as i64 }
        else { n(b.h[0], 1000.0) + 131 * n(b.h[1], 1000.0) + 17161 * n(b.h[2], 1000.0) };
    let mut h = (seed as u32).wrapping_mul(0x9e37_79b1);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    (stuff(b.stuff, look) + if moves { 32 } else { 0 } + 64 * (h % 45)) as f64
}
pub fn stuff(s: Option<BoxStuff>, look: &Look) -> u32 { s.map_or(0, |s| look.stuff[s as usize]) }

/// half a float, rounded to the nearest
pub fn f16(v: f64) -> u32 {
    let x = (v as f32).to_bits();
    let sign = (x >> 16) & 0x8000;
    let exp = ((x >> 23) & 0xff) as i32 - 127 + 15;
    let mut mant = x & 0x7f_ffff;
    if exp <= 0 {
        if exp < -10 { return sign; }
        mant |= 0x80_0000;
        let shift = (14 - exp) as u32;
        let mut r = mant >> shift;
        if (mant >> (shift - 1)) & 1 != 0 { r += 1; }
        return sign | r;
    }
    if exp >= 31 { return sign | 0x7c00; }
    let mut r = sign | ((exp as u32) << 10) | (mant >> 13);
    if mant & 0x1000 != 0 { r += 1; }
    r
}
/// what a half's bits stand for
pub fn f16_value(bits: u32) -> f64 {
    let (s, e, mant) = (if bits & 0x8000 != 0 { -1.0 } else { 1.0 }, ((bits >> 10) & 31) as i32, (bits & 1023) as f64);
    if e == 0 { s * mant * 5.9604644775390625e-8 } else { s * (1.0 + mant / 1024.0) * m::pow(2.0, (e - 15) as f64) }
}

/// where it stands: the words that move when it does
pub fn place(geo: &mut Geo, mat: &mut Mat, c: [f64; 3], yaw: f64, rot: Option<[f64; 4]>) {
    for k in 0..3 { geo.w[k] = (c[k] as f32).to_bits(); }
    geo.w[3] = f16(m::cos(yaw)) | (f16(m::sin(yaw)) << 16);
    let q = rot.unwrap_or_else(|| [0.0, m::sin(yaw / 2.0), 0.0, m::cos(yaw / 2.0)]);
    for k in 0..4 { mat.w[8 + k] = q[k] as f32; }
}

/// a box's rows; `cut_rows` is written where it is cut or round
pub fn pack(b: &Box, c: [f64; 3], h: [f64; 3], moves: bool, look: &Look, marks: &mut Marks, geo: &mut Geo, mat: &mut Mat, cut_rows: &mut Cut) {
    let (own, is_round) = (flags(b), look.round && round(b));
    let mut f = (own & if look.round { 0xf7 } else { 0xff }) | if is_round { ROUND_BIT } else { 0 } | (((b.owner as i64 as u32) & 0xff) << 8) | if b.rot.is_some() { ROT } else { 0 } | (own & GLASS);
    let em = emission(b);
    if em[0] + em[1] + em[2] > 0.0 { f |= EMISSIVE; }
    let a = sent(b, look);
    let mut s = a;
    // (a tilted box carries no markings: a face's frame is by yaw alone)
    if let (true, Some(list), None) = (look.marks, b.marks.as_ref(), b.rot) {
        if let Some((slot, sec)) = marks.slot(list, b.albedo, b.h) {
            f |= slot << 18;
            let wet = b.wet_k.unwrap_or(1.0);
            s = sec.map(|x| x * wet);
        }
    }
    if is_round { round_rows(cut_rows, b.h[0]); }
    else if look.cut && cut(b) { f |= CUT_BIT; cut_rows_of(cut_rows, b, look); }
    place(geo, mat, c, b.yaw, b.rot);
    for k in 0..3 { geo.w[4 + k] = (h[k] as f32).to_bits(); mat.w[k] = a[k] as f32; mat.w[4 + k] = em[k] as f32; mat.w[12 + k] = s[k] as f32; }
    geo.w[7] = f;
    mat.w[3] = b.cut_height as f32;
    mat.w[7] = (finish(b, look) + 8.0 * word(b, moves && key(b) >> 24 != SKIN, look)) as f32;
    mat.w[15] = gain(b) as f32;
}

const PLANES: usize = 8;
fn cut_rows_of(out: &mut Cut, b: &Box, look: &Look) {
    let planes = b.cut.as_ref().unwrap();
    for k in 0..PLANES {
        let r = match planes.get(k) { Some(p) => [p.n[0], p.n[1], p.n[2], p.d], None => [0.0, 0.0, 0.0, 1.0] };
        for i in 0..4 { out.w[k * 4 + i] = r[i] as f32; }
    }
    let (round, arc, old) = (b.cut_round.unwrap_or(0.0), b.cut_arc.unwrap_or(0.0), b.cut_old == Some(true));
    if arc > 0.0 && round >= 1.0 && planes.len() < PLANES { out.w[(PLANES - 1) * 4 + 3] = (1.0 + arc) as f32; }
    let core = b.cut_core.map_or([0.0; 4], |c| [c[0], c[1], c[2], 1.0]);
    for i in 0..3 { out.w[32 + i] = core[i] as f32; }
    let more = match b.cut_glass.as_ref() {
        Some(g) => 2.0 + if g.anim == Some(true) { 4.0 } else { 0.0 } + 8.0 * m::round(m::min(3.75, m::max(0.0, g.rim)) * 4.0) + m::min(0.98, m::max(0.0, g.see)),
        None => (if (1.0..=3.0).contains(&round) { 1024.0 * round } else { 0.0 }) + if old { 512.0 + 4096.0 * (stuff(b.cut_core_stuff.or(b.stuff), look) & 31) as f64 } else { 0.0 },
    };
    out.w[35] = (core[3] + more) as f32;
}
/// the prism a round thing is drawn as: eight planes round its own y, with the box's four sides twelve
fn round_rows(out: &mut Cut, r: f64) {
    for k in 0..PLANES {
        let d = (30 + 30 * k + 30 * (k >> 1)) as f64 * core::f64::consts::PI / 180.0;
        let row = [m::cos(d), 0.0, m::sin(d), r];
        for i in 0..4 { out.w[k * 4 + i] = row[i] as f32; }
    }
    out.w[32..36].copy_from_slice(&[0.0, 0.0, 0.0, 128.0]);
}
