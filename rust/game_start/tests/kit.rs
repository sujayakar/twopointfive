//! THE KIT, AND WHAT A PLAYER'S HANDS AND A GRENADE DO, ASKED OF THE GAME ITSELF.
//!
//! Each of these was read as a fault of the game by an old check that staged it on the page's copy: a field written by hand that the game
//! never hears of, or a function wrapped that the tick does not call. Asked here the game does what the rule says, and a failure below is
//! the game's and not a staging's.
use game::game::{DownBy, DownKind, GuardScriptOver};
use game::gunslots::{GunSlot, NO_PRIMARY};
use game::ids::GuardId;
use game::kit::{KitField, KitValue};
use game_start::play::{nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};

/// what Sam carries: sponge rounds, cameras, the wand, smoke, stun, frags, 40 mm
fn kit(p: &Play) -> (f64, f64, bool, f64, f64, f64, f64) {
    let pl = &p.g.pl[p.g.player];
    let rf = &p.g.fa[pl.rifle];
    (rf.rings, rf.cams, pl.wand, pl.canisters, pl.flashbangs, pl.frags, rf.shells)
}
/// a key pressed and let go, one frame
fn tap(p: &mut Play, code: &str) { p.key_down(code); p.frame(); p.key_up(code); }

// ---------------------------------------------------------------- a burst on a man already down: the debrief card

fn defender(p: &Play, not: Option<GuardId>) -> GuardId {
    p.g.guards.iter().copied().find(|&gd| Some(gd) != not && p.g.gd[gd].team != game::teams::Team::Operative && p.g.gd[gd].civ.is_none() && p.up(gd)).unwrap()
}
/// The border crossing as two.
fn as_two(seed: u32) -> Play {
    let mut p = Play::mission("crossing.json", "?levelFile=crossing.json&quiet&squad=1", &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.5);
    assert!(game::quiet::quiet_of(&p.g) && p.g.staged.team.len() == 1, "the staging is wrong: this is not a quiet mission as two");
    p
}
/// A defender down and breathing by `by`'s hand, on the card, come to rest. A partner's man goes on the card by the tally at the end of the step he goes
/// down in, which is asked here because the staging puts him down between two steps.
fn one_down(seed: u32, by: DownBy) -> (Play, GuardId) {
    let mut p = as_two(seed);
    let down = defender(&p, None);
    p.falls_as(down, DownKind::Struck, true, by);
    game::teamtally::team_tally_tick(&mut p.g);
    assert_eq!(p.g.mission.stats.knockouts, 1.0, "seed {seed}: the staging is wrong: his knockout is not on the card");
    p.run(2.5);
    (p, down)
}
/// a burst a little off him, by the man with that id
fn burst_beside(p: &mut Play, man: GuardId, who: f64, name: Option<&str>, seed: u32) {
    let at = p.g.ch[p.g.gd[man].ch].pos;
    let spec = game::frag::BlastSpec { who, name: name.map(String::from), fx: Some(false), ..Default::default() };
    game::frag::blast(&mut p.g, [at[0] + 0.6 + 0.2 * nudge(seed, 1), 0.05, at[2] + 0.2 * nudge(seed, 2)], &spec);
}
fn card(p: &Play) -> (f64, f64) { (p.g.mission.stats.kills, p.g.mission.stats.knockouts) }
fn timeline_has(p: &Play, kind: game::chronicle::ChronKind, words: &str) -> bool {
    p.g.chronicle.events.iter().any(|e| e.kind == kind && format!("{:?}", e.data).contains(words))
}

/// THE RULE: a burst that kills a man out cold moves him from knockout to kill, whichever of the two put him down and whichever threw it, and he is counted
/// once. The timeline names the grenade, and a partner's as a teammate's.
/// A FAILURE MEANS: the debrief card counts one man twice or not at all.
#[test]
fn a_burst_on_a_man_out_cold_moves_him_from_knockout_to_kill() {
    use game::chronicle::ChronKind::Kill;
    for (seed, by, sams, name, row) in [
        (3, DownBy::Player, false, None, "a teammate's frag grenade"),
        (4, DownBy::Guard, true, Some("40 mm grenade"), "the 40 mm grenade"),
        (5, DownBy::Player, true, None, "the frag grenade"),
        (6, DownBy::Guard, false, None, "a teammate's frag grenade"),
    ] {
        let (mut p, man) = one_down(seed, by);
        let who = if sams { p.g.ch[p.g.pl[p.g.player].ch].id } else { p.id_of(p.g.staged.team[0]) };
        burst_beside(&mut p, man, who, name, seed);
        p.run(0.5);
        assert_eq!(p.g.gd[man].down_kind, Some(DownKind::Shot), "{by:?}, thrown by Sam {sams}: the burst did not reach him");
        assert_eq!(card(&p), (1.0, 0.0), "{by:?}, thrown by Sam {sams}: kills, knockouts");
        assert!(timeline_has(&p, Kill, row), "{by:?}, thrown by Sam {sams}: no row names {row}");
        let e = p.g.chronicle.events.iter().find(|e| e.kind == Kill).unwrap();
        let said = game::chronicle::event_text(&p.g, e);
        assert!(said.ends_with(&format!("killed — {row}")), "{by:?}, thrown by Sam {sams}: the row reads {said}");
    }
}

/// THE RULE, IN ONE STEP: a man the partner puts down and bursts in the same step is one kill, by the tally at that step's end, and never a knockout.
#[test]
fn a_man_put_down_and_burst_in_one_step_is_one_kill() {
    let mut p = as_two(7);
    let man = defender(&p, None);
    let partner = p.id_of(p.g.staged.team[0]);
    p.falls_as(man, DownKind::Struck, true, DownBy::Guard);
    burst_beside(&mut p, man, partner, None, 7);
    assert_eq!(p.g.gd[man].down_kind, Some(DownKind::Shot), "the burst did not reach him");
    game::teamtally::team_tally_tick(&mut p.g);
    p.run(0.5);
    assert_eq!(card(&p), (1.0, 0.0));
}

/// THE CONTROL: a defender's own grenade kills the man and moves nothing on the card.
#[test]
fn a_defenders_grenade_on_a_man_out_cold_moves_nothing_on_the_card() {
    let (mut p, man) = one_down(3, DownBy::Player);
    let other = defender(&p, Some(man));
    let who = p.id_of(other);
    let before = card(&p);
    burst_beside(&mut p, man, who, None, 3);
    p.run(0.5);
    assert_eq!(p.g.gd[man].down_kind, Some(DownKind::Shot), "the burst did not reach him");
    assert_eq!(card(&p), before);
}

// ---------------------------------------------------------------- the suppressor row

/// THE RULE: a gun that comes only one way refuses its suppressor row, in words that name the gun and say why, and the row does not move; a gun with the
/// choice takes the step and says nothing.
/// A FAILURE MEANS: the card offers a choice the kit refuses, or refuses it without a reason.
#[test]
fn a_suppressor_row_that_cannot_move_says_why() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 1, ..Default::default() });
    let mut said = Vec::new();
    for slot in [GunSlot::Primary, GunSlot::Secondary] {
        let (pick, can) = match slot { GunSlot::Primary => (KitField::Primary, KitField::Suppressed), GunSlot::Secondary => (KitField::Secondary, KitField::SideCan) };
        for id in game::gunslots::gun_picks(&p.g, slot) {
            if id == NO_PRIMARY { continue; }
            assert!(game::kit::kit_set(&mut p.g, 1.0, pick, &KitValue::Text(id.clone())), "{id}: the card took no such gun");
            let peek = game::kit::kit_peek(&mut p.g, 1.0, can, 1.0);
            if game::kit::can_choice(&p.g, &id) {
                assert!(peek.why.is_empty(), "{id} has the choice and the row says {:?}", peek.why);
                let moved = game::kit::kit_set(&mut p.g, 1.0, can, &peek.value);
                assert!(moved, "{id} has the choice and the row would not move");
            } else {
                let name = game::gear::gear_text(&game::gunslots::pick_word(&p.g, &id, None));
                assert!(peek.why == format!("{name}: made with it") || peek.why == format!("{name}: none fits it"), "{id}: the row says {:?}", peek.why);
                let flipped = match peek.value { KitValue::Bool(b) => KitValue::Bool(!b), v => panic!("{id}: a suppressor row holds {v:?}") };
                assert!(!game::kit::kit_set(&mut p.g, 1.0, can, &flipped), "{id}: the row said no and moved");
                said.push(peek.why);
            }
        }
    }
    assert!(said.iter().any(|w| w.ends_with("made with it")) && said.iter().any(|w| w.ends_with("none fits it")), "the list has lost a kind of gun: {said:?}");
}

// ---------------------------------------------------------------- the card a run wears

/// THE RULE: a quiet mission under its fog, alone or as two, wears the stealth card: five rounds, the link's two cameras, the wand, nothing loud. A night
/// with no fog has no cameras and no wand, and its free pouches carry cans. A loud mission wears the loud card, before a quiet run and after one.
/// A FAILURE MEANS: a run has the wrong kit from its first frame.
#[test]
fn each_run_wears_its_own_card() {
    let read = |n: &str, link: &str| { let mut p = Play::mission(n, link, &Opts::default()); p.run(0.5); kit(&p) };
    let loud = read("office.json", "?levelFile=office.json&roomclear&squad4");
    assert_eq!((loud.0, loud.1, loud.2), (0.0, 0.0, false), "a loud run has no sponge rounds, no cameras and no wand");
    assert!(loud.5 > 0.0 && loud.6 > 0.0, "a loud run carries frags and 40 mm: {loud:?}");
    let stealth = (5.0, 2.0, true, 0.0, 0.0, 0.0, 0.0);
    for n in ["vault.json", "bazaar.json", "police-post.json"] {
        assert_eq!(read(n, &format!("?levelFile={n}&quiet")), stealth, "{n}, alone");
    }
    assert_eq!(read("crossing.json", "?levelFile=crossing.json&quiet&squad=1"), stealth, "as two");
    assert_eq!(read("office.json", "?levelFile=office.json&roomclear&squad4"), loud, "the next loud run is the first again");
    let mut night = Play::level("warehouse.json", false, &Opts::default());
    night.run(0.5);
    assert_eq!(kit(&night), (5.0, 0.0, false, 2.0, 1.0, 0.0, 0.0), "the plain night");
}

// ---------------------------------------------------------------- the gun's setup: the launcher's load

/// THE RULE: with the rifle up, 2 opens its setup and moves nothing, 2 again puts the next load he has any of on the launcher and shuts it, and a load he
/// is out of is stepped over. On the stack as on the open floor, and he stays on the wall.
/// A FAILURE MEANS: the launcher's key does nothing, or spends a press on an empty load.
#[test]
fn the_setups_load_row_steps_to_the_next_load_he_has() {
    use game::weapons::RifleAttachment as A;
    let mut p = Play::level("warehouse.json", false, &Opts { seed: 4101, ..Default::default() });
    p.g.god_mode = true;
    p.sam_takes_his_rifle();
    let (pl, rf) = (p.g.player, p.g.pl[p.g.player].rifle);
    assert!(game::game::player_rifle_up(&p.g, pl), "the staging is wrong: the rifle is not in his hands");
    p.g.fa[rf].shells = 2.0;
    let load = |p: &Play| game::weapons::rifle_attachment(&p.g, rf);
    assert_eq!(load(&p), Some(A::Ring));
    tap(&mut p, "Digit2");
    assert!(p.g.sets[p.g.seat].hand.setup.is_some() && load(&p) == Some(A::Ring), "2 opens the setup and moves nothing");
    tap(&mut p, "Digit2");
    assert!(p.g.sets[p.g.seat].hand.setup.is_none() && load(&p) == Some(A::He40), "2 again: the next load, and the setup shut");

    // the cameras, handed over as a mission hands them; the launcher then empty: from the camera to the ring, and from the ring past the 40 mm
    game::stickycam::sticky_issue(&mut p.g, pl, 2.0, None);
    p.g.fa[rf].shells = 0.0;
    p.run(0.3);
    tap(&mut p, "Digit2"); tap(&mut p, "Digit2");
    assert_eq!(load(&p), Some(A::Cam), "from the 40 mm, the next load he has");
    tap(&mut p, "Digit2"); tap(&mut p, "Digit2");
    assert_eq!(load(&p), Some(A::Ring));
    tap(&mut p, "Digit2"); tap(&mut p, "Digit2");
    assert_eq!(load(&p), Some(A::Cam), "the empty 40 mm is stepped over");

    // the same two presses stacked on a wall
    p.g.fa[rf].shells = 2.0;
    p.sam_put([21.5, 10.9]);
    let c = p.g.pl[pl].ch;
    p.g.ch[c].body_yaw = std::f64::consts::PI;
    p.frame();
    tap(&mut p, "KeyQ");
    p.run(50.0 / 60.0);
    assert!(p.g.pl[pl].wall.is_some(), "the staging is wrong: Q did not put him on the wall");
    let was = load(&p);
    tap(&mut p, "Digit2"); tap(&mut p, "Digit2");
    assert!(p.g.pl[pl].wall.is_some() && load(&p) != was, "on the stack: on the wall {}, load {:?} from {was:?}", p.g.pl[pl].wall.is_some(), load(&p));
}

// ---------------------------------------------------------------- a man in the gun's hold

/// Sam with a man in the gun's hold, settled. The man, and where he stands.
fn in_the_guns_hold() -> (Play, GuardId, [f64; 2]) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&nofow", &Opts { seed: 904, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.2);
    let man = defender(&p, None);
    tap(&mut p, "Digit1");
    p.run(1.0);
    let pl = p.g.player;
    assert!(p.g.pl[pl].slot == game::game::SlotKind::Pistol && !p.g.pl[pl].holstered, "the staging is wrong: the pistol is not in his hand");
    let at = p.at(man);
    p.sam_put([at[0] - 0.7, at[1]]);
    p.run(0.1);
    game::player::start_grab(&mut p.g, man, None);
    p.until(2.5, |p| holding(p)).expect("the grab never settled");
    p.run(0.5);
    (p, man, at)
}
fn holding(p: &Play) -> bool { p.g.pl[p.g.player].holding.as_ref().map_or(false, |h| h.phase == game::character::HoldPhase::Held) }
fn rounds(p: &Play) -> f64 { game::weapons::firearm_rounds_ready(&p.g, p.g.pl[p.g.player].pistol) }
fn charge(p: &Play) -> f64 { p.g.pl[p.g.player].ocp.charge }
fn click(p: &mut Play, b: Button) { p.press(b); p.frame(); p.release(b); }

/// THE RULE: in the gun's hold the left button is one round past him and the right is the OCP's press, the hold going on through either. With nothing lit
/// under the reticle the press is its refusal and spends nothing; with a light under it the light takes the charge.
/// A FAILURE MEANS: the buttons of the hold have gone to something else.
#[test]
fn in_the_guns_hold_the_buttons_are_a_round_and_the_ocp() {
    let (mut p, man, at) = in_the_guns_hold();
    p.point_at([at[0] + 6.0, at[1]]);
    p.run(0.1);
    let r0 = rounds(&p);
    click(&mut p, Button::Left);
    p.run(0.5);
    assert_eq!(r0 - rounds(&p), 1.0, "the left button: one round");
    assert!(holding(&p) && p.up(man), "the hold ended on the left button");
    drop(p);

    let (mut p, man, at) = in_the_guns_hold();
    p.point_at([at[0] + 6.0, at[1]]);
    p.run(0.1);
    assert!(p.g.sets[p.g.seat].aim_target.is_none() && p.g.sets[p.g.seat].aim_guard.is_none(), "the staging is wrong: something is under the reticle");
    let (r0, c0, heard) = (rounds(&p), charge(&p), p.sounds.len());
    click(&mut p, Button::Right);
    assert_eq!(rounds(&p), r0, "the right button: no round");
    assert!(p.sounds[heard..].iter().any(|x| x.name == "deny") && charge(&p) >= c0, "nothing lit: the refusal, and no charge spent");

    // a light of the room under the reticle
    let eye = p.sam_at();
    let lamp = p.g.sets[p.g.seat].interactables.iter().filter(|i| i.kind == game::game::InteractableKind::Light && i.target.is_some()).map(|i| i.pos).min_by(|a, b| {
        let d = |q: &[f64; 3]| (q[0] - eye[0]).hypot(q[2] - eye[1]);
        d(a).partial_cmp(&d(b)).unwrap()
    }).expect("the office has lights");
    p.cursor_on(lamp);
    p.until(1.0, |p| p.g.sets[p.g.seat].aim_target.is_some()).expect("the staging is wrong: the reticle did not snap to the lamp");
    let c0 = charge(&p);
    click(&mut p, Button::Right);
    assert!(charge(&p) < c0, "a lit lamp under the reticle: the charge is spent");
    p.run(0.5);
    assert!(holding(&p) && p.up(man), "the hold ended on the right button");
}

// ---------------------------------------------------------------- guns at walls and the carriages

/// A room, its south wall's face at the answer's z, and a defender on a script half a metre from it with his rifle up at a point past it. (A defender, because
/// only the room fight sets `over` on one of ours and it takes back what it did not set.)
fn at_a_wall() -> (Play, GuardId, f64, [f64; 3]) {
    let mut s = Scene::new("gun-at-a-wall");
    s.room("west", 10.0, 10.0, 18.0, 16.0).door("out", Axis::Z, 18.0, 12.4);
    s.bulb(14.0, 13.0).sam(11.0, 11.0);
    let mut p = Play::scene(&s, &Opts { seed: 37, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.2);
    let x = 14.0;
    let near = 13.0 + game::scene_collision::raycast(&mut p.g, [x, 1.3, 13.0], [0.0, 0.0, 1.0], 6.0).expect("the wall").t;
    let gd = game::guards::spawn_unit(&mut p.g, &game::guards::UnitOpts { team: game::teams::Team::Guard, pos: [x, 0.0, near - 0.5], yaw: Some(0.0), weapon: Some(game::weapons::WeaponId::Fn2000), light: Some(false), ..Default::default() }).unwrap();
    let aim = [x, 1.35, near + 8.0];
    game::squad::order(&mut p.g, gd, game::squad::OrderPatch { goal: Some(Some([x, 0.0, near - 0.5])), face: Some(Some(0.0)), aim_at: Some(Some(aim)), upper: Some(Some(game::character::UpperMode::Aim)), ..Default::default() });
    p.run(1.5);
    (p, gd, near, aim)
}

/// THE RULE: come back from a wall inside the band, a calm man's gun stays down; with a man in his sights it is up at once (not after the wait a calm man's
/// gun has), clear of the wall; and nearer than that it is down for him too, and clear of the wall there as well.
/// A FAILURE MEANS: men in a fight lose shots they have, or fire with the gun in a wall.
#[test]
fn a_gun_is_up_with_a_man_in_his_sights_as_soon_as_it_is_clear_of_the_wall() {
    let (mut p, gd, near, aim) = at_a_wall();
    let c = p.g.gd[gd].ch;
    let put = |p: &mut Play, off: f64, hot: bool, frames: usize| {
        p.g.ch[c].pos[2] = near - off;
        let over = if hot { Some(Some(GuardScriptOver { aim_at: aim, face: None, walk: None })) } else { None };
        game::squad::order(&mut p.g, gd, game::squad::OrderPatch { goal: Some(Some([14.0, 0.0, near - off])), over, ..Default::default() });
        for _ in 0..frames { p.frame(); }
    };
    put(&mut p, 0.7, false, 30);
    assert!(p.g.gd[gd].muzzle_down, "calm, 0.7 m off: the band keeps it down");
    put(&mut p, 0.7, true, 3);
    assert!(!p.g.gd[gd].muzzle_down, "in a fight, 0.7 m off: up inside three frames");
    for _ in 0..60 { p.frame(); }
    assert!(!p.g.gd[gd].muzzle_down && p.g.ch[c].anim.upper == game::character::UpperMode::Aim, "in a fight, 0.7 m off: it stays up");
    assert!(near - p.g.ch[c].muzzle[2] > 0.0 && !game::combat::arm_through(&mut p.g, c), "up, and clear of the wall");
    put(&mut p, 0.45, true, 60);
    assert!(p.g.gd[gd].muzzle_down, "in a fight, 0.45 m off: it would be in the wall and comes down");
    assert!(near - p.g.ch[c].muzzle[2] > 0.0 && !game::combat::arm_through(&mut p.g, c), "down, and clear of the wall");
}

/// THE RULE: the three carriages a man can be told are three poses: his trigger hand stands somewhere else in each, and no bone goes astray.
/// A FAILURE MEANS: the animator is reporting a carriage it is not drawing.
#[test]
fn each_carriage_is_a_pose_of_its_own() {
    use game::character::{Bone, LongStance as L};
    let mut p = Play::mission("office.json", "?levelFile=office.json&nofow", &Opts { seed: 4741, ..Default::default() });
    p.g.god_mode = true;
    let at = p.sam_at();
    let man = game::guards::spawn_unit(&mut p.g, &game::guards::UnitOpts { team: game::teams::Team::Guard, pos: [at[0] + 3.0, 0.0, at[1]], yaw: Some(0.0), weapon: Some(game::weapons::WeaponId::Aks74u), carbine_rig: Some(true), light: Some(false), ..Default::default() }).unwrap();
    p.g.gd[man].hold = true;
    let c = p.g.gd[man].ch;
    p.run(0.5);
    let mut hands: Vec<[f64; 3]> = Vec::new();
    for told in [L::PatrolLong, L::LowLong, L::PortLong] {
        p.g.ch[c].anim.long_stance = Some(told);
        p.run(1.25);
        assert_eq!(p.g.ch[c].anim.long_shown, Some(told));
        let (hand, feet) = (*p.g.ch[c].bones.get(Bone::HandR).expect("no hand"), p.g.ch[c].pos);
        let from_feet = [hand[0] - feet[0], hand[1] - feet[1], hand[2] - feet[2]];
        assert!(from_feet.iter().all(|x| x.is_finite() && x.abs() < 2.0), "{told:?}: the hand stands at {from_feet:?}");
        for h in &hands {
            let apart = ((h[0] - from_feet[0]).powi(2) + (h[1] - from_feet[1]).powi(2) + (h[2] - from_feet[2]).powi(2)).sqrt();
            assert!(apart > 0.05, "{told:?}: the hand is {apart} m from another carriage's");
        }
        hands.push(from_feet);
    }
}

// ---------------------------------------------------------------- what a box's shape is to a ray, and a charge on an outer wall

/// THE RULE: a ray down onto a box that says an arc lands on the round, never under it, and no further over it than the coarsest facets stand off.
/// A FAILURE MEANS: the arc's top reads as its flat box to a round or a line of sight.
#[test]
fn a_ray_onto_an_arc_lands_on_the_round() {
    let mut p = Play::mission("bazaar.json", "?levelFile=bazaar.json&quiet", &Opts::default());
    let ids: Vec<_> = p.g.tab.boxes.iter().map(|(i, _)| i).collect();
    let mut asked = 0;
    for b in ids {
        let (arc, c, h, yaw) = { let x = &p.g.tab.boxes[b]; (x.shape.as_ref().and_then(|s| s.arc), x.c, x.h, x.yaw) };
        let Some(arc) = arc else { continue };
        let w = if arc.axis == 1.0 { h[2] } else { h[0] };
        let r = (w * w + arc.rise * arc.rise) / (2.0 * arc.rise);
        let axis_y = h[1] - r;
        let stand = r * (1.0 / ((w / r).asin() / 1.5 / 2.0).cos() - 1.0);   // (two facets: the coarsest a box's own planes leave it)
        for k in [-0.5, -0.1, 0.1, 0.5] {
            let along = k * w;
            let l = if arc.axis == 1.0 { [0.0, along] } else { [along, 0.0] };
            let (s, co) = (yaw.sin(), yaw.cos());
            let o = [c[0] + l[0] * co + l[1] * s, c[1] + 3.0, c[2] - l[0] * s + l[1] * co];
            let hit = game::scene_boxes::ray_box(&mut p.g, o, [0.0, -1.0, 0.0], b.into(), 6.0).expect("a ray down missed the box");
            let y = o[1] - hit.t - c[1];
            let off = (y - axis_y).hypot(along) - r;
            assert!(off > -1e-6 && off <= stand + 1e-6, "{off} off the round (rise {}, half width {w}, stand {stand})", arc.rise);
            asked += 1;
        }
    }
    assert!(asked > 0, "the level has lost its arcs");
}

/// THE RULE: a charge the squad sets on an outer wall opens it.
/// A FAILURE MEANS: wall charges have stopped making holes in a level's outer walls.
#[test]
fn a_charge_on_an_outer_wall_makes_a_hole() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 5, ..Default::default() });
    p.sam_unhurt();
    p.run(0.1);
    game::wallcharge::wall_charge_raise_carried(&mut p.g, 2.0);
    // the first face of a wall of the level that takes a charge from the floor in front of it and has the outside behind it
    let mut outer = None;
    for &b in &p.g.col.boxes.clone() {
        let (c, h) = { let x = &p.g.tab.boxes[b]; (x.c, x.h) };
        let across_x = h[0] < h[2];
        for side in [1.0, -1.0] {
            let (at, from) = if across_x { ([c[0] + side * (h[0] + 0.05), 0.0, c[2]], [c[0] + side * (h[0] + 2.0), c[2]]) } else { ([c[0], 0.0, c[2] + side * (h[2] + 0.05)], [c[0], c[2] + side * (h[2] + 2.0)]) };
            if game::scene_collision::nav_grid_is_blocked(&p.g, from[0], from[1]) { continue; }
            let Some(w) = game::wallcharge::wall_at(&mut p.g, at, Some([from[0], 0.0, from[1]])) else { continue };
            if w.ok && w.outer == Some(true) { outer = Some((at, from)); break; }
        }
        if outer.is_some() { break; }
    }
    let (at, from) = outer.expect("the office has no outer wall that takes a charge");
    p.sam_put(from);
    p.frame();
    if let Some(why) = game::wallcharge::wall_charge_order(&mut p.g, at, None) { panic!("the wall charge was refused: {why}"); }
    p.until(60.0, |p| !p.g.sets[p.g.seat].wall_charge.set.is_empty()).unwrap_or_else(|| panic!("a minute and no charge is on the wall: {}", p.roll()));
    let before = p.g.destr.walls.changes.len();
    let charge = p.g.sets[p.g.seat].wall_charge.set[0];
    game::wallcharge::wall_charge_blow(&mut p.g, Some(charge), Some(true));
    p.run(0.2);
    assert!(p.g.destr.walls.changes[before..].iter().any(|r| r.kind == game::wallbreak::LevelChangeKind::Hole), "no hole");
}

// ---------------------------------------------------------------- a grenade by a door

/// THE RULE: four men by a door, a grenade three metres into the room with no cover, Sam where their own walk does not take them through it. A shut door:
/// they get off it, none is dealt a way out by it, and it stays on its latch to the burst. A door standing open: two at least are through it at the burst, and
/// the burst does not touch those. (A man already walking into a shut door when the shout reaches him opens it as any man does: that is not this rule.)
/// A FAILURE MEANS: men run for a door that is shut, or do not use one that is open.
#[test]
fn men_getting_off_a_grenade_leave_by_a_door_that_stands_open_and_not_a_shut_one() {
    use game::combat::kill_guard;
    use game::frag::BlastResult::{Killed, Wounded};
    const FUSE: f64 = 3.2;
    for open in [false, true] {
        let mut p = Play::mission("floodlit.json", "?levelFile=floodlit.json&roomclear=5&squad4", &Opts { seed: 5, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.5);
        let ours: Vec<_> = p.mates().into_iter().take(4).collect();
        assert_eq!(ours.len(), 4, "the staging is wrong: not four men");
        for &m in &ours { p.g.gd[m].god = false; }
        let theirs: Vec<_> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team == game::teams::Team::Hostile && p.up(gd)).collect();
        let (thrower, rest) = theirs.split_first().unwrap();
        for &x in rest { kill_guard(&mut p.g, x, [0.0, 0.0, 1.0], Some(true), None, None, game::game::With::Hand); }
        p.sam_put([9.0, 26.2]);
        p.run(0.1);
        let d = game::doors::doors_by_name(&p.g, "foot_gate").unwrap();
        let frame_x = p.g.dr[d].frame_centre[0];
        let marks = [[2.8, 22.75], [2.8, 20.75], [2.0, 23.45], [2.0, 20.05]];
        game::doors::door_set_locked(&mut p.g, d, false);
        game::doors::door_set_latched(&mut p.g, d, !open);
        game::doors::door_set_angle(&mut p.g, d, if open { 1.5 } else { 0.0 });
        for (i, &m) in ours.iter().enumerate() { p.man_stands(m, marks[i]); }
        p.run(0.05);
        for (i, &m) in ours.iter().enumerate() { p.man_stands(m, marks[i]); }
        let who = p.id_of(*thrower);
        let (dealt0, fled0, bursts0) = (p.g.stats.frag.by_door, p.g.stats.frag.fled, p.g.stats.frag.bursts);
        game::throwables::throwables_spawn(&mut p.g, game::throwables::ItemKind::Frag, [1.0, 0.05, 21.75], [0.0, 0.0, 0.0], Some(game::throwables::ThrowablesSpawnOpts { fuse: Some(FUSE), who: Some(who), ..Default::default() }));
        let (mut latched, mut swung, mut there) = (true, 0.0f64, Vec::new());
        for _ in 0..(((FUSE + 0.5) * 60.0) as usize) {
            there = ours.iter().map(|&m| (m, p.at(m))).collect();
            latched &= game::doors::door_latched(&mut p.g, d);
            swung = swung.max(game::doors::door_angle(&p.g, d).abs());
            p.frame();
            if p.g.stats.frag.bursts > bursts0 { break; }
        }
        assert!(p.g.stats.frag.bursts > bursts0, "the grenade never went off");
        assert!(p.g.stats.frag.fled > fled0, "nobody got off it");
        let dealt = p.g.stats.frag.by_door - dealt0;
        if !open {
            assert_eq!((dealt, latched), (0, true), "the door is shut: deals of it, and on its latch");
            assert!(swung < 0.02, "the door is shut and swung {swung}");
            continue;
        }
        let through: Vec<_> = there.iter().filter(|(_, at)| at[0] > frame_x + 0.2).map(|(m, _)| *m).collect();
        assert!(through.len() >= 2, "the door stands open and {} are through it at the burst ({dealt} deals of it)", through.len());
        let rep = p.g.m.frag.frag_log.last().expect("no report of the burst").clone();
        for m in through {
            let id = p.id_of(m);
            assert!(!rep.hits.iter().any(|h| h.id == id && matches!(h.result, Killed | Wounded)), "a man through the doorway was hit");
        }
    }
}

// ---------------------------------------------------------------- a Training level's kit

/// THE RULE: a Training level is played with the kit its lesson is taught with: what a player is built with of his own four things, whatever the issue card
/// counts out, as the address of its level says. The same address of a mission's level wears the card.
/// A FAILURE MEANS: a lesson asks for a thing he has too few of, on a page that starts its game from the address.
#[test]
fn a_training_level_wears_its_lessons_kit() {
    for s in [1, 2, 3] {
        let start = |level: &str| Play::mission(level, &format!("?levelFile={level}&roomclear=1&squad4"), &Opts { seed: s, ..Default::default() });
        let built = kit(&start("tut-team.json"));
        assert_eq!((built.3, built.4, built.5, built.0), (4.0, 2.0, 2.0, 5.0), "seed {s}: smoke, stuns, frags and sponge rounds on a lesson");
        let issued = kit(&start("killhouse.json"));
        assert_eq!((issued.3, issued.4, issued.5), (1.0, 1.0, 1.0), "seed {s}: the control wears the issue card");
    }
}
