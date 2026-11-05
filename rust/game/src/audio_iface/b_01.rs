//! THE BRING-UP'S OWN PART of audio_iface: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/audio_iface.ts:6 GameAudioLike.play)
pub fn game_audio_like_play(g: &mut crate::game::Game, name: &str, pos: Option<V3>, gain: Option<f64>, opts: Option<&JsMap<String, crate::Opaque>>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/audio_iface.ts:9 GameAudioLike.footstep)
pub fn game_audio_like_footstep(g: &mut crate::game::Game, pos: V3, loudness: f64, is_player: bool, mat: Option<crate::floor::FloorMat>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/audio_iface.ts:11 GameAudioLike.rain)
pub fn game_audio_like_rain(g: &mut crate::game::Game, level: f64) {}
