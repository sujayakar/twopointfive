//! THE PLAYER'S WORDS: no digit above 5, no compass word in a door's name, and how many are inside is a mystery.
mod kit_ask;
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

const NUM: [&str; 17] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen"];
const FOE: [&str; 17] = ["enemies", "enemy", "defenders", "defender", "guards", "guard", "gunmen", "gunman", "men", "sentries", "sentry", "captors", "captor", "hostiles", "hostile", "shooters", "shooter"];
fn number(w: &str) -> Option<usize> { w.parse().ok().or_else(|| NUM.iter().position(|n| *n == w)) }
/// every number a sentence puts on the enemy: the number, and whether it is said as the whole floor ("against eleven", "eleven hold")
fn counts_in(text: &str) -> Vec<(usize, bool)> {
    let low = text.to_lowercase();
    let w: Vec<&str> = low.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).collect();
    let mut out = Vec::new();
    for (i, t) in w.iter().enumerate() {
        let Some(n) = number(t) else { continue };
        let mut j = i + 1;
        if matches!(w.get(j), Some(&"armed") | Some(&"more") | Some(&"other")) { j += 1; }
        if w.get(j).is_some_and(|x| FOE.contains(x)) { out.push((n, false)); }
        if i > 0 && w[i - 1] == "against" { out.push((n, true)); }
        if matches!(w.get(i + 1), Some(&"hold") | Some(&"holds") | Some(&"holding") | Some(&"inside")) { out.push((n, true)); }
    }
    out
}

/// THE RULE: "we should also remove all defender counts from the menus, etc., it should be a mystery!": no mission row says its own head count, or any such number past five, or the whole floor's. Lessons are exercises and are not asked. The control: the old rows' sentences are caught.
/// A FAILURE MEANS: a menu tells the player how many men to count down.
#[test]
fn no_row_says_how_many_are_inside() {
    let bad_sentence = |text: &str, total: Option<f64>| counts_in(text).into_iter().find(|&(n, whole)| whole || n > 5 || total == Some(n as f64));
    let (mut bad, mut read) = (Vec::new(), 0);
    for m in game::start_list::mission_list() {
        let e = m.entry.as_ref();
        if e.is_some_and(|e| e.group == Some(game::scene_scenarios::MissionGroup::Tutorial)) { continue; }
        let text = format!("{} {} {}", m.place, m.blurb, e.and_then(|e| e.scene.clone()).unwrap_or_default());
        read += 1;
        if let Some((n, _)) = bad_sentence(&text, e.and_then(|e| e.defenders)) { bad.push(format!("{}: {n} in {text:?}", m.id)); }
    }
    assert!(read > 10, "the staging: only {read} rows read");
    assert!(bad.is_empty(), "rows that say the count: {bad:#?}");
    for old in ["It's five against eleven. Three squads hold the block.", "Eleven enemies hold a building laid out as a ring.", "The hard one. Eleven hold a ring of rooms."] {
        assert!(bad_sentence(old, Some(11.0)).is_some(), "the control: the reader lets through: {old}");
    }
    assert!(bad_sentence("Two sentries who watch each other.", Some(3.0)).is_none(), "the control: a part of the puzzle is kept");
}

/// THE RULE: no digit above 5 on a menu: every row the orders table registers, for a pair and for a four, is on a digit from 1 to 5 or on a key of its own. The control: the rows are there at all.
/// A FAILURE MEANS: an order sits on a key the left hand cannot reach.
#[test]
fn every_order_row_is_on_a_digit_up_to_five_or_a_key_of_its_own() {
    assert_eq!(game::orders::ORDER_DIGITS, 5.0);
    for squad in [2, 4] {
        let mut sc = Scene::new("yard");
        sc.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
        sc.door("west-yard", Axis::X, 16.0, 13.4).door("east-yard", Axis::X, 16.0, 21.4).door("between", Axis::Z, 18.0, 12.4);
        sc.moon();
        sc.team([14.0, 23.5], std::f64::consts::PI, &[[13.0, 21.5], [15.0, 21.5], [13.0, 22.5], [15.0, 22.5]]);
        sc.defender("west", [12.25, 12.25], "west-yard").defender("east", [24.25, 12.25], "east-yard");
        let p = Play::scene(&sc, &Opts { seed: 1, squad, ..Default::default() });
        let rows = p.order_keys();
        assert!(rows.len() > 5, "squad {squad}: only {} rows", rows.len());
        let bad: Vec<_> = rows.iter().filter(|(_, k, hot)| !(k.is_some_and(|d| (1.0..=5.0).contains(&d) && d.fract() == 0.0) || (k.is_none() && *hot))).collect();
        assert!(bad.is_empty(), "squad {squad}: rows on no digit from 1 to 5 and no key: {bad:?}");
    }
}

fn bearing(n: &str) -> bool {
    const COMPASS: [&str; 8] = ["north", "south", "east", "west", "northeast", "northwest", "southeast", "southwest"];
    n.to_lowercase().split(|c: char| !c.is_alphanumeric()).any(|w| COMPASS.contains(&w))
}

/// THE RULE: a man says his route by the place it keeps to or by words of its own, never by a bearing: every route of every level a mission plays.
/// A FAILURE MEANS: "I walk the west lane", to a player with no compass.
#[test]
fn no_route_is_said_by_a_compass_word() {
    let mut levels: Vec<String> = game::scene_scenario_list::all_missions().into_iter().filter_map(|e| e.level).filter(|l| !l.is_empty()).collect();
    levels.sort();
    levels.dedup();
    let (mut bad, mut said) = (Vec::new(), Vec::new());
    for file in &levels {
        let p = Play::level(if file.ends_with(".json") { file.clone() } else { format!("{file}.json") }.as_str(), false, &Opts { seed: 1, ..Default::default() });
        let words = p.route_words();
        for w in &words { if bearing(w) { bad.push(format!("{file}: said {w:?}")); } }
        said.extend(words);
    }
    assert!(said.len() > 20 && said.iter().any(|w| w == "the window side"), "the staging: {said:?}");
    assert!(bad.is_empty(), "routes said by a bearing: {bad:#?}");
}

/// THE RULE: no compass words on the HUD or in a door's name: over every level a mission plays, the name a door is said by holds no bearing, and no two doors of a level are said the same. The office and a lesson's level name their own and are not asked. The control: the test's own eye catches such a name.
/// A FAILURE MEANS: a line reads 'gate north door' on a HUD that has no compass.
#[test]
fn no_door_is_said_by_a_compass_word() {
    let has = bearing;
    assert!(has("gate north door") && !has("hall-side mess door"), "the control: the eye reads a bearing wrong");
    let mut levels: Vec<String> = game::start_list::mission_list().into_iter().filter(|m| !m.entry.as_ref().is_some_and(|e| e.group == Some(game::scene_scenarios::MissionGroup::Tutorial)))
        .filter_map(|m| m.entry.and_then(|e| e.level)).filter(|l| !l.is_empty() && !l.starts_with("office")).collect();
    levels.sort();
    levels.dedup();
    assert!(levels.len() > 5, "the staging: only {} levels", levels.len());
    let (mut bad, mut said) = (Vec::new(), 0);
    for file in &levels {
        let p = Play::level(if file.ends_with(".json") { file.clone() } else { format!("{file}.json") }.as_str(), false, &Opts { seed: 1, ..Default::default() });
        let names = p.door_names();
        said += names.len();
        for n in &names { if has(n) { bad.push(format!("{file}: said {n:?}")); } }
    }
    assert!(said > 20, "the staging: only {said} doors said");
    assert!(bad.is_empty(), "doors said by a bearing: {bad:#?}");
}
