//! The types of `audio_iface`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: GameAudioLike, src/game/audio_iface.ts:5)
#[derive(Clone, Copy, Debug)]
pub struct GameAudioLike {
}
impl GameAudioLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GameAudioLike {
        }
    }
}
impl Default for GameAudioLike { fn default() -> Self { Self::zeroed() } }

/// (src/game/audio_iface.ts:13 nullAudio: a table nothing writes)
pub fn null_audio() -> crate::audio_iface::GameAudioLike { crate::audio_iface::GameAudioLike::zeroed() }
