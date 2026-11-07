//! TWO MACHINES OF ONE GAME. A game of two players runs on two machines, each stepping the whole game on both players' commands and showing its own
//! player's seat. Whatever one machine draws, the two must hold the same game in every part of its state.
use game_start::play::{parting, Opts, Play};

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
