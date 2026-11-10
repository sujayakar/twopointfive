//! The types of `levelfit`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct LevelFit {
    pub doors: f64,
    pub things: f64,
    pub parts: f64,
    pub sheets: f64,
    pub fixtures: f64,
    pub cameras: f64,
    pub lasers: f64,
    pub beacons: f64,
    pub lamps: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreakRoom {
    pub boxes: f64,
    pub items: f64,
    pub cuts: f64,
}

#[derive(Clone, Debug, Default)]
pub struct FitRow {
    pub what: String,
    pub has: f64,
    pub cap: f64,
    pub play: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct Stood {
    pub level: crate::levelfit::StoodLevel,
    pub col: crate::levelfit::StoodCol,
    pub props: Option<crate::levelfit::StoodProps>,
    pub player: Option<PlayerId>,
    pub guards: Option<Vec<GuardId>>,
    pub civilians: Option<Vec<GuardId>>,
    pub seats: Option<Vec<SeatId>>,
}
impl Default for Stood {
    fn default() -> Self {
        Stood {
            level: crate::levelfit::StoodLevel::default(),
            col: crate::levelfit::StoodCol::default(),
            props: None,
            player: None,
            guards: Some(Vec::new()),
            civilians: Some(Vec::new()),
            seats: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StoodLevel {
    pub doors: Vec<crate::Opaque>,
    pub props: Vec<crate::levelfit::StoodLevelPropsItem>,
    pub boxes: Vec<BoxId>,
    pub lights: Option<Vec<crate::Opaque>>,
    pub switchables: Option<Vec<crate::levelfit::StoodLevelSwitchablesItem>>,
    pub beacons: Option<Vec<crate::Opaque>>,
    pub devices: Option<Vec<crate::levelfit::StoodLevelDevicesItem>>,
    pub civilians: Option<Vec<crate::Opaque>>,
}

#[derive(Clone, Debug, Default)]
pub struct StoodLevelPropsItem {
    pub parts: Vec<crate::Opaque>,
}

#[derive(Clone, Debug, Default)]
pub struct StoodLevelSwitchablesItem {
    pub boxes: Vec<f64>,
    pub group: String,
}

#[derive(Clone, Debug, Default)]
pub struct StoodLevelDevicesItem {
    pub kind: String,
}

#[derive(Clone, Debug, Default)]
pub struct StoodCol {
    pub boxes: Vec<BoxId>,
    pub dynamic_boxes: Vec<BoxId>,
}

#[derive(Clone, Debug, Default)]
pub struct StoodProps {
    pub all: Vec<crate::levelfit::StoodPropsAllItem>,
}

#[derive(Clone, Debug, Default)]
pub struct StoodPropsAllItem {
    pub boxes: Vec<crate::Opaque>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HandOverMost {
    pub door_men: f64,
}

#[derive(Clone, Debug, Default)]
pub struct LevelCountsLevel {
    pub doors: Vec<crate::Opaque>,
    pub props: Vec<crate::levelfit::StoodLevelPropsItem>,
    pub boxes: Vec<BoxId>,
    pub lights: Option<Vec<crate::Opaque>>,
    pub switchables: Option<Vec<crate::levelfit::StoodLevelSwitchablesItem>>,
    pub beacons: Option<Vec<crate::Opaque>>,
    pub devices: Option<Vec<crate::levelfit::StoodLevelDevicesItem>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FlatPanel {
    pub thin: f64,
    pub a0: f64,
    pub a1: f64,
    pub area: f64,
}
