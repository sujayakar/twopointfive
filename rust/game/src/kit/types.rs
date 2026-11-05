//! The types of `kit`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SlotTable, src/game/kit.ts:248)
#[derive(Clone, Debug)]
pub struct SlotTable {
    pub v: f64,
    pub pouches: Vec<f64>,
    pub big: Vec<f64>,
    pub squad: Option<f64>,
    pub items: JsMap<String, crate::kit::SlotItem>,
    pub unknown: Vec<String>,
    pub loadouts: Vec<crate::kit::SlotLoadout>,
}
impl SlotTable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotTable {
            v: 0.0,
            pouches: Vec::new(),
            big: Vec::new(),
            squad: None,
            items: JsMap::new(),
            unknown: Vec::new(),
            loadouts: Vec::new(),
        }
    }
}
impl Default for SlotTable { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlotItem, src/game/kit.ts:242)
#[derive(Clone, Debug)]
pub struct SlotItem {
    pub id: String,
    pub pouches: f64,
    pub max: Vec<f64>,
    pub squad_max: Option<f64>,
    pub big: Option<bool>,
    pub holds: Option<f64>,
}
impl SlotItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotItem {
            id: String::new(),
            pouches: 0.0,
            max: Vec::new(),
            squad_max: None,
            big: None,
            holds: None,
        }
    }
}
impl Default for SlotItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SlotLoadout, src/game/kit.ts:245)
#[derive(Clone, Debug)]
pub struct SlotLoadout {
    pub id: String,
    pub name: String,
    pub counts: Vec<JsMap<String, f64>>,
    pub cards: Vec<JsMap<String, crate::kit::KitValue>>,
    pub missing: Vec<String>,
}
impl SlotLoadout {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotLoadout {
            id: String::new(),
            name: String::new(),
            counts: Vec::new(),
            cards: Vec::new(),
            missing: Vec::new(),
        }
    }
}
impl Default for SlotLoadout { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitWeapon {
    Pistol,
    Rifle,
    Shotgun,
}
impl KitWeapon {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitWeapon::Pistol => "pistol",
            KitWeapon::Rifle => "rifle",
            KitWeapon::Shotgun => "shotgun",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigKit {
    Shotgun,
    None,
    Hammer,
}
impl BigKit {
    pub const fn as_str(self) -> &'static str {
        match self {
            BigKit::Shotgun => "shotgun",
            BigKit::None => "none",
            BigKit::Hammer => "hammer",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigPick {
    Shotgun,
    None,
    Hammer,
    Wall,
}
impl BigPick {
    pub const fn as_str(self) -> &'static str {
        match self {
            BigPick::Shotgun => "shotgun",
            BigPick::None => "none",
            BigPick::Hammer => "hammer",
            BigPick::Wall => "wall",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SuitId {
    Issued,
    Black,
    Grey,
    Navy,
    Olive,
    Jungle,
    Khaki,
    Desert,
}
impl SuitId {
    pub const fn as_str(self) -> &'static str {
        match self {
            SuitId::Issued => "issued",
            SuitId::Black => "black",
            SuitId::Grey => "grey",
            SuitId::Navy => "navy",
            SuitId::Olive => "olive",
            SuitId::Jungle => "jungle",
            SuitId::Khaki => "khaki",
            SuitId::Desert => "desert",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TrimId {
    Red,
    Blue,
    Green,
    Issued,
    Black,
    Amber,
    White,
}
impl TrimId {
    pub const fn as_str(self) -> &'static str {
        match self {
            TrimId::Red => "red",
            TrimId::Blue => "blue",
            TrimId::Green => "green",
            TrimId::Issued => "issued",
            TrimId::Black => "black",
            TrimId::Amber => "amber",
            TrimId::White => "white",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CanMix {
    Smoke,
    Stun,
    Issued,
    Even,
}
impl CanMix {
    pub const fn as_str(self) -> &'static str {
        match self {
            CanMix::Smoke => "smoke",
            CanMix::Stun => "stun",
            CanMix::Issued => "issued",
            CanMix::Even => "even",
        }
    }
}

/// (TypeScript: MemberKit, src/game/kit.ts:87)
#[derive(Clone, Debug)]
pub struct MemberKit {
    pub weapon: crate::kit::KitWeapon,
    pub suppressed: bool,
    pub light: bool,
    pub nvg: bool,
    pub bangs: f64,
    pub charges: f64,
    pub smoke: f64,
    pub frags: f64,
    pub walls: f64,
    pub shells: f64,
    pub suit: crate::kit::SuitId,
    pub trim: crate::kit::TrimId,
    pub wand: bool,
    pub cams: f64,
    pub bare: Option<bool>,
    pub big: crate::kit::BigKit,
    pub primary: String,
    pub secondary: String,
    pub side_can: bool,
    pub rings: f64,
}
impl MemberKit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MemberKit {
            weapon: crate::kit::KitWeapon::Pistol,
            suppressed: false,
            light: false,
            nvg: false,
            bangs: 0.0,
            charges: 0.0,
            smoke: 0.0,
            frags: 0.0,
            walls: 0.0,
            shells: 0.0,
            suit: crate::kit::SuitId::Issued,
            trim: crate::kit::TrimId::Red,
            wand: false,
            cams: 0.0,
            bare: None,
            big: crate::kit::BigKit::Shotgun,
            primary: String::new(),
            secondary: String::new(),
            side_can: false,
            rings: 0.0,
        }
    }
}
impl Default for MemberKit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KitHand, src/game/kit.ts:170)
#[derive(Clone, Copy, Debug)]
pub struct KitHand {
    pub rings: f64,
    pub cams: f64,
    pub wand: bool,
}
impl KitHand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitHand {
            rings: 0.0,
            cams: 0.0,
            wand: false,
        }
    }
}
impl Default for KitHand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitField {
    Smoke,
    Light,
    Wand,
    Bangs,
    Frags,
    Shells,
    Charges,
    Cams,
    Walls,
    Suit,
    Bare,
    Big,
    Primary,
    Secondary,
    SideCan,
    Rings,
    Weapon,
    Suppressed,
    Nvg,
    Trim,
    Cans,
}
impl KitField {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitField::Smoke => "smoke",
            KitField::Light => "light",
            KitField::Wand => "wand",
            KitField::Bangs => "bangs",
            KitField::Frags => "frags",
            KitField::Shells => "shells",
            KitField::Charges => "charges",
            KitField::Cams => "cams",
            KitField::Walls => "walls",
            KitField::Suit => "suit",
            KitField::Bare => "bare",
            KitField::Big => "big",
            KitField::Primary => "primary",
            KitField::Secondary => "secondary",
            KitField::SideCan => "sideCan",
            KitField::Rings => "rings",
            KitField::Weapon => "weapon",
            KitField::Suppressed => "suppressed",
            KitField::Nvg => "nvg",
            KitField::Trim => "trim",
            KitField::Cans => "cans",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum KitValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: KitRow, src/game/kit.ts:744)
#[derive(Clone, Debug)]
pub struct KitRow {
    pub field: crate::kit::KitField,
    pub label: String,
    pub values: Vec<crate::kit::KitValue>,
    pub item: Option<String>,
}
impl KitRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitRow {
            field: crate::kit::KitField::Smoke,
            label: String::new(),
            values: Vec::new(),
            item: None,
        }
    }
}
impl Default for KitRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitPreset {
    Quiet,
    Issued,
    Loud,
}
impl KitPreset {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitPreset::Quiet => "quiet",
            KitPreset::Issued => "issued",
            KitPreset::Loud => "loud",
        }
    }
}

/// (TypeScript: Store, src/game/kit.ts:1072)
#[derive(Clone, Copy, Debug)]
pub struct Store {
}
impl Store {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Store {
        }
    }
}
impl Default for Store { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:158 BUILT, src/game/kit.ts:158)
#[derive(Clone, Copy, Debug)]
pub struct BUILT {
    pub smoke: f64,
    pub bangs: f64,
    pub frags: f64,
    pub shells: f64,
}
impl BUILT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BUILT {
            smoke: 0.0,
            bangs: 0.0,
            frags: 0.0,
            shells: 0.0,
        }
    }
}
impl Default for BUILT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:175 KIT, src/game/kit.ts:175)
#[derive(Clone, Debug)]
pub struct KitModKIT {
    pub on: bool,
    pub members: Vec<crate::kit::MemberKit>,
}
impl KitModKIT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModKIT {
            on: false,
            members: Vec::new(),
        }
    }
}
impl Default for KitModKIT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:178 SUITS, src/game/kit.ts:178)
#[derive(Clone, Debug)]
pub struct SUITSValue {
    pub label: String,
    pub tint: V3,
}
impl SUITSValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SUITSValue {
            label: String::new(),
            tint: [0.0; 3],
        }
    }
}
impl Default for SUITSValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:187 TRIMS, src/game/kit.ts:187)
#[derive(Clone, Debug)]
pub struct TRIMSValue {
    pub label: String,
    pub tint: V3,
}
impl TRIMSValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TRIMSValue {
            label: String::new(),
            tint: [0.0; 3],
        }
    }
}
impl Default for TRIMSValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:197 CAN_MIXES, src/game/kit.ts:197)
#[derive(Clone, Debug)]
pub struct CANMIXESValue {
    pub label: String,
    pub smoke: f64,
    pub stun: f64,
}
impl CANMIXESValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CANMIXESValue {
            label: String::new(),
            smoke: 0.0,
            stun: 0.0,
        }
    }
}
impl Default for CANMIXESValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:797 YOU_GUNS_CAN, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct YOUGUNSCAN {
    pub pistol: Option<String>,
    pub rifle: Option<String>,
    pub shotgun: Option<String>,
}
impl YOUGUNSCAN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        YOUGUNSCAN {
            pistol: None,
            rifle: None,
            shotgun: None,
        }
    }
}
impl Default for YOUGUNSCAN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1007 KIT_PRESETS, src/game/kit.ts:1007)
#[derive(Clone, Debug)]
pub struct KITPRESETSItem {
    pub id: String,
    pub label: String,
    pub line: String,
}
impl KITPRESETSItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KITPRESETSItem {
            id: String::new(),
            label: String::new(),
            line: String::new(),
        }
    }
}
impl Default for KITPRESETSItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum KitModIssuing {
    Bool(bool),
    Own,
}

/// (TypeScript: inline at src/game/kit.ts:1180 lessonRow, src/game/kit.ts:1180)
#[derive(Clone, Copy, Debug)]
pub struct KitModLessonRow {
    pub packs: Option<crate::kit::KitModLessonRowPacks>,
}
impl KitModLessonRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModLessonRow {
            packs: None,
        }
    }
}
impl Default for KitModLessonRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1180 KitModLessonRow.packs, src/game/kit.ts:1180)
#[derive(Clone, Copy, Debug)]
pub struct KitModLessonRowPacks {
    pub frags: Option<f64>,
}
impl KitModLessonRowPacks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModLessonRowPacks {
            frags: None,
        }
    }
}
impl Default for KitModLessonRowPacks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1203 kept0, src/game/kit.ts:1203)
#[derive(Clone, Copy, Debug)]
pub struct KitModKept0 {
    pub charges: Option<f64>,
    pub walls: Option<f64>,
}
impl KitModKept0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModKept0 {
            charges: None,
            walls: None,
        }
    }
}
impl Default for KitModKept0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1222 manOf, src/game/kit.ts:1222)
#[derive(Clone, Debug)]
pub struct KitModManOfValue {
    pub i: f64,
    pub kit: crate::kit::MemberKit,
}
impl KitModManOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModManOfValue {
            i: 0.0,
            kit: crate::kit::MemberKit::zeroed(),
        }
    }
}
impl Default for KitModManOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1225 KIT_STAGED, src/game/kit.ts:1225)
#[derive(Clone, Debug)]
pub struct KitModKITSTAGED {
    pub n: f64,
    pub ops: Vec<GuardId>,
}
impl KitModKITSTAGED {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModKITSTAGED {
            n: 0.0,
            ops: Vec::new(),
        }
    }
}
impl Default for KitModKITSTAGED { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1239 issuedPlayer, src/game/kit.ts:1239)
#[derive(Clone, Copy, Debug)]
pub struct KitModIssuedPlayerValue {
    pub rifle: bool,
    pub gun: FirearmId,
    pub side: FirearmId,
    pub slot: crate::game::SlotKind,
    pub smoke: f64,
    pub stun: f64,
    pub frags: f64,
    pub shells: f64,
    pub rings: f64,
    pub wand: bool,
    pub big: crate::kit::BigKit,
    pub cams: f64,
    pub tint: V3,
    pub tint2: V3,
}
impl KitModIssuedPlayerValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModIssuedPlayerValue {
            rifle: false,
            gun: FirearmId(0),
            side: FirearmId(0),
            slot: crate::game::SlotKind::Pistol,
            smoke: 0.0,
            stun: 0.0,
            frags: 0.0,
            shells: 0.0,
            rings: 0.0,
            wand: false,
            big: crate::kit::BigKit::Shotgun,
            cams: 0.0,
            tint: [0.0; 3],
            tint2: [0.0; 3],
        }
    }
}
impl Default for KitModIssuedPlayerValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1242 youBare, src/game/kit.ts:1242)
#[derive(Clone, Copy, Debug)]
pub struct KitModYouBareValue {
    pub light: bool,
    pub nvg: bool,
}
impl KitModYouBareValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModYouBareValue {
            light: false,
            nvg: false,
        }
    }
}
impl Default for KitModYouBareValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1311 KIT_SOLO, src/game/kit.ts:1311)
#[derive(Clone, Copy, Debug)]
pub struct KitModKITSOLO {
    pub on: bool,
    pub stand: bool,
}
impl KitModKITSOLO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitModKITSOLO {
            on: false,
            stand: false,
        }
    }
}
impl Default for KitModKITSOLO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KindOf {
    You,
    Mate,
}
impl KindOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            KindOf::You => "you",
            KindOf::Mate => "mate",
        }
    }
}

/// (TypeScript: inline at src/game/kit.ts:427 stealthKept ->, src/game/kit.ts:427)
#[derive(Clone, Debug)]
pub struct StealthKept {
    pub stealth: Option<crate::kit::MemberKit>,
}
impl StealthKept {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StealthKept {
            stealth: None,
        }
    }
}
impl Default for StealthKept { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:455 dealTable ->, src/game/kit.ts:455)
#[derive(Clone, Debug)]
pub struct DealTable {
    pub kits: Vec<Vec<crate::kit::MemberKit>>,
    pub words: JsSet<String>,
}
impl DealTable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DealTable {
            kits: Vec::new(),
            words: JsSet::new(),
        }
    }
}
impl Default for DealTable { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SlotsCheck {
    Arm0(crate::kit::SlotsCheckArm0),
    Arm1(crate::kit::SlotsCheckArm1),
}

/// (TypeScript: inline at src/game/kit.ts:579 slotsCheck ->, src/game/kit.ts:579)
#[derive(Clone, Debug)]
pub struct SlotsCheckArm0 {
    pub table: crate::kit::SlotTable,
}
impl SlotsCheckArm0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotsCheckArm0 {
            table: crate::kit::SlotTable::zeroed(),
        }
    }
}
impl Default for SlotsCheckArm0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:579 slotsCheck ->, src/game/kit.ts:579)
#[derive(Clone, Debug)]
pub struct SlotsCheckArm1 {
    pub why: String,
}
impl SlotsCheckArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlotsCheckArm1 {
            why: String::new(),
        }
    }
}
impl Default for SlotsCheckArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:622 kitCost ->, src/game/kit.ts:622)
#[derive(Clone, Debug)]
pub struct KitCost {
    pub used: f64,
    pub total: f64,
    pub free: f64,
    pub lines: Vec<crate::kit::KitCostLinesItem>,
    pub big: Option<crate::kit::KitCostBig>,
}
impl KitCost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitCost {
            used: 0.0,
            total: 0.0,
            free: 0.0,
            lines: Vec::new(),
            big: None,
        }
    }
}
impl Default for KitCost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:622 KitCost.lines, src/game/kit.ts:622)
#[derive(Clone, Debug)]
pub struct KitCostLinesItem {
    pub item: String,
    pub n: f64,
    pub pouches: f64,
}
impl KitCostLinesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitCostLinesItem {
            item: String::new(),
            n: 0.0,
            pouches: 0.0,
        }
    }
}
impl Default for KitCostLinesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:622 KitCost.big, src/game/kit.ts:622)
#[derive(Clone, Copy, Debug)]
pub struct KitCostBig {
    pub used: f64,
    pub total: f64,
}
impl KitCostBig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitCostBig {
            used: 0.0,
            total: 0.0,
        }
    }
}
impl Default for KitCostBig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:628 squadCost ->, src/game/kit.ts:628)
#[derive(Clone, Copy, Debug)]
pub struct SquadCost {
    pub used: f64,
    pub total: f64,
    pub free: f64,
}
impl SquadCost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadCost {
            used: 0.0,
            total: 0.0,
            free: 0.0,
        }
    }
}
impl Default for SquadCost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum LoadoutCards {
    Text(String),
    Array(Vec<crate::kit::MemberKit>),
}

/// (TypeScript: inline at src/game/kit.ts:727 kitLoadouts ->, src/game/kit.ts:727)
#[derive(Clone, Debug)]
pub struct KitLoadoutsItem {
    pub id: String,
    pub name: String,
}
impl KitLoadoutsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitLoadoutsItem {
            id: String::new(),
            name: String::new(),
        }
    }
}
impl Default for KitLoadoutsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:926 kitRowCost ->, src/game/kit.ts:926)
#[derive(Clone, Debug)]
pub struct KitRowCost {
    pub pouches: f64,
    pub rate: String,
}
impl KitRowCost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitRowCost {
            pouches: 0.0,
            rate: String::new(),
        }
    }
}
impl Default for KitRowCost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:932 kitName ->, src/game/kit.ts:932)
#[derive(Clone, Debug)]
pub struct KitName {
    pub title: String,
    pub sub: String,
}
impl KitName {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitName {
            title: String::new(),
            sub: String::new(),
        }
    }
}
impl Default for KitName { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:964 kitPeek ->, src/game/kit.ts:964)
#[derive(Clone, Debug)]
pub struct KitPeek {
    pub value: crate::kit::KitValue,
    pub why: String,
    pub passed: String,
}
impl KitPeek {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitPeek {
            value: crate::kit::KitValue::Text(String::new()),
            why: String::new(),
            passed: String::new(),
        }
    }
}
impl Default for KitPeek { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:965 peek ->, src/game/kit.ts:965)
#[derive(Clone, Debug)]
pub struct Peek {
    pub value: crate::kit::KitValue,
    pub why: String,
    pub passed: String,
}
impl Peek {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Peek {
            value: crate::kit::KitValue::Text(String::new()),
            why: String::new(),
            passed: String::new(),
        }
    }
}
impl Default for Peek { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:980 kitTry ->, src/game/kit.ts:980)
#[derive(Clone, Debug)]
pub struct KitTry {
    pub ok: bool,
    pub value: crate::kit::KitValue,
    pub why: String,
    pub passed: String,
}
impl KitTry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitTry {
            ok: false,
            value: crate::kit::KitValue::Text(String::new()),
            why: String::new(),
            passed: String::new(),
        }
    }
}
impl Default for KitTry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1017 kitPreset ->, src/game/kit.ts:1017)
#[derive(Clone, Debug)]
pub struct KitPreset2 {
    pub ok: bool,
    pub why: String,
}
impl KitPreset2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitPreset2 {
            ok: false,
            why: String::new(),
        }
    }
}
impl Default for KitPreset2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1055 kitPastePeek ->, src/game/kit.ts:1055)
#[derive(Clone, Debug)]
pub struct KitPastePeek {
    pub card: crate::kit::MemberKit,
    pub changed: Vec<crate::kit::KitField>,
    pub why: String,
}
impl KitPastePeek {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitPastePeek {
            card: crate::kit::MemberKit::zeroed(),
            changed: Vec::new(),
            why: String::new(),
        }
    }
}
impl Default for KitPastePeek { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1056 pastePeek ->, src/game/kit.ts:1056)
#[derive(Clone, Debug)]
pub struct PastePeek {
    pub card: crate::kit::MemberKit,
    pub changed: Vec<crate::kit::KitField>,
    pub why: String,
}
impl PastePeek {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PastePeek {
            card: crate::kit::MemberKit::zeroed(),
            changed: Vec::new(),
            why: String::new(),
        }
    }
}
impl Default for PastePeek { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1061 kitPaste ->, src/game/kit.ts:1061)
#[derive(Clone, Debug)]
pub struct KitPaste {
    pub ok: bool,
    pub why: String,
    pub changed: Vec<crate::kit::KitField>,
}
impl KitPaste {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitPaste {
            ok: false,
            why: String::new(),
            changed: Vec::new(),
        }
    }
}
impl Default for KitPaste { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1173 kitPacks(e), src/game/kit.ts:1173)
#[derive(Clone, Copy, Debug)]
pub struct KitPacksE {
    pub packs: Option<crate::kit::KitModLessonRowPacks>,
}
impl KitPacksE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitPacksE {
            packs: None,
        }
    }
}
impl Default for KitPacksE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1181 kitLessonPage(row), src/game/kit.ts:1181)
#[derive(Clone, Copy, Debug)]
pub struct KitLessonPageRow {
    pub packs: Option<crate::kit::KitLessonPageRowPacks>,
}
impl KitLessonPageRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitLessonPageRow {
            packs: None,
        }
    }
}
impl Default for KitLessonPageRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1181 KitLessonPageRow.packs, src/game/kit.ts:1181)
#[derive(Clone, Copy, Debug)]
pub struct KitLessonPageRowPacks {
    pub frags: Option<f64>,
}
impl KitLessonPageRowPacks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitLessonPageRowPacks {
            frags: None,
        }
    }
}
impl Default for KitLessonPageRowPacks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1212 keptNow ->, src/game/kit.ts:1212)
#[derive(Clone, Copy, Debug)]
pub struct KeptNow {
    pub charges: Option<f64>,
    pub walls: Option<f64>,
}
impl KeptNow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeptNow {
            charges: None,
            walls: None,
        }
    }
}
impl Default for KeptNow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1378 kitPool(issued), src/game/kit.ts:1378)
#[derive(Clone, Copy, Debug)]
pub struct KitPoolIssued {
    pub bangs: f64,
    pub charges: f64,
}
impl KitPoolIssued {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitPoolIssued {
            bangs: 0.0,
            charges: 0.0,
        }
    }
}
impl Default for KitPoolIssued { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kit.ts:1378 kitPool ->, src/game/kit.ts:1378)
#[derive(Clone, Copy, Debug)]
pub struct KitPool {
    pub bangs: f64,
    pub charges: f64,
    pub frags: f64,
}
impl KitPool {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitPool {
            bangs: 0.0,
            charges: 0.0,
            frags: 0.0,
        }
    }
}
impl Default for KitPool { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct KitMod {
    /// (src/game/kit.ts:162 ruleOn)
    pub rule_on: bool,
    /// (src/game/kit.ts:164 bigOn)
    pub big_on: bool,
    /// (src/game/kit.ts:171 hand)
    pub hand: Option<crate::kit::KitHand>,
    /// (src/game/kit.ts:175 KIT)
    pub kit: crate::kit::KitModKIT,
    /// (src/game/kit.ts:353 team)
    pub team: f64,
    /// (src/game/kit.ts:423 loudYou)
    pub loud_you: Option<crate::kit::MemberKit>,
    /// (src/game/kit.ts:423 stealthYou)
    pub stealth_you: Option<crate::kit::MemberKit>,
    /// (src/game/kit.ts:443 staged)
    pub staged: bool,
    /// (src/game/kit.ts:499 dealOwn)
    pub deal_own: bool,
    /// (src/game/kit.ts:921 rowCache)
    pub row_cache: JsMap<String, Vec<crate::kit::KitRow>>,
    /// (src/game/kit.ts:1075 KIT_NOTE)
    pub kit_note: Vec<String>,
    /// (src/game/kit.ts:1148 issuedPage)
    pub issued_page: bool,
    /// (src/game/kit.ts:1148 issuing)
    pub issuing: crate::kit::KitModIssuing,
    /// (src/game/kit.ts:1180 lessonRow)
    pub lesson_row: Option<crate::kit::KitModLessonRow>,
    /// (src/game/kit.ts:1203 kept0)
    pub kept0: Option<crate::kit::KitModKept0>,
    /// (src/game/kit.ts:1211 keptWand)
    pub kept_wand: Vec<bool>,
    /// (src/game/kit.ts:1222 manOf)
    pub man_of: JsMap<GuardId, crate::kit::KitModManOfValue>,
    /// (src/game/kit.ts:1223 bareEyed)
    pub bare_eyed: JsSet<GuardId>,
    /// (src/game/kit.ts:1225 KIT_STAGED)
    pub kit_staged: crate::kit::KitModKITSTAGED,
    /// (src/game/kit.ts:1239 issuedPlayer)
    pub issued_player: JsMap<PlayerId, crate::kit::KitModIssuedPlayerValue>,
    /// (src/game/kit.ts:1242 youBare)
    pub you_bare: JsMap<PlayerId, crate::kit::KitModYouBareValue>,
    /// (src/game/kit.ts:1311 KIT_SOLO)
    pub kit_solo: crate::kit::KitModKITSOLO,
}
impl KitMod {
    pub fn blank() -> Self {
        KitMod {
            rule_on: true,
            big_on: false,
            hand: None,
            kit: { let card = |bangs: f64, charges: f64, smoke: f64, frags: f64, shells: f64| crate::kit::MemberKit { weapon: crate::kit::KitWeapon::Rifle, suppressed: true, light: true, nvg: true, bangs, charges, smoke, frags, walls: 0.0, shells, suit: crate::kit::SuitId::Issued, trim: crate::kit::TrimId::Issued, wand: false, cams: 0.0, bare: None, big: crate::kit::BigKit::None, primary: String::from("mk18"), secondary: String::from("g19"), side_can: true, rings: 0.0 }; crate::kit::KitModKIT { on: true, members: alloc::vec![card(1.0, 0.0, 1.0, 1.0, 1.0), card(2.0, 1.0, 0.0, 1.0, 0.0), card(1.0, 1.0, 0.0, 1.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0)] } },
            team: 4.0,
            loud_you: None,
            stealth_you: None,
            staged: false,
            deal_own: false,
            row_cache: JsMap::new(),
            kit_note: alloc::vec![String::from(""); 5],
            issued_page: false,
            issuing: crate::kit::KitModIssuing::Bool(false),
            lesson_row: None,
            kept0: None,
            kept_wand: Vec::new(),
            man_of: JsMap::new(),
            bare_eyed: JsSet::new(),
            kit_staged: crate::kit::KitModKITSTAGED { n: 0.0, ops: Vec::new() },
            issued_player: JsMap::new(),
            you_bare: JsMap::new(),
            kit_solo: crate::kit::KitModKITSOLO { on: false, stand: false },
        }
    }
}

/// (src/game/kit.ts:128 KIT_MEMBERS)
pub const KIT_MEMBERS: f64 = 5.0;

/// (src/game/kit.ts:130 KIT_LS)
pub const KIT_LS: &str = "twopointsix.kit";

/// (src/game/kit.ts:135 ISSUED_FRAGS: a table nothing writes)
pub fn issued_frags() -> Vec<f64> { alloc::vec![1.0, 1.0, 1.0, 0.0, 0.0] }

/// (src/game/kit.ts:137 ISSUED_SHELLS: a table nothing writes)
pub fn issued_shells() -> Vec<f64> { alloc::vec![1.0, 0.0, 0.0, 0.0, 0.0] }

/// (src/game/kit.ts:138 SHELLS_MAX)
pub const SHELLS_MAX: f64 = 4.0;

/// (src/game/kit.ts:140 SHELLS_MATE_MAX)
pub const SHELLS_MATE_MAX: f64 = 1.0;

/// (src/game/kit.ts:142 CAMS_MAX)
pub const CAMS_MAX: f64 = 3.0;

/// (src/game/kit.ts:144 RINGS_MAX)
pub const RINGS_MAX: f64 = 5.0;

/// (src/game/kit.ts:147 ISSUED: a table nothing writes)
pub fn issued() -> Vec<crate::kit::MemberKit> { { let card = |bangs: f64, charges: f64, smoke: f64, frags: f64, shells: f64| crate::kit::MemberKit { weapon: crate::kit::KitWeapon::Rifle, suppressed: true, light: true, nvg: true, bangs, charges, smoke, frags, walls: 0.0, shells, suit: crate::kit::SuitId::Issued, trim: crate::kit::TrimId::Issued, wand: false, cams: 0.0, bare: None, big: crate::kit::BigKit::None, primary: String::from("mk18"), secondary: String::from("g19"), side_can: true, rings: 0.0 }; alloc::vec![card(1.0, 0.0, 1.0, 1.0, 1.0), card(2.0, 1.0, 0.0, 1.0, 0.0), card(1.0, 1.0, 0.0, 1.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0)] } }

/// (src/game/kit.ts:154 FRAGS_MAX)
pub const FRAGS_MAX: f64 = 3.0;

/// (src/game/kit.ts:158 BUILT: a table nothing writes)
pub fn built() -> crate::kit::BUILT { crate::kit::BUILT { smoke: 4.0, bangs: 2.0, frags: 2.0, shells: 3.0 } }

/// (src/game/kit.ts:160 COUNTS0: a table nothing writes)
pub fn counts0() -> [String; 4] { [String::from("smoke"), String::from("bangs"), String::from("frags"), String::from("shells")] }

/// (src/game/kit.ts:178 SUITS: a table nothing writes)
pub fn suits() -> JsMap<String, crate::kit::SUITSValue> { { let mut t = JsMap::new(); t.set(String::from("black"), crate::kit::SUITSValue { label: String::from("black"), tint: [0.045, 0.05, 0.055] }); t.set(String::from("grey"), crate::kit::SUITSValue { label: String::from("urban grey"), tint: [0.16, 0.17, 0.18] }); t.set(String::from("navy"), crate::kit::SUITSValue { label: String::from("navy"), tint: [0.035, 0.055, 0.13] }); t.set(String::from("olive"), crate::kit::SUITSValue { label: String::from("olive drab"), tint: [0.1, 0.115, 0.055] }); t.set(String::from("jungle"), crate::kit::SUITSValue { label: String::from("jungle green"), tint: [0.045, 0.11, 0.055] }); t.set(String::from("khaki"), crate::kit::SUITSValue { label: String::from("khaki"), tint: [0.3, 0.26, 0.17] }); t.set(String::from("desert"), crate::kit::SUITSValue { label: String::from("desert tan"), tint: [0.4, 0.32, 0.2] }); t } }

/// (src/game/kit.ts:187 TRIMS: a table nothing writes)
pub fn trims() -> JsMap<String, crate::kit::TRIMSValue> { { let mut t = JsMap::new(); t.set(String::from("black"), crate::kit::TRIMSValue { label: String::from("black"), tint: [0.03, 0.03, 0.035] }); t.set(String::from("red"), crate::kit::TRIMSValue { label: String::from("red"), tint: [0.55, 0.07, 0.05] }); t.set(String::from("blue"), crate::kit::TRIMSValue { label: String::from("blue"), tint: [0.07, 0.2, 0.6] }); t.set(String::from("green"), crate::kit::TRIMSValue { label: String::from("green"), tint: [0.1, 0.32, 0.18] }); t.set(String::from("amber"), crate::kit::TRIMSValue { label: String::from("amber"), tint: [0.6, 0.38, 0.05] }); t.set(String::from("white"), crate::kit::TRIMSValue { label: String::from("white"), tint: [0.6, 0.6, 0.58] }); t } }

/// (src/game/kit.ts:197 CAN_MIXES: a table nothing writes)
pub fn can_mixes() -> JsMap<String, crate::kit::CANMIXESValue> { { let mut t = JsMap::new(); t.set(String::from("issued"), crate::kit::CANMIXESValue { label: String::from("4 smoke, 2 stun"), smoke: 4.0, stun: 2.0 }); t.set(String::from("smoke"), crate::kit::CANMIXESValue { label: String::from("6 smoke, no stun"), smoke: 6.0, stun: 0.0 }); t.set(String::from("even"), crate::kit::CANMIXESValue { label: String::from("3 smoke, 3 stun"), smoke: 3.0, stun: 3.0 }); t.set(String::from("stun"), crate::kit::CANMIXESValue { label: String::from("2 smoke, 4 stun"), smoke: 2.0, stun: 4.0 }); t } }

/// (src/game/kit.ts:203 BANGS_MAX)
pub const BANGS_MAX: f64 = 3.0;

/// (src/game/kit.ts:203 CHARGES_MAX)
pub const CHARGES_MAX: f64 = 2.0;

/// (src/game/kit.ts:203 WALLS_MAX)
pub const WALLS_MAX: f64 = 2.0;

/// (src/game/kit.ts:206 SMOKE_MAX)
pub const SMOKE_MAX: f64 = 6.0;

/// (src/game/kit.ts:206 STUN_MAX)
pub const STUN_MAX: f64 = 4.0;

/// (src/game/kit.ts:208 SMOKE_MATE_MAX)
pub const SMOKE_MATE_MAX: f64 = 2.0;

/// (src/game/kit.ts:250 SLOT_FIELD: a table nothing writes)
pub fn slot_field() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("smoke"), String::from("smoke")); t.set(String::from("stun"), String::from("bangs")); t.set(String::from("frag"), String::from("frags")); t.set(String::from("shell40"), String::from("shells")); t.set(String::from("doorCharge"), String::from("charges")); t.set(String::from("wallCharge"), String::from("walls")); t.set(String::from("shotgun"), String::from("shotgun")); t.set(String::from("wand"), String::from("wand")); t.set(String::from("stickyCam"), String::from("cams")); t.set(String::from("hammer"), String::from("hammer")); t.set(String::from("sponge"), String::from("rings")); t } }

/// (src/game/kit.ts:252 SLOT_AMOUNT: a table nothing writes)
pub fn slot_amount() -> JsMap<String, crate::FnField> { { let mut t = JsMap::new(); t.set(String::from("smoke"), crate::FnField); t.set(String::from("bangs"), crate::FnField); t.set(String::from("frags"), crate::FnField); t.set(String::from("shells"), crate::FnField); t.set(String::from("charges"), crate::FnField); t.set(String::from("walls"), crate::FnField); t.set(String::from("shotgun"), crate::FnField); t.set(String::from("hammer"), crate::FnField); t.set(String::from("cams"), crate::FnField); t.set(String::from("rings"), crate::FnField); t.set(String::from("wand"), crate::FnField); t } }

/// (src/game/kit.ts:261 SLOT_PUT: a table nothing writes)
pub fn slot_put() -> JsMap<String, crate::FnField> { { let mut t = JsMap::new(); t.set(String::from("smoke"), crate::FnField); t.set(String::from("bangs"), crate::FnField); t.set(String::from("frags"), crate::FnField); t.set(String::from("shells"), crate::FnField); t.set(String::from("charges"), crate::FnField); t.set(String::from("walls"), crate::FnField); t.set(String::from("wand"), crate::FnField); t.set(String::from("cams"), crate::FnField); t.set(String::from("rings"), crate::FnField); t.set(String::from("shotgun"), crate::FnField); t.set(String::from("hammer"), crate::FnField); t } }

/// (src/game/kit.ts:267 SLOT_WORD: a table nothing writes)
pub fn slot_word() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("smoke"), String::from("smoke canister")); t.set(String::from("bangs"), String::from("stun canister")); t.set(String::from("frags"), String::from("frag")); t.set(String::from("shells"), String::from("launcher grenade")); t.set(String::from("charges"), String::from("door charge")); t.set(String::from("walls"), String::from("wall charge")); t.set(String::from("shotgun"), String::from("shotgun")); t.set(String::from("wand"), String::from("optical wand")); t.set(String::from("cams"), String::from("sticky camera")); t.set(String::from("hammer"), String::from("hammer")); t.set(String::from("rings"), String::from("sponge round")); t } }

/// (src/game/kit.ts:270 OWN_MAX: a table nothing writes)
pub fn own_max() -> JsMap<String, [f64; 2]> { { let mut t = JsMap::new(); t.set(String::from("smoke"), [6.0, 2.0]); t.set(String::from("bangs"), [4.0, 3.0]); t.set(String::from("frags"), [3.0, 2.0]); t.set(String::from("shells"), [4.0, 1.0]); t.set(String::from("charges"), [2.0, 2.0]); t.set(String::from("walls"), [2.0, 2.0]); t.set(String::from("shotgun"), [1.0, 1.0]); t.set(String::from("wand"), [1.0, 1.0]); t.set(String::from("cams"), [3.0, 0.0]); t.set(String::from("hammer"), [1.0, 1.0]); t.set(String::from("rings"), [5.0, 0.0]); t } }

/// (src/game/kit.ts:272 TRIM: a table nothing writes)
pub fn trim() -> Vec<String> { alloc::vec![String::from("rings"), String::from("shells"), String::from("smoke"), String::from("frags"), String::from("charges"), String::from("bangs"), String::from("cams"), String::from("wand"), String::from("walls"), String::from("shotgun"), String::from("hammer")] }

/// (src/game/kit.ts:274 BIG_TRIM: a table nothing writes)
pub fn big_trim() -> Vec<String> { alloc::vec![String::from("hammer"), String::from("shotgun"), String::from("walls")] }

/// (src/game/kit.ts:277 SET_LOADOUT: a table nothing writes)
pub fn set_loadout() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("quiet"), String::from("quiet")); t.set(String::from("loud"), String::from("stun")); t } }

/// (src/game/kit.ts:381 MORE_BREAK)
pub const MORE_BREAK: bool = false;

/// (src/game/kit.ts:383 DEAL_TOOL: a table nothing writes)
pub fn deal_tool() -> Vec<(f64, String)> { alloc::vec![(2.0, String::from("shotgun")), (4.0, String::from("hammer"))] }

/// (src/game/kit.ts:400 STEALTH_BREAK)
pub const STEALTH_BREAK: bool = false;

/// (src/game/kit.ts:448 DEAL_BREAK)
pub const DEAL_BREAK: bool = false;

/// (src/game/kit.ts:450 DEALT: a table nothing writes)
pub fn dealt() -> [String; 4] { [String::from("charges"), String::from("bangs"), String::from("frags"), String::from("smoke")] }

/// (src/game/kit.ts:450 DEAL_TURN: a table nothing writes)
pub fn deal_turn() -> Vec<f64> { alloc::vec![1.0, 3.0, 2.0, 4.0] }

/// (src/game/kit.ts:771 GUN_NOTE: a table nothing writes)
pub fn gun_note() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("fn2000"), String::from("FN2000: small flash, but every 5.56 is heard as gunfire")); t.set(String::from("aks74u"), String::from("AKS-74U: bursts, a bright flash, tracers, the hard report")); t.set(String::from("fiveseven"), String::from("Five-seveN: the one gun the floor does not hear as gunfire")); t.set(String::from("guard9mm"), String::from("9 mm service pistol: loud, single shots")); t.set(String::from("m870"), String::from("M870: no suppressor fits it — every shell is heard as gunfire")); t.set(String::from("ak74m"), String::from("")); t.set(String::from("akms"), String::from("")); t.set(String::from("pkm"), String::from("")); t.set(String::from("mk12"), String::from("Mk 12: holds its group across a yard · single shots, slow to come round, no launcher")); t.set(String::from("mp7"), String::from("MP7: the floor does not hear it as gunfire · no reach past a room, no launcher")); t.set(String::from("mk46"), String::from("Mk 46: a hundred on the belt to hold a lane · slow to come round, long to reload, no launcher")); t.set(String::from("m1014"), String::from("M1014: a room, as fast as the trigger is pulled · opens no lock, no reach, no launcher")); t.set(String::from("hk45"), String::from("HK45: the quietest gun there is · a short magazine, a slower hand")); t.set(String::from("g18"), String::from("Glock 18: a long magazine on automatic · loud, and it climbs")); t.set(String::from("mk18bare"), String::from("FN2000 without its suppressor: a bright flash and the hard report")); t.set(String::from("g19bare"), String::from("Five-seveN without its suppressor: every round is heard as gunfire")); t } }

/// (src/game/kit.ts:790 GUN_SHORT: a table nothing writes)
pub fn gun_short() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("fn2000"), String::from("FN2000")); t.set(String::from("aks74u"), String::from("AKS-74U")); t.set(String::from("fiveseven"), String::from("Five-seveN")); t.set(String::from("guard9mm"), String::from("9 mm pistol")); t.set(String::from("m870"), String::from("M870 shotgun")); t.set(String::from("ak74m"), String::from("")); t.set(String::from("akms"), String::from("")); t.set(String::from("mk12"), String::from("Mk 12")); t.set(String::from("mp7"), String::from("MP7")); t.set(String::from("mk46"), String::from("Mk 46")); t.set(String::from("m1014"), String::from("M1014")); t.set(String::from("hk45"), String::from("HK45")); t.set(String::from("g18"), String::from("Glock 18")); t.set(String::from("pkm"), String::from("")); t.set(String::from("mk18bare"), String::from("FN2000")); t.set(String::from("g19bare"), String::from("Five-seveN")); t } }

/// (src/game/kit.ts:791 suitRow: a table nothing writes)
pub fn suit_row() -> crate::kit::KitRow { crate::kit::KitRow { field: crate::kit::KitField::Suit, label: String::from("Suit"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("grey")), crate::kit::KitValue::Text(String::from("navy")), crate::kit::KitValue::Text(String::from("olive")), crate::kit::KitValue::Text(String::from("jungle")), crate::kit::KitValue::Text(String::from("khaki")), crate::kit::KitValue::Text(String::from("desert"))], item: None } }

/// (src/game/kit.ts:792 trimRow: a table nothing writes)
pub fn trim_row() -> crate::kit::KitRow { crate::kit::KitRow { field: crate::kit::KitField::Trim, label: String::from("Trim"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("red")), crate::kit::KitValue::Text(String::from("blue")), crate::kit::KitValue::Text(String::from("green")), crate::kit::KitValue::Text(String::from("amber")), crate::kit::KitValue::Text(String::from("white"))], item: None } }

/// (src/game/kit.ts:794 YOU_GUNS: a table nothing writes)
pub fn you_guns() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("rifle"), String::from("FN2000 + Five-seveN")); t.set(String::from("pistol"), String::from("Five-seveN only")); t.set(String::from("shotgun"), String::from("M870 shotgun + Five-seveN")); t } }

/// (src/game/kit.ts:797 YOU_GUNS_CAN: a table nothing writes)
pub fn you_guns_can_table() -> crate::kit::YOUGUNSCAN { crate::kit::YOUGUNSCAN { pistol: Some(String::from("Suppressed Five-seveN")), rifle: None, shotgun: None } }

/// (src/game/kit.ts:802 SHOTGUN_NOTE)
pub const SHOTGUN_NOTE: &str = "drops a man across a room, poor down a corridor; shoots a lock off its door";

/// (src/game/kit.ts:807 cansRow: a table nothing writes)
pub fn cans_row() -> crate::kit::KitRow { crate::kit::KitRow { field: crate::kit::KitField::Cans, label: String::from("Cans"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("smoke")), crate::kit::KitValue::Text(String::from("even")), crate::kit::KitValue::Text(String::from("stun"))], item: None } }

/// (src/game/kit.ts:826 BIG_WORD: a table nothing writes)
pub fn big_word() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("none"), String::from("empty")); t.set(String::from("shotgun"), String::from("breaching shotgun")); t.set(String::from("hammer"), String::from("dynamic hammer")); t.set(String::from("wall"), String::from("wall charge")); t } }

/// (src/game/kit.ts:827 BIG_NOTE: a table nothing writes)
pub fn big_note() -> JsMap<String, [String; 2]> { { let mut t = JsMap::new(); t.set(String::from("none"), [String::from(""), String::from("")]); t.set(String::from("shotgun"), [String::from("shoots a lock off · heard as gunfire"), String::from("shoots a lock off · heard as gunfire")]); t.set(String::from("hammer"), [String::from("breaks a lock from either side · loud, not gunfire"), String::from("breaks a lock from either side · loud, not gunfire")]); t.set(String::from("wall"), [String::from("a door through a wall, where one is set"), String::from("a door through a wall, where one is set")]); t } }

/// (src/game/kit.ts:1007 KIT_PRESETS: a table nothing writes)
pub fn kit_presets() -> Vec<crate::kit::KITPRESETSItem> { alloc::vec![crate::kit::KITPRESETSItem { id: String::from("issued"), label: String::from("As issued"), line: String::from("the build as it was") }, crate::kit::KITPRESETSItem { id: String::from("quiet"), label: String::from("Quiet set"), line: String::from("suppressed pistol, no white light, black") }, crate::kit::KITPRESETSItem { id: String::from("loud"), label: String::from("Loud set"), line: String::from("bare carbine, light, more stun") }] }

/// (src/game/kit.ts:1034 PASTE_BREAK)
pub const PASTE_BREAK: bool = false;

/// (src/game/kit.ts:1171 WHOLE_BREAK)
pub const WHOLE_BREAK: bool = false;

/// (src/game/kit.ts:1171 PICKED_BREAK)
pub const PICKED_BREAK: bool = false;
