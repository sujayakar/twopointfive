//! src/game/carlamps.ts lines 1 to 105
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A box's row, to read and to write through. (src/scene/boxes.ts:66 on, Box.*)
fn seam_box_row<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// Three channels within `eps`, each. (carlamps.ts near3)
pub fn near3(a: &[f64], b: &[f64], eps: f64) -> bool {
    m::abs(a[0] - b[0]) <= eps && m::abs(a[1] - b[1]) <= eps && m::abs(a[2] - b[2]) <= eps
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
    let albedo = seam_box_row(g, b).albedo;
    if !near3(&albedo, &want, 0.0) {
        seam_box_row(g, b).albedo = want;
        if broken { g.stats.carlamps.painted += 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim_lightshome::Fixture;

    fn fixture(g: &mut Game, group: &str, base_emissive: V3) -> FixtureId {
        g.tab.sim_lightshome_fixture.push(Fixture { group: String::from(group), base_emissive, ..Fixture::blank() })
    }

    #[test]
    fn near3_holds_each_channel_to_eps_inclusive() {
        assert!(near3(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0], 0.0));
        assert!(near3(&[1.0, 2.0, 3.0], &[1.5, 2.0, 3.0], 0.5));
        assert!(!near3(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.5], 0.25));
        assert!(!near3(&[f64::NAN, 2.0, 3.0], &[f64::NAN, 2.0, 3.0], 1.0));
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

// These stand on g.col.boxes as a table of rows, as skeleton 4 had it. Compiled out until the table is back (notes.md, ## Skeleton).
#[cfg(any())]
mod tests_need_box_rows {
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
