//! WHERE A MAN GOES FOR COVER (suppression `cover_from`, asked by a man breaking off, under fire, startled or at a blown wall).
use game::ids::GuardId;
use game_start::play::{Opts, Play};

/// Mousehole with four: the clinic's nine men, every squad told to stay in its rooms, and a burst from Sam in the corridor.
fn clinic(seed: u32, hour: &str) -> (Play, Vec<GuardId>) {
    let link = format!("?levelFile=mousehole.json&roomclear&squad=4&hour={hour}");
    let mut p = Play::mission("mousehole.json", &link, &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.g.fog_of_war.on = false;
    p.run(0.5);
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| !p.g.team(gd).ours()).collect();
    (p, foes)
}

/// THE RULE: a man whose squad stays in its rooms takes cover in them. The clinic's duty room has a locked steel fire exit onto a yard the team cannot walk to: staff carry the key. A man who has
/// broken and run may still bolt by it, and a mission doesn't wait for a man who has fled.
/// A FAILURE MEANS: a defender lets himself out for cover and the mission can't be finished, since the team is refused at that door (before: Mousehole by day, the last man in the yard until the clock ran out).
#[test]
fn a_man_who_stays_in_his_rooms_takes_cover_in_them() {
    for (seed, hour) in [(4, "midday"), (2, "midday")] {   // (the first deal is the one where cover took him out; on the second a man breaks and runs)
        let (mut p, foes) = clinic(seed, hour);
        assert!(foes.iter().all(|&f| p.g.gd[f].stays_in_rooms), "the staging is wrong: a man free to leave");
        p.sam_put([12.0, 15.6]);
        p.run(0.3);
        p.sam_takes_his_rifle();
        p.sam_fires_at([20.0, 15.6], 0.3);
        for _ in 0..(60 * 40) {
            p.frame();
            for &f in &foes {
                if game::condition::down(&p.g, p.g.gd[f].ch) || p.g.gd[f].fled { continue; }
                let q = p.g.ch[p.g.gd[f].ch].pos;
                assert!(q[0] < 33.0, "seed {seed} {hour}: {} is outside at {q:?} and has not fled", p.g.gd[f].callsign);
            }
        }
    }
}

/// THE RULE: a man under fire does not stop to open a door: floor behind a shut one is no cover, whether or not he may leave. The same man with the door standing open may take it.
/// A FAILURE MEANS: a man runs for cover through a door shut in his face, a locked one if he holds the key.
#[test]
fn floor_behind_a_shut_door_is_no_cover() {
    let (mut p, foes) = clinic(1, "night");
    let door = game::doors::doors_by_name(&p.g, "fire_exit").unwrap();
    let man = foes[8];
    p.g.gd[man].stays_in_rooms = false;
    let c = p.g.gd[man].ch;
    p.g.ch[c].pos = [32.2, 0.0, 9.2];
    let threat = [28.0, 0.0, 9.2];
    let shut = game::suppression::cover_from(&mut p.g, man, threat);
    assert!(shut.map_or(true, |s| s[0] < 33.0), "the door shut: cover at {shut:?}, past it");
    p.g.dr[door].leaf.locked = false;
    p.g.dr[door].leaf.latched = false;
    let open = game::suppression::cover_from(&mut p.g, man, threat);
    assert!(open.map_or(false, |s| s[0] > 33.0), "the staging is wrong: the door off its latch and still no cover past it: {open:?}");
    p.g.gd[man].stays_in_rooms = true;
    let stays = game::suppression::cover_from(&mut p.g, man, threat);
    assert!(stays.map_or(true, |s| s[0] < 33.0), "told to stay in his rooms, the door open: cover at {stays:?}, outside them");
}
