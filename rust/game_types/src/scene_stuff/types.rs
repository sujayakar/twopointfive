//! The types of `scene_stuff`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum BoxStuff {
    Carpet,
    Grass,
    Rubber,
    Metal,
    Glass,
    Wood,
    Plastic,
    Fabric,
    Steel,
    Enamel,
    Tarmac,
    MudWall,
    Plaster,
    Block,
    Earth,
    Gravel,
    Slab,
    Sheet,
    Cloth,
    Paint,
    Veneer,
    Carpaint,
    Stucco,
    Skim,
    GlossBlock,
    BoardConcrete,
    CastPanel,
    Paver,
    Brushed,
    Louvre,
    Plain,
    Waits,
}
impl BoxStuff {
    pub const fn as_str(self) -> &'static str {
        match self {
            BoxStuff::Carpet => "carpet",
            BoxStuff::Grass => "grass",
            BoxStuff::Rubber => "rubber",
            BoxStuff::Metal => "metal",
            BoxStuff::Glass => "glass",
            BoxStuff::Wood => "wood",
            BoxStuff::Plastic => "plastic",
            BoxStuff::Fabric => "fabric",
            BoxStuff::Steel => "steel",
            BoxStuff::Enamel => "enamel",
            BoxStuff::Tarmac => "tarmac",
            BoxStuff::MudWall => "mud wall",
            BoxStuff::Plaster => "plaster",
            BoxStuff::Block => "block",
            BoxStuff::Earth => "earth",
            BoxStuff::Gravel => "gravel",
            BoxStuff::Slab => "slab",
            BoxStuff::Sheet => "sheet",
            BoxStuff::Cloth => "cloth",
            BoxStuff::Paint => "paint",
            BoxStuff::Veneer => "veneer",
            BoxStuff::Carpaint => "carpaint",
            BoxStuff::Stucco => "stucco",
            BoxStuff::Skim => "skim",
            BoxStuff::GlossBlock => "gloss block",
            BoxStuff::BoardConcrete => "board concrete",
            BoxStuff::CastPanel => "cast panel",
            BoxStuff::Paver => "paver",
            BoxStuff::Brushed => "brushed",
            BoxStuff::Louvre => "louvre",
            BoxStuff::Plain => "plain",
            BoxStuff::Waits => "waits",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Paint {
    pub name: String,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Finish {
    pub rough: Option<f64>,
    pub metal: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WordFor {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SceneStuffMod {
    /// (src/scene/stuff.ts:110 PAINTS)
    pub paints: JsMap<[u64; 3], crate::scene_stuff::Paint>,
}
impl SceneStuffMod {
    pub fn blank() -> Self {
        SceneStuffMod {
            paints: { let mut m = crate::js::JsMap::new(); let c = crate::rig_props::col(); for (row, stuff) in [(c.can, crate::scene_stuff::BoxStuff::Metal), (c.fde, crate::scene_stuff::BoxStuff::MudWall), (c.shell, crate::scene_stuff::BoxStuff::Cloth), ([0.4, 0.41, 0.42], crate::scene_stuff::BoxStuff::Sheet)] { m.set([row[0].to_bits(), row[1].to_bits(), row[2].to_bits()], crate::scene_stuff::Paint { name: String::new(), rough: None, metal: None, stuff: Some(stuff), does: None }); } m },
        }
    }
}

