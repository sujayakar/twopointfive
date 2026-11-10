//! WHAT THE GAME OFFERS AND WHAT IT THEN DOES, on three places the old checks read as faults.
//!
//! Two of them are held here as they are: a keypad door's entry rows (`door_tools_stackable`) and Sam's wall row (`wall_charge_sam_rows`). The
//! third was a fault: a defender bringing furniture to a door standing open let the brace go with his hand on the leaf (`digin::hand_waits`).
use game::orders::{squad_orders_give, squad_orders_target_at, squad_orders_why_not, OrderTarget};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];
/// a number in -1..1 that is a fixed function of the seed and of `k`
fn nudge(seed: u32, k: u32) -> f64 {
    let mut x = seed.wrapping_mul(0x9E37_79B9) ^ k.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 16; x = x.wrapping_mul(0x7FEB_352D); x ^= x >> 15; x = x.wrapping_mul(0x846C_A68B); x ^= x >> 16;
    x as f64 / 2_147_483_648.0 - 1.0
}

// ---------------------------------------------------------------- a keypad door on the face it opens toward

/// The manager's office, its keypad door to the server room hung to open toward the office, Sam in the office, nobody left to fight, the squad's
/// big slots empty (a man with the hammer or the breaching shotgun takes the lock himself): the door, its middle, and where Sam stands.
fn keypad(seed: u32, charges: Option<f64>) -> (Play, game::ids::DoorId, [f64; 3]) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.5);
    for m in p.mates() { if let Some(mut e) = p.g.m.kit.man_of.get(m).cloned() { e.kit.big = game::kit::BigKit::None; p.g.m.kit.man_of.set(m, e); } }
    if let Some(n) = charges { p.g.sets[p.g.seat].door_tools.charges = n; }
    for gd in p.g.guards.clone() { if p.g.gd[gd].team != game::teams::Team::Operative && p.up(gd) { game::combat::kill_guard(&mut p.g, gd, [0.0, 0.0, 1.0], Some(true), None, None, game::game::With::Hand); } }
    p.run(1.0);
    let office = game::doors::doors_by_name(&p.g, "manager").unwrap();
    p.g.dr[office].leaf.locked = false;
    let d = game::doors::doors_by_name(&p.g, "server_manager").unwrap();
    let c = p.g.dr[d].frame_centre;
    let at = [c[0] + 2.0, 0.0, c[2]];
    if !game::doors::doors_pull_face(&mut p.g, d, at) { let s = game::doors::door_swing(&mut p.g, d); game::doors::door_set_swing(&mut p.g, d, -s, None, None); }
    p.sam_put([at[0], at[2]]);
    p.run(1.0);
    (p, d, c)
}
/// why the game would not take that row there (None: the menu shows it live)
fn row(p: &mut Play, id: &str, t: &OrderTarget) -> Option<String> {
    let d = p.g.tab.orders_order_def.iter().map(|(i, _)| i).find(|&d| p.g.tab.orders_order_def[d].id == id).unwrap();
    squad_orders_why_not(&mut p.g, d, t, None)
}

/// THE RULE: the menu does not read a lock for the men. The entry rows are live until somebody has tried that face: the order is taken, the lead
/// goes to the door and says why it will not do, and nobody boots it. From then on the rows are grey in his words and the order is refused in them,
/// by the one call the menu and the order share. The charge row stays live throughout.
/// A FAILURE MEANS: a row the menu shows live is no longer an order the game takes, or a grey row no longer gives the game's own reason.
#[test]
fn a_keypad_door_is_tried_then_greyed_in_the_words_it_was_refused_in() {
    for (charges, words) in [(None, "opens toward us. no boot. charge it."), (Some(0.0), "opens toward us. no boot, no charge.")] {
        let (mut p, d, c) = keypad(7, charges);
        let t = squad_orders_target_at(&mut p.g, [c[0], 0.0, c[2]]);
        assert!(matches!(t, OrderTarget::Door(_)), "the staging is wrong: the order does not land on the door");
        for id in ["openClear", "bangClear"] { assert_eq!(row(&mut p, id, &t), None, "{id}: nobody has tried that face and the menu has read the lock"); }
        assert_eq!(squad_orders_give(&mut p.g, "openClear", Some(&t), None, None), None, "a live row was refused");
        p.until(60.0, |p| p.said.iter().any(|l| l.text == words)).unwrap_or_else(|| panic!("a minute and the lead has not said {words:?}: {}", p.roll()));
        p.run(0.5);
        assert!(p.g.sets[p.g.seat].door_tools.ops.is_empty(), "he said it and the order goes on");
        assert!(p.g.dr[d].leaf.locked, "the door gave");
        let t = squad_orders_target_at(&mut p.g, [c[0], 0.0, c[2]]);
        for id in ["openClear", "bangClear"] { assert_eq!(row(&mut p, id, &t).as_deref(), Some(words), "{id}: known shut now, and the row is not grey in his words"); }
        assert_eq!(squad_orders_give(&mut p.g, "openClear", Some(&t), None, None).as_deref(), Some(words), "the order is not refused in the row's words");
        assert_eq!(row(&mut p, "charge", &t).is_none(), charges.is_none(), "the charge row is live exactly while a charge is carried");
    }
}

// ---------------------------------------------------------------- a defender's hand on a closing leaf

/// The compound's bunk room: its defender is told the team is outside door `a_s`, which stands open as the men who came through left it. Who, and the door.
fn bunk(seed: u32) -> (Play, game::ids::RoomfightDefId, game::ids::DoorId) {
    let mut p = Play::mission("compound.json", &format!("?levelFile=compound.json&roomclear={seed}&squad4"), &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.run(1.0);
    let man = p.g.staged.rooms.iter().find(|r| r.def.id == "bunk").unwrap().defenders[0];
    let def = p.g.fight.defs.iter().copied().find(|&d| p.g.tab.roomfight_def[d].gd == man).unwrap();
    let room = p.g.tab.roomfight_def[def].room.unwrap();
    let e = p.g.tab.roomfight_room[room].entries.iter().copied().find(|e| p.g.dr[e.door].def.name == "a_s").unwrap();
    let swing = game::doors::door_swing(&mut p.g, e.door);
    p.g.dr[e.door].leaf.latched = false;
    p.g.dr[e.door].leaf.angle = swing * (1.5 + 0.3 * nudge(seed, 0));
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
        p.until(30.0, |p| wedged(p, door)).unwrap_or_else(|| panic!("seed {s}: 30 s and {} is not wedged", p.g.dr[door].def.name));
        assert!(p.g.dr[door].leaf.locked, "seed {s}: wedged and not locked");
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
        let put = |p: &mut Play| { p.g.gd[mate].hold = true; p.g.gd[mate].pinned = true; let b = p.g.gd[mate].ch; p.g.ch[b].pos = [c[0], 0.0, c[2]]; };
        let shut = |p: &Play| phase(p, def) == game::digin::Phase::Shut;
        let mut n = 0;
        while !shut(&p) && n < 30 * 60 { put(&mut p); p.frame(); n += 1; }
        assert!(shut(&p), "seed {s}: 30 s and his hand never went on the leaf");
        let him = p.g.m.digin.states.as_ref().unwrap().men.get(man).unwrap();
        assert!(him.brace >= 0.0 && him.prop.is_some(), "seed {s}: the staging is wrong: he has no furniture to wedge it with");
        let on = p.time();
        while shut(&p) && p.time() < on + 8.0 { put(&mut p); p.frame(); }
        assert!(!shut(&p), "seed {s}: 8 s on, he still has his hand on a leaf that a body has stopped (angle {:.2})", p.g.dr[door].leaf.angle);
        assert!(!wedged(&p, door), "seed {s}: a door with a man in its doorway was wedged");
    }
}

// ---------------------------------------------------------------- Sam's wall row

/// THE RULE: on a place the game takes a charge, Sam at its foot with the cursor on what the camera sees of that wall (its top) is offered the
/// row, and the row is for that place. The places are those the old check read as no row: its cursor lay on a point of the near face, which from
/// the camera can be behind the wall's own top, a shelf or the next wall, so the ray met that and the game answered for that.
/// A FAILURE MEANS: a wall the game would take a charge on has no row for a man standing at it.
#[test]
fn sam_is_offered_the_wall_row_where_a_charge_can_be_set() {
    for (level, index, u, side) in [("block.json", 115, 10.44, -1.0), ("block.json", 115, 11.19, -1.0), ("depot.json", 9, 10.48, -1.0), ("pass.json", 16, 29.05, -1.0), ("pass.json", 41, 14.22, 1.0), ("mousehole.json", 36, 13.05, 1.0)] {
        let mut p = Play::mission(level, &format!("?levelFile={level}&roomclear&squad4"), &Opts { seed: 7, ..Default::default() });
        p.g.god_mode = true;
        for gd in p.g.guards.clone() { if p.g.gd[gd].team != game::teams::Team::Operative { p.g.gd[gd].hold = true; p.g.gd[gd].pinned = true; } }
        p.run(0.5);
        let (c, h) = { let b = &p.g.tab.boxes[p.g.col.boxes[index]]; (b.c, b.h) };
        let along_x = h[0] >= h[2];
        let foot_of = if along_x { [u, 0.0, c[2] + side * 2.0] } else { [c[0] + side * 2.0, 0.0, u] };
        let at = if along_x { [u, 0.0, c[2]] } else { [c[0], 0.0, u] };
        let w = game::wallcharge::wall_at(&mut p.g, at, Some(foot_of)).unwrap_or_else(|| panic!("{level} static {index}: no wall there"));
        assert!(w.index == index as f64 && w.ok, "{level} static {index}: the staging is wrong ({})", w.why);
        let w = game::wallcharge::wall_charge_settle(&mut p.g, &w);
        assert!((w.u - u).abs() < 1e-6, "{level} static {index}: the charge settles at u {} and not at {u}", w.u);
        assert!(game::wallcharge::wall_charge_far_floor(&mut p.g, &w, None).is_some(), "{level} static {index} u {u}: the game takes no charge there now");
        game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
        let spot = game::wallcharge::wall_charge_set_spot(&mut p.g, &w).unwrap_or_else(|| panic!("{level} static {index} u {u}: no place to set it from"));
        p.sam_put([spot[0], spot[2]]);
        let top = if along_x { [u, c[1] + h[1], c[2]] } else { [c[0], c[1] + h[1], u] };
        for _ in 0..90 { p.cursor_on(top); p.frame(); }
        let rows: Vec<String> = p.g.sets[p.g.seat].nearby.iter().map(|r| format!("{:?}:{:?}", r.kind, r.verb)).collect();
        assert!(rows.iter().any(|r| r == "Wall:Some(Charge)"), "{level} static {index} u {u}: no row for the charge, with Sam at {:?} and the cursor at {:?}: {rows:?}", p.sam_at(), p.g.sets[p.g.seat].aim_point);
        let o = p.g.sets[p.g.seat].wall_charge.offered.clone().unwrap();
        assert!(o.index == index as f64 && (o.u - u).abs() < 0.02, "{level} static {index} u {u}: the row is for wall {} u {}", o.index, o.u);
    }
}

// ---------------------------------------------------------------- a can up behind a man

/// THE RULE: with a can up, crouched behind a man whose back is turned, the row is the plain grab and the way to the knockout; a man who is only
/// suspicious is no different.
/// A FAILURE MEANS: the can has changed the close-in offer. (The old check read "the grab is from behind", because its man had turned on it.)
#[test]
fn a_can_up_behind_a_man_offers_the_grab() {
    for state in [game::game::GuardState::Patrol, game::game::GuardState::Suspicious] {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad=1", &Opts { seed: 7, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.2);
        let a = p.hostiles()[0];
        p.drop_all_but(&[a]);
        let (at, east) = ([14.0, 11.2], std::f64::consts::FRAC_PI_2);
        let aware = if state == game::game::GuardState::Patrol { 0.0 } else { 0.4 };
        p.sam_put([at[0] - 0.8, at[1]]);
        let me = p.g.player;
        p.g.pl[me].crouch = true;
        for _ in 0..10 { p.pin_man(a, at, east, state, aware); p.frame(); }
        p.g.pl[me].slot = game::game::SlotKind::Smoke;
        p.pin_man(a, at, east, state, aware);
        p.frame();
        let row = p.g.sets[p.g.seat].nearby.iter().find(|r| r.guard == Some(a)).map(|r| r.line2.clone()).unwrap_or_default();
        assert!(row.contains("grab him") && row.contains("holster for the knockout") && !row.contains("from behind"), "{state:?}: the row is '{row}'");
    }
}
