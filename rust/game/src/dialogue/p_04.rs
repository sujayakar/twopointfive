//! src/game/dialogue.ts lines 1218 to 1392
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Tooling, read only by tools/qa/dialogue-probes.ts: not translated.   (dialogue.ts samCoverage)
pub fn sam_coverage(g: &mut crate::game::Game) -> crate::dialogue::SamCoverage { todo!("tooling: read only by tools/qa, not translated") }

/// Tooling, read only by tools/qa/dialogue-probes.ts: not translated.   (dialogue.ts scriptCoverage)
pub fn script_coverage() -> Vec<crate::dialogue::ScriptCoverageItem> { todo!("tooling: read only by tools/qa, not translated") }

/// Tooling, read only by tools/qa/dialogue-probes.ts and guidewords-check.ts: not translated.   (dialogue.ts lineLengthReport)
pub fn line_length_report(g: &mut crate::game::Game) -> crate::dialogue::LineLengthReport { todo!("tooling: read only by tools/qa, not translated") }

/// QA and the checkpoint: make this man's NEXT deal a named script (`None` = the pools).   (dialogue.ts forceScript)
pub fn force_script(g: &mut crate::game::Game, talk: GameTalkId, id_2: Option<&str>) {
    g.m.dialogue.script_by_talk.delete(talk);
    g.m.dialogue.forced_by_talk.set(talk, id_2.map(String::from));
}

/// The deal forced on this man and not yet made.   (dialogue.ts forcedScriptOf)
pub fn forced_script_of(g: &mut crate::game::Game, talk: GameTalkId) -> Option<String> {
    g.m.dialogue.forced_by_talk.get(talk).cloned().flatten()
}

/// The script this man's hold plays, if one has been dealt; never deals.   (dialogue.ts peekScript)
pub fn peek_script(g: &mut crate::game::Game, talk: GameTalkId) -> Option<crate::dialogue::Script> {
    g.m.dialogue.script_by_talk.get(talk).cloned().flatten()
}

/// The script of this id.   (dialogue.ts scriptById)
pub fn script_by_id(id_2: &str) -> Option<crate::dialogue::Script> {
    scripts().into_iter().find(|sc| sc.id == id_2)
}

/// The script for this man: the one already dealt, else, at his first exchange only, dealt now: unplayed first, then a stable hash of his callsign.   (dialogue.ts scriptFor)
pub fn script_for(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId) -> Option<crate::dialogue::Script> {
    if let Some(dealt) = g.m.dialogue.script_by_talk.get(talk) { return dealt.clone(); }
    let mut sc: Option<Script> = None;
    if let Some(id) = g.m.dialogue.forced_by_talk.get(talk).cloned() {
        g.m.dialogue.forced_by_talk.delete(talk);
        sc = match id { Some(id) if !id.is_empty() => script_by_id(&id), _ => None };
    } else if g.tab.game_talk[talk].stage == 0.0 {
        let mut fits: Vec<Script> = Vec::new();
        for x in scripts() { if script_fits(g, &x, ctx, talk) { fits.push(x); } }
        if !fits.is_empty() {
            let c: &DialogueCtx = ctx;
            js::sort_by(&mut fits, |a, b| {
                let by_script = played_n(c, &alloc::format!("script:{}", a.id)) - played_n(c, &alloc::format!("script:{}", b.id));
                if js::truthy(by_script) { return by_script; }
                let by_shape = played_n(c, &alloc::format!("shape:{}", a.shape.as_str())) - played_n(c, &alloc::format!("shape:{}", b.shape.as_str()));
                if js::truthy(by_shape) { return by_shape; }
                crate::math_dice::hash_str(&alloc::format!("{}|{}", c.who, a.id)) - crate::math_dice::hash_str(&alloc::format!("{}|{}", c.who, b.id))
            });
            sc = fits.into_iter().next();
        }
    }
    if let Some(s) = &sc {
        if ctx.recent.is_some() {
            let (time, script_key, shape_key) = (ctx.time, alloc::format!("script:{}", s.id), alloc::format!("shape:{}", s.shape.as_str()));
            let recent = ctx.recent.as_mut().unwrap();
            recent.set(script_key.clone(), time);
            recent.set(shape_key.clone(), time);
            // ctx.recent IS recentByGame in the TypeScript (gather, :633); here it is a copy, so the game's own table is written too.
            if let Some(table) = g.m.dialogue.recent_by_game.as_mut() {
                table.set(script_key, time);
                table.set(shape_key, time);
            }
        }
    }
    g.m.dialogue.script_by_talk.set(talk, sc.clone());
    sc
}

/// `Number(played(key))`: 1 if this session has played it and the rotation is on.   (dialogue.ts scriptFor, played)
fn played_n(ctx: &DialogueCtx, key: &str) -> f64 {
    let played = SCRIPT_ROTATE && ctx.recent.as_ref().is_some_and(|r| r.has(String::from(key)));
    if played { 1.0 } else { 0.0 }
}

/// Does this script fit the man and the moment, and does every line it would say to him resolve now.   (dialogue.ts scriptFits)
pub fn script_fits(g: &mut crate::game::Game, sc: &crate::dialogue::Script, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId) -> bool {
    if sc.t != ctx.t || !sc.k.contains(&ctx.k) { return false; }
    if let Some(band) = &sc.band { if !band.contains(&ctx.band) { return false; } }
    if let Some(needs) = &sc.needs { if needs.iter().any(|n| !fact(ctx, *n)) { return false; } }
    if let Some(not) = &sc.not { if not.iter().any(|n| fact(ctx, *n)) { return false; } }
    if let Some(pin) = sc.pin {
        if crate::dialogue::pending_intel(g, ctx, talk, None).first().map(|x| x.kind) != Some(pin) { return false; }
    }
    let tokens = tokens_of(ctx);
    for s in stages() {
        let b = beat_for(sc, s, ctx);
        for text in [Some(b.sam.as_str()), b.ask.as_deref(), b.after.as_deref(), b.take.as_deref(), b.mild.as_deref()] {
            if let Some(text) = text { if !text.is_empty() && expand(text, &tokens).is_none() { return false; } }
        }
        if him_of(&b, ctx).is_none() { return false; }
    }
    true
}

/// Still his script this press: his knowledge of Sam has not climbed past what it was written for.   (dialogue.ts scriptLive)
pub fn script_live(sc: &crate::dialogue::Script, ctx: &crate::dialogue::DialogueCtx) -> bool {
    sc.k.contains(&ctx.k)
}

/// The beat for stage `s`: a `room` beat swapped for the room's own menace where the table has the room.   (dialogue.ts beatFor)
pub fn beat_for(sc: &crate::dialogue::Script, s: crate::dialogue::StageKey, ctx: &crate::dialogue::DialogueCtx) -> crate::dialogue::Beat {
    let b = script_beat(sc, s).clone();
    if b.room != Some(true) { return b; }
    if ctx.own_rooms == Some(true) { return b; }
    let table = room_menace();
    let room = String::from(ctx.tokens.room.as_deref().unwrap_or(""));
    let Some(list) = table.get(room) else { return b; };
    if list.is_empty() { return b; }
    let i = js::rem(crate::math_dice::hash_str(&alloc::format!("{}|room|{}", ctx.who, sc.id)), list.len() as f64) as usize;
    let r = &list[i];
    Beat { sam: r.sam.clone(), sam_now: None, him: r.him.clone(), after: r.after.clone().or(b.after.clone()), ..b }
}

/// The man's line off a beat for this man; `None` if it does not resolve.   (dialogue.ts himOf)
pub fn him_of(b: &crate::dialogue::Beat, ctx: &crate::dialogue::DialogueCtx) -> Option<crate::dialogue::HimOf> {
    let (raw, k) = him_raw(b, ctx)?;
    let text = expand(&raw, &tokens_of(ctx))?;
    if text.is_empty() { None } else { Some(HimOf { text, k }) }
}

/// Which of the beat's lines is this man's: by his knowledge, else his temperament, else '*'; `mild` where STOIC_BITE is 0.   (dialogue.ts himOf)
fn him_raw(b: &Beat, ctx: &DialogueCtx) -> Option<(String, HimOfK)> {
    if STOIC_BITE == 0.0 {
        if let Some(mild) = b.mild.as_ref().filter(|x| !x.is_empty()) { return Some((mild.clone(), HimOfK::Empty)); }
    }
    match &b.him {
        Him::Text(text) => Some((text.clone(), HimOfK::Empty)),
        Him::Arm1(by) => {
            if let Some(text) = by.get(String::from(ctx.k.as_str())) { return Some((text.clone(), him_k(ctx.k))); }
            by.get(String::from(ctx.t.as_str())).or_else(|| by.get(String::from("*"))).map(|text| (text.clone(), HimOfK::Empty))
        }
    }
}

/// A knowledge word as himOf's result has it.
fn him_k(k: Knowledge) -> HimOfK {
    match k {
        Knowledge::None => HimOfK::None,
        Knowledge::Heard => HimOfK::Heard,
        Knowledge::Glimpsed => HimOfK::Glimpsed,
        Knowledge::Saw => HimOfK::Saw,
        Knowledge::SawHurt => HimOfK::SawHurt,
    }
}

/// `ctx.facts[n]`, a row of the record by its word.   (dialogue.ts:1309)
fn fact(ctx: &DialogueCtx, n: Need) -> bool {
    let f = &ctx.facts;
    let row = match n {
        Need::Shift => f.shift,
        Need::RollcallLive => f.rollcall_live,
        Need::Blackout => f.blackout,
        Need::BodyFound => f.body_found,
        Need::FloorUp => f.floor_up,
        Need::Lockdown => f.lockdown,
        Need::Episode => f.episode,
        Need::HeardSmall => f.heard_small,
        Need::HeardShots => f.heard_shots,
        Need::HeardBang => f.heard_bang,
        Need::BodyCalled => f.body_called,
        Need::BodyKnown => f.body_known,
        Need::MissingKnown => f.missing_known,
        Need::HeldLong => f.held_long,
        Need::Dazzled => f.dazzled,
        Need::SawShot => f.saw_shot,
        Need::SawDrop => f.saw_drop,
        Need::AlertNow => f.alert_now,
        Need::DrewOnYou => f.drew_on_you,
        Need::PartnerKnown => f.partner_known,
        Need::Office => f.office,
    };
    row == Some(true)
}

/// `ctx.tokens` as `expand` takes it: keyed by the TypeScript's own words (`sawWhere`), which is what `expand` looks up.   (dialogue.ts:1311, 1331)
fn tokens_of(ctx: &DialogueCtx) -> JsMap<String, Option<String>> {
    let t = &ctx.tokens;
    let mut out = JsMap::new();
    for (key, value) in [
        ("door", &t.door), ("code", &t.code), ("missing", &t.missing), ("body", &t.body), ("n", &t.n), ("up", &t.up), ("post", &t.post),
        ("room", &t.room), ("route", &t.route), ("partner", &t.partner), ("victim", &t.victim), ("sawWhere", &t.saw_where),
        ("heardWhere", &t.heard_where), ("partnerRoute", &t.partner_route), ("caller", &t.caller), ("checker", &t.checker),
        ("checked", &t.checked), ("clearRoom", &t.clear_room), ("breakerRoom", &t.breaker_room), ("checkEta", &t.check_eta),
        ("minutes", &t.minutes), ("badge", &t.badge), ("padDoor", &t.pad_door),
    ] {
        out.set(String::from(key), value.clone());
    }
    out
}

/// `sc.beats[s]`. A stage past the table throws in the TypeScript (`b.room` of undefined), and panics here.   (dialogue.ts:1318)
fn script_beat(sc: &Script, s: StageKey) -> &Beat {
    match s {
        StageKey::X => &sc.beats.x,
        StageKey::Num(n) if n == 1.0 => &sc.beats._1,
        StageKey::Num(n) if n == 2.0 => &sc.beats._2,
        StageKey::Num(n) if n == 3.0 => &sc.beats._3,
        StageKey::Num(_) => panic!("no beat for this stage"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(k: Knowledge, t: crate::game::Temperament) -> DialogueCtx {
        DialogueCtx { k, t, ..DialogueCtx::zeroed() }
    }
    fn beat(pairs: &[(&str, &str)]) -> Beat {
        let mut by = JsMap::new();
        for (key, text) in pairs { by.set(String::from(*key), String::from(*text)); }
        Beat { him: Him::Arm1(by), ..Beat::zeroed() }
    }

    #[test]
    fn him_goes_by_knowledge_then_temperament_then_star() {
        let c = ctx(Knowledge::Heard, crate::game::Temperament::Hard);
        let all = beat(&[("*", "any"), ("hard", "two words"), ("heard", "he heard")]);
        let (text, k) = him_raw(&all, &c).unwrap();
        assert_eq!((text.as_str(), k.as_str()), ("he heard", "heard"));
        let (text, k) = him_raw(&beat(&[("*", "any"), ("hard", "two words")]), &c).unwrap();
        assert_eq!((text.as_str(), k.as_str()), ("two words", "*"));
        let (text, k) = him_raw(&beat(&[("*", "any"), ("nervy", "no")]), &c).unwrap();
        assert_eq!((text.as_str(), k.as_str()), ("any", "*"));
        assert!(him_raw(&beat(&[("nervy", "no")]), &c).is_none());
    }

    #[test]
    fn a_plain_line_is_for_everyone() {
        let c = ctx(Knowledge::Saw, crate::game::Temperament::Steady);
        let b = Beat { him: Him::Text(String::from("line")), ..Beat::zeroed() };
        let (text, k) = him_raw(&b, &c).unwrap();
        assert_eq!((text.as_str(), k.as_str()), ("line", "*"));
    }

    #[test]
    fn a_fact_is_its_own_row_and_only_true_counts() {
        let mut c = ctx(Knowledge::None, crate::game::Temperament::Steady);
        c.facts.office = Some(true);
        c.facts.heard_shots = Some(false);
        assert!(fact(&c, Need::Office));
        assert!(!fact(&c, Need::HeardShots));
        assert!(!fact(&c, Need::Shift));
    }

    #[test]
    fn the_tokens_map_has_the_tables_words() {
        let mut c = ctx(Knowledge::None, crate::game::Temperament::Steady);
        c.tokens.saw_where = Some(String::from("the lobby"));
        let map = tokens_of(&c);
        assert_eq!(map.len(), tokens().len());
        for word in tokens() { assert!(map.has(word)); }
        assert_eq!(map.get(String::from("sawWhere")).cloned().flatten().as_deref(), Some("the lobby"));
    }

    #[test]
    fn a_stage_reaches_its_beat() {
        let mut sc = Script::zeroed();
        sc.beats._1.sam = String::from("one");
        sc.beats._2.sam = String::from("two");
        sc.beats._3.sam = String::from("three");
        sc.beats.x.sam = String::from("x");
        for (s, want) in [(StageKey::Num(1.0), "one"), (StageKey::Num(2.0), "two"), (StageKey::Num(3.0), "three"), (StageKey::X, "x")] {
            assert_eq!(script_beat(&sc, s).sam, want);
        }
    }

    #[test]
    fn played_only_with_a_real_recent() {
        let mut c = ctx(Knowledge::None, crate::game::Temperament::Steady);
        assert_eq!(played_n(&c, "script:a"), 0.0);
        let mut r = JsMap::new();
        r.set(String::from("script:a"), 1.0);
        c.recent = Some(r);
        assert_eq!(played_n(&c, "script:a"), 1.0);
        assert_eq!(played_n(&c, "script:b"), 0.0);
    }
}
