//! A MAN'S NERVE: the duck when a round comes close, the beat when he is surprised, a blast, and the man who drops it and runs.
mod stealth_ask;
use game::game::GuardState;
use game::guards::InHand;
use game::ids::GuardId;
use game::morale::Training;
use game::teams::Team;
use game_start::play::{dist, nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
const EAST: f64 = std::f64::consts::FRAC_PI_2;
const YARD_A: [f64; 2] = [4.8, 5.8];
const YARD_B: [f64; 2] = [6.6, 5.8];

/// A lit room, a man posted on a cell looking east along it, Sam out in the dark yard.
fn lit_room(s: u32) -> (Scene, [f64; 2]) {
    let at = [12.25 + nudge(s, 0).round(), 13.25];
    let mut sc = Scene::new("lit-room");
    sc.room("room", 10.0, 10.0, 26.0, 16.0).door("south", Axis::X, 16.0, 18.0).bulb(14.0, 13.0).bulb(19.0, 13.0);
    sc.sentry("post", at, [25.0, at[1]]).sam(30.0, 25.0);
    (sc, at)
}
fn peak(p: &mut Play, m: GuardId, frames: usize) -> f64 { let mut most = 0.0f64; for _ in 0..frames { p.frame(); most = most.max(p.duck(m)); } most }

/// THE RULE: "we should have some flinching when there's fire near a character": a hostile round past a calm man ducks him, a nearer one deeper; a round of his own side going past him does nothing.
/// A FAILURE MEANS: men stand under fire like posts, or duck at their own side's rounds.
#[test]
fn a_near_round_makes_a_man_duck() {
    for s in SEEDS {
        let duck_at = |off: f64, own: bool| {
            let (sc, _) = lit_room(s);
            let mut p = Play::scene(&sc, &seed(s));
            let m = p.guard("post");
            p.sam_unhurt();
            p.g.k.morale.training = Training::Green;
            p.g.gd[m].heard_up_to = 1e12;
            p.run(0.5);
            let shooter = if own { let o = p.man_of(Team::Guard, [30.0, 20.0], 0.0); p.g.gd[o].ch } else { p.g.pl[p.g.player].ch };
            p.round_past(shooter, m, off, 8.0);
            peak(&mut p, m, 24)
        };
        let (far, near, own) = (duck_at(1.4, false), duck_at(0.3, false), duck_at(0.3, true));
        assert!(far > 0.02, "seed {s}: a round 1.4 m past him and he shows {far:.3} of the duck");
        assert!(near > far, "seed {s}: a nearer round is no deeper: {near:.3} against {far:.3}");
        assert!(own <= 0.001, "seed {s}: his own side's round ducked him: {own:.3}");
    }
}

/// THE RULE: "a quick beat and then they pull out their gun or run": a calm man who first sees Sam starts the frame he knows, for the beat his feet are still and nothing is raised, then the gun comes up.
/// CONTROL: a man already looking for a while takes no start.
/// A FAILURE MEANS: a surprised man has his gun up on the frame he sees you, or a man who was already hunting freezes.
#[test]
fn a_man_surprised_has_a_beat() {
    for s in SEEDS {
        let (sc, at) = lit_room(s);
        let mut p = Play::scene(&sc, &seed(s));
        let m = p.guard("post");
        p.sam_unhurt();
        p.run(0.5);
        let beat = p.g.k.startle.beat;
        let fired = p.rounds_fired(m);
        p.sam_put([at[0] + 4.0, at[1]]);
        p.until(4.0, |p| p.state(m) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: four seconds lit in front of him: {:?}", p.state(m)));
        assert!(p.startled(m), "seed {s}: he does not start the frame he knows");
        assert!(!p.gun_up(m), "seed {s}: the gun is up as he starts");
        let t = p.until(beat + 1.0, |p| { assert!(dist(p.at(m), at) < 0.2 && p.rounds_fired(m) == fired, "seed {s}: moved or fired inside the beat"); !p.startled(m) }).expect("the beat ends");
        assert!((t - beat).abs() < 0.1, "seed {s}: the beat held {t:.2} s, it asks {beat}");
        p.until(1.5, |p| p.gun_up(m)).unwrap_or_else(|| panic!("seed {s}: the beat is over and no gun comes up"));
        drop(p);

        let (sc, at) = lit_room(s);
        let mut p = Play::scene(&sc, &seed(s));
        let m = p.guard("post");
        p.sam_unhurt();
        p.run(0.5);
        p.pin_man(m, at, EAST, GuardState::Suspicious, 0.45);
        p.let_go(m);
        p.g.gd[m].last_known = Some([at[0] + 1.0, 0.0, at[1]]);
        p.run(p.g.k.startle.fresh + 0.5);
        p.sam_put([at[0] + 4.0, at[1]]);
        p.until(4.0, |p| { assert!(!p.startled(m), "seed {s}: a man already looking started"); p.state(m) == GuardState::Alert }).unwrap_or_else(|| panic!("seed {s}: the man already looking never had him: {:?}", p.state(m)));
    }
}

/// The compound, every lamp as it is, one man behind the hut's door and everybody else parked across the yard, Sam off in the yard: the door's outside face, the man.
fn the_hut(s: u32, training: Training) -> (Play, GuardId, [f64; 3], [f64; 2]) {
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &seed(s));
    p.sam_unhurt();
    p.g.k.morale.training = training;
    p.run(0.2);
    let d = game::doors::doors_by_name(&p.g, "hut_w").expect("the hut's door");
    p.door_shut("hut_w");
    let w = p.g.dr[d].def.width;
    let men = p.hostiles_by(p.door_at("hut_w"));
    let t = men[0];
    let inside = game::doors::doors_side(&p.g, d, p.g.pos_of(t));
    let spot = game::doortools::door_pt(&p.g, d, w / 2.0, 2.4, inside, Some(0.0));
    for (i, &o) in men[1..].iter().enumerate() { p.man_stands(o, [4.8 + (i % 4) as f64 * 0.9, 5.8 + (i / 4) as f64 * 0.9]); p.parked(o); }
    for &o in p.mates().iter() { p.g.cond[p.g.gd[o].ch].blind_until = 1e9; }
    p.man_stands(t, [spot[0], spot[2]]);
    let yard = game::doortools::door_pt(&p.g, d, w / 2.0 + 9.0, 6.0, -inside, Some(0.0));
    p.sam_put([yard[0], yard[2]]);
    p.run(0.7);
    let hole = game::doortools::door_pt(&p.g, d, w / 2.0, p.g.k.he40.standoff + 0.03, -inside, Some(1.0 - p.g.k.frag.lift));
    (p, t, hole, [spot[0], spot[2]])
}
fn shell(p: &mut Play, hole: [f64; 3]) { let who = p.g.player_id(p.g.player); game::frag::blast(&mut p.g, hole, &game::frag::BlastSpec { who, name: Some(String::from("40 mm grenade")), ..game::frag::BlastSpec::default() }); }

/// THE RULE: a blast shakes the men it does not hurt, and green men and steady men differently: alone with the lamps burning and the 40 mm on his door a green man is broken; a steady man is not. Alive, either.
/// A FAILURE MEANS: training counts for nothing, or a blast is only its damage.
#[test]
fn a_blast_breaks_a_green_man_alone_and_not_a_steady_one() {
    for s in SEEDS {
        for (training, broken) in [(Training::Green, true), (Training::Trained, false)] {
            let (mut p, t, hole, _) = the_hut(s, training);
            assert!(game::morale::rung_now(&mut p.g, t) < 2.0, "seed {s}: broken before the blast");
            shell(&mut p, hole);
            let got = p.until(5.0, |p| !p.up(t) || game::morale::rung_now(&mut p.g, t) >= 2.0);
            assert!(p.up(t), "seed {s}: the blast killed him: the door stands between them");
            assert_eq!(got.is_some(), broken, "seed {s}, {training:?}: broken within five seconds of the blast");
        }
    }
}

/// THE RULE: a man the blast broke, caught dry with the enemy in his eyes, drops it and runs: his hand empties, two seconds on he is two metres from where he dropped it, no faster than a chase, and not a round leaves him.
/// CONTROLS: not dry, or the enemy further off than the rule's reach, nobody runs.
/// A FAILURE MEANS: a broken man goes on standing there out of rounds, or a man with rounds left gives up.
#[test]
fn a_broken_man_caught_dry_drops_it_and_runs() {
    for s in SEEDS {
        for (dry, reach, runs) in [(true, None, true), (false, None, false), (true, Some(1.0), false)] {
            let (mut p, t, hole, _) = the_hut(s, Training::Green);
            if let Some(m) = reach { p.g.k.runner.close_m = m; }
            p.let_go(t);
            shell(&mut p, hole);
            p.until(4.0, |p| game::morale::rung_now(&mut p.g, t) >= 2.0).unwrap_or_else(|| panic!("seed {s}: the blast did not break him"));
            let d = game::doors::doors_by_name(&p.g, "hut_w").expect("the hut's door");
            let w = p.g.dr[d].def.width;
            let side = game::doors::doors_side(&p.g, d, p.g.pos_of(t));
            let before = game::doortools::door_pt(&p.g, d, w / 2.0, 0.3, side, Some(0.0));
            p.sam_put([before[0], before[2]]);
            if dry { let gun = p.g.gd[t].weapon; p.g.fa[gun].chamber = 0.0; p.g.fa[gun].mag = Some(0.0); }
            let fired = p.rounds_fired(t);
            let ran = p.until(10.0, |p| p.fled(t) || p.hand_of(t) != Some(InHand::Weapon)).is_some();
            let gun = p.g.gd[t].weapon;
            if dry { assert_eq!(ran, runs, "seed {s}, dry, reach {reach:?}: he ran: {}", p.why(t)); }
            // (not dry he fires on the man in his eyes: he may run out in these ten seconds, and only then may he go)
            else { assert!(!ran || game::weapons::firearm_rounds_ready(&p.g, gun) == 0.0, "seed {s}: he ran with rounds left"); }
            if runs {
                let from = p.at(t);
                p.run(2.0);
                assert!(dist(p.at(t), from) > 2.0, "seed {s}: two seconds on he is {:.2} m from where he dropped it", dist(p.at(t), from));
                assert!(p.speed(t) <= p.g.k.tune.guard_chase + 0.3, "seed {s}: never faster than a chase: {:.2}", p.speed(t));
                assert_eq!(p.rounds_fired(t), fired, "seed {s}: a round left his gun after the shout");
                assert!(p.fled(t), "seed {s}: he is not marked as run");
            }
        }
    }
}

/// THE RULE: "if a guy gives up he should drop his gun (including pistol) and just run for it no matter what": a rifleman with his picture full of enemies in the only doorway still runs for the outside, and his rifle and his
/// thigh's pistol both lie on the floor.
/// A FAILURE MEANS: the fault it was found as: a man who dropped it took the corner for good with the fire exit two shut doors away.
#[test]
fn a_rifleman_who_has_given_up_runs_no_matter_what_and_drops_both_guns() {
    for s in SEEDS {
        let mut p = Play::mission("block.json", "?levelFile=block.json&roomclear&squad4", &seed(s));
        p.sam_unhurt();
        p.run(0.2);
        let men = p.hostiles();
        let a = men.iter().copied().find(|&m| p.g.weapon(p.g.fa[p.g.gd[m].weapon].def).cls != game::weapons::WeaponClass::Pistol).expect("a man with a long gun");
        for &o in men.iter() { if o != a { p.parked(o); p.man_stands(o, [5.0 + o.0 as f64 % 8.0, 26.0]); } }
        for &o in p.mates().iter() { p.parked(o); }
        p.man_stands(a, [32.0, 4.9]);
        p.g.cond[p.g.gd[a].ch].blind_until = 1e9;
        p.g.gd[a].heard_up_to = 1e12;
        p.run(0.1);
        p.sam_put([8.6, 13.9]);
        p.run(0.1);
        let door = [33.3, 0.0, 12.0];
        let sam = p.g.pl[p.g.player].ch;
        game::picture::picture_tell(&mut p.g, a, sam, door, None);
        assert!(game::runner::runner_break(&mut p.g, a), "seed {s}: he would not break");
        let from = p.at(a);
        p.until(25.0, |p| !p.g.guards.contains(&a) || dist(p.at(a), from) > 8.0).unwrap_or_else(|| panic!("seed {s}: 25 s and he has not run: {}", p.why(a)));
        assert!(game::runner::runner_bare(&p.g, a), "seed {s}: his thigh is not empty");
    }
}

/// THE RULE: a man who has run is nobody's company: a mate beside him has no mate; beside the same man not run he has one.
/// A FAILURE MEANS: a man who gave up still steadies the men next to him.
#[test]
fn a_man_who_has_run_is_nobodys_company() {
    for s in SEEDS {
        for broke in [true, false] {
            let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &seed(s));
            p.sam_unhurt();
            p.run(0.2);
            let men = p.hostiles();
            let (a, b) = (men[0], men[1]);
            for &o in men[2..].iter() { p.parked(o); p.man_stands(o, [34.0 + o.0 as f64 % 8.0, 34.0]); }
            for &o in p.mates().iter() { p.parked(o); }
            p.man_stands(a, YARD_A);
            p.man_stands(b, YARD_B);
            p.parked(b);
            p.run(0.2);
            assert!(game::morale::has_mate(&mut p.g, b), "seed {s}: CONTROL: a mate three metres off is company");
            if broke { assert!(game::runner::runner_break(&mut p.g, a), "seed {s}: he would not break"); p.until(2.0, |p| p.fled(a)).expect("he runs"); }
            p.man_stands(a, YARD_A);
            assert_eq!(game::morale::has_mate(&mut p.g, b), !broke, "seed {s}, he broke {broke}: company");
        }
    }
}
