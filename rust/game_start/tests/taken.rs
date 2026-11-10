//! ORDERS TAKEN AWAY END WHAT THEY STARTED: what an order, or what watches over a man under one, put up (a gun off its sling, a raised gun, a walk
//! backwards) comes down when his orders are taken.
use game::ids::GuardId;
use game::orders::{squad_orders_clear, squad_orders_give, squad_orders_target_at, Who};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [5, 6, 7];

/// the office's room clear with every defender held blind, so that nothing but the orders moves the squad
fn office(seed: u32) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.5);
    for gd in p.g.guards.clone() { if p.g.gd[gd].team == game::teams::Team::Hostile { p.g.cond[p.g.gd[gd].ch].blind_until = 1e9; } }
    p
}

/// THE RULE: the squad taken off its orders while a man walks backwards on an opening he watches: from the next frame nobody keeps the sidestep, the
/// place he aimed at or the raised gun.
/// A FAILURE MEANS: what the cover wrote into a man's script outlives the order it was written under.
#[test]
fn a_man_taken_off_his_orders_lowers_the_gun_the_cover_raised() {
    for s in SEEDS {
        let mut p = office(s);
        let sam = [13.0, 17.6];
        p.sam_put(sam);
        p.order("fallIn", sam, Who::All);
        p.run(4.0);
        let backwards = |p: &Play, m: GuardId| p.g.gd[m].script.as_ref().map_or(false, |x| x.strafe == Some(true)) && p.g.cover.jobs.get(m).map_or(false, |j| j.back);
        let mut k = 0.0;
        let cut = p.until(9.0, |p| { k += 1.0; p.sam_put([sam[0] + 13.0 * k / 540.0, sam[1]]); p.mates().into_iter().any(|m| backwards(p, m)) });
        assert!(cut.is_some(), "seed {s}: nobody walked backwards on an opening in nine seconds of Sam's walk");
        squad_orders_clear(&mut p.g, None);
        p.frame();
        for m in p.mates() {
            let x = p.g.gd[m].script.as_ref().expect("a man with no order is on the runner's script");
            assert!(!p.g.cover.jobs.has(m) && x.strafe != Some(true) && x.aim_at.is_none() && x.upper != Some(game::character::UpperMode::Aim), "seed {s}: {} a frame after his orders were taken: watching {}, strafe {:?}, aim {:?}, upper {:?}", p.g.gd[m].callsign, p.g.cover.jobs.has(m), x.strafe, x.aim_at, x.upper);
        }
    }
}

/// THE RULE: a man with the shotgun off its sling for a lock, his orders taken before he fires: his own gun is back in his hands and the shotgun slung.
/// A FAILURE MEANS: a door's work goes on after the order it was for.
#[test]
fn a_man_taken_off_a_locked_door_slings_the_shotgun() {
    for s in SEEDS {
        let mut p = office(s);
        for m in p.mates() { if let Some(e) = p.g.m.kit.man_of.get_mut(m) { e.kit.big = game::kit::BigKit::Shotgun; } }
        let d = p.g.doors.list.iter().copied().find(|&d| p.g.dr[d].def.name == "manager").expect("the office has the manager's door");
        let c = p.g.dr[d].frame_centre;
        p.sam_put([c[0] + 2.0, c[2]]);
        p.order("fallIn", [c[0] + 2.0, c[2]], Who::All);
        p.run(6.0);
        let own = p.mates().into_iter().map(|m| (m, p.g.gd[m].weapon)).collect::<Vec<_>>();
        let t = squad_orders_target_at(&mut p.g, [c[0], 0.0, c[2]]);
        assert_eq!(squad_orders_give(&mut p.g, "openClear", Some(&t), Some(Who::All), None), None, "seed {s}: the order to open and clear the locked door was refused");
        let out = |p: &Play, m: GuardId| p.g.gd[m].slung.as_ref().map_or(false, |x| x.out);
        // (an entry halts at a door it finds shut, for Sam's word: the row given again, as the lead's "your call" asks)
        let mut k = 0;
        let up = p.until(45.0, |p| { k += 1; if k % 60 == 0 && p.g.door_tools.ops.is_empty() { let _ = squad_orders_give(&mut p.g, "openClear", Some(&t), Some(Who::All), None); } p.mates().into_iter().any(|m| out(p, m)) });
        assert!(up.is_some(), "seed {s}: nobody brought the shotgun up for the lock");
        squad_orders_clear(&mut p.g, None);
        p.frame();
        p.frame();
        assert!(p.g.door_tools.ops.is_empty(), "seed {s}: the door's work goes on two frames after the orders were taken");
        for (m, gun) in &own { assert!(!out(&p, *m) && p.g.gd[*m].weapon == *gun, "seed {s}: {} still has the shotgun up two frames after his orders were taken", p.g.gd[*m].callsign); }
    }
}

/// THE RULE: a man who had a living enemy in his eyes a moment ago is dealt no opening to watch until that is four seconds old; his mates keep theirs.
/// A FAILURE MEANS: the squad turns a man off the corridor he just saw an enemy in to a door behind him.
#[test]
fn a_man_with_a_fresh_sighting_is_dealt_no_opening() {
    for s in SEEDS {
        let mut p = office(s);
        let sam = [20.6, 17.6];
        p.sam_put([13.0, 17.6]);
        p.order("fallIn", [13.0, 17.6], Who::All);
        p.run(4.0);
        p.sam_put(sam);
        p.run(4.0);
        let Some(m) = p.mates().into_iter().find(|&m| p.g.cover.jobs.has(m)) else { panic!("seed {s}: nobody was dealt an opening") };
        let op = game::roomfight::room_fight_op_of(&mut p.g, m).expect("a squad man of a room clear has his row of the fight");
        let h = p.g.guards.iter().copied().find(|&h| p.g.gd[h].team == game::teams::Team::Hostile && p.up(h)).expect("a defender is up");
        let t = p.time();
        let secs = p.g.k.cover.contact_secs;
        let meter = p.g.tab.roomfight_op[op].meters.get_mut(h).expect("the fight keeps a row a defender");
        meter.seen_t = t - 0.5;
        p.run(0.35);
        assert!(!p.g.cover.jobs.has(m), "seed {s}: {} has a sighting 0.9 s old and is still on an opening", p.g.gd[m].callsign);
        let again = p.until(secs + 2.0, |p| p.g.cover.jobs.has(m)).expect("he is dealt one again");
        assert!(again + 0.85 >= secs - 0.3, "seed {s}: dealt an opening again {:.1} s after the sighting (the rule's {secs} s)", again + 0.85);
    }
}

/// THE RULE: four men stacked on a door are two on the door and two who keep its rear: those two are dealt it for as long as the stack stands, and
/// their guns are on what is behind it.
/// A FAILURE MEANS: the whole stack watches the door.
#[test]
fn a_stack_of_four_keeps_its_rear() {
    for (level, door) in [("office.json", "stairwell"), ("block.json", "m_s")] {
        let mut p = Play::mission(level, &format!("?levelFile={level}&roomclear&squad4"), &Opts { seed: 5, ..Default::default() });
        p.sam_unhurt();
        p.run(0.5);
        for gd in p.g.guards.clone() { if p.g.gd[gd].team == game::teams::Team::Hostile { p.g.cond[p.g.gd[gd].ch].blind_until = 1e9; } }
        let d = p.g.doors.list.iter().copied().find(|&d| p.g.dr[d].def.name == door).unwrap_or_else(|| panic!("{level} has the door {door}"));
        let c = p.g.dr[d].frame_centre;
        p.order("stack", [c[0], c[2]], Who::All);
        let (mut frames, mut behind) = (0, 0);
        for _ in 0..(14 * 60) {
            p.frame();
            let rear: Vec<GuardId> = p.mates().into_iter().filter(|&m| game::orders::squad_orders_of(&p.g, &game::seats::here(&p.g), m).map_or(false, |a| { let a = &p.g.tab.orders_assignment[a]; a.spare == Some(true) && a.from.is_some() })).collect();
            assert_eq!(rear.len(), 2, "{level} {door}: {} of the four keep the rear", rear.len());
            frames += rear.len();
            for m in rear {
                let at = p.at(m);
                if let Some(j) = p.g.cover.jobs.get(m) { if (j.to[0] - at[0]) * (c[0] - at[0]) + (j.to[2] - at[1]) * (c[2] - at[1]) < 0.0 { behind += 1; } }
            }
        }
        assert!(behind * 2 > frames, "{level} {door}: the rear men's guns are behind the stack on {behind} of {frames} of their frames");
    }
}
