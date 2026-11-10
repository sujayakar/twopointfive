//! WHAT MEN CARRY, AS A LIST OF VALUES. Nothing of the game is written to make it: a step lays the list into the frame's rows, and a viewer's own picture draws one as it is.
use crate::game::Game;
use crate::ids::*;
use crate::scene_boxes::{make_box_value, Box, MakeBoxP};
use alloc::vec::Vec;

#[derive(Default)]
pub struct Carried {
    pub boxes: Vec<CarriedBox>,
    /// whose boxes are being made: the word on a man's parts
    pub owner: f64,
}

#[derive(Default)]
pub struct CarriedBox {
    pub b: Box,
    /// a thin line, and how thin
    pub thin: Option<crate::thinline::Thin>,
    /// a line of infrared, which only a tube's picture shows
    pub ir: bool,
    /// a beam that a picture may draw as light in the air; true: the box after it is the bead where it lands
    pub air: Option<bool>,
}

impl Carried {
    pub fn put(&mut self, p: &MakeBoxP) { self.boxes.push(CarriedBox { b: make_box_value(p), ..CarriedBox::default() }); }

    /// Motion identity: every box gets its man's id and its ordinal among his boxes. Owner 0 stays on the index rule.
    pub fn keyed(&mut self) {
        let mut ord = [0u32; 256];
        for x in self.boxes.iter_mut() {
            let o = crate::js::to_i32(x.b.owner) as usize & 0xff;
            if o == 0 { continue; }
            x.b.mkey = Some(crate::scene_boxes::mkey().rig + ((o << 8) as f64) + ord[o].min(255) as f64);
            ord[o] += 1;
        }
    }

    /// into the frame's rows, and what is noted beside a row
    pub fn laid(self, g: &mut Game, out: &mut Vec<AnyBoxId>) {
        let from = out.len();
        let notes: Vec<_> = self.boxes.iter().map(|x| (x.thin, x.ir, x.air)).collect();
        for x in self.boxes { out.push(crate::scene_boxes::box_frame(g, x.b).into()); }
        for (i, (thin, ir, air)) in notes.into_iter().enumerate() {
            let id = out[from + i];
            if let Some(t) = thin { crate::thinline::thin_note(g, id, t.hw0, t.k); }
            if ir { crate::irlaser::ir_note(g, id); }
            if let Some(bead) = air { crate::thinline::thin_air(g, id, if bead { Some(out[from + i + 1]) } else { None }); }
        }
    }
}
