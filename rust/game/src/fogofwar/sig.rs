#![allow(unused_variables)]
//! Every function and method of `fogofwar`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/fogofwar.ts:303 fieldDeg)
pub fn field_deg(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/fogofwar.ts:304 fieldNear)
pub fn field_near(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/fogofwar.ts:309 gazeOf)
pub fn gaze_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/fogofwar.ts:338 fogEyeCount)
pub fn fog_eye_count(v: f64) -> f64 { todo!() }

/// (src/game/fogofwar.ts:341 fogFlags)
pub fn fog_flags(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:363 thingKeyOf)
pub fn thing_key_of(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/fogofwar.ts:381 isProp)
pub fn is_prop(thing_key: f64) -> bool { todo!() }

/// (src/game/fogofwar.ts:383 ghostBox)
pub fn ghost_box(g: &mut crate::game::Game, s: BoxId) -> BoxId { todo!() }

/// (src/game/fogofwar.ts:390 fogThrough)
pub fn fog_through(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/fogofwar.ts:443 FogOfWar.show)
pub fn fog_of_war_show(g: &mut crate::game::Game, gd: GuardId, on: bool) { todo!() }

/// (src/game/fogofwar.ts:471 FogOfWar.pageLook)
pub fn fog_of_war_page_look(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:474 FogOfWar.constructor)
pub fn fog_of_war_new(g: &mut crate::game::Game) -> crate::fogofwar::FogOfWar { todo!() }

/// (src/game/fogofwar.ts:477 FogOfWar.active)
pub fn fog_of_war_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/fogofwar.ts:479 FogOfWar.onTeam)
pub fn fog_of_war_on_team(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/fogofwar.ts:483 FogOfWar.update)
pub fn fog_of_war_update(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/fogofwar.ts:484 FogOfWar.pass)
pub fn fog_of_war_pass(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/fogofwar.ts:615 FogOfWar.eyeOf)
pub fn fog_of_war_eye_of(g: &mut crate::game::Game, c_2: CharId) -> crate::fogofwar::FogEye { todo!() }

/// (src/game/fogofwar.ts:629 FogOfWar.caller)
pub fn fog_of_war_caller(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/fogofwar.ts:630 FogOfWar.sideOf)
pub fn fog_of_war_side_of(g: &mut crate::game::Game, p: V3) -> crate::close::CloseId { todo!() }

/// (src/game/fogofwar.ts:636 FogOfWar.callWords)
pub fn fog_of_war_call_words(g: &mut crate::game::Game, kind: crate::fogofwar::FogOfWarCallWordsKind, p: V3) -> String { todo!() }

/// (src/game/fogofwar.ts:643 FogOfWar.tellCalls)
pub fn fog_of_war_tell_calls(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/fogofwar.ts:669 FogOfWar.inField)
pub fn fog_of_war_in_field(g: &mut crate::game::Game, e: crate::fogofwar::FogEye, p: V3) -> bool { todo!() }

/// (src/game/fogofwar.ts:677 FogOfWar.flushAsks)
pub fn fog_of_war_flush_asks(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:683 FogOfWar.reset)
pub fn fog_of_war_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:688 FogOfWar.sightBlocked)
pub fn fog_of_war_sight_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/fogofwar.ts:692 FogOfWar.thinBlock)
pub fn fog_of_war_thin_block(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:702 FogOfWar.lineToMan)
pub fn fog_of_war_line_to_man(g: &mut crate::game::Game, e: crate::fogofwar::FogEye, gd: GuardId, chest: V3) -> bool { todo!() }

/// (src/game/fogofwar.ts:716 FogOfWar.sees)
pub fn fog_of_war_sees(g: &mut crate::game::Game, eyes: &[crate::fogofwar::FogEye], order: f64, want: f64, gd: GuardId, chest: Option<V3>) -> f64 { todo!() }

/// (src/game/fogofwar.ts:724 FogOfWar.handEyes)
pub fn fog_of_war_hand_eyes(g: &mut crate::game::Game, eyes: &[crate::fogofwar::FogEye]) { todo!() }

/// (src/game/fogofwar.ts:729 FogOfWar.eyeOnBody)
pub fn fog_of_war_eye_on_body(g: &mut crate::game::Game, e: crate::fogofwar::FogEye, gd: GuardId) -> bool { todo!() }

/// (src/game/fogofwar.ts:735 FogOfWar.eyeOn)
pub fn fog_of_war_eye_on(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/fogofwar.ts:741 FogOfWar.doorSeen)
pub fn fog_of_war_door_seen(g: &mut crate::game::Game, d: DoorId, shown: f64) -> bool { todo!() }

/// (src/game/fogofwar.ts:763 FogOfWar.doorHeld)
pub fn fog_of_war_door_held(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/fogofwar.ts:765 FogOfWar.doorShown)
pub fn fog_of_war_door_shown(g: &mut crate::game::Game, d: DoorId) -> f64 { todo!() }

/// (src/game/fogofwar.ts:767 FogOfWar.doorOfBox)
pub fn fog_of_war_door_of_box(g: &mut crate::game::Game, box_: BoxId) -> Option<DoorId> { todo!() }

/// (src/game/fogofwar.ts:770 FogOfWar.doorBoxes)
pub fn fog_of_war_door_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/fogofwar.ts:791 FogOfWar.lampSeen)
pub fn fog_of_war_lamp_seen(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/fogofwar.ts:808 FogOfWar.fullClear)
pub fn fog_of_war_full_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:817 FogOfWar.lampFace)
pub fn fog_of_war_lamp_face(g: &mut crate::game::Game, t: FixtureId, lit: Option<bool>) { todo!() }

/// (src/game/fogofwar.ts:826 FogOfWar.faceSeen)
pub fn fog_of_war_face_seen(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/fogofwar.ts:834 FogOfWar.facesLift)
pub fn fog_of_war_faces_lift(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:835 FogOfWar.facesBack)
pub fn fog_of_war_faces_back(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:845 FogOfWar.snapshot)
pub fn fog_of_war_snapshot(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogSnap> { todo!() }

/// (src/game/fogofwar.ts:866 FogOfWar.restore)
pub fn fog_of_war_restore(g: &mut crate::game::Game, s: Option<&crate::fogofwar::FogSnap>) { todo!() }

/// (src/game/fogofwar.ts:890 FogOfWar.lampLit)
pub fn fog_of_war_lamp_lit(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/fogofwar.ts:893 FogOfWar.lampKnown)
pub fn fog_of_war_lamp_known(g: &mut crate::game::Game, t: FixtureId, lit: bool) -> bool { todo!() }

/// (src/game/fogofwar.ts:902 FogOfWar.fullBlock)
pub fn fog_of_war_full_block(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/fogofwar.ts:925 FogOfWar.lampTake)
pub fn fog_of_war_lamp_take(g: &mut crate::game::Game, l: LightId, i: f64, far: bool) { todo!() }

/// (src/game/fogofwar.ts:933 FogOfWar.litBlock)
pub fn fog_of_war_lit_block(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:959 FogOfWar.spotSeen)
pub fn fog_of_war_spot_seen(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/fogofwar.ts:963 FogOfWar.groundLook)
pub fn fog_of_war_ground_look(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/game/fogofwar.ts:977 FogOfWar.cellSeenAt)
pub fn fog_of_war_cell_seen_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/fogofwar.ts:983 FogOfWar.thingSeen)
pub fn fog_of_war_thing_seen(g: &mut crate::game::Game, x: f64, z: f64, ox: f64, oz: f64) -> bool { todo!() }

/// (src/game/fogofwar.ts:987 FogOfWar.span)
pub fn fog_of_war_span(g: &mut crate::game::Game, list: &[BoxId], i: f64, j: f64) { todo!() }

/// (src/game/fogofwar.ts:993 FogOfWar.seesThing)
pub fn fog_of_war_sees_thing(g: &mut crate::game::Game, boxes: &[BoxId]) -> bool { todo!() }

/// (src/game/fogofwar.ts:997 FogOfWar.things)
pub fn fog_of_war_things(g: &mut crate::game::Game, list: &mut Vec<BoxId>) -> Vec<BoxId> { todo!() }

/// (src/game/fogofwar.ts:1065 FogOfWar.beamRuns)
pub fn fog_of_war_beam_runs(g: &mut crate::game::Game, from: V3, dir: V3, l: f64) -> Vec<[f64; 2]> { todo!() }

/// (src/game/fogofwar.ts:1083 FogOfWar.seen)
pub fn fog_of_war_seen(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/fogofwar.ts:1087 FogOfWar.sighting)
pub fn fog_of_war_sighting(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::fogofwar::FogOfWarSighting> { todo!() }

/// (src/game/fogofwar.ts:1093 FogOfWar.drawn)
pub fn fog_of_war_drawn(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/fogofwar.ts:1100 FogOfWar.firedOn)
pub fn fog_of_war_fired_on(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/fogofwar.ts:1103 FogOfWar.lastSeen)
pub fn fog_of_war_last_seen(g: &mut crate::game::Game) -> Vec<crate::fogofwar::FogOfWarLastSeenItem> { todo!() }

/// (src/game/fogofwar.ts:1114 FogOfWar.hidden)
pub fn fog_of_war_hidden(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/fogofwar.ts:1118 FogOfWar.oursOn)
pub fn fog_of_war_ours_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/fogofwar.ts:1122 FogOfWar.oursId)
pub fn fog_of_war_ours_id(g: &mut crate::game::Game, who: Option<f64>) -> bool { todo!() }

/// (src/game/fogofwar.ts:1125 FogOfWar.lightBy)
pub fn fog_of_war_light_by(g: &mut crate::game::Game, l: LightId, who: Option<f64>) -> LightId { todo!() }

/// (src/game/fogofwar.ts:1129 FogOfWar.begin)
pub fn fog_of_war_begin(g: &mut crate::game::Game, who: Option<f64>) -> f64 { todo!() }

/// (src/game/fogofwar.ts:1130 FogOfWar.done)
pub fn fog_of_war_done(g: &mut crate::game::Game, was: f64) { todo!() }

/// (src/game/fogofwar.ts:1132 FogOfWar.doing)
pub fn fog_of_war_doing<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, who: Option<f64>, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/fogofwar.ts:1135 FogOfWar.hush)
pub fn fog_of_war_hush(g: &mut crate::game::Game) { todo!() }

/// (src/game/fogofwar.ts:1140 FogOfWar.airHides)
pub fn fog_of_war_air_hides(g: &mut crate::game::Game, c_2: CharId) -> bool { todo!() }

/// (src/game/fogofwar.ts:1142 FogOfWar.filter)
pub fn fog_of_war_filter(g: &mut crate::game::Game, all: &[CharId]) -> Vec<CharId> { todo!() }

/// (src/game/fogofwar.ts:1144 FogOfWar.everybody)
pub fn fog_of_war_everybody(g: &mut crate::game::Game, all: &[CharId]) -> Vec<CharId> { todo!() }

/// (src/game/fogofwar.ts:1146 FogOfWar.raw)
pub fn fog_of_war_raw(g: &mut crate::game::Game) -> Option<Vec<BoxId>> { todo!() }

/// (src/game/fogofwar.ts:1149 FogOfWar.summary)
pub fn fog_of_war_summary(g: &mut crate::game::Game) -> String { todo!() }
