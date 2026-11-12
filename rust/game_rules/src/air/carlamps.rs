pub use ::game_types::carlamps::types;
pub use types::*;
use crate::ids::*;
use crate::m;
use crate::v3;
use alloc::{string::String, vec, vec::Vec};

/// One lamp box that fits a generator's lamp: its place in the list and in `sigs()`.
#[derive(Clone, Copy)]
struct Cand { i: usize, s: usize }

/// What each generator writes for a lamp.
fn sigs() -> Vec<crate::carlamps::LampSig> {
    let (white, red) = ([0.7, 0.7, 0.66], [0.42, 0.06, 0.05]);   // (the look's white plastic and its red truck)
    vec![
        LampSig { what: crate::carlamps::CarLamp::TailLamp, h: [0.02, 0.06, 0.16], albedo: None, emissive: Some([0.5, 0.03, 0.02]), gap: 1.24 },   // the car
        LampSig { what: crate::carlamps::CarLamp::HeadLamp, h: [0.02, 0.07, 0.14], albedo: Some([0.8, 0.8, 0.75]), emissive: None, gap: 1.24 },
        LampSig { what: crate::carlamps::CarLamp::HeadLamp, h: [0.03, 0.07, 0.16], albedo: Some(white), emissive: None, gap: 1.32 },                // the pickup and the van
        LampSig { what: crate::carlamps::CarLamp::TailLamp, h: [0.03, 0.08, 0.1], albedo: Some(red), emissive: None, gap: 1.44 },                   // the pickup
        LampSig { what: crate::carlamps::CarLamp::TailLamp, h: [0.02, 0.15, 0.04], albedo: Some(red), emissive: None, gap: 1.72 },                  // the van
        LampSig { what: crate::carlamps::CarLamp::HeadLamp, h: [0.02, 0.08, 0.1], albedo: Some(white), emissive: None, gap: 1.4 },                  // the pass's lorry
    ]
}

/// Three channels within `eps`, each.
fn near3(a: &[f64], b: &[f64], eps: f64) -> bool {
    m::abs(a[0] - b[0]) <= eps && m::abs(a[1] - b[1]) <= eps && m::abs(a[2] - b[2]) <= eps
}

/// The box glows at all.
fn glows(g: &crate::game::Game, b: BoxId) -> bool {
    let e = g.tab.boxes[b].emissive;
    e[0] > 0.0 || e[1] > 0.0 || e[2] > 0.0
}

/// The box is this generator's lamp: size, then glow or paint.
fn fits(g: &crate::game::Game, b: BoxId, s: crate::carlamps::LampSig) -> bool {
    let (rot, h, glow, albedo) = { let x = &g.tab.boxes[b]; (x.rot, x.h, x.emissive, x.albedo) };
    if rot.is_some() || !near3(&h, &s.h, g.k.carlamps.fit) { return false; }
    if let Some(emissive) = s.emissive { return near3(&glow, &emissive, g.k.carlamps.tone); }
    !glows(g, b) && near3(&albedo, &s.albedo.unwrap(), g.k.carlamps.tone)
}

/// The lamp boxes of a level's vehicles in box order: each fits a generator's lamp and has its twin across the vehicle.
pub fn find_car_lamps(g: &crate::game::Game, boxes: &[BoxId]) -> Vec<crate::carlamps::FoundLamp> {
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
        let (ca, yaw_a) = { let x = &g.tab.boxes[aa]; (x.c, x.yaw) };
        let twin_k = g.k.carlamps.twin;
        let twin = cand.iter().any(|b| {
            let other = &g.tab.boxes[boxes[b.i]];
            b.i != a.i && b.s == a.s
                && m::abs(other.c[1] - ca[1]) <= twin_k
                && m::abs(other.yaw - yaw_a) <= 1e-3
                && m::abs(v3::dist_xz(other.c, ca) - s.gap) <= twin_k
        });
        if twin {
            let what = s.what;
            out.push(FoundLamp { box_: a.i as f64, what, lit: s.emissive.is_some() });
        }
    }
    out
}

/// The light targets of a level's vehicle lamps: emissive, off the mains, no stagger, no die drawn.
pub fn car_lamp_targets(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<crate::fixtures::Fixture> {
    let t = &mut g.stats.carlamps;
    t.found = 0;   
    let (mut heads, mut tails) = (0, 0);
    let found = find_car_lamps(g, boxes);
    let mut out: Vec<crate::fixtures::Fixture> = Vec::with_capacity(found.len());
    for l in found {
        let b = boxes[l.box_ as usize];
        g.stats.carlamps.found += 1;
        let n = if l.what == crate::carlamps::CarLamp::HeadLamp {  heads += 1; heads } else {  tails += 1; tails };
        let (c, emissive, albedo) = { let x = &g.tab.boxes[b]; (x.c, x.emissive, x.albedo) };
        out.push(crate::fixtures::Fixture {
            name: alloc::format!("{} {}", l.what.as_str(), n),
            group: String::from(VEHICLE_GROUP),
            kind: crate::fixtures::FixtureKind::Emissive,
            pos: [c[0], c[1], c[2]],
            boxes: vec![l.box_],
            base_emissive: [emissive[0], emissive[1], emissive[2]],
            on: l.lit,
            factor: if l.lit { 1.0 } else { 0.0 },
            interactive: true,
            lens: Some([albedo[0], albedo[1], albedo[2]]),
            ..crate::fixtures::Fixture::blank()
        });
    }
    out
}

/// A vehicle's lamp: glass a round breaks whether it burns or not.
pub fn lamp_glass(g: &crate::game::Game, t: FixtureId) -> bool {
    g.tab.fixture[t].group == VEHICLE_GROUP
}

/// A vehicle's lamp that was dark already: breaking it puts no light out.
pub fn dead_lamp(g: &crate::game::Game, t: FixtureId) -> bool {
    let f = &g.tab.fixture[t];
    f.group == VEHICLE_GROUP && !(f.base_emissive[0] > 0.0 || f.base_emissive[1] > 0.0 || f.base_emissive[2] > 0.0)
}

/// A broken lamp's box painted dark, a whole one as the level has it (`lens`): every frame, from the target's own `broken` bit.
pub(super) fn car_lamp_paint(g: &mut crate::game::Game, t: FixtureId) {
    let f = &g.tab.fixture[t];
    let (Some(lens), Some(&i)) = (f.lens, f.boxes.first()) else { return };
    let Some(&b) = g.engine.world.statics.get(i as usize) else { return };
    let broken = g.tab.fixture[t].broken;
    let want = if broken { g.k.carlamps.dark } else { lens };
    let albedo = g.tab.boxes[b].albedo;
    if !near3(&albedo, &want, 0.0) {
        g.tab.boxes.edit(b).albedo = want;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::v3::V3;
    use super::*;
    use crate::fixtures::Fixture;

    fn fixture(g: &mut Game, group: &str, base_emissive: V3) -> FixtureId {
        g.tab.fixture.push(Fixture { group: String::from(group), base_emissive, ..Fixture::blank() })
    }

    #[test]
    fn near3_holds_each_channel_to_eps_inclusive() {
        assert!(near3(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0], 0.0));
        assert!(near3(&[1.0, 2.0, 3.0], &[1.5, 2.0, 3.0], 0.5));
        assert!(!near3(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.5], 0.25));
        assert!(!near3(&[f64::NAN, 2.0, 3.0], &[f64::NAN, 2.0, 3.0], 1.0));
    }

    #[test]
    fn every_generator_lamp_says_glow_or_paint_never_both() {
        let s = sigs();
        assert_eq!(s.len(), 6);
        assert!(s.iter().all(|x| x.emissive.is_some() != x.albedo.is_some()));
    }

    #[test]
    fn a_vehicle_fixture_is_glass_and_dead_only_if_it_never_glowed() {
        let mut g = Game::blank();
        let dark = fixture(&mut g, VEHICLE_GROUP, [0.0, 0.0, 0.0]);
        let lit = fixture(&mut g, VEHICLE_GROUP, [0.0, 0.0, 0.02]);
        let room = fixture(&mut g, "room", [0.0, 0.0, 0.0]);
        assert!(lamp_glass(&mut g, dark) && lamp_glass(&mut g, lit) && !lamp_glass(&mut g, room));
        assert!(dead_lamp(&mut g, dark) && !dead_lamp(&mut g, lit) && !dead_lamp(&mut g, room));
    }

}

#[cfg(any())]
mod tests_need_box_rows {
    use super::*;
    use crate::scene_boxes::Box as WorldBox;

    fn tail(g: &Game, x: f64, z: f64) -> BoxId {
        g.col.boxes.push(WorldBox { c: [x, 0.7, z], h: [0.02, 0.06, 0.16], emissive: [0.5, 0.03, 0.02], ..WorldBox::default() })
    }

    #[test]
    fn a_lamp_needs_its_twin_across_the_vehicle() {
        let mut g = Game::blank();
        let (a, b) = (tail(&g, 0.0, 0.0), tail(&g, 0.0, 1.24));
        let lone = tail(&g, 9.0, 9.0);
        let found = find_car_lamps(&g, &[lone, a, b]);
        let at: Vec<f64> = found.iter().map(|l| l.box_).collect();
        assert_eq!(at, vec![1.0, 2.0]);
        assert!(found.iter().all(|l| l.lit && l.what == crate::carlamps::CarLamp::TailLamp));
    }

    #[test]
    fn a_tilted_box_is_no_lamp_and_a_glowing_one_no_lens() {
        let mut g = Game::blank();
        let (a, b) = (tail(&g, 0.0, 0.0), tail(&g, 0.0, 1.24));
        g.col.boxes[a].rot = Some([0.0, 0.0, 0.0, 1.0]);
        assert!(find_car_lamps(&g, &[a, b]).is_empty());
        let lens = g.col.boxes.push(WorldBox { h: [0.02, 0.07, 0.14], albedo: [0.8, 0.8, 0.75], ..WorldBox::default() });
        let s = sigs()[1];
        assert!(fits(&g, lens, s));
        g.col.boxes[lens].emissive = [0.0, 0.1, 0.0];
        assert!(!fits(&g, lens, s));
    }

    #[test]
    fn targets_number_each_kind_and_count_for_the_checks() {
        let mut g = Game::blank();
        let (a, b) = (tail(&g, 0.0, 0.0), tail(&g, 0.0, 1.24));
        g.stats.carlamps.found = 7;
        let t = car_lamp_targets(&g, &[a, b]);
        assert_eq!((t[0].name.as_str(), t[1].name.as_str()), ("tail lamp 1", "tail lamp 2"));
        assert_eq!(t[1].boxes, vec![1.0]);
        assert_eq!(t[0].group, VEHICLE_GROUP);
        assert!(t[0].on && t[0].factor == 1.0 && !t[0].mains && t[0].stagger == 0.0);
        let s = &g.stats.carlamps;
        assert_eq!((s.found, s.head, s.tail, s.lit), (2, 0, 2, 2));
    }
}
