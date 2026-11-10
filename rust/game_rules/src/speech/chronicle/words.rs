use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

type Data = JsMap<String, Option<ChronEventDataValue>>;

/// A field of a row: absent and undefined are one.
pub(super) fn field<'a>(d: &'a Data, k: &str) -> Option<&'a ChronEventDataValue> {
    d.get(String::from(k)).and_then(|v| v.as_ref())
}

/// `String(d.k)` and `${d.k}`: a missing field writes 'undefined'.
fn text(d: &Data, k: &str) -> String {
    match field(d, k) {
        None => String::from("undefined"),
        Some(ChronEventDataValue::Text(s)) => s.clone(),
        Some(ChronEventDataValue::Num(x)) => js::num_str(*x),
        Some(ChronEventDataValue::Bool(b)) => String::from(if *b { "true" } else { "false" }),
    }
}

/// `d.k` as a condition.
fn on(d: &Data, k: &str) -> bool {
    match field(d, k) {
        None => false,
        Some(ChronEventDataValue::Text(s)) => !s.is_empty(),
        Some(ChronEventDataValue::Num(x)) => js::truthy(*x),
        Some(ChronEventDataValue::Bool(b)) => *b,
    }
}

/// `d.k === 'word'`.
fn is(d: &Data, k: &str, word: &str) -> bool {
    matches!(field(d, k), Some(ChronEventDataValue::Text(s)) if s == word)
}

/// `Number(d.k)`. A text field is read as a decimal only: no writer puts text in the fields asked (notes, Traps).
fn number(d: &Data, k: &str) -> f64 {
    match field(d, k) {
        None => f64::NAN,
        Some(ChronEventDataValue::Num(x)) => *x,
        Some(ChronEventDataValue::Bool(b)) => if *b { 1.0 } else { 0.0 },
        Some(ChronEventDataValue::Text(s)) => {
            let t = s.trim();
            if t.is_empty() { return 0.0; }
            match t {
                "Infinity" | "+Infinity" => return f64::INFINITY,
                "-Infinity" => return f64::NEG_INFINITY,
                _ => {}
            }
            if !t.chars().all(|c| c.is_ascii_digit() || matches!(c, '+' | '-' | '.' | 'e' | 'E')) { return f64::NAN; }
            t.parse::<f64>().unwrap_or(f64::NAN)
        }
    }
}

/// `TABLE[String(d.k)] ?? d.k`.
fn word_or_own(table: &JsMap<String, String>, d: &Data, k: &str) -> String {
    let own = text(d, k);
    match table.get(own.clone()) {
        Some(w) => w.clone(),
        None => own,
    }
}

/// `STAGE_NAME[Number(d.stage)] ?? d.stage`.
fn stage_words(d: &Data) -> String {
    let names = stage_name();
    let x = number(d, "stage");
    let i = x as usize;
    if i as f64 == x && i < names.len() { names[i].clone() } else { text(d, "stage") }
}

/// How a man was put out, in a row's words.
fn put_out_how(d: &Data) -> String {
    if is(d, "method", "choke") { String::from("choked out") }
    else if is(d, "method", "whip") { String::from("pistol-whipped") }
    else if is(d, "method", "ring") { crate::gear::gear_text("a ring to the head") }
    else if is(d, "method", "charge") { String::from("the wall charge") }
    else { String::from("the takedown") }
}

/// is this row the floor's alarm going down (the card gives such a row no pin)
pub fn alarm_down(e: &crate::chronicle::ChronEvent) -> bool { e.kind == ChronKind::Alarm && !on(&e.data, "up") }

/// A row's words: the debrief card's timeline.
pub fn event_text(g: &crate::game::Game, e: &crate::chronicle::ChronEvent) -> String {
    let d = &e.data;
    let n = js::or(number(d, "n"), 1.0);
    let s = |k: &str| text(d, k);
    let when = |k: &str, then: String| if on(d, k) { then } else { String::new() };
    match e.kind {
        ChronKind::Objective => s("text"),
        ChronKind::Spotted => format!("SPOTTED by {}{}", s("by"), if number(d, "lvl") >= 1.0 { " — and the floor was already up" } else { "" }),
        ChronKind::Contact => format!("{}: CONTACT — {}", s("who"), word_or_own(&cause(), d, "cause")),
        ChronKind::Heard => {
            let k = js::or(number(d, "k"), 1.0);
            let react = s("react");
            let verb = if k > 1.0 {
                String::from(if react == "alerted" { "alerted" } else if react == "investigates" { "investigate" } else { "prick up their ears" })
            } else if react == "pricks up" {
                String::from("pricks up his ears")
            } else {
                react
            };
            let times = if n > 1.0 { format!(" ×{}", js::num_str(n)) } else { String::new() };
            let more = if k > 3.0 { format!(" +{}", js::num_str(k - 3.0)) } else { String::new() };
            format!("heard: {}{} · {}{} {}", s("what"), times, s("by"), more, verb)
        }
        ChronKind::Lost => format!("{} lost you — searching", s("who")),
        ChronKind::Calm => format!("{} gives up the search", s("who")),
        ChronKind::Alarm => {
            if on(d, "up") {
                let why = when("cause", format!(" — {}", word_or_own(&cause(), d, "cause")));
                format!("ALARM ▲ {}{}", stage_words(d), why)
            } else {
                format!("alarm ▼ {}", stage_words(d))
            }
        }
        ChronKind::Radio => format!("{} (radio): “{}”", s("who"), s("text")),
        ChronKind::Shot => {
            let times = if n > 1.0 { format!(" ×{}", js::num_str(n)) } else { String::new() };
            let weapon = crate::gear::gear_text(&s("weapon"));
            let into = if is(d, "into", "geometry") {
                String::from("into the scenery")
            } else if s("into").starts_with("past ") {
                s("into")
            } else {
                format!("into {}", s("into"))
            };
            format!("fired{} ({}) — {}", times, weapon, into)
        }
        ChronKind::LightShot => format!("shot out {}", s("name")),
        ChronKind::Ocp => format!("OCP → {}", s("what")),
        ChronKind::Blackout => String::from(
            if is(d, "cause", "shot") { "shot the breaker — mains gone for good" }
            else if is(d, "cause", "hand") { "threw the breaker by hand" }
            else { "OCP → the breaker: mains tripped" },
        ),
        ChronKind::Thrown => {
            if is(d, "kind", "cam") { String::from("placed a sticky camera") }
            else if is(d, "kind", "he40") { String::from("fired a 40 mm grenade") }
            else if is(d, "kind", "frag") { String::from("threw a frag grenade") }
            else { format!("threw a {} canister", if is(d, "kind", "flash") { "stun" } else { "smoke" }) }
        }
        ChronKind::Grab => format!("grabbed {} from behind", s("who")),
        ChronKind::Interrogated => {
            let gave = word_or_own(&intel(), d, "gave");
            if on(d, "civilian") {
                let rest = if on(d, "gave") {
                    format!("{} gave up {}", if field(d, "she").is_some() { s("she") } else { String::from("she") }, gave)
                } else {
                    format!("nothing more in {}", if field(d, "her").is_some() { s("her") } else { String::from("her") })
                };
                format!("questioned {} — {}", s("who"), rest)
            } else {
                let rest = if on(d, "gave") {
                    format!("he gave up {}", gave)
                } else {
                    String::from(if number(d, "stage") >= 4.0 { "nothing left in him" } else { "nothing useful yet" })
                };
                format!("questioned {} — {}", s("who"), rest)
            }
        }
        ChronKind::KeyTaken => format!("took {}'s key", s("who")),
        ChronKind::Ko => format!("{} out cold — {}", s("who"), put_out_how(d)),
        ChronKind::Kill => {
            if is(d, "weapon", "knife") {
                format!("{} killed — the knife", s("who"))
            } else if on(d, "blast") {
                format!("{} killed — {}", s("who"), crate::gear::gear_text(&s("weapon")))
            } else {
                format!("{} shot dead ({})", s("who"), crate::gear::gear_text(&s("weapon")))
            }
        }
        ChronKind::BodyFound => {
            let how = if on(d, "breathing") { "out cold" } else { "dead" };
            format!("{} found {} {}{}", s("by"), s("who"), how, if on(d, "civilian") { " — the floor goes up hard" } else { "" })
        }
        ChronKind::DoorKicked => match door_written(g, &s("door")).filter(|w| !w.is_empty()) {
            Some(w) => format!("kicked in the {}", w),
            None => format!("kicked in the {} door", s("door")),
        },
        ChronKind::LockPicked => match door_written(g, &s("door")).filter(|w| !w.is_empty()) {
            Some(w) => format!("picked the lock on the {}", w),
            None => format!("picked the {} door's lock", s("door")),
        },
        ChronKind::Hit => format!("hit{}", when("by", format!(" — {}'s round", s("by")))),
        ChronKind::Down => {
            let by = s("by");
            let whose = if by.ends_with("charge") { by } else { format!("{}'s round", by) };
            format!("DOWN{}", when("by", format!(" — {}", whose)))
        }
        ChronKind::NeedBreak => {
            if is(d, "what", "left") {
                let to = if on(d, "own") {
                    format!("{} ({})", word_or_own(&need_short(), d, "kind"), s("to"))
                } else {
                    word_or_own(&need_words(), d, "kind")
                };
                format!("{} left his route — {}", s("who"), to)
            } else if is(d, "what", "back") {
                format!("{} back on his route ({} s away)", s("who"), s("away"))
            } else {
                format!("{}'s {} break broken off ({})", s("who"), s("kind"), s("why"))
            }
        }
        ChronKind::CivilianSpotted => {
            let what = match civ_cause().get(s("cause")) {
                Some(w) => w.clone(),
                None => String::from("saw you"),
            };
            format!("{} {}", s("who"), what)
        }
        ChronKind::CivilianReported => {
            if is(d, "how", "phone") {
                format!("{} rang it in — {}{}", s("who"), word_or_own(&civ_cause(), d, "cause"), when("sent", format!(" — {} sent", s("sent"))))
            } else if is(d, "how", "outside") {
                format!("{} got out and control rang the floor{}", s("who"), when("sent", format!(" — {} sent", s("sent"))))
            } else {
                format!("{} told {} — {}{}", s("who"), s("to"), word_or_own(&civ_cause(), d, "cause"), if is(d, "how", "found") { " (he found her hiding)" } else { "" })
            }
        }
        ChronKind::CivilianComplied => format!("told {} to stay down — and let her go", s("who")),
        ChronKind::CivilianKO => format!("{} (the {}) out cold — {} — a CIVILIAN", s("who"), s("label"), put_out_how(d)),
        ChronKind::CivilianKilled => format!("{} (the {}) KILLED ({}) — a civilian", s("who"), s("label"), crate::gear::gear_text(&s("weapon"))),
        ChronKind::CameraSpotted => format!("camera {} called you in{}", s("n"), when("sent", format!(" — {} sent to look", s("sent")))),
        ChronKind::CameraAlarm => format!("camera {} had you long enough — the floor goes up", s("n")),
        ChronKind::LaserTripped => {
            let by = if is(d, "by", "player") {
                String::from("broke")
            } else if is(d, "by", "body") {
                String::from("the body you hauled broke")
            } else {
                format!("a thrown {} broke", if is(d, "by", "flash") { String::from("stun") } else { s("by") })
            };
            format!("{} the {} beam{}", by, e.room, when("sent", format!(" — {} sent", s("sent"))))
        }
        ChronKind::DeviceDown => format!("{} shot dead{}", s("device").replace('_', " "), when("sent", format!(" — {} sent to it", s("sent")))),
        ChronKind::CodeLearned => {
            let code = match door_written(g, &s("door")).filter(|w| !w.is_empty()) {
                Some(w) => format!("code to the {}", w),
                None => format!("{} code", s("door").replace('_', " ")),
            };
            if is(d, "how", "told") {
                format!("{} gave up the {}", s("from"), code)
            } else {
                format!("overheard the {}{}", code, when("from", format!(" ({})", s("from"))))
            }
        }
        ChronKind::CodeEntered => {
            let name = match door_written(g, &s("door")) {
                Some(w) => w,
                None => format!("{} door", s("door").replace('_', " ")),
            };
            format!("keyed the code — the {}", name)
        }
        ChronKind::Checkpoint => format!("checkpoint {}{}", if is(d, "what", "load") { "loaded" } else { "saved" }, when("label", format!(" ({})", s("label")))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ChronEventDataValue::{Bool as B, Num as N};

    #[allow(non_snake_case)]
    fn T(s: &str) -> ChronEventDataValue { ChronEventDataValue::Text(String::from(s)) }

    /// What the debrief's timeline reads for a row of this kind in this room with these fields.
    fn says(kind: ChronKind, fields: Vec<(&str, ChronEventDataValue)>) -> String {
        let mut e = ChronEvent::default();
        e.kind = kind;
        e.room = String::from("vault");
        for (k, v) in fields { e.data.set(String::from(k), Some(v)); }
        event_text(&Game::blank(), &e)
    }

    /// A FAILURE MEANS: the timeline says "Ivan investigates" of four men, or does not say the sound came three times.
    #[test]
    fn a_sound_heard_is_told_with_how_often_and_how_many() {
        let heard = |k: f64, n: f64, react: &str| says(ChronKind::Heard, vec![("what", T("a door")), ("by", T("Ivan")), ("k", N(k)), ("n", N(n)), ("react", T(react))]);
        assert_eq!(heard(1.0, 1.0, "pricks up"), "heard: a door · Ivan pricks up his ears");
        assert_eq!(heard(1.0, 3.0, "investigates"), "heard: a door ×3 · Ivan investigates");
        assert_eq!(heard(3.0, 1.0, "investigates"), "heard: a door · Ivan investigate");
        assert_eq!(heard(5.0, 2.0, "pricks up"), "heard: a door ×2 · Ivan +2 prick up their ears");
    }

    /// A FAILURE MEANS: the player cannot read how the floor went up and came down.
    #[test]
    fn the_alarm_is_told_by_its_stage_going_up_with_the_cause() {
        let alarm = |stage: f64, up: bool, cause: &str| says(ChronKind::Alarm, vec![("stage", N(stage)), ("up", B(up)), ("cause", T(cause))]);
        assert_eq!(alarm(1.0, true, "body"), "ALARM ▲ heightened — a body");
        assert_eq!(alarm(2.0, true, ""), "ALARM ▲ lockdown");
        assert_eq!(alarm(0.0, false, ""), "alarm ▼ calm");
    }

    /// A FAILURE MEANS: a guard's questioning reads as a civilian's, or "nothing left" before a man has been pressed.
    #[test]
    fn a_questioning_is_told_by_what_he_gave_and_how_far_it_went() {
        let q = |f: Vec<(&str, ChronEventDataValue)>| says(ChronKind::Interrogated, f);
        assert_eq!(q(vec![("who", T("A")), ("gave", T("routes")), ("stage", N(2.0))]), "questioned A — he gave up a colleague's route");
        assert_eq!(q(vec![("who", T("A")), ("gave", T("")), ("stage", N(3.0))]), "questioned A — nothing useful yet");
        assert_eq!(q(vec![("who", T("A")), ("gave", T("")), ("stage", N(4.0))]), "questioned A — nothing left in him");
        assert_eq!(q(vec![("who", T("A")), ("civilian", B(true)), ("gave", T("code"))]), "questioned A — she gave up a door code");
        assert_eq!(q(vec![("who", T("A")), ("civilian", B(true)), ("gave", T("")), ("her", T("him"))]), "questioned A — nothing more in him");
    }

    /// A FAILURE MEANS: the timeline misnames what put a man down or who went down by what.
    #[test]
    fn a_man_out_or_down_is_told_by_what_did_it() {
        assert_eq!(says(ChronKind::Ko, vec![("who", T("A")), ("method", T("choke"))]), "A out cold — choked out");
        assert_eq!(says(ChronKind::Ko, vec![("who", T("A")), ("method", T("hand"))]), "A out cold — the takedown");
        assert_eq!(says(ChronKind::CivilianKO, vec![("who", T("A")), ("label", T("clerk")), ("method", T("charge"))]), "A (the clerk) out cold — the wall charge — a CIVILIAN");
        assert_eq!(says(ChronKind::Kill, vec![("who", T("A")), ("weapon", T("knife"))]), "A killed — the knife");
        assert_eq!(says(ChronKind::Kill, vec![("who", T("A")), ("weapon", T("the wall charge")), ("blast", B(true))]), "A killed — the wall charge");
        assert_eq!(says(ChronKind::Down, vec![("by", T("A"))]), "DOWN — A's round");
        assert_eq!(says(ChronKind::Down, vec![("by", T("the wall charge"))]), "DOWN — the wall charge");
        assert_eq!(says(ChronKind::Down, vec![]), "DOWN");
        assert_eq!(says(ChronKind::Hit, vec![]), "hit");
        assert_eq!(says(ChronKind::BodyFound, vec![("by", T("A")), ("who", T("B")), ("breathing", B(true)), ("civilian", B(true))]), "A found B out cold — the floor goes up hard");
        assert_eq!(says(ChronKind::BodyFound, vec![("by", T("A")), ("who", T("B"))]), "A found B dead");
    }

    /// A FAILURE MEANS: the player is not told how a civilian gave him away, or how many that sent.
    #[test]
    fn a_civilians_report_is_told_by_how_it_got_out() {
        let r = |f: Vec<(&str, ChronEventDataValue)>| says(ChronKind::CivilianReported, f);
        assert_eq!(r(vec![("who", T("A")), ("how", T("phone")), ("cause", T("bang")), ("sent", N(2.0))]), "A rang it in — heard a stun charge — 2 sent");
        assert_eq!(r(vec![("who", T("A")), ("how", T("outside")), ("sent", N(0.0))]), "A got out and control rang the floor");
        assert_eq!(r(vec![("who", T("A")), ("how", T("told")), ("to", T("B")), ("cause", T("noise"))]), "A told B — a crash beside her");
        assert_eq!(r(vec![("who", T("A")), ("how", T("found")), ("to", T("B")), ("cause", T("scream"))]), "A told B — heard the screaming (he found her hiding)");
        assert_eq!(says(ChronKind::CivilianSpotted, vec![("who", T("A")), ("cause", T("gun"))]), "A saw you, gun in hand");
        assert_eq!(says(ChronKind::CivilianSpotted, vec![("who", T("A")), ("cause", T("odd"))]), "A saw you");
    }

    /// A FAILURE MEANS: the player is blamed for a beam his body or his canister broke, and the row loses the room.
    #[test]
    fn a_beam_is_told_by_what_broke_it_in_its_room() {
        let by = |by: &str| says(ChronKind::LaserTripped, vec![("by", T(by))]);
        assert_eq!(by("player"), "broke the vault beam");
        assert_eq!(by("body"), "the body you hauled broke the vault beam");
        assert_eq!(by("flash"), "a thrown stun broke the vault beam");
        assert_eq!(says(ChronKind::LaserTripped, vec![("by", T("smoke")), ("sent", N(2.0))]), "a thrown smoke broke the vault beam — 2 sent");
    }

    /// A FAILURE MEANS: the timeline loses why a man left his post: a need is said in words, a room he walked to in brackets.
    #[test]
    fn a_mans_break_is_told_from_leaving_to_coming_back() {
        let need = |f: Vec<(&str, ChronEventDataValue)>| says(ChronKind::NeedBreak, f);
        assert_eq!(need(vec![("what", T("left")), ("who", T("A")), ("kind", T("coffee"))]), "A left his route — a coffee at the break-room counter");
        assert_eq!(need(vec![("what", T("left")), ("who", T("A")), ("kind", T("smoke")), ("own", B(true)), ("to", T("the terrace"))]), "A left his route — a smoke (the terrace)");
        assert_eq!(need(vec![("what", T("back")), ("who", T("A")), ("away", N(12.5))]), "A back on his route (12.5 s away)");
        assert_eq!(need(vec![("what", T("broken")), ("who", T("A")), ("kind", T("water")), ("why", T("alarm"))]), "A's water break broken off (alarm)");
    }

    /// A FAILURE MEANS: a thrown thing, a device's id or a checkpoint reads wrong on the card.
    #[test]
    fn a_thing_used_is_told_by_its_own_word() {
        assert_eq!(says(ChronKind::Thrown, vec![("kind", T("cam"))]), "placed a sticky camera");
        assert_eq!(says(ChronKind::Thrown, vec![("kind", T("flash"))]), "threw a stun canister");
        assert_eq!(says(ChronKind::Thrown, vec![("kind", T("smoke"))]), "threw a smoke canister");
        assert_eq!(says(ChronKind::Blackout, vec![("cause", T("shot"))]), "shot the breaker — mains gone for good");
        assert_eq!(says(ChronKind::DeviceDown, vec![("device", T("camera_two")), ("sent", N(2.0))]), "camera two shot dead — 2 sent to it");
        assert_eq!(says(ChronKind::Checkpoint, vec![("what", T("load")), ("label", T("gate"))]), "checkpoint loaded (gate)");
        assert_eq!(says(ChronKind::Checkpoint, vec![("what", T("save"))]), "checkpoint saved");
    }
}
