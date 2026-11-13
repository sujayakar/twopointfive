//! A SQUAD COMES TO A FIGHT TOGETHER (rust/game_rules/src/pursuit/enemysquads): its men in its other rooms are there at the same time, and one alone keeps his own door.
mod stealth_ask;
use game::enemysquads::SquadEventWhat as What;
use game::game::DownBy;
use game::ids::GuardId;
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];

/// a room clear, nobody looking for anybody
fn floor(level: &str, seed: u32) -> Play {
    let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4"), &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.sam_unseen();
    p.run(0.5);
    for o in p.mates() { p.parked(o); }
    p
}
fn events(p: &Play, what: What) -> Vec<f64> { p.squad_names().iter().flat_map(|q| p.squad_events(q)).filter(|e| e.0 == what).map(|e| e.1).collect() }
/// how far a man has to go
fn walk(p: &mut Play, m: GuardId, to: [f64; 2]) -> f64 {
    let from = p.g.pos_of(m);
    let way = game::scene_collision::nav_grid_find_path(&mut p.g, from, [to[0], 0.0, to[1]], None, None).expect("a way");
    way.iter().fold((0.0, p.at(m)), |(l, q), w| (l + dist(q, [w[0], w[2]]), [w[0], w[2]])).0
}

/// THE RULE: called to one of their rooms, two men of a squad from its other rooms are there within a moment of each other: the near one leaves later, up to five seconds.
/// A FAILURE MEANS: they come in one at a time, and are shot one at a time.
#[test]
fn a_squads_men_are_there_together() {
    let mut pairs = 0;
    for s in SEEDS {
        for level in ["compound", "office", "pass"] {
            let mut p = floor(level, s);
            let hit = p.hostiles()[0];
            let at = p.at(hit);
            p.flash_lands([at[0] + 1.0, at[1]]);
            if p.until(6.0, |p| p.g.tab.enemysquads_squad.iter().any(|(_, q)| q.due.len() >= 2)).is_none() { continue; }
            let due: Vec<(GuardId, f64, [f64; 2])> = p.g.tab.enemysquads_squad.iter().find(|(_, q)| q.due.len() >= 2).map(|(_, q)| q.due.iter().map(|d| (d.gd, d.t, [d.at[0], d.at[2]])).collect()).unwrap();
            let now = p.time();
            let pace = p.g.k.tune.guard_chase;
            let there: Vec<(f64, f64)> = due.iter().map(|&(m, t, to)| (t + walk(&mut p, m, to) / pace, t - now)).collect();
            let last = there.iter().map(|x| x.0).fold(0.0, f64::max);
            for &(arrives, leaves_in) in &there { assert!(last - arrives < 0.3 || leaves_in > 5.0, "{level}, seed {s}: one is there {:.1} s before the last, and left after {leaves_in:.1} s", last - arrives); }
            pairs += 1;
        }
    }
    assert!(pairs >= 3, "the staging is wrong: {pairs} squads with two men called");
}

/// THE RULE: one man of a squad left, and nobody of theirs up in the room that was hit: he stays on his door.
/// CONTROL: a mate up in that room, and he comes.
/// A FAILURE MEANS: the last of them walk into a lost room by themselves, one after the other.
#[test]
fn one_man_alone_keeps_his_door() {
    let (mut stayed, mut came) = (0, 0);
    for s in SEEDS {
        for lost in [true, false] {
            let mut p = floor("office", s);
            let men = p.hostiles();
            // (the man who stands most by himself, so that nobody dropped here is of his room, and the man nearest him)
            let gap = |p: &Play, m: GuardId| men.iter().filter(|&&o| o != m).map(|&o| dist(p.at(m), p.at(o))).fold(f64::MAX, f64::min);
            let other = *men.iter().max_by(|&&a, &&b| gap(&p, a).total_cmp(&gap(&p, b))).unwrap();
            let hit = *men.iter().filter(|&&m| m != other).min_by(|&&a, &&b| dist(p.at(a), p.at(other)).total_cmp(&dist(p.at(b), p.at(other)))).unwrap();
            // (the rest have run: gone, and nothing heard of it)
            for &m in men.iter().filter(|&&m| m != hit && m != other) { p.g.gd[m].fled = true; p.man_stands(m, [2.0, 2.0]); }
            p.run(0.2);
            let at = p.at(hit);
            p.flash_lands([at[0] + 1.0, at[1]]);
            p.until(6.0, |p| !events(p, What::Contact).is_empty()).unwrap_or_else(|| panic!("seed {s}: no contact"));
            if lost { p.falls(hit, DownBy::Player); }
            p.run(4.0);
            if lost {
                assert!(events(&p, What::Reinforce).is_empty(), "seed {s}: he goes in alone");
                stayed += events(&p, What::Stays).len();
            } else {
                came += events(&p, What::Reinforce).len();
            }
        }
    }
    assert!(stayed >= 2 && came >= 2, "the staging is wrong: {stayed} stayed, {came} came");
}

/// the man and the room of every man of a squad who is due at a fight and not off yet
fn due(p: &Play) -> Vec<(game::ids::EnemysquadsSquadId, GuardId, String)> {
    p.g.tab.enemysquads_squad.iter().flat_map(|(q, x)| x.due.iter().map(move |d| (q, d.gd, d.room.as_ref().map_or(String::new(), |r| r.name.clone())))).collect()
}
/// the room of his squad's he stands in
fn his_room(p: &Play, q: game::ids::EnemysquadsSquadId, m: GuardId) -> Option<String> {
    game::enemysquads::room_of(&p.g, q, p.g.pos_of(m), None).map(|r| r.name)
}

/// THE RULE: a man about to leave for another room's fight stays when his own room is hit, and goes to the last room hit when that is another again.
/// A FAILURE MEANS: men walk out of the room that is being entered, to one the team has left (before: 92 of 314 men called to a fight, in 104 clears).
#[test]
fn a_man_about_to_go_goes_where_they_are_now() {
    let (mut stays, mut turned) = (0, 0);
    for s in SEEDS {
        for level in ["compound", "office", "pass", "depot", "warehouse", "barracks"] {
            let mut p = floor(level, s);
            let hit = p.hostiles()[0];
            let at = p.at(hit);
            p.flash_lands([at[0] + 1.0, at[1]]);
            if p.until(6.0, |p| !due(p).is_empty()).is_none() { continue; }
            let (q, m, a) = due(&p)[0].clone();
            let men = p.g.tab.enemysquads_squad[q].men.clone();
            let mine = his_room(&p, q, m);
            let third = men.iter().copied().find(|&o| p.up(o) && his_room(&p, q, o).is_some_and(|r| r != a && Some(&r) != mine.as_ref()));
            // another room of theirs, and then his own
            if let Some(o) = third {
                let b = his_room(&p, q, o).unwrap();
                let at = p.at(o);
                p.flash_lands([at[0] + 1.0, at[1]]);
                p.until(1.0, |p| p.g.tab.enemysquads_squad[q].hit_room.as_ref().is_some_and(|r| r.name == b));
                if let Some(x) = due(&p).into_iter().find(|x| x.1 == m) { if his_room(&p, q, m) == mine { assert_eq!(x.2, b, "{level}, seed {s}: he is due in {} when they are in {b}", x.2); turned += 1; } }
            }
            if !due(&p).iter().any(|x| x.1 == m) { continue; }
            let at = p.at(m);
            p.flash_lands([at[0] + 1.0, at[1]]);
            let Some(_) = p.until(1.0, |p| p.g.tab.enemysquads_squad[q].hit_room.as_ref().map(|r| r.name.clone()) == mine) else { continue };
            assert!(!due(&p).iter().any(|x| x.1 == m), "{level}, seed {s}: his own room is hit and he is due in another");
            stays += 1;
        }
    }
    assert!(stays >= 3 && turned >= 3, "the staging is wrong: {stays} stay, {turned} turned");
}

/// THE RULE: two men or more of a drilled squad called to one of their rooms, with a way round to one of its doors that no gun they know of looks down, go in by it as a player's men do, by the
/// squad's own door tools, the first two through it within a moment of each other.
/// CONTROL: with nobody steady in it, a squad sends each man by himself.
/// A FAILURE MEANS: they trickle back into a room one at a time, or green men by themselves work as a squad.
#[test]
fn a_drilled_squad_goes_back_into_a_room_together() {
    let (mut went, mut green) = (0, 0);
    let mut apart: Vec<f64> = Vec::new();
    for s in SEEDS {
        for level in ["compound", "warehouse", "barracks", "pass"] {
            for word in ["", "&training=green"] {
                let mut p = Play::mission(&format!("{level}.json"), &format!("?levelFile={level}.json&roomclear&squad4{word}"), &Opts { seed: s, ..Default::default() });
                p.sam_unhurt();
                p.sam_unseen();
                p.run(0.5);
                for o in p.mates() { p.parked(o); }
                let hit = p.hostiles()[0];
                let at = p.at(hit);
                p.flash_lands([at[0] + 1.0, at[1]]);
                let entry = |p: &Play| p.g.tab.enemysquads_squad.iter().find_map(|(_, q)| q.entries.first().map(|e| (e.op, e.men.clone())));
                if p.until(12.0, |p| entry(p).is_some()).is_none() { if word.is_empty() { continue; } else { green += (!events(&p, What::Reinforce).is_empty()) as usize; continue; } }
                assert!(word.is_empty(), "{level}, seed {s}: green men work as a squad");
                let (op, men) = entry(&p).unwrap();
                assert!(men.len() >= 2, "{level}, seed {s}: {men:?}");
                p.until(40.0, |p| p.g.tab.doortools_door_op[op].crossed.len() >= 2 || p.g.tab.doortools_door_op[op].ended.is_some());
                let mut at: Vec<f64> = p.g.tab.doortools_door_op[op].crossed.values().copied().collect();
                at.sort_by(f64::total_cmp);
                if at.len() >= 2 { apart.push(at[1] - at[0]); went += 1; }
            }
        }
    }
    assert!(went >= 3 && green >= 3, "the staging is wrong: {went} went in together, {green} green squads sent men");
    assert!(apart.iter().all(|&a| a < 4.0), "the first two through a door: {apart:.1?} s apart");
}
