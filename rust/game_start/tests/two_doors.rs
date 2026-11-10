//! TWO TEAMS AT NEIGHBORING DOORS (rust/game_rules/src/entries/stackroom.rs, doorway/): each keeps out of the other's way, and a shut door stays shut until somebody goes through it or works it.
use game::doors::{door_latched, doors_by_name};
use game::ids::DoorId;
use game::orders::Who;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];
/// the middles of two doors of the block's hall, three metres apart on one wall
const FRAME: [f64; 2] = [30.25, 12.0];
const COMMS: [f64; 2] = [33.25, 12.0];

/// The block, the squad in its hall with Sam, unseen and unhurt.
fn hall(seed: u32) -> (Play, DoorId, DoorId) {
    let mut p = Play::mission("block.json", "?levelFile=block.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.sam_unseen();
    p.run(0.3);
    p.sam_put([27.5, 13.4]);
    let spots = [[28.6, 13.0], [28.6, 13.9], [26.6, 13.0], [26.6, 13.9]];
    for (i, m) in p.mates().into_iter().enumerate() { p.man_stands(m, spots[i]); }
    p.order("fallIn", p.sam_at(), Who::All);
    p.run(1.5);
    let (a, b) = (doors_by_name(&p.g, "frame_to_hall").unwrap(), doors_by_name(&p.g, "b_s").unwrap());
    (p, a, b)
}

/// THE RULE: two pairs each told to breach one of two neighboring doors with a charge both set it, stand off clear of the other's door and the other's blast, and go in together on one word, given
/// while the second charge is still being set.
/// A FAILURE MEANS: one pair is posted in front of the other's door, where it is in the way and in the blast (before: the second pair's man was crowded against his own door as he left it, it came
/// open, and their charge was off).
#[test]
fn two_charge_teams_at_neighboring_doors_both_go() {
    for s in SEEDS {
        let (mut p, frame, comms) = hall(s);
        let (red, blue) = (p.red(), p.blue());
        p.order("charge", FRAME, Who::Blue);
        p.run(1.7);
        p.order("charge", COMMS, Who::Red);
        p.run(3.0);
        for (men, own, other) in [(&red, comms, frame), (&blue, frame, comms)] {
            for &m in men.iter() {
                let post = p.mark_of(m).unwrap_or_else(|| panic!("seed {s}: {} has no mark", p.g.gd[m].callsign));
                let at = [post[0], 0.0, post[1]];
                let lead = game_start::play::dist(post, if own == comms { COMMS } else { FRAME }) < 1.0;   // (the man at the leaf, setting it)
                if lead { continue; }
                assert!(!game::doorway::before_another_door(&p.g, at, own), "seed {s}: {} stands off at {post:?}, before a door that is not theirs", p.g.gd[m].callsign);
                let c = p.g.dr[other].frame_centre;
                assert!(game_start::play::dist(post, [c[0], c[2]]) > p.g.k.doortools.backblast_m, "seed {s}: {} stands off at {post:?}, in the other charge's blast", p.g.gd[m].callsign);
            }
        }
        p.order("go", p.sam_at(), Who::All);
        let shut = |p: &Play, d| door_latched(&p.g, d) && !game::doors::door_off(&p.g, d);
        let first = p.until(20.0, |p| !shut(p, frame) || !shut(p, comms)).unwrap_or_else(|| panic!("seed {s}: twenty seconds after the word and neither door is open ({})", p.roll()));
        p.until(1.0, |p| !shut(p, frame) && !shut(p, comms)).unwrap_or_else(|| panic!("seed {s}: one door went {first:.1} s after the word and the other did not ({})", p.roll()));
        for (men, own) in [(&red, comms), (&blue, frame)] {
            let c = p.g.dr[own].frame_centre[2];
            p.until(12.0, |p| men.iter().any(|&m| p.at(m)[1] < c - 0.5 || !p.up(m))).unwrap_or_else(|| panic!("seed {s}: nobody of theirs went in by {}", p.g.dr[own].def.name));
        }
    }
}

/// THE RULE: a man who brushes a shut door on his way along the wall leaves it shut.
/// A FAILURE MEANS: a touch opens a door again: men walking by open rooms nobody has told them to.
#[test]
fn a_man_brushing_a_shut_door_leaves_it_shut() {
    for s in SEEDS {
        let (mut p, _, comms) = hall(s);
        let m = p.red()[0];
        for x in [33.55, 33.35, 33.15] {
            p.man_stands(m, [x, 12.28]);
            p.order("move", [24.5, 13.2], Who::Red);
            for _ in 0..90 { p.frame(); assert!(door_latched(&p.g, comms), "seed {s}: he walked by at {:?} and the door came open", p.at(m)); }
        }
    }
}
