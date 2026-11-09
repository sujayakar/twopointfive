//! The numbers of lighting.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// A WEAPON LIGHT STOPS AT A WALL. A man's light is a light AT ITS LENS — on the carbine's handguard, under the pistol's rail, at the hand torch's glass — and a man
/// pressed to a wall (stacked on a door, crossing its frame, backed into a corner) has the reach to carry that lens into the wall or out through its far face: a
/// shadow ray stops a few centimetres short of the emitter it is cast at, so a light that close to the far face lit the room beyond, and to the CPU's light estimate
/// the men in it. Rounds have their own rule for the pressed muzzle; this is the light's, and it asks more: any leaf and an intact pane stop the LENS too (the light
/// then shines through the pane from this side, as light does). `gap` = how far short of the surface the EDGE of the emitter (lens + its radius) is kept, metres.
/// Off = the light at its lens wherever that is, as it was — the table's own value, so a host that never says otherwise is unchanged.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct LightwallK {
    #[k(0.03)] pub gap: f64,
}
