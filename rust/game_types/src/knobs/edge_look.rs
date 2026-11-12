//! The numbers of edge_look: how a broken wall or piece of cover looks, its ragged standing edges, the rubble at its foot and the soot round a blast's gap.
//! No rule reads them, but the pieces are real boxes: they fill the frame's box list and the fog's records, and boots scuff the rubble.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EdgeLookK {
    /// switch. Off: an opened wall gets none of this table's pieces or soot, and the break table deals its own heap and cuts its own ragged edges again
    #[k(true)] pub on: bool,
    /// most standing pieces in the level: past it the walls dressed longest ago lose all of theirs · most on one wall: past it that wall's next pieces are not made
    #[k(480.0)] pub max_decor: f64,
    #[k(150.0)] pub per_wall: f64,
    /// most rubble pieces in the level: past it the oldest go first
    #[k(520.0)] pub max_rubble: f64,
    /// box rows left free on the frame's list. These pieces get the list's size, less the level's boxes, less the frame's own share, less this: standing pieces first, then
    ///  rubble newest first, until that runs out
    #[k(160.0)] pub box_slack: f64,
    /// m a port's side may stand out of the straight cut (a port is an upper opening with its sill still under it): × 0.7 low down, up to × 1.3 at its top.
    ///  Every side's reach is also capped by `run_k` and by the wall left behind it
    #[k(0.25)] pub port_out: f64,
    /// m a hole's side may stand out (a hole is open down to the floor): the low part of its upper half, and × 1.5 beside the sills · the top of its upper half,
    ///  for a ragged arch
    #[k(0.12)] pub hole_out: f64,
    #[k(0.6)] pub hole_top: f64,
    /// m a torn edge may stand out, along a lintel's underside and the sills' tops: over a hole · over a port
    #[k(0.26)] pub sill_hole: f64,
    #[k(0.14)] pub sill_port: f64,
    /// a side's reach is at most this share of the opening's width (× 0.6 beside the sills), so a narrow gap stays a gap
    #[k(0.27)] pub run_k: f64,
    /// m the band of wall still spanning an opening hangs down, × 0.75 to 1.25 and at most 0.3 of the height over the sill: over a port · over a hole
    #[k(0.42)] pub lintel_port: f64,
    #[k(0.44)] pub lintel_hole: f64,
    /// the odds that a port keeps a lintel where the wall stands at least 1.1 m over the sill (a hole always does, and so does a wall that reaches the ceiling)
    #[k(0.85)] pub lintel_odds: f64,
    /// m: the least depth of a lintel on a wall that reaches the ceiling, plus up to 0.07, because from above only a deep one reads
    #[k(0.52)] pub lintel_cut: f64,
    /// rubble pieces per opened cell: an upper cell · a sill cell (both × `blast.more` from a blast) · a cell of cover
    #[k(9.0)] pub per_upper: f64,
    #[k(6.0)] pub per_sill: f64,
    #[k(2.0)] pub per_cover: f64,
    /// cells shed into rubble per frame, rounded down and at least 1. 0 = all at once, the frame it opens
    #[k(4.0)] pub spread: f64,
    /// m past the wall's face a piece may be thrown: by anything but a blast · by a blast. Most land far short of it: a random share,
    ///  squared (to the power 2.6 from a blast)
    #[k(0.8)] pub throw_fire: f64,
    #[k(0.9)] pub throw_blast: f64,
    pub blast: EdgeLookKBlast,
    pub cut_heap: EdgeLookKCutHeap,
    pub scorch: EdgeLookKScorch,
    /// off: no soot marks are laid, though the pieces round the gap are still sooted
    #[k(true)] pub scorch_decals: bool,
}

/// What a blast changes about a cell's rubble.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EdgeLookKBlast {
    /// × the pieces of an opened cell of a wall (cover gets no more)
    #[k(2.0)] pub more: f64,
    /// the share of its pieces cut as lumps and not flakes (any other cause: 0.34). Wood is splinters either way
    #[k(0.62)] pub lumps: f64,
    /// the share of those lumps cut bigger · × their size, on top of the × 2.5 of every blast's lump
    #[k(0.3)] pub slabs: f64,
    #[k(1.7)] pub slab_k: f64,
    /// the share of its pieces thrown well into the room, 1.2 to 2.8 × `throw_blast`; the rest heap at the wall's foot
    #[k(0.1)] pub flung: f64,
    /// × the throw on the side the blast came from (the far side gets all of it; any other cause: 0.45)
    #[k(0.7)] pub near_k: f64,
    /// × the thickness of its flakes
    #[k(2.0)] pub flake_k: f64,
}

/// The heap a blast leaves at a wall's foot: its big slabs, its small lumps, and the tones of its pieces.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EdgeLookKCutHeap {
    /// the share of its pieces cut as big flat slabs of the wall's own sheet (none off wood, cardboard or fabric)
    #[k(0.07)] pub slabs: f64,
    /// m: a slab's half length, least · most
    #[k([0.15, 0.34])] pub slab: [f64; 2],
    /// m: the most a slab's half thickness can be (it is 0.45 of the wall's half thickness, at least 0.012)
    #[k(0.03)] pub slab_t: f64,
    /// the share of lumps cut small (none off wood, cardboard or fabric)
    #[k(0.42)] pub smalls: f64,
    /// × a small lump's size, least · most
    #[k([0.35, 0.6])] pub small: [f64; 2],
    /// a piece's color goes darker by up to this share
    #[k(0.22)] pub tone: f64,
    /// a piece's broken faces go up to this share of the way to the wall's facing: a little of the paper's brown in some
    #[k(0.45)] pub core_paper: f64,
    /// a piece's broken faces go darker by up to this share
    #[k(0.3)] pub core_tone: f64,
}

/// The soot round a blast's gap.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EdgeLookKScorch {
    /// m: a soot mark's radius on the wall's face, half the gap's width + this: the side the blast came from · the far side
    #[k(1.3)] pub near: f64,
    #[k(1.0)] pub far: f64,
    /// m: a soot mark's radius on the floor: half the gap's width × 0.5 + this on the near side, × 0.4 + 0.7 of this on the far side
    #[k(1.3)] pub floor: f64,
    /// how heavy the marks are: the near side · the far side · × either one on the floor
    #[k(1.0)] pub soot_near: f64,
    #[k(0.75)] pub soot_far: f64,
    #[k(0.5)] pub soot_floor: f64,
    /// m: the radius of the marks on the cut's own edges, × 0.85 to 1.15 (0 = none). Also how far past the gap the standing pieces go darker (0 there = 0.9)
    #[k(0.95)] pub rim: f64,
    /// × the near or far weight, for those marks
    #[k(0.85)] pub soot_rim: f64,
}
