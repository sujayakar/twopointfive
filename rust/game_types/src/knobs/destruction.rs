//! The numbers of destruction.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

/// ------------------------------------------------------------------ knobs
/// The lane's numbers.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosK {
    #[k(0.5)] pub def: f64,
    /// s: the beaten-zone memory
    #[k(2.0)] pub heat_tau: f64,
    /// m: heat grid cell
    #[k(2.0)] pub cell: f64,
    /// thresholds on E = heat × 2 × chaos
    #[k(1.0)] pub t_decal: f64,
    #[k(1.0)] pub t_paper: f64,
    #[k(1.5)] pub t_burst: f64,
    #[k(1.5)] pub t_puff: f64,
    #[k(4.0)] pub t_knock: f64,
    #[k(9.0)] pub t_tile: f64,
    /// what a stun charge pours in where it went off (→ E 9 at chaos 0.5: a tile, the desk cleared, papers up)
    #[k(9.0)] pub bang_heat: f64,
    /// per-second caps (token buckets) and level-wide caps
    #[k(8.0)] pub puffs_per_sec: f64,
    #[k(2.0)] pub puffs_per_frame: f64,
    #[k(6.0)] pub papers_per_sec: f64,
    #[k(4.0)] pub knocks_per_sec: f64,
    #[k(2.2)] pub tile_gap_secs: f64,
    #[k(8.0)] pub tiles_per_encounter: f64,
    #[k(12.0)] pub decals_per_sec: f64,
    /// a hit within this of a stack (or through the space over it) scatters it
    #[k(0.75)] pub paper_reach_m: f64,
    /// desk clutter within this of a hit goes
    #[k(0.45)] pub knock_reach_m: f64,
    /// sheet particles a converted stack becomes (× the stack's size)
    #[k(26.0)] pub sheets_per_stack: f64,
    /// the smoke solver's dust puff (coarse lattice): optically THIN — a pale haze the beams light, not a sooty plume (density 11 self-shadowed black in the first captures)
    pub puff: ChaosKPuff,
    pub tile: ChaosKTile,
    /// knock-off speed scale at chaos ≤ 0.5, and the multiplier reached at chaos 1
    #[k(1.0)] pub vel_low: f64,
    #[k(2.5)] pub vel_high_k: f64,
    /// IMPULSES (strength = the Δv m/s a sheet at the centre gets, radius m, up = the upward share on the radial push; render/sheetModel.ts flies the reference the numbers were
    /// picked on: a charge lifts the leaves within a metre well over a metre and lands them a stride off, hops the ones at two metres knee height, shivers the ones at three
    /// and a half and leaves the ones at four; a round hops the leaf beside it a few centimetres and scoots it a span; a thump twitches what lies within a stride)
    pub imp: ChaosKImp,
    /// THE LOUD LEVELS. Fixtures die to ANYBODY's rounds and to blasts,
    /// the panel over a beaten zone goes with the tile beside it, and all of it is capped per encounter so a long fight cannot put the whole floor in the dark.
    pub loud: ChaosKLoud,
    /// A BLAST'S PALL (the maintainer, 10-14: "i'd also expect much more dust/smoke from a wall charge" / "same goes for 40mm" / "and frag"). A charge's dust was the
    ///  round's puff, a few times over, under the round's caps: a thin haze for a second. ON: a wall charge and every burst that
    ///  comes through blast() with its chaos (the hand grenade, the 40 mm, a bomb → pallBurst) also raise a PALL — a handful of wide, slow, seconds-long emitters
    ///  into the smoke solver, on the charge's side of the wall and, through the hole, on the far side; in a ring round a burst with a darker column over it — and
    ///  a slow big layer of particle dust with it. The solver's part is real density: every sight line that crosses it reads it, the player's picture and the
    ///  men's eyes alike (one model), so for a few seconds neither side sees through it; it thins by the solver's own decay. `solver` false = the particle layer
    ///  alone (picture only: nobody is hidden by it). Its own allowance, not the puffs': `perWindow` palls in any `windowSecs` (one more burst in the window
    ///  raises the old dust alone — six, not three: his word on it, 10-14, "it should feel pretty chaotic with HE going off"; the wall charge is never the one refused, and counts), and no emitter — nor the particle layer over it — while the solver
    ///  has fewer than `slack` slots free. Not under a level script's show. The dice are a hash of the place and the moment: none of the lane's are drawn, so
    ///  with the solver a null object (the harness) a run is the same run with the rule on or off. An emitter keeps `clear` radii off anything solid (judged
    ///  at `clearY` m), shrinking to `minRadius` of its radius at the least with its density up to `maxGain` times for it (pallEmit).
    ///  Rows: `near` / `far` emitters either side of the wall (far only through a hole), `ring` round a burst (× its size: 1 = the hand grenade's 2.5 m) and
    ///  one darker column over it (`coreGain` × the density, `smoke` its tint), each `radius` m wide (gaussian), `density` /s at the core for `ttl` s, thrown
    ///  `speed` m/s, `temp` its lift, `push` / `expand` as any emitter's, scattered `spread` m, standing `out` m off the wall; `dust` particles (near, far |
    ///  round a burst) living `dustLife` s, `dustSize` m, `dustSpeed`. Measured on the page (docs/agents/destruction.md): hidden across it for 3 to 4 s.
    ///  On (?pall=0 = the dust as it was, ?pall=look = the particle layer alone).
    pub pall: ChaosKPall,
    /// WHAT A BLAST AND A BROKEN THING LEAVE ON THE FLOOR. OFF (the module's default, the frozen stand, a
    ///  recording from before the key): a blast leaves its scorch and the chunks the pool still holds, and the rest of the dust is gone with the air. ON: where a
    ///  blast's pall stood a pale film of settled dust and grit lies on the floor (and on the desk tops under it), shaped to the room — a heart at the burst, lobes
    ///  round it, none of them across a standing wall — with the soot rayed out of its middle; a wall charge lays one each side it reached; a wall or a piece of
    ///  cover that gives under fire leaves a small one at its foot; and a chunk the debris pool takes for its slot leaves the crushed mark of itself where it lay.
    ///  ALL OF IT is marks laid only for the picture.
    pub mess: ChaosKMess,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKPuff {
    /// the smoke solver's dust puff (coarse lattice): optically THIN — a pale haze the beams light, not a sooty plume (density 11 self-shadowed black in the first captures)
    #[k(0.2)] pub radius: f64,
    #[k(3.5)] pub density: f64,
    #[k(0.35)] pub ttl: f64,
    #[k(0.55)] pub speed: f64,
    #[k(0.35)] pub temp: f64,
    #[k(6.0)] pub push: f64,
    #[k(8.0)] pub expand: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKTile {
    #[k([0.29, 0.0075, 0.29])] pub half: V3,
    #[k([0.66, 0.66, 0.63])] pub albedo: V3,
}

/// IMPULSES (strength = the Δv m/s a sheet at the centre gets, radius m, up = the upward share on the radial push; render/sheetModel.ts flies the reference the numbers were
/// picked on: a charge lifts the leaves within a metre well over a metre and lands them a stride off, hops the ones at two metres knee height, shivers the ones at three
/// and a half and leaves the ones at four; a round hops the leaf beside it a few centimetres and scoots it a span; a thump twitches what lies within a stride)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKImp {
    pub bang: ChaosKImpBang,
    /// biasK × the round's travel along the surface it struck; a chunk within debrisM is nudged
    pub impact: ChaosKImpImpact,
    /// body falls / a boot through a door / furniture knocking: ≤ perFrame a frame, the rest not worth queuing
    pub thud: ChaosKImpThud,
    /// strength × (1 at chaos ≤ 0.5 → chaosK at chaos 1), the knock-offs' velocity policy again
    #[k(1.5)] pub chaos_k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKImpBang {
    #[k(4.0)] pub radius: f64,
    #[k(3.0)] pub strength: f64,
    #[k(2.0)] pub up: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKImpImpact {
    /// biasK × the round's travel along the surface it struck; a chunk within debrisM is nudged
    #[k(0.6)] pub radius: f64,
    #[k(1.5)] pub strength: f64,
    #[k(0.5)] pub up: f64,
    #[k(0.6)] pub bias_k: f64,
    #[k(0.5)] pub debris_m: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKImpThud {
    /// body falls / a boot through a door / furniture knocking: ≤ perFrame a frame, the rest not worth queuing
    #[k(1.0)] pub radius: f64,
    #[k(0.8)] pub strength: f64,
    #[k(0.6)] pub up: f64,
    #[k(4.0)] pub per_frame: f64,
}

/// THE LOUD LEVELS. Fixtures die to ANYBODY's rounds and to blasts,
/// the panel over a beaten zone goes with the tile beside it, and all of it is capped per encounter so a long fight cannot put the whole floor in the dark.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKLoud {
    /// a round passing this close to a fixture (or stopping in its box) is in it — the AI's rule; the player keeps combat.ts's wider aim-assisted one
    #[k(0.18)] pub fixture_hit_m: f64,
    /// a charge this close to a fixture, with a clear line to it, breaks it (× √power): from the floor, the ceiling panel within ~1.3 m of straight overhead, a desk lamp across the room's corner
    #[k(3.2)] pub fixture_blast_m: f64,
    /// …a STUN charge only this close: the lamp it lands right beside. A flash and a report are not a blast — the room it stunned stays lit (3.2 with stunPanels on = a stun as it was: every lit fixture in the charge's reach)
    #[k(0.6)] pub stun_fixture_m: f64,
    /// …but no more than this many die to stray fire, blasts and shaken ceilings in one encounter (the player's own aimed shots are not counted)
    #[k(10.0)] pub fixtures_per_encounter: f64,
    /// and never two inside this (blasts aside)
    #[k(0.8)] pub fixture_gap_secs: f64,
    /// the puffs a charge raises off the floor it lay on (scale; the ceiling's is 1.6) …
    #[k(1.2)] pub blast_dust_floor: f64,
    /// … this many (× power), spread over the next frames inside the puff caps (dustBurst)
    #[k(2.0)] pub blast_dust_puffs: f64,
    /// a round's heat counts this much under a scenario: a firefight between men who mostly HIT puts few rounds into the walls (6 to 20 a run, measured), so the
    /// first round into plaster already raises its dust and a short burst clears the desk — the office's own escalation (a held trigger) is left as tuned
    #[k(2.0)] pub heat_k: f64,
    /// dustBurst: at most this many puffs waiting, none older than this
    #[k(32.0)] pub burst_queue: f64,
    #[k(3.0)] pub burst_secs: f64,
}

/// A BLAST'S PALL (the maintainer, 10-14: "i'd also expect much more dust/smoke from a wall charge" / "same goes for 40mm" / "and frag"). A charge's dust was the
///  round's puff, a few times over, under the round's caps: a thin haze for a second. ON: a wall charge and every burst that
///  comes through blast() with its chaos (the hand grenade, the 40 mm, a bomb → pallBurst) also raise a PALL — a handful of wide, slow, seconds-long emitters
///  into the smoke solver, on the charge's side of the wall and, through the hole, on the far side; in a ring round a burst with a darker column over it — and
///  a slow big layer of particle dust with it. The solver's part is real density: every sight line that crosses it reads it, the player's picture and the
///  men's eyes alike (one model), so for a few seconds neither side sees through it; it thins by the solver's own decay. `solver` false = the particle layer
///  alone (picture only: nobody is hidden by it). Its own allowance, not the puffs': `perWindow` palls in any `windowSecs` (one more burst in the window
///  raises the old dust alone — six, not three: his word on it, 10-14, "it should feel pretty chaotic with HE going off"; the wall charge is never the one refused, and counts), and no emitter — nor the particle layer over it — while the solver
///  has fewer than `slack` slots free. Not under a level script's show. The dice are a hash of the place and the moment: none of the lane's are drawn, so
///  with the solver a null object (the harness) a run is the same run with the rule on or off. An emitter keeps `clear` radii off anything solid (judged
///  at `clearY` m), shrinking to `minRadius` of its radius at the least with its density up to `maxGain` times for it (pallEmit).
///  Rows: `near` / `far` emitters either side of the wall (far only through a hole), `ring` round a burst (× its size: 1 = the hand grenade's 2.5 m) and
///  one darker column over it (`coreGain` × the density, `smoke` its tint), each `radius` m wide (gaussian), `density` /s at the core for `ttl` s, thrown
///  `speed` m/s, `temp` its lift, `push` / `expand` as any emitter's, scattered `spread` m, standing `out` m off the wall; `dust` particles (near, far |
///  round a burst) living `dustLife` s, `dustSize` m, `dustSpeed`. Measured on the page (docs/agents/destruction.md): hidden across it for 3 to 4 s.
///  On (?pall=0 = the dust as it was, ?pall=look = the particle layer alone).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKPall {
    #[k(6.0)] pub per_window: f64,
    #[k(4.0)] pub window_secs: f64,
    #[k(8.0)] pub slack: f64,
    #[k(1.8)] pub clear: f64,
    #[k(1.7)] pub clear_y: f64,
    #[k(0.5)] pub min_radius: f64,
    #[k(2.0)] pub max_gain: f64,
    pub wall: ChaosKPallWall,
    pub burst: ChaosKPallBurst,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKPallWall {
    #[k(4.0)] pub near: f64,
    #[k(3.0)] pub far: f64,
    #[k(0.45)] pub radius: f64,
    #[k(0.8)] pub density: f64,
    #[k(1.5)] pub ttl: f64,
    #[k(1.6)] pub speed: f64,
    #[k(0.25)] pub temp: f64,
    #[k(5.0)] pub push: f64,
    #[k(1.2)] pub expand: f64,
    #[k(0.9)] pub spread: f64,
    #[k(0.5)] pub out: f64,
    #[k([36.0, 24.0])] pub dust: [f64; 2],
    #[k(4.5)] pub dust_life: f64,
    #[k(0.6)] pub dust_size: f64,
    #[k(1.2)] pub dust_speed: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKPallBurst {
    #[k(5.0)] pub ring: f64,
    #[k(1.3)] pub core_gain: f64,
    #[k(0.42)] pub radius: f64,
    #[k(0.85)] pub density: f64,
    #[k(1.2)] pub ttl: f64,
    #[k(1.4)] pub speed: f64,
    #[k(0.3)] pub temp: f64,
    #[k(5.0)] pub push: f64,
    #[k(1.5)] pub expand: f64,
    #[k(0.6)] pub spread: f64,
    #[k(48.0)] pub dust: f64,
    #[k(3.8)] pub dust_life: f64,
    #[k(0.5)] pub dust_size: f64,
    #[k(1.8)] pub dust_speed: f64,
    #[k([0.34, 0.33, 0.32])] pub smoke: [f64; 3],
}

/// WHAT A BLAST AND A BROKEN THING LEAVE ON THE FLOOR. OFF (the module's default, the frozen stand, a
///  recording from before the key): a blast leaves its scorch and the chunks the pool still holds, and the rest of the dust is gone with the air. ON: where a
///  blast's pall stood a pale film of settled dust and grit lies on the floor (and on the desk tops under it), shaped to the room — a heart at the burst, lobes
///  round it, none of them across a standing wall — with the soot rayed out of its middle; a wall charge lays one each side it reached; a wall or a piece of
///  cover that gives under fire leaves a small one at its foot; and a chunk the debris pool takes for its slot leaves the crushed mark of itself where it lay.
///  ALL OF IT is marks laid only for the picture.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMess {
    /// what STANDS in dust's way is whatever meets the band standY‥clearY (a wall, a pane, a cabinet; not a desk, a sill or a door's header) · how far a film
    ///  tucks under what it meets · the least film worth laying · two films nearer than this × the smaller one's reach are one · a burst above this leaves none
    #[k(1.05)] pub stand_y: f64,
    #[k(1.7)] pub clear_y: f64,
    #[k(0.06)] pub under: f64,
    #[k(0.28)] pub min_radius: f64,
    #[k(0.8)] pub crowd: f64,
    #[k(2.2)] pub roof_y: f64,
    /// a burst (the hand grenade = size 1; the 40 mm and a bomb by their radius): the heart's reach, how many lobes, how far out and how wide, how thick
    pub burst: ChaosKMessBurst,
    /// a wall charge: the charge's side, and the side its wall went to (only when it went through)
    pub wall: ChaosKMessWall,
    /// a wall / cover cell opened by fire or a fall: one patch each side of what opened
    pub foot: ChaosKMessFoot,
    /// a desk top (anything flat between these heights) under a film takes its own, this much of the floor's
    pub top: ChaosKMessTop,
    /// a chunk the pool took: nothing under this size · the mark's reach = its longest half extent × grow, between min and max · none within crowd × reach of another
    pub rubble: ChaosKMessRubble,
}

/// a burst (the hand grenade = size 1; the 40 mm and a bomb by their radius): the heart's reach, how many lobes, how far out and how wide, how thick
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMessBurst {
    #[k(1.15)] pub core: f64,
    #[k(6.0)] pub lobes: f64,
    #[k(1.25)] pub reach: f64,
    #[k(0.95)] pub lobe: f64,
    #[k(0.8)] pub strength: f64,
}

/// a wall charge: the charge's side, and the side its wall went to (only when it went through)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMessWall {
    #[k(0.55)] pub out: f64,
    pub near: ChaosKMessWallNear,
    pub far: ChaosKMessWallFar,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMessWallNear {
    #[k(1.0)] pub core: f64,
    #[k(4.0)] pub lobes: f64,
    #[k(1.1)] pub reach: f64,
    #[k(0.85)] pub lobe: f64,
    #[k(0.7)] pub strength: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMessWallFar {
    #[k(1.2)] pub core: f64,
    #[k(5.0)] pub lobes: f64,
    #[k(1.4)] pub reach: f64,
    #[k(0.95)] pub lobe: f64,
    #[k(0.9)] pub strength: f64,
}

/// a wall / cover cell opened by fire or a fall: one patch each side of what opened
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMessFoot {
    #[k(0.32)] pub out: f64,
    #[k(0.5)] pub radius: f64,
    #[k(0.25)] pub per_metre: f64,
    #[k(0.5)] pub strength: f64,
}

/// a desk top (anything flat between these heights) under a film takes its own, this much of the floor's
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMessTop {
    #[k(0.08)] pub lo: f64,
    #[k(1.4)] pub hi: f64,
    #[k(0.75)] pub strength: f64,
}

/// a chunk the pool took: nothing under this size · the mark's reach = its longest half extent × grow, between min and max · none within crowd × reach of another
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ChaosKMessRubble {
    #[k(0.035)] pub least: f64,
    #[k(1.5)] pub grow: f64,
    #[k(0.09)] pub min: f64,
    #[k(0.42)] pub max: f64,
    #[k(0.7)] pub crowd: f64,
    #[k(0.85)] pub strength: f64,
}
