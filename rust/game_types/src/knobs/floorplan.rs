//! The numbers of floorplan.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// THE PLAN ON THE FLOOR'S NUMBERS. What it lists is the Rust game's (rust/game_rules/src/view/plan.rs, `view.plan`) and ui/planfloor.ts draws it: its men's ways, marks and fans, what waits on the
/// key, the throws. The maintainer (10-13): "it would be good to have the squad move to do a throw (and maybe visualize who's doing it and the path on the ground?)"; the Door Kickers 2
/// focus list's item 6: "four placed men are four lines of text. Done looks like: cover the top-left corner: you still know each man's mark, sector and which door waits on F. Our men only."
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PlanK {
    /// seconds an order just given is drawn at full strength — its men ringed, its ways bright
    #[k(2.5)] pub fresh_secs: f64,
    /// how strong a way is drawn after that, while he still walks it · a mark and a fan under a man who stands
    #[k(0.6)] pub walk_dim: f64,
    #[k(0.45)] pub rest_dim: f64,
    /// a mark's ring, metres · a fan's reach, metres, and half angle, degrees · a sight line to what he watches is drawn out to this far, metres
    #[k(0.3)] pub ring_m: f64,
    #[k(1.6)] pub sector_m: f64,
    #[k(24.0)] pub sector_deg: f64,
    #[k(14.0)] pub sight_m: f64,
    /// the nav grid is asked for a leg his own path does not cover at most this often, seconds
    #[k(0.5)] pub reask_secs: f64,
    /// the quiet pair's man waiting on the mark: the key (and, with `keyLit` off, its bar) is this far ahead of his feet, metres
    #[k(0.55)] pub pair_bar_m: f64,
    /// A GAME OF SEVERAL PLAYERS: another player's squad's plan is drawn on this page this much as strong as the page's own (ui/planfloor.ts
    ///  plansToDraw); 0 = not asked for, the page as it was. A guess: nobody has seen it on a GPU. Picture only
    #[k(0.45)] pub others: f64,
}
