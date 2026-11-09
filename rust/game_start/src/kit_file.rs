//! THE TEAM & KIT PICTURE'S BLOCK AS A LEVEL FILE SAYS IT (`scenarios.kitView`). The game keeps none of it: the picture is planned and stood from the file's own tree (`kit_plan.rs`, `kit_set.rs`,
//! `kit_stage.rs`), and these are the records a file is held to (`file.rs`, `faults.rs`).
use crate::file::{BoxData, DoorData, LightData};
use crate::json::Json;
use crate::read::{At, FromJson};
use crate::{json_struct, json_words};
use alloc::{string::String, vec::Vec};
use game::js::JsMap;
use game::V3;

/// the block: a way for each of the team's starts, and the sets a way may stand on
pub struct KitView { pub ways: Vec<KitViewWay>, pub sets: Option<JsMap<String, KitViewSet>> }
json_struct!(KitView { ways: "ways", sets: "sets" });
/// a piece of floor of its own, in the level format's own records
pub struct KitViewSet { pub boxes: Vec<BoxData>, pub doors: Option<Vec<DoorData>>, pub lights: Option<Vec<LightData>>, pub ref_: [f64; 2] }
json_struct!(KitViewSet { boxes: "boxes", doors: "doors", lights: "lights", ref_: "ref" });
/// one way: what it is anchored on, where the team stands, what it hides and how it is lit
pub struct KitViewWay {
    pub door: Option<String>, pub side: Option<f64>, pub pin: Option<KitViewWayPin>, pub slots: Option<Vec<Option<[f64; 3]>>>, pub light: Option<KitViewLight>,
    pub hide: Option<Vec<String>>, pub hide_paint: Option<Vec<String>>, pub hide_more: Option<Vec<String>>, pub room_lights: Option<f64>, pub span: Option<[[f64; 2]; 2]>,
    pub kind: Option<KitViewWayKind>, pub set: Option<String>, pub ref_: Option<[f64; 2]>, pub hand: Option<KitViewWayHand>, pub slot_gap: Option<f64>, pub wall: Option<KitViewWayWall>,
    pub lane: Option<f64>, pub out: Option<f64>, pub inside: Option<f64>, pub pane: Option<KitViewWayPane>, pub show: Option<Vec<String>>, pub indoors: Option<bool>,
    pub keep_lights: Option<Vec<String>>, pub day_lights: Option<Vec<String>>, pub lit_inside: Option<bool>, pub add: Option<KitViewWayAdd>,
}
json_struct!(KitViewWay {
    door: "door", side: "side", pin: "pin", slots: "slots", light: "light", hide: "hide", hide_paint: "hidePaint", hide_more: "hideMore", room_lights: "roomLights", span: "span",
    kind: "kind", set: "set", ref_: "ref", hand: "hand", slot_gap: "slotGap", wall: "wall", lane: "lane", out: "out", inside: "inside", pane: "pane", show: "show", indoors: "indoors",
    keep_lights: "keepLights", day_lights: "dayLights", lit_inside: "litInside", add: "add",
});
#[derive(Clone, Copy, PartialEq)]
pub enum KitViewWayKind { Door, Wall, Gap }
json_words!(KitViewWayKind { Door: "door", Wall: "wall", Gap: "gap" });
pub enum KitViewWayHand { Left, Right }
json_words!(KitViewWayHand { Left: "left", Right: "right" });
pub struct KitViewWayPin { pub yaw: f64, pub pitch: f64, pub dist: Option<f64> }
json_struct!(KitViewWayPin { yaw: "yaw", pitch: "pitch", dist: "dist" });
pub struct KitViewWayWall { pub face: Option<f64>, pub thickness: Option<f64>, pub top: Option<f64> }
json_struct!(KitViewWayWall { face: "face", thickness: "thickness", top: "top" });
/// a pane a way looks through: its own record, or false
pub enum KitViewWayPane { Bool(bool), Of(KitViewWayPaneOf) }
impl FromJson for KitViewWayPane {
    fn from_json(j: &Json, at: &At) -> Self { match j { Json::Bool(b) => KitViewWayPane::Bool(*b), _ => KitViewWayPane::Of(FromJson::from_json(j, at)) } }
}
pub struct KitViewWayPaneOf { pub box_: String, pub stand: Option<[f64; 3]> }
json_struct!(KitViewWayPaneOf { box_: "box", stand: "stand" });
/// a piece or a lamp more on a level's own way
pub struct KitViewWayAdd { pub boxes: Option<Vec<BoxData>>, pub lamps: Option<Vec<LightData>>, pub laid: Option<bool> }
json_struct!(KitViewWayAdd { boxes: "boxes", lamps: "lamps", laid: "laid" });
/// what a way says of its light over the picture's own
pub struct KitViewLight {
    pub porch: Option<KitViewPorch>, pub fill: Option<KitViewFill>, pub room_lights: Option<f64>, pub moon_scale: Option<f64>, pub sky_scale: Option<f64>,
    pub sam: Option<KitViewSam>,
}
json_struct!(KitViewLight { porch: "porch", fill: "fill", room_lights: "roomLights", moon_scale: "moonScale", sky_scale: "skyScale", sam: "sam" });
pub struct KitViewPorch {
    pub kind: Option<f64>, pub offset: Option<V3>, pub aim: Option<V3>, pub color: Option<V3>, pub intensity: Option<f64>, pub range: Option<f64>, pub inner_deg: Option<f64>,
    pub outer_deg: Option<f64>, pub radius: Option<f64>, pub volumetric: Option<f64>, pub broad: Option<bool>, pub fixture: Option<KitViewPorchFixture>, pub day: Option<f64>,
}
json_struct!(KitViewPorch {
    kind: "kind", offset: "offset", aim: "aim", color: "color", intensity: "intensity", range: "range", inner_deg: "innerDeg", outer_deg: "outerDeg", radius: "radius",
    volumetric: "volumetric", broad: "broad", fixture: "fixture", day: "day",
});
pub struct KitViewPorchFixture { pub at: V3, pub half: V3, pub glow: f64 }
json_struct!(KitViewPorchFixture { at: "at", half: "half", glow: "glow" });
pub struct KitViewFill {
    pub kind: Option<f64>, pub offset: Option<V3>, pub aim: Option<V3>, pub color: Option<V3>, pub intensity: Option<f64>, pub range: Option<f64>, pub inner_deg: Option<f64>,
    pub outer_deg: Option<f64>, pub radius: Option<f64>, pub volumetric: Option<f64>, pub broad: Option<bool>,
}
json_struct!(KitViewFill {
    kind: "kind", offset: "offset", aim: "aim", color: "color", intensity: "intensity", range: "range", inner_deg: "innerDeg", outer_deg: "outerDeg", radius: "radius",
    volumetric: "volumetric", broad: "broad",
});
pub struct KitViewSam { pub on: Option<bool>, pub intensity: Option<f64>, pub inner_deg: Option<f64>, pub outer_deg: Option<f64>, pub volumetric: Option<f64>, pub aim: Option<V3> }
json_struct!(KitViewSam { on: "on", intensity: "intensity", inner_deg: "innerDeg", outer_deg: "outerDeg", volumetric: "volumetric", aim: "aim" });
