//! The numbers of aimweight.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AimweightK {
    /// top rate the gun's heading turns at, rad/s, for a gun of handling 1 (the pistol); × handlingOf(the gun in his hands)
    #[k(11.0)] pub rate: f64,
    /// seconds: the spring's time constant — how soon a small correction lands, and how far the gun trails a steadily moving cursor (rate × this)
    #[k(0.05)] pub smooth: f64,
    /// how much a heavier gun slows the turn: the top rate is × handling ^ this (1 = the men's own share; the rifle's 0.8 alone is hard to feel)
    #[k(1.5)] pub heft: f64,
    /// the handling of empty hands (a holstered / slung gun, a can in the fist): he turns his head and shoulders, not a weapon
    #[k(1.25)] pub bare: f64,
    /// metres across the line, at the target's range (never less than a metre out), inside which the gun counts as ON the cursor: shots are the build's own
    #[k(0.3)] pub on_m: f64,
    /// how fast the circle's RANGE and height follow the cursor's, 1/s (picture only: a round flies along the heading whatever the range)
    #[k(18.0)] pub reach: f64,
    /// seconds a launcher's load, let go of while the tube is still swinging, waits for the circle to reach the cursor (its arc drawn meanwhile); past that it goes
    ///  where the circle is. 0 = sent the frame the button comes up, wherever the circle is (as the weight first shipped)
    #[k(0.5)] pub launch_wait: f64,
}
