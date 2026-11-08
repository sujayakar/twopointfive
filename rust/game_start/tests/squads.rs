//! THE SQUADS ANSWER EACH OTHER ON THE NET: what one squad calls, the others do something about, each as its own rooms and its own orders allow.
use game::enemysquads::{contact, SquadChoice};
use game::ids::EnemysquadsSquadId;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

fn compound(seed: u32) -> Play {
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(1.0);
    p
}

fn squad(p: &Play, name: &str) -> EnemysquadsSquadId {
    let st = p.g.m.enemysquads.states.as_ref().expect("the compound's squads stand");
    st.squads.iter().copied().find(|&q| p.g.tab.enemysquads_squad[q].name == name).unwrap_or_else(|| panic!("the compound has no squad {name}"))
}

/// THE RULE: West calls contact in its own room and the net carries it. Yard, five men over two rooms, sends men out on the call and keeps a man home for each room;
/// Keep, whose orders are never to leave, shuts its doors and sends nobody. CONTROL: the same seconds with no call, and nobody goes anywhere.
/// A FAILURE MEANS: a squad hears a fight and sits it out, leaves its own rooms empty, or walks out against its orders.
#[test]
fn a_call_on_the_net_brings_men_out_of_the_squad_that_can_spare_them() {
    for seed in SEEDS {
        {
            let mut quiet = compound(seed);
            let yard = squad(&quiet, "Yard");
            quiet.run(8.0);
            assert!(quiet.g.tab.enemysquads_squad[yard].state0.is_none() && quiet.g.tab.enemysquads_squad[yard].sent.is_empty(), "seed {seed}: with no call Yard moved: {:?}", quiet.g.tab.enemysquads_squad[yard].state0);
        }

        let mut p = compound(seed);
        let (west, yard, keep) = (squad(&p, "West"), squad(&p, "Yard"), squad(&p, "Keep"));
        let room = p.g.tab.enemysquads_squad[west].rooms[0].clone();
        let rally = p.g.tab.enemysquads_squad[west].def.rally.expect("West has a rally");
        contact(&mut p.g, west, rally, Some(&room), None, None, None);
        p.run(8.0);
        let y = &p.g.tab.enemysquads_squad[yard];
        assert!(matches!(y.state0, Some(SquadChoice::Help) | Some(SquadChoice::Flank)), "seed {seed}: Yard's answer was {:?}", y.state0);
        assert!(y.sent.len() >= 2 && y.men.len() - y.sent.len() >= y.rooms.len(), "seed {seed}: Yard sent {} of {} men with {} rooms to keep", y.sent.len(), y.men.len(), y.rooms.len());
        let k = &p.g.tab.enemysquads_squad[keep];
        assert!(k.state0 == Some(SquadChoice::Barricade) && k.sent.is_empty(), "seed {seed}: Keep's answer was {:?}, {} sent", k.state0, k.sent.len());
    }
}
