//! THE WORLD'S FAULTS AS THE OLD CHECKS READ THEM, ASKED OF THE GAME ITSELF: a round into a wall and its marks, the breaker's line, the words for Sam on a
//! stack's place, a man who sees a blow, what a restart leaves of a line, a run's verdict and the suit a quiet mission wears.
//!
//! Each of those checks staged through the page's copy (a hook on a function the tick does not call, a wrap of the copy's own `msg`, a row that does not
//! cross). Asked here the game does what the rule says, and a failure below is the game's and not a staging's.
use game::game::{DownBy, DownKind};
use game::orders::OrderTarget;
use game_start::play::{dist, Opts, Play};
use game_start::scene::{Axis, Scene};

fn hostile_down(p: &mut Play) { for gd in p.g.guards.clone() { if p.g.gd[gd].team == game::teams::Team::Hostile && p.up(gd) { game::combat::kill_guard(&mut p.g, gd, [0.0, 0.0, 1.0], Some(true), None, None, None); } } }

// ---------------------------------------------------------------- a round into a wall

/// THE RULE: a round through a thin wall leaves one mark on its far face and one where it went in, the far face first, and the wall's wear takes the one where it went in. So a wall crossed is two
/// marks, not one and not five.
/// A FAILURE MEANS: the marks a player sees on a wall no longer come in the order the wear reads them, or a round leaves more than it owes.
#[test]
fn a_round_through_a_wall_leaves_the_far_hole_then_the_mark_it_wears() {
    use game::render_decals::{mark_set, DecalKind, Mark};
    let mut p = Play::level("office.json", false, &Opts { seed: 8201, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.1);
    p.sam_put([21.5, 11.4]);
    p.run(0.5);
    // a round fired between two ticks: the marks the next one lays of it, in their order
    let fired = |p: &mut Play, at: [f64; 3]| -> (Vec<Mark>, usize) {
        let (c, had, crossed) = (p.g.pl[p.g.player].ch, mark_set(&p.g).map_or(0, |s| s.serial), p.g.stats.wallpen.crossed);
        game::combat::fire_weapon(&mut p.g, c, at, None);
        let crossed = (p.g.stats.wallpen.crossed - crossed) as usize;
        p.frame();
        (mark_set(&p.g).map_or(Vec::new(), |s| s.list.iter().filter(|m| m.serial > had && matches!(m.kind, DecalKind::Bullet | DecalKind::Damage)).copied().collect()), crossed)
    };
    let (marks, crossed) = fired(&mut p, [24.0, 1.2, 10.0]);
    assert!(crossed > 0, "the staging is wrong: the round crossed no wall");
    assert!(marks.len() == 2 * crossed || marks.len() == 2 * crossed + 1, "{} walls crossed and {} marks", crossed, marks.len());
    let from = p.g.ch[p.g.pl[p.g.player].ch].muzzle;
    let along = |q: [f64; 3]| (q[0] - from[0]) * (24.0 - from[0]) + (q[2] - from[2]) * (10.0 - from[2]);
    let worn: Vec<u32> = mark_set(&p.g).unwrap().wear.cells.iter().flat_map(|c| c.1.clone()).collect();
    assert_eq!(worn.len(), crossed, "each wall crossed counts the round, once");
    for (k, pair) in marks.chunks_exact(2).take(crossed).enumerate() {
        let (far, went_in) = (pair[0], pair[1]);
        assert!(along(far.pos) > along(went_in.pos), "wall {k}: the mark told first is not the far face's");
        assert!(far.n[0] * went_in.n[0] + far.n[1] * went_in.n[1] + far.n[2] * went_in.n[2] < -0.99, "wall {k}: the two marks face opposite ways");
        assert!(worn.contains(&went_in.serial) && !worn.contains(&far.serial), "wall {k}: the wear takes the mark where it went in");
    }
    // the control: a round stopped by something too thick to cross is one mark and no far hole, and nothing is counted crossed
    let at = p.sam_at();
    let (marks, crossed) = fired(&mut p, [at[0] + 1.0, 0.0, at[1]]);   // (the floor a metre off)
    assert_eq!((marks.len(), crossed), (1, 0), "into the floor: marks, walls crossed");
}

// ---------------------------------------------------------------- the squad's words

/// THE RULE: a teammate who throws the breaker gets the game's line for it, as Sam's own hand does, and says it is done. Once each.
/// A FAILURE MEANS: the mains go dark with no word why.
#[test]
fn a_teammate_throwing_the_breaker_gets_the_games_line_and_his_own() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear", &Opts { seed: 3, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.2);
    for d in p.g.doors.list.clone() { game::doors::door_set_locked(&mut p.g, d, false); }
    p.run(0.3);
    let th = game::workthings::work_things_things(&p.g, &game::seats::here(&p.g)).into_iter().find(|x| x.kind == game::workthings::WorkThingKind::Breaker).expect("the office has a breaker");
    let t = game::orders::squad_orders_target_at(&mut p.g, th.face);
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "work", Some(&t), None, None), None, "the order was refused");
    let lines0 = p.said.len();
    p.until(40.0, |p| p.g.light_home.mains.active).unwrap_or_else(|| panic!("forty seconds and the mains are on: {}", p.roll()));
    let mut thrown = 1;
    for _ in 0..300 { let was = p.g.light_home.mains.active; p.frame(); if !was && p.g.light_home.mains.active { thrown += 1; } }
    assert_eq!(thrown, 1, "the mains go once");
    assert_eq!(p.said[lines0..].iter().filter(|l| l.text == "lights out.").count(), 1, "his own line, once");
}

/// THE RULE: where the one thing on a stack's places is Sam, the menu's greyed word, the order's refusal and the door tools' test all say so, in one set of words;
/// where there is no room with him away too, they all say that.
/// A FAILURE MEANS: one fact has two refusals on a player's screen.
#[test]
fn sam_on_a_stacks_place_is_one_refusal_everywhere() {
    let mut p = Play::mission("barracks.json", "?levelFile=barracks.json&roomclear=1&squad4", &Opts { seed: 1, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.3);
    hostile_down(&mut p);
    p.run(0.2);
    let (d, side) = (game::doors::doors_by_name(&p.g, "sg_w").unwrap(), 1.0);
    let men: Vec<_> = p.mates().into_iter().take(4).collect();
    let fc = p.g.dr[d].frame_centre;
    for (i, m) in men.iter().enumerate() { p.man_stands(*m, [14.85 - 1.8 * (i as f64 / 2.0).floor(), 21.75 - 0.9 * (i % 2) as f64]); }
    game::game::game_teleport_player(&mut p.g, [1e4, 0.0, 1e4]);
    let places = game::orders::squad_orders_stack_posts_wait(&mut p.g, d, side, &men).expect("the staging is wrong: no room with Sam away");
    let last = places.last().unwrap().pos;
    p.sam_put([last[0], last[2]]);
    let t = OrderTarget::Door(game::orders::OrderTargetDoor { at: [fc[0], 0.0, fc[2]], door: d, side });
    let def = p.g.tab.orders_order_def.iter().map(|(i, _)| i).find(|&x| p.g.tab.orders_order_def[x].id == "stack").unwrap();
    const IN_WAY: &str = "you're in the way.";
    assert_eq!(game::orders::squad_orders_why_not(&mut p.g, def, &t, None).as_deref(), Some(IN_WAY), "the menu's greyed word");
    match game::doortools::door_tools_can_threshold(&mut p.g, &t, &men) {
        game::doortools::DoorToolsCanThreshold::Text(s) => assert_eq!(s, IN_WAY, "the door tools' test"),
        other => panic!("the door tools' test: {other:?}"),
    }
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "stack", Some(&t), None, None).as_deref(), Some(IN_WAY), "the order");
    // the control: Sam off every place, the same order is taken and nobody is refused
    let aside = [fc[0] - 4.0, fc[2] + 4.0];
    assert!(places.iter().all(|q| dist(aside, [q.pos[0], q.pos[2]]) > 1.5) && !game::scene_collision::nav_grid_is_blocked(&p.g, aside[0], aside[1]), "the staging is wrong: no floor aside");
    p.sam_put(aside);
    assert_eq!(game::orders::squad_orders_why_not(&mut p.g, def, &t, None), None, "Sam off the places: the row is live");
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "stack", Some(&t), None, None), None, "Sam off the places: the order is taken");
    drop(p);

    // a door with no room for four, Sam or no Sam: all three say that, and not that he is in the way
    const NO_ROOM: &str = "no room on that door.";
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear=1&squad4", &Opts { seed: 1, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.3);
    hostile_down(&mut p);
    p.run(0.2);
    let d = game::doors::doors_by_name(&p.g, "hall_n").unwrap();
    let men: Vec<_> = p.mates().into_iter().take(4).collect();
    let fc = p.g.dr[d].frame_centre;
    p.sam_put([2.0, 17.0]);
    let t = OrderTarget::Door(game::orders::OrderTargetDoor { at: [fc[0], 0.0, fc[2]], door: d, side });
    assert_eq!(game::orders::squad_orders_why_not(&mut p.g, def, &t, None).as_deref(), Some(NO_ROOM), "the menu's greyed word");
    match game::doortools::door_tools_can_threshold(&mut p.g, &t, &men) {
        game::doortools::DoorToolsCanThreshold::Text(s) => assert_eq!(s, NO_ROOM, "the door tools' test"),
        other => panic!("the door tools' test: {other:?}"),
    }
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "stack", Some(&t), None, None).as_deref(), Some(NO_ROOM), "the order");
}

// ---------------------------------------------------------------- a man who sees a blow

/// A room with A on his mark, B on his, Sam away; B held, looking where `look` says.
fn two_men(look: [f64; 2]) -> (Play, game::ids::GuardId, game::ids::GuardId) {
    let mut s = Scene::new("a-man-sees-a-blow");
    s.room("west", 10.0, 10.0, 22.0, 16.0).door("out", Axis::Z, 22.0, 12.4);
    s.bulb(16.0, 13.0);
    s.sentry("a", [18.0, 13.0], [18.0, 11.0]).sentry("b", [12.0, 13.0], look).sam(11.0, 15.5);
    let mut p = Play::scene(&s, &Opts { seed: 402, ..Default::default() });
    p.g.god_mode = true;
    let (a, b) = (p.guard("a"), p.guard("b"));
    p.g.gd[b].hold = true;
    p.run(0.8);   // (his head comes to where he looks)
    (p, a, b)
}

/// THE RULE: a man sees a blow only along his head: with it on the man struck he has seen it; with his head turned off him he has not, whatever his body would allow.
/// A FAILURE MEANS: the witness record has gone back to asking something other than the eye every other sense asks.
#[test]
fn a_man_sees_a_blow_only_along_his_head() {
    let (mut p, a, b) = two_men([18.0, 13.0]);
    p.falls_as(a, DownKind::Struck, true, DownBy::Player);
    p.run(0.1);
    assert!(p.g.gd[b].witness.saw_act.is_some(), "his head on the man: he saw it");
    drop(p);
    // his chest on the man, his head turned off him: he has not seen it
    let (mut p, a, b) = two_men([18.0, 13.0]);
    let c = p.g.gd[b].ch;
    let y = p.g.ch[c].body_yaw;
    p.g.ch[c].aim_yaw = y + 1.5;
    p.g.ch[c].anim.look_yaw_extra = 1.5;
    game::character::character_update(&mut p.g, c, 0.0);
    let look = game::guards::look_of(&p.g, c);
    assert!(look[0] < 0.5, "the staging is wrong: his head has not turned off the man: {look:?}");
    p.falls_as(a, DownKind::Struck, true, DownBy::Player);
    assert!(p.g.gd[b].witness.saw_act.is_none(), "his chest on the man and his head off him: he did not");
    drop(p);
    let (mut p, a, b) = two_men([12.0, 10.0]);
    p.falls_as(a, DownKind::Struck, true, DownBy::Player);
    p.run(0.1);
    assert!(p.g.gd[b].witness.saw_act.is_none(), "his head turned away: he did not");
}

// ---------------------------------------------------------------- what a restart leaves

/// THE RULE: nothing a man was saying outlives a restart: not on the man, not over his head on the HUD.
/// A FAILURE MEANS: a line from the run that ended hangs over a man in the new one, and the clocks that read it hold him back.
#[test]
fn a_restart_leaves_no_line_on_anybody() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&rescue", &Opts { seed: 1, ..Default::default() });
    p.run(1.0);
    for gd in p.g.guards.clone() { game::game::game_say(&mut p.g, gd, "still talking", None, Some(true)); }
    p.frame();
    assert!(p.view().bubbles.iter().any(|b| b.text == "still talking"), "the staging is wrong: nothing on the HUD");
    game::savestate::control::restart(&mut p.g);
    assert!(p.g.guards.iter().all(|&gd| p.g.gd[gd].bubble.as_ref().map_or(true, |b| b.text != "still talking")), "a line on a man");
    p.frame();
    assert!(!p.view().bubbles.iter().any(|b| b.text == "still talking"), "a line on the HUD");
}

// ---------------------------------------------------------------- a run's verdict

/// Won with every defender down, the verdict latched.
fn won(level: &str, link: &str) -> game::verdict::RunVerdict {
    let mut p = Play::mission(level, link, &Opts { seed: 1, ..Default::default() });
    p.run(0.1);
    let at = p.sam_at();
    p.sam_put([at[0] + 2.0, at[1]]);
    p.run(0.1);
    for gd in p.g.guards.clone() { if p.g.gd[gd].team == game::teams::Team::Hostile && p.up(gd) { p.falls(gd, DownBy::Player); } }
    p.run(4.0);
    p.g.verdict.res.clone().unwrap_or_else(|| panic!("no verdict: {}", p.roll()))
}

/// THE RULE: a run's card has a row a man; one pair carries no pair colours, and four in two pairs carry theirs.
/// A FAILURE MEANS: the debrief's men are not told apart as the squad was, or are told apart where there is nothing to tell.
#[test]
fn the_verdicts_men_carry_pair_colours_only_where_there_are_two_pairs() {
    use game::host_findme::OutlineHue::{Blue, Red};
    let one = won("office.json", "?levelFile=office.json&nightbreach");
    assert!(one.won && one.men.len() == 3, "the night breach: won {}, {} rows", one.won, one.men.len());
    assert!(one.men[1..].iter().all(|m| m.tone.is_none()), "one pair: no colours");
    let four = won("office.json", "?levelFile=office.json&roomclear&squad4");
    let of = |hue| four.men[1..].iter().filter(|m| m.tone == Some(hue)).count();
    assert!(four.won && four.men.len() == 5 && of(Red) == 2 && of(Blue) == 2, "four: won {}, {} rows, {} red, {} blue", four.won, four.men.len(), of(Red), of(Blue));
}

/// THE RULE: Sam going down is the verdict DOWN, not won, his own row dead, with no approach to its name, once the try has started.
/// A FAILURE MEANS: the card reads a lost run as anything else.
#[test]
fn sam_going_down_is_the_verdict_down() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 1, ..Default::default() });
    p.run(0.1);
    let at = p.sam_at();
    p.sam_put([at[0] + 2.0, at[1]]);
    p.run(0.3);
    p.seat_goes_down(0);
    p.run(0.2);
    let v = p.g.verdict.res.clone().unwrap_or_else(|| panic!("no verdict: {}", p.roll()));
    assert_eq!((v.won, v.word.as_str()), (false, "DOWN"));
    assert!(v.approach.is_none() && v.men[0].human && v.men[0].state == game::verdict::ManState::Dead, "approach {:?}, his row {:?}", v.approach, v.men[0].state);
}

// ---------------------------------------------------------------- the suit a quiet mission wears

fn wears(p: &Play, c: game::ids::CharId, part: &str) -> bool {
    let rig = p.g.ch[c].rig;
    let cast = p.g.tab.rig[rig].cast.expect("no cast on this stand");
    let at = p.g.tab.anim_gltf_cast_info[cast].parts.iter().position(|n| n == part).unwrap_or_else(|| panic!("no part {part}"));
    (p.g.ch[c].parts as u32) & (1 << at) != 0
}

/// THE RULE: on a quiet mission Sam and his partner wear the suit, no helmet; on a loud one Sam and his men wear the helmet; and a picture that says it is
/// dressing a quiet mission (the game's `suit_asked`) gets the suit on a staged room clear.
/// A FAILURE MEANS: a player sees helmets on a mission that has none.
#[test]
fn a_quiet_mission_wears_the_suit_and_a_loud_one_the_helmet() {
    let mut p = Play::mission("crossing.json", "?levelFile=crossing.json&quiet&squad=1", &Opts::default());
    p.run(0.2);
    let team: Vec<_> = std::iter::once(p.g.pl[p.g.player].ch).chain(p.g.staged.team.iter().map(|&m| p.g.gd[m].ch)).collect();
    assert_eq!(team.len(), 2);
    for &c in &team { assert!(!wears(&p, c, "helmet"), "a quiet mission: a helmet"); }
    drop(p);
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts::default());
    p.run(0.2);
    let team: Vec<_> = std::iter::once(p.g.pl[p.g.player].ch).chain(p.mates().iter().map(|&m| p.g.gd[m].ch)).collect();
    assert_eq!(team.len(), 5);
    for &c in &team { assert!(wears(&p, c, "helmet"), "a loud mission: no helmet"); }
    p.g.m.scene_look.suit_asked = Some(game::scene_look::SceneLookModSuitAsked::Quiet);
    p.run(0.1);
    for &c in &team { assert!(!wears(&p, c, "helmet"), "asked to dress as a quiet mission: a helmet"); }
}

// ---------------------------------------------------------------- the start card's show, and glass in the air

/// THE RULE: while nobody plays (the title's reel) the men live their night, and nothing of it is the player's: no game is kept, nothing is counted, nobody speaks to him. The encounter put back is a
/// fresh night's. CONTROL: the same seconds played are counted.
/// A FAILURE MEANS: a night begins with what the reel behind the title did, or a man starts on top of the way in.
#[test]
fn a_night_after_the_reel_is_a_fresh_night() {
    let mut p = Play::level("office.json", false, &Opts { seed: 1, ..Default::default() });
    p.run(0.1);
    game::savestate::control::restart(&mut p.g);
    let marks: Vec<_> = p.g.guards.iter().map(|&g| p.at(g)).collect();
    (p.g.aside, p.g.player_invisible) = (true, true);
    p.run(240.0);
    assert!(p.view().mission.text.is_empty() && p.view().mission.pos.is_none(), "the player is told where to go");
    assert!(p.g.guards.iter().zip(&marks).any(|(&g, m)| dist(p.at(g), *m) > 3.0), "nobody walked his beat");
    assert_eq!((p.g.mission.stats.elapsed, p.g.saves.stats.autosaves, p.view().radio.len()), (0.0, 0.0, 0), "the reel was counted, kept or spoken over: {:?}", p.view().radio);
    (p.g.aside, p.g.player_invisible) = (false, false);
    game::game::game_restart(&mut p.g);
    p.frame();
    let now: Vec<_> = p.g.guards.iter().map(|&g| p.at(g)).collect();
    assert!(marks.iter().zip(&now).all(|(a, b)| dist(*a, *b) < 0.05), "a frame after the reel: {now:?} against a fresh night's {marks:?}");
    p.run(2.0);
    assert!(p.g.mission.stats.elapsed > 1.9);
}

/// THE RULE: a game kept while glass is in the air and put back plays on as it did, word for word: the falls are the game's, in its block.
/// A FAILURE MEANS: a checkpoint taken in a firefight by a window comes back to another game.
#[test]
fn a_game_kept_with_glass_in_the_air_plays_on_as_it_did() {
    let mut p = Play::level("office.json", false, &Opts { seed: 7, ..Default::default() });
    p.g.god_mode = true;
    p.run(0.2);
    // the first pane of the level with floor in front of it
    let panes: Vec<_> = p.g.col.boxes.clone().into_iter().filter(|&b| game::glass::is_pane(&mut p.g, Some(b))).collect();
    assert!(!panes.is_empty(), "the office has lost its glass");
    let c = p.g.pl[p.g.player].ch;
    let mut fell = false;
    'panes: for b in panes {
        let (at, h) = { let x = &p.g.tab.boxes[b]; (x.c, x.h) };
        for side in [2.0, -2.0] {
            let from = if h[0] < h[2] { [at[0] + side, at[2]] } else { [at[0], at[2] + side] };
            if game::scene_collision::nav_grid_is_blocked(&p.g, from[0], from[1]) { continue; }
            p.sam_put(from);
            p.run(0.2);
            for _ in 0..12 { game::combat::fire_weapon(&mut p.g, c, [at[0], 1.2, at[2]], None); p.run(0.1); if game::sim_fallshome::fall_store_waiting(&p.g) > 0.0 { fell = true; break 'panes; } }
        }
    }
    assert!(fell, "the staging is wrong: twelve rounds and no glass in the air");
    let waiting = game::sim_fallshome::fall_store_waiting(&p.g);
    let kept = game::savestate::control::Moment::of(&p.g);
    let mut first = Vec::new();
    for _ in 0..120 { p.frame(); first.push(p.words()); }
    assert!(game::sim_fallshome::fall_store_waiting(&p.g) != waiting, "the staging is wrong: nothing landed in two seconds");
    p.g = kept.stands(p.g.sim);
    assert_eq!(game::sim_fallshome::fall_store_waiting(&p.g), waiting, "put back: the landings waiting");
    for (k, want) in first.iter().enumerate() {
        p.frame();
        let got = p.words();
        assert!(&got == want, "the word parts at tick {k}: {:?}", game_start::play::parting(std::slice::from_ref(want), std::slice::from_ref(&got)));
    }
}
