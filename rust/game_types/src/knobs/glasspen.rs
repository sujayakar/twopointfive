//! The numbers of glasspen.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

/// the numbers
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct GlasspenK {
    /// a round's energy at the muzzle by weapon row (1 = a 5.56 rifle round), and by class for a row not listed
    #[k({ let mut t = JsMap::new(); t.set(String::from("fn2000"), 1.0); t.set(String::from("mk18bare"), 1.0); t.set(String::from("g19bare"), 0.6); t.set(String::from("ak74m"), 1.0); t.set(String::from("akms"), 1.1); t.set(String::from("pkm"), 1.3); t.set(String::from("aks74u"), 0.9); t.set(String::from("fiveseven"), 0.6); t.set(String::from("guard9mm"), 0.5); t.set(String::from("m870"), 0.3); t.set(String::from("m1014"), 0.3); t.set(String::from("mp7"), 0.6); t })] pub energy: JsMap<String, f64>,
    #[k({ let mut t = JsMap::new(); t.set(String::from("rifle"), 1.0); t.set(String::from("carbine"), 0.9); t.set(String::from("pistol"), 0.5); t })] pub by_class: JsMap<String, f64>,
    /// the energy a line-of-fire question assumes when it has no man's gun to ask (a station picked for 'whoever stands here'): the sidearm's
    #[k(0.5)] pub default_energy: f64,
    /// energy a sheet takes from a round crossing it square-on, per refThick of glass
    #[k({ let mut t = JsMap::new(); t.set(crate::glasspen::PaneType::Annealed, 0.1); t.set(crate::glasspen::PaneType::Tempered, 0.16); t.set(crate::glasspen::PaneType::Laminated, 0.42); t })] pub cost: JsMap<crate::glasspen::PaneType, f64>,
    #[k(0.012)] pub ref_thick: f64,
    /// the slant's cap on 1 ∕ cos θ, and the cosine under which a round skids along the sheet and is spent in it (≈ 80° off the normal)
    #[k(3.0)] pub oblique_max: f64,
    #[k(0.17)] pub graze_cos: f64,
    /// under this much left on the far side the round is spent in the pane; panes one round can cross
    #[k(0.12)] pub min_energy: f64,
    #[k(4.0)] pub max_panes: f64,
    /// the scatter cone's half-angle in degrees at toll = all the round had (× toll ∕ energy in), and the share of it the round breaks toward the normal on a slant
    #[k(5.0)] pub deflect_deg: f64,
    #[k(0.5)] pub bend_k: f64,
    /// a crossing within holeR (m) of an earlier one on the pane goes through the hole: × holeK on the toll and on the turn; crossings remembered per pane
    #[k(0.04)] pub hole_r: f64,
    #[k(0.25)] pub hole_k: f64,
    #[k(24.0)] pub holes: f64,
    /// A LOAD OF SHOT: a pellet meeting a pane within shellR (m) of a mark its own shell already made there adds no mark of its own (the shell rule, above);
    ///  0 = every pellet its own impact, hole and jet (the probes' red path — not the old behaviour, which lives behind the glass parts' own switches)
    #[k(0.04)] pub shell_r: f64,
    /// GLASS IS NOT COVER: a place by a door is no place to stack when at least this many of the floor lattice's
    ///  points beyond that door have a line of FIRE to a man stood there that crosses a pane, for a round of stackEnergy (a rifle's: the worst gun he may meet); 0 = the rule off
    #[k(3.0)] pub stack_eyes: f64,
    #[k(1.0)] pub stack_energy: f64,
    #[k(0.75)] pub stack_least_k: f64,
    /// our energy → the fracture model's impact unit (its 1 = one pistol round square on)
    #[k(2.0)] pub fracture_k: f64,
}
