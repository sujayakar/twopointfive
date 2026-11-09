//! A LESSON STEP'S WORDS ARE KNOWN, OR THE LEVEL IS REFUSED: a test no row has, a flag, a slot or men no arm holds would be a step no deed ever passes. Every level file with a lesson reads; the same file
//! with one wrong word is refused where it is read (not read as a blank), and the refusal names the level, the step and the word. What points at nothing on the file's own floor, or asks what no deed
//! could answer, is the judge's to say (`faults.rs`).
use game_start::{faults, files};
use game_start::json::{self, Json};
use game_start::play::{Opts, Play};

fn opts() -> Opts { Opts { seed: 9, ..Default::default() } }
fn data_of(name: &str) -> Json { json::parse(&files::level_text(name)).unwrap_or_else(|why| panic!("{}: {}", name, why)) }

/// A FAILURE MEANS: a level file with a lesson no longer reads, so its mission cannot be stood.
#[test]
fn every_level_with_a_lesson_reads() {
    let mut names: Vec<String> = std::fs::read_dir(files::root().join("levels")).expect("the levels folder").map(|e| e.unwrap().file_name().into_string().unwrap()).filter(|n| n.ends_with(".json") && !n.ends_with(".lint.json")).collect();
    names.sort();
    let mut read = 0;
    for name in &names {
        if data_of(name).get("scenarios").get("lesson").is_null() { continue; }
        let p = Play::level(name, false, &opts());
        assert!(p.g.level.scenarios.as_ref().and_then(|s| s.lesson.as_ref()).is_some_and(|l| !l.steps.is_empty()), "{}: its lesson did not arrive", name);
        read += 1;
    }
    assert!(read >= 6, "{} levels have a lesson: the six tutorials have gone from the folder", read);
}

/// the level with its second step's test written as `done`
fn with_done(data: &Json, done: &str) -> Json {
    let mut lesson = data.get("scenarios").get("lesson").clone();
    let mut steps: Vec<Json> = lesson.get("steps").items().to_vec();
    steps[1].set_member("done", json::parse(done).unwrap());
    lesson.set_member("steps", json::arr(steps));
    let mut scen = data.get("scenarios").clone();
    scen.set_member("lesson", lesson);
    let mut out = data.clone();
    out.set_member("scenarios", scen);
    out
}

/// A FAILURE MEANS: a level with that wrong word read, or was refused in words that do not lead its author to it.
#[test]
fn a_wrong_word_of_each_kind_is_refused_by_name() {
    let data = data_of("tut-road.json");
    let link = "?levelFile=tut-road.json";
    Play::mission_of(&with_done(&data, r#"{ "is": "flag", "flag": "crouch" }"#), link, &opts());   // (the doctoring itself stands a level)
    for (done, word) in [
        (r#"{ "is": "crouched" }"#, "crouched"),
        (r#"{ "is": "flag", "flag": "ducked" }"#, "ducked"),
        (r#"{ "is": "slot", "slot": "knife" }"#, "knife"),
        (r#"{ "is": "orderTo", "who": "green" }"#, "green"),
        (r#"{ "is": "stackOn", "door": "d", "who": "mauve", "set": true }"#, "mauve"),
        (r#"{ "is": "door", "door": "hut", "state": "ajar" }"#, "ajar"),
        (r#"{ "is": "stage", "stage": "failed" }"#, "failed"),
        (r#"{ "is": "meter", "which": "heat", "below": 1 }"#, "heat"),
        (r#"{ "is": "threw", "kind": "brick" }"#, "brick"),
        (r#"{ "is": "downed", "how": "tickle" }"#, "tickle"),
        (r#"{ "is": "pair", "what": "wave" }"#, "wave"),
        (r#"{ "is": "launcher", "on": "bayonet", "fired": false }"#, "bayonet"),
        (r#"{ "is": "did", "what": "danced" }"#, "danced"),
        (r#"{ "is": "any", "of": [{ "is": "loaded" }, { "is": "all", "of": [{ "is": "slot", "slot": "spoon" }] }] }"#, "spoon"),
    ] {
        let doctored = with_done(&data, done);
        let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { Play::mission_of(&doctored, link, &opts()); })).expect_err(word);
        let text = refused.downcast_ref::<String>().cloned().unwrap_or_default();
        for said in ["tut-road", "lesson.steps[1]", word] { assert!(text.contains(said), "'{}' does not say '{}'", text, said); }
    }
}

/// A FAILURE MEANS: a step that waits on a door, a room or a route this file has not, or on what no deed could answer, goes into the tree with no word from the judge.
#[test]
fn a_test_that_points_at_nothing_is_the_judges_fault() {
    let data = data_of("tut-road.json");
    assert_eq!(faults::of(&data), Vec::<String>::new());
    for (done, word) in [
        (r#"{ "is": "door", "door": "no-such-door", "state": "open" }"#, "no-such-door"),
        (r#"{ "is": "stackOn", "door": "no-such-door", "who": "all" }"#, "no-such-door"),
        (r#"{ "is": "stacks", "doors": ["no-such-door"] }"#, "no-such-door"),
        (r#"{ "is": "did", "what": "order:stack", "on": "no-such-door" }"#, "no-such-door"),
        (r#"{ "is": "did", "what": "act:breakerOff", "on": "hut" }"#, "act:breakerOff"),
        (r#"{ "is": "taken", "room": "no-such-room" }"#, "no-such-room"),
        (r#"{ "is": "calm", "route": "no-such-route" }"#, "no-such-route"),
        (r#"{ "is": "in", "area": { "x0": 1, "x1": 1, "z0": 0, "z1": 2 } }"#, "an area that holds nothing"),
        (r#"{ "is": "near", "at": [0, 0, 0], "r": 0 }"#, "nearer than"),
        (r#"{ "is": "men", "down": 0 }"#, "at least one"),
        (r#"{ "is": "any", "of": [] }"#, "a list of no tests"),
        (r#"{ "is": "all", "of": [{ "is": "any", "of": [{ "is": "door", "door": "deep-door", "state": "open" }] }] }"#, "deep-door"),
    ] {
        let told = faults::of(&with_done(&data, done)).join("\n");
        for said in ["lesson.steps[1]", word] { assert!(told.contains(said), "'{}' does not say '{}'", told, said); }
    }
}
