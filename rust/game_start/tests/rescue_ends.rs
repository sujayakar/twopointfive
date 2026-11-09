//! HOW A HOSTAGE RESCUE ENDS: not in a room just stood in; the hostages are led out; and once it is loud the men left come for them.
mod world_ask;
use game::game::Verb;
use game_start::play::{dist, Opts, Play};
use world_ask::RescueAsk;

/// The office's rescue with no team, Sam out of everybody's sight and unhurt; the men coming for the hostages let run or held off.
fn floor(s: u32) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&rescue&squad=0", &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    p.g.player_invisible = true;
    p.run(1.0);
    p
}

/// THE RULE: the room taken and stood in for 20 s, longer than the old hold, with men of theirs alive elsewhere: it is secured and the run has NOT ended.
/// A FAILURE MEANS: the mission ends behind the player's back with the building still full.
#[test]
fn the_rescue_does_not_end_in_a_room_just_stood_in() {
    let mut p = floor(7);
    p.hostage_room_falls(true);
    let mid = p.hostage_room_middle();
    p.sam_put(mid);
    p.run(20.0);
    assert!(p.rescue_said("secured").is_some() && p.rescue_result().is_none() && !p.theirs_up().is_empty(), "secured {:?}, the run {:?}, {} of theirs up", p.rescue_said("secured"), p.rescue_result(), p.theirs_up().len());
}

/// THE RULE: beside a hostage her row is "on your feet — with me"; taken, every living hostage goes behind Sam; led back to the mark the team was staged on, the run ends rescued, its line says they are out, every hostage alive and at that mark, men of theirs still alive.
/// CONTROL: the word and then its opposite: they stop where they stand and the row reads the word again.
/// A FAILURE MEANS: there is no way to end a rescue but to kill everybody, or the hostages cannot be got out.
#[test]
fn the_hostages_are_led_out_and_that_ends_it() {
    for s in [7u32] {
        let mut p = floor(s);
        p.hostage_room_falls(true);
        let mid = p.hostage_room_middle();
        p.sam_put(mid);
        p.run(0.5);
        let men = p.hostages_up();
        assert!(p.hostage_row_taken(men[0], Verb::Lead), "seed {s}: no row on her beside her");
        assert!(men.iter().all(|&h| p.led_by_sam(h)), "seed {s}: not all behind him");
        assert!(p.hostage_row_taken(men[0], Verb::Stay), "seed {s}: no 'stay here' row");
        assert!(men.iter().all(|&h| !p.led_by_sam(h)), "seed {s}: told to stay and still behind him");
        assert!(p.hostage_row_taken(men[0], Verb::Lead), "seed {s}: the row does not read the word again");
        let out = p.way_out();
        p.sam_walks_the_grid(out);
        p.until(30.0, |p| p.rescue_result().is_some()).unwrap_or_else(|| panic!("seed {s}: at the way out and the run goes on: {:?}", men.iter().map(|&h| dist(p.at(h), out)).collect::<Vec<_>>()));
        let (won, why) = p.rescue_result().unwrap();
        assert!(won && why.ends_with("hostages out") && p.hostages_up().len() == men.len() && !p.theirs_up().is_empty(), "seed {s}: rescued {won}, '{why}', {} of {} hostages, {} of theirs", p.hostages_up().len(), men.len(), p.theirs_up().len());
        for &h in &men { assert!(dist(p.at(h), out) < p.g.k.rescueend.out_m, "seed {s}: a hostage {:.1} m from the way out", dist(p.at(h), out)); }
    }
}

/// THE RULE: once it is loud and the room has fallen the men left alive come for the hostages: the first word comes the rule's seconds after the fall, the next its gap after that; and with nobody to stop him a man who came shoots a hostage, logged as executed and never the team's fire.
/// CONTROL: the same men put down without a sound and nothing loud done: past the first word's time nobody is sent and nobody shot.
/// A FAILURE MEANS: a rescue has no clock once it has gone loud, or a quiet one has.
#[test]
fn once_it_is_loud_the_men_left_come_for_the_hostages() {
    let mut p = floor(7);
    let (after, gap) = (p.g.k.rescueflush.after_secs, p.g.k.rescueflush.gap_secs);
    p.a_loud_round_at_the_room();
    p.run(0.5);
    p.hostage_room_falls(true);
    let fell = p.time();
    p.until(after + gap + 1.0, |p| p.rescue_lines("flush:room:").len() + p.rescue_lines("flush:side:").len() + p.rescue_lines("flush:noise:").len() >= 2).unwrap_or_else(|| panic!("no second word: {:?}", p.rescue_lines("flush:")));
    let words: Vec<f64> = p.g.rescue.log.iter().filter(|l| ["flush:room:", "flush:side:", "flush:noise:", "flush:out:"].iter().any(|k| l.what.starts_with(k))).map(|l| l.t - fell).collect();
    assert!((words[0] - after).abs() < 0.2 && words[1] - words[0] >= gap - 0.05, "the words at {words:?} s (after {after}, gap {gap})");
    let (n, t0) = (p.hostages_up().len(), p.time());
    p.until(30.0, |p| p.hostages_up().len() < n).unwrap_or_else(|| panic!("{:.0} s and nobody shot: {:?}", p.time() - t0, p.rescue_lines("flush:")));
    p.run(0.1);
    assert!(!p.rescue_lines("executed:").is_empty() && p.rescue_lines("teamfire:").is_empty(), "the log: {:?} / {:?}", p.rescue_lines("executed:"), p.rescue_lines("teamfire:"));
}

/// THE CONTROL: the same men put down without a sound and nothing loud done: past the first word's time nobody is sent and nobody shot.
#[test]
fn a_room_taken_quietly_starts_no_clock() {
    let mut p = floor(7);
    let (after, gap) = (p.g.k.rescueflush.after_secs, p.g.k.rescueflush.gap_secs);
    p.hostage_room_falls(false);
    p.run(after + gap + 2.0);
    assert!(p.rescue_lines("flush:").is_empty() && p.hostages_up().len() == p.g.rescue.hostages.len(), "without a sound: {:?}, {} of {} hostages", p.rescue_lines("flush:"), p.hostages_up().len(), p.g.rescue.hostages.len());
}

/// The rescue's floor with two roamers on their posts and the door man dead, his last known place at `to`: who the radio check sends to look there.
fn sent_to(s: u32, to: impl Fn(&[[f64; 3]]) -> [f64; 3]) -> (game::ids::GuardId, Vec<(game::ids::GuardId, [f64; 3])>, [f64; 3]) {
    let mut p = floor(s);
    p.g.k.rescue.roamers = 2.0;
    game::rescue::rescue_restage(&mut p.g);
    p.run(1.0);
    let cast = game::rescue::rescue_cast(&p.g);
    let roamers: Vec<(game::ids::GuardId, [f64; 3])> = cast.iter().filter(|c| c.role == game::rescue::Role::Roamer).map(|c| (c.gd, c.post)).collect();
    assert_eq!(roamers.len(), 2, "two roamers on their posts");
    let lost = cast.iter().find(|c| c.role == game::rescue::Role::Door).expect("a door man").gd;
    let c = p.g.gd[lost].ch;
    p.g.gd[lost].state = game::game::GuardState::Dead;
    p.g.ch[c].alive = false;
    let posts: Vec<[f64; 3]> = roamers.iter().map(|r| r.1).collect();
    let goal = to(&posts);
    let t = p.g.time;
    let st = p.g.m.rescuecheck.states.as_mut().expect("the check is on");
    st.next_at = t;
    st.where_.set(lost, goal);
    p.until(30.0, |p| p.g.m.rescuecheck.states.as_ref().is_some_and(|s| s.out.is_some())).unwrap_or_else(|| panic!("seed {s}: nobody went: {:?}", p.g.m.rescuecheck.states.as_ref().map(|s| (s.stage, s.next_at))));
    let out = p.g.m.rescuecheck.states.as_ref().unwrap().out.as_ref().unwrap();
    (out.gd, roamers, out.goal)
}

/// THE RULE: when a man does not answer the radio check, of the men free to go the one nearest on the floor to where he was last walks there, by both ways across the map: the place is put
/// beside one roamer, then nearer the other one way across the map and still nearer the first by the straight line.
/// A FAILURE MEANS: the far man leaves his post to look while one stands next to the place, so the player finds the wrong door unwatched.
#[test]
fn the_radio_check_sends_the_free_man_nearest_on_the_floor() {
    for s in [7u32, 11] {
        for near in [0usize, 1] {
            // beside him
            let (went, roamers, goal) = sent_to(s, |posts| {
                let (a, b) = (posts[near], posts[1 - near]);
                [a[0] + 0.3 * (b[0] - a[0]), 0.0, a[2] + 0.3 * (b[2] - a[2])]
            });
            assert_eq!(went, roamers[near].0, "seed {s}: {goal:?}: posts {roamers:?}");
            // nearer the other one way across the map, nearer him by the straight line
            let (went, _, goal) = sent_to(s, |posts| {
                let (a, b) = (posts[near], posts[1 - near]);
                let (dx, dz) = (a[0] - b[0], a[2] - b[2]);
                assert!(dz.abs() > 0.5, "the posts lie apart both ways: {posts:?}");
                [(a[0] + b[0]) / 2.0 - 0.1 * dx, 0.0, (a[2] + b[2]) / 2.0 + dz * 0.2 * (dx * dx / (dz * dz) + 1.0)]
            });
            assert_eq!(went, roamers[near].0, "seed {s}: by the straight line, {goal:?}: posts {roamers:?}");
        }
    }
}
