#![allow(unused_variables)]
//! Every function and method of `dialogue`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/dialogue.ts:63 bandOf)
pub fn band_of(a: f64) -> crate::dialogue::Band { todo!() }

/// (src/game/dialogue.ts:88 knowledgeOf)
pub fn knowledge_of(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::Knowledge { todo!() }

/// (src/game/dialogue.ts:120 agitationOf)
pub fn agitation_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/dialogue.ts:121 agitationTerms)
pub fn agitation_terms(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::AgitationTerms { todo!() }

/// (src/game/dialogue.ts:144 temperamentOf)
pub fn temperament_of(g: &mut crate::game::Game, gd: GuardId) -> crate::game::Temperament { todo!() }

/// (src/game/dialogue.ts:146 givesAt)
pub fn gives_at(t: crate::game::Temperament, band: crate::dialogue::Band) -> Vec<crate::dialogue::StageKey> { todo!() }

/// (src/game/dialogue.ts:188 tokenWorst)
pub fn token_worst(g: &mut crate::game::Game) -> JsMap<crate::dialogue::Token, String> { todo!() }

/// (src/game/dialogue.ts:208 saidText)
pub fn said_text(g: &mut crate::game::Game, l: &crate::dialogue::SaidTextL, ctx: &crate::dialogue::DialogueCtx) -> String { todo!() }

/// (src/game/dialogue.ts:347 intelTable)
pub fn intel_table(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind) -> crate::dialogue::IntelTable { todo!() }

/// (src/game/dialogue.ts:352 driveAway)
pub fn drive_away(g: &mut crate::game::Game) -> crate::dialogue::DriveAway { todo!() }

/// (src/game/dialogue.ts:354 intelText)
pub fn intel_text(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind, t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> String { todo!() }

/// (src/game/dialogue.ts:356 shapedIntelText)
pub fn shaped_intel_text(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind, sc: &crate::dialogue::ShapedIntelTextSc, t: crate::game::Temperament) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:359 checkOnSelf)
pub fn check_on_self(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/dialogue.ts:362 intelLead)
pub fn intel_lead(t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> String { todo!() }

/// (src/game/dialogue.ts:366 runOn)
pub fn run_on(g: &mut crate::game::Game, lead: &str, fact: &str) -> String { todo!() }

/// (src/game/dialogue.ts:368 badgeOf)
pub fn badge_of(callsign: &str) -> String { todo!() }

/// (src/game/dialogue.ts:381 placeName)
pub fn place_name(g: &mut crate::game::Game, p: V3) -> String { todo!() }

/// (src/game/dialogue.ts:385 isOffice)
pub fn is_office(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/dialogue.ts:386 withThe)
pub fn with_the(n: &str) -> String { todo!() }

/// (src/game/dialogue.ts:391 levelRects)
pub fn level_rects(g: &mut crate::game::Game) -> Option<Vec<crate::dialogue::PlaceRect>> { todo!() }

/// (src/game/dialogue.ts:407 rectDist)
pub fn rect_dist(r: &crate::dialogue::Rect, p: V3) -> f64 { todo!() }

/// (src/game/dialogue.ts:412 ownPlace)
pub fn own_place(g: &mut crate::game::Game, p: V3) -> Option<crate::dialogue::OwnPlace> { todo!() }

/// (src/game/dialogue.ts:421 watchedPlace)
pub fn watched_place(g: &mut crate::game::Game, from: V3, p: V3) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:426 ownName)
pub fn own_name(g: &mut crate::game::Game, p: V3) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:429 placeIn)
pub fn place_in(g: &mut crate::game::Game, p: V3) -> Option<crate::dialogue::PlaceRect> { todo!() }

/// (src/game/dialogue.ts:440 wayName)
pub fn way_name(g: &mut crate::game::Game, a: V3, b: V3, into: Option<&str>) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:454 placeRects)
pub fn place_rects(g: &mut crate::game::Game) -> Vec<crate::dialogue::PlaceRect> { todo!() }

/// (src/game/dialogue.ts:466 roomName)
pub fn room_name(g: &mut crate::game::Game, p: V3) -> String { todo!() }

/// (src/game/dialogue.ts:489 routeName)
pub fn route_name(g: &mut crate::game::Game, route_i: f64) -> String { todo!() }

/// (src/game/dialogue.ts:493 doorName)
pub fn door_name(g: &mut crate::game::Game, n: &str) -> String { todo!() }

/// (src/game/dialogue.ts:495 numWord)
pub fn num_word(n: f64) -> String { todo!() }

/// (src/game/dialogue.ts:498 distToRoute)
pub fn dist_to_route(pts: &[V3], p: V3) -> f64 { todo!() }

/// (src/game/dialogue.ts:508 keyedDoorOn)
pub fn keyed_door_on(g: &mut crate::game::Game, gd: GuardId) -> Option<DoorId> { todo!() }

/// (src/game/dialogue.ts:515 keyedDoorOnRoute)
pub fn keyed_door_on_route(g: &mut crate::game::Game, gd: GuardId) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:517 etaWords)
pub fn eta_words(secs: f64) -> String { todo!() }

/// (src/game/dialogue.ts:536 heardOnNet)
pub fn heard_on_net(held_since: f64, t: f64) -> bool { todo!() }

/// (src/game/dialogue.ts:538 missingKnownTo)
pub fn missing_known_to(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/dialogue.ts:540 knownDown)
pub fn known_down(g: &mut crate::game::Game, gd: GuardId) -> JsSet<String> { todo!() }

/// (src/game/dialogue.ts:547 staffCount)
pub fn staff_count(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/dialogue.ts:548 partnerOf)
pub fn partner_of(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/dialogue.ts:560 heldSinceOf)
pub fn held_since_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/dialogue.ts:566 gather)
pub fn gather(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::DialogueCtx { todo!() }

/// (src/game/dialogue.ts:650 tokensIn)
pub fn tokens_in(text: &str) -> Vec<String> { todo!() }

/// (src/game/dialogue.ts:652 expand)
pub fn expand(text: &str, tokens: &JsMap<String, Option<String>>) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:657 stageKeyOf)
pub fn stage_key_of(n: f64) -> crate::dialogue::StageKey { todo!() }

/// (src/game/dialogue.ts:660 givingStage)
pub fn giving_stage(t: crate::game::Temperament, band: crate::dialogue::Band, s: crate::dialogue::StageKey, stage_n: f64, given_none: bool) -> bool { todo!() }

/// (src/game/dialogue.ts:664 lineMatches)
pub fn line_matches(l: &crate::dialogue::Line, k: crate::dialogue::Knowledge, band: crate::dialogue::Band, t: crate::game::Temperament, s: crate::dialogue::StageKey, use_t: bool, use_a: bool) -> bool { todo!() }

/// (src/game/dialogue.ts:671 needsHold)
pub fn needs_hold(g: &mut crate::game::Game, l: &crate::dialogue::Line, facts: crate::dialogue::NeedsHoldFacts) -> bool { todo!() }

/// (src/game/dialogue.ts:675 pendingIntel)
pub fn pending_intel(g: &mut crate::game::Game, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId, sc: Option<&crate::dialogue::Script>) -> Vec<crate::dialogue::PendingIntelItem> { todo!() }

/// (src/game/dialogue.ts:689 intelFor)
pub fn intel_for(g: &mut crate::game::Game, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId, sc: Option<&crate::dialogue::Script>) -> Option<crate::dialogue::IntelFor> { todo!() }

/// (src/game/dialogue.ts:692 swallowed)
pub fn swallowed(g: &mut crate::game::Game, sc: Option<&crate::dialogue::Script>, s: crate::dialogue::StageKey, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId) -> bool { todo!() }

/// (src/game/dialogue.ts:701 pickLine)
pub fn pick_line(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId, rnd: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> crate::dialogue::DialoguePick { todo!() }

/// (src/game/dialogue.ts:764 interrogate)
pub fn interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> { todo!() }

/// (src/game/dialogue.ts:778 promptFor)
pub fn prompt_for(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::PromptFor { todo!() }

/// (src/game/dialogue.ts:785 dialogueSummary)
pub fn dialogue_summary(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/dialogue.ts:930 samMemoryOf)
pub fn sam_memory_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::SamMemory { todo!() }

/// (src/game/dialogue.ts:943 peekGive)
pub fn peek_give(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId) -> Option<crate::dialogue::IntelKind> { todo!() }

/// (src/game/dialogue.ts:949 samStageOf)
pub fn sam_stage_of(s: crate::dialogue::StageKey, gives: bool) -> crate::dialogue::SamStage { todo!() }

/// (src/game/dialogue.ts:951 samFacts)
pub fn sam_facts(g: &mut crate::game::Game, ctx: &crate::dialogue::DialogueCtx, variant: crate::character::HoldVariant) -> JsMap<crate::dialogue::SamNeed, bool> { todo!() }

/// (src/game/dialogue.ts:956 samLineFits)
pub fn sam_line_fits(l: &crate::dialogue::SamLine, ctx: &crate::dialogue::DialogueCtx, facts: &JsMap<crate::dialogue::SamNeed, bool>) -> bool { todo!() }

/// (src/game/dialogue.ts:965 chooseSamLine)
pub fn choose_sam_line(g: &mut crate::game::Game, tiers: &[Vec<crate::dialogue::SamLine>], ctx: &crate::dialogue::DialogueCtx, facts: &JsMap<crate::dialogue::SamNeed, bool>, mem: Option<&mut crate::dialogue::SamMemory>, rnd: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> Option<crate::dialogue::ChooseSamLine> { todo!() }

/// (src/game/dialogue.ts:989 pickSamLine)
pub fn pick_sam_line(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId, variant: crate::character::HoldVariant, mem: Option<&mut crate::dialogue::SamMemory>, rnd: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> crate::dialogue::SamPick { todo!() }

/// (src/game/dialogue.ts:1003 samLineFor)
pub fn sam_line_for(g: &mut crate::game::Game, gd: GuardId, variant: crate::character::HoldVariant, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> crate::dialogue::SamPick { todo!() }

/// (src/game/dialogue.ts:1007 samKeyLineFor)
pub fn sam_key_line_for(g: &mut crate::game::Game, gd: GuardId, variant: crate::character::HoldVariant, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::SamKeyLineFor> { todo!() }

/// (src/game/dialogue.ts:1016 samRemarkFor)
pub fn sam_remark_for(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::SamRemarkFor> { todo!() }

/// (src/game/dialogue.ts:1025 synthCtx)
pub fn synth_ctx(g: &mut crate::game::Game, k: crate::dialogue::Knowledge, band: crate::dialogue::Band, t: crate::game::Temperament, over: Option<&crate::dialogue::SynthCtxOver>) -> crate::dialogue::DialogueCtx { todo!() }

/// (src/game/dialogue.ts:1046 blockedPhrase)
pub fn blocked_phrase(text: &str) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:1062 lintDialogue)
pub fn lint_dialogue(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/dialogue.ts:1214 coverageMatrix)
pub fn coverage_matrix() -> crate::dialogue::CoverageMatrix { todo!() }

/// (src/game/dialogue.ts:1220 samCoverage)
pub fn sam_coverage(g: &mut crate::game::Game) -> crate::dialogue::SamCoverage { todo!() }

/// (src/game/dialogue.ts:1226 scriptCoverage)
pub fn script_coverage() -> Vec<crate::dialogue::ScriptCoverageItem> { todo!() }

/// (src/game/dialogue.ts:1233 lineLengthReport)
pub fn line_length_report(g: &mut crate::game::Game) -> crate::dialogue::LineLengthReport { todo!() }

/// (src/game/dialogue.ts:1279 forceScript)
pub fn force_script(g: &mut crate::game::Game, talk: GameTalkId, id_2: Option<&str>) { todo!() }

/// (src/game/dialogue.ts:1281 forcedScriptOf)
pub fn forced_script_of(g: &mut crate::game::Game, talk: GameTalkId) -> Option<String> { todo!() }

/// (src/game/dialogue.ts:1283 peekScript)
pub fn peek_script(g: &mut crate::game::Game, talk: GameTalkId) -> Option<crate::dialogue::Script> { todo!() }

/// (src/game/dialogue.ts:1284 scriptById)
pub fn script_by_id(id_2: &str) -> Option<crate::dialogue::Script> { todo!() }

/// (src/game/dialogue.ts:1288 scriptFor)
pub fn script_for(g: &mut crate::game::Game, ctx: &mut crate::dialogue::DialogueCtx, talk: GameTalkId) -> Option<crate::dialogue::Script> { todo!() }

/// (src/game/dialogue.ts:1306 scriptFits)
pub fn script_fits(g: &mut crate::game::Game, sc: &crate::dialogue::Script, ctx: &crate::dialogue::DialogueCtx, talk: GameTalkId) -> bool { todo!() }

/// (src/game/dialogue.ts:1315 scriptLive)
pub fn script_live(sc: &crate::dialogue::Script, ctx: &crate::dialogue::DialogueCtx) -> bool { todo!() }

/// (src/game/dialogue.ts:1317 beatFor)
pub fn beat_for(sc: &crate::dialogue::Script, s: crate::dialogue::StageKey, ctx: &crate::dialogue::DialogueCtx) -> crate::dialogue::Beat { todo!() }

/// (src/game/dialogue.ts:1324 himOf)
pub fn him_of(b: &crate::dialogue::Beat, ctx: &crate::dialogue::DialogueCtx) -> Option<crate::dialogue::HimOf> { todo!() }

/// (src/game/dialogue.ts:1650 registerScriptLines)
pub fn register_script_lines(g: &mut crate::game::Game) { todo!() }

/// (src/game/dialogue.ts:1715 radioState)
pub fn radio_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::RadioState { todo!() }

/// (src/game/dialogue.ts:1722 radioLive)
pub fn radio_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/dialogue.ts:1724 radioClear)
pub fn radio_clear(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/dialogue.ts:1728 handlerBeat)
pub fn handler_beat(g: &mut crate::game::Game, beat: crate::dialogue::HandlerBeat, delay: Option<f64>) -> bool { todo!() }

/// (src/game/dialogue.ts:1744 tickRadio)
pub fn tick_radio(g: &mut crate::game::Game) { todo!() }

/// (src/game/dialogue.ts:1766 pairWorst)
pub fn pair_worst(text: &str) -> String { todo!() }

/// (src/game/dialogue.ts:1810 needsLines)
pub fn needs_lines(g: &mut crate::game::Game, callsigns: Option<&[String]>) -> Vec<String> { todo!() }

/// (src/game/dialogue.ts:1947 civilianLines)
pub fn civilian_lines(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/dialogue.ts:1978 hostageLines)
pub fn hostage_lines() -> Vec<String> { todo!() }

/// (src/game/dialogue.ts:1998 snatchLines)
pub fn snatch_lines() -> Vec<String> { todo!() }

/// (src/game/dialogue.ts:2002 ambientState)
pub fn ambient_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::AmbientState { todo!() }

/// (src/game/dialogue.ts:2003 ambientLive)
pub fn ambient_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/dialogue.ts:2005 chatty)
pub fn chatty(g: &mut crate::game::Game, x: GuardId) -> bool { todo!() }

/// (src/game/dialogue.ts:2007 stillChatty)
pub fn still_chatty(g: &mut crate::game::Game, x: GuardId) -> bool { todo!() }

/// (src/game/dialogue.ts:2008 endAmbient)
pub fn end_ambient(g: &mut crate::game::Game) { todo!() }

/// (src/game/dialogue.ts:2014 ambientRestart)
pub fn ambient_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/dialogue.ts:2016 pairSaid)
pub fn pair_said(l: &crate::dialogue::PairTalkLinesItem, own: bool) -> String { todo!() }

/// (src/game/dialogue.ts:2019 updateAmbient)
pub fn update_ambient(g: &mut crate::game::Game, dt: f64) { todo!() }
