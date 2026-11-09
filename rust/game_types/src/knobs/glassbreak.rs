//! The numbers of glassbreak.
use crate::bytes::ToBytes;
use crate::js::*;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum GlassbreakKRhoKey {
    Annealed,
    Laminated,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum GlassbreakKRadialsKey {
    Annealed,
    Laminated,
}

/// the knobs
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct GlassbreakK {
    #[k(8.0)] pub max_sites: f64,
    /// energy (1 = one pistol round square on) a pane takes before it gives: annealed = its radials RUN to the frame (daggers, the support rule); tempered = it dices
    pub break_e: GlassbreakKBreakE,
    /// a blow at least this hard punches the crater out of annealed glass / knocks the piece it lands on out; a softer one only cracks
    #[k(0.5)] pub punch_e: f64,
    /// one blow this hard runs an annealed pane by itself
    #[k(2.5)] pub run_single_e: f64,
    /// the blow that RUNS a pane breaks its own star out: this many of the rings round the site it landed on (0‥2) follow the crater, runRingSecs apart
    #[k(2.0)] pub run_star: f64,
    #[k(0.03)] pub run_ring_secs: f64,
    /// falling glass is heard (one crack + one 'glass' hearing event) at most once per pane per this many seconds
    #[k(0.25)] pub crack_gap: f64,
    /// base radii (m) of the three chords round a blow, by type — each ≥ 1.57× the last (glassPattern.ts: the chords of a sector must nest) — and [3] the base distance of
    ///  the daggers' snaps (annealed) / of the web's outer ring (laminated); × (0.75 + 0.25 e) clamped 0.8‥1.4
    #[k({ let mut t = JsMap::new(); t.set(GlassbreakKRhoKey::Annealed, [0.03, 0.07, 0.14, 0.5]); t.set(GlassbreakKRhoKey::Laminated, [0.018, 0.05, 0.11, 0.22]); t })] pub rho: JsMap<GlassbreakKRhoKey, [f64; 4]>,
    /// radial cracks a blow starts: [fewest, spread] (≤ 12 in all), and the share of an annealed site's sectors with no middle chord
    #[k({ let mut t = JsMap::new(); t.set(GlassbreakKRadialsKey::Annealed, [9.0, 4.0]); t.set(GlassbreakKRadialsKey::Laminated, [10.0, 3.0]); t })] pub radials: JsMap<GlassbreakKRadialsKey, [f64; 2]>,
    #[k(0.45)] pub merge: f64,
    /// a new site's ring zone is shrunk to the room the others leave; under this (m) there is no room and the blow counts on the nearest site instead
    #[k(0.05)] pub rz_min: f64,
    /// SUPPORT: seconds between a change and the pass that drops what it left unheld; each piece then goes after a further stagger (distance from the blow + a hash)
    #[k(0.2)] pub think: f64,
    #[k(0.18)] pub stagger_per_m: f64,
    #[k(0.12)] pub stagger_jit: f64,
    /// a piece that is not STANDING is held only up to this area (m²) per metre of grip — its edge on a frame side × grip.side, on the top × grip.top, against a held neighbour × grip.wedged
    #[k(0.12)] pub hang_area: f64,
    pub grip: GlassbreakKGrip,
    /// a piece STANDS on the sill / on the held neighbour below it across a crack flatter than standNy (the edge normal's downward share) when that footing is at least
    ///  standFoot of its own width and its centre is over it (± balanceTol m)
    #[k(0.7)] pub stand_ny: f64,
    #[k(0.3)] pub stand_foot: f64,
    #[k(0.02)] pub balance_tol: f64,
    /// blast(): default reach (m) to a pane's nearest point, and the power a stun charge's 'bang' carries; a TOOTH — a piece on the frame under
    ///  toothArea m² — is left in by a blast weaker than toothK there (the jagged rim a blown window keeps)
    #[k(2.6)] pub blast_m: f64,
    #[k(0.6)] pub bang_power: f64,
    #[k(0.03)] pub tooth_area: f64,
    #[k(0.8)] pub tooth_k: f64,
    /// blast(): the strength k = power × (1 − d ∕ radius) past which each kind gives — annealed RUNS (plain float glass goes first), toughened glass DICES (four or five times
    ///  the strength: only close in), a laminated sheet leaves its frame whole. Tuning round two: they were 0.35 / 0.2 / 0.75 as literals, which had a stun charge 1.2 m off dice
    ///  a toughened facade sheet and only star a float partition — the wrong way round
    #[k(0.2)] pub blast_run_k: f64,
    #[k(0.45)] pub blast_dice_k: f64,
    #[k(0.75)] pub blast_sheet_k: f64,
    /// RAKING (tuning round two): Sam walking into what is left of an annealed pane that has run and is at least rakeOpen open takes every piece in his way with him (he alone — glassbreak.ts rake() — all
    ///  but the teeth, everything reaching below walkClearH) — a magazine left jagged remains standing on the sill and the opening could never be stepped through. Heard as
    ///  falling glass. 0 = off: the remains stand until a round or a blast takes them
    #[k(0.5)] pub rake_open: f64,
    #[k(0.5)] pub rake_m: f64,
    /// tempered: the dice drawn and handed on as `grain` (m), the clump a diced pane lets go in (m), the wave's delay and speed outward from the blow
    /// (round two: 0.05 / 5 — the sheet frosts for a beat before the curtain comes down, and comes down a little slower)
    #[k(0.012)] pub dice_grain: f64,
    #[k(0.12)] pub clump_m: f64,
    #[k(0.09)] pub dice_hang: f64,
    #[k(4.2)] pub dice_speed: f64,
    #[k(40.0)] pub clumps_per_frame: f64,
    /// queued pieces let go per frame (a round's own piece is never deferred)
    #[k(8.0)] pub releases_per_frame: f64,
    /// how a piece leaves: out along the blow (m/s, × 0.4‥1.4), spin (rad/s); a blast throws at blastOut × its strength there
    #[k(0.7)] pub out: f64,
    #[k(5.0)] pub spin: f64,
    #[k(5.0)] pub blast_out: f64,
    /// the STAND-IN while no onRelease consumer is attached (the base's shards): thin Debris boxes + pale chips
    #[k([0.72, 0.8, 0.82])] pub shard_albedo: V3,
    #[k([0.8, 0.88, 0.9])] pub chip_color: V3,
    #[k(90.0)] pub chips_per_m2: f64,
    #[k(8.0)] pub chips_min: f64,
    #[k(260.0)] pub dice_chips_per_m2: f64,
    #[k(12.0)] pub crumb_every: f64,
    #[k(0.6)] pub gust_speed_k: f64,
    /// a pane is walked through once every piece reaching below this height (m above the floor) has fallen
    #[k(1.9)] pub walk_clear_h: f64,
}

/// energy (1 = one pistol round square on) a pane takes before it gives: annealed = its radials RUN to the frame (daggers, the support rule); tempered = it dices
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GlassbreakKBreakE {
    #[k(3.0)] pub annealed: f64,
    #[k(1.0)] pub tempered: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GlassbreakKGrip {
    #[k(2.0)] pub side: f64,
    #[k(1.0)] pub top: f64,
    #[k(0.25)] pub wedged: f64,
}

/// GLASS AS BUILDINGS HAVE IT (two rules, each ON in the page — `?glassdrop=0`, `?glassplace=0` — and off here and on the frozen stand, `TPS_GLASSDROP=1`,
///    drop  — toughened glass lets go ALL AT ONCE. Its stored stress crazes the whole sheet in a millisecond; nothing travels across it that an eye could follow. So
///            the sheet stands white for `hang` seconds and then every clump leaves on the one step (stepDice → dropAll), handed on as at most `blocks` fragments
///            that fall: out along the blow a little, no push across the pane. (The wave was the same release spread over half a second at diceSpeed.)
///            HEARD ONCE: `hang` is kept under GLASSBREAK.crackGap, so the sheet's breaking is the one crack and the one hearing event the blow's own clump files
///            (emit); its fall files no second, and the landings are falling glass's own sounds. A hang past crackGap would file a second of each.
///    place — which glass a pane is comes from WHERE IT SITS, as a glazier would have it: glass a body could fall into is safety glass. A pane whose foot is under
///            `lowM` above the floor (it reaches the floor: a partition, a shopfront, frontage on a kick wall) or that is a panel of a door (within `doorM` of a
///            door's opening on the plan and its foot under `doorFootM`: glass beside a door within a body's reach, not a light over it) is TOUGHENED; a window that starts at a sill is plain FLOAT glass, outside wall or not. Narrow inside
///            sidelights and high outside strips stay laminated, as they were. (The first rule read the wall instead: outside = toughened, inside = float.)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GlassrealK {
    #[k(0.1)] pub hang: f64,
    #[k(48.0)] pub blocks: f64,
    #[k(0.7)] pub low_m: f64,
    #[k(0.3)] pub door_m: f64,
    #[k(1.5)] pub door_foot_m: f64,
}
