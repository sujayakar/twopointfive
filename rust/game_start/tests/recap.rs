//! EVERY DEATH EXPLAINS ITSELF, AND THE GAME KEEPS IT (rust/game_rules/src/deathrecap): who of ours fell, to whom, from where, with what, and the floor's last seconds,
//! read off the game's own record. Each test holds one rule over three seeds beside a control that differs in the one thing the rule is about. Before the tick kept the
//! recap the page's TypeScript did, on its copy of the game, with a word of its own over a teammate's head: nothing here could have been asked of the game at all.
use game::deathrecap::{RecapCause, RecapDeath};
use game::game::{DownBy, DownKind, MissionStage};
use game::ids::GuardId;
use game_start::play::{dist, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;   // (a yaw: 0 looks south, along +z)
/// a number in -1..1 that is a fixed function of the seed and of `k`
fn nudge(seed: u32, k: u32) -> f64 {
    let mut x = seed.wrapping_mul(0x9E37_79B9) ^ k.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 16; x = x.wrapping_mul(0x7FEB_352D); x ^= x >> 15; x = x.wrapping_mul(0x846C_A68B); x ^= x >> 16;
    x as f64 / 2_147_483_648.0 - 1.0
}
fn off(seed: u32, k: u32, at: [f64; 2]) -> [f64; 2] { [at[0] + 0.3 * nudge(seed, k), at[1] + 0.3 * nudge(seed, k + 1)] }

/// WHERE THE MEN STAND: two rooms side by side with a shut door between them and a door each to the yard south of them, lit; a defender in each; Sam and his pair where a
/// test says. The west room's defender has a clear line across his room. The AI is off, so that the only rounds are the test's: what the recap reads is what the test did.
const SHOOTER: [f64; 2] = [16.5, 11.0];
const VICTIM: [f64; 2] = [12.0, 14.0];
/// in the east room: a wall and a shut door from anything in the west room
const EAST: [f64; 2] = [22.0, 14.0];
/// in the west room with a line to the man down
const IN_THE_ROOM: [f64; 2] = [15.0, 15.0];
/// in the yard, no line into the room, under 8 m from the man down
const YARD_NEAR: [f64; 2] = [14.0, 19.0];
/// in the yard, more than 8 m from the man down
const YARD_FAR: [f64; 2] = [24.0, 21.0];

fn stood(seed: u32, seats: u32, sam: [f64; 2], victim: [f64; 2], mate: [f64; 2]) -> Play {
    let mut s = Scene::new("recap");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team(sam, NORTH, &[victim, mate, [13.0, 25.0], [15.0, 25.0]]);
    s.defender("west", off(seed, 0, SHOOTER), "west-yard").defender("east", [24.25, 11.25], "east-yard");
    let mut p = Play::scene(&s, &Opts { seed, seats, squad: 2, ..Default::default() });
    p.g.ai_enabled = false;
    p.run(0.2);
    p
}
fn shooter(p: &Play) -> GuardId { p.hostiles_by(SHOOTER)[0] }
fn pair(p: &Play) -> (GuardId, GuardId) { let m = p.mates(); (m[0], m[1]) }
fn last_death(p: &Play) -> RecapDeath { p.recap_deaths().last().cloned().expect("a death is kept") }
fn rounds_into(p: &mut Play, by: GuardId, man: GuardId) {
    for _ in 0..40 {
        if p.state(man) == game::game::GuardState::Dead { return; }
        let at = p.at(man);
        p.man_fires_at(by, at);
        p.frame();
    }
    panic!("forty rounds and he is still up: {}", p.roll());
}
fn rounds_into_seat(p: &mut Play, by: GuardId, seat: usize) {
    for _ in 0..40 {
        if p.seat_down(seat) { return; }
        let at = p.seat_at(seat);
        p.man_fires_at(by, at);
        p.frame();
    }
    panic!("forty rounds and seat {} is still up: {}", seat, p.roll());
}

/// A FAILURE MEANS: the card opens while a player still plays, or it no longer names who, from where and with what, or how far off his gaze it came.
#[test]
fn the_card_is_shut_while_he_plays_and_names_the_man_who_shot_him_when_he_is_down() {
    for seed in SEEDS {
        let sam = off(seed, 2, [12.0, 12.0]);
        let mut p = stood(seed, 1, sam, YARD_NEAR, EAST);
        let by = shooter(&p);
        p.point_at([sam[0], 2.0]);   // Sam looks north, the shooter off to his east: then he is looked for, then shot
        p.run(1.0);
        assert!(p.recap_card().is_none(), "seed {}: nothing is handed out while he is up", seed);
        rounds_into_seat(&mut p, by, 0);
        let card = p.recap_card().expect("he is down: the card has his record");
        let d = &card.takes[card.main as usize].death;
        assert!(d.player && d.victim == p.seat_id(0), "seed {}: the take it opens on is his: {:?}", seed, d);
        assert_eq!(d.cause, RecapCause::Round, "seed {}", seed);
        assert_eq!(d.by, p.id_of(by), "seed {}: it names the man", seed);
        assert_eq!(d.by_name, p.g.gd[by].callsign, "seed {}: by his name", seed);
        assert!(!d.weapon.is_empty(), "seed {}: and his gun", seed);
        assert!(dist(d.from.unwrap(), p.at(by)) < 1.5, "seed {}: the muzzle the round left is at the shooter: {:?} against {:?}", seed, d.from, p.at(by));
        assert!(dist(d.by_at.unwrap(), p.at(by)) < 0.01, "seed {}: and where he stood", seed);
        assert!((d.dist - dist(d.from.unwrap(), d.at)).abs() < 1e-6, "seed {}: how far", seed);
        assert!(d.loud, "seed {}: a rifle's report is a thing ears place", seed);
        drop(card);
        drop(p);
        // CONTROL: he goes down and nobody fired: the card opens, and it names nobody.
        let mut q = stood(seed, 1, sam, YARD_NEAR, EAST);
        q.run(1.0);
        q.seat_goes_down(0);
        q.frame();
        let card = q.recap_card().expect("he is down");
        let d = &card.takes[card.main as usize].death;
        assert!(d.player && d.cause == RecapCause::Unknown && d.by == -1.0 && d.by_name.is_empty() && d.from.is_none(), "seed {}: nobody fired, nobody is named: {:?}", seed, d);
    }
}

/// A FAILURE MEANS: the angle between his gaze and the way it came no longer tells a round from behind from one he was looking at.
#[test]
fn the_card_knows_whether_it_came_from_behind_him() {
    for seed in SEEDS {
        let sam = off(seed, 3, [12.0, 12.0]);
        let mut p = stood(seed, 1, sam, YARD_NEAR, EAST);
        let by = shooter(&p);
        let (s, t) = (p.at(by), [sam[0], sam[1]]);
        // he looks straight away from the shooter
        p.point_at([t[0] - (s[0] - t[0]), t[1] - (s[1] - t[1])]);
        p.run(1.5);
        rounds_into_seat(&mut p, by, 0);
        let away = last_death(&p).off;
        drop(p);
        // CONTROL: the same round with him looking at the shooter
        let mut q = stood(seed, 1, sam, YARD_NEAR, EAST);
        let by = shooter(&q);
        let at = q.at(by);
        q.point_at(at);
        q.run(1.5);
        rounds_into_seat(&mut q, by, 0);
        let at_him = last_death(&q).off;
        assert!(away > 120.0 && at_him < 60.0, "seed {}: looking away it came {} degrees off his gaze, looking at him {}", seed, away, at_him);
    }
}

/// A FAILURE MEANS: a second player would be told what the first one's card knows while he is still playing: the shooter's place, the floor without its fog.
#[test]
fn in_a_game_of_two_the_card_stays_shut_until_both_are_down() {
    for seed in SEEDS {
        let sam = off(seed, 4, [12.0, 12.0]);
        let mut p = stood(seed, 2, sam, YARD_NEAR, EAST);
        let by = shooter(&p);
        p.seat_stands(1, [11.0, 15.0]);
        p.run(1.0);
        rounds_into_seat(&mut p, by, 0);
        p.run(1.5);
        assert!(p.seat_down(0) && !p.seat_down(1), "seed {}: the first down, the second up", seed);
        assert_eq!(last_death(&p).name, "Player 1", "seed {}: he is kept, by his name", seed);
        assert!(p.recap_card().is_none(), "seed {}: and nothing is handed out while the second plays", seed);
        let second = p.seat_id(1);
        p.seat_goes_down(1);
        p.frame();
        let card = p.recap_card().expect("both are down");
        assert_eq!(card.takes.len(), 2, "seed {}: a take a man", seed);
        assert_eq!(card.takes[card.main as usize].death.victim, second, "seed {}: it opens on the last player to go down", seed);
        drop(card);
        drop(p);
        // CONTROL: with one seat the same fall opens it at once.
        let mut q = stood(seed, 1, sam, YARD_NEAR, EAST);
        let by = shooter(&q);
        rounds_into_seat(&mut q, by, 0);
        assert!(q.recap_card().is_some(), "seed {}: alone, his fall ends the run", seed);
    }
}

/// A FAILURE MEANS: the card's playback is short of a man, of the round, or of the second after the fall; or a card raised the frame he falls has nothing to show.
#[test]
fn a_take_carries_the_floors_last_seconds() {
    for seed in SEEDS {
        let sam = off(seed, 5, [12.0, 12.0]);
        let mut p = stood(seed, 1, sam, YARD_NEAR, EAST);
        let by = shooter(&p);
        p.run(10.0);
        rounds_into_seat(&mut p, by, 0);
        let early = p.recap_card().expect("the frame he falls");
        let e = &early.takes[0];
        assert!(e.frames.last().unwrap().t - e.death.t < 0.1, "seed {}: what there is of it, with no second after the fall yet", seed);
        drop(early);
        p.run(1.7);
        let card = p.recap_card().expect("he is down");
        assert_eq!(card.takes.len(), 1, "seed {}: the whole take in place of the early one: {:?}", seed, card.takes.iter().map(|t| (t.death.name.clone(), t.death.t, t.death.cause, t.death.by_name.clone())).collect::<Vec<_>>());
        let t = &card.takes[0];
        let men = 1.0 + p.g.guards.len() as f64;
        let span = t.frames.last().unwrap().t - t.frames[0].t;
        let tail = t.frames.last().unwrap().t - t.death.t;
        assert!(t.frames.iter().all(|f| f.n == men && f.men.len() == men as usize * game::deathrecap::MAN_STRIDE), "seed {}: every man in every frame", seed);
        assert!(t.frames.iter().all(|f| f.doors.len() == p.g.doors.list.len() * 2), "seed {}: and every leaf", seed);
        assert!((span - 9.2).abs() < 0.11, "seed {}: eight seconds before the fall and one and a bit after it: {}", seed, span);
        assert!((tail - 1.2).abs() < 0.11, "seed {}: {} s past the fall", seed, tail);
        assert!(t.shots.iter().any(|s| s.who == p.id_of(by) && s.to.is_some()), "seed {}: his round is in it", seed);
        assert_eq!(t.cast.len() as f64, men, "seed {}: and every man is named", seed);
        assert_eq!(t.doors.len(), p.g.doors.list.len(), "seed {}", seed);
        assert_eq!(p.g.stats.deathrecap.takes, 1, "seed {}: one take filed", seed);
        drop(card);
        drop(p);
        // CONTROL: a short run: only the frames that exist, never more than the record keeps.
        let mut q = stood(seed, 1, sam, YARD_NEAR, EAST);
        let by = shooter(&q);
        q.run(2.0);
        rounds_into_seat(&mut q, by, 0);
        q.run(1.7);
        let card = q.recap_card().expect("he is down");
        let span = card.takes[0].frames.last().unwrap().t - card.takes[0].frames[0].t;
        assert!(span < 5.0, "seed {}: there was no eight seconds to keep: {}", seed, span);
    }
}

/// A FAILURE MEANS: a teammate lost while a player plays is forgotten, or told; or a man of the other side is taken for one of ours.
#[test]
fn one_of_ours_lost_is_kept_and_handed_out_to_nobody_and_a_defender_is_not_kept() {
    for seed in SEEDS {
        let mut p = stood(seed, 1, YARD_NEAR, VICTIM, EAST);
        let by = shooter(&p);
        let (a, _) = pair(&p);
        p.run(0.5);
        rounds_into(&mut p, by, a);
        p.frame();
        assert_eq!(p.recap_deaths().len(), 1, "seed {}", seed);
        let d = last_death(&p);
        assert!(!d.player && d.victim == p.id_of(a) && d.by == p.id_of(by) && d.cause == RecapCause::Round, "seed {}: {:?}", seed, d);
        p.run(2.0);
        assert!(p.recap_card().is_none(), "seed {}: Sam plays on: nothing is handed out", seed);
        // CONTROL: the other side going down is nobody's death to explain
        p.falls(by, DownBy::Player);
        p.run(0.5);
        assert_eq!(p.recap_deaths().len(), 1, "seed {}: a defender down is not kept", seed);
    }
}

/// A FAILURE MEANS: a man put out cold by a hand is explained as if he had been shot.
#[test]
fn a_man_put_out_cold_is_not_a_death_to_explain() {
    for seed in SEEDS {
        for how in [DownKind::Struck, DownKind::Choked] {
            let mut p = stood(seed, 1, YARD_NEAR, VICTIM, EAST);
            let (a, _) = pair(&p);
            p.falls_as(a, how, true, DownBy::Player);
            p.run(0.5);
            assert!(p.recap_deaths().is_empty(), "seed {}: {:?} is nobody shot: {:?}", seed, how, p.recap_deaths());
            drop(p);
        }
        // CONTROL: shot, the same man is kept
        let mut q = stood(seed, 1, YARD_NEAR, VICTIM, EAST);
        let (a, _) = pair(&q);
        q.falls_as(a, DownKind::Shot, false, DownBy::Guard);
        q.run(0.5);
        assert_eq!(q.recap_deaths().len(), 1, "seed {}: shot, he is", seed);
    }
}

/// A FAILURE MEANS: a stun can going off beside a man who drops is named his killer, or a fragmentation burst is not.
#[test]
fn a_burst_is_named_and_a_stun_can_is_not() {
    for seed in SEEDS {
        let mut p = stood(seed, 1, YARD_FAR, VICTIM, EAST);
        let (a, _) = pair(&p);
        let at = p.at(a);
        p.frag_lands([at[0] + 0.6, at[1]]);
        p.until(8.0, |p| p.state(a) == game::game::GuardState::Dead).expect("a burst at his feet puts him down");
        p.frame();
        let d = last_death(&p);
        assert_eq!(d.cause, RecapCause::Blast, "seed {}: {:?}", seed, d);
        assert!(d.by_name.is_empty(), "seed {}: a can dropped by the sandbox's call is nobody's: {:?}", seed, d);
        assert!(d.from.is_some() && d.loud && d.weapon.is_empty(), "seed {}: {:?}", seed, d);
        drop(p);
        // CONTROL: a flash goes off beside him; he drops in the same breath, by nothing anybody can name.
        let mut q = stood(seed, 1, YARD_FAR, VICTIM, EAST);
        let (a, _) = pair(&q);
        let at = q.at(a);
        game::game::game_drop_throwable(&mut q.g, game::game::ThrowKind::Flash, [at[0] + 0.6, 0.0, at[1]]);
        q.until(8.0, |q| q.g.events.iter().any(|e| e.kind == game::director::EventKind::Bang)).expect("the flash goes off");
        q.falls_as(a, DownKind::Shot, false, DownBy::World);
        q.frame();
        let d = last_death(&q);
        assert_eq!(d.cause, RecapCause::Unknown, "seed {}: a stun can kills nobody: {:?}", seed, d);
    }
}

/// A FAILURE MEANS: a hostage shot the way a captor shoots one (an AI round never tests a civilian: his round goes through her and the game puts her down by hand) is put
/// down to nobody.
#[test]
fn a_man_dropped_on_a_rounds_line_is_that_rounds() {
    for seed in SEEDS {
        let mut p = stood(seed, 1, YARD_FAR, VICTIM, EAST);
        let by = shooter(&p);
        let (a, _) = pair(&p);
        let at = p.at(a);
        // a round that passes him a hand's width off, with nobody's name on it, and in the same breath he is down
        p.man_fires_at(by, [at[0], at[1] + 0.4]);
        p.falls_as(a, DownKind::Shot, false, DownBy::World);
        p.frame();
        let d = last_death(&p);
        assert!(d.cause == RecapCause::Round && d.by == p.id_of(by) && !d.weapon.is_empty() && d.from.is_some(), "seed {}: {:?}", seed, d);
        drop(p);
        // CONTROL: the round goes well wide of him.
        let mut q = stood(seed, 1, YARD_FAR, VICTIM, EAST);
        let by = shooter(&q);
        let (a, _) = pair(&q);
        q.man_fires_at(by, [11.0, 11.0]);
        q.falls_as(a, DownKind::Shot, false, DownBy::World);
        q.frame();
        let d = last_death(&q);
        assert_eq!(d.cause, RecapCause::Unknown, "seed {}: {:?}", seed, d);
    }
}

/// A FAILURE MEANS: a man playing on past the end of a run is shown what happened after it, or, once every player is down, is not shown it.
#[test]
fn a_run_played_on_past_its_end_shows_what_happened_up_to_it() {
    for seed in SEEDS {
        let mut p = stood(seed, 1, YARD_FAR, VICTIM, EAST);
        let (a, b) = pair(&p);
        p.falls(a, DownBy::Guard);
        p.run(2.0);
        assert!(p.recap_card().is_none(), "seed {}: the run is on", seed);
        p.g.mission.stage = MissionStage::Done;   // (a mission ended with the player up: a scene has no drive to leave by)
        p.run(0.2);
        let card = p.recap_card().expect("the run is over");
        assert_eq!(card.takes.len(), 1, "seed {}", seed);
        p.falls(b, DownBy::Guard);
        p.run(2.0);
        assert_eq!(p.recap_deaths().len(), 2, "seed {}: he is kept", seed);
        let card = p.recap_card().expect("the run is over");
        assert!(card.takes.len() == 1 && card.takes[0].death.victim == p.id_of(a), "seed {}: and not shown: {} takes", seed, card.takes.len());
        // CONTROL: every player down: all of it may be shown.
        p.seat_goes_down(0);
        p.frame();
        let card = p.recap_card().expect("the run is over");
        assert_eq!(card.takes.len(), 3, "seed {}: the two lost and Sam", seed);
    }
}

/// A FAILURE MEANS: a restart shows the last run's deaths, or forgets a run that is still on.
#[test]
fn a_restart_starts_the_record_over() {
    for seed in SEEDS {
        let mut p = stood(seed, 1, YARD_NEAR, VICTIM, EAST);
        let (a, _) = pair(&p);
        p.falls(a, DownBy::Guard);
        p.run(2.0);
        // CONTROL: run on, it is kept.
        assert_eq!(p.recap_deaths().len(), 1, "seed {}", seed);
        assert_eq!(p.recap_lost().len(), 1, "seed {}", seed);
        game::game::game_restart_encounter(&mut p.g);
        p.g.ai_enabled = false;
        p.run(0.5);
        assert!(p.recap_deaths().is_empty() && p.recap_lost().is_empty(), "seed {}: that run's deaths are not this run's: {:?}", seed, p.recap_deaths());
        assert!(p.recap_card().is_none(), "seed {}", seed);
    }
}

// ---------------------------------------------------------------- while the run is on: as much as the squad knows

/// a pair's man is shot in the west room, with Sam and his mate where a case says. Nobody hears it unless `hear` (the reach of ears shut for the run, so each line is about
/// the eyes alone). Answers the game, with the man lost and his mate.
fn lost_with(seed: u32, seats: u32, sam: [f64; 2], mate: [f64; 2], hear: bool) -> (Play, GuardId, GuardId, GuardId) {
    let mut p = stood(seed, seats, sam, off(seed, 6, VICTIM), mate);
    p.g.k.recap.live_hear_m = if hear { 1e9 } else { 0.0 };
    let by = shooter(&p);
    let (a, b) = pair(&p);
    p.run(0.5);
    rounds_into(&mut p, by, a);
    (p, a, b, by)
}
fn is_an_eighth(v: [f64; 2]) -> bool { v.iter().all(|x| [0.0, 1.0, std::f64::consts::FRAC_1_SQRT_2].iter().any(|k| (x.abs() - k).abs() < 1e-9)) }
fn calls(p: &Play) -> i32 { p.g.stats.deathrecap.calls }

/// A FAILURE MEANS: the squad knows where a round came from with nobody having seen or heard it, or the card opens early.
#[test]
fn lost_where_nobody_sees_or_hears_him_is_only_down() {
    for seed in SEEDS {
        let (mut p, a, b, _) = lost_with(seed, 1, YARD_NEAR, EAST, false);
        p.run(2.5);
        let l = p.recap_lost();
        assert_eq!(l.len(), 1, "seed {}: his row is struck", seed);
        assert_eq!(l[0].id, p.id_of(a), "seed {}", seed);
        assert!(l[0].from.is_none() && l[0].known_at == f64::INFINITY, "seed {}: and nobody knows anything: {:?}", seed, l[0]);
        assert!(calls(&p) == 0 && p.cue_over(b).is_none() && p.recap_card().is_none(), "seed {}: nobody says anything, the card is shut", seed);
        // … and his mate coming to where he sees him lie changes nothing: a man found tells nobody which way it came
        p.man_stands(b, IN_THE_ROOM);
        p.run(1.5);
        let l = p.recap_lost();
        assert!(l[0].from.is_none() && calls(&p) == 0, "seed {}: a man found is only down: {:?}", seed, l[0]);
        drop(p);
        // CONTROL: his mate in the room as he goes down
        let (mut q, _, _, _) = lost_with(seed, 1, YARD_NEAR, IN_THE_ROOM, false);
        q.run(2.5);
        assert!(q.recap_lost()[0].from.is_some(), "seed {}: seen, the squad has the way", seed);
    }
}

/// A FAILURE MEANS: the squad has the way at once, or finer than an eighth of the compass, or a man who only heard it says a side.
#[test]
fn a_shot_they_hear_gives_the_way_after_a_beat_and_no_finer_than_an_eighth() {
    for seed in SEEDS {
        let (mut p, _, b, _) = lost_with(seed, 1, YARD_NEAR, EAST, true);
        p.frame();
        let d = last_death(&p);
        let l = p.recap_lost();
        assert!(l[0].from.is_none(), "seed {}: nothing the frame after the fall: {:?}", seed, l[0]);
        p.run(1.5);
        let l = p.recap_lost();
        let u = l[0].from.expect("they heard it");
        assert!(l[0].known_at - l[0].t >= p.g.k.recap.live_beat - 1e-9, "seed {}: after the beat", seed);
        assert!(is_an_eighth(u), "seed {}: an eighth of the compass: {:?}", seed, u);
        let src = d.from.unwrap();
        let (dx, dz) = (src[0] - l[0].at[0], src[1] - l[0].at[1]);
        let len = (dx * dx + dz * dz).sqrt();
        let degrees = ((u[0] * dx + u[1] * dz) / len).clamp(-1.0, 1.0).acos().to_degrees();
        assert!(degrees <= 22.5 + 1e-6, "seed {}: the nearest one: {} degrees off", seed, degrees);
        assert_eq!(calls(&p), 0, "seed {}: nobody saw it, nobody says", seed);
        assert!(p.cue_over(b).is_none(), "seed {}", seed);
    }
}

/// A FAILURE MEANS: a shut door no longer muffles what the squad hears through it (the sound model's own rule), or ears no longer carry through one at all.
#[test]
fn ears_have_it_through_a_shut_door_only_from_close_by() {
    for seed in SEEDS {
        // just the far side of the door between the rooms, the walk round about four metres, with a shut leaf on it
        let near = heard_at(seed, [19.0, 11.5]);
        // CONTROL: the far corner of that room: the same door across a walk too long for it
        let far = heard_at(seed, [25.0, 14.8]);
        assert!(near && !far, "seed {}: by the door {}, across the room {}", seed, near, far);
    }
}
fn heard_at(seed: u32, mate: [f64; 2]) -> bool {
    let mut p = stood(seed, 1, YARD_FAR, off(seed, 6, VICTIM), mate);
    let by = shooter(&p);
    let (a, _) = pair(&p);
    p.run(0.5);
    rounds_into(&mut p, by, a);
    p.run(1.5);
    p.recap_lost()[0].from.is_some()
}

/// A FAILURE MEANS: the mate who saw it says nothing, or says the wrong side; or says it to a player too far off for 'left' to mean anything.
#[test]
fn the_mate_who_saw_it_says_the_side() {
    for seed in SEEDS {
        let (mut p, _, b, _) = lost_with(seed, 1, YARD_NEAR, IN_THE_ROOM, false);
        let by_way = p.until(2.0, |p| p.recap_lost().first().is_some_and(|l| l.from.is_some())).expect("he saw it");
        let u = p.recap_lost()[0].from.unwrap();
        assert!(is_an_eighth(u), "seed {}", seed);
        let sam = p.sam_at();
        let side = game::fogofwar::fog_of_war_side_of(&mut p.g, [sam[0] + u[0] * 50.0, 0.0, sam[1] + u[1] * 50.0]);
        let word = match side { game::close::CloseId::Front => "from the front!".to_string(), game::close::CloseId::Behind => "from behind us!".to_string(), s => format!("from the {}!", s.as_str()) };
        assert_eq!(p.cue_over(b).as_deref(), Some(word.as_str()), "seed {}: the fog's own word for it, by the player's shoulders ({} s)", seed, by_way);
        assert_eq!(calls(&p), 1, "seed {}", seed);
        p.run(1.0);
        assert_eq!(calls(&p), 1, "seed {}: and once", seed);
        // what the squad is handed is a way off the man: never the shooter's name, never his place
        let told = format!("{:?}", p.recap_lost()[0]);
        let by = shooter(&p);
        assert!(!told.contains(&p.g.gd[by].callsign), "seed {}: {}", seed, told);
        assert!(p.recap_card().is_none(), "seed {}", seed);
        drop(p);
        // CONTROL: the player stands too far off for a side to mean anything. The squad has the way, nobody says it.
        let (mut q, _, b, _) = lost_with(seed, 1, YARD_FAR, IN_THE_ROOM, false);
        q.run(2.0);
        assert!(q.recap_lost()[0].from.is_some() && calls(&q) == 0 && q.cue_over(b).is_none(), "seed {}: too far off: {:?}", seed, q.cue_over(b));
    }
}

/// A FAILURE MEANS: a dead man tells what he saw.
#[test]
fn the_mate_who_saw_it_and_goes_down_himself_tells_nobody() {
    for seed in SEEDS {
        let (mut p, _, b, _) = lost_with(seed, 1, YARD_NEAR, IN_THE_ROOM, false);
        p.run(0.2);
        p.falls_as(b, DownKind::Struck, true, DownBy::World);
        p.run(2.0);
        assert!(p.recap_lost()[0].from.is_none() && calls(&p) == 0, "seed {}: what he saw goes with him: {:?}", seed, p.recap_lost()[0]);
        drop(p);
        // CONTROL: up, he tells
        let (mut q, _, _, _) = lost_with(seed, 1, YARD_NEAR, IN_THE_ROOM, false);
        q.run(2.2);
        assert!(q.recap_lost()[0].from.is_some() && calls(&q) == 1, "seed {}", seed);
    }
}

/// A FAILURE MEANS: a player is told what he watched himself, by his mate.
#[test]
fn nobody_tells_a_player_what_he_watched() {
    for seed in SEEDS {
        let (mut p, _, b, _) = lost_with(seed, 1, [11.0, 11.0], IN_THE_ROOM, false);
        p.run(2.0);
        assert!(p.recap_lost()[0].from.is_some(), "seed {}: the way is known", seed);
        assert!(calls(&p) == 0 && p.cue_over(b).is_none(), "seed {}: nobody says it: {:?}", seed, p.cue_over(b));
        drop(p);
        // A GAME OF TWO: it is enough that either watched it: the second player sees, the first does not, and nobody is told.
        let mut q = stood(seed, 2, YARD_NEAR, off(seed, 6, VICTIM), IN_THE_ROOM);
        q.g.k.recap.live_hear_m = 0.0;
        q.seat_stands(1, [11.0, 11.0]);
        let by = shooter(&q);
        let (a, b) = pair(&q);
        q.run(0.5);
        rounds_into(&mut q, by, a);
        q.run(2.0);
        assert!(q.recap_lost()[0].from.is_some() && calls(&q) == 0 && q.cue_over(b).is_none(), "seed {}: the second player watched it: {:?}", seed, q.cue_over(b));
        drop(q);
        // CONTROL IN A GAME OF TWO: neither watched it, the second player out of the room as well: the mate says it.
        let mut r = stood(seed, 2, YARD_NEAR, off(seed, 6, VICTIM), IN_THE_ROOM);
        r.g.k.recap.live_hear_m = 0.0;
        r.seat_stands(1, [15.0, 19.0]);
        let by = shooter(&r);
        let (a, b) = pair(&r);
        r.run(0.5);
        rounds_into(&mut r, by, a);
        r.run(2.0);
        assert!(calls(&r) == 1 && r.cue_over(b).is_some(), "seed {}: neither of them saw it: the mate says it", seed);
    }
}

/// the word a mate says for a way, by what the fog says of it for the first player still up
fn the_word(p: &mut Play, u: [f64; 2], seat: usize) -> String {
    let at = p.seat_at(seat);
    match game::fogofwar::fog_of_war_side_of(&mut p.g, [at[0] + u[0] * 50.0, 0.0, at[1] + u[1] * 50.0]) {
        game::close::CloseId::Front => "from the front!".to_string(),
        game::close::CloseId::Behind => "from behind us!".to_string(),
        s => format!("from the {}!", s.as_str()),
    }
}

/// A FAILURE MEANS: the first player down leaves his mate telling the air, or the side is said by the wrong man's shoulders: it is called to the first player still up.
#[test]
fn in_a_game_of_two_the_side_is_called_to_the_player_still_up() {
    for seed in SEEDS {
        let mut p = stood(seed, 2, YARD_FAR, off(seed, 6, VICTIM), IN_THE_ROOM);
        p.g.k.recap.live_hear_m = 0.0;
        p.seat_stands(1, YARD_NEAR);
        let by = shooter(&p);
        let (a, b) = pair(&p);
        // (the second player looks away from the first's way, so that the two would not say one word)
        p.point_at([YARD_NEAR[0] + 6.0, YARD_NEAR[1]]);
        p.run(0.5);
        p.seat_goes_down(0);
        p.frame();
        rounds_into(&mut p, by, a);
        p.run(1.5);
        let u = p.recap_lost()[0].from.expect("his mate saw it");
        let word = the_word(&mut p, u, 1);
        assert_eq!(calls(&p), 1, "seed {}: he is called to", seed);
        assert_eq!(p.cue_over(b), Some(word), "seed {}: by his own shoulders", seed);
        drop(p);
        // CONTROL: the first player up and far off, the second near: it is the first player up who is called to, so nobody is
        let mut q = stood(seed, 2, YARD_FAR, off(seed, 6, VICTIM), IN_THE_ROOM);
        q.g.k.recap.live_hear_m = 0.0;
        q.seat_stands(1, YARD_NEAR);
        let by = shooter(&q);
        let (a, _) = pair(&q);
        q.run(0.5);
        rounds_into(&mut q, by, a);
        q.run(1.5);
        assert_eq!(calls(&q), 0, "seed {}", seed);
    }
}

/// A FAILURE MEANS: a player down still tells what he saw, and his mate is held from saying it to the player who did not.
#[test]
fn in_a_game_of_two_a_player_who_saw_it_and_goes_down_tells_nobody() {
    for seed in SEEDS {
        let mut p = stood(seed, 2, YARD_NEAR, off(seed, 6, VICTIM), IN_THE_ROOM);
        p.g.k.recap.live_hear_m = 0.0;
        p.seat_stands(1, [11.0, 11.0]);   // the second player watches it, the first is in the yard
        let by = shooter(&p);
        let (a, b) = pair(&p);
        p.run(0.5);
        rounds_into(&mut p, by, a);
        p.run(0.1);
        p.seat_goes_down(1);
        p.run(2.0);
        assert!(calls(&p) == 1 && p.cue_over(b).is_some(), "seed {}: the one who watched is down: the mate says it to the one who did not: {:?}", seed, p.cue_over(b));
        drop(p);
        // CONTROL: he stays up: nobody says it
        let mut q = stood(seed, 2, YARD_NEAR, off(seed, 6, VICTIM), IN_THE_ROOM);
        q.g.k.recap.live_hear_m = 0.0;
        q.seat_stands(1, [11.0, 11.0]);
        let by = shooter(&q);
        let (a, _) = pair(&q);
        q.run(0.5);
        rounds_into(&mut q, by, a);
        q.run(2.1);
        assert_eq!(calls(&q), 0, "seed {}", seed);
    }
}

/// A FAILURE MEANS: a load brings a death's picture back into a run in which it has not happened, or a word said before the save is said again after it.
#[test]
fn a_kept_game_has_no_record_and_the_next_tick_starts_one_from_the_floor_as_it_is() {
    for seed in SEEDS {
        let (mut p, _, b, _) = lost_with(seed, 1, YARD_NEAR, IN_THE_ROOM, false);
        // the save is made before the beat is out, with the man lying on the floor
        game::deathrecap::recap_left_behind(&mut p.g);
        p.run(2.0);
        assert!(p.recap_deaths().is_empty() && p.recap_lost().is_empty(), "seed {}: the body the save brought back is no death of this run: {:?}", seed, p.recap_deaths());
        assert!(calls(&p) == 0 && p.cue_over(b).is_none(), "seed {}: and nobody says anything about it", seed);
        // … and the record goes on from there: the next man down is seen.
        p.falls(b, DownBy::Guard);
        p.run(0.5);
        assert_eq!(p.recap_deaths().len(), 1, "seed {}: the next man down is kept", seed);
        drop(p);
        // CONTROL: no save: the mate says it.
        let (mut q, _, b, _) = lost_with(seed, 1, YARD_NEAR, IN_THE_ROOM, false);
        q.run(2.0);
        assert!(q.recap_deaths().len() == 1 && calls(&q) == 1 && q.cue_over(b).is_some(), "seed {}", seed);
    }
}

/// A FAILURE MEANS: the recap draws from the game's die or deals serials: it would move every part of the game that keeps one. Each tick's step is taken a second time on
/// the record as it stood before the frame, so that the man's fall, the explaining, the ears and the word all happen inside a step this test can hold still around.
#[test]
fn the_recap_moves_nothing_a_rule_reads() {
    for seed in SEEDS {
        let mut p = stood(seed, 1, YARD_NEAR, off(seed, 6, VICTIM), IN_THE_ROOM);
        p.g.k.recap.live_hear_m = 1e9;
        let by = shooter(&p);
        let (a, _) = pair(&p);
        p.run(0.5);
        let (mut explained, mut said) = (0, 0);
        for i in 0..120 {
            if i >= 10 && p.state(a) != game::game::GuardState::Dead { let at = p.at(a); p.man_fires_at(by, at); }
            let (before, cues) = (p.g.recap.clone(), p.g.cues.bubbles.clone());
            p.frame();
            p.g.recap = before;
            p.g.cues.bubbles = cues;
            let (serial, seq, t, deaths, calls0) = (p.g.serial_next, p.g.event_seq, p.g.time, p.g.stats.deathrecap.deaths, calls(&p));
            let die = |p: &Play| { let d = p.g.dice.as_ref().expect("the game has its die"); (game::math_dice::sim_dice_n(d), game::math_dice::sim_dice_s(d)) };
            let dice = die(&p);
            game::deathrecap::recap_tick(&mut p.g);
            assert!(p.g.serial_next == serial && p.g.event_seq == seq && p.g.time == t, "seed {}: it dealt a number", seed);
            assert_eq!(die(&p), dice, "seed {}: it drew from the die", seed);
            explained += p.g.stats.deathrecap.deaths - deaths;
            said += calls(&p) - calls0;
        }
        assert!(explained >= 1 && said >= 1, "seed {}: and it did its work inside the held step: {} explained, {} said", seed, explained, said);
    }
}
