//! WHAT THERE IS TO FRAME (rust/game_draw shot.rs), of games that stand: reading it changes nothing of the game, it is asked for only while wanted, a cut forgets, and it says what the
//! floor holds. That it says what the old read said is tools/draw/shot-check.ts's, while both stand.
use game_draw::{shot::Numbers, want, Drawn};
use game_start::play::{Opts, Play};

/// as the page's table has them (src/host/autoframe.ts)
fn numbers() -> Numbers {
    Numbers { fight_m: 22.0, near_m: 9.0, sway_linger: 2.0, lock_m: 8.0, lock_near_m: 2.0, lock_stick: 0.75, lock_linger: 1.0, lock_calm: 4.0, beat_secs: 3.5, beat_fade: 1.5, beat_m: 16.0, beyond_m: 2.5, door_m: 5.0, door_aim: 0.6, door_mps: 0.4, door_keep_m: 2.5, room_max_m: 12.0, room_k: 0.92, sky_k: 1.15, close_k: 0.8, fire_k: 1.1, fire_at: 0.3, lock_in: 0.4, zoom_amt: 1.0 }
}
fn office(seed: u32) -> Play { Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() }) }
fn drawn() -> Drawn { let mut d = Drawn::default(); d.shot.numbers = numbers(); d }

/// THE RULE: the read takes the game shared, and the openings and rooms no rule has needed yet are worked out for the host: the game read every tick is, in every part of its state,
/// the game nobody read. A FAILURE MEANS: a reader it calls has come to write through the shared game, or a memo of the game's was filled.
#[test]
fn reading_what_there_is_to_frame_is_no_part_of_the_game() {
    let played = |read: bool| {
        let mut p = office(5);
        let mut d = drawn();
        (0..300).map(|k| {
            p.g.god_mode = true;
            if k == 30 { p.key_down("KeyW"); }
            p.frame();
            if read { d.take(&p.g, want::shot); }
            p.words()
        }).collect::<Vec<_>>()
    };
    let (plain, read) = (played(false), played(true));
    assert_eq!(plain.iter().zip(&read).position(|(a, b)| a != b), None, "the game read parts from the one nobody read");
}

/// THE RULE: no row where nobody asked; a row a tick where the view is his own; he stands where the row says.
#[test]
fn a_row_a_tick_while_it_is_asked_for() {
    let mut p = office(6);
    let mut d = drawn();
    p.run(0.5);
    d.take(&p.g, want::view);
    assert!(d.shot.rows.is_empty(), "a row nobody asked for");
    p.frame();
    d.take(&p.g, want::shot);
    assert_eq!(d.shot.rows.len(), 1);
    let at = p.g.ch[p.g.pl[p.g.player].ch].pos;
    assert!((d.shot.rows[0].at[0] as f64 - at[0]).abs() < 1e-3 && (d.shot.rows[0].at[2] as f64 - at[2]).abs() < 1e-3, "he stands elsewhere than the row says");
    p.frame();
    d.take(&p.g, want::view);
    assert!(d.shot.rows.is_empty(), "a row left from when it was asked for");
}

/// THE RULE: a walk key held is not feet that are still; a button down is a hold. The hand is the tick's own.
#[test]
fn the_row_reads_the_hand_the_tick_was_given() {
    let mut p = office(7);
    let mut d = drawn();
    p.run(1.0);
    p.frame();
    d.take(&p.g, want::shot);
    assert_eq!((d.shot.rows[0].still, d.shot.rows[0].hold), (1, 0), "standing with nothing down");
    p.key_down("KeyW");
    p.frame();
    d.take(&p.g, want::shot);
    assert_eq!(d.shot.rows[0].still, 0, "a walk key held");
    p.key_up("KeyW");
    let input = p.host.input;
    p.g.tab.input[input].buttons = 1.0;
    p.frame();
    d.take(&p.g, want::shot);
    assert_eq!(d.shot.rows[0].hold, 1, "a button down");
}

/// THE RULE: a door order's beat is its phase changing, and fades after that: the order's own clock moving under one phase (a shot at the door, a swing) is no new beat, so a breach
/// that stays at one phase lets a newer order have the frame. A FAILURE MEANS: the beat is told by something that moves more often than the phase does.
#[test]
fn a_beat_is_a_phase_changing() {
    let mut p = office(10);
    p.g.god_mode = true;
    let mut d = drawn();
    let at = p.sam_at();
    let door = p.g.doors.list.iter().copied().min_by(|&a, &b| { let (u, v) = (p.g.dr[a].frame_centre, p.g.dr[b].frame_centre); ((u[0] - at[0]).hypot(u[2] - at[1])).total_cmp(&(v[0] - at[0]).hypot(v[2] - at[1])) }).unwrap();
    let c = p.g.dr[door].frame_centre;
    let t = game::orders::squad_orders_target_at(&mut p.g, [c[0], 1.0, c[2]]);
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "openClear", Some(&t), None, None), None, "the order was refused");
    let op = *p.g.door_tools.ops.last().expect("no order under way");
    let (first, numbers) = (p.g.time, numbers());
    let mut last_w = 1.0;
    let phase = p.g.tab.doortools_door_op[op].phase;
    for _ in 0..360 {
        // (the clock moved, every tick, and the phase held where the order is)
        p.frame();
        if !p.g.door_tools.ops.contains(&op) { break; }
        let now = p.g.time;
        let o = &mut p.g.tab.doortools_door_op[op];
        (o.phase, o.t0) = (phase, now);
        d.take(&p.g, want::shot);
        last_w = d.shot.rows[0].beat_w;
    }
    assert!(p.g.time - first > numbers.beat_secs + numbers.beat_fade, "the order did not stand long enough to tell ({} s)", p.g.time - first);
    assert_eq!(last_w, 0.0, "a phase held past its beat still has the frame");
}

/// THE RULE: a cut leaves nothing of the game before it: no man's last sight, no beat, no way out; and what the page said of the numbers stands.
#[test]
fn a_cut_forgets_what_was_framed() {
    let mut p = office(8);
    let mut d = drawn();
    p.run(1.0);
    p.frame();
    d.take(&p.g, want::shot);
    d.cut();
    assert!(d.shot.rows.is_empty() && d.shot.threats.is_empty() && d.shot.fight.is_empty(), "something was left over a cut");
    assert_eq!(d.shot.numbers.fight_m, 22.0, "the numbers are the page's, not the game's");
}

/// THE RULE: a hostile man in his line within reach is a man he could shoot; hidden by the fog, or put out of reach by the numbers, nobody is.
#[test]
fn the_men_he_could_shoot_are_the_numbers_men() {
    let mut p = office(9);
    p.g.god_mode = true;
    p.frame();
    let at = p.g.ch[p.g.pl[p.g.player].ch].pos;
    let Some(man) = p.g.guards.iter().copied().find(|&gd| game::teams::is_hostile(&p.g, game::teams::Team::Player, p.g.gd[gd].team) && p.g.gd[gd].state != game::game::GuardState::Dead) else { panic!("no hostile man on the floor") };
    // (a side of him with nothing in the way)
    let side = {
        let (ask, eye) = (game::ask::WorldAsk::of(&p.g), [at[0], 1.5, at[2]]);
        (0..16).map(|i| i as f64 * core::f64::consts::PI / 8.0).find(|a| !ask.line(eye, [at[0] + a.sin() * 2.0, 1.3, at[2] + a.cos() * 2.0], game::ask::Line::Sight)).expect("he is walled in")
    };
    let c = p.g.gd[man].ch;
    p.g.ch[c].pos = [at[0] + side.sin() * 2.0, at[1], at[2] + side.cos() * 2.0];
    p.frame();
    p.g.fog_of_war.hidden_chars.clear();
    let mut d = drawn();
    d.take(&p.g, want::shot);
    assert!(!d.shot.threats.is_empty() && !d.shot.fight.is_empty(), "a hostile man two metres off, in his line, is nobody");
    p.frame();
    let mut far = Drawn::default();
    far.shot.numbers = Numbers { fight_m: 0.0, ..numbers() };
    far.take(&p.g, want::shot);
    assert!(far.shot.threats.is_empty(), "a man past the reach");
    let (ch, mut hid) = (p.g.gd[man].ch, drawn());
    p.g.fog_of_war.hidden_chars.add(ch);
    hid.take(&p.g, want::shot);
    assert!(hid.shot.threats.iter().all(|t| (t.x as f64 - p.g.ch[ch].pos[0]).abs() > 1e-3), "a man the fog hides");
}
