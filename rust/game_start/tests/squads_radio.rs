//! A DEFENDING SQUAD'S NET. A drop nobody saw is found out by its radio check; a noise nobody can place sends nobody out.
mod stealth_ask;
use game::enemysquads::SquadEventWhat as What;
use game::game::GuardState;
use game::ids::GuardId;
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];

fn compound(s: u32) -> Play {
    let mut p = Play::mission("compound.json", &format!("?levelFile=compound.json&roomclear={s}&squad4"), &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.run(1.0);
    p
}
/// the man of a squad standing farthest from his nearest mate, and how far
fn loneliest(p: &Play, men: &[GuardId]) -> (GuardId, f64) {
    men.iter().map(|&m| (m, men.iter().filter(|&&o| o != m).map(|&o| dist(p.at(m), p.at(o))).fold(f64::MAX, f64::min))).max_by(|a, b| a.1.total_cmp(&b.1)).expect("a squad")
}
fn first(ev: &[(What, f64)], what: What) -> Option<f64> { ev.iter().find(|e| e.0 == what).map(|e| e.1) }

/// THE RULE: a man dropped with nobody looking, in the dark, is no contact at the drop. His squad's check finds him out: asked for, looked for, found, and only then contact, in that order, and the squads next door do nothing until it does.
/// CONTROL: the same floor with nobody dropped: no contact.
/// A FAILURE MEANS: the squad knows of a silent kill at once, or never learns of it.
#[test]
fn a_drop_nobody_saw_is_found_by_the_squads_check() {
    for s in SEEDS {
        let mut p = compound(s);
        p.g.k.body_light.lit = 99.0;
        p.g.k.body_light.lit_near = 99.0;
        p.g.player_invisible = true;
        for &o in p.mates().iter() { p.parked(o); }
        let (men, _) = p.squad_men("Yard");
        let (m, _) = loneliest(&p, &men);
        game::combat::kill_guard(&mut p.g, m, [1.0, 0.0, 0.0], Some(true), None, None, game::game::With::Hand);
        p.run(0.1);
        assert!(first(&p.squad_events("Yard"), What::Contact).is_none(), "seed {s}: contact at the drop, with nobody looking");
        p.until(90.0, |p| first(&p.squad_events("Yard"), What::Contact).is_some()).unwrap_or_else(|| panic!("seed {s}: 90 s and nobody found him: {:?}", p.squad_events("Yard")));
        let ev = p.squad_events("Yard");
        let (asks, looks, found, contact) = (first(&ev, What::Asks), first(&ev, What::Looks), first(&ev, What::Found), first(&ev, What::Contact));
        let (asks, looks, found, contact) = (asks.expect("asked for"), looks.expect("looked for"), found.expect("found"), contact.expect("contact"));
        assert!(asks < looks && looks <= found && found <= contact, "seed {s}: the order: asks {asks} looks {looks} found {found} contact {contact}");
        for q in ["West", "Keep", "Gate"] {
            let early: Vec<_> = p.squad_events(q).into_iter().filter(|e| e.1 < contact && !matches!(e.0, What::Check | What::NetCheck)).collect();
            assert!(early.is_empty(), "seed {s}: {q} moved before the body was found: {early:?}");
        }
        drop(p);

        let mut p = compound(s);
        p.g.k.body_light.lit = 99.0;
        p.g.k.body_light.lit_near = 99.0;
        p.g.player_invisible = true;
        for &o in p.mates().iter() { p.parked(o); }
        p.run(20.0);
        assert!(first(&p.squad_events("Yard"), What::Contact).is_none(), "seed {s}: CONTROL: nobody dropped and it is contact");
    }
}

/// THE RULE: one loud report where the team starts, nobody in contact: every squad says one thing, its doors, and sends nobody; the squad that may not leave stays in its rooms.
/// CONTROL: no report: no squad says anything.
/// A FAILURE MEANS: a noise pulls men out of their rooms to be shot in the yard.
#[test]
fn nobody_walks_out_at_a_noise() {
    for s in SEEDS {
        let mut p = compound(s);
        for &o in p.mates().iter() { p.parked(o); }
        p.loud_report();
        p.run(6.5);
        for q in p.squad_names() {
            let said: Vec<_> = p.squad_events(&q).into_iter().filter(|e| !matches!(e.0, What::Check | What::NetCheck)).collect();
            assert_eq!(said.len(), 1, "seed {s}: {q} said: {said:?}");
            assert_eq!(said[0].0, What::Barricade, "seed {s}: {q} did not stand to on its doors");
            assert_eq!(p.squad_men(&q).1, 0, "seed {s}: {q} sent men at a noise");
        }
        let (keep, _) = p.squad_men("Keep");
        for m in keep { if p.state(m) != GuardState::Dead { assert!(p.in_squad_rooms("Keep", p.at(m)), "seed {s}: Keep's man left its rooms: {:?}", p.at(m)); } }
        drop(p);

        let mut p = compound(s);
        for &o in p.mates().iter() { p.parked(o); }
        p.run(6.5);
        for q in p.squad_names() {
            let said: Vec<_> = p.squad_events(&q).into_iter().filter(|e| !matches!(e.0, What::Check | What::NetCheck)).collect();
            assert!(said.is_empty(), "seed {s}: CONTROL: no report and {q} said: {said:?}");
        }
    }
}
