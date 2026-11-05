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

/// A hand-made context for the cell (k, band, t): every token filled with a plausible value unless overridden, the facts a knowledge level implies on.   (dialogue.ts synthCtx)
pub fn synth_ctx(g: &mut crate::game::Game, k: crate::dialogue::Knowledge, band: crate::dialogue::Band, t: crate::game::Temperament, over: Option<&crate::dialogue::SynthCtxOver>) -> crate::dialogue::DialogueCtx {
    use crate::dialogue::{Band, DialogueCtx, DialogueCtxFacts, DialogueCtxTokens, IntelItem, IntelKind, Knowledge};
    let blank = crate::dialogue::SynthCtxOver::zeroed();
    let over = over.unwrap_or(&blank);
    let a = if band == Band::High { 0.75 } else if band == Band::Mid { 0.45 } else { 0.1 };
    let who = over.who.clone().unwrap_or_else(|| String::from("synthetic"));
    let word = |w: &str| Some(String::from(w));
    let mut tokens = DialogueCtxTokens::zeroed();
    tokens.partner = word("Novak");
    tokens.victim = if k == Knowledge::SawHurt { word("Kowalski") } else { None };
    tokens.room = word("the corridor");
    tokens.saw_where = if k == Knowledge::Glimpsed || k == Knowledge::Saw || k == Knowledge::SawHurt { word("the server room") } else { None };
    tokens.heard_where = if k == Knowledge::Heard { word("the cubicles") } else { None };
    tokens.route = word("the corridor");
    tokens.partner_route = word("the cubicles");
    tokens.post = word("the east end");
    tokens.door = word("the server room door");
    tokens.caller = word("Reyes");
    tokens.breaker_room = word("the storage room");
    tokens.check_eta = word("in a minute or so");
    tokens.n = word("three");
    tokens.up = word("three");
    tokens.badge = Some(badge_of(&who));
    tokens.code = word("7391");
    tokens.pad_door = word("the office side door");
    if let Some(o) = &over.tokens {
        macro_rules! lay { ($($f:ident),*) => { $( if o.$f.is_some() { tokens.$f = o.$f.clone(); } )* } }
        lay!(door, code, missing, body, n, up, post, room, route, partner, victim, saw_where, heard_where, partner_route, caller, checker, checked, clear_room, breaker_room, check_eta, minutes, badge, pad_door);
    }
    let mut facts = DialogueCtxFacts::zeroed();
    facts.heard_small = Some(k == Knowledge::Heard);
    facts.saw_shot = Some(k == Knowledge::SawHurt);
    facts.partner_known = Some(tokens.partner.as_ref().is_some_and(|p| !p.is_empty()));
    facts.shift = Some(true);
    facts.office = Some(true);
    if let Some(o) = &over.facts {
        macro_rules! lay { ($($f:ident),*) => { $( if o.$f.is_some() { facts.$f = o.$f; } )* } }
        lay!(shift, rollcall_live, blackout, body_found, floor_up, lockdown, episode, heard_small, heard_shots, heard_bang, body_called, body_known, missing_known, held_long, dazzled, saw_shot, saw_drop, alert_now, drew_on_you, partner_known, office);
    }
    let live: Vec<IntelKind> = over.intel_live.clone().unwrap_or_else(|| vec![IntelKind::Door, IntelKind::Rollcall, IntelKind::Routes]);
    let mut intel: Vec<IntelItem> = Vec::new();
    for kind in INTEL_KIND_LIST {
        let text = intel_text(g, kind, t, k);
        intel.push(IntelItem { kind, text, live: live.contains(&kind), known: true });
    }
    DialogueCtx { who, k, a, band, t, why: Vec::new(), facts, tokens, own_rooms: None, nameless: None, intel, time: 0.0, recent: None }
}

/// The blocklisted phrase inside `text`, or None.   (dialogue.ts blockedPhrase)
pub fn blocked_phrase(text: &str) -> Option<String> {
    let s = text.to_lowercase();
    crate::dialogue::originality_blocklist().into_iter().find(|b| s.contains(b.as_str()))
}

/// Dev lint over the tables (the QA probe runs it; empty = clean).   (dialogue.ts lintDialogue)
pub fn lint_dialogue(g: &mut crate::game::Game) -> Vec<String> {
    use crate::dialogue::{Band, Him, Knowledge, LineK, SamLineS, SamNeed, ShapeKey, StageKey};
    use crate::game::Temperament;
    let f = |x: &str| String::from(x);
    let mut worst_tokens: JsMap<String, Option<String>> = JsMap::new();
    for (k, v) in token_worst(g).iter() { worst_tokens.set(f(k.as_str()), Some(v.clone())); }
    let mut p = Lint { problems: Vec::new(), ids: JsSet::new(), texts: JsSet::new(), worst_tokens };
    let sam_lines = g.m.dialogue.sam_lines.clone();
    let lines = crate::dialogue::lines();
    let scripts = crate::dialogue::scripts();
    let tokens_all = crate::dialogue::tokens();
    let always = crate::dialogue::tokens_always();
    let by_k = crate::dialogue::tokens_by_k();
    let sam_needs = crate::dialogue::sam_needs();
    let sam_stages = crate::dialogue::sam_stages();
    let intel_kinds = crate::dialogue::intel_kinds();
    let knowledge = crate::dialogue::knowledge();
    let temperaments = crate::dialogue::temperaments();
    let shapes = crate::dialogue::shapes();
    let needs = crate::dialogue::needs();
    let stages = crate::dialogue::stages();
    let intel_text_tab = crate::dialogue::intel_text_table();
    let shape_lead = crate::dialogue::shape_lead();
    let has = |list: &[String], w: &str| list.iter().any(|x| x == w);
    let sure_of = |k: &str| -> Vec<String> {
        let mut s = always.clone();
        if let Some(own) = by_k.get(f(k)) { s.extend(own.iter().cloned()); }
        s
    };

    for l in &sam_lines {
        p.id(&l.id, alloc::format!("duplicate id {}", l.id));
        if l.s == SamLineS::Script { continue; }
        p.dup(&l.id, &l.text); p.orig(&l.id, &l.text);
        if l.text.trim().is_empty() { p.push(alloc::format!("{}: empty", l.id)); }
        let sure = match l.k { Some(k) => sure_of(k.as_str()), None => always.clone() };
        for tk in tokens_in(&l.text) {
            if !has(&tokens_all, &tk) { p.push(alloc::format!("{}: unknown token {{{}}}", l.id, tk)); }
            else if !sure.contains(&tk) {
                let who = match l.k { Some(k) => alloc::format!("k '{}'", k.as_str()), None => f("any k") };
                p.push(alloc::format!("{}: token {{{}}} is not guaranteed for {} (the player's lines must always resolve)", l.id, tk, who));
            }
        }
        let listed: &[SamNeed] = l.needs.as_deref().unwrap_or(&[]);
        for n in listed { if !has(&sam_needs, n.as_str()) { p.push(alloc::format!("{}: unknown need '{}'", l.id, n.as_str())); } }
        let mut n_set: JsSet<SamNeed> = JsSet::new();
        for n in listed { n_set.add(*n); }
        if (n_set.has(SamNeed::Gun) && (n_set.has(SamNeed::Arm) || n_set.has(SamNeed::Knife))) || (n_set.has(SamNeed::Alarm) && n_set.has(SamNeed::Calm)) || (n_set.has(SamNeed::Calm) && n_set.has(SamNeed::Lockdown)) {
            let words: Vec<&str> = n_set.iter().map(|n| n.as_str()).collect();
            p.push(alloc::format!("{}: contradictory needs [{}] — the line can never come up", l.id, words.join(", ")));
        }
        let w = p.worst(&l.text);
        if js_len(&w) > crate::dialogue::SAM_MAX_CHARS { p.push(alloc::format!("{}: {} chars after expansion (> {}): \"{}\"", l.id, js_len(&w), crate::dialogue::SAM_MAX_CHARS, w)); }
        if let Some(lw) = l.w { if !(lw > 0.0) { p.push(alloc::format!("{}: weight {} (must be > 0)", l.id, lw)); } }
        if l.s != SamLineS::Q && l.s != SamLineS::Key && !has(&sam_stages, l.s.as_str()) { p.push(alloc::format!("{}: bad stage '{}'", l.id, l.s.as_str())); }
        if l.s == SamLineS::Q && !l.q.is_some_and(|q| has(&intel_kinds, q.as_str())) { p.push(alloc::format!("{}: a 'q' line needs q from INTEL_KINDS (has {})", l.id, l.q.map_or("none", |q| q.as_str()))); }
        if l.s != SamLineS::Q { if let Some(q) = l.q { p.push(alloc::format!("{}: q '{}' on a '{}' line (only 'q' lines carry one)", l.id, q.as_str(), l.s.as_str())); } }
        if let Some(t) = l.t { if !has(&temperaments, t.as_str()) { p.push(alloc::format!("{}: bad t {}", l.id, t.as_str())); } }
        if let Some(k) = l.k { if !has(&knowledge, k.as_str()) { p.push(alloc::format!("{}: bad k {}", l.id, k.as_str())); } }
    }
    // the reworded rows (now / away / bare / guide): only what a new wording can break
    for l in &sam_lines { p.reworded(&l.id, &l.text, &[("now", l.now.as_deref()), ("away", l.away.as_deref()), ("guide", l.guide.as_deref())], crate::dialogue::SAM_MAX_CHARS); }
    for l in &lines { p.reworded(&l.id, &l.text, &[("now", l.now.as_deref()), ("away", l.away.as_deref()), ("bare", l.bare.as_deref()), ("guide", l.guide.as_deref())], crate::dialogue::MAX_CHARS); }
    let plain = |l: &crate::dialogue::SamLine| l.t.is_none() && l.k.is_none() && l.needs.as_ref().map_or(true, |n| n.is_empty());
    for s in &sam_stages {
        let n = sam_lines.iter().filter(|l| l.s.as_str() == s && plain(l)).count();
        if (n as f64) < crate::dialogue::SAM_POOL_MIN { p.push(alloc::format!("Player '{}': only {} lines that need nothing (want ≥ {} — a hold with no colour on would run dry)", s, n, crate::dialogue::SAM_POOL_MIN)); }
    }
    {
        let n = sam_lines.iter().filter(|l| l.s == SamLineS::Key && plain(l)).count();
        if (n as f64) < crate::dialogue::SAM_KEY_MIN { p.push(alloc::format!("Player 'key': only {} remarks that need nothing (want ≥ {} — a key handed over must always get a word)", n, crate::dialogue::SAM_KEY_MIN)); }
    }
    for v in [SamNeed::Gun, SamNeed::Arm] {
        for s in &sam_stages {
            if s == "ask" { continue; }
            if !sam_lines.iter().any(|l| l.s.as_str() == s && l.needs.as_ref().is_some_and(|n| n.contains(&v))) { p.push(alloc::format!("Player '{}': no line for the '{}' hold", s, v.as_str())); }
        }
    }
    for l in &lines {
        p.id(&l.id, alloc::format!("duplicate id {}", l.id));
        p.dup(&l.id, &l.text); p.orig(&l.id, &l.text);
        for tk in tokens_in(&l.text) { if !has(&tokens_all, &tk) { p.push(alloc::format!("{}: unknown token {{{}}}", l.id, tk)); } }
        for n in l.needs.as_deref().unwrap_or(&[]) { if !has(&needs, n.as_str()) { p.push(alloc::format!("{}: unknown need '{}'", l.id, n.as_str())); } }
        let w = p.worst(&l.text);
        if js_len(&w) > crate::dialogue::MAX_CHARS { p.push(alloc::format!("{}: {} chars after expansion (> {}): \"{}\"", l.id, js_len(&w), crate::dialogue::MAX_CHARS, w)); }
        if l.text.trim().is_empty() { p.push(alloc::format!("{}: empty", l.id)); }
        if let Some(lw) = l.w { if !(lw > 0.0) { p.push(alloc::format!("{}: weight {} (must be > 0 — the weighted pick degenerates otherwise)", l.id, lw)); } }
        if l.k != LineK::Empty && !has(&knowledge, l.k.as_str()) { p.push(alloc::format!("{}: bad k {}", l.id, l.k.as_str())); }
        if !stages.iter().any(|s| stage_is(*s, l.s)) { p.push(alloc::format!("{}: bad stage {}", l.id, stage_word(l.s))); }
    }
    // the useful things: keys, tokens, and length with the worst lead-in anybody who can say that text can put in front (shape-voiced texts take none)
    let mut shape_ts: JsMap<ShapeKey, JsSet<Temperament>> = JsMap::new();
    for sc in &scripts { shape_ts.get_or_insert_with(sc.shape, JsSet::new).add(sc.t); }
    for kind in INTEL_KIND_LIST {
        let table = intel_table(g, kind);
        let mut rows: Vec<(String, String)> = intel_rows!(table).into_iter().filter_map(|(k, v)| v.clone().map(|t| (f(k), t))).collect();
        if kind == crate::dialogue::IntelKind::CheckOn { rows.push((f("self"), check_on_self(g))); }
        let own = intel_text_tab.get(f(kind.as_str()));
        for (tk, text) in &rows {
            if text.is_empty() { continue; }
            let at = alloc::format!("intel {}/{}", kind.as_str(), tk);
            p.orig(&at, text); p.dup(&at, text);
            for t2 in tokens_in(text) { if !has(&tokens_all, &t2) { p.push(alloc::format!("{}: unknown token {{{}}}", at, t2)); } }
            let (a_part, b_part) = match tk.split_once(':') { Some((a, b)) => (a, Some(b)), None => (tk.as_str(), None) };
            let is_shape = has(&shapes, a_part);
            if a_part != "*" && a_part != "self" && !is_shape && !has(&temperaments, a_part) { p.push(alloc::format!("intel {}: bad key '{}'", kind.as_str(), tk)); }
            if let Some(b) = b_part { if !(if is_shape { has(&temperaments, b) } else { has(&knowledge, b) }) { p.push(alloc::format!("intel {}: bad second part in key '{}'", kind.as_str(), tk)); } }
            if kind == crate::dialogue::IntelKind::Door {
                let low = text.to_lowercase();
                if !["belt", "mine", "pocket", "clip"].iter().any(|w| low.contains(w)) { p.push(alloc::format!("intel door/{}: \"{}\" never puts the key on his person — the player takes it off him as this is said", tk, text)); }
            }
            let len = if is_shape { js_len(&p.worst(text)) } else {
                let users: Vec<Temperament> = if a_part == "*" || a_part == "self" {
                    TEMPERAMENT_LIST.into_iter().filter(|t| a_part == "self" || !own.is_some_and(|o| intel_own_of(o, *t).is_some_and(|x| !x.is_empty()))).collect()
                } else { TEMPERAMENT_LIST.into_iter().filter(|t| t.as_str() == a_part).collect() };
                let ks: Vec<Knowledge> = match b_part { Some(b) => KNOWLEDGE_LIST.into_iter().filter(|k| k.as_str() == b).collect(), None => KNOWLEDGE_LIST.to_vec() };
                js_len(&p.worst(text)) + lead_worst(&shape_ts, &shape_lead, &users, &ks)
            };
            if len > crate::dialogue::MAX_CHARS { p.push(alloc::format!("{}: {} chars after expansion with the lead-in (> {}): \"{}\"", at, len, crate::dialogue::MAX_CHARS, p.worst(text))); }
        }
    }
    let drive = drive_away(g);
    let drive_tab = crate::dialogue::drive_away_table();
    for (tk, text) in [("steady", &drive.steady), ("nervy", &drive.nervy), ("hard", &drive.hard), ("*", &drive.under)] {
        let Some(text) = text else { continue };
        let at = alloc::format!("intel drive/away {}", tk);
        p.orig(&at, text); p.dup(&at, text);
        let users: Vec<Temperament> = if tk == "*" {
            TEMPERAMENT_LIST.into_iter().filter(|t| match t { Temperament::Steady => drive_tab.steady.is_none(), Temperament::Nervy => drive_tab.nervy.is_none(), Temperament::Hard => drive_tab.hard.is_none() }).collect()
        } else { TEMPERAMENT_LIST.into_iter().filter(|t| t.as_str() == tk).collect() };
        let len = js_len(text) + lead_worst(&shape_ts, &shape_lead, &users, &KNOWLEDGE_LIST);
        if !tokens_in(text).is_empty() || len > crate::dialogue::MAX_CHARS { p.push(alloc::format!("{}: a token, or {} chars with the lead-in (> {})", at, len, crate::dialogue::MAX_CHARS)); }
    }
    // coverage of the pools: reachable without needs (nor `only`) and with only the guaranteed tokens
    for k in KNOWLEDGE_LIST { for band in BAND_LIST { for t in TEMPERAMENT_LIST { for s in &stages {
        let sure = sure_of(k.as_str());
        let reach = |own: bool| lines.iter().any(|l| line_matches(l, k, band, t, *s, false, false) && l.needs.as_ref().map_or(true, |n| n.is_empty()) && l.only.is_none() && tokens_in(&l.text).iter().all(|x| sure.contains(x)) && (!own || l.k.as_str() == k.as_str()));
        if !reach(false) { p.push(alloc::format!("cell {}·{}·{} stage {}: no line always reachable", k.as_str(), band.as_str(), t.as_str(), stage_word(*s))); }
        else if (is_stage(*s, 1.0) || is_stage(*s, 2.0)) && !reach(true) { p.push(alloc::format!("cell {}·{}·{} stage {}: only floor-colour lines, none written for '{}'", k.as_str(), band.as_str(), t.as_str(), stage_word(*s), k.as_str())); }
    } } } }
    // …and a spawned unit's own cells (always hard, no `only` row, no man's name)
    for k in KNOWLEDGE_LIST { for band in BAND_LIST { for s in &stages {
        let mut sure = always.clone();
        if let Some(own) = by_k.get(f(k.as_str())) { sure.extend(own.iter().filter(|x| *x != "victim").cloned()); }
        let any = lines.iter().any(|l| line_matches(l, k, band, Temperament::Hard, *s, true, false) && l.needs.as_ref().map_or(true, |n| n.is_empty()) && l.only.is_none() && tokens_in(l.bare.as_deref().unwrap_or(&l.text)).iter().all(|x| sure.contains(x)));
        if !any { p.push(alloc::format!("cell {}·{}·unit stage {}: no line a unit can always reach", k.as_str(), band.as_str(), stage_word(*s))); }
    } } }
    // the scripts
    let mut per_shape_t: JsMap<String, f64> = JsMap::new();
    for sc in &scripts {
        let at = alloc::format!("script {}", sc.id);
        p.id(&sc.id, alloc::format!("duplicate id {} (script)", sc.id));
        if !has(&shapes, sc.shape.as_str()) { p.push(alloc::format!("{}: unknown shape '{}'", at, sc.shape.as_str())); }
        if !has(&temperaments, sc.t.as_str()) { p.push(alloc::format!("{}: bad t '{}'", at, sc.t.as_str())); }
        if sc.k.is_empty() || sc.k.iter().any(|k| !has(&knowledge, k.as_str())) {
            let words: Vec<&str> = sc.k.iter().map(|k| k.as_str()).collect();
            p.push(alloc::format!("{}: bad k list [{}]", at, words.join(", ")));
        }
        if sc.k.contains(&Knowledge::SawHurt) { p.push(alloc::format!("{}: written for sawHurt — a man who watched the player hurt somebody is frightened, never funny (the pools have him)", at)); }
        if let Some(b) = &sc.band { if b.is_empty() || b.iter().any(|x| !BAND_LIST.contains(x)) { p.push(alloc::format!("{}: bad band list", at)); } }
        for n in sc.needs.iter().flatten().chain(sc.not.iter().flatten()) { if !has(&needs, n.as_str()) { p.push(alloc::format!("{}: unknown need '{}'", at, n.as_str())); } }
        if let Some(pin) = sc.pin { if !has(&intel_kinds, pin.as_str()) { p.push(alloc::format!("{}: pin '{}' is no useful thing", at, pin.as_str())); } }
        let key = alloc::format!("{}·{}", sc.shape.as_str(), sc.t.as_str());
        let was = per_shape_t.get(key.clone()).copied().unwrap_or(0.0);
        per_shape_t.set(key, was + 1.0);
        let mut wide = always.clone();
        if sc.pin == Some(crate::dialogue::IntelKind::Door) { wide.push(f("door")); }
        wide.push(f("partner"));
        let gives = gives_at(sc.t, Band::Mid);
        for s in &stages {
            let s = *s;
            let b = beat_of(&sc.beats, s);
            let bt = alloc::format!("{} beat {}", at, stage_word(s));
            p.sam_len(&alloc::format!("{} sam", bt), Some(&b.sam));
            p.sam_len(&alloc::format!("{} sam now", bt), b.sam_now.as_deref());
            if is_stage(s, 1.0) && !tokens_in(&b.sam).is_empty() { p.push(alloc::format!("{}: the opener \"{}\" carries a token — openers stay literal (other probes match the player's first line verbatim)", bt, b.sam)); }
            for tk in tokens_in(&b.sam) { if !wide.contains(&tk) { p.push(alloc::format!("{} sam: token {{{}}} is not guaranteed", bt, tk)); } }
            for (key, text) in him_strings(&b.him) {
                p.him_len(&alloc::format!("{} him/{}", bt, key), &text);
                let mut ok = wide.clone();
                if has(&knowledge, &key) { if let Some(own) = by_k.get(key.clone()) { ok.extend(own.iter().cloned()); } }
                for tk in tokens_in(&text) {
                    if !has(&tokens_all, &tk) { p.push(alloc::format!("{} him/{}: unknown token {{{}}}", bt, key, tk)); }
                    else if !ok.contains(&tk) { p.push(alloc::format!("{} him/{}: token {{{}}} is not guaranteed", bt, key, tk)); }
                }
                if key != "*" && !has(&knowledge, &key) && !has(&temperaments, &key) { p.push(alloc::format!("{} him: bad key '{}'", bt, key)); }
                if has(&knowledge, &key) && !sc.k.iter().any(|k| k.as_str() == key) { p.push(alloc::format!("{} him/{}: a variant for a knowledge the script is not written for", bt, key)); }
            }
            if let Him::Arm1(m) = &b.him {
                if !m.has(f("*")) && !sc.k.iter().all(|k| m.has(f(k.as_str()))) { p.push(alloc::format!("{} him: no '*' fallback and not every k covered", bt)); }
            }
            let room = b.room == Some(true);
            if room && !is_stage(s, 1.0) && !is_stage(s, 2.0) { p.push(alloc::format!("{}: a room beat past the second press (the menace is the threat, not the payoff)", bt)); }
            p.sam_len(&alloc::format!("{} ask", bt), b.ask.as_deref());
            p.sam_len(&alloc::format!("{} after", bt), b.after.as_deref());
            p.sam_len(&alloc::format!("{} take", bt), b.take.as_deref());
            if let Some(mild) = &b.mild { p.him_len(&alloc::format!("{} mild", bt), mild); }
            for text in [&b.ask, &b.after, &b.take, &b.mild] {
                let Some(text) = text else { continue };
                if text.is_empty() { continue; }
                for tk in tokens_in(text) {
                    if !has(&tokens_all, &tk) { p.push(alloc::format!("{}: unknown token {{{}}}", bt, tk)); }
                    else if !wide.contains(&tk) { p.push(alloc::format!("{}: token {{{}}} is not guaranteed", bt, tk)); }
                }
            }
            let asks = b.ask.as_ref().is_some_and(|a| !a.is_empty());
            let takes = b.take.as_ref().is_some_and(|a| !a.is_empty());
            if is_stage(s, 1.0) && asks { p.push(alloc::format!("{}: an 'ask' on the opener (nobody gives at the first exchange)", bt)); }
            if (asks || takes) && !gives.iter().any(|x| stage_is(*x, s)) && !matches!(s, StageKey::X) { p.push(alloc::format!("{}: ask / take on a beat a {} man never gives at", bt, sc.t.as_str())); }
            if room && is_stage(s, 1.0) && asks { p.push(alloc::format!("{}: room opener with an ask", bt)); }
        }
    }
    for (key, n) in per_shape_t.iter() { if n < &crate::dialogue::SCRIPT_VARIANTS_MIN { p.push(alloc::format!("scripts {}: only {} written (want ≥ {} variants per shape × temperament)", key, n, crate::dialogue::SCRIPT_VARIANTS_MIN)); } }
    // every man below sawHurt, on a plain floor with the ordinary useful things live, draws SOME script
    let lint_over = crate::dialogue::SynthCtxOver { who: Some(f("lint")), ..Default::default() };
    for t in TEMPERAMENT_LIST { for k in KNOWLEDGE_LIST { for band in BAND_LIST {
        if k == Knowledge::SawHurt { continue; }
        let ctx = synth_ctx(g, k, band, t, Some(&lint_over));
        let talk = crate::game::new_talk(g);
        let mut any = false;
        for sc in &scripts { if script_fits(g, sc, &ctx, talk) { any = true; break; } }
        if !any { p.push(alloc::format!("scripts: nothing fits {}·{}·{} on a plain floor (door / rollcall / routes live)", t.as_str(), k.as_str(), band.as_str())); }
    } } }
    // the rooms (a room with no entry gets the menace script's own generic beat)
    for (room, list) in crate::dialogue::room_menace().iter() {
        if list.len() < 2 { p.push(alloc::format!("room '{}': only {} variant (want ≥ 2)", room, list.len())); }
        for (i, r) in list.iter().enumerate() {
            let at = alloc::format!("room '{}' #{}", room, i);
            p.sam_len(&alloc::format!("{} sam", at), Some(&r.sam));
            if !tokens_in(&r.sam).is_empty() { p.push(alloc::format!("{}: token in the player's threat (rooms stay literal)", at)); }
            for (key, text) in him_strings(&r.him) {
                p.him_len(&alloc::format!("{} him/{}", at, key), &text);
                if tokens_in(&text).iter().any(|tk| !always.contains(tk)) { p.push(alloc::format!("{} him/{}: token not always resolvable", at, key)); }
                if key != "*" && !has(&temperaments, &key) { p.push(alloc::format!("{} him: bad key '{}' (rooms vary by temperament only)", at, key)); }
            }
            if let Him::Arm1(m) = &r.him { if !m.has(f("*")) { p.push(alloc::format!("{}: no '*' reply", at)); } }
            p.sam_len(&alloc::format!("{} after", at), r.after.as_deref());
        }
    }
    // the handler
    let handler_lines = crate::dialogue::handler_lines();
    for beat in crate::dialogue::handler_beats() {
        let list = handler_lines.get(beat.clone());
        let n = list.map_or(0, |l| l.len());
        let Some(list) = list.filter(|l| l.len() >= 2) else { p.push(alloc::format!("handler '{}': {} exchanges (want ≥ 2)", beat, n)); continue };
        for (i, x) in list.iter().enumerate() {
            let at = alloc::format!("handler '{}' #{}", beat, i);
            let mut says: Vec<(&str, &String)> = vec![("handler", &x.h), ("sam", &x.s)];
            if let Some(h_now) = x.h_now.as_ref().filter(|h| !h.is_empty()) { says.push(("handler now", h_now)); }
            for (who, text) in says {
                let w0 = alloc::format!("{} {}", at, who);
                p.orig(&w0, text); p.dup(&w0, text);
                let w = text.replace("{rack}", "12");
                if js_len(&w) > crate::dialogue::SAM_MAX_CHARS { p.push(alloc::format!("{}: \"{}\" is {} chars (> {} — the radio strip and the player's bubble are one line)", w0, w, js_len(&w), crate::dialogue::SAM_MAX_CHARS)); }
                if brace_other_than(text, &["rack}"]) { p.push(alloc::format!("{}: only {{rack}} resolves on the radio", w0)); }
            }
        }
    }
    // the pairs
    let pairs = crate::dialogue::ambient_pairs();
    if pairs.len() < 6 { p.push(alloc::format!("pairs: only {} conversations (want ≥ 6)", pairs.len())); }
    for pr in &pairs {
        let at = alloc::format!("pair '{}'", pr.id);
        p.id(&alloc::format!("pair:{}", pr.id), alloc::format!("duplicate pair id {}", pr.id));
        if pr.lines.len() < 2 || pr.lines.len() > 4 { p.push(alloc::format!("{}: {} lines (want 2‥4)", at, pr.lines.len())); }
        if pr.lines.first().map(|l| l.who) != Some(0.0) { p.push(alloc::format!("{}: the first line is not the opener's (who 0)", at)); }
        for (i, l) in pr.lines.iter().enumerate() {
            let w0 = alloc::format!("{} #{}", at, i);
            p.orig(&w0, &l.text); p.dup(&w0, &l.text);
            let w = pair_worst(&l.text);
            if js_len(&w) > crate::dialogue::AIM_CHARS { p.push(alloc::format!("{}: \"{}\" is {} chars (> {})", w0, w, js_len(&w), crate::dialogue::AIM_CHARS)); }
            if brace_other_than(&l.text, &["other}", "Other}", "code}", "Code}", "padDoor}", "PadDoor}"]) { p.push(alloc::format!("{}: only {{other}} / {{code}} / {{padDoor}} resolve between two men", w0)); }
            if l.who != 0.0 && l.who != 1.0 { p.push(alloc::format!("{}: who {}", w0, l.who)); }
        }
    }
    // the needs (game/needs.ts): a pool per kind of at least two, the replies, the way back
    {
        let nl = crate::dialogue::needs_lines_table();
        let mut pools: Vec<(String, Vec<String>)> = nl.announce.iter().map(|(k, v)| (alloc::format!("needs announce '{}'", k), v.clone())).collect();
        pools.push((f("needs reply"), nl.reply.clone()));
        pools.push((f("needs replyOther"), nl.reply_other.clone()));
        pools.push((f("needs back"), nl.back.clone()));
        pools.push((f("needs now"), nl.now.values().cloned().collect()));
        for k in ["coffee", "smoke", "water", "stretch"] {
            let n = nl.announce.get(f(k)).map_or(0, |l| l.len());
            if n < 2 { p.push(alloc::format!("needs announce '{}': {} lines (want ≥ 2)", k, n)); }
            if !nl.short.get(f(k)).is_some_and(|x| !x.is_empty()) { p.push(alloc::format!("needs short '{}' missing (the radio line names it)", k)); }
        }
        for (at, list) in &pools {
            for (i, text) in list.iter().enumerate() {
                let w0 = alloc::format!("{} #{}", at, i);
                p.orig(&w0, text); p.dup(&w0, text);
                let w = text.replace("{other}", "Lindqvist");
                if js_len(&w) > crate::dialogue::AIM_CHARS { p.push(alloc::format!("{}: \"{}\" is {} chars (> {})", w0, w, js_len(&w), crate::dialogue::AIM_CHARS)); }
                if brace_other_than(text, &["other}"]) { p.push(alloc::format!("{}: only {{other}} resolves here", w0)); }
            }
        }
        let stretch = nl.short.get(f("stretch")).cloned().unwrap_or_default();
        let r = nl.radio.replacen("{who}", "Lindqvist", 1).replacen("{short}", &stretch, 1);
        if js_len(&r) > crate::dialogue::AIM_CHARS { p.push(alloc::format!("needs radio: \"{}\" is {} chars (> {})", r, js_len(&r), crate::dialogue::AIM_CHARS)); }
    }
    p.problems
}

/// Lines per (K × stage) for the report and the probe, a line counted under the K it serves ('*' lines under 'any').   (dialogue.ts coverageMatrix)
pub fn coverage_matrix() -> crate::dialogue::CoverageMatrix {
    use crate::dialogue::{CoverageMatrix, CoverageMatrixRowsItem};
    let lines = crate::dialogue::lines();
    let stages = crate::dialogue::stages();
    let mut rows: Vec<CoverageMatrixRowsItem> = Vec::new();
    let mut words = crate::dialogue::knowledge();
    words.push(String::from("*"));
    for k in &words {
        let mut counts: JsMap<String, f64> = JsMap::new();
        let mut total = 0.0;
        for s in &stages {
            let n = lines.iter().filter(|l| l.k.as_str() == k && stage_is(l.s, *s)).count() as f64;
            counts.set(stage_word(*s), n);
            total += n;
        }
        rows.push(CoverageMatrixRowsItem { k: if k == "*" { String::from("any (needs-gated)") } else { k.clone() }, counts, total });
    }
    let mut intel = 0.0;
    for (_, v) in crate::dialogue::intel_text_table().iter() {
        for (_, text) in intel_rows!(v) { if text.as_ref().is_some_and(|t| !t.is_empty()) { intel += 1.0; } }
    }
    CoverageMatrix { rows, total: lines.len() as f64, intel: intel + 1.0 }
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

/// `a === b` of two StageKeys.
fn stage_is(a: crate::dialogue::StageKey, b: crate::dialogue::StageKey) -> bool {
    use crate::dialogue::StageKey;
    match (a, b) { (StageKey::Num(x), StageKey::Num(y)) => x == y, (StageKey::X, StageKey::X) => true, _ => false }
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

const KNOWLEDGE_LIST: [crate::dialogue::Knowledge; 5] = [crate::dialogue::Knowledge::None, crate::dialogue::Knowledge::Heard, crate::dialogue::Knowledge::Glimpsed, crate::dialogue::Knowledge::Saw, crate::dialogue::Knowledge::SawHurt];
const BAND_LIST: [crate::dialogue::Band; 3] = [crate::dialogue::Band::Low, crate::dialogue::Band::Mid, crate::dialogue::Band::High];
const TEMPERAMENT_LIST: [crate::game::Temperament; 3] = [crate::game::Temperament::Steady, crate::game::Temperament::Nervy, crate::game::Temperament::Hard];
const INTEL_KIND_LIST: [crate::dialogue::IntelKind; 11] = [
    crate::dialogue::IntelKind::CheckOn, crate::dialogue::IntelKind::Clearing, crate::dialogue::IntelKind::RollcallLive, crate::dialogue::IntelKind::Blackout,
    crate::dialogue::IntelKind::Door, crate::dialogue::IntelKind::Code, crate::dialogue::IntelKind::Rollcall, crate::dialogue::IntelKind::LockdownPosts,
    crate::dialogue::IntelKind::Routes, crate::dialogue::IntelKind::Missing, crate::dialogue::IntelKind::Drive,
];

/// `Object.entries` of an intel table: the TypeScript's key beside the field it became (a struct's order, not the literal's: only the order of the lint's messages feels it).
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

/// `INTEL_TEXT[kind][t]`.
fn intel_own_of(v: &crate::dialogue::INTELTEXTValue, t: crate::game::Temperament) -> Option<&String> {
    use crate::game::Temperament;
    match t { Temperament::Steady => v.steady.as_ref(), Temperament::Nervy => v.nervy.as_ref(), Temperament::Hard => v.hard.as_ref() }
}

/// `SHAPE_LEAD[shape]`.
fn shape_lead_of(l: &crate::dialogue::SHAPELEAD, s: crate::dialogue::ShapeKey) -> Option<&String> {
    use crate::dialogue::ShapeKey;
    match s {
        ShapeKey::Pedant => l.pedant.as_ref(), ShapeKey::Selfown => l.selfown.as_ref(), ShapeKey::Folder => l.folder.as_ref(), ShapeKey::Ramble => l.ramble.as_ref(), ShapeKey::Fan => l.fan.as_ref(),
        ShapeKey::Stoic => l.stoic.as_ref(), ShapeKey::Con => l.con.as_ref(), ShapeKey::Game => l.game.as_ref(), ShapeKey::Menace => l.menace.as_ref(),
    }
}

/// `sc.beats[s]`.
fn beat_of(b: &crate::dialogue::ScriptBeats, s: crate::dialogue::StageKey) -> &crate::dialogue::Beat {
    if is_stage(s, 1.0) { &b._1 } else if is_stage(s, 2.0) { &b._2 } else if is_stage(s, 3.0) { &b._3 } else { &b.x }
}

/// lintDialogue's himStrings: a Him as (key, text) rows.
fn him_strings(h: &crate::dialogue::Him) -> Vec<(String, String)> {
    match h {
        crate::dialogue::Him::Text(t) => vec![(String::from("*"), t.clone())],
        crate::dialogue::Him::Arm1(m) => m.iter().map(|(k, v)| (k, v.clone())).collect(),
    }
}

/// `/\{(?!a\}|b\})/.test(text)`: a brace that opens none of the allowed tokens (each given with its closing brace).
fn brace_other_than(text: &str, allowed: &[&str]) -> bool {
    text.match_indices('{').any(|(i, _)| !allowed.iter().any(|a| text[i + 1..].starts_with(a)))
}

/// lintDialogue's leadWorst: the longest lead any of these men can put before a fact (leadWorstT inside: a shape's lead counts for a man not hard who plays that shape).
fn lead_worst(shape_ts: &JsMap<crate::dialogue::ShapeKey, JsSet<crate::game::Temperament>>, lead: &crate::dialogue::SHAPELEAD, ts: &[crate::game::Temperament], ks: &[crate::dialogue::Knowledge]) -> f64 {
    let mut worst = 0.0;
    for t in ts {
        let lens: Vec<f64> = ks.iter().map(|k| js_len(&intel_lead(*t, *k))).collect();
        let mut one = m::max_n(&lens);
        if *t != crate::game::Temperament::Hard {
            for (sh, who) in shape_ts.iter() {
                if who.has(*t) { if let Some(l) = shape_lead_of(lead, sh) { one = m::max(one, js_len(l)); } }
            }
        }
        worst = m::max(worst, one);
    }
    worst
}

/// lintDialogue's running state and its closures orig, dup, worst, samLen, himLen.
struct Lint {
    problems: Vec<String>,
    ids: JsSet<String>,
    texts: JsSet<String>,
    worst_tokens: JsMap<String, Option<String>>,
}

impl Lint {
    fn push(&mut self, s: String) { self.problems.push(s); }
    /// An id that must be seen once; `twin` is the message if it was seen before.
    fn id(&mut self, id: &str, twin: String) {
        if self.ids.has(String::from(id)) { self.push(twin); }
        self.ids.add(String::from(id));
    }
    fn worst(&self, text: &str) -> String {
        expand(text, &self.worst_tokens).unwrap_or_else(|| String::from(text))
    }
    fn orig(&mut self, at: &str, text: &str) {
        if let Some(b) = blocked_phrase(text) { self.push(alloc::format!("{}: \"{}\" carries the blocklisted '{}' — write it fresh", at, text, b)); }
    }
    fn dup(&mut self, at: &str, text: &str) {
        let key = text.trim().to_lowercase();
        if self.texts.has(key.clone()) { self.push(alloc::format!("duplicate text ({}): \"{}\"", at, text)); }
        self.texts.add(key);
    }
    fn sam_len(&mut self, at: &str, text: Option<&str>) {
        let Some(text) = text else { return };
        self.orig(at, text);
        let w = self.worst(text);
        if js_len(&w) > crate::dialogue::SAM_MAX_CHARS { self.push(alloc::format!("{}: the player's \"{}\" is {} chars (> {})", at, w, js_len(&w), crate::dialogue::SAM_MAX_CHARS)); }
        if text.trim().is_empty() || text.trim() == "…" { self.push(alloc::format!("{}: the player says nothing (\"{}\")", at, text)); }
    }
    fn him_len(&mut self, at: &str, text: &str) {
        self.orig(at, text); self.dup(at, text);
        let w = self.worst(text);
        if js_len(&w) > crate::dialogue::MAX_CHARS { self.push(alloc::format!("{}: \"{}\" is {} chars (> {})", at, w, js_len(&w), crate::dialogue::MAX_CHARS)); }
        if text.trim().is_empty() || text.trim() == "…" { self.push(alloc::format!("{}: he says nothing (\"{}\")", at, text)); }
    }
    /// The reworded rows of one line: the same tokens as the row they stand in for, so only length, the blocklist and a twin can break.
    fn reworded(&mut self, id: &str, own: &str, rows: &[(&str, Option<&str>)], max: f64) {
        let own_tokens = tokens_in(own);
        for (key, text) in rows {
            let Some(text) = text else { continue };
            let at = alloc::format!("{} {}", id, key);
            self.dup(&at, text); self.orig(&at, text);
            let w = self.worst(text);
            if js_len(&w) > max { self.push(alloc::format!("{}: {} chars after expansion (> {}): \"{}\"", at, js_len(&w), max, w)); }
            if tokens_in(text).iter().any(|tk| !own_tokens.contains(tk)) { self.push(alloc::format!("{}: a token its row's own text does not carry", at)); }
        }
    }
}

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

    #[test]
    fn the_lists_written_here_are_the_tables_in_their_order() {
        let w: Vec<&str> = KNOWLEDGE_LIST.iter().map(|x| x.as_str()).collect();
        assert_eq!(w, crate::dialogue::knowledge().iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let w: Vec<&str> = BAND_LIST.iter().map(|x| x.as_str()).collect();
        assert_eq!(w, crate::dialogue::bands().iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let w: Vec<&str> = TEMPERAMENT_LIST.iter().map(|x| x.as_str()).collect();
        assert_eq!(w, crate::dialogue::temperaments().iter().map(|s| s.as_str()).collect::<Vec<_>>());
        let w: Vec<&str> = INTEL_KIND_LIST.iter().map(|x| x.as_str()).collect();
        assert_eq!(w, crate::dialogue::intel_kinds().iter().map(|s| s.as_str()).collect::<Vec<_>>());
    }

    #[test]
    fn a_blocked_phrase_is_found_whatever_its_case() {
        assert_eq!(blocked_phrase("A real NINJA move").as_deref(), Some("ninja"));
        assert_eq!(blocked_phrase("Quiet now. You and I are going to talk."), None);
    }

    #[test]
    fn a_brace_is_only_fine_when_it_opens_an_allowed_token() {
        assert!(!brace_other_than("rack {rack} ok", &["rack}"]));
        assert!(brace_other_than("{other}", &["rack}"]));
        assert!(brace_other_than("{rack} and {", &["rack}"]));
        assert!(!brace_other_than("no braces", &["rack}"]));
    }

    /// against `coverageMatrix()` of the TypeScript under bun. `intel` is short of its 43 by the 'sc:<script>' rows the skeleton's table has no field for (notes: Skeleton).
    #[test]
    fn coverage_counts_as_the_typescript_does() {
        let c = coverage_matrix();
        let want = [("none", [5.0, 4.0, 3.0, 4.0], 16.0), ("heard", [6.0, 4.0, 2.0, 2.0], 14.0), ("glimpsed", [4.0, 3.0, 2.0, 2.0], 11.0), ("saw", [6.0, 4.0, 2.0, 2.0], 14.0), ("sawHurt", [7.0, 5.0, 2.0, 4.0], 18.0), ("any (needs-gated)", [0.0, 9.0, 0.0, 3.0], 12.0)];
        assert_eq!(c.rows.len(), want.len());
        for (row, (k, counts, total)) in c.rows.iter().zip(want) {
            assert_eq!(row.k, k);
            assert_eq!(row.total, total);
            for (s, n) in ["1", "2", "3", "x"].iter().zip(counts) { assert_eq!(row.counts.get(String::from(*s)).copied(), Some(n), "{} {}", k, s); }
        }
        assert_eq!(c.total, 85.0);
        assert_eq!(c.intel, 43.0 - 5.0);   // the five 'sc:' rows of INTEL_TEXT
    }
}
