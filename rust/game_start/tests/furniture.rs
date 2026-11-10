//! FURNITURE AND A BLAST: a wall that holds shields it; a restart and a load put it back. The warehouse's office, whose wall along z 17 the maintainer watched a charge go through.
mod world_ask;
use game_start::play::{dist, nudge, Opts, Play};
use world_ask::FurnAsk;

const CHARGE: [f64; 3] = [8.0, 0.9, 16.9];
const INSIDE: [f64; 3] = [8.2, 0.9, 17.5];
const DESK: [f64; 2] = [8.2, 18.2];
const CHAIR: [f64; 2] = [8.2, 19.1];

fn office(s: u32) -> Play {
    let mut p = Play::level("warehouse.json", false, &Opts { seed: s, ..Default::default() });
    p.g.tune.chaos = 0.5;
    p.g.god_mode = true;
    p.g.ai_enabled = false;
    p.g.player_invisible = true;
    p.run(0.1);
    p
}

/// THE RULE: a wall charge on the office wall throws the chair behind the hole AWAY from it. A blast the wall holds against (of no strength for the wall) moves and breaks nothing behind it; the same blast on their side of it takes the desk and throws the chair.
/// A FAILURE MEANS: furniture comes apart through a standing wall, or a charge that opens a wall leaves the room as it was.
#[test]
fn a_wall_that_holds_shields_the_furniture() {
    for s in [1u32] {
        // the charge that opens it
        let mut p = office(s);
        let chair = p.prop_near(CHAIR);
        let was = p.prop_at(chair);
        let at = [CHARGE[0] + 0.3 * nudge(s, 0), CHARGE[1], CHARGE[2]];
        assert!(p.wall_blast(at, None) >= 1.0, "seed {s}: the charge holed no wall");
        p.run(0.12);   // (a lived-in room: the next thing behind him turns him back further on)
        let (now, a) = (p.prop_at(chair), [was[0] - at[0], was[1] - at[2]]);
        let away = ((now[0] - was[0]) * a[0] + (now[1] - was[1]) * a[1]) / a[0].hypot(a[1]);
        assert!(away > 0.05, "seed {s}: the chair moved {:.2} m, {away:.2} m of it away from the charge at {at:?}: {was:?} to {now:?}", dist(now, was));
        drop(p);
        // a wall that holds
        let mut p = office(s);
        let (chair, desk) = (p.prop_near(CHAIR), p.furniture_at(DESK));
        let was = p.prop_at(chair);
        assert_eq!(p.wall_blast(CHARGE, Some(0.0)), 0.0, "seed {s}: a blast of no strength holed the wall");
        p.run(0.6);
        assert!(dist(p.prop_at(chair), was) < 1e-6 && p.standing(&desk) == desk.len(), "seed {s}: behind a wall that held: the chair moved {:.2} m, {} of {} of the desk stand", dist(p.prop_at(chair), was), p.standing(&desk), desk.len());
        drop(p);
        // the same blast on their side of it
        let mut p = office(s);
        let (chair, desk) = (p.prop_near(CHAIR), p.furniture_at(DESK));
        let was = p.prop_at(chair);
        p.wall_blast(INSIDE, Some(0.0));
        p.run(0.6);
        assert!(dist(p.prop_at(chair), was) > 0.5 && p.standing(&desk) == 0, "seed {s}: on their side: the chair moved {:.2} m, {} of {} of the desk stand", dist(p.prop_at(chair), was), p.standing(&desk), desk.len());
    }
}

/// THE RULE: a restart puts every static's flags, every prop and the walk grid back; the same blast after it is the same blast, to the last static. CONTROL: the blast did change all that, so the restart had something to do.
/// A FAILURE MEANS: a second try starts in the wreck of the first, or two tries of one blast differ.
#[test]
fn a_restart_puts_the_furniture_back_and_the_same_blast_breaks_it_the_same() {
    let at = [8.2, 0.5, 19.0];
    let mut p = office(3);
    let whole = p.level_as_it_stands();
    p.wall_blast(at, None);
    p.run(0.5);
    let first = p.level_as_it_stands();
    assert!(first[0] != whole[0], "the staging: the blast took nothing");
    game::savestate::control::restart(&mut p.g);
    p.g.god_mode = true;
    p.g.ai_enabled = false;
    p.g.player_invisible = true;
    p.run(0.1);
    let back = p.level_as_it_stands();
    for (k, what) in ["flags", "props", "walk grid"].iter().enumerate() { assert!(back[k] == whole[k], "the restart left the {what} changed"); }
    p.wall_blast(at, None);
    p.run(0.5);
    assert!(p.level_as_it_stands()[0] == first[0], "the same blast took other statics");
}

/// THE RULE: a load keeps what a blast broke: the statics gone, the props out, the grid as it was at the save, though the game went on to break more. A restart after the load gives the level back whole.
/// A FAILURE MEANS: a checkpoint brings a desk back from the dead, or keeps a later wreck.
#[test]
fn a_load_keeps_broken_furniture_broken_and_a_restart_after_it_restores() {
    let mut p = office(4);
    let whole = p.level_as_it_stands();
    p.wall_blast(CHARGE, None);
    p.run(1.0);
    let at_save = p.level_as_it_stands();
    assert!(at_save[0] != whole[0], "the staging: the blast took nothing");
    let kept = p.keep();
    p.wall_blast([8.2, 0.5, 19.0], None);
    p.run(0.5);
    assert!(p.level_as_it_stands()[0] != at_save[0], "the staging: the second blast took nothing");
    p.put_back(&kept);
    p.frame();
    let loaded = p.level_as_it_stands();
    assert!(loaded[0] == at_save[0] && loaded[2] == at_save[2], "the load did not give the level as it was saved");
    game::savestate::control::restart(&mut p.g);
    p.run(0.1);
    let back = p.level_as_it_stands();
    assert!(back[0] == whole[0] && back[2] == whole[2], "a restart after the load did not give the level back");
}
