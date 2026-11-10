//! TOLD WHY: a press of his own that did nothing leaves its word as a greyed row after his rows (`interact::refuse`, `view.empty`), and whatever else the player must be told a man of the
//! team says (`squadnet::team_says`). These are the ten places that used to write only to a log nobody reads.
mod doors_ask;
use doors_ask::*;
use game::game::{DownBy, DownKind, GuardState};
use game::handring::EMPTY_SHOW_SECS;
use game::ids::*;
use game::nightbreach::NightBreachState;
use game::orders::{OrderTarget, Who};
use game::view::{BubbleTone, ThingKind};
use game_start::play::{nudge, Button, Opts, Play};
use game_start::scene::{Axis, Scene};

fn word(p: &Play) -> Option<String> { p.view().empty.map(|e| e.word) }
fn sams_line(p: &Play) -> Option<String> { p.view().bubbles.iter().find(|b| b.tone == BubbleTone::Own).map(|b| b.text.clone()) }
fn said_by_one_of(p: &Play, men: &[GuardId], text: &str) -> bool { p.said.iter().any(|l| l.text == text && men.contains(&l.who)) }
fn no_team_line(p: &Play, text: &str) -> bool { !p.said.iter().any(|l| l.text.contains(text)) }

/// A lit room with a door, Sam in it with his pistol up and his cursor down the room.
fn gun_room(s: u32, sam: [f64; 2]) -> Play {
    let mut sc = Scene::new("gun-room");
    sc.room("west", 10.0, 10.0, 18.0, 16.0).door("south", Axis::X, 16.0, 12.4).bulb(14.0, 13.0);
    sc.sam(sam[0], sam[1]);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.g.player_invisible = true;
    let pl = p.g.player;
    p.g.pl[pl].slot = game::game::SlotKind::Pistol;
    p.g.pl[pl].holstered = false;
    p.g.pl[pl].equip_t = 0.0;
    p.run(0.8);
    p
}

/// THE RULE: the trigger on a stunned man's gun, or a gun held low for a wall, says why, and held it keeps saying so. It goes with its time once the trigger is let go.
/// CONTROL: the same click with the gun clear fires, and says nothing.
/// A FAILURE MEANS: a click that does nothing is answered by nothing.
#[test]
fn the_trigger_with_the_gun_low_says_why() {
    for s in SEEDS {
        let z = 13.0 + 0.5 * nudge(s, 0);
        let mut p = gun_room(s, [14.0, z]);
        p.point_at([17.0, z]);
        p.run(0.6);
        p.press(Button::Left);
        p.frame();
        p.release(Button::Left);
        assert_eq!(word(&p), None, "seed {s}: the control: a clear gun says nothing");

        p.run(0.5);
        game::samstun::stagger_player(&mut p.g, 9.0, [1.0, 0.0, 0.0]);
        p.frame();
        assert_eq!(word(&p), None, "seed {s}: nobody has pressed");
        p.press(Button::Left);
        p.frame();
        assert_eq!(word(&p).as_deref(), Some("you're still stunned"), "seed {s}");
        p.run(EMPTY_SHOW_SECS + 0.5);
        assert_eq!(word(&p).as_deref(), Some("you're still stunned"), "seed {s}: the trigger is still held");
        p.release(Button::Left);
        p.run(EMPTY_SHOW_SECS + 0.2);
        assert_eq!(word(&p), None, "seed {s}: gone with its time");
        drop(p);

        let mut p = gun_room(s, [17.45, z]);
        p.point_at([25.0, z]);
        p.until(1.5, |p| p.view().aim.ring.is_some_and(|r| r.shut)).unwrap_or_else(|| panic!("seed {s}: at the wall the gun is not shut"));
        p.press(Button::Left);
        p.frame();
        p.release(Button::Left);
        assert_eq!(word(&p).as_deref(), Some("no room to raise your gun here"), "seed {s}");
    }
}

/// THE RULE: a teammate in Sam's line: the click names him. CONTROL: the same click with him out of the line says nothing of him.
/// A FAILURE MEANS: the player can't tell why his gun stays down beside a friend.
#[test]
fn the_trigger_with_a_friend_in_the_line_names_him() {
    for s in SEEDS {
        let (sc, _) = yard(s);
        let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.g.player_invisible = true;
        let pl = p.g.player;
        p.g.pl[pl].slot = game::game::SlotKind::Pistol;
        p.g.pl[pl].holstered = false;
        p.g.pl[pl].equip_t = 0.0;
        p.run(0.8);
        let (sam, men) = (p.sam_at(), p.men());
        p.drop_all_but(&men);
        let name = p.g.gd[men[0]].callsign.clone();
        p.point_at([sam[0] + 4.0, sam[1]]);
        p.man_stands(men[1], [sam[0] - 4.0, sam[1] + 4.0]);
        p.man_stands(men[0], [sam[0] - 4.0, sam[1] - 4.0]);
        p.run(1.0);
        p.press(Button::Left);
        p.frame();
        p.release(Button::Left);
        assert!(word(&p).map_or(true, |w| !w.contains(&name)), "seed {s}: the control: {:?}", word(&p));
        p.run(game::knobs::FriendsK::default().quiet_s + 0.1);   // (his own round was a fight: the gun does not drop for a friend in one)
        for _ in 0..30 { p.man_stands(men[0], [sam[0] + 2.0, sam[1]]); p.frame(); }
        p.man_stands(men[0], [sam[0] + 2.0, sam[1]]);
        p.press(Button::Left);
        p.frame();
        p.release(Button::Left);
        assert_eq!(word(&p), Some(format!("{name} is in your line of fire")), "seed {s}");
    }
}

/// THE RULE: a press's word stands beside rows he has, after them, where an empty tap's word stands down. It is the view's own row with its kind.
/// A FAILURE MEANS: a door in reach hides why the gun did nothing.
#[test]
fn a_press_keeps_its_word_beside_the_rows_he_has() {
    for s in SEEDS {
        let mut p = gun_room(s, [13.0 + 0.2 * nudge(s, 0), 15.3]);
        p.until(1.0, |p| !p.view().interact.rows.is_empty()).unwrap_or_else(|| panic!("seed {s}: the staging: no row at the door"));
        game::samstun::stagger_player(&mut p.g, 9.0, [1.0, 0.0, 0.0]);
        p.press(Button::Left);
        p.frame();
        p.release(Button::Left);
        let v = p.view();
        assert!(!v.interact.rows.is_empty(), "seed {s}: the staging: his rows went");
        let e = v.empty.unwrap_or_else(|| panic!("seed {s}: no word beside his rows"));
        assert_eq!((e.word.as_str(), e.kind, e.dist), ("you're still stunned", ThingKind::Wall, None), "seed {s}");
    }
}

/// The office, Sam out on the verge where nobody sees him.
fn office(seed: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.g.player_invisible = true;
    p.run(0.1);
    p
}

/// THE RULE: a code he overhears Sam says through his own bubble. With a man in his arm Sam says nothing, because his bubble would hold the next question back, and he has the code all the same.
/// CONTROL: out of earshot he learns nothing and says nothing.
/// A FAILURE MEANS: a code goes into his head with no word, or the line has moved a hold's pace.
#[test]
fn a_code_learnt_is_sams_own_line_and_not_with_a_man_in_his_arm() {
    for s in SEEDS {
        let mut p = office(s);
        p.sam_put([2.0, 2.0]);
        p.run(0.1);
        let m = p.g.civilians[0];
        assert!(!game::devices::devices_overhear_said(&mut p.g, m, "pad_side"), "seed {s}: the control: she is out of earshot");
        p.frame();
        assert_eq!(sams_line(&p), None, "seed {s}: the control");

        let at = p.at(m);
        p.sam_put([at[0] + 0.8, at[1]]);
        p.run(0.05);
        assert!(game::devices::devices_overhear_said(&mut p.g, m, "pad_side"), "seed {s}: beside her he hears");
        p.frame();
        assert_eq!(sams_line(&p).as_deref(), Some("got the side door code."), "seed {s}");
        drop(p);

        let mut p = office(s);
        let m = p.g.civilians[0];
        let at = p.at(m);
        p.sam_put([at[0] + 0.7, at[1]]);
        let me = p.g.player;
        p.g.pl[me].holstered = true;
        p.run(0.05);
        game::player::start_grab(&mut p.g, m, None);
        p.run(1.4);
        assert!(p.g.pl[me].holding().is_some(), "seed {s}: the staging: she is not in his arm");
        let before = sams_line(&p);
        assert!(game::devices::devices_overhear_said(&mut p.g, m, "pad_side"), "seed {s}");
        p.frame();
        assert!(p.g.pl[me].codes.has(String::from("pad_side")), "seed {s}: he has the code");
        assert_eq!(sams_line(&p), before, "seed {s}: Sam spoke with a man in his arm");
    }
}

/// The night breach of two on the office, nobody able to see Sam.
fn night(s: u32, squad: &str) -> Play {
    let mut p = Play::mission("office.json", &format!("?levelFile=office.json&nightbreach&squad={squad}"), &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.g.player_invisible = true;
    p.run(0.5);
    p
}
/// the word key, pressed
fn f(p: &mut Play) { p.key_down("KeyF"); p.frame(); p.key_up("KeyF"); p.frame(); }
fn sends_in(p: &mut Play, s: u32) {
    f(p);
    p.until(40.0, |p| game::nightbreach::night_breach_state(&p.g) == NightBreachState::Set).unwrap_or_else(|| panic!("seed {s}: never set: {}", p.roll()));
    f(p);
    assert!(game::nightbreach::night_breach_state(&p.g) != NightBreachState::Set, "seed {s}: no go");
}

/// THE RULE: a drill past calling off refuses the order, in the mouth of a man on it. CONTROL: before the word the same order is taken, with no such line.
/// A FAILURE MEANS: the order goes nowhere and nobody says so.
#[test]
fn an_order_the_drill_wont_let_go_is_said_by_a_man_on_it() {
    for s in SEEDS {
        let mut p = night(s, "2");
        let sam = p.sam_at();
        let men = p.men();
        assert_eq!(p.try_order("fallIn", sam, Who::All), None, "seed {s}: the control");
        p.frame();
        assert!(no_team_line(&p, "committed"), "seed {s}: the control");
        drop(p);

        let mut p = night(s, "2");
        let sam = p.sam_at();
        let men2 = p.men();
        let _ = men;
        sends_in(&mut p, s);
        p.until(10.0, |p| game::nightbreach::night_breach_state(&p.g) == NightBreachState::Going).unwrap_or_else(|| panic!("seed {s}: never going in"));
        let w = "we're committed. we're going in.";
        assert_eq!(p.try_order("fallIn", sam, Who::All).as_deref(), Some(w), "seed {s}");
        p.frame();
        assert!(said_by_one_of(&p, &men2, w), "seed {s}: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
    }
}

fn no_room_for_it(_: &mut game::game::Game, _: &[GuardId], _: &OrderTarget, _: Option<&game::orders::GiveOpts>) -> Option<String> { Some(String::from("can't hold it here.")) }
/// The yard's team, its two rooms' defenders off the floor.
fn yard_two(s: u32) -> Play {
    let (sc, _) = yard(s);
    let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
    p.sam_unhurt();
    p.g.player_invisible = true;
    p.run(0.3);
    let keep = p.men();
    let _ = &keep;
    p
}

/// THE RULE: a row's own issue that refuses is said by a man it was asked of, not by the seat's log. CONTROL: the row's own issue gives the order, with no such line.
/// A FAILURE MEANS: the order goes nowhere and nobody says so.
#[test]
fn an_order_its_row_refuses_is_said_by_a_man_asked() {
    for s in SEEDS {
        let mut p = yard_two(s);
        let (sam, men) = (p.sam_at(), p.men());
        assert_eq!(p.try_order("hold", sam, Who::All), None, "seed {s}: the control");
        p.frame();
        assert!(no_team_line(&p, "can't hold it here."), "seed {s}: the control");
        drop(p);

        let mut p = yard_two(s);
        let (sam, men2) = (p.sam_at(), p.men());
        let _ = men;
        let def = p.g.sets[p.g.seat].orders.defs.iter().copied().find(|&d| p.g.tab.orders_order_def[d].id == "hold").expect("the row");
        let row = game::orders::Row { issue: no_room_for_it, ..*p.g.tab.orders_order_def[def].row };
        p.g.tab.orders_order_def[def].row = Box::leak(Box::new(row));
        assert_eq!(p.try_order("hold", sam, Who::All).as_deref(), Some("can't hold it here."), "seed {s}");
        p.frame();
        assert!(said_by_one_of(&p, &men2, "can't hold it here."), "seed {s}");
    }
}

/// THE RULE: nobody there: the answer to a press falls to his own row, with no full stop. CONTROL: with a man there it is his line and no row. The goal's refusal goes through this, and `shift.rs` holds it said.
/// A FAILURE MEANS: alone, a refused press is answered by nothing.
#[test]
fn an_answer_with_nobody_there_is_his_own_row() {
    for s in SEEDS {
        let mut p = yard_two(s);
        let none = game::planmode::GOAL_NONE;
        game::interact::say_why(&mut p.g, None, ThingKind::Wall, none, none.trim_end_matches('.'));
        p.frame();
        assert!(p.said_it(none) && word(&p).is_none(), "seed {s}: the control: {:?}", word(&p));

        for m in p.men() { p.falls(m, DownBy::Guard); }
        game::interact::say_why(&mut p.g, None, ThingKind::Wall, none, none.trim_end_matches('.'));
        p.frame();
        assert_eq!(word(&p).as_deref(), Some("nobody's free for that"), "seed {s}");
    }
}

/// A silent squad's man says it on the net, with his hand at the radio, as any line of his does.
#[test]
fn a_silent_squad_says_it_on_the_net() {
    for s in SEEDS {
        let mut p = yard_two(s);
        let sam = p.sam_at();
        let m = p.men()[0];
        assert!(game::squadnet::team_says(&mut p.g, Some(m), "loud words."), "seed {s}");
        p.frame();
        assert!(p.said.iter().any(|l| l.text == "loud words." && l.who == m && !l.radio), "seed {s}: the control: out loud");
        p.run(4.0);
        p.order("silent", sam, Who::All);
        p.run(0.5);
        assert!(game::squadnet::team_says(&mut p.g, Some(m), "quiet words."), "seed {s}");
        p.frame();
        assert!(p.said.iter().any(|l| l.text == "quiet words." && l.who == m && l.radio), "seed {s}: not on the net");
    }
}

/// The rescue on the office, nobody roaming.
fn rescue(s: u32, squad: u32, set: f64) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&rescue", &Opts { seed: s, squad, ..Default::default() });
    p.sam_unhurt();
    p.rescue_quiet_floor();
    p.rescue_on(99.0, 99.0, set);
    p.run(0.3);
    p
}
fn hostage_door(p: &Play) -> DoorId { let d = p.g.level.scenarios.as_ref().and_then(|s| s.rescue.clone()).expect("the rescue").room.door; p.door(&d) }

/// THE RULE: F on the hostage room with a charge going on, or men already at the door, is answered by the man at it.
/// A FAILURE MEANS: F does nothing and nothing says why.
#[test]
fn f_at_the_hostage_room_with_men_on_its_door_is_answered_by_them() {
    for s in SEEDS {
        let mut p = rescue(s, 2, 0.0);
        let d = hostage_door(&p);
        let sam = p.sam_at();
        assert_eq!(p.order_on_face("charge", d, sam), None, "seed {s}: the staging");
        let men = p.op_men(p.last_op(d).expect("the order"));
        p.frame();
        p.tap("KeyF");
        assert!(said_by_one_of(&p, &men, "charge isn't set yet."), "seed {s}: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
        drop(p);

        let mut p = rescue(s, 2, 0.0);
        let d = hostage_door(&p);
        let sam = p.sam_at();
        assert_eq!(p.order_on_face("bangClear", d, sam), None, "seed {s}: the staging");
        let men = p.op_men(p.last_op(d).expect("the order"));
        p.frame();
        p.tap("KeyF");
        assert!(said_by_one_of(&p, &men, "we're on that door already."), "seed {s}: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
    }
}

/// THE RULE: F with every man holding what he was given is said by one of them; with nobody up it is the player's own row. CONTROL: F with free men sends them, with neither.
/// A FAILURE MEANS: F does nothing and nothing says why.
#[test]
fn f_at_the_hostage_room_with_nobody_free_says_so() {
    let (holding, alone) = ("we're all holding what you gave us. call us in first.", "you'll have to take the room yourself");
    for s in SEEDS {
        let mut p = rescue(s, 2, 30.0);
        p.tap("KeyF");
        assert!(no_team_line(&p, holding) && word(&p).as_deref() != Some(alone), "seed {s}: the control");
        drop(p);

        let mut p = rescue(s, 2, 30.0);
        let (sam, men) = (p.sam_at(), p.men());
        for (m, off) in men.iter().zip([-2.0, 2.0]) { assert_eq!(p.order_men("cover", [sam[0] + off, sam[1]], &[*m]), None, "seed {s}: the staging"); }
        p.run(0.5);
        p.tap("KeyF");
        assert!(said_by_one_of(&p, &men, holding), "seed {s}: {:?} word {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>(), word(&p));
        drop(p);

        let mut p = rescue(s, 2, 30.0);
        for m in p.men() { p.falls(m, DownBy::Guard); }
        p.frame();
        p.tap("KeyF");
        assert_eq!(word(&p).as_deref(), Some(alone), "seed {s}");
    }
}

/// THE RULE: an entry that goes without its can is said by a man going in: none left, or (two doors) one left and which door it goes in. CONTROL: with the cans there is no such line.
/// A FAILURE MEANS: the player finds out from the man walking in.
#[test]
fn an_entry_short_of_cans_is_said_by_a_man_going_in() {
    let none = "no bangs left. going in without one.";
    for s in SEEDS {
        let mut p = rescue(s, 2, 30.0);
        p.tap("KeyF");
        assert!(no_team_line(&p, none), "seed {s}: the control");
        drop(p);

        let mut p = rescue(s, 2, 30.0);
        let men = p.men();
        p.g.sets[p.g.seat].door_tools.bangs = 0.0;
        p.tap("KeyF");
        p.frame();
        assert!(said_by_one_of(&p, &men, none), "seed {s}: the rescue: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
        drop(p);

        let mut p = rescue(s, 2, 30.0);
        let men = p.men();
        p.g.sets[p.g.seat].door_tools.bangs = 1.0;
        let b = game::rescue::rescue_from_bounds(&p.g).expect("the staging: the side door's room");   // (Sam in the side door's room: one man goes round by it)
        let mid = [(b.x0 + b.x1) / 2.0, (b.z0 + b.z1) / 2.0];
        p.sam_put(mid);
        for &m in &men { p.man_stands(m, mid); }
        p.frame();
        f(&mut p);
        p.run(1.5);   // (the man's "on you." came first in that frame, and this waits its turn on his bubble)
        assert!(p.said.iter().any(|l| men.contains(&l.who) && l.text.starts_with("one bang left. it's going in the ") && l.text.ends_with(" door.")), "seed {s}: two doors, one can: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
        drop(p);

        let mut p = night(s, "2");
        let men = p.men();
        p.g.sets[p.g.seat].door_tools.bangs = 0.0;
        sends_in(&mut p, s);
        p.frame();
        assert!(said_by_one_of(&p, &men, none), "seed {s}: the breach: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
    }
}

/// THE RULE: the point man down at the door: a living man names him and hands the room to the player, once. CONTROL: with him up nobody says so.
/// A FAILURE MEANS: the drill ends and the player is left to guess whose job it is now.
#[test]
fn a_failed_drill_is_said_by_a_living_man() {
    for s in SEEDS {
        for (q, how) in [("?levelFile=office.json&nightbreach&squad=2", "night"), ("?levelFile=office.json&roomclear&squad=2", "room clear")] {
            let mut p = Play::mission("office.json", q, &Opts { seed: s, ..Default::default() });
            p.sam_unhurt();
            p.g.player_invisible = true;
            p.run(0.5);
            f(&mut p);
            p.run(1.0);
            assert!(no_team_line(&p, "down at the door"), "seed {s}, {how}: the control");
            let ops = game::nightbreach::night_breach_drill_ops(&p.g);
            let name = p.g.gd[ops[0]].callsign.clone();
            p.falls(ops[0], DownBy::Guard);
            let w = format!("{name}'s down at the door. it's on you.");
            p.until(8.0, |p| p.said_it(&w)).unwrap_or_else(|| panic!("seed {s}, {how}: {:?} state {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>(), game::nightbreach::night_breach_state(&p.g)));
            assert_eq!(p.said.iter().filter(|l| l.text == w).count(), 1, "seed {s}, {how}: once");
            assert!(p.lines(ops[1]).contains(&w.as_str()), "seed {s}, {how}: by the living man");
        }
    }
}

/// THE RULE: a drill called off from inside is said by a living man, once.
/// A FAILURE MEANS: the men stop and nothing says the job moved to the player.
#[test]
fn a_recalled_drill_is_said_by_a_living_man() {
    let w = "calling it off. we're with you.";
    for s in SEEDS {
        let mut p = night(s, "2");
        let sam = p.sam_at();
        sends_in(&mut p, s);
        assert!(no_team_line(&p, w), "seed {s}: the control");
        p.until(40.0, |p| game::nightbreach::night_breach_state(&p.g) == NightBreachState::Inside).unwrap_or_else(|| panic!("seed {s}: never inside: {}", p.roll()));
        let _ = p.try_order("fallIn", sam, Who::All);
        p.until(3.0, |p| p.said_it(w)).unwrap_or_else(|| panic!("seed {s}: {:?} state {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>(), game::nightbreach::night_breach_state(&p.g)));
    }
}

/// THE RULE: a set charge whose door moves is said by the man who set it; Sam's own (no press of the blow is left to answer) by the nearest; with nobody up, by nobody. CONTROL: while the door stays shut nobody says so.
/// A FAILURE MEANS: a charge drops off a door and the player still thinks he has one.
#[test]
fn a_charge_off_its_door_is_said_by_who_set_it() {
    let w = "that door came open. charge is off.";
    for s in SEEDS {
        let (sc, at) = yard(s);
        let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let d = p.door("west-yard");
        p.order("charge", at, Who::All);
        p.until(45.0, |p| p.last_op(d).is_some_and(|o| p.op_phase(o) == game::doortools::Phase::Armed)).unwrap_or_else(|| panic!("seed {s}: never armed: {}", p.roll()));
        let lead = p.op_lead(p.last_op(d).unwrap());
        p.run(0.2);
        assert!(no_team_line(&p, w), "seed {s}: the control");
        p.leaf_put_open(d, 1.2);
        p.run(0.2);
        assert!(p.lines(lead).contains(&w), "seed {s}: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
        drop(p);

        let (sc, _) = yard(s);
        let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let d = p.door("west-yard");
        let c = p.g.tab.doortools_set_charge.push(game::doortools::SetCharge { door: d, side: 1.0, by: String::from(game::ownblow::SAM_BY) });
        p.g.sets[p.g.seat].door_tools.set.push(c);
        p.run(0.1);
        let men = p.men();
        let at = p.sam_at();
        let near = *men.iter().min_by(|a, b| game_start::play::dist(p.at(**a), at).total_cmp(&game_start::play::dist(p.at(**b), at))).unwrap();
        p.leaf_put_open(d, 1.2);
        p.frame();
        assert!(p.lines(near).contains(&w), "seed {s}: Sam's own: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
        drop(p);

        let (sc, _) = yard(s);
        let mut p = Play::scene(&sc, &Opts { seed: s, squad: 2, ..Default::default() });
        p.sam_unhurt();
        p.run(0.2);
        let d = p.door("west-yard");
        let c = p.g.tab.doortools_set_charge.push(game::doortools::SetCharge { door: d, side: 1.0, by: String::from(game::ownblow::SAM_BY) });
        p.g.sets[p.g.seat].door_tools.set.push(c);
        for m in p.men() { p.falls(m, DownBy::Guard); }
        p.frame();
        p.leaf_put_open(d, 1.2);
        p.frame();
        assert!(no_team_line(&p, w) && word(&p).is_none(), "seed {s}: alone: nobody to say it and no press to answer");
    }
}

/// THE RULE: the wall key with no wall says why: open floor, a wall he can't get to, hands full, and a remembered Q that runs out. CONTROL: Q at a wall takes it and says nothing.
/// A FAILURE MEANS: Q does nothing and nothing says why.
#[test]
fn the_wall_key_with_no_wall_says_why() {
    let mut p = office(1);
    p.sam_put([20.0, 10.7]);
    p.run(0.1);
    p.tap("KeyQ");
    p.run(0.3);
    assert!(p.g.pl[p.g.player].wall.is_some() && word(&p).is_none(), "the control: {:?}", word(&p));
    p.tap("KeyQ");
    p.run(EMPTY_SHOW_SECS + 0.2);

    p.sam_put([2.0, 2.0]);
    p.run(0.1);
    p.tap("KeyQ");
    assert_eq!(word(&p).as_deref(), Some("no wall to take here"));
    p.run(EMPTY_SHOW_SECS + 0.2);
    assert_eq!(word(&p), None, "gone with its time");

    // a remembered Q, on the walk: it says so when it runs out, and a stride is still his
    p.key_down("KeyW");
    p.run(0.3);
    p.tap("KeyQ");
    p.until(1.0, |p| word(p).is_some()).expect("a remembered Q ran out and said nothing");
    p.key_up("KeyW");
    assert_eq!(word(&p).as_deref(), Some("no wall to take here"));
    p.run(EMPTY_SHOW_SECS + 0.2);

    // a wall with furniture against it: somewhere in the office Q finds the wall and can't get to it
    let mut found = None;
    'floor: for x in 4..76 { for z in 4..56 {
        let at = [x as f64 * 0.5, z as f64 * 0.5];
        p.sam_put(at);
        p.frame();
        p.tap("KeyQ");
        if word(&p).as_deref() == Some("can't get to that wall") { found = Some(at); break 'floor; }
        if p.g.pl[p.g.player].wall.is_some() { p.tap("KeyQ"); }
    } }
    assert!(found.is_some(), "nowhere on the office's floor is a wall out of reach for what stands against it");
    p.run(EMPTY_SHOW_SECS + 0.2);

    let m = p.g.guards[0];
    p.g.pl[p.g.player].holstered = true;
    p.falls_as(m, DownKind::Struck, true, DownBy::Player);
    p.run(0.1);
    let at = p.at(m);
    p.sam_put([at[0] + 0.5, at[1]]);
    p.run(0.1);
    game::game::game_toggle_drag(&mut p.g, Some(m));
    p.run(0.2);
    assert!(p.g.pl[p.g.player].dragging().is_some(), "the staging: he is not dragging");
    p.tap("KeyQ");
    assert_eq!(word(&p).as_deref(), Some("your hands are full"));
}

/// The quiet mission with a partner on the bazaar, a civilian's death no failure.
fn quiet_with_a_partner(s: u32) -> Play {
    let mut p = Play::mission("bazaar.json", "?levelFile=bazaar.json&quiet&squad=1", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.g.player_invisible = true;
    p.g.tune.civilian_kill_fails = false;
    p.run(0.5);
    p
}

/// THE RULE: a civilian Sam puts down is said by a teammate, dead or breathing. CONTROLS: down by somebody else: no such line; alone: nothing, and no row, for nothing was pressed.
/// A FAILURE MEANS: the player goes on as if nothing happened to her until the end card.
#[test]
fn a_civilian_put_down_is_said_by_a_teammate() {
    for s in SEEDS {
        let mut p = quiet_with_a_partner(s);
        let m = p.g.civilians[0];
        p.falls_as(m, DownKind::Shot, true, DownBy::Guard);
        p.frame();
        assert!(no_team_line(&p, "civilian"), "seed {s}: the control: another's hand");
        drop(p);

        let mut p = quiet_with_a_partner(s);
        let m = p.g.civilians[0];
        let men = p.men();
        p.falls_as(m, DownKind::Shot, true, DownBy::Player);
        p.frame();
        assert!(said_by_one_of(&p, &men, "that was a civilian."), "seed {s}: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
        drop(p);

        let mut p = quiet_with_a_partner(s);
        let m = p.g.civilians[0];
        let men = p.men();
        let _cid = p.g.gd[m].civ.unwrap();
        p.falls_as(m, DownKind::Struck, true, DownBy::Player);
        p.frame();
        assert!(said_by_one_of(&p, &men, "civilian's out. still breathing."), "seed {s}: {:?}", p.said.iter().map(|l| l.text.as_str()).collect::<Vec<_>>());
        drop(p);

        let mut p = office(s);
        p.g.tune.civilian_kill_fails = false;
        let m = p.g.civilians[0];
        p.falls_as(m, DownKind::Struck, true, DownBy::Player);
        p.frame();
        assert!(no_team_line(&p, "civilian") && word(&p).is_none(), "seed {s}: alone");
        assert_eq!(p.state(m), GuardState::Dead, "seed {s}: the staging");
    }
}

/// THE RULE: on a wall, a slot key and the hand's key change what comes out when he leaves it, and say so. CONTROL: off the wall the same key draws, and leaves no word.
/// A FAILURE MEANS: the slot goes up and nothing comes to his hand and nothing says why.
#[test]
fn a_slot_key_on_a_wall_says_what_comes_out_when_he_leaves() {
    for s in SEEDS {
        let mut p = office(s);
        let me = p.g.player;
        p.sam_put([2.0, 2.0]);
        p.run(0.1);
        p.tap("Digit1");
        assert_eq!(word(&p), None, "seed {s}: the control: off the wall");

        p.g.pl[me].holstered = true;
        p.sam_put([20.0, 10.7]);
        p.run(0.1);
        p.tap("KeyQ");
        p.run(0.6);
        assert!(p.g.pl[me].wall.is_some(), "seed {s}: the staging: no wall");
        p.run(EMPTY_SHOW_SECS + 0.2);
        p.tap("Digit1");
        let w = word(&p).unwrap_or_else(|| panic!("seed {s}: no word"));
        assert!(w.ends_with(" comes out when you leave the wall"), "seed {s}: {w}");
        p.tap("KeyE");
        assert_eq!(word(&p).as_deref(), Some("it'll stay holstered when you leave the wall"), "seed {s}");
        p.tap("KeyE");
        assert!(word(&p).is_some_and(|w| w.ends_with(" comes out when you leave the wall")), "seed {s}");
    }
}

/// THE RULE: the pair run for the door and go over to stacking on it once the lead is within 3.2 m of his mark, on the floor: however high the mark is written and whichever way he comes at it.
/// A FAILURE MEANS: the pair stop to stack in the wrong room, or run on until the drill's clock gives up, and the go comes long after the player gave it.
#[test]
fn the_pair_stack_when_the_lead_is_near_his_mark_on_the_floor() {
    for s in SEEDS {
        for high in [0.0, 3.0] {
            let mut p = night(s, "2");
            let drill = |p: &Play| p.g.night_breach.drill.as_ref().expect("the drill").clone();
            let d = drill(&p);
            p.g.night_breach.drill.as_mut().unwrap().plan.stack[0][1] += high;
            let (lead, mark) = (p.g.gd[d.a].ch, d.plan.stack[0]);
            let far = |p: &Play| game::v3::dist_xz(p.g.ch[lead].pos, mark);
            let start = far(&p);
            assert!(start > 3.2, "seed {s}: he starts away from his mark: {start}");
            f(&mut p);
            let t0 = p.time();
            let phase = |p: &Play| p.g.night_breach.drill.as_ref().unwrap().phase;
            let mut last = far(&p);
            while phase(&p) == game::breach::Phase::Wait || phase(&p) == game::breach::Phase::Approach {
                assert!(p.time() - t0 < 30.0, "seed {s}: no stack");
                last = far(&p);
                p.frame();
            }
            assert!(far(&p) < 3.2, "seed {s}, mark {high} m up: stacks {} m from his mark", far(&p));
            assert!(last >= 3.0, "seed {s}, mark {high} m up: he was already {last} m off a frame before");
            assert!(p.time() - t0 < 15.0, "seed {s}, mark {high} m up: only the clock let them stack");
        }
    }
}
