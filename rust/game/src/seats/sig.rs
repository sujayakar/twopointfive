#![allow(unused_variables)]
//! Every function and method of `seats`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/seats.ts:39 seatsOfSearch)
pub fn seats_of_search(g: &mut crate::game::Game, search: Option<&str>) -> f64 { todo!() }

/// (src/game/seats.ts:68 seatView)
pub fn seat_view(g: &mut crate::game::Game, own: &mut crate::seats::SeatSet) -> PlayerId { todo!() }

/// (src/game/seats.ts:78 seatsStand)
pub fn seats_stand(g: &mut crate::game::Game, n: f64) -> Vec<SeatId> { todo!() }

/// (src/game/seats.ts:96 miss)
pub fn miss(g: &mut crate::game::Game, k: &str) { todo!() }

/// (src/game/seats.ts:103 rawGame)
pub fn raw_game(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/seats.ts:107 seatManId)
pub fn seat_man_id(i: f64) -> f64 { todo!() }

/// (src/game/seats.ts:109 isManId)
pub fn is_man_id(g: &mut crate::game::Game, id_2: f64) -> bool { todo!() }

/// (src/game/seats.ts:111 men)
pub fn men(g: &mut crate::game::Game, g_2: &crate::seats::MenG) -> Vec<PlayerId> { todo!() }

/// (src/game/seats.ts:113 menUp)
pub fn men_up(g: &mut crate::game::Game) -> Vec<PlayerId> { todo!() }

/// (src/game/seats.ts:115 allDown)
pub fn all_down(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/seats.ts:117 anyDown)
pub fn any_down(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/seats.ts:122 seatsKind)
pub fn seats_kind(kind: &str) -> bool { todo!() }

/// (src/game/seats.ts:123 seatsRefuse)
pub fn seats_refuse(g: &mut crate::game::Game, kind: &str) -> Option<String> { todo!() }

/// (src/game/seats.ts:125 hostMan)
pub fn host_man(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/seats.ts:128 manHere)
pub fn man_here(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/seats.ts:129 eachSeat)
pub fn each_seat(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game, &mut crate::game::Game)) { todo!() }

/// (src/game/seats.ts:132 firstOf)
pub fn first_of<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game, &mut crate::game::Game) -> Option<T>) -> Option<T> { todo!() }

/// (src/game/seats.ts:134 seatOfChar)
pub fn seat_of_char(g: &mut crate::game::Game, c_2: Option<CharId>) -> Option<SeatId> { todo!() }

/// (src/game/seats.ts:136 manOf)
pub fn man_of(g: &mut crate::game::Game, c_2: Option<CharId>) -> Option<PlayerId> { todo!() }

/// (src/game/seats.ts:138 seatOfId)
pub fn seat_of_id(g: &mut crate::game::Game, id_2: f64) -> Option<SeatId> { todo!() }

/// (src/game/seats.ts:139 manById)
pub fn man_by_id(g: &mut crate::game::Game, id_2: f64) -> Option<PlayerId> { todo!() }

/// (src/game/seats.ts:141 viewOfId)
pub fn view_of_id(g: &mut crate::game::Game, id_2: f64) -> Option<PlayerId> { todo!() }

/// (src/game/seats.ts:143 seatOfMan)
pub fn seat_of_man(g: &mut crate::game::Game, pl: PlayerId) -> SeatId { todo!() }

/// (src/game/seats.ts:152 seatTone)
pub fn seat_tone(seat: f64) -> crate::seats::SeatTone { todo!() }

/// (src/game/seats.ts:153 playerName)
pub fn player_name(g: &mut crate::game::Game, pl: PlayerId) -> String { todo!() }

/// (src/game/seats.ts:155 nearestMan)
pub fn nearest_man(g: &mut crate::game::Game, at: &[f64]) -> Option<PlayerId> { todo!() }

/// (src/game/seats.ts:163 standMen)
pub fn stand_men(g: &mut crate::game::Game) { todo!() }

/// (src/game/seats.ts:170 spotBeside)
pub fn spot_beside(g: &mut crate::game::Game, at: V3, i: f64) -> V3 { todo!() }

/// (src/game/seats.ts:180 stageMen)
pub fn stage_men(g: &mut crate::game::Game, at: V3, yaw: f64, each: Option<&mut dyn FnMut(&mut crate::game::Game, PlayerId)>) { todo!() }

/// (src/game/seats.ts:190 cutToMen)
pub fn cut_to_men(g: &mut crate::game::Game) { todo!() }

/// (src/game/seats.ts:197 facedSeat)
pub fn faced_seat(g: &mut crate::game::Game, gd: GuardId) -> SeatId { todo!() }

/// (src/game/seats.ts:207 facedMan)
pub fn faced_man(g: &mut crate::game::Game, gd: GuardId) -> PlayerId { todo!() }

/// (src/game/seats.ts:208 facedView)
pub fn faced_view(g: &mut crate::game::Game, gd: GuardId) -> PlayerId { todo!() }

/// (src/game/seats.ts:212 viewOfBody)
pub fn view_of_body(g: &mut crate::game::Game, c_2: crate::seats::ViewOfBodyC) -> PlayerId { todo!() }

/// (src/game/seats.ts:218 viewFor)
pub fn view_for(g: &mut crate::game::Game, gd: GuardId) -> PlayerId { todo!() }

/// (src/game/seats.ts:224 dealSeats)
pub fn deal_seats(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/seats.ts:235 seatOf)
pub fn seat_of(g: &mut crate::game::Game, gd: GuardId) -> SeatId { todo!() }

/// (src/game/seats.ts:237 seatHas)
pub fn seat_has(g: &mut crate::game::Game, s: SeatId, gd: GuardId) -> bool { todo!() }

/// (src/game/seats.ts:240 oneBook)
pub fn one_book(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/seats.ts:242 leaderOf)
pub fn leader_of(g: &mut crate::game::Game, gd: GuardId) -> PlayerId { todo!() }

/// (src/game/seats.ts:244 viewOf)
pub fn view_of(g: &mut crate::game::Game, gd: GuardId) -> PlayerId { todo!() }

/// (src/game/seats.ts:246 ordersOf)
pub fn orders_of(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/seats.ts:248 orderOf)
pub fn order_of(g: &mut crate::game::Game, gd: GuardId) -> Option<OrdersAssignmentId> { todo!() }

/// (src/game/seats.ts:250 teamActive)
pub fn team_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/seats.ts:252 teamSquad)
pub fn team_squad(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/seats.ts:254 teamMen)
pub fn team_men(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/seats.ts:256 teamPaired)
pub fn team_paired(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/seats.ts:259 seatShare)
pub fn seat_share(g: &mut crate::game::Game, n: f64) -> f64 { todo!() }

/// (src/game/seats.ts:267 lastOrder)
pub fn last_order(g: &mut crate::game::Game) -> Option<crate::orders::SquadOrdersLogItem> { todo!() }

/// (src/game/seats.ts:272 teamBooks)
pub fn team_books(g: &mut crate::game::Game) -> Vec<crate::orders::SquadOrders> { todo!() }

/// (conventions, SEATS) a pass about another seat: its SEAT_KEYS fields are swapped onto Game for the call, and back
pub fn with_seat<R>(g: &mut crate::game::Game, seat: PlayerId, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R { todo!() }
/// INVENTED: which row of a table kept per view (seats.ts SEAT_TABLES) this pass reads: the seat on Game, or CAST_KEY while g.in_cast
pub fn view_key(g: &mut crate::game::Game) -> PlayerId { todo!() }
/// the row a second cast of the squad's code keeps in a table kept per view
pub const CAST_KEY: PlayerId = PlayerId(u32::MAX);

