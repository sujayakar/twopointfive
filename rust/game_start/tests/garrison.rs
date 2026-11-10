//! EVERY DEFENDER IS IN A SQUAD, wherever he is dealt: a rescue's men and the night breach's as a room clear's (rust/game_rules/src/pursuit/squaddeal.rs).
mod stealth_ask;
use game::enemysquads::SquadEventWhat as What;
use game::ids::GuardId;
use game::teams::Team;
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];

/// a mission of the list, stood as a click on its row stands it, nobody looking for anybody
fn mission(id: &str, seed: u32) -> Play {
    let e = game::scene_scenario_list::all_missions().into_iter().find(|e| e.id == id).expect("on the list");
    let mut p = Play::mission(e.level.as_deref().unwrap_or("office.json"), &game::scene_scenario_list::pick_search(&e, ""), &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.sam_unseen();
    p.run(0.5);
    for o in p.mates() { p.parked(o); }
    p
}
fn defenders(p: &Play) -> Vec<GuardId> { p.g.guards.iter().copied().filter(|&m| p.g.gd[m].team == Team::Hostile).collect() }
fn events(p: &Play) -> Vec<(What, f64)> { p.squad_names().iter().flat_map(|q| p.squad_events(q)).collect() }
/// the men of a squad who are not with the hostages
fn free(p: &Play) -> Vec<GuardId> { defenders(p).into_iter().filter(|&m| !game::mission::leads(&p.g, m)).collect() }

/// THE RULE: every defender of the three rescues and of the night breach is in a squad.
/// A FAILURE MEANS: men who check on nobody, call nobody and come to nobody.
#[test]
fn every_defender_is_in_a_squad() {
    for id in ["hostage-rescue", "rescue-consulate", "rescue-registry", "night-breach"] {
        for s in SEEDS {
            let p = mission(id, s);
            let men = defenders(&p);
            assert!(men.len() >= 2, "{id}, seed {s}: {} men", men.len());
            for &m in &men { assert!(game::enemysquads::squad_of(&p.g, m).is_some(), "{id}, seed {s}: {} is in no squad", p.g.gd[m].callsign); }
        }
    }
}

/// THE RULE: a man of a rescue dropped with nobody looking is missed by his squad's radio check: asked for, and looked for.
/// CONTROL: nobody dropped, and nobody is asked for.
/// A FAILURE MEANS: the guards of a house with hostages in it never miss one of theirs.
#[test]
fn a_rescues_men_miss_one_of_theirs() {
    for s in SEEDS {
        let mut p = mission("rescue-consulate", s);
        let men = free(&p);
        let alone = men.iter().copied().max_by(|&a, &b| {
            let far = |m: GuardId| defenders(&p).into_iter().filter(|&o| o != m).map(|o| dist(p.at(m), p.at(o))).fold(f64::MAX, f64::min);
            far(a).total_cmp(&far(b))
        }).expect("a man who is not with the hostages");
        game::combat::kill_guard(&mut p.g, alone, [1.0, 0.0, 0.0], Some(true), None, None, None);
        let asked = p.until(90.0, |p| events(p).iter().any(|e| e.0 == What::Asks));
        assert!(asked.is_some(), "seed {s}: 90 s and nobody asks for him: {:?}", events(&p));
        let looked = p.until(40.0, |p| events(p).iter().any(|e| e.0 == What::Looks));
        assert!(looked.is_some(), "seed {s}: nobody goes to look: {:?}", events(&p));
        drop(p);

        let mut p = mission("rescue-consulate", s);
        p.run(60.0);
        assert!(!events(&p).iter().any(|e| e.0 == What::Asks), "seed {s}: CONTROL: everybody up, and somebody is asked for");
    }
}

/// THE RULE: the team among a rescue's men is contact for their squad, and a mate of another room comes to them.
/// CONTROL: no bang: nobody leaves his post.
/// A FAILURE MEANS: each of them waits in his own room to be shot in turn.
#[test]
fn a_rescues_men_come_to_each_other() {
    for s in SEEDS {
        let mut p = mission("rescue-consulate", s);
        let hit = free(&p)[0];
        let at = p.at(hit);
        p.flash_lands([at[0] + 1.5, at[1]]);
        let came = p.until(20.0, |p| events(p).iter().any(|e| matches!(e.0, What::Reinforce | What::Help | What::Flank)));
        assert!(came.is_some(), "seed {s}: nobody comes to him: {:?}", events(&p));
        drop(p);

        let mut p = mission("rescue-consulate", s);
        p.run(20.0);
        assert!(!events(&p).iter().any(|e| matches!(e.0, What::Contact | What::Reinforce | What::Help | What::Flank)), "seed {s}: CONTROL: nothing happened, and they move: {:?}", events(&p));
    }
}
