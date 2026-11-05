//! The types of `frag`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: BlastReport, src/game/frag.ts:206)
#[derive(Clone, Debug)]
pub struct BlastReport {
    pub at: V3,
    pub who: f64,
    pub hits: Vec<crate::frag::BlastHit>,
    pub killed: f64,
    pub finished: f64,
    pub wounded: f64,
    pub shielded: f64,
    pub stunned: f64,
    pub player: Option<crate::frag::BlastResult>,
}
impl BlastReport {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastReport {
            at: [0.0; 3],
            who: 0.0,
            hits: Vec::new(),
            killed: 0.0,
            finished: 0.0,
            wounded: 0.0,
            shielded: 0.0,
            stunned: 0.0,
            player: None,
        }
    }
}
impl Default for BlastReport { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BlastHit, src/game/frag.ts:205)
#[derive(Clone, Debug)]
pub struct BlastHit {
    pub id: f64,
    pub name: String,
    pub team: crate::teams::Team,
    pub d: f64,
    pub exposed: f64,
    pub result: crate::frag::BlastResult,
    pub down: Option<bool>,
    pub behind: Option<crate::frag::BlastHitBehind>,
}
impl BlastHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastHit {
            id: 0.0,
            name: String::new(),
            team: crate::teams::Team::Guard,
            d: 0.0,
            exposed: 0.0,
            result: crate::frag::BlastResult::Wounded,
            down: None,
            behind: None,
        }
    }
}
impl Default for BlastHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BlastResult {
    Wounded,
    Killed,
    Shielded,
    Stunned,
}
impl BlastResult {
    pub const fn as_str(self) -> &'static str {
        match self {
            BlastResult::Wounded => "wounded",
            BlastResult::Killed => "killed",
            BlastResult::Shielded => "shielded",
            BlastResult::Stunned => "stunned",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BlastHitBehind {
    Door,
    Cover,
}
impl BlastHitBehind {
    pub const fn as_str(self) -> &'static str {
        match self {
            BlastHitBehind::Door => "door",
            BlastHitBehind::Cover => "cover",
        }
    }
}

/// (TypeScript: BlastSpec, src/game/frag.ts:191)
#[derive(Clone, Debug)]
pub struct BlastSpec {
    pub who: f64,
    pub lethal_m: Option<f64>,
    pub wound_m: Option<f64>,
    pub serial: Option<f64>,
    pub name: Option<String>,
    pub fx: Option<bool>,
    pub destr: Option<crate::frag::BlastSpecDestr>,
}
impl BlastSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastSpec {
            who: 0.0,
            lethal_m: None,
            wound_m: None,
            serial: None,
            name: None,
            fx: None,
            destr: None,
        }
    }
}
impl Default for BlastSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:202 BlastSpec.destr, src/game/frag.ts:202)
#[derive(Clone, Copy, Debug)]
pub struct BlastSpecDestr {
    pub radius: Option<f64>,
    pub power: Option<f64>,
}
impl BlastSpecDestr {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastSpecDestr {
            radius: None,
            power: None,
        }
    }
}
impl Default for BlastSpecDestr { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassView {
    Blown,
    Wary,
    Counted,
    Whole,
}
impl GlassView {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassView::Blown => "blown",
            GlassView::Wary => "wary",
            GlassView::Counted => "counted",
            GlassView::Whole => "whole",
        }
    }
}

/// (TypeScript: Live, src/game/frag.ts:446)
#[derive(Clone, Debug)]
pub struct Live {
    pub it: ItemId,
    pub side: crate::teams::Team,
    pub knows: JsMap<GuardId, f64>,
    pub shouted: JsSet<crate::teams::Team>,
    pub flee: JsMap<GuardId, Option<crate::game::GuardScript>>,
    pub via: Option<JsSet<GuardId>>,
    pub flee_at: JsMap<GuardId, V3>,
    pub deal_t: JsMap<GuardId, f64>,
    pub counted: JsSet<GuardId>,
    pub cued: bool,
    pub rest: Option<crate::frag::Rest>,
    pub made: Option<JsMap<GuardId, crate::frag::LiveMadeValue>>,
}
impl Live {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Live {
            it: ItemId(0),
            side: crate::teams::Team::Guard,
            knows: JsMap::new(),
            shouted: JsSet::new(),
            flee: JsMap::new(),
            via: None,
            flee_at: JsMap::new(),
            deal_t: JsMap::new(),
            counted: JsSet::new(),
            cued: false,
            rest: None,
            made: None,
        }
    }
}
impl Default for Live { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Rest, src/game/frag.ts:469)
#[derive(Clone, Debug)]
pub struct Rest {
    pub t: f64,
    pub still: bool,
    pub at: V3,
    pub path: Vec<V3>,
}
impl Rest {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rest {
            t: 0.0,
            still: false,
            at: [0.0; 3],
            path: Vec::new(),
        }
    }
}
impl Default for Rest { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:446 Live.made, src/game/frag.ts:446)
#[derive(Clone, Copy, Debug)]
pub struct LiveMadeValue {
    pub at: V3,
    pub t: f64,
}
impl LiveMadeValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LiveMadeValue {
            at: [0.0; 3],
            t: 0.0,
        }
    }
}
impl Default for LiveMadeValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Carrier, src/game/frag.ts:813)
#[derive(Clone, Debug)]
pub struct Carrier {
    pub gd: GuardId,
    pub frags: f64,
    pub blocked_t: f64,
    pub bunch_t: f64,
    pub try_at: f64,
    pub wind: Option<crate::frag::CarrierWind>,
    pub away: Option<f64>,
    pub spots: Option<crate::frag::CarrierSpots>,
}
impl Carrier {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Carrier {
            gd: GuardId(0),
            frags: 0.0,
            blocked_t: 0.0,
            bunch_t: 0.0,
            try_at: 0.0,
            wind: None,
            away: None,
            spots: None,
        }
    }
}
impl Default for Carrier { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:813 Carrier.wind, src/game/frag.ts:813)
#[derive(Clone, Copy, Debug)]
pub struct CarrierWind {
    pub until: f64,
    pub at: V3,
    pub swung: bool,
    pub last_t: f64,
    pub hand: Option<V3>,
    pub toward: Option<V3>,
}
impl CarrierWind {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CarrierWind {
            until: 0.0,
            at: [0.0; 3],
            swung: false,
            last_t: 0.0,
            hand: None,
            toward: None,
        }
    }
}
impl Default for CarrierWind { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:813 Carrier.spots, src/game/frag.ts:813)
#[derive(Clone, Debug)]
pub struct CarrierSpots {
    pub at: V3,
    pub from: V3,
    pub t: f64,
    pub list: Vec<V3>,
}
impl CarrierSpots {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CarrierSpots {
            at: [0.0; 3],
            from: [0.0; 3],
            t: 0.0,
            list: Vec::new(),
        }
    }
}
impl Default for CarrierSpots { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DefBook, src/game/frag.ts:814)
#[derive(Clone, Debug)]
pub struct DefBook {
    pub men: Vec<GuardId>,
    pub carriers: Vec<crate::frag::Carrier>,
    pub last_t: f64,
    pub serial: f64,
    pub thrown: f64,
}
impl DefBook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefBook {
            men: Vec::new(),
            carriers: Vec::new(),
            last_t: 0.0,
            serial: 0.0,
            thrown: 0.0,
        }
    }
}
impl Default for DefBook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:267 WALLS_FOR, src/game/frag.ts:267)
#[derive(Clone, Debug)]
pub struct FragModWALLSFORValue {
    pub time: f64,
    pub serial: f64,
    pub at: JsMap<String, Vec<crate::wallbreak::WallOpening>>,
}
impl FragModWALLSFORValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FragModWALLSFORValue {
            time: 0.0,
            serial: 0.0,
            at: JsMap::new(),
        }
    }
}
impl Default for FragModWALLSFORValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:280 TORN, src/game/frag.ts:280)
#[derive(Clone, Debug)]
pub struct FragModTORNValue {
    pub time: f64,
    pub doors: Vec<crate::frag::FragModTORNValueDoorsItem>,
}
impl FragModTORNValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FragModTORNValue {
            time: 0.0,
            doors: Vec::new(),
        }
    }
}
impl Default for FragModTORNValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:280 FragModTORNValue.doors, src/game/frag.ts:280)
#[derive(Clone, Copy, Debug)]
pub struct FragModTORNValueDoorsItem {
    pub off: bool,
    pub box_: BoxId,
}
impl FragModTORNValueDoorsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FragModTORNValueDoorsItem {
            off: false,
            box_: BoxId(0),
        }
    }
}
impl Default for FragModTORNValueDoorsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:470 rollOn ->, src/game/frag.ts:470)
#[derive(Clone, Debug)]
pub struct RollOn {
    pub at: V3,
    pub path: Vec<V3>,
}
impl RollOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RollOn {
            at: [0.0; 3],
            path: Vec::new(),
        }
    }
}
impl Default for RollOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:501 restOfThrow ->, src/game/frag.ts:501)
#[derive(Clone, Debug)]
pub struct RestOfThrow {
    pub at: V3,
    pub path: Vec<V3>,
}
impl RestOfThrow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RestOfThrow {
            at: [0.0; 3],
            path: Vec::new(),
        }
    }
}
impl Default for RestOfThrow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:542 fleeSpot(c), src/game/frag.ts:542)
#[derive(Clone, Copy, Debug)]
pub struct FleeSpotC {
    pub pos: V3,
}
impl FleeSpotC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FleeSpotC {
            pos: [0.0; 3],
        }
    }
}
impl Default for FleeSpotC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:542 fleeSpot(wise), src/game/frag.ts:542)
#[derive(Clone, Debug)]
pub struct FleeSpotWise {
    pub path: Vec<V3>,
    pub keep: Option<V3>,
    pub not: Option<V3>,
    pub apart: Option<crate::frag::FleeSpotWiseApart>,
}
impl FleeSpotWise {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FleeSpotWise {
            path: Vec::new(),
            keep: None,
            not: None,
            apart: None,
        }
    }
}
impl Default for FleeSpotWise { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:542 FleeSpotWise.apart, src/game/frag.ts:542)
#[derive(Clone, Debug)]
pub struct FleeSpotWiseApart {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}
impl FleeSpotWiseApart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FleeSpotWiseApart {
            stay: Vec::new(),
            go: Vec::new(),
        }
    }
}
impl Default for FleeSpotWiseApart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:619 fleeByDoor(c), src/game/frag.ts:619)
#[derive(Clone, Copy, Debug)]
pub struct FleeByDoorC {
    pub pos: V3,
}
impl FleeByDoorC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FleeByDoorC {
            pos: [0.0; 3],
        }
    }
}
impl Default for FleeByDoorC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:619 fleeByDoor(wise), src/game/frag.ts:619)
#[derive(Clone, Debug)]
pub struct FleeByDoorWise {
    pub path: Vec<V3>,
    pub keep: Option<V3>,
    pub not: Option<V3>,
    pub apart: Option<crate::frag::FleeByDoorWiseApart>,
}
impl FleeByDoorWise {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FleeByDoorWise {
            path: Vec::new(),
            keep: None,
            not: None,
            apart: None,
        }
    }
}
impl Default for FleeByDoorWise { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:619 FleeByDoorWise.apart, src/game/frag.ts:619)
#[derive(Clone, Debug)]
pub struct FleeByDoorWiseApart {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}
impl FleeByDoorWiseApart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FleeByDoorWiseApart {
            stay: Vec::new(),
            go: Vec::new(),
        }
    }
}
impl Default for FleeByDoorWiseApart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:619 fleeByDoor ->, src/game/frag.ts:619)
#[derive(Clone, Copy, Debug)]
pub struct FleeByDoor {
    pub goal: V3,
    pub via: bool,
}
impl FleeByDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FleeByDoor {
            goal: [0.0; 3],
            via: false,
        }
    }
}
impl Default for FleeByDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:683 takenBy ->, src/game/frag.ts:683)
#[derive(Clone, Debug)]
pub struct TakenBy {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}
impl TakenBy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TakenBy {
            stay: Vec::new(),
            go: Vec::new(),
        }
    }
}
impl Default for TakenBy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:778 ownBang(e), src/game/frag.ts:778)
#[derive(Clone, Copy, Debug)]
pub struct OwnBangE {
    pub frag: Option<bool>,
    pub who: Option<f64>,
}
impl OwnBangE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OwnBangE {
            frag: None,
            who: None,
        }
    }
}
impl Default for OwnBangE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/frag.ts:795 fragMarks ->, src/game/frag.ts:795)
#[derive(Clone, Copy, Debug)]
pub struct FragMarksItem {
    pub at: V3,
    pub left: f64,
}
impl FragMarksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FragMarksItem {
            at: [0.0; 3],
            left: 0.0,
        }
    }
}
impl Default for FragMarksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WindUp {
    Own,
    Arc,
}
impl WindUp {
    pub const fn as_str(self) -> &'static str {
        match self {
            WindUp::Own => "own",
            WindUp::Arc => "arc",
        }
    }
}

/// (TypeScript: inline at src/game/frag.ts:943 free ->, src/game/frag.ts:943)
#[derive(Clone, Debug)]
pub struct Free {
    pub b: crate::frag::DefBook,
    pub c: crate::frag::Carrier,
}
impl Free {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Free {
            b: crate::frag::DefBook::zeroed(),
            c: crate::frag::Carrier::zeroed(),
        }
    }
}
impl Default for Free { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FragThrowAt {
    Thrown,
    Own,
    Busy,
    Arc,
}
impl FragThrowAt {
    pub const fn as_str(self) -> &'static str {
        match self {
            FragThrowAt::Thrown => "thrown",
            FragThrowAt::Own => "own",
            FragThrowAt::Busy => "busy",
            FragThrowAt::Arc => "arc",
        }
    }
}

/// (TypeScript: inline at src/game/frag.ts:978 lands(sol), src/game/frag.ts:978)
#[derive(Clone, Copy, Debug)]
pub struct LandsSol {
    pub land: V3,
}
impl LandsSol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LandsSol {
            land: [0.0; 3],
        }
    }
}
impl Default for LandsSol { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct FragMod {
    /// (src/game/frag.ts:209 FRAG_LOG)
    pub frag_log: Vec<crate::frag::BlastReport>,
    /// (src/game/frag.ts:212 FRAG_WHY)
    pub frag_why: JsMap<String, f64>,
    /// (src/game/frag.ts:267 WALLS_FOR)
    pub walls_for: Option<crate::frag::FragModWALLSFORValue>,
    /// (src/game/frag.ts:280 TORN)
    pub torn: Option<crate::frag::FragModTORNValue>,
    /// (src/game/frag.ts:448 warned)
    pub warned: JsMap<GuardId, f64>,
    /// (src/game/frag.ts:449 books)
    pub books: Option<Vec<crate::frag::Live>>,
    /// (src/game/frag.ts:815 defBooks)
    pub def_books: Option<crate::frag::DefBook>,
}
impl FragMod {
    pub fn blank() -> Self {
        FragMod {
            frag_log: Vec::new(),
            frag_why: JsMap::new(),
            walls_for: None,
            torn: None,
            warned: JsMap::new(),
            books: None,
            def_books: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct FragStats {
    pub thrown: i32,
    pub bursts: i32,
    pub killed: i32,
    pub wounded: i32,
    pub shielded: i32,
    pub shouts: i32,
    pub fled: i32,
    pub by_door: i32,
    pub door_walks: i32,
    pub own: i32,
    pub theirs: i32,
    pub cues: i32,
    pub held: i32,
    pub sam_stunned: i32,
    pub stunned: i32,
}

/// (src/game/frag.ts:268 NO_WALLS: a table nothing writes)
pub fn no_walls() -> Vec<crate::wallbreak::WallOpening> { Vec::new() }

/// (src/game/frag.ts:531 LOOK_RINGS: a table nothing writes)
pub fn look_rings() -> Vec<f64> { alloc::vec![1.6, 2.5, 3.5, 4.5, 5.0, 6.25] }

/// (src/game/frag.ts:830 SHOUTS: a table nothing writes)
pub fn shouts() -> Vec<String> { alloc::vec![String::from("frag out!"), String::from("grenade out!"), String::from("fire in the hole!")] }
