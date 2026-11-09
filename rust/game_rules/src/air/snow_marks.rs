#![allow(unused_variables)]
use crate::game::Game;

/// (src/game/snowMarks.ts:73 clearSnowMarks)
pub fn clear_snow_marks(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::ClearSnowMarks); }
