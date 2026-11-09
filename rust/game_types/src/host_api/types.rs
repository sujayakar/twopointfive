//! The types of `host_api`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
#[bytes(parts)]
pub struct HostApi {
    pub world: crate::scene_boxes::BoxWorld,
    pub lights: crate::render_lights::LightSet,
    pub weather: Option<crate::render_frame::WeatherFrame>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub air: Option<crate::host_api::HostApiAir>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub fires: Option<Vec<crate::render_passes::FireRecord>>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub host_books: Option<crate::host_api::HostBooks>,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub skin: Option<crate::render_gbuffer::SkinnedDrawable>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub device: crate::Opaque,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HostApiAir {
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Debug, Default)]
pub struct HostBooks {
    pub fire: Option<crate::fire::FireShown>,
    pub breath: Option<crate::snow_breath::BreathShown>,
    pub snow: Option<crate::snow_fx::SnowState>,
    pub snow_blast: Option<crate::snow_blast::SnowBlastShown>,
    pub rain: Option<crate::rain_fx::RainState>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HostApiGbuffer {
}
