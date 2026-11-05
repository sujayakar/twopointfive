#![allow(unused_variables)]
//! Every function and method of `sensors`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/sensors.ts:24 ProbeLike.query)
pub fn probe_like_query(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>, rank: Option<f64>) { todo!() }

/// (src/game/sensors.ts:25 ProbeLike.querySegment)
pub fn probe_like_query_segment(g: &mut crate::game::Game, key: &str, a: V3, b: V3, rank: Option<f64>) { todo!() }

/// (src/game/sensors.ts:26 ProbeLike.get)
pub fn probe_like_get(g: &mut crate::game::Game, key: &str) -> Option<Vec<f64>> { todo!() }

/// (src/game/sensors.ts:47 askRank)
pub fn ask_rank(key: &str, segment: bool) -> f64 { todo!() }

/// (src/game/sensors.ts:51 encodeValue)
pub fn encode_value(v: Option<f64>) -> Option<crate::sensors::SensorEnc> { todo!() }

/// (src/game/sensors.ts:59 decodeValue)
pub fn decode_value(e: Option<crate::sensors::SensorEnc>) -> Option<f64> { todo!() }

/// (src/game/sensors.ts:65 sameValue)
pub fn same_value(a: Option<f64>, b: Option<f64>) -> bool { todo!() }

/// (src/game/sensors.ts:91 Sensors.mode)
pub fn sensors_mode(g: &mut crate::game::Game) -> crate::sensors::SensorMode { todo!() }

/// (src/game/sensors.ts:93 Sensors.frame)
pub fn sensors_frame(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/sensors.ts:94 Sensors.stats)
pub fn sensors_stats(g: &mut crate::game::Game) -> crate::sensors::SensorStats { todo!() }

/// (src/game/sensors.ts:96 Sensors.firstDivergence)
pub fn sensors_first_divergence(g: &mut crate::game::Game) -> Option<crate::sensors::SensorDivergence> { todo!() }

/// (src/game/sensors.ts:97 Sensors.divergenceLog)
pub fn sensors_divergence_log(g: &mut crate::game::Game) -> Vec<crate::sensors::SensorDivergence> { todo!() }

/// (src/game/sensors.ts:105 Sensors.setOwn)
pub fn sensors_set_own(g: &mut crate::game::Game, f: Option<&mut dyn FnMut(&mut crate::game::Game, V3, V3) -> f64>) { todo!() }

/// (src/game/sensors.ts:107 Sensors.ownReset)
pub fn sensors_own_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/sensors.ts:109 Sensors.constructor)
pub fn sensors_new(g: &mut crate::game::Game, probe_of: &mut dyn FnMut(&mut crate::game::Game) -> crate::sensors::ProbeLike) -> crate::sensors::Sensors { todo!() }

/// (src/game/sensors.ts:113 Sensors.ask)
pub fn sensors_ask(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>) { todo!() }

/// (src/game/sensors.ts:115 Sensors.askSegment)
pub fn sensors_ask_segment(g: &mut crate::game::Game, key: &str, a: V3, b: V3) { todo!() }

/// (src/game/sensors.ts:117 Sensors.forget)
pub fn sensors_forget(g: &mut crate::game::Game, key: &str) { todo!() }

/// (src/game/sensors.ts:119 Sensors.read)
pub fn sensors_read(g: &mut crate::game::Game, key: &str) -> Option<f64> { todo!() }

/// (src/game/sensors.ts:139 Sensors.segment)
pub fn sensors_segment(g: &mut crate::game::Game, key: &str, a: V3, b: V3, ask: Option<bool>) -> Option<f64> { todo!() }

/// (src/game/sensors.ts:143 Sensors.meterPair)
pub fn sensors_meter_pair(g: &mut crate::game::Game, kc: &str, kh: &str, pc: V3, ph: V3, id_2: f64) -> Option<f64> { todo!() }

/// (src/game/sensors.ts:149 Sensors.lightAt)
pub fn sensors_light_at(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>, ask: Option<bool>) -> Option<f64> { todo!() }

/// (src/game/sensors.ts:153 Sensors.beginFrame)
pub fn sensors_begin_frame(g: &mut crate::game::Game) { todo!() }

/// (src/game/sensors.ts:170 Sensors.endFrame)
pub fn sensors_end_frame(g: &mut crate::game::Game) { todo!() }

/// (src/game/sensors.ts:188 Sensors.push)
pub fn sensors_push(g: &mut crate::game::Game, r: &[Option<crate::sensors::SensorsPushRItem>]) { todo!() }

/// (src/game/sensors.ts:189 Sensors.diverge)
pub fn sensors_diverge(g: &mut crate::game::Game, d: &crate::sensors::SensorDivergence) { todo!() }

/// (src/game/sensors.ts:193 Sensors.live)
pub fn sensors_live(g: &mut crate::game::Game) { todo!() }

/// (src/game/sensors.ts:195 Sensors.record)
pub fn sensors_record(g: &mut crate::game::Game) { todo!() }

/// (src/game/sensors.ts:201 Sensors.journal)
pub fn sensors_journal(g: &mut crate::game::Game) -> crate::sensors::SensorJournal { todo!() }

/// (src/game/sensors.ts:203 Sensors.approxBytes)
pub fn sensors_approx_bytes(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/sensors.ts:205 Sensors.replay)
pub fn sensors_replay(g: &mut crate::game::Game, src: &crate::sensors::SensorJournal) { todo!() }

/// (src/game/sensors.ts:210 Sensors.summary)
pub fn sensors_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/sensors.ts:215 first)
pub fn first(r: Option<&[f64]>) -> Option<f64> { todo!() }
