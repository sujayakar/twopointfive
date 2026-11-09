//! The numbers of nvg.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct NvgK {
    /// s of quiet — nothing loud heard, no alarm (nobody seen hunting: huntFrees) — before a compromised team goes dark again
    #[k(30.0)] pub quiet_secs: f64,
    /// s Sam's light counts as on after it was last switched on in his hands (changing guns, a holster and a draw put it away for a moment)
    #[k(1.5)] pub lead_tail: f64,
    /// the place is dark to the bare eye under this on the meter's scale: goggles come down (LIGHTS.litAt's line — over it the room's own lights do)
    #[k(0.5)] pub dark_at: f64,
    /// too dark even for the tube: something he is looking at reads under this THROUGH it (own judgement only: the light rule has him rather than he walk in
    ///  blind; 0 = never — dark is dark). Read off the CPU's direct light plus the moon, no bounce: it calls black some floor the page's picture shows faintly.
    ///  blackReads / clearReads: readings running (two a second) before a place counts as black / as showing again — no white light for one sample in passing
    #[k(0.15)] pub black_at: f64,
    #[k(2.0)] pub black_reads: f64,
    #[k(3.0)] pub clear_reads: f64,
    /// the reflectance of a man's clothes in the tube's signal, and the rms of its scintillation at no signal (post.wgsl: ±0.5 × 0.16, uniform)
    #[k(0.3)] pub albedo: f64,
    #[k(0.046)] pub noise: f64,
    /// s a man's light must be off (and the place dark) before his goggles come down / on before they go up: a light the rule pulses for a moment flips nothing
    #[k(0.25)] pub down_secs: f64,
    #[k(0.7)] pub up_secs: f64,
    /// s between two of the team's 'going dark' calls
    #[k(8.0)] pub call_gap: f64,
}
