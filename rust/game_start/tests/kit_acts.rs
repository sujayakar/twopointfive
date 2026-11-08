//! THE KIT PICTURE'S ACTS: the man whose card is open uses the thing under the highlight, at the door, again and again. Every row of every card is played here on the kinds of way a level has: a door, a set, a blank wall, an open gap, the quiet picture, the solo night. There is no other game to compare with: a row's marks are what it did, in the game's own books.
use game::game::GuardState;
use game::kitstage::{self, walk, KitStage};
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};

const THINGS_SAM: &[&str] = &["weapon", "light", "suit", "bangs", "smoke", "frags", "charges", "walls", "cams", "shells", "rings", "wand", "bigtool"];
const THINGS_MATE: &[&str] = &["weapon", "light", "suit", "bangs", "smoke", "frags", "charges", "walls", "wand", "bigtool"];
/// every word a row may refuse with: what the old page's rows said, word for word. Anything else is a row gone wrong.
const REFUSALS: &[&str] = &[
    "steel: a charge will not cut it", "steel: no gap under it", "steel: it holds", "no leaf to set it on", "no leaf to look under", "no leaf to break", "no leaf at this way in", "no room at the door",
    "no line through the door", "a blank wall: no opening to light", "a blank wall: no opening to throw through", "no wand on this card", "no card on him", "no launcher for it on this card", "no launcher on this card",
    "no wall inside to stick it to", "nothing for a door in his big slot", "no wall by the stack to set it on", "no floor for Sam to stand clear of the wall charge", "the wall charge", "the squad has no orders while the player is walked",
    "of the team could not stand clear", "no throw from here", "no clear throw through the door", "it did not go in", "the door never offered it", "the launcher never", "the squad never had the order", "the leaf never shut",
    "not a suppressed gun", "no lamp burning to put out", "nobody to take it",
];

fn stood(level: &str, words: &str) -> Play {
    let link = format!("?levelFile={}&kitview{}{}", std::path::Path::new(level).file_name().unwrap().to_str().unwrap(), if words.is_empty() { "" } else { "&" }, words);
    Play::mission(level, &link, &Opts { seed: 3, ..Default::default() })
}
fn st(p: &Play) -> &KitStage { p.g.kit_stage.as_ref().expect("a stage") }
fn sam(p: &Play) -> [f64; 3] { p.g.ch[p.g.pl[p.g.player].ch].pos }
fn flat(a: [f64; 3], b: [f64; 2]) -> f64 { ((a[0] - b[0]).powi(2) + (a[2] - b[1]).powi(2)).sqrt() }
/// the furthest any man is from where he lives
fn off_places(p: &Play) -> f64 {
    let s = st(p);
    let mut d = flat(sam(p), s.sam);
    for &x in &s.order { d = d.max(flat(p.g.ch[p.g.gd[x].ch].pos, walk::home_of(&p.g, s, x).0)); }
    d
}
fn team_up(p: &Play) -> bool { st(p).order.iter().all(|&x| p.g.gd[x].state != GuardState::Dead) && !p.g.pl[p.g.player].down }

/// a wall way and a gap way on a small set of a level's own
fn with_way(level: &str, way: &str, set: &str) -> String {
    let mut d: Json = json::parse(&game_start::files::level_text(level)).unwrap();
    let block = format!("{{\"sets\":{{\"{}\":{}}},\"ways\":[{}]}}", if way.contains("gate") { "gate" } else { "porch" }, set, way);
    let mut scen = d.get("scenarios").clone();
    scen.set_member("kitView", json::parse(&block).unwrap());
    d.set_member("scenarios", scen);
    let dir = std::env::temp_dir().join("kit-acts-test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{}.json", if way.contains("gate") { "gap" } else { "span" }));
    std::fs::write(&path, json::to_line(&d)).unwrap();
    path.to_str().unwrap().to_string()
}
fn set_boxes(lintel: bool) -> String {
    let lintel = if lintel { "{\"id\":\"set_lintel\",\"c\":[20,2.5,10],\"h\":[0.43,0.3,0.1],\"material\":\"concrete\"}," } else { "" };
    format!("{{\"ref\":[20,14],\"boxes\":[{{\"id\":\"set_ground\",\"c\":[20,-0.125,14],\"h\":[20,0.125,14],\"material\":\"asphalt\"}},{{\"id\":\"set_wall_a\",\"c\":[16.785,1.4,10],\"h\":[2.785,1.4,0.1],\"material\":\"concrete\"}},{}{{\"id\":\"set_wall_b\",\"c\":[20.965,1.4,10],\"h\":[0.535,1.4,0.1],\"material\":\"concrete\"}},{{\"id\":\"set_sill\",\"c\":[22.1,0.45,10],\"h\":[0.6,0.45,0.1],\"material\":\"concrete\"}},{{\"id\":\"set_pane\",\"c\":[22.1,1.5,10],\"h\":[0.6,0.6,0.006],\"material\":\"glassExt\",\"flags\":[\"Glass\"]}},{{\"id\":\"set_head\",\"c\":[22.1,2.45,10],\"h\":[0.6,0.35,0.1],\"material\":\"concrete\"}},{{\"id\":\"set_wall_c\",\"c\":[24.35,1.4,10],\"h\":[1.65,1.4,0.1],\"material\":\"concrete\"}},{{\"id\":\"set_crate\",\"group\":\"set.things\",\"c\":[24.5,0.4,13.2],\"h\":[0.4,0.4,0.4],\"material\":\"wood\"}}],\"lights\":[{{\"id\":\"set_lamp\",\"kind\":\"point\",\"pos\":[20,2.6,10.6],\"dir\":[0,-1,0],\"color\":[1,0.8,0.5],\"intensity\":6,\"range\":8,\"innerDeg\":0,\"outerDeg\":90,\"group\":\"set\",\"enabled\":true,\"volumetric\":0}}]}}", lintel)
}
fn wall_way() -> String { with_way("records.json", "{\"span\":[[22.9,10],[24.1,10]],\"set\":\"porch\"}", &set_boxes(true)) }
fn gap_way() -> String { with_way("records.json", "{\"span\":[[19.57,10],[20.43,10]],\"set\":\"gate\"}", &set_boxes(false)) }

/// what a thing must show, by its marks. None: this row has none it must show (the gun with no line scans).
fn wanted(thing: &str, quiet: bool, mate: bool) -> &'static [&'static str] {
    match thing {
        "light" => &["lit"], "bangs" => &["bang"], "smoke" => &["smoke"], "frags" => &["frag"], "charges" => &["set", "blow"], "walls" => &["set", "blow"], "cams" => &["cam", "stuck"], "shells" | "rings" => &["up"],
        "wand" => &["knee", "through", "out"], "bigtool" => &["gave"], "suit" => if quiet && !mate { &["lean"] } else { &["turn"] },
        "weapon" => if quiet && !mate { &["shot", "dark"] } else { &[] },
        _ => &[],
    }
}
fn known_refusal(r: &str) -> bool { let w = r.split_once(": ").map_or(r, |x| x.1); REFUSALS.iter().any(|k| w.contains(k)) }

/// one row of one card played to its end. What it showed: its marks, or the words it was refused in.
fn play_row(p: &mut Play, what: &str, who: u32, thing: &str, table: &mut Vec<String>) {
    let quiet = st(p).quiet;
    let mate = who > 0;
    let beats = st(p).a.beat_n;
    let serial = st(p).a.restages;
    let (mut got, mut refused): (Option<game::kitstage::BeatOver>, Option<String>) = (None, None);
    let field = if thing == "suit" { "suit" } else { thing };
    kitstage::open(&mut p.g, who, field);
    let mut waited = 0.0;
    let mut first_field = None;
    // the row begins, at once or as soon as what it waits for clears
    while waited < 40.0 {
        p.frame();
        waited += 1.0 / 60.0;
        let s = st(p);
        assert!(!s.none, "{}: nothing staged any more ({} on card {})", what, thing, who);
        if let Some(b) = s.a.beat.as_ref() { if b.n > beats && first_field.is_none() { first_field = Some(b.field); } }
        // (the charge that opens a blank wall FOR the row asked is that row's first act: the row itself is the one after it)
        if let Some(l) = s.a.last.as_ref() { if l.n > beats && !(l.marks.contains(&"prelude") && thing != "walls" && s.a.shut_wall.is_none()) { got = Some(l.clone()); break; } }
    }
    let got = got.unwrap_or_else(|| panic!("{}: {} on card {} never played a row in {} s (begun: {:?}, refused: {:?}, asked: {:?}, marks: {:?})", what, thing, who, waited, first_field, st(p).a.refused, st(p).a.asked, st(p).a.beat.as_ref().map(|b| (b.phase, b.marks.clone()))));
    if let Some(r) = st(p).a.refused.clone() { refused = Some(r); }
    assert!(!got.marks.contains(&"cap"), "{}: {} on card {} ran to its cap: {:?}, last phase {}", what, thing, who, got.marks, got.phase);
    let want = wanted(thing, quiet, mate);
    let want_thing = if thing == "suit" { "turn" } else { thing };
    let on_row = got.field == want_thing && !got.marks.contains(&"kept");
    let shown = on_row && want.iter().all(|w| got.marks.contains(w));
    let lit_off = thing == "light" && got.field == "weapon";   // (no light fitted: the gun comes up dark)
    let word = if shown { got.marks.join(",") } else if lit_off { String::from("no light on the card: the gun") } else {
        let r = refused.clone().unwrap_or_else(|| panic!("{}: {} on card {}: row {} showed {:?}, wanted {:?}, and no word why (how {})", what, thing, who, got.field, got.marks, want, got.how));
        assert!(known_refusal(&r), "{}: {} on card {} refused in words nobody wrote: {}", what, thing, who, r);
        format!("REFUSED {}", r.split_once(": ").map_or(r.as_str(), |x| x.1))
    };
    if want.is_empty() && thing == "weapon" { assert!(got.marks.iter().all(|m| *m != "lost"), "{}: the gun: {:?}", what, got.marks); }
    table.push(format!("{} | card {} | {} | {}", what, who, thing, word));
    println!("{}", table.last().unwrap());
    // what it left, staged again under the dissolve: nobody answers it, so half a second; and every man ends on his place
    if st(p).a.dirty || st(p).veil_asked.is_some() {
        let s0 = st(p).a.restages;
        let mut t = 0.0;
        while st(p).a.restages == s0 && t < 8.0 { p.frame(); t += 1.0 / 60.0; }
        assert!(st(p).a.restages != s0, "{}: {} on card {}: what it left was never staged again", what, thing, who);
    } else {
        let now = p.g.time;
        p.g.kit_stage.as_mut().unwrap().a.rest_until = now + 100.0;
    }
    let mut t = 0.0;
    while off_places(p) > 0.25 && t < 8.0 { p.frame(); t += 1.0 / 60.0; }
    assert!(off_places(p) <= 0.25, "{}: {} on card {}: {} m off his place 8 s after the row ({} serial {})", what, thing, who, off_places(p), got.how, st(p).a.restages - serial);
    assert!(team_up(p), "{}: {} on card {}: one of the team is down", what, thing, who);
    let s = st(p);
    assert_eq!((s.roused_men, s.roused_alarms), (0, 0), "{}: {} on card {}: the park had to put men or alarms right", what, thing, who);
    assert_eq!(p.g.escalation, 0.0, "{}: {} on card {}: the floor was raised", what, thing, who);
    assert_eq!((s.a.hits_sam, s.a.hits_team), (0, 0), "{}: {} on card {}: the row hit its own", what, thing, who);
    assert!(p.g.room_clear.end.is_none() && p.g.rescue.result.is_none(), "{}: {} on card {}: the mission ended", what, thing, who);
    // the rest it was given is over: the next row is the test's
    let now = p.g.time;
    let a = &mut p.g.kit_stage.as_mut().unwrap().a;
    a.rest_until = now;
    a.last = None;
}
/// every tile of Sam's card and of a teammate's, each highlighted in turn
fn every_row(level: &str, words: &str, what: &str) -> Vec<String> {
    let mut p = stood(level, words);
    assert!(!st(&p).none, "{}: nothing staged", what);
    let mut table = Vec::new();
    // (the cards' big slots hold a tool: their tile's thing is on the door)
    use game::kit::BigKit;
    for (i, big) in [(0usize, BigKit::Hammer), (1, BigKit::Hammer)] { if let Some(c) = p.g.m.kit.kit.members.get_mut(i) { c.big = big; } }
    kitstage::cards(&mut p.g, -1);
    p.run(1.0);
    let rows = game::kit::kit_rows(&mut p.g, 0.0);
    for r in rows { let f = r.field.as_str(); assert!(game::kitstage::acts::thing(&p.g, f, 0).is_some() || matches!(f, "bare" | "cans"), "{}: Sam's tile {} has no row", what, f); }
    for &t in THINGS_SAM { play_row(&mut p, what, 0, t, &mut table); }
    if !st(&p).order.is_empty() {
        for &t in THINGS_MATE { play_row(&mut p, what, 1, t, &mut table); }
    }
    table
}

#[test] fn records_door() { assert!(!every_row("records.json", "roomclear&squad4", "records").is_empty()); }
#[test] fn compound() { every_row("compound.json", "roomclear&squad4", "compound"); }
#[test] fn mousehole_way_1() { every_row("mousehole.json", "roomclear&squad4", "mousehole 1"); }
#[test] fn mousehole_way_2() { every_row("mousehole.json", "roomclear&squad4&way=2", "mousehole 2"); }
#[test] fn a_wall_span() { let l = wall_way(); let p = stood(&l, "roomclear&squad4"); assert!(st(&p).walled, "a span on a blank wall"); drop(p); let t = every_row(&l, "roomclear&squad4", "wall span"); assert!(t.iter().any(|r| r.contains("| walls |") && r.contains("hole")), "the wall charge's hole on the span: {:?}", t); }
#[test] fn a_gap() { let l = gap_way(); let p = stood(&l, "roomclear&squad4"); assert!(st(&p).door.is_none() && !st(&p).walled, "a gap"); drop(p); every_row(&l, "roomclear&squad4", "gap"); }
#[test] fn a_set() { every_row("registry.json", "rescue&squad=1", "registry set"); }
#[test] fn bazaar_quiet() { every_row("bazaar.json", "quiet&squad=1", "bazaar quiet"); }
#[test] fn vault_quiet() { every_row("vault.json", "quiet", "vault quiet"); }
#[test] fn solo() { every_row("office.json", "", "solo"); }

#[test]
fn a_pick_cuts() {
    let mut p = stood("records.json", "roomclear&squad4");
    kitstage::open(&mut p.g, 0, "suit");
    p.until(10.0, |p| st(p).a.beat.as_ref().map_or(false, |b| b.field == "turn" && b.t0 + 0.3 < p.g.time));
    let (n, serial) = (st(&p).a.beat.as_ref().expect("the turn is under way").n, st(&p).a.restages);
    kitstage::open(&mut p.g, 0, "weapon");
    let t0 = p.g.time;
    p.until(0.1, |p| st(p).a.beat.as_ref().map_or(false, |b| b.n > n));
    let b = st(&p).a.beat.as_ref().expect("the pick's row is up");
    assert!(b.n > n && b.field == "weapon" && p.g.time - t0 <= 0.1, "the row began {} s after the pick", p.g.time - t0);
    assert_eq!(st(&p).a.last.as_ref().unwrap().how, "cut");
    assert_eq!(st(&p).a.restages, serial, "no restage: the turn left nothing");
    // a row that left damage: cut after its mark, the room is staged again first, and the pick then plays
    kitstage::open(&mut p.g, 0, "frags");
    p.until(14.0, |p| st(p).a.beat.as_ref().map_or(false, |b| b.field == "frags" && b.marks.contains(&"frag")));
    if st(&p).a.beat.as_ref().map_or(false, |b| b.marks.contains(&"frag")) {
        let s0 = st(&p).a.restages;
        kitstage::open(&mut p.g, 0, "suit");
        p.until(3.0, |p| st(p).a.restages != s0);
        assert!(st(&p).a.restages != s0, "what the frag left is staged again");
        p.until(1.0, |p| st(p).a.beat.as_ref().map_or(false, |b| b.field == "turn"));
        assert_eq!(st(&p).a.beat.as_ref().map(|b| b.field), Some("turn"), "and the pick plays");
    }
}

#[test]
fn the_card_switch_walks() {
    let mut p = stood("records.json", "roomclear&squad4");
    let serial = st(&p).a.restages;
    let team = game::scenario_run::team_of(&p.g);
    for &who in &[1u32, 2, 3, 4, 0] {
        kitstage::open(&mut p.g, who, "");
        let mut t = 0.0;
        let mut arrived = None;
        while t < 8.0 {
            p.frame();
            t += 1.0 / 60.0;
            let s = st(&p);
            let mut at: Vec<[f64; 3]> = s.order.iter().map(|&x| p.g.ch[p.g.gd[x].ch].pos).collect();
            at.push(sam(&p));
            for i in 0..at.len() { for j in 0..i {
                let d = ((at[i][0] - at[j][0]).powi(2) + (at[i][2] - at[j][2]).powi(2)).sqrt();
                assert!(d >= 0.45, "card {}: two men {} m apart at {} s", who, d, t);
            } }
            if who > 0 && arrived.is_none() {
                let m = team[who as usize - 1];
                let k = s.order.iter().position(|&x| x == m).unwrap();
                let place = walk::home_of(&p.g, s, m).0;
                if flat(p.g.ch[p.g.gd[m].ch].pos, place) <= 0.25 && (k == 0 || (k == 3 && s.four)) { arrived = Some(t); }
            }
        }
        if who > 0 { assert!(arrived.map_or(false, |t| t <= 5.0), "card {}: his man at the head at {:?} s", who, arrived); }
        assert_eq!(st(&p).a.restages, serial, "card {}: no restage", who);
    }
}

fn tell(p: &mut Play, edit: &mut dyn FnMut(&mut game::kit::MemberKit), i: usize, size: i32) {
    edit(&mut p.g.m.kit.kit.members[i]);
    kitstage::cards(&mut p.g, size);
}
#[test]
fn a_card_changed_is_written_live() {
    use game::kit::{KitWeapon, SuitId, TrimId};
    let mut p = stood("records.json", "roomclear&squad4");
    let serial = st(&p).a.restages;
    let team = game::scenario_run::team_of(&p.g);
    kitstage::open(&mut p.g, 1, "weapon");
    p.run(4.0);
    let x = team[0];
    let c = p.g.gd[x].ch;
    // a teammate's gun: his hands' errand, then he wears it, and the gun's row follows the swap
    let was_gun = p.g.fa[p.g.gd[x].weapon].def;
    tell(&mut p, &mut |m| m.weapon = if m.weapon == KitWeapon::Pistol { KitWeapon::Rifle } else { KitWeapon::Pistol }, 1, -1);
    p.frame();
    assert!(st(&p).a.asked == Some("swap") || st(&p).a.beat.as_ref().map_or(false, |b| b.field == "swap"), "a gun changed plays the swap");
    p.until(3.0, |p| p.g.fa[p.g.gd[x].weapon].def != was_gun);
    assert!(p.g.fa[p.g.gd[x].weapon].def != was_gun, "he wears the new gun");
    p.until(6.0, |p| st(p).a.beat.as_ref().map_or(false, |b| b.field == "weapon"));
    assert_eq!(st(&p).a.beat.as_ref().map(|b| b.field), Some("weapon"), "the swap goes on to the gun's row");
    // his suppressor
    let g0 = p.g.fa[p.g.gd[x].weapon].def;
    tell(&mut p, &mut |m| m.suppressed = !m.suppressed, 1, -1);
    p.until(2.0, |p| p.g.fa[p.g.gd[x].weapon].def != g0);
    let _ = g0;
    // his light: off, the game's own field says so
    tell(&mut p, &mut |m| m.light = false, 1, -1);
    assert!(p.g.gd[x].light_dead_until.is_infinite(), "no light on him");
    tell(&mut p, &mut |m| m.light = true, 1, -1);
    assert_eq!(p.g.gd[x].light_dead_until, -1.0, "the light on him");
    // suit and trim
    let (t0, t1) = (p.g.ch[c].tint, p.g.ch[c].tint2);
    tell(&mut p, &mut |m| m.suit = SuitId::Black, 1, -1);
    assert!(p.g.ch[c].tint != t0, "the suit");
    tell(&mut p, &mut |m| m.trim = TrimId::Issued, 1, -1);
    let _ = t1;
    tell(&mut p, &mut |m| m.suit = SuitId::Issued, 1, -1);
    assert_eq!(p.g.ch[c].tint, t0, "the suit as issued");
    // counts: on the pools
    tell(&mut p, &mut |m| m.bangs = 3.0, 1, -1);
    p.frame();
    assert!(p.g.door_tools.bangs >= 3.0, "the count is in the squad's bag: {}", p.g.door_tools.bangs);
    assert_eq!(st(&p).a.restages, serial, "no restage for any of it");
    // Sam's gun and counts
    kitstage::open(&mut p.g, 0, "weapon");
    p.run(1.0);
    let (r0, s0) = (p.g.pl[p.g.player].rifle, p.g.pl[p.g.player].pistol);
    tell(&mut p, &mut |m| m.weapon = if m.weapon == KitWeapon::Pistol { KitWeapon::Rifle } else { KitWeapon::Pistol }, 0, -1);
    assert!(st(&p).a.sam_card_due, "his card is owed him");
    p.until(3.0, |p| !st(p).a.sam_card_due);
    assert!(!st(&p).a.sam_card_due, "his card is put on him");
    let _ = (r0, s0);
    p.until(3.0, |p| !p.g.pl[p.g.player].holstered && st(p).a.beat.as_ref().map_or(false, |b| b.field == "weapon"));
    assert_eq!(p.g.pl[p.g.player].holstered, false, "the gun comes up again");
    assert_eq!(st(&p).a.beat.as_ref().map(|b| b.field), Some("weapon"), "his swap goes on to the gun's row");
    tell(&mut p, &mut |m| m.frags = 2.0, 0, -1);
    p.run(1.0);
    assert_eq!(st(&p).a.restages, serial, "no restage for Sam's card");
    // the goggles: asked through the veil
    let v0 = st(&p).veil;
    tell(&mut p, &mut |m| m.nvg = !m.nvg, 1, -1);
    assert_eq!(st(&p).veil, v0 + 1, "goggles have no live path: the page is asked for a copy");
    assert_eq!(st(&p).a.restages, serial, "and nothing moves until it has laid one");
    kitstage::under(&mut p.g);
    p.frame();
    assert!(st(&p).a.restages != serial, "laid: staged again");
}

/// 60 s of a mission left alone with the gun highlighted: the loop repeats, nobody drifts
fn left_alone(level: &str, words: &str) {
    let what = format!("{} {}", level, words);
    let mut p = stood(level, words);
    kitstage::open(&mut p.g, 0, "weapon");
    let (mut most, mut rows) = (0.0f64, 0u32);
    for _ in 0..3600 {
        p.frame();
        let s = st(&p);
        rows = s.a.beat_n;
        if s.a.beat.is_none() && s.veil_asked.is_none() { most = most.max(s.order.iter().map(|&x| flat(p.g.ch[p.g.gd[x].ch].pos, walk::home_of(&p.g, s, x).0)).fold(0.0, f64::max)); }
    }
    let s = st(&p);
    assert!(!s.none, "{}: gone", what);
    assert!(rows > 3, "{}: {} rows in a minute: the loop does not repeat", what, rows);
    assert!(most <= 0.5, "{}: a man drifted {} m off his place at rest", what, most);
    assert_eq!((s.roused_men, s.roused_alarms), (0, 0), "{}", what);
    assert_eq!(p.g.escalation, 0.0, "{}", what);
}
#[test] fn sixty_seconds_records() { left_alone("records.json", "roomclear&squad4"); }
#[test] fn sixty_seconds_compound() { left_alone("compound.json", "roomclear&squad4"); }
#[test] fn sixty_seconds_mousehole() { left_alone("mousehole.json", "roomclear&squad4"); left_alone("mousehole.json", "roomclear&squad4&way=2"); }
#[test] fn sixty_seconds_quiet() { left_alone("bazaar.json", "quiet&squad=1"); left_alone("vault.json", "quiet"); left_alone("office.json", ""); }
#[test] fn sixty_seconds_span_and_gap() { left_alone(&wall_way(), "roomclear&squad4"); left_alone(&gap_way(), "roomclear&squad4"); }

/// THE RED PATH: a stage whose director never begins a row is what a count of rows catches
#[test]
fn a_driver_that_begins_nothing_is_caught() {
    let mut p = stood("records.json", "roomclear&squad4");
    p.g.kit_stage.as_mut().unwrap().still = true;
    p.run(6.0);
    assert_eq!(st(&p).a.beat_n, 0, "still: no row");
    if std::env::var("KITACTS_BREAK").is_ok() { assert!(st(&p).a.beat_n > 0, "KITACTS_BREAK: the check sees a driver that never begins a row"); }
    p.g.kit_stage.as_mut().unwrap().still = false;
    p.run(6.0);
    assert!(st(&p).a.beat_n > 0, "the director begins rows");
}
