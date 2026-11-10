//! THE MARKING TABLE: every list of painted marks some box carries, packed once by what it says. A box's list is found by
//! its content, its box's colour and size: every crate of a kind shares one.
use super::Row4;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use game::js::JsSet;
use game::m;
use game::scene_markings::{mark_key, mark_stats, MarkRes, MarkingDef};

const PER_BOX: usize = 4;
const TABLE: usize = 6144;
const SLOT_MAX: usize = (1 << 14) - 1;
const HEAD: usize = 8;
const PER_FLOAT: usize = 12;
const PER_ROW: usize = 48;

#[derive(Default)]
pub struct Marks {
    pub texel: f64,
    pub rows: Vec<Row4>,
    /// the rows have grown since the host last took them
    pub dirty: bool,
    by: BTreeMap<u64, Option<(u32, [f64; 3])>>,
    area_lit: Vec<u32>,
}

fn res(r: Option<MarkRes>) -> usize { match r.unwrap_or(MarkRes::Base) { MarkRes::Fine => 0, MarkRes::Base => 1, MarkRes::Coarse => 2 } }
fn rows_of(mk: &MarkingDef) -> usize { HEAD + (mk.size[0] as usize * mk.size[1] as usize).div_ceil(PER_ROW) }

impl Marks {
    /// which marks are lights (their glow is an analytic light's); true = another set than before, and every box's place in the table is dealt anew
    pub fn lit(&mut self, lit: &JsSet<u32>) -> bool {
        if lit.len() == self.area_lit.len() && lit.iter().zip(&self.area_lit).all(|(a, b)| a == *b) { return false; }
        self.area_lit = lit.keys();
        self.again();
        true
    }
    /// every list is packed anew as it is next met
    pub fn again(&mut self) { self.by.clear(); self.rows.clear(); self.dirty = true; }

    /// the first row (from 1) of a box's list, and the colour a ray that does not see the marks is to find; None = no marks, or no room
    pub fn slot(&mut self, list: &[MarkingDef], base: [f64; 3], h: [f64; 3]) -> Option<(u32, [f64; 3])> {
        if list.is_empty() { return None; }
        let mut k = 0xcbf2_9ce4_8422_2325u64;
        let mut eat = |v: u64| k = (k ^ v).wrapping_mul(0x0000_0100_0000_01b3);
        for mk in list {
            eat(mark_key(mk) as u64);
            eat(res(mk.res) as u64);
            for p in &mk.pal { for v in p.albedo.iter().flatten().chain(p.emit.iter().flatten()).chain(p.mul.iter()).chain(p.rough.iter()) { eat(v.to_bits()); } eat(p.albedo.is_some() as u64 | (p.emit.is_some() as u64) << 1 | (p.mul.is_some() as u64) << 2 | (p.rough.is_some() as u64) << 3); }
        }
        for v in base.iter().chain(h.iter()) { eat(v.to_bits()); }
        if let Some(s) = self.by.get(&k) { return *s; }
        let list = &list[..list.len().min(PER_BOX)];
        let rows: usize = list.iter().map(rows_of).sum();
        let made = if self.rows.len() + rows > TABLE || self.rows.len() + 1 > SLOT_MAX { None } else {
            let first = self.rows.len();
            self.pack(list, base);
            Some((first as u32 + 1, self.secondary(list, base, h)))
        };
        self.by.insert(k, made);
        made
    }

    /// the colour by area: each mark's mean change by its share of the box's faces
    fn secondary(&self, list: &[MarkingDef], base: [f64; 3], h: [f64; 3]) -> [f64; 3] {
        let total = 8.0 * (h[1] * h[2] + h[0] * h[2] + h[0] * h[1]);
        if total <= 0.0 { return base; }
        let mut out = base;
        for mk in list {
            let st = mark_stats(mk, base, Some(self.texel));
            let w = m::min(st.area / total, 1.0);
            for i in 0..3 { out[i] += st.d_albedo[i] * w; }
        }
        out.map(|x| m::min(m::max(x, 0.0), 1.0))
    }

    fn pack(&mut self, list: &[MarkingDef], base: [f64; 3]) {
        let stats: Vec<_> = list.iter().map(|mk| mark_stats(mk, base, Some(self.texel))).collect();
        let any = stats.iter().any(|s| s.emask != 0.0);
        for (i, (mk, st)) in list.iter().zip(&stats).enumerate() {
            let at = self.rows.len();
            self.rows.resize(at + rows_of(mk), Row4 { w: [0.0; 4] });
            let rows = &mut self.rows[at..];
            let mut says = 0u32;
            for k in 0..3 { if let Some(p) = mk.pal.get(k) { if p.albedo.is_some() || p.mul.map_or(false, |x| x != 1.0) { says |= 1 << k; } if p.rough.is_some() { says |= 8 << k; } } }
            let lit = self.area_lit.contains(&mark_key(mk));
            let meta = game::scene_markings::face_at(mk.face) as u32 + 8 * res(mk.res) as u32 + 32 * (i + 1 == list.len()) as u32 + 64 * any as u32 + 128 * lit as u32 + 256 * st.emask as u32 + 2048 * says;
            rows[0].w = [mk.at[0] as f32, mk.at[1] as f32, mk.size[0] as f32, mk.size[1] as f32];
            rows[1].w = [meta as f32, st.f[0] as f32, st.f[1] as f32, st.f[2] as f32];
            for k in 0..3 {
                let (p, a) = (mk.pal.get(k), st.alb[k]);
                let e = p.and_then(|p| p.emit).unwrap_or([0.0; 3]);
                rows[2 + k * 2].w = [a[0] as f32, a[1] as f32, a[2] as f32, p.and_then(|p| p.rough).unwrap_or(-1.0) as f32];
                rows[3 + k * 2].w = [e[0] as f32, e[1] as f32, e[2] as f32, 0.0];
            }
            let (w, h) = (mk.size[0] as usize, mk.size[1] as usize);
            let mut words = [0u32; 4 * ((32 * 32 + PER_ROW - 1) / PER_ROW)];
            for j in 0..h {
                let line = mk.cells[h - 1 - j].as_bytes();
                for x in 0..w {
                    let e = match line.get(x) { Some(b'a') => 1, Some(b'b') => 2, Some(b'c') => 3, _ => 0u32 };
                    let c = j * w + x;
                    words[c / PER_FLOAT] += e << (2 * (c % PER_FLOAT));
                }
            }
            for c in 0..(w * h).div_ceil(PER_FLOAT) { rows[HEAD + c / 4].w[c % 4] = words[c] as f32; }
        }
        self.dirty = true;
    }
}
