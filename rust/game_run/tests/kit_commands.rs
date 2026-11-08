//! THE KIT PICTURE'S COMMANDS, AS A HOST HANDS THEM: bytes in, the stage moved, and the game stepped by the one step every host uses. A failure means a host's list no longer reaches the stage, or the tick no longer holds it.
use game_run::command;
use game_start::play::{Opts, Play};

fn u32s(o: &mut Vec<u8>, x: u32) { o.extend(x.to_le_bytes()); }
fn text(o: &mut Vec<u8>, s: &str) { u32s(o, s.len() as u32); o.extend(s.as_bytes()); }
fn step(p: &mut Play) {
    let (h, g) = (p.host, &mut p.g);
    let me = game::game::game_host_seat(g);
    let mut c = game::seats::with_seat(g, me, &mut |g| game::cmd::sample_cmd(g, h.input, h.cam, h.canvas, None));
    c.keys.push(String::from("KeyW"));
    game_run::step(g, 1.0 / 60.0, &c);
}

#[test]
fn the_commands_reach_the_stage_and_the_step_holds_it() {
    let mut p = Play::mission("records.json", "?levelFile=records.json&kitview&roomclear&squad4", &Opts { seed: 3, ..Default::default() });
    let sam = |p: &Play| { let pl = p.g.player; p.g.ch[p.g.pl[pl].ch].pos };
    let at = sam(&p);
    // (a key in the seat's hand is dropped: he stands)
    for _ in 0..60 { step(&mut p); }
    let now = sam(&p);
    assert!((now[0] - at[0]).abs() + (now[2] - at[2]).abs() < 0.05, "Sam walked from {:?} to {:?}", at, now);
    let st = |p: &Play| p.g.kit_stage.as_ref().unwrap().clone();
    assert!(st(&p).pinned.is_some());

    // 30: the cards and the size
    let mut b = Vec::new();
    u32s(&mut b, 30);
    text(&mut b, "{\"v\":2,\"own\":true,\"stealth\":true,\"members\":[{\"weapon\":\"rifle\"}]}");
    b.extend(2i32.to_le_bytes());
    command::apply(&mut p.g, &b).unwrap();
    // (another size cannot be written live: the view's `veil` asks the page for a copy, nothing is staged until it has laid one)
    let veil = st(&p).veil;
    assert_eq!(veil, 1, "the size asks for the page's copy");
    step(&mut p);
    assert_eq!(st(&p).order.len(), 4, "nothing staged before the copy is laid");
    // 33: the copy is laid
    command::apply(&mut p.g, &33u32.to_le_bytes()).unwrap();
    step(&mut p);
    assert_eq!(st(&p).order.len(), 2);
    assert_eq!(st(&p).size, Some(2));
    assert!(st(&p).veil_asked.is_none());
    // (under 0: as it stands)
    let mut b = Vec::new();
    u32s(&mut b, 30);
    text(&mut b, "{\"members\":[]}");
    b.extend((-1i32).to_le_bytes());
    command::apply(&mut p.g, &b).unwrap();
    step(&mut p);
    assert_eq!(st(&p).order.len(), 2);
    assert_eq!(st(&p).veil, veil, "nothing moved: no copy asked for");

    // 31: the open card: no copy, no restage, the same men walk
    let mut b = Vec::new();
    u32s(&mut b, 31); u32s(&mut b, 2); text(&mut b, "bangs");
    command::apply(&mut p.g, &b).unwrap();
    step(&mut p);
    assert_eq!((st(&p).who, st(&p).field.as_str()), (2, "bangs"));
    assert_eq!(st(&p).order[0], game::scenario_run::team_of(&p.g)[1]);
    assert_eq!(st(&p).veil, veil);

    // 32: the frame
    let cam = p.g.host_cam.unwrap();
    let was = p.g.tab.camera[cam].pin.unwrap();
    let mut b = Vec::new();
    u32s(&mut b, 32);
    for x in [1.6, -0.4, 0.0, 0.6, 0.5, 0.19] { b.extend(f64::to_le_bytes(x)); }
    for x in [-0.2, -0.9, 0.1, 0.85, f64::NAN, f64::NAN] { b.extend(f64::to_le_bytes(x)); }
    command::apply(&mut p.g, &b).unwrap();
    let now = p.g.tab.camera[cam].pin.unwrap();
    assert!(now.look != was.look && st(&p).frame.unwrap().hung.is_some(), "the frame moved nothing");
    step(&mut p);
    assert_eq!(p.g.tab.camera[cam].pin.unwrap().look, now.look, "the tick moved the pin");
}

/// a copy the page never answers: half a second of game time from the asking, the room is staged all the same
#[test]
fn a_copy_never_laid_is_not_waited_for() {
    let mut p = Play::mission("records.json", "?levelFile=records.json&kitview&roomclear&squad4", &Opts { seed: 3, ..Default::default() });
    let st = |p: &Play| p.g.kit_stage.as_ref().unwrap().clone();
    let mut b = Vec::new();
    u32s(&mut b, 30);
    text(&mut b, "{\"members\":[]}");
    b.extend(1i32.to_le_bytes());
    command::apply(&mut p.g, &b).unwrap();
    assert_eq!(st(&p).veil, 1);
    for _ in 0..28 { step(&mut p); }
    assert_eq!(st(&p).order.len(), 4, "still waiting at 0.47 s");
    for _ in 0..4 { step(&mut p); }
    assert_eq!(st(&p).order.len(), 1, "staged again by 0.53 s");
}
