//! src/game/dialogue.ts lines 1 to 376
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

fn st(s: &str) -> String { String::from(s) }

// ---------------------------------------------------------------- axes

/// Agitation to its band.   (dialogue.ts bandOf)
pub fn band_of(a: f64) -> crate::dialogue::Band {
    if a >= BAND_HIGH { Band::High } else if a >= BAND_MID { Band::Mid } else { Band::Low }
}

/// What this man knows of Sam, the highest level that holds, from his own record only.   (dialogue.ts knowledgeOf)
pub fn knowledge_of(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::Knowledge {
    if g.gd[gd].witness.saw_act.is_some() || g.gd[gd].witness.saw_held.is_some() { return Knowledge::SawHurt; }
    if g.gd[gd].last_seen_t >= 0.0 || g.gd[gd].witness.alerted_by == Some(crate::game::WitnessAlertedBy::Sight) || {
        let weapon = g.gd[gd].weapon;
        crate::weapons::firearm_spent(g, weapon) > 0.0
    } { return Knowledge::Saw; }
    if g.gd[gd].sight_t >= 0.0 { return Knowledge::Glimpsed; }
    let w = &g.gd[gd].witness;
    let h = &w.heard;
    if h.shots + h.kick + h.bang + h.small > 0.0
        || w.alerted_by == Some(crate::game::WitnessAlertedBy::Sound)
        || w.alerted_by == Some(crate::game::WitnessAlertedBy::Radio)
        || w.alerted_by == Some(crate::game::WitnessAlertedBy::Body)
        || w.saw_body.is_some()
        || w.called_to_body.is_some()
        || w.dazzled > 0.0
    { return Knowledge::Heard; }
    Knowledge::None
}

/// How rattled this man is, 0 to 1.   (dialogue.ts agitationOf)
pub fn agitation_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { agitation_terms(g, gd).a }

/// One term of the sum, and its label for the panel.   (dialogue.ts agitationTerms: add)
fn add(a: &mut f64, why: &mut Vec<String>, v: f64, label: &str) {
    if m::abs(v) < 1e-6 { return; }
    *a += v;
    if m::abs(v) >= 0.005 { why.push(format!("{} {}{:.2}", label, if v > 0.0 { "+" } else { "" }, v)); }
}

/// The sum of what has rattled this man, clamped, with its terms for the panel and the probes.   (dialogue.ts agitationTerms)
pub fn agitation_terms(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::AgitationTerms {
    use crate::game::{GuardState, Temperament};
    let t = temperament_of(g, gd);
    let talk = g.gd[gd].talk;
    let mut why: Vec<String> = Vec::new();
    let mut a = 0.0;
    let esc = crate::guards::escalation_of(g);
    add(&mut a, &mut why, 0.22 * esc, "floor");
    if g.alarm.episode && !g.quiet_utility { add(&mut a, &mut why, 0.15, "episode"); }
    let state = g.gd[gd].state;
    let alert_t = g.gd[gd].witness.alert_t;
    if state == GuardState::Alert || (alert_t >= 0.0 && g.time - alert_t < 60.0) {
        add(&mut a, &mut why, 0.2, "ownAlert");
    } else if state == GuardState::Suspicious || state == GuardState::Search {
        add(&mut a, &mut why, 0.1, "ownEdge");
    }
    if g.gd[gd].witness.saw_act.is_some() { add(&mut a, &mut why, 0.3, "sawAct"); }
    if g.gd[gd].witness.saw_body.is_some() {
        add(&mut a, &mut why, 0.2, "sawBody");
    } else if g.gd[gd].witness.called_to_body.is_some() {
        add(&mut a, &mut why, 0.12, "calledToBody");
    }
    if g.gd[gd].witness.heard.shots > 0.0 { add(&mut a, &mut why, 0.15, "shots"); }
    if g.gd[gd].witness.heard.kick + g.gd[gd].witness.heard.bang > 0.0 { add(&mut a, &mut why, 0.1, "bangs"); }
    if g.gd[gd].witness.dazzled > 0.0 { add(&mut a, &mut why, 0.15, "dazzled"); }
    if missing_known_to(g, gd).is_some() { add(&mut a, &mut why, 0.1, "missing"); }
    if crate::sim_lightshome::mains_active(g) { add(&mut a, &mut why, 0.08, "blackout"); }
    let hs = held_since_of(g, gd);
    let held = if hs >= 0.0 { m::max(0.0, g.time - hs) } else { 0.0 };
    if held > 0.0 {
        add(&mut a, &mut why, if t == Temperament::Nervy { m::min(0.15, 0.004 * held) } else { -m::min(0.12, 0.003 * held) }, "held");
    }
    if matches!(&g.gd[gd].held, Some(h) if h.variant == crate::character::HoldVariant::Gun) { add(&mut a, &mut why, 0.1, "gunToHead"); }
    let abort_t = g.tab.game_talk[talk].abort_t;
    if abort_t >= 0.0 && (g.gd[gd].held.is_some() || g.time - abort_t < ABORT_RATTLE_SECS) { add(&mut a, &mut why, 0.2, "aborted"); }
    let presses = g.tab.game_talk[talk].presses;
    if presses > 3.0 { add(&mut a, &mut why, m::min(0.15, 0.03 * (presses - 3.0)), "pressed"); }
    add(&mut a, &mut why, if t == Temperament::Nervy { 0.15 } else if t == Temperament::Hard { -0.1 } else { 0.0 }, "bias");
    let shaken = crate::morale::shaken_of(g, gd);
    let dread = m::max(0.0, shaken - m::max(0.0, a));
    add(&mut a, &mut why, dread, "dread");
    AgitationTerms { a: m::max(0.0, m::min(1.0, a)), why }
}

/// By callsign; a spawned unit, whatever it is called, is a fighting man: hard.   (dialogue.ts temperamentOf)
pub fn temperament_of(g: &mut crate::game::Game, gd: GuardId) -> crate::game::Temperament {
    if g.gd[gd].unit.is_some() { return crate::game::Temperament::Hard; }
    let callsign = g.gd[gd].callsign.clone();
    match crate::game::temperament().get(callsign).map(String::as_str) {
        Some("nervy") => crate::game::Temperament::Nervy,
        Some("hard") => crate::game::Temperament::Hard,
        _ => crate::game::Temperament::Steady,
    }
}

/// The stages at which this temperament gives the useful thing.   (dialogue.ts givesAt)
pub fn gives_at(t: crate::game::Temperament, band: crate::dialogue::Band) -> Vec<crate::dialogue::StageKey> {
    match t {
        crate::game::Temperament::Nervy => vec![StageKey::Num(2.0), StageKey::Num(3.0)],
        crate::game::Temperament::Hard => if band == Band::Low { vec![] } else { vec![StageKey::Num(3.0)] },
        _ => vec![StageKey::Num(3.0)],
    }
}

// ---------------------------------------------------------------- tokens, the line table

/// Every token, to read a table's word back as its Token.
const TOKEN_ALL: [Token; 23] = [Token::Partner, Token::Victim, Token::Body, Token::Missing, Token::Room, Token::SawWhere, Token::HeardWhere, Token::Route, Token::PartnerRoute, Token::Post, Token::Door, Token::Caller, Token::Checker, Token::Checked, Token::ClearRoom, Token::BreakerRoom, Token::CheckEta, Token::Minutes, Token::N, Token::Up, Token::Badge, Token::Code, Token::PadDoor];

/// The longest value each token can take, as the page has it: TOKEN_WORST with TOKEN_WORST_NOW laid over. Only the lint reads it.   (dialogue.ts tokenWorst)
pub fn token_worst(g: &mut crate::game::Game) -> JsMap<crate::dialogue::Token, String> { token_worst_of() }

fn token_worst_of() -> JsMap<Token, String> {
    let mut t = JsMap::new();
    for (word, v) in crate::dialogue::token_worst_table().iter() {
        if let Some(k) = TOKEN_ALL.iter().find(|k| k.as_str() == word.as_str()) { t.set(*k, v.clone()); }
    }
    if let Some(door) = crate::dialogue::token_worst_now().door { t.set(Token::Door, door); }
    t
}

/// A string the TypeScript's `||` would take: present and not empty.
fn said(s: &Option<String>) -> Option<String> { s.as_ref().filter(|x| !x.is_empty()).cloned() }

/// The words of a row as they are said to this man and by him.   (dialogue.ts saidText)
pub fn said_text(g: &mut crate::game::Game, l: &crate::dialogue::SaidTextL, ctx: &crate::dialogue::DialogueCtx) -> String { said_of(l, ctx) }

fn said_of(l: &crate::dialogue::SaidTextL, ctx: &crate::dialogue::DialogueCtx) -> String {
    if ctx.nameless == Some(true) { if let Some(x) = said(&l.bare) { return x; } }
    if ctx.facts.office != Some(true) { if let Some(x) = said(&l.away) { return x; } }
    if let Some(x) = said(&l.guide) { return x; }
    if let Some(x) = said(&l.now) { return x; }
    l.text.clone()
}

// ---------------------------------------------------------------- intel

/// The useful things' texts by kind and by who says them, as the page has them: INTEL_TEXT with INTEL_NOW and INTEL_GUIDE laid over. (dialogue.ts INTEL_TEXT, INTEL_NOW, INTEL_GUIDE)
static INTEL_TABLE: &[(IntelKind, &[(&str, &str)])] = &[
    (IntelKind::CheckOn, &[
        ("*", "{Checker}'s walking {checked}'s route — he went quiet. He'll come past here."),
        ("nervy", "{Checker}'s out looking for {checked}— he went quiet— he'll walk right into us—"),
        ("hard", "{Checker}'s walking {checked}'s route. This way. Soon."),
    ]),
    (IntelKind::Clearing, &[
        ("*", "They're clearing rooms off your last position. {ClearRoom} first."),
        ("hard", "Rooms are being cleared. {ClearRoom} first. Then this one."),
    ]),
    (IntelKind::RollcallLive, &[
        ("*", "That's a check now. If one of us doesn't answer, {caller} sends someone over."),
        ("hard", "That's the check. If I don't answer, {caller} sends somebody here."),
    ]),
    (IntelKind::Blackout, &[
        ("*", "Lights die, one of us goes to the breaker in {breakerRoom} — armed."),
        ("hard", "If the lights go out, somebody goes to the breaker in {breakerRoom}. Armed."),
    ]),
    (IntelKind::Door, &[
        ("*", "{Door} is on a key. We all carry one. Mine's on my belt."),
        ("hard", "{Door} is keyed. We all carry one. Take mine and get out."),
        ("selfown:steady", "…Belt. The key's on my belt. Well played, I suppose."),
        ("selfown:nervy", "…belt. It's on my belt. Please don't make it a whole thing."),
        ("sc:selfown-s1", "…On my belt. The key's on my belt. Eleven years, for this."),
        ("sc:selfown-s2", "…Belt. Left side. Take it— you were going to anyway."),
        ("sc:selfown-s3", "…Belt clip. Under the hi-vis. Nicely done, I suppose."),
        ("sc:selfown-n2", "…This one. On my belt. I heard it as I said it. I HEARD it."),
        ("sc:selfown-n3", "…key. Belt, left side. I don't feel better. You said I would."),
        ("con", "{Door}'s keyed; every belt carries one. Here— inspect mine."),
        ("fan", "{Door}'s keyed— key's on my belt. Do yours open anything? No?"),
    ]),
    (IntelKind::Code, &[
        ("*", "{PadDoor} is on a pad — {code}. Same four digits every night."),
        ("nervy", "The pad by {padDoor}— it's {code}— {code}— don't make me say it twice—"),
        ("hard", "{PadDoor}: {code} on the pad. Now you know what I know."),
        ("con", "{PadDoor} takes {code} on the pad, sir. Logged monthly, changed never."),
        ("fan", "The pad on {padDoor} is {code}— do YOU get codes, or is it all wire?"),
    ]),
    (IntelKind::Rollcall, &[
        ("*", "Radio check {checkEta}. Miss it, they send a man down your route."),
        ("nervy", "A check— {checkEta}— I don't answer, somebody comes looking."),
        ("nervy:sawHurt", "The check's {checkEta}— {victim} won't answer it— then they ALL come—"),
        ("hard", "Radio check {checkEta}. No answer, they send somebody. Do the math."),
        ("con", "Radio check {checkEta}, sir. No answer, a man walks that route. By the book."),
        ("fan", "Radio check's {checkEta}— miss it, they send someone. Do YOU have checks?"),
    ]),
    (IntelKind::LockdownPosts, &[
        ("*", "We're paired now. One man holds {post}. Nobody walks alone."),
        ("hard", "Pairs. A man on {post}. Nobody alone — except you."),
    ]),
    (IntelKind::Routes, &[
        ("*", "{N} of us tonight. I've got {route}, {partner} has {partnerRoute}."),
        ("nervy", "It's {n} of us— me on {route}, {partner} on {partnerRoute}— that's it—"),
        ("hard", "{N}. Me, {route}. {Partner}, {partnerRoute}. Happy?"),
        ("con", "{N} on shift: I take {route}, {partner} {partnerRoute}. All logged, sir."),
        ("fan", "{N} of us— I do {route}, {partner} {partnerRoute}. What's YOUR route like?"),
    ]),
    (IntelKind::Missing, &[
        ("*", "Nobody can raise {missing}. His route turns up empty, this all locks down."),
        ("hard", "{Missing} is off the air. His route comes up empty, we lock down."),
    ]),
    (IntelKind::Drive, &[
        ("*", "Your rack dropped off the board. Somebody's already been sent to look at it."),
        ("hard", "Your rack went dark on the board. Somebody's on his way to it."),
    ]),
];

/// What a kind's row of the table holds, in the written order; a kind always has one.
fn intel_rows(kind: IntelKind) -> &'static [(&'static str, &'static str)] {
    for (k, rows) in INTEL_TABLE { if *k == kind { return rows; } }
    &[]
}

fn intel_get(rows: &[(&'static str, &'static str)], key: &str) -> Option<&'static str> {
    for (k, v) in rows { if *k == key { return Some(*v); } }
    None
}

/// A kind's rows as the skeleton's struct. It has no field for the `sc:` rows: `shaped_of` reads those off the rows.
fn table_of(rows: &[(&'static str, &'static str)]) -> crate::dialogue::IntelTable {
    let v = |key: &str| intel_get(rows, key).map(st);
    crate::dialogue::IntelTable {
        steady: v("steady"),
        nervy: v("nervy"),
        hard: v("hard"),
        under: v("*"),
        pedant: v("pedant"),
        selfown: v("selfown"),
        folder: v("folder"),
        ramble: v("ramble"),
        fan: v("fan"),
        stoic: v("stoic"),
        con: v("con"),
        game: v("game"),
        menace: v("menace"),
        steady_none: v("steady:none"),
        steady_heard: v("steady:heard"),
        steady_glimpsed: v("steady:glimpsed"),
        steady_saw: v("steady:saw"),
        steady_saw_hurt: v("steady:sawHurt"),
        nervy_none: v("nervy:none"),
        nervy_heard: v("nervy:heard"),
        nervy_glimpsed: v("nervy:glimpsed"),
        nervy_saw: v("nervy:saw"),
        nervy_saw_hurt: v("nervy:sawHurt"),
        hard_none: v("hard:none"),
        hard_heard: v("hard:heard"),
        hard_glimpsed: v("hard:glimpsed"),
        hard_saw: v("hard:saw"),
        hard_saw_hurt: v("hard:sawHurt"),
        __none: v("*:none"),
        __heard: v("*:heard"),
        __glimpsed: v("*:glimpsed"),
        __saw: v("*:saw"),
        __saw_hurt: v("*:sawHurt"),
        pedant_steady: v("pedant:steady"),
        pedant_nervy: v("pedant:nervy"),
        pedant_hard: v("pedant:hard"),
        selfown_steady: v("selfown:steady"),
        selfown_nervy: v("selfown:nervy"),
        selfown_hard: v("selfown:hard"),
        folder_steady: v("folder:steady"),
        folder_nervy: v("folder:nervy"),
        folder_hard: v("folder:hard"),
        ramble_steady: v("ramble:steady"),
        ramble_nervy: v("ramble:nervy"),
        ramble_hard: v("ramble:hard"),
        fan_steady: v("fan:steady"),
        fan_nervy: v("fan:nervy"),
        fan_hard: v("fan:hard"),
        stoic_steady: v("stoic:steady"),
        stoic_nervy: v("stoic:nervy"),
        stoic_hard: v("stoic:hard"),
        con_steady: v("con:steady"),
        con_nervy: v("con:nervy"),
        con_hard: v("con:hard"),
        game_steady: v("game:steady"),
        game_nervy: v("game:nervy"),
        game_hard: v("game:hard"),
        menace_steady: v("menace:steady"),
        menace_nervy: v("menace:nervy"),
        menace_hard: v("menace:hard"),
    }
}

/// A kind's table of texts.   (dialogue.ts intelTable)
pub fn intel_table(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind) -> crate::dialogue::IntelTable {
    table_of(intel_rows(kind))
}

/// The words for what was taken, off the office: DRIVE_AWAY with DRIVE_AWAY_GUIDE laid over.   (dialogue.ts driveAway)
pub fn drive_away(g: &mut crate::game::Game) -> crate::dialogue::DriveAway { drive_away_of() }

fn drive_away_of() -> crate::dialogue::DriveAway {
    let (was, now) = (crate::dialogue::drive_away_table(), crate::dialogue::drive_away_guide());
    crate::dialogue::DriveAway {
        steady: now.steady.or(was.steady),
        nervy: now.nervy.or(was.nervy),
        hard: now.hard.or(was.hard),
        under: now.under.or(was.under),
    }
}

fn intel_text_of(kind: IntelKind, t: crate::game::Temperament, k: Knowledge) -> &'static str {
    let rows = intel_rows(kind);
    intel_get(rows, &format!("{}:{}", t.as_str(), k.as_str()))
        .or_else(|| intel_get(rows, t.as_str()))
        .or_else(|| intel_get(rows, &format!("*:{}", k.as_str())))
        .or_else(|| intel_get(rows, "*"))
        .unwrap_or("")
}

/// The text a man of this temperament and knowledge gives for `kind`, the most specific variant written.   (dialogue.ts intelText)
pub fn intel_text(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind, t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> String {
    st(intel_text_of(kind, t, k))
}

fn shaped_of(kind: IntelKind, id: &str, shape: ShapeKey, t: crate::game::Temperament) -> Option<&'static str> {
    let rows = intel_rows(kind);
    intel_get(rows, &format!("sc:{}", id))
        .or_else(|| intel_get(rows, &format!("{}:{}", shape.as_str(), t.as_str())))
        .or_else(|| intel_get(rows, shape.as_str()))
}

/// The script's own or the shape-voiced variant of `kind`, or None when none is written.   (dialogue.ts shapedIntelText)
pub fn shaped_intel_text(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind, sc: &crate::dialogue::ShapedIntelTextSc, t: crate::game::Temperament) -> Option<String> {
    shaped_of(kind, &sc.id, sc.shape, t).map(st)
}

/// The checker's own words when he is the man walking the silent colleague's route.   (dialogue.ts checkOnSelf)
pub fn check_on_self(g: &mut crate::game::Game) -> String {
    st(crate::dialogue::CHECKON_SELF_NOW)
}

/// What goes in front of a given fact.   (dialogue.ts intelLead)
pub fn intel_lead(t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> String {
    match t {
        crate::game::Temperament::Hard => st("…Fine. "),
        crate::game::Temperament::Steady if k == Knowledge::Saw || k == Knowledge::SawHurt => st("…All right. "),
        _ => String::new(),
    }
}

/// A fact behind a lead that runs on into it starts in lower case, unless it starts with 'I' or a man's name. By hand, the regexes of TS 366:
/// `/[a-z—] $/` on the lead, `/^(?!I\b|\{(?:Checker|Checked|Missing|Partner|Victim|Caller|Body)\})(\{?)([A-Z])/` on the fact.
fn run_on_text(lead: &str, fact: &str) -> String {
    let mut back = lead.chars().rev();
    let runs = back.next() == Some(' ') && matches!(back.next(), Some('a'..='z') | Some('—'));
    if !runs { return st(fact); }
    if let Some(after) = fact.strip_prefix('I') {
        // \b after the I: JavaScript's \w is ASCII only, with no `u` flag.
        if !matches!(after.chars().next(), Some(c) if c.is_ascii_alphanumeric() || c == '_') { return st(fact); }
    }
    for name in ["{Checker}", "{Checked}", "{Missing}", "{Partner}", "{Victim}", "{Caller}", "{Body}"] {
        if fact.starts_with(name) { return st(fact); }
    }
    let (brace, rest) = match fact.strip_prefix('{') { Some(r) => ("{", r), None => ("", fact) };
    match rest.chars().next() {
        Some(c) if c.is_ascii_uppercase() => format!("{}{}{}", brace, c.to_ascii_lowercase(), &rest[1..]),
        _ => st(fact),
    }
}

/// (dialogue.ts runOn)
pub fn run_on(g: &mut crate::game::Game, lead: &str, fact: &str) -> String { run_on_text(lead, fact) }

/// Four digits off a callsign, the same every night for the same man.   (dialogue.ts badgeOf)
pub fn badge_of(callsign: &str) -> String {
    let n = 1000.0 + js::rem(crate::math_dice::hash_str(&format!("badge:{}", callsign)), 9000.0);
    format!("{}", n as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Temperament;

    fn fnv(s: &str) -> u32 {
        let mut h: u32 = 0x811c9dc5;
        for b in s.bytes() { h ^= b as u32; h = h.wrapping_mul(0x01000193); }
        h
    }

    const TS: [Temperament; 3] = [Temperament::Steady, Temperament::Nervy, Temperament::Hard];
    const KS: [Knowledge; 5] = [Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw, Knowledge::SawHurt];
    const SH: [ShapeKey; 9] = [ShapeKey::Pedant, ShapeKey::Selfown, ShapeKey::Folder, ShapeKey::Ramble, ShapeKey::Fan, ShapeKey::Stoic, ShapeKey::Con, ShapeKey::Game, ShapeKey::Menace];

    /// Every plain variant against the TypeScript's own tables, laid over as the page has them.
    #[test]
    fn intel_text_matches_the_typescript() {
        let mut all = String::new();
        for (kind, _) in INTEL_TABLE {
            for t in TS { for k in KS { all.push_str(intel_text_of(*kind, t, k)); all.push('\n'); } }
        }
        assert_eq!(fnv(&all), 0xcc8530bf);
    }

    /// Every shaped variant against the same, with a script id that has rows of its own and one that has none.
    #[test]
    fn shaped_text_matches_the_typescript() {
        let mut all = String::new();
        for (kind, _) in INTEL_TABLE {
            for id in ["selfown-s1", "selfown-s2", "selfown-s3", "selfown-n2", "selfown-n3", "nope"] {
                for shape in SH { for t in TS { all.push_str(shaped_of(*kind, id, shape, t).unwrap_or("<null>")); all.push('\n'); } }
            }
        }
        assert_eq!(fnv(&all), 0x7e8f96ae);
    }

    /// The whole table of lines, field by field, against the TypeScript's.
    #[test]
    fn lines_match_the_typescript() {
        let rows = lines();
        assert_eq!(rows.len(), 85);
        let mut all = String::new();
        for (i, l) in rows.iter().enumerate() {
            if i > 0 { all.push('\n'); }
            let s = match l.s { StageKey::Num(n) => format!("{}", n as i64), StageKey::X => st("x") };
            let needs: Vec<&str> = l.needs.iter().flatten().map(|n| n.as_str()).collect();
            let w = l.w.map(|x| format!("{}", x)).unwrap_or_default();
            let o = |x: &Option<String>| x.clone().unwrap_or_default();
            all.push_str(&[
                l.id.clone(), st(l.k.as_str()), st(l.a.map(|x| x.as_str()).unwrap_or("")), st(l.t.map(|x| x.as_str()).unwrap_or("")), s, l.text.clone(),
                needs.join(","), w, st(l.only.map(|x| x.as_str()).unwrap_or("")), o(&l.now), o(&l.away), o(&l.guide), o(&l.bare),
            ].join("|"));
        }
        assert_eq!(fnv(&all), 0x9b0c9d9b);
    }

    /// The hand-written match of runOn against what the TypeScript's regexes give.
    #[test]
    fn run_on_matches_the_typescript_regex() {
        let cases: &[(&str, &str, &str)] = &[
            ("Oh— and ", "{Checker}'s walking", "{Checker}'s walking"),
            ("Oh— and ", "{N} of us", "{n} of us"),
            ("Oh— and ", "I was sent", "I was sent"),
            ("Oh— and ", "I'm out", "I'm out"),
            ("Oh— and ", "I", "I"),
            ("Oh— and ", "Is it", "is it"),
            ("Oh— and ", "It's", "it's"),
            ("Oh— and ", "Radio check", "radio check"),
            ("Oh— and ", "{PadDoor} is", "{padDoor} is"),
            ("Oh— and ", "{ClearRoom} first", "{clearRoom} first"),
            ("Oh— and ", "{Door} is", "{door} is"),
            ("Oh— and ", "Nobody", "nobody"),
            ("Oh— and ", "…belt", "…belt"),
            ("Oh— and ", "{checked}", "{checked}"),
            ("Oh— and ", "{Body} x", "{Body} x"),
            ("Oh— and ", "{Bodyguard} y", "{bodyguard} y"),
            ("Oh— and ", "{Victim}", "{Victim}"),
            ("Oh— and ", "{Victim", "{victim"),
            ("Oh— and ", "{I} x", "{i} x"),
            ("Oh— and ", "", ""),
            ("Oh— and ", "Éa", "Éa"),
            ("Oh— and ", "I—no", "I—no"),
            ("Oh— and ", "I_a", "i_a"),
            ("Oh— and ", "I1", "i1"),
            ("Sir: ", "{Checker}'s walking", "{Checker}'s walking"),
            ("Sir: ", "{N} of us", "{N} of us"),
            ("Sir: ", "I was sent", "I was sent"),
            ("Sir: ", "I'm out", "I'm out"),
            ("Sir: ", "I", "I"),
            ("Sir: ", "Is it", "Is it"),
            ("Sir: ", "It's", "It's"),
            ("Sir: ", "Radio check", "Radio check"),
            ("Sir: ", "{PadDoor} is", "{PadDoor} is"),
            ("Sir: ", "{ClearRoom} first", "{ClearRoom} first"),
            ("Sir: ", "{Door} is", "{Door} is"),
            ("Sir: ", "Nobody", "Nobody"),
            ("Sir: ", "…belt", "…belt"),
            ("Sir: ", "{checked}", "{checked}"),
            ("Sir: ", "{Body} x", "{Body} x"),
            ("Sir: ", "{Bodyguard} y", "{Bodyguard} y"),
            ("Sir: ", "{Victim}", "{Victim}"),
            ("Sir: ", "{Victim", "{Victim"),
            ("Sir: ", "{I} x", "{I} x"),
            ("Sir: ", "", ""),
            ("Sir: ", "Éa", "Éa"),
            ("Sir: ", "I—no", "I—no"),
            ("Sir: ", "I_a", "I_a"),
            ("Sir: ", "I1", "I1"),
            ("—okay— ", "{Checker}'s walking", "{Checker}'s walking"),
            ("—okay— ", "{N} of us", "{n} of us"),
            ("—okay— ", "I was sent", "I was sent"),
            ("—okay— ", "I'm out", "I'm out"),
            ("—okay— ", "I", "I"),
            ("—okay— ", "Is it", "is it"),
            ("—okay— ", "It's", "it's"),
            ("—okay— ", "Radio check", "radio check"),
            ("—okay— ", "{PadDoor} is", "{padDoor} is"),
            ("—okay— ", "{ClearRoom} first", "{clearRoom} first"),
            ("—okay— ", "{Door} is", "{door} is"),
            ("—okay— ", "Nobody", "nobody"),
            ("—okay— ", "…belt", "…belt"),
            ("—okay— ", "{checked}", "{checked}"),
            ("—okay— ", "{Body} x", "{Body} x"),
            ("—okay— ", "{Bodyguard} y", "{bodyguard} y"),
            ("—okay— ", "{Victim}", "{Victim}"),
            ("—okay— ", "{Victim", "{victim"),
            ("—okay— ", "{I} x", "{i} x"),
            ("—okay— ", "", ""),
            ("—okay— ", "Éa", "Éa"),
            ("—okay— ", "I—no", "I—no"),
            ("—okay— ", "I_a", "i_a"),
            ("—okay— ", "I1", "i1"),
            ("", "{Checker}'s walking", "{Checker}'s walking"),
            ("", "{N} of us", "{N} of us"),
            ("", "I was sent", "I was sent"),
            ("", "I'm out", "I'm out"),
            ("", "I", "I"),
            ("", "Is it", "Is it"),
            ("", "It's", "It's"),
            ("", "Radio check", "Radio check"),
            ("", "{PadDoor} is", "{PadDoor} is"),
            ("", "{ClearRoom} first", "{ClearRoom} first"),
            ("", "{Door} is", "{Door} is"),
            ("", "Nobody", "Nobody"),
            ("", "…belt", "…belt"),
            ("", "{checked}", "{checked}"),
            ("", "{Body} x", "{Body} x"),
            ("", "{Bodyguard} y", "{Bodyguard} y"),
            ("", "{Victim}", "{Victim}"),
            ("", "{Victim", "{Victim"),
            ("", "{I} x", "{I} x"),
            ("", "", ""),
            ("", "Éa", "Éa"),
            ("", "I—no", "I—no"),
            ("", "I_a", "I_a"),
            ("", "I1", "I1"),
            ("…Fine. ", "{Checker}'s walking", "{Checker}'s walking"),
            ("…Fine. ", "{N} of us", "{N} of us"),
            ("…Fine. ", "I was sent", "I was sent"),
            ("…Fine. ", "I'm out", "I'm out"),
            ("…Fine. ", "I", "I"),
            ("…Fine. ", "Is it", "Is it"),
            ("…Fine. ", "It's", "It's"),
            ("…Fine. ", "Radio check", "Radio check"),
            ("…Fine. ", "{PadDoor} is", "{PadDoor} is"),
            ("…Fine. ", "{ClearRoom} first", "{ClearRoom} first"),
            ("…Fine. ", "{Door} is", "{Door} is"),
            ("…Fine. ", "Nobody", "Nobody"),
            ("…Fine. ", "…belt", "…belt"),
            ("…Fine. ", "{checked}", "{checked}"),
            ("…Fine. ", "{Body} x", "{Body} x"),
            ("…Fine. ", "{Bodyguard} y", "{Bodyguard} y"),
            ("…Fine. ", "{Victim}", "{Victim}"),
            ("…Fine. ", "{Victim", "{Victim"),
            ("…Fine. ", "{I} x", "{I} x"),
            ("…Fine. ", "", ""),
            ("…Fine. ", "Éa", "Éa"),
            ("…Fine. ", "I—no", "I—no"),
            ("…Fine. ", "I_a", "I_a"),
            ("…Fine. ", "I1", "I1"),
            ("…All right. ", "{Checker}'s walking", "{Checker}'s walking"),
            ("…All right. ", "{N} of us", "{N} of us"),
            ("…All right. ", "I was sent", "I was sent"),
            ("…All right. ", "I'm out", "I'm out"),
            ("…All right. ", "I", "I"),
            ("…All right. ", "Is it", "Is it"),
            ("…All right. ", "It's", "It's"),
            ("…All right. ", "Radio check", "Radio check"),
            ("…All right. ", "{PadDoor} is", "{PadDoor} is"),
            ("…All right. ", "{ClearRoom} first", "{ClearRoom} first"),
            ("…All right. ", "{Door} is", "{Door} is"),
            ("…All right. ", "Nobody", "Nobody"),
            ("…All right. ", "…belt", "…belt"),
            ("…All right. ", "{checked}", "{checked}"),
            ("…All right. ", "{Body} x", "{Body} x"),
            ("…All right. ", "{Bodyguard} y", "{Bodyguard} y"),
            ("…All right. ", "{Victim}", "{Victim}"),
            ("…All right. ", "{Victim", "{Victim"),
            ("…All right. ", "{I} x", "{I} x"),
            ("…All right. ", "", ""),
            ("…All right. ", "Éa", "Éa"),
            ("…All right. ", "I—no", "I—no"),
            ("…All right. ", "I_a", "I_a"),
            ("…All right. ", "I1", "I1"),
            ("x ", "{Checker}'s walking", "{Checker}'s walking"),
            ("x ", "{N} of us", "{n} of us"),
            ("x ", "I was sent", "I was sent"),
            ("x ", "I'm out", "I'm out"),
            ("x ", "I", "I"),
            ("x ", "Is it", "is it"),
            ("x ", "It's", "it's"),
            ("x ", "Radio check", "radio check"),
            ("x ", "{PadDoor} is", "{padDoor} is"),
            ("x ", "{ClearRoom} first", "{clearRoom} first"),
            ("x ", "{Door} is", "{door} is"),
            ("x ", "Nobody", "nobody"),
            ("x ", "…belt", "…belt"),
            ("x ", "{checked}", "{checked}"),
            ("x ", "{Body} x", "{Body} x"),
            ("x ", "{Bodyguard} y", "{bodyguard} y"),
            ("x ", "{Victim}", "{Victim}"),
            ("x ", "{Victim", "{victim"),
            ("x ", "{I} x", "{i} x"),
            ("x ", "", ""),
            ("x ", "Éa", "Éa"),
            ("x ", "I—no", "I—no"),
            ("x ", "I_a", "i_a"),
            ("x ", "I1", "i1"),
            ("Z ", "{Checker}'s walking", "{Checker}'s walking"),
            ("Z ", "{N} of us", "{N} of us"),
            ("Z ", "I was sent", "I was sent"),
            ("Z ", "I'm out", "I'm out"),
            ("Z ", "I", "I"),
            ("Z ", "Is it", "Is it"),
            ("Z ", "It's", "It's"),
            ("Z ", "Radio check", "Radio check"),
            ("Z ", "{PadDoor} is", "{PadDoor} is"),
            ("Z ", "{ClearRoom} first", "{ClearRoom} first"),
            ("Z ", "{Door} is", "{Door} is"),
            ("Z ", "Nobody", "Nobody"),
            ("Z ", "…belt", "…belt"),
            ("Z ", "{checked}", "{checked}"),
            ("Z ", "{Body} x", "{Body} x"),
            ("Z ", "{Bodyguard} y", "{Bodyguard} y"),
            ("Z ", "{Victim}", "{Victim}"),
            ("Z ", "{Victim", "{Victim"),
            ("Z ", "{I} x", "{I} x"),
            ("Z ", "", ""),
            ("Z ", "Éa", "Éa"),
            ("Z ", "I—no", "I—no"),
            ("Z ", "I_a", "I_a"),
            ("Z ", "I1", "I1"),
            ("and  ", "{Checker}'s walking", "{Checker}'s walking"),
            ("and  ", "{N} of us", "{N} of us"),
            ("and  ", "I was sent", "I was sent"),
            ("and  ", "I'm out", "I'm out"),
            ("and  ", "I", "I"),
            ("and  ", "Is it", "Is it"),
            ("and  ", "It's", "It's"),
            ("and  ", "Radio check", "Radio check"),
            ("and  ", "{PadDoor} is", "{PadDoor} is"),
            ("and  ", "{ClearRoom} first", "{ClearRoom} first"),
            ("and  ", "{Door} is", "{Door} is"),
            ("and  ", "Nobody", "Nobody"),
            ("and  ", "…belt", "…belt"),
            ("and  ", "{checked}", "{checked}"),
            ("and  ", "{Body} x", "{Body} x"),
            ("and  ", "{Bodyguard} y", "{Bodyguard} y"),
            ("and  ", "{Victim}", "{Victim}"),
            ("and  ", "{Victim", "{Victim"),
            ("and  ", "{I} x", "{I} x"),
            ("and  ", "", ""),
            ("and  ", "Éa", "Éa"),
            ("and  ", "I—no", "I—no"),
            ("and  ", "I_a", "I_a"),
            ("and  ", "I1", "I1"),
        ];
        for (lead, fact, want) in cases { assert_eq!(run_on_text(lead, fact), *want, "lead {:?} fact {:?}", lead, fact); }
    }

    /// The struct holds every row of a kind but the script's own, under the field its key names.
    #[test]
    fn table_of_puts_each_key_in_its_field() {
        let door = table_of(intel_rows(IntelKind::Door));
        assert_eq!(door.under.as_deref(), Some("{Door} is on a key. We all carry one. Mine's on my belt."));
        assert_eq!(door.selfown_steady.as_deref(), Some("…Belt. The key's on my belt. Well played, I suppose."));
        assert_eq!(door.con.as_deref(), Some("{Door}'s keyed; every belt carries one. Here— inspect mine."));
        assert_eq!(door.steady, None);
        let check = table_of(intel_rows(IntelKind::Rollcall));
        assert_eq!(check.nervy_saw_hurt.as_deref(), Some("The check's {checkEta}— {victim} won't answer it— then they ALL come—"));
    }

    #[test]
    fn drive_away_takes_the_guides_words() {
        let d = drive_away_of();
        assert_eq!(d.under.as_deref(), Some("They already know it's gone. Somebody's been sent to look."));
        assert_eq!(d.hard.as_deref(), Some("They know it's gone. Somebody's on his way to look."));
        assert_eq!(d.steady, None);
    }

    #[test]
    fn token_worst_has_every_token_and_the_new_door() {
        let t = token_worst_of();
        assert_eq!(t.len(), 23);
        assert_eq!(t.get(Token::Door).map(String::as_str), Some("the phone shop back door"));
        assert_eq!(t.get(Token::PadDoor).map(String::as_str), Some("the office side door"));
    }

    /// The order of TS 208: bare from a nameless man, away off the office, guide, now, text; an empty string falls through.
    #[test]
    fn said_text_takes_the_first_that_applies() {
        let l = SaidTextL { text: st("t"), now: Some(st("n")), away: Some(st("a")), bare: Some(st("b")), guide: Some(st("g")) };
        let mut ctx = DialogueCtx::zeroed();
        assert_eq!(said_of(&l, &ctx), "a");
        ctx.facts.office = Some(false);
        assert_eq!(said_of(&l, &ctx), "a");
        ctx.facts.office = Some(true);
        assert_eq!(said_of(&l, &ctx), "g");
        ctx.nameless = Some(true);
        assert_eq!(said_of(&l, &ctx), "b");
        let l2 = SaidTextL { text: st("t"), now: None, away: Some(st("")), bare: Some(st("")), guide: Some(st("")) };
        ctx.facts.office = None;
        assert_eq!(said_of(&l2, &ctx), "t");
    }

    #[test]
    fn agitation_add_drops_the_negligible_and_labels_what_shows() {
        let (mut a, mut why) = (0.0, Vec::new());
        add(&mut a, &mut why, 5e-7, "tiny");
        assert_eq!(a, 0.0);
        add(&mut a, &mut why, 0.004, "small");
        assert_eq!(a, 0.004);
        assert!(why.is_empty());
        add(&mut a, &mut why, 0.22, "floor");
        add(&mut a, &mut why, -0.12, "held");
        assert_eq!(why, vec![st("floor +0.22"), st("held -0.12")]);
    }
}
