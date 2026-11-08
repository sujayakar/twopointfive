//! The desk as a page has it: asks by their bytes, answers read off the wire's shape.
use game::io::{Io, R};
use game::kit::{MemberKit, TrimId};
use game::view::{BriefView, CardRow, CardView, Pressed, Setting, SheetView, ShortView, StartView, TeamView};
use game_run::command::apply;
use game_run::desk::Desk;
use game_run::kept;

const CARD: u32 = 0;
const STEP: u32 = 1;
const COPY: u32 = 4;
const PASTE: u32 = 5;
const KIT_COMMAND: u32 = 18;

fn text(b: &mut Vec<u8>, s: &str) { b.extend_from_slice(&(s.len() as u32).to_le_bytes()); b.extend_from_slice(s.as_bytes()); }
fn head(n: u32, who: u32) -> Vec<u8> { let mut b = n.to_le_bytes().to_vec(); b.extend_from_slice(&who.to_le_bytes()); b }

/// the card's answer, in the bytes that go to the page
fn card_bytes(d: &mut Desk, who: u32) -> Vec<u8> { d.ask(&head(CARD, who)).expect("an ask the desk has") }
fn card(d: &mut Desk, who: u32) -> CardView { CardView::r(&mut R::new(&card_bytes(d, who))) }
fn all_cards(d: &mut Desk) -> Vec<Vec<u8>> { (0..5).map(|who| card_bytes(d, who)).collect() }
fn press(d: &mut Desk, ask: &[u8]) -> Pressed { Pressed::r(&mut R::new(&d.ask(ask).expect("an ask the desk has"))) }
fn step(d: &mut Desk, who: u32, field: &str, dir: i32) -> Pressed {
    let mut b = head(STEP, who);
    text(&mut b, field);
    b.extend_from_slice(&dir.to_le_bytes());
    press(d, &b)
}
fn card_row<'a>(c: &'a CardView, field: &str) -> &'a CardRow { c.rows.iter().find(|r| r.field == field).unwrap_or_else(|| panic!("no row {field}")) }
fn desk_with(kept: &str) -> Desk {
    let mut d = Desk::new();
    let mut b = KIT_COMMAND.to_le_bytes().to_vec();
    text(&mut b, kept);
    apply(&mut d.g, &b).expect("a command the game has");
    d
}
fn members(d: &Desk) -> Vec<MemberKit> { d.g.m.kit.kit.members.clone() }
/// the five cards, as words to hold to each other
fn written(d: &Desk) -> Vec<String> { d.g.m.kit.kit.members.iter().map(kept::written).collect() }

#[test]
fn asked_twice_a_card_is_the_same_and_the_kept_text_has_not_moved() {
    let mut d = desk_with(r#"{"members":[{"weapon":"pistol","smoke":2},{"primary":"mk12","bangs":0},{},{"trim":"red"},{"big":"hammer"}]}"#);
    let kept_before = kept::write(&mut d.g);
    let first = all_cards(&mut d);
    assert_eq!(first, all_cards(&mut d));
    assert_eq!(kept_before, kept::write(&mut d.g));
}

#[test]
fn kept_then_written_then_read_again_is_the_same_five_cards() {
    let mut d = desk_with(r#"{"own":true,"members":[{"weapon":"pistol","sideCan":false,"suit":"navy","smoke":2,"bangs":0,"shells":0},{"primary":"mp7","light":false,"sideCan":true},{"secondary":"g18","nvg":false,"sideCan":true},{"rings":0,"big":"shotgun","trim":"amber"},{"wand":true,"sideCan":true,"bare":true}]}"#);
    let cards = all_cards(&mut d);
    let written = kept::write(&mut d.g);
    let mut again = desk_with(&written);
    assert_eq!(cards, all_cards(&mut again));
    assert_eq!(written, kept::write(&mut again.g));
}

#[test]
fn a_press_that_changes_no_kept_card_writes_nothing() {
    let mut d = Desk::new();
    press(&mut d, &head(4, 1));
    let p = press(&mut d, &head(5, 4));
    assert!(p.ok && p.kept.len() == 1, "pasted: written once");
    let again = press(&mut d, &head(5, 4));
    assert!(again.ok && again.changed.is_empty() && again.kept.is_empty(), "he wears it already");
    let (p, again) = (press(&mut d, &head(3, 4)), press(&mut d, &head(3, 4)));
    assert!(p.kept.len() == 1 && again.ok && again.kept.is_empty(), "as issued already");
}

#[test]
fn a_press_refused_writes_nothing() {
    // your four pouches are full as issued, so one more smoke has no room
    let mut d = Desk::new();
    let (cards, kept_before) = (all_cards(&mut d), kept::write(&mut d.g));
    let p = step(&mut d, 0, "smoke", 1);
    assert!(!p.ok);
    assert!(p.why.starts_with("needs a pouch"), "{}", p.why);
    assert!(p.kept.is_empty(), "nothing to write");
    assert_eq!(kept::write(&mut d.g), kept_before);
    assert_eq!(cards, all_cards(&mut d));
}

#[test]
fn a_card_that_overfills_its_pouches_is_brought_inside_with_the_sentence() {
    let mut d = desk_with(r#"{"members":[{"smoke":4,"bangs":4,"frags":1,"shells":1,"sideCan":true}]}"#);
    let c = card(&mut d, 0);
    assert_eq!(c.pouches.total, 4.0);
    assert!(c.pouches.used <= c.pouches.total, "{} of {}", c.pouches.used, c.pouches.total);
    assert!(c.words.ends_with("Your saved kit was over the pouches: 3 smoke canisters, 3 stun canisters left behind."), "{}", c.words);
}

#[test]
fn the_copied_card_goes_onto_another_man_fitted_to_his_rows() {
    // yours holds three smoke, a camera and a look: he may carry two smoke, has no cameras' row, and keeps his colours
    let mut d = desk_with(r#"{"members":[{"smoke":3,"bangs":0,"frags":0,"shells":0,"cams":1,"suit":"desert","trim":"red","primary":"mk18","sideCan":true},{"trim":"blue","suit":"navy","bangs":0,"charges":0,"frags":0,"sideCan":true}]}"#);
    assert!(press(&mut d, &head(COPY, 0)).ok);
    let p = press(&mut d, &head(PASTE, 1));
    assert!(p.ok, "{}", p.why);
    assert!(p.changed.iter().any(|f| f == "smoke"), "{:?}", p.changed);
    let m = &members(&d)[1];
    assert_eq!(m.smoke, 2.0);
    assert_eq!(m.cams, 0.0);
    assert!(m.trim == TrimId::Blue && m.suit.as_str() == "navy");
    let c = card(&mut d, 1);
    assert_eq!(card_row(&c, "smoke").n, 2.0);
    assert!(c.copied && c.clip && c.paste_why.is_empty());
}

const FOR: u32 = 6;
const DONE: u32 = 7;
const TEAM: u32 = 8;
const SIZE: u32 = 9;
const NAMED: u32 = 10;

fn go(d: &mut Desk, mission: &str, link: &str, issue: &str, walls: u32) -> Pressed {
    let mut b = FOR.to_le_bytes().to_vec();
    text(&mut b, mission);
    text(&mut b, link);
    text(&mut b, issue);
    b.extend_from_slice(&walls.to_le_bytes());
    press(d, &b)
}
fn team_bytes(d: &mut Desk) -> Vec<u8> { d.ask(&TEAM.to_le_bytes()).expect("an ask the desk has") }
fn team(d: &mut Desk) -> TeamView { TeamView::r(&mut R::new(&team_bytes(d))) }
fn size(d: &mut Desk, n: u32) -> Pressed { press(d, &head(SIZE, n)) }
fn named(d: &mut Desk, id: &str) -> Pressed {
    let mut b = NAMED.to_le_bytes().to_vec();
    text(&mut b, id);
    press(d, &b)
}

#[test]
fn a_stealth_mission_stands_its_card_for_yours_and_leaving_puts_yours_back() {
    let mut d = desk_with(r#"{"members":[{"weapon":"pistol","smoke":2,"bangs":0,"trim":"red"}]}"#);
    let (yours, kept_before) = (written(&d), kept::write(&mut d.g));
    let p = go(&mut d, "bazaar-quiet", "?levelFile=bazaar.json&quiet", "", 0);
    assert!(p.ok);
    let c = card(&mut d, 0);
    assert_eq!(card_row(&c, "rings").n, 5.0, "the stealth card carries the launcher's rounds");
    assert_eq!(card_row(&c, "primary").icon, "rifle");
    assert_eq!(members(&d)[0].trim, TrimId::Red, "your colours go with you");
    assert!(p.kept.is_empty(), "a mission asked for changes no kept card");
    assert_eq!(kept::write(&mut d.g), kept_before, "what is kept is your own card");
    let p = press(&mut d, &DONE.to_le_bytes());
    assert_eq!(written(&d), yours);
    assert!(p.kept.is_empty());
    assert_eq!(kept::write(&mut d.g), kept_before);
}

#[test]
fn a_size_the_mission_cannot_take_is_refused_with_its_words_and_changes_nothing() {
    let mut d = Desk::new();
    assert_eq!(team(&mut d).size, 4.0);
    go(&mut d, "bazaar-quiet", "?levelFile=bazaar.json&quiet", "", 0);
    let t = team(&mut d);
    assert!(t.size <= 1.0, "the mission's own size stands in for four: {}", t.size);
    assert!(!t.sizes[2].ok);
    let (before, cards) = (team_bytes(&mut d), all_cards(&mut d));
    let p = size(&mut d, 3);
    assert!(!p.ok);
    assert_eq!(p.why, t.sizes[3].note);
    assert!(p.why.starts_with("A quiet mission is for one or two"), "{}", p.why);
    assert_eq!(before, team_bytes(&mut d));
    assert_eq!(cards, all_cards(&mut d));
}

#[test]
fn a_named_kit_goes_on_five_cards_and_going_alone_on_yours_only() {
    let kit = {
        let mut d = Desk::new();
        team(&mut d).kits.iter().find(|k| k.id != "issued").expect("a kit beside the issue").id.clone()
    };
    let mut d = Desk::new();
    let was = written(&d);
    assert!(named(&mut d, &kit).ok);
    let on_all = written(&d);
    assert!(team(&mut d).kits.iter().any(|k| k.id == kit && k.now), "the row lights the kit that is on");
    assert!((1..5).any(|i| on_all[i] != was[i]), "a team's teammates change");

    let mut d = Desk::new();
    assert!(size(&mut d, 0).ok);
    let alone = written(&d);
    assert!(named(&mut d, &kit).ok);
    let now = written(&d);
    assert_eq!(now[1..], alone[1..], "going alone leaves the others' cards");
    assert_eq!(now[0], on_all[0]);
    assert!(!named(&mut d, "no such kit").ok);
}

#[test]
fn asked_twice_the_team_is_the_same_and_the_kept_text_has_not_moved() {
    let issue = r#"{"members":{"1":{"weapon":"shotgun","bangs":1,"frags":2}}}"#;
    for (mission, link, issue) in [("depot-four", "?levelFile=depot.json&roomclear&squad4", ""), ("hall-four", "?levelFile=hall.json&roomclear&squad4", issue), ("solo-night", "?solo", ""), ("no such mission", "?x", "")] {
        let mut d = desk_with(r#"{"members":[{"weapon":"pistol","smoke":2},{"primary":"mk12","bangs":0},{},{"trim":"red"},{"big":"hammer"}]}"#);
        go(&mut d, mission, link, issue, 0);
        let kept_before = kept::write(&mut d.g);
        let first = team_bytes(&mut d);
        assert_eq!(first, team_bytes(&mut d), "{mission}");
        assert_eq!(kept_before, kept::write(&mut d.g), "{mission}");
    }
}

#[test]
fn a_mission_that_issues_its_own_kit_has_no_cards_and_says_who_carries_what() {
    let issue = r#"{"members":{"1":{"weapon":"shotgun","bangs":1,"frags":2}}}"#;
    let mut d = Desk::new();
    go(&mut d, "hall-four", "?levelFile=hall.json", issue, 0);
    let t = team(&mut d);
    assert!(!t.cards && t.kits.is_empty());
    assert!(t.note.ends_with("This mission issues its own kit, whatever this machine keeps: Red One carries the shotgun."), "{}", t.note);
    assert_eq!(t.men[1].words, "shotgun · 1 stun · 2 frags");
    assert_eq!(t.men[0].words, "as issued");
    go(&mut d, "hall-four", "?levelFile=hall.json&noissue", issue, 0);
    assert!(team(&mut d).cards, "a link with noissue wears the kept cards");
}

// ---------------------------------------------------------------- the mission list

const START: u32 = 11;
const SHEET: u32 = 12;
const SET: u32 = 13;
const MISSION: u32 = 14;
const SHORT: u32 = 15;
const NEXT: u32 = 16;
const LEAVE: u32 = 17;
const LEVER_COMMAND: u32 = 13;
const QUIET_FOG_COMMAND: u32 = 19;
const QUIET_CAMS_COMMAND: u32 = 20;
const LEVELS: &str = "twopointsix.difficulty";
const LEVERS: &str = "twopointsix.levers";
const QUIET_FOG: &str = "twopointsix.quietfog";
const QUIET_CAMS: &str = "twopointsix.quietcams";

fn worded(n: u32, words: &[&str]) -> Vec<u8> {
    let mut b = n.to_le_bytes().to_vec();
    for w in words { text(&mut b, w); }
    b
}
fn start_bytes(d: &mut Desk, search: &str) -> Vec<u8> { d.ask(&worded(START, &[search])).expect("an ask the desk has") }
fn start(d: &mut Desk, search: &str) -> StartView { StartView::r(&mut R::new(&start_bytes(d, search))) }
fn sheet_bytes(d: &mut Desk, mission: &str) -> Vec<u8> { d.ask(&worded(SHEET, &[mission])).expect("an ask the desk has") }
fn sheet(d: &mut Desk, mission: &str) -> SheetView { SheetView::r(&mut R::new(&sheet_bytes(d, mission))) }
fn set(d: &mut Desk, mission: &str, key: &str, to: &str) -> Pressed { press(d, &worded(SET, &[mission, key, to])) }
fn word_of(d: &mut Desk, ask: &[u8]) -> String { String::r(&mut R::new(&d.ask(ask).expect("an ask the desk has"))) }
fn mission_of(d: &mut Desk, search: &str) -> String { word_of(d, &worded(MISSION, &[search])) }
fn short(d: &mut Desk, id: &str, search: &str, hard: i32) -> String {
    let mut b = worded(SHORT, &[id, search]);
    b.extend_from_slice(&hard.to_le_bytes());
    word_of(d, &b)
}
fn next(d: &mut Desk, search: &str) -> ShortView { ShortView::r(&mut R::new(&d.ask(&worded(NEXT, &[search])).expect("an ask the desk has"))) }
fn row<'a>(s: &'a SheetView, key: &str) -> &'a Setting { s.settings.iter().find(|r| r.key == key).unwrap_or_else(|| panic!("no row {key}")) }
fn lit(s: &SheetView, key: &str) -> Vec<String> { row(s, key).choices.iter().filter(|c| c.on).map(|c| c.id.clone()).collect() }
/// what a press says this machine keeps under a name: None where it says nothing of it
fn kept_of(p: &Pressed, key: &str) -> Option<String> { p.kept.iter().find(|k| k.key == key).map(|k| k.text.clone()) }
fn link_of(v: &StartView, id: &str) -> String { v.rows.iter().find(|r| r.id == id).unwrap_or_else(|| panic!("no row {id}")).link.clone() }

#[test]
fn asked_twice_the_list_and_the_sheet_are_the_same_and_nothing_is_kept_anew() {
    for address in ["", "?hour=dusk&weather=rain&fog=own&mute", "?levelFile=depot.json&roomclear&squad4&way=2", "?quietfog=cones&cams=0"] {
        let mut d = Desk::new();
        let kept_before = kept::write(&mut d.g);
        let list = start_bytes(&mut d, address);
        let ids: Vec<String> = StartView::r(&mut R::new(&list)).rows.iter().map(|r| r.id.clone()).collect();
        let sheets: Vec<Vec<u8>> = ids.iter().map(|id| sheet_bytes(&mut d, id)).collect();
        go(&mut d, &ids[0], "", "", 0);
        let team_first = team_bytes(&mut d);
        let words = (mission_of(&mut d, address), short(&mut d, "depot", address, -1), next(&mut d, address).now, word_of(&mut d, &worded(LEAVE, &[address])));
        assert_eq!(list, start_bytes(&mut d, address));
        let again: Vec<Vec<u8>> = ids.iter().map(|id| sheet_bytes(&mut d, id)).collect();
        assert_eq!(sheets, again);
        assert_eq!(team_first, team_bytes(&mut d));
        assert_eq!(words, (mission_of(&mut d, address), short(&mut d, "depot", address, -1), next(&mut d, address).now, word_of(&mut d, &worded(LEAVE, &[address]))));
        assert_eq!(kept_before, kept::write(&mut d.g));
    }
}

#[test]
fn a_lever_moved_by_hand_is_kept_and_lit_as_moved_and_put_back_on_its_levels_rung_is_kept_no_longer() {
    let mut d = Desk::new();
    start(&mut d, "");
    let moved = set(&mut d, "depot-four", "fog", "cones");
    assert!(moved.ok);
    assert_eq!(kept_of(&moved, LEVERS).as_deref(), Some(r#"{"fog":"cones"}"#));
    let s = sheet(&mut d, "depot-four");
    assert!(row(&s, "fog").moved);
    assert_eq!(lit(&s, "fog"), ["cones"]);
    assert!(!row(&s, "ai").moved);
    // the levers are kept in the order they were moved, as the machine's own store has them
    assert_eq!(kept_of(&set(&mut d, "depot-four", "ai", "hard"), LEVERS).as_deref(), Some(r#"{"fog":"cones","ai":"hard"}"#));
    assert_eq!(kept_of(&set(&mut d, "depot-four", "ai", "standard"), LEVERS).as_deref(), Some(r#"{"fog":"cones"}"#));
    let mut other = Desk::new();
    assert_eq!(kept_of(&set(&mut other, "depot-four", "kit", "stuns"), LEVERS).as_deref(), Some(r#"{"kit":"stuns"}"#));
    assert_eq!(kept_of(&set(&mut other, "depot-four", "fog", "own"), LEVERS).as_deref(), Some(r#"{"kit":"stuns","fog":"own"}"#));
    let back = set(&mut d, "depot-four", "fog", "team");
    assert!(back.ok);
    assert_eq!(kept_of(&back, LEVERS).as_deref(), Some(""));
    let s = sheet(&mut d, "depot-four");
    assert!(!row(&s, "fog").moved);
    assert_eq!(lit(&s, "fog"), ["team"]);
}

#[test]
fn a_level_pressed_puts_every_lever_on_its_rungs() {
    let mut d = Desk::new();
    start(&mut d, "");
    for (key, to) in [("fog", "own"), ("ai", "hard"), ("kit", "stuns"), ("told", "guided")] { assert!(set(&mut d, "depot-four", key, to).ok); }
    let hard = set(&mut d, "depot-four", "difficulty", "hard");
    assert_eq!(kept_of(&hard, LEVELS).as_deref(), Some("hard"));
    assert_eq!(kept_of(&hard, LEVERS).as_deref(), Some(""));
    let s = sheet(&mut d, "depot-four");
    assert_eq!(lit(&s, "difficulty"), ["hard"]);
    for (key, rung) in [("fog", "cones"), ("ai", "hard"), ("kit", "none"), ("told", "bare")] {
        assert_eq!(lit(&s, key), [rung]);
        assert!(!row(&s, key).moved);
    }
    assert_eq!(kept_of(&set(&mut d, "depot-four", "difficulty", "standard"), LEVELS).as_deref(), Some(""));
    assert_eq!(lit(&sheet(&mut d, "depot-four"), "fog"), ["team"]);
}

#[test]
fn a_quiet_missions_fog_press_keeps_the_quiet_pick_and_not_the_squads() {
    let mut d = Desk::new();
    let plain = link_of(&start(&mut d, ""), "bazaar-quiet");
    let p = set(&mut d, "bazaar-quiet", "fog", "cones");
    assert_eq!(kept_of(&p, QUIET_FOG).as_deref(), Some("cones"));
    assert_eq!(kept_of(&p, LEVERS), None);
    assert_eq!(lit(&sheet(&mut d, "bazaar-quiet"), "fog"), ["cones"]);
    assert_eq!(lit(&sheet(&mut d, "depot-four"), "fog"), ["team"]);
    let v = start(&mut d, "");
    assert_eq!(link_of(&v, "bazaar-quiet"), format!("{plain}&quietfog=cones"));
    assert!(!link_of(&v, "depot-four").contains("fog"));
    // … its own rung back where a quiet mission starts is kept no longer, and the cameras' press is its own
    assert_eq!(kept_of(&set(&mut d, "bazaar-quiet", "fog", "team"), QUIET_FOG).as_deref(), Some(""));
    assert_eq!(kept_of(&set(&mut d, "bazaar-quiet", "cams", "0"), QUIET_CAMS).as_deref(), Some("0"));
    assert_eq!(link_of(&start(&mut d, ""), "bazaar-quiet"), format!("{plain}&cams=0"));
}

#[test]
fn what_is_kept_of_the_quiet_picks_is_read_as_the_store_is() {
    let mut d = Desk::new();
    let mut b = worded(QUIET_FOG_COMMAND, &["own"]);
    b.extend_from_slice(&QUIET_CAMS_COMMAND.to_le_bytes());
    b.extend_from_slice(&0u32.to_le_bytes());
    d.machine(&b).expect("commands the game has");
    d.machine(&worded(QUIET_FOG_COMMAND, &["no rung"])).expect("commands the game has");
    let s = sheet(&mut d, "bazaar-quiet");
    assert_eq!(lit(&s, "fog"), ["own"]);
    assert_eq!(lit(&s, "cams"), ["0"]);
}

#[test]
fn a_lever_the_address_says_rides_on_the_links_and_one_the_machine_keeps_does_not() {
    let mut kept = Desk::new();
    kept.machine(&worded(LEVER_COMMAND, &["fog", "own"])).expect("a command the game has");
    assert!(!link_of(&start(&mut kept, ""), "depot-four").contains("fog=own"));
    let mut said = Desk::new();
    said.machine(&worded(LEVER_COMMAND, &["fog", "own"])).expect("a command the game has");
    assert!(link_of(&start(&mut said, "?fog=own"), "depot-four").contains("fog=own"));
    // … and pressed, the machine keeps them all: the page a link loads reads them back
    let p = set(&mut said, "depot-four", "ai", "hard");
    assert_eq!(kept_of(&p, LEVERS).as_deref(), Some(r#"{"fog":"own","ai":"hard"}"#));
    assert!(!link_of(&start(&mut said, "?fog=own"), "depot-four").contains("fog=own"));
}

#[test]
fn a_way_in_on_a_mission_with_one_is_refused_and_a_row_not_drawn_takes_no_press() {
    let mut d = Desk::new();
    let v = start(&mut d, "");
    assert!(v.rows.iter().any(|r| r.id == "bazaar-quiet" && r.ways.is_empty()));
    let kept_before = sheet_bytes(&mut d, "bazaar-quiet");
    let no = set(&mut d, "bazaar-quiet", "way", "1");
    assert!(!no.ok);
    assert_eq!(no.why, "");
    assert!(!row(&sheet(&mut d, "bazaar-quiet"), "way").shown);
    assert_eq!(kept_before, sheet_bytes(&mut d, "bazaar-quiet"));
    // a mission with two: its second is on its link and on no other's, its dealt is not offered
    assert!(set(&mut d, "depot-four", "way", "2").ok);
    assert!(!set(&mut d, "depot-four", "way", "any").ok);
    let v = start(&mut d, "");
    assert!(link_of(&v, "depot-four").ends_with("way=2"));
    assert!(!link_of(&v, "compound-four").contains("way="));
    // a row's own hour and weather: no row to press
    assert!(!set(&mut d, "tut-kit", "hour", "dawn").ok);
    assert!(!set(&mut d, "tut-kit", "weather", "rain").ok);
    // a lesson reads no lever; a level built for snow has its own weather
    assert!(!set(&mut d, "tut-team", "fog", "cones").ok);
    assert!(!set(&mut d, "pass-four", "weather", "rain").ok);
    assert!(!set(&mut d, "nobody", "hour", "dawn").ok);
}

#[test]
fn every_missions_row_link_starts_that_mission() {
    let mut d = Desk::new();
    let ids: Vec<String> = game::start_list::mission_list().into_iter().map(|m| m.id).collect();
    for id in &ids {
        // (a row cut from the lists is drawn where its own address asks for it)
        let v = start(&mut d, &format!("?loadout={id}"));
        let link = link_of(&v, id);
        assert_eq!(&mission_of(&mut d, &link), id, "{link}");
        assert_eq!(&start(&mut d, &link).named, id, "{link}");
    }
    let listed = start(&mut d, "");
    assert!(listed.rows.iter().all(|r| !r.off_list));
    assert!(ids.len() > listed.rows.len());
    assert_eq!(mission_of(&mut d, "?mute"), "");
}

#[test]
fn a_link_of_the_short_list_reads_back_to_its_row_and_one_word_more_on_it_does_not() {
    let mut d = Desk::new();
    for (id, then) in [("compound", "depot"), ("depot", "solo"), ("solo", "")] {
        for hard in [-1, 0, 1] {
            let link = short(&mut d, id, "?quality=low&hour=dusk&mute", hard);
            assert!(!link.is_empty());
            assert!(!link.contains("hour="), "{link}");
            assert_eq!(start(&mut d, &link).pick, id, "{link}");
            assert_eq!(next(&mut d, &link).now, id);
            assert_eq!(next(&mut d, &link).next, then);
            assert!(start(&mut d, &link).playing);
            let more = format!("{link}&hour=dusk");
            assert_eq!(start(&mut d, &more).pick, "", "{more}");
            assert_eq!(next(&mut d, &more).now, "");
        }
    }
    assert_eq!(short(&mut d, "nobody", "", -1), "");
    // the row after it, carrying on a difficulty chosen over the row's own
    let hard = short(&mut d, "compound", "", 1);
    let v = next(&mut d, &hard);
    assert_eq!(v.title, "The depot \u{b7} hard");
    assert!(v.link.contains("difficulty=hard"));
    let own = short(&mut d, "compound", "", -1);
    assert_eq!(next(&mut d, &own).title, "The depot");
    // a quit takes all of it off, the visitor's own flags left
    let from = short(&mut d, "compound", "?mute", 1);
    assert_eq!(word_of(&mut d, &worded(LEAVE, &[&from])), "?mute");
}

#[test]
fn the_team_works_out_its_own_link_from_the_list_with_the_size_on_it() {
    let mut d = Desk::new();
    start(&mut d, "?mute");
    go(&mut d, "depot-four", "", "", 0);
    let own = team(&mut d).link;
    assert_eq!(own, "?levelFile=depot.json&roomclear&squad4&difficulty=standard&pick=depot&mute");
    assert!(size(&mut d, 2).ok);
    assert_eq!(team(&mut d).link, "?levelFile=depot.json&roomclear&difficulty=standard&pick=depot&mute&squad=2");
    assert!(size(&mut d, 4).ok);
    // a setting moved: the row's own link, no longer the short list's
    assert!(set(&mut d, "depot-four", "hour", "dusk").ok);
    assert_eq!(team(&mut d).link, "?levelFile=depot.json&roomclear&squad4&mute&hour=dusk");
    // what a quiet mission hands out follows its lever
    go(&mut d, "bazaar-quiet", "", "", 0);
    assert_eq!(team(&mut d).link, "?levelFile=bazaar.json&quiet&mute&hour=dusk");
    let c = card(&mut d, 0);
    assert_eq!(card_row(&c, "cams").idle, false);
    assert!(set(&mut d, "bazaar-quiet", "fog", "off").ok);
    assert!(team(&mut d).link.contains("quietfog=off"));
    let c = card(&mut d, 0);
    assert_eq!(card_row(&c, "cams").idle, c.rows.iter().any(|r| r.field == "cams" && !r.none), "no fog: what only looks into it does nothing");
    assert!(row(&sheet(&mut d, "bazaar-quiet"), "cams").shown);
}

const BRIEF: u32 = 18;
const SAME: u32 = 19;
fn brief(d: &mut Desk, search: &str) -> BriefView { BriefView::r(&mut R::new(&d.ask(&worded(BRIEF, &[search])).expect("an ask the desk has"))) }
fn same(d: &mut Desk, link: &str, page: &str) -> bool { u32::r(&mut R::new(&d.ask(&worded(SAME, &[link, page])).expect("an ask the desk has"))) != 0 }

#[test]
fn a_brief_is_the_row_its_address_starts_with_the_team_it_deploys() {
    let mut d = Desk::new();
    let rows = start(&mut d, "").rows;
    for r in &rows {
        let b = brief(&mut d, &r.link);
        assert_eq!(b.id, r.id, "{}", r.link);
        assert_eq!(b.title, r.title);
        assert_eq!(b.team, r.team_words, "{}", r.link);
        assert_eq!(b.level, r.level);
    }
    assert_eq!(brief(&mut d, "?solo").id, "solo-night");
    assert_eq!(brief(&mut d, "?solo").team, "you, alone");
    assert_eq!(brief(&mut d, "").id, "", "a plain page starts none");
    assert_eq!(brief(&mut d, "?mute").id, "");
    // (a row deployed at another size than its own is still that row, at that size)
    let r = rows.iter().find(|r| r.kind == "roomclear" && r.squad == 4.0 && !r.link.contains("&squad=")).expect("a room clear of four");
    let two = format!("{}&squad=2", r.link.replacen("squad4", "roomclear", 1));
    let b = brief(&mut d, &two);
    assert_eq!(b.level, r.level, "{two}");
    assert_eq!(b.kind, "roomclear");
    assert_eq!(b.team, "you + the pair", "{two}");
    // (a level being built is played the moment it loads: no brief, whatever else the address says)
    assert_eq!(brief(&mut d, &format!("{}&level=relief", r.link)).id, "");
    assert_eq!(brief(&mut d, "?solo&level=relief").id, "");
    // (a lesson has its scene and what it teaches; a way the address asks for is that way, and dealt marks none)
    let lesson = rows.iter().find(|r| r.lesson).expect("a lesson");
    let b = brief(&mut d, &lesson.link);
    assert!(!b.scene.is_empty() && !b.teaches.is_empty(), "{}", lesson.id);
    let ways = rows.iter().find(|r| r.ways.len() > 1).expect("a mission with two ways");
    assert_eq!(brief(&mut d, &format!("{}&way=2", ways.link)).way, 2.0);
    assert_eq!(brief(&mut d, &format!("{}&way=any", ways.link)).way, 0.0);
    assert_eq!(brief(&mut d, &ways.link).way, 0.0);
}

#[test]
fn a_link_needs_no_page_load_only_where_the_page_stands_what_it_starts() {
    let mut d = Desk::new();
    // (the solo night stands under the fog as it ships: off a plain page, that is a load of its own link)
    assert!(!same(&mut d, "?solo&pick=solo", ""), "the night under the fog is a load of its own link");
    assert!(same(&mut d, "?solo&pick=solo", "?solo"), "off the night's own page");
    assert!(same(&mut d, "?solo&quietfog=off", ""), "the plain night off the plain page");
    assert!(same(&mut d, "?solo&quietfog=off&hour=midday", "?hour=midday"), "at the hour the page stands at");
    assert!(!same(&mut d, "?solo&quietfog=off&hour=midday", ""), "another hour is a load");
    assert!(!same(&mut d, "?solo&quietfog=off&weather=rain", ""), "another weather is a load");
    assert!(same(&mut d, "?solo&quietfog=off&weather=rain", "?weather=rain"));
    assert!(!same(&mut d, "?solo&quietfog=off", "?roomclear"), "a page that stands a scenario stands another thing");
}

#[test]
fn offlist_0_draws_every_row_of_the_table_and_a_visit_draws_a_cut_row_it_names() {
    let mut d = Desk::new();
    let all = start(&mut d, "?offlist=0").rows;
    let drawn = start(&mut d, "").rows;
    let cut = all.iter().find(|r| r.off_list).expect("a row cut from the lists");
    assert!(!drawn.iter().any(|r| r.id == cut.id));
    assert!(all.len() > drawn.len());
    assert!(start(&mut d, &format!("?loadout={}", cut.id)).rows.iter().any(|r| r.id == cut.id));
}

#[test]
fn no_group_draws_more_than_eight_rows_and_every_group_draws_one() {
    let mut d = Desk::new();
    let rows = start(&mut d, "").rows;
    let mut groups: Vec<&str> = rows.iter().map(|r| r.group.as_str()).collect();
    groups.dedup();
    assert_eq!(groups.len(), 6, "every group once, in a run of its own: {groups:?}");
    for g in groups { let n = rows.iter().filter(|r| r.group == g).count(); assert!((1..=8).contains(&n), "{g}: {n} rows"); }
}
