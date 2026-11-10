//! The types of `kit`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
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

/// a thing of which a card carries so many, in the order the table lists them
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum Thing {
    #[default] Smoke,
    Bangs,
    Frags,
    Shells,
    Charges,
    Walls,
    Shotgun,
    Hammer,
    Cams,
    Rings,
    Wand,
}

impl Thing {
    pub const ALL: [Thing; 11] = [Thing::Smoke, Thing::Bangs, Thing::Frags, Thing::Shells, Thing::Charges, Thing::Walls, Thing::Shotgun, Thing::Hammer, Thing::Cams, Thing::Rings, Thing::Wand];

    pub fn of(word: &str) -> Option<Thing> { Thing::ALL.into_iter().find(|t| t.as_str() == word) }

    /// what one of it is called, and the most you and a teammate may carry where nothing else limits it
    const fn row(self) -> (&'static str, [f64; 2]) {
        match self {
            Thing::Smoke => ("smoke canister", [6.0, 2.0]),
            Thing::Bangs => ("stun canister", [4.0, 3.0]),
            Thing::Frags => ("frag", [3.0, 2.0]),
            Thing::Shells => ("launcher grenade", [4.0, 1.0]),
            Thing::Charges => ("door charge", [2.0, 2.0]),
            Thing::Walls => ("wall charge", [2.0, 2.0]),
            Thing::Shotgun => ("shotgun", [1.0, 1.0]),
            Thing::Hammer => ("hammer", [1.0, 1.0]),
            Thing::Cams => ("sticky camera", [3.0, 0.0]),
            Thing::Rings => ("sponge round", [5.0, 0.0]),
            Thing::Wand => ("optical wand", [1.0, 1.0]),
        }
    }
    pub const fn name(self) -> &'static str { self.row().0 }
    pub const fn most(self, mate: bool) -> f64 { self.row().1[mate as usize] }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SlotTable {
    pub v: f64,
    pub pouches: Vec<f64>,
    pub big: Vec<f64>,
    pub squad: Option<f64>,
    pub items: JsMap<crate::kit::Thing, crate::kit::SlotItem>,
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
    pub counts: Vec<JsMap<crate::kit::Thing, f64>>,
    pub cards: Vec<JsMap<String, crate::kit::KitValue>>,
    pub missing: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum KitWeapon {
    #[default] Pistol,
    Rifle,
    Shotgun,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum BigKit {
    #[default] Shotgun,
    None,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum BigPick {
    Shotgun,
    None,
    Hammer,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum TrimId {
    #[default] Red,
    Blue,
    Issued,
    Black,
    Green,
    Amber,
    White,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum CanMix {
    Smoke,
    Stun,
    Issued,
    Even,
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

impl MemberKit {
    /// his count of a thing that is counted; none for a tool he has or has not
    pub fn n_mut(&mut self, k: Thing) -> Option<&mut f64> {
        Some(match k {
            Thing::Smoke => &mut self.smoke,
            Thing::Bangs => &mut self.bangs,
            Thing::Frags => &mut self.frags,
            Thing::Shells => &mut self.shells,
            Thing::Charges => &mut self.charges,
            Thing::Walls => &mut self.walls,
            Thing::Cams => &mut self.cams,
            Thing::Rings => &mut self.rings,
            Thing::Shotgun | Thing::Hammer | Thing::Wand => return None,
        })
    }
    pub fn n(&self, k: Thing) -> f64 {
        match k {
            Thing::Smoke => self.smoke,
            Thing::Bangs => self.bangs,
            Thing::Frags => self.frags,
            Thing::Shells => self.shells,
            Thing::Charges => self.charges,
            Thing::Walls => self.walls,
            Thing::Cams => self.cams,
            Thing::Rings => self.rings,
            Thing::Shotgun | Thing::Hammer | Thing::Wand => 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitHand {
    pub rings: f64,
    pub cams: f64,
    pub wand: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
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
    pub item: Option<crate::kit::Thing>,
    /// A row that reads the card itself
    /// (code: in no bytes)
    #[bytes(skip)]
    pub get: Option<crate::kit::KitRowGet>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum KitPreset {
    Quiet,
    Issued,
    Loud,
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct KitMod {
    pub hand: Option<crate::kit::KitHand>,
    #[is({ let card = |bangs: f64, charges: f64, smoke: f64, frags: f64, shells: f64| crate::kit::MemberKit { weapon: crate::kit::KitWeapon::Rifle, suppressed: true, light: true, nvg: true, bangs, charges, smoke, frags, walls: 0.0, shells, suit: crate::kit::SuitId::Issued, trim: crate::kit::TrimId::Issued, wand: false, cams: 0.0, bare: None, big: crate::kit::BigKit::None, primary: String::from("mk18"), secondary: String::from("g19"), side_can: true, rings: 0.0 }; crate::kit::KitModKIT { on: true, members: alloc::vec![card(1.0, 0.0, 1.0, 1.0, 1.0), card(2.0, 1.0, 0.0, 1.0, 0.0), card(1.0, 1.0, 0.0, 1.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0)] } })] pub kit: crate::kit::KitModKIT,
    #[is(4.0)] pub team: f64,
    pub loud_you: Option<crate::kit::MemberKit>,
    pub stealth_you: Option<crate::kit::MemberKit>,
    pub staged: bool,
    pub deal_own: bool,
    pub row_cache: JsMap<String, Vec<crate::kit::KitRow>>,
    #[is(alloc::vec![String::from(""); 5])] pub kit_note: Vec<String>,
    pub issued_page: bool,
    #[is(crate::kit::KitModIssuing::Bool(false))] pub issuing: crate::kit::KitModIssuing,
    /// this page is a lesson's: its stagings wear the issue
    pub lesson_page: bool,
    pub kept0: Option<crate::kit::KitModKept0>,
    pub kept_wand: Vec<bool>,
    pub man_of: JsMap<GuardId, crate::kit::KitModManOfValue>,
    pub bare_eyed: JsSet<GuardId>,
    #[is(crate::kit::KitModKITSTAGED { n: 0.0, ops: Vec::new() })] pub kit_staged: crate::kit::KitModKITSTAGED,
    pub issued_player: JsMap<PlayerId, crate::kit::KitModIssuedPlayerValue>,
    pub you_bare: JsMap<PlayerId, crate::kit::KitModYouBareValue>,
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





pub fn trim() -> Vec<crate::kit::Thing> { alloc::vec![crate::kit::Thing::Rings, crate::kit::Thing::Shells, crate::kit::Thing::Smoke, crate::kit::Thing::Frags, crate::kit::Thing::Charges, crate::kit::Thing::Bangs, crate::kit::Thing::Cams, crate::kit::Thing::Wand, crate::kit::Thing::Walls, crate::kit::Thing::Shotgun, crate::kit::Thing::Hammer] }

pub fn big_trim() -> Vec<crate::kit::Thing> { alloc::vec![crate::kit::Thing::Hammer, crate::kit::Thing::Shotgun, crate::kit::Thing::Walls] }

pub fn set_loadout() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("quiet"), String::from("quiet")); t.set(String::from("loud"), String::from("stun")); t } }

pub fn deal_tool() -> Vec<(f64, crate::kit::Thing)> { alloc::vec![(2.0, crate::kit::Thing::Shotgun), (4.0, crate::kit::Thing::Hammer)] }

pub fn dealt() -> [crate::kit::Thing; 4] { [crate::kit::Thing::Charges, crate::kit::Thing::Bangs, crate::kit::Thing::Frags, crate::kit::Thing::Smoke] }

pub fn deal_turn() -> Vec<f64> { alloc::vec![1.0, 3.0, 2.0, 4.0] }

pub fn gun_short() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("fn2000"), String::from("FN2000")); t.set(String::from("aks74u"), String::from("AKS-74U")); t.set(String::from("fiveseven"), String::from("Five-seveN")); t.set(String::from("guard9mm"), String::from("9 mm pistol")); t.set(String::from("m870"), String::from("M870 shotgun")); t.set(String::from("ak74m"), String::from("")); t.set(String::from("akms"), String::from("")); t.set(String::from("mk12"), String::from("Mk 12")); t.set(String::from("mp7"), String::from("MP7")); t.set(String::from("mk46"), String::from("Mk 46")); t.set(String::from("m1014"), String::from("M1014")); t.set(String::from("hk45"), String::from("HK45")); t.set(String::from("g18"), String::from("Glock 18")); t.set(String::from("pkm"), String::from("")); t.set(String::from("mk18bare"), String::from("FN2000")); t.set(String::from("g19bare"), String::from("Five-seveN")); t } }

pub fn suit_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Suit, label: String::from("Suit"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("grey")), crate::kit::KitValue::Text(String::from("navy")), crate::kit::KitValue::Text(String::from("olive")), crate::kit::KitValue::Text(String::from("jungle")), crate::kit::KitValue::Text(String::from("khaki")), crate::kit::KitValue::Text(String::from("desert"))], item: None } }

pub fn trim_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Trim, label: String::from("Trim"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("red")), crate::kit::KitValue::Text(String::from("blue")), crate::kit::KitValue::Text(String::from("green")), crate::kit::KitValue::Text(String::from("amber")), crate::kit::KitValue::Text(String::from("white"))], item: None } }

pub fn you_guns() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("rifle"), String::from("FN2000 + Five-seveN")); t.set(String::from("pistol"), String::from("Five-seveN only")); t.set(String::from("shotgun"), String::from("M870 shotgun + Five-seveN")); t } }

pub fn cans_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Cans, label: String::from("Cans"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("smoke")), crate::kit::KitValue::Text(String::from("even")), crate::kit::KitValue::Text(String::from("stun"))], item: None } }

pub fn big_word() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("none"), String::from("empty")); t.set(String::from("shotgun"), String::from("breaching shotgun")); t.set(String::from("hammer"), String::from("dynamic hammer")); t.set(String::from("wall"), String::from("wall charge")); t } }

pub fn big_note() -> JsMap<String, [String; 2]> { { let mut t = JsMap::new(); t.set(String::from("none"), [String::from(""), String::from("")]); t.set(String::from("shotgun"), [String::from("shoots a lock off · heard as gunfire"), String::from("shoots a lock off · heard as gunfire")]); t.set(String::from("hammer"), [String::from("breaks a lock from either side · loud, not gunfire"), String::from("breaks a lock from either side · loud, not gunfire")]); t.set(String::from("wall"), [String::from("a door through a wall, where one is set"), String::from("a door through a wall, where one is set")]); t } }

