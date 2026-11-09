//! TWO MACHINES OF ONE GAME, ONE GAME OF WHAT BREAKS. Machines stand alike and each shows a seat of its own: two seats, the office, grenades and a wall charge,
//! every part of the state by name for ninety seconds, read twice a second.
//! A FAILURE MEANS: a rule reads what a host holds or which seat it shows.
use game_start::play::{Opts, Play};

const FRAMES: usize = 90 * 60;
const EVERY: usize = 30;

/// the words of every `EVERY`th frame, on a machine showing the seat `shown`
fn run(shown: usize) -> Vec<Vec<u32>> {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 7, seats: 2, dress: true, ..Default::default() });
    p.sam_unhurt();
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
    let who = p.g.ch[p.g.pl[p.g.player].ch].id;
    let mut out = Vec::with_capacity(FRAMES / EVERY);
    let mut blown = false;
    for k in 0..FRAMES {
        if k == 120 || k == 900 { game::frag::blast(&mut p.g, [20.0, 0.5, 15.0], &game::frag::BlastSpec { who, ..Default::default() }); }
        if k == 180 { p.sam_put([14.6, 15.2]); let _ = game::wallcharge::wall_charge_order(&mut p.g, [12.06, 0.0, 13.78], None); }
        if !blown { if let Some(&c) = p.g.wall_charge.set.first() { game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true)); blown = true; } }
        p.net_frame(shown);
        if k % EVERY == 0 { out.push(p.words()); }
    }
    assert!(blown, "the staging is wrong: the charge never went");
    out
}

/// THE RULE: every part is the same on both.
#[test]
fn what_breaks_is_the_same_on_two_machines() {
    let (a, b) = (run(1), run(0));
    let names = game_start::parts::names();
    for k in 0..a.len() {
        for (i, part) in names.iter().enumerate() {
            assert_eq!(a[k][i], b[k][i], "{part} parted by frame {}", k * EVERY + 1);
        }
    }
}
