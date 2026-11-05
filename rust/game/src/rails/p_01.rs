//! src/game/rails.ts lines 1 to 213
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A step's `after` line as its order stands: given (1), or given and waiting on his word (2). (rails.ts said)
pub fn said(a: &crate::rails::SaidA, gave: f64) -> Option<String> {
    if gave == 2.0 { a.word.clone().or_else(|| a.line.clone()) } else { a.line.clone() }
}

/// A lookout as one of the fog's eyes. (rails.ts lookoutEye)
pub fn lookout_eye(id_2: f64, e: crate::rails::LookoutEyeE) -> crate::fogofwar::FogEye {
    crate::fogofwar::FogEye {
        id: id_2,
        pos: e.at,
        look: [m::sin(e.yaw), 0.0, m::cos(e.yaw)],
        at: e.at,
        fov: Some(e.fov.unwrap_or(120.0)),
        near: Some(0.0),
        wand: None,
    }
}

/// `list.includes(x)` of a list of strings.
fn includes(list: &[String], x: &str) -> bool {
    list.iter().any(|v| v == x)
}

/// A string's `.length`: UTF-16 units, not bytes.
fn js_len(s: &str) -> usize {
    s.encode_utf16().count()
}

/// A string as a condition: only the empty one is false.
fn truthy_str(s: &Option<String>) -> bool {
    s.as_ref().is_some_and(|v| !v.is_empty())
}

/// The acts a `rails.rows` list names without the act that undoes them. (rails.ts rowsWithheld)
pub fn rows_withheld(l: Option<&crate::scene_scenarios::LessonData>) -> Vec<String> {
    let Some(r) = l.and_then(|l| l.rails.as_ref()).and_then(|r| r.rows.as_ref()) else { return Vec::new() };
    let table = crate::rails::undo();
    table.iter()
        .filter(|u| includes(r, &u[0]) && !includes(r, &u[1]))
        .map(|u| alloc::format!("rails.rows names '{}' and not '{}': {}", u[0], u[1], u[2]))
        .collect()
}

/// A list of actions in a lesson block: each must be one of ACTS. (rails.ts railsFaults `bad`)
fn bad(out: &mut Vec<String>, w: &str, a: Option<&[String]>) {
    let Some(a) = a else { return };
    let acts = crate::rails::acts();
    for x in a {
        if !acts.has(x.clone()) {
            out.push(alloc::format!("{w}: no action '{x}' ({})", rails_actions().join(", ")));
        }
    }
}

/// What is wrong with a lesson block's rails keys. (rails.ts railsFaults)
pub fn rails_faults(l: &crate::scene_scenarios::LessonData) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let Some(r) = l.rails.as_ref() else {
        for (i, t) in l.steps.iter().enumerate() {
            if t.allow.is_some() || t.deny.is_some() || truthy_str(&t.cue) || truthy_str(&t.checkpoint) || t.fog.is_some() || truthy_str(&t.no_tap) || t.eye.is_some() || t.on_way.is_some() || t.explain.is_some() || t.toward.is_some() || t.after.is_some() {
                out.push(alloc::format!("steps[{i}] '{}' carries rails keys and the lesson has no rails record", t.id));
            }
        }
        return out;
    };
    bad(&mut out, "rails.allow", Some(&r.allow));
    if includes(&r.allow, "stalk") {
        out.push(String::from("rails.allow: 'stalk' is open unless a step denies it, so naming it in the frame's list means nothing (a step's deny takes it, a later step's allow gives it back)"));
    }
    let flat_ring = r.flat_ring == Some(true);
    let mut cues: JsSet<String> = JsSet::new();
    let mut open: JsSet<String> = JsSet::new();
    for a in &r.allow {
        open.add(a.clone());
    }
    let mut goal_said = false;
    for (i, t) in l.steps.iter().enumerate() {
        let w = alloc::format!("steps[{i}] '{}'", t.id);
        bad(&mut out, &alloc::format!("{w}.allow"), t.allow.as_deref());
        bad(&mut out, &alloc::format!("{w}.deny"), t.deny.as_deref());
        if let Some(allow) = &t.allow {
            for a in allow {
                open.add(a.clone());
            }
        }
        if let Some(deny) = &t.deny {
            for a in deny {
                open.delete(a.clone());
            }
        }
        if open.has(String::from("goal")) && !open.has(String::from("sprint")) && !goal_said {
            goal_said = true;
            out.push(alloc::format!("{w}: 'goal' is open here and 'sprint' is not — goal needs sprint: Shift is his sprint's key"));
        }
        if let Some(e) = &t.explain {
            let title_len = e.title.as_ref().map_or(0, |s| js_len(s));
            let text_len = js_len(&e.text);
            if text_len as f64 > CARD_CHARS || title_len > 40 {
                out.push(alloc::format!("{w}.explain: a card is a heading of at most 40 characters and at most {CARD_CHARS} of text (it is {title_len} and {text_len}): a card nobody reads teaches nothing"));
            }
            out.extend(crate::lesson::lesson_token_faults(&alloc::format!("{w}.explain.text"), &e.text, CARD_CHARS, None, None));
            if e.title.as_ref().is_some_and(|s| s.contains(['{', '}'])) {
                out.push(alloc::format!("{w}.explain.title has a brace: a card's title is shown as written, a move's token goes in its text"));
            }
        }
        let after_line = t.after.as_ref().and_then(|a| a.line.as_ref());
        let after_word = t.after.as_ref().and_then(|a| a.word.as_ref());
        let lines = [
            ("noTap", t.no_tap.as_ref()),
            ("menuLine", t.menu_line.as_ref()),
            ("pageLine", t.page_line.as_ref()),
            ("flatLine", t.flat_line.as_ref()),
            ("after.line", after_line),
            ("after.word", after_word),
        ];
        for (k, line) in lines {
            if let Some(line) = line {
                out.extend(crate::lesson::lesson_token_faults(&alloc::format!("{w}.{k}"), line, 96.0, None, None));
            }
        }
        if flat_ring && t.menu_line.is_some() && t.flat_line.is_none() {
            out.push(alloc::format!("{w}.menuLine names the menu's pages and rails.flatRing gives this mission one ring: the step wants a flatLine too"));
        }
        if !flat_ring && t.flat_line.is_some() {
            out.push(alloc::format!("{w}.flatLine: the lesson's rails.flatRing is not set, so no ring here is flat"));
        }
        if let Some(cue) = &t.cue {
            if cues.has(cue.clone()) {
                out.push(alloc::format!("{w}.cue '{cue}' is another step's too: a cue starts one beat"));
            }
            cues.add(cue.clone());
        }
    }
    if let Some(rows) = &r.rows {
        if !(!rows.is_empty() && rows.iter().all(|v| !v.is_empty())) {
            out.push(String::from("rails.rows: a list of verbs, at least one (absent: every row)"));
        }
    }
    if r.waits.is_some() {
        if r.sky.is_some() {
            out.push(String::from("rails.waits: { low?: true } (the squad holds where it starts until a step allows `order`), and never with rails.sky"));
        } else if includes(&r.allow, "order") || !l.steps.iter().any(|t| t.allow.as_ref().is_some_and(|a| includes(a, "order"))) {
            out.push(String::from("rails.waits: the wait ends at the first step that allows 'order', so one step has to and rails.allow must not (they would wait for ever, or not at all)"));
        }
    }
    if let Some(orders) = &r.orders {
        if orders.is_empty() {
            out.push(String::from("rails.orders: a list of order ids (at least one)"));
        }
    }
    if let Some(eye) = r.sky.as_ref().and_then(|s| s.eye.as_ref()) {
        if !eye.fov.map_or(true, |f| f > 0.0 && f <= 360.0) {
            out.push(String::from("rails.sky.eye: { at: a Vec3, yaw: radians, fov?: degrees up to 360 }"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_scenarios::{LessonData, LessonStep, RailsData, RailsDataSky, RailsDataSkyEye, RailsDataWaits};

    fn words(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| String::from(*s)).collect()
    }
    fn step(id: &str) -> LessonStep {
        let mut t = LessonStep::zeroed();
        t.id = String::from(id);
        t
    }
    fn framed(allow: &[&str], steps: Vec<LessonStep>) -> LessonData {
        let mut r = RailsData::zeroed();
        r.allow = words(allow);
        LessonData { title: String::new(), steps, rails: Some(r) }
    }

    #[test]
    fn said_takes_the_word_only_at_two() {
        let both = SaidA { line: Some(String::from("line")), word: Some(String::from("word")) };
        let no_word = SaidA { line: Some(String::from("line")), word: None };
        assert_eq!(said(&both, 1.0).as_deref(), Some("line"));
        assert_eq!(said(&both, 2.0).as_deref(), Some("word"));
        assert_eq!(said(&no_word, 2.0).as_deref(), Some("line"));
    }

    #[test]
    fn lookout_eye_defaults() {
        let e = lookout_eye(LOOKOUT_ID, LookoutEyeE { at: [1.0, 2.0, 3.0], yaw: 0.0, fov: None });
        assert_eq!(e.id, 199.0);
        assert_eq!(e.pos, [1.0, 2.0, 3.0]);
        assert_eq!(e.at, e.pos);
        assert_eq!(e.look, [0.0, 0.0, 1.0]);
        assert_eq!(e.fov, Some(120.0));
        assert_eq!(e.near, Some(0.0));
        assert_eq!(lookout_eye(STEP_EYE_ID, LookoutEyeE { at: [0.0; 3], yaw: 0.0, fov: Some(90.0) }).fov, Some(90.0));
    }

    #[test]
    fn rows_withheld_names_a_half_alone() {
        assert!(rows_withheld(None).is_empty());
        let mut l = framed(&[], Vec::new());
        assert!(rows_withheld(Some(&l)).is_empty());
        l.rails.as_mut().unwrap().rows = Some(words(&["push", "ease", "pull", "lightsOff"]));
        assert_eq!(rows_withheld(Some(&l)), words(&["rails.rows names 'lightsOff' and not 'lightsOn': a room's lights he puts out cannot be put on"]));
    }

    #[test]
    fn faults_without_a_frame() {
        let mut quiet = step("a");
        quiet.cue = Some(String::new());
        let mut cued = step("b");
        cued.cue = Some(String::from("go"));
        let l = LessonData { title: String::new(), steps: vec![quiet, cued], rails: None };
        assert_eq!(rails_faults(&l), words(&["steps[1] 'b' carries rails keys and the lesson has no rails record"]));
    }

    #[test]
    fn faults_of_the_actions() {
        let mut one = step("a");
        one.allow = Some(words(&["nope"]));
        let f = rails_faults(&framed(&["stalk"], vec![one]));
        assert_eq!(f.len(), 2);
        assert!(f[0].starts_with("rails.allow: 'stalk' is open"));
        assert!(f[1].starts_with("steps[0] 'a'.allow: no action 'nope' (move, pan, sprint,"));
    }

    #[test]
    fn goal_wants_sprint_and_is_said_once() {
        let mut one = step("a");
        one.allow = Some(words(&["goal"]));
        let two = step("b");
        let f = rails_faults(&framed(&[], vec![one, two]));
        assert_eq!(f, words(&["steps[0] 'a': 'goal' is open here and 'sprint' is not — goal needs sprint: Shift is his sprint's key"]));
        let mut sprints = step("a");
        sprints.allow = Some(words(&["goal"]));
        assert!(rails_faults(&framed(&["sprint"], vec![sprints])).is_empty());
    }

    #[test]
    fn a_cue_is_one_steps() {
        let mut one = step("a");
        one.cue = Some(String::from("go"));
        let mut two = step("b");
        two.cue = Some(String::from("go"));
        assert_eq!(rails_faults(&framed(&[], vec![one, two])), words(&["steps[1] 'b'.cue 'go' is another step's too: a cue starts one beat"]));
    }

    #[test]
    fn faults_of_the_frame() {
        let mut l = framed(&[], Vec::new());
        {
            let r = l.rails.as_mut().unwrap();
            r.rows = Some(words(&["push", ""]));
            r.orders = Some(Vec::new());
            let mut w = RailsDataWaits::zeroed();
            w.low = Some(true);
            r.waits = Some(w);
            let mut s = RailsDataSky::zeroed();
            let mut eye = RailsDataSkyEye::zeroed();
            eye.fov = Some(361.0);
            s.eye = Some(eye);
            r.sky = Some(s);
        }
        let f = rails_faults(&l);
        assert_eq!(f.len(), 4);
        assert!(f[0].starts_with("rails.rows:"));
        assert!(f[1].starts_with("rails.waits: { low?: true }"));
        assert!(f[2].starts_with("rails.orders:"));
        assert!(f[3].starts_with("rails.sky.eye:"));
        l.rails.as_mut().unwrap().sky = None;
        assert!(rails_faults(&l).iter().any(|x| x.starts_with("rails.waits: the wait ends")));
    }

    #[test]
    fn a_card_size_reads_as_the_typescript_wrote_it() {
        assert_eq!(alloc::format!("{CARD_CHARS}"), "260");
    }

    #[test]
    fn a_string_length_is_utf16_units() {
        assert_eq!(js_len("é€😀"), 4);
    }
}
