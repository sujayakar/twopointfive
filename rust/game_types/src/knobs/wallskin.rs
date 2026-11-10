//! The numbers of wallskin.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallskinK {
    /// m: a sheet is no deeper than this across the face it lies on: a coat of paint, a run of conduit and its clips, a board of card. Deeper is a body
    ///  (a duct, a case, a cabinet's drawn front) and stays the hangers' alone
    #[k(0.04)] pub thick: f64,
    /// m: … and its near side stands no further than this off the face of the solid behind it. Bands are authored on the face or a few millimetres proud,
    ///  but not all: of the levels' own, a dado stands 6 cm off its partition and a cork board 1.5 cm off its, with nothing between. The nearest solid this
    ///  near behind a part of a sheet is what that part is on: a paper on the front of a solid board is on the board, not on the wall behind the board
    #[k(0.08)] pub off: f64,
    /// caps: the pieces one sheet may stand in (16 at most: a piece's key is its sheet and one of 16 places), and all sheets together (past either the
    ///  sheet is cut by whole columns, or left as it last stood, and counted)
    #[k(12.0)] pub per_skin: f64,
    #[k(256.0)] pub max_pieces: f64,
    /// the frame's box list keeps this many rows free after ours (BoxWorld throws past WORLD.maxBoxes)
    #[k(64.0)] pub box_slack: f64,
}
