//! The types of `scene_materials`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum DoesWord {
    #[default] Snow,
    Carpet,
    Lino,
    Stone,
    Grass,
    Wood,
    Plastic,
    Concrete,
    Fabric,
    Steel,
    Enamel,
    Tarmac,
    Casing,
    Screen,
    #[word("unnamed dark")] UnnamedDark,
    Plasterboard,
    Partition,
    Ceiling,
    Laminate,
    Pane,
    #[word("dark carpet")] DarkCarpet,
    Card,
    Frame,
    Plant,
    #[word("packed snow")] PackedSnow,
    #[word("old snow")] OldSnow,
    Unnamed,
    #[word("unnamed grey")] UnnamedGrey,
    #[word("unnamed light")] UnnamedLight,
    #[word("unnamed light grey")] UnnamedLightGrey,
    #[word("unnamed pale")] UnnamedPale,
    #[word("unnamed pale grey")] UnnamedPaleGrey,
    #[word("dark plastic")] DarkPlastic,
    #[word("grey plastic")] GreyPlastic,
    #[word("dark glass")] DarkGlass,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum Surf {
    #[default] Carpet,
    Metal,
    Glass,
    Wood,
    Plastic,
    Cardboard,
    Drywall,
    Concrete,
    Fabric,
    Foliage,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum BreakClass {
    #[default] Hard,
    Metal,
    Wood,
    Board,
    Soft,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum SnowKind {
    Snow,
    SnowPacked,
    SnowOld,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Does {
    pub surf: crate::scene_materials::Surf,
    pub breaks: crate::scene_materials::BreakClass,
    pub steel: Option<bool>,
    pub wall: Option<crate::scene_materials::DoesWall>,
    pub board: Option<bool>,
    pub cover: Option<crate::scene_materials::DoesCover>,
    pub struct_: Option<bool>,
    pub counter: Option<bool>,
    pub casing: Option<bool>,
    pub paper: Option<bool>,
    pub screen: Option<bool>,
    pub snow: Option<crate::scene_materials::SnowKind>,
    pub wet: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum DoesWall {
    Board,
    Screen,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum DoesCover {
    Wood,
    Cardboard,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SceneMaterialsMod {
    #[is({ let mut m = JsMap::new(); m.set(String::from("wood"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Wood, breaks: crate::scene_materials::BreakClass::Wood, cover: Some(crate::scene_materials::DoesCover::Wood), ..crate::scene_materials::Does::default() }); m.set(String::from("steel"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("enamel"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("fabric"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Fabric, breaks: crate::scene_materials::BreakClass::Soft, ..crate::scene_materials::Does::default() }); m.set(String::from("carpet"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("grass"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("tarmac"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), casing: Some(true), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("plasterboard"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), wall: Some(crate::scene_materials::DoesWall::Board), ..crate::scene_materials::Does::default() }); m.set(String::from("partition"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), wall: Some(crate::scene_materials::DoesWall::Screen), ..crate::scene_materials::Does::default() }); m.set(String::from("ceiling"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("laminate"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), counter: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("pane"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("concrete"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("stone"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("lino"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("dark carpet"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("card"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Cardboard, breaks: crate::scene_materials::BreakClass::Board, cover: Some(crate::scene_materials::DoesCover::Cardboard), ..crate::scene_materials::Does::default() }); m.set(String::from("casing"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Hard, casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("frame"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("screen"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Glass, breaks: crate::scene_materials::BreakClass::Hard, casing: Some(true), screen: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("plant"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Foliage, breaks: crate::scene_materials::BreakClass::Soft, ..crate::scene_materials::Does::default() }); m.set(String::from("snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), paper: Some(true), snow: Some(crate::scene_materials::SnowKind::Snow), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("packed snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), snow: Some(crate::scene_materials::SnowKind::SnowPacked), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("old snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), snow: Some(crate::scene_materials::SnowKind::SnowOld), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed dark"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed light"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed light grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed pale"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed pale grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("dark plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("grey plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("dark glass"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Glass, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::default() }); m })] pub rows: JsMap<String, crate::scene_materials::Does>,
}

pub fn default() -> crate::scene_materials::Does { crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, steel: None, wall: None, board: None, cover: None, struct_: None, counter: None, casing: None, paper: None, screen: None, snow: None, wet: None } }
