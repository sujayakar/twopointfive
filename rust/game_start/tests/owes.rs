//! WHAT A WHOLE RUN OWES (`game_start::owes`): held on a clear played to its end, and each kind of fault made to happen.
use game::scene_scenario_list::{all_missions, pick_search};
use game_start::hands_off::{HandsOff, Plan};
use game_start::lessons::{tutorials, Lesson};
use game_start::owes::{self, Owes};
use game_start::play::{Opts, Play};

fn stand(id: &str) -> Play {
    let e = all_missions().into_iter().find(|e| e.id == id).unwrap();
    Play::mission(e.level.as_deref().unwrap_or("office.json"), &pick_search(&e, ""), &Opts { seed: 9, dress: true, ..Default::default() })
}
/// the run's words each half second, with `owes` watching or not
fn clear(id: &str, watch: bool) -> (Vec<Vec<u32>>, Owes) {
    let (mut p, mut owes) = (stand(id), Owes::new());
    let mut h = HandsOff::new(p.g.seats[0], Plan { seed: 9, cap: 240.0, ..Default::default() });
    let mut rows = Vec::new();
    for i in 0.. {
        if h.end().is_some() { break; }
        p.played_by(&mut h);
        if watch { owes.tick(&mut p.g); }
        if i % 30 == 29 { rows.push(p.words()); }
    }
    (rows, owes)
}

/// A FAILURE MEANS: the watching plays the game: a run held to these is not the run a player has.
#[test]
fn asking_writes_nothing() {
    let (with, owes) = clear("depot-four", true);
    let (without, _) = clear("depot-four", false);
    assert!(!with.is_empty());
    assert!(with == without, "the two runs part at {:?} s", with.iter().zip(&without).position(|(a, b)| a != b).map(|k| k / 2));
    assert!(owes.faults.is_empty(), "{:?}", owes.faults);
}

/// A FAILURE MEANS: the check holds nothing: a man put one metre into a wall, or set down far off in a tick, goes unsaid.
#[test]
fn a_man_in_a_wall_and_a_man_who_jumps_are_said() {
    let mut p = stand("room-clear-four");
    let mut owes = Owes::new();
    for _ in 0..30 { p.frame(); owes.tick(&mut p.g); }
    assert!(owes.faults.is_empty(), "before any fault: {:?}", owes.faults);
    let man = p.mates()[0];
    let c = p.g.gd[man].ch;
    let at = p.g.ch[c].pos;
    // (the middle of a wall of the level's own, the nearest more than a jump away from him)
    let far = |q: [f64; 3]| (q[0] - at[0]).hypot(q[2] - at[2]);
    let walls = p.g.col.boxes.iter().map(|&b| &p.g.tab.boxes[b]).filter(|b| b.h[1] > 0.8 && b.c[1] < 1.5 && far(b.c) > 2.0).map(|b| b.c);
    let b = walls.min_by(|a, b| far(*a).total_cmp(&far(*b))).expect("a wall");
    p.g.ch[c].pos = [b[0], 0.0, b[2]];
    owes.tick(&mut p.g);
    let kinds: Vec<&str> = owes.faults.iter().map(|f| f.kind).collect();
    assert!(kinds.contains(&"in a box") && kinds.contains(&"popped"), "{:?}", owes.faults);
}

/// A FAILURE MEANS: a run on the list did a thing the game may never do. (Prints what each run ended on and its counts, which nobody judges here)
#[test]
#[ignore = "every run on the list: a minute and a half in a release build"]
fn every_run_owes() {
    let only = std::env::var("RUN").ok().filter(|r| !r.is_empty());
    let (mut wrong, mut runs) = (Vec::new(), 0);
    for e in all_missions() {
        if only.as_ref().is_some_and(|r| *r != e.id) { continue; }
        let mut held = |id: &str, owes: Owes, end: String| {
            println!("{:18} {:44} {}", id, end, owes.line());
            runs += 1;
            for f in &owes.faults { println!("    {} {}, {} times, first at {:.1} s: {}", f.who, f.kind, f.times, f.t, f.words); wrong.push(format!("{} {} {}", id, f.who, f.kind)); }
        };
        let mut o = Owes::new();
        if let Some(t) = tutorials().into_iter().find(|t| t.id == e.id) {
            let mut p = stand(&e.id);
            let mut l = Lesson::new(p.g.seats[0], t, 0.0);
            owes::played_out(&mut p, &mut l, 700.0, &mut o);
            let end = format!("{:.1} s, {}", l.me.t(), l.why().unwrap_or_else(|| String::from("ended")));
            held(&e.id, o, end);
        } else if matches!(e.kind, game::scene_scenarios::ScenarioEntryKind::Roomclear | game::scene_scenarios::ScenarioEntryKind::Nightbreach | game::scene_scenarios::ScenarioEntryKind::Rescue) {
            let mut p = stand(&e.id);
            let mut h = HandsOff::new(p.g.seats[0], Plan { seed: 9, cap: 240.0, ..Default::default() });
            owes::played_out(&mut p, &mut h, 245.0, &mut o);
            let end = format!("{:.1} s, {:?}", h.t(), h.end());
            held(&e.id, o, end);
        }
    }
    assert!(runs > 0, "no run on the list is called that");
    println!("{} runs", runs);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// A FAILURE MEANS: that kind of fault can happen with nobody told: each row breaks one thing by hand in a run that owed nothing until then.
#[test]
fn each_fault_is_said() {
    use game::doors as d;
    use game::game::{GuardState, MissionStage};
    type Break = fn(&mut Play, game::ids::GuardId);
    let rows: [(&str, Break); 12] = [
        ("no number", |p, m| { let c = p.g.gd[m].ch; p.g.ch[c].pos[0] = f64::NAN; }),
        ("off the level", |p, m| { let c = p.g.gd[m].ch; p.g.ch[c].pos = [1e4, 0.0, 1e4]; }),
        ("speed", |p, m| p.g.gd[m].speed = 50.0),
        ("awareness", |p, m| p.g.gd[m].awareness = 2.0),
        ("alive", |p, m| { let c = p.g.gd[m].ch; p.g.ch[c].alive = false; }),
        ("down with no how", |p, m| { let c = p.g.gd[m].ch; p.g.gd[m].state = GuardState::Dead; p.g.ch[c].alive = false; }),
        ("door", |p, _| { let k = p.g.doors.list[0]; d::door_set_latched(&mut p.g, k, true); d::door_set_angle(&mut p.g, k, 0.5); }),
        ("door", |p, _| { let k = p.g.doors.list[0]; d::door_set_pick(&mut p.g, k, 2.0); }),
        ("alarm", |p, _| { p.g.escalation = 1.0; p.g.escalation_t = -1.0; }),
        ("lockdown duty", |p, m| { let other = p.g.guards.iter().copied().find(|&x| x != m).unwrap(); p.g.gd[m].leader = Some(other); }),
        ("tracers", |p, _| p.g.tracers.live += 1.0),
        ("stage", |p, _| { p.g.mission.stage = MissionStage::Infiltrate; }),
    ];
    for (kind, broken) in rows {
        let mut p = stand("room-clear-four");
        let mut owes = Owes::new();
        if kind == "stage" { p.g.mission.stage = MissionStage::Exfil; }
        for _ in 0..30 { owes.tick(&mut p.g); }
        assert!(owes.faults.is_empty(), "{}, before it: {:?}", kind, owes.faults);
        let man = p.mates()[0];
        broken(&mut p, man);
        owes.tick(&mut p.g);
        assert!(owes.faults.iter().any(|f| f.kind == kind), "{} is unsaid: {:?}", kind, owes.faults);
    }
}

/// A FAILURE MEANS: a count that stays nought whatever happens: every count read as nought on a run says nothing unless it can read more.
#[test]
fn each_count_counts() {
    use game::game::GuardState;
    // on one spot: a second and a bit of two of the team on each other is one
    let mut p = stand("room-clear-four");
    let mut owes = Owes::new();
    let m = p.mates();
    for _ in 0..90 { p.frame(); let at = p.at(m[0]); let c = p.g.gd[m[1]].ch; (p.g.ch[c].pos[0], p.g.ch[c].pos[2]) = (at[0] + 0.1, at[1]); owes.tick(&mut p.g); }
    assert_eq!(owes.overlap, 1, "{}", owes.line());
    drop(p);
    // a teammate, down by Sam's round, is one; and one down by a defender is none
    let mut p = stand("room-clear-four");
    let mut owes = Owes::new();
    let m = p.mates();
    owes.tick(&mut p.g);
    for (man, killer, n) in [(m[0], 0.0, 0), (m[1], game::consts::PLAYER_ID, 1)] {
        let c = p.g.gd[man].ch;
        (p.g.gd[man].state, p.g.ch[c].alive, p.g.gd[man].killer_id) = (GuardState::Dead, false, killer);
        owes.tick(&mut p.g);
        assert_eq!(owes.friendly, n, "{}", owes.line());
    }
    drop(p);
    // a man under an order who stays where he is: the game gives up on him and says so, and that is one, however long he stands there after it
    let held = |watchdog: bool| {
        let mut p = stand("room-clear-four");
        let mut owes = Owes::new();
        let man = p.mates()[0];
        let at = p.at(man);
        let far = p.sam_at();
        p.order("move", [far[0], far[1] + 3.0], game::orders::Who::All);
        let mut stuck_at_3 = 0;
        for i in 0..60 * 7 {
            if !watchdog { let seat = game::seats::here(&p.g); let a = game::orders::squad_orders_of(&p.g, &seat, man).unwrap(); p.g.tab.orders_assignment[a].mark_t = p.g.time; }
            p.frame();
            let c = p.g.gd[man].ch;
            (p.g.ch[c].pos[0], p.g.ch[c].pos[2]) = (at[0], at[1]);
            owes.tick(&mut p.g);
            if i == 60 * 3 - 30 { stuck_at_3 = owes.stuck; }
        }
        (stuck_at_3, owes.stuck)
    };
    assert_eq!(held(true), (0, 1), "the game's own watch on");
    // ... and with that watch held off, as a wait at a door holds it, four seconds still say it, and again only ten after that
    assert_eq!(held(false), (0, 1), "the game's own watch held off");
}
