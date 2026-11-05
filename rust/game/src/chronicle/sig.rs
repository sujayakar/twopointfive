#![allow(unused_variables)]
//! Every function and method of `chronicle`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/chronicle.ts:38 roomsOf)
pub fn rooms_of(g: &mut crate::game::Game) -> Vec<crate::chronicle::RoomsOfItem> { todo!() }

/// (src/game/chronicle.ts:44 roomOf)
pub fn room_of(g: &mut crate::game::Game, p: V3) -> String { todo!() }

/// (src/game/chronicle.ts:56 indoors)
pub fn indoors(p: V3) -> bool { todo!() }

/// (src/game/chronicle.ts:92 isChatter)
pub fn is_chatter(g: &mut crate::game::Game, kind: crate::chronicle::ChronKind, data: &JsMap<String, Option<crate::chronicle::IsChatterDataValue>>) -> bool { todo!() }

/// (src/game/chronicle.ts:116 newCounts)
pub fn new_counts() -> crate::chronicle::ChronCounts { todo!() }

/// (src/game/chronicle.ts:134 Chronicle.add)
pub fn chronicle_add(g: &mut crate::game::Game, t: f64, kind: crate::chronicle::ChronKind, pos: V3, data: &JsMap<String, Option<crate::chronicle::ChronicleAddDataValue>>) -> Option<usize> { todo!() }

/// (src/game/chronicle.ts:141 Chronicle.recent)
pub fn chronicle_recent<'g>(g: &'g mut crate::game::Game, kind: crate::chronicle::ChronKind, key_name: &str, key: &crate::chronicle::ChronicleRecentKey, t: f64, win: f64) -> Option<&'g mut usize> { todo!() }

/// (src/game/chronicle.ts:146 Chronicle.heardRow)
pub fn chronicle_heard_row<'g>(g: &'g mut crate::game::Game, e: u32) -> Option<&'g mut usize> { todo!() }

/// (src/game/chronicle.ts:147 Chronicle.bindHeard)
pub fn chronicle_bind_heard(g: &mut crate::game::Game, e: u32, row: usize) { todo!() }

/// (src/game/chronicle.ts:148 Chronicle.heardCounted)
pub fn chronicle_heard_counted(g: &mut crate::game::Game, e: u32) -> bool { todo!() }

/// (src/game/chronicle.ts:149 Chronicle.markHeardCounted)
pub fn chronicle_mark_heard_counted(g: &mut crate::game::Game, e: u32) { todo!() }

/// (src/game/chronicle.ts:151 Chronicle.sample)
pub fn chronicle_sample(g: &mut crate::game::Game, x: f64, z: f64, t: f64) { todo!() }

/// (src/game/chronicle.ts:162 nowOf)
pub fn now_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/chronicle.ts:163 posOf)
pub fn pos_of(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/chronicle.ts:165 chronicle)
pub fn chronicle(g: &mut crate::game::Game, kind: crate::chronicle::ChronKind, data: Option<&JsMap<String, Option<crate::chronicle::ChronicleDataValue>>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:171 chronicleN)
pub fn chronicle_n(g: &mut crate::game::Game, kind: crate::chronicle::ChronKind, count: crate::chronicle::ChronCountKey, data: Option<&JsMap<String, Option<crate::chronicle::ChronicleNDataValue>>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:178 tickChronicle)
pub fn tick_chronicle(g: &mut crate::game::Game) { todo!() }

/// (src/game/chronicle.ts:184 chronObjective)
pub fn chron_objective(g: &mut crate::game::Game, text: &str, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:188 chronSpotted)
pub fn chron_spotted(g: &mut crate::game::Game, by: Option<GuardId>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:194 chronContact)
pub fn chron_contact(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:200 chronHeard)
pub fn chron_heard(g: &mut crate::game::Game, gd: GuardId, ev: crate::game::GameEvent, where_: Option<V3>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:222 heardWhat)
pub fn heard_what(e: crate::game::GameEvent) -> String { todo!() }

/// (src/game/chronicle.ts:238 chronLost)
pub fn chron_lost(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:240 chronCalm)
pub fn chron_calm(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:244 chronAlarm)
pub fn chron_alarm(g: &mut crate::game::Game, stage: f64, up: bool, cause: Option<&str>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:250 chronRadio)
pub fn chron_radio(g: &mut crate::game::Game, gd: GuardId, text: &str) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:257 chronShot)
pub fn chron_shot(g: &mut crate::game::Game, weapon: &str, hit_guard: Option<GuardId>, hit_p: V3) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:270 chronLightShot)
pub fn chron_light_shot(g: &mut crate::game::Game, tg: FixtureId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:272 chronOcp)
pub fn chron_ocp(g: &mut crate::game::Game, what: &str, pos: V3) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:274 chronBlackout)
pub fn chron_blackout(g: &mut crate::game::Game, cause: &str) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:276 fixtureName)
pub fn fixture_name(tg: &crate::chronicle::FixtureNameTg) -> String { todo!() }

/// (src/game/chronicle.ts:286 chronThrown)
pub fn chron_thrown(g: &mut crate::game::Game, kind: &str) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:288 chronGrab)
pub fn chron_grab(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:290 chronInterrogated)
pub fn chron_interrogated(g: &mut crate::game::Game, gd: GuardId, kind: Option<&str>, stage: f64) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:296 chronKey)
pub fn chron_key(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:300 chronDown)
pub fn chron_down(g: &mut crate::game::Game, gd: GuardId, how: crate::game::DownKind, line: Option<&str>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:306 chronBodyFound)
pub fn chron_body_found(g: &mut crate::game::Game, body: GuardId, by: GuardId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:311 chronHit)
pub fn chron_hit(g: &mut crate::game::Game, by: Option<&str>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:317 chronDoor)
pub fn chron_door(g: &mut crate::game::Game, kind: crate::chronicle::ChronDoorKind, d: DoorId) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:320 chronCheckpoint)
pub fn chron_checkpoint(g: &mut crate::game::Game, what: crate::chronicle::ChronCheckpointWhat, label: Option<&str>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/chronicle.ts:335 runCounts)
pub fn run_counts(g: &mut crate::game::Game) -> crate::chronicle::RunCounts { todo!() }

/// (src/game/chronicle.ts:345 blankCounts)
pub fn blank_counts() -> crate::chronicle::RunCounts { todo!() }

/// (src/game/chronicle.ts:366 stealthScore)
pub fn stealth_score(g: &mut crate::game::Game, c_2: &crate::chronicle::RunCounts) -> crate::chronicle::StealthScore { todo!() }

/// (src/game/chronicle.ts:389 times)
pub fn times(n: f64) -> String { todo!() }

/// (src/game/chronicle.ts:398 ratingOf)
pub fn rating_of(score: f64) -> crate::chronicle::RatingOf { todo!() }

/// (src/game/chronicle.ts:407 badgesOf)
pub fn badges_of(g: &mut crate::game::Game, c_2: &crate::chronicle::RunCounts) -> Vec<crate::chronicle::Badge> { todo!() }

/// (src/game/chronicle.ts:422 fmtMMSS)
pub fn fmt_mmss(sec: f64) -> String { todo!() }

/// (src/game/chronicle.ts:429 debriefOf)
pub fn debrief_of(g: &mut crate::game::Game) -> crate::chronicle::Debrief { todo!() }

/// (src/game/chronicle.ts:436 summaryLine)
pub fn summary_line(c_2: &crate::chronicle::RunCounts, score: f64, word: &str, badges: &[crate::chronicle::Badge], state: crate::chronicle::SummaryLineState, scored: Option<bool>) -> String { todo!() }

/// (src/game/chronicle.ts:446 fmtEvent)
pub fn fmt_event(g: &mut crate::game::Game, e: &crate::chronicle::ChronEvent) -> String { todo!() }

/// (src/game/chronicle.ts:459 doorWritten)
pub fn door_written(g: &mut crate::game::Game, door: &str) -> Option<String> { todo!() }

/// (src/game/chronicle.ts:461 eventText)
pub fn event_text(g: &mut crate::game::Game, e: &crate::chronicle::ChronEvent) -> String { todo!() }

/// (src/game/chronicle.ts:528 pinned)
pub fn pinned(e: &crate::chronicle::ChronEvent) -> bool { todo!() }
