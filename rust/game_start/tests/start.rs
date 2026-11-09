//! THE START ITSELF: a level of `levels/` stands and runs with nothing made by TypeScript, and a game stood again in the same process is the
//! same game (rust/sim keeps blocks and tables in statics: a stand must leave nothing of the game before it).
use game_start::play::{Opts, Play};

/// a hash of each part of rust/sim's block in slot 0, and the die's count
fn block() -> (Vec<u32>, f64) { (game_start::block::part_hashes(0), sim::state::state(0).dice.n) }

/// A real level's room clear, run for ten seconds: the block after it.
fn kit_yard(seed: u32) -> (Vec<u32>, f64) {
    let mut p = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
    assert_eq!(p.mates().len(), 4, "the room clear stages four teammates");
    assert!(p.hostiles_by([20.0, 14.0]).len() >= 4, "…and the level's garrison: {}", p.roll());
    p.run(10.0);
    assert!((p.time() - 10.0).abs() < 1e-6);
    block()
}

/// A FAILURE MEANS: a stand left something of the last game behind (a static table of rust/sim or of the game crate that a new game does not
/// make anew), so what a test sees depends on the tests that ran before it.
#[test]
fn a_game_stood_again_is_the_same_game() {
    let a = kit_yard(1);
    let other = kit_yard(2);
    let again = kit_yard(1);
    assert_ne!(a, other, "another seed is another game (or the die is not in play)");
    assert_eq!(a, again, "seed 1 stood, then seed 2, then seed 1 again: the first and the third are one game");
}

/// A level as its file has it, on the page's own stand (no mission started, the page's senses): what the compare tool and a host stand.
/// A FAILURE MEANS: the plain start of a real level (the constructor, the solo night's card, the host's first frames) no longer runs.
#[test]
fn a_level_stands_as_its_file_has_it() {
    let mut p = Play::level("kit-yard.json", false, &Opts::default());
    assert!(p.mates().is_empty(), "no mission was started, so nobody was staged: {}", p.roll());
    p.run(10.0);
    assert!((p.time() - 10.0).abs() < 1e-6 && p.frames == 600);
}

/// A mission started from the words of its address, as the page starts one: the row of the mission list that owns the level lays its hour, the
/// link's fog of war goes on, and the mission is staged. The same level with the mission alone started on it stays at the hour a game is made at.
/// A FAILURE MEANS: the game no longer reads an address's words itself (`game::query`), or a start from them stopped laying what the row says
/// before it stages the mission (rowset.ts, hours.ts, scenarioRun.ts startFromQuery).
#[test]
fn a_mission_starts_from_its_link_as_the_page_starts_it() {
    use game::hours::HourId;
    let made_at = { let mut p = Play::level("tut-queue.json", true, &Opts::default()); assert!(!p.g.fog_of_war.on, "no address asked for the fog"); game::hours::hour_of(&mut p.g) };
    assert_ne!(made_at, HourId::Dusk, "a game is not made at dusk");
    let mut p = Play::mission("tut-queue.json", "?levelFile=tut-queue.json&roomclear=1&squad4", &Opts::default());
    assert_eq!(game::hours::hour_of(&mut p.g), HourId::Dusk, "the row of the list that owns this level says dusk");
    assert!(p.g.fog_of_war.on, "a room clear's link stands in the fog of war");
    assert_eq!(p.mates().len(), 4, "…and its mission is staged: {}", p.roll());
}

/// Three of the game's levels that between them hold every kind of record beyond a plain floor: they stand, and what the file says is on the floor.
/// A FAILURE MEANS: the start stopped making something a level file says (a round thing, a box's shape and its trim, a
/// civilian, a camera, a laser, a keypad, the vents, the daylight), or stopped standing such a level at all. (That what it makes stays what it made is the gate's `start` rows' to hold:
/// this only holds that it is made.)
#[test]
fn what_a_level_file_says_is_on_the_floor() {
    use game_start::play::Floor;
    let plain = Opts::default();
    // the pass: 89 boxes say a shape, 12 of them tilted (a tilted box keeps its plain shape and has no trim), and they say 134 pieces of trim
    let mut p = Play::level("pass.json", false, &plain);
    let f = p.floor();
    assert!(f.shaped > 60 && f.shaped <= 89 && f.trim > 100 && f.trim <= 134 && f.boxes == 682 + f.trim, "the pass's shapes and their trim: {:?}", f);
    p.run(1.0);
    drop(p);
    // the long hall in the look's dress: drums and tyres, each one round box in the file and on the floor
    let p = Play::level("hall.json", false, &plain);
    let f = p.floor();
    assert!(f.round > 0 && f.boxes == 435, "the long hall's round things (and no box added or lost for them): {:?}", f);
    drop(p);
    // the office: two civilians, three cameras, two lasers and a keypad, six vents, a daylight
    let mut p = Play::level("office.json", false, &plain);
    assert_eq!(p.floor(), Floor { boxes: p.floor().boxes, round: p.floor().round, shaped: 22, trim: 28, civilians: 2, cameras: 3, lasers: 2, keypads: 1, vents: 6, daylight: true });
    p.run(1.0);
}

/// A FAILURE MEANS: a level file under `levels/` holds a record the reader has no line for, or a member no field of the game's type takes (the
/// list names it): a level the game ships can no longer start in Rust. Or, on the last line, the reader has stopped refusing what it cannot carry.
#[test]
fn every_level_file_is_read_whole_and_a_stranger_is_refused() {
    use game_start::{files, json, level};
    let dir = files::root().join("levels");
    let mut n = 0;
    for e in std::fs::read_dir(&dir).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_str().unwrap().to_string();
        if !name.ends_with(".json") { continue; }
        let data = json::parse(&std::fs::read_to_string(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {}", name, e));
        if !data.has("boxes") { continue; }   // (a file of the folder that is not a level: the lint's list)
        let lacks = std::panic::catch_unwind(|| level::lacks_of(&data)).unwrap_or_else(|_| panic!("{} could not be read (the reader's own words are above)", name));
        assert!(lacks.is_empty(), "{} is not carried whole: {:?}", name, lacks);
        n += 1;
    }
    assert!(n >= 30, "levels/ holds the game's level files ({} read)", n);
    // a file from a later reader, and a record with a member the game's type has no field for
    let mut later = json::parse(&files::level_text("kit-yard.json")).unwrap();
    if let json::Json::Obj(m, _) = &mut later {
        let beam = json::obj(vec![("kind", json::s("laser")), ("id", json::s("beam")), ("from", json::nums(&[0.0, 1.0, 0.0])), ("to", json::nums(&[1.0, 1.0, 0.0])), ("colour", json::s("red"))]);
        for (k, v) in m.iter_mut() { if k == "version" { *v = json::n(3.0); } if k == "devices" { *v = json::arr(vec![beam.clone()]); } }
    }
    let lacks = level::lacks_of(&later);
    assert!(lacks.iter().any(|l| l.starts_with("version")), "a file of another version is named: {:?}", lacks);
    assert!(lacks.iter().any(|l| l == "devices[0].colour"), "a member no field takes is named by its place: {:?}", lacks);
    // a file of the version before, whose round things are four slabs: refused by its version, and told the command
    let mut old = json::parse(&files::level_text("kit-yard.json")).unwrap();
    if let json::Json::Obj(m, _) = &mut old { for (k, v) in m.iter_mut() { if k == "version" { *v = json::n(1.0); } } }
    let lacks = std::panic::catch_unwind(|| level::lacks_of(&old)).unwrap_or_else(|_| panic!("a file of version 1 could not be read"));
    assert!(lacks.iter().any(|l| l.starts_with("version 1") && l.contains("round-convert")), "a file of version 1 is named, with the command: {:?}", lacks);
    // …and of a record the loader reads by hand, and at the top of the file
    let mut grown = json::parse(&files::level_text("kit-yard.json")).unwrap();
    if let json::Json::Obj(m, _) = &mut grown {
        m.push(("elevators".to_string(), json::arr(vec![])));
        for (k, v) in m.iter_mut() { if k == "doors" { if let json::Json::Arr(d) = v { if let json::Json::Obj(door, _) = &mut d[0] { door.push(("auto".to_string(), json::b(true))); } } } }
    }
    let lacks = level::lacks_of(&grown);
    assert_eq!(lacks, ["doors[0].auto", "elevators"], "what a file gained since the reader was written is named, and nothing else");
}
