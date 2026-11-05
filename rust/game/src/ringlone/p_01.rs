//! src/game/ringlone.ts lines 1 to 89
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The way wedge `key` of `n` points, in radians; 1 is up, clockwise. (ringlone.ts wedgeMid)
pub fn wedge_mid(key: f64, n: f64) -> f64 {
    (-90.0 + (key - 1.0) * 360.0 / n) * m::PI / 180.0
}

/// The one row of a ring that has one (None: the wheel), for what the page draws. (ringlone.ts loneWedge)
pub fn lone_wedge(g: &mut crate::game::Game, v: Option<&crate::cmdring::RingView>) -> Option<crate::cmdring::RingViewWedgesItem> {
    match v {
        Some(v) if v.wedges.len() == 1 => Some(v.wedges[0].clone()),
        _ => None,
    }
}

/// The digit of a ring with one row, from the bits of the digits that have a wedge; 0 = the wheel, or no ring. (ringlone.ts loneKey)
pub fn lone_key(g: &mut crate::game::Game, keys: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> f64 {
    let k = keys(g);
    lone_digit(k)
}

/// loneKey's arithmetic on the bits. A signed result: a `k` above 0 whose int32 is 0 (0.5) gives -1 there.
fn lone_digit(k: f64) -> f64 {
    if k > 0.0 && js::to_i32(k) & js::to_i32(k - 1.0) == 0 { (31 - m::clz32(js::to_u32(k)) as i32) as f64 } else { 0.0 }
}

/// A wedge's words as drawn. (ringlone.ts wedgeWords)
pub fn wedge_words(w: &crate::ringlone::Wedge) -> String {
    if w.opens == Some(true) { format!("{} ▸", w.text) } else { w.text.clone() }
}

/// The box of wedge `key`'s pill on a wheel of `n`, its words `text_w` px wide: over the middle for a wedge pointing up or down, else out to its side. (ringlone.ts boxOf)
pub fn box_of(key: f64, n: f64, text_w: f64, tall: Option<bool>) -> crate::ringlone::BoxOf {
    let tall = tall.unwrap_or(false);
    let l = crate::ringlone::lone();
    let a = wedge_mid(key, n);
    let c = m::cos(a);
    let px = l.r * c;
    let py = l.r * m::sin(a);
    let e = l.h / 2.0;
    let wd = l.h + text_w + l.pad;
    let mid = m::abs(c) < 0.2;
    let left = !mid && c < 0.0;
    let x0 = if mid { -wd / 2.0 } else if left { px + e - wd } else { px - e };
    let grow = if tall { l.lead } else { 0.0 };
    let up = py < 0.0;
    crate::ringlone::BoxOf {
        x0,
        y0: py - e - (if up { grow } else { 0.0 }),
        x1: x0 + wd,
        y1: py + e + (if up { 0.0 } else { grow }),
        px,
        py,
        left,
    }
}

/// A pill's words as its lines: one, or two for a label longer than `max` letters that has a space to break at. (ringlone.ts loneLines)
pub fn lone_lines(g: &mut crate::game::Game, w: &crate::ringlone::Wedge) -> Vec<String> {
    lines_of(w)
}

/// loneLines' reckoning. Lengths count UTF-16 units and the cuts are made at ASCII spaces, so the byte offsets land where the TypeScript's do.
fn lines_of(w: &crate::ringlone::Wedge) -> Vec<String> {
    let max = crate::ringlone::lone().max;
    let s = wedge_words(w);
    if len16(&s) <= max { return vec![s]; }
    let mut cut: Option<[String; 2]> = None;
    let mut best = m::INF;
    for (i, _) in s.match_indices(' ') {
        let after = &s[i + 1..];
        let dot = after.starts_with("· ");
        let a = js_trim(&s[..i]);
        let b = js_trim(if dot { &after["· ".len()..] } else { after });
        if a.is_empty() || b.is_empty() || b == "▸" || a.ends_with('·') || b.starts_with('·') { continue; }
        let longer = m::max(len16(a), len16(b));
        let score = if dot && longer <= max { longer - max } else { longer }; // (a dot's break that fits comes before every other)
        if score < best { best = score; cut = Some([String::from(a), String::from(b)]); }
    }
    match cut {
        Some([a, b]) => vec![a, b],
        None => vec![s],
    }
}

/// A string's `.length` in the TypeScript.
fn len16(s: &str) -> f64 {
    s.encode_utf16().count() as f64
}

/// `String.prototype.trim`: Rust's `trim` also takes U+0085 and leaves U+FEFF.
fn js_trim(s: &str) -> &str {
    s.trim_matches(|c: char| (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}')
}

/// The width a line of a pill's words is set to. (ringlone.ts wordsW)
pub fn words_w(s: &str) -> f64 {
    let l = crate::ringlone::lone();
    m::min(len16(s), l.max) * l.char
}

/// The pill of wedge `w` on a wheel of `n` as drawn: its outline, the digit at the end by the middle, its words. (ringlone.ts lonePill)
pub fn lone_pill(g: &mut crate::game::Game, w: &crate::ringlone::Wedge, n: f64) -> crate::ringlone::LonePill {
    pill_of(w, n)
}

/// lonePill's reckoning.
fn pill_of(w: &crate::ringlone::Wedge, n: f64) -> crate::ringlone::LonePill {
    let l = crate::ringlone::lone();
    let e = l.h / 2.0;
    let rows = lines_of(w);
    let tall = rows.len() > 1;
    let widths: Vec<f64> = rows.iter().map(|s| words_w(s)).collect();
    let text_w = m::max_n(&widths);
    let crate::ringlone::BoxOf { x0, y0, x1, y1, py, left, .. } = box_of(w.key, n, text_w, Some(tall));
    let (x0e, x1e, y0s, y1s) = (fixed1(x0 + e), fixed1(x1 - e), fixed1(y0), fixed1(y1));
    let d = if tall {
        format!(
            "M{x0e},{y0s} H{x1e} A{e},{e} 0 0 1 {},{} V{} A{e},{e} 0 0 1 {x1e},{y1s} H{x0e} A{e},{e} 0 0 1 {},{} V{} A{e},{e} 0 0 1 {x0e},{y0s} Z",
            fixed1(x1), fixed1(y0 + e), fixed1(y1 - e), fixed1(x0), fixed1(y1 - e), fixed1(y0 + e),
        )
    } else {
        format!("M{x0e},{y0s} H{x1e} A{e},{e} 0 0 1 {x1e},{y1s} H{x0e} A{e},{e} 0 0 1 {x0e},{y0s} Z")
    };
    let cy = if tall { (y0 + y1) / 2.0 } else { py };
    let count = rows.len() as f64;
    let lines: Vec<crate::ringlone::LonePillLinesItem> = rows
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let y = cy + 3.5 + (i as f64 - (count - 1.0) / 2.0) * l.lead;
            let w = words_w(&s);
            crate::ringlone::LonePillLinesItem { s, y, w }
        })
        .collect();
    let ty = lines[0].y;
    crate::ringlone::LonePill {
        x0,
        y0,
        x1,
        y1,
        text_w,
        lines,
        d,
        nx: if left { x1 - e } else { x0 + e },
        ny: cy + 4.0,
        tx: if left { x1 - l.h } else { x0 + l.h },
        ty,
        anchor: if left { crate::ringlone::LonePillAnchor::End } else { crate::ringlone::LonePillAnchor::Start },
    }
}

/// `v.toFixed(1)`: Rust's `{:.1}` breaks an exact tie to even where the language breaks it upward, which differs only at a fraction of exactly .25,
/// and it gives -0 a sign where the language does not. A pill's numbers are far below 1e21, where toFixed changes form.
fn fixed1(v: f64) -> String {
    let a = m::abs(v);
    let tie = a - m::floor(a) == 0.25;
    let body = if tie { format!("{}.3", m::floor(a)) } else { format!("{:.1}", a) };
    if v < 0.0 { format!("-{body}") } else { body }
}

/// The place the game reads for wedge `key`'s pill: the longest pill's box, two lines tall. (ringlone.ts pillBox)
pub fn pill_box(g: &mut crate::game::Game, key: f64, n: f64) -> crate::ringlone::BoxOf {
    let l = crate::ringlone::lone();
    box_of(key, n, l.max * l.char, Some(true))
}

/// Is a point of the ring's own px (from its middle) on the place of wedge `key`'s pill. (ringlone.ts onPill)
pub fn on_pill(g: &mut crate::game::Game, key: f64, n: f64, x: f64, y: f64) -> bool {
    let p = pill_box(g, key, n);
    x >= p.x0 && x <= p.x1 && y >= p.y0 && y <= p.y1
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The TypeScript's own bits under EXACT.on: key, n, tall, x0, y0, x1, y1, px, py, left (pillBox with HANDRING.loneWrap set to `tall`).
    const BOXES: &[(f64, f64, bool, [u64; 6], bool)] = &[
        (1.0, 5.0, true, [0xc054800000000000, 0xc04c800000000000, 0x4054800000000000, 0xc037000000000000, 0x3ce2c0889645a1c0, 0xc041000000000000], false),
        (1.0, 5.0, false, [0xc054800000000000, 0xc046800000000000, 0x4054800000000000, 0xc037000000000000, 0x3ce2c0889645a1c0, 0xc041000000000000], false),
        (2.0, 5.0, true, [0x403555fef4789a50, 0xc040c0d78aa8b9f9, 0x40672abfde8f134a, 0x3fdf943aaba30380, 0x40402aff7a3c4d28, 0xc025035e2aa2e7e4], false),
        (3.0, 5.0, true, [0x4021f82a6979600a, 0x403081af155173f2, 0x40659f82a6979601, 0x404940d78aa8b9f9, 0x4033fc1534bcb005, 0x403b81af155173f2], false),
        (3.0, 5.0, false, [0x4021f82a6979600a, 0x403081af155173f2, 0x40659f82a6979601, 0x404340d78aa8b9f9, 0x4033fc1534bcb005, 0x403b81af155173f2], false),
        (4.0, 5.0, true, [0xc0659f82a6979600, 0x403081af155173f2, 0xc021f82a69796000, 0x404940d78aa8b9f9, 0xc033fc1534bcb003, 0x403b81af155173f2], true),
        (5.0, 5.0, true, [0xc0672abfde8f134a, 0xc040c0d78aa8b9fa, 0xc03555fef4789a50, 0x3fdf943aaba302c0, 0xc0402aff7a3c4d27, 0xc025035e2aa2e7ea], true),
        (2.0, 3.0, true, [0x403271e296ddd176, 0x4017fffffffffffc, 0x4066ce3c52dbba2f, 0x4044000000000000, 0x403d71e296ddd176, 0x4030ffffffffffff], false),
        (3.0, 3.0, false, [0xc066ce3c52dbba2f, 0x4017fffffffffffc, 0xc03271e296ddd178, 0x403bffffffffffff, 0xc03d71e296ddd176, 0x4030ffffffffffff], true),
        (3.0, 8.0, true, [0x4037000000000000, 0xc026000000000000, 0x4067600000000000, 0x4037000000000000, 0x4041000000000000, 0x0000000000000000], false),
        (7.0, 8.0, true, [0xc067600000000000, 0xc025fffffffffffe, 0xc037000000000000, 0x4037000000000001, 0xc041000000000000, 0x3cf2c0889645a1c0], true),
    ];

    #[test]
    fn a_pill_s_place_has_the_typescript_s_bits() {
        let l = crate::ringlone::lone();
        for &(key, n, tall, bits, left) in BOXES {
            let p = box_of(key, n, l.max * l.char, Some(tall));
            assert_eq!(
                [p.x0.to_bits(), p.y0.to_bits(), p.x1.to_bits(), p.y1.to_bits(), p.px.to_bits(), p.py.to_bits()],
                bits,
                "wedge {key} of {n}, tall {tall}"
            );
            assert_eq!(p.left, left);
        }
    }

    #[test]
    fn a_wedge_s_way_has_the_typescript_s_bits() {
        for (key, n, bits) in [
            (1.0, 5.0, 0xbff921fb54442d18u64),
            (2.0, 5.0, 0xbfd41b2f769cf0e0),
            (3.0, 5.0, 0x3fee28c731eb6950),
            (4.0, 5.0, 0x400197c987c952c4),
            (5.0, 5.0, 0x400ba5614317cb35),
            (2.0, 3.0, 0x3fe0c152382d7365),
            (3.0, 8.0, 0x0000000000000000),
            (8.0, 8.0, 0x400f6a7a2955385e),
        ] {
            assert_eq!(wedge_mid(key, n).to_bits(), bits, "wedge {key} of {n}");
        }
    }

    #[test]
    fn the_one_digit_is_read_as_the_typescript_reads_it() {
        // the values printed by loneKey under bun, and -1 for a fraction (31 - clz32(0) = -1)
        for (k, want) in [
            (0.0, 0.0), (1.0, 0.0), (2.0, 1.0), (3.0, 0.0), (4.0, 2.0), (8.0, 3.0), (16.0, 4.0), (31.0, 0.0),
            (-4.0, 0.0), (2147483648.0, 31.0), (3.5, 0.0), (0.5, -1.0),
        ] {
            assert_eq!(lone_digit(k), want, "bits {k}");
        }
    }

    fn wedge(key: f64, text: &str, opens: bool) -> Wedge {
        Wedge { key, text: String::from(text), opens: Some(opens) }
    }

    #[test]
    fn a_label_breaks_where_the_typescript_breaks_it() {
        // loneLines under bun with HANDRING.loneWrap on
        let cases: &[(&str, bool, &[&str])] = &[
            ("breach with a wall charge · 3 left", false, &["breach with a wall", "charge · 3 left"]),
            ("breach with a wall charge · 3 left", true, &["breach with a wall", "charge · 3 left ▸"]),
            ("whistle", true, &["whistle ▸"]),
            ("exactly twenty lette", false, &["exactly twenty lette"]),
            ("exactly twenty lette", true, &["exactly", "twenty lette ▸"]),
            ("exactly twenty letter", false, &["exactly", "twenty letter"]),
            ("exactly twenty letter", true, &["exactly twenty", "letter ▸"]),
            ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false, &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]),
            ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", true, &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa ▸"]),
            ("short · then something a good deal longer than max", false, &["short · then something a", "good deal longer than max"]),
            ("x · y · z and more words to go past", true, &["x · y · z and more", "words to go past ▸"]),
            ("  leading and trailing spaces here now  ", false, &["leading and trailing", "spaces here now"]),
            ("  leading and trailing spaces here now  ", true, &["leading and trailing", "spaces here now   ▸"]),
            ("· starts with a dot and runs on and on", false, &["· starts with a dot", "and runs on and on"]),
            ("ends with a dot and runs on and on ·", true, &["ends with a dot and", "runs on and on · ▸"]),
            ("one · two", true, &["one · two ▸"]),
            ("thirty letters then ▸ more words", true, &["thirty letters", "then ▸ more words ▸"]),
        ];
        for &(text, opens, want) in cases {
            assert_eq!(lines_of(&wedge(1.0, text, opens)), want, "{text}");
        }
    }

    #[test]
    fn to_fixed_rounds_as_the_language_does() {
        // v.toFixed(1) under bun
        for (bits, want) in [
            (0x3fd0000000000000u64, "0.3"), (0xbfd0000000000000, "-0.3"), (0x4002000000000000, "2.3"), (0x3fe8000000000000, "0.8"),
            (0x3fa999999999999a, "0.1"), (0x3fc3333333333333, "0.1"), (0xbfa47ae147ae147b, "-0.0"), (0x8000000000000000, "0.0"),
            (0x0000000000000000, "0.0"), (0x3ff7333333333333, "1.4"), (0x4028b33333333333, "12.3"), (0x405ed00000000000, "123.3"),
            (0xc05ef00000000000, "-123.8"), (0x3fee666666666666, "0.9"), (0x3fa47ae147ae147b, "0.0"),
        ] {
            assert_eq!(fixed1(f64::from_bits(bits)), want, "{bits:#x}");
        }
    }

    /// lonePill under bun with EXACT.on: text, key, n, outline, anchor is end, lines' y bits, lines' w bits, and nx ny tx ty x0 y0 x1 y1 textW bits.
    const PILLS: &[(&str, f64, f64, &str, bool, &[u64], &[u64], [u64; 9])] = &[
        ("whistle", 1.0, 5.0, "M-28.1,-45.0 H28.1 A11,11 0 0 1 28.1,-23.0 H-28.1 A11,11 0 0 1 -28.1,-45.0 Z", false,
         &[0xc03e800000000000], &[0x4047199999999999],
         [0xc03c199999999998, 0xc03e000000000000, 0xc031199999999998, 0xc03e800000000000, 0xc0438ccccccccccc, 0xc046800000000000, 0x40438ccccccccccc, 0xc037000000000000, 0x4047199999999999]),
        ("whistle", 2.0, 5.0, "M32.3,-21.5 H88.5 A11,11 0 0 1 88.5,0.5 H32.3 A11,11 0 0 1 32.3,-21.5 Z", false,
         &[0xc01c06bc5545cfc8], &[0x4047199999999999],
         [0x40402aff7a3c4d28, 0xc01a06bc5545cfc8, 0x4045aaff7a3c4d28, 0xc01c06bc5545cfc8, 0x403555fef4789a50, 0xc03581af155173f2, 0x4058e24c89eaf360, 0x3fdf943aaba30380, 0x4047199999999999]),
        ("breach with a wall charge · 3 left", 1.0, 5.0, "M-64.4,-57.0 H64.4 A11,11 0 0 1 75.4,-46.0 V-34.0 A11,11 0 0 1 64.4,-23.0 H-64.4 A11,11 0 0 1 -75.4,-34.0 V-46.0 A11,11 0 0 1 -64.4,-57.0 Z", false,
         &[0xc045400000000000, 0xc03e800000000000], &[0x405db33333333333, 0x4058c00000000000],
         [0xc05019999999999a, 0xc042000000000000, 0xc04ab33333333334, 0xc045400000000000, 0xc052d9999999999a, 0xc04c800000000000, 0x4052d9999999999a, 0xc037000000000000, 0x405db33333333333]),
        ("breach with a wall charge · 3 left", 4.0, 5.0, "M-148.8,16.5 H-20.0 A11,11 0 0 1 -9.0,27.5 V39.5 A11,11 0 0 1 -20.0,50.5 H-148.8 A11,11 0 0 1 -159.8,39.5 V27.5 A11,11 0 0 1 -148.8,16.5 Z", true,
         &[0x403f01af155173f2, 0x404580d78aa8b9f9], &[0x405db33333333333, 0x4058c00000000000],
         [0xc033fc1534bcb000, 0x4042c0d78aa8b9f9, 0xc03efc1534bcb000, 0x403f01af155173f2, 0xc063f91c40312f9a, 0x403081af155173f2, 0xc021f82a69796000, 0x404940d78aa8b9f9, 0x405db33333333333]),
        ("go", 3.0, 3.0, "M-52.6,6.0 H-29.4 A11,11 0 0 1 -29.4,28.0 H-52.6 A11,11 0 0 1 -52.6,6.0 Z", true,
         &[0x40347fffffffffff], &[0x402a666666666666],
         [0xc03d71e296ddd176, 0x4034ffffffffffff, 0xc04438f14b6ee8bb, 0x40347fffffffffff, 0xc04fd28ae5088255, 0x4017fffffffffffc, 0xc03271e296ddd176, 0x403bffffffffffff, 0x402a666666666666]),
        ("open the door and go in with everything", 7.0, 8.0, "M-176.0,-11.0 H-34.0 A11,11 0 0 1 -23.0,0.0 V12.0 A11,11 0 0 1 -34.0,23.0 H-176.0 A11,11 0 0 1 -187.0,12.0 V0.0 A11,11 0 0 1 -176.0,-11.0 Z", true,
         &[0x400c000000000008, 0x402f000000000002], &[0x4060800000000000, 0x405db33333333333],
         [0xc041000000000000, 0x4024000000000002, 0xc046800000000000, 0x400c000000000008, 0xc067600000000000, 0xc025fffffffffffe, 0xc037000000000000, 0x4037000000000001, 0x4060800000000000]),
    ];

    #[test]
    fn a_pill_as_drawn_has_the_typescript_s_bits() {
        for &(text, key, n, d, end, ys, ws, bits) in PILLS {
            let p = pill_of(&wedge(key, text, false), n);
            assert_eq!(p.d, d, "{text} {key}");
            assert_eq!(p.anchor == LonePillAnchor::End, end, "{text} {key}");
            assert_eq!(p.lines.iter().map(|l| l.y.to_bits()).collect::<Vec<_>>(), ys, "{text} {key}");
            assert_eq!(p.lines.iter().map(|l| l.w.to_bits()).collect::<Vec<_>>(), ws, "{text} {key}");
            assert_eq!(
                [p.nx, p.ny, p.tx, p.ty, p.x0, p.y0, p.x1, p.y1, p.text_w].map(f64::to_bits),
                bits,
                "{text} {key}"
            );
        }
    }
}
