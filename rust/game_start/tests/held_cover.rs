//! COVER ON THE FLOOR: stopped, everybody watches a way in of his own; an order wins, and cover comes back with no word.
mod kit_ask;
use game::ids::GuardId;
use game::orders::Who;
use game_start::play::{Opts, Play};
use kit_ask::KitAsk;

/// The office's room clear with its defenders blind, the squad fallen in on Sam, walked by his own keys down the aisle and stopped there.
fn stopped_in_the_aisle(seed: u32, squad: &str) -> (Play, f64) {
    let mut p = Play::mission("office.json", &format!("?levelFile=office.json&roomclear&{squad}"), &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    for gd in p.men_on_the_floor() { p.g.ch[p.g.gd[gd].ch].cond.dazzled_until = 1e9; }
    p.order("fallIn", p.sam_at(), Who::All);
    p.run(1.0);
    p.sam_put([13.0, 17.6]);
    p.camera_square();
    p.run(2.0);
    p.point_at([26.0, 17.6]);
    p.key_down("KeyD");
    let walking = p.until(12.0, |p| p.sam_at()[0] >= 20.6).is_some();
    p.key_up("KeyD");
    assert!(walking, "Sam walks the aisle: at {:?}", p.sam_at());
    let t0 = p.time();
    (p, t0)
}

fn dealt(p: &Play) -> usize { p.mates().into_iter().filter(|&m| p.opening_of(m).is_some()).count() }

/// THE RULE: with the squad stopped, inside three seconds every man who has an opening left in a clear line is squared up to an opening of his own, gun on it, feet still; at most one is boxed in by the file.
/// THE CONTROL: on the walk there, the same men are not all squared up at once.
/// A FAILURE MEANS: a squad halted in a room stares at the same door, or at nothing.
#[test]
fn stopped_everybody_squares_up_to_an_opening_of_his_own() {
    for seed in [3, 4, 5] {
        let (mut p, t0) = stopped_in_the_aisle(seed, "squad4");
        let men = p.mates();
        assert_eq!(men.len(), 4);
        let set = |p: &Play, m: GuardId| p.squared_up(m) || (p.opening_of(m).is_none() && !p.opening_to_spare_for(m));
        assert!(men.iter().any(|&m| !set(&p, m)), "seed {seed}, control: walking, somebody is still on his way to his opening");
        let settled = p.until(3.0, |p| p.mates().iter().all(|&m| set(p, m)));
        assert!(settled.is_some(), "seed {seed}: not squared up {:.1} s after he stopped: {:?}", p.time() - t0, men.iter().map(|&m| (p.opening_of(m), p.squared_up(m))).collect::<Vec<_>>());
        let jobs: Vec<_> = men.iter().filter_map(|&m| p.opening_of(m)).collect();
        assert!(jobs.len() >= 3, "seed {seed}: only {} of four dealt an opening", jobs.len());
        for (i, a) in jobs.iter().enumerate() { assert!(!jobs[i + 1..].contains(a), "seed {seed}: two men on one opening: {jobs:?}"); }
    }
}

/// THE RULE: an order wins: from the next deal after 'stack' on a door the men at the jambs hold no opening and a rear man none until his last steps to his mark; 'fall in' puts them back inside two and a half seconds with nothing else said.
/// THE CONTROL: an order to the red pair alone leaves blue dealt.
/// A FAILURE MEANS: a man watches a doorway when he has been told to stack, or never goes back to watching.
#[test]
fn an_order_takes_them_off_their_openings_and_fall_in_puts_them_back() {
    for seed in [3, 4, 5] {
        let (mut p, _) = stopped_in_the_aisle(seed, "squad4");
        p.run(3.0);
        assert!(dealt(&p) >= 3, "seed {seed}: stopped, {} dealt", dealt(&p));
        let d = p.door("server");
        let c = p.g.dr[d].frame_centre;
        p.order("stack", [c[0], c[2]], Who::All);
        let deal = p.deal_secs();
        p.run(deal + 0.05);
        let free_to_watch = |p: &Play, m: GuardId| !p.stacks_at_the_jamb(m) && !p.stacks_on_his_walk(m);
        let broke = p.always(3.0, |p| p.mates().iter().all(|&m| p.opening_of(m).is_none() || free_to_watch(p, m)));
        assert!(broke.is_none(), "seed {seed}: under 'stack' a jamb man or a rear man on his walk holds an opening: {:?}", p.mates().iter().map(|&m| (p.opening_of(m).is_some(), p.stacks_at_the_jamb(m), p.stacks_on_his_walk(m))).collect::<Vec<_>>());
        p.order("fallIn", p.sam_at(), Who::All);
        let back = p.until(2.5, |p| dealt(p) >= 3);
        assert!(back.is_some(), "seed {seed}: fall in, and {} are back on openings after 2.5 s", dealt(&p));

        p.run(1.0);
        p.order("stack", [c[0], c[2]], Who::Red);
        p.run(deal + 0.05);
        let blue = p.blue();
        assert!(p.always(2.0, |p| blue.iter().any(|&m| p.opening_of(m).is_some())).is_none(), "seed {seed}, control: blue has no order and goes on watching");
        assert!(p.red().iter().any(|&m| p.stacks_at_the_jamb(m) && p.opening_of(m).is_none()), "seed {seed}: red's man at the jamb is the door's");
    }
}

/// THE RULE: Sam's own gun on an opening counts as covering it: the man who had it takes one nobody has.
/// THE CONTROL: Sam's gun anywhere else leaves that man where he was.
/// A FAILURE MEANS: a teammate and the player watch one door while another stands open behind them.
#[test]
fn sams_gun_on_an_opening_sends_the_man_to_another() {
    let mut moved = 0;
    for seed in [3, 4, 5] {
        let (mut p, _) = stopped_in_the_aisle(seed, "squad2");
        p.run(3.0);
        let Some(&m) = p.mates().iter().find(|&&m| p.opening_of(m).is_some() && p.opening_to_spare_for(m)) else { continue };
        let was = p.opening_of(m).unwrap();
        p.run(2.0);
        assert_eq!(p.opening_of(m), Some(was), "seed {seed}, control: Sam looks on down the aisle, and his man keeps his opening");
        p.point_at(p.opening_at(was));
        let gone = p.until(2.0, |p| p.opening_of(m).is_some_and(|o| o != was));
        assert!(gone.is_some(), "seed {seed}: Sam's gun on it, and he stays: Sam covers {:?}", p.openings_of_sam());
        assert!(p.openings_of_sam().contains(&was), "seed {seed}: his gun is not counted on it");
        moved += 1;
    }
    assert!(moved > 0, "on no seed had the man an opening to spare");
}
