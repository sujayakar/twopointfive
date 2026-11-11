//! A GAME OF TWO: what the world's lines of the tick say about the team is about every player, not the first.
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
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| !p.g.team(gd).ours()).collect();
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
            p.g.cond[p.g.pl[m].ch].down = true;
            assert!(game::doze::live(&mut p.g), "seed {s}: seat {seat}'s man down and the other up: the floor is still calm");
            p.g.ch[p.g.gd[t1].ch].team = game::teams::Team::Guard;   // (the floor's alarm is the guards': a level's night, not this room clear's defenders)
            p.g.alarm.episode = true;
            let was = p.g.escalation;
            p.pin_man(t1, [21.0, AISLE], 0.0, game::game::GuardState::Patrol, 0.0);
            p.frame();
            assert!(p.g.escalation > was, "seed {s}: an alarm over with seat {seat}'s man down and the other up raises the floor: {} from {was}", p.g.escalation);
            drop(p);
            let (mut q, t2) = two(s);
            let (a, b) = (man(&q, 0), man(&q, 1));
            q.g.cond[q.g.pl[a].ch].down = true;
            q.g.cond[q.g.pl[b].ch].down = true;
            assert!(!game::doze::live(&mut q.g), "seed {s}: CONTROL: both down, nobody is left to doze for");
            q.g.ch[q.g.gd[t2].ch].team = game::teams::Team::Guard;
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
            p.g.gd[t1].faces = Some(seat as f64);
            p.g.gd[t1].foe = Some(p.g.pl[man(&p, seat)].ch);
            p.g.gd[t1].last_seen_t = p.g.time;
            assert!(close(&p, t1), "seed {s}: he has seat {seat}'s man a metre off and calls it no close fight");
        }
    }
}

/// THE RULE: an enemy fights one player, by name. He keeps him while that player is up, however near the other comes; the other is his man when that one goes down; and
/// where he believes each of them to be is two places.
/// A failure means "the player" is one man to him: he turns to whoever steps nearer, shoots at a man who is down, or takes one player's place for the other's.
#[test]
fn an_enemy_names_the_player_he_fights() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = two(s);
            let (a, b) = (p.g.pl[man(&p, seat)].ch, p.g.pl[man(&p, 1 - seat)].ch);
            let held = |p: &mut Play, secs: usize| for _ in 0..60 * secs { p.pin_man(t1, [21.0, AISLE], -std::f64::consts::FRAC_PI_2, game::game::GuardState::Alert, 1.0); p.frame(); };
            p.seat_stands(seat, [14.0, AISLE]);
            p.seat_stands(1 - seat, [3.0, 3.0]);
            (p.g.gd[t1].foe, p.g.gd[t1].god) = (None, true);
            held(&mut p, 1);
            assert_eq!(p.g.gd[t1].foe, Some(a), "seed {s}: he has seat {seat}'s man in front of him and nobody else on the floor");
            p.seat_stands(1 - seat, [18.0, AISLE]);
            held(&mut p, 1);
            assert_eq!(p.g.gd[t1].foe, Some(a), "seed {s}: the other player stepped nearer, and he let seat {seat}'s man go");
            for (c, at) in [(a, [14.0, 0.0, AISLE]), (b, [18.0, 0.0, AISLE])] { game::picture::picture_tell(&mut p.g, t1, c, at, None); }
            let of = |c| game::picture::ideas_of(&p.g, t1).into_iter().find(|i| i.who == c).map(|i| i.at[0]);
            let (at_a, at_b) = (of(a).expect("a place for the one"), of(b).expect("a place for the other"));
            assert!(at_b - at_a > 2.0, "seed {s}: he believes both players in one place: {at_a} and {at_b}");
            p.g.cond[a].down = true;
            held(&mut p, 1);
            assert_eq!(p.g.gd[t1].foe, Some(b), "seed {s}: seat {seat}'s man is down, and he does not have the other");
        }
    }
}

// (the calls whose seat goes in the mend: one place each)
fn clear(p: &Play, at: [f64; 3]) -> bool { game::qrf::clear_of_foes(&p.g, at) }
fn foes(p: &Play, gd: GuardId) -> Vec<game::runner::Foe> { game::runner::foes(&p.g, gd) }

/// THE RULE: what is about the team is about every man on the floor: a place clear of foes, whom a runner runs from, who sees a hole in a wall.
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
            let pc = p.g.pl[man(&p, seat)].ch;
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
    for _ in 0..3600 { p.frame(); if !p.g.sets[p.g.seat].wall_charge.set.is_empty() { break; } }
    let Some(&c) = p.g.sets[p.g.seat].wall_charge.set.first() else { return Some(String::from("a minute and no charge is on the wall")) };
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
            let r = game::squad::perceive(&mut p.g, t1, 0.0);
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
            p.g.cond[p.g.pl[other].ch].down = true;
            p.seat_stands(seat, [16.0, AISLE]);
            let pc = p.g.pl[m].ch;
            let (from, chest) = ([20.0, 1.5, AISLE], [16.0, 1.2, AISLE]);
            let look = game::v3::normalize(game::v3::sub(chest, from));
            let e = game::guards::Eye { pos: from, look, range: 20.0, half_deg: 45.0, near_m: 0.0, near_deg: 45.0, v_half_deg: Some(45.0) };
            let vis = game::guards::eye_contact(&mut p.g, e, pc, false, 1.0, None);
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
            let a = game::orders::squad_orders_of(&game::seats::seat(&p.g, sn), m).expect("he is on the order");
            let pod = p.g.door_path.walks.get(m).unwrap().pod;
            p.g.tab.task[a].post = Some(pod);
            p.g.tab.task[a].via = vec![pod];
            p.frame();
            assert!(p.g.tab.task[a].via.is_empty(), "seed {s}: seat {seat}'s man's walk did not take his order's way up");
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
            for d in p.g.doors.list.clone() { p.g.dr[d].leaf.locked = false; }
            p.run(0.3);
            let sn = p.g.seats[seat];
            let th = game::workthings::work_things_things(&game::seats::seat(&p.g, sn)).into_iter().find(|x| x.kind == game::workthings::WorkThingKind::Breaker).expect("the office has a breaker");
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

/// A game of two with the partner's rules up (`?pair`), every squad silent, one defender left, held.
fn partners(seed: u32) -> (Play, GuardId) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad=2", &Opts { seed, seats: 2, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.3);
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| !p.g.team(gd).ours()).collect();
    p.drop_all_but(&foes[..1]);
    let t1 = foes[0];
    p.g.k.pairup.on = true;
    game::pairup::pair_up_apply(&mut p.g, None);
    p.g.k.silent.on = true;
    for i in 0..2 { let sn = p.g.seats[i]; game::seats::with_seat(&mut p.g, sn, &mut |g| game::silent::set_silent(g, true)); }
    p.frame();
    (p, t1)
}
/// THE RULE: each player has the partner's rows on his own menu and his own word for the go: a mark waiting on one player's man goes on that player's F, and on nobody else's.
/// A failure means the second player has no partner's rows, or the first player's F sends the second player's man.
#[test]
fn each_player_has_his_own_partner() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = partners(s);
            let sn = p.g.seats[seat];
            let has = |p: &Play, sn, id: &str| { let seat = game::seats::seat(&p.g, sn); seat.orders.defs.iter().any(|d| p.g.tab.orders_order_def[*d].id == id) };
            assert!(has(&p, sn, "pairMark"), "seed {s}: seat {seat} has no partner's rows on his menu");
            let mine = game::orders::squad_orders_squad(&p.g, &game::seats::seat(&p.g, sn));
            let at = [22.0, AISLE];
            const EAST: f64 = std::f64::consts::FRAC_PI_2;
            let theirs = game::orders::squad_orders_squad(&p.g, &game::seats::seat(&p.g, p.g.seats[1 - seat]));
            for _ in 0..10 {
                p.pin_man(t1, at, EAST, game::game::GuardState::Patrol, 0.0);
                p.seat_stands(seat, [11.0, 18.5]);
                for &m in &mine { p.man_stands(m, [13.0, AISLE]); }
                p.seat_stands(1 - seat, [3.0, 3.0]);
                for &m in &theirs { p.man_stands(m, [3.5, 3.5]); }
                p.frame();
            }
            let picked = game::orders::squad_orders_people_near(&p.g, &game::seats::seat(&p.g, sn), [at[0], 0.0, at[1]]).map(|(h, _)| h);
            assert_eq!(picked, Some(t1), "seed {s}: seat {seat}'s cursor cannot pick a man for his partner");
            let why = game::seats::with_seat(&mut p.g, sn, &mut |g| {
                let t = game::orders::OrderTarget::Foe(game::orders::OrderTargetFoe { at: [at[0], 0.0, at[1]], foe: t1 });
                game::orders::squad_orders_give(g, "pairMark", Some(&t), Some(game::orders::Who::All), None)
            });
            assert_eq!(why, None, "seed {s}: seat {seat}'s mark was refused");
            p.pin_man(t1, at, EAST, game::game::GuardState::Patrol, 0.0);
            p.frame();
            let job = |p: &Play| mine.iter().find_map(|m| p.g.pair.jobs.get(*m).copied());
            assert!(job(&p).map_or(false, |j| !j.go), "seed {s}: seat {seat}'s man waits on the mark: {:?}", job(&p));
            // the other player's F sends nobody of his
            let other = p.g.tab.cmd_seat[p.g.seats[1 - seat]].input;
            p.g.tab.input[other].pressed.add(String::from("KeyF"));
            p.pin_man(t1, at, EAST, game::game::GuardState::Patrol, 0.0);
            p.net_frame(0);
            assert!(job(&p).map_or(false, |j| !j.go), "seed {s}: the other player's F sent seat {seat}'s man");
            let marks = p.g.stats.pairorders.marks;
            let own = p.g.tab.cmd_seat[sn].input;
            p.g.tab.input[own].pressed.add(String::from("KeyF"));
            p.pin_man(t1, at, EAST, game::game::GuardState::Patrol, 0.0);
            p.net_frame(0);
            assert!(p.g.stats.pairorders.marks > marks, "seed {s}: seat {seat}'s own F did not send his man: {:?}", job(&p));
        }
    }
}

/// THE RULE: a player folded by a blast is folded whoever else is down. A failure means that with his partner down, a rule about him (a grab, a hand on a switch) lets him act while he is folded.
#[test]
fn a_player_is_folded_whoever_else_is_down() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, _) = two(s);
            let (him, other) = (p.g.pl[man(&p, seat)].ch, p.g.pl[man(&p, 1 - seat)].ch);
            p.g.cond[him].fold_until = p.g.time + 5.0;
            let folded = |p: &Play| game::samstun::sam_staggered(&p.g, &game::seats::seat(&p.g, p.g.seats[seat]), None);
            assert!(folded(&p), "seed {s}: CONTROL: seat {seat}'s man is folded with both up");
            p.g.cond[other].down = true;
            assert!(folded(&p), "seed {s}: seat {seat}'s man reads as not folded because the other player is down");
            p.g.cond[him].down = true;
            assert!(!folded(&p), "seed {s}: a man who is down shows no stun");
        }
    }
}

/// THE RULE: a camera watches every player in its cone. The player nearest the lens does not hide the other.
/// A failure means standing behind a camera shields your partner.
#[test]
fn a_camera_sees_any_player_in_its_cone() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, _) = two(s);
            let c = *p.g.devices.cameras.first().expect("the office has a camera");
            let def = p.g.tab.devices_camera_rt[c].def.clone();
            let yaw = p.g.tab.devices_camera_rt[c].yaw;
            let look = game::devices::lens_dir(yaw, def.pitch.unwrap_or(game::scene_level_data::device_defaults().pitch));
            let m = man(&p, seat);
            for x in p.mates() { p.man_stands(x, [3.5, 3.5]); }
            p.seat_stands(1 - seat, [def.pos[0] - look[0] * 0.4, def.pos[2] - look[2] * 0.4]);
            let spot = [4.0, 3.0, 5.0, 2.0, 6.0].into_iter().map(|d| [def.pos[0] + look[0] * d, 0.0, def.pos[2] + look[2] * d])
                .find(|q| !game::scene_collision::segment_blocked(&mut p.g, def.pos, [q[0], 1.1, q[2]], None, None, None)).expect("the lens looks down some clear floor");
            p.seat_stands(seat, [spot[0], spot[2]]);
            p.frame();
            let pc = p.g.pl[m].ch;
            let spot = p.g.ch[pc].pos;
            p.g.shown[p.g.pl[m].ch].visibility = 1.0;
            let dd = game::scene_level_data::device_defaults();
            let half = def.fov_deg.unwrap_or(dd.fov_deg) / 2.0;
            let yaw = p.g.tab.devices_camera_rt[c].yaw;
            let look = game::devices::lens_dir(yaw, def.pitch.unwrap_or(dd.pitch));
            let e = game::guards::Eye { pos: def.pos, look, range: def.range.unwrap_or(dd.range), half_deg: half, near_m: 0.0, near_deg: half, v_half_deg: Some(half) };
            assert!(game::guards::eye_contact(&mut p.g, e, pc, false, 1.0, None).visible, "seed {s}: CONTROL: the lens has seat {seat}'s man in its cone at {spot:?}");
            game::devices::update_camera(&mut p.g, c, 1.0 / 60.0, true);
            let last = p.g.tab.devices_camera_rt[c].last_seen;
            assert!(last.map_or(false, |l| (l[0] - spot[0]).abs() < 0.1 && (l[2] - spot[2]).abs() < 0.1), "seed {s}: seat {seat}'s man stands at {spot:?} in the cone, the other behind the lens, and the camera last saw {last:?}");
        }
    }
}

/// THE RULE: a kill by a blast is written down with the man who set it off, not whoever stands.
/// A failure means the debrief names the first player's gun for the second player's grenade.
#[test]
fn a_blast_kill_is_the_man_who_set_it_off() {
    for s in SEEDS {
        for seat in 0..2 {
            let (mut p, t1) = two(s);
            let (m, other) = (man(&p, seat), man(&p, 1 - seat));
            p.g.pl[other].holstered = true;
            p.g.pl[m].holstered = false;
            let c = p.g.gd[t1].ch;
            let (who, at) = (p.g.player_id(m), p.g.ch[c].pos);
            let n0 = p.g.chronicle.events.len();
            let spec = game::frag::BlastSpec { who, name: Some(String::from("grenade")), fx: Some(false), ..Default::default() };
            game::frag::blast(&mut p.g, [at[0] + 0.1, 0.05, at[2]], &spec);
            let row = p.g.chronicle.events[n0..].iter().find(|e| e.kind == game::chronicle::ChronKind::Kill).cloned();
            let Some(row) = row else { panic!("seed {s}: CONTROL: a grenade at his feet and no kill row: {:?}", p.g.gd[t1].state) };
            let said = format!("{:?}", row.data);
            assert!(said.contains("grenade"), "seed {s}: seat {seat}'s blast is written with what is in somebody's hands: {said}");
            assert_eq!(game::chronicle::event_text(&p.g, &row), format!("{} killed — the grenade", p.g.gd[t1].callsign), "seed {s}: seat {seat}");
        }
    }
}

/// THE RULE: a kill by a blast names what went off, not the gun in the hand, for one player as for two.
/// A failure means the debrief says "shot dead (FN2000)" of a man a grenade killed.
#[test]
fn a_blast_kill_names_what_went_off_with_one_player() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.3);
        let t1 = p.g.guards.iter().copied().find(|&gd| !p.g.team(gd).ours()).expect("a defender");
        let pl = p.g.player;
        p.g.pl[pl].holstered = false;
        assert!(game::game::player_gun_in_hand(&p.g, pl).is_some(), "seed {s}: CONTROL: a gun in his hand");
        let c = p.g.gd[t1].ch;
        let (who, at) = (p.g.player_id(pl), p.g.ch[c].pos);
        let n0 = p.g.chronicle.events.len();
        let spec = game::frag::BlastSpec { who, name: Some(String::from("grenade")), fx: Some(false), ..Default::default() };
        game::frag::blast(&mut p.g, [at[0] + 0.1, 0.05, at[2]], &spec);
        let row = p.g.chronicle.events[n0..].iter().find(|e| e.kind == game::chronicle::ChronKind::Kill).cloned();
        let Some(row) = row else { panic!("seed {s}: CONTROL: a grenade at his feet and no kill row") };
        assert_eq!(game::chronicle::event_text(&p.g, &row), format!("{} killed — the grenade", p.g.gd[t1].callsign), "seed {s}");
    }
}

/// THE RULE: a civilian killed by a blast is written down under what went off.
/// A failure means the debrief says she was killed by the gun in his hand.
#[test]
fn a_civilian_killed_by_a_blast_names_what_went_off() {
    for s in SEEDS {
        let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.2);
        let her = *p.g.civilians.first().expect("CONTROL: the office has a civilian");
        let pl = p.g.player;
        p.g.pl[pl].holstered = false;
        assert!(game::game::player_gun_in_hand(&p.g, pl).is_some(), "seed {s}: CONTROL: a gun in his hand");
        let c = p.g.gd[her].ch;
        let (who, at) = (p.g.player_id(pl), p.g.ch[c].pos);
        let n0 = p.g.chronicle.events.len();
        let spec = game::frag::BlastSpec { who, name: Some(String::from("grenade")), fx: Some(false), ..Default::default() };
        game::frag::blast(&mut p.g, [at[0] + 0.1, 0.05, at[2]], &spec);
        let row = p.g.chronicle.events[n0..].iter().find(|e| e.kind == game::chronicle::ChronKind::CivilianKilled).cloned();
        let Some(row) = row else { panic!("seed {s}: CONTROL: a grenade at her feet and no row") };
        let said = game::chronicle::event_text(&p.g, &row);
        assert!(said.contains("(grenade)"), "seed {s}: {said}");
    }
}
