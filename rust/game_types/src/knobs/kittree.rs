//! The numbers of kittree.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// THE GEAR SCREEN AS A TREE (KITTREE). The maintainer, on Team & kit (10-19): "have some way of distinguishing different guns (some stats?), attachments
/// like silencer should be per gun", "should be symmetric between self and teammates", "sponge should show up, should only be part of stealth builds by
/// default", "ideally rifle attachments should be hierarchical under the rifle", "weird that we have both per person defaults (as issued, quiet set, etc.)
/// and then squad kits too".
/// A card is ONE LIST OF ROWS FOR EVERY MAN, you among them:
///   · each gun is a row with what rides on it UNDER it: its own suppressor (`suppressed` the primary's, `sideCan` the secondary's), the
///     weapon light, and what the launcher fires — sponge rounds (`rings`, a row for the first time), 40 mm grenades, sticky cameras — only on a gun that
///     has the tube;
///   · YOUR card has the suppressor, light and goggles rows a teammate's always had, and they are what you deploy with: a suppressor taken off is the same
///     gun with a bare muzzle (the pick's own bare row: gunslots.ts PICKS), no light is a dead L, no goggles a dead N;
///   · sponge rounds are a count like any other: NONE as issued, and a quiet or solo mission hands you `lend` of them over your card, as it hands its
///     cameras — so a stealth run starts with the five it always had and a loud one with none;
///   · the card's own row of sets is one reset (the squad's row of kits is the one list of named kits: ui/loadout.ts shows it at every team size).
/// What a gun is good at is drawn on its tile (ui/gunStats.ts: four bars read off its weapons.ts row).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct KittreeK {
    /// … and what a quiet or solo mission hands you over your card
    #[k(5.0)] pub lend: f64,
}
