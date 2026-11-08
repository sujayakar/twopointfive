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
    for level in ["office.json", "barracks.json", "block.json", "compound.json", "warehouse.json", "depot.json", "records.json", "block.json"] {
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
