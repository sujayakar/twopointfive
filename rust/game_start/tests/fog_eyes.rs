//! THE FOG OF WAR AS THE TEAM HAS IT: whose eyes show whom, what a cone hides, what is kept where it was last seen.
mod stealth_ask;
use game::game::GuardState;
use game::ids::GuardId;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
const EAST: f64 = std::f64::consts::FRAC_PI_2;
const NORTH: f64 = std::f64::consts::PI;

/// Two rooms with shut doors, a defender in the west room, the team in the yard with no line to him, the fog of war up.
fn shut_in(s: u32) -> (Play, GuardId, [f64; 2]) {
    let mut sc = Scene::new("shut-in");
    sc.room("west", 10.0, 10.0, 22.0, 16.0).room("east", 22.0, 10.0, 28.0, 16.0);
    sc.door("west-yard", Axis::X, 16.0, 19.4).door("east-yard", Axis::X, 16.0, 25.0).door("between", Axis::Z, 22.0, 12.4);
    sc.bulb(16.0, 13.0).bulb(25.0, 13.0).moon();
    sc.team([14.0, 21.0], NORTH, &[[13.0, 22.0], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    let post = [18.25, 12.25 + 0.5 * nudge(s, 0).round()];
    sc.defender("west", post, "west-yard").defender("east", [25.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.fog_up();
    p.run(0.2);
    let h = p.defender_at(post);
    (p, h, post)
}
fn pinned(p: &mut Play, h: GuardId, at: [f64; 2], frames: usize, mut each: impl FnMut(&mut Play)) {
    for _ in 0..frames { p.pin_man(h, at, EAST, GuardState::Patrol, 0.0); each(p); p.frame(); }
}

/// THE RULE: "teammates should see whatever we see": a line from anybody on the team shows a man. With no eye on him he is hidden; a teammate stood in the room shows him, with Sam
/// outside with no line; the teammate back out, he is hidden again past the linger. CONTROL: the first of those three.
/// A FAILURE MEANS: sight is Sam's alone, or a man stays in the picture with no line to him.
#[test]
fn a_line_from_anybody_on_the_team_shows_a_man() {
    for s in SEEDS {
        let (mut p, h, post) = shut_in(s);
        pinned(&mut p, h, post, 30, |_| {});
        assert!(p.fogged(h), "seed {s}: nobody has a line, and he is in the picture");
        let m = p.mates()[0];
        let (inside, out) = ([14.25, 13.25], p.at(m));
        pinned(&mut p, h, post, 15, |p| p.man_stands(m, inside));
        assert!(!p.fogged(h), "seed {s}: a teammate in the room has a line, and he is hidden");
        pinned(&mut p, h, post, 45, |p| p.man_stands(m, out));
        assert!(p.fogged(h), "seed {s}: the teammate gone, and he is still in the picture");
    }
}

/// THE RULE: what the team does is always shown: a doing of Sam's or a teammate's is the team's own, a hidden defender's or nobody's is not.
/// A FAILURE MEANS: the team's own flash, round or broken box vanishes from the picture where no eye is on it.
#[test]
fn what_the_team_does_is_always_shown() {
    for s in SEEDS {
        let (mut p, h, post) = shut_in(s);
        pinned(&mut p, h, post, 30, |_| {});
        let (sam, mate, foe) = (p.g.player_id(p.g.player), p.id_of(p.mates()[0]), p.id_of(h));
        assert!(p.fogged(h), "seed {s}: staging: he is hidden");
        assert!(p.ours(Some(sam)) && p.ours(Some(mate)), "seed {s}: Sam's doing and a teammate's are the team's");
        assert!(!p.ours(Some(foe)) && !p.ours(None), "seed {s}: a hidden defender's or nobody's are not");
    }
}

/// THE RULE: on Hard an eye is a cone, a man behind every back is hidden; on Standard it is all round. Sam in a man's room with a clear line and the team's other eyes shut out:
/// facing him he is seen; his back turned the man is hidden; cone off he is seen with his back turned; and a metre off, inside the near field, he is seen all round.
/// A FAILURE MEANS: Hard's fog no longer costs anything, or Standard's does.
#[test]
fn on_hard_an_eye_is_a_cone_and_on_standard_all_round() {
    for s in SEEDS {
        let (mut p, h, post) = shut_in(s);
        p.sam_takes_his_rifle();
        let stand = [12.25, 13.25];
        p.sam_put(stand);
        p.g.k.fog.cone = true;
        let d = [post[0] - stand[0], post[1] - stand[1]];
        p.point_at(post);
        pinned(&mut p, h, post, 50, |_| {});
        assert!(!p.fogged(h), "seed {s}: the cone: facing him, he is hidden");
        p.point_at([stand[0] - d[0], stand[1] - d[1]]);
        pinned(&mut p, h, post, 80, |_| {});
        assert!(p.fogged(h), "seed {s}: the cone: his back to him at {:.1} m, the man is seen", dist(p.sam_at(), post));
        p.g.k.fog.cone = false;
        pinned(&mut p, h, post, 3, |_| {});
        assert!(!p.fogged(h), "seed {s}: Standard: all round, and he is hidden");
        p.g.k.fog.cone = true;
        pinned(&mut p, h, post, 40, |_| {});
        assert!(p.fogged(h), "seed {s}: the cone back: hidden again");
        p.sam_put([post[0] - 1.0, post[1]]);
        p.point_at([post[0] - 6.0, post[1]]);
        pinned(&mut p, h, post, 40, |_| {});
        assert!(!p.fogged(h), "seed {s}: a metre off with his back to him, inside the near field, and he is hidden");
    }
}

/// THE RULE: a loose thing that moves where nobody sees it stays in the picture where it was last seen; when somebody looks there the picture has it where it is. CONTROL: furniture
/// moved in plain sight is not kept.
/// A FAILURE MEANS: the player reads the enemy's movements off the furniture through the fog, or the picture keeps a chair that is long gone.
#[test]
fn a_loose_thing_that_moves_where_nobody_sees_it_stays_where_it_was_last_seen() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.drop_all_but(&[]);
        p.run(1.0);
        let sam = p.sam_at();
        let mut far = p.furniture();
        far.sort_by(|a, b| dist(p.prop_at(*b), sam).partial_cmp(&dist(p.prop_at(*a), sam)).unwrap());
        let it = far[(s as usize) % 3];
        let home = p.prop_at(it);
        assert!(p.things_kept().iter().all(|(_, held)| !held), "seed {s}: a calm floor keeps nothing back");
        p.prop_shoved(it, [home[0] + 0.7, home[1] + 0.3]);
        p.run(1.5);
        let kept: Vec<[f64; 2]> = p.things_kept().iter().filter(|(_, held)| *held).map(|(at, _)| *at).collect();
        assert!(kept.len() == 1 && dist(kept[0], home) < 0.3, "seed {s}: shoved where nobody sees: kept {kept:?}, it stood at {home:?}");
        p.sam_put([home[0] + 1.3, home[1]]);
        p.run(1.0);
        assert!(p.things_kept().iter().all(|(_, held)| !held), "seed {s}: Sam at it, and the picture still keeps it back");
        // CONTROL: with Sam beside it, moved again: the picture goes with it
        p.prop_shoved(it, [home[0] + 0.9, home[1] + 0.5]);
        p.run(0.5);
        assert!(p.things_kept().iter().all(|(_, held)| !held), "seed {s}: moved in plain sight, and kept back");
    }
}

/// THE RULE: "their movement is visible, but it looks like they're walking through a closed door": no teammate walks through a leaf the picture has shut. Under the fewest eyes that can bring a leaf up to date
/// (only Sam's own, his back to the door) the team opens and clears a door: no man crosses a leaf drawn shut, and none is at the doorway for more than a fifth of a second with the leaf drawn wrong.
/// A FAILURE MEANS: the picture shows a man walking through a shut door.
#[test]
fn no_teammate_walks_through_a_leaf_the_picture_has_shut() {
    use game::orders::Who;
    for s in SEEDS {
        let mut p = Play::mission("records.json", "?levelFile=records.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.fog_up();
        p.g.k.fog.own = true;
        p.run(0.5);
        let foes = p.hostiles();
        let sam0 = p.sam_at();
        let mut doors = p.door_names();
        doors.sort_by(|a, b| dist(p.door_at(a), sam0).partial_cmp(&dist(p.door_at(b), sam0)).unwrap());
        let name = doors.into_iter().find(|n| {
            let (_, angle, _) = p.leaf(n);
            let at = p.door_at(n);
            if angle.abs() >= 0.05 { return false; }
            let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
            game::orders::squad_orders_give(&mut p.g, "openClear", Some(&t), Some(Who::All), None).is_none()
        }).unwrap_or_else(|| panic!("seed {s}: no door takes the order"));
        let door = p.door_at(&name);
        let men = p.mates();
        let side = |p: &Play, m: GuardId| { let (a, b) = (door_normal(p, &name), p.at(m)); ((b[0] - door[0]) * a[0] + (b[1] - door[1]) * a[1]) >= 0.0 };
        let mut was: Vec<bool> = men.iter().map(|&m| side(&p, m)).collect();
        let (mut opened, mut run, mut most, mut crossed) = (false, 0, 0, 0);
        let sam = p.sam_at();
        let behind = [sam[0] * 2.0 - door[0], sam[1] * 2.0 - door[1]];
        p.point_at(behind);
        for _ in 0..(60 * 15) {
            for &f in foes.iter() { p.g.gd[f].duty.holds = true; p.g.gd[f].duty.pinned = true; }
            p.frame();
            let (held, angle, shown) = p.leaf(&name);
            if angle.abs() > 0.3 { opened = true; }
            if held && (angle - shown).abs() > 0.15 && men.iter().any(|&m| p.up(m) && dist(p.at(m), door) < 1.0) { run += 1; most = most.max(run); } else { run = 0; }
            for (i, &m) in men.iter().enumerate() {
                let now = side(&p, m);
                if now != was[i] && dist(p.at(m), door) < 1.0 && held && shown.abs() < 0.3 { crossed += 1; }
                was[i] = now;
            }
        }
        assert!(opened, "seed {s}: 15 s and the door '{name}' never opened");
        assert_eq!(crossed, 0, "seed {s}: men through the leaf of '{name}' drawn shut");
        assert!(most <= 12, "seed {s}: a man at the doorway of '{name}' with the leaf drawn wrong for {most} frames");
    }
}
fn door_normal(p: &Play, name: &str) -> [f64; 2] { let d = game::doors::doors_by_name(&p.g, name).unwrap(); let th = p.g.dr[d].def.closed_dir; [-th.sin(), th.cos()] }

/// On the open lot north of the office in this weather, the team at its west end, the fog of war up: a man of the other side held 25 m east of them, and nobody else about.
fn across_the_lot(s: u32, weather: &str) -> (Play, GuardId, [f64; 2]) {
    let mut p = Play::mission("office.json", &format!("?levelFile=office.json&roomclear&squad4&weather={weather}"), &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    let h = p.hostiles()[0];
    p.drop_all_but(&[h]);
    p.fog_up();
    let post = [27.0, 2.0 + nudge(s, 0)];
    (p, h, post)
}
/// …some frames of it: Sam and his men held where they are, but for one if said
fn held_there(p: &mut Play, h: GuardId, post: [f64; 2], forward: bool, frames: usize) {
    let mates = p.mates();
    for _ in 0..frames {
        p.sam_put([2.0, 2.0]);
        for (k, &m) in mates.iter().enumerate() { p.man_stands(m, if k == 0 && forward { [post[0] - 10.0, 2.0] } else { [1.5 + 0.5 * k as f64, 3.0] }); }
        if p.up(h) { p.pin_man(h, post, EAST, GuardState::Patrol, 0.0); }
        p.frame();
    }
}

/// THE RULE: the player is shown whom his team can see, and the weather's fog stands in the way of both alike. Twenty-five metres off in fog a man no eye of the team's gets through to is
/// not in the picture; a teammate ten metres from him shows him; and so with a body. CONTROL: in clear weather all of them see him, and he is shown.
/// A FAILURE MEANS: the player is shown a man his own squad cannot see, so it looks dumb for not shooting; or the fog hides a man somebody has.
#[test]
fn the_weathers_fog_hides_whom_no_eye_of_the_team_gets_through_to() {
    let sees = |p: &mut Play, h: GuardId| { let c = p.g.gd[h].ch; p.mates().iter().filter(|&&m| { let o = p.g.gd[m].ch; game::roomfight::pair_smoke(&mut p.g, o, c) >= 0.3 }).count() };
    for s in SEEDS {
        let (mut p, h, post) = across_the_lot(s, "clear");
        held_there(&mut p, h, post, false, 40);
        assert!(sees(&mut p, h) == 4 && !p.fogged(h), "seed {s}: in clear weather he is seen by {} of four, hidden: {}", sees(&mut p, h), p.fogged(h));
        drop(p);

        let (mut p, h, post) = across_the_lot(s, "fog");
        held_there(&mut p, h, post, false, 40);
        assert!(sees(&mut p, h) == 0 && p.fogged(h), "seed {s}: in fog he is seen by {} of four, hidden: {}", sees(&mut p, h), p.fogged(h));
        held_there(&mut p, h, post, true, 15);
        assert!(sees(&mut p, h) == 1 && !p.fogged(h), "seed {s}: a teammate at ten metres: seen by {}, hidden: {}", sees(&mut p, h), p.fogged(h));
        held_there(&mut p, h, post, false, 60);
        assert!(p.fogged(h), "seed {s}: the teammate back, and he is still in the picture");
        p.falls(h, game::game::DownBy::World);
        held_there(&mut p, h, post, false, 60);
        assert!(p.fogged(h), "seed {s}: his body is in the picture, and nobody has seen it");
        held_there(&mut p, h, post, true, 15);
        assert!(!p.fogged(h), "seed {s}: a teammate at ten metres from his body, and it is hidden");
    }
}

/// THE RULE: the cone is as wide as its row says: a man 10 degrees inside the edge is seen, one 10 degrees outside it is hidden.
/// A FAILURE MEANS: Hard's fog hides a man the player is looking at, or shows one he is not.
#[test]
fn the_cone_is_as_wide_as_its_row_says() {
    for s in SEEDS {
        for side in [1.0, -1.0] {
            let (mut p, h, post) = shut_in(s);
            p.sam_takes_his_rifle();
            let stand = [12.25, 13.25];
            p.sam_put(stand);
            p.g.k.fog.cone = true;
            let (d, half) = ([post[0] - stand[0], post[1] - stand[1]], p.g.k.fog.cone_deg / 2.0);
            for (off, seen) in [(half - 10.0, true), (half + 10.0, false)] {
                let a = (side * off).to_radians();
                p.point_at([stand[0] + d[0] * a.cos() - d[1] * a.sin(), stand[1] + d[0] * a.sin() + d[1] * a.cos()]);
                pinned(&mut p, h, post, 90, |_| {});
                assert_eq!(!p.fogged(h), seen, "seed {s}: {off} degrees off where Sam looks, the cone {half} each way");
            }
        }
    }
}
