//! WHAT MEN CARRY, AS A LIST OF VALUES. Nothing of the game is written to make it: a step lays the list into the frame's rows, and a viewer's own picture draws one as it is.
use crate::game::Game;
use crate::ids::*;
use crate::scene_boxes::{box_value, make_box_value, Box, MakeBoxP};
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

#[derive(Default)]
pub struct Carried {
    pub boxes: Vec<CarriedBox>,
    /// whose boxes are being made: the word on a man's parts
    pub owner: f64,
    /// the axes of the part before, its heading and its turn: the parts of one thing lie along the same three
    turned: Option<([V3; 3], f64, Quat)>,
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

/// A part of what a man carries: where, how big, along which axes (z its length), and what it looks like.
pub struct Part { pub owner: f64, pub c: V3, pub h: V3, pub axes: [V3; 3], pub albedo: V3, pub emissive: V3, pub flags: f64, pub stuff: Option<crate::scene_stuff::BoxStuff> }

impl Carried {
    /// room for what a squad and a building's men carry
    pub fn with_room() -> Self { Carried { boxes: Vec::with_capacity(256), ..Carried::default() } }

    /// A part, its heading and its turn worked out once for the parts that lie along the same axes.
    pub fn part(&mut self, p: Part) {
        let bits = |a: &[V3; 3]| a.map(|v| v.map(f64::to_bits));
        let (yaw, rot) = match &self.turned {
            Some((a, yaw, rot)) if bits(a) == bits(&p.axes) => (*yaw, *rot),
            _ => {
                let [x, y, z] = p.axes;
                let (yaw, rot) = (crate::m::atan2(z[0], z[2]), v3::quat::from_basis(x, y, z));
                self.turned = Some((p.axes, yaw, rot));
                (yaw, rot)
            }
        };
        let b = Box { stuff: p.stuff, ..box_value(p.c, p.h, yaw, p.albedo, p.emissive, p.flags, p.owner, Some(rot)) };
        self.boxes.push(CarriedBox { b, ..CarriedBox::default() });
    }
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
        let notes: Vec<_> = self.boxes.iter().enumerate().filter(|(_, x)| x.thin.is_some() || x.ir || x.air.is_some()).map(|(i, x)| (i, x.thin, x.ir, x.air)).collect();
        for x in self.boxes { out.push(crate::scene_boxes::box_frame(g, x.b).into()); }
        for (i, thin, ir, air) in notes {
            let id = out[from + i];
            if let Some(t) = thin { crate::thinline::thin_note(g, id, t.hw0, t.k); }
            if ir { crate::irlaser::ir_note(g, id); }
            if let Some(bead) = air { crate::thinline::thin_air(g, id, if bead { Some(out[from + i + 1]) } else { None }); }
        }
    }
}
