//! The types of `wallbreak`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// who is told of a row of the change feed, on the frame it is filed (wallbreak.ts note): WallReact = the game's one WallReact takes the row into its queue (wallreact.ts:167)
#[derive(Clone, Debug)]
pub enum ChangeListener {
    WallReact,
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LevelChangeKind {
    Door,
    Cover,
    Hole,
    Port,
    Reset,
}
impl LevelChangeKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LevelChangeKind::Door => "door",
            LevelChangeKind::Cover => "cover",
            LevelChangeKind::Hole => "hole",
            LevelChangeKind::Port => "port",
            LevelChangeKind::Reset => "reset",
        }
    }
}

/// (TypeScript: LevelChange, src/game/wallbreak.ts:87)
#[derive(Clone, Debug)]
pub struct LevelChange {
    pub serial: f64,
    pub t: f64,
    pub kind: crate::wallbreak::LevelChangeKind,
    pub cause: crate::wallbreak::LevelChangeCause,
    pub id: f64,
    pub name: Option<String>,
    pub pos: V3,
    pub rect: [f64; 4],
    pub opened: Vec<f64>,
    pub walkable: bool,
    pub sight: bool,
    pub dead: bool,
    pub who: f64,
}
impl LevelChange {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelChange {
            serial: 0.0,
            t: 0.0,
            kind: crate::wallbreak::LevelChangeKind::Door,
            cause: crate::wallbreak::LevelChangeCause::Blast,
            id: 0.0,
            name: None,
            pos: [0.0; 3],
            rect: [0.0; 4],
            opened: Vec::new(),
            walkable: false,
            sight: false,
            dead: false,
            who: 0.0,
        }
    }
}
impl Default for LevelChange { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LevelChangeCause {
    Blast,
    Fire,
    Load,
    Reset,
    Collapse,
}
impl LevelChangeCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            LevelChangeCause::Blast => "blast",
            LevelChangeCause::Fire => "fire",
            LevelChangeCause::Load => "load",
            LevelChangeCause::Reset => "reset",
            LevelChangeCause::Collapse => "collapse",
        }
    }
}

/// (TypeScript: BlastOpts, src/game/wallbreak.ts:113)
#[derive(Clone, Copy, Debug)]
pub struct BlastOpts {
    pub radius: Option<f64>,
    pub power: Option<f64>,
    pub who: Option<f64>,
    pub chaos: Option<bool>,
    pub breach: Option<f64>,
    pub hung: Option<crate::wallbreak::BlastOptsHung>,
}
impl BlastOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastOpts {
            radius: None,
            power: None,
            who: None,
            chaos: None,
            breach: None,
            hung: None,
        }
    }
}
impl Default for BlastOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:129 BlastOpts.hung, src/game/wallbreak.ts:129)
#[derive(Clone, Copy, Debug)]
pub struct BlastOptsHung {
    pub wall: f64,
    pub r: f64,
}
impl BlastOptsHung {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastOptsHung {
            wall: 0.0,
            r: 0.0,
        }
    }
}
impl Default for BlastOptsHung { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallOpening, src/game/wallbreak.ts:132)
#[derive(Clone, Debug)]
pub struct WallOpening {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub standing: Vec<BoxId>,
    pub left: Vec<BoxId>,
    pub cells: f64,
    pub hole: bool,
}
impl WallOpening {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallOpening {
            id: 0.0,
            kind: crate::wallbreak::BKind::Wall,
            standing: Vec::new(),
            left: Vec::new(),
            cells: 0.0,
            hole: false,
        }
    }
}
impl Default for WallOpening { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BKind {
    Wall,
    Cover,
}
impl BKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            BKind::Wall => "wall",
            BKind::Cover => "cover",
        }
    }
}

/// (TypeScript: BlastResult, src/game/wallbreak.ts:135)
#[derive(Clone, Copy, Debug)]
pub struct BlastResult {
    pub walls: f64,
    pub cover: f64,
    pub doors: f64,
    pub panes: f64,
    pub opened: f64,
    pub serial: f64,
}
impl BlastResult {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastResult {
            walls: 0.0,
            cover: 0.0,
            doors: 0.0,
            panes: 0.0,
            opened: 0.0,
            serial: 0.0,
        }
    }
}
impl Default for BlastResult { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BlastEvent, src/game/wallbreak.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct BlastEvent {
    pub at: V3,
    pub radius: f64,
    pub power: f64,
    pub who: f64,
    pub res: crate::wallbreak::BlastResult,
}
impl BlastEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlastEvent {
            at: [0.0; 3],
            radius: 0.0,
            power: 0.0,
            who: 0.0,
            res: crate::wallbreak::BlastResult::zeroed(),
        }
    }
}
impl Default for BlastEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Breakable, src/game/wallbreak.ts:141)
#[derive(Clone, Debug)]
pub struct Breakable {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: crate::scene_materials::Surf,
    pub box_: BoxId,
    pub flags0: f64,
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub yb: Vec<f64>,
    pub open: Vec<u8>,
    pub dmg: Vec<f32>,
    pub slots: Vec<f64>,
    pub live: f64,
    pub n_open: f64,
    pub hard: Option<bool>,
}
impl Breakable {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Breakable {
            id: 0.0,
            kind: crate::wallbreak::BKind::Wall,
            surf: crate::scene_materials::Surf::Carpet,
            box_: BoxId(0),
            flags0: 0.0,
            nx: 0.0,
            ny: 0.0,
            nz: 0.0,
            yb: Vec::new(),
            open: Vec::new(),
            dmg: Vec::new(),
            slots: Vec::new(),
            live: 0.0,
            n_open: 0.0,
            hard: None,
        }
    }
}
impl Default for Breakable { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallsSave, src/game/wallbreak.ts:156)
#[derive(Clone, Debug)]
pub struct WallsSave {
    pub v: f64,
    pub b: Vec<(f64, String, Vec<f64>, Option<f64>)>,
    pub h: Option<Vec<[f64; 2]>>,
}
impl WallsSave {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallsSave {
            v: 0.0,
            b: Vec::new(),
            h: None,
        }
    }
}
impl Default for WallsSave { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AABB, src/game/wallbreak.ts:162)
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl AABB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AABB {
            x0: 0.0,
            x1: 0.0,
            y0: 0.0,
            y1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for AABB { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallBreak, src/game/wallbreak.ts:169)
#[derive(Clone, Debug)]
pub struct WallBreak {
    pub authored: f64,
    pub serial: f64,
    pub changes: Vec<crate::wallbreak::LevelChange>,
    pub listeners: Vec<crate::wallbreak::ChangeListener>,
    pub list: Vec<crate::wallbreak::Breakable>,
    pub owner: JsMap<crate::Num, crate::wallbreak::Breakable>,
    pub hidden: JsMap<crate::Num, f64>,
    pub counter: f64,
    pub dirty: bool,
    pub scratch: Vec<i32>,
    pub scratch2: Vec<i32>,
    pub fixture_boxes: JsSet<crate::Num>,
    pub ghosts: Vec<f64>,
    pub wear: crate::wallwear::WallWear,
    pub room: crate::levelfit::BreakRoom,
    pub shell_gaps: JsSet<crate::Num>,
    pub hung_now: Option<crate::wallbreak::WallBreakHungNow>,
    pub hung_gone: JsMap<crate::Num, f64>,
}
impl WallBreak {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        WallBreak {
            authored: 0.0,
            serial: 0.0,
            changes: Vec::new(),
            listeners: Vec::new(),
            list: Vec::new(),
            owner: JsMap::new(),
            hidden: JsMap::new(),
            counter: 1.0,
            dirty: false,
            scratch: alloc::vec![0; 96],
            scratch2: alloc::vec![0; 96],
            fixture_boxes: JsSet::new(),
            ghosts: Vec::new(),
            wear: crate::wallwear::WallWear::blank(),
            room: crate::levelfit::BreakRoom::zeroed(),
            shell_gaps: JsSet::new(),
            hung_now: None,
            hung_gone: JsMap::new(),
        }
    }
}
impl Default for WallBreak { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/wallbreak.ts:576 WallBreak.hungNow, src/game/wallbreak.ts:576)
#[derive(Clone, Debug)]
pub struct WallBreakHungNow {
    pub wall: f64,
    pub at: V3,
    pub n: V3,
    pub set: Vec<f64>,
}
impl WallBreakHungNow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakHungNow {
            wall: 0.0,
            at: [0.0; 3],
            n: [0.0; 3],
            set: Vec::new(),
        }
    }
}
impl Default for WallBreakHungNow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:254 inventory ->, src/game/wallbreak.ts:254)
#[derive(Clone, Copy, Debug)]
pub struct WallBreakInventory {
    pub walls: f64,
    pub cover: f64,
    pub wall_m: f64,
    pub pieces: f64,
    pub hidden: f64,
    pub touched: f64,
}
impl WallBreakInventory {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakInventory {
            walls: 0.0,
            cover: 0.0,
            wall_m: 0.0,
            pieces: 0.0,
            hidden: 0.0,
            touched: 0.0,
        }
    }
}
impl Default for WallBreakInventory { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:259 breakableAt ->, src/game/wallbreak.ts:259)
#[derive(Clone, Copy, Debug)]
pub struct WallBreakBreakableAt {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: crate::scene_materials::Surf,
    pub open: f64,
    pub cells: f64,
    pub hard: Option<bool>,
}
impl WallBreakBreakableAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakBreakableAt {
            id: 0.0,
            kind: crate::wallbreak::BKind::Wall,
            surf: crate::scene_materials::Surf::Carpet,
            open: 0.0,
            cells: 0.0,
            hard: None,
        }
    }
}
impl Default for WallBreakBreakableAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:305 all ->, src/game/wallbreak.ts:305)
#[derive(Clone, Copy, Debug)]
pub struct WallBreakAllItem {
    pub id: f64,
    pub kind: crate::wallbreak::BKind,
    pub surf: crate::scene_materials::Surf,
    pub box_: BoxId,
    pub open: f64,
    pub cells: f64,
}
impl WallBreakAllItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakAllItem {
            id: 0.0,
            kind: crate::wallbreak::BKind::Wall,
            surf: crate::scene_materials::Surf::Carpet,
            box_: BoxId(0),
            open: 0.0,
            cells: 0.0,
        }
    }
}
impl Default for WallBreakAllItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:332 rectBox ->, src/game/wallbreak.ts:332)
#[derive(Clone, Copy, Debug)]
pub struct WallBreakRectBox {
    pub c: V3,
    pub h: V3,
}
impl WallBreakRectBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakRectBox {
            c: [0.0; 3],
            h: [0.0; 3],
        }
    }
}
impl Default for WallBreakRectBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallBreakOpenCellsCause {
    Blast,
    Fire,
    Load,
    Reset,
    Collapse,
}
impl WallBreakOpenCellsCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallBreakOpenCellsCause::Blast => "blast",
            WallBreakOpenCellsCause::Fire => "fire",
            WallBreakOpenCellsCause::Load => "load",
            WallBreakOpenCellsCause::Reset => "reset",
            WallBreakOpenCellsCause::Collapse => "collapse",
        }
    }
}

/// (TypeScript: inline at src/game/wallbreak.ts:422 raze(budget), src/game/wallbreak.ts:422)
#[derive(Clone, Copy, Debug)]
pub struct WallBreakRazeBudget {
    pub chunks: f64,
}
impl WallBreakRazeBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakRazeBudget {
            chunks: 0.0,
        }
    }
}
impl Default for WallBreakRazeBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:425 razeIn(budget), src/game/wallbreak.ts:425)
#[derive(Clone, Copy, Debug)]
pub struct WallBreakRazeInBudget {
    pub chunks: f64,
}
impl WallBreakRazeInBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakRazeInBudget {
            chunks: 0.0,
        }
    }
}
impl Default for WallBreakRazeInBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:449 note(c), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Debug)]
pub struct WallBreakNoteC {
    pub dead: bool,
    pub kind: crate::wallbreak::LevelChangeKind,
    pub cause: crate::wallbreak::WallBreakNoteCCause,
    pub name: Option<String>,
    pub who: f64,
    pub pos: V3,
    pub id: f64,
    pub sight: bool,
    pub rect: [f64; 4],
    pub opened: Vec<f64>,
    pub walkable: bool,
}
impl WallBreakNoteC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakNoteC {
            dead: false,
            kind: crate::wallbreak::LevelChangeKind::Door,
            cause: crate::wallbreak::WallBreakNoteCCause::Blast,
            name: None,
            who: 0.0,
            pos: [0.0; 3],
            id: 0.0,
            sight: false,
            rect: [0.0; 4],
            opened: Vec::new(),
            walkable: false,
        }
    }
}
impl Default for WallBreakNoteC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallBreakNoteCCause {
    Blast,
    Fire,
    Load,
    Reset,
    Collapse,
}
impl WallBreakNoteCCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallBreakNoteCCause::Blast => "blast",
            WallBreakNoteCCause::Fire => "fire",
            WallBreakNoteCCause::Load => "load",
            WallBreakNoteCCause::Reset => "reset",
            WallBreakNoteCCause::Collapse => "collapse",
        }
    }
}

/// (TypeScript: inline at src/game/wallbreak.ts:656 planBlast ->, src/game/wallbreak.ts:656)
#[derive(Clone, Debug)]
pub struct WallBreakPlanBlastItem {
    pub b: crate::wallbreak::Breakable,
    pub cells: Vec<f64>,
    pub d: f64,
}
impl WallBreakPlanBlastItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakPlanBlastItem {
            b: crate::wallbreak::Breakable::zeroed(),
            cells: Vec::new(),
            d: 0.0,
        }
    }
}
impl Default for WallBreakPlanBlastItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallbreak.ts:663 planOf ->, src/game/wallbreak.ts:663)
#[derive(Clone, Debug)]
pub struct WallBreakPlanOfItem {
    pub b: crate::wallbreak::Breakable,
    pub cells: Vec<f64>,
    pub d: f64,
}
impl WallBreakPlanOfItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallBreakPlanOfItem {
            b: crate::wallbreak::Breakable::zeroed(),
            cells: Vec::new(),
            d: 0.0,
        }
    }
}
impl Default for WallBreakPlanOfItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WallbreakMod {
    /// (src/game/wallbreak.ts:159 WALL_LOG)
    pub wall_log: Vec<String>,
}
impl WallbreakMod {
    pub fn blank() -> Self {
        WallbreakMod {
            wall_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WallbreakStats {
    pub rounds: i32,
    pub ports: i32,
    pub holes: i32,
    pub cover_cells: i32,
    pub cover_dead: i32,
    pub blasts: i32,
    pub refused: i32,
    pub falls: i32,
    pub floating: i32,
    pub chunks: i32,
    pub chunks_demoted: i32,
    pub nav_opened: i32,
}

/// (src/game/wallbreak.ts:80 WALLS_DEFAULTS: a table nothing writes)
pub fn walls_defaults() -> crate::knobs::WallsK { crate::knobs::page_walls() }

/// (src/game/wallbreak.ts:161 HIDE)
pub const HIDE: f64 = 3.0;
