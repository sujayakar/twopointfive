//! WHAT LIES AND WHAT PRESSES ON A LEVEL BUILT FOR SNOW: what snow each static is, the men's feet and heads as the tick left them, the things that move over the ground, and the blasts that
//! went off. The snow itself (the field it lies in, every print, every mark, every flake and breath) is the picture's own, made from these rows. Empty on every other level.
use crate::Row;
use alloc::vec::Vec;
use game::character::Bone;
use game::fogofwar::fog_of_war_hidden as hidden;
use game::game::Game;
use game::ids::*;
use game::m;
use game::out_events::OutEvent;
use game::scene_materials::SnowKind;

/// a `says` word's bits for a man: alive, the fog hides him, dead (a body), and which bones he has
pub const ALIVE: u32 = 1;
pub const HIDDEN: u32 = 2;
pub const DEAD: u32 = 4;
pub const BONES: u32 = 8;
const LEFT_TOE: u32 = 16;
const RIGHT_TOE: u32 = 32;

section! {
    /// a man, as what stands on the snow and breathes over it
    pub struct Foot {
        pub id: u32,
        pub says: u32,
        pub pos: [f32; 3],
        pub vel: [f32; 3],
        pub yaw: f32,
        pub hips: [f32; 3],
        pub head: [f32; 3],
        pub chest: [f32; 3],
        pub foot_l: [f32; 3],
        pub foot_r: [f32; 3],
        pub toe_l: [f32; 3],
        pub toe_r: [f32; 3],
        /// where his head faces
        pub facing: [f32; 3],
    }
}
section! {
    /// a part of a thing that moves over the ground: each of a prop's parts, a thrown thing, a chunk of debris
    pub struct Thing {
        /// 0 a prop's part · 1 a thrown thing · 2 a chunk
        pub kind: u32,
        pub key: u32,
        pub part: u32,
        /// 1 at rest · 2 lying on something that is not the ground
        pub says: u32,
        pub c: [f32; 3],
        pub h: [f32; 3],
        pub yaw: f32,
        /// where the whole thing stands, its velocity and its turn (a prop's)
        pub at: [f32; 2],
        pub vel: [f32; 3],
        pub spin: f32,
        /// the whole thing's radius and weight, and how far over the floor this part's underside hangs
        pub radius: f32,
        pub mass: f32,
        pub foot: f32,
    }
}
event! {
    /// a blast went off
    pub struct SnowBlast {
        pub at: [f32; 3],
        pub radius: f32,
    }
}
section! {
    /// what snow a static is, by its place: 0 none · 1 snow · 2 trodden · 3 old
    pub struct SnowWord {
        pub w: u32,
    }
}

#[derive(Default)]
pub struct Snow {
    pub feet: Vec<Foot>,
    pub things: Vec<Thing>,
    pub blasts: Vec<SnowBlast>,
    pub words: Vec<SnowWord>,
    of: (usize, f64),
}

impl Snow {
    pub fn take(&mut self, g: &Game) {
        (self.feet.clear(), self.things.clear(), self.blasts.clear());
        if !game::weather_kinds::snow_level(g, game::scene_level_load::level_name_of(g).as_deref()) { self.words.clear(); return; }
        let tick = crate::tick(g);
        for e in &g.out { if let OutEvent::SnowBlast(b) = e { let serial = self.blasts.len() as u32; self.blasts.push(SnowBlast { tick, serial, at: b.at.map(|x| x as f32), radius: b.radius as f32 }); } }
        // (what snow a static is moves only with the list)
        let list = (g.engine.world.statics.len(), g.engine.world.geom_serial);
        if self.of != list { self.of = list; self.words.clear();
        for &id in &g.engine.world.statics {
            let w = match game::scene_materials::does_of(g, g.tab.boxes[id].does).snow { Some(SnowKind::Snow) => 1, Some(SnowKind::SnowPacked) => 2, Some(SnowKind::SnowOld) => 3, None => 0 };
            self.words.push(SnowWord { w });
        } }
        for &s in &g.seats { let c = g.pl[game::cmd::seat_man(g, s)].ch; self.man(g, c, false, false); }
        for &gd in g.guards.iter().chain(game::civilians::civilians_shown(g).iter()) { self.man(g, g.gd[gd].ch, hidden(g, gd), game::condition::down(g, g.gd[gd].ch)); }
        self.props(g);
        self.items(g);
        self.chunks(g);
    }

    fn man(&mut self, g: &Game, id: CharId, hid: bool, dead: bool) {
        let c = &g.ch[id];
        let f3 = |v: [f64; 3]| v.map(|x| x as f32);
        let b = |k: Bone| c.bone(k).map(f3);
        let Some(hips) = b(Bone::Hips) else { return };
        let mut r = Foot::ZERO;
        r.id = c.id as u32;
        r.says = c.alive as u32 * ALIVE | hid as u32 * HIDDEN | dead as u32 * DEAD | BONES;
        (r.pos, r.vel, r.yaw, r.hips) = (f3(c.pos), f3(c.vel), c.body_yaw as f32, hips);
        r.chest = b(Bone::Chest).unwrap_or(hips);
        r.head = b(Bone::Head).unwrap_or(r.chest);
        r.foot_l = b(Bone::FootL).unwrap_or(hips);
        r.foot_r = b(Bone::FootR).unwrap_or(hips);
        match b(Bone::ToeL) { Some(t) => { r.toe_l = t; r.says |= LEFT_TOE; } None => r.toe_l = r.foot_l }
        match b(Bone::ToeR) { Some(t) => { r.toe_r = t; r.says |= RIGHT_TOE; } None => r.toe_r = r.foot_r }
        r.facing = f3(game::character::character_head_dir(g, id));
        self.feet.push(r);
    }

    fn props(&mut self, g: &Game) {
        for &id in &g.props.props {
            let p = &g.tab.props_prop[id];
            let (vx, vz, spin, rest) = (p.vx, p.vz, p.w, p.asleep);
            for (i, part) in p.def.parts.iter().enumerate() {
                let Some(&bx) = p.boxes.get(i) else { continue };
                let b = &g.tab.boxes[bx];
                self.things.push(Thing { kind: 0, key: id.0 as u32, part: i as u32, says: rest as u32, at: [p.x as f32, p.z as f32], c: b.c.map(|x| x as f32), h: part.h.map(|x| x as f32), yaw: b.yaw as f32, vel: [vx as f32, 0.0, vz as f32], spin: spin as f32, radius: p.radius as f32, mass: p.mass as f32, foot: (part.off[1] - part.h[1]) as f32 });
            }
        }
    }

    fn items(&mut self, g: &Game) {
        for &id in &g.items.items {
            let it = &g.tab.item[id];
            let mut t = Thing::ZERO;
            (t.kind, t.key) = (1, id.ix() as u32);
            t.says = it.settled as u32 | ((it.on != 0) as u32) << 1;
            t.c = [it.pos[0] as f32, it.pos[1] as f32, it.pos[2] as f32];
            t.at = [t.c[0], t.c[2]];
            t.vel = [it.vel[0] as f32, it.vel[1] as f32, it.vel[2] as f32];
            t.radius = it.radius as f32;
            self.things.push(t);
        }
    }

    fn chunks(&mut self, g: &Game) {
        for &id in &g.destr.debris.list {
            let (c, b) = (&g.tab.debris_chunk[id], &g.tab[g.tab.debris_chunk[id].box_]);
            let mut t = Thing::ZERO;
            (t.kind, t.key) = (2, id.ix() as u32);
            t.says = c.rest as u32 | (c.on.is_some() as u32) << 1;
            t.c = b.c.map(|x| x as f32);
            t.at = [t.c[0], t.c[2]];
            t.h = b.h.map(|x| x as f32);
            t.vel = c.v.map(|x| x as f32);
            t.radius = m::max(b.h[0], m::max(b.h[1], b.h[2])) as f32;
            self.things.push(t);
        }
    }
}
