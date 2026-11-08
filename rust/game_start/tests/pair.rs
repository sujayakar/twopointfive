//! THE SILENT PARTNER'S WORDS (rust/game_rules/src/spotted, pairorders, silent `posture_word`, guards `say`): what the squad line says while a
//! defender is onto him, and what he says when he has finished a blown job by hand. The partner is on in the page where a quiet mission brings a
//! squad, and with `?pair`.
use game::game::GuardState;
use game::ids::GuardId;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [10, 11, 12];
const EAST: f64 = std::f64::consts::FRAC_PI_2;
const AISLE_MID: [f64; 2] = [22.0, 17.6];

/// The office with Sam and one partner on the aisle, the squad silent and falling in on Sam, one defender left on his feet with his back to them.
fn pair(seed: u32) -> (Play, GuardId, GuardId) { pair_with_body(seed, None).0 }
/// The same, and where `body` says the second defender stood there and was dropped.
fn pair_with_body(seed: u32, body: Option<[f64; 2]>) -> ((Play, GuardId, GuardId), Option<GuardId>) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad=1", &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.g.k.pairup.on = true;   // (what a quiet mission with a squad does, and `?pair`)
    game::pairup::pair_up_apply(&mut p.g, None);
    p.g.k.silent.on = true;
    p.run(0.3);
    let m = p.mates()[0];
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative).collect();
    p.drop_all_but(&foes[..if body.is_some() { 2 } else { 1 }]);
    game::silent::set_silent(&mut p.g, true, None);
    let t1 = foes[0];
    let lying = body.map(|at| {
        // (stood there for his bones to be there, then dropped)
        let t2 = foes[1];
        for _ in 0..3 { let c = p.g.gd[t2].ch; p.g.ch[c].pos = [at[0], 0.0, at[1]]; p.g.gd[t2].hold = true; p.g.gd[t2].pinned = true; pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); }
        game::combat::kill_guard(&mut p.g, t2, [0.0, 0.0, 1.0], Some(true), None, None, None, None);
        t2
    });
    pin(&mut p, t1, GuardState::Patrol, 0.0);
    p.sam_put([11.0, 18.5]);
    let c = p.g.gd[m].ch;
    p.g.ch[c].pos = [13.0, 0.0, 17.6];
    for _ in 0..10 { pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); }
    let t = game::orders::squad_orders_target_at(&mut p.g, [11.0, 0.0, 18.5]);
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "fallIn", Some(&t), Some(game::orders::Who::All), None), None, "seed {seed}: fall in was refused");
    for _ in 0..30 { pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); }
    ((p, m, t1), lying)
}
/// the defender held where he is with his back to the pair: his senses are not what is asked here
fn pin(p: &mut Play, h: GuardId, state: GuardState, aware: f64) { p.pin_man(h, AISLE_MID, EAST, state, aware); }
fn word(p: &mut Play) -> String { p.view().squad.and_then(|v| v.posture).unwrap_or_default() }

/// THE RULE: the word on the squad line says what the partner is doing about it while it lasts: 'SILENT · low' with a suspicious man about,
/// 'SILENT · contact' once an alert man has them, plain 'SILENT' before.
/// A FAILURE MEANS: the line no longer reads the partner's mode (before: `posture_word` knew only 'SILENT' and 'SILENT · compromised').
#[test]
fn the_squad_line_says_low_and_contact_while_they_last() {
    for s in SEEDS {
        let (mut p, _, t1) = pair(s);
        assert_eq!(word(&mut p), "SILENT", "seed {s}: nobody is onto them yet");
        for _ in 0..60 { pin(&mut p, t1, GuardState::Suspicious, 0.3); p.frame(); }
        assert_eq!(word(&mut p), "SILENT · low", "seed {s}: a suspicious man 10 m off: {}", p.roll());
        // (his first round in their ears makes it 'SILENT · compromised', silent's own word from then on: 'contact' is what stands until it)
        let mut seen = Vec::new();
        for _ in 0..60 { pin(&mut p, t1, GuardState::Alert, 1.0); p.g.gd[t1].foe = Some(game::game::Foe::Player); p.g.gd[t1].last_seen_t = p.g.time; p.frame(); let w = word(&mut p); if !seen.contains(&w) { seen.push(w); } }
        assert!(seen.iter().any(|w| w == "SILENT · contact"), "seed {s}: an alert man has eyes on Sam and the word went {seen:?}: {}", p.roll());
    }
}

/// A game of two with a squad each, every squad silent, all but one defender down.
fn two_seats(seed: u32) -> (Play, GuardId, Vec<Vec<GuardId>>) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, seats: 2, ..Default::default() });
    p.g.god_mode = true;
    p.g.k.pairup.on = true;
    game::pairup::pair_up_apply(&mut p.g, None);
    p.g.k.silent.on = true;
    p.run(0.3);
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative).collect();
    p.drop_all_but(&foes[..1]);
    let mut men = Vec::new();
    for i in 0..2 {
        let s = p.g.seats[i];
        game::seats::with_seat(&mut p.g, s, &mut |g| game::silent::set_silent(g, true, None));
        let mine = game::orders::squad_orders_squad(&p.g, &game::seats::seat(&p.g, s));
        assert!(!mine.is_empty(), "seed {seed}: seat {i} has no men");
        men.push(mine);
    }
    (p, foes[0], men)
}

/// THE RULE: in a game of two each player's squad line says what his own men are doing about it. A suspicious man near one seat's squad and out of reach of the
/// other's: that seat reads 'SILENT · low', and the other plain 'SILENT'. Whichever seat it is.
/// A FAILURE MEANS: the partner's rule is walking one seat's men only (before: `spotted_tick` ran for the first seat, so the second seat's line never moved).
#[test]
fn in_a_game_of_two_each_squad_line_says_low_for_its_own_men() {
    for s in SEEDS {
        for near in 0..2 {
            let (mut p, t1, men) = two_seats(s);
            let spots: [[f64; 2]; 2] = if near == 0 { [[15.0, 17.6], [3.0, 3.0]] } else { [[3.0, 3.0], [15.0, 17.6]] };
            for _ in 0..60 {
                pin(&mut p, t1, GuardState::Suspicious, 0.3);
                for i in 0..2 {
                    p.seat_stands(i, spots[i]);
                    for &m in &men[i] { p.man_stands(m, [spots[i][0] + 0.5, spots[i][1]]); }
                }
                p.frame();
            }
            let say = |p: &mut Play, i: usize| { let seat = p.g.seats[i]; game::seats::with_seat(&mut p.g, seat, &mut |g| game::silent::posture_word(g, &game::seats::here(g)).unwrap_or_default()) };
            assert_eq!(say(&mut p, near), "SILENT · low", "seed {s}: the suspicious man is near seat {near}'s men: {}", p.roll());
            assert_eq!(say(&mut p, 1 - near), "SILENT", "seed {s}: he is out of reach of seat {}'s men: {}", 1 - near, p.roll());
        }
    }
}

/// THE RULE: a blown job finished by hand ends in the partner's own words, and he falls in again, with the fight's "tango down." after them: two
/// lines said in one frame are both shown, one after the other.
/// A FAILURE MEANS: a later line of the same frame has replaced an earlier one (before: "he's down. on you." was never seen).
#[test]
fn a_blown_job_finished_by_hand_ends_in_his_own_words() {
    for s in SEEDS {
        let (mut p, m, t1) = pair(s);
        let at = p.g.ch[p.g.gd[t1].ch].pos;
        let t = game::orders::OrderTarget::Foe(game::orders::OrderTargetFoe { at, foe: t1 });
        assert_eq!(game::orders::squad_orders_give(&mut p.g, "pairMark", Some(&t), Some(game::orders::Who::All), None), None, "seed {s}: the order on the man was refused");
        let mut n = 0;
        while !p.g.pair.jobs.get(m).map_or(false, |j| j.set) && n < 30 * 60 { pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); n += 1; }
        assert!(p.g.pair.jobs.get(m).map_or(false, |j| j.set), "seed {s}: 30 s and he is not set behind the man");
        let mut n = 0;
        while p.up(t1) && n < 6 * 60 { pin(&mut p, t1, GuardState::Suspicious, 0.6); p.frame(); n += 1; }
        assert!(!p.up(t1), "seed {s}: six seconds and the man is still up");
        p.run(1.0);
        assert_eq!(p.g.stats.pairup.finishes, 1, "seed {s}: he was to be finished by hand");
        let lines = p.lines(m);
        let (a, b) = (lines.iter().position(|l| *l == "he's down. on you."), lines.iter().position(|l| *l == "tango down."));
        assert!(a.is_some() && b.is_some() && a < b, "seed {s}: both lines, his own first: {lines:?}");
        let (ta, tb) = { let at = |w: &str| p.said.iter().find(|x| x.who == m && x.text == w).unwrap().t; (at("he's down. on you."), at("tango down.")) };
        assert!(tb - ta > 0.9, "seed {s}: the second came {:.2} s after the first: nobody had time to read it", tb - ta);
        assert!(p.g.pair.jobs.get(m).is_none(), "seed {s}: the job goes on");
    }
}

/// THE RULE: a man in the partner's hands finds no body, and from the frames after the hands are off he does. Held turned to a colleague lying
/// behind him, shot partner and all. (The old check read this as a fault because it stopped turning him once the partner was shot, so he looked away.)
/// A FAILURE MEANS: the hold on the body outlives the hands, or the grip no longer hides it.
#[test]
fn the_hold_on_a_body_ends_with_the_hands() {
    let body = [20.4, 18.1];
    let ((mut p, m, t1), t2) = pair_with_body(10, Some(body));
    let t2 = t2.unwrap();
    let at = p.g.ch[p.g.gd[t1].ch].pos;
    let t = game::orders::OrderTarget::Foe(game::orders::OrderTargetFoe { at, foe: t1 });
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "pairMark", Some(&t), Some(game::orders::Who::All), None), None, "the order on the man was refused");
    let mut n = 0;
    while !p.g.pair.jobs.get(m).map_or(false, |j| j.set) && n < 30 * 60 { pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); n += 1; }
    game::pairorders::pair_orders_mark(&mut p.g, Some(game::pairorders::PairOrdersMarkBy::Api));
    let mut n = 0;
    while !p.g.pair.jobs.get(m).map_or(false, |j| j.t0 >= 0.0) && n < 5 * 60 { pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); n += 1; }
    assert!(game::pairorders::pair_gripped(&mut p.g, t1), "his hands never went on");
    let to_body = (body[0] - AISLE_MID[0]).atan2(body[1] - AISLE_MID[1]);
    let turn = |p: &mut Play| { let c = p.g.gd[t1].ch; p.g.ch[c].body_yaw = to_body; p.g.ch[c].aim_yaw = to_body; p.g.gd[t1].hold = true; p.g.gd[t1].pinned = true; };
    for _ in 0..5 { turn(&mut p); p.frame(); assert!(!game::guards::body_seen(&mut p.g, t1, t2, None), "with hands on him he found the body"); }
    game::combat::kill_guard(&mut p.g, m, [1.0, 0.0, 0.0], Some(false), None, None, None, None);
    let mut seen = 0;
    for _ in 0..54 { turn(&mut p); p.frame(); assert!(!game::pairorders::pair_gripped(&mut p.g, t1), "the hands are off and the grip stands"); if game::guards::body_seen(&mut p.g, t1, t2, None) { seen += 1; } }
    assert!(seen > 30, "turned to the body with the hands off, he saw it on {seen} frames of 54");
}

// ---------------------------------------------------------------- is this way seen

/// The office with Sam and one partner, the squad silent, the one defender left held where `at` says with his back to `yaw`'s far side; the partner sent to take
/// him on the mark. `oneeye`: the pair rules' own switch, as a quiet mission throws it; off, the orders alone, as `?pairorders`.
fn sent_to_a_back(oneeye: bool, at: [f64; 2], yaw: f64) -> (Play, GuardId, GuardId) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear=10&squad=1", &Opts { seed: 10, ..Default::default() });
    p.g.god_mode = true;
    p.g.fog_of_war.on = false;   // (whether Sam can see him to give the order is not what is asked)
    p.g.k.pair.on = true;
    if oneeye { p.g.k.pairup.on = true; game::pairup::pair_up_apply(&mut p.g, None); }
    p.g.k.silent.on = true;
    p.run(0.5);
    game::silent::set_silent(&mut p.g, true, None);
    let m = p.mates()[0];
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative).collect();
    let foe = foes[0];
    p.drop_all_but(&foes[..1]);
    for _ in 0..5 { p.pin_man(foe, at, yaw, GuardState::Patrol, 0.0); p.frame(); }
    let t = game::orders::OrderTarget::Foe(game::orders::OrderTargetFoe { at: [at[0], 0.0, at[1]], foe });
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "pairMark", Some(&t), None, None), None, "the order was refused");
    (p, m, foe)
}
fn status_of(p: &mut Play, m: GuardId) -> String { game::orders::squad_orders_status_of(&mut p.g, m) }

/// THE RULE: the only way to a man's back lies along the corridor he is looking down: the partner does not set out, the word on his row names who has him, and when
/// the wait is up he says he can't get there. The same with the pair rules' switch on or off: the way has one eye.
/// A FAILURE MEANS: a partner walks into a man's face, or (before the second test of the floor went) says 'no way' with nobody named on the orders alone.
#[test]
fn a_way_across_his_eyes_is_waited_on_by_one_eye() {
    for oneeye in [true, false] {
        let west = -std::f64::consts::FRAC_PI_2;
        let (mut p, m, foe) = sent_to_a_back(oneeye, [33.5, 11.3], west);
        let start = p.g.ch[p.g.gd[m].ch].pos;
        let t0 = p.time();
        for _ in 0..60 { p.pin_man(foe, [33.5, 11.3], west, GuardState::Patrol, 0.0); p.frame(); }
        let word = status_of(&mut p, m);
        assert_eq!(word, "waiting · Tango 1 is facing this way", "switch {oneeye}: a second in");
        let mut done = None;
        for _ in 0..(60 * 22) {
            p.pin_man(foe, [33.5, 11.3], west, GuardState::Patrol, 0.0);
            p.frame();
            if game::pairorders::pair_orders_job_of(&p.g, m).is_none() { done = Some(p.time() - t0); break; }
        }
        assert!(done.is_some(), "switch {oneeye}: he never gave it up");
        assert!(p.lines(m).contains(&"can't get behind him unseen."), "switch {oneeye}: said {:?}", p.lines(m));
        let end = p.g.ch[p.g.gd[m].ch].pos;
        assert!((end[0] - start[0]).hypot(end[2] - start[2]) < 0.5, "switch {oneeye}: he walked {start:?} to {end:?} on a way he could not take");
    }
}

/// THE RULE: where there is a way round, he takes it. The man in the break room faces the door to the aisle; the floor behind him is dark and there is a second door to
/// it. The partner goes round by that door, sets up at the man's back and never stands in front of him.
/// A FAILURE MEANS: a partner gives up, or goes the short way across the man's eyes, with a dark way round to be had.
#[test]
fn a_dark_way_round_is_a_way() {
    for oneeye in [true, false] {
        let west = -std::f64::consts::FRAC_PI_2;
        let at = [32.3, 17.95];
        let (mut p, m, foe) = sent_to_a_back(oneeye, at, west);
        let mut in_front = 0;
        let mut set = false;
        for _ in 0..(60 * 45) {
            p.pin_man(foe, at, west, GuardState::Patrol, 0.0);
            p.frame();
            let q = p.g.ch[p.g.gd[m].ch].pos;
            if (q[0] - at[0]).hypot(q[2] - at[1]) < 2.6 && q[0] < at[0] { in_front += 1; }
            if game::pairorders::pair_orders_job_of(&p.g, m).map_or(false, |j| j.set) { set = true; break; }
        }
        assert!(set, "switch {oneeye}: he never got there: {} said {:?}", status_of(&mut p, m), p.lines(m));
        assert_eq!(in_front, 0, "switch {oneeye}: frames in front of him inside 2.6 m");
        let q = p.g.ch[p.g.gd[m].ch].pos;
        assert!(q[0] > at[0] && (q[0] - at[0]).hypot(q[2] - at[1]) < 1.6, "switch {oneeye}: set at {q:?}, the man at {at:?}, looking west");
    }
}
