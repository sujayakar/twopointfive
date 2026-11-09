//! The types of `scene_materials`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum BreakClass {
    #[default] Hard,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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
impl DoesWall {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoesWall::Board => "board",
            DoesWall::Screen => "screen",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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

#[derive(Clone, Debug, Default)]
pub struct Material {
    pub looks: Option<bool>,
    pub does: Option<crate::scene_materials::Does>,
}

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

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct MATERIALSWood {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSWoodDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSWoodDoes {
    pub surf: String,
    pub breaks: String,
    pub cover: String,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSSteel {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSSteelDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSSteelDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSEnamel {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSEnamelDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSEnamelDoes {
    pub surf: String,
    pub breaks: String,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSFabric {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSFabricDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSFabricDoes {
    pub surf: String,
    pub breaks: String,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCarpet {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSCarpetDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCarpetDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSGrass {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSGrassDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSGrassDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSTarmac {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSTarmacDoes,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPlastic {
    pub looks: bool,
    pub does: crate::scene_materials::MATERIALSPlasticDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPlasticDoes {
    pub surf: String,
    pub breaks: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSMudWall {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSPlaster {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSBlock {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSEarth {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSGravel {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSSlab {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSSheet {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSCloth {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSMetal {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSPaint {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSVeneer {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSCarpaint {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSStucco {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSSkim {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSGlossBlock {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSBoardConcrete {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSCastPanel {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSPaver {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSBrushed {
    pub looks: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MATERIALSLouvre {
    pub looks: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPlasterboard {
    pub does: crate::scene_materials::MATERIALSPlasterboardDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPlasterboardDoes {
    pub surf: String,
    pub breaks: String,
    pub wall: String,
    pub board: bool,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPartition {
    pub does: crate::scene_materials::MATERIALSPartitionDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPartitionDoes {
    pub surf: String,
    pub breaks: String,
    pub wall: String,
    pub board: bool,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCeiling {
    pub does: crate::scene_materials::MATERIALSCeilingDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCeilingDoes {
    pub surf: String,
    pub breaks: String,
    pub board: bool,
    pub struct_: bool,
    pub paper: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSLaminate {
    pub does: crate::scene_materials::MATERIALSLaminateDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSLaminateDoes {
    pub surf: String,
    pub breaks: String,
    pub board: bool,
    pub struct_: bool,
    pub counter: bool,
    pub paper: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPane {
    pub does: crate::scene_materials::MATERIALSPaneDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPaneDoes {
    pub surf: String,
    pub breaks: String,
    pub board: bool,
    pub struct_: bool,
    pub paper: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSConcrete {
    pub does: crate::scene_materials::MATERIALSConcreteDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSConcreteDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSStone {
    pub does: crate::scene_materials::MATERIALSStoneDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSStoneDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSLino {
    pub does: crate::scene_materials::MATERIALSLinoDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSLinoDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSDarkCarpet {
    pub does: crate::scene_materials::MATERIALSDarkCarpetDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSDarkCarpetDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
    pub casing: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCard {
    pub does: crate::scene_materials::MATERIALSCardDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCardDoes {
    pub surf: String,
    pub breaks: String,
    pub cover: String,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCasing {
    pub does: crate::scene_materials::MATERIALSCasingDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSCasingDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSFrame {
    pub does: crate::scene_materials::MATERIALSFrameDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSFrameDoes {
    pub surf: String,
    pub breaks: String,
    pub struct_: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSScreen {
    pub does: crate::scene_materials::MATERIALSScreenDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSScreenDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
    pub screen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPlant {
    pub does: crate::scene_materials::MATERIALSPlantDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPlantDoes {
    pub surf: String,
    pub breaks: String,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSSnow {
    pub does: crate::scene_materials::MATERIALSSnowDoes,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct MATERIALSPackedSnow {
    pub does: crate::scene_materials::MATERIALSPackedSnowDoes,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct MATERIALSOldSnow {
    pub does: crate::scene_materials::MATERIALSOldSnowDoes,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamed {
    pub does: crate::scene_materials::MATERIALSUnnamedDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedDoes {
    pub surf: String,
    pub breaks: String,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedGrey {
    pub does: crate::scene_materials::MATERIALSUnnamedGreyDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedGreyDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedDark {
    pub does: crate::scene_materials::MATERIALSUnnamedDarkDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedDarkDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedLight {
    pub does: crate::scene_materials::MATERIALSUnnamedLightDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedLightDoes {
    pub surf: String,
    pub breaks: String,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedLightGrey {
    pub does: crate::scene_materials::MATERIALSUnnamedLightGreyDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedLightGreyDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedPale {
    pub does: crate::scene_materials::MATERIALSUnnamedPaleDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedPaleDoes {
    pub surf: String,
    pub breaks: String,
    pub paper: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedPaleGrey {
    pub does: crate::scene_materials::MATERIALSUnnamedPaleGreyDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSUnnamedPaleGreyDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
    pub paper: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSDarkPlastic {
    pub does: crate::scene_materials::MATERIALSDarkPlasticDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSDarkPlasticDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSGreyPlastic {
    pub does: crate::scene_materials::MATERIALSGreyPlasticDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSGreyPlasticDoes {
    pub surf: String,
    pub breaks: String,
    pub steel: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSDarkGlass {
    pub does: crate::scene_materials::MATERIALSDarkGlassDoes,
}

#[derive(Clone, Debug, Default)]
pub struct MATERIALSDarkGlassDoes {
    pub surf: String,
    pub breaks: String,
    pub casing: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DoesOfB {
    pub does: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct DoesWetB {
    pub does: Option<String>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SceneMaterialsMod {
    /// (src/scene/materials.ts:123 ROWS)
    pub rows: JsMap<String, crate::scene_materials::Does>,
}
impl SceneMaterialsMod {
    pub fn blank() -> Self {
        SceneMaterialsMod {
            rows: { let mut m = JsMap::new(); m.set(String::from("wood"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Wood, breaks: crate::scene_materials::BreakClass::Wood, cover: Some(crate::scene_materials::DoesCover::Wood), ..crate::scene_materials::Does::default() }); m.set(String::from("steel"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("enamel"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("fabric"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Fabric, breaks: crate::scene_materials::BreakClass::Soft, ..crate::scene_materials::Does::default() }); m.set(String::from("carpet"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("grass"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("tarmac"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), casing: Some(true), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("plasterboard"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), wall: Some(crate::scene_materials::DoesWall::Board), ..crate::scene_materials::Does::default() }); m.set(String::from("partition"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), wall: Some(crate::scene_materials::DoesWall::Screen), ..crate::scene_materials::Does::default() }); m.set(String::from("ceiling"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("laminate"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), counter: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("pane"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Hard, board: Some(true), struct_: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("concrete"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("stone"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("lino"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("dark carpet"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Carpet, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("card"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Cardboard, breaks: crate::scene_materials::BreakClass::Board, cover: Some(crate::scene_materials::DoesCover::Cardboard), ..crate::scene_materials::Does::default() }); m.set(String::from("casing"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Hard, casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("frame"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("screen"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Glass, breaks: crate::scene_materials::BreakClass::Hard, casing: Some(true), screen: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("plant"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Foliage, breaks: crate::scene_materials::BreakClass::Soft, ..crate::scene_materials::Does::default() }); m.set(String::from("snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), paper: Some(true), snow: Some(crate::scene_materials::SnowKind::Snow), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("packed snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), snow: Some(crate::scene_materials::SnowKind::SnowPacked), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("old snow"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Hard, struct_: Some(true), snow: Some(crate::scene_materials::SnowKind::SnowOld), wet: Some(String::from("unnamed dark")), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Concrete, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed dark"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Metal, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed light"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed light grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed pale"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("unnamed pale grey"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Drywall, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), paper: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("dark plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("grey plastic"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Plastic, breaks: crate::scene_materials::BreakClass::Metal, steel: Some(true), ..crate::scene_materials::Does::default() }); m.set(String::from("dark glass"), crate::scene_materials::Does { surf: crate::scene_materials::Surf::Glass, breaks: crate::scene_materials::BreakClass::Metal, casing: Some(true), ..crate::scene_materials::Does::default() }); m },
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
