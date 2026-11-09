//! The numbers of wallpen.
use crate::bytes::ToBytes;
use crate::js::*;
use game_derive::Knobs;

/// the numbers (constants until somebody has played it)
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct WallpenK {
    /// energy a thing takes from a round crossing it: its skins …
    #[k({ let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.1); t.set(crate::wallpen::ThinStuff::Wood, 0.0); t.set(crate::wallpen::ThinStuff::Fabric, 0.03); t.set(crate::wallpen::ThinStuff::Cardboard, 0.02); t.set(crate::wallpen::ThinStuff::Foliage, 0.0); t })] pub base: JsMap<crate::wallpen::ThinStuff, f64>,
    /// … + this per metre of the round's path inside the box (plasterboard is two sheets and air; a leaf is wood right through: 5 cm square-on takes 0.225)
    #[k({ let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.5); t.set(crate::wallpen::ThinStuff::Wood, 4.5); t.set(crate::wallpen::ThinStuff::Fabric, 1.0); t.set(crate::wallpen::ThinStuff::Cardboard, 1.0); t.set(crate::wallpen::ThinStuff::Foliage, 0.3); t })] pub per_m: JsMap<crate::wallpen::ThinStuff, f64>,
    /// m: how thick across (its thin level axis) an upright thing of each stuff may be and still be a thin thing — plasterboard's is the wall engine's own
    #[k({ let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.17); t.set(crate::wallpen::ThinStuff::Wood, 0.12); t.set(crate::wallpen::ThinStuff::Fabric, 0.13); t.set(crate::wallpen::ThinStuff::Cardboard, 0.7); t.set(crate::wallpen::ThinStuff::Foliage, 2.0); t })] pub thin_max: JsMap<crate::wallpen::ThinStuff, f64>,
    /// m: the longest path through one a round survives (longer: it is running along inside the sheet, and is spent in it)
    #[k({ let mut t = JsMap::new(); t.set(crate::wallpen::ThinStuff::Drywall, 0.4); t.set(crate::wallpen::ThinStuff::Wood, 0.14); t.set(crate::wallpen::ThinStuff::Fabric, 0.3); t.set(crate::wallpen::ThinStuff::Cardboard, 0.8); t.set(crate::wallpen::ThinStuff::Foliage, 2.5); t })] pub max_thru: JsMap<crate::wallpen::ThinStuff, f64>,
    /// under this much left on the far side the round is spent in the thing; things one round can cross
    #[k(0.12)] pub min_energy: f64,
    #[k(5.0)] pub max_things: f64,
    /// the scatter cone's half-angle in degrees at toll = all the round had (× toll ∕ energy in)
    #[k(4.0)] pub deflect_deg: f64,
    /// m from the first face crossed to the place he knows, beyond which the man is not 'behind it' · rounds he puts through on one sighting · s before the same
    ///  thing is his to fire through again
    #[k(4.0)] pub behind_m: f64,
    #[k(4.0)] pub thru_rounds: f64,
    #[k(6.0)] pub thru_rest_secs: f64,
}
