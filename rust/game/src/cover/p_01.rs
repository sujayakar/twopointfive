//! src/game/cover.ts lines 1 to 233
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The gaps of a nav grid: `along_x` lays the runs along x, the caller asks both ways; `solid` says whether a line is shut to sight. (cover.ts gapsOf)
pub fn gaps_of(nav: &crate::cover::GapsOfNav, along_x: bool, k: Option<crate::knobs::CoverK>, solid: Option<&mut dyn FnMut(V3, V3) -> bool>) -> Vec<CoverOpeningId> {
    // (`K = COVER`, with no `g` to read it from: the page's table, the same for the four rows read here, which nothing writes as a game runs)
    let k = k.unwrap_or_else(crate::knobs::page_cover);
    gap_openings(nav, along_x, &k, solid).into_iter().map(seam_opening_id).collect()
}

/// A row of `g.tab.cover_opening` for a new opening: gapsOf has no `g`. (cover.ts:197, the literal in `mk`)
fn seam_opening_id(o: Opening) -> CoverOpeningId { todo!("needs: gaps_of to return Vec<Opening>, or to take g") }

/// One run of free cells in row `v`, cells `a` to `b`. (cover.ts Run)
#[derive(Clone, Copy)]
struct Run { a: i32, b: i32, v: i32 }

/// The grid as gapsOf reads it, its axes swapped when the runs lie along z.
struct Grid<'a> { nav: &'a crate::cover::GapsOfNav, along_x: bool, w: i32, h: i32, c: f64 }

impl Grid<'_> {
    /// Blocked, or off the grid. (cover.ts blk)
    fn blk(&self, u: i32, v: i32) -> bool {
        if u < 0 || v < 0 || u >= self.w || v >= self.h { return true; }
        let nav_w = self.nav.w as i32;
        let i = if self.along_x { v * nav_w + u } else { u * nav_w + v };
        self.nav.blocked[i as usize] == 1
    }
    /// (cover.ts pt)
    fn pt(&self, u: f64, v: f64) -> V3 { if self.along_x { [u, CHEST, v] } else { [v, CHEST, u] } }
    /// The middle of a run. (cover.ts at)
    fn at(&self, r: Run) -> V3 { self.pt(((r.a + r.b + 1) as f64) * 0.5 * self.c, ((r.v as f64) + 0.5) * self.c) }
    /// Any free cell across the run's width in row `v`. (cover.ts open)
    fn open(&self, r: Run, v: i32) -> bool {
        for u in r.a..=r.b { if !self.blk(u, v) { return true; } }
        false
    }
    /// Something a man cannot be seen over on this side of the chain: tried down the middle of a run's row and along both its edges. (cover.ts side)
    fn side(&self, ch: &[Run], lo: bool, solid: &mut Option<&mut dyn FnMut(V3, V3) -> bool>) -> bool {
        let Some(solid) = solid else { return true; };
        for r in ch {
            for k in [0.5, 0.02, 0.98] {
                let v = ((r.v as f64) + k) * self.c;
                let u = ((r.a + r.b + 1) as f64) * 0.5 * self.c;
                let beyond = if lo { ((r.a as f64) - 0.5) * self.c - 0.45 } else { ((r.b as f64) + 1.5) * self.c + 0.45 };
                if solid(self.pt(u, v), self.pt(beyond, v)) { return true; }
            }
        }
        false
    }
    /// (cover.ts mk)
    fn mk(&self, kind: crate::cover::OpeningKind, r: Run, other: Option<Run>, ci: usize, n: [f64; 2]) -> Opening {
        let p = self.at(r);
        Opening {
            id: format!("{}@{},{}", kind.as_str(), to_fixed_2(p[0]), to_fixed_2(p[2])),
            kind,
            at: p,
            n,
            half: ((r.b - r.a + 1) as f64) * self.c * 0.5 + 0.34,
            door: None,
            pass: match other { Some(_) => Some(format!("{}{}", if self.along_x { "z" } else { "x" }, ci)), None => None },
            far: match other { Some(o) => Some(self.at(o)), None => None },
        }
    }
}

/// `x.toFixed(2)` of a positive number: the language takes the larger on an exact tie (an odd eighth), where Rust's format takes the even.
fn to_fixed_2(x: f64) -> String {
    let e = x * 8.0;
    if m::floor(e) == e && js::rem(e, 2.0) != 0.0 { format!("{:.2}", x + 0.001) } else { format!("{:.2}", x) }
}

/// The body of gapsOf, the openings as values. (cover.ts gapsOf)
fn gap_openings(nav: &crate::cover::GapsOfNav, along_x: bool, k: &crate::knobs::CoverK, mut solid: Option<&mut dyn FnMut(V3, V3) -> bool>) -> Vec<Opening> {
    let (nav_w, nav_h) = (nav.w as i32, nav.h as i32);
    let grid = Grid { nav, along_x, w: if along_x { nav_w } else { nav_h }, h: if along_x { nav_h } else { nav_w }, c: nav.cell };
    let (w, h) = (grid.w, grid.h);
    let mut chains: Vec<Vec<Run>> = Vec::new();
    for v in 0..h {
        let mut u = 0;
        while u < w {
            if grid.blk(u, v) { u += 1; continue; }
            let a = u;
            while u < w && !grid.blk(u, v) { u += 1; }
            let b = u - 1;
            // (a run that reaches the grid's edge is open ground, not a gap)
            if ((b - a + 1) as f64) > k.gap_cells || a == 0 || b == w - 1 { continue; }
            let link = chains.iter_mut().find(|ch| {
                let l = ch[ch.len() - 1];
                if l.v >= v || (l.v as f64) < (v as f64) - 1.0 - k.bridge || l.a > b || l.b < a { return false; }
                // (bridged rows: wider there, not walled)
                for w in (l.v + 1)..v {
                    for x in a.max(l.a)..=b.min(l.b) { if grid.blk(x, w) { return false; } }
                }
                true
            });
            match link {
                Some(ch) => ch.push(Run { a, b, v }),
                None => chains.push(vec![Run { a, b, v }]),
            }
        }
    }
    let mut out: Vec<Opening> = Vec::new();
    let n: [f64; 2] = if along_x { [0.0, 1.0] } else { [1.0, 0.0] };
    for (ci, ch) in chains.iter().enumerate() {
        let (first, last) = (ch[0], ch[ch.len() - 1]);
        let l = last.v - first.v + 1;
        let (lo, hi) = (grid.open(first, first.v - 1), grid.open(last, last.v + 1));
        if !grid.side(ch, true, &mut solid) || !grid.side(ch, false, &mut solid) { continue; }
        if lo && hi {
            if (l as f64) <= k.through_rows {
                out.push(grid.mk(crate::cover::OpeningKind::Gap, ch[ch.len() >> 1], None, ci, n));
            } else {
                out.push(grid.mk(crate::cover::OpeningKind::Mouth, first, Some(last), ci, n));
                out.push(grid.mk(crate::cover::OpeningKind::Mouth, last, Some(first), ci, n));
            }
        } else if (lo || hi) && (l as f64) >= k.alcove_rows {
            out.push(grid.mk(crate::cover::OpeningKind::Mouth, if lo { first } else { last }, None, ci, n));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// gapsOf's own output under bun with EXACT on: name, alongX, solid given, then each opening (id, kind, at, n, half, pass, far as double bits) and the count and checksum of `solid`'s calls.
    const TS: &str = "\
doorway true false gap@2.00,1.25 gap 4000000000000000 3ff199999999999a 3ff4000000000000 0/1 3feae147ae147ae2 - -\n\
doorway true false calls 0 0\n\
doorway true true gap@2.00,1.25 gap 4000000000000000 3ff199999999999a 3ff4000000000000 0/1 3feae147ae147ae2 - -\n\
doorway true true calls 2 82bbfffffffff158\n\
doorway false false calls 0 0\n\
doorway false true calls 0 0\n\
passage true false mouth@2.00,0.75 mouth 4000000000000000 3ff199999999999a 3fe8000000000000 0/1 3feae147ae147ae2 z0 4000000000000000/3ff199999999999a/4006000000000000\n\
passage true false mouth@2.00,2.75 mouth 4000000000000000 3ff199999999999a 4006000000000000 0/1 3feae147ae147ae2 z0 4000000000000000/3ff199999999999a/3fe8000000000000\n\
passage true false calls 0 0\n\
passage true true mouth@2.00,0.75 mouth 4000000000000000 3ff199999999999a 3fe8000000000000 0/1 3feae147ae147ae2 z0 4000000000000000/3ff199999999999a/4006000000000000\n\
passage true true mouth@2.00,2.75 mouth 4000000000000000 3ff199999999999a 4006000000000000 0/1 3feae147ae147ae2 z0 4000000000000000/3ff199999999999a/3fe8000000000000\n\
passage true true calls 2 bd47fffffffff158\n\
passage false false calls 0 0\n\
passage false true calls 0 0\n\
alcove true false mouth@1.00,0.38 mouth 3ff0000000000000 3ff199999999999a 3fd8000000000000 0/1 3fe2e147ae147ae2 - -\n\
alcove true false calls 0 0\n\
alcove true true mouth@1.00,0.38 mouth 3ff0000000000000 3ff199999999999a 3fd8000000000000 0/1 3fe2e147ae147ae2 - -\n\
alcove true true calls 2 7edaaaaaaaaaa40d\n\
alcove false false calls 0 0\n\
alcove false true calls 0 0\n\
bridge true false mouth@1.00,0.38 mouth 3ff0000000000000 3ff199999999999a 3fd8000000000000 0/1 3fe2e147ae147ae2 z0 3ff0000000000000/3ff199999999999a/3ffa000000000000\n\
bridge true false mouth@1.00,1.63 mouth 3ff0000000000000 3ff199999999999a 3ffa000000000000 0/1 3fe2e147ae147ae2 z0 3ff0000000000000/3ff199999999999a/3fd8000000000000\n\
bridge true false calls 0 0\n\
bridge true true mouth@1.00,0.38 mouth 3ff0000000000000 3ff199999999999a 3fd8000000000000 0/1 3fe2e147ae147ae2 z0 3ff0000000000000/3ff199999999999a/3ffa000000000000\n\
bridge true true mouth@1.00,1.63 mouth 3ff0000000000000 3ff199999999999a 3ffa000000000000 0/1 3fe2e147ae147ae2 z0 3ff0000000000000/3ff199999999999a/3fd8000000000000\n\
bridge true true calls 2 7edaaaaaaaaaa40d\n\
bridge false false calls 0 0\n\
bridge false true calls 2 4b30aaaaaaaaa473\n\
twin true false gap@0.25,0.25 gap 3fd0000000000000 3ff199999999999a 3fd0000000000000 0/1 3fd8f5c28f5c28f6 - -\n\
twin true false gap@0.70,0.25 gap 3fe6666666666667 3ff199999999999a 3fd0000000000000 0/1 3fdc28f5c28f5c2a - -\n\
twin true false gap@0.55,0.45 gap 3fe199999999999a 3ff199999999999a 3fdccccccccccccd 0/1 3fd8f5c28f5c28f6 - -\n\
twin true false calls 0 0\n\
twin true true gap@0.25,0.25 gap 3fd0000000000000 3ff199999999999a 3fd0000000000000 0/1 3fd8f5c28f5c28f6 - -\n\
twin true true calls 11 ae68b55c5d7fa066\n\
twin false false calls 0 0\n\
twin false true calls 2 cf655555555546be";

    const GRIDS: &[(&str, f64, &[&str])] = &[
        ("doorway", 0.5, &["..........", "..........", "###..#####", "..........", ".........."]),
        ("passage", 0.5, &["..........", "###..#####", "###..#####", "###..#####", "###..#####", "###..#####", ".........."]),
        ("alcove", 0.25, &["..........", "###..#####", "###..#####", "###..#####", "###..#####", "##########"]),
        ("bridge", 0.25, &[".........", "###..####", "###..####", "#.....###", "###..####", "###..####", "###..####", "........."]),
        ("twin", 0.1, &["............", "##.###..####", "##.###..####", "............", "#####.######", "............"]),
    ];

    #[test]
    fn gaps_match_the_typescript_bits() {
        let mut k = crate::knobs::CoverK::zeroed();
        (k.gap_cells, k.through_rows, k.alcove_rows, k.bridge) = (3.0, 3.0, 3.0, 3.0);
        let mut out: Vec<String> = Vec::new();
        for &(name, cell, rows) in GRIDS {
            let (h, w) = (rows.len(), rows[0].len());
            let blocked: Vec<u8> = rows.iter().flat_map(|r| r.bytes().map(|b| (b == b'#') as u8)).collect();
            let nav = crate::cover::GapsOfNav { w: w as f64, h: h as f64, cell, blocked };
            for along_x in [true, false] {
                for use_solid in [false, true] {
                    let (mut calls, mut sum) = (0, 0u64);
                    let mut solid = |a: V3, b: V3| -> bool {
                        calls += 1;
                        for x in a.iter().chain(b.iter()) { sum = (sum << 1) ^ (sum >> 63) ^ x.to_bits(); }
                        js::rem(m::floor(a[0] * 7.0 + b[2] * 3.0), 5.0) != 0.0
                    };
                    let got = gap_openings(&nav, along_x, &k, if use_solid { Some(&mut solid) } else { None });
                    for o in got {
                        out.push(format!("{} {} {} {} {} {:x} {:x} {:x} {}/{} {:x} {} {}", name, along_x, use_solid, o.id, o.kind.as_str(),
                            o.at[0].to_bits(), o.at[1].to_bits(), o.at[2].to_bits(), o.n[0], o.n[1], o.half.to_bits(), o.pass.as_deref().unwrap_or("-"),
                            match o.far { Some(f) => format!("{:x}/{:x}/{:x}", f[0].to_bits(), f[1].to_bits(), f[2].to_bits()), None => String::from("-") }));
                    }
                    out.push(format!("{} {} {} calls {} {:x}", name, along_x, use_solid, calls, sum));
                }
            }
        }
        let want: Vec<&str> = TS.lines().collect();
        assert_eq!(out.iter().map(|s| s.as_str()).collect::<Vec<_>>(), want);
    }

    #[test]
    fn to_fixed_takes_the_larger_on_a_tie() {
        assert_eq!(to_fixed_2(0.125), "0.13");
        assert_eq!(to_fixed_2(0.375), "0.38");
        assert_eq!(to_fixed_2(1.625), "1.63");
        assert_eq!(to_fixed_2(2.0), "2.00");
        assert_eq!(to_fixed_2(0.7000000000000001), "0.70");
    }
}
