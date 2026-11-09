use crate::game::Game;
use crate::m;

/// The tube's gain, never under 1: the curve is ln(1 + L·gain) / ln(1 + gain).   (env.ts tubeGain)
pub fn tube_gain(g: &crate::game::Game) -> f64 {
    m::max(if g.k.env.tube_gain > 0.0 { g.k.env.tube_gain } else { 1.0 }, 1.0)
}
