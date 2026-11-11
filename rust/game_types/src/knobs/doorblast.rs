//! The numbers of doorblast.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DoorblastK {
    /// metres inside which a stun can's blast takes an ordinary leaf off (what `detonate_stun` hands `doors_blast`): 0, a stun canister takes no door. Other
    ///  blasts hand their own number.
    pub tear_m: DoorblastKTearM,
    /// how hard a charge set on the leaf throws it, 0‥1 of `speed`
    #[k(0.55)] pub charge_k: f64,
    /// the radius (m) a set charge hands the destruction goal's blast entry point (`g.destr.blast`, where the build carries one: a build without it calls nothing):
    ///  SMALL on purpose — a charge shaped for the leaf reaches what leans on the frame and whoever listens to a blast, and takes no hole out of the wall beside the
    ///  door (that would be a wall charge: the destruction page's open call). 0 = the charge calls nobody
    #[k(0.8)] pub charge_r: f64,
    /// m/s the slab leaves at: at the very edge of tearM … hard against the blast
    #[k([3.2, 6.5])] pub speed: [f64; 2],
    /// its hop off the sill (m/s up), the rate its top starts over at (rad/s) and the turn the tearing hinge side gives it (rad/s)
    #[k(0.9)] pub up: f64,
    #[k(2.4)] pub tip: f64,
    #[k(1.3)] pub spin: f64,
    /// m/s² its dragging foot takes off it while it topples · the share of its speed a slab keeps through the slam flat · m/s² sliding flat · share kept off a wall
    #[k(4.0)] pub scrape: f64,
    #[k(0.4)] pub slam: f64,
    #[k(12.0)] pub slide: f64,
    #[k(0.15)] pub bounce: f64,
    /// free floor ahead (m beyond the leaf's own height) it needs to topple forward; with less it goes to the far wall and falls back through its doorway
    #[k(0.6)] pub room: f64,
    /// how loud the slab landing is (a 'door' noise at the slab, blamed on whoever set the blast off)
    #[k(0.9)] pub land_level: f64,
}

/// metres inside which a stun can's blast takes an ordinary leaf off (what `detonate_stun` hands `doors_blast`): 0, a stun canister takes no door. Other
///  blasts hand their own number.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DoorblastKTearM {
    #[k(0.0)] pub stun: f64,
}

/// GUNFIRE SPLINTERS A WOODEN LEAF APART (the maintainer: 'doors should splinter apart on sustained gunfire (at least no slower than drywall)', and from the
///  squad lesson, where six rounds read as too easy: 'it should be a real gameplay tradeoff (requires a lot of ammo? maybe like 12 shots?)', so `toll` below is 2).
///  THE COUNT comes off the wall engine: it hands every round that met a leaf to `leaf_round` with the weight a wall's cell would have taken for it, and the leaf
///  takes the destruction scale back out, so A COUNT IS A COST the same wherever the scale stands above nothing (a wall's rides it): a rifle round is 1, a pistol round
///  `walls.pistol_k` of that, of a shotgun's pellets the ones the walls are handed. The leaf stands `leaf_tough` of it — `toll` times plasterboard's own number (or wood's
///  if that were ever less) — ONE count for the whole leaf, wherever on it and on whichever face the rounds land, and it gives whole where drywall keeps its sill. A
///  round that goes on through counts as one that stops; so does one fired with the gun pressed through the leaf.
///  Past half its toll the leaf sheds splinters with every round. Then it goes IN PIECES, by the destruction's own gradient: strips along the grain cut
///  to uneven lengths and thrown the way the round was going as rigid debris (the pool's own cap: what it refuses goes as splinters), each TAGGED WOOD and
///  dressed by the break table (a snapped end, a splinter's taper, a board's torn corners — a door's paint would otherwise
///  read as masonry or sheet metal), then slivers, then dust-fine splinters; where the cuts fall goes by where the last round struck, so no two doors break alike. What is left is
///  the LOCK RAIL lying by the frame with the handles on it — the leaf's own box, so to every reader this is a door off its hinges (`leaf.off`: an open
///  doorway to sight, shots, sound, smoke, the AI and every door order; a reset hangs it; a checkpoint files the rail where it lies and a leaf's count
///  while it hangs). A steel door never does; nor does a leaf already off. Lives only where walls break (`wall_break_on`).
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct DoorshotK {
    /// how many times plasterboard's toll a wooden leaf stands (2 = twelve rifle rounds, twenty-four pistol; 1 = drywall's own six and twelve; ?doorcost=0 = 1)
    #[k(2.0)] pub toll: f64,
    /// the pieces: strips across the leaf × lengths up it, how uneven the cuts are (0‥1), and how many go as rigid pieces (past it, splinters)
    #[k(4.0)] pub strips: f64,
    #[k(3.0)] pub lengths: f64,
    #[k(0.6)] pub uneven: f64,
    #[k(11.0)] pub chunks: f64,
    /// m/s a piece leaves at along the round's way (lo, hi) · across it (±) · up (lo, hi) · its tumble (rad/s)
    #[k(alloc::vec![1.0, 3.2])] pub fly: Vec<f64>,
    #[k(1.1)] pub side: f64,
    #[k(alloc::vec![0.2, 1.8])] pub up: Vec<f64>,
    #[k(9.0)] pub spin: f64,
    /// every piece leaves already going over, its top the way the round went: tipped this far (rad: lo, hi) and toppling at this rate (rad/s: lo, hi) — a
    ///  plank let go dead upright comes down on its end and the debris solver, rightly, lets it stand there
    #[k(alloc::vec![0.15, 0.45])] pub tilt: Vec<f64>,
    #[k(alloc::vec![2.0, 5.0])] pub topple: Vec<f64>,
    /// splinters thrown on through, and back toward the gun, when it goes, at these m/s · the most a round sheds from a leaf past half its toll
    #[k(alloc::vec![60.0, 24.0])] pub chips: Vec<f64>,
    #[k(alloc::vec![4.0, 2.5])] pub chip_v: Vec<f64>,
    #[k(8.0)] pub shed: f64,
    /// the middle of the gradient: finger-long slivers thrown on through when it goes (how many, m/s)
    #[k(alloc::vec![22.0, 3.2])] pub slivers: Vec<f64>,
    /// how loud the leaf going is — a 'kick' at the door, blamed on whoever fired the round: the door's own crack and dust, and to the floor's ears a door
    ///  going in (the hearing check answers a kick floor-wide; the debrief files 'a door kicked in'), however quiet the gun was
    #[k(0.8)] pub level: f64,
}
