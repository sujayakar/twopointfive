//! A GAME OF TWO: what the world's lines of the tick say about the team is about every player, not the first (tools/blitz/sweeps/seat-tick.py lists them).
use game::game::Foe;
use game::ids::GuardId;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 2] = [21, 22];
/// a clear run of floor down the office's aisle
const AISLE: f64 = 17.6;

/// The office's room clear for two, the players at each end of the aisle with every man of both squads out of the way behind them, one defender left, held.
fn two(seed: u32) -> (Play, GuardId) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, seats: 2, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.3);
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative).collect();
    p.drop_all_but(&foes[..1]);
    (p, foes[0])
}
fn man(p: &Play, seat: usize) -> game::ids::PlayerId { game::seats::seat(&p.g, p.g.seats[seat]).player }

/// THE RULE: a round in flight stops at either player. A failure means it flies through one of them.
#[test]
fn a_round_in_flight_stops_at_either_player() {
    for s in SEEDS {
        for seat in 0..2 {
            for shell in [false, true] {
                let (mut p, t1) = two(s);
                let (x_man, x_from) = (14.0, 21.0);
                p.seat_stands(seat, [x_man, AISLE]);
                p.seat_stands(1 - seat, [3.0, 3.0]);
                for m in p.mates() { p.man_stands(m, [3.5, 3.5]); }
                p.pin_man(t1, [x_from, AISLE], -std::f64::consts::FRAC_PI_2, game::game::GuardState::Patrol, 0.0);
                p.frame();
                let shooter = p.g.gd[t1].ch;
                let (from, to) = ([x_from - 0.5, 1.3, AISLE], [x_man, 1.2, AISLE]);
                if shell { game::he40::fire_shell(&mut p.g, shooter, from, to); } else { game::launcher::fire_ring(&mut p.g, shooter, from, to); }
                let mut least = f64::INFINITY;
                for _ in 0..40 {
                    p.pin_man(t1, [x_from, AISLE], -std::f64::consts::FRAC_PI_2, game::game::GuardState::Patrol, 0.0);
                    p.frame();
                    if let Some(r) = p.g.rings.first() { least = least.min(r.pos[0]); }
                    if let Some(r) = p.g.shells.first() { least = least.min(r.pos[0]); }
                }
                assert!(least > x_man - 1.0, "seed {s}: the {} from {x_from} went on to x {least} past seat {seat}'s man at {x_man}", if shell { "shell" } else { "ring" });
                assert!(least < x_man + 2.0, "seed {s}: CONTROL: the {} stopped short of him at x {least}", if shell { "shell" } else { "ring" });
            }
        }
    }
}

/// THE RULE: the team is down when all of it is: one man down and the other up, the floor goes on as if they are up. A rule about one man asks that man.
/// A failure means the first player's fall stops the alarm's rise and the guards' dozing for both.
#[test]
fn the_team_is_down_when_all_of_it_is() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = two(s);
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            assert!(game::doze::live(&mut p.g), "seed {s}: CONTROL: both up, a calm floor dozes");
            let m = man(&p, seat);
            p.g.pl[m].down = true;
            assert!(game::doze::live(&mut p.g), "seed {s}: seat {seat}'s man down and the other up: the floor is still calm");
            p.g.gd[t1].team = game::teams::Team::Guard;   // (the floor's alarm is the guards': a level's night, not this room clear's defenders)
            p.g.alarm.episode = true;
            let was = p.g.escalation;
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            p.frame();
            assert!(p.g.escalation > was, "seed {s}: an alarm over with seat {seat}'s man down and the other up raises the floor: {} from {was}", p.g.escalation);
            drop(p);
            let (mut q, t2) = two(s);
            let (a, b) = (man(&q, 0), man(&q, 1));
            q.g.pl[a].down = true;
            q.g.pl[b].down = true;
            assert!(!game::doze::live(&mut q.g), "seed {s}: CONTROL: both down, nobody is left to doze for");
            q.g.gd[t2].team = game::teams::Team::Guard;
            q.g.alarm.episode = true;
            let was = q.g.escalation;
            q.pin_man(t2, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            game::guards::update_escalation(&mut q.g, 1.0 / 60.0);
            assert_eq!(q.g.escalation, was, "seed {s}: CONTROL: both down, an alarm over raises nothing");
        }
    }
}

fn close(p: &Play, gd: GuardId) -> bool { game::lanes::close_fight(&p.g, gd) }

/// THE RULE: a defender's foe, when it is a player, is the player he faces.
/// A failure means he is measured against the first player wherever the second stands.
#[test]
fn a_defenders_foe_is_the_player_he_faces() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = two(s);
            p.seat_stands(seat, [20.0, AISLE]);
            p.seat_stands(1 - seat, [3.0, 3.0]);
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Alert, 1.0);
            p.g.gd[t1].foe = Some(Foe::Player);
            p.g.gd[t1].faces = Some(seat as f64);
            p.g.gd[t1].last_seen_t = p.g.time;
            assert!(close(&p, t1), "seed {s}: he has seat {seat}'s man a metre off and calls it no close fight");
        }
    }
}

// (the calls whose seat goes in the mend: one place each)
fn muzzle(p: &Play, gd: GuardId) -> bool { game::game::friend_before_muzzle(&p.g, gd) }
fn clear(p: &Play, at: [f64; 3]) -> bool { game::qrf::clear_of_foes(&p.g, at) }
fn foes(p: &Play, gd: GuardId) -> Vec<game::runner::Foe> { game::runner::foes(&p.g, gd) }

/// THE RULE: what is about the team is about every man on the floor: a friend before the muzzle, a place clear of foes, whom a runner runs from, who sees a hole in a wall.
/// A failure means the second player is not on the floor for that rule.
#[test]
fn the_team_is_both_players() {
    let mut red: Vec<String> = Vec::new();
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = two(s);
            let mates = p.mates();
            for &m in &mates { p.man_stands(m, [3.5, 3.5]); }
            p.seat_stands(0, [3.0, 3.0]);
            p.seat_stands(1, [3.0, 3.0]);
            // a place clear of foes
            let at = [21.0, 0.0, AISLE];
            assert!(clear(&p, at), "seed {s}: CONTROL: nobody near");
            p.seat_stands(seat, [21.0, AISLE]);
            if clear(&p, at) { red.push(format!("seed {s}: seat {seat}'s man stands on the place and it is called clear of foes")); }
            // a friend before the muzzle: the man who shoots stands off down the aisle, the player in his lane
            let m = mates[0];
            let c = p.g.gd[m].ch;
            p.man_stands(m, [12.0, AISLE]);
            let y = game::character::character_gun_yaw(&p.g, c);
            let ahead = (0..32).map(|i| i as f64 * std::f64::consts::TAU / 32.0).map(|a| [1.5 * a.sin(), 1.5 * a.cos()])
                .find(|d| game::character::in_dip_lane(d[0], d[1], y)).expect("a lane has a place 1.5 m out");
            let pc = p.g.pl[man(&p, seat)].ch;
            p.g.ch[pc].pos = [12.0 + ahead[0], 0.0, AISLE + ahead[1]];
            if !muzzle(&p, m) { red.push(format!("seed {s}: seat {seat}'s man stands in the lane and no friend is before the muzzle")); }
            // whom a runner runs from
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            p.g.ch[pc].pos = [20.0, 0.0, AISLE];
            if !foes(&p, t1).iter().any(|f| (f.pos[0] - 20.0).abs() < 0.01 && (f.pos[2] - AISLE).abs() < 0.01) { red.push(format!("seed {s}: seat {seat}'s man is the player nearest the runner and not among his foes")); }
            // who sees a hole in a wall
            if let Some(why) = wall_seen(&mut p, seat) { red.push(format!("seed {s}: {why}")); }
        }
    }
    assert!(red.is_empty(), "{}", red.join("\n"));
}

/// a hole in the office's wall, blown by the game's own call, that nobody on the team knows of; one player stands looking at it, everybody else is far off: `None` if the team learns of it
fn wall_seen(p: &mut Play, seat: usize) -> Option<String> {
    const WALL: [f64; 3] = [12.06, 0.0, 13.78];
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
    p.sam_put([14.6, 15.2]);
    p.frame();
    if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, WALL, None) { return Some(format!("the wall charge was refused: {why}")); }
    for _ in 0..3600 { p.frame(); if !p.g.wall_charge.set.is_empty() { break; } }
    let Some(&c) = p.g.wall_charge.set.first() else { return Some(String::from("a minute and no charge is on the wall")) };
    game::wallcharge::wall_charge_blow(&mut p.g, Some(c), Some(true));
    for _ in 0..30 { p.frame(); }
    let Some(&b) = p.g.m.wallreact.wall_react.breaches.first() else { return Some(String::from("the blast left no hole the rule knows")) };
    p.g.tab.wallreact_breach[b].ops = -1.0;
    let at = p.g.tab.wallreact_breach[b].at;
    for m in p.mates() { p.man_stands(m, [3.5, 3.5]); }
    p.seat_stands(0, [3.0, 3.0]);
    p.seat_stands(1, [3.0, 3.0]);
    let n = p.g.tab.wallreact_breach[b].n;
    let stand = [at[0] + n[0] * 3.0, at[2] + n[1] * 3.0];
    let pc = p.g.pl[man(p, seat)].ch;
    p.g.ch[pc].pos = [stand[0], 0.0, stand[1]];
    p.g.ch[pc].aim_yaw = (at[0] - stand[0]).atan2(at[2] - stand[1]);
    game::wallreact::wall_react_notice(&mut p.g);
    if p.g.tab.wallreact_breach[b].ops < 0.0 { return Some(format!("seat {seat}'s man looks at the hole from {stand:?} and the team does not know of it")); }
    None
}

/// One of the response team or the floor's clearing party with the second player at arm's length, the first across the floor.
fn man_at_arms(s: u32, seat: usize) -> (Play, GuardId) {
    let (mut p, t1) = two(s);
    for m in p.mates() { p.man_stands(m, [3.5, 3.5]); }
    p.seat_stands(1 - seat, [3.0, 3.0]);
    p.seat_stands(seat, [20.6, AISLE]);
    p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
    p.g.gd[t1].faces = Some(seat as f64);
    p.g.gd[t1].last_known = None;
    p.g.gd[t1].awareness = 0.0;
    (p, t1)
}
/// THE RULE: a man's senses, whoever runs them, are about the player he faces: at arm's length he is found, and it is his place that is fixed.
/// A failure means the man sees one player and fixes on the other, or walks into the second with no word.
#[test]
fn a_mans_senses_are_about_the_player_he_faces() {
    for s in SEEDS {
        for seat in 0..2 {
            let near = |p: &Play, t: GuardId| p.g.gd[t].last_known.map_or(false, |k| ((k[0] - 20.6f64).powi(2) + (k[2] - AISLE).powi(2)).sqrt() < 1.0);
            let (mut p, t1) = man_at_arms(s, seat);
            let r = game::squad::perceive(&mut p.g, t1, 1.0 / 60.0, 0.0);
            assert!(r.is_some() || near(&p, t1), "seed {s}: the clearing man has nothing");
            assert!(near(&p, t1), "seed {s}: seat {seat}'s man at arm's length and the clearing man fixes {:?}", p.g.gd[t1].last_known);
            drop(p);
            let (mut p, t1) = man_at_arms(s, seat);
            game::qrf::perceive_scripted(&mut p.g, t1, 1.0 / 60.0);
            assert!(near(&p, t1), "seed {s}: seat {seat}'s man at arm's length and the response man fixes {:?}", p.g.gd[t1].last_known);
        }
    }
}

/// THE RULE: an eye asks about the man it looks at. Another player being down does not blind it.
#[test]
fn an_eye_sees_a_man_up_with_the_other_down() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, _) = two(s);
            let m = man(&p, seat);
            let other = man(&p, 1 - seat);
            p.g.pl[other].down = true;
            p.seat_stands(seat, [16.0, AISLE]);
            let pc = p.g.pl[m].ch;
            let (from, chest) = ([20.0, 1.5, AISLE], [16.0, 1.2, AISLE]);
            let look = game::v3::normalize(game::v3::sub(chest, from));
            let e = game::guards::Eye { pos: from, look, range: 20.0, half_deg: 45.0, near_m: 0.0, near_deg: 45.0, v_half_deg: Some(45.0) };
            let vis = game::guards::eye_contact(&mut p.g, e, pc, true, false, 1.0, None);
            assert!(vis.visible, "seed {s}: seat {seat}'s man stands in the cone, up, with the other down");
        }
    }
}

/// THE RULE: a teammate's lane is clear of both players.
#[test]
fn a_teammates_lane_is_clear_of_both_players() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, _) = two(s);
            let shooter = p.mates()[0];
            for x in p.mates() { p.man_stands(x, [3.5, 3.5]); }
            p.seat_stands(1 - seat, [3.0, 3.0]);
            p.seat_stands(seat, [16.0, AISLE]);
            assert!(!game::coverfire::cover_lane(&mut p.g, shooter, [12.0, 1.4, AISLE], [20.0, 1.4, AISLE]), "seed {s}: seat {seat}'s man stands in the lane and it is called clear");
        }
    }
}

/// THE RULE: a man's walk through a door goes by his own seat's order, whichever seat he is. Back from a fight his order's way is taken up again.
/// A failure means the walk finds no order for the second player's men.
#[test]
fn a_walk_through_a_door_goes_by_his_own_seats_order() {
    for s in SEEDS {
        for seat in 0..2 {
            let scene = store(s);
            let door = scene.door_centre("store-yard");
            let mut p = Play::scene(&scene, &Opts { seed: s, seats: 2, ..Default::default() });
            let inside = p.defender_at([17.25, 9.25]);
            p.falls(inside, game::game::DownBy::Player);
            p.run(0.5);
            p.seat_orders(seat, "stack", door, game::orders::Who::All);
            p.run(7.0);
            p.seat_orders(seat, "openClear", door, game::orders::Who::All);
            let sn = p.g.seats[seat];
            let mine = game::orders::squad_orders_squad(&p.g, &game::seats::seat(&p.g, sn));
            let walker = p.until(5.0, |p| mine.iter().any(|m| p.g.door_path.walks.has(*m))).map(|_| *mine.iter().find(|m| p.g.door_path.walks.has(**m)).unwrap());
            let Some(m) = walker else { panic!("seed {s}, seat {seat}: CONTROL: five seconds after the word no man of his has a walk") };
            let a = game::orders::squad_orders_of(&p.g, &game::seats::seat(&p.g, sn), m).expect("he is on the order");
            let pod = p.g.door_path.walks.get(m).unwrap().pod;
            p.g.tab.orders_assignment[a].post = Some(pod);
            p.g.tab.orders_assignment[a].via = vec![pod];
            p.frame();
            assert!(p.g.tab.orders_assignment[a].via.is_empty(), "seed {s}: seat {seat}'s man's walk did not take his order's way up");
        }
    }
}

/// THE RULE: a man's hand at work tells what his work is, whichever seat he is.
/// A failure means the second player's man works a breaker with the hand of a man pushing a door.
#[test]
fn a_hand_at_work_says_what_the_work_is_for_either_seat() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, _) = two(s);
            for d in p.g.doors.list.clone() { game::doors::door_set_locked(&mut p.g, d, false); }
            p.run(0.3);
            let sn = p.g.seats[seat];
            let th = game::workthings::work_things_things(&p.g, &game::seats::seat(&p.g, sn)).into_iter().find(|x| x.kind == game::workthings::WorkThingKind::Breaker).expect("the office has a breaker");
            p.seat_orders(seat, "work", [th.face[0], th.face[2]], game::orders::Who::All);
            let mine = game::orders::squad_orders_squad(&p.g, &game::seats::seat(&p.g, sn));
            let chars: Vec<_> = mine.iter().map(|m| p.g.gd[*m].ch).collect();
            let at = p.until(40.0, |p| chars.iter().any(|c| game::workthings::work_hand(&mut p.g, *c).is_some()));
            assert!(at.is_some(), "seed {s}: CONTROL: forty seconds and no hand of seat {seat}'s is out: {}", p.roll());
            let c = *chars.iter().find(|c| game::workthings::work_hand(&mut p.g, **c).is_some()).unwrap();
            assert!(game::workthings::work_job(&mut p.g, c).is_some(), "seed {s}: seat {seat}'s man has a hand out and no job");
        }
    }
}

/// THE RULE: a player's hand on a light switch is planted the same for either seat, and held by his own state.
#[test]
fn a_hand_on_a_switch_is_planted_for_either_seat() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, _) = two(s);
            let sw = p.g.m.lightswitch.states.as_ref().and_then(|st| st.all.first().cloned()).expect("the office has a switch");
            let m = man(&p, seat);
            let pc = p.g.pl[m].ch;
            p.seat_stands(seat, [sw.pos[0] + sw.n[0] * 0.5, sw.pos[2] + sw.n[2] * 0.5]);
            game::lightswitch::flip_switch(&mut p.g, &sw, false, pc);
            let plant = |p: &Play| p.g.m.lightswitch.states.as_ref().unwrap().hands.last().map(|h| h.plant);
            assert_eq!(plant(&p), Some(true), "seed {s}: seat {seat}'s man flips a switch");
            // his plant goes by where HE stands, wherever the other player is
            p.seat_stands(1 - seat, [3.0, 3.0]);
            p.frame();
            assert_eq!(plant(&p), Some(true), "seed {s}: the other player's place broke seat {seat}'s plant");
            p.seat_stands(seat, [sw.pos[0] + sw.n[0] * 3.0, sw.pos[2] + sw.n[2] * 3.0]);
            p.frame();
            assert_eq!(plant(&p), Some(false), "seed {s}: seat {seat}'s man walked off and his plant held");
        }
    }
}

/// THE RULE: a defender's live grenade is called to whichever player it is near.
#[test]
fn a_grenade_is_called_to_either_player() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, _) = two(s);
            p.seat_stands(1 - seat, [3.0, 3.0]);
            p.seat_stands(seat, [18.0, AISLE]);
            p.frag_lands([19.0, AISLE]);
            p.frame();
            p.g.m.frag.books.as_mut().unwrap()[0].cued = false;
            game::frag::sam_cue(&mut p.g, 0, [19.0, 0.2, AISLE]);
            assert!(p.g.m.frag.books.as_ref().unwrap()[0].cued, "seed {s}: a grenade a metre from seat {seat}'s man and nobody is told");
        }
    }
}

/// (coop.rs's store: a door and a room behind it)
fn store(seed: u32) -> Scene {
    let mut s = Scene::new("store");
    s.room("store", 10.0, 8.0, 18.0, 14.0).room("far", 26.0, 8.0, 34.0, 14.0);
    s.door("store-yard", Axis::X, 14.0, 13.4 + nudge(seed, 0)).locked("store-yard").door("far-yard", Axis::X, 14.0, 29.4).door("far-back", Axis::X, 8.0, 29.4);
    s.bulb(14.0, 11.0).bulb(30.0, 11.0).moon();
    s.team([14.0, 21.0], std::f64::consts::PI, &[[13.0, 22.0 + nudge(seed, 1)], [15.0, 22.0], [13.0, 23.0], [15.0, 23.0]]);
    s.defender("far", [32.25, 10.25], "far-yard").defender("store", [17.25, 9.25], "store-yard");
    s
}
