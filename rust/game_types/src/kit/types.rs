//! The types of `kit`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// KitRow.get and .set: a row that is not one field of the card reads and writes it itself. The arm says which
#[derive(Clone, Debug)]
pub enum KitRowGet {
    Mix,
    Big,
    Primary,
    Secondary,
    SlotCan { slot: crate::gunslots::GunSlot },
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SlotTable {
    pub v: f64,
    pub pouches: Vec<f64>,
    pub big: Vec<f64>,
    pub squad: Option<f64>,
    pub items: JsMap<String, crate::kit::SlotItem>,
    pub unknown: Vec<String>,
    pub loadouts: Vec<crate::kit::SlotLoadout>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SlotItem {
    pub id: String,
    pub pouches: f64,
    pub max: Vec<f64>,
    pub squad_max: Option<f64>,
    pub big: Option<bool>,
    pub holds: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SlotLoadout {
    pub id: String,
    pub name: String,
    pub counts: Vec<JsMap<String, f64>>,
    pub cards: Vec<JsMap<String, crate::kit::KitValue>>,
    pub missing: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum KitWeapon {
    #[default] Pistol,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum BigKit {
    #[default] Shotgun,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum SuitId {
    #[default] Issued,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum TrimId {
    #[default] Red,
    Blue,
    Issued,
    Black,
    Green,
    Amber,
    White,
}
impl TrimId {
    pub const fn as_str(self) -> &'static str {
        match self {
            TrimId::Red => "red",
            TrimId::Blue => "blue",
            TrimId::Issued => "issued",
            TrimId::Black => "black",
            TrimId::Green => "green",
            TrimId::Amber => "amber",
            TrimId::White => "white",
        }
    }
}

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

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitHand {
    pub rings: f64,
    pub cams: f64,
    pub wand: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum KitField {
    #[default] Smoke,
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

#[derive(Clone, Debug, ToBytes)]
pub enum KitValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitRow {
    pub field: crate::kit::KitField,
    pub label: String,
    pub values: Vec<crate::kit::KitValue>,
    pub item: Option<String>,
    /// kit.ts:767: a row that reads the card itself
    /// (code: in no bytes)
    #[bytes(skip)]
    pub get: Option<crate::kit::KitRowGet>,
}

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

#[derive(Clone, Copy, Debug, Default)]
pub struct BUILT {
    pub smoke: f64,
    pub bangs: f64,
    pub frags: f64,
    pub shells: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitModKIT {
    pub on: bool,
    pub members: Vec<crate::kit::MemberKit>,
}

#[derive(Clone, Debug, Default)]
pub struct SUITSValue {
    pub label: String,
    pub tint: V3,
}

#[derive(Clone, Debug, Default)]
pub struct TRIMSValue {
    pub label: String,
    pub tint: V3,
}

#[derive(Clone, Debug, Default)]
pub struct CANMIXESValue {
    pub label: String,
    pub smoke: f64,
    pub stun: f64,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum KitModIssuing {
    Bool(bool),
    Own,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitModKept0 {
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitModManOfValue {
    pub i: f64,
    pub kit: crate::kit::MemberKit,
    /// the card in a line, as he was staged with it: a HUD is told it each tick. Off the wire: a whole game read in says it again (`kit::kit_words_again`)
    pub words: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitModKITSTAGED {
    pub n: f64,
    pub ops: Vec<GuardId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitModYouBareValue {
    pub light: bool,
    pub nvg: bool,
}

#[derive(Clone, Debug, Default)]
pub struct StealthKept {
    pub stealth: Option<crate::kit::MemberKit>,
}

#[derive(Clone, Debug, Default)]
pub struct DealTable {
    pub kits: Vec<Vec<crate::kit::MemberKit>>,
    pub words: JsSet<String>,
}

#[derive(Clone, Debug, Default)]
pub struct KitCost {
    pub used: f64,
    pub total: f64,
    pub free: f64,
    pub big: Option<crate::kit::KitCostBig>,
}

#[derive(Clone, Debug, Default)]
pub struct KitCostLinesItem {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitCostBig {
    pub used: f64,
    pub total: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadCost {
    pub used: f64,
    pub total: f64,
    pub free: f64,
}

#[derive(Clone, Debug)]
pub enum LoadoutCards {
    Text(String),
    Array(Vec<crate::kit::MemberKit>),
}

#[derive(Clone, Debug, Default)]
pub struct KitLoadoutsItem {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct KitRowCost {
    pub pouches: f64,
    pub rate: String,
}

#[derive(Clone, Debug)]
pub struct KitPeek {
    pub value: crate::kit::KitValue,
    pub why: String,
}
impl Default for KitPeek {
    fn default() -> Self {
        KitPeek {
            value: crate::kit::KitValue::Text(String::new()),
            why: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Peek {
    pub value: crate::kit::KitValue,
    pub why: String,
    pub passed: String,
}
impl Default for Peek {
    fn default() -> Self {
        Peek {
            value: crate::kit::KitValue::Text(String::new()),
            why: String::new(),
            passed: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KitTry {
    pub ok: bool,
    pub why: String,
    pub passed: String,
}
impl Default for KitTry {
    fn default() -> Self {
        KitTry {
            ok: false,
            why: String::new(),
            passed: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct KitPreset2 {
    pub ok: bool,
    pub why: String,
}

#[derive(Clone, Debug, Default)]
pub struct KitPastePeek {
    pub changed: Vec<crate::kit::KitField>,
    pub why: String,
}

#[derive(Clone, Debug, Default)]
pub struct PastePeek {
    pub card: crate::kit::MemberKit,
    pub changed: Vec<crate::kit::KitField>,
    pub why: String,
}

#[derive(Clone, Debug, Default)]
pub struct KitPaste {
    pub ok: bool,
    pub why: String,
    pub changed: Vec<crate::kit::KitField>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitPoolIssued {
    pub bangs: f64,
    pub charges: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitPool {
    pub bangs: f64,
    pub charges: f64,
    pub frags: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct KitMod {
    pub hand: Option<crate::kit::KitHand>,
    pub kit: crate::kit::KitModKIT,
    pub team: f64,
    pub loud_you: Option<crate::kit::MemberKit>,
    pub stealth_you: Option<crate::kit::MemberKit>,
    pub staged: bool,
    pub deal_own: bool,
    pub row_cache: JsMap<String, Vec<crate::kit::KitRow>>,
    pub kit_note: Vec<String>,
    pub issued_page: bool,
    pub issuing: crate::kit::KitModIssuing,
    /// this page is a lesson's: its stagings wear the issue
    pub lesson_page: bool,
    pub kept0: Option<crate::kit::KitModKept0>,
    pub kept_wand: Vec<bool>,
    pub man_of: JsMap<GuardId, crate::kit::KitModManOfValue>,
    pub bare_eyed: JsSet<GuardId>,
    pub kit_staged: crate::kit::KitModKITSTAGED,
    pub issued_player: JsMap<PlayerId, crate::kit::KitModIssuedPlayerValue>,
    pub you_bare: JsMap<PlayerId, crate::kit::KitModYouBareValue>,
}
impl KitMod {
    pub fn blank() -> Self {
        KitMod {
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
            lesson_page: false,
            kept0: None,
            kept_wand: Vec::new(),
            man_of: JsMap::new(),
            bare_eyed: JsSet::new(),
            kit_staged: crate::kit::KitModKITSTAGED { n: 0.0, ops: Vec::new() },
            issued_player: JsMap::new(),
            you_bare: JsMap::new(),
        }
    }
}

pub const KIT_MEMBERS: f64 = 5.0;

pub fn issued_frags() -> Vec<f64> { alloc::vec![1.0, 1.0, 1.0, 0.0, 0.0] }

pub const SHELLS_MAX: f64 = 4.0;

pub const CAMS_MAX: f64 = 3.0;

pub const RINGS_MAX: f64 = 5.0;

pub fn issued() -> Vec<crate::kit::MemberKit> { { let card = |bangs: f64, charges: f64, smoke: f64, frags: f64, shells: f64| crate::kit::MemberKit { weapon: crate::kit::KitWeapon::Rifle, suppressed: true, light: true, nvg: true, bangs, charges, smoke, frags, walls: 0.0, shells, suit: crate::kit::SuitId::Issued, trim: crate::kit::TrimId::Issued, wand: false, cams: 0.0, bare: None, big: crate::kit::BigKit::None, primary: String::from("mk18"), secondary: String::from("g19"), side_can: true, rings: 0.0 }; alloc::vec![card(1.0, 0.0, 1.0, 1.0, 1.0), card(2.0, 1.0, 0.0, 1.0, 0.0), card(1.0, 1.0, 0.0, 1.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0)] } }

pub const FRAGS_MAX: f64 = 3.0;

pub fn built() -> crate::kit::BUILT { crate::kit::BUILT { smoke: 4.0, bangs: 2.0, frags: 2.0, shells: 3.0 } }

pub fn suits() -> JsMap<String, crate::kit::SUITSValue> { { let mut t = JsMap::new(); t.set(String::from("black"), crate::kit::SUITSValue { label: String::from("black"), tint: [0.045, 0.05, 0.055] }); t.set(String::from("grey"), crate::kit::SUITSValue { label: String::from("urban grey"), tint: [0.16, 0.17, 0.18] }); t.set(String::from("navy"), crate::kit::SUITSValue { label: String::from("navy"), tint: [0.035, 0.055, 0.13] }); t.set(String::from("olive"), crate::kit::SUITSValue { label: String::from("olive drab"), tint: [0.1, 0.115, 0.055] }); t.set(String::from("jungle"), crate::kit::SUITSValue { label: String::from("jungle green"), tint: [0.045, 0.11, 0.055] }); t.set(String::from("khaki"), crate::kit::SUITSValue { label: String::from("khaki"), tint: [0.3, 0.26, 0.17] }); t.set(String::from("desert"), crate::kit::SUITSValue { label: String::from("desert tan"), tint: [0.4, 0.32, 0.2] }); t } }

pub fn trims() -> JsMap<String, crate::kit::TRIMSValue> { { let mut t = JsMap::new(); t.set(String::from("black"), crate::kit::TRIMSValue { label: String::from("black"), tint: [0.03, 0.03, 0.035] }); t.set(String::from("red"), crate::kit::TRIMSValue { label: String::from("red"), tint: [0.55, 0.07, 0.05] }); t.set(String::from("blue"), crate::kit::TRIMSValue { label: String::from("blue"), tint: [0.07, 0.2, 0.6] }); t.set(String::from("green"), crate::kit::TRIMSValue { label: String::from("green"), tint: [0.1, 0.32, 0.18] }); t.set(String::from("amber"), crate::kit::TRIMSValue { label: String::from("amber"), tint: [0.6, 0.38, 0.05] }); t.set(String::from("white"), crate::kit::TRIMSValue { label: String::from("white"), tint: [0.6, 0.6, 0.58] }); t } }

pub fn can_mixes() -> JsMap<String, crate::kit::CANMIXESValue> { { let mut t = JsMap::new(); t.set(String::from("issued"), crate::kit::CANMIXESValue { label: String::from("4 smoke, 2 stun"), smoke: 4.0, stun: 2.0 }); t.set(String::from("smoke"), crate::kit::CANMIXESValue { label: String::from("6 smoke, no stun"), smoke: 6.0, stun: 0.0 }); t.set(String::from("even"), crate::kit::CANMIXESValue { label: String::from("3 smoke, 3 stun"), smoke: 3.0, stun: 3.0 }); t.set(String::from("stun"), crate::kit::CANMIXESValue { label: String::from("2 smoke, 4 stun"), smoke: 2.0, stun: 4.0 }); t } }

pub const BANGS_MAX: f64 = 3.0;

pub const CHARGES_MAX: f64 = 2.0;

pub const WALLS_MAX: f64 = 2.0;

pub const SMOKE_MAX: f64 = 6.0;

pub const STUN_MAX: f64 = 4.0;

pub const SMOKE_MATE_MAX: f64 = 2.0;

pub fn slot_amount() -> JsMap<String, crate::FnField> { { let mut t = JsMap::new(); t.set(String::from("smoke"), crate::FnField); t.set(String::from("bangs"), crate::FnField); t.set(String::from("frags"), crate::FnField); t.set(String::from("shells"), crate::FnField); t.set(String::from("charges"), crate::FnField); t.set(String::from("walls"), crate::FnField); t.set(String::from("shotgun"), crate::FnField); t.set(String::from("hammer"), crate::FnField); t.set(String::from("cams"), crate::FnField); t.set(String::from("rings"), crate::FnField); t.set(String::from("wand"), crate::FnField); t } }

pub fn slot_put() -> JsMap<String, crate::FnField> { { let mut t = JsMap::new(); t.set(String::from("smoke"), crate::FnField); t.set(String::from("bangs"), crate::FnField); t.set(String::from("frags"), crate::FnField); t.set(String::from("shells"), crate::FnField); t.set(String::from("charges"), crate::FnField); t.set(String::from("walls"), crate::FnField); t.set(String::from("wand"), crate::FnField); t.set(String::from("cams"), crate::FnField); t.set(String::from("rings"), crate::FnField); t.set(String::from("shotgun"), crate::FnField); t.set(String::from("hammer"), crate::FnField); t } }

pub fn slot_word() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("smoke"), String::from("smoke canister")); t.set(String::from("bangs"), String::from("stun canister")); t.set(String::from("frags"), String::from("frag")); t.set(String::from("shells"), String::from("launcher grenade")); t.set(String::from("charges"), String::from("door charge")); t.set(String::from("walls"), String::from("wall charge")); t.set(String::from("shotgun"), String::from("shotgun")); t.set(String::from("wand"), String::from("optical wand")); t.set(String::from("cams"), String::from("sticky camera")); t.set(String::from("hammer"), String::from("hammer")); t.set(String::from("rings"), String::from("sponge round")); t } }

pub fn own_max() -> JsMap<String, [f64; 2]> { { let mut t = JsMap::new(); t.set(String::from("smoke"), [6.0, 2.0]); t.set(String::from("bangs"), [4.0, 3.0]); t.set(String::from("frags"), [3.0, 2.0]); t.set(String::from("shells"), [4.0, 1.0]); t.set(String::from("charges"), [2.0, 2.0]); t.set(String::from("walls"), [2.0, 2.0]); t.set(String::from("shotgun"), [1.0, 1.0]); t.set(String::from("wand"), [1.0, 1.0]); t.set(String::from("cams"), [3.0, 0.0]); t.set(String::from("hammer"), [1.0, 1.0]); t.set(String::from("rings"), [5.0, 0.0]); t } }

pub fn trim() -> Vec<String> { alloc::vec![String::from("rings"), String::from("shells"), String::from("smoke"), String::from("frags"), String::from("charges"), String::from("bangs"), String::from("cams"), String::from("wand"), String::from("walls"), String::from("shotgun"), String::from("hammer")] }

pub fn big_trim() -> Vec<String> { alloc::vec![String::from("hammer"), String::from("shotgun"), String::from("walls")] }

pub fn set_loadout() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("quiet"), String::from("quiet")); t.set(String::from("loud"), String::from("stun")); t } }

pub fn deal_tool() -> Vec<(f64, String)> { alloc::vec![(2.0, String::from("shotgun")), (4.0, String::from("hammer"))] }

pub fn dealt() -> [String; 4] { [String::from("charges"), String::from("bangs"), String::from("frags"), String::from("smoke")] }

pub fn deal_turn() -> Vec<f64> { alloc::vec![1.0, 3.0, 2.0, 4.0] }

pub fn gun_short() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("fn2000"), String::from("FN2000")); t.set(String::from("aks74u"), String::from("AKS-74U")); t.set(String::from("fiveseven"), String::from("Five-seveN")); t.set(String::from("guard9mm"), String::from("9 mm pistol")); t.set(String::from("m870"), String::from("M870 shotgun")); t.set(String::from("ak74m"), String::from("")); t.set(String::from("akms"), String::from("")); t.set(String::from("mk12"), String::from("Mk 12")); t.set(String::from("mp7"), String::from("MP7")); t.set(String::from("mk46"), String::from("Mk 46")); t.set(String::from("m1014"), String::from("M1014")); t.set(String::from("hk45"), String::from("HK45")); t.set(String::from("g18"), String::from("Glock 18")); t.set(String::from("pkm"), String::from("")); t.set(String::from("mk18bare"), String::from("FN2000")); t.set(String::from("g19bare"), String::from("Five-seveN")); t } }

pub fn suit_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Suit, label: String::from("Suit"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("grey")), crate::kit::KitValue::Text(String::from("navy")), crate::kit::KitValue::Text(String::from("olive")), crate::kit::KitValue::Text(String::from("jungle")), crate::kit::KitValue::Text(String::from("khaki")), crate::kit::KitValue::Text(String::from("desert"))], item: None } }

pub fn trim_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Trim, label: String::from("Trim"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("red")), crate::kit::KitValue::Text(String::from("blue")), crate::kit::KitValue::Text(String::from("green")), crate::kit::KitValue::Text(String::from("amber")), crate::kit::KitValue::Text(String::from("white"))], item: None } }

pub fn you_guns() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("rifle"), String::from("FN2000 + Five-seveN")); t.set(String::from("pistol"), String::from("Five-seveN only")); t.set(String::from("shotgun"), String::from("M870 shotgun + Five-seveN")); t } }

pub fn cans_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Cans, label: String::from("Cans"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("smoke")), crate::kit::KitValue::Text(String::from("even")), crate::kit::KitValue::Text(String::from("stun"))], item: None } }

pub fn big_word() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("none"), String::from("empty")); t.set(String::from("shotgun"), String::from("breaching shotgun")); t.set(String::from("hammer"), String::from("dynamic hammer")); t.set(String::from("wall"), String::from("wall charge")); t } }

pub fn big_note() -> JsMap<String, [String; 2]> { { let mut t = JsMap::new(); t.set(String::from("none"), [String::from(""), String::from("")]); t.set(String::from("shotgun"), [String::from("shoots a lock off · heard as gunfire"), String::from("shoots a lock off · heard as gunfire")]); t.set(String::from("hammer"), [String::from("breaks a lock from either side · loud, not gunfire"), String::from("breaks a lock from either side · loud, not gunfire")]); t.set(String::from("wall"), [String::from("a door through a wall, where one is set"), String::from("a door through a wall, where one is set")]); t } }

