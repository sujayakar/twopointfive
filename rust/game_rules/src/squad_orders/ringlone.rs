pub use ::game_types::ringlone::types;
pub use types::*;
use crate::js;
use crate::m;

/// The way wedge `key` of `n` points, in radians; 1 is up, clockwise.
fn wedge_mid(key: f64, n: f64) -> f64 {
    (-90.0 + (key - 1.0) * 360.0 / n) * m::PI / 180.0
}

/// The digit of a ring with one row, from the bits of the digits that have a wedge; 0 = the wheel, or no ring.
pub fn lone_key(g: &mut crate::game::Game, keys: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> f64 {
    let k = keys(g);
    lone_digit(k)
}

/// loneKey's arithmetic on the bits. A signed result: a `k` above 0 whose int32 is 0 (0.5) gives -1 there.
fn lone_digit(k: f64) -> f64 {
    if k > 0.0 && js::to_i32(k) & js::to_i32(k - 1.0) == 0 { (31 - m::clz32(js::to_u32(k)) as i32) as f64 } else { 0.0 }
}

/// The box of wedge `key`'s pill on a wheel of `n`, its words `text_w` px wide: over the middle for a wedge pointing up or down, else out to its side.
fn box_of(key: f64, n: f64, text_w: f64, tall: Option<bool>) -> crate::ringlone::BoxOf {
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

/// The place the game reads for wedge `key`'s pill: the longest pill's box, two lines tall.
fn pill_box(key: f64, n: f64) -> crate::ringlone::BoxOf {
    let l = crate::ringlone::lone();
    box_of(key, n, l.max * l.char, Some(true))
}

/// Is a point of the ring's own px (from its middle) on the place of wedge `key`'s pill.
pub(super) fn on_pill(key: f64, n: f64, x: f64, y: f64) -> bool {
    let p = pill_box(key, n);
    x >= p.x0 && x <= p.x1 && y >= p.y0 && y <= p.y1
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pill of a ring with one row. A failure means it is drawn over the middle of the ring, or the cursor misses what it sees.
    #[test]
    fn a_pill_stands_out_along_its_wedge_and_grows_away_from_the_middle() {
        let l = crate::ringlone::lone();
        let text = l.max * l.char;
        let (wd, e) = (l.h + text + l.pad, l.h / 2.0);
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-9, "{a} {b}");

        // at the top, wedge 1: over the middle, its second line above it
        let up = box_of(1.0, 8.0, text, Some(true));
        near(up.px, 0.0);
        near(up.py, -l.r);
        assert!(!up.left);
        near(up.x0, -wd / 2.0);
        near(up.x1, wd / 2.0);
        near(up.y0, -l.r - e - l.lead);
        near(up.y1, -l.r + e);
        near(box_of(1.0, 8.0, text, Some(false)).y0, -l.r - e);    // one line is shorter

        // at the bottom: its second line below it
        let down = box_of(5.0, 8.0, text, Some(true));
        near(down.y0, l.r - e);
        near(down.y1, l.r + e + l.lead);

        // out to the right, clockwise from the top
        let right = box_of(3.0, 8.0, text, Some(false));
        near(right.px, l.r);
        assert!(!right.left);
        near(right.x0, l.r - e);
        near(right.x1, l.r - e + wd);

        // out to the left, its words running in from the far end
        let left = box_of(7.0, 8.0, text, Some(false));
        near(left.px, -l.r);
        assert!(left.left);
        near(left.x1, -l.r + e);
        near(left.x0, -l.r + e - wd);
    }

    /// A ring with one row knows the row's digit from the one bit up; more than one bit, or none, is the wheel.
    #[test]
    fn a_ring_with_one_row_finds_its_digit_in_the_one_bit() {
        for (k, want) in [(1.0, 0.0), (2.0, 1.0), (4.0, 2.0), (16.0, 4.0), (0.0, 0.0), (3.0, 0.0), (31.0, 0.0)] {
            assert_eq!(lone_digit(k), want, "bits {k}");
        }
    }
}
