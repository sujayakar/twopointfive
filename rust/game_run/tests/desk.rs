//! The desk as a page has it: asks by their bytes, answers read off the wire's shape.
use game::io::{Io, R};
use game::kit::{MemberKit, TrimId};
use game::view::{CardRow, CardView, Pressed, TeamView};
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
fn row<'a>(c: &'a CardView, field: &str) -> &'a CardRow { c.rows.iter().find(|r| r.field == field).unwrap_or_else(|| panic!("no row {field}")) }
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
fn a_press_refused_writes_nothing() {
    // your four pouches are full as issued, so one more smoke has no room
    let mut d = Desk::new();
    let (cards, kept_before) = (all_cards(&mut d), kept::write(&mut d.g));
    let p = step(&mut d, 0, "smoke", 1);
    assert!(!p.ok);
    assert!(p.why.starts_with("needs a pouch"), "{}", p.why);
    assert_eq!(p.kept[0].text, kept_before);
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
    assert_eq!(row(&c, "smoke").n, 2.0);
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
    assert_eq!(row(&c, "rings").n, 5.0, "the stealth card carries the launcher's rounds");
    assert_eq!(row(&c, "primary").icon, "rifle");
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
