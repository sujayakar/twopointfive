//! src/scene/shapes.ts lines 1 to 211
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// one number into the FNV-1a word, to the millimetre. (shapes.ts shapeStamp `num`)
fn num(h: &mut u32, v: f64) {
    *h ^= js::to_i32(m::round(v * 1000.0)) as u32;
    *h = m::imul(*h as i32, 0x01000193) as u32;
}
/// a form's geometry; a BoxShape and a ShapePiece each carry a form's fields, so they are handed in. (shapes.ts shapeStamp `form`)
fn form(h: &mut u32, round: Option<f64>, arc: Option<ShapeFormArc>, cut: Option<&[[f64; 4]]>) {
    num(h, round.unwrap_or(0.0));
    num(h, match arc { Some(a) => a.axis, None => 0.0 });
    num(h, match arc { Some(a) => a.rise, None => 0.0 });
    num(h, match cut { Some(c) => c.len() as f64, None => 0.0 });
    for q in cut.unwrap_or(&[]) { for &v in q { num(h, v); } }
}

/// A number of the shapes a level stands on, over its first `n` boxes: 0 = no box stands shaped. (shapes.ts shapeStamp)
pub fn shape_stamp(g: &mut crate::game::Game, boxes: &[BoxId], n: Option<f64>) -> f64 {
    let n = n.unwrap_or(boxes.len() as f64);
    let mut h: u32 = 0x811c9dc5;
    let mut any = false;
    let mut i = 0;
    while (i as f64) < n && i < boxes.len() {
        let b = &g.col.boxes[boxes[i]];
        if b.shaped == Some(true) {
            let s = b.shape.as_ref().unwrap();
            any = true;
            num(&mut h, i as f64 / 1000.0);
            form(&mut h, s.round, s.arc, s.cut.as_deref());
            num(&mut h, match &s.with { Some(w) => w.len() as f64, None => 0.0 });
            for w in s.with.as_deref().unwrap_or(&[]) {
                for &v in &w.off { num(&mut h, v); }
                for &v in &w.h { num(&mut h, v); }
                num(&mut h, w.yaw.unwrap_or(0.0));
                form(&mut h, w.round, w.arc, w.cut.as_deref());
            }
        }
        i += 1;
    }
    if any { if h != 0 { h as f64 } else { 1.0 } } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_boxes::Box as B;

    fn shaped(s: BoxShape) -> B { let mut b = B::zeroed(); b.shaped = Some(true); b.shape = Some(s); b }
    fn piece(off: V3, h: V3) -> ShapePiece { let mut w = ShapePiece::zeroed(); w.off = off; w.h = h; w }

    /// the list of the TypeScript run: [plain, round, says a shape but does not stand it, arc with cuts, pieces]
    fn level(g: &mut Game) -> [BoxId; 5] {
        let plain = g.col.boxes.push(B::zeroed());
        let round = g.col.boxes.push(shaped(BoxShape { round: Some(3.0), ..BoxShape::zeroed() }));
        let mut says = B::zeroed();
        says.shape = Some(BoxShape { round: Some(1.0), ..BoxShape::zeroed() });
        let says = g.col.boxes.push(says);
        let arc = g.col.boxes.push(shaped(BoxShape {
            arc: Some(ShapeFormArc { axis: 1.0, rise: 0.135 }),
            cut: Some(vec![[0.5, 0.0, -0.25, 1.2345], [-1.0, 2.5, 0.0, -0.0005]]),
            old: Some(true),
            ..BoxShape::zeroed()
        }));
        let mut w0 = piece([0.1, -0.2, 3000000.5], [0.25, 0.5, 0.75]);
        w0.yaw = Some(-1.5707963267948966);
        w0.round = Some(1.0);
        w0.albedo = Some([1.0, 0.0, 0.0]);
        let mut w1 = piece([-2147483.649, 4294967.296, 0.0], [1e-4, 1.0, 2.0]);
        w1.arc = Some(ShapeFormArc { axis: 3.0, rise: 0.4 });
        w1.cut = Some(vec![[1.0, 1.0, 1.0, 9.0005]]);
        let with = g.col.boxes.push(shaped(BoxShape { with: Some(vec![w0, w1]), ..BoxShape::zeroed() }));
        [plain, round, says, arc, with]
    }

    /// the bits bun gave for shapes.ts on the same boxes
    #[test]
    fn stamps_are_the_typescripts_bits() {
        let mut g = Game::blank();
        let l = level(&mut g);
        let [plain, _, says, arc, with] = l;
        assert_eq!(shape_stamp(&mut g, &[plain, says], None).to_bits(), 0);
        assert_eq!(shape_stamp(&mut g, &l, None).to_bits(), 0x41e12d58ad800000);
        assert_eq!(shape_stamp(&mut g, &l, Some(2.0)).to_bits(), 0x41a3170738000000);
        assert_eq!(shape_stamp(&mut g, &l, Some(1.0)).to_bits(), 0);
        assert_eq!(shape_stamp(&mut g, &l, Some(99.0)).to_bits(), 0x41e12d58ad800000);
        assert_eq!(shape_stamp(&mut g, &l, Some(3.5)).to_bits(), 0x41e16416abe00000);
        assert_eq!(shape_stamp(&mut g, &[arc], None).to_bits(), 0x41d083df0e400000);
        assert_eq!(shape_stamp(&mut g, &[plain, plain, with], None).to_bits(), 0x41dc34078a000000);
    }
}
