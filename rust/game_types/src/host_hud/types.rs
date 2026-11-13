//! The types of `host_hud`.
use crate::bytes::ToBytes;

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct View {
    pub client_width: f64,
    pub client_height: f64,
    pub width: f64,
    pub height: f64,
}
