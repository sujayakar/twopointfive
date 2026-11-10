//! WHAT THE HUD MARKS AND DRAWS OF THE AIM (rust/game_rules/src/tick/view/marks.rs): which men the fog hides from a seat's team, what stands over each man, the last sightings, the grenades and
//! the cameras, the meters, the ring and the arc. The HUD reads these and works nothing out; a test here holds what it is told to what the game says.
use game::game::{GuardState, SlotKind, ThrowKind};
use game::view::{HeadTone, RingOn, TickView};
use game_start::play::{dist, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];

/// two rooms with a wall and a shut door between: Sam in the west one, a sentry in the east one
fn two_rooms(seed: u32) -> (Play, game::ids::GuardId) {
    let mut s = Scene::new("marks");
    s.room("west", 6.0, 8.0, 16.0, 18.0).room("east", 16.0, 8.0, 26.0, 18.0);
    s.door("between", Axis::Z, 16.0, 13.0);
    s.bulb(11.0, 13.0).bulb(21.0, 13.0);
    s.sentry("post", [21.0, 13.0], [10.0, 13.0]).sam(9.0, 13.0);
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    let gd = p.guard("post");
    (p, gd)
}

fn mark_of(v: &TickView, man: f64) -> &game::view::ManMark { v.marks.men.iter().find(|m| m.man == man).unwrap_or_else(|| panic!("man {} is not listed: {:?}", man, v.marks.men)) }

/// the fog on, as a click on a row of the list raises it
fn fog_up(p: &mut Play) { p.g.fog_of_war.on = true; p.run(0.5); }

/// A FAILURE MEANS: the HUD marks a man the team cannot see, or hides one it can: the fog is told to a machine by this list and no other. CONTROL: with the fog down nobody is hidden.
#[test]
fn the_fog_hides_from_the_team_the_men_it_cannot_see() {
    for seed in SEEDS {
        let (mut p, gd) = two_rooms(seed);
        p.run(0.5);
        let man = p.id_of(gd);
        assert!(!mark_of(&p.view(), man).hidden, "seed {}: the fog is down and a man is hidden", seed);
        fog_up(&mut p);
        assert!(mark_of(&p.view(), man).hidden, "seed {}: a man behind a wall and a shut door is shown", seed);
        // in the room with Sam, in his line: the fog leaves him
        p.man_stands(gd, [12.0, 13.0]);
        p.run(0.3);
        assert!(!mark_of(&p.view(), man).hidden, "seed {}: a man three metres from Sam, in his line, is hidden", seed);
    }
}

/// A FAILURE MEANS: the '?' where a man was last seen goes missing, stands somewhere else, or stays. CONTROL: with neither the row nor the lever giving it any seconds, no mark.
#[test]
fn a_man_who_goes_out_of_sight_leaves_a_mark_where_he_was_last_seen() {
    for seed in SEEDS {
        let (mut p, gd) = two_rooms(seed);
        p.g.k.fog.last_seen = 2.0;
        fog_up(&mut p);
        let at = [12.0, 13.0];
        p.man_stands(gd, at);
        p.run(0.3);
        assert!(p.view().marks.seen.is_empty(), "seed {}: nobody is out of sight, and a mark stands", seed);
        p.man_stands(gd, [22.0, 13.0]);
        p.g.ai_enabled = false;
        p.until(2.0, |p| !p.view().marks.seen.is_empty()).unwrap_or_else(|| panic!("seed {}: he went out of sight and no mark stands", seed));
        let m = p.view().marks.seen[0];
        assert!(m.man == p.id_of(gd) && dist([m.at[0], m.at[2]], at) < 1.0 && m.life == 2.0, "seed {}: {:?}", seed, m);
        // at 0 the 'you're told' lever decides: its seconds by the rung; a man the wand's lens had keeps the wand's own
        p.g.k.fog.last_seen = 0.0;
        p.g.k.told.rung = game::told::ToldRung::Guided;
        assert_eq!(p.view().marks.seen[0].life, p.g.k.told.guided, "seed {}", seed);
        p.g.k.told.rung = game::told::ToldRung::Standard;
        assert_eq!(p.view().marks.seen[0].life, p.g.k.told.standard, "seed {}", seed);
        let t = p.g.fog_of_war.recs.get(gd).unwrap().t;
        p.g.fog_of_war.recs.get_mut(gd).unwrap().wand_t = Some(t);
        assert_eq!(p.view().marks.seen[0].life, p.g.k.told.standard.max(p.g.k.wand.mark_secs), "seed {}", seed);
        p.g.fog_of_war.recs.get_mut(gd).unwrap().wand_t = None;
        p.g.k.fog.last_seen = 2.0;
        p.run(2.5);
        assert!(p.view().marks.seen.is_empty(), "seed {}: the mark has outlived its seconds", seed);
        drop(p);
        let (mut q, gd) = two_rooms(seed);
        q.g.k.fog.last_seen = 0.0; q.g.k.told.rung = game::told::ToldRung::Bare;
        fog_up(&mut q);
        q.man_stands(gd, [12.0, 13.0]);
        q.run(0.3);
        q.man_stands(gd, [22.0, 13.0]);
        q.run(1.0);
        assert!(q.view().marks.seen.is_empty(), "seed {}: no seconds, and a mark stands", seed);
    }
}

/// A FAILURE MEANS: what stands over a man is no longer what he is: a wary mark over one of your own (it says nothing you can act on), or a defender without his.
#[test]
fn the_mark_over_a_man_says_what_he_is() {
    for seed in SEEDS {
        let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        p.g.ai_enabled = false;
        let (foe, mate) = (p.hostiles()[0], p.mates()[0]);
        let set = |p: &mut Play, gd, st, aware| { p.g.gd[gd].state = st; p.g.gd[gd].awareness = aware; };
        let shown = |p: &Play, gd| { let v = p.view(); let m = mark_of(&v, p.id_of(gd)); (m.text.clone(), m.tone) };
        set(&mut p, foe, GuardState::Patrol, 0.6);
        assert_eq!(shown(&p, foe), ("····".into(), HeadTone::Sus), "seed {}: a defender's dots: 1 + floor(awareness × 6)", seed);
        set(&mut p, mate, GuardState::Patrol, 0.6);
        assert_eq!(shown(&p, mate), ("".into(), HeadTone::Calm), "seed {}: no dots over a man of your own", seed);
        set(&mut p, foe, GuardState::Search, 0.2);
        assert_eq!(shown(&p, foe), ("?".into(), HeadTone::Sus), "seed {}", seed);
        set(&mut p, mate, GuardState::Search, 0.2);
        assert_eq!(shown(&p, mate), ("".into(), HeadTone::Calm), "seed {}: no '?' over a man of your own", seed);
        set(&mut p, foe, GuardState::Alert, 1.0);
        assert_eq!(shown(&p, foe), ("!".into(), HeadTone::Alert), "seed {}", seed);
        p.g.tune.aim_model = true; p.g.arms[p.g.gd[foe].ch].aim.track = 1.0;
        assert_eq!(shown(&p, foe), ("◎!".into(), HeadTone::Alert), "seed {}: he has you lined up", seed);
        set(&mut p, mate, GuardState::Alert, 1.0); p.g.arms[p.g.gd[mate].ch].aim.track = 1.0;
        assert_eq!(shown(&p, mate), ("!".into(), HeadTone::Alert), "seed {}: never the '◎' of a man who has you lined up", seed);
        p.g.cond[p.g.gd[foe].ch].blind_until = p.g.time + 5.0;
        assert_eq!(shown(&p, foe).0, "✹◎!", "seed {}: a stun goes in front", seed);
        p.falls(foe, game::game::DownBy::Player);
        assert_eq!(shown(&p, foe), ("✕".into(), HeadTone::Dead), "seed {}", seed);
    }
}

/// A FAILURE MEANS: the debug labels leak into play, or are gone from the debugger. CONTROL: off, no words.
#[test]
fn the_debug_words_are_there_only_with_the_labels_on() {
    let (mut p, gd) = two_rooms(1);
    p.frame();
    let man = p.id_of(gd);
    assert_eq!(mark_of(&p.view(), man).debug, "");
    p.g.show_debug = true;
    assert!(mark_of(&p.view(), man).debug.starts_with("patrol "), "{:?}", mark_of(&p.view(), man));
}

/// A GAME OF TWO. A FAILURE MEANS: the second machine is told about the first player's eyes and ears: the white of a stun on the wrong screen, the first man's light on his meter.
#[test]
fn in_a_game_of_two_each_is_told_of_his_own_blindness_and_his_own_meters() {
    for seed in SEEDS {
        let mut p = Play::level("defuse.json", true, &Opts { seed, seats: 2, ..Default::default() });
        p.frame();
        let second = game::seats::seat(&p.g, p.g.seats[1]).player;
        let first = game::seats::seat(&p.g, p.g.seats[0]).player;
        assert!(p.view_of(0).marks.blind == 0.0 && p.view_of(1).marks.blind == 0.0, "seed {}: CONTROL: nobody is blind", seed);
        p.g.cond[p.g.pl[second].ch].blind_until = p.g.time + 10.0;
        assert_eq!((p.view_of(0).marks.blind, p.view_of(1).marks.blind), (0.0, 1.0), "seed {}: the second man is dazzled", seed);
        p.g.shown[p.g.pl[first].ch].visibility = 0.25; p.g.shown[p.g.pl[second].ch].visibility = 0.75;
        p.g.pl[first].noise = 0.5; p.g.pl[second].noise = 0.1;
        let (a, b) = (p.view_of(0).marks, p.view_of(1).marks);
        assert_eq!((a.light, a.noise, b.light, b.noise), (0.25, 0.5, 0.75, 0.1), "seed {}", seed);
    }
}

/// the fist holds a can, the cursor on a place
fn can_in_hand(p: &mut Play, slot: SlotKind, at: [f64; 2]) {
    let pl = p.g.player;
    p.g.pl[pl].slot = slot; p.g.pl[pl].holstered = false;
    match slot { SlotKind::Frag => p.g.pl[pl].frags = 3.0, SlotKind::Flash => p.g.pl[pl].flashbangs = 3.0, _ => p.g.pl[pl].canisters = 3.0 }
    p.point_at(at);
    p.run(0.3);
}

/// A FAILURE MEANS: the ring of a preview stands somewhere the can will not rest: the HUD's question and the game's own road have parted. Asked of a game that may write (the road a rule
/// takes) and of the game shared (the view's), the can's roll is one answer, to the bit. CONTROL: a can that rolls ends past its arc's first touch.
#[test]
fn the_can_comes_to_rest_where_the_view_says() {
    for seed in SEEDS {
        let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
        p.frame();
        let at = p.sam_at();
        can_in_hand(&mut p, SlotKind::Smoke, [at[0] + 3.0, at[1] + 1.0]);
        let v = p.view().aim.throw.expect("a can in the fist: an arc is up");
        let (kind, sol) = { let pl = p.g.player; (p.g.pl[pl].held_kind, p.g.pl[pl].throw_preview.clone().unwrap()) };
        let rest = v.rest.expect("a can comes to rest");
        let own = game::frag::rest_of_throw(&mut *p.g, kind, sol.points[0], sol.v0);
        assert_eq!(rest.at.map(f64::to_bits), own.at.map(f64::to_bits), "seed {}: at {:?} and {:?}", seed, rest.at, own.at);
        assert_eq!(rest.path.len(), own.path.len(), "seed {}", seed);
        assert!(v.points.len() > 1 && !v.short && v.reach.is_none(), "seed {}: smoke reaches nobody: {:?}", seed, v.reach);
        can_in_hand(&mut p, SlotKind::Frag, [at[0] + 3.0, at[1] + 1.0]);
        let r = p.view().aim.throw.and_then(|t| t.reach).expect("a frag has a reach");
        assert_eq!((r.kill, r.hurt), (p.g.k.frag.lethal_m, p.g.k.frag.lethal_m.max(p.g.k.frag.wound_m)), "seed {}", seed);
        can_in_hand(&mut p, SlotKind::Flash, [at[0] + 3.0, at[1] + 1.0]);
        let r = p.view().aim.throw.and_then(|t| t.reach).expect("a flash has a reach");
        assert!(r.stun > 0.0 && r.kill == 0.0, "seed {}: {:?}", seed, r);
    }
}

/// A FAILURE MEANS: the 40 mm's arc says the wrong thing: rings round where a dud would land, or no rings and no word on the arc that would burst. CONTROL: on an armed arc no word stands.
#[test]
fn the_launchers_arc_is_told_whether_it_arms() {
    for seed in SEEDS {
        let mut s = Scene::new("range");
        s.room("range", 6.0, 8.0, 46.0, 28.0).room("back", 46.0, 8.0, 52.0, 28.0);
        s.door("end", Axis::Z, 46.0, 18.0);
        s.bulb(16.0, 18.0).bulb(32.0, 18.0).sam(9.0, 18.0);
        let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
        p.sam_unhurt();
        p.frame();
        p.sam_takes_his_rifle();
        let (pl, at) = (p.g.player, p.sam_at());
        let r = p.g.pl[pl].rifle;
        p.g.fa[r].attachments = vec![game::weapons::RifleAttachment::He40];
        p.g.fa[r].att_idx = 0;
        p.g.fa[r].shells = 3.0;
        let arc = |p: &mut Play, to: [f64; 2]| { p.point_at(to); p.press(game_start::play::Button::Right); p.run(0.3); let a = p.view().aim; p.release(game_start::play::Button::Right); a };
        let far = arc(&mut p, [at[0] + 14.0, at[1]]);
        let t = far.throw.as_ref().expect("the launcher's button held: an arc is up");
        assert!(!t.short && !far.too_close, "seed {}: {:?}", seed, t.short);
        assert_eq!(t.rest.is_some(), false, "seed {}: a launcher's round has no roll", seed);
        let reach = t.reach.expect("an armed arc has rings");
        assert_eq!((reach.kill, reach.hurt, reach.at), (p.g.k.frag.lethal_m, p.g.k.frag.lethal_m.max(p.g.k.frag.wound_m), t.land), "seed {}", seed);
        p.run(3.0);
        let near = arc(&mut p, [at[0] + 2.0, at[1]]);
        let t = near.throw.as_ref().expect("an arc is up");
        assert!(t.short && t.reach.is_none(), "seed {}: too short to arm", seed);
        assert!(near.too_close, "seed {}: the word at the reticle", seed);
    }
}

/// A FAILURE MEANS: asking with an arc up writes the game: the rest is the one question of the view that rolls a can, and the reader goes by the copy that stands.
#[test]
fn asking_with_an_arc_up_writes_nothing() {
    let run = |ask: bool| -> Vec<Vec<u32>> {
        let mut p = Play::level("office.json", false, &Opts { seed: 4, ..Default::default() });
        p.frame();
        let at = p.sam_at();
        can_in_hand(&mut p, SlotKind::Flash, [at[0] + 2.0, at[1] + 2.0]);
        (0..4).map(|_| { for _ in 0..30 { p.frame(); if ask { assert!(p.view().aim.throw.is_some()); } } p.words() }).collect()
    };
    assert_eq!(run(false), run(true), "the asked game differs");
}

/// A GAME OF TWO. A FAILURE MEANS: the second machine draws the first man's ring and the first man's arc.
#[test]
fn in_a_game_of_two_the_ring_and_the_arc_are_the_seats_own() {
    for seed in SEEDS {
        let mut p = Play::level("defuse.json", true, &Opts { seed, seats: 2, ..Default::default() });
        p.frame();
        let (first, second) = (game::seats::seat(&p.g, p.g.seats[0]).player, game::seats::seat(&p.g, p.g.seats[1]).player);
        p.g.tune.aim_model = true;
        p.g.pl[first].holstered = true; p.g.pl[second].holstered = false; p.g.pl[second].slot = SlotKind::Pistol;
        assert!(p.view_of(0).aim.ring.is_none(), "seed {}: CONTROL: his hands are empty", seed);
        let r = p.view_of(1).aim.ring.expect("the second man has a gun out");
        assert!(r.cone.is_some() && r.on == RingOn::Free, "seed {}: {:?}", seed, r);
        // the first man's arc is his: the second is told of none
        let at = p.sam_at();
        p.g.pl[first].holstered = false;
        can_in_hand(&mut p, SlotKind::Smoke, [at[0] + 3.0, at[1]]);
        assert!(p.view_of(0).aim.throw.is_some() && p.view_of(1).aim.throw.is_none(), "seed {}", seed);
        // and the second man's is told to him, whichever seat `g.seat` names
        let sol = p.g.pl[first].throw_preview.clone().unwrap();
        p.g.pl[second].throw_preview = Some(sol); p.g.pl[second].held_kind = ThrowKind::Smoke; p.g.pl[second].throw_held = true;
        assert!(p.view_of(1).aim.throw.is_some_and(|t| t.rest.is_some()), "seed {}", seed);
    }
}

/// A FAILURE MEANS: the ring is not barred when his gun cannot get a round out, or barred when it can: stood with the muzzle in a wall, and stood in the open.
#[test]
fn the_ring_is_barred_while_the_wall_has_his_gun() {
    for seed in SEEDS {
        let (mut p, _) = two_rooms(seed);
        p.g.ai_enabled = false;
        p.sam_takes_his_rifle();
        p.point_at([11.0, 13.0]);
        p.run(0.5);
        assert!(!p.view().aim.ring.expect("a gun is out").shut, "seed {}: CONTROL: in the open it is free", seed);
        p.sam_put([15.75, 11.0]);
        p.point_at([18.0, 11.0]);
        p.until(1.5, |p| p.view().aim.ring.is_some_and(|r| r.shut)).unwrap_or_else(|| panic!("seed {}: his muzzle is in the wall and the ring is not barred", seed));
    }
}

/// A FAILURE MEANS: a grenade Sam has not been cued on gets a mark, or one he has does not; the mark goes when it goes off.
#[test]
fn the_grenade_is_marked_once_he_has_been_cued_on_it() {
    for seed in SEEDS {
        let (mut p, _) = two_rooms(seed);
        p.g.ai_enabled = false;
        p.frame();
        assert!(p.view().marks.frags.is_empty(), "seed {}", seed);
        p.frag_lands([10.0, 15.0]);
        p.frame();
        assert!(p.view().marks.frags.is_empty(), "seed {}: CONTROL: nobody has cued him", seed);
        p.g.m.frag.books.as_mut().unwrap()[0].cued = true;
        let m = p.view().marks.frags[0];
        assert!(dist([m.at[0], m.at[2]], [10.0, 15.0]) < 1.5 && m.left > 0.0, "seed {}: {:?}", seed, m);
        p.run(p.frag_fuse() + 0.5);
        assert!(p.view().marks.frags.is_empty(), "seed {}: it has gone off", seed);
    }
}

/// A FAILURE MEANS: the picture's goggles and the big slot's tool are told of the wrong men. CONTROL: a defender wears nothing listed.
#[test]
fn who_wears_what_is_the_games_word() {
    for seed in SEEDS {
        let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
        p.frame();
        let worn = p.view().worn;
        for gd in p.mates() { assert_eq!(worn.iter().any(|w| w.man == p.id_of(gd) && w.goggles), game::nvg::nvg_worn(&p.g, gd), "seed {}", seed); }
        for gd in p.hostiles() { assert!(!worn.iter().any(|w| w.man == p.id_of(gd)), "seed {}: a defender is listed", seed); }
        let pl = p.g.player;
        p.g.pl[pl].big_kit = game::kit::BigKit::Hammer;
        let sam = p.g.ch[p.g.pl[pl].ch].id;
        assert!(p.view().worn.iter().any(|w| w.man == sam && w.big == Some(game::bigkit::BigCarried::Hammer)), "seed {}", seed);
    }
}

/// A FAILURE MEANS: a civilian has lost the small mark of her own, or keeps one with nobody on the floor. CONTROL: calm at work, no mark, and still listed for the fog's word.
#[test]
fn a_civilian_has_the_small_mark_of_her_own() {
    use game::civilians::CivState;
    for seed in SEEDS {
        let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
        p.frame();
        p.g.ai_enabled = false;
        let cv = *p.g.civilians.first().expect("the office has civilians");
        let (man, c) = (p.id_of(cv), p.g.gd[cv].civ.unwrap());
        p.g.gd[cv].awareness = 0.0; p.g.cv[c].state = CivState::Work;
        { let v = p.view(); let m = mark_of(&v, man); assert!(m.civ && m.text.is_empty(), "seed {}: {:?}", seed, m); }
        p.g.cv[c].state = CivState::Startled;
        assert_eq!(mark_of(&p.view(), man).text, "?", "seed {}", seed);
        p.g.cv[c].state = CivState::Phone;
        { let v = p.view(); let m = mark_of(&v, man); assert_eq!((m.text.as_str(), m.tone), ("!☎", HeadTone::Alert), "seed {}", seed); }
        p.g.cv[c].state = CivState::Gone;
        assert!(!p.view().marks.men.iter().any(|m| m.man == man), "seed {}: she has left the floor and is still listed", seed);
    }
}

/// A GAME OF TWO. A FAILURE MEANS: the camera of the man who shows this seat leans for the other man's stalk: the wait at a corner and the man stalked are the seat's own, and the wait
/// goes with the tick's word for it. CONTROL: nobody stalks, nobody waits.
#[test]
fn the_stalk_and_the_wait_are_the_seats_own() {
    for seed in SEEDS {
        let mut p = Play::level("defuse.json", true, &Opts { seed, seats: 2, ..Default::default() });
        p.frame();
        let second = game::seats::seat(&p.g, p.g.seats[1]).player;
        assert!(p.view_of(1).marks.stalk.is_none() && !p.view_of(1).marks.stalk_waits, "seed {}", seed);
        let man = p.mates()[0];
        let t = p.g.time;
        let mut row = game::stalk::fresh();
        row.wait = Some(true); row.t = t; row.on = Some(man);
        p.g.m.stalk.state.set(second, row);
        p.g.pl[second].wall = p.g.pl[second].wall.or(Some(game::player::WallHold::default()));
        let (a, b) = (p.view_of(0).marks, p.view_of(1).marks);
        assert!(a.stalk.is_none() && !a.stalk_waits, "seed {}: the first man is told of the second man's stalk", seed);
        assert!(b.stalk == Some(p.id_of(man)) && b.stalk_waits, "seed {}: {:?} {}", seed, b.stalk, b.stalk_waits);
        p.g.m.stalk.state.get_mut(second).unwrap().t = t - 0.5;
        assert!(!p.view_of(1).marks.stalk_waits, "seed {}: the word is old", seed);
    }
}
