//! SIX FAULTS A REVIEW FOUND: who startles at a lamp, who is sent to the rack, a routed shot's fix, the item cap's victim, a ring on a civilian, a door put back.
mod world_ask;
use game::game::{DownBy, DownKind, GuardState};
use game::throwables::ItemKind;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{GlassAsk, ReviewAsk, WorldAsk, YardAsk};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

/// A corridor with a lamp, a man 5 m up it who has a line to the lamp, and a man in the room through its wall, as near; Sam out in the dark.
fn lamp_in_a_corridor(s: u32) -> (Play, game::ids::GuardId, game::ids::GuardId, game::ids::FixtureId) {
    let off = 0.3 * nudge(s, 0);
    let mut sc = Scene::new("lamp");
    sc.room("corridor", 6.0, 12.0, 30.0, 16.0).room("beside", 6.0, 6.0, 30.0, 12.0).door("yard", Axis::Z, 6.0, 13.4).door("in", Axis::X, 12.0, 25.4);
    sc.bulb(14.0, 14.0);
    sc.sentry("along", [19.25 + off, 14.0], [14.0, 14.0]).sentry("through", [14.0, 9.25 + off], [14.0, 7.0]);
    let mut p = Play::scene(&sc, &seed(s));
    p.g.player_invisible = true;
    p.run(0.1);
    let (a, b, t) = (p.guard("along"), p.guard("through"), p.first_lamp());
    p.posted_calm(a, [19.25 + off, 14.0], [14.0, 14.0]);
    p.posted_calm(b, [14.0, 9.25 + off], [14.0, 7.0]);
    p.run(0.2);
    (p, a, b, t)
}

/// THE RULE (A6.3): a light going out is SEEN: a line to it, not a distance. The OCP's spell on a lamp is a small thing to the man with a line to it: awareness up, under the suspicious line, no fix to walk to; the man through the wall as near has nothing. The same fixture SHOT dark: awareness 0.4 and his fix under it.
/// A FAILURE MEANS: an OCP brings men over ("08-12: an OCP'd light must not bring men over"), or men feel a lamp through a wall.
#[test]
fn who_startles_at_a_lamp_is_who_has_a_line_to_it() {
    for s in SEEDS {
        let (mut p, a, b, t) = lamp_in_a_corridor(s);
        p.lamp_knocked_out(t);
        p.run(0.05);
        assert!(p.lamp_disabled(t), "seed {s}: the OCP did not take the lamp");
        assert!(p.awareness(a) >= 0.15 && p.awareness(a) < 0.3 && p.fix(a).is_none() && p.small_heard(a) >= 1.0, "seed {s}: the man with a line: awareness {:.2}, fix {:?}, small {}", p.awareness(a), p.fix(a), p.small_heard(a));
        assert!(p.awareness(b) < 0.05 && p.fix(b).is_none() && p.small_heard(b) == 0.0, "seed {s}: through the wall: awareness {:.2}, fix {:?}, small {}", p.awareness(b), p.fix(b), p.small_heard(b));
        drop(p);

        let (mut p, a, _, t) = lamp_in_a_corridor(s);
        p.lamp_shot_dark(t);
        p.run(0.05);
        assert!(p.awareness(a) >= 0.38 && p.fix(a).map_or(false, |f| dist(f, [14.0, 14.0]) < 0.1), "seed {s}: shot dark: awareness {:.2}, fix {:?}", p.awareness(a), p.fix(a));
    }
}

/// THE RULE (A6.5): the rack-check errand is never handed to the man in Sam's arm, a choreographed man or a unit: nobody, or the nearest calm free man. CONTROL: the same men freed: the nearest takes it.
/// A FAILURE MEANS: a man being choked gets up to go and check a rack.
#[test]
fn the_rack_errand_goes_to_the_nearest_man_free_to_go() {
    let mut p = Play::level("office.json", false, &seed(4105));
    p.g.god_mode = true;
    p.run(0.08);
    let men = p.g.guards.clone();
    let (k, n, r) = (men[0], men[1], men[2]);
    for (m, at, face) in [(k, [19.5, 11.3], [30.0, 11.3]), (n, [26.0, 11.3], [10.0, 11.3]), (r, [31.0, 16.0], [31.0, 26.0])] { p.posted_calm(m, at, face); p.g.gd[m].hold = false; }
    p.sam_put([18.7, 11.3]);
    p.key_down("KeyC"); p.frame(); p.key_up("KeyC");
    p.frame();
    game::player::start_grab(&mut p.g, k, None);
    p.until(1.5, |p| p.g.pl[p.g.player].holding.as_ref().map_or(false, |h| h.phase == game::character::HoldPhase::Held)).unwrap_or_else(|| panic!("he was not taken"));
    p.g.gd[n].script = Some(game::game::GuardScript::zeroed());
    p.g.gd[r].unit = Some(game::game::UnitInfo { name: String::from("a unit"), spawned_at: 0.0, light: true, pair: None, seat: None });
    game::mission::send_rack_check(&mut p.g);
    assert!(!p.on_errand(k) && !p.on_errand(n) && !p.on_errand(r), "held, scripted, a unit: errands {} {} {}", p.on_errand(k), p.on_errand(n), p.on_errand(r));
    p.g.gd[n].script = None;
    p.g.gd[r].unit = None;
    game::mission::send_rack_check(&mut p.g);
    assert!(!p.on_errand(k) && (p.on_errand(n) || p.on_errand(r)), "freed: errands {} {} {}", p.on_errand(k), p.on_errand(n), p.on_errand(r));
    assert_eq!(p.state(k), GuardState::Patrol, "the man in the arm");
}

/// A room off a corridor by a door that stands open; a calm man outside it round the jamb, his back to it; Sam inside, by the wall, or in line with the door.
fn pop(s: u32, sam: [f64; 2]) -> (Play, game::ids::GuardId) {
    let mut sc = Scene::new("pop");
    sc.room("room", 10.0, 8.0, 20.0, 14.0).room("corridor", 10.0, 14.0, 20.0, 18.0).door("door", Axis::X, 14.0, 14.4).door("yard", Axis::Z, 10.0, 15.4);
    sc.patrol("out", &[[13.25 + 0.2 * nudge(s, 0), 15.25], [13.25 + 0.2 * nudge(s, 0), 15.75]], 60.0).sam(sam[0], sam[1]);
    let mut p = Play::scene(&sc, &seed(s));
    p.g.player_invisible = true;
    p.door_stands_open("door");
    p.run(0.3);
    let m = p.guard("out");
    (p, m)
}
fn pistol_at_the_far_wall(p: &mut Play) {
    let pl = p.g.player;
    p.key_down("Digit1");
    p.run(0.1);
    p.key_up("Digit1");
    p.run(0.7);
    let c = p.g.pl[pl].ch;
    let at = p.sam_at();
    game::combat::fire_weapon(&mut p.g, c, [at[0], 1.0, at[1] - 3.0], true, None);
    p.run(0.05);
}

/// THE RULE (A6.7): the suppressed pistol fired round a corner: the calm man's fix is the corner the pop came round (the sound path's own turn), never the shooter's feet through the wall. CONTROL: Sam in the man's own line through the door: his fix is Sam.
/// A FAILURE MEANS: a man hears through a wall to the pixel and walks to it.
#[test]
fn a_routed_shot_puts_his_fix_at_the_corner_it_came_round() {
    for s in SEEDS {
        let sam = [13.0, 12.6 + 0.3 * nudge(s, 1)];
        let (mut p, m) = pop(s, sam);
        let q = p.at(m);
        let r = game::scene_collision::sound_paths_propagate(&mut p.g, [sam[0], 1.0, sam[1]], [q[0], 1.5, q[1]], None);
        assert!(!r.direct, "seed {s}: the staging: the straight line is open");
        pistol_at_the_far_wall(&mut p);
        let fix = p.fix(m).unwrap_or_else(|| panic!("seed {s}: the pop did not reach him: awareness {:.2}; the way {:.1} m, carry {:.2}", p.awareness(m), r.path_len, r.carry));
        assert!(p.awareness(m) >= 0.48 && dist(fix, [r.via[0], r.via[2]]) < 0.3 && dist(fix, p.sam_at()) > 1.0, "seed {s}: his fix {fix:?}, the corner ({:.1}, {:.1}), Sam {:?}", r.via[0], r.via[2], p.sam_at());
    }
    let sam = [15.0, 12.0];
    let (mut p, m) = pop(1, sam);
    p.man_put(m, [15.0, 16.0], [15.0, 18.0]);
    p.run(0.1);
    pistol_at_the_far_wall(&mut p);
    assert!(p.fix(m).map_or(false, |f| dist(f, p.sam_at()) < 1.0), "in line through the open door his fix is {:?}, Sam {:?}", p.fix(m), p.sam_at());
}

/// THE RULE (B7.5): thirty-two things and a live can the oldest: the next evicts a dropped pistol, not the can; a smoke can still venting likewise; a spent can goes, and a magazine goes before the oldest dropped gun. Thirty-two live cans and another: nothing live is thrown away.
/// A FAILURE MEANS: a grenade in the air vanishes because a magazine fell.
#[test]
fn the_item_cap_never_eats_a_live_fuse_or_a_venting_can() {
    let mut p = Play::level("kit-yard.json", false, &seed(1));
    let fill = |p: &mut Play, k: ItemKind, n: usize| for _ in 0..n { p.thing(k, None, None); };
    p.things_gone();
    let can = p.thing(ItemKind::Flash, None, None);
    fill(&mut p, ItemKind::Pistol, 32);
    assert!(p.things().len() == 32 && p.things().contains(&can), "a live can the oldest: {} things, the can there {}", p.things().len(), p.things().contains(&can));
    p.things_gone();
    let smoke = p.thing(ItemKind::Smoke, Some(true), Some(20.0));
    fill(&mut p, ItemKind::Torch, 32);
    assert!(p.things().len() == 32 && p.things().contains(&smoke), "a can venting the oldest: {} things", p.things().len());
    p.things_gone();
    let spent = p.thing(ItemKind::Smoke, Some(true), Some(0.0));
    fill(&mut p, ItemKind::Pistol, 32);
    assert!(p.things().len() == 32 && !p.things().contains(&spent), "a spent can the oldest stayed");
    p.things_gone();
    fill(&mut p, ItemKind::Pistol, 10);
    let mag = p.thing(ItemKind::Mag, None, None);
    fill(&mut p, ItemKind::Pistol, 21);
    let first = p.things()[0];
    p.thing(ItemKind::Pistol, None, None);
    assert!(p.things().len() == 32 && !p.things().contains(&mag) && p.things().contains(&first), "a magazine eleventh: it goes before the oldest gun");
    p.things_gone();
    fill(&mut p, ItemKind::Flash, 33);
    assert_eq!(p.things().len(), 33, "thirty-three live cans");
}

/// THE RULE (B7.2, B7.3): a ring to a civilian's head puts her out cold by Sam's hand with a row of the record; one in the ribs frightens: on his feet, alarmed; a ring let go with the tube inside a man's body registers on him instead of sailing through.
/// A FAILURE MEANS: rings pass civilians, or point-blank passes everyone.
#[test]
fn a_ring_finds_a_civilian_and_a_man_at_the_muzzle() {
    let mut p = Play::level("office.json", false, &seed(4113));
    p.g.god_mode = true;
    p.g.player_invisible = true;
    p.run(0.08);
    let men = p.g.guards.clone();
    for (k, &m) in men.iter().enumerate() { p.parked_far(m, k); }
    let civs = p.g.civilians.clone();
    let (m, o) = (civs[0], civs[1]);
    let c = p.g.pl[p.g.player].ch;
    p.man_stands(m, [20.0, 11.25]);
    p.sam_put([14.0, 11.25]);
    p.frame();
    p.man_stands(m, [20.0, 11.25]);
    p.frame();
    let head = *p.g.ch[p.g.gd[m].ch].bones.get(game::character::Bone::Head).expect("a head");
    let at = p.sam_at();
    game::launcher::fire_ring(&mut p.g, c, [at[0] + 0.4, 1.35, at[1]], [head[0], head[1] + 0.04, head[2]]);
    p.until(0.8, |p| p.state(m) == GuardState::Dead).unwrap_or_else(|| panic!("the ring to her head: she is {:?}", p.state(m)));
    assert!(p.g.gd[m].down_kind == Some(DownKind::Struck) && p.g.gd[m].down_by == Some(DownBy::Player) && p.civ_ko_rows() >= 1, "her down: {:?} by {:?}, rows {}", p.g.gd[m].down_kind, p.g.gd[m].down_by, p.civ_ko_rows());
    p.sam_put([20.5, 11.6]);
    p.man_put(o, [26.0, 11.25], [14.0, 11.25]);
    p.run(0.05);
    p.man_put(o, [26.0, 11.25], [14.0, 11.25]);
    p.frame();
    let hips = *p.g.ch[p.g.gd[o].ch].bones.get(game::character::Bone::Hips).expect("hips");
    let at = p.sam_at();
    game::launcher::fire_ring(&mut p.g, c, [at[0] + 0.4, 1.0, at[1]], [hips[0], hips[1] + 0.05, hips[2]]);
    p.until(0.6, |p| p.civ_state(o) == game::civilians::CivState::Alarmed).unwrap_or_else(|| panic!("the ring in his ribs at {hips:?}, he at {:?}, Sam {at:?}: {:?}, {:?}; rings {}", p.at(o), p.state(o), p.civ_state(o), p.g.rings.len()));
    assert_ne!(p.state(o), GuardState::Dead, "the ring in his ribs put him down");
    let n = men[1];
    p.posted_calm(n, [17.0, 11.25], [14.0, 11.25]);
    p.g.gd[n].heard_up_to = 1e12;
    p.frame();
    p.sam_put([16.55, 11.25]);
    p.frame();
    let nh = *p.g.ch[p.g.gd[n].ch].bones.get(game::character::Bone::Hips).expect("hips");
    let was = p.reeled_at(n);
    game::launcher::fire_ring(&mut p.g, c, [nh[0] - 0.15, 1.15, nh[2]], [nh[0] + 3.0, 1.15, nh[2]]);
    p.run(0.1);
    assert!(p.state(n) == GuardState::Dead || p.reeled_at(n) > was, "a ring let go inside him: {:?}, reeled {}", p.state(n), p.reeled_at(n) > was);
}

/// THE RULE (B7.12): a door put back forgets who last touched it: back at its authored angle, still, and nobody's. CONTROL: used by Sam, it says so.
/// A FAILURE MEANS: a door put back blames Sam for a later noise.
#[test]
fn a_door_put_back_forgets_who_touched_it() {
    for s in SEEDS {
        let mut sc = Scene::new("door");
        sc.room("a", 10.0, 10.0, 18.0, 16.0).room("b", 18.0, 10.0, 26.0, 16.0).door("d", Axis::Z, 18.0, 12.0 + 0.6 * nudge(s, 0));
        sc.sam(16.0, 12.8);
        let mut p = Play::scene(&sc, &seed(s));
        p.run(0.1);
        let d = game::doors::doors_by_name(&p.g, "d").unwrap();
        let (at, who) = (p.sam_at(), p.g.ch[p.g.pl[p.g.player].ch].id);
        game::doors::doors_use(&mut p.g, d, [at[0], 0.0, at[1]], who, None);
        p.run(0.3);
        assert!(game::doors::door_last_who(&p.g, d) == who && !game::doors::door_latched(&p.g, d), "seed {s}: used by Sam: last who {}, latched {}", game::doors::door_last_who(&p.g, d), game::doors::door_latched(&p.g, d));
        game::doors::door_reset(&mut p.g, d);
        assert!(game::doors::door_last_who(&p.g, d) == -1.0 && game::doors::door_latched(&p.g, d) && game::doors::door_angle(&p.g, d).abs() < 1e-3, "seed {s}: put back: last who {}, latched {}, angle {}", game::doors::door_last_who(&p.g, d), game::doors::door_latched(&p.g, d), game::doors::door_angle(&p.g, d));
    }
}
