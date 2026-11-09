//! The types of `scene_stuff`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
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
    #[word("mud wall")] MudWall,
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
    #[word("gloss block")] GlossBlock,
    #[word("board concrete")] BoardConcrete,
    #[word("cast panel")] CastPanel,
    Paver,
    Brushed,
    Louvre,
    Plain,
    Waits,
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SceneStuffMod {
    /// (src/scene/stuff.ts:110 PAINTS)
    #[is({ let mut m = crate::js::JsMap::new(); let c = crate::rig_props::col(); for (row, stuff) in [(c.can, crate::scene_stuff::BoxStuff::Metal), (c.fde, crate::scene_stuff::BoxStuff::MudWall), (c.shell, crate::scene_stuff::BoxStuff::Cloth), ([0.4, 0.41, 0.42], crate::scene_stuff::BoxStuff::Sheet)] { m.set([row[0].to_bits(), row[1].to_bits(), row[2].to_bits()], crate::scene_stuff::Paint { name: String::new(), rough: None, metal: None, stuff: Some(stuff), does: None }); } m })] pub paints: JsMap<[u64; 3], crate::scene_stuff::Paint>,
}

