//! THE TEAM AT A DOOR: who goes first, where a stack stands, how four go by the doors Sam took.
mod doors_ask;
use doors_ask::*;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

fn two(seed: u32) -> Opts { Opts { seed, squad: 2, ..Default::default() } }
fn four(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
fn stood(seed: u32, o: Opts) -> (Play, DoorId, [f64; 2]) {
    let (s, at) = yard(seed);
    let mut p = Play::scene(&s, &o);
    p.sam_unhurt();
    p.run(0.2);
    let d = p.door("west-yard");
    (p, d, at)
}
fn hush(p: &mut Play) { let keep = p.defender_at([24.25, 12.25]); p.drop_all_but(&[keep]); p.pin_man(keep, [24.25, 12.25], 0.0, game::game::GuardState::Patrol, 0.0); }

/// One man of the pair has stood looking in at the open door, the other down the wall, and then, if `set`, both have stacked on it. The entry order: who the order says goes first, who came over first, and what each had seen.
fn lead_run(s: u32, looker: usize, set: bool) -> (GuardId, Option<GuardId>, String) {
    let (mut p, d, at) = stood(s, two(s));
    hush(&mut p);
    let men = p.men();
    let (a, b) = (men[looker], men[1 - looker]);
    let wall = [at[0] + 6.0, at[1] + 1.0];   // (off along the wall: the opening shows him next to nothing)
    p.man_stands(b, wall);
    p.order_floor("move", wall, Who::Num(1.0 - looker as f64));
    p.leaf_put_open(d, 1.5);
    p.order("move", [at[0], at[1] + 2.2], Who::Num(looker as f64));
    p.until(12.0, |p| p.arrived(a) && p.arrived(b)).unwrap_or_else(|| panic!("seed {s}: not on their marks: {}", p.roll()));
    p.run(1.5);
    let (sa, sb) = (p.seen_of_room(a, [at[0], at[1] - 2.0]), p.seen_of_room(b, [at[0], at[1] - 2.0]));
    assert!(sa >= sb + p.tools_k().lead_seen_min.max(1.0), "seed {s}: the staging: he has seen {sa} cells, the other {sb}");
    if set {
        p.order("stack", at, Who::All);
        p.until(15.0, |p| p.arrived(a) && p.arrived(b) && p.speed(a) < 0.05 && p.speed(b) < 0.05).unwrap_or_else(|| panic!("seed {s}: never set: {}", p.roll()));
    }
    p.order("openClear", at, Who::All);
    p.until(3.0, |p| !p.ops_on(d).is_empty());
    let op = p.ops_on(d)[0];
    p.until(30.0, |p| p.first_over(op).is_some()).unwrap_or_else(|| panic!("seed {s}: nobody went in: {:?}", p.op_beats(op)));
    (a, p.first_over(op), format!("{sa} cells against {sb}, the order's lead {:?}: {:?}", p.op_lead(op), p.op_beats(op)))
}

/// THE RULE: "i'm going to lead with this man because he's already seen most of the room". With the door standing open and the pair set on it, the man who stood looking in goes over the
/// line first, whichever of the two the order calls its lead. The control is the door shut until the order: nobody has seen anything, and the order's lead goes first.
/// A FAILURE MEANS: the first man is picked by his place in the order and not by what he has seen (`door_tools_seen_most`).
#[test]
fn the_man_who_has_seen_most_of_the_room_goes_in_first() {
    for s in SEEDS {
        let looker = (s % 2) as usize;   // (each seed another man of the pair, so the order's lead is sometimes he and sometimes not)
        let (a, first, why) = lead_run(s, looker, true);
        assert_eq!(first, Some(a), "seed {s}: the man who looked in was to go first: {why}");
        let (mut p, d, at) = stood(s, two(s));
        hush(&mut p);
        p.order("openClear", at, Who::All);
        p.until(3.0, |p| !p.ops_on(d).is_empty());
        let op = p.ops_on(d)[0];
        p.until(30.0, |p| p.first_over(op).is_some()).unwrap_or_else(|| panic!("seed {s}: the control: nobody went in"));
        assert_eq!(p.first_over(op), Some(p.op_lead(op)), "seed {s}: the control: nobody had seen anything, and somebody else led: {:?}", p.op_beats(op));
    }
}

/// THE RULE as above, with the order given while the pair is still walking up: he whom the game names goes first, though the other is nearer the frame and moving.
/// A FAILURE MEANS: at the frame the nearer of two men of one entry goes first again, whoever was named (before: the man named gave way at the jamb and came over 0.75 s behind).
#[test]
fn the_man_named_first_goes_first_when_the_order_comes_on_the_walk() {
    for s in SEEDS {
        for looker in [0, 1] {
            let (a, first, why) = lead_run(s, looker, false);
            assert_eq!(first, Some(a), "seed {s}: {why}");
        }
    }
}

// ---------------------------------------------------------------- off the line of his fire

/// Sam on the yard, firing east along z = `lane`, a stride or more back from where a teammate crosses it on an order. The second man of the pair is parked out of it.
/// Run for `fire` seconds of a round every 0.4 s (none if `fire` is 0), then until he is on his mark: rounds fired, frames in the lane under fire, longest stood, the speed he reached, whether he got there, frames he was behind Sam's back, how long it took.
struct Crossing { rounds: f64, in_lane: i32, stood: f64, fastest: f64, there: Option<f64>, behind: i32, there_under_fire: bool, moved_after: Option<f64> }
fn crossing(s: u32, back: f64, fire: f64) -> Crossing {
    let (lane, x) = (21.0 + nudge(s, 50), 19.5 + nudge(s, 51));
    let (sc, _) = yard(s);
    let mut p = Play::scene(&sc, &two(s));
    p.sam_unhurt();
    hush(&mut p);
    let far = p.defender_at([24.25, 12.25]);
    p.drop_all_but(&[far]);
    let (from, to) = ([x, lane + 3.5], [x, lane - 3.5]);
    let sam = [x - back, lane];
    let (m, other) = (p.men()[0], p.men()[1]);
    p.order("move", from, Who::Num(0.0));
    p.order("move", [8.0, 25.0], Who::Num(1.0));
    p.sam_put(sam);
    p.until(14.0, |p| p.arrived(m) && p.arrived(other)).unwrap_or_else(|| panic!("seed {s}: not at the start: {}", p.roll()));
    p.sam_takes_his_rifle();
    p.point_at([x + 12.0, lane]);
    p.run(0.6);
    let (t0, r0) = (p.time(), p.sam_rounds());
    p.order("move", to, Who::Num(0.0));
    let mut c = Crossing { rounds: 0.0, in_lane: 0, stood: 0.0, fastest: 0.0, there: None, behind: 0, there_under_fire: false, moved_after: None };
    let (mut still, mut f, mut last_round) = (0.0, 0, -9.0);
    while p.time() - t0 < fire + 14.0 {
        let now = p.time() - t0;
        p.sam_put(sam);
        if now < fire && f % 24 == 0 { p.sam_clicks(); }
        f += 1;
        let was = p.sam_rounds();
        p.frame();
        p.release(game_start::play::Button::Left);
        if p.sam_rounds() > was { last_round = now; }
        let firing = p.sam_rounds() > r0 && now - last_round < p.fire_lasts();
        let (at, v) = (p.at(m), p.speed(m));
        c.fastest = c.fastest.max(v);
        let (along, side) = (at[0] - sam[0], (at[1] - lane).abs());
        if firing {
            if along > 0.15 && side <= p.lane_half(along) { c.in_lane += 1; }
            if v < 0.1 { still += 1.0 / 60.0; c.stood = c.stood.max(still); } else { still = 0.0; }
            if along < -0.3 { c.behind += 1; }
        }
        if now >= fire && c.moved_after.is_none() && v > 0.5 { c.moved_after = Some(now - fire); }
        if c.there.is_none() && dist(at, to) < 0.5 { c.there = Some(now); c.there_under_fire = firing; }
        if c.there.is_some() && now > fire + 1.5 { break; }
    }
    c.rounds = p.sam_rounds() - r0;
    c
}

/// THE RULE: a teammate walking on an order waits short of the lane the player is firing down, stands there for as long as the fire lasts, is never in it, and walks on inside a second and a bit when it stops; he never runs. The control is
/// the same order with Sam's finger off the trigger: he walks straight over.
/// A FAILURE MEANS: Sam's own men walk through his fire on an order, or wait for good.
#[test]
fn a_teammate_waits_short_of_the_lane_and_crosses_when_the_fire_stops() {
    for s in SEEDS {
        let c = crossing(s, 6.5, 7.0);
        assert!(c.rounds >= 12.0, "seed {s}: {} rounds", c.rounds);
        assert_eq!(c.in_lane, 0, "seed {s}: frames in the lane under fire");
        assert!(c.stood >= 4.0, "seed {s}: he stood {:.2} s", c.stood);
        assert!(c.there.is_some() && !c.there_under_fire, "seed {s}: on his mark after the fire: {:?}", c.there);
        assert!(c.moved_after.is_some_and(|t| t < 1.6), "seed {s}: walking again {:?} s after the last round", c.moved_after);
        assert!(c.fastest < 2.6, "seed {s}: he ran: {:.2} m/s", c.fastest);
        let q = crossing(s, 6.5, 0.0);
        assert_eq!(q.rounds, 0.0);
        assert!(q.there.is_some_and(|t| t < 7.0), "seed {s}: the control: with no fire he walks straight over: {:?}", q.there);
    }
}

/// THE RULE: with Sam a stride back from the crossing the way round behind him is worth the walk: after the wait's ceiling he walks round behind Sam, and is on his mark with the fire still going, never in the lane. (The control is the test above: 6.5 m back it is not.)
/// A FAILURE MEANS: a man waits for ever where a short way round stands open, or the way round is a way through the fire.
#[test]
fn with_sam_a_stride_back_he_goes_round_behind_him() {
    for s in SEEDS {
        let c = crossing(s, 1.2, 11.5);
        assert_eq!(c.in_lane, 0, "seed {s}: frames in the lane under fire");
        assert!(c.behind > 30, "seed {s}: frames behind Sam's back: {}", c.behind);
        assert!(c.there_under_fire, "seed {s}: on his mark with the fire still going: {:?}", c.there);
    }
}

// ---------------------------------------------------------------- four at a door, and fallen in

/// THE RULE: stack up with four: each on a mark of his own, a pair a jamb, nobody in the opening, everybody on Sam's side, 'set.' called once. A pair sent to the door the other pair holds files in behind them, no two marks the same. The control is three men: one down, they still stand a body's room apart.
/// A FAILURE MEANS: two men are dealt one place, or a stack stands in the door it is on.
#[test]
fn four_mark_a_door_in_two_files() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, four(s));
        hush(&mut p);
        let (red, blue) = (p.red(), p.blue());
        let men: Vec<GuardId> = red.iter().chain(blue.iter()).copied().collect();
        p.order("stack", at, Who::All);
        p.until(25.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: not all on their marks: {}", p.roll()));
        p.run(1.5);
        let w = p.door_w(d);
        let hi = |p: &Play, m| p.along(d, p.at(m)) > w / 2.0;
        assert!(hi(&p, red[0]) == hi(&p, red[1]) && hi(&p, blue[0]) == hi(&p, blue[1]) && hi(&p, red[0]) != hi(&p, blue[0]), "seed {s}: red on one jamb, blue on the other: {}", p.roll());
        let side = p.face(d, p.sam_at());
        for &m in &men {
            assert!(dist(p.at(m), p.post_of(m).unwrap()) < 0.4, "seed {s}: on his own mark");
            assert!(!p.in_doorway(d, p.at(m)) && p.off(d, p.at(m), side) > 0.0, "seed {s}: nobody in the opening, everybody on Sam's side: {:?}", p.at(m));
            for &o in &men { if o != m { assert!(dist(p.at(m), p.at(o)) > 0.7, "seed {s}: two men {:.2} m apart", dist(p.at(m), p.at(o))); } }
        }
        assert_eq!(p.said.iter().filter(|l| l.text == "set.").count(), 1, "seed {s}: 'set.' is called once");
        // one down: fallen in and sent again
        p.falls(men[3], game::game::DownBy::Player);
        p.order("fallIn", p.sam_at(), Who::All);
        p.run(2.5);
        p.order("stack", at, Who::All);
        let left = p.men();
        p.until(20.0, |p| left.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: the three not set: {}", p.roll()));
        for &m in &left { for &o in &left { if o != m { assert!(dist(p.at(m), p.at(o)) > 0.7, "seed {s}: three men, {:.2} m apart", dist(p.at(m), p.at(o))); } } }
    }
}

#[test]
fn a_pair_sent_to_a_door_the_other_pair_holds_files_in_behind_them() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, four(s));
        hush(&mut p);
        let (red, blue) = (p.red(), p.blue());
        p.order("stack", at, Who::Red);
        p.until(20.0, |p| red.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: red not set"));
        p.order("stack", at, Who::Blue);
        p.until(20.0, |p| blue.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: blue not set"));
        p.run(1.0);
        let men: Vec<GuardId> = red.iter().chain(blue.iter()).copied().collect();
        for &m in &men {
            assert!(!p.in_doorway(d, p.at(m)), "seed {s}: nobody in the opening");
            for &o in &men { if o != m { assert!(dist(p.post_of(m).unwrap(), p.post_of(o).unwrap()) > 0.7, "seed {s}: two marks {:.2} m apart", dist(p.post_of(m).unwrap(), p.post_of(o).unwrap())); assert!(dist(p.at(m), p.at(o)) > 0.6, "seed {s}"); } }
        }
    }
}

/// Sam walks at a stride along `legs`, by the sandbox's own teleport, a frame at a time; `each` is asked after every frame.
fn sam_walks(p: &mut Play, legs: &[[f64; 2]], mut each: impl FnMut(&mut Play)) {
    const SPEED: f64 = 1.55;
    let mut at = p.sam_at();
    for &to in legs {
        while dist(at, to) > 0.03 {
            let k = (SPEED / 60.0 / dist(at, to)).min(1.0);
            at = [at[0] + (to[0] - at[0]) * k, at[1] + (to[1] - at[1]) * k];
            p.sam_put(at);
            p.frame();
            each(p);
        }
    }
}

/// THE RULE: fall in keeps all four with Sam, each pair together in one file, through two doors and across a room: nobody left behind, nobody in a doorway at the end, never two men in one place. The control is 'hold': the same walk leaves them where they stand.
/// A FAILURE MEANS: a door or a turn sheds a man from the file.
#[test]
fn fall_in_keeps_all_four_by_the_doors_sam_took() {
    for s in SEEDS {
        let (mut p, _, at) = stood(s, four(s));
        hush(&mut p);
        let (red, blue) = (p.red(), p.blue());
        let men: Vec<GuardId> = red.iter().chain(blue.iter()).copied().collect();
        p.order("fallIn", p.sam_at(), Who::All);
        p.until(14.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: never fell in: {}", p.roll()));
        p.run(1.5);
        let mut file = men.clone();
        file.sort_by(|a, b| p.file_place(*a).partial_cmp(&p.file_place(*b)).unwrap());
        assert!(red.contains(&file[0]) == red.contains(&file[1]) && red.contains(&file[2]) == red.contains(&file[3]), "seed {s}: each pair together in the file");
        for &m in &men { assert!(dist(p.at(m), p.sam_at()) < 5.8, "seed {s}: closed up: {:.2} m", dist(p.at(m), p.sam_at())); }
        let (west, between) = (p.door("west-yard"), p.door("between"));
        let legs = [[at[0], at[1] + 2.0], [at[0], at[1] - 2.0], [15.0, 13.0], [20.5, 13.0], [24.5, 13.0]];
        let (mut worst, mut closest) = (0.0f64, 9.0f64);
        let men2 = men.clone();
        sam_walks(&mut p, &legs, |p| {
            for &m in &men2 { worst = worst.max(dist(p.at(m), p.sam_at())); for &o in &men2 { if o != m { closest = closest.min(dist(p.at(m), p.at(o))); } } }
        });
        let t = p.until(12.0, |p| men2.iter().all(|&m| p.arrived(m)));
        p.run(2.0);
        assert!(t.is_some(), "seed {s}: they never closed up: {}", p.roll());
        assert!(worst < 9.0, "seed {s}: the furthest any man fell back: {worst:.2} m");
        assert!(closest > 0.5, "seed {s}: never two men in one place: {closest:.2} m");
        for &m in &men {
            assert!(dist(p.at(m), p.sam_at()) < 5.8 && p.at(m)[0] > 18.0, "seed {s}: he ends with Sam, past both doors: {:.2} m, {:?}", dist(p.at(m), p.sam_at()), p.at(m));
            assert!(!p.in_doorway(west, p.at(m)) && !p.in_doorway(between, p.at(m)), "seed {s}: nobody is left in a doorway");
        }
        drop(p);

        let (mut p, _, at) = stood(s, four(s));
        hush(&mut p);
        p.order("hold", p.sam_at(), Who::All);
        let men = p.men();
        p.run(0.5);
        let was: Vec<_> = men.iter().map(|&m| p.at(m)).collect();
        sam_walks(&mut p, &[[at[0], at[1] + 2.0], [at[0], at[1] - 2.0]], |_| {});
        for (i, &m) in men.iter().enumerate() { assert!(dist(p.at(m), was[i]) < 1.0, "seed {s}: the control: on hold he stayed: {:.2} m", dist(p.at(m), was[i])); }
    }
}

/// A short room, its yard door at its west end and its door to the next room at its north-east corner; the next room's yard door just past their wall, so that from the first door the yard is the short walk to the next room's south end.
fn two_ways(seed: u32) -> (Play, [f64; 2], [f64; 2], [f64; 2]) {
    let mut sc = Scene::new("two-ways");
    sc.room("west", 14.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("west-yard", Axis::X, 16.0, 14.3).door("east-yard", Axis::X, 16.0, 18.4).door("between", Axis::Z, 18.0, 10.4);
    sc.bulb(16.0, 13.0).bulb(22.0, 13.0).moon();
    let a = sc.door_centre("west-yard");
    sc.team([a[0] + nudge(seed, 52), 21.0], NORTH, &[[a[0] - 1.0, 22.0], [a[0] + 1.0, 22.0]]);
    sc.defender("west", [15.25, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let (b, c) = (sc.door_centre("between"), sc.door_centre("east-yard"));
    let mut p = Play::scene(&sc, &two(seed));
    p.sam_unhurt();
    p.run(0.2);
    hush(&mut p);
    (p, a, b, c)
}

/// THE RULE: a follower goes by the doors Sam took. Held at the door Sam went in by until Sam is well into the next room, and let go, he walks Sam's trail through the first room, though the yard's door is the shorter walk to his place. The control is a 'move there' from the same place to the same place: it takes the short way.
/// A FAILURE MEANS: a follower who lost his turn at a door walks round the room the file went through, apart from his partner.
#[test]
fn a_follower_goes_by_the_doors_sam_took() {
    let in_west = |a: [f64; 2]| a[0] > 14.0 && a[0] < 18.0 && a[1] > 10.0 && a[1] < 16.0;
    for s in SEEDS {
        let (mut p, a, b, _) = two_ways(s);
        let dest = [20.5, 14.5];
        p.order("fallIn", p.sam_at(), Who::All);
        let men = p.men();
        p.until(10.0, |p| men.iter().all(|&m| p.arrived(m)));
        sam_walks(&mut p, &[[a[0], a[1] + 2.5]], |_| {});
        let (lead, pin) = (men[0], [a[0], a[1] + 0.9]);
        sam_walks(&mut p, &[[a[0], a[1] - 1.0], [b[0] - 0.6, b[1]], [b[0] + 1.6, b[1]], dest], |p| { p.man_stands(lead, pin); });
        let mut inside = 0;
        p.until(11.0, |p| { if in_west(p.at(lead)) { inside += 1; } p.sam_put(dest); dist(p.at(lead), dest) < 5.5 && !in_west(p.at(lead)) });
        assert!(inside > 30, "seed {s}: he was {:.1} s in the room Sam went through, and is at {:?}", inside as f64 / 60.0, p.at(lead));
        assert!(dist(p.at(lead), dest) < 5.5, "seed {s}: he ends with Sam: {:.1} m", dist(p.at(lead), dest));
        drop(p);

        let (mut p, a, _, _) = two_ways(s);
        let m = p.men()[0];
        p.order("move", [a[0], a[1] + 1.0], Who::Num(0.0));
        p.until(10.0, |p| p.arrived(m)).unwrap_or_else(|| panic!("seed {s}: the control's start"));
        p.order("move", dest, Who::Num(0.0));
        let mut inside = 0;
        p.until(14.0, |p| { if in_west(p.at(m)) { inside += 1; } p.arrived(m) }).unwrap_or_else(|| panic!("seed {s}: the control never got there: {:?}", p.at(m)));
        assert_eq!(inside, 0, "seed {s}: the control: the short way is the yard's door");
    }
}

// ---------------------------------------------------------------- which face, and which way round

/// The team in the west room with its door to the east room standing open, Sam in the yard at the east room's yard door, the east room's man off the floor as is every other. `known`: somebody has stood in the east room.
fn taken_west(seed: u32, known: bool) -> (Play, DoorId, [f64; 2]) {
    let mut sc = Scene::new("taken-west");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    sc.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4 + nudge(seed, 53)).door("between", Axis::Z, 18.0, 12.4);
    sc.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    let c = sc.door_centre("east-yard");
    sc.team([c[0], c[1] + 2.0], NORTH, &[[15.0, 12.0], [16.5, 12.0], [15.0, 14.0], [16.5, 14.0]]);
    sc.defender("west", [11.25, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    let mut p = Play::scene(&sc, &four(seed));
    p.sam_unhurt();
    p.run(0.2);
    p.drop_all_but(&[]);
    let (b, w, d) = (p.door("between"), p.door("west-yard"), p.door("east-yard"));
    p.leaf_put_open(b, 1.5);
    p.leaf_stays(b);
    p.leaf_put_open(w, 1.5);
    p.leaf_stays(w);
    if known {
        let m = p.men()[0];
        for _ in 0..30 { p.man_stands(m, [22.0, 13.0]); p.frame(); }
        p.man_stands(m, [16.5, 12.0]);
    }
    p.run(0.3);
    (p, d, c)
}

/// THE RULE: "they'll go through uncleared rooms to get there!". The squad's stack on an ordered door is dealt the face whose floor the team knows, whatever the walks come to: on the yard's side, nobody setting foot in the room nobody has been in. The control is that room known: the walks decide, and its own side is the short one.
/// A FAILURE MEANS: a stack walks through the room it is about to clear to stand on the door's far side.
#[test]
fn a_stack_is_dealt_the_face_whose_floor_is_known() {
    let in_east = |a: [f64; 2]| a[0] > 18.0 && a[0] < 26.0 && a[1] > 10.0 && a[1] < 16.0;
    for s in SEEDS {
        let (mut p, _, c) = taken_west(s, false);
        let men = p.men();
        p.order("stack", c, Who::All);
        let mut inside = 0;
        p.until(40.0, |p| { for &m in &men { if in_east(p.at(m)) { inside += 1; } } men.iter().all(|&m| p.arrived(m)) }).unwrap_or_else(|| panic!("seed {s}: not all set: {}", p.roll()));
        assert_eq!(inside, 0, "seed {s}: man-frames in the room nobody has been in");
        for &m in &men { assert!(p.at(m)[1] > 16.0, "seed {s}: on the yard's side: {:?}", p.at(m)); }
        drop(p);

        let (mut p, _, c) = taken_west(s, true);
        let men = p.men();
        p.order("stack", c, Who::All);
        p.until(40.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: the control: not all set: {}", p.roll()));
        assert!(men.iter().all(|&m| p.at(m)[1] < 16.0), "seed {s}: the control: the room known, the walks decide, and they stack on its side: {}", p.roll());
    }
}

/// THE RULE: a door face with no floor at either jamb and no marks authored for it is no reason to refuse: the men work out marks of their own, on free floor, off the line of the opening, a body's room apart. (The office's keypad door from the server room: racks down one side of the aisle, a cart on the hinge jamb, the corner on the latch's.) The control is that door's premise: neither jamb has floor.
/// A FAILURE MEANS: every door order on such a face is refused again ('no room on that door.').
#[test]
fn a_door_with_no_floor_at_its_jambs_gets_marks_of_the_mens_own() {
    for s in SEEDS {
        for squad in [2, 4] {
            let mut p = Play::mission("office.json", &format!("?levelFile=office.json&roomclear&squad{squad}"), &Opts { seed: s, squad, ..Default::default() });
            p.sam_unhurt();
            p.run(0.2);
            p.drop_all_but(&[]);
            let d = p.door("server_manager");
            let mut spots = vec![[18.6, 8.6], [17.8, 8.2], [17.8, 9.0], [17.0, 8.2], [17.0, 9.0]].into_iter();
            p.sam_put(spots.next().unwrap());
            for m in p.men() { p.man_stands(m, spots.next().unwrap()); }
            p.run(0.2);
            let side = p.face(d, p.sam_at());
            for along in [-0.42, p.door_w(d) + 0.42] { let at = p.on_face(d, along, 0.45 * side); assert!(!p.floor_free(at), "seed {s}: the premise: this jamb has floor at {at:?}"); }
            let c = p.frame_at(d);
            assert_eq!(p.row("stack", c).1, None, "seed {s}, {squad} men: the row is grey");
            p.order("stack", c, Who::All);
            let men = p.men();
            p.until(25.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}, {squad} men: not on their marks: {}", p.roll()));
            p.run(1.0);
            for &m in &men {
                assert!(dist(p.at(m), p.post_of(m).unwrap()) < 0.3 && !p.in_doorway(d, p.at(m)), "seed {s}, {squad} men: {:?}", p.at(m));
                assert!(p.floor_free(p.post_of(m).unwrap()), "seed {s}: his mark is on free floor");
                for &o in &men { if o != m { assert!(dist(p.at(m), p.at(o)) > 0.6, "seed {s}, {squad} men: {:.2} m apart", dist(p.at(m), p.at(o))); } }
            }
        }
    }
}

/// The queue lesson's level: a pair side by side at the front of the hall told to stack on its back door, `gap` metres between them. The furthest they get from each other on the way, and how many got there.
fn round_the_hall(s: u32, gap: f64) -> (f64, usize) {
    let mut p = Play::mission("tut-queue.json", "?levelFile=tut-queue.json&roomclear=1&squad4", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    p.drop_all_but(&[]);
    p.sam_put([30.8, 20.6]);
    let (two, rest) = (p.blue(), p.red());
    for (i, &m) in rest.iter().enumerate() { p.man_stands(m, [16.0 + i as f64 * 1.2, 9.0]); }
    let (x, z) = (30.1, 8.0 + nudge(s, 54).abs());
    p.man_stands(two[0], [x, z]);
    p.man_stands(two[1], [x + gap, z]);
    p.order("stack", [30.8, 19.0], Who::Blue);
    let mut far = 0.0f64;
    p.until(50.0, |p| { far = far.max(dist(p.at(two[0]), p.at(two[1]))); p.arrived(two[0]) && p.arrived(two[1]) });
    (far, two.iter().filter(|&&m| p.arrived(m)).count())
}

/// THE RULE: two men of a pair handed one order on one door from one spot walk round a building by the same side, and both stand on their marks. The control is the pair stood 12 m apart when told: then the two are apart from the first frame, as a man round each side would be.
/// A FAILURE MEANS: the pair's two ways part round the hall, a building between them.
#[test]
fn a_pair_goes_round_a_building_one_way() {
    for s in SEEDS {
        let (far, there) = round_the_hall(s, 1.4);
        assert_eq!(there, 2, "seed {s}: both on their marks");
        assert!(far <= 6.0, "seed {s}: the furthest the two got from each other: {far:.1} m");
        let (far, _) = round_the_hall(s, 12.0);
        assert!(far > 6.0, "seed {s}: the control: {far:.1} m");
    }
}
