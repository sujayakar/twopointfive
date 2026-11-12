//! A ROUND GOES THROUGH WHAT IT SAYS IT GOES THROUGH AND STOPS IN THE REST. A SAVE CARRIES THE BROKEN LEVEL.
mod world_ask;
use game::combat::RoundMet;
use game::out_events::OutEvent;
use game::particle_fx::surf_at;
use game::scene_collision::{nav_grid_is_blocked, segment_blocked};
use game::scene_materials::Surf;
use game::smoke_types::SmokeEmitterKind;
use game_start::play::{Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box_of, scene_with, FurnAsk};

/// Two rooms, a plasterboard partition at x 18 with a shut wooden door in it, Sam in the west with his rifle, a man held just beyond; `more`: what else stands.
fn rooms(s: u32, man: [f64; 2], more: Vec<game_start::json::Json>) -> (Play, game::ids::GuardId) {
    let mut sc = Scene::new("partition");
    sc.room("west", 10.0, 8.0, 18.0, 18.0).room("east", 18.0, 8.0, 26.0, 18.0).door("yard", Axis::Z, 26.0, 15.4).door("mid", Axis::Z, 18.0, 15.4);
    sc.bulb(14.0, 13.0).sentry("beyond", man, [24.0, man[1]]).sam(15.0, man[1]);
    let mut p = scene_with(&sc, more, &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true; p.g.ai_enabled = false; p.g.k.chaos.def = 0.5;
    p.sam_takes_his_rifle();
    p.run(0.4);
    let gd = p.guard("beyond");
    (p, gd)
}
fn fire_once(p: &mut Play, at: [f64; 2]) { p.cursor_on([at[0], 1.2, at[1]]); p.frame(); p.press(Button::Left); p.frame(); p.frame(); p.release(Button::Left); p.run(0.16); }

/// THE RULE: a round through a plasterboard partition, or a shut wooden leaf, goes on and finds the man beyond. CONTROL: concrete in the same line stops it: the same rounds, and he stands.
/// A FAILURE MEANS: a man behind drywall is safe from rifle fire, or a man behind concrete is not.
#[test]
fn a_round_goes_through_what_it_says_it_goes_through_and_stops_in_the_rest() {
    for s in [1u32, 2] {
        for (what, man) in [("the partition", [19.0, 12.0]), ("the door", [19.0, 15.8])] {
            let (mut p, gd) = rooms(s, man, vec![]);
            let c0 = p.g.stats.wallpen.crossed;
            for _ in 0..8 { if p.up(gd) { fire_once(&mut p, man); } }
            assert!(!p.up(gd), "seed {s}: eight rounds through {what} and he stands: crossed {}", p.g.stats.wallpen.crossed - c0);
            assert!(p.g.stats.wallpen.crossed > c0, "seed {s}: down, and no round crossed {what}");
        }
        let (mut p, gd) = rooms(s, [19.0, 12.0], vec![a_box_of("hard", [18.4, 1.2, 12.0], [0.1, 1.2, 1.5], "white", "concrete")]);
        for _ in 0..8 { fire_once(&mut p, [19.0, 12.0]); }
        assert!(p.up(gd), "seed {s}: a man behind concrete went down");
    }
}

fn shoot_until(p: &mut Play, at: [f64; 2], limit: usize, done: &dyn Fn(&Play) -> bool) -> usize {
    let w = p.g.pl[p.g.player].rifle;
    let n0 = p.g.fa[w].shots_fired;
    p.cursor_on([at[0], 1.2, at[1]]);
    p.run(0.6);   // (his aim comes to it)
    for _ in 0..limit {
        if done(p) { break; }
        (p.g.fa[w].chamber, p.g.fa[w].mag) = (1.0, Some(30.0));
        fire_once(p, at);
    }
    (p.g.fa[w].shots_fired - n0) as usize
}

/// THE RULE (the maintainer: "doors should splinter apart on sustained gunfire", "it should be a real gameplay tradeoff"): a shut wooden leaf hangs through the rounds the toll gives it, twice what opens a stretch of plasterboard, then it is off its hinges and stops no eye. CONTROL: the plasterboard's own count on the same rifle is the yardstick, and the door still stands at it.
/// A FAILURE MEANS: a door is a wall, or one round of spray is a breach.
#[test]
fn gunfire_splinters_a_wooden_leaf_at_its_toll() {
    let (mut p, gd) = rooms(1, [22.0, 12.0], vec![]);
    let _ = gd;
    let opened = |p: &Play| game::wallbreak::wall_break_all(&p.g).iter().any(|w| w.kind == game::wallbreak::BKind::Wall && w.open > 0.0);
    p.sam_put([15.5, 11.0]);
    let wall = shoot_until(&mut p, [18.0, 11.0], 40, &opened);
    assert!(wall > 0 && opened(&p), "the partition never opened");
    drop(p);
    let (mut p, _) = rooms(1, [22.0, 12.0], vec![]);
    let d = game::doors::doors_by_name(&p.g, "mid").expect("the door");
    let z = p.g.dr[d].frame_centre[2];
    p.sam_put([15.5, z]);
    p.frame();
    let (a, b) = ([16.0, 1.6, z], [20.0, 1.6, z]);
    assert!(segment_blocked(&mut p.g, a, b, None, None, None), "the shut leaf stops no eye");
    let toll = p.g.k.doorshot.toll as usize;
    let n = shoot_until(&mut p, [18.0, z], wall * toll - 1, &|p: &Play| p.g.dr[d].leaf.off);
    assert!(!p.g.dr[d].leaf.off, "off its hinges after {n} rounds, the yardstick is {wall}");
    let more = shoot_until(&mut p, [18.0, z], 8, &|p: &Play| p.g.dr[d].leaf.off);
    assert!(p.g.dr[d].leaf.off, "{} rounds and it hangs", n + more);
    assert_eq!(n + more, wall * toll, "rounds to take the leaf, with the yardstick at {wall}");
    p.run(0.2);
    assert!(!segment_blocked(&mut p.g, a, b, None, None, None), "off its hinges and it stops the eye");
}

/// THE RULE: what a hit looks like goes by what a thing says it is: the same paint saying steel is metal, saying plastic is plastic. CONTROL: the same paint saying nothing is neither.
/// A FAILURE MEANS: the sparks go back to reading a colour.
#[test]
fn a_hit_goes_by_what_the_thing_says_it_is() {
    let (mut p, _) = rooms(1, [22.0, 12.0], vec![a_box_of("body", [12.0, 0.5, 10.0], [0.5, 0.5, 0.5], "black", "steel"), a_box_of("tyre", [12.0, 0.5, 12.0], [0.5, 0.5, 0.5], "black", "plastic"), a_box_of("bare", [12.0, 0.5, 14.0], [0.5, 0.5, 0.5], "black", "unnamed")]);
    let of = |p: &mut Play, z: f64| { let b = p.g.col.boxes.iter().copied().find(|&b| { let c = p.g.tab.boxes[b].c; c[0] == 12.0 && c[2] == z }).expect("the box"); surf_at(&mut p.g, b) };
    assert_eq!(of(&mut p, 10.0), Surf::Metal);
    assert_eq!(of(&mut p, 12.0), Surf::Plastic);
    let bare = of(&mut p, 14.0);
    assert!(bare != Surf::Metal && bare != Surf::Plastic, "saying nothing, black paint reads {bare:?}");
}

/// THE RULE: the weather changes what nothing is to a round: rifle rounds that stop in the open ground of the compound raise the same puffs of dust under a downpour,
/// the lot soaked, as on a clear night. CONTROL: the clear night's rounds did raise dust, the downpour did soak the ground and the clear night did not.
/// A FAILURE MEANS: the rain decides a material again, and a round into wet tarmac is a round into steel: it dents, it sparks and no dust rises.
#[test]
fn a_round_into_soaked_ground_does_what_it_does_dry() {
    let dust = |weather: &str| {
        let mut p = Play::mission("compound.json", &format!("?levelFile=compound.json&weather={weather}&roomclear&squad4"), &Opts { seed: 8, ..Default::default() });
        p.g.god_mode = true; p.g.ai_enabled = false; p.g.k.chaos.def = 0.5;
        let soaked = p.until(2.0, |p| p.g.weather.wet_on).is_some();
        // the ground the whole level stands on, by its place among the statics: what a round's flight says it stopped in
        let ground = p.g.col.boxes.iter().position(|&b| { let (c, h) = (p.g.tab.boxes[b].c, p.g.tab.boxes[b].h); c[1] + h[1] == 0.0 && h[0] >= 20.0 && h[2] >= 14.0 }).expect("the ground") as f64;
        let (sam, rifle) = (p.g.pl[p.g.player].ch, p.g.pl[p.g.player].rifle);
        let def = p.g.m.weapons.weapons.get(p.g.fa[rifle].def).unwrap().clone();
        let (at, from) = (p.sam_at(), p.smoke.len());
        for _ in 0..6 {
            game::destruction::destruction_round(&mut p.g, sam, [at[0] + 1.5, 0.0, at[1]], [0.0, -1.0, 0.0], &RoundMet { n: [0.0, 1.0, 0.0], index: Some(ground), prop: None, box_: None }, &def);
            p.run(0.16);
        }
        let puffs = p.smoke[from..].iter().filter(|e| matches!(e, OutEvent::SmokeSystemLikeEmit(x) if matches!(x.e.kind, SmokeEmitterKind::Puff))).count();
        (soaked, puffs)
    };
    let (dry_soaked, dry) = dust("clear");
    let (soaked, wet) = dust("downpour");
    assert!(soaked && !dry_soaked, "the downpour soaks the ground and a clear night does not");
    assert!(dry > 0, "six rifle rounds into dry ground raise dust");
    assert_eq!(wet, dry, "puffs of dust off the soaked ground, against the dry");
}

/// THE RULE: a save after a wreck carries the walls: loaded after more damage, the same cells are open, the same pieces stand, the grid is the same, and the hole is a hole to the feet and the eye. CONTROL: the more damage did change them.
/// A FAILURE MEANS: a load brings back walls the player blew, or holes that no longer stop a round.
#[test]
fn a_save_carries_the_broken_level() {
    for s in [19u32, 20] {
        let (mut p, _) = rooms(s, [22.0, 12.0], vec![]);
        p.wall_blast([17.7, 0.3, 11.0], None);
        p.run(0.2);
        let as_saved = p.level_as_it_stands();
        let open: Vec<_> = game::wallbreak::wall_break_all(&p.g).iter().map(|w| (w.id, w.open)).collect();
        let k = p.keep();
        p.wall_blast([17.7, 0.3, 14.0], None);
        p.run(0.2);
        assert_ne!(p.level_as_it_stands(), as_saved, "seed {s}: the second wreck changed nothing");
        p.put_back(&k);
        p.frame();
        assert_eq!(p.level_as_it_stands(), as_saved, "seed {s}: the load is not the level as it was saved");
        assert_eq!(game::wallbreak::wall_break_all(&p.g).iter().map(|w| (w.id, w.open)).collect::<Vec<_>>(), open, "seed {s}: the open cells");
        assert!(!segment_blocked(&mut p.g, [16.8, 1.6, 11.0], [19.2, 1.6, 11.0], None, None, None) && !nav_grid_is_blocked(&p.g, 18.0, 11.0), "seed {s}: the hole is not a hole after the load");
    }
}

/// THE RULE (a fault once found: the first row of a two-wall blast was flooded with the second hole already in the floor, and a defender's evidence was carried onto a short door list): a blast that opens two walls is flooded once: every door a defender had evidence of, he still has it, at least as much. CONTROL: the rooms did become one, and the blast did open two walls.
/// A FAILURE MEANS: a man forgets what he knew of a door because the wall beside him fell.
#[test]
fn a_blast_that_opens_two_walls_floods_the_rooms_once() {
    use game::roomfight::room_fight_room_at as room_at;
    let (mut asked, mut bad) = (0, Vec::new());
    for level in ["compound.json", "block.json", "warehouse.json"] {
        let mut p = Play::mission(level, &format!("?levelFile={level}&roomclear=6&squad4"), &Opts { seed: 6, ..Default::default() });
        p.g.god_mode = true; p.g.k.chaos.def = 0.5;
        p.run(1.0);
        let mut place = None;
        for w in game::wallbreak::wall_break_all(&p.g) {
            if w.kind != game::wallbreak::BKind::Wall { continue; }
            let (c, h) = (p.g.tab.boxes[w.box_].c, p.g.tab.boxes[w.box_].h);
            for along in [-0.25, 0.25, 0.0, -0.4, 0.4] {
                let n = if h[0] >= h[2] { [0.0, 1.0] } else { [1.0, 0.0] };
                let mid = if h[0] >= h[2] { [c[0] + along * h[0] * 2.0, c[2]] } else { [c[0], c[2] + along * h[2] * 2.0] };
                for sign in [1.0, -1.0] {
                    let (a, b) = ([mid[0] + n[0] * sign * 1.2, 0.0, mid[1] + n[1] * sign * 1.2], [mid[0] - n[0] * sign * 1.2, 0.0, mid[1] - n[1] * sign * 1.2]);
                    let (ra, rb) = (room_at(&mut p.g, a), room_at(&mut p.g, b));
                    if ra.is_none() || rb.is_none() || ra == rb { continue; }
                    let at = [mid[0] + n[0] * sign * 0.3, 1.0, mid[1] + n[1] * sign * 0.3];
                    let plan = game::destruction::destruction_walls_for(&mut p.g, at, None);
                    if plan.iter().filter(|o| o.kind == game::wallbreak::BKind::Wall).count() < 2 { continue; }
                    let d = p.g.fight.defs.iter().copied().find(|&d| { let x = &p.g.tab.roomfight_def[d]; x.room == ra && p.up(x.gd) });
                    if let Some(d) = d { place = Some((at, a, b, d)); break; }
                }
                if place.is_some() { break; }
            }
            if place.is_some() { break; }
        }
        let Some((at, a, b, d)) = place else { continue };
        asked += 1;
        let room = p.g.tab.roomfight_def[d].room.unwrap();
        let doors: Vec<_> = p.g.tab.roomfight_room[room].entries.iter().map(|e| e.door).collect();
        p.g.tab.roomfight_def[d].ev = (0..doors.len()).map(|i| 0.1 * (i + 1) as f64).collect();
        let was: Vec<_> = doors.iter().copied().zip(p.g.tab.roomfight_def[d].ev.iter().copied()).collect();
        p.wall_blast(at, None);
        p.run(0.1);
        let (ra, rb) = (room_at(&mut p.g, a), room_at(&mut p.g, b));
        if ra != rb { bad.push(format!("{level}: the rooms are not one")); }
        let (now, ev) = { let x = &p.g.tab.roomfight_def[d]; (x.room.unwrap(), x.ev.clone()) };
        let list: Vec<_> = p.g.tab.roomfight_room[now].entries.iter().map(|e| e.door).collect();
        for (door, v) in was {
            let got = list.iter().position(|&x| x == door).and_then(|i| ev.get(i).copied());
            if !got.is_some_and(|g| g >= v - 0.05) { bad.push(format!("{level}: door {} had {v}, now {got:?}", p.g.dr[door].def.name)); }
        }
    }
    assert!(asked >= 1 && bad.is_empty(), "{asked} levels asked: {bad:#?}");
}
