//! THE TABLE OF MATERIALS, what a thing does: what it is to a round, a charge and a blast. A row a word, declared once here: the level tools' half is written from it
//! (tools/levels/lib/materials.ts, by the program `half`). The rules read these columns and nothing else of a material: no paint, no paint's name, no colour.
//!
//! - A box says two words at most, and neither is ever worked out from the other: `stuff`, the material it is drawn as (the picture's own rows: src/render/lookTex.ts
//!   STUFFS), and `does`, the row here it takes. They cannot be one word. A level and its look twin are one simulation under two pictures (the look's skin draws a
//!   plasterboard partition as a mud wall, and a charge must still cut it), and one paint is many things (what is drawn as 'paint' is a steel door, a timber locker and a
//!   concrete kerb to a blast). Some words stand on both sides: 'wood' is drawn as wood and breaks as wood.
//! - The word is on the box. Whoever makes a box writes it there once: the loader from the level file (the box's own `does`, else its material row's), a kit from the paint
//!   it was handed, the game from the box a piece came off. A rule takes the box's row or the default one (`Unnamed`), and asks nothing else. A level file carries the
//!   word, never a row's place.
//! - Until 2026-11-05 the rules read a box's colour: a paint of the office's palette by its name, any other by how bright and how grey it was. Every row is one answer that
//!   rule gave, under a plain name, and every level file was written again with the word its colour said, so no rule moved (tools/levels/doesWas.ts keeps the old rule).
//!   So some rows are accidents kept: the 'unnamed' rows are tones, not stuffs. Each is a choice still owed: an edit here, or a word in a generator.
use crate::{bytes::ToBytes, crosses::Crosses, Words};

/// what a round knocks out of a thing, and the class a wall of it is to the wall engine, a round and a charge
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words, Crosses)]
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

/// how a blast breaks a piece of furniture (`furn_break` has a row a class)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words, Crosses)]
pub enum BreakClass {
    #[default] Hard,
    Metal,
    Wood,
    Board,
    Soft,
}

/// lying snow, by how trodden it is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words, Crosses)]
pub enum SnowKind {
    Snow,
    SnowPacked,
    SnowOld,
}

/// the wall engine's partitions: a plasterboard wall where it stands full height and low cover where it does not · a cubicle's partition, a full wall the same way and, thin
/// and low, cloth on a frame
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words, Crosses)]
pub enum DoesWall {
    Board,
    Screen,
}

/// low cover the wall engine breaks, and furniture a blast takes where it is a panel or a small carton
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words, Crosses)]
pub enum DoesCover {
    Wood,
    Cardboard,
}

/// WHAT A MATERIAL DOES: every column a rule reads
#[derive(Clone, Copy, PartialEq, Debug, ToBytes, Crosses)]
pub struct Does {
    pub surf: Surf,
    pub breaks: BreakClass,
    /// sheet steel on a frame: a cabinet or a locker of it comes apart as sheets and a carcass
    pub steel: bool,
    pub wall: Option<DoesWall>,
    /// plasterboard to a round: it goes through a thin run of it under a roof
    pub board: bool,
    pub cover: Option<DoesCover>,
    /// the building, not its furniture: a blast leaves it standing and its mouth throws none of it
    pub structure: bool,
    /// … but low down it is a counter or an appliance, which the mouth of a hole does throw
    pub counter: bool,
    /// a dark casing: a screen's housing, a monitor's foot and neck are found by it
    pub casing: bool,
    /// a small thin sheet of it lying on a desk or pinned to a wall is paper
    pub paper: bool,
    /// a thin slab of it is a screen or a television
    pub screen: bool,
    pub snow: Option<SnowKind>,
}
/// a row with nothing said past its two classes, and each thing a row may say
const fn row(surf: Surf, breaks: BreakClass) -> Does {
    Does { surf, breaks, steel: false, wall: None, board: false, cover: None, structure: false, counter: false, casing: false, paper: false, screen: false, snow: None }
}
impl Does {
    const fn steel(self) -> Does { Does { steel: true, ..self } }
    const fn wall(self, w: DoesWall) -> Does { Does { wall: Some(w), ..self } }
    const fn board(self) -> Does { Does { board: true, ..self } }
    const fn cover(self, c: DoesCover) -> Does { Does { cover: Some(c), ..self } }
    const fn structure(self) -> Does { Does { structure: true, ..self } }
    const fn counter(self) -> Does { Does { counter: true, ..self } }
    const fn casing(self) -> Does { Does { casing: true, ..self } }
    const fn paper(self) -> Does { Does { paper: true, ..self } }
    const fn screen(self) -> Does { Does { screen: true, ..self } }
    const fn snow(self, s: SnowKind) -> Does { Does { snow: Some(s), ..self } }
}

/// THE WORDS AND THEIR ROWS
macro_rules! materials {
    ($($(#[$($m:tt)*])* $word:ident => $row:expr;)*) => {
        /// a word a box may say for what it does
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
        pub enum DoesWord { $($(#[$($m)*])* $word,)* }
        impl DoesWord {
            pub const ALL: &'static [DoesWord] = &[$(DoesWord::$word),*];
            /// its row
            pub const fn row(self) -> &'static Does { &ROWS[self as usize] }
            /// the word as a file says it
            pub fn named(w: &str) -> Option<DoesWord> { DoesWord::ALL.iter().copied().find(|d| d.as_str() == w) }
        }
        static ROWS: [Does; DoesWord::ALL.len()] = { use {BreakClass::*, DoesCover as Cover, DoesWall as Wall, Surf as S}; [$($row),*] };
    };
}
materials! {
    /// THE DEFAULT ROW: what a box that says nothing does. Never written on a box or in a file where nothing else speaks: absent means this
    #[default] Unnamed => row(S::Concrete, Metal);

    // drawn and done under one word
    /// timber: a desk, a crate, a pallet, a locker
    Wood => row(S::Wood, Wood).cover(Cover::Wood);
    /// clean painted steel: shelving, a cabinet, a drum, a vehicle's body
    Steel => row(S::Metal, Metal).steel();
    /// a small enamelled thing: an extinguisher
    Enamel => row(S::Metal, Metal);
    /// a seat's cloth, a tarpaulin, a sack, a blanket
    Fabric => row(S::Fabric, Soft);
    /// a fitted carpet, a rug
    Carpet => row(S::Carpet, Hard).structure();
    /// a lawn
    Grass => row(S::Carpet, Hard).structure();
    /// the car park, a road, a yard's dust and gravel (`casing`: the office's tarmac is dark, and the index's rule was by tone)
    Tarmac => row(S::Concrete, Hard).structure().casing();
    /// a crate, a chair's shell, a jerrycan
    Plastic => row(S::Plastic, Metal);

    // done only: what the office's paints were to the rules, by what each is
    /// an interior wall: a charge cuts it, a round goes through
    Plasterboard => row(S::Drywall, Hard).wall(Wall::Board).board().structure();
    /// a cubicle's partition: cloth over board
    Partition => row(S::Drywall, Hard).wall(Wall::Screen).board().structure();
    /// ceiling board, a soffit
    Ceiling => row(S::Drywall, Hard).board().structure().paper();
    /// the office's white: a counter, an appliance, a whiteboard, a sheet of paper
    Laminate => row(S::Drywall, Hard).board().structure().counter().paper();
    /// the paint of a pane of glass (a pane itself is the pane engine's, by its flag)
    Pane => row(S::Drywall, Hard).board().structure().paper();
    /// structure: a shell, a column, a blast wall
    Concrete => row(S::Concrete, Hard).structure();
    /// a stone or tiled floor
    Stone => row(S::Concrete, Hard).structure();
    /// a sheet floor
    Lino => row(S::Concrete, Hard).structure();
    /// a rug in a dark dye, and whatever small thing wears its paint (`casing`: by tone, as the index read it)
    #[word("dark carpet")] DarkCarpet => row(S::Carpet, Hard).structure().casing();
    /// a carton
    Card => row(S::Cardboard, Board).cover(Cover::Cardboard);
    /// the office's black: a rack, a bezel, a chair's base
    Casing => row(S::Metal, Hard).casing();
    /// a window's frame, an outside door's leaf
    Frame => row(S::Metal, Hard).structure();
    /// a dead screen's glass
    Screen => row(S::Glass, Hard).casing().screen();
    /// a pot plant, produce, straw
    Plant => row(S::Foliage, Soft);

    // lying snow: the ground to a round and a blast, in any weather
    /// fresh (`paper`: it is pale, and the index's rule was by tone)
    Snow => row(S::Concrete, Hard).structure().snow(SnowKind::Snow).paper();
    /// trodden or driven
    #[word("packed snow")] PackedSnow => row(S::Concrete, Hard).structure().snow(SnowKind::SnowPacked);
    /// slush, the dirty ring round a cleared patch
    #[word("old snow")] OldSnow => row(S::Concrete, Hard).structure().snow(SnowKind::SnowOld);

    // done only: what nobody named, by its tone: how bright, and whether a plain grey (a steel cabinet to a blast)
    #[word("unnamed grey")] UnnamedGrey => row(S::Concrete, Metal).steel();
    #[word("unnamed dark")] UnnamedDark => row(S::Metal, Metal).casing();
    #[word("unnamed light")] UnnamedLight => row(S::Drywall, Metal);
    #[word("unnamed light grey")] UnnamedLightGrey => row(S::Drywall, Metal).steel();
    #[word("unnamed pale")] UnnamedPale => row(S::Drywall, Metal).paper();
    #[word("unnamed pale grey")] UnnamedPaleGrey => row(S::Drywall, Metal).steel().paper();
    // … and the look's plastics and its dark glass, which were given a surface and classed by tone for the rest
    /// black plastic, rubber
    #[word("dark plastic")] DarkPlastic => row(S::Plastic, Metal).casing();
    /// a white plastic chair, grey enough to be a cabinet
    #[word("grey plastic")] GreyPlastic => row(S::Plastic, Metal).steel();
    /// solid dark glass: a vehicle's window
    #[word("dark glass")] DarkGlass => row(S::Glass, Metal).casing();
}

/// WHAT A BOX DOES: its own word's row, else the default row. The one question a rule asks of a material
pub fn does_of(word: Option<DoesWord>) -> &'static Does { word.unwrap_or_default().row() }
