//! The types of `scene_materials`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoesWord {
    Snow,
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
    UnnamedDark,
    Plasterboard,
    Partition,
    Ceiling,
    Laminate,
    Pane,
    DarkCarpet,
    Card,
    Frame,
    Plant,
    PackedSnow,
    OldSnow,
    Unnamed,
    UnnamedGrey,
    UnnamedLight,
    UnnamedLightGrey,
    UnnamedPale,
    UnnamedPaleGrey,
    DarkPlastic,
    GreyPlastic,
    DarkGlass,
}
impl DoesWord {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoesWord::Snow => "snow",
            DoesWord::Carpet => "carpet",
            DoesWord::Lino => "lino",
            DoesWord::Stone => "stone",
            DoesWord::Grass => "grass",
            DoesWord::Wood => "wood",
            DoesWord::Plastic => "plastic",
            DoesWord::Concrete => "concrete",
            DoesWord::Fabric => "fabric",
            DoesWord::Steel => "steel",
            DoesWord::Enamel => "enamel",
            DoesWord::Tarmac => "tarmac",
            DoesWord::Casing => "casing",
            DoesWord::Screen => "screen",
            DoesWord::UnnamedDark => "unnamed dark",
            DoesWord::Plasterboard => "plasterboard",
            DoesWord::Partition => "partition",
            DoesWord::Ceiling => "ceiling",
            DoesWord::Laminate => "laminate",
            DoesWord::Pane => "pane",
            DoesWord::DarkCarpet => "dark carpet",
            DoesWord::Card => "card",
            DoesWord::Frame => "frame",
            DoesWord::Plant => "plant",
            DoesWord::PackedSnow => "packed snow",
            DoesWord::OldSnow => "old snow",
            DoesWord::Unnamed => "unnamed",
            DoesWord::UnnamedGrey => "unnamed grey",
            DoesWord::UnnamedLight => "unnamed light",
            DoesWord::UnnamedLightGrey => "unnamed light grey",
            DoesWord::UnnamedPale => "unnamed pale",
            DoesWord::UnnamedPaleGrey => "unnamed pale grey",
            DoesWord::DarkPlastic => "dark plastic",
            DoesWord::GreyPlastic => "grey plastic",
            DoesWord::DarkGlass => "dark glass",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Surf {
    Carpet,
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
impl Surf {
    pub const fn as_str(self) -> &'static str {
        match self {
            Surf::Carpet => "carpet",
            Surf::Metal => "metal",
            Surf::Glass => "glass",
            Surf::Wood => "wood",
            Surf::Plastic => "plastic",
            Surf::Cardboard => "cardboard",
            Surf::Drywall => "drywall",
            Surf::Concrete => "concrete",
            Surf::Fabric => "fabric",
            Surf::Foliage => "foliage",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BreakClass {
    Hard,
    Metal,
    Wood,
    Board,
    Soft,
}
impl BreakClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            BreakClass::Hard => "hard",
            BreakClass::Metal => "metal",
            BreakClass::Wood => "wood",
            BreakClass::Board => "board",
            BreakClass::Soft => "soft",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnowKind {
    Snow,
    SnowPacked,
    SnowOld,
}
impl SnowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnowKind::Snow => "snow",
            SnowKind::SnowPacked => "snowPacked",
            SnowKind::SnowOld => "snowOld",
        }
    }
}

/// (TypeScript: Does, src/scene/materials.ts:34)
#[derive(Clone, Debug)]
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
impl Does {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Does {
            surf: crate::scene_materials::Surf::Carpet,
            breaks: crate::scene_materials::BreakClass::Hard,
            steel: None,
            wall: None,
            board: None,
            cover: None,
            struct_: None,
            counter: None,
            casing: None,
            paper: None,
            screen: None,
            snow: None,
            wet: None,
        }
    }
}
impl Default for Does { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoesWall {
    Board,
    Screen,
}
impl DoesWall {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoesWall::Board => "board",
            DoesWall::Screen => "screen",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoesCover {
    Wood,
    Cardboard,
}
impl DoesCover {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoesCover::Wood => "wood",
            DoesCover::Cardboard => "cardboard",
        }
    }
}

/// (TypeScript: Material, src/scene/materials.ts:64)
#[derive(Clone, Debug)]
pub struct Material {
    pub looks: Option<bool>,
    pub does: Option<crate::scene_materials::Does>,
}
impl Material {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Material {
            looks: None,
            does: None,
        }
    }
}
impl Default for Material { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MaterialWord {
    Snow,
    Carpet,
    Lino,
    Stone,
    Grass,
    Metal,
    Wood,
    Plastic,
    Concrete,
    Fabric,
    Steel,
    Enamel,
    Tarmac,
    Casing,
    Screen,
    UnnamedDark,
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
    Plasterboard,
    Partition,
    Ceiling,
    Laminate,
    Pane,
    DarkCarpet,
    Card,
    Frame,
    Plant,
    PackedSnow,
    OldSnow,
    Unnamed,
    UnnamedGrey,
    UnnamedLight,
    UnnamedLightGrey,
    UnnamedPale,
    UnnamedPaleGrey,
    DarkPlastic,
    GreyPlastic,
    DarkGlass,
}
impl MaterialWord {
    pub const fn as_str(self) -> &'static str {
        match self {
            MaterialWord::Snow => "snow",
            MaterialWord::Carpet => "carpet",
            MaterialWord::Lino => "lino",
            MaterialWord::Stone => "stone",
            MaterialWord::Grass => "grass",
            MaterialWord::Metal => "metal",
            MaterialWord::Wood => "wood",
            MaterialWord::Plastic => "plastic",
            MaterialWord::Concrete => "concrete",
            MaterialWord::Fabric => "fabric",
            MaterialWord::Steel => "steel",
            MaterialWord::Enamel => "enamel",
            MaterialWord::Tarmac => "tarmac",
            MaterialWord::Casing => "casing",
            MaterialWord::Screen => "screen",
            MaterialWord::UnnamedDark => "unnamed dark",
            MaterialWord::MudWall => "mud wall",
            MaterialWord::Plaster => "plaster",
            MaterialWord::Block => "block",
            MaterialWord::Earth => "earth",
            MaterialWord::Gravel => "gravel",
            MaterialWord::Slab => "slab",
            MaterialWord::Sheet => "sheet",
            MaterialWord::Cloth => "cloth",
            MaterialWord::Paint => "paint",
            MaterialWord::Veneer => "veneer",
            MaterialWord::Carpaint => "carpaint",
            MaterialWord::Stucco => "stucco",
            MaterialWord::Skim => "skim",
            MaterialWord::GlossBlock => "gloss block",
            MaterialWord::BoardConcrete => "board concrete",
            MaterialWord::CastPanel => "cast panel",
            MaterialWord::Paver => "paver",
            MaterialWord::Brushed => "brushed",
            MaterialWord::Louvre => "louvre",
            MaterialWord::Plasterboard => "plasterboard",
            MaterialWord::Partition => "partition",
            MaterialWord::Ceiling => "ceiling",
            MaterialWord::Laminate => "laminate",
            MaterialWord::Pane => "pane",
            MaterialWord::DarkCarpet => "dark carpet",
            MaterialWord::Card => "card",
            MaterialWord::Frame => "frame",
            MaterialWord::Plant => "plant",
            MaterialWord::PackedSnow => "packed snow",
            MaterialWord::OldSnow => "old snow",
            MaterialWord::Unnamed => "unnamed",
            MaterialWord::UnnamedGrey => "unnamed grey",
            MaterialWord::UnnamedLight => "unnamed light",
            MaterialWord::UnnamedLightGrey => "unnamed light grey",
            MaterialWord::UnnamedPale => "unnamed pale",
            MaterialWord::UnnamedPaleGrey => "unnamed pale grey",
            MaterialWord::DarkPlastic => "dark plastic",
            MaterialWord::GreyPlastic => "grey plastic",
            MaterialWord::DarkGlass => "dark glass",
        }
    }
}

/// (TypeScript: inline at src/scene/materials.ts:69 MATERIALS, src/scene/materials.ts:69)
#[derive(Clone, Debug)]
pub struct MATERIALS {
    pub wood: crate::scene_materials::MATERIALSWood,
    pub steel: crate::scene_materials::MATERIALSSteel,
    pub enamel: crate::scene_materials::MATERIALSEnamel,
    pub fabric: crate::scene_materials::MATERIALSFabric,
    pub carpet: crate::scene_materials::MATERIALSCarpet,
    pub grass: crate::scene_materials::MATERIALSGrass,
    pub tarmac: crate::scene_materials::MATERIALSTarmac,
    pub plastic: crate::scene_materials::MATERIALSPlastic,
    pub mud_wall: crate::scene_materials::MATERIALSMudWall,
    pub plaster: crate::scene_materials::MATERIALSPlaster,
    pub block: crate::scene_materials::MATERIALSBlock,
    pub earth: crate::scene_materials::MATERIALSEarth,
    pub gravel: crate::scene_materials::MATERIALSGravel,
    pub slab: crate::scene_materials::MATERIALSSlab,
    pub sheet: crate::scene_materials::MATERIALSSheet,
    pub cloth: crate::scene_materials::MATERIALSCloth,
    pub metal: crate::scene_materials::MATERIALSMetal,
    pub paint: crate::scene_materials::MATERIALSPaint,
    pub veneer: crate::scene_materials::MATERIALSVeneer,
    pub carpaint: crate::scene_materials::MATERIALSCarpaint,
    pub stucco: crate::scene_materials::MATERIALSStucco,
    pub skim: crate::scene_materials::MATERIALSSkim,
    pub gloss_block: crate::scene_materials::MATERIALSGlossBlock,
    pub board_concrete: crate::scene_materials::MATERIALSBoardConcrete,
    pub cast_panel: crate::scene_materials::MATERIALSCastPanel,
    pub paver: crate::scene_materials::MATERIALSPaver,
    pub brushed: crate::scene_materials::MATERIALSBrushed,
    pub louvre: crate::scene_materials::MATERIALSLouvre,
    pub plasterboard: crate::scene_materials::MATERIALSPlasterboard,
    pub partition: crate::scene_materials::MATERIALSPartition,
    pub ceiling: crate::scene_materials::MATERIALSCeiling,
    pub laminate: crate::scene_materials::MATERIALSLaminate,
    pub pane: crate::scene_materials::MATERIALSPane,
    pub concrete: crate::scene_materials::MATERIALSConcrete,
    pub stone: crate::scene_materials::MATERIALSStone,
    pub lino: crate::scene_materials::MATERIALSLino,
    pub dark_carpet: crate::scene_materials::MATERIALSDarkCarpet,
    pub card: crate::scene_materials::MATERIALSCard,
    pub casing: crate::scene_materials::MATERIALSCasing,
    pub frame: crate::scene_materials::MATERIALSFrame,
    pub screen: crate::scene_materials::MATERIALSScreen,
    pub plant: crate::scene_materials::MATERIALSPlant,
    pub snow: crate::scene_materials::MATERIALSSnow,
    pub packed_snow: crate::scene_materials::MATERIALSPackedSnow,
    pub old_snow: crate::scene_materials::MATERIALSOldSnow,
    pub unnamed: crate::scene_materials::MATERIALSUnnamed,
    pub unnamed_grey: crate::scene_materials::MATERIALSUnnamedGrey,
    pub unnamed_dark: crate::scene_materials::MATERIALSUnnamedDark,
    pub unnamed_light: crate::scene_materials::MATERIALSUnnamedLight,
    pub unnamed_light_grey: crate::scene_materials::MATERIALSUnnamedLightGrey,
    pub unnamed_pale: crate::scene_materials::MATERIALSUnnamedPale,
    pub unnamed_pale_grey: crate::scene_materials::MATERIALSUnnamedPaleGrey,
    pub dark_plastic: crate::scene_materials::MATERIALSDarkPlastic,
    pub grey_plastic: crate::scene_materials::MATERIALSGreyPlastic,
    pub dark_glass: crate::scene_materials::MATERIALSDarkGlass,
}
impl MATERIALS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALS {
            wood: crate::scene_materials::MATERIALSWood::zeroed(),
            steel: crate::scene_materials::MATERIALSSteel::zeroed(),
            enamel: crate::scene_materials::MATERIALSEnamel::zeroed(),
            fabric: crate::scene_materials::MATERIALSFabric::zeroed(),
            carpet: crate::scene_materials::MATERIALSCarpet::zeroed(),
            grass: crate::scene_materials::MATERIALSGrass::zeroed(),
            tarmac: crate::scene_materials::MATERIALSTarmac::zeroed(),
            plastic: crate::scene_materials::MATERIALSPlastic::zeroed(),
            mud_wall: crate::scene_materials::MATERIALSMudWall::zeroed(),
            plaster: crate::scene_materials::MATERIALSPlaster::zeroed(),
            block: crate::scene_materials::MATERIALSBlock::zeroed(),
            earth: crate::scene_materials::MATERIALSEarth::zeroed(),
            gravel: crate::scene_materials::MATERIALSGravel::zeroed(),
            slab: crate::scene_materials::MATERIALSSlab::zeroed(),
            sheet: crate::scene_materials::MATERIALSSheet::zeroed(),
            cloth: crate::scene_materials::MATERIALSCloth::zeroed(),
            metal: crate::scene_materials::MATERIALSMetal::zeroed(),
            paint: crate::scene_materials::MATERIALSPaint::zeroed(),
            veneer: crate::scene_materials::MATERIALSVeneer::zeroed(),
            carpaint: crate::scene_materials::MATERIALSCarpaint::zeroed(),
            stucco: crate::scene_materials::MATERIALSStucco::zeroed(),
            skim: crate::scene_materials::MATERIALSSkim::zeroed(),
            gloss_block: crate::scene_materials::MATERIALSGlossBlock::zeroed(),
            board_concrete: crate::scene_materials::MATERIALSBoardConcrete::zeroed(),
            cast_panel: crate::scene_materials::MATERIALSCastPanel::zeroed(),
            paver: crate::scene_materials::MATERIALSPaver::zeroed(),
            brushed: crate::scene_materials::MATERIALSBrushed::zeroed(),
            louvre: crate::scene_materials::MATERIALSLouvre::zeroed(),
            plasterboard: crate::scene_materials::MATERIALSPlasterboard::zeroed(),
            partition: crate::scene_materials::MATERIALSPartition::zeroed(),
            ceiling: crate::scene_materials::MATERIALSCeiling::zeroed(),
            laminate: crate::scene_materials::MATERIALSLaminate::zeroed(),
            pane: crate::scene_materials::MATERIALSPane::zeroed(),
            concrete: crate::scene_materials::MATERIALSConcrete::zeroed(),
            stone: crate::scene_materials::MATERIALSStone::zeroed(),
            lino: crate::scene_materials::MATERIALSLino::zeroed(),
            dark_carpet: crate::scene_materials::MATERIALSDarkCarpet::zeroed(),
            card: crate::scene_materials::MATERIALSCard::zeroed(),
            casing: crate::scene_materials::MATERIALSCasing::zeroed(),
            frame: crate::scene_materials::MATERIALSFrame::zeroed(),
            screen: crate::scene_materials::MATERIALSScreen::zeroed(),
            plant: crate::scene_materials::MATERIALSPlant::zeroed(),
            snow: crate::scene_materials::MATERIALSSnow::zeroed(),
            packed_snow: crate::scene_materials::MATERIALSPackedSnow::zeroed(),
            old_snow: crate::scene_materials::MATERIALSOldSnow::zeroed(),
            unnamed: crate::scene_materials::MATERIALSUnnamed::zeroed(),
            unnamed_grey: crate::scene_materials::MATERIALSUnnamedGrey::zeroed(),
            unnamed_dark: crate::scene_materials::MATERIALSUnnamedDark::zeroed(),
            unnamed_light: crate::scene_materials::MATERIALSUnnamedLight::zeroed(),
            unnamed_light_grey: crate::scene_materials::MATERIALSUnnamedLightGrey::zeroed(),
            unnamed_pale: crate::scene_materials::MATERIALSUnnamedPale::zeroed(),
            unnamed_pale_grey: crate::scene_materials::MATERIALSUnnamedPaleGrey::zeroed(),
            dark_plastic: crate::scene_materials::MATERIALSDarkPlastic::zeroed(),
            grey_plastic: crate::scene_materials::MATERIALSGreyPlastic::zeroed(),
            dark_glass: crate::scene_materials::MATERIALSDarkGlass::zeroed(),
        }
    }
}
impl Default for MATERIALS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:71 MATERIALS.wood, src/scene/materials.ts:71)
#[derive(Clone, Debug)]
pub struct MATERIALSWood {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSWoodDoes,
}
impl MATERIALSWood {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSWood {
            looks: false,
            does: crate::scene_materials::MATERIALSWoodDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSWood { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:71 MATERIALSWood.does, src/scene/materials.ts:71)
#[derive(Clone, Debug)]
pub struct MATERIALSWoodDoes {
    pub surf: String,
    pub breaks: String,
    pub cover: String,
}
impl MATERIALSWoodDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSWoodDoes {
            surf: String::new(),
            breaks: String::new(),
            cover: String::new(),
        }
    }
}
impl Default for MATERIALSWoodDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:72 MATERIALS.steel, src/scene/materials.ts:72)
#[derive(Clone, Debug)]
pub struct MATERIALSSteel {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSSteelDoes,
}
impl MATERIALSSteel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSSteel {
            looks: false,
            does: crate::scene_materials::MATERIALSSteelDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSSteel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:72 MATERIALSSteel.does, src/scene/materials.ts:72)
#[derive(Clone, Debug)]
pub struct MATERIALSSteelDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}
impl MATERIALSSteelDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSSteelDoes {
            surf: String::new(),
            breaks: String::new(),
            steel: false,
        }
    }
}
impl Default for MATERIALSSteelDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:73 MATERIALS.enamel, src/scene/materials.ts:73)
#[derive(Clone, Debug)]
pub struct MATERIALSEnamel {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSEnamelDoes,
}
impl MATERIALSEnamel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSEnamel {
            looks: false,
            does: crate::scene_materials::MATERIALSEnamelDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSEnamel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:73 MATERIALSEnamel.does, src/scene/materials.ts:73)
#[derive(Clone, Debug)]
pub struct MATERIALSEnamelDoes {
    pub surf: String,
    pub breaks: String,
}
impl MATERIALSEnamelDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSEnamelDoes {
            surf: String::new(),
            breaks: String::new(),
        }
    }
}
impl Default for MATERIALSEnamelDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:74 MATERIALS.fabric, src/scene/materials.ts:74)
#[derive(Clone, Debug)]
pub struct MATERIALSFabric {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSFabricDoes,
}
impl MATERIALSFabric {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSFabric {
            looks: false,
            does: crate::scene_materials::MATERIALSFabricDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSFabric { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:74 MATERIALSFabric.does, src/scene/materials.ts:74)
#[derive(Clone, Debug)]
pub struct MATERIALSFabricDoes {
    pub surf: String,
    pub breaks: String,
}
impl MATERIALSFabricDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSFabricDoes {
            surf: String::new(),
            breaks: String::new(),
        }
    }
}
impl Default for MATERIALSFabricDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:75 MATERIALS.carpet, src/scene/materials.ts:75)
#[derive(Clone, Debug)]
pub struct MATERIALSCarpet {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSCarpetDoes,
}
impl MATERIALSCarpet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCarpet {
            looks: false,
            does: crate::scene_materials::MATERIALSCarpetDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSCarpet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:75 MATERIALSCarpet.does, src/scene/materials.ts:75)
#[derive(Clone, Debug)]
pub struct MATERIALSCarpetDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}
impl MATERIALSCarpetDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCarpetDoes {
            surf: String::new(),
            breaks: String::new(),
            struct_: false,
        }
    }
}
impl Default for MATERIALSCarpetDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:76 MATERIALS.grass, src/scene/materials.ts:76)
#[derive(Clone, Debug)]
pub struct MATERIALSGrass {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSGrassDoes,
}
impl MATERIALSGrass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSGrass {
            looks: false,
            does: crate::scene_materials::MATERIALSGrassDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSGrass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:76 MATERIALSGrass.does, src/scene/materials.ts:76)
#[derive(Clone, Debug)]
pub struct MATERIALSGrassDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}
impl MATERIALSGrassDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSGrassDoes {
            surf: String::new(),
            breaks: String::new(),
            struct_: false,
        }
    }
}
impl Default for MATERIALSGrassDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:77 MATERIALS.tarmac, src/scene/materials.ts:77)
#[derive(Clone, Debug)]
pub struct MATERIALSTarmac {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSTarmacDoes,
}
impl MATERIALSTarmac {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSTarmac {
            looks: false,
            does: crate::scene_materials::MATERIALSTarmacDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSTarmac { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:77 MATERIALSTarmac.does, src/scene/materials.ts:77)
#[derive(Clone, Debug)]
pub struct MATERIALSTarmacDoes {
    pub casing: bool,
    pub wet: String,
    pub surf: String,
    pub breaks: String,
    pub steel: Option<bool>,
    pub wall: Option<String>,
    pub board: Option<bool>,
    pub cover: Option<String>,
    pub struct_: Option<bool>,
    pub counter: Option<bool>,
    pub paper: Option<bool>,
    pub screen: Option<bool>,
    pub snow: Option<String>,
}
impl MATERIALSTarmacDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSTarmacDoes {
            casing: false,
            wet: String::new(),
            surf: String::new(),
            breaks: String::new(),
            steel: None,
            wall: None,
            board: None,
            cover: None,
            struct_: None,
            counter: None,
            paper: None,
            screen: None,
            snow: None,
        }
    }
}
impl Default for MATERIALSTarmacDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:78 MATERIALS.plastic, src/scene/materials.ts:78)
#[derive(Clone, Debug)]
pub struct MATERIALSPlastic {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSPlasticDoes,
}
impl MATERIALSPlastic {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPlastic {
            looks: false,
            does: crate::scene_materials::MATERIALSPlasticDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSPlastic { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:78 MATERIALSPlastic.does, src/scene/materials.ts:78)
#[derive(Clone, Debug)]
pub struct MATERIALSPlasticDoes {
    pub surf: String,
    pub breaks: String,
}
impl MATERIALSPlasticDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPlasticDoes {
            surf: String::new(),
            breaks: String::new(),
        }
    }
}
impl Default for MATERIALSPlasticDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:80 MATERIALS.mud wall, src/scene/materials.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSMudWall {
    pub looks: bool,
}
impl MATERIALSMudWall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSMudWall {
            looks: false,
        }
    }
}
impl Default for MATERIALSMudWall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:80 MATERIALS.plaster, src/scene/materials.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSPlaster {
    pub looks: bool,
}
impl MATERIALSPlaster {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPlaster {
            looks: false,
        }
    }
}
impl Default for MATERIALSPlaster { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:80 MATERIALS.block, src/scene/materials.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSBlock {
    pub looks: bool,
}
impl MATERIALSBlock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSBlock {
            looks: false,
        }
    }
}
impl Default for MATERIALSBlock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:80 MATERIALS.earth, src/scene/materials.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSEarth {
    pub looks: bool,
}
impl MATERIALSEarth {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSEarth {
            looks: false,
        }
    }
}
impl Default for MATERIALSEarth { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:80 MATERIALS.gravel, src/scene/materials.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSGravel {
    pub looks: bool,
}
impl MATERIALSGravel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSGravel {
            looks: false,
        }
    }
}
impl Default for MATERIALSGravel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:80 MATERIALS.slab, src/scene/materials.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSSlab {
    pub looks: bool,
}
impl MATERIALSSlab {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSSlab {
            looks: false,
        }
    }
}
impl Default for MATERIALSSlab { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:80 MATERIALS.sheet, src/scene/materials.ts:80)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSSheet {
    pub looks: bool,
}
impl MATERIALSSheet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSSheet {
            looks: false,
        }
    }
}
impl Default for MATERIALSSheet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:81 MATERIALS.cloth, src/scene/materials.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSCloth {
    pub looks: bool,
}
impl MATERIALSCloth {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCloth {
            looks: false,
        }
    }
}
impl Default for MATERIALSCloth { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:81 MATERIALS.metal, src/scene/materials.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSMetal {
    pub looks: bool,
}
impl MATERIALSMetal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSMetal {
            looks: false,
        }
    }
}
impl Default for MATERIALSMetal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:81 MATERIALS.paint, src/scene/materials.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSPaint {
    pub looks: bool,
}
impl MATERIALSPaint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPaint {
            looks: false,
        }
    }
}
impl Default for MATERIALSPaint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:81 MATERIALS.veneer, src/scene/materials.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSVeneer {
    pub looks: bool,
}
impl MATERIALSVeneer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSVeneer {
            looks: false,
        }
    }
}
impl Default for MATERIALSVeneer { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:81 MATERIALS.carpaint, src/scene/materials.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSCarpaint {
    pub looks: bool,
}
impl MATERIALSCarpaint {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCarpaint {
            looks: false,
        }
    }
}
impl Default for MATERIALSCarpaint { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:81 MATERIALS.stucco, src/scene/materials.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSStucco {
    pub looks: bool,
}
impl MATERIALSStucco {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSStucco {
            looks: false,
        }
    }
}
impl Default for MATERIALSStucco { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:81 MATERIALS.skim, src/scene/materials.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSSkim {
    pub looks: bool,
}
impl MATERIALSSkim {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSSkim {
            looks: false,
        }
    }
}
impl Default for MATERIALSSkim { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:82 MATERIALS.gloss block, src/scene/materials.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSGlossBlock {
    pub looks: bool,
}
impl MATERIALSGlossBlock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSGlossBlock {
            looks: false,
        }
    }
}
impl Default for MATERIALSGlossBlock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:82 MATERIALS.board concrete, src/scene/materials.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSBoardConcrete {
    pub looks: bool,
}
impl MATERIALSBoardConcrete {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSBoardConcrete {
            looks: false,
        }
    }
}
impl Default for MATERIALSBoardConcrete { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:82 MATERIALS.cast panel, src/scene/materials.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSCastPanel {
    pub looks: bool,
}
impl MATERIALSCastPanel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCastPanel {
            looks: false,
        }
    }
}
impl Default for MATERIALSCastPanel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:82 MATERIALS.paver, src/scene/materials.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSPaver {
    pub looks: bool,
}
impl MATERIALSPaver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPaver {
            looks: false,
        }
    }
}
impl Default for MATERIALSPaver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:82 MATERIALS.brushed, src/scene/materials.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSBrushed {
    pub looks: bool,
}
impl MATERIALSBrushed {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSBrushed {
            looks: false,
        }
    }
}
impl Default for MATERIALSBrushed { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:82 MATERIALS.louvre, src/scene/materials.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct MATERIALSLouvre {
    pub looks: bool,
}
impl MATERIALSLouvre {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSLouvre {
            looks: false,
        }
    }
}
impl Default for MATERIALSLouvre { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:84 MATERIALS.plasterboard, src/scene/materials.ts:84)
#[derive(Clone, Debug)]
pub struct MATERIALSPlasterboard {
    pub does: crate::scene_materials::MATERIALSPlasterboardDoes,
}
impl MATERIALSPlasterboard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPlasterboard {
            does: crate::scene_materials::MATERIALSPlasterboardDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSPlasterboard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:84 MATERIALSPlasterboard.does, src/scene/materials.ts:84)
#[derive(Clone, Debug)]
pub struct MATERIALSPlasterboardDoes {
    pub surf: String,
    pub breaks: String,
    pub wall: String,
    pub board: bool,
    pub struct_: bool,
}
impl MATERIALSPlasterboardDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPlasterboardDoes {
            surf: String::new(),
            breaks: String::new(),
            wall: String::new(),
            board: false,
            struct_: false,
        }
    }
}
impl Default for MATERIALSPlasterboardDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:85 MATERIALS.partition, src/scene/materials.ts:85)
#[derive(Clone, Debug)]
pub struct MATERIALSPartition {
    pub does: crate::scene_materials::MATERIALSPartitionDoes,
}
impl MATERIALSPartition {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPartition {
            does: crate::scene_materials::MATERIALSPartitionDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSPartition { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:85 MATERIALSPartition.does, src/scene/materials.ts:85)
#[derive(Clone, Debug)]
pub struct MATERIALSPartitionDoes {
    pub surf: String,
    pub breaks: String,
    pub wall: String,
    pub board: bool,
    pub struct_: bool,
}
impl MATERIALSPartitionDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPartitionDoes {
            surf: String::new(),
            breaks: String::new(),
            wall: String::new(),
            board: false,
            struct_: false,
        }
    }
}
impl Default for MATERIALSPartitionDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:86 MATERIALS.ceiling, src/scene/materials.ts:86)
#[derive(Clone, Debug)]
pub struct MATERIALSCeiling {
    pub does: crate::scene_materials::MATERIALSCeilingDoes,
}
impl MATERIALSCeiling {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCeiling {
            does: crate::scene_materials::MATERIALSCeilingDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSCeiling { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:86 MATERIALSCeiling.does, src/scene/materials.ts:86)
#[derive(Clone, Debug)]
pub struct MATERIALSCeilingDoes {
    pub surf: String,
    pub breaks: String,
    pub board: bool,
    pub struct_: bool,
    pub paper: bool,
}
impl MATERIALSCeilingDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCeilingDoes {
            surf: String::new(),
            breaks: String::new(),
            board: false,
            struct_: false,
            paper: false,
        }
    }
}
impl Default for MATERIALSCeilingDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:87 MATERIALS.laminate, src/scene/materials.ts:87)
#[derive(Clone, Debug)]
pub struct MATERIALSLaminate {
    pub does: crate::scene_materials::MATERIALSLaminateDoes,
}
impl MATERIALSLaminate {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSLaminate {
            does: crate::scene_materials::MATERIALSLaminateDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSLaminate { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:87 MATERIALSLaminate.does, src/scene/materials.ts:87)
#[derive(Clone, Debug)]
pub struct MATERIALSLaminateDoes {
    pub surf: String,
    pub breaks: String,
    pub board: bool,
    pub struct_: bool,
    pub counter: bool,
    pub paper: bool,
}
impl MATERIALSLaminateDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSLaminateDoes {
            surf: String::new(),
            breaks: String::new(),
            board: false,
            struct_: false,
            counter: false,
            paper: false,
        }
    }
}
impl Default for MATERIALSLaminateDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:88 MATERIALS.pane, src/scene/materials.ts:88)
#[derive(Clone, Debug)]
pub struct MATERIALSPane {
    pub does: crate::scene_materials::MATERIALSPaneDoes,
}
impl MATERIALSPane {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPane {
            does: crate::scene_materials::MATERIALSPaneDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSPane { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:88 MATERIALSPane.does, src/scene/materials.ts:88)
#[derive(Clone, Debug)]
pub struct MATERIALSPaneDoes {
    pub surf: String,
    pub breaks: String,
    pub board: bool,
    pub struct_: bool,
    pub paper: bool,
}
impl MATERIALSPaneDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPaneDoes {
            surf: String::new(),
            breaks: String::new(),
            board: false,
            struct_: false,
            paper: false,
        }
    }
}
impl Default for MATERIALSPaneDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:89 MATERIALS.concrete, src/scene/materials.ts:89)
#[derive(Clone, Debug)]
pub struct MATERIALSConcrete {
    pub does: crate::scene_materials::MATERIALSConcreteDoes,
}
impl MATERIALSConcrete {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSConcrete {
            does: crate::scene_materials::MATERIALSConcreteDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSConcrete { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:89 MATERIALSConcrete.does, src/scene/materials.ts:89)
#[derive(Clone, Debug)]
pub struct MATERIALSConcreteDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}
impl MATERIALSConcreteDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSConcreteDoes {
            surf: String::new(),
            breaks: String::new(),
            struct_: false,
        }
    }
}
impl Default for MATERIALSConcreteDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:90 MATERIALS.stone, src/scene/materials.ts:90)
#[derive(Clone, Debug)]
pub struct MATERIALSStone {
    pub does: crate::scene_materials::MATERIALSStoneDoes,
}
impl MATERIALSStone {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSStone {
            does: crate::scene_materials::MATERIALSStoneDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSStone { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:90 MATERIALSStone.does, src/scene/materials.ts:90)
#[derive(Clone, Debug)]
pub struct MATERIALSStoneDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}
impl MATERIALSStoneDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSStoneDoes {
            surf: String::new(),
            breaks: String::new(),
            struct_: false,
        }
    }
}
impl Default for MATERIALSStoneDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:91 MATERIALS.lino, src/scene/materials.ts:91)
#[derive(Clone, Debug)]
pub struct MATERIALSLino {
    pub does: crate::scene_materials::MATERIALSLinoDoes,
}
impl MATERIALSLino {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSLino {
            does: crate::scene_materials::MATERIALSLinoDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSLino { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:91 MATERIALSLino.does, src/scene/materials.ts:91)
#[derive(Clone, Debug)]
pub struct MATERIALSLinoDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}
impl MATERIALSLinoDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSLinoDoes {
            surf: String::new(),
            breaks: String::new(),
            struct_: false,
        }
    }
}
impl Default for MATERIALSLinoDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:92 MATERIALS.dark carpet, src/scene/materials.ts:92)
#[derive(Clone, Debug)]
pub struct MATERIALSDarkCarpet {
    pub does: crate::scene_materials::MATERIALSDarkCarpetDoes,
}
impl MATERIALSDarkCarpet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSDarkCarpet {
            does: crate::scene_materials::MATERIALSDarkCarpetDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSDarkCarpet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:92 MATERIALSDarkCarpet.does, src/scene/materials.ts:92)
#[derive(Clone, Debug)]
pub struct MATERIALSDarkCarpetDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
    pub casing: bool,
}
impl MATERIALSDarkCarpetDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSDarkCarpetDoes {
            surf: String::new(),
            breaks: String::new(),
            struct_: false,
            casing: false,
        }
    }
}
impl Default for MATERIALSDarkCarpetDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:93 MATERIALS.card, src/scene/materials.ts:93)
#[derive(Clone, Debug)]
pub struct MATERIALSCard {
    pub does: crate::scene_materials::MATERIALSCardDoes,
}
impl MATERIALSCard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCard {
            does: crate::scene_materials::MATERIALSCardDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSCard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:93 MATERIALSCard.does, src/scene/materials.ts:93)
#[derive(Clone, Debug)]
pub struct MATERIALSCardDoes {
    pub surf: String,
    pub breaks: String,
    pub cover: String,
}
impl MATERIALSCardDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCardDoes {
            surf: String::new(),
            breaks: String::new(),
            cover: String::new(),
        }
    }
}
impl Default for MATERIALSCardDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:94 MATERIALS.casing, src/scene/materials.ts:94)
#[derive(Clone, Debug)]
pub struct MATERIALSCasing {
    pub does: crate::scene_materials::MATERIALSCasingDoes,
}
impl MATERIALSCasing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCasing {
            does: crate::scene_materials::MATERIALSCasingDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSCasing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:94 MATERIALSCasing.does, src/scene/materials.ts:94)
#[derive(Clone, Debug)]
pub struct MATERIALSCasingDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}
impl MATERIALSCasingDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSCasingDoes {
            surf: String::new(),
            breaks: String::new(),
            casing: false,
        }
    }
}
impl Default for MATERIALSCasingDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:95 MATERIALS.frame, src/scene/materials.ts:95)
#[derive(Clone, Debug)]
pub struct MATERIALSFrame {
    pub does: crate::scene_materials::MATERIALSFrameDoes,
}
impl MATERIALSFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSFrame {
            does: crate::scene_materials::MATERIALSFrameDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:95 MATERIALSFrame.does, src/scene/materials.ts:95)
#[derive(Clone, Debug)]
pub struct MATERIALSFrameDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}
impl MATERIALSFrameDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSFrameDoes {
            surf: String::new(),
            breaks: String::new(),
            struct_: false,
        }
    }
}
impl Default for MATERIALSFrameDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:96 MATERIALS.screen, src/scene/materials.ts:96)
#[derive(Clone, Debug)]
pub struct MATERIALSScreen {
    pub does: crate::scene_materials::MATERIALSScreenDoes,
}
impl MATERIALSScreen {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSScreen {
            does: crate::scene_materials::MATERIALSScreenDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSScreen { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:96 MATERIALSScreen.does, src/scene/materials.ts:96)
#[derive(Clone, Debug)]
pub struct MATERIALSScreenDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
    pub screen: bool,
}
impl MATERIALSScreenDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSScreenDoes {
            surf: String::new(),
            breaks: String::new(),
            casing: false,
            screen: false,
        }
    }
}
impl Default for MATERIALSScreenDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:97 MATERIALS.plant, src/scene/materials.ts:97)
#[derive(Clone, Debug)]
pub struct MATERIALSPlant {
    pub does: crate::scene_materials::MATERIALSPlantDoes,
}
impl MATERIALSPlant {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPlant {
            does: crate::scene_materials::MATERIALSPlantDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSPlant { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:97 MATERIALSPlant.does, src/scene/materials.ts:97)
#[derive(Clone, Debug)]
pub struct MATERIALSPlantDoes {
    pub surf: String,
    pub breaks: String,
}
impl MATERIALSPlantDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPlantDoes {
            surf: String::new(),
            breaks: String::new(),
        }
    }
}
impl Default for MATERIALSPlantDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:98 MATERIALS.snow, src/scene/materials.ts:98)
#[derive(Clone, Debug)]
pub struct MATERIALSSnow {
    pub does: crate::scene_materials::MATERIALSSnowDoes,
}
impl MATERIALSSnow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSSnow {
            does: crate::scene_materials::MATERIALSSnowDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSSnow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:98 MATERIALSSnow.does, src/scene/materials.ts:98)
#[derive(Clone, Debug)]
pub struct MATERIALSSnowDoes {
    pub snow: String,
    pub paper: bool,
    pub wet: String,
    pub surf: String,
    pub breaks: String,
    pub steel: Option<bool>,
    pub wall: Option<String>,
    pub board: Option<bool>,
    pub cover: Option<String>,
    pub struct_: Option<bool>,
    pub counter: Option<bool>,
    pub casing: Option<bool>,
    pub screen: Option<bool>,
}
impl MATERIALSSnowDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSSnowDoes {
            snow: String::new(),
            paper: false,
            wet: String::new(),
            surf: String::new(),
            breaks: String::new(),
            steel: None,
            wall: None,
            board: None,
            cover: None,
            struct_: None,
            counter: None,
            casing: None,
            screen: None,
        }
    }
}
impl Default for MATERIALSSnowDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:99 MATERIALS.packed snow, src/scene/materials.ts:99)
#[derive(Clone, Debug)]
pub struct MATERIALSPackedSnow {
    pub does: crate::scene_materials::MATERIALSPackedSnowDoes,
}
impl MATERIALSPackedSnow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPackedSnow {
            does: crate::scene_materials::MATERIALSPackedSnowDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSPackedSnow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:99 MATERIALSPackedSnow.does, src/scene/materials.ts:99)
#[derive(Clone, Debug)]
pub struct MATERIALSPackedSnowDoes {
    pub snow: String,
    pub wet: String,
    pub surf: String,
    pub breaks: String,
    pub steel: Option<bool>,
    pub wall: Option<String>,
    pub board: Option<bool>,
    pub cover: Option<String>,
    pub struct_: Option<bool>,
    pub counter: Option<bool>,
    pub casing: Option<bool>,
    pub paper: Option<bool>,
    pub screen: Option<bool>,
}
impl MATERIALSPackedSnowDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSPackedSnowDoes {
            snow: String::new(),
            wet: String::new(),
            surf: String::new(),
            breaks: String::new(),
            steel: None,
            wall: None,
            board: None,
            cover: None,
            struct_: None,
            counter: None,
            casing: None,
            paper: None,
            screen: None,
        }
    }
}
impl Default for MATERIALSPackedSnowDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:100 MATERIALS.old snow, src/scene/materials.ts:100)
#[derive(Clone, Debug)]
pub struct MATERIALSOldSnow {
    pub does: crate::scene_materials::MATERIALSOldSnowDoes,
}
impl MATERIALSOldSnow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSOldSnow {
            does: crate::scene_materials::MATERIALSOldSnowDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSOldSnow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:100 MATERIALSOldSnow.does, src/scene/materials.ts:100)
#[derive(Clone, Debug)]
pub struct MATERIALSOldSnowDoes {
    pub snow: String,
    pub wet: String,
    pub surf: String,
    pub breaks: String,
    pub steel: Option<bool>,
    pub wall: Option<String>,
    pub board: Option<bool>,
    pub cover: Option<String>,
    pub struct_: Option<bool>,
    pub counter: Option<bool>,
    pub casing: Option<bool>,
    pub paper: Option<bool>,
    pub screen: Option<bool>,
}
impl MATERIALSOldSnowDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSOldSnowDoes {
            snow: String::new(),
            wet: String::new(),
            surf: String::new(),
            breaks: String::new(),
            steel: None,
            wall: None,
            board: None,
            cover: None,
            struct_: None,
            counter: None,
            casing: None,
            paper: None,
            screen: None,
        }
    }
}
impl Default for MATERIALSOldSnowDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:103 MATERIALS.unnamed, src/scene/materials.ts:103)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamed {
    pub does: crate::scene_materials::MATERIALSUnnamedDoes,
}
impl MATERIALSUnnamed {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamed {
            does: crate::scene_materials::MATERIALSUnnamedDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSUnnamed { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:103 MATERIALSUnnamed.does, src/scene/materials.ts:103)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedDoes {
    pub surf: String,
    pub breaks: String,
}
impl MATERIALSUnnamedDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedDoes {
            surf: String::new(),
            breaks: String::new(),
        }
    }
}
impl Default for MATERIALSUnnamedDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:104 MATERIALS.unnamed grey, src/scene/materials.ts:104)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedGrey {
    pub does: crate::scene_materials::MATERIALSUnnamedGreyDoes,
}
impl MATERIALSUnnamedGrey {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedGrey {
            does: crate::scene_materials::MATERIALSUnnamedGreyDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSUnnamedGrey { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:104 MATERIALSUnnamedGrey.does, src/scene/materials.ts:104)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedGreyDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}
impl MATERIALSUnnamedGreyDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedGreyDoes {
            surf: String::new(),
            breaks: String::new(),
            steel: false,
        }
    }
}
impl Default for MATERIALSUnnamedGreyDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:105 MATERIALS.unnamed dark, src/scene/materials.ts:105)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedDark {
    pub does: crate::scene_materials::MATERIALSUnnamedDarkDoes,
}
impl MATERIALSUnnamedDark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedDark {
            does: crate::scene_materials::MATERIALSUnnamedDarkDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSUnnamedDark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:105 MATERIALSUnnamedDark.does, src/scene/materials.ts:105)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedDarkDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}
impl MATERIALSUnnamedDarkDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedDarkDoes {
            surf: String::new(),
            breaks: String::new(),
            casing: false,
        }
    }
}
impl Default for MATERIALSUnnamedDarkDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:106 MATERIALS.unnamed light, src/scene/materials.ts:106)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedLight {
    pub does: crate::scene_materials::MATERIALSUnnamedLightDoes,
}
impl MATERIALSUnnamedLight {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedLight {
            does: crate::scene_materials::MATERIALSUnnamedLightDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSUnnamedLight { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:106 MATERIALSUnnamedLight.does, src/scene/materials.ts:106)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedLightDoes {
    pub surf: String,
    pub breaks: String,
}
impl MATERIALSUnnamedLightDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedLightDoes {
            surf: String::new(),
            breaks: String::new(),
        }
    }
}
impl Default for MATERIALSUnnamedLightDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:107 MATERIALS.unnamed light grey, src/scene/materials.ts:107)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedLightGrey {
    pub does: crate::scene_materials::MATERIALSUnnamedLightGreyDoes,
}
impl MATERIALSUnnamedLightGrey {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedLightGrey {
            does: crate::scene_materials::MATERIALSUnnamedLightGreyDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSUnnamedLightGrey { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:107 MATERIALSUnnamedLightGrey.does, src/scene/materials.ts:107)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedLightGreyDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}
impl MATERIALSUnnamedLightGreyDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedLightGreyDoes {
            surf: String::new(),
            breaks: String::new(),
            steel: false,
        }
    }
}
impl Default for MATERIALSUnnamedLightGreyDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:108 MATERIALS.unnamed pale, src/scene/materials.ts:108)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedPale {
    pub does: crate::scene_materials::MATERIALSUnnamedPaleDoes,
}
impl MATERIALSUnnamedPale {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedPale {
            does: crate::scene_materials::MATERIALSUnnamedPaleDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSUnnamedPale { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:108 MATERIALSUnnamedPale.does, src/scene/materials.ts:108)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedPaleDoes {
    pub surf: String,
    pub breaks: String,
    pub paper: bool,
}
impl MATERIALSUnnamedPaleDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedPaleDoes {
            surf: String::new(),
            breaks: String::new(),
            paper: false,
        }
    }
}
impl Default for MATERIALSUnnamedPaleDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:109 MATERIALS.unnamed pale grey, src/scene/materials.ts:109)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedPaleGrey {
    pub does: crate::scene_materials::MATERIALSUnnamedPaleGreyDoes,
}
impl MATERIALSUnnamedPaleGrey {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedPaleGrey {
            does: crate::scene_materials::MATERIALSUnnamedPaleGreyDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSUnnamedPaleGrey { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:109 MATERIALSUnnamedPaleGrey.does, src/scene/materials.ts:109)
#[derive(Clone, Debug)]
pub struct MATERIALSUnnamedPaleGreyDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
    pub paper: bool,
}
impl MATERIALSUnnamedPaleGreyDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSUnnamedPaleGreyDoes {
            surf: String::new(),
            breaks: String::new(),
            steel: false,
            paper: false,
        }
    }
}
impl Default for MATERIALSUnnamedPaleGreyDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:111 MATERIALS.dark plastic, src/scene/materials.ts:111)
#[derive(Clone, Debug)]
pub struct MATERIALSDarkPlastic {
    pub does: crate::scene_materials::MATERIALSDarkPlasticDoes,
}
impl MATERIALSDarkPlastic {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSDarkPlastic {
            does: crate::scene_materials::MATERIALSDarkPlasticDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSDarkPlastic { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:111 MATERIALSDarkPlastic.does, src/scene/materials.ts:111)
#[derive(Clone, Debug)]
pub struct MATERIALSDarkPlasticDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}
impl MATERIALSDarkPlasticDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSDarkPlasticDoes {
            surf: String::new(),
            breaks: String::new(),
            casing: false,
        }
    }
}
impl Default for MATERIALSDarkPlasticDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:112 MATERIALS.grey plastic, src/scene/materials.ts:112)
#[derive(Clone, Debug)]
pub struct MATERIALSGreyPlastic {
    pub does: crate::scene_materials::MATERIALSGreyPlasticDoes,
}
impl MATERIALSGreyPlastic {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSGreyPlastic {
            does: crate::scene_materials::MATERIALSGreyPlasticDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSGreyPlastic { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:112 MATERIALSGreyPlastic.does, src/scene/materials.ts:112)
#[derive(Clone, Debug)]
pub struct MATERIALSGreyPlasticDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}
impl MATERIALSGreyPlasticDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSGreyPlasticDoes {
            surf: String::new(),
            breaks: String::new(),
            steel: false,
        }
    }
}
impl Default for MATERIALSGreyPlasticDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:113 MATERIALS.dark glass, src/scene/materials.ts:113)
#[derive(Clone, Debug)]
pub struct MATERIALSDarkGlass {
    pub does: crate::scene_materials::MATERIALSDarkGlassDoes,
}
impl MATERIALSDarkGlass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSDarkGlass {
            does: crate::scene_materials::MATERIALSDarkGlassDoes::zeroed(),
        }
    }
}
impl Default for MATERIALSDarkGlass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:113 MATERIALSDarkGlass.does, src/scene/materials.ts:113)
#[derive(Clone, Debug)]
pub struct MATERIALSDarkGlassDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}
impl MATERIALSDarkGlassDoes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MATERIALSDarkGlassDoes {
            surf: String::new(),
            breaks: String::new(),
            casing: false,
        }
    }
}
impl Default for MATERIALSDarkGlassDoes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:133 doesOf(b), src/scene/materials.ts:133)
#[derive(Clone, Debug)]
pub struct DoesOfB {
    pub does: Option<String>,
}
impl DoesOfB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoesOfB {
            does: None,
        }
    }
}
impl Default for DoesOfB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/materials.ts:135 doesWet(b), src/scene/materials.ts:135)
#[derive(Clone, Debug)]
pub struct DoesWetB {
    pub does: Option<String>,
}
impl DoesWetB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoesWetB {
            does: None,
        }
    }
}
impl Default for DoesWetB { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneMaterialsMod {
    /// (src/scene/materials.ts:123 ROWS)
    pub rows: JsMap<String, crate::scene_materials::Does>,
}
impl SceneMaterialsMod {
    pub fn blank() -> Self {
        SceneMaterialsMod {
            rows: { let mut m = JsMap::new(); m.set(String::from("wood"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Wood, breaks: crate::scene_materials::BreakClass::Wood, cover: Some(crate::scene_materials::DoesCover::Wood), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("steel"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("enamel"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::zeroed() }); m.set(String::from("fabric"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Fabric, breaks: crate::scene_materials::BreakClass::Soft, ..crate::scene_materials::Does::zeroed() }); m.set(String::from("carpet"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("grass"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("tarmac"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), casing: Some(true), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::zeroed() }); m.set(String::from("plasterboard"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), wall: Some(crate::scene_materials::DoesWall::Board), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("partition"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), wall: Some(crate::scene_materials::DoesWall::Screen), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("ceiling"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), paper: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("laminate"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), counter: Some(true), paper: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("pane"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), paper: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("concrete"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("stone"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("lino"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("dark carpet"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), casing: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("card"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Cardboard, breaks: crate::scene_materials::BreakClass::Board, cover: Some(crate::scene_materials::DoesCover::Cardboard), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("casing"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Hard, casing: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("frame"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("screen"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Glass, breaks: crate::scene_materials::BreakClass::Hard, casing: Some(true), screen: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("plant"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Foliage, breaks: crate::scene_materials::BreakClass::Soft, ..crate::scene_materials::Does::zeroed() }); m.set(String::from("snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), paper: Some(true), snow: Some(crate::scene_materials::SnowKind::Snow), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("packed snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), snow: Some(crate::scene_materials::SnowKind::SnowPacked), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("old snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), snow: Some(crate::scene_materials::SnowKind::SnowOld), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("unnamed"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::zeroed() }); m.set(String::from("unnamed grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("unnamed dark"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("unnamed light"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::zeroed() }); m.set(String::from("unnamed light grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("unnamed pale"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, paper: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("unnamed pale grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), paper: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("dark plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("grey plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::zeroed() }); m.set(String::from("dark glass"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Glass, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::zeroed() }); m },
        }
    }
}

/// (src/scene/materials.ts:67 GROUND: a table nothing writes)
pub fn ground() -> crate::scene_materials::Does { crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, steel: None, wall: None, board: None, cover: None, struct_: Some(true), counter: None, casing: None, paper: None, screen: None, snow: None, wet: None } }

/// (src/scene/materials.ts:69 MATERIALS: a table nothing writes)
pub fn materials() -> crate::scene_materials::MATERIALS { crate::scene_materials::MATERIALS { wood: crate::scene_materials::MATERIALSWood { looks: true, does: crate::scene_materials::MATERIALSWoodDoes { surf: String::from("wood"), breaks: String::from("wood"), cover: String::from("wood") } }, steel: crate::scene_materials::MATERIALSSteel { looks: true, does: crate::scene_materials::MATERIALSSteelDoes { surf: String::from("metal"), breaks: String::from("metal"), steel: true } }, enamel: crate::scene_materials::MATERIALSEnamel { looks: true, does: crate::scene_materials::MATERIALSEnamelDoes { surf: String::from("metal"), breaks: String::from("metal") } }, fabric: crate::scene_materials::MATERIALSFabric { looks: true, does: crate::scene_materials::MATERIALSFabricDoes { surf: String::from("fabric"), breaks: String::from("soft") } }, carpet: crate::scene_materials::MATERIALSCarpet { looks: true, does: crate::scene_materials::MATERIALSCarpetDoes { surf: String::from("carpet"), breaks: String::from("hard"), struct_: true } }, grass: crate::scene_materials::MATERIALSGrass { looks: true, does: crate::scene_materials::MATERIALSGrassDoes { surf: String::from("carpet"), breaks: String::from("hard"), struct_: true } }, tarmac: crate::scene_materials::MATERIALSTarmac { looks: true, does: crate::scene_materials::MATERIALSTarmacDoes { casing: true, wet: String::from("unnamed dark"), surf: String::from("concrete"), breaks: String::from("hard"), steel: None, wall: None, board: None, cover: None, struct_: Some(true), counter: None, paper: None, screen: None, snow: None } }, plastic: crate::scene_materials::MATERIALSPlastic { looks: true, does: crate::scene_materials::MATERIALSPlasticDoes { surf: String::from("plastic"), breaks: String::from("metal") } }, mud_wall: crate::scene_materials::MATERIALSMudWall { looks: true }, plaster: crate::scene_materials::MATERIALSPlaster { looks: true }, block: crate::scene_materials::MATERIALSBlock { looks: true }, earth: crate::scene_materials::MATERIALSEarth { looks: true }, gravel: crate::scene_materials::MATERIALSGravel { looks: true }, slab: crate::scene_materials::MATERIALSSlab { looks: true }, sheet: crate::scene_materials::MATERIALSSheet { looks: true }, cloth: crate::scene_materials::MATERIALSCloth { looks: true }, metal: crate::scene_materials::MATERIALSMetal { looks: true }, paint: crate::scene_materials::MATERIALSPaint { looks: true }, veneer: crate::scene_materials::MATERIALSVeneer { looks: true }, carpaint: crate::scene_materials::MATERIALSCarpaint { looks: true }, stucco: crate::scene_materials::MATERIALSStucco { looks: true }, skim: crate::scene_materials::MATERIALSSkim { looks: true }, gloss_block: crate::scene_materials::MATERIALSGlossBlock { looks: true }, board_concrete: crate::scene_materials::MATERIALSBoardConcrete { looks: true }, cast_panel: crate::scene_materials::MATERIALSCastPanel { looks: true }, paver: crate::scene_materials::MATERIALSPaver { looks: true }, brushed: crate::scene_materials::MATERIALSBrushed { looks: true }, louvre: crate::scene_materials::MATERIALSLouvre { looks: true }, plasterboard: crate::scene_materials::MATERIALSPlasterboard { does: crate::scene_materials::MATERIALSPlasterboardDoes { surf: String::from("drywall"), breaks: String::from("hard"), wall: String::from("board"), board: true, struct_: true } }, partition: crate::scene_materials::MATERIALSPartition { does: crate::scene_materials::MATERIALSPartitionDoes { surf: String::from("drywall"), breaks: String::from("hard"), wall: String::from("screen"), board: true, struct_: true } }, ceiling: crate::scene_materials::MATERIALSCeiling { does: crate::scene_materials::MATERIALSCeilingDoes { surf: String::from("drywall"), breaks: String::from("hard"), board: true, struct_: true, paper: true } }, laminate: crate::scene_materials::MATERIALSLaminate { does: crate::scene_materials::MATERIALSLaminateDoes { surf: String::from("drywall"), breaks: String::from("hard"), board: true, struct_: true, counter: true, paper: true } }, pane: crate::scene_materials::MATERIALSPane { does: crate::scene_materials::MATERIALSPaneDoes { surf: String::from("drywall"), breaks: String::from("hard"), board: true, struct_: true, paper: true } }, concrete: crate::scene_materials::MATERIALSConcrete { does: crate::scene_materials::MATERIALSConcreteDoes { surf: String::from("concrete"), breaks: String::from("hard"), struct_: true } }, stone: crate::scene_materials::MATERIALSStone { does: crate::scene_materials::MATERIALSStoneDoes { surf: String::from("concrete"), breaks: String::from("hard"), struct_: true } }, lino: crate::scene_materials::MATERIALSLino { does: crate::scene_materials::MATERIALSLinoDoes { surf: String::from("concrete"), breaks: String::from("hard"), struct_: true } }, dark_carpet: crate::scene_materials::MATERIALSDarkCarpet { does: crate::scene_materials::MATERIALSDarkCarpetDoes { surf: String::from("carpet"), breaks: String::from("hard"), struct_: true, casing: true } }, card: crate::scene_materials::MATERIALSCard { does: crate::scene_materials::MATERIALSCardDoes { surf: String::from("cardboard"), breaks: String::from("board"), cover: String::from("cardboard") } }, casing: crate::scene_materials::MATERIALSCasing { does: crate::scene_materials::MATERIALSCasingDoes { surf: String::from("metal"), breaks: String::from("hard"), casing: true } }, frame: crate::scene_materials::MATERIALSFrame { does: crate::scene_materials::MATERIALSFrameDoes { surf: String::from("metal"), breaks: String::from("hard"), struct_: true } }, screen: crate::scene_materials::MATERIALSScreen { does: crate::scene_materials::MATERIALSScreenDoes { surf: String::from("glass"), breaks: String::from("hard"), casing: true, screen: true } }, plant: crate::scene_materials::MATERIALSPlant { does: crate::scene_materials::MATERIALSPlantDoes { surf: String::from("foliage"), breaks: String::from("soft") } }, snow: crate::scene_materials::MATERIALSSnow { does: crate::scene_materials::MATERIALSSnowDoes { snow: String::from("snow"), paper: true, wet: String::from("unnamed dark"), surf: String::from("concrete"), breaks: String::from("hard"), steel: None, wall: None, board: None, cover: None, struct_: Some(true), counter: None, casing: None, screen: None } }, packed_snow: crate::scene_materials::MATERIALSPackedSnow { does: crate::scene_materials::MATERIALSPackedSnowDoes { snow: String::from("snowPacked"), wet: String::from("unnamed dark"), surf: String::from("concrete"), breaks: String::from("hard"), steel: None, wall: None, board: None, cover: None, struct_: Some(true), counter: None, casing: None, paper: None, screen: None } }, old_snow: crate::scene_materials::MATERIALSOldSnow { does: crate::scene_materials::MATERIALSOldSnowDoes { snow: String::from("snowOld"), wet: String::from("unnamed dark"), surf: String::from("concrete"), breaks: String::from("hard"), steel: None, wall: None, board: None, cover: None, struct_: Some(true), counter: None, casing: None, paper: None, screen: None } }, unnamed: crate::scene_materials::MATERIALSUnnamed { does: crate::scene_materials::MATERIALSUnnamedDoes { surf: String::from("concrete"), breaks: String::from("metal") } }, unnamed_grey: crate::scene_materials::MATERIALSUnnamedGrey { does: crate::scene_materials::MATERIALSUnnamedGreyDoes { surf: String::from("concrete"), breaks: String::from("metal"), steel: true } }, unnamed_dark: crate::scene_materials::MATERIALSUnnamedDark { does: crate::scene_materials::MATERIALSUnnamedDarkDoes { surf: String::from("metal"), breaks: String::from("metal"), casing: true } }, unnamed_light: crate::scene_materials::MATERIALSUnnamedLight { does: crate::scene_materials::MATERIALSUnnamedLightDoes { surf: String::from("drywall"), breaks: String::from("metal") } }, unnamed_light_grey: crate::scene_materials::MATERIALSUnnamedLightGrey { does: crate::scene_materials::MATERIALSUnnamedLightGreyDoes { surf: String::from("drywall"), breaks: String::from("metal"), steel: true } }, unnamed_pale: crate::scene_materials::MATERIALSUnnamedPale { does: crate::scene_materials::MATERIALSUnnamedPaleDoes { surf: String::from("drywall"), breaks: String::from("metal"), paper: true } }, unnamed_pale_grey: crate::scene_materials::MATERIALSUnnamedPaleGrey { does: crate::scene_materials::MATERIALSUnnamedPaleGreyDoes { surf: String::from("drywall"), breaks: String::from("metal"), steel: true, paper: true } }, dark_plastic: crate::scene_materials::MATERIALSDarkPlastic { does: crate::scene_materials::MATERIALSDarkPlasticDoes { surf: String::from("plastic"), breaks: String::from("metal"), casing: true } }, grey_plastic: crate::scene_materials::MATERIALSGreyPlastic { does: crate::scene_materials::MATERIALSGreyPlasticDoes { surf: String::from("plastic"), breaks: String::from("metal"), steel: true } }, dark_glass: crate::scene_materials::MATERIALSDarkGlass { does: crate::scene_materials::MATERIALSDarkGlassDoes { surf: String::from("glass"), breaks: String::from("metal"), casing: true } } } }

/// (src/scene/materials.ts:120 DOES_DEFAULT)
pub const DOES_DEFAULT: &str = "unnamed";

/// (src/scene/materials.ts:124 DEFAULT: a table nothing writes)
pub fn default() -> crate::scene_materials::Does { crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, steel: None, wall: None, board: None, cover: None, struct_: None, counter: None, casing: None, paper: None, screen: None, snow: None, wet: None } }
