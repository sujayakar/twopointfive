#![allow(unused_variables)]
//! Every function and method of `netquiet`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/netquiet.ts:112 netQuietLines)
pub fn net_quiet_lines(g: &mut crate::game::Game, kind: Option<crate::netquiet::LineKind>) -> Vec<String> { todo!() }

/// (src/game/netquiet.ts:128 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/netquiet.ts:129 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/netquiet.ts:131 fed)
pub fn fed(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/netquiet.ts:132 loudOf)
pub fn loud_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/netquiet.ts:134 side)
pub fn side(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:135 ours)
pub fn ours(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:137 onNet)
pub fn on_net(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:139 lostTo)
pub fn lost_to(g: &mut crate::game::Game, m: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:140 mind)
pub fn mind<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::netquiet::Mind { todo!() }

/// (src/game/netquiet.ts:148 shock)
pub fn shock(g: &mut crate::game::Game, gd: GuardId, amount: f64) { todo!() }

/// (src/game/netquiet.ts:151 pay)
pub fn pay(g: &mut crate::game::Game, gd: GuardId, id_2: f64, amount: f64) { todo!() }

/// (src/game/netquiet.ts:153 owe)
pub fn owe(n: &mut crate::netquiet::Mind, kind: crate::netquiet::LineKind, name: &str, after: f64) { todo!() }

/// (src/game/netquiet.ts:154 speak)
pub fn speak(g: &mut crate::game::Game, gd: GuardId, kind: crate::netquiet::LineKind, name: &str, radio: bool) { todo!() }

/// (src/game/netquiet.ts:160 netUp)
pub fn net_up(g: &mut crate::game::Game, gd: GuardId, but: Option<&[GuardId]>) -> bool { todo!() }

/// (src/game/netquiet.ts:165 quietFor)
pub fn quiet_for(g: &mut crate::game::Game, asker: GuardId, silent: &[GuardId], hearers: &[GuardId]) { todo!() }

/// (src/game/netquiet.ts:177 netQuietAsked)
pub fn net_quiet_asked(g: &mut crate::game::Game, asker: Option<GuardId>, who: GuardId, men: &[GuardId]) { todo!() }

/// (src/game/netquiet.ts:184 netQuietClosed)
pub fn net_quiet_closed(g: &mut crate::game::Game, asker: GuardId, silent: &[GuardId], heard: &[GuardId]) { todo!() }

/// (src/game/netquiet.ts:192 bodyOf)
pub fn body_of(g: &mut crate::game::Game, finder: GuardId, men: &[GuardId]) -> Option<GuardId> { todo!() }

/// (src/game/netquiet.ts:202 netQuietBody)
pub fn net_quiet_body(g: &mut crate::game::Game, finder: Option<GuardId>, men: &[GuardId]) { todo!() }

/// (src/game/netquiet.ts:214 foeInView)
pub fn foe_in_view(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:223 netQuietFall)
pub fn net_quiet_fall(g: &mut crate::game::Game, victim: GuardId) { todo!() }

/// (src/game/netquiet.ts:242 noiseOf)
pub fn noise_of(g: &mut crate::game::Game, gd: GuardId, d: Option<RoomfightDefId>, small: bool, loud: bool) -> Option<V3> { todo!() }

/// (src/game/netquiet.ts:258 whipping)
pub fn whipping(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:262 arcAt)
pub fn arc_at(g: &mut crate::game::Game, e: &[crate::roomfight::Entry], p: V3) -> crate::netquiet::ArcAt { todo!() }

/// (src/game/netquiet.ts:281 lowSpot)
pub fn low_spot(g: &mut crate::game::Game, gd: GuardId, r: RoomfightRoomId) -> Option<V3> { todo!() }

/// (src/game/netquiet.ts:303 free)
pub fn free(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:307 lastKind)
pub fn last_kind(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/netquiet.ts:317 knowsSquad)
pub fn knows_squad(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:323 mine)
pub fn mine(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/netquiet.ts:325 syncLastMan)
pub fn sync_last_man(g: &mut crate::game::Game) { todo!() }

/// (src/game/netquiet.ts:327 posted)
pub fn posted(g: &mut crate::game::Game, gd: GuardId, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/netquiet.ts:330 netQuietTick)
pub fn net_quiet_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/netquiet.ts:425 rouse)
pub fn rouse(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/netquiet.ts:427 ease)
pub fn ease(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/netquiet.ts:428 wary)
pub fn wary(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/netquiet.ts:430 settle)
pub fn settle(g: &mut crate::game::Game, gd: GuardId, d: RoomfightDefId) { todo!() }

/// (src/game/netquiet.ts:438 watch)
pub fn watch(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/netquiet.ts:447 netQuietLook)
pub fn net_quiet_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/netquiet.ts:456 netQuietPose)
pub fn net_quiet_pose(g: &mut crate::game::Game, gd: GuardId, dt: f64, eyes_on_foe: bool) -> Option<crate::netquiet::NetQuietPose> { todo!() }

/// (src/game/netquiet.ts:469 netQuietSpot)
pub fn net_quiet_spot(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::netquiet::NetQuietSpot> { todo!() }

/// (src/game/netquiet.ts:474 netQuietOf)
pub fn net_quiet_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::netquiet::Mind> { todo!() }

/// (src/game/netquiet.ts:476 netQuietSummary)
pub fn net_quiet_summary(g: &mut crate::game::Game) -> String { todo!() }
