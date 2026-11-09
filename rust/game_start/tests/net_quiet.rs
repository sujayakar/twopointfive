//! THE NET GOES QUIET: a mate who does not answer shakes the man who asked and the men who heard; the calls get worse; one dead man is one shock; and squads told to stay in their rooms still miss a room that goes silent.
mod world_ask;
use game::morale::Training;
use game_start::play::{dist, Opts, Play};
use world_ask::NetAsk;

/// The compound's room clear, Sam alone and out of everybody's sight, the level's own squads and rooms.
fn compound(s: u32, training: Training) -> Play {
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear", &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    p.g.player_invisible = true;
    p.g.k.morale.training = training;
    p.run(0.5);
    p
}

/// THE RULE: a man of the squad does not answer its leader: the asker takes the unanswered check's shock, the two who heard it the smaller, a steady man a share of it, the silent man nothing; each man once for that silence, however often he is asked for.
/// A FAILURE MEANS: the dread either does not grow at all or grows without end for one silent man.
#[test]
fn a_mate_who_does_not_answer_shakes_the_man_who_asked_and_the_men_who_heard() {
    for s in [61u32, 63] {
        let mut p = compound(s, Training::Mixed);
        let (leader, shed) = (p.men_of_room("garage")[0], p.men_of_room("shed"));
        let (b, c, silent) = (shed[0], shed[1], shed[3]);
        let men = p.squad_men_of(leader);
        let was: Vec<f64> = [leader, b, c, silent].iter().map(|&m| p.shaken(m)).collect();
        let k = p.g.k.netquiet;
        game::netquiet::net_quiet_asked(&mut p.g, Some(leader), silent, &men);
        let now: Vec<f64> = [leader, b, c, silent].iter().map(|&m| p.shaken(m)).collect();
        let share = |p: &mut Play, m| if p.steady(m) { k.steady_k } else { 1.0 };
        let want = [k.unanswered * share(&mut p, leader), k.heard_it * share(&mut p, b), k.heard_it * share(&mut p, c), 0.0];
        for i in 0..4 { assert!((now[i] - was[i] - want[i]).abs() < 1e-6, "seed {s}: man {i} took {:.3}, owed {:.3}", now[i] - was[i], want[i]); }
        game::netquiet::net_quiet_asked(&mut p.g, Some(leader), silent, &men);
        let again: Vec<f64> = [leader, b, c, silent].iter().map(|&m| p.shaken(m)).collect();
        assert_eq!(again, now, "seed {s}: asked again for the same silence and shaken again");
    }
}

/// THE RULE: the calls get worse, each a beat after its check, on the net: the name, then why is nobody answering; that again and NOT the last line with his own squad silent but other squads' men up; somebody talk to me only with nobody left on the whole net.
/// A FAILURE MEANS: a man cries out to nobody while the floor is full of his friends, or the net never shows the player what he has done to it.
#[test]
fn the_calls_get_worse_and_the_last_line_is_only_for_the_last_man() {
    let mut p = compound(61, Training::Mixed);
    let (a, shed) = (p.men_of_room("garage")[0], p.men_of_room("shed"));
    let men = p.squad_men_of(a);
    let (beat, gap) = (p.g.k.netquiet.ask_beat, p.g.k.netquiet.bark_secs + 0.2);
    let next = |p: &mut Play, secs: f64| { let n = p.said.len(); p.run(secs); p.said[n..].iter().filter(|l| l.who == a).map(|l| (l.text.clone(), l.radio)).collect::<Vec<_>>() };
    game::netquiet::net_quiet_asked(&mut p.g, Some(a), shed[3], &men);
    let first = next(&mut p, beat + 0.6);
    assert!(first.len() == 1 && first[0].1, "the first check: said {first:?}");
    p.run(gap);
    game::netquiet::net_quiet_asked(&mut p.g, Some(a), shed[1], &men);
    let second = next(&mut p, beat + 0.6);
    assert!(second.first().map_or(false, |l| l.1 && p.net_line(&l.0, "why", "")), "the second check: said {second:?}");
    p.run(gap);
    game::netquiet::net_quiet_asked(&mut p.g, Some(a), shed[0], &[a, shed[0]]);
    let third = next(&mut p, beat + 0.6);
    assert!(third.first().map_or(false, |l| p.net_line(&l.0, "why", "")) && p.hostiles().iter().any(|m| !men.contains(m)), "his squad quiet, others on the net: said {third:?}");
    p.run(gap);
    for m in p.hostiles() { if m != a { p.put_down(m); } }
    let last = next(&mut p, 5.0);
    assert!(last.iter().any(|l| l.1 && p.net_line(&l.0, "last", "")), "nobody left on the net: said {last:?}");
}

/// THE RULE: one dead man is one shock, the largest of three: the man who saw him fall with no enemy in view takes that and nothing more for the check nobody answered or for finding him. CONTROL: the man who looked away takes the check's, and only the rest of being told of the body.
/// A FAILURE MEANS: a single kill counts three times and breaks a squad by arithmetic.
#[test]
fn one_dead_man_is_one_shock() {
    let mut p = compound(62, Training::Green);
    let shed = p.men_of_room("shed");
    let (a, b, c) = (shed[0], shed[1], shed[2]);
    let men = p.squad_men_of(a);
    let (pa, pc) = (p.at(a), p.at(c));
    let d = dist(pa, pc);
    let side = [pa[0] + (pc[0] - pa[0]) / d * 1.5, pa[1] + (pc[1] - pa[1]) / d * 1.5];
    p.man_stands(b, side);
    p.run(0.05);
    let yaw = |q: [f64; 2], r: [f64; 2]| (r[0] - q[0]).atan2(r[1] - q[1]);
    let (ca, cb) = (p.g.gd[a].ch, p.g.gd[b].ch);
    (p.g.ch[ca].body_yaw, p.g.ch[ca].aim_yaw) = (yaw(pa, pc), yaw(pa, pc));
    let away = yaw(pc, side);
    (p.g.ch[cb].body_yaw, p.g.ch[cb].aim_yaw) = (away, away);
    let k = p.g.k.netquiet;
    let was = [p.shaken(a), p.shaken(b)];
    p.falls_as(c, game::game::DownKind::Shot, true, game::game::DownBy::Player);
    let fall = [p.shaken(a), p.shaken(b)];
    game::netquiet::net_quiet_asked(&mut p.g, Some(a), c, &men);
    let check = [p.shaken(a), p.shaken(b)];
    game::netquiet::net_quiet_body(&mut p.g, Some(a), &men);
    let body = [p.shaken(a), p.shaken(b)];
    assert!((fall[0] - was[0] - k.fall_unseen).abs() < 1e-6 && (body[0] - was[0] - k.fall_unseen).abs() < 1e-6, "the watcher: fall {:.3}, then the check {:.3}, then the body {:.3} (owed {:.3} in all)", fall[0] - was[0], check[0] - fall[0], body[0] - check[0], k.fall_unseen);
    assert!((check[1] - was[1] - k.heard_it).abs() < 1e-6 && (body[1] - was[1] - k.body_told).abs() < 1e-6, "the man who looked away: fall {:.3}, check {:.3}, told {:.3} (owed {:.3} then {:.3} in all)", fall[1] - was[1], check[1] - fall[1], body[1] - check[1], k.heard_it, k.body_told);
}

/// The block's floor, four to a mission and every squad told to stay in its rooms; no team on it, Sam out of sight.
fn block(s: u32, on: bool) -> Play {
    let mut p = Play::mission("block.json", "?levelFile=block.json&roomclear&squad=0", &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true;
    p.g.player_invisible = true;
    p.g.k.stayask.men = if on { 1.0 } else { 0.0 };
    p.run(1.0);
    p
}
/// the room whose men the quiet plan puts down: the farthest from Sam with a man in it
fn back_room(p: &Play) -> Vec<game::ids::GuardId> {
    let far = |r: &&game::roomclear::RoomState| r.defenders.first().map_or(0.0, |&m| dist(p.at(m), p.sam_at()));
    p.g.room_clear.rooms.iter().filter(|r| !r.defenders.is_empty()).max_by(|a, b| far(a).partial_cmp(&far(b)).unwrap()).map(|r| r.defenders.clone()).expect("a room with a man")
}

/// THE RULE: squads told to stay still miss a room that goes silent: the men of a room put down one a second without a sound: the roll call, then the squad asked for by name, then the word that one man goes, in that order, and ONE man leaves his post. CONTROL: the rule gives no man to send: nobody goes.
/// A FAILURE MEANS: a whole wing can be emptied in silence and the floor never knows, or a silence empties the floor.
#[test]
fn squads_that_stay_in_their_rooms_still_miss_a_room_that_goes_silent() {
    let (roll, ask, goes, left, said) = silent_wing(true);
    let at = |t: Option<f64>, what: &str| t.unwrap_or_else(|| panic!("no {what}: {said:?}"));
    let (roll, ask, goes) = (at(roll, "roll call"), at(ask, "ask by name"), at(goes, "man sent"));
    assert!(roll < ask && ask <= goes && left == 1, "roll call {roll:.1} s, asked by name {ask:.1} s, one sent {goes:.1} s, {left} men off their posts: {said:?}");
}

/// THE CONTROL: the rule's own number at nobody to send: the same silence sends nobody.
#[test]
fn with_nobody_to_send_a_silence_moves_nobody() {
    let (_, _, goes, left, said) = silent_wing(false);
    assert!(goes.is_none() && left == 0, "nobody to send: one sent at {goes:?}, {left} off their posts: {said:?}");
}

/// the back room's men put down one a second without a sound, and the net's check come due: when it asked, when it asked by name, when it sent a man, how many left their posts, what was said
fn silent_wing(on: bool) -> (Option<f64>, Option<f64>, Option<f64>, usize, Vec<String>) {
    use game::enemysquads::SquadEventWhat::{NetAsks, NetCheck, NetLooks};
    {
        let mut p = block(7, on);
        let victims = back_room(&p);
        let men: Vec<_> = p.hostiles().into_iter().filter(|m| !victims.contains(m)).collect();
        let posts: Vec<[f64; 2]> = men.iter().map(|&m| p.at(m)).collect();
        for &v in &victims { p.put_down(v); p.run(1.0); }
        p.net_check_due(1.0);   // (its clock runs a minute and more: this is a test of what it finds)
        let went = p.until(14.0, |p| p.net_event_at(NetLooks).is_some());
        if went.is_some() { p.run(3.0); }
        let left = men.iter().zip(&posts).filter(|(&m, q)| dist(p.at(m), **q) > 1.0).count();
        (p.net_event_at(NetCheck), p.net_event_at(NetAsks), p.net_event_at(NetLooks), left, p.said.iter().filter(|l| l.radio).map(|l| l.text.clone()).collect::<Vec<_>>())
    }
}
