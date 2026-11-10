//! HOW SAM'S PARTNER FOLLOWS WHILE THE PAIR IS SILENT AND NOBODY HAS NOTICED IT.
mod stealth_ask;
use game::game::GuardState;
use game::ids::GuardId;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];
const EAST: f64 = std::f64::consts::FRAC_PI_2;

/// The pair on the office's room clear, silent and fallen in on Sam at the corridor's west end; one man left on the floor, in the break room, his back to its west door; the
/// west door shut.
fn the_pair(s: u32) -> (Play, GuardId, GuardId, [f64; 2]) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad=1", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.pair_silent();
    p.run(0.3);
    let m = p.mates()[0];
    let foes: Vec<GuardId> = p.hostiles();
    let t = foes[0];
    p.drop_all_but(&[t]);
    let beyond = [32.25, 17.75 + 0.3 * nudge(s, 0)];
    p.door_shut("breakroom_w");
    p.pin_man(t, beyond, EAST, GuardState::Patrol, 0.0);
    p.sam_put([20.0, 17.75]);
    p.man_stands(m, [19.0, 17.75]);
    p.run(0.3);
    let at = p.sam_at();
    p.order("fallIn", [at[0] - 0.6, at[1]], Who::All);
    p.run(0.6);
    (p, m, t, beyond)
}

/// THE RULE: low when Sam is low and at his pace, upright when he is. Sam crouched and creeping: the partner makes the walk low, never over the low pace. Sam upright: he is
/// upright, never over the silent walk.
/// A FAILURE MEANS: a partner upright beside a creeping Sam gives the pair away, or one crouched beside a walking Sam falls behind.
#[test]
fn the_partner_is_low_when_sam_is_low_and_upright_when_he_is() {
    for s in SEEDS {
        for low in [true, false] {
            let (mut p, m, _, _) = the_pair(s);
            if low { p.key_down("KeyC"); p.frame(); p.key_up("KeyC"); p.run(0.5); }
            let (lo_pace, walk) = (p.g.k.sneak.low_pace, p.g.k.silent.pace);
            let (mut moving, mut was_low, mut top) = (0, 0, 0.0f64);
            let to = [24.0 + nudge(s, 1), 17.75];
            for _ in 0..8 {
                p.sam_goes_to(to, 1.0);
                p.run(0.05);
                if p.speed(m) > 0.3 { moving += 1; if p.low(m) { was_low += 1; } }
                top = top.max(p.speed(m));
                if dist(p.sam_at(), to) < 0.3 { break; }
            }
            assert!(moving >= 2, "seed {s}, Sam low {low}: the partner hardly moved ({moving}): {}", p.why(m));
            assert_eq!(was_low == moving, low, "seed {s}, Sam low {low}: the partner low on {was_low} of {moving} looks");
            assert!(top <= if low { lo_pace } else { walk } + 0.1, "seed {s}, Sam low {low}: the partner's top pace {top:.2}");
        }
    }
}

/// THE RULE: at a shut door with a man beyond it the partner is out of the doorway's line before the leaf moves, still, and close to Sam; and the man looking at the door as it
/// opens does not take him in.
/// A FAILURE MEANS: the partner stands in the doorway's line when the door opens, and the first thing the man beyond sees is him.
#[test]
fn the_partner_is_out_of_the_doorways_line_before_a_shut_door_with_a_man_beyond_it_opens() {
    for s in SEEDS {
        let (mut p, m, t, beyond) = the_pair(s);
        p.key_down("KeyC");
        p.frame();
        p.key_up("KeyC");
        let door = p.door_at("breakroom_w");
        assert!(p.sam_goes_to([door[0] - 0.8, door[1]], 14.0), "seed {s}: Sam never got to the door: he is at {:?}", p.sam_at());
        for _ in 0..(60 * 5) { p.pin_man(t, beyond, EAST, GuardState::Patrol, 0.0); p.frame(); }
        let at = p.at(m);
        assert!(!p.in_doors_line("breakroom_w", beyond, at), "seed {s}: the partner stands at {at:?}, in the door's line from the man at {beyond:?}");
        assert!(p.speed(m) < 0.1, "seed {s}: he is still moving ({:.2} m/s)", p.speed(m));
        assert!(dist(at, p.sam_at()) < 5.0, "seed {s}: he stands {:.1} m from Sam", dist(at, p.sam_at()));
        p.door_stands_open("breakroom_w");
        for _ in 0..90 { p.pin_man(t, beyond, -EAST, GuardState::Patrol, 0.0); p.frame(); assert!(!p.eyes_take_in(t, m), "seed {s}: the man turned to the door takes the partner in, at {:?}", p.at(m)); }
        drop(p);

        // CONTROL: the same walk with nobody beyond the door: he has no reason to leave Sam's trail, and stays behind him on it, in that line
        let (mut p, m, t, beyond) = the_pair(s);
        p.falls(t, game::game::DownBy::Player);
        p.key_down("KeyC");
        p.frame();
        p.key_up("KeyC");
        assert!(p.sam_goes_to([door[0] - 0.8, door[1]], 14.0), "seed {s}: control: Sam never got to the door");
        p.run(5.0);
        assert!(dist(p.at(m), p.sam_at()) < 3.5, "seed {s}: control: he is {:.1} m from Sam", dist(p.at(m), p.sam_at()));
        assert!(p.in_doors_line("breakroom_w", beyond, p.at(m)), "seed {s}: control: with nobody beyond it he left Sam's trail all the same, for {:?}", p.at(m));
    }
}

/// Lights out as two: the mains thrown by the partner on order, a man sent to the panel, Sam calling fall in and walking on at a sneak's pace; the man's errand left on him, or (`to_a_sound`) taken off so that
/// he walks to the same place as a man who heard something there. What became of the follower while that man was coming: (longest still on his walk with Sam gone on, metres from Sam at the end, frames on the
/// walk, of them low, frames with the man on his way, rousing anybody).
fn off_the_walk(s: u32, to_a_sound: bool) -> (f64, f64, usize, usize, Option<String>) {
    const WAY: [[f64; 2]; 11] = [[9.3, 16.7], [9.3, 14.8], [8.7, 12.1], [9.0, 9.3], [11.0, 8.0], [13.7, 7.4], [16.1, 7.4], [17.0, 8.5], [17.1, 10.0], [17.3, 12.5], [17.7, 14.6]];
    let mut p = Play::mission("lights-out.json", "?levelFile=lights-out.json&quiet&squad=1", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.pair_silent();
    p.g.k.spotted.on = true;
    p.g.player_invisible = to_a_sound;   // (Sam's lead runs up that man's way: how near they meet is the dice's. Asked is the follower)
    let b = p.mates()[0];
    p.sam_put(WAY[0]);
    p.man_stands(b, [9.9 + 0.3 * nudge(s, 3), 17.3]);
    p.key_down("KeyC");
    p.frame();
    p.key_up("KeyC");
    p.run(1.0);
    p.throws_the_breaker();
    p.until(25.0, |p| p.mains_out()).unwrap_or_else(|| panic!("seed {s}: 25 s and the partner has not thrown the breaker: {}", p.why(b)));
    let man = p.sent_to_the_breaker().unwrap_or_else(|| panic!("seed {s}: the mains out and nobody sent to the panel"));
    let spot = p.errand_at(man);
    let t0 = p.time();
    let walk_on = |p: &mut Play| if to_a_sound { p.walks_to_a_sound(man, spot); };
    for _ in 0..72 { walk_on(&mut p); p.frame(); }
    p.order("fallIn", [9.0, 17.5], Who::All);
    let (mut still, mut most, mut on, mut low, mut seg) = (0.0f64, 0.0f64, 0, 0, 0);
    let mut roused = None;
    for _ in 0..(26 * 60) {
        walk_on(&mut p);
        if p.time() - t0 > 5.2 && seg < WAY.len() - 1 && !p.sam_goes_to_step(WAY[seg + 1]) { seg += 1; } else { p.frame(); }
        let v = p.speed(b);
        let coming = p.up(man) && p.follower_on_walk(b, man, to_a_sound);
        if coming { on += 1; if p.low(b) { low += 1; } }
        if coming && v < 0.1 && dist(p.at(b), p.sam_at()) > 3.0 { still += 1.0 / 60.0; most = most.max(still); } else { still = 0.0; }
        if roused.is_none() { roused = p.g.guards.iter().find(|&&x| !p.g.team(x).ours() && matches!(p.state(x), GuardState::Alert | GuardState::Search)).map(|&x| p.g.gd[x].callsign.clone()); }
    }
    (most, dist(p.at(b), p.sam_at()), on, low, roused)
}

/// THE RULE: a follower is not left standing on the walk of the man who comes to the breaker: he does not stand on it with Sam gone on, he never runs, he stays low with Sam, and he ends at Sam's shoulder.
/// A FAILURE MEANS: the partner waits for that man's back on the very stretch the man is walking.
#[test]
fn a_follower_is_not_left_standing_on_the_walk_of_the_man_sent_to_the_breaker() {
    for s in SEEDS {
        let (most, gap, on, low, _) = off_the_walk(s, false);
        assert!(most < 1.0, "seed {s}: still on the walk for {most:.2} s");
        assert!(gap < 3.0, "seed {s}: he ends {gap:.2} m from Sam");
        assert!(low * 10 >= on * 9, "seed {s}: low on {low} of {on} frames on the walk");
    }
}

/// THE RULE: the same of a man who comes to a sound, with no errand: the hole the breaker's rule left. Nobody is roused by it.
/// A FAILURE MEANS: the follower is walked into by a man who heard something.
#[test]
fn a_follower_is_not_left_standing_on_the_walk_of_a_man_who_comes_to_a_sound() {
    for s in SEEDS {
        let (most, gap, on, low, roused) = off_the_walk(s, true);
        assert!(most < 1.0 && gap < 3.0, "seed {s}: still on the walk for {most:.2} s, {gap:.2} m from Sam at the end");
        assert!(on > 60 && low * 10 >= on * 9, "seed {s}: low on {low} of {on} frames on the walk");
        assert!(roused.is_none(), "seed {s}: {roused:?} was roused");
    }
}

/// The pair, one man left, lit, the partner `d` metres in front of him and in his line, Sam a stride off: the man held as the state says, looking at the partner.
fn at_arms_length(s: u32, d: f64, state: GuardState) -> (Play, GuardId, GuardId, [f64; 2]) {
    let (mut p, m, t, _) = the_pair(s);
    p.g.k.spotted.on = true;
    let foe = [14.25 + 0.5 * nudge(s, 2).round(), 13.25];
    p.door_shut("breakroom_w");
    let at = [foe[0] + d, foe[1]];
    p.sam_put([foe[0] + d + 1.0, foe[1] + 2.2]);
    p.man_stands(m, at);
    p.run(0.2);
    p.man_stands(m, at);
    p.pin_man(t, foe, EAST, state, 0.6);
    (p, m, t, foe)
}

/// THE RULE: a roused man at arm's length is taken by hand, quietly: suspicious and facing the partner two metres off he is out cold inside three seconds, struck, not a round fired. CONTROL: a man already alert,
/// strides away: never the hands, the gun.
/// A FAILURE MEANS: the partner answers a man who has turned to him with a loud rifle, or runs across a room at a man who has him covered.
#[test]
fn a_roused_man_at_arms_length_is_taken_by_hand_quietly() {
    for s in SEEDS {
        let (mut p, m, t, foe) = at_arms_length(s, 1.8, GuardState::Suspicious);
        let fired = p.rounds_fired(m);
        let down = p.until(3.0, |p| { if p.up(t) { p.pin_man(t, foe, EAST, GuardState::Suspicious, 0.6); } !p.up(t) });
        assert!(down.is_some(), "seed {s}: three seconds and he is up: {}", p.why(m));
        assert_eq!(p.down_kind(t), Some(game::game::DownKind::Struck), "seed {s}: how he went down");
        assert_eq!(p.rounds_fired(m), fired, "seed {s}: rounds fired");
        drop(p);

        let (mut p, m, t, foe) = at_arms_length(s, 6.0, GuardState::Alert);
        let fired = p.rounds_fired(m);
        let at = p.at(m);
        for _ in 0..(60 * 6) { if !p.up(t) { break; } p.pin_man(t, foe, EAST, GuardState::Alert, 1.0); p.frame(); }
        assert!(p.up(t) || p.down_kind(t) != Some(game::game::DownKind::Struck), "seed {s}: an alert man strides off taken by hand");
        assert!(p.rounds_fired(m) > fired, "seed {s}: he never fired on an alert man at {:.1} m from {at:?}: {}", dist(p.at(m), foe), p.why(m));
    }
}

/// THE RULE: which rounds are held: on a suspicious man, whatever the awareness; on an alert man turned away, far off, no eyes on them. Not once he faces them, or the floor has gone loud.
/// A FAILURE MEANS: the partner opens fire on a man who is only wondering, or sits on his rounds with a man looking at him.
#[test]
fn which_rounds_the_silent_partner_holds() {
    for s in SEEDS {
        let (mut p, m, t, foe) = at_arms_length(s, 10.0, GuardState::Suspicious);
        let pose = |p: &mut Play, st: GuardState, aw: f64, yaw: f64| { p.pin_man(t, foe, yaw, st, aw); p.run(0.03); p.pin_man(t, foe, yaw, st, aw); };
        pose(&mut p, GuardState::Suspicious, 0.4, EAST);
        assert!(p.holds_round(m, t), "seed {s}: suspicious at 0.4");
        pose(&mut p, GuardState::Suspicious, 0.7, EAST);
        assert!(p.holds_round(m, t), "seed {s}: suspicious at 0.7");
        pose(&mut p, GuardState::Alert, 1.0, -EAST);
        p.g.gd[t].foe = None;
        p.g.gd[t].last_seen_t = -100.0;
        assert!(p.holds_round(m, t), "seed {s}: alert, turned away, far off");
        pose(&mut p, GuardState::Alert, 1.0, EAST);
        p.g.gd[t].foe = None;
        p.g.gd[t].last_seen_t = -100.0;
        assert!(!p.holds_round(m, t), "seed {s}: alert and facing them");
        pose(&mut p, GuardState::Alert, 1.0, -EAST);
        p.g.gd[t].foe = None;
        p.g.gd[t].last_seen_t = -100.0;
        p.g.fight.hot_t = p.g.time;
        p.run(0.03);
        assert!(!p.holds_round(m, t), "seed {s}: a floor gone loud holds nothing");
    }
}
