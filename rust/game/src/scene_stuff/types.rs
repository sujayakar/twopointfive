//! The types of `scene_stuff`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Stuff {
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
    FieldStone,
    MudBrick,
    Plain,
    Waits,
}
impl Stuff {
    pub const fn as_str(self) -> &'static str {
        match self {
            Stuff::Carpet => "carpet",
            Stuff::Grass => "grass",
            Stuff::Rubber => "rubber",
            Stuff::Metal => "metal",
            Stuff::Glass => "glass",
            Stuff::Wood => "wood",
            Stuff::Plastic => "plastic",
            Stuff::Fabric => "fabric",
            Stuff::Steel => "steel",
            Stuff::Enamel => "enamel",
            Stuff::Tarmac => "tarmac",
            Stuff::MudWall => "mud wall",
            Stuff::Plaster => "plaster",
            Stuff::Block => "block",
            Stuff::Earth => "earth",
            Stuff::Gravel => "gravel",
            Stuff::Slab => "slab",
            Stuff::Sheet => "sheet",
            Stuff::Cloth => "cloth",
            Stuff::Paint => "paint",
            Stuff::Veneer => "veneer",
            Stuff::Carpaint => "carpaint",
            Stuff::Stucco => "stucco",
            Stuff::Skim => "skim",
            Stuff::GlossBlock => "gloss block",
            Stuff::BoardConcrete => "board concrete",
            Stuff::CastPanel => "cast panel",
            Stuff::Paver => "paver",
            Stuff::Brushed => "brushed",
            Stuff::Louvre => "louvre",
            Stuff::FieldStone => "field stone",
            Stuff::MudBrick => "mud brick",
            Stuff::Plain => "plain",
            Stuff::Waits => "waits",
        }
    }
}

/// (TypeScript: Paint, src/scene/stuff.ts:107)
#[derive(Clone, Debug)]
pub struct Paint {
    pub name: String,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}
impl Paint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Paint {
            name: String::new(),
            rough: None,
            metal: None,
            stuff: None,
            does: None,
        }
    }
}
impl Default for Paint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Finish, src/scene/stuff.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct Finish {
    pub rough: Option<f64>,
    pub metal: Option<bool>,
}
impl Finish {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Finish {
            rough: None,
            metal: None,
        }
    }
}
impl Default for Finish { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/stuff.ts:131 ONE_PAINT, src/scene/stuff.ts:131)
#[derive(Clone, Copy, Debug)]
pub struct ONEPAINT {
    pub refuses: bool,
}
impl ONEPAINT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ONEPAINT {
            refuses: false,
        }
    }
}
impl Default for ONEPAINT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/stuff.ts:153 sayPaint(said), src/scene/stuff.ts:153 sayPaint(said))
#[derive(Clone, Copy, Debug)]
pub struct SayPaintSaid {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
}
impl SayPaintSaid {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SayPaintSaid {
            stuff: None,
            does: None,
            rough: None,
            metal: None,
        }
    }
}
impl Default for SayPaintSaid { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/stuff.ts:163 madeOf(finish), src/scene/stuff.ts:163)
#[derive(Clone, Copy, Debug)]
pub struct MadeOfFinish {
    pub rough: f64,
    pub metal: Option<bool>,
}
impl MadeOfFinish {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MadeOfFinish {
            rough: 0.0,
            metal: None,
        }
    }
}
impl Default for MadeOfFinish { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/stuff.ts:211 wordFor ->, src/scene/stuff.ts:211)
#[derive(Clone, Copy, Debug)]
pub struct WordFor {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}
impl WordFor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WordFor {
            stuff: None,
            does: None,
        }
    }
}
impl Default for WordFor { fn default() -> Self { Self::zeroed() } }

/// (src/scene/stuff.ts:131 ONE_PAINT: a table nothing writes)
pub fn one_paint() -> crate::scene_stuff::ONEPAINT { crate::scene_stuff::ONEPAINT { refuses: false } }
