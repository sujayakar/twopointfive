//! WHAT A SEAT'S HUD IS TOLD OF ITS OWN KEY AND OF THE FLOOR (`view.interact`, `alarm`, `stage`, `look`; rust/game_rules/src/interact/hud.rs): the overlay draws these and asks nothing of a game.
use game::game::{Interactable, InteractableKind as K};
use game::view::{BackKey, ListRow, Stage, ThingKind, TickView};
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
const NORTH: f64 = std::f64::consts::PI;

fn yard(seed: u32) -> (Scene, [f64; 2]) {
    let x = 13.4 + nudge(seed, 0);
    let mut s = Scene::new("hud-yard");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("west-yard", Axis::X, 16.0, x).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0).moon();
    s.team([x, 22.0], NORTH, &[[x - 1.0, 23.0], [x + 1.0, 23.0], [x - 1.0, 24.0], [x + 1.0, 24.0]]);
    s.defender("west", [11.25, 11.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
    (s, [x + 0.6, 16.0])
}
fn play(seed: u32) -> (Play, [f64; 2]) {
    let (s, door) = yard(seed);
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.2);
    (p, door)
}
/// Sam a stride from the door, and the cursor on it
fn at_door(p: &mut Play, door: [f64; 2]) {
    p.sam_put([door[0] - 0.2, door[1] + 1.0]);
    p.point_at(door);
    p.run(0.2);
}
fn labels(rows: &[ListRow]) -> Vec<String> { rows.iter().map(|r| r.label.clone()).collect() }

/// A FAILURE MEANS: a line of a row is parted otherwise than as it was authored: a hint read as a key, a key's other way lost.
#[test]
fn a_rows_second_line_is_its_keys_and_what_no_key_gives() {
    let read = |s: &str| game::interact::row_opts(s).into_iter().map(|o| (o.key, o.text)).collect::<Vec<_>>();
    let pair = |k: &str, t: &str| (String::from(k), String::from(t));
    assert_eq!(read("Space  push it shut  ·  or sprint into it"), [pair("Space", "push it shut"), pair("", "or sprint into it")]);
    assert_eq!(read("F or sprint into it  kick it in"), [pair("F", "kick it in"), pair("", "or sprint into it")]);
    assert_eq!(read("LMB  knock him out  ·  RMB  grab him"), [pair("LMB", "knock him out"), pair("RMB", "grab him")]);
    assert_eq!(read("picking… 40%  ·  Space or a step breaks off"), [pair("", "picking… 40%"), pair("", "Space or a step breaks off")]);
    assert_eq!(read("looking under it — the cursor turns the lens"), [pair("", "looking under it — the cursor turns the lens")]);
    assert_eq!(read(""), []);
}

/// THE RULE: the panel's rows are the rows the tick built, and the lit one is the highlight the key acts on. A FAILURE MEANS: the panel says something his key would not do. The control is open floor, where there is nothing.
#[test]
fn the_list_is_the_rows_the_tick_built_and_lights_what_the_key_does() {
    for seed in SEEDS {
        let (mut p, door) = play(seed);
        let I = p.view().interact;
        assert!(I.rows.is_empty() && I.lit == -1.0 && I.ring.is_none() && !I.open, "seed {seed}: nothing in reach on the yard: {I:?}");
        at_door(&mut p, door);
        let I = p.view().interact;
        let hv = p.g.hover.clone().expect("a door a stride away is the highlight");
        assert!(I.corner, "seed {seed}: no tour's puppet");
        assert_eq!(I.rows[0].label, hv.line1, "seed {seed}");
        assert_eq!(I.lit, 0.0, "seed {seed}: what the key would do is the line that is lit: {I:?}");
        assert_eq!(I.rows[0].kind, ThingKind::Door, "seed {seed}");
        assert!(I.rows[0].in_reach && I.rows[0].dist.is_none() && !I.rows[0].opts.is_empty() && !I.rows[0].opts[0].key.is_empty(), "seed {seed}: {:?}", I.rows[0]);
        let R = I.ring.expect("the ring is on the door");
        assert_eq!((R.kind, R.at), (ThingKind::Door, hv.pos), "seed {seed}");
        // the whole list, as a tour's puppet keeps it: every row in the tick's order, and the highlight moves with him
        p.g.puppet = Some(Default::default());
        let n = p.g.nearby.len();
        let I = p.view().interact;
        assert!(!I.corner, "seed {seed}");
        assert_eq!(labels(&I.rows), p.g.nearby.iter().map(|r| r.line1.clone()).collect::<Vec<_>>(), "seed {seed}");
        assert!(I.rows.iter().skip(1).all(|r| r.cont == (r.label == I.rows[0].label)), "seed {seed}: rows of one thing are one heading: {:?}", I.rows);
        if n > 1 {
            let pl = p.g.player; p.g.pl[pl].near_sel = 1; game::player::pick_hover(&mut p.g);
            assert_eq!(p.view().interact.lit, 1.0, "seed {seed}: the highlight is the tick's");
        }
        p.g.puppet = None;
    }
}

/// A FAILURE MEANS: the corner lights a line no key gives, or keeps a row out of reach without saying how far. The control is the same row in reach.
#[test]
fn a_row_out_of_reach_says_how_far_and_the_ring_stands_down_on_a_row_about_no_thing() {
    for seed in SEEDS {
        let (mut p, door) = play(seed);
        at_door(&mut p, door);
        let near = p.view().interact.rows[0].clone();
        let hv = p.g.hover.clone().unwrap();
        assert!(near.in_reach && near.dist.is_none(), "seed {seed}");
        let Some(row) = p.g.nearby.first_mut() else { panic!("seed {seed}: a row") };
        row.in_reach = false;
        let far = p.view().interact.rows[0].clone();
        assert!(!far.in_reach, "seed {seed}");
        let d = far.dist.expect("how far off a row out of reach is");
        assert!(d > 0.0 && d < 3.0, "seed {seed}: a stride from the door: {d}");
        p.g.hover = Some(game::game::Interactable { kind: game::game::InteractableKind::Charge, ..hv });
        assert!(p.view().interact.ring.is_none(), "seed {seed}: a charge to blow is on his list wherever he stands: the ring stays the gun's");
    }
}

/// THE RULE: a dot for every thing the key could act on but a living man, who has his own mark, as the tick has them; one ringed by the highlight and one hidden by the fog. A FAILURE MEANS: a dot over nothing, or a dot over a man the fog hides.
#[test]
fn the_dots_are_the_ticks_things_and_the_highlight_rings_one() {
    for seed in SEEDS {
        let (mut p, door) = play(seed);
        at_door(&mut p, door);
        let v = p.view();
        let (dots, things) = (&v.interact.dots, p.g.interactables.iter().filter(|t| t.kind != K::Guard).collect::<Vec<_>>());
        assert_eq!(dots.len(), things.len(), "seed {seed}");
        assert!(dots.iter().zip(things.iter()).all(|(d, t)| d.at == t.pos && d.off == t.off && d.kind == game::interact::thing_kind(t.kind)), "seed {seed}");
        assert!(dots.iter().all(|d| d.kind != ThingKind::Guard), "seed {seed}: a living man has his mark, and no dot is sent of him");
        assert_eq!(dots.iter().filter(|d| d.kind == ThingKind::Door).count(), 3, "seed {seed}: the three doors: {dots:?}");
        assert_eq!(dots.iter().filter(|d| d.ringed).count(), 1, "seed {seed}: the door the ring is on, and no other: {dots:?}");
        let ringed = dots.iter().find(|d| d.ringed).unwrap();
        assert_eq!((ringed.kind, ringed.at), (ThingKind::Door, p.g.hover.as_ref().unwrap().pos), "seed {seed}");
        // CONTROL: the highlight gone, no dot is ringed
        let hv = p.g.hover.take().unwrap();
        assert!(p.view().interact.dots.iter().all(|d| !d.ringed), "seed {seed}");
        // his own charge on that door, its row lit: the ring stays the gun's, so the door's dot stands
        p.g.hover = Some(Interactable { kind: K::Charge, ..hv });
        let I = p.view().interact;
        assert!(I.ring.is_none() && I.dots.iter().all(|d| !d.ringed), "seed {seed}: {I:?}");
    }
}

/// THE RULE: of the things about men, exactly the one whose man the fog hides is hidden. A FAILURE MEANS: a body's dot shows a man the team cannot see, or the fog takes another's. The control is the same dots with nobody hidden.
#[test]
fn the_fog_hides_exactly_the_dot_of_the_man_it_hides() {
    for seed in SEEDS {
        let (mut p, _) = play(seed);
        let hs = p.hostiles();
        let (a, b) = (hs[0], hs[1]);
        p.falls(a, game::game::DownBy::Player); p.falls(b, game::game::DownBy::Player);
        p.run(0.2);
        let of = |p: &Play, gd| p.g.interactables.iter().find(|t| t.kind == K::Body && t.guard == Some(gd)).map(|t| t.pos);
        let (at_a, at_b) = (of(&p, a).expect("the first body has a row"), of(&p, b).expect("the second"));
        p.g.fog_of_war.hidden_chars.clear();
        assert!(p.view().interact.dots.iter().all(|d| !d.hidden), "seed {seed}: nobody is hidden");
        let c = p.g.gd[a].ch; p.g.fog_of_war.hidden_chars.add(c);
        let dots = p.view().interact.dots;
        let hid: Vec<_> = dots.iter().filter(|d| d.hidden).collect();
        assert_eq!(hid.len(), 1, "seed {seed}: {dots:?}");
        assert_eq!((hid[0].kind, hid[0].at), (ThingKind::Body, at_a), "seed {seed}");
        assert!(dots.iter().any(|d| d.at == at_b && !d.hidden), "seed {seed}: the other stands");
    }
}

/// THE RULE: a thing on a man rides his figure: it names him and how far over his feet; a body stays where the tick has it. A FAILURE MEANS: a guard's ring lags the man drawn between ticks, or a body's dot follows his standing record.
#[test]
fn a_thing_on_a_man_names_him_and_a_body_stays_at_its_place() {
    for seed in SEEDS {
        let (mut p, _) = play(seed);
        let gd = p.hostiles()[0];
        let feet = p.g.ch[p.g.gd[gd].ch].pos;
        let on = |kind| Interactable { kind, guard: Some(gd), pos: [feet[0], feet[1] + 1.35, feet[2]], ..Interactable::zeroed() };
        p.g.hover = Some(on(K::Guard));
        let R = p.view().interact.ring.expect("a ring on him");
        assert_eq!(R.man, Some(p.id_of(gd)), "seed {seed}");
        assert!((R.up - 1.35).abs() < 1e-9, "seed {seed}: {R:?}");
        p.g.hover = Some(on(K::Body));
        let R = p.view().interact.ring.expect("a ring on the body");
        assert!(R.man.is_none() && R.at[1] == feet[1] + 1.35, "seed {seed}: {R:?}");
        p.g.interactables = vec![on(K::Held), on(K::Body), on(K::Guard)];
        let D = p.view().interact.dots;
        assert_eq!(D.len(), 2, "seed {seed}: no dot of a living man");
        assert!(D[0].man == Some(p.id_of(gd)) && (D[0].up - 1.35).abs() < 1e-9 && D[1].man.is_none(), "seed {seed}: {D:?}");
    }
}

/// A FAILURE MEANS: the HUD reads another alarm than the one the guards are under. The control is calm, and a show, where the alarm is nothing.
#[test]
fn the_alarm_is_the_floors_with_its_clock() {
    for seed in SEEDS {
        let (mut p, _) = play(seed);
        assert!(p.view().alarm.is_none(), "seed {seed}: calm");
        game::guards::raise_escalation(&mut p.g, 2.0, None, Some(game::guards::RaiseEscalationCause::Search), None);
        let a = p.view().alarm.expect("raised");
        assert_eq!(a.level, p.g.escalation, "seed {seed}");
        assert!((a.left - (p.g.escalation_t - p.g.time)).abs() < 1e-9 && a.left > 0.0, "seed {seed}: {a:?}");
        assert_eq!(a.live, p.g.alarm.episode, "seed {seed}");
        p.run(0.5);
        let b = p.view().alarm.expect("still up");
        if a.level == b.level && !b.live { assert!(b.left < a.left, "seed {seed}: its clock runs: {a:?} {b:?}"); }
        p.g.quiet_utility = true;
        assert!(p.view().alarm.is_none(), "seed {seed}: a show moves no alarm");
        p.g.quiet_utility = false;
    }
}

/// A FAILURE MEANS: a seat is told the other's list. Two players stand, one at a door and one on open floor, each seat asked; then the other way round.
#[test]
fn each_seat_is_told_its_own_list() {
    for seed in SEEDS {
        let (s, door) = yard(seed);
        let mut p = Play::scene(&s, &Opts { seed, seats: 2, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let open = [door[0], 26.0];
        let list = |v: TickView| v.interact.rows.iter().filter(|r| r.kind == ThingKind::Door).count();
        p.seat_stands(0, open);
        p.seat_stands(1, [door[0] - 0.2, door[1] + 1.0]);
        p.run(0.3);
        assert_eq!(list(p.view_of(0)), 0, "seed {seed}: the first player is on open floor");
        assert!(list(p.view_of(1)) > 0, "seed {seed}: the second player is at the door: {:?}", p.view_of(1).interact);
        assert!(p.view_of(1).interact.ring.is_some() && p.view_of(0).interact.ring.is_none(), "seed {seed}");
        p.seat_stands(1, open);
        p.seat_stands(0, [door[0] - 0.2, door[1] + 1.0]);
        p.run(0.3);
        assert!(list(p.view_of(0)) > 0 && list(p.view_of(1)) == 0, "seed {seed}: and the other way round");
    }
}

/// A FAILURE MEANS: the HUD is told a number of a table other than the one the tick reads. Each is read against the table and then turned and read back; the control is a row not turned.
#[test]
fn the_look_is_the_tables_as_they_stand() {
    for seed in SEEDS {
        let (mut p, _) = play(seed);
        let (L, k) = (p.view().look, &p.g.k);
        assert_eq!(L.ring.places, k.cmdring.wedges, "seed {seed}");
        assert_eq!((L.ring.hole, L.ring.rim), (game::ringlone::RING_HOLE, game::ringlone::RING_RIM), "seed {seed}");
        let (b, l) = (game::ringlone::ring_box(), game::ringlone::lone());
        assert_eq!((L.ring.box_x, L.ring.box_y), (b.x, b.y), "seed {seed}");
        let P = &L.ring.lone;
        assert_eq!([P.r, P.h, P.pad, P.letter, P.max, P.lead], [l.r, l.h, l.pad, l.char, l.max, l.lead], "seed {seed}: the drawing follows the numbers the tick hit-tests");
        let Q = &L.plan;
        assert_eq!([Q.fresh_secs, Q.walk_dim, Q.rest_dim, Q.ring_m, Q.sector_m, Q.sector_deg, Q.reask_secs, Q.others],
            [k.plan.fresh_secs, k.plan.walk_dim, k.plan.rest_dim, k.plan.ring_m, k.plan.sector_m, k.plan.sector_deg, k.plan.reask_secs, k.plan.others], "seed {seed}");
        assert_eq!((L.net.edge, L.net.near, L.net.edge_in), (k.squadnet.edge, k.squadnet.near, k.squadnet.edge_in), "seed {seed}");
        assert_eq!((L.attention.ring_m, L.attention.way_reask), (k.attention.ring_m, k.attention.way_reask), "seed {seed}");
        assert_eq!((L.line_clear, L.mark_clear), (k.rails.line_clear, k.rails.mark_clear), "seed {seed}");
        let was = L;
        p.g.k.plan.ring_m = 0.7; p.g.k.plan.others = 0.0; p.g.k.squadnet.edge = !p.g.k.squadnet.edge; p.g.k.rails.mark_clear = !p.g.k.rails.mark_clear; p.g.k.cmdring.wedges = 6.0;
        let look = p.view().look;
        assert_eq!((look.plan.ring_m, look.plan.others), (0.7, 0.0), "seed {seed}");
        assert_eq!(look.net.edge, !was.net.edge, "seed {seed}");
        assert_eq!(look.mark_clear, !was.mark_clear, "seed {seed}");
        assert_eq!(look.ring.places, 6.0, "seed {seed}");
        assert_eq!(look.plan.sector_m, was.plan.sector_m, "seed {seed}: CONTROL");
        p.g.k.backkey.tab = true; assert_eq!(p.view().look.back, BackKey::Tab, "seed {seed}");
        p.g.k.backkey.tab = false; assert_eq!(p.view().look.back, BackKey::Esc, "seed {seed}");
    }
}

/// A FAILURE MEANS: a word the HUD sets is not the game's own: the stage as the mission has it, a man's card as he was staged with it, the floor as the hearing model has it.
#[test]
fn the_stage_a_mans_card_and_the_floor_under_him_are_the_games() {
    for seed in SEEDS {
        let (mut p, _) = play(seed);
        let v = p.view();
        assert_eq!(v.stage, Stage::Infiltrate, "seed {seed}");
        assert_eq!(p.g.mission.stage.as_str(), "infiltrate", "seed {seed}");
        // each man's line is the card he was staged with, the first he carries told from the last
        let mates = v.squad.clone().expect("a squad").mates;
        assert_eq!(mates.len(), 4, "seed {seed}");
        let by = p.mates();
        for m in &mates {
            let gd = *by.iter().find(|&&gd| p.id_of(gd) == m.man).unwrap();
            let c = game::kit::kit_of(&p.g, gd).expect("staged");
            assert!(m.kit.contains(&format!("{} stun, {} charge", c.bangs, c.charges)) && m.kit.contains(if c.nvg { "goggles" } else { "no goggles" }), "seed {seed}: {} for {c:?}", m.kit);
        }
        assert!(mates.iter().any(|m| m.kit.contains("shotgun")) && mates.iter().any(|m| m.kit.contains("hammer")), "seed {seed}: the cards differ man to man: {mates:?}");
        // a game read in from the wire says them again
        let by0 = by[0];
        let before = game::kit::kit_line(&p.g, by0).to_string();
        for gd in p.mates() { let mut e = p.g.m.kit.man_of.get(gd).cloned().unwrap(); e.words.clear(); p.g.m.kit.man_of.set(gd, e); }
        assert!(game::kit::kit_line(&p.g, by0).is_empty(), "seed {seed}");
        game::kit::kit_words_again(&mut p.g);
        assert_eq!(game::kit::kit_line(&p.g, by0), before, "seed {seed}");
        let at = p.sam_at();
        let word = v.marks.floor.expect("the floors differ");
        assert_eq!(format!("{word:?}").to_lowercase(), game::floor::floor_of(&p.g, [at[0], 0.0, at[1]]).as_str(), "seed {seed}");
        p.g.tune.floor_materials = false;
        assert_eq!(p.view().marks.floor, None, "seed {seed}: the floors all one: nothing is said of them");
    }
}

/// A FAILURE MEANS: asking what a HUD is told moved the game, with a list up, a highlight and an alarm running, in both seats. The whole run is held against the same run not asked.
#[test]
fn asking_writes_nothing() {
    for seed in SEEDS {
        let run = |ask: bool| -> Vec<Vec<u32>> {
            let (s, door) = yard(seed);
            let mut p = Play::scene(&s, &Opts { seed, seats: 2, squad: 2, ..Default::default() });
            p.sam_unhurt();
            p.seat_stands(1, [door[0] - 0.2, door[1] + 1.0]);
            let mut out = Vec::new();
            for k in 0..8 {
                if k == 3 { game::guards::raise_escalation(&mut p.g, 1.0, None, Some(game::guards::RaiseEscalationCause::Search), None); }
                for _ in 0..30 { p.frame(); if ask { assert!(p.view_of(0).stage == p.view_of(1).stage); } }
                out.push(p.words());
            }
            out
        };
        let (never, always) = (run(false), run(true));
        for (k, (a, b)) in never.iter().zip(always.iter()).enumerate() {
            let parts: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
            assert!(parts.is_empty(), "seed {seed}: at {k} the asked game differs in parts {parts:?}");
        }
    }
}

/// THE RULE: a held attention is told as the floor shows it: the place, its clock, whether the thread is his own way, the floor's or a straight line, and whether the fog hides him. A FAILURE MEANS: a thread is drawn
/// through a wall he will walk round, or over a man the team cannot see. The control is a man whose attention is his own: he is not listed.
#[test]
fn a_held_attention_is_told_with_its_way_and_its_fog() {
    use game::attention::{DrawKind, Hold, HoldMode};
    for seed in SEEDS {
        let (mut p, _) = play(seed);
        p.g.k.attention.on = true; p.g.k.attention.marks = true; p.g.k.attention.way = true;
        let gd = p.hostiles()[0];
        assert!(p.view().attention.is_empty(), "seed {seed}: nobody's attention is taken yet");
        let at = { let c = p.at(gd); [c[0] + 3.0, 0.0, c[1]] };
        let hold = |mode: HoldMode, until: f64, t: f64| Hold { at, kind: DrawKind::Noise, mode, t0: t, secs: 5.0, until, full: 5.0, glimpse: 0.0, told: false, nth: 1, ears: 1.0, noises: 1.0, yaw: None, serial: 1 };
        // on his way over, on his own path: the rest of that path
        let t = p.g.time;
        game::attention::mem(&mut p.g, gd).hold = Some(hold(HoldMode::Go, -1.0, t));
        let path: Vec<[f64; 3]> = vec![[at[0] - 2.0, 0.0, at[2]], [at[0] - 1.0, 0.0, at[2] + 1.0], [at[0], 0.0, at[2]]];
        p.g.gd[gd].path = path.clone(); p.g.gd[gd].path_i = 1; p.g.gd[gd].path_goal = Some(at);
        let L = p.view().attention.into_iter().next().expect("the man whose attention is taken");
        assert_eq!(L.man, p.id_of(gd), "seed {seed}");
        assert_eq!((L.at[0], L.at[2]), (at[0], at[2]), "seed {seed}");
        assert!(L.walking && L.walks && !L.ending && !L.worn && L.left == 1.0 && L.mode == game::view::AttentionMode::Go, "seed {seed}: {L:?}");
        assert_eq!(L.own.as_deref(), Some(&path[1..]), "seed {seed}: the rest of his own path");
        // a path to somewhere else is not his way to this: the floor is asked
        p.g.gd[gd].path_goal = Some([at[0] + 9.0, 0.0, at[2]]);
        let L = p.view().attention.into_iter().next().unwrap();
        assert!(L.walks && L.own.is_none(), "seed {seed}: {L:?}");
        // the rule off: straight
        p.g.k.attention.way = false;
        assert!(!p.view().attention[0].walks, "seed {seed}");
        p.g.k.attention.way = true;
        // at the thing the ring runs down, and he says when he is done
        let t = p.g.time;
        game::attention::mem(&mut p.g, gd).hold = Some(hold(HoldMode::Go, t + 2.5, t));
        let L = p.view().attention.into_iter().next().unwrap();
        assert!(!L.walking && !L.walks && (L.left - 0.5).abs() < 1e-9 && L.ending == (2.5 <= p.g.k.attention.tell_secs), "seed {seed}: {L:?}");
        game::attention::mem(&mut p.g, gd).hold = Some(hold(HoldMode::Look, t + 0.1, t));
        assert!(p.view().attention[0].ending, "seed {seed}: the last beat");
        // the fog
        assert!(!p.view().attention[0].hidden, "seed {seed}");
        let c = p.g.gd[gd].ch; p.g.fog_of_war.hidden_chars.add(c);
        assert!(p.view().attention[0].hidden, "seed {seed}: the fog hides him");
        // the marks off, and under a show: nothing is drawn of it
        p.g.k.attention.marks = false; assert!(p.view().attention.is_empty(), "seed {seed}");
        p.g.k.attention.marks = true; p.g.touring = true; assert!(p.view().attention.is_empty(), "seed {seed}");
        p.g.touring = false;
    }
}
