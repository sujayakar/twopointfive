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

/// The digit of a ring with one row, from the bits of the digits that have a wedge; 0 = the wheel, or no ring. (ringlone.ts loneKey)
pub fn lone_key(g: &mut crate::game::Game, keys: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> f64 {
    let k = keys(g);
    lone_digit(k)
}

/// loneKey's arithmetic on the bits. A signed result: a `k` above 0 whose int32 is 0 (0.5) gives -1 there.
fn lone_digit(k: f64) -> f64 {
    if k > 0.0 && js::to_i32(k) & js::to_i32(k - 1.0) == 0 { (31 - m::clz32(js::to_u32(k)) as i32) as f64 } else { 0.0 }
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

/// The place the game reads for wedge `key`'s pill: the longest pill's box, two lines tall. (ringlone.ts pillBox)
pub fn pill_box(g: &crate::game::Game, key: f64, n: f64) -> crate::ringlone::BoxOf {
    let l = crate::ringlone::lone();
    box_of(key, n, l.max * l.char, Some(true))
}

/// Is a point of the ring's own px (from its middle) on the place of wedge `key`'s pill. (ringlone.ts onPill)
pub fn on_pill(g: &crate::game::Game, key: f64, n: f64, x: f64, y: f64) -> bool {
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

    #[test]
    fn to_fixed_rounds_as_the_language_does() {
        // v.toFixed(1) under bun
        for (bits, want) in [
            (0x3fd0000000000000u64, "0.3"), (0xbfd0000000000000, "-0.3"), (0x4002000000000000, "2.3"), (0x3fe8000000000000, "0.8"),
            (0x3fa999999999999a, "0.1"), (0x3fc3333333333333, "0.1"), (0xbfa47ae147ae147b, "-0.0"), (0x8000000000000000, "0.0"),
            (0x0000000000000000, "0.0"), (0x3ff7333333333333, "1.4"), (0x4028b33333333333, "12.3"), (0x405ed00000000000, "123.3"),
            (0xc05ef00000000000, "-123.8"), (0x3fee666666666666, "0.9"), (0x3fa47ae147ae147b, "0.0"),
        ] {
            assert_eq!(js::to_fixed(f64::from_bits(bits), 1), want, "{bits:#x}");
        }
    }

}
