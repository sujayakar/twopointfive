//! MEN AT THEIR EASE. A man in the dark sleeps and what wakes him; a man goes for his coffee one at a time, says so first, and a shot ends it.
mod stealth_ask;
use game::game::GuardState;
use game::ids::GuardId;
use game::needs::{NeedKind, TripPhase};
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
/// the pin of the police post's quiet mission whose deal is the post's own hand-set one (first start, first spot, first patrol set)
const POST_PIN: u32 = 32;

/// The police post's quiet mission, Sam out of everybody's sight, and its man asleep with a long wait left.
fn sleeper(seed: u32) -> (Play, GuardId) {
    let link = format!("?levelFile=police-post.json&quiet={POST_PIN}");
    let mut p = Play::mission("police-post.json", &link, &Opts { seed, ..Default::default() });
    assert_eq!(p.quiet_deal(), (0.0, 0.0, 0.0), "the pin's deal has moved");
    p.sam_unhurt();
    p.g.player_invisible = true;
    let m = p.until(120.0, |p| p.g.guards.iter().copied().find(|&m| p.asleep(m) && p.g.gd[m].wait > 30.0).is_some()).map(|_| p.g.guards.iter().copied().find(|&m| p.asleep(m)).unwrap());
    let m = m.unwrap_or_else(|| panic!("seed {seed}: two minutes and nobody slept"));
    for o in p.g.guards.clone() { if o != m { p.g.cond[p.g.gd[o].ch].blind_until = 1e9; } }
    p.run(1.0);
    (p, m)
}
/// a place `d` metres from him with clear air between (from his own chest out)
fn clear_at(p: &mut Play, m: GuardId, d: f64) -> [f64; 2] {
    let c = p.g.gd[m].ch;
    let at = p.g.ch[c].pos;
    for k in 0..72 {
        let a = k as f64 * std::f64::consts::PI / 36.0;
        let q = [at[0] + a.sin() * d, 0.0, at[2] + a.cos() * d];
        if !game::scene_collision::segment_blocked(&mut p.g, [at[0], 1.2, at[2]], [q[0], 1.2, q[2]], None, None, None) && !game::scene_collision::segment_blocked(&mut p.g, [at[0], 0.4, at[2]], [q[0], 0.4, q[2]], None, None, None) { return [q[0], q[2]]; }
    }
    panic!("no clear air round his chair");
}
/// one of Sam's footfalls, as a hearing man takes it, Sam himself a man whose noise counts
fn footfall(p: &mut Play, at: [f64; 2], level: f64, frames: usize) {
    p.g.player_invisible = false;
    p.footfall(at, level);
    for _ in 0..frames { p.frame(); }
    p.g.player_invisible = true;
}

/// THE RULE: a man with a long wait in the dark sits in the chair there and sleeps. His eyes are shut: Sam lit in front of him for three seconds is not seen. His ears are dull, not gone: twelve crouched steps at a metre and a walking step at 2.8 m leave him asleep, a walking step at a metre wakes him: suspicious at once, on the noise.
/// A FAILURE MEANS: a sleeper is either a man awake with a mark over him, or a piece of furniture you can run past.
#[test]
fn a_man_on_a_long_wait_in_the_dark_sleeps_with_his_eyes_shut_and_his_ears_dull() {
    for s in SEEDS {
        let (mut p, m) = sleeper(s);
        assert!(p.seated(m), "seed {s}: asleep but not in the chair");
        let c = p.g.gd[m].ch;
        let (at, y) = (p.g.ch[c].pos, p.g.ch[c].body_yaw);
        p.g.player_invisible = false;
        let me = p.g.player;
        p.g.pl[me].crouch = true;
        p.sam_put([at[0] + y.sin() * 2.0, at[2] + y.cos() * 2.0]);
        let mut peak = 0.0f64;
        for _ in 0..180 { p.g.shown[p.g.pl[me].ch].visibility = 1.0; p.frame(); peak = peak.max(p.awareness(m)); }
        assert!(p.asleep(m) && peak < 0.05, "seed {s}: lit in front of him: awareness {peak:.2}, asleep {}", p.asleep(m));
        p.sam_put([at[0] + 30.0, at[2]]);
        p.g.player_invisible = true;

        let (near, mid) = (clear_at(&mut p, m, 1.0), clear_at(&mut p, m, 2.8));
        for _ in 0..12 { footfall(&mut p, near, 0.125, 20); }
        assert!(p.asleep(m), "seed {s}: twelve crouched steps at a metre woke him");
        footfall(&mut p, mid, 0.26, 5);
        assert!(p.asleep(m), "seed {s}: a walking step at 2.8 m woke him");
        footfall(&mut p, near, 0.26, 3);
        assert!(!p.asleep(m), "seed {s}: a walking step at a metre and he slept on");
        assert_eq!(p.state(m), GuardState::Suspicious, "seed {s}: woken by a noise he is suspicious on the frame");
        let fix = p.g.gd[m].last_known.expect("a fix on the noise");
        assert!(dist([fix[0], fix[2]], near) < 1.0, "seed {s}: placed {:.1} m from the noise", dist([fix[0], fix[2]], near));
    }
}

/// THE RULE: a running step across the room (3.4 m) wakes a sleeper a walking one at 2.8 m does not.
/// A FAILURE MEANS: his ears count no difference between a walk and a run.
#[test]
fn a_running_step_across_the_room_wakes_him() {
    for s in SEEDS {
        let (mut p, m) = sleeper(s);
        let far = clear_at(&mut p, m, 3.4);
        footfall(&mut p, far, 0.6, 3);
        assert!(!p.asleep(m), "seed {s}: a running step at 3.4 m and he slept on");
    }
}

/// The office, Sam a ghost on the verge, and Kowalski due a coffee.
fn due_a_coffee(seed: u32) -> (Play, [GuardId; 3]) {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.g.tune.needs = true;
    p.run(0.1);
    p.sam_put([2.0, 2.0]);
    p.g.player_invisible = true;
    let m = [p.g.guards[0], p.g.guards[1], p.g.guards[2]];
    p.run(0.1);
    assert!(game::needs::needs_set_level(&mut p.g, m[0], NeedKind::Coffee, 1.2), "the first man was not dealt coffee");
    (p, m)
}

/// THE RULE: a man's trip is telegraphed: scheduled, he stays on his route through the whole lead, says it at the say lead, walks at the end of it; and while he is gone no other man is, whatever his own coffee reads.
/// A FAILURE MEANS: the floor empties as one, or a man walks off with no tell for the player to read.
#[test]
fn a_trip_is_told_first_and_one_man_goes_at_a_time() {
    for s in SEEDS {
        let (mut p, [a, b, c]) = due_a_coffee(s);
        let t = p.until(60.0, |p| p.trip(a).is_some()).unwrap_or_else(|| panic!("seed {s}: a minute with his coffee full and no trip"));
        let _ = t;
        let trip = p.trip(a).unwrap();
        let (lead, say) = (p.g.k.needs.announce_lead, p.g.k.needs.say_lead);
        let go = trip.t0 + lead;
        assert_eq!(trip.phase, TripPhase::Telegraph, "seed {s}: he went with no tell");
        let _ = game::needs::needs_set_level(&mut p.g, b, NeedKind::Coffee, 1.2);
        let _ = game::needs::needs_set_level(&mut p.g, c, NeedKind::Coffee, 1.2);
        let lines = p.lines(a).len();
        let mut said_at = None;
        let left = p.until(lead + 2.0, |p| { if said_at.is_none() && p.lines(a).len() > lines { said_at = Some(p.g.time); } p.trip(a).map_or(true, |t| t.phase != TripPhase::Telegraph) }).expect("he leaves");
        let _ = left;
        assert!((p.g.time - go).abs() <= 0.3, "seed {s}: he went {:.2} s off the telegraph's end", p.g.time - go);
        let said = said_at.unwrap_or_else(|| panic!("seed {s}: he never said it"));
        assert!((said - (go - say)).abs() <= 0.1, "seed {s}: said {:.2} s before going, the rule asks {say}", go - said);
        p.until(45.0, |p| {
            let away = [b, c].into_iter().any(|o| p.trip(o).is_some_and(|t| t.phase != TripPhase::Telegraph));
            assert!(!away, "seed {s}: two men gone at once");
            p.trip(a).is_none()
        }).unwrap_or_else(|| panic!("seed {s}: 45 s and he is not back: {:?}", p.trip(a).map(|t| t.phase)));
    }
}

/// THE RULE: a shot with him on his way to the counter breaks the trip off inside two seconds, and he does not go back to it.
/// CONTROL: nothing heard, the same man reaches his dwell (the test before this holds that too).
/// A FAILURE MEANS: a man goes on walking to the coffee through a gunfight.
#[test]
fn a_shot_breaks_a_trip_off() {
    for s in SEEDS {
        for shot in [true, false] {
            let (mut p, [a, _, _]) = due_a_coffee(s);
            p.until(60.0, |p| p.trip(a).is_some_and(|t| t.phase == TripPhase::Going)).unwrap_or_else(|| panic!("seed {s}: he never went"));
            p.run(1.5);
            if shot {
                p.g.player_invisible = false;
                p.loud_report();
                p.until(2.0, |p| p.trip(a).is_none()).unwrap_or_else(|| panic!("seed {s}: two seconds after a shot he is still on his way: {:?}", p.trip(a).map(|t| t.phase)));
                p.run(8.0);
                assert!(p.trip(a).is_none(), "seed {s}: he went back to it");
            } else {
                p.until(40.0, |p| p.trip(a).is_some_and(|t| t.phase == TripPhase::Dwell)).unwrap_or_else(|| panic!("seed {s}: CONTROL: nothing heard and he never reached the counter"));
            }
        }
    }
}
