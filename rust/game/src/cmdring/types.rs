//! The types of `cmdring`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RingState, src/game/cmdring.ts:201)
#[derive(Clone, Copy, Debug)]
pub struct RingState {
    pub ax: f64,
    pub ay: f64,
    pub z: f64,
    pub t0: f64,
    pub lit: f64,
    pub armed: bool,
    pub free: Option<bool>,
    pub more: Option<bool>,
    pub at: Option<V3>,
    pub ox: Option<f64>,
    pub oy: Option<f64>,
    pub mx: Option<f64>,
    pub my: Option<f64>,
    pub clear: Option<bool>,
    pub pt: Option<f64>,
}
impl RingState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RingState {
            ax: 0.0,
            ay: 0.0,
            z: 0.0,
            t0: 0.0,
            lit: 0.0,
            armed: false,
            free: None,
            more: None,
            at: None,
            ox: None,
            oy: None,
            mx: None,
            my: None,
            clear: None,
            pt: None,
        }
    }
}
impl Default for RingState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RingView, src/game/cmdring.ts:210)
#[derive(Clone, Debug)]
pub struct RingView {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    pub lit: f64,
    pub tone: Option<String>,
    pub caption: String,
    pub wedges: Vec<crate::cmdring::RingViewWedgesItem>,
}
impl RingView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RingView {
            x: 0.0,
            y: 0.0,
            zoom: 0.0,
            lit: 0.0,
            tone: None,
            caption: String::new(),
            wedges: Vec::new(),
        }
    }
}
impl Default for RingView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cmdring.ts:210 RingView.wedges, src/game/cmdring.ts:210)
#[derive(Clone, Debug)]
pub struct RingViewWedgesItem {
    pub key: f64,
    pub text: String,
    pub off: bool,
    pub opens: bool,
}
impl RingViewWedgesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RingViewWedgesItem {
            key: 0.0,
            text: String::new(),
            off: false,
            opens: false,
        }
    }
}
impl Default for RingViewWedgesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SeatRow, src/game/cmdring.ts:248)
#[derive(Clone, Debug)]
pub struct SeatRow {
    pub key: Option<f64>,
    pub hot_key: Option<String>,
    pub opens: Option<String>,
    pub free: Option<bool>,
    pub targets: Vec<String>,
    pub listed: Option<Vec<String>>,
}
impl SeatRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SeatRow {
            key: None,
            hot_key: None,
            opens: None,
            free: None,
            targets: Vec::new(),
            listed: None,
        }
    }
}
impl Default for SeatRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FlatRow, src/game/cmdring.ts:272)
#[derive(Clone, Debug)]
pub struct FlatRow {
    pub id: String,
    pub key: Option<f64>,
    pub hot_key: Option<String>,
    pub opens: Option<String>,
    pub free: Option<bool>,
    pub targets: Vec<String>,
    pub listed: Option<Vec<String>>,
}
impl FlatRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlatRow {
            id: String::new(),
            key: None,
            hot_key: None,
            opens: None,
            free: None,
            targets: Vec::new(),
            listed: None,
        }
    }
}
impl Default for FlatRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RingHint {
    Door,
    Wall,
    Man,
    Floor,
    Foe,
    Thing,
    Squad,
}
impl RingHint {
    pub const fn as_str(self) -> &'static str {
        match self {
            RingHint::Door => "door",
            RingHint::Wall => "wall",
            RingHint::Man => "man",
            RingHint::Floor => "floor",
            RingHint::Foe => "foe",
            RingHint::Thing => "thing",
            RingHint::Squad => "squad",
        }
    }
}

/// (TypeScript: inline at src/game/cmdring.ts:255 ringSeats ->, src/game/cmdring.ts:255)
#[derive(Clone, Debug)]
pub struct RingSeats {
    pub row: Vec<f64>,
    pub more: f64,
    pub over: Vec<f64>,
}
impl RingSeats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RingSeats {
            row: Vec::new(),
            more: 0.0,
            over: Vec::new(),
        }
    }
}
impl Default for RingSeats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cmdring.ts:287 flatSeats ->, src/game/cmdring.ts:287)
#[derive(Clone, Debug)]
pub struct FlatSeats {
    pub row: Vec<f64>,
    pub more: f64,
    pub over: Vec<f64>,
}
impl FlatSeats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlatSeats {
            row: Vec::new(),
            more: 0.0,
            over: Vec::new(),
        }
    }
}
impl Default for FlatSeats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RingLatch {
    Stay,
    Hold,
    Give,
}
impl RingLatch {
    pub const fn as_str(self) -> &'static str {
        match self {
            RingLatch::Stay => "stay",
            RingLatch::Hold => "hold",
            RingLatch::Give => "give",
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CmdringStats {
    pub opened: i32,
    pub quick: i32,
    pub given: i32,
    pub pages: i32,
    pub cancelled: i32,
    pub who: i32,
    pub fired: i32,
    pub twice: i32,
}

/// (src/game/cmdring.ts:276 FLAT_TOP: a table nothing writes)
pub fn flat_top() -> JsMap<String, JsMap<String, f64>> { { let mut t = JsMap::new(); t.set(String::from("floor"), { let mut t = JsMap::new(); t.set(String::from("cover"), 2.0); t.set(String::from("fallIn"), 3.0); t.set(String::from("hold"), 4.0); t }); t.set(String::from("door"), { let mut t = JsMap::new(); t.set(String::from("cover"), 2.0); t.set(String::from("openClear"), 3.0); t.set(String::from("bangClear"), 4.0); t }); t.set(String::from("none"), { let mut t = JsMap::new(); t.set(String::from("fallIn"), 3.0); t.set(String::from("hold"), 4.0); t }); t.set(String::from("foe"), JsMap::new()); t } }

/// (src/game/cmdring.ts:279 FLAT_LEFT: a table nothing writes)
pub fn flat_left() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("door"), alloc::vec![String::from("fallIn"), String::from("hold")]); t } }
