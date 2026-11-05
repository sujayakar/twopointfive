//! src/game/carlamps.ts lines 1 to 105
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One lamp box that fits a generator's lamp: its place in the list and in `sigs()`.
#[derive(Clone, Copy)]
struct Cand { i: usize, s: usize }

/// What each generator writes for a lamp. (carlamps.ts sigs)
pub fn sigs() -> Vec<crate::carlamps::LampSig> {
    let p = crate::scene_gwot_palette::gwot_palette();
    let (white, red) = (p.plastic_white, p.truck_red);
    vec![
        LampSig { what: LampSigWhat::TailLamp, h: [0.02, 0.06, 0.16], albedo: None, emissive: Some([0.5, 0.03, 0.02]), gap: 1.24 },   // the car
        LampSig { what: LampSigWhat::HeadLamp, h: [0.02, 0.07, 0.14], albedo: Some([0.8, 0.8, 0.75]), emissive: None, gap: 1.24 },
        LampSig { what: LampSigWhat::HeadLamp, h: [0.03, 0.07, 0.16], albedo: Some(white), emissive: None, gap: 1.32 },                // the pickup and the van
        LampSig { what: LampSigWhat::TailLamp, h: [0.03, 0.08, 0.1], albedo: Some(red), emissive: None, gap: 1.44 },                   // the pickup
        LampSig { what: LampSigWhat::TailLamp, h: [0.02, 0.15, 0.04], albedo: Some(red), emissive: None, gap: 1.72 },                  // the van
        LampSig { what: LampSigWhat::HeadLamp, h: [0.02, 0.08, 0.1], albedo: Some(white), emissive: None, gap: 1.4 },                  // the pass's lorry
    ]
}

/// Three channels within `eps`, each. (carlamps.ts near3)
pub fn near3(a: &[f64], b: &[f64], eps: f64) -> bool {
    m::abs(a[0] - b[0]) <= eps && m::abs(a[1] - b[1]) <= eps && m::abs(a[2] - b[2]) <= eps
}

/// The box glows at all. (carlamps.ts glows)
pub fn glows(g: &mut crate::game::Game, b: BoxId) -> bool {
    let e = g.col.boxes[b].emissive;
    e[0] > 0.0 || e[1] > 0.0 || e[2] > 0.0
}

/// The box is this generator's lamp: size, then glow or paint. (carlamps.ts fits)
pub fn fits(g: &mut crate::game::Game, b: BoxId, s: crate::carlamps::LampSig) -> bool {
    if g.col.boxes[b].rot.is_some() || !near3(&g.col.boxes[b].h, &s.h, g.k.carlamps.fit) { return false; }
    if let Some(emissive) = s.emissive { return near3(&g.col.boxes[b].emissive, &emissive, g.k.carlamps.tone); }
    !glows(g, b) && near3(&g.col.boxes[b].albedo, &s.albedo.unwrap(), g.k.carlamps.tone)
}

/// The lamp boxes of a level's vehicles in box order: each fits a generator's lamp and has its twin across the vehicle. (carlamps.ts findCarLamps)
pub fn find_car_lamps(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<crate::carlamps::FoundLamp> {
    let sig = sigs();
    let mut cand: Vec<Cand> = Vec::new();
    for i in 0..boxes.len() {
        let s = sig.iter().position(|x| fits(g, boxes[i], *x));
        if let Some(s) = s { cand.push(Cand { i, s }); }
    }
    let mut out: Vec<FoundLamp> = Vec::new();
    for a in cand.iter().copied() {
        let aa = boxes[a.i];
        let s = sig[a.s];
        let (ca, yaw_a) = (g.col.boxes[aa].c, g.col.boxes[aa].yaw);
        let twin_k = g.k.carlamps.twin;
        let twin = cand.iter().any(|b| {
            let other = &g.col.boxes[boxes[b.i]];
            b.i != a.i && b.s == a.s
                && m::abs(other.c[1] - ca[1]) <= twin_k
                && m::abs(other.yaw - yaw_a) <= 1e-3
                && m::abs(m::hyp2(other.c[0] - ca[0], other.c[2] - ca[2]) - s.gap) <= twin_k
        });
        if twin {
            let what = match s.what { LampSigWhat::HeadLamp => FoundLampWhat::HeadLamp, LampSigWhat::TailLamp => FoundLampWhat::TailLamp };
            out.push(FoundLamp { box_: a.i as f64, what, lit: s.emissive.is_some() });
        }
    }
    out
}

/// The light targets of a level's vehicle lamps: emissive, off the mains, no stagger, no die drawn. (carlamps.ts carLampTargets)
pub fn car_lamp_targets(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<crate::sim_lightshome::FixtureDef> {
    let t = &mut g.stats.carlamps;
    t.found = 0; t.head = 0; t.tail = 0; t.lit = 0;
    let (mut heads, mut tails) = (0, 0);
    let found = find_car_lamps(g, boxes);
    let mut out: Vec<crate::sim_lightshome::FixtureDef> = Vec::with_capacity(found.len());
    for l in found {
        let b = boxes[l.box_ as usize];
        g.stats.carlamps.found += 1;
        let n = if l.what == FoundLampWhat::HeadLamp { g.stats.carlamps.head += 1; heads += 1; heads } else { g.stats.carlamps.tail += 1; tails += 1; tails };
        if l.lit { g.stats.carlamps.lit += 1; }
        let (c, emissive, albedo) = (g.col.boxes[b].c, g.col.boxes[b].emissive, g.col.boxes[b].albedo);
        out.push(crate::sim_lightshome::FixtureDef {
            name: alloc::format!("{} {}", l.what.as_str(), n),
            group: String::from(VEHICLE_GROUP),
            kind: crate::sim_lightshome::FixtureDefKind::Emissive,
            pos: [c[0], c[1], c[2]],
            light: None,
            boxes: vec![l.box_],
            base_emissive: [emissive[0], emissive[1], emissive[2]],
            base_intensity: 0.0,
            on: l.lit,
            broken: false,
            disabled_until: -1.0,
            fluorescent_flicker: false,
            factor: if l.lit { 1.0 } else { 0.0 },
            mains: false,
            interactive: true,
            stagger: 0.0,
            area_lights: None,
            lens: Some([albedo[0], albedo[1], albedo[2]]),
        });
    }
    out
}

/// A recording from before the table is played with no lamp a target. A value here is `Opaque`, so only absence is seen, not a null. (carlamps.ts carLampsTape)
pub fn car_lamps_tape(g: &mut crate::game::Game, tables: &JsMap<String, crate::Opaque>) {
    if !tables.has(String::from("CARLAMPS")) { g.k.carlamps.on = false; }
}

/// A vehicle's lamp: glass a round breaks whether it burns or not. (carlamps.ts lampGlass)
pub fn lamp_glass(g: &mut crate::game::Game, t: FixtureId) -> bool {
    g.tab.sim_lightshome_fixture[t].group == VEHICLE_GROUP
}

/// A vehicle's lamp that was dark already: breaking it puts no light out. (carlamps.ts deadLamp)
pub fn dead_lamp(g: &mut crate::game::Game, t: FixtureId) -> bool {
    let f = &g.tab.sim_lightshome_fixture[t];
    f.group == VEHICLE_GROUP && !(f.base_emissive[0] > 0.0 || f.base_emissive[1] > 0.0 || f.base_emissive[2] > 0.0)
}

/// A broken lamp's box painted dark, a whole one as the level has it (`lens`): every frame, from the target's own `broken` bit. (carlamps.ts carLampPaint)
pub fn car_lamp_paint(g: &mut crate::game::Game, t: FixtureId) {
    let f = &g.tab.sim_lightshome_fixture[t];
    let (Some(lens), Some(&i)) = (f.lens, f.boxes.first()) else { return };
    let Some(&b) = g.engine.world.statics.get(i as usize) else { return };
    let broken = crate::sim_lightshome::fixture_broken(g, t);
    let want = if broken { g.k.carlamps.dark } else { lens };
    if !near3(&g.col.boxes[b].albedo, &want, 0.0) {
        g.col.boxes[b].albedo = want;
        if broken { g.stats.carlamps.painted += 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_boxes::Box as WorldBox;

    fn tail(g: &mut Game, x: f64, z: f64) -> BoxId {
        g.col.boxes.push(WorldBox { c: [x, 0.7, z], h: [0.02, 0.06, 0.16], emissive: [0.5, 0.03, 0.02], ..WorldBox::zeroed() })
    }

    #[test]
    fn a_lamp_needs_its_twin_across_the_vehicle() {
        let mut g = Game::blank();
        let (a, b) = (tail(&mut g, 0.0, 0.0), tail(&mut g, 0.0, 1.24));
        let lone = tail(&mut g, 9.0, 9.0);
        let found = find_car_lamps(&mut g, &[lone, a, b]);
        let at: Vec<f64> = found.iter().map(|l| l.box_).collect();
        assert_eq!(at, vec![1.0, 2.0]);
        assert!(found.iter().all(|l| l.lit && l.what == FoundLampWhat::TailLamp));
    }

    #[test]
    fn a_tilted_box_is_no_lamp_and_a_glowing_one_no_lens() {
        let mut g = Game::blank();
        let (a, b) = (tail(&mut g, 0.0, 0.0), tail(&mut g, 0.0, 1.24));
        g.col.boxes[a].rot = Some([0.0, 0.0, 0.0, 1.0]);
        assert!(find_car_lamps(&mut g, &[a, b]).is_empty());
        let lens = g.col.boxes.push(WorldBox { h: [0.02, 0.07, 0.14], albedo: [0.8, 0.8, 0.75], ..WorldBox::zeroed() });
        let s = sigs()[1];
        assert!(fits(&mut g, lens, s));
        g.col.boxes[lens].emissive = [0.0, 0.1, 0.0];
        assert!(!fits(&mut g, lens, s));
    }

    #[test]
    fn targets_number_each_kind_and_count_for_the_checks() {
        let mut g = Game::blank();
        let (a, b) = (tail(&mut g, 0.0, 0.0), tail(&mut g, 0.0, 1.24));
        g.stats.carlamps.found = 7;
        let t = car_lamp_targets(&mut g, &[a, b]);
        assert_eq!((t[0].name.as_str(), t[1].name.as_str()), ("tail lamp 1", "tail lamp 2"));
        assert_eq!(t[1].boxes, vec![1.0]);
        assert_eq!(t[0].group, VEHICLE_GROUP);
        assert!(t[0].on && t[0].factor == 1.0 && !t[0].mains && t[0].stagger == 0.0);
        let s = &g.stats.carlamps;
        assert_eq!((s.found, s.head, s.tail, s.lit), (2, 0, 2, 2));
    }
}
