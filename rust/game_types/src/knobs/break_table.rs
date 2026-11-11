//! The numbers of break table.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

/// the knobs that are not a material's: the rigid pool's shares for ONE blast (thrown wall / cover pieces, heap slabs dealt at rest at the wall foot,
///  furniture pieces), what a refused / evicted piece leaves, the china rule
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BreaksK {
    pub share: BreaksKShare,
    #[k([6.0, 8.0])] pub refuse_chips: [f64; 2],
    /// THE EVICTION POP's two answers (both ride the cut switch; off = the pool's oldest piece gone into `crumbleChips` in a ball, as it was): the slot a spawn over
    ///  the cap takes is the SMALLEST resting piece's (its longest half extent; ties the oldest) — a mug's worth goes before a door-sized plank; and what an evicted
    ///  piece leaves is sized to it: `sizedChips` from a piece of `sizedSpan`[0] m (longest half extent) up to one of [1], born ALONG the box, its row's dust once
    #[k([4.0, 14.0])] pub sized_chips: [f64; 2],
    #[k([0.05, 0.5])] pub sized_span: [f64; 2],
    /// a small thing of the plastic row (a mug: nothing over 6 cm) landing this fast is gone into its particles
    #[k(3.5)] pub china_land: f64,
    #[k(0.06)] pub china_max: f64,
    /// how far a thrown piece's paint is taken toward its core colour (the renderer draws a cut face in the box's own colour: until a cut face has its own, the whole
    ///  piece is tinted), and its chips
    #[k(0.3)] pub core_tint: f64,
    #[k(0.5)] pub chip_tint: f64,
    /// the heap: slab size range (m), how far out from the wall foot it is dealt (m), each slab's tip off flat (rad) so overlapping slabs do not lie in one plane
    pub heap: BreaksKHeap,
    /// what stands keeps to the planes it needs
    #[k(4.0)] pub max_standing_planes: f64,
    /// a ragged end's turn ACROSS the piece (through a wall's thickness): steep, because from above that is the side of the end that is seen
    #[k([35.0, 55.0])] pub across_deg: [f64; 2],
    /// a loose fragment under this longest half-extent keeps its plain box (at the game's camera a 12 cm flake is a few pixels: an outline there is cut records for nothing)
    #[k(0.08)] pub loose_min: f64,
    /// the KINKED metal panel (the one "bent" a cut box can draw: two halves of one plate meeting on a fold): the angle between the halves (deg), how far the fold
    ///  may turn off square (deg) and sit off the middle (share of the half length), the shortest plate that kinks (half length, m), and the gap between two (s: one a blast)
    pub kink: BreaksKKink,
    /// FURNITURE'S STEEL (a cabinet painted as steel goes to the table as what it is — sheet steel on a frame — not as one solid block): the sheet's half thickness
    ///  (m); the shallow fold every torn-off sheet carries (`kink`, deg between its halves); a carcass that goes over whole is DENTED along `dents` of its edges, each a
    ///  crease `dent` m deep at one end running out toward the other, never deeper than `dent_most` of the side it bites; and stands OPEN where its door was: that face's
    ///  frame `rim` m wide stands `lip` m back from the dark panel it holds (the paint × `inside`)
    pub steel: BreaksKSteel,
    /// the uprights a plasterboard partition keeps where fire has opened a run of it above a standing sill (one static stand-in each): half width along the wall
    ///  (m), its depth as a share of the wall's, its metal
    /// (`foot`: how far below the sill it starts, m; `does`: what a round makes of it)
    pub stud: BreaksKStud,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BreaksKShare {
    pub big: BreaksKShareBig,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BreaksKShareBig {
    #[k(5.0)] pub thrown: f64,
    #[k(5.0)] pub heap: f64,
    #[k(8.0)] pub furniture: f64,
}

/// the heap: slab size range (m), how far out from the wall foot it is dealt (m), each slab's tip off flat (rad) so overlapping slabs do not lie in one plane
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BreaksKHeap {
    #[k([0.15, 0.55])] pub span: [f64; 2],
    #[k([0.0, 0.7])] pub out: [f64; 2],
    #[k([0.07, 0.24])] pub tip: [f64; 2],
}

/// the KINKED metal panel (the one "bent" a cut box can draw: two halves of one plate meeting on a fold): the angle between the halves (deg), how far the fold
///  may turn off square (deg) and sit off the middle (share of the half length), the shortest plate that kinks (half length, m), and the gap between two (s: one a blast)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BreaksKKink {
    #[k([25.0, 42.0])] pub deg: [f64; 2],
    #[k(20.0)] pub skew: f64,
    #[k(0.4)] pub off: f64,
    #[k(0.2)] pub min_long: f64,
    #[k(0.5)] pub gap_secs: f64,
}

/// FURNITURE'S STEEL (a cabinet painted as steel goes to the table as what it is — sheet steel on a frame — not as one solid block): the sheet's half thickness
///  (m); the shallow fold every torn-off sheet carries (`kink`, deg between its halves); a carcass that goes over whole is DENTED along `dents` of its edges, each a
///  crease `dent` m deep at one end running out toward the other, never deeper than `dent_most` of the side it bites; and stands OPEN where its door was: that face's
///  frame `rim` m wide stands `lip` m back from the dark panel it holds (the paint × `inside`)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BreaksKSteel {
    #[k(0.008)] pub sheet: f64,
    #[k([9.0, 24.0])] pub kink: [f64; 2],
    #[k([3.0, 5.0])] pub dents: [f64; 2],
    #[k([0.025, 0.06])] pub dent: [f64; 2],
    #[k(0.14)] pub dent_most: f64,
    #[k(0.035)] pub rim: f64,
    #[k(0.004)] pub lip: f64,
    #[k(0.16)] pub inside: f64,
}

/// the uprights a plasterboard partition keeps where fire has opened a run of it above a standing sill (one static stand-in each): half width along the wall
///  (m), its depth as a share of the wall's, its metal
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BreaksKStud {
    #[k(0.02)] pub half: f64,
    #[k(0.7)] pub depth_k: f64,
    #[k(0.13)] pub foot: f64,
    #[k([0.56, 0.57, 0.6])] pub albedo: V3,
    #[k(0.35)] pub rough: f64,
    /// the light grey's row: a light thing that puffs like plaster, though a stud is steel
    #[k(crate::scene_materials::DoesWord::UnnamedLightGrey)] pub does: crate::scene_materials::DoesWord,
}
