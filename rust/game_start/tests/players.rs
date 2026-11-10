//! TWO PLAYERS OF ONE GAME: each is a player of his own (his man, his hands, his pair), the team is one team, and a fight of the two is the same fight on both machines.
use game::ids::*;
use game::orders::Who;
use game::teams::Team;
use game_start::play::{nudge, parting, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [5, 6, 7];

fn man(p: &Play, seat: usize) -> PlayerId { game::cmd::seat_man(&p.g, p.g.seats[seat]) }
fn body(p: &Play, seat: usize) -> CharId { p.g.pl[man(p, seat)].ch }
fn at(p: &Play, seat: usize) -> [f64; 3] { p.g.ch[body(p, seat)].pos }
fn pair(p: &Play, seat: usize) -> Vec<GuardId> { game::orders::squad_orders_squad(&p.g, &game::seats::seat(&p.g, p.g.seats[seat])) }
fn orders(p: &Play, seat: usize) -> Vec<Option<OrdersAssignmentId>> { pair(p, seat).iter().map(|&m| game::orders::squad_orders_of(&game::seats::seat(&p.g, p.g.seats[seat]), m)).collect() }
/// a player's keys and trigger as he holds them, his view on his man, his cursor on a place of the world
fn hands(p: &mut Play, seat: usize, keys: &[&str], on: Option<[f64; 3]>, fire: bool) {
    let s = p.g.seats[seat];
    let (input, cam, canvas) = (p.g.tab.cmd_seat[s].input, p.g.tab.cmd_seat[s].cam, p.host.canvas);
    let follow = at(p, seat);
    game::camera::follow_camera_update(&mut p.g, cam, 1.0 / 60.0, follow, None);
    let mouse = on.map(|w| game::camera::follow_camera_project(&p.g, cam, w, canvas.client_width, canvas.client_height, None, None)).filter(|m| m.2);
    let i = &mut p.g.tab.input[input];
    for k in ["KeyW", "KeyA", "KeyS", "KeyD"] { i.keys.delete(String::from(k)); }
    for k in keys { i.keys.add(String::from(*k)); }
    i.buttons = if fire { 1.0 } else { 0.0 };
    if let Some((x, y, _)) = mouse { (i.mouse_x, i.mouse_y) = (x, y); }
}
/// a frame on both players' hands, on a machine that shows the seat `shown`
fn frame(p: &mut Play, shown: usize) {
    p.net_frame(shown);
    for s in p.g.seats.clone() { let i = p.g.tab.cmd_seat[s].input; game::input::input_end_frame(&mut p.g, i); }
}

/// A store with a yard south of it and a second building down the yard, a defender in each, the team of two players in the yard.
fn store(seed: u32) -> Scene {
    let mut s = Scene::new("store");
    s.room("store", 10.0, 8.0, 18.0, 14.0).room("far", 26.0, 8.0, 34.0, 14.0);
    s.door("store-yard", Axis::X, 14.0, 13.4 + nudge(seed, 0)).door("far-yard", Axis::X, 14.0, 29.4).door("far-back", Axis::X, 8.0, 29.4);
    s.bulb(14.0, 11.0).bulb(30.0, 11.0).moon();
    s.team([14.0, 21.0], std::f64::consts::PI, &[[13.0, 22.0 + nudge(seed, 1)], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("far", [32.25, 10.25], "far-yard").defender("store", [17.25, 9.25], "store-yard");
    s
}

/// THE RULE: in a game of two each player walks by his own record, his order reaches his own two men and nobody else's, and the fog of war's eyes are both players' and all four men's.
/// A FAILURE MEANS: one player's hands, book or sight is the other's.
#[test]
fn each_player_is_a_player_of_his_own() {
    for seed in SEEDS {
        let mut p = Play::scene(&store(seed), &Opts { seed, seats: 2, ..Default::default() });
        p.g.fog_of_war.on = true;
        for _ in 0..30 { frame(&mut p, 0); }
        let (mine, his) = (pair(&p, 0), pair(&p, 1));
        assert!(body(&p, 0) != body(&p, 1) && mine.len() == 2 && his.len() == 2 && mine.iter().all(|m| !his.contains(m)), "seed {seed}: two men, and the four dealt two a player: {mine:?} and {his:?}");
        let from = [at(&p, 0), at(&p, 1)];
        for _ in 0..60 { hands(&mut p, 0, &["KeyD"], None, false); hands(&mut p, 1, &["KeyA"], None, false); frame(&mut p, 0); }
        let way: Vec<[f64; 2]> = (0..2).map(|i| { let to = at(&p, i); [to[0] - from[i][0], to[2] - from[i][2]] }).collect();
        assert!(way.iter().all(|w| w[0].hypot(w[1]) > 1.0) && way[0][0] * way[1][0] + way[0][1] * way[1][1] < 0.0, "seed {seed}: one walks right and one left, each by his own keys: {way:?}");
        for (giver, other) in [(1, 0), (0, 1)] {
            let (was, others) = (orders(&p, giver), orders(&p, other));
            let a = at(&p, giver);
            p.seat_orders(giver, "move", [a[0], 18.0], Who::All);
            for _ in 0..6 { hands(&mut p, 0, &[], None, false); hands(&mut p, 1, &[], None, false); frame(&mut p, 0); }
            assert!(orders(&p, giver).iter().zip(&was).all(|(n, w)| n.is_some() && n != w), "seed {seed}: seat {giver}'s order reaches his own two");
            assert_eq!(orders(&p, other), others, "seed {seed}: …and nobody of seat {other}'s");
        }
        let eyes: Vec<f64> = p.g.fog_of_war.eyes.iter().map(|e| e.id).collect();
        for c in [body(&p, 0), body(&p, 1)].into_iter().chain(mine.iter().chain(&his).map(|&m| p.g.gd[m].ch)) { assert!(eyes.contains(&p.g.ch[c].id), "seed {seed}: the team's eyes are {eyes:?}, without #{}", p.g.ch[c].id); }
    }
}

/// THE RULE: every man of the team is found in the dark on both players' screens, by the same hue on both: the first player's by one, the second's by another, a teammate's by his pair's. A player who
/// is down gives his place up on the other's screen; with both down nobody is found.
#[test]
fn a_mans_hue_is_the_same_on_both_screens() {
    for seed in SEEDS {
        let mut p = Play::scene(&store(seed), &Opts { seed, seats: 2, ..Default::default() });
        for _ in 0..10 { frame(&mut p, 0); }
        let found = |p: &mut Play, shown: usize| -> Vec<(u32, u32)> { let s = p.g.seats.clone(); game::seats::show_seat(&mut p.g, s[shown]); let mut out = Vec::new(); game_draw::rims::fill(&p.g, &mut out); game::seats::show_seat(&mut p.g, s[0]); out.iter().map(|r| (r.id, r.hue)).collect() };
        let (a, b) = (found(&mut p, 0), found(&mut p, 1));
        let sorted = |v: &[(u32, u32)]| { let mut v = v.to_vec(); v.sort(); v };
        let id = |p: &Play, seat: usize| p.g.ch[body(p, seat)].id as u32;
        assert_eq!(a.len(), 6, "seed {seed}: two players and four men: {a:?}");
        assert_eq!(sorted(&a), sorted(&b), "seed {seed}: the same hues on both screens");
        assert_eq!((a[0], b[0]), ((id(&p, 0), 0), (id(&p, 1), 3)), "seed {seed}: each screen's first is its own man");
        let of = |hue: u32| a.iter().filter(|r| r.1 == hue).count();
        assert_eq!((of(1), of(2)), (2, 2), "seed {seed}: a pair of each hue: {a:?}");
        let second = man(&p, 1);
        p.g.cond[p.g.pl[second].ch].down = true;
        assert!(!found(&mut p, 0).contains(&(id(&p, 1), 3)) && found(&mut p, 0).len() == 5, "seed {seed}: the second player is down");
        assert_eq!(found(&mut p, 1)[0], (id(&p, 1), 3), "seed {seed}: on his own screen he is where he lies");
        let first = man(&p, 0);
        p.g.cond[p.g.pl[first].ch].down = true;
        assert!(found(&mut p, 0).is_empty() && found(&mut p, 1).is_empty(), "seed {seed}: both are down");
    }
}

/// THE RULE: both players shoot it out with the two defenders, one after the other, keys pressed all the while, and the machine that shows one of them holds the same game, in every part of its state,
/// as the machine that shows the other, their views unlike.
/// A FAILURE MEANS: something of a fight is decided by whose picture a machine draws.
#[test]
fn a_fight_of_two_is_one_fight_on_both_machines() {
    const KEYS: [&str; 16] = ["KeyG", "KeyF", "KeyE", "KeyQ", "KeyC", "KeyR", "KeyV", "KeyN", "KeyX", "KeyZ", "Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Space"];
    for seed in SEEDS {
        let play = |shown: usize| -> (Vec<Vec<u32>>, usize, f64) {
            let mut p = Play::scene(&store(seed), &Opts { seed, seats: 2, dress: true, ..Default::default() });
            p.sam_unhurt();
            let theirs = p.g.tab.cmd_seat[p.g.seats[1]].cam;
            game::camera::follow_camera_rotate(&mut p.g, theirs, 1.3);
            let up = |p: &Play| p.g.guards.iter().filter(|&&m| p.g.team(m) == Team::Hostile && !game::condition::down(&p.g, p.g.gd[m].ch)).count();
            let (had, mut rows, mut rounds) = (up(&p), Vec::new(), 0.0f64);
            for k in 0..1500usize {
                if k == 60 || k == 780 { let x = if k == 60 { 11.5 } else { 27.5 }; p.seat_stands(0, [x, 12.5]); p.seat_stands(1, [x, 9.5]); }
                for seat in 0..2 {
                    let a = at(&p, seat);
                    let foe = p.hostiles_by([a[0], a[2]]).first().map(|&h| { let q = p.g.pos_of(h); [q[0], 1.2, q[2]] });
                    let n = k + seat * 37;
                    hands(&mut p, seat, &[["KeyW", "KeyD", "KeyS", "KeyA"][n / 40 % 4]], foe, n % 120 > 60);
                    if n % 90 == 0 { let i = p.g.tab.cmd_seat[p.g.seats[seat]].input; p.g.tab.input[i].pressed.add(String::from(KEYS[n / 90 % KEYS.len()])); }
                }
                frame(&mut p, shown);
                rounds = rounds.max(p.g.tracers.spawned);
                if k % 10 == 9 { rows.push(p.words()); }
            }
            (rows, had - up(&p), rounds)
        };
        let ((mine, fell, rounds), (theirs, ..)) = (play(0), play(1));
        assert!(fell > 0, "seed {seed}: the staging: nobody of theirs went down ({rounds} tracers)");
        if let Some((k, parts)) = parting(&mine, &theirs) { panic!("seed {seed}: the machine that shows the other player parts in {parts} {:.1} s on", (k + 1) as f64 / 6.0); }
    }
}

/// THE RULE: the flash of a gun of anybody on the team, either player's or a teammate's, is the team's and lights the picture whoever sees it; a defender's lights only what the team sees.
#[test]
fn a_flash_of_either_player_is_the_teams() {
    for seed in SEEDS {
        let mut p = Play::scene(&store(seed), &Opts { seed, seats: 2, ..Default::default() });
        p.g.fog_of_war.on = true;
        for _ in 0..10 { frame(&mut p, 0); }
        let foe = p.hostiles()[0];
        let (first, second) = (p.g.seats[0], p.g.seats[1]);
        // (who fires, in whose seat, the class wanted; each at the sky, so that nobody holds his fire for a friend in the line)
        let mut men = vec![(body(&p, 0), first, true, 0.0), (body(&p, 1), second, true, 0.0)];
        for m in p.mates() { men.push((p.g.gd[m].ch, first, false, 0.0)); }
        men.push((p.g.gd[foe].ch, first, false, 1.0));
        for (c, seat, _player, class) in men {
            let (id, at, had) = (p.g.ch[c].id, p.g.ch[c].pos, p.g.engine.lights.lights.clone());
            let o = game::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..game::combat::FireOpts::default() };
            game::seats::with_seat(&mut p.g, seat, &mut |g| game::combat::fire_weapon(g, c, [at[0], at[1] + 30.0, at[2] + 1.0], Some(&o)));
            let of: Vec<f64> = p.g.engine.lights.lights.iter().filter(|l| !had.contains(l) && p.g.tab.light[**l].owner == id).map(|&l| game::render_lights::light_set_fog_class(&p.g, l)).collect();
            assert!(!of.is_empty() && of.iter().all(|&k| k == class), "seed {seed}: the flashes of #{id} are {of:?}, not all {class}");
        }
    }
}

/// THE RULE: a moment of a fight of two is put back whole: the words are the moment's, and the same hands play the same ticks from it, in every part.
#[test]
fn a_moment_of_a_game_of_two_put_back_plays_the_same() {
    for seed in SEEDS {
        let mut p = Play::scene(&store(seed), &Opts { seed, seats: 2, dress: true, ..Default::default() });
        p.sam_unhurt();
        for (seat, z) in [(0, 12.5), (1, 9.5)] { p.seat_stands(seat, [11.5, z]); }
        let play = |p: &mut Play, from: usize, n: usize| -> Vec<Vec<u32>> {
            (from..from + n).map(|k| {
                for seat in 0..2 {
                    let a = at(p, seat);
                    let foe = p.hostiles_by([a[0], a[2]]).first().map(|&h| { let q = p.g.pos_of(h); [q[0], 1.2, q[2]] });
                    hands(p, seat, &[["KeyW", "KeyD", "KeyS", "KeyA"][(k + seat * 37) / 40 % 4]], foe, (k + seat * 11) % 40 > 20);
                    if k == 0 { let i = p.g.tab.cmd_seat[p.g.seats[seat]].input; p.g.tab.input[i].pressed.add(String::from("Digit2")); }
                }
                frame(p, 0);
                p.words()
            }).collect()
        };
        play(&mut p, 0, 100);
        let there = p.words();
        let kept = game::savestate::control::Moment::of(&p.g);
        let once = play(&mut p, 100, 90);
        assert!(once[89] != there && p.g.fx_seed > kept.game.fx_seed, "seed {seed}: the staging: nobody fired");
        p.g = kept.laid(p.g.sim);
        assert_eq!(p.words(), there, "seed {seed}: the moment put back is the moment");
        let again = play(&mut p, 100, 90);
        if let Some((k, parts)) = parting(&once, &again) { panic!("seed {seed}: from the moment put back the same hands part in {parts} at tick {k}"); }
    }
}

/// THE RULE: a mission written round one commander refuses a game of two, in words.
#[test]
fn a_rescue_refuses_two_players() {
    let mut p = Play::level("office.json", false, &Opts { seed: 5, seats: 2, ..Default::default() });
    let why = game::scenario_run::start_scenario(&mut p.g, &game::scene_scenario_list::ScenarioSpec { kind: game::scene_scenarios::MissionKind::Rescue, squad: 2.0, level: None, seed: None });
    assert!(why.is_some_and(|w| !w.is_empty()) && p.g.rescue.result.is_none());
    for _ in 0..30 { frame(&mut p, 0); }
}

/// THE RULE: a restart in a game of two leaves each machine showing its own player, every hand checked as it was, and the one game the same on both.
/// A FAILURE MEANS: the second player's page is on the first player's man from the first restart on, or nobody's hand is checked any more.
#[test]
fn after_a_restart_each_machine_still_shows_its_own() {
    let run = |shown: usize| {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 5, seats: 2, dress: true, ..Default::default() });
        for s in p.g.seats.clone() { p.g.tab.cmd_seat[s].remote = true; }
        let seat = p.g.seats[shown];
        game::seats::show_seat(&mut p.g, seat);
        for _ in 0..90 { hands(&mut p, 0, &["KeyW"], None, false); hands(&mut p, 1, &["KeyD"], None, false); frame(&mut p, shown); }
        game::seats::show_seat(&mut p.g, seat);
        assert!(game::savestate::control::restart(&mut p.g));
        assert_eq!((p.g.local_seat, p.g.seat, p.g.player), (Some(seat), seat, man(&p, shown)), "seat {shown} is shown");
        assert!(p.g.seats.iter().all(|&s| p.g.tab.cmd_seat[s].remote), "a hand is no longer checked");
        let first = p.words();
        for _ in 0..60 { hands(&mut p, 1, &["KeyW"], None, false); frame(&mut p, shown); }
        (first, p.words())
    };
    let (a, b) = (run(0), run(1));
    let names = game_start::parts::names();
    for (x, y) in [(&a.0, &b.0), (&a.1, &b.1)] { assert!(game_start::parts::differ(&names, x, y).is_empty(), "{:?}", game_start::parts::differ(&names, x, y)); }
}
