//! WHAT A MACHINE KEEPS, AS ONE RECORD (`host::kept::Kept`, laid by `host::lay`): each field reaches the game a start stands, the address is over each, a lesson is taught on the standard rules, and a
//! text that is no such record is refused by the field's name.
//! (A file of its own: these stands hold no `Play`, so they must not run beside the tests that do: the character's clips are one store of the process.)
use game::difficulty::{difficulty_now, lever_set, DifficultyNow, LeverKey};
use game_start::files;
use game_start::host::desk::Desk;
use game_start::host::kept::{self, Kept};
use game_start::host::knobs;
use game_start::json;
use game_start::stand::{stand, StartOpts, Stood};

const LINK: &str = "?levelFile=depot.json&roomclear&squad4";
const LESSON: &str = "?levelFile=tut-road.json";
/// a number the panel's kept rows hold, and one at the top of its row
const ROW: &str = "wind.vent_k";
const CARDS: &str = r#"{"v":2,"members":[{"weapon":"pistol","trim":"red"},{"primary":"mp7"},{},{},{}]}"#;

/// one stand at a time: the tests of this file share the block of a slot
static ONE: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn one() -> std::sync::MutexGuard<'static, ()> { ONE.lock().unwrap_or_else(|p| p.into_inner()) }

/// (slot 1: these stands hold no `Play`, whose games stand in slot 0)
fn stood(level: &str, link: &str, kept: &str) -> Stood {
    let ch = files::CharacterFiles::read();
    let data = json::parse(&files::level_text(level)).expect("a level's file");
    stand(&data, ch.assets(), &StartOpts { seed: 1, slot: 1, link: Some(String::from(link)), kept: Kept::read(kept).expect("a record"), may_refuse: true, ..Default::default() })
}
fn depot(words: &str, kept: &str) -> Stood { stood("depot.json", &format!("{LINK}{words}"), kept) }
fn card(s: &Stood) -> String { s.game.m.kit.kit.members[1].primary.clone() }

/// THE RULE: every field of the record is on the game a start stands. A FAILURE MEANS: a page's kept setting no longer reaches its game. The control is the same stand with nothing kept.
#[test]
fn each_field_reaches_the_game_a_start_stands() {
    let _one = one();
    let plain = depot("", "");
    assert_eq!(difficulty_now(&plain.game), DifficultyNow::Standard);
    assert_eq!(lever_set(&plain.game, LeverKey::Ai), None);
    let (row, row_top, issued) = (knobs::get(&plain.game, ROW).unwrap(), 3.0, card(&plain));
    assert_ne!(row, 2.5);
    assert_ne!(issued, "mp7");
    assert!(matches!(plain.game.m.quietfog.cams, None));
    drop(plain);
    let text = format!(r#"{{"hard":true,"levers":{{"ai":"hard"}},"cards":{},"quietFog":"own","quietCams":false,"rows":{{"{ROW}":2.5}},"picks":{{"nerve":"brittle"}}}}"#, kept::text(CARDS));
    let s = depot("", &text);
    assert!(matches!(difficulty_now(&s.game), DifficultyNow::Hard | DifficultyNow::Custom), "the level");
    assert_eq!(s.game.m.difficulty.base, game::difficulty::Difficulty::Hard, "the level");
    assert_eq!(lever_set(&s.game, LeverKey::Ai), Some("hard"), "a lever");
    assert_eq!(card(&s), "mp7", "the cards");
    assert_eq!(s.game.m.quietfog.pick.map(|r| r.as_str()), Some("own"), "the quiet rung");
    assert_eq!(s.game.m.quietfog.cams, Some(false), "the quiet cameras");
    assert_eq!(knobs::get(&s.game, ROW), Some(2.5), "a row");
    assert!(matches!(s.game.k.morale.nerve, game::morale::Nerve::Brittle), "a pick");
    drop(s);
    // (a row is held to its row's range, by the row's own scale)
    let s = depot("", &format!(r#"{{"rows":{{"{ROW}":99}}}}"#));
    assert_eq!(knobs::get(&s.game, ROW), Some(row_top), "a kept row is held to its row's range");
}

/// THE RULE: the address is over what is kept, field by field; and a level the address says keeps the levers kept, which stand over it. A FAILURE MEANS: a link does not play what it says on a machine
/// that keeps something. The control is the test above: the same record with no word beside it.
#[test]
fn the_address_is_over_each_field() {
    let _one = one();
    let s = depot("&difficulty=standard", r#"{"hard":true,"levers":{"ai":"hard"}}"#);
    assert_eq!(s.game.m.difficulty.base, game::difficulty::Difficulty::Standard, "the level");
    assert_eq!(lever_set(&s.game, LeverKey::Ai), Some("hard"), "a lever kept stands under the level the address says");
    drop(s);
    let s = depot("&enemyai=standard", r#"{"levers":{"ai":"hard"}}"#);
    assert_eq!(lever_set(&s.game, LeverKey::Ai), Some("standard"), "a lever");
    drop(s);
    let s = depot("&nokit", &format!(r#"{{"cards":{}}}"#, kept::text(CARDS)));
    assert_ne!(card(&s), "mp7", "the cards");
    drop(s);
    let s = depot(&format!("&knob.{ROW}=1.5"), &format!(r#"{{"rows":{{"{ROW}":2.5}}}}"#));
    assert_eq!(knobs::get(&s.game, ROW), Some(1.5), "a row, by the number's name");
    drop(s);
    let s = depot("&nerve=steady", r#"{"picks":{"nerve":"brittle"}}"#);
    assert!(matches!(s.game.k.morale.nerve, game::morale::Nerve::Steady), "a pick");
}

/// THE RULE: a native start reads the address's short words, as a page's start does, with a kept row or none beside them. A FAILURE MEANS: a test or a server stands a link on other numbers than a page.
/// The control is the same link with no word.
#[test]
fn a_short_word_turns_its_number_on_a_native_start_with_a_kept_row_beside_it() {
    let _one = one();
    let s = depot("", "");
    assert_eq!(knobs::get(&s.game, "cmdring.twice"), Some(1.0));
    drop(s);
    let s = depot("&qtwice=0", &format!(r#"{{"rows":{{"{ROW}":2.5}}}}"#));
    assert_eq!(knobs::get(&s.game, "cmdring.twice"), Some(0.0), "the word");
    assert_eq!(knobs::get(&s.game, ROW), Some(2.5), "the row beside it");
}

/// THE RULE: a lesson stands on the standard rules whatever the machine keeps, and hard only by `lessonhard`. A FAILURE MEANS: a tutorial taught on rules it does not teach. The control is the
/// same record on a mission, above.
#[test]
fn a_lesson_stands_on_the_standard_rules() {
    let _one = one();
    let text = r#"{"hard":true,"levers":{"ai":"hard"}}"#;
    let s = stood("tut-road.json", LESSON, text);
    assert_eq!(s.game.m.difficulty.base, game::difficulty::Difficulty::Standard, "the level");
    assert_eq!(lever_set(&s.game, LeverKey::Ai), None, "a lever");
    drop(s);
    let s = stood("tut-road.json", &format!("{LESSON}&lessonhard"), text);
    assert_eq!(s.game.m.difficulty.base, game::difficulty::Difficulty::Hard, "the level, by the word");
    assert_eq!(lever_set(&s.game, LeverKey::Ai), None, "a lever, still");
}

/// THE RULE: a text that is no such record is refused, and says which field; what is absent is nothing kept; a rung, a lever or a row's name that is none is nothing. A FAILURE MEANS: a broken record
/// stands a game on half of what it says, or a stale word in a store stops a start.
#[test]
fn a_text_that_is_no_such_record_is_refused_by_the_fields_name() {
    let _one = one();
    assert_eq!(Kept::read("").unwrap(), Kept::default());
    assert_eq!(Kept::read("{}").unwrap(), Kept::default());
    for (text, says) in [("nothing", "JSON"), ("[1]", "record"), ("3", "record"), (r#"{"what":1}"#, "`what`"), (r#"{"hard":"yes"}"#, "`hard`"), (r#"{"levers":3}"#, "`levers`"), (r#"{"levers":{"ai":3}}"#, "`levers`"), (r#"{"cards":{}}"#, "`cards`"), (r#"{"quietFog":1}"#, "`quietFog`"), (r#"{"quietCams":1}"#, "`quietCams`"), (r#"{"rows":{"a":"b"}}"#, "`rows`"), (r#"{"picks":{"a":1}}"#, "`picks`")] {
        let why = Kept::read(text).expect_err(text);
        assert!(why.contains(says), "{text}: {why}");
    }
    // (no such lever is no row of the record; no such rung, row or pick is nothing on the game)
    assert_eq!(Kept::read(r#"{"levers":{"nothing":"hard"}}"#).unwrap().levers, Vec::new());
    let s = depot("", r#"{"levers":{"ai":"nonsense"},"quietFog":"no rung","rows":{"no.such.row":1,"cmdring.twice":0},"picks":{"no.such.pick":"a","nerve":"no such word"}}"#);
    assert_eq!(lever_set(&s.game, LeverKey::Ai), None);
    assert_eq!(s.game.m.quietfog.pick, None);
    assert_eq!(knobs::get(&s.game, "cmdring.twice"), Some(1.0), "only a kept row of the panel is a row");
    assert!(matches!(s.game.k.morale.nerve, game::morale::Nerve::Normal));
}

/// THE RULE: the desk is laid what a start is, by the same function, and a record that is none is told to its host. A FAILURE MEANS: the cards show what the game will not stage.
#[test]
fn the_desk_is_laid_the_same_record() {
    let mut d = Desk::new();
    assert!(d.machine("[]", "").is_err());
    d.machine(&format!(r#"{{"hard":true,"cards":{},"quietCams":false}}"#, kept::text(CARDS)), &format!("{LINK}&nokit&qtwice=0")).expect("a record");
    assert_eq!(d.g.m.difficulty.base, game::difficulty::Difficulty::Hard);
    assert_ne!(d.g.m.kit.kit.members[1].primary, "mp7", "the address's word");
    assert_eq!(d.g.m.quietfog.cams, Some(false));
    assert_eq!(knobs::get(&d.g, "cmdring.twice"), Some(0.0));
    let mut d = Desk::new();
    d.machine(&format!(r#"{{"cards":{}}}"#, kept::text(CARDS)), "").expect("a record");
    assert_eq!(d.g.m.kit.kit.members[1].primary, "mp7");
}
