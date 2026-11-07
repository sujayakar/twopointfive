//! src/game/dialogue.ts lines 817 to 1218
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A line that can be said now: the row, its resolved text and its weight.   (dialogue.ts chooseSamLine's pool)
type Eligible = (crate::dialogue::SamLine, String, f64);

/// Sam's memory of his own lines, made on first asking: the row itself.   (dialogue.ts samMemoryOf)
pub fn sam_memory_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::SamMemory {
    g.m.dialogue.sam_mem_by_game.get_or_insert_with(|| crate::dialogue::SamMemory { said: JsMap::new(), last: None })
}

/// What the man would give at the coming exchange as things stand, or None.   (dialogue.ts peekGive)
pub fn peek_give(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId) -> Option<crate::dialogue::IntelKind> {
    let stage_n = g.tab.game_talk[talk].stage + 1.0;
    let s = stage_key_of(stage_n);
    let sc = live_script(g, ctx, talk, stage_n);
    let given_none = g.tab.game_talk[talk].given.len() == 0;
    if giving_stage(ctx.t, ctx.band, s, stage_n, given_none) && !swallowed(g, sc.as_ref(), s, ctx, talk) {
        intel_for(g, ctx, talk, sc.as_ref()).map(|i| i.kind)
    } else {
        None
    }
}

/// The category for the exchange at stage s.   (dialogue.ts samStageOf)
pub fn sam_stage_of(s: crate::dialogue::StageKey, gives: bool) -> crate::dialogue::SamStage {
    use crate::dialogue::SamStage;
    if is_stage(s, 1.0) { return SamStage::Open; }
    if gives { return SamStage::Ask; }
    if is_stage(s, 2.0) { return SamStage::Press; }
    if is_stage(s, 3.0) { return SamStage::Ask; }
    SamStage::Dry
}

/// Sam's facts for SAM_NEEDS, off the man's context plus the hold's variant. CLOSE.noKnife is settled on: `knife` is never true.   (dialogue.ts samFacts)
pub fn sam_facts(g: &mut crate::game::Game, ctx: &crate::dialogue::DialogueCtx, variant: crate::character::HoldVariant) -> JsMap<crate::dialogue::SamNeed, bool> {
    use crate::character::HoldVariant;
    use crate::dialogue::SamNeed;
    let f = &ctx.facts;
    let lockdown = f.lockdown.unwrap_or(false);
    let alarm = f.floor_up.unwrap_or(false) || f.episode.unwrap_or(false) || lockdown;
    let mut facts = JsMap::new();
    facts.set(SamNeed::Gun, variant == HoldVariant::Gun);
    facts.set(SamNeed::Arm, variant == HoldVariant::Arm);
    facts.set(SamNeed::Knife, false);
    facts.set(SamNeed::Blackout, f.blackout.unwrap_or(false));
    facts.set(SamNeed::Alarm, alarm);
    facts.set(SamNeed::Calm, !alarm);
    facts.set(SamNeed::Lockdown, lockdown);
    facts.set(SamNeed::HeldLong, f.held_long.unwrap_or(false));
    facts
}

/// Does this line fit the man and the moment.   (dialogue.ts samLineFits)
pub fn sam_line_fits(l: &crate::dialogue::SamLine, ctx: &crate::dialogue::DialogueCtx, facts: &JsMap<crate::dialogue::SamNeed, bool>) -> bool {
    if let Some(t) = l.t { if t != ctx.t { return false; } }
    if let Some(k) = l.k { if k != ctx.k { return false; } }
    match &l.needs {
        None => true,
        Some(needs) => needs.iter().all(|n| facts.get(*n).copied().unwrap_or(false)),
    }
}

/// The chooser both of Sam's picks share: the first tier with anything fresh, else anything that fits (never the very last line again while there is
/// another); a weighted draw; the choice written to the memory.   (dialogue.ts chooseSamLine)
pub fn choose_sam_line(g: &mut crate::game::Game, tiers: &[Vec<crate::dialogue::SamLine>], ctx: &crate::dialogue::DialogueCtx, facts: &JsMap<crate::dialogue::SamNeed, bool>, mem: Option<&mut crate::dialogue::SamMemory>, rnd: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> Option<crate::dialogue::ChooseSamLine> {
    let mut mem = mem;
    let mut pool: Vec<Eligible> = Vec::new();
    let mut fresh = false;
    for tier in tiers {
        pool = sam_eligible(g, tier, ctx, facts, mem.as_deref(), true);
        if !pool.is_empty() { fresh = true; break; }
    }
    if pool.is_empty() {
        let flat: Vec<crate::dialogue::SamLine> = tiers.iter().flatten().cloned().collect();
        pool = sam_eligible(g, &flat, ctx, facts, mem.as_deref(), false);
        let last = mem.as_deref().and_then(|m| m.last.clone()).filter(|l| !l.is_empty());
        if pool.len() > 1 {
            if let Some(last) = last {
                let rest: Vec<Eligible> = pool.iter().filter(|p| p.0.id != last).cloned().collect();
                if !rest.is_empty() { pool = rest; }
            }
        }
    }
    if pool.is_empty() { return None; }
    let roll = rnd(g);
    let chosen = pool.swap_remove(sam_draw(&pool, roll));
    if let Some(m) = mem.as_deref_mut() {
        m.said.set(chosen.0.id.clone(), ctx.time);
        m.last = Some(chosen.0.id.clone());
    }
    Some(crate::dialogue::ChooseSamLine { l: chosen.0, text: chosen.1, fresh })
}

/// Sam's line for the coming exchange: the script's beat if a script is live, else the category's pool (the 'q' tier first when the man is about to give).   (dialogue.ts pickSamLine)
pub fn pick_sam_line(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId, variant: crate::character::HoldVariant, mem: Option<&mut crate::dialogue::SamMemory>, rnd: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> crate::dialogue::SamPick {
    use crate::dialogue::{SamLineS, SamPick};
    let mut mem = mem;
    let stage_n = g.tab.game_talk[talk].stage + 1.0;
    let s = stage_key_of(stage_n);
    let q = if is_stage(s, 1.0) { None } else { peek_give(g, ctx, talk) };
    let cat = sam_stage_of(s, q.is_some());
    let sc = live_script(g, ctx, talk, stage_n);
    if let Some(sc) = &sc {
        let beat = beat_for(sc, s, ctx);
        let ask = if q.is_some() { beat.ask.clone().filter(|a| !a.is_empty()) } else { None };
        let asks = ask.is_some();
        let line = match ask {
            Some(a) => a,
            None => { let now = beat.sam_now.clone().unwrap_or_else(|| beat.sam.clone()); crate::words::wg(g, &now, &beat.sam) }
        };
        let tokens = tokens_map(&ctx.tokens);
        if let Some(text) = expand(&line, &tokens).filter(|t| !t.is_empty() && js_len(t) <= crate::dialogue::SAM_MAX_CHARS) {
            let id = alloc::format!("S:{}:{}{}", sc.id, stage_word(s), if asks { "?" } else { "" });
            if let Some(m) = mem.as_deref_mut() { m.said.set(id.clone(), ctx.time); m.last = Some(id.clone()); }
            return SamPick { text, id, s: cat, q, stage: s, fresh: true, script: Some(sc.id.clone()) };
        }
    }
    let tiers: Vec<Vec<crate::dialogue::SamLine>> = match q {
        Some(kind) => vec![
            sam_lines_where(g, |l| l.s == SamLineS::Q && l.q == Some(kind)),
            sam_lines_where(g, |l| l.s == SamLineS::Ask),
        ],
        None => vec![sam_lines_where(g, |l| l.s.as_str() == cat.as_str())],
    };
    let facts = sam_facts(g, ctx, variant);
    let c = choose_sam_line(g, &tiers, ctx, &facts, mem, rnd);
    match c {
        Some(c) => SamPick { text: c.text, id: c.l.id, s: cat, q, stage: s, fresh: c.fresh, script: None },
        None => SamPick { text: String::from("…"), id: String::from("-"), s: cat, q, stage: s, fresh: false, script: None },
    }
}

/// Sam's line for the next exchange with a real man in his arm, remembered on the Game.   (dialogue.ts samLineFor)
pub fn sam_line_for(g: &mut crate::game::Game, gd: GuardId, variant: crate::character::HoldVariant, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> crate::dialogue::SamPick {
    let mut die = |g: &mut Game| crate::math_dice::sim_random(g);
    let rnd: &mut dyn FnMut(&mut Game) -> f64 = match rnd { Some(r) => r, None => &mut die };
    let mut ctx = gather(g, gd);
    let talk = g.gd[gd].talk;
    let mut mem = sam_memory_lend(g);
    let pick = pick_sam_line(g, &mut ctx, talk, variant, Some(&mut mem), rnd);
    *sam_memory_of(g) = mem;
    pick
}

/// Sam's word as he pockets the key a man has just given up: a remark, no exchange.   (dialogue.ts samKeyLineFor)
pub fn sam_key_line_for(g: &mut crate::game::Game, gd: GuardId, variant: crate::character::HoldVariant, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::SamKeyLineFor> {
    use crate::dialogue::SamLineS;
    let mut die = |g: &mut Game| crate::math_dice::sim_random(g);
    let rnd: &mut dyn FnMut(&mut Game) -> f64 = match rnd { Some(r) => r, None => &mut die };
    let ctx = gather(g, gd);
    let tiers = [sam_lines_where(g, |l| l.s == SamLineS::Key)];
    let facts = sam_facts(g, &ctx, variant);
    let mut mem = sam_memory_lend(g);
    let c = choose_sam_line(g, &tiers, &ctx, &facts, Some(&mut mem), rnd);
    *sam_memory_of(g) = mem;
    c.map(|c| crate::dialogue::SamKeyLineFor { text: c.text, id: c.l.id })
}

/// One of Sam's pistol openers as a remark, for the moment he draws on a man already in his arm. Nobody answers it.   (dialogue.ts samRemarkFor)
pub fn sam_remark_for(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::SamRemarkFor> {
    use crate::dialogue::{SamLineS, SamNeed};
    let mut die = |g: &mut Game| crate::math_dice::sim_random(g);
    let rnd: &mut dyn FnMut(&mut Game) -> f64 = match rnd { Some(r) => r, None => &mut die };
    let ctx = gather(g, gd);
    let tiers = [sam_lines_where(g, |l| l.s == SamLineS::Open && l.needs.as_ref().is_some_and(|n| n.contains(&SamNeed::Gun)))];
    let facts = sam_facts(g, &ctx, crate::character::HoldVariant::Gun);
    let mut mem = sam_memory_lend(g);
    let c = choose_sam_line(g, &tiers, &ctx, &facts, Some(&mut mem), rnd);
    *sam_memory_of(g) = mem;
    c.map(|c| crate::dialogue::SamRemarkFor { text: c.text, id: c.l.id })
}

/// The hold's script if it is still live at this exchange: `sc0 && stageN <= 4 && scriptLive(sc0, ctx) ? sc0 : null`, written twice in the TypeScript (:945, :993).
fn live_script(g: &mut Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId, stage_n: f64) -> Option<crate::dialogue::Script> {
    match script_for(g, ctx, talk) {
        Some(sc) if stage_n <= 4.0 && script_live(&sc, ctx) => Some(sc),
        _ => None,
    }
}

/// chooseSamLine's closure `eligible`: the lines of `lines` that fit, are fresh if asked, and resolve to words.
fn sam_eligible(g: &mut Game, lines: &[crate::dialogue::SamLine], ctx: &crate::dialogue::DialogueCtx, facts: &JsMap<crate::dialogue::SamNeed, bool>, mem: Option<&crate::dialogue::SamMemory>, fresh_only: bool) -> Vec<Eligible> {
    let mut out: Vec<Eligible> = Vec::new();
    let tokens = tokens_map(&ctx.tokens);
    for l in lines {
        if !sam_line_fits(l, ctx, facts) { continue; }
        if fresh_only {
            if let Some(m) = mem {
                if let Some(at) = m.said.get(l.id.clone()) {
                    if ctx.time - *at < crate::dialogue::SAM_RECENT_SECS { continue; }
                }
            }
        }
        let said = crate::dialogue::SaidTextL { text: l.text.clone(), now: l.now.clone(), away: l.away.clone(), bare: None, guide: l.guide.clone() };
        let line = said_text(g, &said, ctx);
        let text = match expand(&line, &tokens) { Some(t) if !t.is_empty() => t, _ => continue };
        out.push((l.clone(), text, sam_weight(l)));
    }
    out
}

/// A line's share of the draw: the variant / floor / man lines beat the neutral ones. The product in the TypeScript's order.
fn sam_weight(l: &crate::dialogue::SamLine) -> f64 {
    let w = l.w.unwrap_or(1.0);
    let needs = if l.needs.as_ref().is_some_and(|n| !n.is_empty()) { 1.6 } else { 1.0 };
    let t = if l.t.is_some() { 1.5 } else { 1.0 };
    let k = if l.k.is_some() { 2.5 } else { 1.0 };
    w * needs * t * k
}

/// Which row of the pool a roll of the die picks (the pool is not empty): the walk down the weights, the last row if rounding leaves some over.
fn sam_draw(pool: &[Eligible], roll: f64) -> usize {
    let mut total = 0.0;
    for p in pool { total += p.2; }
    let mut r = roll * total;
    let mut chosen = pool.len() - 1;
    for (i, p) in pool.iter().enumerate() {
        r -= p.2;
        if r <= 0.0 { chosen = i; break; }
    }
    chosen
}

/// `SAM_LINES.filter(...)`, as copies of the rows.
fn sam_lines_where(g: &Game, keep: impl Fn(&crate::dialogue::SamLine) -> bool) -> Vec<crate::dialogue::SamLine> {
    g.m.dialogue.sam_lines.iter().filter(|l| keep(l)).cloned().collect()
}

/// The memory's row out of the game for the length of a pick (a pick takes `g` and the row at once); the caller writes it back with `*sam_memory_of(g) = mem`.
/// Nothing a pick reaches reads the row: the only other samMemoryOf calls in the game are :770 (interrogate) and the :2083 restore.
fn sam_memory_lend(g: &mut Game) -> crate::dialogue::SamMemory {
    core::mem::take(sam_memory_of(g))
}

/// `ctx.tokens` as the map `expand` takes: the keys are the Token words (`sawWhere`), which is how a line's `{SawWhere}` is looked up.
fn tokens_map(t: &crate::dialogue::DialogueCtxTokens) -> JsMap<String, Option<String>> {
    use crate::dialogue::Token;
    let mut m = JsMap::new();
    for (k, v) in [
        (Token::Door, &t.door), (Token::Code, &t.code), (Token::Missing, &t.missing), (Token::Body, &t.body), (Token::N, &t.n), (Token::Up, &t.up),
        (Token::Post, &t.post), (Token::Room, &t.room), (Token::Route, &t.route), (Token::Partner, &t.partner), (Token::Victim, &t.victim),
        (Token::SawWhere, &t.saw_where), (Token::HeardWhere, &t.heard_where), (Token::PartnerRoute, &t.partner_route), (Token::Caller, &t.caller),
        (Token::Checker, &t.checker), (Token::Checked, &t.checked), (Token::ClearRoom, &t.clear_room), (Token::BreakerRoom, &t.breaker_room),
        (Token::CheckEta, &t.check_eta), (Token::Minutes, &t.minutes), (Token::Badge, &t.badge), (Token::PadDoor, &t.pad_door),
    ] { m.set(String::from(k.as_str()), v.clone()); }
    m
}

/// `s === n` of a StageKey and a number.
fn is_stage(s: crate::dialogue::StageKey, n: f64) -> bool {
    matches!(s, crate::dialogue::StageKey::Num(x) if x == n)
}

/// `String(s)` of a StageKey (only ever 1, 2, 3 or 'x': stageKeyOf).
fn stage_word(s: crate::dialogue::StageKey) -> String {
    match s {
        crate::dialogue::StageKey::Num(n) => alloc::format!("{}", n),
        crate::dialogue::StageKey::X => String::from("x"),
    }
}

/// A string's `.length` in JavaScript: UTF-16 units, not bytes (the lines hold "…" and "—").
fn js_len(s: &str) -> f64 {
    s.encode_utf16().count() as f64
}

/// `Object.entries` of an intel table but its 'sc:<script id>' rows (the map `sc`, walked after these): the TypeScript's key beside the field it became (a struct's order, not the literal's: only the order of the lint's messages feels it).
macro_rules! intel_rows { ($t:expr) => { [
    ("steady", &$t.steady), ("nervy", &$t.nervy), ("hard", &$t.hard), ("*", &$t.under),
    ("pedant", &$t.pedant), ("selfown", &$t.selfown), ("folder", &$t.folder), ("ramble", &$t.ramble), ("fan", &$t.fan), ("stoic", &$t.stoic), ("con", &$t.con), ("game", &$t.game), ("menace", &$t.menace),
    ("steady:none", &$t.steady_none), ("steady:heard", &$t.steady_heard), ("steady:glimpsed", &$t.steady_glimpsed), ("steady:saw", &$t.steady_saw), ("steady:sawHurt", &$t.steady_saw_hurt),
    ("nervy:none", &$t.nervy_none), ("nervy:heard", &$t.nervy_heard), ("nervy:glimpsed", &$t.nervy_glimpsed), ("nervy:saw", &$t.nervy_saw), ("nervy:sawHurt", &$t.nervy_saw_hurt),
    ("hard:none", &$t.hard_none), ("hard:heard", &$t.hard_heard), ("hard:glimpsed", &$t.hard_glimpsed), ("hard:saw", &$t.hard_saw), ("hard:sawHurt", &$t.hard_saw_hurt),
    ("*:none", &$t.__none), ("*:heard", &$t.__heard), ("*:glimpsed", &$t.__glimpsed), ("*:saw", &$t.__saw), ("*:sawHurt", &$t.__saw_hurt),
    ("pedant:steady", &$t.pedant_steady), ("pedant:nervy", &$t.pedant_nervy), ("pedant:hard", &$t.pedant_hard),
    ("selfown:steady", &$t.selfown_steady), ("selfown:nervy", &$t.selfown_nervy), ("selfown:hard", &$t.selfown_hard),
    ("folder:steady", &$t.folder_steady), ("folder:nervy", &$t.folder_nervy), ("folder:hard", &$t.folder_hard),
    ("ramble:steady", &$t.ramble_steady), ("ramble:nervy", &$t.ramble_nervy), ("ramble:hard", &$t.ramble_hard),
    ("fan:steady", &$t.fan_steady), ("fan:nervy", &$t.fan_nervy), ("fan:hard", &$t.fan_hard),
    ("stoic:steady", &$t.stoic_steady), ("stoic:nervy", &$t.stoic_nervy), ("stoic:hard", &$t.stoic_hard),
    ("con:steady", &$t.con_steady), ("con:nervy", &$t.con_nervy), ("con:hard", &$t.con_hard),
    ("game:steady", &$t.game_steady), ("game:nervy", &$t.game_nervy), ("game:hard", &$t.game_hard),
    ("menace:steady", &$t.menace_steady), ("menace:nervy", &$t.menace_nervy), ("menace:hard", &$t.menace_hard),
] } }
use intel_rows;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogue::{Knowledge, SamLine, SamNeed, SamStage, StageKey};
    use crate::game::Temperament;

    #[test]
    fn stage_categories_opener_beats_giving_and_giving_beats_the_stage() {
        let cases = [
            (StageKey::Num(1.0), false, SamStage::Open), (StageKey::Num(1.0), true, SamStage::Open),
            (StageKey::Num(2.0), false, SamStage::Press), (StageKey::Num(2.0), true, SamStage::Ask),
            (StageKey::Num(3.0), false, SamStage::Ask), (StageKey::Num(3.0), true, SamStage::Ask),
            (StageKey::X, false, SamStage::Dry), (StageKey::X, true, SamStage::Ask),
        ];
        for (s, gives, want) in cases { assert_eq!(sam_stage_of(s, gives), want); }
    }

    /// the weights against the TypeScript's own bits (`1.4 * 1.6 * 1.5 * 2.5` under bun: 0x4020cccccccccccc, not 8.4)
    #[test]
    fn weights_multiply_in_the_typescript_order() {
        let mut l = SamLine::zeroed();
        assert_eq!(sam_weight(&l).to_bits(), 1.0f64.to_bits());
        l.needs = Some(Vec::new());
        assert_eq!(sam_weight(&l).to_bits(), 1.0f64.to_bits());   // an empty list is falsy by its length
        l.needs = Some(vec![SamNeed::Gun]);
        assert_eq!(sam_weight(&l).to_bits(), 0x3ff999999999999a);
        l.w = Some(1.4);
        assert_eq!(sam_weight(&l).to_bits(), 0x4001eb851eb851eb);
        l.t = Some(Temperament::Hard);
        l.k = Some(Knowledge::None);   // 'none' is a truthy word: it weighs
        assert_eq!(sam_weight(&l).to_bits(), 0x4020cccccccccccc);
    }

    fn pool(weights: &[f64]) -> Vec<Eligible> {
        weights.iter().map(|w| (SamLine::zeroed(), String::new(), *w)).collect()
    }

    #[test]
    fn the_draw_walks_the_weights_and_falls_to_the_last() {
        let p = pool(&[1.0, 2.0, 1.0]);
        assert_eq!(sam_draw(&p, 0.0), 0);    // a roll of zero stops on the first row (r <= 0)
        assert_eq!(sam_draw(&p, 0.25), 0);   // exactly its share: still the first
        assert_eq!(sam_draw(&p, 0.2500001), 1);
        assert_eq!(sam_draw(&p, 0.75), 1);
        assert_eq!(sam_draw(&p, 0.9), 2);
        assert_eq!(sam_draw(&p, 2.0), 2);    // nothing stopped the walk: the last row
    }

    #[test]
    fn a_line_named_for_a_man_fits_only_him() {
        let mut ctx = crate::dialogue::DialogueCtx::zeroed();
        ctx.t = Temperament::Hard;
        ctx.k = Knowledge::Saw;
        let facts = JsMap::new();
        let mut l = SamLine::zeroed();
        assert!(sam_line_fits(&l, &ctx, &facts));
        l.t = Some(Temperament::Nervy);
        assert!(!sam_line_fits(&l, &ctx, &facts));
        l.t = Some(Temperament::Hard);
        assert!(sam_line_fits(&l, &ctx, &facts));
        l.k = Some(Knowledge::None);   // 'none' asks for a man who knows nothing, not for any man
        assert!(!sam_line_fits(&l, &ctx, &facts));
    }

    #[test]
    fn a_line_with_needs_wants_every_fact_on() {
        let ctx = crate::dialogue::DialogueCtx::zeroed();
        let mut facts = JsMap::new();
        facts.set(SamNeed::Gun, true);
        facts.set(SamNeed::Alarm, false);
        let mut l = SamLine::zeroed();
        l.needs = Some(vec![SamNeed::Gun]);
        assert!(sam_line_fits(&l, &ctx, &facts));
        l.needs = Some(vec![SamNeed::Gun, SamNeed::Alarm]);
        assert!(!sam_line_fits(&l, &ctx, &facts));
        l.needs = Some(vec![SamNeed::Knife]);   // no row at all reads false
        assert!(!sam_line_fits(&l, &ctx, &facts));
    }

    #[test]
    fn the_memory_is_the_row_and_a_write_through_it_stays() {
        let mut g = Game::blank();
        assert!(g.m.dialogue.sam_mem_by_game.is_none());
        sam_memory_of(&mut g).last = Some(String::from("So1"));
        sam_memory_of(&mut g).said.set(String::from("So1"), 3.0);
        assert_eq!(g.m.dialogue.sam_mem_by_game.as_ref().unwrap().last.as_deref(), Some("So1"));
        let mut lent = sam_memory_lend(&mut g);
        lent.last = Some(String::from("Sp1"));
        *sam_memory_of(&mut g) = lent;
        let row = sam_memory_of(&mut g);
        assert_eq!(row.last.as_deref(), Some("Sp1"));
        assert_eq!(row.said.get(String::from("So1")).copied(), Some(3.0));
    }

    #[test]
    fn sams_facts_read_the_floor_and_the_hold_and_the_knife_is_gone() {
        let mut g = Game::blank();
        let mut ctx = crate::dialogue::DialogueCtx::zeroed();
        let f = sam_facts(&mut g, &ctx, crate::character::HoldVariant::Arm);
        assert_eq!(f.get(SamNeed::Arm), Some(&true));
        assert_eq!(f.get(SamNeed::Gun), Some(&false));
        assert_eq!(f.get(SamNeed::Knife), Some(&false));   // CLOSE.noKnife, with the arm on him too
        assert_eq!(f.get(SamNeed::Calm), Some(&true));
        ctx.facts.lockdown = Some(true);
        let f = sam_facts(&mut g, &ctx, crate::character::HoldVariant::Gun);
        assert_eq!(f.get(SamNeed::Alarm), Some(&true));    // a lockdown is an alarm
        assert_eq!(f.get(SamNeed::Calm), Some(&false));
        assert_eq!(f.get(SamNeed::Lockdown), Some(&true));
    }

    #[test]
    fn lengths_count_as_javascript_does_and_stages_print_as_it_does() {
        assert_eq!(js_len("Lights die — what's next…"), 25.0);
        assert_eq!(js_len("—"), 1.0);
        assert_eq!(stage_word(StageKey::Num(2.0)), "2");
        assert_eq!(stage_word(StageKey::X), "x");
    }

    #[test]
    fn the_token_map_keys_are_the_words_a_line_is_looked_up_by() {
        let mut t = crate::dialogue::DialogueCtxTokens::zeroed();
        t.saw_where = Some(String::from("the vault"));
        let m = tokens_map(&t);
        assert_eq!(m.get(String::from("sawWhere")), Some(&Some(String::from("the vault"))));
        assert_eq!(m.get(String::from("door")), Some(&None));
        assert_eq!(m.len(), crate::dialogue::tokens().len());
    }

}
