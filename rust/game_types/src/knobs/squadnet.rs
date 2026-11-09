//! The numbers of squadnet.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SquadnetK {
    /// how far from you a line from off the screen is drawn, as a share of the view's shorter side (with `edge` off)
    #[k(0.24)] pub near: f64,
    /// a line from off the screen is drawn at the edge of the view on his side, not near you
    #[k(true)] pub edge: bool,
    /// how far inside the view's border such a line is kept, as a share of the view's shorter side
    #[k(0.04)] pub edge_in: f64,
    /// the click a line from off the screen arrives with (gain; 0 = silent), pitched up off the defenders' net's own squelch
    #[k(0.3)] pub click: f64,
    #[k(1.3)] pub click_rate: f64,
    /// a teammate down: the same click, louder and pitched down (gain; rate)
    #[k(0.7)] pub down_gain: f64,
    #[k(0.55)] pub down_rate: f64,
    /// seconds a 'down' row stands
    #[k(5.0)] pub down_secs: f64,
}
