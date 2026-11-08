//! TWO MACHINES OF ONE GAME. A game of two players runs on two machines, each stepping the whole game on both players' commands and showing its own
//! player's seat. Whatever one machine draws, the two must hold the same game in every part of its state.
use game::ids::GuardId;
use game::orders::Who;
use game_start::play::{nudge, parting, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [5, 6, 7];

/// `frames` of the office's room clear on a machine that shows the seat `shown`, with the first player's goggles down from the start
fn goggled(seed: u32, shown: usize, frames: usize) -> (Vec<Vec<u32>>, usize) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, seats: 2, ..Default::default() });
    let mut lines = 0;
    let rows = (0..frames).map(|_| {
        let man = game::seats::seat(&p.g, p.g.seats[0]).player;
        p.g.pl[man].nv = true;
        p.net_frame(shown);
        lines = lines.max(p.g.m.irlaser.ir_made.len());
        p.words()
    }).collect();
    (rows, lines)
}

/// THE RULE: one player's goggles are down and the other's are up. Both machines hold the same game.
/// A FAILURE MEANS: the step has come to ask whose picture THIS machine draws. (Before the mend the infrared lines a tube shows, boxes of the frame's
/// list, were made only on the machine whose shown player wore his goggles: the two machines' tables of boxes parted as the goggles came down.)
#[test]
fn one_players_goggles_do_not_part_the_two_machines() {
    for s in SEEDS {
        let ((mine, my_lines), (theirs, _)) = (goggled(s, 0, 90), goggled(s, 1, 90));
        assert!(my_lines > 0, "seed {s}: no infrared line was made in a second and a half: nothing here to part on");
        if let Some((k, parts)) = parting(&mine, &theirs) { panic!("seed {s}: with one player's goggles down, the machine that shows the other parts at frame {k} in {parts}"); }
    }
}

/// A locked store with a yard south of it, the team of two players in the yard facing it, a defender in it and one in a second
/// building down the yard.
fn store(seed: u32) -> Scene {
    let mut s = Scene::new("store");
    s.room("store", 10.0, 8.0, 18.0, 14.0).room("far", 26.0, 8.0, 34.0, 14.0);
    s.door("store-yard", Axis::X, 14.0, 13.4 + nudge(seed, 0)).locked("store-yard").door("far-yard", Axis::X, 14.0, 29.4).door("far-back", Axis::X, 8.0, 29.4);
    s.bulb(14.0, 11.0).bulb(30.0, 11.0).moon();
    s.team([14.0, 21.0], std::f64::consts::PI, &[[13.0, 22.0 + nudge(seed, 1)], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("far", [32.25, 10.25], "far-yard").defender("store", [17.25, 9.25], "store-yard");
    s
}
/// the men a player's order goes to
fn pair(p: &Play, seat: usize) -> Vec<GuardId> { game::orders::squad_orders_recipients(&p.g, &game::seats::seat(&p.g, p.g.seats[seat]), None, Some(Who::All)) }

/// THE RULE: each player's pair, stacked on a locked door and sent in, is through it within five seconds, whichever player's pair it is.
/// A FAILURE MEANS: what one seat's module does every frame reaches the other seat's men. (Before the mend the second seat's door tools put the breaching
/// shotgun of the first seat's man away every frame, and his pair took about nine seconds on a lock the other pair took two on.)
#[test]
fn each_players_pair_is_through_a_locked_door() {
    for s in SEEDS {
        for seat in [0, 1] {
            let scene = store(s);
            let door = scene.door_centre("store-yard");
            let mut p = Play::scene(&scene, &Opts { seed: s, seats: 2, ..Default::default() });
            // (the store's own defender is down before they come: the door is what is asked here, not his fire)
            let inside = p.defender_at([17.25, 9.25]);
            p.falls(inside, game::game::DownBy::Player);
            p.run(0.5);
            let men = pair(&p, seat);
            assert_eq!(men.len(), 2, "seed {s}: four teammates, two to a player");
            p.seat_orders(seat, "stack", door, Who::All);
            p.run(7.0);
            assert!(men.iter().all(|&m| p.at(m)[1] > 14.0), "seed {s}, seat {seat}: nobody is in the store before the word: {}", p.roll());
            p.seat_orders(seat, "openClear", door, Who::All);
            let took = p.until(5.0, |p| men.iter().all(|&m| p.at(m)[1] < 13.9)).unwrap_or_else(|| panic!("seed {s}, seat {seat}: five seconds after the word the pair is not through the locked door: {}", p.roll()));
            assert!(took > 0.5, "seed {s}, seat {seat}: through a locked door in {took:.2} s: it was not locked");
        }
    }
}

/// THE RULE: a game of two players holds no more men than the picture can draw, on every floor a room clear of two is played on.
/// A FAILURE MEANS: a budget of men counts one player. (Before the mend the barracks stood seventeen men for a picture of sixteen, and the page threw as the
/// fog let the last of them be drawn.)
#[test]
fn a_game_of_two_holds_no_more_men_than_the_picture_draws() {
    for level in ["office.json", "barracks.json", "block.json", "compound.json", "warehouse.json", "depot.json", "records.json"] {
        let p = Play::mission(level, &format!("?levelFile={level}&roomclear&squad4"), &Opts { seed: 5, seats: 2, ..Default::default() });
        let men = p.g.seats.len() + p.g.guards.len() + p.g.civilians.len();
        assert!(men as f64 <= game::anim_characters::MAX_CHARS, "{level}: {men} men for a picture of {}", game::anim_characters::MAX_CHARS);
    }
}

/// THE RULE: a grenade goes off beside the two players' fight, and the machine that shows one of them holds the same game, in every part of its state, as
/// the machine that shows the other.
/// A FAILURE MEANS: something the burst makes or asks (a box, a count, a thrown piece) is decided by whose picture this machine draws.
#[test]
fn a_burst_does_not_part_the_two_machines() {
    for s in SEEDS {
        let play = |shown: usize| -> Vec<Vec<u32>> {
            let scene = store(s);
            let mut p = Play::scene(&scene, &Opts { seed: s, seats: 2, ..Default::default() });
            let door = scene.door_centre("store-yard");
            (0..240).filter_map(|k| {
                if k == 5 { p.frag_lands([door[0], door[1] + 1.5]); }
                p.net_frame(shown);
                (k % 6 == 5).then(|| p.words())   // (the game's parts ten times a second)
            }).collect()
        };
        if let Some((k, parts)) = parting(&play(0), &play(1)) { panic!("seed {s}: a grenade beside the door, and the machine that shows the other player parts in {parts} {:.1} s on", (k + 1) as f64 * 0.1); }
    }
}

/// THE RULE: two machines that look at different places, one of them drawing and one of them drawing nothing, hold the same game in every part of its state: on a level with
/// a fire, a wind and snow.
/// A FAILURE MEANS: the tick has come to ask where THIS machine's picture looks.
#[test]
fn where_a_machine_looks_does_not_part_the_two_machines() {
    let play = |shown: usize, looks: bool| -> Vec<Vec<u32>> {
        let mut p = Play::mission("pass.json", "?levelFile=pass.json&roomclear&squad4", &Opts { seed: SEEDS[0], seats: 2, dress: true, ..Default::default() });
        let fire = (0..p.g.targets.len()).map(|i| p.g.targets[i]).find(|&t| game::fire::is_fire(&p.g, t)).map(|t| game::fire::mouth_of(&p.g, t).pos).expect("the pass has a brazier");
        let mut drawn = game_draw::Drawn::default();
        (0..90).filter_map(|k| {
            if looks {
                p.g.engine.view_focus = Some(fire);
                let c = &mut p.g.tab.camera[p.host.cam];
                (c.target, c.pos, c.yaw, c.distance) = (fire, [fire[0], fire[1] + 12.0, fire[2] + 9.0], 0.4 + k as f64 * 0.01, 14.0);
            }
            p.net_frame(shown);
            if looks { drawn.take(&p.g, u32::MAX); drawn.frame(&p.g, 0.5, fire); }
            (k % 6 == 5).then(|| p.words())
        }).collect()
    };
    if let Some((k, parts)) = parting(&play(0, false), &play(1, true)) { panic!("the machine that looks at the fire parts in {parts} {:.1} s on", (k + 1) as f64 / 10.0); }
}

/// Two players in the yard, every seat refereed as over the net, the second's man walking on `keys`; before every tenth tick a machine that `looks` looks 24 records ahead
/// for him. Answers the words at each tick, and for each look whether his legs were his own and
/// nobody within a metre and a half of him all through it, where it had him after each record and where the game then had him.
fn walked(seed: u32, keys: &str, looks: bool) -> (Vec<Vec<u32>>, Vec<(bool, Vec<[f64; 3]>, Vec<[f64; 3]>)>) {
    let mut scene = store(seed);
    scene.team([14.0, 21.0], std::f64::consts::PI, &[[6.0, 19.0], [22.0, 19.0], [6.0, 24.0], [22.0, 24.0]]);
    let mut p = Play::scene(&scene, &Opts { seed, seats: 2, ..Default::default() });
    for s in p.g.seats.clone() { p.g.tab.cmd_seat[s].remote = true; }
    // (the second player's man starts beside the first's: to the open end of the yard)
    let second = p.g.seats[1];
    game::seats::with_seat(&mut p.g, second, &mut |g| game::player::teleport_player(g, [14.0, 0.0, 24.5]));
    game::game::game_aside_lens(&mut p.g);
    let (seats, canvas) = (p.g.seats.clone(), p.host.canvas);
    let mut cmds: Vec<game::cmd::PlayerCmd> = seats.iter().map(|&s| {
        let (input, cam) = (p.g.tab.cmd_seat[s].input, p.g.tab.cmd_seat[s].cam);
        game::seats::with_seat(&mut p.g, s, &mut |g| game::cmd::sample_cmd(g, input, cam, canvas, None))
    }).collect();
    cmds[1].keys = vec![String::from(keys)];
    // (as a record comes off the wire: the eye and the matrices are for the game to work out)
    for c in cmds.iter_mut() { (c.view.vp, c.view.ivp) = (vec![f64::NAN; 16], vec![f64::NAN; 16]); }
    let ch = p.g.pl[game::cmd::seat_man(&p.g, seats[1])].ch;
    let (mut rows, mut seen, mut at) = (Vec::new(), Vec::new(), Vec::new());
    let mut alone: Vec<bool> = Vec::new();
    for k in 0..90 {
        if looks && k % 10 == 5 {
            let mut path = Vec::new();
            let a = game::walkahead::walk_ahead(&mut p.g, seats[1], &vec![cmds[1].clone(); 24], 1.0 / 60.0, Some(&mut path));
            seen.push((k, a.free, path));
        }
        game::game::game_step_seats(&mut p.g, 1.0 / 60.0, &cmds);
        p.g.out.clear();
        at.push(p.g.ch[ch].pos);
        // (a look holds everybody else where the tick has them, and plays no beat that begins inside it: both are the game's to say)
        let others = p.g.guards.iter().map(|&gd| p.g.gd[gd].ch).chain(p.g.seats.iter().map(|&s| p.g.pl[game::cmd::seat_man(&p.g, s)].ch)).filter(|&o| o != ch);
        let near = others.map(|o| game_start::play::dist([p.g.ch[o].pos[0], p.g.ch[o].pos[2]], [at[k][0], at[k][2]])).fold(f64::INFINITY, f64::min);
        let legs = game::seats::with_seat(&mut p.g, seats[1], &mut |g| game::player::legs_own(g));
        alone.push(legs && near > 1.5);
        rows.push(p.words());
    }
    let seen = seen.into_iter().map(|(k, free, path)| { let n = path.len().min(at.len() - k); (free && n > 0 && alone[k..k + n].iter().all(|a| *a), path[..n].to_vec(), at[k..k + n].to_vec()) }).collect();
    (rows, seen)
}

/// THE RULE: a look ahead over a player's records answers where the game then has his man, to the bit, record by record.
/// A FAILURE MEANS: the walk of a look ahead and the tick's own walk have come apart.
/// THE RULE: a machine that looks ahead holds the same game, in every part of its state, as one that never does.
/// A FAILURE MEANS: a look writes what a tick reads.
#[test]
fn a_look_ahead_is_where_the_game_then_has_him_and_writes_nothing() {
    for (seed, keys) in [(SEEDS[0], "KeyW"), (SEEDS[1], "KeyD"), (SEEDS[2], "KeyS")] {
        let (with, seen) = walked(seed, keys, true);
        if let Some((k, parts)) = parting(&walked(seed, keys, false).0, &with) { panic!("seed {seed}: the machine that looks ahead parts at tick {k} in {parts}"); }
        let free: Vec<_> = seen.iter().filter(|x| x.0).collect();
        assert!(free.len() >= 2, "seed {seed}: his legs were his own and nobody near him in {} of {} looks", free.len(), seen.len());
        assert!(free.iter().any(|x| game_start::play::dist([x.1[0][0], x.1[0][2]], [x.1[x.1.len() - 1][0], x.1[x.1.len() - 1][2]]) > 0.3), "seed {seed}: no look walked him");
        for (i, x) in free.iter().enumerate() { assert_eq!(x.1, x.2, "seed {seed}: look {i} of a walk on {keys}"); }
    }
}
