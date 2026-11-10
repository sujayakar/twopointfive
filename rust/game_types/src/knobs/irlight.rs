//! The numbers of irlight.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct IrlightK {
    /// meter: the least the tube's picture must make of a floor (0‥1 of its range) for his light to stay out. The maintainer's strongroom read about 0.3 in
    ///  his shot; 0.05 is grain. Never judged on a real GPU
    #[k(0.1)] pub show: f64,
    /// auto: s the light stays on after the last reading that found floor too dark for the tube (NVG.blackAt, by the place's own light)
    #[k(1.5)] pub hold: f64,
    /// how bright the flood reads THROUGH THE TUBE: on-axis intensity × the picture's exposure. 8 (it was 30, and five of them at once put a lot at the tube's
    ///  ceiling): by the shader's own curve one light puts the floor two metres ahead at about 0.6 of the tube's range, four metres at 0.4, a wall six metres
    ///  off a little under half — and where all five of a stacked squad's overlap, four metres off reads about 0.6 where it read white
    #[k(8.0)] pub power: f64,
    /// m it reaches (the shader's range window: full to about half of this, gone at it)
    #[k(22.0)] pub reach: f64,
    /// the flood's half angle, degrees, and where inside it the full strength ends (20, was 40: the edge rolls off over most of the cone)
    #[k(65.0)] pub cone: f64,
    #[k(20.0)] pub core: f64,
    /// how far below level it points, degrees (6, was 11: less of it on the floor at his feet, which is nearest the lens and so the hottest thing in it)
    #[k(6.0)] pub dip: f64,
    #[k(5.0)] pub near_m: f64,
    #[k(0.003)] pub near_min: f64,
    #[k(3.0)] pub near_up: f64,
    #[k(12.0)] pub near_down: f64,
}
