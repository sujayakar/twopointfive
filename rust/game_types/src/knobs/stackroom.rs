//! The numbers of stackroom.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StackroomK {
    /// a body's room between a mark dealt again and every other mark, Sam and marks held on the door · … in the last resort (6)
    #[k(0.75)] pub apart: f64,
    #[k(0.5)] pub squeeze: f64,
    /// a mark is round a wall, not on the face's floor, when the walk to it from the doorway is over `roundM` + `roundK` × the straight line
    #[k(2.0)] pub round_m: f64,
    #[k(1.6)] pub round_k: f64,
    /// (4) how far a man walks from the doorway for a place on the face's floor · (5) how far the next room's doorway may be by walking, and how far from that
    ///  doorway its places are
    #[k(9.0)] pub walk_m: f64,
    #[k(14.0)] pub next_walk_m: f64,
    #[k(3.5)] pub next_m: f64,
    /// (7) how far back from the doorway a place in the opening's own lane is, down a passage no wider than its door
    #[k(1.6)] pub lane_back: f64,
    /// a leaf at this angle or more (rad), unlatched, stands open (one off its hinges always does) · a place is IN VIEW when `viewCells` nav cells of the floor
    ///  within `viewFar` m beyond the door have a line to it through the opening at `viewEye` m, the leaves as they stand (the play-through watcher's measure)
    #[k(0.6)] pub view_ajar: f64,
    #[k(3.0)] pub view_cells: f64,
    #[k(7.0)] pub view_far: f64,
    #[k(1.2)] pub view_eye: f64,
    /// the places tried: from the jamb on along the wall every `viewStep` m to `viewPast` m, at the makers' own distance off the wall, then `viewOff` · a body's
    ///  room between one of them and every other mark · a man is ON a hand of the opening when he stands `viewOut` m or more past that jamb, on this face
    ///  within `viewMen` m of the door (before the opening, or further off, he can take a place on either hand without crossing) · the door's wall stands behind
    ///  a place when something solid is met between it and `viewWall` m beyond the door's line
    #[k(0.3)] pub view_step: f64,
    #[k(4.2)] pub view_past: f64,
    #[k(0.4)] pub view_off: f64,
    #[k(0.85)] pub view_apart: f64,
    #[k(0.3)] pub view_out: f64,
    #[k(12.0)] pub view_men: f64,
    #[k(0.35)] pub view_wall: f64,
}
