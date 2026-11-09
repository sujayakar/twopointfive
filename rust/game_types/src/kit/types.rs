//! The types of `kit`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
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
    pub item: Option<String>,
    /// kit.ts:767: a row that reads the card itself
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
    /// (src/game/kit.ts:171 hand)
    pub hand: Option<crate::kit::KitHand>,
    /// (src/game/kit.ts:175 KIT)
    #[is({ let card = |bangs: f64, charges: f64, smoke: f64, frags: f64, shells: f64| crate::kit::MemberKit { weapon: crate::kit::KitWeapon::Rifle, suppressed: true, light: true, nvg: true, bangs, charges, smoke, frags, walls: 0.0, shells, suit: crate::kit::SuitId::Issued, trim: crate::kit::TrimId::Issued, wand: false, cams: 0.0, bare: None, big: crate::kit::BigKit::None, primary: String::from("mk18"), secondary: String::from("g19"), side_can: true, rings: 0.0 }; crate::kit::KitModKIT { on: true, members: alloc::vec![card(1.0, 0.0, 1.0, 1.0, 1.0), card(2.0, 1.0, 0.0, 1.0, 0.0), card(1.0, 1.0, 0.0, 1.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0), card(0.0, 0.0, 0.0, 0.0, 0.0)] } })] pub kit: crate::kit::KitModKIT,
    /// (src/game/kit.ts:353 team)
    #[is(4.0)] pub team: f64,
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
    #[is(alloc::vec![String::from(""); 5])] pub kit_note: Vec<String>,
    /// (src/game/kit.ts:1148 issuedPage)
    pub issued_page: bool,
    /// (src/game/kit.ts:1148 issuing)
    #[is(crate::kit::KitModIssuing::Bool(false))] pub issuing: crate::kit::KitModIssuing,
    /// this page is a lesson's: its stagings wear the issue
    pub lesson_page: bool,
    /// (src/game/kit.ts:1203 kept0)
    pub kept0: Option<crate::kit::KitModKept0>,
    /// (src/game/kit.ts:1211 keptWand)
    pub kept_wand: Vec<bool>,
    /// (src/game/kit.ts:1222 manOf)
    pub man_of: JsMap<GuardId, crate::kit::KitModManOfValue>,
    /// (src/game/kit.ts:1223 bareEyed)
    pub bare_eyed: JsSet<GuardId>,
    /// (src/game/kit.ts:1225 KIT_STAGED)
    #[is(crate::kit::KitModKITSTAGED { n: 0.0, ops: Vec::new() })] pub kit_staged: crate::kit::KitModKITSTAGED,
    /// (src/game/kit.ts:1239 issuedPlayer)
    pub issued_player: JsMap<PlayerId, crate::kit::KitModIssuedPlayerValue>,
    /// (src/game/kit.ts:1242 youBare)
    pub you_bare: JsMap<PlayerId, crate::kit::KitModYouBareValue>,
}

/// (src/game/kit.ts:128 KIT_MEMBERS)
pub const KIT_MEMBERS: f64 = 5.0;

/// (src/game/kit.ts:135 ISSUED_FRAGS: a table nothing writes)
pub fn issued_frags() -> Vec<f64> { alloc::vec![1.0, 1.0, 1.0, 0.0, 0.0] }

/// (src/game/kit.ts:138 SHELLS_MAX)
pub const SHELLS_MAX: f64 = 4.0;

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

/// (src/game/kit.ts:383 DEAL_TOOL: a table nothing writes)
pub fn deal_tool() -> Vec<(f64, String)> { alloc::vec![(2.0, String::from("shotgun")), (4.0, String::from("hammer"))] }

/// (src/game/kit.ts:450 DEALT: a table nothing writes)
pub fn dealt() -> [String; 4] { [String::from("charges"), String::from("bangs"), String::from("frags"), String::from("smoke")] }

/// (src/game/kit.ts:450 DEAL_TURN: a table nothing writes)
pub fn deal_turn() -> Vec<f64> { alloc::vec![1.0, 3.0, 2.0, 4.0] }

/// (src/game/kit.ts:790 GUN_SHORT: a table nothing writes)
pub fn gun_short() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("fn2000"), String::from("FN2000")); t.set(String::from("aks74u"), String::from("AKS-74U")); t.set(String::from("fiveseven"), String::from("Five-seveN")); t.set(String::from("guard9mm"), String::from("9 mm pistol")); t.set(String::from("m870"), String::from("M870 shotgun")); t.set(String::from("ak74m"), String::from("")); t.set(String::from("akms"), String::from("")); t.set(String::from("mk12"), String::from("Mk 12")); t.set(String::from("mp7"), String::from("MP7")); t.set(String::from("mk46"), String::from("Mk 46")); t.set(String::from("m1014"), String::from("M1014")); t.set(String::from("hk45"), String::from("HK45")); t.set(String::from("g18"), String::from("Glock 18")); t.set(String::from("pkm"), String::from("")); t.set(String::from("mk18bare"), String::from("FN2000")); t.set(String::from("g19bare"), String::from("Five-seveN")); t } }

/// (src/game/kit.ts:791 suitRow: a table nothing writes)
pub fn suit_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Suit, label: String::from("Suit"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("grey")), crate::kit::KitValue::Text(String::from("navy")), crate::kit::KitValue::Text(String::from("olive")), crate::kit::KitValue::Text(String::from("jungle")), crate::kit::KitValue::Text(String::from("khaki")), crate::kit::KitValue::Text(String::from("desert"))], item: None } }

/// (src/game/kit.ts:792 trimRow: a table nothing writes)
pub fn trim_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Trim, label: String::from("Trim"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("black")), crate::kit::KitValue::Text(String::from("red")), crate::kit::KitValue::Text(String::from("blue")), crate::kit::KitValue::Text(String::from("green")), crate::kit::KitValue::Text(String::from("amber")), crate::kit::KitValue::Text(String::from("white"))], item: None } }

/// (src/game/kit.ts:794 YOU_GUNS: a table nothing writes)
pub fn you_guns() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("rifle"), String::from("FN2000 + Five-seveN")); t.set(String::from("pistol"), String::from("Five-seveN only")); t.set(String::from("shotgun"), String::from("M870 shotgun + Five-seveN")); t } }

/// (src/game/kit.ts:807 cansRow: a table nothing writes)
pub fn cans_row() -> crate::kit::KitRow { crate::kit::KitRow { get: None, field: crate::kit::KitField::Cans, label: String::from("Cans"), values: alloc::vec![crate::kit::KitValue::Text(String::from("issued")), crate::kit::KitValue::Text(String::from("smoke")), crate::kit::KitValue::Text(String::from("even")), crate::kit::KitValue::Text(String::from("stun"))], item: None } }

/// (src/game/kit.ts:826 BIG_WORD: a table nothing writes)
pub fn big_word() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("none"), String::from("empty")); t.set(String::from("shotgun"), String::from("breaching shotgun")); t.set(String::from("hammer"), String::from("dynamic hammer")); t.set(String::from("wall"), String::from("wall charge")); t } }

/// (src/game/kit.ts:827 BIG_NOTE: a table nothing writes)
pub fn big_note() -> JsMap<String, [String; 2]> { { let mut t = JsMap::new(); t.set(String::from("none"), [String::from(""), String::from("")]); t.set(String::from("shotgun"), [String::from("shoots a lock off · heard as gunfire"), String::from("shoots a lock off · heard as gunfire")]); t.set(String::from("hammer"), [String::from("breaks a lock from either side · loud, not gunfire"), String::from("breaks a lock from either side · loud, not gunfire")]); t.set(String::from("wall"), [String::from("a door through a wall, where one is set"), String::from("a door through a wall, where one is set")]); t } }

