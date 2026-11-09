//! The types of `audio_iface`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a call of GameAudioLike.play asks beyond its name, place and gain, by the shapes the calls have (audio_iface.ts:6). None: no opts, and the mixer draws its own rate (audio.ts:190)
#[derive(Clone, Debug)]
pub enum PlayOpts {
    Rate { rate: f64 },
    Thunder { lowpass: f64, rate: f64, pan: f64, wet: f64 },
}

