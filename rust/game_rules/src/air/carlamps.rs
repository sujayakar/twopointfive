pub use ::game_types::carlamps::types;
pub use types::*;
use crate::ids::*;
use alloc::{string::String, vec, vec::Vec};

/// The light targets of a level's vehicle lamps, which its file names (`vehicleLamps`): emissive, off the mains, no stagger, no die drawn. One burns where its box glows.
pub fn car_lamp_targets(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<crate::fixtures::Fixture> {
    let level = g.level.clone();
    g.stats.carlamps.found = level.vehicle_lamps.len() as i32;
    let (mut heads, mut tails) = (0, 0);
    let mut out: Vec<crate::fixtures::Fixture> = Vec::with_capacity(level.vehicle_lamps.len());
    for l in &level.vehicle_lamps {
        let b = boxes[l.box_ as usize];
        let n = if l.what == crate::carlamps::CarLamp::HeadLamp { heads += 1; heads } else { tails += 1; tails };
        let (c, emissive, albedo) = { let x = &g.tab.boxes[b]; (x.c, x.emissive, x.albedo) };
        let lit = emissive.iter().any(|e| *e > 0.0);
        out.push(crate::fixtures::Fixture {
            name: alloc::format!("{} {}", l.what.as_str(), n),
            group: String::from(VEHICLE_GROUP),
            kind: crate::fixtures::FixtureKind::Emissive,
            pos: [c[0], c[1], c[2]],
            boxes: vec![l.box_],
            base_emissive: [emissive[0], emissive[1], emissive[2]],
            on: lit,
            factor: if lit { 1.0 } else { 0.0 },
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
    if albedo != want {
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
    fn a_vehicle_fixture_is_glass_and_dead_only_if_it_never_glowed() {
        let mut g = Game::blank();
        let dark = fixture(&mut g, VEHICLE_GROUP, [0.0, 0.0, 0.0]);
        let lit = fixture(&mut g, VEHICLE_GROUP, [0.0, 0.0, 0.02]);
        let room = fixture(&mut g, "room", [0.0, 0.0, 0.0]);
        assert!(lamp_glass(&mut g, dark) && lamp_glass(&mut g, lit) && !lamp_glass(&mut g, room));
        assert!(dead_lamp(&mut g, dark) && !dead_lamp(&mut g, lit) && !dead_lamp(&mut g, room));
    }

}
