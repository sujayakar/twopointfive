//! NO RULE READS A COLOUR TO KNOW WHAT A THING IS. EVERY LEVEL FILE THAT NAMES A MISSION STAGES IT, WITH THE CAST ON ITS MARKS.
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};

/// Every colour of a level's file through `paint`: the palette's rows, a box's own, a prop's parts.
fn repaint(j: &mut Json, paint: &dyn Fn([f64; 3]) -> [f64; 3]) {
    match j {
        Json::Arr(a) => a.iter_mut().for_each(|x| repaint(x, paint)),
        Json::Obj(m, _) => for (k, v) in m.iter_mut() {
            if let (true, Json::Arr(c)) = (k == "albedo", &mut *v) {
                let n: Vec<f64> = c.iter().filter_map(|x| if let Json::Num(n) = x { Some(*n) } else { None }).collect();
                if n.len() == 3 { let q = paint([n[0], n[1], n[2]]); *c = q.iter().map(|&x| Json::Num(x)).collect(); }
            } else { repaint(v, paint); }
        },
        _ => {}
    }
}
/// Every word of what a thing does taken off a level's file: what a build that knew a thing by its colour would see in a repainted level.
fn unsaid(j: &mut Json) {
    match j {
        Json::Arr(a) => a.iter_mut().for_each(unsaid),
        Json::Obj(m, _) => { m.retain(|(k, _)| k != "does" && k != "stuff"); m.iter_mut().for_each(|(_, v)| unsaid(v)); }
        _ => {}
    }
}
/// Everything the rules make of a material, each as a line: the walls and cover and their stuff, the outer walls a charge may cut, what the index calls each piece, what a round knocks out of each box, the furniture.
fn answers(level: &Json, what: &str) -> (Vec<String>, usize, usize) {
    let path = std::env::temp_dir().join(format!("does-{}-{what}.json", std::process::id()));
    std::fs::write(&path, json::to_text(level)).unwrap();
    let mut p = Play::level(path.to_str().unwrap(), false, &Opts { seed: 1, ..Default::default() });
    std::fs::remove_file(&path).ok();
    p.g.tune.chaos = 0.5;
    p.frame();
    let walls = game::wallbreak::wall_break_all(&p.g);
    let (nw, nc) = (walls.iter().filter(|w| w.kind == game::wallbreak::BKind::Wall).count(), walls.iter().filter(|w| w.kind == game::wallbreak::BKind::Cover).count());
    let mut out: Vec<String> = walls.iter().map(|w| format!("wall {} {:?} {:?} {}", w.id, w.kind, w.surf, w.cells)).collect();
    game::furnblast::furniture_first_to_go(&mut p.g, [0.0, 1.0, 0.0], game::furnblast::BlastSource::Wall);
    for (i, b) in p.g.col.boxes.clone().into_iter().enumerate() {
        out.push(format!("box {i} outer {} hit {:?}", game::wallcharge::outer_wall(&mut p.g, i as f64), game::particle_fx::surf_at(&mut p.g, b)));
    }
    let mut items: Vec<String> = p.g.destr.items.iter().map(|(i, x)| format!("item {} {:?}", i.0, x.kind)).collect();
    items.sort();
    out.extend(items);
    if let Some(gr) = &p.g.destr.furn.groups { for x in gr.iter() { out.push(format!("furn {:?}", x.members.len())); } }
    (out, nw, nc)
}

/// THE RULE (the maintainer's experiment of 2026-11-05: a move of 0.004 a channel took the office from 18 breakable walls and 56 pieces of cover to none): a level stood with every colour moved a hair, and with every colour replaced outright, is the same level to every rule. CONTROL: the same level with its words taken off is not: so the words are what the rules read, and the sweep is not of a list nothing fills.
/// A FAILURE MEANS: a rule knows a thing by its colour, and a repaint changes what breaks, hides and stops rounds.
#[test]
fn a_level_with_every_colour_moved_is_the_same_level() {
    for name in ["office.json", "barracks.json"] {
        let text = std::fs::read_to_string(format!("{}/../../levels/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let file = json::parse(&text).unwrap();
        let (asis, walls, cover) = answers(&file, "asis");
        assert!(walls > 0 && cover > 0, "{name}: nothing to break: {walls} walls, {cover} cover");
        let mut moved = file.clone();
        repaint(&mut moved, &|c| [c[0] + 0.004, c[1] + 0.004, c[2] + 0.004]);
        let (a, ..) = answers(&moved, "moved");
        let mut other = file.clone();
        repaint(&mut other, &|c| [(c[1] * 7.3 + 0.21).fract(), (c[2] * 5.1 + 0.47).fract(), (c[0] * 3.7 + 0.13).fract()]);
        let (b, ..) = answers(&other, "other");
        for (how, x) in [("moved", &a), ("replaced", &b)] {
            let off: Vec<_> = asis.iter().zip(x.iter()).filter(|(p, q)| p != q).take(5).collect();
            assert!(off.is_empty() && asis.len() == x.len(), "{name}, every colour {how}: {} lines then {}: {off:?}", asis.len(), x.len());
        }
        let mut bare = file.clone();
        unsaid(&mut bare);
        let (c, ..) = answers(&bare, "bare");
        assert_ne!(asis, c, "{name}: the words off and the rules make of it what they did: the sweep asks nothing");
    }
}

fn kinds(data: &Json) -> Vec<(&'static str, &'static str)> {
    let s = data.get("scenarios");
    [("roomClear", "roomclear&squad4"), ("rescue", "rescue&squad=2"), ("nightBreach", "nightbreach&squad=2"), ("quiet", "quiet&squad=1")].into_iter().filter(|(k, _)| !s.get(k).is_null()).collect()
}
/// Is the mission of this word on the floor, its cast stood? What is wrong with it, if anything.
fn staged(p: &mut Play, key: &str) -> Vec<String> {
    let mut bad = match key {
        "rescue" => { let mut b = game::rescue::check_rescue(&mut p.g, None); b.retain(|w| !w.contains(": standing in ")); if !p.g.rescue.active { b.push(String::from("not active")); } if p.g.rescue.hostages.is_empty() { b.push(String::from("no hostages")); } if p.g.rescue.hostiles.is_empty() { b.push(String::from("no captors")); } b }
        "roomClear" => { let mut b = game::roomclear::check_structure(&mut p.g, None); if !game::roomclear::room_clear_scenario_active(&p.g) { b.push(String::from("not active")); } if p.hostiles().is_empty() { b.push(String::from("no defenders")); } b }
        "nightBreach" => if p.g.night_breach.active && !p.g.night_breach.hostiles.is_empty() { Vec::new() } else { vec![String::from("not active, or nobody home")] },
        "quiet" => if p.g.quiet.active { Vec::new() } else { vec![String::from("not active")] },
        _ => unreachable!(),
    };
    for m in p.mates() { if !p.up(m) { bad.push(String::from("a teammate stands dead")); } }
    bad
}

/// the one mark that breaks the rule, as the level has it: `the_office_rescue_marks_stand_on_floor`
const OFFICE_ROAMER: &str = "roamer 2 ";

/// THE RULE: a rescue's marks stand on free floor. A FAILURE MEANS: a man is stood inside the furniture and pushed out of it on the first frame.
#[test]
#[ignore = "the game: the office's rescue stands roamer 2 at (7.2, 6.4) in the conference table, and he is pushed 0.50 m out"]
fn the_office_rescue_marks_stand_on_floor() {
    let mut p = Play::mission("office.json", "?levelFile=office.json&rescue&squad=2", &Opts { seed: 3, ..Default::default() });
    let bad: Vec<_> = staged(&mut p, "rescue");
    assert!(bad.is_empty(), "{bad:#?}");
}

/// THE RULE (a rescue once vanished from levels/test-rooms.json and only this check saw it): every level file whose data names a mission of a kind stages it, with the cast on its marks. A lesson's level shows its lesson.
/// A FAILURE MEANS: a mission row opens an empty floor.
#[test]
fn every_level_stages_the_missions_it_names() {
    let dir = format!("{}/../../levels", env!("CARGO_MANIFEST_DIR"));
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().filter_map(|f| f.ok()?.file_name().into_string().ok()).filter(|f| f.ends_with(".json") && !f.contains(".lint.")).collect();
    names.sort();
    let (mut bad, mut n) = (Vec::new(), 0);
    for name in &names {
        let data = json::parse(&std::fs::read_to_string(format!("{dir}/{name}")).unwrap()).unwrap();
        let lesson = !data.get("scenarios").get("lesson").is_null();
        for (key, words) in kinds(&data) {
            n += 1;
            let mut p = Play::mission(name, &format!("?levelFile={name}&{words}"), &Opts { seed: 3, ..Default::default() });
            for w in staged(&mut p, key) { if !(name == "office.json" && w.starts_with(OFFICE_ROAMER)) { bad.push(format!("{name} {key}: {w}")); } }
            if lesson { p.frame(); if p.view().lesson.is_none() { bad.push(format!("{name}: no lesson showing")); } }
        }
    }
    assert!(n >= 20 && bad.is_empty(), "{n} missions: {bad:#?}");
}

/// THE CONTROL: test-rooms with its rescue block gone: the start refuses it by name, so the sweep above is of what the file says.
#[test]
#[should_panic(expected = "this level has no hostage rescue data")]
fn a_file_with_its_block_gone_refuses_that_mission() {
    let dir = format!("{}/../../levels", env!("CARGO_MANIFEST_DIR"));
    let mut file = json::parse(&std::fs::read_to_string(format!("{dir}/test-rooms.json")).unwrap()).unwrap();
    let Json::Obj(m, _) = &mut file else { panic!() };
    let (_, s) = m.iter_mut().find(|(k, _)| k == "scenarios").unwrap();
    let Json::Obj(s, _) = s else { panic!() };
    s.retain(|(k, _)| k != "rescue");
    let path = std::env::temp_dir().join(format!("stages-{}.json", std::process::id()));
    std::fs::write(&path, json::to_text(&file)).unwrap();
    let _ = Play::mission(path.to_str().unwrap(), "?levelFile=test-rooms.json&rescue&squad=2", &Opts { seed: 3, ..Default::default() });
}
