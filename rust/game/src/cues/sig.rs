#![allow(unused_variables)]
//! Every function and method of `cues`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/cues.ts:83 saveCues)
pub fn save_cues(g: &mut crate::game::Game) { todo!() }

/// (src/game/cues.ts:96 cueClass)
pub fn cue_class(text: &str) -> f64 { todo!() }

/// (src/game/cues.ts:124 said)
pub fn said(g: &mut crate::game::Game, k: crate::cues::CueKind) -> Vec<String> { todo!() }

/// (src/game/cues.ts:128 cueLines)
pub fn cue_lines(g: &mut crate::game::Game) -> Vec<crate::cues::CueLinesItem> { todo!() }

/// (src/game/cues.ts:135 isShout)
pub fn is_shout(text: &str) -> bool { todo!() }

/// (src/game/cues.ts:138 newStats)
pub fn new_stats() -> crate::cues::CuesStats { todo!() }

/// (src/game/cues.ts:141 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/cues.ts:158 Cues.constructor)
pub fn cues_new(g: &mut crate::game::Game) -> crate::cues::Cues { todo!() }

/// (src/game/cues.ts:161 Cues.reset)
pub fn cues_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/cues.ts:164 Cues.alive)
pub fn cues_alive(g: &mut crate::game::Game) -> Vec<crate::cues::CueRow> { todo!() }

/// (src/game/cues.ts:166 Cues.file)
pub fn cues_file(g: &mut crate::game::Game, text: &str, cls: f64, who: &str, kind: crate::cues::CuesFileKind) -> bool { todo!() }

/// (src/game/cues.ts:195 Cues.raw)
pub fn cues_raw(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/cues.ts:197 Cues.heard)
pub fn cues_heard(g: &mut crate::game::Game, gd: GuardId, text: &str) { todo!() }

/// (src/game/cues.ts:204 Cues.net)
pub fn cues_net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: f64) { todo!() }

/// (src/game/cues.ts:213 Cues.call)
pub fn cues_call(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool { todo!() }

/// (src/game/cues.ts:218 Cues.ears)
pub fn cues_ears(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/cues.ts:230 Cues.overheard)
pub fn cues_overheard(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool { todo!() }

/// (src/game/cues.ts:247 Cues.aside)
pub fn cues_aside(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool { todo!() }

/// (src/game/cues.ts:249 Cues.netWait)
pub fn cues_net_wait(g: &mut crate::game::Game, cls: f64) -> f64 { todo!() }

/// (src/game/cues.ts:251 Cues.adopt)
pub fn cues_adopt(g: &mut crate::game::Game, msgs: &[crate::cues::CuesAdoptMsgsItem]) { todo!() }

/// (src/game/cues.ts:253 Cues.bubbleOf)
pub fn cues_bubble_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::cues::Bubble> { todo!() }

/// (src/game/cues.ts:261 Cues.at)
pub fn cues_at(g: &mut crate::game::Game, p: V3) -> crate::cues::CuesAt { todo!() }

/// (src/game/cues.ts:262 Cues.man)
pub fn cues_man<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::cues::Man { todo!() }

/// (src/game/cues.ts:264 Cues.cue)
pub fn cues_cue(g: &mut crate::game::Game, gd: GuardId, kind: crate::cues::CueKind, vars: Option<&JsMap<String, String>>) -> bool { todo!() }

/// (src/game/cues.ts:279 Cues.kindBusy)
pub fn cues_kind_busy(g: &mut crate::game::Game, kind: crate::cues::CueKind) -> bool { todo!() }

/// (src/game/cues.ts:280 Cues.fit)
pub fn cues_fit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/cues.ts:281 Cues.squadVoice)
pub fn cues_squad_voice(g: &mut crate::game::Game, name: &str, by: &str) -> Option<GuardId> { todo!() }

/// (src/game/cues.ts:287 Cues.tick)
pub fn cues_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/cues.ts:316 Cues.note)
pub fn cues_note(g: &mut crate::game::Game, who: &str, what: &str) { todo!() }

/// (src/game/cues.ts:346 Cues.summary)
pub fn cues_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/cues.ts:354 stripRows)
pub fn strip_rows(g: &mut crate::game::Game) -> Vec<crate::cues::StripRowsItem> { todo!() }
