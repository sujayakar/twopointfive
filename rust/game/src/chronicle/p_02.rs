//! src/game/chronicle.ts lines 459 to 529
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

type Data = JsMap<String, Option<ChronEventDataValue>>;

/// A field of a row: absent and undefined are one.
fn field<'a>(d: &'a Data, k: &str) -> Option<&'a ChronEventDataValue> {
    d.get(String::from(k)).and_then(|v| v.as_ref())
}

/// A number as JavaScript writes it (`String(x)`, `${x}`).
fn num_text(x: f64) -> String {
    if x.is_nan() { return String::from("NaN"); }
    if x == 0.0 { return String::from("0"); }
    if !x.is_finite() { return String::from(if x < 0.0 { "-Infinity" } else { "Infinity" }); }
    let sign = if x < 0.0 { "-" } else { "" };
    let sci = format!("{:e}", if x < 0.0 { -x } else { x });
    let (mant, exp) = sci.split_once('e').unwrap();
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let n = exp.parse::<i32>().unwrap() + 1;
    if k <= n && n <= 21 {
        let mut s = format!("{sign}{digits}");
        for _ in 0..n - k { s.push('0'); }
        return s;
    }
    if 0 < n && n <= 21 {
        let (a, b) = digits.split_at(n as usize);
        return format!("{sign}{a}.{b}");
    }
    if -6 < n && n <= 0 {
        let mut s = format!("{sign}0.");
        for _ in 0..-n { s.push('0'); }
        s.push_str(&digits);
        return s;
    }
    let e = n - 1;
    let es = if e < 0 { '-' } else { '+' };
    let ea = if e < 0 { -e } else { e };
    if k == 1 { return format!("{sign}{digits}e{es}{ea}"); }
    let (a, b) = digits.split_at(1);
    format!("{sign}{a}.{b}e{es}{ea}")
}

/// `String(d.k)` and `${d.k}`: a missing field writes 'undefined'.
fn text(d: &Data, k: &str) -> String {
    match field(d, k) {
        None => String::from("undefined"),
        Some(ChronEventDataValue::Text(s)) => s.clone(),
        Some(ChronEventDataValue::Num(x)) => num_text(*x),
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

/// How a man was put out, in a row's words (chronicle.ts:480 and :491, one chain).
fn put_out_how(g: &mut Game, d: &Data) -> String {
    if is(d, "method", "choke") { String::from("choked out") }
    else if is(d, "method", "whip") { String::from("pistol-whipped") }
    else if is(d, "method", "ring") { crate::gear::gear_text(g, "a ring to the head") }
    else if is(d, "method", "charge") { String::from("the wall charge") }
    else { String::from("the takedown") }
}

/// The name a door row's door was written, asked when the row is read. None = the row reads its id's words.   (chronicle.ts doorWritten)
pub fn door_written(g: &mut crate::game::Game, door: &str) -> Option<String> {
    let d = match crate::doors::doors_by_name(g, door) {
        Some(d) => Some(d),
        None => crate::doors::doors_by_name(g, &door.replace(' ', "_")),
    };
    match d {
        Some(d) => crate::doornames::door_written_name(g, d),
        None => None,
    }
}

/// A row's words: the debrief card's timeline.   (chronicle.ts eventText)
pub fn event_text(g: &mut crate::game::Game, e: &crate::chronicle::ChronEvent) -> String {
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
            let times = if n > 1.0 { format!(" ×{}", num_text(n)) } else { String::new() };
            let more = if k > 3.0 { format!(" +{}", num_text(k - 3.0)) } else { String::new() };
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
            let times = if n > 1.0 { format!(" ×{}", num_text(n)) } else { String::new() };
            let weapon = crate::gear::gear_text(g, &s("weapon"));
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
        ChronKind::Ko => {
            if is(d, "method", "wounded") {
                format!("{} shot — down wounded, out of the fight", s("who"))
            } else {
                format!("{} out cold — {}", s("who"), put_out_how(g, d))
            }
        }
        ChronKind::Kill => {
            if is(d, "weapon", "knife") {
                format!("{} killed — the knife", s("who"))
            } else if is(d, "weapon", "a wall charge") {
                format!("{} killed — the wall charge", s("who"))
            } else {
                format!("{} shot dead ({})", s("who"), crate::gear::gear_text(g, &s("weapon")))
            }
        }
        ChronKind::BodyFound => {
            let how = if on(d, "wounded") { "shot and breathing" } else if on(d, "breathing") { "out cold" } else { "dead" };
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
        ChronKind::CivilianKO => format!("{} (the {}) out cold — {} — a CIVILIAN", s("who"), s("label"), put_out_how(g, d)),
        ChronKind::CivilianKilled => format!("{} (the {}) KILLED ({}) — a civilian", s("who"), s("label"), crate::gear::gear_text(g, &s("weapon"))),
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

/// `PIN[kind]`: the table's own row of a kind.
fn pin_of(p: &PIN, kind: ChronKind) -> Option<&PINDown> {
    match kind {
        ChronKind::Down => p.down.as_ref(),
        ChronKind::Shot => p.shot.as_ref(),
        ChronKind::Blackout => p.blackout.as_ref(),
        ChronKind::Heard => p.heard.as_ref(),
        ChronKind::Objective => p.objective.as_ref(),
        ChronKind::Grab => p.grab.as_ref(),
        ChronKind::Ocp => p.ocp.as_ref(),
        ChronKind::Thrown => p.thrown.as_ref(),
        ChronKind::Radio => p.radio.as_ref(),
        ChronKind::Spotted => p.spotted.as_ref(),
        ChronKind::Contact => p.contact.as_ref(),
        ChronKind::Lost => p.lost.as_ref(),
        ChronKind::Calm => p.calm.as_ref(),
        ChronKind::Alarm => p.alarm.as_ref(),
        ChronKind::LightShot => p.light_shot.as_ref(),
        ChronKind::Interrogated => p.interrogated.as_ref(),
        ChronKind::KeyTaken => p.key_taken.as_ref(),
        ChronKind::Ko => p.ko.as_ref(),
        ChronKind::Kill => p.kill.as_ref(),
        ChronKind::BodyFound => p.body_found.as_ref(),
        ChronKind::DoorKicked => p.door_kicked.as_ref(),
        ChronKind::LockPicked => p.lock_picked.as_ref(),
        ChronKind::Hit => p.hit.as_ref(),
        ChronKind::NeedBreak => p.need_break.as_ref(),
        ChronKind::CivilianSpotted => p.civilian_spotted.as_ref(),
        ChronKind::CivilianReported => p.civilian_reported.as_ref(),
        ChronKind::CivilianComplied => p.civilian_complied.as_ref(),
        ChronKind::CivilianKO => p.civilian_ko.as_ref(),
        ChronKind::CivilianKilled => p.civilian_killed.as_ref(),
        ChronKind::CameraSpotted => p.camera_spotted.as_ref(),
        ChronKind::CameraAlarm => p.camera_alarm.as_ref(),
        ChronKind::LaserTripped => p.laser_tripped.as_ref(),
        ChronKind::DeviceDown => p.device_down.as_ref(),
        ChronKind::CodeLearned => p.code_learned.as_ref(),
        ChronKind::CodeEntered => p.code_entered.as_ref(),
        ChronKind::Checkpoint => p.checkpoint.as_ref(),
    }
}

/// Does this row get a pin on the floor map (an alarm going down does not).   (chronicle.ts pinned)
pub fn pinned(e: &crate::chronicle::ChronEvent) -> bool {
    pin_of(&pin(), e.kind).is_some() && !(e.kind == ChronKind::Alarm && !on(&e.data, "up"))
}

#[cfg(test)]
mod tests {
    //! Expected strings printed by the TypeScript's own eventText, pinned and String(x) under bun. Rows that reach gear_text or a door's name are
    //! left out: those are other parts' stubs here.
    use super::*;
    use ChronEventDataValue::{Bool as B, Num as N};

    #[allow(non_snake_case)]
    fn T(s: &str) -> ChronEventDataValue { ChronEventDataValue::Text(String::from(s)) }

    fn row(kind: ChronKind, room: &str, fields: Vec<(&str, ChronEventDataValue)>) -> ChronEvent {
        let mut e = ChronEvent::zeroed();
        e.kind = kind;
        e.room = String::from(room);
        for (k, v) in fields { e.data.set(String::from(k), Some(v)); }
        e
    }

    #[test]
    fn rows_read_as_the_typescript_words_them() {
        let mut g = Game::blank();
        let rows: Vec<(ChronKind, &str, Vec<(&str, ChronEventDataValue)>, &str)> = vec![
        (ChronKind::Objective, "hall", vec![("text", T("the drive is out"))], "the drive is out"),
        (ChronKind::Objective, "hall", vec![], "undefined"),
        (ChronKind::Spotted, "hall", vec![("by", T("Kowalski")), ("lvl", N(0.0))], "SPOTTED by Kowalski"),
        (ChronKind::Spotted, "hall", vec![("by", T("Kowalski")), ("lvl", N(1.0))], "SPOTTED by Kowalski — and the floor was already up"),
        (ChronKind::Spotted, "hall", vec![], "SPOTTED by undefined"),
        (ChronKind::Contact, "hall", vec![("who", T("Kowalski")), ("cause", T("sight"))], "Kowalski: CONTACT — eyes on you"),
        (ChronKind::Contact, "hall", vec![("who", T("Kowalski")), ("cause", T("mystery"))], "Kowalski: CONTACT — mystery"),
        (ChronKind::Contact, "hall", vec![("who", T("Kowalski"))], "Kowalski: CONTACT — undefined"),
        (ChronKind::Heard, "hall", vec![("what", T("a door")), ("by", T("Ivan")), ("k", N(1.0)), ("n", N(1.0)), ("react", T("pricks up"))], "heard: a door · Ivan pricks up his ears"),
        (ChronKind::Heard, "hall", vec![("what", T("a door")), ("by", T("Ivan")), ("k", N(1.0)), ("n", N(3.0)), ("react", T("investigates"))], "heard: a door ×3 · Ivan investigates"),
        (ChronKind::Heard, "hall", vec![("what", T("a door")), ("by", T("Ivan")), ("k", N(2.0)), ("n", N(1.0)), ("react", T("alerted"))], "heard: a door · Ivan alerted"),
        (ChronKind::Heard, "hall", vec![("what", T("a door")), ("by", T("Ivan")), ("k", N(3.0)), ("n", N(1.0)), ("react", T("investigates"))], "heard: a door · Ivan investigate"),
        (ChronKind::Heard, "hall", vec![("what", T("a door")), ("by", T("Ivan")), ("k", N(5.0)), ("n", N(2.0)), ("react", T("pricks up"))], "heard: a door ×2 · Ivan +2 prick up their ears"),
        (ChronKind::Heard, "hall", vec![("what", T("a door")), ("by", T("Ivan")), ("k", N(0.0)), ("n", N(0.0)), ("react", T("x"))], "heard: a door · Ivan x"),
        (ChronKind::Heard, "hall", vec![("what", T("a door")), ("by", T("Ivan")), ("k", T("4")), ("n", T("2")), ("react", T("x"))], "heard: a door ×2 · Ivan +1 prick up their ears"),
        (ChronKind::Lost, "hall", vec![("who", T("A"))], "A lost you — searching"),
        (ChronKind::Calm, "hall", vec![("who", T("A"))], "A gives up the search"),
        (ChronKind::Alarm, "hall", vec![("stage", N(1.0)), ("up", B(true)), ("cause", T("body"))], "ALARM ▲ heightened — a body"),
        (ChronKind::Alarm, "hall", vec![("stage", N(2.0)), ("up", B(true)), ("cause", T(""))], "ALARM ▲ lockdown"),
        (ChronKind::Alarm, "hall", vec![("stage", N(7.0)), ("up", B(true)), ("cause", T("odd"))], "ALARM ▲ 7 — odd"),
        (ChronKind::Alarm, "hall", vec![("stage", N(0.0)), ("up", B(false)), ("cause", T(""))], "alarm ▼ calm"),
        (ChronKind::Alarm, "hall", vec![("stage", N(1.5)), ("up", B(false))], "alarm ▼ 1.5"),
        (ChronKind::Alarm, "hall", vec![("up", B(false))], "alarm ▼ undefined"),
        (ChronKind::Radio, "hall", vec![("who", T("A")), ("text", T("moving"))], "A (radio): “moving”"),
        (ChronKind::LightShot, "hall", vec![("name", T("the lamp"))], "shot out the lamp"),
        (ChronKind::Ocp, "hall", vec![("what", T("a torch"))], "OCP → a torch"),
        (ChronKind::Blackout, "hall", vec![("cause", T("shot"))], "shot the breaker — mains gone for good"),
        (ChronKind::Blackout, "hall", vec![("cause", T("hand"))], "threw the breaker by hand"),
        (ChronKind::Blackout, "hall", vec![("cause", T("ocp"))], "OCP → the breaker: mains tripped"),
        (ChronKind::Thrown, "hall", vec![("kind", T("cam"))], "placed a sticky camera"),
        (ChronKind::Thrown, "hall", vec![("kind", T("he40"))], "fired a 40 mm grenade"),
        (ChronKind::Thrown, "hall", vec![("kind", T("frag"))], "threw a frag grenade"),
        (ChronKind::Thrown, "hall", vec![("kind", T("flash"))], "threw a stun canister"),
        (ChronKind::Thrown, "hall", vec![("kind", T("smoke"))], "threw a smoke canister"),
        (ChronKind::Grab, "hall", vec![("who", T("A"))], "grabbed A from behind"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("civilian", B(true)), ("gave", T("code")), ("she", T("he"))], "questioned A — he gave up a door code"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("civilian", B(true)), ("gave", T("code"))], "questioned A — she gave up a door code"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("civilian", B(true)), ("gave", T("")), ("her", T("him"))], "questioned A — nothing more in him"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("civilian", B(true)), ("gave", T(""))], "questioned A — nothing more in her"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("gave", T("odd")), ("stage", N(2.0))], "questioned A — he gave up odd"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("gave", T("routes")), ("stage", N(2.0))], "questioned A — he gave up a colleague's route"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("gave", T("")), ("stage", N(4.0))], "questioned A — nothing left in him"),
        (ChronKind::Interrogated, "hall", vec![("who", T("A")), ("gave", T("")), ("stage", N(3.0))], "questioned A — nothing useful yet"),
        (ChronKind::KeyTaken, "hall", vec![("who", T("A"))], "took A's key"),
        (ChronKind::Ko, "hall", vec![("who", T("A")), ("method", T("wounded"))], "A shot — down wounded, out of the fight"),
        (ChronKind::Ko, "hall", vec![("who", T("A")), ("method", T("choke"))], "A out cold — choked out"),
        (ChronKind::Ko, "hall", vec![("who", T("A")), ("method", T("whip"))], "A out cold — pistol-whipped"),
        (ChronKind::Ko, "hall", vec![("who", T("A")), ("method", T("charge"))], "A out cold — the wall charge"),
        (ChronKind::Ko, "hall", vec![("who", T("A")), ("method", T("hand"))], "A out cold — the takedown"),
        (ChronKind::Kill, "hall", vec![("who", T("A")), ("weapon", T("knife"))], "A killed — the knife"),
        (ChronKind::Kill, "hall", vec![("who", T("A")), ("weapon", T("a wall charge"))], "A killed — the wall charge"),
        (ChronKind::BodyFound, "hall", vec![("by", T("A")), ("who", T("B")), ("wounded", B(true))], "A found B shot and breathing"),
        (ChronKind::BodyFound, "hall", vec![("by", T("A")), ("who", T("B")), ("breathing", B(true)), ("civilian", B(true))], "A found B out cold — the floor goes up hard"),
        (ChronKind::BodyFound, "hall", vec![("by", T("A")), ("who", T("B"))], "A found B dead"),
        (ChronKind::Hit, "hall", vec![("by", T("A"))], "hit — A's round"),
        (ChronKind::Hit, "hall", vec![], "hit"),
        (ChronKind::Down, "hall", vec![("by", T("A"))], "DOWN — A's round"),
        (ChronKind::Down, "hall", vec![("by", T("the wall charge"))], "DOWN — the wall charge"),
        (ChronKind::Down, "hall", vec![], "DOWN"),
        (ChronKind::NeedBreak, "hall", vec![("what", T("left")), ("who", T("A")), ("kind", T("coffee"))], "A left his route — a coffee at the break-room counter"),
        (ChronKind::NeedBreak, "hall", vec![("what", T("left")), ("who", T("A")), ("kind", T("odd"))], "A left his route — odd"),
        (ChronKind::NeedBreak, "hall", vec![("what", T("left")), ("who", T("A")), ("kind", T("smoke")), ("own", B(true)), ("to", T("the terrace"))], "A left his route — a smoke (the terrace)"),
        (ChronKind::NeedBreak, "hall", vec![("what", T("back")), ("who", T("A")), ("away", N(12.5))], "A back on his route (12.5 s away)"),
        (ChronKind::NeedBreak, "hall", vec![("what", T("broken")), ("who", T("A")), ("kind", T("water")), ("why", T("alarm"))], "A's water break broken off (alarm)"),
        (ChronKind::CivilianSpotted, "hall", vec![("who", T("A")), ("cause", T("gun"))], "A saw you, gun in hand"),
        (ChronKind::CivilianSpotted, "hall", vec![("who", T("A")), ("cause", T("odd"))], "A saw you"),
        (ChronKind::CivilianReported, "hall", vec![("who", T("A")), ("how", T("phone")), ("cause", T("bang")), ("sent", N(2.0))], "A rang it in — heard a stun charge — 2 sent"),
        (ChronKind::CivilianReported, "hall", vec![("who", T("A")), ("how", T("phone")), ("cause", T("odd"))], "A rang it in — odd"),
        (ChronKind::CivilianReported, "hall", vec![("who", T("A")), ("how", T("outside")), ("sent", N(0.0))], "A got out and control rang the floor"),
        (ChronKind::CivilianReported, "hall", vec![("who", T("A")), ("how", T("outside")), ("sent", N(3.0))], "A got out and control rang the floor — 3 sent"),
        (ChronKind::CivilianReported, "hall", vec![("who", T("A")), ("how", T("told")), ("to", T("B")), ("cause", T("noise"))], "A told B — a crash beside her"),
        (ChronKind::CivilianReported, "hall", vec![("who", T("A")), ("how", T("found")), ("to", T("B")), ("cause", T("scream"))], "A told B — heard the screaming (he found her hiding)"),
        (ChronKind::CivilianComplied, "hall", vec![("who", T("A"))], "told A to stay down — and let her go"),
        (ChronKind::CivilianKO, "hall", vec![("who", T("A")), ("label", T("clerk")), ("method", T("choke"))], "A (the clerk) out cold — choked out — a CIVILIAN"),
        (ChronKind::CivilianKO, "hall", vec![("who", T("A")), ("label", T("clerk")), ("method", T("charge"))], "A (the clerk) out cold — the wall charge — a CIVILIAN"),
        (ChronKind::CivilianKO, "hall", vec![("who", T("A")), ("label", T("clerk")), ("method", T("hand"))], "A (the clerk) out cold — the takedown — a CIVILIAN"),
        (ChronKind::CameraSpotted, "hall", vec![("n", N(3.0)), ("sent", N(2.0))], "camera 3 called you in — 2 sent to look"),
        (ChronKind::CameraSpotted, "hall", vec![("n", N(3.0))], "camera 3 called you in"),
        (ChronKind::CameraAlarm, "hall", vec![("n", N(1.0))], "camera 1 had you long enough — the floor goes up"),
        (ChronKind::LaserTripped, "vault", vec![("by", T("player"))], "broke the vault beam"),
        (ChronKind::LaserTripped, "vault", vec![("by", T("body")), ("sent", N(1.0))], "the body you hauled broke the vault beam — 1 sent"),
        (ChronKind::LaserTripped, "vault", vec![("by", T("flash"))], "a thrown stun broke the vault beam"),
        (ChronKind::LaserTripped, "vault", vec![("by", T("smoke")), ("sent", N(2.0))], "a thrown smoke broke the vault beam — 2 sent"),
        (ChronKind::DeviceDown, "hall", vec![("device", T("camera_two")), ("sent", N(2.0))], "camera two shot dead — 2 sent to it"),
        (ChronKind::DeviceDown, "hall", vec![("device", T("a_b_c"))], "a b c shot dead"),
        (ChronKind::Checkpoint, "hall", vec![("what", T("load")), ("label", T("gate"))], "checkpoint loaded (gate)"),
        (ChronKind::Checkpoint, "hall", vec![("what", T("save"))], "checkpoint saved"),
        ];
        for (kind, room, fields, want) in rows {
            assert_eq!(event_text(&mut g, &row(kind, room, fields)), want);
        }
    }

    #[test]
    fn numbers_write_as_javascript_writes_them() {
        let rows: Vec<(f64, &str)> = vec![
        (f64::from_bits(0x0), "0"),
        (f64::from_bits(0x8000000000000000), "0"),
        (f64::from_bits(0x3ff0000000000000), "1"),
        (f64::from_bits(0xbff0000000000000), "-1"),
        (f64::from_bits(0x4029000000000000), "12.5"),
        (f64::from_bits(0x3fb999999999999a), "0.1"),
        (f64::from_bits(0x3fd5555555555555), "0.3333333333333333"),
        (f64::from_bits(0x4059000000000000), "100"),
        (f64::from_bits(0x419d6f3454000000), "123456789"),
        (f64::from_bits(0x4415af1d78b58c40), "100000000000000000000"),
        (f64::from_bits(0x444b1ae4d6e2ef50), "1e+21"),
        (f64::from_bits(0x4454542ba12a337c), "1.5e+21"),
        (f64::from_bits(0x45246a317b18cbde), "1.234e+25"),
        (f64::from_bits(0x3eb0c6f7a0b5ed8d), "0.000001"),
        (f64::from_bits(0x3eb92a737110e454), "0.0000015"),
        (f64::from_bits(0x3e7ad7f29abcaf48), "1e-7"),
        (f64::from_bits(0x3e80c6f7a0b5ed8d), "1.25e-7"),
        (f64::from_bits(0xbe25798ee2308c3a), "-2.5e-9"),
        (f64::from_bits(0x1), "5e-324"),
        (f64::from_bits(0x7fefffffffffffff), "1.7976931348623157e+308"),
        (f64::from_bits(0x7ff8000000000000), "NaN"),
        (f64::from_bits(0x7ff0000000000000), "Infinity"),
        (f64::from_bits(0xfff0000000000000), "-Infinity"),
        ];
        for (x, want) in rows { assert_eq!(num_text(x), want); }
    }

    #[test]
    fn pins_are_the_typescript_pins() {
        let rows: Vec<(ChronKind, bool, bool)> = vec![
        (ChronKind::Objective, false, false),
        (ChronKind::Spotted, true, true),
        (ChronKind::Contact, false, false),
        (ChronKind::Heard, false, false),
        (ChronKind::Lost, false, false),
        (ChronKind::Calm, false, false),
        (ChronKind::Alarm, true, false),
        (ChronKind::Radio, false, false),
        (ChronKind::Shot, false, false),
        (ChronKind::LightShot, true, true),
        (ChronKind::Ocp, true, true),
        (ChronKind::Blackout, true, true),
        (ChronKind::Thrown, false, false),
        (ChronKind::Grab, false, false),
        (ChronKind::Interrogated, false, false),
        (ChronKind::KeyTaken, false, false),
        (ChronKind::Ko, true, true),
        (ChronKind::Kill, true, true),
        (ChronKind::BodyFound, true, true),
        (ChronKind::DoorKicked, true, true),
        (ChronKind::LockPicked, true, true),
        (ChronKind::Hit, false, false),
        (ChronKind::Down, true, true),
        (ChronKind::NeedBreak, false, false),
        (ChronKind::CivilianSpotted, true, true),
        (ChronKind::CivilianReported, true, true),
        (ChronKind::CivilianComplied, false, false),
        (ChronKind::CivilianKO, true, true),
        (ChronKind::CivilianKilled, true, true),
        (ChronKind::CameraSpotted, true, true),
        (ChronKind::CameraAlarm, true, true),
        (ChronKind::LaserTripped, true, true),
        (ChronKind::DeviceDown, false, false),
        (ChronKind::CodeLearned, false, false),
        (ChronKind::CodeEntered, false, false),
        (ChronKind::Checkpoint, false, false),
        ];
        for (kind, with_up, without) in rows {
            assert_eq!(pinned(&row(kind, "hall", vec![("up", B(true))])), with_up);
            assert_eq!(pinned(&row(kind, "hall", vec![])), without);
        }
    }
}
