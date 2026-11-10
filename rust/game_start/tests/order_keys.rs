//! THE ORDERS THEMSELVES: the keys that give them, and the men going and staying where they say. The maintainer: "it's hard to hit numbers bigger than 5 when left hand is on wasd".
mod doors_ask;
use doors_ask::*;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, Opts, Play};

fn opts(seed: u32, squad: u32) -> Opts { Opts { seed, squad, ..Default::default() } }
fn stood(seed: u32, squad: u32) -> (Play, DoorId, [f64; 2]) {
    let (s, at) = yard(seed);
    let mut p = Play::scene(&s, &opts(seed, squad));
    p.sam_unhurt();
    p.run(0.2);
    let d = p.door("west-yard");
    (p, d, at)
}
fn hush(p: &mut Play) { let keep = p.defender_at([24.25, 12.25]); p.drop_all_but(&[keep]); p.pin_man(keep, [24.25, 12.25], 0.0, game::game::GuardState::Patrol, 0.0); }

/// the keys above the left hand's reach
const HIGH: [&str; 5] = ["Digit0", "Digit6", "Digit7", "Digit8", "Digit9"];
fn nothing_from(p: &mut Play, what: &str, s: u32) {
    let (menu, given, slot) = (p.menu_up(), p.orders_given(), p.sam_slot());
    for c in HIGH {
        p.tap(c);
        assert_eq!(p.menu_up(), menu, "seed {s}: {c} with {what} up moved the menu");
        assert_eq!((p.orders_given(), p.any_op()), (given, false), "seed {s}: {c} with {what} up gave an order");
        assert_eq!(p.sam_slot(), slot, "seed {s}: {c} with {what} up changed what is in Sam's hands");
    }
}

/// THE RULE: "it's hard to hit numbers bigger than 5 when left hand is on wasd". The digits above 5 and 0 give nothing with any page of any menu up or with none. The control is a digit in reach on the same page: it gives its order.
/// A FAILURE MEANS: an order has gone onto a digit the left hand cannot reach, or those keys have been handed something else.
#[test]
fn a_digit_above_five_gives_nothing_with_any_page_up() {
    for s in SEEDS {
        for squad in [2, 4] {
            let (mut p, _, at) = stood(s, squad);
            hush(&mut p);
            nothing_from(&mut p, "no menu", s);
            p.menu_opens([at[0] + 4.0, at[1] + 6.0], None);
            nothing_from(&mut p, "the floor's menu", s);
            p.tap("Escape");
            p.menu_opens(p.sam_at(), None);
            nothing_from(&mut p, "Sam's menu", s);
            p.tap("Escape");
            p.menu_opens(at, None);
            assert!(p.menu_up().is_some(), "seed {s}: the staging");
            nothing_from(&mut p, "a door's menu", s);
            p.tap("Escape");
            p.menu_opens(at, Some(game::doortools::TOOLS_PAGE));
            assert_eq!(p.menu_up().unwrap().0.as_deref(), Some(game::doortools::TOOLS_PAGE), "seed {s}: the staging");
            nothing_from(&mut p, "a door's tools page", s);
            let key = p.row_key("openClear").expect("the tools page has its row");
            assert!(key <= 5.0, "seed {s}: the row sits on {key}");
            p.tap(&format!("Digit{key}"));
            assert!(p.any_op() || p.menu_up().is_none(), "seed {s}: the control: a digit in reach gives its order");
            assert!(p.orders_given() > 0, "seed {s}: the control: a digit in reach gives its order");
        }
    }
}

/// THE RULE: move there puts the men on marks of their own round the point, a body's room apart, gun on what the room dealt them; 'set.' is called once; six seconds on they have not shifted. The control is the pair not told: it does not go.
/// A FAILURE MEANS: a move walks them to one spot, or they drift off it, or the order reaches men it was not given to.
#[test]
fn move_there_puts_the_pair_on_the_spot_and_keeps_them_there() {
    for s in SEEDS {
        let (mut p, _, at) = stood(s, 4);
        hush(&mut p);
        let (red, blue) = (p.red(), p.blue());
        p.run(0.5);
        let was: Vec<_> = blue.iter().map(|&m| p.at(m)).collect();
        let spot = [at[0] + 6.0, at[1] + 2.0];
        p.order("move", spot, Who::Red);
        p.run(0.33);
        for &m in &red { assert!(p.speed(m) > 0.1, "seed {s}: he steps off at once"); }
        p.until(12.0, |p| red.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: not there in 12 s: {}", p.roll()));
        p.run(1.0);
        for &m in &red {
            assert!(dist(p.at(m), p.post_of(m).unwrap()) < 0.35 && dist(p.at(m), spot) < 2.55, "seed {s}: on his own mark round the point: {:?}", p.at(m));
            let w = p.watch_of(m).expect("the room dealt him something to watch");
            assert!(off_by(p.gun_yaw(m), yaw_to(p.at(m), w)) < 0.3, "seed {s}: his gun is on it");
        }
        assert!(dist(p.at(red[0]), p.at(red[1])) > 0.8, "seed {s}: a body's room apart");
        assert_eq!(p.said.iter().filter(|l| l.text == "set.").count(), 1, "seed {s}: 'set.' once");
        for (i, &m) in blue.iter().enumerate() { assert!(dist(p.at(m), was[i]) < 1.5, "seed {s}: the control: blue was not told"); }
        let here: Vec<_> = red.iter().map(|&m| p.at(m)).collect();
        p.run(3.0);
        for (i, &m) in red.iter().enumerate() { assert!(dist(p.at(m), here[i]) < 0.1, "seed {s}: three seconds on he has shifted"); }
        assert_eq!(p.said.iter().filter(|l| l.text == "set.").count(), 1, "seed {s}: and nobody said it again");
    }
}

/// THE RULE: hold, given while they run to a far spot, stops both within a stride of where they were, and they stay. The control is the same run with no hold: they get there.
/// A FAILURE MEANS: a hold finishes the walk it was given to stop.
#[test]
fn hold_stops_them_where_they_are() {
    for s in SEEDS {
        for held in [true, false] {
            let (mut p, _, at) = stood(s, 2);
            hush(&mut p);
            let men = p.men();
            p.run(0.5);
            let far = [at[0] + 14.0, at[1] + 6.0];
            p.order("move", far, Who::All);
            p.run(0.9);
            for &m in &men { assert!(p.speed(m) > 0.5, "seed {s}: he is on his way"); }
            if !held { p.until(14.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: the control: not there")); continue; }
            p.order("hold", p.sam_at(), Who::All);
            let here: Vec<_> = men.iter().map(|&m| p.at(m)).collect();
            p.run(4.0);
            for (i, &m) in men.iter().enumerate() {
                assert!(dist(p.at(m), here[i]) < 0.6 && p.speed(m) < 0.1, "seed {s}: he went on {:.2} m", dist(p.at(m), here[i]));
                assert!(dist(p.at(m), far) > 1.5, "seed {s}: he reached the far spot");
            }
        }
    }
}

/// THE RULE: cover, on a place or on a door: each man turns his gun onto it where he stands, and eight seconds on he is still on it, gun and body. The control is the turn itself: before the order he is well off it.
/// A FAILURE MEANS: a cover walks him, or he turns away by himself.
#[test]
fn cover_turns_a_man_onto_it_where_he_stands_and_he_stays_on_it() {
    for s in SEEDS {
        let (mut p, d, at) = stood(s, 2);
        hush(&mut p);
        let men = p.men();
        p.order("move", [at[0] + 4.0, at[1] + 5.0], Who::All);
        p.until(12.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: not there"));
        p.run(0.5);
        for target in [[at[0] + 13.0, at[1] + 8.0], p.frame_at(d)] {
            for &m in &men { if off_by(p.gun_yaw(m), yaw_to(p.at(m), target)) < 0.5 { let away = [p.at(m)[0] + 3.0 * (at[0] - target[0]).signum(), p.at(m)[1]]; p.order("cover", away, Who::All); p.run(1.5); } }
            let here: Vec<_> = men.iter().map(|&m| p.at(m)).collect();
            for &m in &men { assert!(off_by(p.gun_yaw(m), yaw_to(p.at(m), target)) > 0.3, "seed {s}: the staging: he is off it before the order"); }
            p.order("cover", target, Who::All);
            p.until(4.0, |p| men.iter().all(|&m| p.arrived(m) && off_by(p.gun_yaw(m), yaw_to(p.at(m), target)) < 0.2)).unwrap_or_else(|| panic!("seed {s}: he did not turn onto {target:?}"));
            let mut worst = 0.0f64;
            for _ in 0..480 { p.frame(); for &m in &men { worst = worst.max(off_by(p.gun_yaw(m), yaw_to(p.at(m), target))); } }
            assert!(worst < 0.2, "seed {s}: he came {worst:.2} rad off it");
            for (i, &m) in men.iter().enumerate() {
                assert!(dist(p.at(m), here[i]) < 0.3, "seed {s}: he moved {:.2} m", dist(p.at(m), here[i]));
                assert!(off_by(p.facing(m), yaw_to(p.at(m), target)) < 0.4, "seed {s}: his body is on it too");
            }
        }
    }
}

/// THE RULE: fall in closes them up behind Sam, and Sam turning on the spot moves nobody. The control is Sam walking off: they go with him.
/// A FAILURE MEANS: a file shuffles with the player's cursor.
#[test]
fn fall_in_closes_up_and_sam_turning_on_the_spot_moves_nobody() {
    for s in SEEDS {
        let (mut p, _, at) = stood(s, 4);
        hush(&mut p);
        let men = p.men();
        p.order("move", [at[0] + 8.0, at[1] + 7.0], Who::All);
        p.until(12.0, |p| men.iter().all(|&m| p.arrived(m)));
        for &m in &men { assert!(dist(p.at(m), p.sam_at()) > 5.0, "seed {s}: the staging: he is away from Sam"); }
        p.order("fallIn", p.sam_at(), Who::All);
        p.until(14.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {s}: they never closed up: {}", p.roll()));
        p.run(1.5);
        for &m in &men { assert!(dist(p.at(m), p.sam_at()) < 5.8, "seed {s}: closed up"); }
        let here: Vec<_> = men.iter().map(|&m| p.at(m)).collect();
        let sam = p.sam_at();
        for k in 0..16 {
            let a = k as f64 * 0.8;
            p.point_at([sam[0] + 4.0 * a.sin(), sam[1] + 4.0 * a.cos()]);
            p.run(0.25);
        }
        for (i, &m) in men.iter().enumerate() { assert!(dist(p.at(m), here[i]) < 0.3, "seed {s}: Sam turned on the spot and he moved {:.2} m", dist(p.at(m), here[i])); }
        // the control: Sam walks off, and they go with him
        p.sam_put([sam[0] + 2.0, sam[1] + 1.0]);
        p.run(0.2);
        for k in 1..=120 { let t = k as f64 / 120.0; p.sam_put([sam[0] + 2.0 + 4.0 * t, sam[1] + 1.0 + 2.0 * t]); p.frame(); }
        p.run(3.0);
        for &m in &men { assert!(dist(p.at(m), p.sam_at()) < 5.8, "seed {s}: the control: they went with him"); }
    }
}

// ---------------------------------------------------------------- the key on a stack, a man back from the fight, a choice on a pair, the word's entry

/// the four stacked and set on the west door, the floor quiet
fn set_on_door(seed: u32) -> (Play, DoorId, [f64; 2]) {
    let (mut p, d, at) = stood(seed, 4);
    hush(&mut p);
    let men = p.men();
    p.order("stack", at, Who::All);
    p.until(25.0, |p| men.iter().all(|&m| p.arrived(m))).unwrap_or_else(|| panic!("seed {seed}: never set: {}", p.roll()));
    p.run(1.5);
    (p, d, at)
}
fn chest(at: [f64; 2]) -> [f64; 3] { [at[0], 1.2, at[1]] }

/// THE RULE: "tapping Q on a door they're stacked up on would just resend the stack up command, which would then have them say ('stacked up' or 'we're ready to go' ...)". The key on that door: one man answers, no order is written, no door is worked, nobody leaves his mark; the key again: another man, another line. The control is two of the four taken off the stack: then it is given.
/// A FAILURE MEANS: a second tap goes in, or the squad shuffles for an order it has already carried out.
#[test]
fn the_key_on_a_door_they_are_stacked_on_is_answered_and_moves_nobody() {
    for s in SEEDS {
        let (mut p, d, at) = set_on_door(s);
        let men = p.men();
        let mut heard: Vec<(GuardId, String)> = Vec::new();
        for _ in 0..2 {
            let (given, at0, said) = (p.orders_given(), men.iter().map(|&m| p.at(m)).collect::<Vec<_>>(), p.said.len());
            p.key_tapped_on(chest(at));
            p.run(1.0);
            let new: Vec<_> = p.said[said..].iter().filter(|l| men.contains(&l.who)).map(|l| (l.who, l.text.clone())).collect();
            assert_eq!(new.len(), 1, "seed {s}: one man answers: {new:?}");
            assert_eq!(p.orders_given(), given, "seed {s}: no order is written");
            assert!(!p.any_op(), "seed {s}: no door is worked");
            for (i, &m) in men.iter().enumerate() { assert!(dist(p.at(m), at0[i]) < 0.05 && p.arrived(m), "seed {s}: he left his mark"); }
            heard.push(new[0].clone());
        }
        assert!(heard[0].0 != heard[1].0 && heard[0].1 != heard[1].1, "seed {s}: another man, another line: {heard:?}");
        assert!(p.door_shut(d), "seed {s}: the door was not touched");
        // two of them off the stack: it is given again, to all four
        let two = [men[2], men[3]];
        p.order("fallIn", p.sam_at(), Who::Num(2.0));
        p.order("fallIn", p.sam_at(), Who::Num(3.0));
        p.run(2.0);
        for &m in &two { assert_eq!(p.doing(m).as_deref(), Some("fallIn"), "seed {s}: the staging"); }
        let given = p.orders_given();
        p.key_tapped_on(chest(at));
        assert_eq!(p.orders_given(), given + 1, "seed {s}: the control: it is given");
        assert_eq!(p.last_order().map(|o| (o.0, o.1.len())), Some(("stack".to_string(), 4)), "seed {s}: the control: to all four");
    }
}

/// THE RULE: "let's also remove the Q vs. F for deciding how to go through a room. F always just does a basic clear". Stacked, the word's key is open and clear by all four, no stun spent. The control: bang and clear, given as the ring's row gives it, starts that entry.
/// A FAILURE MEANS: the key picks a tool.
#[test]
fn the_word_is_always_a_plain_entry_and_a_tool_is_asked_for() {
    for s in SEEDS {
        let (mut p, d, _) = set_on_door(s);
        let (bangs, men) = (p.left().1, p.men().len());
        p.tap("KeyF");
        let op = *p.ops_on(d).first().unwrap_or_else(|| panic!("seed {s}: nothing started on the word"));
        assert!(!p.op_is_can(op) && p.op_men(op).len() == men, "seed {s}: {:?}", p.op_beats(op));
        p.run(4.0);
        assert_eq!(p.left().1, bangs, "seed {s}: no stun spent");
        drop(p);

        let (mut p, d, at) = set_on_door(s);
        p.order("bangClear", at, Who::All);
        p.run(0.1);
        let op = *p.ops_on(d).first().unwrap_or_else(|| panic!("seed {s}: the control: nothing started"));
        assert!(p.op_is_can(op), "seed {s}: the control: {:?}", p.op_beats(op));
    }
}

/// THE RULE: a man back from the fight walks for where his order stands. With a corner left in his way from before, and the way now straight, he ends on his place and not on the corner. The control is the corner itself: the staging's walk would have ended there.
/// A FAILURE MEANS: he stands on the old corner and says he could not get through.
#[test]
fn a_man_back_from_the_fight_walks_for_where_his_order_stands() {
    for s in SEEDS {
        let (mut p, _, _at) = stood(s, 4);
        hush(&mut p);
        let men = p.men();
        let sam = p.sam_at();
        let (spot, corner) = ([sam[0] + 5.0, sam[1] - 0.5], [sam[0] - 6.0, sam[1] - 0.5]);
        p.order("move", spot, Who::All);
        p.run(2.0 / 60.0);
        let posts: Vec<_> = men.iter().map(|&m| p.post_of(m).expect("a place")).collect();
        for &m in &men { p.corner_in_his_way(m, corner); }
        for &m in &men { p.fight_has(m); }
        p.frame();
        for &m in &men { assert!(p.order_waits_on_fight(m), "seed {s}: the staging: the fight has him"); }
        p.until(8.0, |p| men.iter().all(|&m| !p.order_waits_on_fight(m))).unwrap_or_else(|| panic!("seed {s}: the fight never let go"));
        p.until(12.0, |p| men.iter().enumerate().all(|(i, &m)| dist(p.at(m), posts[i]) < 1.0));
        p.run(1.0);
        for (i, &m) in men.iter().enumerate() {
            assert!(dist(p.at(m), posts[i]) < 1.2, "seed {s}: {:.2} m off his place", dist(p.at(m), posts[i]));
            assert!(dist(p.at(m), corner) > 1.5, "seed {s}: he stands on the corner");
        }
    }
}

/// THE RULE: "quick Q mode should always do the whole squad". A choice left on a pair is everyone's again on the next frame and the key goes to all four; the key with the cursor on a teammate is not his alone. The control is an order given to one pair by the menu's own part: that one goes to two.
/// A FAILURE MEANS: the player's quick key goes to half his men, and he cannot see why.
#[test]
fn the_quick_key_is_the_whole_squads() {
    for s in SEEDS {
        let (mut p, _, at) = stood(s, 4);
        hush(&mut p);
        let men = p.men();
        p.choice_on(Who::Red);
        p.run(2.0 / 60.0);
        let floor = [at[0] + 4.0, at[1] + 4.0];
        p.key_tapped_on([floor[0], 0.04, floor[1]]);
        assert_eq!(p.last_order().map(|o| o.1.len()), Some(4), "seed {s}: a choice on a pair: {:?}", p.last_order());
        p.run(0.5);
        let a = p.at(men[1]);
        p.key_tapped_on(chest(a));
        assert_eq!(p.last_order().map(|o| o.1.len()), Some(4), "seed {s}: the cursor on a man: {:?}", p.last_order());
        p.order("hold", p.sam_at(), Who::Red);
        assert_eq!(p.last_order().map(|o| o.1.len()), Some(2), "seed {s}: the control: {:?}", p.last_order());
    }
}
