//! The types of `teamfeel`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// one kept feel row's value: a switch or a number (teamfeel.ts read, Table's unknown)
#[derive(Clone, Debug)]
pub enum FeelValue {
    Num { v: f64 },
    Flag { v: bool },
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FeelName {
    PAIR,
    FLASH,
    PACE,
    LIGHTS,
    NVG,
    IR,
    IRLIGHT,
    FOG,
    LASER,
    DOORTOOLS,
    ROOMFIGHT,
    ORDERS,
    COVER,
    ENTRY,
    REAR,
    SILENT,
    FRAG,
    DOORMANNERS,
}
impl FeelName {
    pub const fn as_str(self) -> &'static str {
        match self {
            FeelName::PAIR => "PAIR",
            FeelName::FLASH => "FLASH",
            FeelName::PACE => "PACE",
            FeelName::LIGHTS => "LIGHTS",
            FeelName::NVG => "NVG",
            FeelName::IR => "IR",
            FeelName::IRLIGHT => "IRLIGHT",
            FeelName::FOG => "FOG",
            FeelName::LASER => "LASER",
            FeelName::DOORTOOLS => "DOORTOOLS",
            FeelName::ROOMFIGHT => "ROOMFIGHT",
            FeelName::ORDERS => "ORDERS",
            FeelName::COVER => "COVER",
            FeelName::ENTRY => "ENTRY",
            FeelName::REAR => "REAR",
            FeelName::SILENT => "SILENT",
            FeelName::FRAG => "FRAG",
            FeelName::DOORMANNERS => "DOORMANNERS",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct TeamfeelMod {
    /// (src/game/teamfeel.ts:87 shipped)
    pub shipped: Option<JsMap<String, crate::teamfeel::FeelValue>>,
    /// (src/game/teamfeel.ts:104 touched)
    pub touched: JsSet<String>,
}
impl TeamfeelMod {
    pub fn blank() -> Self {
        TeamfeelMod {
            shipped: None,
            touched: JsSet::new(),
        }
    }
}

/// (src/game/teamfeel.ts:28 KEPT: a table nothing writes)
pub fn kept() -> JsMap<String, JsMap<String, Option<[f64; 3]>>> { { let mut t = JsMap::new(); t.set(String::from("FLASH"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("secs.0"), Some([1.0, 12.0, 0.5])); t.set(String::from("secs.1"), Some([0.5, 12.0, 0.5])); t.set(String::from("stagger.0"), Some([0.0, 6.0, 0.25])); t.set(String::from("stagger.1"), Some([0.0, 6.0, 0.25])); t.set(String::from("concussM"), Some([0.0, 12.0, 0.5])); t.set(String::from("concussK"), Some([0.0, 1.0, 0.05])); t.set(String::from("ownM"), Some([0.0, 6.0, 0.25])); t.set(String::from("fuse"), Some([0.2, 2.5, 0.1])); t }); t.set(String::from("PACE"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("max"), Some([1.2, 4.7, 0.1])); t.set(String::from("accel"), Some([1.0, 12.0, 0.2])); t.set(String::from("turn"), Some([0.0, 30.0, 1.0])); t.set(String::from("swing"), Some([0.0, 60.0, 2.0])); t }); t.set(String::from("LIGHTS"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("litOff"), None); t.set(String::from("litAt"), Some([0.05, 0.9, 0.05])); t.set(String::from("lookDark"), None); t.set(String::from("afterSecs"), Some([0.0, 10.0, 0.5])); t.set(String::from("hotSecs"), Some([0.0, 15.0, 0.5])); t.set(String::from("stepSecs"), Some([0.0, 15.0, 0.5])); t.set(String::from("onM"), Some([0.2, 5.0, 0.1])); t.set(String::from("aloneM"), Some([2.0, 20.0, 0.5])); t.set(String::from("litMan"), None); t.set(String::from("wallOff"), None); t }); t.set(String::from("NVG"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("lead"), None); t.set(String::from("own"), None); t.set(String::from("quietSecs"), Some([0.0, 90.0, 5.0])); t.set(String::from("darkAt"), Some([0.05, 0.9, 0.05])); t.set(String::from("blackAt"), Some([0.0, 0.6, 0.05])); t.set(String::from("huntFrees"), None); t }); t.set(String::from("IR"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("sam"), None); t.set(String::from("carry"), None); t.set(String::from("teamOff"), None); t }); t.set(String::from("IRLIGHT"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("team"), None); t.set(String::from("auto"), None); t.set(String::from("meter"), None); t.set(String::from("near"), None); t.set(String::from("nearM"), Some([1.0, 12.0, 0.5])); t.set(String::from("show"), Some([0.02, 0.4, 0.01])); t.set(String::from("power"), Some([1.0, 100.0, 1.0])); t.set(String::from("cone"), Some([30.0, 85.0, 5.0])); t.set(String::from("reach"), Some([8.0, 30.0, 1.0])); t }); t.set(String::from("FOG"), { let mut t = JsMap::new(); t.set(String::from("own"), None); t.set(String::from("calls"), None); t.set(String::from("doors"), None); t.set(String::from("lasers"), None); t.set(String::from("lastSeen"), Some([0.0, 10.0, 0.5])); t }); t.set(String::from("LASER"), { let mut t = JsMap::new(); t.set(String::from("enemy"), None); t }); t.set(String::from("FRAG"), { let mut t = JsMap::new(); t.set(String::from("defenders"), None); t.set(String::from("defMax"), Some([0.0, 6.0, 1.0])); t.set(String::from("defShare"), Some([1.0, 11.0, 1.0])); t.set(String::from("defFuse"), Some([2.0, 6.0, 0.25])); t.set(String::from("defWind"), Some([0.0, 3.0, 0.1])); t.set(String::from("defGap"), Some([0.0, 60.0, 1.0])); t.set(String::from("fuse"), Some([1.5, 6.0, 0.25])); t.set(String::from("lethalM"), Some([1.0, 8.0, 0.5])); t.set(String::from("woundM"), Some([2.0, 14.0, 0.5])); t.set(String::from("glassM"), Some([0.5, 8.0, 0.25])); t.set(String::from("glassPower"), Some([0.0, 3.0, 0.1])); t.set(String::from("paneEnergy"), Some([0.0, 1.0, 0.05])); t }); t.set(String::from("SILENT"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("start"), None); t.set(String::from("pace"), Some([0.0, 2.2, 0.1])); t.set(String::from("hotPace"), None); t.set(String::from("step"), Some([0.0, 0.8, 0.05])); t.set(String::from("lights"), None); t.set(String::from("hush"), None); t.set(String::from("doors"), None); t.set(String::from("kicks"), None); t.set(String::from("picks"), None); t.set(String::from("bootWedged"), None); t.set(String::from("cans"), None); t.set(String::from("holdFire"), None); t.set(String::from("noticed"), Some([0.1, 1.0, 0.05])); t.set(String::from("toolsLoud"), None); t.set(String::from("glass"), None); t }); t.set(String::from("DOORTOOLS"), { let mut t = JsMap::new(); t.set(String::from("secondThrows"), None); t.set(String::from("lastOut"), None); t.set(String::from("slicePace"), Some([0.0, 2.2, 0.1])); t.set(String::from("corners"), None); t.set(String::from("sectors"), None); t.set(String::from("search"), None); t.set(String::from("scanRate"), Some([0.4, 4.0, 0.1])); t.set(String::from("charges"), Some([0.0, 6.0, 1.0])); t.set(String::from("bangs"), Some([0.0, 9.0, 1.0])); t.set(String::from("wordOnKey"), None); t.set(String::from("thrSlice"), None); t.set(String::from("thrLight"), Some([0.0, 2.0, 1.0])); t.set(String::from("thrMin"), Some([0.0, 15.0, 0.5])); t.set(String::from("thrQuiet"), Some([0.0, 10.0, 0.5])); t.set(String::from("thrBangs"), Some([0.0, 4.0, 1.0])); t.set(String::from("thrIn"), Some([1.0, 4.0, 1.0])); t.set(String::from("thrSet"), None); t.set(String::from("thrTube"), None); t.set(String::from("thrDark"), None); t.set(String::from("thrDoors"), None); t.set(String::from("thrDoorsM"), Some([0.0, 12.0, 0.5])); t.set(String::from("thrDoorsLit"), None); t.set(String::from("thrFirst"), Some([0.0, 2.0, 1.0])); t.set(String::from("thrFirstMin"), Some([0.0, 15.0, 0.5])); t.set(String::from("thrFirstStale"), Some([0.0, 15.0, 0.5])); t.set(String::from("thrFirstIn"), Some([0.0, 4.0, 0.2])); t.set(String::from("thrFirstOnly"), None); t.set(String::from("thrLast"), None); t.set(String::from("otherDoorM"), Some([0.0, 12.0, 0.5])); t.set(String::from("thrHoldFirst"), None); t.set(String::from("thrBangFirst"), None); t.set(String::from("thrOutsideRow"), None); t.set(String::from("thrLeaveFallIn"), None); t.set(String::from("thrHandsOff"), None); t.set(String::from("thrHeldAll"), None); t.set(String::from("thrMenuWord"), None); t.set(String::from("cornerOwed"), None); t.set(String::from("collapse"), None); t.set(String::from("cornerTurn"), None); t.set(String::from("leadSeen"), None); t.set(String::from("leadSeenMin"), Some([0.0, 80.0, 2.0])); t.set(String::from("leadSeenSecs"), Some([5.0, 120.0, 5.0])); t.set(String::from("buddy"), None); t.set(String::from("buddyDoors"), None); t.set(String::from("buddyM"), Some([2.0, 12.0, 0.5])); t.set(String::from("buddyPairM"), Some([5.0, 30.0, 1.0])); t.set(String::from("buddyLeftM"), Some([4.0, 20.0, 1.0])); t.set(String::from("flowOn"), None); t.set(String::from("flowCells"), Some([8.0, 120.0, 4.0])); t.set(String::from("openAir"), None); t.set(String::from("flowMax"), Some([0.0, 900.0, 20.0])); t.set(String::from("threatFirst"), None); t.set(String::from("threatSecs"), Some([2.0, 30.0, 1.0])); t.set(String::from("threatCap"), Some([0.5, 4.0, 0.1])); t.set(String::from("fragWay"), None); t.set(String::from("bangWay"), None); t.set(String::from("fragWayWait"), Some([0.0, 3.0, 0.1])); t.set(String::from("openHand"), None); t.set(String::from("gateHand"), None); t }); t.set(String::from("ROOMFIGHT"), { let mut t = JsMap::new(); t.set(String::from("stepEars"), None); t.set(String::from("arcHalfDeg"), Some([30.0, 180.0, 5.0])); t.set(String::from("teamHot"), Some([5.0, 120.0, 5.0])); t.set(String::from("cued"), None); t.set(String::from("letGo"), Some([0.0, 3.0, 0.1])); t.set(String::from("stepLook"), None); t.set(String::from("samLine"), None); t.set(String::from("turnWalk"), None); t.set(String::from("nearest"), None); t.set(String::from("closeLane"), None); t.set(String::from("samBehind"), None); t.set(String::from("doorway"), None); t.set(String::from("sideStep"), None); t.set(String::from("fallBack"), None); t.set(String::from("rally"), None); t.set(String::from("samTravel"), None); t.set(String::from("samBeside"), None); t.set(String::from("noShotGo"), Some([0.0, 5.0, 0.25])); t.set(String::from("keepTrack"), None); t.set(String::from("rallyK"), Some([0.0, 1.0, 0.05])); t.set(String::from("fallBackIn"), None); t.set(String::from("sights"), None); t.set(String::from("sightsFoe"), None); t.set(String::from("sightsDeg"), Some([20.0, 58.0, 1.0])); t.set(String::from("coverKeep"), None); t.set(String::from("doorEars"), None); t.set(String::from("doorLookSecs"), Some([1.0, 8.0, 0.5])); t.set(String::from("doorShots"), None); t.set(String::from("doorShotM"), Some([0.0, 12.0, 0.5])); t }); t.set(String::from("ORDERS"), { let mut t = JsMap::new(); t.set(String::from("run"), Some([1.2, 4.7, 0.1])); t.set(String::from("brisk"), Some([1.2, 4.7, 0.1])); t.set(String::from("entry"), Some([1.2, 4.7, 0.1])); t.set(String::from("asideHold"), Some([0.0, 4.0, 0.25])); t.set(String::from("offDoor"), Some([0.0, 1.5, 0.1])); t.set(String::from("clickGives"), None); t.set(String::from("wheelElement"), None); t.set(String::from("doorRay"), None); t.set(String::from("workOut"), None); t.set(String::from("hallFile"), None); t.set(String::from("secondFloor"), None); t.set(String::from("cleared"), None); t.set(String::from("knownFace"), None); t.set(String::from("ownDoor"), None); t.set(String::from("clearedDear"), Some([1.0, 40.0, 1.0])); t }); t.set(String::from("COVER"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("stacked"), None); t.set(String::from("rearWalkM"), Some([0.0, 30.0, 0.5])); t.set(String::from("inRoom"), None); t.set(String::from("edge"), None); t.set(String::from("laneWalk"), None); t.set(String::from("step"), None); t.set(String::from("stepM"), Some([0.5, 2.5, 0.1])); t.set(String::from("backKeep"), None); t }); t.set(String::from("ENTRY"), { let mut t = JsMap::new(); t.set(String::from("offOpenings"), None); t.set(String::from("watch"), None); t.set(String::from("nearM"), Some([0.0, 6.0, 0.5])); t.set(String::from("gaps"), None); t.set(String::from("offMouth"), None); t }); t.set(String::from("REAR"), { let mut t = JsMap::new(); t.set(String::from("settle"), None); t.set(String::from("secs"), Some([0.0, 3.0, 0.25])); t.set(String::from("file"), None); t }); t.set(String::from("PAIR"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("steady"), None); t.set(String::from("markOnHands"), None); t.set(String::from("watched"), None); t.set(String::from("quietFirst"), None); t.set(String::from("stageM"), Some([1.2, 4.0, 0.1])); t.set(String::from("secs"), Some([0.2, 2.0, 0.1])); t.set(String::from("pace"), Some([0.6, 2.2, 0.1])); t.set(String::from("goPace"), Some([0.6, 2.2, 0.1])); t.set(String::from("coneM"), Some([0.0, 16.0, 1.0])); t.set(String::from("crouch"), None); t.set(String::from("darkVis"), Some([0.05, 0.9, 0.05])); t.set(String::from("darkM"), Some([0.0, 3.0, 0.5])); t.set(String::from("darkRefuse"), None); t.set(String::from("darkCrouch"), None); t.set(String::from("grip"), None); t.set(String::from("gripBody"), None); t }); t.set(String::from("DOORMANNERS"), { let mut t = JsMap::new(); t.set(String::from("on"), None); t.set(String::from("pass"), None); t.set(String::from("stand"), None); t.set(String::from("swing"), None); t.set(String::from("lane"), None); t.set(String::from("laneMove"), None); t.set(String::from("laneLeaf"), None); t.set(String::from("laneWalk"), None); t.set(String::from("watch"), None); t.set(String::from("fight"), None); t.set(String::from("gap"), Some([0.8, 2.0, 0.05])); t.set(String::from("patience"), Some([0.5, 1.2, 0.1])); t.set(String::from("restClear"), None); t.set(String::from("markClear"), None); t.set(String::from("podClear"), None); t.set(String::from("shortClear"), None); t.set(String::from("stepCover"), None); t.set(String::from("planeDeg"), Some([0.0, 60.0, 5.0])); t.set(String::from("ownPlace"), None); t.set(String::from("leafRound"), None); t.set(String::from("leafStall"), None); t }); t } }

/// (src/game/teamfeel.ts:79 LS_KEY)
pub const LS_KEY: &str = "twopointsix.teamfeel";
