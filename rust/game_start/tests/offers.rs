//! A DEFENDER'S HAND ON A CLOSING DOOR (rust/game_rules/src/digin/p_01.rs `hand_waits`): one bringing furniture to a door left standing open let
//! the brace go with his hand on the leaf.
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];
/// a number in -1..1 that is a fixed function of the seed and of `k`
fn nudge(seed: u32, k: u32) -> f64 {
    let mut x = seed.wrapping_mul(0x9E37_79B9) ^ k.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 16; x = x.wrapping_mul(0x7FEB_352D); x ^= x >> 15; x = x.wrapping_mul(0x846C_A68B); x ^= x >> 16;
    x as f64 / 2_147_483_648.0 - 1.0
}

// ---------------------------------------------------------------- a defender's hand on a closing leaf

/// The compound's bunk room: its defender is told the team is outside door `a_s`, which stands open as the men who came through left it. Who, and the door.
fn bunk(seed: u32) -> (Play, game::ids::RoomfightDefId, game::ids::DoorId) {
    let mut p = Play::mission("compound.json", &format!("?levelFile=compound.json&roomclear={seed}&squad4"), &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.run(1.0);
    let man = p.g.room_clear.rooms.iter().find(|r| r.def.id == "bunk").unwrap().defenders[0];
    let def = p.g.fight.defs.iter().copied().find(|&d| p.g.tab.roomfight_def[d].gd == man).unwrap();
    let room = p.g.tab.roomfight_def[def].room.unwrap();
    let e = p.g.tab.roomfight_room[room].entries.iter().copied().find(|e| p.g.dr[e.door].def.name == "a_s").unwrap();
    let swing = game::doors::door_swing(&mut p.g, e.door);
    game::doors::door_set_latched(&mut p.g, e.door, false);
    game::doors::door_set_angle(&mut p.g, e.door, swing * (1.5 + 0.3 * nudge(seed, 0)));
    assert!(game::digin::dig_in_order(&mut p.g, def, [e.at[0] + e.out[0] * 8.0, 0.0, e.at[2] + e.out[1] * 8.0], 0.0), "seed {seed}: he was not free to be told");
    (p, def, e.door)
}
fn phase(p: &Play, def: game::ids::RoomfightDefId) -> game::digin::Phase { p.g.m.digin.states.as_ref().unwrap().men.get(p.g.tab.roomfight_def[def].gd).unwrap().phase }
fn wedged(p: &Play, door: game::ids::DoorId) -> bool { p.g.m.digin.states.as_ref().unwrap().wedges.has(door) }

/// THE RULE: with the furniture at a door left standing open, his hand stays on the leaf until it latches, and then he wedges it. A pulled leaf
/// from wide open takes about five seconds to come home.
/// A FAILURE MEANS: a man with furniture is on a fixed wait in `hand_waits` again (before: 2.5 s, then he let the brace go with the leaf 0.1 rad from home, and the door was left unwedged).
#[test]
fn a_defender_stays_with_a_closing_door_and_wedges_it() {
    for s in SEEDS {
        let (mut p, _, door) = bunk(s);
        p.until(30.0, |p| wedged(p, door)).unwrap_or_else(|| panic!("seed {s}: 30 s and {} is not wedged: {:?}", p.g.dr[door].def.name, p.g.m.digin.digin_log.iter().map(|l| l.what.clone()).collect::<Vec<_>>()));
        assert!(game::doors::door_locked(&mut p.g, door), "seed {s}: wedged and not locked");
    }
}

/// THE CONTROL: a man standing in the doorway stops the leaf, and the defender does not wait on it for ever: he lets go on his clock and goes to cover.
#[test]
fn a_defender_lets_go_of_a_leaf_a_body_has_stopped() {
    for s in SEEDS {
        let (mut p, def, door) = bunk(s);
        let c = p.g.dr[door].frame_centre;
        let man = p.g.tab.roomfight_def[def].gd;
        let mate = p.g.guards.iter().copied().find(|&gd| gd != man && p.g.gd[gd].team != game::teams::Team::Operative && p.up(gd)).unwrap();
        let mut put = |p: &mut Play| { p.g.gd[mate].hold = true; p.g.gd[mate].pinned = true; let b = p.g.gd[mate].ch; p.g.ch[b].pos = [c[0], 0.0, c[2]]; };
        let shut = |p: &Play| phase(p, def) == game::digin::Phase::Shut;
        let mut n = 0;
        while !shut(&p) && n < 30 * 60 { put(&mut p); p.frame(); n += 1; }
        assert!(shut(&p), "seed {s}: 30 s and his hand never went on the leaf: {:?}", p.g.m.digin.digin_log.iter().map(|l| l.what.clone()).collect::<Vec<_>>());
        let him = p.g.m.digin.states.as_ref().unwrap().men.get(man).unwrap();
        assert!(him.brace >= 0.0 && him.prop.is_some(), "seed {s}: the staging is wrong: he has no furniture to wedge it with");
        let on = p.time();
        while shut(&p) && p.time() < on + 8.0 { put(&mut p); p.frame(); }
        assert!(!shut(&p), "seed {s}: 8 s on, he still has his hand on a leaf that a body has stopped (angle {:.2})", game::doors::door_angle(&mut p.g, door));
        assert!(!wedged(&p, door), "seed {s}: a door with a man in its doorway was wedged");
    }
}
