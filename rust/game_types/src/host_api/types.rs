//! The types of `host_api`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
#[bytes(parts)]
pub struct HostApi {
    pub world: crate::scene_boxes::BoxWorld,
    pub lights: crate::render_lights::LightSet,
    /// (the host's own: in no bytes)
    #[bytes(skip)]
    pub settings: crate::root_settings::RenderSettings,
    pub weather: Option<crate::render_frame::WeatherFrame>,
    /// (the host's own: in no bytes)
    #[bytes(skip)]
    pub sharp_on: Option<bool>,
    /// (the host's own: in no bytes)
    #[bytes(skip)]
    pub effective_scale: f64,
    /// (the picture's: in no bytes)
    #[bytes(skip)]
    pub host_books: Option<crate::host_api::HostBooks>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub device: crate::Opaque,
}

#[derive(Clone, Debug, Default)]
pub struct HostBooks {
    pub fire: Option<crate::fire::FireShown>,
}

