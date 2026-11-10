//! The numbers of wet.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The knobs (Tab ▸ Weather ▸ wet characters; kept per machine like the whistle's). The first block is the model's, the second the look's — those ride to the
///  GPU per character instance so the shaders need no uniform of their own.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WetK {
    /// coat rise per second in a downpour (× (0.4 + 0.6 × rain rate))
    #[k(0.16666666666666666)] pub soak_per_sec: f64,
    /// coat fall per second out of the rain
    #[k(0.04)] pub dry_per_sec: f64,
    /// the drops standing on him run off / soak in this long after he comes out of the rain
    #[k(3.0)] pub bead_runoff_secs: f64,
    /// fraction of the drop lattice's cells holding a drop at full beads (× the beads level, which follows the rain rate)
    #[k(0.6)] pub bead_density: f64,
    /// drop radius (m) — ×0.75‥1.25 per drop
    #[k(0.0075)] pub bead_size: f64,
    /// how far a drop runs before it is spent (m) — the wet trail it draws behind it is this long at most
    #[k(0.11)] pub trail_len: f64,
    /// GGX roughness (perceptual: alpha = its square) of the water film's streaks and a drop's trail — tight: the glints are the point (each pixel widens the lobe by what it cannot resolve, gbuffer.wgsl fsSkin) …
    #[k(0.12)] pub rough_wet: f64,
    /// … and of merely damp cloth (the last of the coat as it dries: a wide dim lobe)
    #[k(0.55)] pub rough_dry: f64,
    /// wet fabric: albedo × lum^(darken × coat) … (its own luminance to that power: dark cloth drops hardest — soaked khaki to about a quarter — and the hue holds)
    #[k(1.0)] pub darken: f64,
    /// … × (1 − darkenMul × coat), then a quarter more saturated about that grey at full soak
    #[k(0.35)] pub darken_mul: f64,
    /// × the coat's specular (direct lights + the sheen off the ambient); 0 = darkening only
    #[k(1.0)] pub spec_gain: f64,
    /// a soaked man come indoors drips about this long (the rate follows his beads running off)
    #[k(5.0)] pub drip_secs: f64,
    /// wet footprints after coming in from the rain, fainter each step
    #[k(10.0)] pub prints: f64,
    /// m of damp cloth ABOVE a waterline (a man wet to the knee by wading: the coat wicks this far up past the water and fades to dry across it)
    #[k(0.12)] pub line_band: f64,
    /// each print dries out over this long
    #[k(30.0)] pub print_fade_secs: f64,
}
