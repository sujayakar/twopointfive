#![allow(unused_variables)]
//! Every function and method of `audio_iface`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/audio_iface.ts:6 GameAudioLike.play)
pub fn game_audio_like_play(g: &mut crate::game::Game, name: &str, pos: Option<V3>, gain: Option<f64>, opts: Option<&crate::audio_iface::PlayOpts>) { todo!() }

/// (src/game/audio_iface.ts:9 GameAudioLike.footstep)
pub fn game_audio_like_footstep(g: &mut crate::game::Game, pos: V3, loudness: f64, is_player: bool, mat: Option<crate::floor::FloorMat>) { todo!() }

/// (src/game/audio_iface.ts:11 GameAudioLike.rain)
pub fn game_audio_like_rain(g: &mut crate::game::Game, level: f64) { todo!() }


/// INVENTED, for the mixer's update, in place of the `follow` closure of game.ts:1436: where a rolling chair's voice is now, how hard it goes and the game's clock, for the voice whose Roll.voice is `tok`. None: let the voice go.
pub fn audio_follow_now(g: &mut crate::game::Game, tok: f64) -> Option<(V3, f64, f64)> { todo!() }

pub mod null_audio {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/audio_iface.ts:13 nullAudio.play)
    pub fn play() { todo!() }

    /// (src/game/audio_iface.ts:13 nullAudio.footstep)
    pub fn footstep() { todo!() }

}
